//! Конкретный раннер: исполняет `.so` на одной инструкции через Mollusk —
//! тот же загрузчик, syscall'ы и сериализацию аккаунтов, что у валидатора agave.

pub mod example;

use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;

use mollusk_svm::{InvocationInspectCallback, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_runtime::invoke_context::InvokeContext;
use solana_pubkey::Pubkey;
use solana_svm_log_collector::LogCollector;
use solana_transaction_context::instruction_accounts::InstructionAccount;

pub use example::{AccountInput, AccountOutput, Example, ExtraProgram, Outcome, Status};
pub use mollusk_svm;

/// Лимит транзакции без инструкций ComputeBudget, как у валидатора.
pub const DEFAULT_COMPUTE_UNIT_LIMIT: u32 = 200_000;
/// Больший лимит валидатор урезает до этого значения.
pub const MAX_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub registers: bool,
    pub trace: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("{field}: некорректный pubkey `{value}`")]
    Pubkey { field: String, value: String },
    #[error("{field}: некорректный hex: {source}")]
    Hex {
        field: String,
        source: hex::FromHexError,
    },
    #[error("{0}: путь к программе должен оканчиваться на .so")]
    NotSo(PathBuf),
    #[error("аккаунт {0} повторяется с разным состоянием")]
    ConflictingAccount(String),
    #[error("Mollusk: {0}")]
    Mollusk(String),
}

pub fn run_file(path: &Path, options: Options) -> Result<Outcome, RunError> {
    let text = std::fs::read_to_string(path).map_err(|source| RunError::Io {
        path: path.to_owned(),
        source,
    })?;
    let example: Example = serde_json::from_str(&text).map_err(|source| RunError::Json {
        path: path.to_owned(),
        source,
    })?;
    run(&example, path.parent().unwrap_or(Path::new(".")), options)
}

pub fn run(example: &Example, base_dir: &Path, options: Options) -> Result<Outcome, RunError> {
    let program_id = pubkey("program_id", &example.program_id)?;
    let program_path = so_path(base_dir, &example.program)?;
    let extra = example
        .programs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let id = pubkey(&format!("programs[{i}].program_id"), &p.program_id)?;
            let path = so_path(base_dir, &p.program)?;
            let elf = std::fs::read(&path).map_err(|source| RunError::Io { path, source })?;
            Ok((id, elf))
        })
        .collect::<Result<Vec<_>, RunError>>()?;
    let data = from_hex("instruction_data", &example.instruction_data)?;
    let accounts = example
        .accounts
        .iter()
        .enumerate()
        .map(|(i, a)| account(i, a))
        .collect::<Result<Vec<_>, _>>()?;

    let metas = accounts
        .iter()
        .zip(&example.accounts)
        .map(|((key, _), a)| AccountMeta {
            pubkey: *key,
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();
    let instruction = Instruction {
        program_id,
        accounts: metas,
        data,
    };
    let mut unique: Vec<(Pubkey, Account)> = Vec::new();
    for (key, account) in &accounts {
        match unique.iter().find(|(k, _)| k == key) {
            None => unique.push((*key, account.clone())),
            Some((_, first)) if first != account => {
                return Err(RunError::ConflictingAccount(key.to_string()));
            }
            Some(_) => {}
        }
    }

    let tracing = options.registers || options.trace;
    let traces = Rc::new(RefCell::new(Vec::new()));
    let logs = LogCollector::new_ref();
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        // Трассировка включается только при создании Mollusk, а такой конструктор
        // ищет ELF по имени `<name>.so`; абсолютный путь подменяет каталоги поиска.
        let name = program_path.with_extension("");
        let mut mollusk = Mollusk::new_debuggable(&program_id, &name.to_string_lossy(), tracing);
        for (id, elf) in &extra {
            mollusk.add_program_with_loader_and_elf(
                id,
                &solana_sdk_ids::bpf_loader_upgradeable::id(),
                elf,
            );
        }
        mollusk.compute_budget.compute_unit_limit = example
            .compute_unit_limit
            .unwrap_or(DEFAULT_COMPUTE_UNIT_LIMIT)
            .min(MAX_COMPUTE_UNIT_LIMIT)
            .into();
        mollusk.config.panic = false;
        mollusk.logger = Some(logs.clone());
        mollusk.invocation_inspect_callback = Box::new(TopLevelTrace(traces.clone()));
        mollusk.process_instruction(&instruction, &unique)
    }))
    .map_err(|payload| RunError::Mollusk(panic_message(payload)))?;

    let (status, error) = match &result.raw_result {
        Err(e) => (Status::Failure, Some(format!("{e:?}"))),
        Ok(()) if !result.program_result.is_ok() => {
            (Status::Failure, Some("InsufficientFundsForRent".to_owned()))
        }
        Ok(()) => (Status::Success, None),
    };
    let committed = status == Status::Success;
    let trace = traces.take();
    let logs = logs.borrow().get_recorded_content().to_vec();
    Ok(Outcome {
        status,
        error,
        compute_units: result.compute_units_consumed,
        return_data: hex::encode(&result.return_data),
        accounts: unique
            .iter()
            .map(|(key, input)| {
                let account = result
                    .get_account(key)
                    .filter(|_| committed)
                    .unwrap_or(input);
                AccountOutput {
                    key: key.to_string(),
                    owner: account.owner.to_string(),
                    lamports: account.lamports,
                    data: hex::encode(&account.data),
                    executable: account.executable,
                }
            })
            .collect(),
        logs,
        registers: options.registers.then(|| trace.last().copied()).flatten(),
        trace: options.trace.then_some(trace),
    })
}

