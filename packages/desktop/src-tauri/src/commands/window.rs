use tauri::{AppHandle, Manager};

/// Set window decorations (titlebar) visibility.
/// When `decorations` is false, the native titlebar is hidden.
/// This is useful for tiling window managers on Linux (e.g., Hyprland, i3, sway).
#[tauri::command]
pub fn set_window_decorations(app: AppHandle, decorations: bool) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    window
        .set_decorations(decorations)
        .map_err(|e| format!("Failed to set decorations: {e}"))
}

/// Close the main window (and exit the app).
#[tauri::command]
pub fn window_close(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    window
        .close()
        .map_err(|e| format!("Failed to close window: {e}"))
}

/// Minimize the main window.
#[tauri::command]
pub fn window_minimize(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    window
        .minimize()
        .map_err(|e| format!("Failed to minimize window: {e}"))
}

/// Toggle maximize/restore the main window.
#[tauri::command]
pub fn window_toggle_maximize(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    if window
        .is_maximized()
        .map_err(|e| format!("Failed to check maximized state: {e}"))?
    {
        window
            .unmaximize()
            .map_err(|e| format!("Failed to unmaximize window: {e}"))
    } else {
        window
            .maximize()
            .map_err(|e| format!("Failed to maximize window: {e}"))
    }
}

/// Check if the main window is maximized.
#[tauri::command]
pub fn window_is_maximized(app: AppHandle) -> Result<bool, String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    window
        .is_maximized()
        .map_err(|e| format!("Failed to check maximized state: {e}"))
}
