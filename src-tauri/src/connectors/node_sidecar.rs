use tauri::async_runtime::spawn;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandEvent, CommandChild};
use tokio::sync::mpsc;
use serde_json::Value;

pub struct SidecarProcess {
    child: CommandChild,
}

impl SidecarProcess {
    pub fn start(
        app_handle: &tauri::AppHandle,
        username: &str,
        session_id: &str,
    ) -> Result<(Self, mpsc::Receiver<Value>), String> {
        let (tx, rx) = mpsc::channel(100);
        
        let sidecar_command = app_handle.shell().sidecar("tiktok-connector").map_err(|e| e.to_string())?
            .args(["--username", username, "--session-id", session_id]);
            
        let (mut rx_cmd, child) = sidecar_command.spawn().map_err(|e| e.to_string())?;
        
        spawn(async move {
            while let Some(event) = rx_cmd.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        let line_str = String::from_utf8_lossy(&line);
                        if let Ok(json) = serde_json::from_str::<Value>(&line_str) {
                            let _ = tx.send(json).await;
                        }
                    }
                    CommandEvent::Stderr(line) => {
                        let line_str = String::from_utf8_lossy(&line);
                        tracing::error!("Sidecar Error: {}", line_str);
                    }
                    CommandEvent::Terminated(payload) => {
                        tracing::info!("Sidecar terminated: {:?}", payload);
                        break;
                    }
                    CommandEvent::Error(err) => {
                        tracing::error!("Sidecar Command Error: {}", err);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        Ok((Self { child }, rx))
    }
    
    pub fn stop(&mut self) -> Result<(), String> {
        self.child.write(b"{\"type\":\"command\",\"command\":\"stop\"}\n").map_err(|e| e.to_string())?;
        Ok(())
    }
}
