use anyhow::{Context, Result};
use serde_json::json;
use std::path::Path;

use crate::driver::FileFilter;
use crate::skills::parser;

use super::copy;

/// Copy skill + generate extra manifest files (e.g. Codex's plugin.json)
pub fn sync(
    source: &Path,
    target: &Path,
    filter: Option<&FileFilter>,
    generator: &str,
    skill_slug: &str,
) -> Result<()> {
    // First do the normal copy
    copy::sync(source, target, filter)?;

    // Then generate extra files based on the generator type
    match generator {
        "codex-skill" => generate_codex_skill_manifest(source, target, skill_slug),
        other => {
            eprintln!("Warning: unknown generator '{}', skipping manifest generation", other);
            Ok(())
        }
    }
}

/// Generate Codex plugin.json alongside a copied skill
fn generate_codex_skill_manifest(source: &Path, target: &Path, skill_slug: &str) -> Result<()> {
    // Parse SKILL.md to get metadata
    let skill_file = source.join("SKILL.md");
    let skill = if skill_file.exists() {
        parser::parse_skill_file(&skill_file, skill_slug)?
    } else {
        crate::skills::Skill::new(skill_slug)
    };

    let manifest = json!({
        "name": skill.slug,
        "version": "1.0.0",
        "description": skill.description,
        "type": "skill",
        "entry": "SKILL.md",
        "tags": skill.tags,
    });

    let manifest_path = target.join("plugin.json");
    let serialized = serde_json::to_string_pretty(&manifest)?;
    std::fs::write(&manifest_path, serialized)
        .with_context(|| format!("writing {}", manifest_path.display()))?;

    Ok(())
}
