pub mod filter;
pub mod formats;
pub mod parser;
pub mod scanner;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Presence of a skill in a specific tool
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillPresence {
    /// Skill exists as a symlink to the central store
    Symlinked,
    /// Skill exists as a copy (possibly drifted)
    Copied,
    /// Skill is absent from this tool
    Absent,
    /// Skill copy differs from central store
    Drifted,
}

/// A skill discovered in the central store or a tool's skills directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Filesystem-safe directory name
    pub slug: String,
    /// Human-readable name from SKILL.md frontmatter
    pub name: String,
    /// Description from SKILL.md frontmatter
    pub description: String,
    /// Tags/categories from SKILL.md frontmatter
    pub tags: Vec<String>,
    /// Map of tool_id → presence state
    pub presence: HashMap<String, SkillPresence>,
    /// blake3 hash of SKILL.md content
    pub content_hash: String,
}

impl Skill {
    pub fn new(slug: impl Into<String>) -> Self {
        let slug = slug.into();
        Self {
            name: slug.clone(),
            slug,
            description: String::new(),
            tags: Vec::new(),
            presence: HashMap::new(),
            content_hash: String::new(),
        }
    }
}
