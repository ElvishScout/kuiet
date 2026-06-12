use tauri::{AppHandle, Manager, Url, WebviewBuilder, WebviewUrl};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebviewInit {
    pub url: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebviewConfig {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub visibility: Option<bool>,
    pub focused: Option<bool>,
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_url(raw: &Option<String>) -> Result<WebviewUrl, String> {
    match raw {
        Some(u) => Ok(WebviewUrl::External(
            Url::parse(u).map_err(|e| format!("bad url '{}': {}", u, e))?,
        )),
        None => Ok(WebviewUrl::External(Url::parse("about:blank").unwrap())),
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
async fn create_webview(app: AppHandle, wid: String, init: WebviewInit) -> Result<(), String> {
    if app.get_webview(&wid).is_some() {
        return Ok(());
    }

    app.get_window("main")
        .ok_or("main window not found")?
        .add_child(
            WebviewBuilder::new(&wid, parse_url(&init.url)?),
            tauri::Position::Logical(tauri::LogicalPosition::new(-1.0, -1.0)),
            tauri::Size::Logical(tauri::LogicalSize::new(1.0, 1.0)),
        )
        .map_err(|e| format!("create webview '{}': {}", wid, e))?;

    Ok(())
}

#[tauri::command]
async fn config_webview(app: AppHandle, wid: String, config: WebviewConfig) -> Result<(), String> {
    let wv = app
        .get_webview(&wid)
        .ok_or_else(|| format!("webview '{}' not found", wid))?;

    let scale = wv.window().scale_factor().unwrap_or(1.0);

    // ---- size ----
    match (config.width, config.height) {
        (Some(w), Some(h)) => wv
            .set_size(tauri::Size::Logical(tauri::LogicalSize::new(w, h)))
            .map_err(|e| format!("size: {}", e))?,
        (Some(w), None) => {
            let h = wv
                .size()
                .map_err(|e| e.to_string())?
                .to_logical(scale)
                .height;
            wv.set_size(tauri::Size::Logical(tauri::LogicalSize::new(w, h)))
                .map_err(|e| format!("size: {}", e))?;
        }
        (None, Some(h)) => {
            let w = wv
                .size()
                .map_err(|e| e.to_string())?
                .to_logical(scale)
                .width;
            wv.set_size(tauri::Size::Logical(tauri::LogicalSize::new(w, h)))
                .map_err(|e| format!("size: {}", e))?;
        }
        (None, None) => {}
    }

    // ---- position ----
    match (config.x, config.y) {
        (Some(x), Some(y)) => wv
            .set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
            .map_err(|e| format!("position: {}", e))?,
        (Some(x), None) => {
            let y = wv
                .position()
                .map_err(|e| e.to_string())?
                .to_logical(scale)
                .y;
            wv.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
                .map_err(|e| format!("position: {}", e))?;
        }
        (None, Some(y)) => {
            let x = wv
                .position()
                .map_err(|e| e.to_string())?
                .to_logical(scale)
                .x;
            wv.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))
                .map_err(|e| format!("position: {}", e))?;
        }
        (None, None) => {}
    }

    // ---- visibility ----
    match config.visibility {
        Some(true) => wv.show().map_err(|e| format!("show: {}", e))?,
        Some(false) => wv.hide().map_err(|e| format!("hide: {}", e))?,
        None => {}
    }

    // ---- focus ----
    match config.focused {
        Some(true) => wv.set_focus().map_err(|e| format!("focus: {}", e))?,
        _ => {}
    }

    // ---- url (skip if already there) ----
    if let Some(raw) = &config.url {
        let target = Url::parse(raw).map_err(|e| format!("bad url '{}': {}", raw, e))?;
        if wv.url().map_err(|e| e.to_string())? != target {
            wv.navigate(target)
                .map_err(|e| format!("navigate: {}", e))?;
        }
    }

    Ok(())
}

#[tauri::command]
async fn destroy_webview(app: AppHandle, wid: String) -> Result<(), String> {
    match app.get_webview(&wid) {
        Some(wv) => wv.close().map_err(|e| format!("destroy '{}': {}", wid, e)),
        None => Ok(()),
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_webview,
            config_webview,
            destroy_webview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
