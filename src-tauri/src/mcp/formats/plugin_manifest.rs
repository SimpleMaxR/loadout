use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use crate::mcp::McpServer;

/// Codex plugin manifest format:
/// Each MCP server is a directory under plugins_dir/ with a plugin.json
///
/// plugin.json structure:
/// {
///   "name": "server-name",
///   "type": "mcp",
///   "transport": "stdio|http|sse",
///   "command": "...",   // for stdio
///   "url": "...",       // for http/sse
///   "args": [],
///   "env": {}
/// }
pub fn read(plugins_dir: &Path) -> Result<HashMap<String, McpServer>> {
    if !plugins_dir.exists() {
        return Ok(HashMap::new());
    }

    let mut result = HashMap::new();

    for entry in std::fs::read_dir(plugins_dir)
        .with_context(|| format!("reading plugins dir {}", plugins_dir.display()))?
    {
        let entry = entry?;
        let entry_path = entry.path();

        if !entry_path.is_dir() {
            continue;
        }

        let manifest_path = entry_path.join("plugin.json");
        if !manifest_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("reading {}", manifest_path.display()))?;

        let manifest: Value = serde_json::from_str(&content)
            .with_context(|| format!("parsing {}", manifest_path.display()))?;

        // Only process MCP-type plugins
        if manifest.get("type").and_then(|v| v.as_str()) != Some("mcp") {
            continue;
        }

        let name = manifest
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
            })
            .to_string();

        let server = McpServer::new(name.clone(), manifest);
        result.insert(name, server);
    }

    Ok(result)
}

/// Write a Codex MCP plugin manifest
pub fn write(plugins_dir: &Path, _template: &str, server: &McpServer) -> Result<()> {
    let plugin_dir = plugins_dir.join(&server.name);
    std::fs::create_dir_all(&plugin_dir)
        .with_context(|| format!("creating plugin dir {}", plugin_dir.display()))?;

    // Build the plugin.json from the server config
    let mut manifest = server.config.clone();
    let manifest_obj = manifest
        .as_object_mut()
        .context("server config is not a JSON object")?;

    // Ensure required fields
    manifest_obj
        .entry("name")
        .or_insert_with(|| Value::String(server.name.clone()));
    manifest_obj
        .entry("type")
        .or_insert_with(|| Value::String("mcp".to_string()));

    let manifest_path = plugin_dir.join("plugin.json");
    let serialized = serde_json::to_string_pretty(&manifest)?;
    std::fs::write(&manifest_path, serialized)
        .with_context(|| format!("writing {}", manifest_path.display()))?;

    Ok(())
}

/// Remove a Codex MCP plugin (delete the plugin directory)
pub fn remove(plugins_dir: &Path, name: &str) -> Result<()> {
    let plugin_dir = plugins_dir.join(name);
    if plugin_dir.exists() {
        std::fs::remove_dir_all(&plugin_dir)
            .with_context(|| format!("removing plugin dir {}", plugin_dir.display()))?;
    }
    Ok(())
}
