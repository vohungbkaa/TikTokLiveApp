use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::Mutex;
use sqlx::SqlitePool;
use crate::connectors::node_sidecar::SidecarProcess;
use crate::events::ingestion::EventIngestionService;
use crate::events::models::IncomingEvent;

pub struct ConnectorSupervisor {
    app_handle: AppHandle,
    pool: SqlitePool,
    process: Arc<Mutex<Option<SidecarProcess>>>,
}

impl ConnectorSupervisor {
    pub fn new(app_handle: AppHandle, pool: SqlitePool) -> Self {
        Self {
            app_handle,
            pool,
            process: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_connector(&self, username: &str, session_id: &str) -> Result<(), String> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut old_proc) = proc_guard.take() {
            let _ = old_proc.stop();
        }

        let (proc, mut rx) = SidecarProcess::start(&self.app_handle, username, session_id)?;
        *proc_guard = Some(proc);

        let app_handle = self.app_handle.clone();
        let session_id_cloned = session_id.to_string();
        let ingestion_svc = EventIngestionService::new(self.pool.clone(), self.app_handle.clone());
        
        tauri::async_runtime::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Some(msg_type) = msg.get("type").and_then(|t| t.as_str()) {
                    match msg_type {
                        "event" => {
                            if let Some(data) = msg.get("data") {
                                if let Ok(incoming) = serde_json::from_value::<IncomingEvent>(data.clone()) {
                                    let _ = ingestion_svc.ingest(&session_id_cloned, incoming).await;
                                } else {
                                    tracing::error!("Failed to parse incoming event: {:?}", data);
                                }
                            }
                        }
                        "health" => {
                            let _ = app_handle.emit("connector:health", &msg);
                        }
                        "error" => {
                            let _ = app_handle.emit("connector:error", &msg);
                        }
                        _ => {}
                    }
                }
            }
            // If the loop exits, process died
            tracing::info!("Sidecar stdout loop ended.");
            let _ = app_handle.emit("connector:status", "down");
        });

        Ok(())
    }

    pub async fn stop_connector(&self) -> Result<(), String> {
        let mut proc_guard = self.process.lock().await;
        if let Some(mut proc) = proc_guard.take() {
            proc.stop()?;
        }
        Ok(())
    }
}
