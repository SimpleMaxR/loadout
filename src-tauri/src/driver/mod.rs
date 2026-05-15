pub mod detector;
pub mod loader;
pub mod registry;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// MCP format strategy declared in driver TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", rename_all = "kebab-case")]
pub enum McpFormat {
    /// Standard JSON file with a top-level key (Claude Code, Codebuddy, Workbuddy)
    JsonKey {
        path: PathBuf,
        key: String,
    },
    /// Codex: each MCP server is a plugin folder with plugin.json
    PluginManifest {
        plugins_dir: PathBuf,
        template: String,
    },
    /// Future: YAML file with a key
    YamlKey {
        path: PathBuf,
        key: String,
    },
    /// Future: TOML section
    TomlSection {
        path: PathBuf,
        section: String,
    },
}

/// Skill sync strategy declared in driver TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", rename_all = "kebab-case")]
pub enum SkillFormat {
    /// Symlink to central store (instant, zero-copy)
    Symlink,
    /// Full directory copy
    Copy,
    /// Copy + generate extra manifest files via Lua generator
    CopyWithManifest {
        generator: String,
    },
}

/// File filter rules for skill sync
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileFilter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

/// Raw TOML driver definition (as loaded from file)
#[derive(Debug, Deserialize)]
pub struct RawDriverToml {
    pub tool: RawToolSection,
    pub mcp: RawMcpSection,
    pub skills: RawSkillsSection,
}

#[derive(Debug, Deserialize)]
pub struct RawToolSection {
    pub id: String,
    pub display_name: String,
    pub icon: String,
    pub detect: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawMcpSection {
    pub format: String,
    pub path: Option<String>,
    pub key: Option<String>,
    pub manifest_template: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSkillsSection {
    pub format: String,
    pub path: String,
    pub skill_entry: Option<String>,
    pub manifest_generator: Option<String>,
    pub file_filter: Option<FileFilter>,
}

/// Resolved driver config (with expanded paths)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpDriverConfig {
    pub format: McpFormat,
    pub transport_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDriverConfig {
    pub format: SkillFormat,
    pub path: PathBuf,
    pub skill_entry: String,
    pub file_filter: Option<FileFilter>,
}

/// A fully resolved tool driver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDriver {
    pub id: String,
    pub display_name: String,
    pub icon: String,
    /// Detection paths (any present = tool is installed)
    pub detect_paths: Vec<PathBuf>,
    pub mcp: McpDriverConfig,
    pub skills: SkillDriverConfig,
}
