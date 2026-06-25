use std::collections::VecDeque;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::Mutex;
use sqlx::SqlitePool;
use crate::connectors::node_sidecar::SidecarProcess;
use crate::events::ingestion::EventIngestionService;
use crate::events::models::IncomingEvent;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugLogEntry {
    pub ts: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

#[derive(Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorSnapshot {
    pub status: String,
    pub stage: Option<String>,
    pub session_id: Option<String>,
    pub username: Option<String>,
    pub process_alive: bool,
    pub stdout_lines: u64,
    pub event_count: u64,
    pub last_message: Option<String>,
    pub stream_hls_url: Option<String>,
    pub stream_flv_url: Option<String>,
    pub live_page_url: Option<String>,
    pub stream_source: Option<String>,
    pub stream_error: Option<String>,
}

pub struct ConnectorSupervisor {
    app_handle: AppHandle,
    pool: SqlitePool,
    process: Arc<Mutex<Option<SidecarProcess>>>,
    snapshot: Arc<Mutex<ConnectorSnapshot>>,
    debug_logs: Arc<Mutex<VecDeque<DebugLogEntry>>>,
}

impl ConnectorSupervisor {
    pub fn new(app_handle: AppHandle, pool: SqlitePool) -> Self {
        Self {
            app_handle,
            pool,
            process: Arc::new(Mutex::new(None)),
            snapshot: Arc::new(Mutex::new(ConnectorSnapshot::default())),
            debug_logs: Arc::new(Mutex::new(VecDeque::with_capacity(64))),
        }
    }

    pub async fn push_log(&self, level: &str, message: impl Into<String>) {
        let msg = message.into();
        tracing::info!("[connector] {}", msg);

        let entry = DebugLogEntry {
            ts: Utc::now(),
            level: level.to_string(),
            message: msg.clone(),
        };

        {
            let mut logs = self.debug_logs.lock().await;
            if logs.len() >= 100 {
                logs.pop_front();
            }
            logs.push_back(entry.clone());
        }

        let _ = self.app_handle.emit("connector:debug", &entry);
    }

    pub async fn get_debug_logs(&self) -> Vec<DebugLogEntry> {
        self.debug_logs.lock().await.iter().cloned().collect()
    }

    async fn emit_snapshot(&self, snapshot: &ConnectorSnapshot) {
        let _ = self.app_handle.emit("connector:snapshot", snapshot);
    }

    pub async fn get_status(&self) -> ConnectorSnapshot {
        let snapshot = self.snapshot.lock().await;
        let process_alive = self.process.lock().await.is_some();
        let mut out = snapshot.clone();
        out.process_alive = process_alive;
        out
    }

    async fn set_snapshot<F>(&self, update: F)
    where
        F: FnOnce(&mut ConnectorSnapshot),
    {
        let mut snapshot = self.snapshot.lock().await;
        update(&mut snapshot);
    }

