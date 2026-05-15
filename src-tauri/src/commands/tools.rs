use serde::{Deserialize, Serialize};
use tauri::State;

use crate::driver::detector;
use crate::driver::McpFormat;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ToolInfo {
    pub id: String,
    pub display_name: String,
    pub icon: String,
    pub installed: bool,
    pub mcp_path: Option<String>,
    pub skills_path: String,
}

/// List all registered tools with installation status
#[tauri::command]
pub fn list_tools(state: State<'_, AppState>) -> Vec<ToolInfo> {
    let engine = state.engine.lock().unwrap();
    engine
        .drivers
        .iter()
        .map(|d| {
            let mcp_path = match &d.mcp.format {
                McpFormat::JsonKey { path, .. } | McpFormat::YamlKey { path, .. } => {
                    Some(path.to_string_lossy().to_string())
                }
                McpFormat::TomlSection { path, .. } => {
                    Some(path.to_string_lossy().to_string())
                }
                McpFormat::PluginManifest { plugins_dir, .. } => {
                    Some(plugins_dir.to_string_lossy().to_string())
                }
            };
            ToolInfo {
                id: d.id.clone(),
                display_name: d.display_name.clone(),
                icon: d.icon.clone(),
                installed: detector::is_installed(d),
                mcp_path,
                skills_path: d.skills.path.to_string_lossy().to_string(),
            }
        })
        .collect()
}

/// Request to update a tool's MCP and/or Skills paths
#[derive(Debug, Deserialize)]
pub struct UpdateToolPathsRequest {
    pub tool_id: String,
    /// New MCP config file path (None = don't change)
    pub mcp_path: Option<String>,
    /// New skills directory path (None = don't change)
    pub skills_path: Option<String>,
}

/// Update the MCP and/or Skills path for a tool.
/// Writes an override TOML to ~/.loadout/drivers/<tool-id>.toml and hot-reloads
/// the driver in the sync engine.
#[tauri::command]
pub fn update_tool_paths(
    state: State<'_, AppState>,
    req: UpdateToolPathsRequest,
) -> Result<(), String> {
    let mut engine = state.engine.lock().unwrap();

    // Find the existing driver so we can copy unchanged fields
    let driver = engine
        .drivers
        .iter()
        .find(|d| d.id == req.tool_id)
        .cloned()
        .ok_or_else(|| format!("driver not found: {}", req.tool_id))?;

    // Determine final paths (apply overrides or keep existing)
    let mcp_path_str = match &req.mcp_path {
        Some(p) => p.clone(),
        None => {
            // Reconstruct current path string from driver
            match &driver.mcp.format {
                McpFormat::JsonKey { path, .. }
                | McpFormat::YamlKey { path, .. }
                | McpFormat::TomlSection { path, .. } => {
                    path.to_string_lossy().to_string()
                }
                McpFormat::PluginManifest { plugins_dir, .. } => {
                    plugins_dir.to_string_lossy().to_string()
                }
            }
        }
    };

    let skills_path_str = match &req.skills_path {
        Some(p) => p.clone(),
        None => driver.skills.path.to_string_lossy().to_string(),
    };

    // Determine format strings for TOML (keep existing format type)
    let (mcp_format_str, mcp_key_str) = match &driver.mcp.format {
        McpFormat::JsonKey { key, .. } => ("json-key".to_string(), key.clone()),
        McpFormat::YamlKey { key, .. } => ("yaml-key".to_string(), key.clone()),
        McpFormat::TomlSection { section, .. } => ("toml-section".to_string(), section.clone()),
        McpFormat::PluginManifest { template, .. } => {
            ("plugin-manifest".to_string(), template.clone())
        }
    };

    let skill_format_str = match &driver.skills.format {
        crate::driver::SkillFormat::Symlink => "symlink",
        crate::driver::SkillFormat::Copy => "copy",
        crate::driver::SkillFormat::CopyWithManifest { .. } => "copy-with-manifest",
    }
    .to_string();

    let skill_entry = &driver.skills.skill_entry;

    // Build the override TOML content
    let detect_paths: Vec<String> = driver
        .detect_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    let detect_list = detect_paths
        .iter()
        .map(|p| format!("\"{}\"", p))
        .collect::<Vec<_>>()
        .join(", ");

    let mcp_section = match &driver.mcp.format {
        McpFormat::PluginManifest { template, .. } => format!(
            "[mcp]\nformat = \"plugin-manifest\"\npath = \"{}\"\nmanifest_template = \"{}\"",
            mcp_path_str, template
        ),
        _ => format!(
            "[mcp]\nformat = \"{}\"\npath = \"{}\"\nkey = \"{}\"",
            mcp_format_str, mcp_path_str, mcp_key_str
        ),
    };

    let skills_section = match &driver.skills.format {
        crate::driver::SkillFormat::CopyWithManifest { generator } => format!(
            "[skills]\nformat = \"copy-with-manifest\"\npath = \"{}\"\nskill_entry = \"{}\"\nmanifest_generator = \"{}\"",
            skills_path_str, skill_entry, generator
        ),
        _ => format!(
            "[skills]\nformat = \"{}\"\npath = \"{}\"\nskill_entry = \"{}\"",
            skill_format_str, skills_path_str, skill_entry
        ),
    };

    let toml_content = format!(
        "[tool]\nid = \"{}\"\ndisplay_name = \"{}\"\nicon = \"{}\"\ndetect = [{}]\n\n{}\n\n{}\n",
        driver.id, driver.display_name, driver.icon, detect_list, mcp_section, skills_section
    );

    // Write to ~/.loadout/drivers/<tool-id>.toml
    let override_path = engine
        .store
        .layout
        .drivers_dir
        .join(format!("{}.toml", req.tool_id));

    std::fs::write(&override_path, &toml_content)
        .map_err(|e| format!("failed to write override driver: {}", e))?;

    // Hot-reload: parse and replace in engine.drivers
    let new_driver = crate::driver::loader::load_driver(&override_path)
        .map_err(|e| format!("failed to reload driver: {}", e))?;

    if let Some(existing) = engine.drivers.iter_mut().find(|d| d.id == req.tool_id) {
        *existing = new_driver;
    }

    Ok(())
}
