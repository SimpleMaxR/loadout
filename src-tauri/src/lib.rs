pub mod commands;
pub mod driver;
pub mod mcp;
pub mod skills;
pub mod store;
pub mod sync;
pub mod tray;
pub mod watcher;

use driver::{loader, registry::DriverRegistry};
use std::sync::Mutex;
use store::CentralStore;
use sync::SyncEngine;
use tauri::Manager;

/// Global app state shared between Tauri commands
pub struct AppState {
    pub engine: Mutex<SyncEngine>,
}

impl AppState {
    pub fn new(engine: SyncEngine) -> Self {
        Self {
            engine: Mutex::new(engine),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // ── 1. Open (or create) central store ─────────────────────────
            let store = CentralStore::open_default()
                .expect("failed to initialize ~/.loadout store");

            // ── 2. Load built-in drivers (bundled in the app package) ──────
            let mut registry = DriverRegistry::new();

            // Try loading from app resource directory first
            let resource_drivers = app
                .path()
                .resource_dir()
                .map(|p| p.join("drivers"))
                .unwrap_or_default();

            // Fall back to the project-relative drivers/ directory for dev
            let dev_drivers = std::path::PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
            )
            .parent()
            .unwrap_or(&std::path::PathBuf::from("."))
            .join("drivers");

            for dir in [&resource_drivers, &dev_drivers] {
                if let Ok(drivers) = loader::load_drivers_from_dir(dir) {
                    for d in drivers {
                        registry.register(d);
                    }
                }
            }

            // ── 3. Also load user-defined drivers from ~/.loadout/drivers/ ─
            if let Ok(user_drivers) =
                loader::load_drivers_from_dir(store.drivers_dir())
            {
                for d in user_drivers {
                    registry.register(d);
                }
            }

            // ── 4. Create sync engine ──────────────────────────────────────
            let all_drivers: Vec<_> = registry.all().into_iter().cloned().collect();
            let engine = SyncEngine::new(store, all_drivers);

            app.manage(AppState::new(engine));

            // ── 5. Set up menu bar tray ────────────────────────────────────
            tray::menu::setup_tray(&app.handle())?;

            // ── 6. Default to dark theme for native title bar ──────────────
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_theme(Some(tauri::Theme::Dark));
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Tools
            commands::tools::list_tools,
            commands::tools::update_tool_paths,
            // MCP
            commands::mcp::list_mcp_servers,
            commands::mcp::add_mcp_server,
            commands::mcp::sync_mcp_to_tool,
            commands::mcp::remove_mcp_from_tool,
            commands::mcp::remove_mcp_server,
            // Skills
            commands::skills::list_skills,
            commands::skills::sync_skill,
            commands::skills::import_skill,
            commands::skills::remove_skill,
            // Sync
            commands::sync::sync_all_skills,
            commands::sync::sync_all_mcp,
            // Window
            commands::window::set_window_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
