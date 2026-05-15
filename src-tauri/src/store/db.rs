// SQLite index — placeholder for v2 full-text search
// For v1 MVP, we scan directories directly on each request

use anyhow::Result;
use std::path::Path;

pub struct SkillIndex;

impl SkillIndex {
    pub fn new(_db_dir: &Path) -> Result<Self> {
        // v1: no-op, scanning directly
        Ok(Self)
    }
}
