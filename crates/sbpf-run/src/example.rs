//! Формат примера (вход раннера) и результата (выход). Описан в `docs/runner.md`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub program: String,
    pub program_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub programs: Vec<ExtraProgram>,
    pub instruction_data: String,
    #[serde(default)]
    pub accounts: Vec<AccountInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_unit_limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtraProgram {
    pub program_id: String,
    pub program: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInput {
    pub key: String,
    pub owner: String,
    pub lamports: u64,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub executable: bool,
    #[serde(default)]
    pub is_signer: bool,
    #[serde(default)]
    pub is_writable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rent_epoch: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub status: Status,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub compute_units: u64,
    pub return_data: String,
    pub accounts: Vec<AccountOutput>,
    pub logs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registers: Option<Option<[u64; 12]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<Vec<[u64; 12]>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Failure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountOutput {
    pub key: String,
    pub owner: String,
    pub lamports: u64,
    pub data: String,
    pub executable: bool,
}