/// Трасса регистров вызова верхнего уровня; трассы CPI отбрасываются.
struct TopLevelTrace(Rc<RefCell<Vec<[u64; 12]>>>);

impl InvocationInspectCallback for TopLevelTrace {
    fn before_invocation(
        &self,
        _: &Mollusk,
        _: &Pubkey,
        _: &[u8],
        _: &[InstructionAccount],
        _: &mut InvokeContext,
        _: bool,
    ) {
    }

    fn after_invocation(&self, _: &Mollusk, invoke_context: &InvokeContext, enabled: bool) {
        if !enabled {
            return;
        }
        invoke_context.iterate_vm_traces(&|context, _, trace| {
            if context.get_stack_height() == 1 {
                self.0.borrow_mut().extend_from_slice(trace);
            }
        });
    }
}

fn so_path(base_dir: &Path, program: &str) -> Result<PathBuf, RunError> {
    let path = std::path::absolute(base_dir.join(program)).map_err(|source| RunError::Io {
        path: base_dir.join(program),
        source,
    })?;
    if path.extension().is_none_or(|ext| ext != "so") {
        return Err(RunError::NotSo(path));
    }
    if !path.is_file() {
        return Err(RunError::Io {
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
            path,
        });
    }
    Ok(path)
}

fn account(index: usize, a: &AccountInput) -> Result<(Pubkey, Account), RunError> {
    let field = |name: &str| format!("accounts[{index}].{name}");
    Ok((
        pubkey(&field("key"), &a.key)?,
        Account {
            lamports: a.lamports,
            data: from_hex(&field("data"), &a.data)?,
            owner: pubkey(&field("owner"), &a.owner)?,
            executable: a.executable,
            rent_epoch: a.rent_epoch.unwrap_or(u64::MAX),
        },
    ))
}

fn pubkey(field: &str, value: &str) -> Result<Pubkey, RunError> {
    Pubkey::from_str(value).map_err(|_| RunError::Pubkey {
        field: field.to_owned(),
        value: value.to_owned(),
    })
}

fn from_hex(field: &str, value: &str) -> Result<Vec<u8>, RunError> {
    hex::decode(value).map_err(|source| RunError::Hex {
        field: field.to_owned(),
        source,
    })
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
        .unwrap_or_else(|| "паника без сообщения".to_owned())
}
