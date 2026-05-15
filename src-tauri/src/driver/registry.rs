use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use super::{loader, ToolDriver};

/// Central registry of all loaded tool drivers
#[derive(Debug, Default)]
pub struct DriverRegistry {
    drivers: HashMap<String, ToolDriver>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load (or reload) all drivers from a directory
    pub fn load_from_dir(&mut self, dir: &Path) -> Result<()> {
        let drivers = loader::load_drivers_from_dir(dir)?;
        self.drivers.clear();
        for driver in drivers {
            self.drivers.insert(driver.id.clone(), driver);
        }
        Ok(())
    }

    /// Register a single driver (used for built-in bundled drivers)
    pub fn register(&mut self, driver: ToolDriver) {
        self.drivers.insert(driver.id.clone(), driver);
    }

    /// Get a driver by ID
    pub fn get(&self, id: &str) -> Option<&ToolDriver> {
        self.drivers.get(id)
    }

    /// All registered drivers
    pub fn all(&self) -> Vec<&ToolDriver> {
        let mut v: Vec<&ToolDriver> = self.drivers.values().collect();
        v.sort_by_key(|d| &d.id);
        v
    }

    /// IDs of all registered drivers
    pub fn ids(&self) -> Vec<String> {
        self.all().into_iter().map(|d| d.id.clone()).collect()
    }
}
