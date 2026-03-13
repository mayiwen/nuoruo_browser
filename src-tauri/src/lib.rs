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
            // navigate_to,
        ])
        // .setup(|app| {
        //     // 获取主窗口
        //     if let Some(main_window) = app.get_webview_window("main") {
        //         // 1. 初始加载 mayiwen.com
        //         main_window
        //             .navigate("https://mayiwen.com")
        //             .expect("Failed to navigate to initial URL");
        //         // 2. 显示窗口
        //         main_window.show().expect("Failed to show");
        //         main_window.set_focus().expect("Failed to focus");
        //         // 3. (可选) 设置初始标题
        //         main_window.set_title("Browser - mayiwen.com").ok();
        //     }
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 定义一个 Rust 命令，供前端调用
// #[tauri::command]
// fn navigate_to(window: tauri::WebviewWindow, url: String) -> Result<(), String> {
//     // 确保 URL 有协议头，否则 WebView 可能无法识别
//     let final_url = if url.starts_with("http://") || url.starts_with("https://") {
//         url
//     } else {
//         format!("https://{}", url)
//     };

//     // 使用 Rust API 直接控制窗口导航
//     window.navigate(&final_url).map_err(|e| e.to_string())?;

//     // 可选：更新窗口标题
//     window.set_title(&format!("Browser - {}", final_url)).ok();

//     Ok(())
// }

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
