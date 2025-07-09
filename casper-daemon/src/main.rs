use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Add these imports
use std::path::Path;
use casper_core::commands::run_command;
use casper_core::screen::move_mouse;
use casper_core::notifications::show_notification;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = Path::new("/tmp/casper.sock");
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    let listener = UnixListener::bind(socket_path)?;

    println!("Daemon listening on {:?}", socket_path);
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let request = String::from_utf8_lossy(&buf[..n]);
            let req: serde_json::Value = match serde_json::from_str(&request) {
                Ok(v) => v,
                Err(_) => {
                    let response = json!({ "status": "error", "message": "Invalid JSON" });
                    socket.write_all(response.to_string().as_bytes()).await.unwrap_or(());
                    return;
                }
            };

            let response = match req["type"].as_str() {
                Some("run_command") => {
                    let cmd = req["command"].as_str().unwrap_or("");
                    match run_command(cmd) {
                        Ok(output) => json!({ "status": "success", "output": output }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("move_mouse") => {
                    let x = req["x"].as_i64().unwrap_or(0) as i32;
                    let y = req["y"].as_i64().unwrap_or(0) as i32;
                    match move_mouse(x, y) {
                        Ok(_) => json!({ "status": "success" }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("show_notification") => {
                    let summary = req["summary"].as_str().unwrap_or("");
                    let body = req["body"].as_str().unwrap_or("");
                    match show_notification(summary, body) {
                        Ok(_) => json!({ "status": "success" }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                _ => json!({ "status": "error", "message": "Unknown request type" }),
            };

            let response_str = response.to_string();
            socket.write_all(response_str.as_bytes()).await.unwrap_or(());
        });
    }
}