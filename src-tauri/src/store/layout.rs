use anyhow::{Context, Result};
use std::path::PathBuf;

/// Directory layout of ~/.loadout/
#[derive(Debug, Clone)]
pub struct StoreLayout {
    pub root: PathBuf,
    pub skills_dir: PathBuf,     // ~/.loadout/store/skills/
    pub mcp_registry: PathBuf,   // ~/.loadout/store/mcp-registry.json
    pub db_dir: PathBuf,         // ~/.loadout/db/
    pub config_file: PathBuf,    // ~/.loadout/config.toml
    pub drivers_dir: PathBuf,    // ~/.loadout/drivers/ (user-defined)
    pub generators_dir: PathBuf, // ~/.loadout/generators/
}

impl StoreLayout {
    pub fn new(root: PathBuf) -> Self {
        Self {
            skills_dir: root.join("store").join("skills"),
            mcp_registry: root.join("store").join("mcp-registry.json"),
            db_dir: root.join("db"),
            config_file: root.join("config.toml"),
            drivers_dir: root.join("drivers"),
            generators_dir: root.join("generators"),
            root,
        }
    }

    /// Create all required directories
    pub fn ensure_dirs(&self) -> Result<()> {
        for dir in [
            &self.skills_dir,
            &self.db_dir,
            &self.drivers_dir,
            &self.generators_dir,
        ] {
            std::fs::create_dir_all(dir)
                .with_context(|| format!("creating directory {}", dir.display()))?;
        }
        Ok(())
    }
}
