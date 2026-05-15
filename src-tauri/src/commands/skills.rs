use serde::Serialize;
use tauri::State;

use crate::skills::{Skill, SkillPresence};
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct SkillDto {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub content_hash: String,
    pub presence: std::collections::HashMap<String, String>,
}

impl From<Skill> for SkillDto {
    fn from(s: Skill) -> Self {
        Self {
            presence: s
                .presence
                .into_iter()
                .map(|(k, v)| {
                    let label = match v {
                        SkillPresence::Symlinked => "symlinked",
                        SkillPresence::Copied => "copied",
                        SkillPresence::Absent => "absent",
                        SkillPresence::Drifted => "drifted",
                    };
                    (k, label.to_string())
                })
                .collect(),
            slug: s.slug,
            name: s.name,
            description: s.description,
            tags: s.tags,
            content_hash: s.content_hash,
        }
    }
}

/// List all skills in the central store with presence info
#[tauri::command]
pub fn list_skills(state: State<'_, AppState>) -> Vec<SkillDto> {
    let engine = state.engine.lock().unwrap();
    engine
        .read_all_skills()
        .into_iter()
        .map(SkillDto::from)
        .collect()
}

/// Sync a skill to all installed tools
#[tauri::command]
pub fn sync_skill(
    state: State<'_, AppState>,
    slug: String,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    Ok(engine.sync_skill_to_all(&slug))
}

/// Import a skill from a specific tool into the central store
#[tauri::command]
pub fn import_skill(
    state: State<'_, AppState>,
    tool_id: String,
    slug: String,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    engine.import_skill(&tool_id, &slug).map_err(|e| e.to_string())
}

/// Remove a skill from all tools (optionally from store too)
#[tauri::command]
pub fn remove_skill(
    state: State<'_, AppState>,
    slug: String,
    remove_from_store: bool,
) -> Result<crate::sync::report::SyncReport, String> {
    let engine = state.engine.lock().unwrap();
    Ok(engine.remove_skill_from_all(&slug, remove_from_store))
}
