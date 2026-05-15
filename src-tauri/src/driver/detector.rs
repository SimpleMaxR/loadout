use super::ToolDriver;

/// Check if a tool is installed by testing whether any of its detect paths exist
pub fn is_installed(driver: &ToolDriver) -> bool {
    driver.detect_paths.iter().any(|p| p.exists())
}

/// Return only the drivers that are currently installed
pub fn detect_installed<'a>(drivers: &'a [&'a ToolDriver]) -> Vec<&'a ToolDriver> {
    drivers.iter().copied().filter(|d| is_installed(d)).collect()
}
