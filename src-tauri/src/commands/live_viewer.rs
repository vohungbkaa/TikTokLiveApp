use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const VIEWER_LABEL: &str = "tiktok-live-viewer";

fn clean_username(username: &str) -> Result<String, String> {
    let clean = username
        .trim()
        .trim_start_matches('@')
        .split('/')
        .next()
        .unwrap_or(username.trim())
        .trim()
        .to_string();

    if clean.is_empty() {
        Err("Username is empty".into())
    } else {
        Ok(clean)
    }
}

fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn live_url(username: &str) -> Result<String, String> {
    let clean = clean_username(username)?;
    Ok(format!("https://www.tiktok.com/@{clean}/live"))
}

fn viewer_target_url(username: &str, login_first: bool) -> Result<(String, String), String> {
    let live = live_url(username)?;
    if login_first {
        let redirect = url_encode(&live);
        Ok((
            format!("https://www.tiktok.com/login?redirect_url={redirect}"),
            live,
        ))
    } else {
        Ok((live.clone(), live))
    }
}

async fn navigate_or_open(app: &AppHandle, start_url: &str, title: &str) -> Result<(), String> {
    let parsed = start_url
        .parse()
        .map_err(|e| format!("Invalid URL: {e}"))?;

    if let Some(win) = app.get_webview_window(VIEWER_LABEL) {
        win.navigate(parsed)
            .map_err(|e| format!("Failed to navigate live viewer: {e}"))?;
        let _ = win.set_title(title);
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }

    WebviewWindowBuilder::new(app, VIEWER_LABEL, WebviewUrl::External(parsed))
        .title(title)
        .inner_size(430.0, 820.0)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_live_viewer(
    app: AppHandle,
    username: String,
    login_first: Option<bool>,
) -> Result<(), String> {
    let clean = clean_username(&username)?;
    let login_first = login_first.unwrap_or(false);
    let (start_url, live_target) = viewer_target_url(&username, login_first)?;
    let title = format!("@{clean} · TikTok Live");

    tracing::info!(
        "[live_viewer] open login_first={} start={} target={}",
        login_first,
        start_url,
        live_target
    );

    navigate_or_open(&app, &start_url, &title).await
}

#[tauri::command]
pub async fn close_live_viewer(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window(VIEWER_LABEL) {
        win.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
