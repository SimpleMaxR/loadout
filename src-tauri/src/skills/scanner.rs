use anyhow::Result;
use std::path::Path;

use super::{parser, Skill};

/// Scan a skills directory and return all discovered skills
/// Each subdirectory containing a skill_entry file (e.g., SKILL.md) is a skill
pub fn scan_dir(dir: &Path, skill_entry: &str) -> Result<Vec<Skill>> {
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut skills = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            // Also handle symlinks pointing to dirs
            let meta = std::fs::metadata(&path);
            if meta.map(|m| !m.is_dir()).unwrap_or(true) {
                continue;
            }
        }

        let skill_file = path.join(skill_entry);
        if !skill_file.exists() {
            continue;
        }

        let slug = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let skill = parser::parse_skill_file(&skill_file, &slug)?;
        skills.push(skill);
    }

    skills.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(skills)
}
