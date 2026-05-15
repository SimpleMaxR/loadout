use tauri::{AppHandle, Manager, Theme};

/// Set the native window theme (dark / light).
/// This controls the macOS title bar appearance (traffic lights + background tint).
#[tauri::command]
pub fn set_window_theme(app: AppHandle, dark: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let theme = if dark { Theme::Dark } else { Theme::Light };
        let _ = window.set_theme(Some(theme));
    }
}
