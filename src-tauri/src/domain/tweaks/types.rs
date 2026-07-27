use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakApplyResult {
    pub id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakStatus {
    pub id: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakRegistrySnapshot {
    pub hive: String,
    pub path: String,
    pub name: String,
    pub value: Option<TweakRegistryValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum TweakRegistryValue {
    Dword(u32),
    Text(String),
}

pub fn applied(id: &str, message: &str) -> TweakApplyResult {
    result(id, "applied", message)
}

pub fn skipped(id: &str, message: &str) -> TweakApplyResult {
    result(id, "skipped", message)
}

pub fn failed(id: &str, message: &str) -> TweakApplyResult {
    result(id, "failed", message)
}

pub fn result(id: &str, status: &str, message: &str) -> TweakApplyResult {
    TweakApplyResult {
        id: id.to_string(),
        status: status.to_string(),
        message: message.to_string(),
    }
}

pub fn collect_result<const N: usize>(
    id: &str,
    results: [Result<(), String>; N],
    success: &str,
) -> TweakApplyResult {
    let errors = results
        .into_iter()
        .filter_map(|result| result.err())
        .collect::<Vec<_>>();
    if errors.is_empty() {
        applied(id, success)
    } else {
        failed(id, &errors.join("; "))
    }
}
