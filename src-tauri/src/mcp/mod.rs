pub mod formats;
pub mod redactor;
pub mod transport;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub use transport::McpTransport;

/// Presence of an MCP server in a specific tool
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PresenceState {
    /// Server exists in this tool's config
    Present,
    /// Server is absent from this tool's config
    Absent,
    /// Server config differs from canonical
    Drifted,
    /// Tool doesn't support this server's transport
    Incompatible,
}

/// A single MCP server entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub transport: McpTransport,
    /// Raw JSON config (preserves unknown fields)
    pub config: Value,
    pub enabled: bool,
    /// Map of tool_id → presence state
    pub presence: HashMap<String, PresenceState>,
}

impl McpServer {
    pub fn new(name: impl Into<String>, config: Value) -> Self {
        let transport = McpTransport::from_config(&config);
        Self {
            name: name.into(),
            transport,
            config,
            enabled: true,
            presence: HashMap::new(),
        }
    }
}
