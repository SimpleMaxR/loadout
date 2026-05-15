use anyhow::{Context, Result};
use std::path::Path;

/// Create or update a symlink: target -> source
/// If the target already exists as a symlink pointing to the right place, skip.
/// If it's a directory (old copy), remove and replace with symlink.
pub fn sync(source: &Path, target: &Path) -> Result<()> {
    // Already the right symlink?
    if target.is_symlink() {
        let current = std::fs::read_link(target)?;
        if current == source {
            return Ok(()); // Already correct
        }
        // Points elsewhere — remove and re-create
        std::fs::remove_file(target).or_else(|_| std::fs::remove_dir_all(target))?;
    } else if target.exists() {
        // It's a real directory (e.g. from a previous copy) — remove it
        std::fs::remove_dir_all(target)
            .with_context(|| format!("removing old skill dir {}", target.display()))?;
    }

    // Ensure parent directory exists
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating parent dir {}", parent.display()))?;
    }

    // Create the symlink
    #[cfg(unix)]
    std::os::unix::fs::symlink(source, target)
        .with_context(|| format!("creating symlink {} -> {}", target.display(), source.display()))?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(source, target)
        .with_context(|| format!("creating symlink {} -> {}", target.display(), source.display()))?;

    Ok(())
}
