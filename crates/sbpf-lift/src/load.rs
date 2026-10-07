//! Загрузка `.so` через `solana-sbpf`: разбор ELF, релокации, верификация.

use std::{fs, io, path::Path};

use solana_sbpf::{
    elf::{Executable, get_sbpf_version},
    elf_parser::ElfParserError,
    error::EbpfError,
    program::SBPFVersion,
    verifier::RequisiteVerifier,
};

use crate::syscalls::{NoExec, loader};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("не удалось прочитать файл: {0}")]
    Io(#[from] io::Error),
    #[error("не ELF-файл")]
    NotElf,
    #[error("ошибка заголовка ELF: {0}")]
    Header(#[from] ElfParserError),
    /// Поддерживаются только v0 (контракты в мейннете) и v3 (новые деплои).
    #[error("неподдерживаемая версия sBPF: {0:?}")]
    UnsupportedVersion(SBPFVersion),
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
    /// ELF и релокации разбирает `solana-sbpf`, затем байткод проходит `RequisiteVerifier`,
    /// как при деплое: дальше можно полагаться на допустимые опкоды, регистры,
    /// переходы в пределах программы и целые `lddw`.
    pub fn load(bytes: &[u8]) -> Result<Self, LoadError> {
        if !bytes.starts_with(b"\x7fELF") {
            return Err(LoadError::NotElf);
        }
        match get_sbpf_version(bytes)? {
            SBPFVersion::V0 | SBPFVersion::V3 => {}
            v => return Err(LoadError::UnsupportedVersion(v)),
        }
        let executable = Executable::from_elf(bytes, loader()).map_err(LoadError::Elf)?;
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
