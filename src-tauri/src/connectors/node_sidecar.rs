use std::path::{Path, PathBuf};
use tauri::async_runtime::spawn;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;
use tokio::sync::mpsc;
use serde_json::Value;

pub struct SidecarProcess {
    child: CommandChild,
}

const NODE_CANDIDATES: &[(&str, &str)] = &[
    ("node-homebrew", "/opt/homebrew/bin/node"),
    ("node-local", "/usr/local/bin/node"),
    ("node", "node"),
];

fn resolve_node_command() -> &'static str {
    for (name, path) in NODE_CANDIDATES {
        if *path == "node" || Path::new(path).exists() {
            return name;
        }
    }
    "node"
}

fn resolve_bundle_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    if let Ok(path) = app_handle
        .path()
        .resolve("bin/tiktok-connector-bundle.cjs", BaseDirectory::Resource)
    {
        if path.exists() {
            return Ok(path);
        }
    }

    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bin/tiktok-connector-bundle.cjs");
    if dev_path.exists() {
        return Ok(dev_path);
    }

    Err(format!(
        "Sidecar bundle not found. Run `pnpm run build:sidecar` in sidecars/tiktok-connector. Checked: {}",
        dev_path.display()
    ))
}

impl SidecarProcess {
    pub fn start(
        app_handle: &tauri::AppHandle,
        username: &str,
        session_id: &str,
    ) -> Result<(Self, mpsc::Receiver<Value>), String> {
        let (tx, rx) = mpsc::channel(100);

        let bundle_path = resolve_bundle_path(app_handle)?;
        let bundle_str = bundle_path
            .to_str()
            .ok_or_else(|| "Invalid sidecar bundle path".to_string())?
            .to_string();

        let node_cmd = resolve_node_command();
        tracing::info!(
            "Starting TikTok sidecar via {}: {} (user={})",
            node_cmd,
            bundle_str,
            username
        );

        let sidecar_command = app_handle.shell().command(node_cmd).args([
            &bundle_str,
            "--username",
            username,
            "--session-id",
            session_id,
        ]);

        let (mut rx_cmd, child) = sidecar_command.spawn().map_err(|e| e.to_string())?;

        spawn(async move {
            while let Some(event) = rx_cmd.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        let line_str = String::from_utf8_lossy(&line).trim().to_string();
                        if line_str.is_empty() {
                            continue;
                        }
                        match serde_json::from_str::<Value>(&line_str) {
                            Ok(json) => {
                                let _ = tx.send(json).await;
                            }
                            Err(err) => {
                                tracing::warn!(
                                    "Sidecar stdout is not valid JSON ({} bytes): {}",
                                    line_str.len(),
                                    err
                                );
                            }
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
        self.child
            .write(b"{\"type\":\"command\",\"command\":\"stop\"}\n")
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
