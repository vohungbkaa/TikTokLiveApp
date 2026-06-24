use tauri::State;
use crate::connectors::supervisor::{ConnectorSnapshot, ConnectorSupervisor, DebugLogEntry};
use crate::db::DbState;
use crate::sessions::repository::SessionRepository;

fn parse_tiktok_username(platform_session_id: &str) -> Option<String> {
    let mut username = platform_session_id
        .trim()
        .trim_start_matches("https://www.tiktok.com/@")
        .trim_start_matches("https://tiktok.com/@")
        .trim_start_matches('@');
    username = username.split('/').next().unwrap_or(username);
    username = username.split('?').next().unwrap_or(username);
    username = username.trim();

    if username.is_empty() {
        None
    } else {
        Some(username.to_string())
    }
}

#[tauri::command]
pub async fn get_connector_debug_logs(
    supervisor: State<'_, ConnectorSupervisor>,
) -> Result<Vec<DebugLogEntry>, String> {
    Ok(supervisor.get_debug_logs().await)
}

#[tauri::command]
pub async fn get_connector_status(
    supervisor: State<'_, ConnectorSupervisor>,
) -> Result<ConnectorSnapshot, String> {
    Ok(supervisor.get_status().await)
}

#[tauri::command]
pub async fn ensure_live_connector(
    session_id: String,
    force: Option<bool>,
    state: State<'_, DbState>,
    supervisor: State<'_, ConnectorSupervisor>,
) -> Result<ConnectorSnapshot, String> {
    let repo = SessionRepository::new(&state.pool);
    let session = repo
        .get(&session_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Session not found".to_string())?;

    if session.status != "running" {
        return Err("Session is not running".to_string());
    }

    let platform_id = session
        .platform_session_id
        .as_deref()
        .ok_or_else(|| "Session has no TikTok username".to_string())?;
    let username = parse_tiktok_username(platform_id)
        .ok_or_else(|| "Invalid TikTok username".to_string())?;

    tracing::info!(
        "[ensure_live_connector] session={} platform_id={} username=@{}",
        session_id,
        platform_id,
        username
    );

    let snapshot = supervisor
        .ensure_connector_for_session(&session_id, &username, force.unwrap_or(false))
        .await?;

    tracing::info!(
        "[ensure_live_connector] result status={} stage={:?} alive={} stdout_lines={}",
        snapshot.status,
        snapshot.stage,
        snapshot.process_alive,
        snapshot.stdout_lines
    );

    Ok(snapshot)
}
