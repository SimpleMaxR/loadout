use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

use crate::driver::FileFilter;
use crate::skills::filter::CompiledFilter;

/// Copy a skill directory to the target, respecting optional file filter
pub fn sync(source: &Path, target: &Path, filter: Option<&FileFilter>) -> Result<()> {
    let compiled_filter = filter.map(CompiledFilter::new);

    // Remove existing target if any
    if target.exists() {
        std::fs::remove_dir_all(target)
            .with_context(|| format!("removing {}", target.display()))?;
    }

    std::fs::create_dir_all(target)
        .with_context(|| format!("creating {}", target.display()))?;

    for entry in WalkDir::new(source).min_depth(1) {
        let entry = entry?;
        let rel_path = entry.path().strip_prefix(source)?;

        // Apply filter
        if let Some(ref f) = compiled_filter {
            if !f.should_include(rel_path) {
                continue;
            }
        }

        let dest = target.join(rel_path);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&dest)
                .with_context(|| format!("creating dir {}", dest.display()))?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &dest)
                .with_context(|| format!("copying {} to {}", entry.path().display(), dest.display()))?;
        }
    }

    Ok(())
}
