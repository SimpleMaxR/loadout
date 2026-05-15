pub mod json_key;
pub mod plugin_manifest;

use anyhow::Result;
use std::collections::HashMap;

use super::McpServer;
use crate::driver::McpFormat;

/// Read all MCP servers from a tool using its configured format strategy
pub fn read_servers(format: &McpFormat) -> Result<HashMap<String, McpServer>> {
    match format {
        McpFormat::JsonKey { path, key } => json_key::read(path, key),
        McpFormat::PluginManifest { plugins_dir, .. } => plugin_manifest::read(plugins_dir),
        McpFormat::YamlKey { .. } => {
            // Placeholder: yaml-key format not yet implemented
            Ok(HashMap::new())
        }
        McpFormat::TomlSection { .. } => {
            // Placeholder: toml-section format not yet implemented
            Ok(HashMap::new())
        }
    }
}

/// Write (upsert) a single MCP server to a tool's config using its format strategy
pub fn write_server(format: &McpFormat, server: &McpServer) -> Result<()> {
    match format {
        McpFormat::JsonKey { path, key } => json_key::write(path, key, server),
        McpFormat::PluginManifest { plugins_dir, template } => {
            plugin_manifest::write(plugins_dir, template, server)
        }
        McpFormat::YamlKey { .. } => {
            anyhow::bail!("yaml-key write not yet implemented")
        }
        McpFormat::TomlSection { .. } => {
            anyhow::bail!("toml-section write not yet implemented")
        }
    }
}

/// Remove an MCP server from a tool's config
pub fn remove_server(format: &McpFormat, name: &str) -> Result<()> {
    match format {
        McpFormat::JsonKey { path, key } => json_key::remove(path, key, name),
        McpFormat::PluginManifest { plugins_dir, .. } => plugin_manifest::remove(plugins_dir, name),
        McpFormat::YamlKey { .. } => anyhow::bail!("yaml-key remove not yet implemented"),
        McpFormat::TomlSection { .. } => anyhow::bail!("toml-section remove not yet implemented"),
    }
}
