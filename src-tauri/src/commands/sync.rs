use tauri::State;

use crate::AppState;
use crate::sync::report::SyncReport;

/// Sync all skills from central store to all installed tools
#[tauri::command]
pub fn sync_all_skills(state: State<'_, AppState>) -> Vec<SyncReport> {
    let engine = state.engine.lock().unwrap();
    let skills = engine.read_all_skills();

    skills
        .iter()
        .map(|skill| engine.sync_skill_to_all(&skill.slug))
        .collect()
}

/// Sync all MCP servers to all installed tools
#[tauri::command]
pub fn sync_all_mcp(state: State<'_, AppState>) -> Vec<SyncReport> {
    let engine = state.engine.lock().unwrap();
    let servers: Vec<_> = engine.read_all_mcp().into_values().collect();

    servers
        .iter()
        .map(|server| engine.sync_mcp_to_all(server))
        .collect()
}
