pub mod db;
pub mod layout;

use anyhow::Result;
use std::path::{Path, PathBuf};

pub use layout::StoreLayout;

/// The central store at ~/.loadout/
pub struct CentralStore {
    pub layout: StoreLayout,
}

impl CentralStore {
    pub fn new(root: PathBuf) -> Result<Self> {
        let layout = StoreLayout::new(root);
        layout.ensure_dirs()?;
        Ok(Self { layout })
    }

    /// Default location: ~/.loadout
    pub fn default_location() -> PathBuf {
        PathBuf::from(shellexpand::tilde("~/.loadout").into_owned())
    }

    pub fn open_default() -> Result<Self> {
        Self::new(Self::default_location())
    }

    /// Path to a skill in the central store
    pub fn skill_path(&self, slug: &str) -> PathBuf {
        self.layout.skills_dir.join(slug)
    }

    /// Drivers directory (user-defined drivers live here)
    pub fn drivers_dir(&self) -> &Path {
        &self.layout.drivers_dir
    }
}
