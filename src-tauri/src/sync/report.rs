use serde::{Deserialize, Serialize};

/// Individual sync result for one tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub tool_id: String,
    pub item_name: String,
    pub outcome: SyncOutcome,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyncOutcome {
    Success,
    Skipped,
    Failed,
}

/// Collection of sync results for one operation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncReport {
    pub results: Vec<SyncResult>,
}

impl SyncReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, result: SyncResult) {
        self.results.push(result);
    }

    pub fn success(tool_id: impl Into<String>, item_name: impl Into<String>) -> SyncResult {
        SyncResult {
            tool_id: tool_id.into(),
            item_name: item_name.into(),
            outcome: SyncOutcome::Success,
            message: None,
        }
    }

    pub fn failed(
        tool_id: impl Into<String>,
        item_name: impl Into<String>,
        msg: impl Into<String>,
    ) -> SyncResult {
        SyncResult {
            tool_id: tool_id.into(),
            item_name: item_name.into(),
            outcome: SyncOutcome::Failed,
            message: Some(msg.into()),
        }
    }

    pub fn skipped(tool_id: impl Into<String>, item_name: impl Into<String>) -> SyncResult {
        SyncResult {
            tool_id: tool_id.into(),
            item_name: item_name.into(),
            outcome: SyncOutcome::Skipped,
            message: None,
        }
    }

    pub fn has_failures(&self) -> bool {
        self.results.iter().any(|r| r.outcome == SyncOutcome::Failed)
    }
}
