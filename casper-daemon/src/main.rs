use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use casper_core::commands::run_command;
use casper_core::screen::{move_mouse, type_text};
use casper_core::notifications::show_notification;
use casper_core::connections::connect_to_service;
use casper_core::mcp::process_mcp;
use casper_core::ai::process_command;
use casper_core::voice::recognize_voice;
use casper_core::tts::speak;
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
                Some("type_text") => {
                    let text = req["text"].as_str().unwrap_or("");
                    match type_text(text) {
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
                Some("connect_to_service") => {
                    let service = req["service"].as_str().unwrap_or("");
                    let action = req["action"].as_str().unwrap_or("");
                    match connect_to_service(service, action).await {
                        Ok(result) => json!({ "status": "success", "result": result }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("process_mcp") => {
                    let data = req["data"].as_str().unwrap_or("");
                    match process_mcp(data) {
                        Ok(result) => json!({ "status": "success", "result": result }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("process_command") => {
                    let command = req["command"].as_str().unwrap_or("");
                    match process_command(command) {
                        Ok(result) => json!({ "status": "success", "result": result }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("recognize_voice") => {
                    match recognize_voice() {
                        Ok(result) => json!({ "status": "success", "result": result }),
                        Err(e) => json!({ "status": "error", "message": e }),
                    }
                },
                Some("speak") => {
                    let text = req["text"].as_str().unwrap_or("");
                    match speak(text) {
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