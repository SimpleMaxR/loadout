pub mod copy;
pub mod copy_with_manifest;
pub mod symlink;

use anyhow::Result;
use std::path::Path;

use crate::driver::{FileFilter, SkillFormat};

/// Sync a skill to a target tool directory using the appropriate strategy
pub fn sync_skill(
    format: &SkillFormat,
    skill_source: &Path,   // e.g. ~/.loadout/store/skills/git-workflow/
    target_dir: &Path,     // e.g. ~/.codebuddy/skills/
    skill_slug: &str,
    filter: Option<&FileFilter>,
) -> Result<()> {
    let target = target_dir.join(skill_slug);

    match format {
        SkillFormat::Symlink => symlink::sync(skill_source, &target),
        SkillFormat::Copy => copy::sync(skill_source, &target, filter),
        SkillFormat::CopyWithManifest { generator } => {
            copy_with_manifest::sync(skill_source, &target, filter, generator, skill_slug)
        }
    }
}

/// Remove a skill from a target tool directory
pub fn remove_skill(
    format: &SkillFormat,
    target_dir: &Path,
    skill_slug: &str,
) -> Result<()> {
    let target = target_dir.join(skill_slug);

    if !target.exists() && !target.is_symlink() {
        return Ok(());
    }

    match format {
        SkillFormat::Symlink => {
            // Remove just the symlink
            std::fs::remove_file(&target)
                .or_else(|_| std::fs::remove_dir_all(&target))?;
        }
        SkillFormat::Copy | SkillFormat::CopyWithManifest { .. } => {
            std::fs::remove_dir_all(&target)?;
        }
    }

    Ok(())
}
