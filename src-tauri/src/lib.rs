use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_notification::NotificationExt;

/// Show or hide the main window
fn toggle_window<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

/// Navigate the main window to a path and bring it to front
fn navigate_to<R: Runtime>(app: &tauri::AppHandle<R>, path: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let script = format!("window.location.href='{path}'");
        let _ = window.eval(&script);
    }
}

/// Build the system-tray menu
fn build_tray_menu<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let show      = MenuItem::with_id(app, "show",      "Show Urbrain",     true, None::<&str>)?;
    let sep1      = tauri::menu::PredefinedMenuItem::separator(app)?;
    let dashboard = MenuItem::with_id(app, "dashboard", "Dashboard",         true, None::<&str>)?;
    let ops       = MenuItem::with_id(app, "ops",       "AI Ops Center",     true, None::<&str>)?;
    let canvas    = MenuItem::with_id(app, "canvas",    "Workflow Canvas",   true, None::<&str>)?;
    let approvals = MenuItem::with_id(app, "approvals", "Approval Inbox",    true, None::<&str>)?;
    let sep2      = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit      = MenuItem::with_id(app, "quit",      "Quit Urbrain",      true, None::<&str>)?;

    Menu::with_items(app, &[
        &show, &sep1,
        &dashboard, &ops, &canvas, &approvals,
        &sep2, &quit,
    ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Focus the existing window if a second instance is launched
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();

            // Build system tray
            let menu = build_tray_menu(&handle)?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Urbrain AI Platform")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "dashboard" => navigate_to(app, "/"),
                    "ops"       => navigate_to(app, "/operations"),
                    "canvas"    => navigate_to(app, "/canvas"),
                    "approvals" => navigate_to(app, "/autopilot/approvals"),
                    "quit"      => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(move |tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Minimise to tray instead of closing
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            send_desktop_notification,
            get_platform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Urbrain desktop app");
}

/// Send a native desktop notification (callable from the web frontend via Tauri invoke)
#[tauri::command]
fn send_desktop_notification(
    app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    app.notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| e.to_string())
}

/// Return the current platform string (used by the frontend to detect desktop mode)
#[tauri::command]
fn get_platform() -> &'static str {
    std::env::consts::OS
}