    fn health_stage_to_status(stage: &str) -> &'static str {
        match stage {
            "connected" => "connected",
            "connecting" | "reconnecting" | "room_resolve" => "connecting",
            "disconnected" | "stream_ended" => "disconnected",
            _ => "connecting",
        }
    }

    pub async fn start_connector(&self, username: &str, session_id: &str) -> Result<(), String> {
        self.push_log(
            "info",
            format!("start_connector session={} user=@{}", session_id, username),
        )
        .await;

        {
            let mut proc_guard = self.process.lock().await;
            if let Some(mut old_proc) = proc_guard.take() {
                self.push_log("info", "Stopping previous sidecar process").await;
                let _ = old_proc.stop().await;
            }
        }

        self.set_snapshot(|s| {
            s.status = "connecting".to_string();
            s.stage = Some("starting".to_string());
            s.session_id = Some(session_id.to_string());
            s.username = Some(username.to_string());
            s.stdout_lines = 0;
            s.event_count = 0;
            s.last_message = Some("spawn requested".to_string());
        })
        .await;
        self.emit_snapshot(&self.get_status().await).await;

        let spawn_result = SidecarProcess::start(&self.app_handle, username, session_id).await;
        let (proc, mut rx) = match spawn_result {
            Ok(v) => v,
            Err(e) => {
                self.set_snapshot(|s| {
                    s.status = "error".to_string();
                    s.stage = Some("spawn_failed".to_string());
                    s.last_message = Some(e.clone());
                    s.process_alive = false;
                })
                .await;
                self.push_log("error", format!("Sidecar spawn failed: {e}"))
                    .await;
                return Err(e);
            }
        };
        {
            let mut proc_guard = self.process.lock().await;
            *proc_guard = Some(proc);
        }
        self.push_log("info", "Sidecar process spawned").await;

        let app_handle = self.app_handle.clone();
        let session_id_cloned = session_id.to_string();
        let ingestion_svc = EventIngestionService::new(self.pool.clone(), self.app_handle.clone());
        let snapshot = self.snapshot.clone();
        let process = self.process.clone();
        let debug_logs = self.debug_logs.clone();

        async fn push_log_inner(
            app_handle: &AppHandle,
            debug_logs: &Arc<Mutex<VecDeque<DebugLogEntry>>>,
            level: &str,
            message: impl Into<String>,
        ) {
            let msg = message.into();
            tracing::info!("[connector] {}", msg);
            let entry = DebugLogEntry {
                ts: Utc::now(),
                level: level.to_string(),
                message: msg,
            };
            {
                let mut logs = debug_logs.lock().await;
                if logs.len() >= 100 {
                    logs.pop_front();
                }
                logs.push_back(entry.clone());
            }
            let _ = app_handle.emit("connector:debug", &entry);
        }

        tauri::async_runtime::spawn(async move {
            push_log_inner(
                &app_handle,
                &debug_logs,
                "info",
                "Sidecar stdout reader started",
            )
            .await;

            while let Some(msg) = rx.recv().await {
                let msg_type = msg.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
                {
                    let mut snap = snapshot.lock().await;
                    snap.stdout_lines += 1;
                    snap.last_message = Some(format!("type={msg_type}"));
                }

                match msg_type {
                    "event" => {
                        if let Some(data) = msg.get("data") {
                            match serde_json::from_value::<IncomingEvent>(data.clone()) {
                                Ok(incoming) => match ingestion_svc
                                    .ingest(&session_id_cloned, incoming)
                                    .await
                                {
                                    Ok(event) => {
                                        let snapshot_payload = {
                                            let mut snap = snapshot.lock().await;
                                            snap.status = "connected".to_string();
                                            snap.stage = Some("connected".to_string());
                                            snap.event_count += 1;
                                            snap.clone()
                                        };
                                        push_log_inner(
                                            &app_handle,
                                            &debug_logs,
                                            "info",
                                            format!(
                                                "Ingested comment #{} from {}",
                                                snapshot_payload.event_count,
                                                event.display_name.as_deref().unwrap_or("unknown")
                                            ),
                                        )
                                        .await;
                                        let _ = app_handle.emit("connector:snapshot", &snapshot_payload);
                                    }
                                    Err(e) if e == "Duplicate event" => {}
                                    Err(e) => {
                                        push_log_inner(
                                            &app_handle,
                                            &debug_logs,
                                            "warn",
                                            format!("Ingest failed: {e}"),
                                        )
                                        .await;
                                    }
                                }
                                Err(err) => {
                                    push_log_inner(
                                        &app_handle,
                                        &debug_logs,
                                        "error",
                                        format!("Parse incoming event failed: {err}"),
                                    )
                                    .await;
                                }
                            }
                        }
                    }
                    "health" => {
                        if let Some(stage) = msg.get("stage").and_then(|s| s.as_str()) {
                            let status = Self::health_stage_to_status(stage).to_string();
                            let snapshot_payload = {
                                let mut snap = snapshot.lock().await;
                                snap.status = status.clone();
                                snap.stage = Some(stage.to_string());
                                snap.clone()
                            };
                            push_log_inner(
                                &app_handle,
                                &debug_logs,
                                "info",
                                format!("Health stage={stage} status={status}"),
                            )
                            .await;
                            let _ = app_handle.emit("connector:health", &msg);
                            let _ = app_handle.emit("connector:snapshot", &snapshot_payload);
                        }
                    }
                    "stream" => {
                        let snapshot_payload = {
                            let mut snap = snapshot.lock().await;
                            snap.stream_hls_url = msg
                                .get("hls_url")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            snap.stream_flv_url = msg
                                .get("flv_url")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            snap.live_page_url = msg
                                .get("live_page_url")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            snap.stream_source = msg
                                .get("source")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            snap.stream_error = msg
                                .get("error")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            snap.clone()
                        };
                        push_log_inner(
                            &app_handle,
                            &debug_logs,
                            "info",
                            format!(
                                "Stream resolved source={} hls={}",
                                snapshot_payload.stream_source.as_deref().unwrap_or("?"),
                                snapshot_payload.stream_hls_url.is_some()
                            ),
                        )
                        .await;
                        let _ = app_handle.emit("connector:stream", &msg);
                        let _ = app_handle.emit("connector:snapshot", &snapshot_payload);
                    }
                    "error" => {
                        {
                            let mut snap = snapshot.lock().await;
                            snap.status = "error".to_string();
                        }
                        push_log_inner(
                            &app_handle,
                            &debug_logs,
                            "error",
                            format!("Sidecar error event: {msg}"),
                        )
                        .await;
                        let _ = app_handle.emit("connector:error", &msg);
                    }
                    other => {
                        push_log_inner(
                            &app_handle,
                            &debug_logs,
                            "warn",
                            format!("Unknown sidecar message type: {other}"),
                        )
                        .await;
                    }
                }
            }

            push_log_inner(
                &app_handle,
                &debug_logs,
                "warn",
                "Sidecar stdout loop ended",
            )
            .await;
            {
                let mut proc_guard = process.lock().await;
                *proc_guard = None;
            }
            {
                let mut snap = snapshot.lock().await;
                snap.status = "down".to_string();
                snap.stage = Some("terminated".to_string());
                snap.process_alive = false;
            }
            let _ = app_handle.emit("connector:status", "down");
        });

        Ok(())
    }

    pub async fn ensure_connector_for_session(
        &self,
        session_id: &str,
        username: &str,
        force: bool,
    ) -> Result<ConnectorSnapshot, String> {
        let status = self.get_status().await;
        self.push_log(
            "info",
            format!(
                "ensure_connector session={} user=@{} current_status={} alive={} stdout_lines={} force={}",
                session_id, username, status.status, status.process_alive, status.stdout_lines, force
            ),
        )
        .await;

        let same_target = status.session_id.as_deref() == Some(session_id)
            && status.username.as_deref() == Some(username);

        let needs_restart = force
            || !same_target
            || !status.process_alive
            || status.status == "down"
            || status.status == "error"
            || status.status == "disconnected"
            || status.status.is_empty();

        if needs_restart {
            self.push_log(
                "info",
                format!(
                    "Starting connector (was status={} alive={} stdout_lines={})",
                    status.status, status.process_alive, status.stdout_lines
                ),
            )
            .await;
            self.start_connector(username, session_id).await?;
        } else {
            self.push_log(
                "info",
                format!(
                    "Connector already active (status={} stage={:?}), skip restart",
                    status.status, status.stage
                ),
            )
            .await;
        }

        Ok(self.get_status().await)
    }

    pub async fn stop_connector(&self) -> Result<(), String> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut proc) = proc_guard.take() {
            proc.stop().await?;
        }
        self.set_snapshot(|s| {
            s.status = "down".to_string();
            s.stage = Some("stopped".to_string());
            s.process_alive = false;
        })
        .await;
        self.push_log("info", "Connector stopped").await;
        Ok(())
    }
}
