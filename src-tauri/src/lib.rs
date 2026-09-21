use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_notification::NotificationExt;

/// The two live front-ends the desktop app hosts. They're loaded TOP-LEVEL at their
/// own *.urbrain.ai origins (not bundled/iframed) so their SameSite cookie auth works
/// exactly like the browser — same site as api.urbrain.ai.
const CONSUMER_URL: &str = "https://client.urbrain.ai";
const BUSINESS_URL: &str = "https://business.urbrain.ai";

/// Injected into every loaded page: a floating pill to switch between the consumer
/// app and the business dashboard. Highlights the active side (by hostname) and does
/// a top-level navigation on click (cookies persist across *.urbrain.ai — no re-login).
const SWITCHER_JS: &str = r#"(function(){try{
  if(document.getElementById('__urbrain_switch'))return;
  var onBiz=location.hostname.indexOf('business')===0;
  var bar=document.createElement('div');
  bar.id='__urbrain_switch';
  bar.style.cssText='position:fixed;bottom:16px;right:16px;z-index:2147483647;display:flex;gap:2px;background:rgba(17,17,24,.92);border:1px solid rgba(255,255,255,.14);border-radius:9999px;padding:3px;box-shadow:0 6px 24px rgba(0,0,0,.35);font-family:system-ui,-apple-system,sans-serif';
  function mk(label,active,url){var b=document.createElement('button');b.textContent=label;b.style.cssText='appearance:none;border:0;outline:0;border-radius:9999px;padding:6px 16px;font-size:12px;font-weight:600;cursor:pointer;color:'+(active?'#fff':'#9aa1ad')+';background:'+(active?'linear-gradient(135deg,#f97316,#a855f7)':'transparent');b.onclick=function(){if(!active)location.href=url;};return b;}
  bar.appendChild(mk('Consumer',!onBiz,'https://client.urbrain.ai/'));
  bar.appendChild(mk('Business',onBiz,'https://business.urbrain.ai/'));
  (document.body||document.documentElement).appendChild(bar);
}catch(e){}})();"#;

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

/// Navigate the main window (top-level) to a URL and bring it to front.
fn navigate_to<R: Runtime>(app: &tauri::AppHandle<R>, url: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let script = format!("window.location.href='{url}'");
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
    // Switch the window between the two live front-ends: the consumer app
    // (client.urbrain.ai) and the business dashboard (business.urbrain.ai).
    let consumer  = MenuItem::with_id(app, "consumer", "Consumer App",       true, None::<&str>)?;
    let business  = MenuItem::with_id(app, "business", "Business Dashboard",  true, None::<&str>)?;
    let sep3      = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit      = MenuItem::with_id(app, "quit",      "Quit Urbrain",      true, None::<&str>)?;

    Menu::with_items(app, &[
        &show, &sep1,
        &dashboard, &ops, &canvas, &approvals,
        &sep2, &consumer, &business,
        &sep3, &quit,
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
        // Inject the floating Consumer/Business switcher into every page load.
        .on_page_load(|webview, payload| {
            if matches!(payload.event(), tauri::webview::PageLoadEvent::Finished) {
                let _ = webview.eval(SWITCHER_JS);
            }
        })
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
                    "dashboard" => navigate_to(app, CONSUMER_URL),
                    "ops"       => navigate_to(app, "https://client.urbrain.ai/operations"),
                    "canvas"    => navigate_to(app, "https://client.urbrain.ai/canvas"),
                    "approvals" => navigate_to(app, "https://client.urbrain.ai/autopilot/approvals"),
                    "consumer"  => navigate_to(app, CONSUMER_URL),
                    "business"  => navigate_to(app, BUSINESS_URL),
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
