//! Загрузка `.so` через `solana-sbpf`: разбор ELF, релокации, верификация.

use std::{fs, io, path::Path};

use solana_sbpf::{
    elf::{ElfError, Executable},
    error::EbpfError,
    program::SBPFVersion,
    verifier::RequisiteVerifier,
};

use crate::syscalls::{NoExec, loader};

/// Ошибка загрузки программы.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("не удалось прочитать файл: {0}")]
    Io(#[from] io::Error),
    /// Поддерживаются только v0 (все контракты в мейннете) и v3 (новые деплои).
    /// `None` — версия выше v3, загрузчик не сообщает какая.
    #[error("неподдерживаемая версия sBPF: {}", match .0 { Some(v) => format!("{v:?}"), None => "выше V3".into() })]
    UnsupportedVersion(Option<SBPFVersion>),
    #[error("ошибка загрузки ELF: {0}")]
    Elf(EbpfError),
    #[error("программа не прошла верификацию: {0}")]
    Verifier(EbpfError),
}

/// Загруженная и проверенная программа.
pub struct Program {
    executable: Executable<NoExec>,
}

impl Program {
    /// Загружает программу так же, как валидатор при деплое: ELF и релокации
    /// разбирает `solana-sbpf`, затем байткод проходит `RequisiteVerifier`.
    pub fn load(bytes: &[u8]) -> Result<Self, LoadError> {
        let executable = Executable::from_elf(bytes, loader()).map_err(|e| match e {
            EbpfError::ElfError(ElfError::UnsupportedSBPFVersion) => {
                LoadError::UnsupportedVersion(None)
            }
            e => LoadError::Elf(e),
        })?;
        match executable.get_sbpf_version() {
            SBPFVersion::V0 | SBPFVersion::V3 => {}
            v => return Err(LoadError::UnsupportedVersion(Some(v))),
        }
        executable
            .verify::<RequisiteVerifier>()
            .map_err(LoadError::Verifier)?;
        Ok(Self { executable })
    }

    pub fn load_file(path: impl AsRef<Path>) -> Result<Self, LoadError> {
        Self::load(&fs::read(path)?)
    }

    pub fn version(&self) -> SBPFVersion {
        self.executable.get_sbpf_version()
    }

    pub fn executable(&self) -> &Executable<NoExec> {
        &self.executable
    }
}
