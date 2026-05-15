use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use crate::mcp::McpServer;

/// Read MCP servers from a JSON file with a top-level key
/// e.g. ~/.claude/settings.json → mcpServers
pub fn read(path: &Path, key: &str) -> Result<HashMap<String, McpServer>> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading MCP config {}", path.display()))?;

    let json: Value = serde_json::from_str(&content)
        .with_context(|| format!("parsing JSON in {}", path.display()))?;

    let servers_obj = match json.get(key) {
        Some(Value::Object(obj)) => obj.clone(),
        Some(_) => anyhow::bail!("key '{}' in {} is not a JSON object", key, path.display()),
        None => return Ok(HashMap::new()),
    };

    let mut result = HashMap::new();
    for (name, config) in servers_obj {
        let server = McpServer::new(name.clone(), config);
        result.insert(name, server);
    }

    Ok(result)
}

/// Write (upsert) a single MCP server into a JSON file's key
pub fn write(path: &Path, key: &str, server: &McpServer) -> Result<()> {
    // Read existing file or start with empty object
    let mut root: Value = if path.exists() {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("parsing JSON in {}", path.display()))?
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Ensure root and the key exist as objects
    if !root.is_object() {
        root = Value::Object(serde_json::Map::new());
    }

    let root_obj = root.as_object_mut().unwrap();
    if !root_obj.contains_key(key) {
        root_obj.insert(key.to_string(), Value::Object(serde_json::Map::new()));
    }

    let servers = root_obj
        .get_mut(key)
        .and_then(|v| v.as_object_mut())
        .with_context(|| format!("key '{}' is not an object in {}", key, path.display()))?;

    servers.insert(server.name.clone(), server.config.clone());

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating directory {}", parent.display()))?;
    }

    let serialized = serde_json::to_string_pretty(&root)?;
    std::fs::write(path, serialized)
        .with_context(|| format!("writing {}", path.display()))?;

    Ok(())
}

/// Remove an MCP server from a JSON file's key
pub fn remove(path: &Path, key: &str, name: &str) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(path)?;
    let mut root: Value = serde_json::from_str(&content)?;

    if let Some(servers) = root.get_mut(key).and_then(|v| v.as_object_mut()) {
        servers.remove(name);
    }

    let serialized = serde_json::to_string_pretty(&root)?;
    std::fs::write(path, serialized)?;

    Ok(())
}
