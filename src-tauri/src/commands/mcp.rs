use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::State;

use crate::mcp::{McpServer, PresenceState};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct McpServerDto {
    pub name: String,
    pub transport: String,
    pub config: Value,
    pub enabled: bool,
    pub presence: std::collections::HashMap<String, String>,
}

impl From<McpServer> for McpServerDto {
    fn from(s: McpServer) -> Self {
        Self {
            transport: s.transport.label().to_string(),
            presence: s
                .presence
                .into_iter()
                .map(|(k, v)| {
                    let label = match v {
                        PresenceState::Present => "present",
                        PresenceState::Absent => "absent",
                        PresenceState::Drifted => "drifted",
                        PresenceState::Incompatible => "incompatible",
                    };
                    (k, label.to_string())
                })
                .collect(),
            config: crate::mcp::redactor::redact(&s.config),
            name: s.name,
            enabled: s.enabled,
        }
    }
}

/// Read all MCP servers across all installed tools
#[tauri::command]
pub fn list_mcp_servers(state: State<'_, AppState>) -> Vec<McpServerDto> {
    let engine = state.engine.lock().unwrap();
    let servers = engine.read_all_mcp();
    let mut list: Vec<McpServerDto> = servers.into_values().map(McpServerDto::from).collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    list
}

#[derive(Debug, Deserialize)]
pub struct AddMcpServerRequest {
    pub name: String,
    pub config: Value,
}

/// Add an MCP server and sync to all tools
#[tauri::command]
pub fn add_mcp_server(
    state: State<'_, AppState>,
    req: AddMcpServerRequest,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    let server = McpServer::new(req.name, req.config);
    Ok(engine.sync_mcp_to_all(&server))
}

/// Sync a single MCP server to a single specific tool
#[tauri::command]
pub fn sync_mcp_to_tool(
    state: State<'_, AppState>,
    name: String,
    tool_id: String,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    engine.sync_mcp_to_tool(&name, &tool_id).map_err(|e| e.to_string())
}

/// Remove an MCP server from a single specific tool
#[tauri::command]
pub fn remove_mcp_from_tool(
    state: State<'_, AppState>,
    name: String,
    tool_id: String,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    engine.remove_mcp_from_tool(&name, &tool_id).map_err(|e| e.to_string())
}

/// Remove an MCP server from all tools
#[tauri::command]
pub fn remove_mcp_server(
    state: State<'_, AppState>,
    name: String,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    Ok(engine.remove_mcp_from_all(&name))
}
