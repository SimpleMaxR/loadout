use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP transport type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum McpTransport {
    /// Local process via stdin/stdout
    Stdio,
    /// HTTP (legacy)
    Http,
    /// Streamable HTTP (MCP 2025-03)
    StreamableHttp,
    /// Server-Sent Events
    Sse,
    /// Unknown/unrecognized transport
    Unknown,
}

impl McpTransport {
    /// Infer transport type from raw MCP config JSON
    pub fn from_config(config: &Value) -> Self {
        // Check explicit `type` field first
        if let Some(t) = config.get("type").and_then(|v| v.as_str()) {
            return match t {
                "stdio" => McpTransport::Stdio,
                "http" => McpTransport::Http,
                "streamable-http" | "streamableHttp" => McpTransport::StreamableHttp,
                "sse" => McpTransport::Sse,
                _ => McpTransport::Unknown,
            };
        }

        // Infer from presence of url vs command
        if config.get("command").is_some() {
            return McpTransport::Stdio;
        }
        if let Some(url) = config.get("url").and_then(|v| v.as_str()) {
            if url.contains("/sse") {
                return McpTransport::Sse;
            }
            return McpTransport::StreamableHttp;
        }

        McpTransport::Unknown
    }

    pub fn label(&self) -> &'static str {
        match self {
            McpTransport::Stdio => "stdio",
            McpTransport::Http => "http",
            McpTransport::StreamableHttp => "http",
            McpTransport::Sse => "sse",
            McpTransport::Unknown => "unknown",
        }
    }
}
