use std::path::PathBuf;
use std::time::Duration;

use serde_json::Value;
use tauri::async_runtime::spawn;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::mpsc;

pub struct SidecarProcess {
    child: Child,
    stdin: Option<ChildStdin>,
}

fn resolve_node_executable() -> Result<PathBuf, String> {
    for candidate in ["/opt/homebrew/bin/node", "/usr/local/bin/node"] {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path
                .canonicalize()
                .map_err(|e| format!("Failed to resolve node at {}: {e}", path.display()));
        }
    }

    let output = std::process::Command::new("/usr/bin/which")
        .arg("node")
        .output()
        .map_err(|e| format!("Node.js not found (which failed: {e})"))?;

    if !output.status.success() {
        return Err(
            "Node.js not found. Install Node 20+ or ensure /opt/homebrew/bin/node exists.".into(),
        );
    }

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path_str.is_empty() {
        return Err("Node.js not found in PATH.".into());
    }

    PathBuf::from(&path_str)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve node at {path_str}: {e}"))
}

fn resolve_bundle_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dev_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bin/tiktok-connector-bundle.cjs");
    if dev_path.exists() {
        tracing::info!("[sidecar] bundle resolved from dev path: {}", dev_path.display());
        return Ok(dev_path);
    }

    if let Ok(path) = app_handle.path().resolve(
        "bin/tiktok-connector-bundle.cjs",
        BaseDirectory::Resource,
    ) {
        if path.exists() {
            tracing::info!("[sidecar] bundle resolved from resource: {}", path.display());
            return Ok(path);
        }
        tracing::warn!(
            "[sidecar] resource path missing bundle: {}",
            path.display()
        );
    }

    Err(format!(
        "Sidecar bundle not found. Run `pnpm run build:sidecar`. Checked: {}",
        dev_path.display()
    ))
}

impl SidecarProcess {
    pub async fn start(
        app_handle: &tauri::AppHandle,
        username: &str,
        session_id: &str,
    ) -> Result<(Self, mpsc::Receiver<Value>), String> {
        let (tx, rx) = mpsc::channel(100);

        let bundle_path = resolve_bundle_path(app_handle)?;
        let node_path = resolve_node_executable()?;

        if !bundle_path.exists() {
            return Err(format!(
                "Sidecar bundle missing at {}",
                bundle_path.display()
            ));
        }

        tracing::info!(
            "[sidecar] spawn node={} bundle={} user=@{} session={}",
            node_path.display(),
            bundle_path.display(),
            username,
            session_id
        );

        let mut child = Command::new(&node_path)
            .arg(&bundle_path)
            .arg("--username")
            .arg(username)
            .arg("--session-id")
            .arg(session_id)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| {
                tracing::error!(
                    "[sidecar] spawn failed: {} (node={}, bundle={})",
                    e,
                    node_path.display(),
                    bundle_path.display()
                );
                format!(
                    "Failed to spawn sidecar: {e} (node={}, bundle={})",
                    node_path.display(),
                    bundle_path.display()
                )
            })?;

        tracing::info!("[sidecar] spawn ok, pid={:?}", child.id());

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Sidecar stdout unavailable".to_string())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "Sidecar stderr unavailable".to_string())?;
        let stdin = child.stdin.take();

        spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let line_str = line.trim();
                if line_str.is_empty() {
                    continue;
                }
                match serde_json::from_str::<Value>(line_str) {
                    Ok(json) => {
                        let kind = json
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let stage = json.get("stage").and_then(|v| v.as_str()).unwrap_or("");
                        tracing::info!(
                            "[sidecar] stdout {} {}{}",
                            kind,
                            if stage.is_empty() { "" } else { "stage=" },
                            stage
                        );
                        if tx.send(json).await.is_err() {
                            tracing::warn!("[sidecar] stdout channel closed");
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::warn!(
                            "[sidecar] invalid JSON ({} bytes): {} | head={}",
                            line_str.len(),
                            err,
                            &line_str.chars().take(120).collect::<String>()
                        );
                    }
                }
            }
            tracing::info!("[sidecar] stdout reader finished");
        });

        spawn(async move {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                tracing::error!("[sidecar] stderr: {}", line.trim());
            }
        });

        Ok((Self { child, stdin }, rx))
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        tracing::info!("[sidecar] sending stop command");
        let stop_cmd = b"{\"type\":\"command\",\"command\":\"stop\"}\n";

        if let Some(ref mut stdin) = self.stdin {
            if stdin.write_all(stop_cmd).await.is_ok() {
                tokio::time::sleep(Duration::from_millis(800)).await;
            }
        }

        if let Err(e) = self.child.start_kill() {
            tracing::warn!("[sidecar] kill failed: {e}");
        }

        Ok(())
    }
}
