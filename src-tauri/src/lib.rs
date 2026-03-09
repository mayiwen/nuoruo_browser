// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{AppHandle, Emitter, Manager};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            #[cfg(any(windows, target_os = "macos", target_os = "linux"))]
            minimize_window,
            #[cfg(any(windows, target_os = "macos", target_os = "linux"))]
            maximize_window,
            close_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "linux",
    not(any(target_os = "android", target_os = "ios")) // 显式排除移动端
))]
#[tauri::command]
fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "linux",
    not(any(target_os = "android", target_os = "ios")) // 显式排除移动端
))]
#[tauri::command]
fn maximize_window(window: tauri::Window) -> Result<(), String> {
    if let Ok(is_maximized) = window.is_maximized() {
        if is_maximized {
            window.unmaximize().map_err(|e| e.to_string())?;
        } else {
            window.maximize().map_err(|e: tauri::Error| e.to_string())?;
        }
    }
    Ok(())
}
#[tauri::command]
fn close_window(_app_handle: AppHandle, window: tauri::Window) -> Result<(), String> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        window.close().map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        _app_handle.exit(0);
        Ok(())
    }
}
