use notify_debouncer_mini::DebouncedEvent;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeKind {
    /// A SKILL.md or skill directory changed
    Skill { slug: String },
    /// An MCP config file changed
    Mcp { tool_id: String },
    /// A driver TOML file changed
    Driver { driver_id: String },
    /// Unknown change
    Unknown,
}

/// Classify a file system event by looking at the path
pub fn classify(event: &DebouncedEvent, drivers_dir: &Path) -> ChangeKind {
    let path = &event.path;
    let path_str = path.to_string_lossy();

    // Driver change
    if path.starts_with(drivers_dir) {
        let id = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        return ChangeKind::Driver { driver_id: id };
    }

    // SKILL.md change → extract slug from parent directory name
    if path.file_name().and_then(|n| n.to_str()) == Some("SKILL.md") {
        if let Some(parent) = path.parent() {
            if let Some(slug) = parent.file_name().and_then(|n| n.to_str()) {
                return ChangeKind::Skill {
                    slug: slug.to_string(),
                };
            }
        }
    }

    // MCP config files
    let mcp_patterns: &[(&str, &str)] = &[
        ("settings.json", "claude-code"),
        ("mcp.json", "codebuddy"),
        (".mcp.json", "workbuddy"),
        ("config.toml", "codex"),
    ];

    for (filename, tool_id) in mcp_patterns {
        if path_str.ends_with(filename) {
            return ChangeKind::Mcp {
                tool_id: tool_id.to_string(),
            };
        }
    }

    ChangeKind::Unknown
}
