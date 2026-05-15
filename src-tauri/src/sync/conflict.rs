use serde::{Deserialize, Serialize};

/// Conflict between central store and a tool's local version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConflict {
    pub slug: String,
    pub tool_id: String,
    pub store_hash: String,
    pub tool_hash: String,
}

impl SkillConflict {
    pub fn new(
        slug: impl Into<String>,
        tool_id: impl Into<String>,
        store_hash: impl Into<String>,
        tool_hash: impl Into<String>,
    ) -> Self {
        Self {
            slug: slug.into(),
            tool_id: tool_id.into(),
            store_hash: store_hash.into(),
            tool_hash: tool_hash.into(),
        }
    }
}
