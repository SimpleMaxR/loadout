use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::{
    McpDriverConfig, McpFormat, RawDriverToml, SkillDriverConfig, SkillFormat,
    ToolDriver,
};

/// Expand `~` in a path string to the home directory
fn expand(p: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(p).into_owned())
}

/// Load a single driver from a TOML file on disk
pub fn load_driver(path: &Path) -> Result<ToolDriver> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("reading driver file {}", path.display()))?;

    let raw: RawDriverToml =
        toml::from_str(&content).with_context(|| format!("parsing driver TOML {}", path.display()))?;

    resolve_driver(raw)
}

/// Convert raw TOML into a resolved ToolDriver with expanded paths
pub fn resolve_driver(raw: RawDriverToml) -> Result<ToolDriver> {
    let detect_paths = raw.tool.detect.iter().map(|s| expand(s)).collect();

    let mcp = resolve_mcp(&raw.mcp)?;
    let skills = resolve_skills(&raw.skills)?;

    Ok(ToolDriver {
        id: raw.tool.id,
        display_name: raw.tool.display_name,
        icon: raw.tool.icon,
        detect_paths,
        mcp,
        skills,
    })
}

fn resolve_mcp(raw: &super::RawMcpSection) -> Result<McpDriverConfig> {
    let format = match raw.format.as_str() {
        "json-key" => {
            let path = expand(
                raw.path
                    .as_deref()
                    .context("json-key format requires `path`")?,
            );
            let key = raw
                .key
                .clone()
                .context("json-key format requires `key`")?;
            McpFormat::JsonKey { path, key }
        }
        "plugin-manifest" => {
            let plugins_dir = expand(
                raw.path
                    .as_deref()
                    .context("plugin-manifest format requires `path`")?,
            );
            let template = raw
                .manifest_template
                .clone()
                .context("plugin-manifest format requires `manifest_template`")?;
            McpFormat::PluginManifest { plugins_dir, template }
        }
        "yaml-key" => {
            let path = expand(
                raw.path
                    .as_deref()
                    .context("yaml-key format requires `path`")?,
            );
            let key = raw.key.clone().context("yaml-key format requires `key`")?;
            McpFormat::YamlKey { path, key }
        }
        "toml-section" => {
            let path = expand(
                raw.path
                    .as_deref()
                    .context("toml-section format requires `path`")?,
            );
            let section = raw
                .key
                .clone()
                .context("toml-section format requires `key` (used as section name)")?;
            McpFormat::TomlSection { path, section }
        }
        other => anyhow::bail!("unknown MCP format strategy: {}", other),
    };

    Ok(McpDriverConfig {
        format,
        transport_mapping: Default::default(),
    })
}

fn resolve_skills(raw: &super::RawSkillsSection) -> Result<SkillDriverConfig> {
    let path = expand(&raw.path);
    let skill_entry = raw
        .skill_entry
        .clone()
        .unwrap_or_else(|| "SKILL.md".to_string());

    let format = match raw.format.as_str() {
        "symlink" => SkillFormat::Symlink,
        "copy" => SkillFormat::Copy,
        "copy-with-manifest" => {
            let generator = raw
                .manifest_generator
                .clone()
                .context("copy-with-manifest format requires `manifest_generator`")?;
            SkillFormat::CopyWithManifest { generator }
        }
        other => anyhow::bail!("unknown skill format strategy: {}", other),
    };

    let file_filter = raw.file_filter.clone();

    Ok(SkillDriverConfig {
        format,
        path,
        skill_entry,
        file_filter,
    })
}

/// Load all drivers from a directory (*.toml files)
pub fn load_drivers_from_dir(dir: &Path) -> Result<Vec<ToolDriver>> {
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut drivers = Vec::new();
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("reading drivers dir {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            match load_driver(&path) {
                Ok(driver) => drivers.push(driver),
                Err(e) => {
                    eprintln!("Warning: failed to load driver {}: {}", path.display(), e);
                }
            }
        }
    }

    // Sort by id for deterministic ordering
    drivers.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(drivers)
}
