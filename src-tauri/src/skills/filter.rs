use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

use crate::driver::FileFilter;

/// Build a globset from filter rules, relative to skill root
pub struct CompiledFilter {
    include: Option<GlobSet>,
    exclude: Option<GlobSet>,
}

impl CompiledFilter {
    pub fn new(filter: &FileFilter) -> Self {
        Self {
            include: build_globset(&filter.include),
            exclude: build_globset(&filter.exclude),
        }
    }

    /// Returns true if a relative path should be included in the copy
    pub fn should_include(&self, rel_path: &Path) -> bool {
        let path_str = rel_path.to_string_lossy();

        // Check exclude first
        if let Some(ref ex) = self.exclude {
            if ex.is_match(rel_path) || ex.is_match(path_str.as_ref()) {
                return false;
            }
        }

        // If no include filter, include everything
        if let Some(ref inc) = self.include {
            return inc.is_match(rel_path) || inc.is_match(path_str.as_ref());
        }

        true
    }
}

fn build_globset(patterns: &[String]) -> Option<GlobSet> {
    if patterns.is_empty() {
        return None;
    }

    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }

    builder.build().ok()
}
