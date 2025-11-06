use casper_core::actions::{Action, ActionLibrary, ActionPlayer, ActionRecorder};
use casper_core::ai::process_command;
use casper_core::commands::run_command;
use casper_core::connections::connect_to_service;
use casper_core::mcp::process_mcp;
use casper_core::notifications::show_notification;
use casper_core::screen::{
    click_mouse, get_mouse_position, key_down, key_up, mouse_down, mouse_up, move_mouse, press_key,
    scroll, type_text,
};
use casper_core::tts::speak;
use casper_core::voice::recognize_voice;
use casper_core::window::{
    close_window, find_window_by_pattern, focus_window, is_application_visible, is_process_running,
    launch_application, list_windows, maximize_window, minimize_window, move_resize_window,
    open_or_focus_application,
};
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;

struct DaemonState {
    recorder: ActionRecorder,
    player: ActionPlayer,
    library: ActionLibrary,
}

impl DaemonState {
    fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let library_path = format!("{}/.casper/actions", home_dir);

        let mut library = ActionLibrary::new(library_path);
        let _ = library.load_all(); // Load existing sequences

        DaemonState {
            recorder: ActionRecorder::new(),
            player: ActionPlayer::new(),
            library,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = Path::new("/tmp/casper.sock");
    if socket_path.exists() {
        std::fs::remove_file(socket_path)?;
    }
    let listener = UnixListener::bind(socket_path)?;

    let state = Arc::new(Mutex::new(DaemonState::new()));

    println!("🤖 Casper Daemon v0.2.0 listening on {:?}", socket_path);
    println!("📝 Action library: ~/.casper/actions");
    println!("✨ Ready to assist!");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state_clone = Arc::clone(&state);

        tokio::spawn(async move {
            let mut buf = vec![0; 4096]; // Increased buffer size for larger payloads
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let request = String::from_utf8_lossy(&buf[..n]);

            let req: serde_json::Value = match serde_json::from_str(&request) {
                Ok(v) => v,
                Err(e) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Invalid JSON: {}", e)
                    });
                    let _ = socket.write_all(response.to_string().as_bytes()).await;
                    return;
                }
            };

            let response = handle_request(&req, &state_clone).await;
            let response_str = response.to_string();
            let _ = socket.write_all(response_str.as_bytes()).await;
        });
    }
}

async fn handle_request(
    req: &serde_json::Value,
    state: &Arc<Mutex<DaemonState>>,
) -> serde_json::Value {
    match req["type"].as_str() {
        // Basic Commands
        Some("run_command") => {
            let cmd = req["command"].as_str().unwrap_or("");
            match run_command(cmd) {
                Ok(output) => json!({ "status": "success", "output": output }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Screen Control - Mouse
        Some("move_mouse") => {
            let x = req["x"].as_i64().unwrap_or(0) as i32;
            let y = req["y"].as_i64().unwrap_or(0) as i32;
            match move_mouse(x, y) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("click_mouse") => {
            let button = req["button"].as_str().unwrap_or("left");
            match click_mouse(button) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("mouse_down") => {
            let button = req["button"].as_str().unwrap_or("left");
            match mouse_down(button) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("mouse_up") => {
            let button = req["button"].as_str().unwrap_or("left");
            match mouse_up(button) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("scroll") => {
            let amount = req["amount"].as_i64().unwrap_or(1) as i32;
            let direction = req["direction"].as_str().unwrap_or("up");
            match scroll(amount, direction) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("get_mouse_position") => match get_mouse_position() {
            Ok((x, y)) => json!({ "status": "success", "x": x, "y": y }),
            Err(e) => json!({ "status": "error", "message": e }),
        },

        // Screen Control - Keyboard
        Some("type_text") => {
            let text = req["text"].as_str().unwrap_or("");
            match type_text(text) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("press_key") => {
            let key = req["key"].as_str().unwrap_or("");
            match press_key(key) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("key_down") => {
            let key = req["key"].as_str().unwrap_or("");
            match key_down(key) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("key_up") => {
            let key = req["key"].as_str().unwrap_or("");
            match key_up(key) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Window Management
        Some("is_process_running") => {
            let process = req["process"].as_str().unwrap_or("");
            match is_process_running(process) {
                Ok(running) => json!({ "status": "success", "running": running }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("is_application_visible") => {
            let app = req["app"].as_str().unwrap_or("");
            match is_application_visible(app) {
                Ok(visible) => json!({ "status": "success", "visible": visible }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("launch_application") => {
            let app = req["app"].as_str().unwrap_or("");
            match launch_application(app) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("focus_window") => {
            let window = req["window"].as_str().unwrap_or("");
            match focus_window(window) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("list_windows") => match list_windows() {
            Ok(windows) => {
                let windows_json: Vec<_> = windows
                    .iter()
                    .map(|w| {
                        json!({
                            "id": w.id,
                            "pid": w.pid,
                            "desktop": w.desktop,
                            "class": w.class,
                            "title": w.title,
                            "machine": w.machine,
                        })
                    })
                    .collect();
                json!({ "status": "success", "windows": windows_json })
            }
            Err(e) => json!({ "status": "error", "message": e }),
        },
        Some("find_window") => {
            let pattern = req["pattern"].as_str().unwrap_or("");
            match find_window_by_pattern(pattern) {
                Ok(Some(window)) => json!({
                    "status": "success",
                    "window": {
                        "id": window.id,
                        "pid": window.pid,
                        "desktop": window.desktop,
                        "class": window.class,
                        "title": window.title,
                        "machine": window.machine,
                    }
                }),
                Ok(None) => json!({ "status": "success", "window": null }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("maximize_window") => {
            let window_id = req["window_id"].as_str().unwrap_or("");
            match maximize_window(window_id) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("minimize_window") => {
            let window_id = req["window_id"].as_str().unwrap_or("");
            match minimize_window(window_id) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("close_window") => {
            let window_id = req["window_id"].as_str().unwrap_or("");
            match close_window(window_id) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("move_resize_window") => {
            let window_id = req["window_id"].as_str().unwrap_or("");
            let x = req["x"].as_i64().unwrap_or(0) as i32;
            let y = req["y"].as_i64().unwrap_or(0) as i32;
            let width = req["width"].as_i64().unwrap_or(800) as i32;
            let height = req["height"].as_i64().unwrap_or(600) as i32;
            match move_resize_window(window_id, x, y, width, height) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("open_or_focus_application") => {
            let app = req["app"].as_str().unwrap_or("");
            let launch_cmd = req["launch_command"].as_str();
            match open_or_focus_application(app, launch_cmd) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Action Recording
        Some("start_recording") => {
            let name = req["name"].as_str().unwrap_or("Unnamed");
            let description = req["description"].as_str().unwrap_or("");
            let mut state = state.lock().unwrap();
            match state
                .recorder
                .start_recording(name.to_string(), description.to_string())
            {
                Ok(_) => json!({ "status": "success", "message": "Recording started" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("stop_recording") => {
            let mut state = state.lock().unwrap();
            match state.recorder.stop_recording() {
                Ok(sequence) => {
                    state.library.add_sequence(sequence.clone());
                    let _ = state.library.save_all();
                    json!({
                        "status": "success",
                        "message": "Recording stopped",
                        "sequence": sequence.name
                    })
                }
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("record_action") => {
            let action_type = req["action"].as_str().unwrap_or("");
            let mut state = state.lock().unwrap();

            let action = match action_type {
                "move_mouse" => {
                    let x = req["x"].as_i64().unwrap_or(0) as i32;
                    let y = req["y"].as_i64().unwrap_or(0) as i32;
                    Action::MoveMouse { x, y }
                }
                "click_mouse" => {
                    let button = req["button"].as_str().unwrap_or("left").to_string();
                    Action::ClickMouse { button }
                }
                "type_text" => {
                    let text = req["text"].as_str().unwrap_or("").to_string();
                    Action::TypeText { text }
                }
                "press_key" => {
                    let key = req["key"].as_str().unwrap_or("").to_string();
                    Action::PressKey { key }
                }
                "wait" => {
                    let ms = req["milliseconds"].as_u64().unwrap_or(1000);
                    Action::Wait { milliseconds: ms }
                }
                _ => {
                    return json!({
                        "status": "error",
                        "message": format!("Unknown action type: {}", action_type)
                    });
                }
            };

            match state.recorder.record_action(action) {
                Ok(_) => json!({ "status": "success", "message": "Action recorded" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("is_recording") => {
            let state = state.lock().unwrap();
            json!({
                "status": "success",
                "recording": state.recorder.is_recording()
            })
        }

        // Action Playback
        Some("load_sequence") => {
            let name = req["name"].as_str().unwrap_or("");
            let sequence_clone = {
                let state = state.lock().unwrap();
                state.library.get_sequence(name).cloned()
            };

            if let Some(sequence) = sequence_clone {
                let mut state = state.lock().unwrap();
                state.player.load_sequence(sequence.clone());
                json!({
                    "status": "success",
                    "message": format!("Loaded sequence: {}", sequence.name)
                })
            } else {
                json!({
                    "status": "error",
                    "message": format!("Sequence not found: {}", name)
                })
            }
        }
        Some("play_sequence") => {
            let mut state = state.lock().unwrap();
            match state.player.start_playback() {
                Ok(_) => {
                    // Playback happens synchronously here for simplicity
                    drop(state); // Release lock
                    json!({ "status": "success", "message": "Playback started" })
                }
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }
        Some("list_sequences") => {
            let state = state.lock().unwrap();
            let sequences = state.library.list_sequences();
            json!({ "status": "success", "sequences": sequences })
        }
        Some("delete_sequence") => {
            let name = req["name"].as_str().unwrap_or("");
            let mut state = state.lock().unwrap();
            match state.library.delete_sequence(name) {
                Ok(_) => json!({
                    "status": "success",
                    "message": format!("Deleted sequence: {}", name)
                }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Notifications
        Some("show_notification") => {
            let summary = req["summary"].as_str().unwrap_or("");
            let body = req["body"].as_str().unwrap_or("");
            match show_notification(summary, body) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // External Services
        Some("connect_to_service") => {
            let service = req["service"].as_str().unwrap_or("");
            let action = req["action"].as_str().unwrap_or("");
            match connect_to_service(service, action).await {
                Ok(result) => json!({ "status": "success", "result": result }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // MCP
        Some("process_mcp") => {
            let data = req["data"].as_str().unwrap_or("");
            match process_mcp(data) {
                Ok(result) => json!({ "status": "success", "result": result }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // AI
        Some("process_command") => {
            let command = req["command"].as_str().unwrap_or("");
            match process_command(command) {
                Ok(result) => json!({ "status": "success", "result": result }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Voice
        Some("recognize_voice") => match recognize_voice() {
            Ok(result) => json!({ "status": "success", "result": result }),
            Err(e) => json!({ "status": "error", "message": e }),
        },

        // TTS
        Some("speak") => {
            let text = req["text"].as_str().unwrap_or("");
            match speak(text) {
                Ok(_) => json!({ "status": "success" }),
                Err(e) => json!({ "status": "error", "message": e }),
            }
        }

        // Ping/Status
        Some("ping") => json!({
            "status": "success",
            "message": "pong",
            "version": "0.2.0"
        }),

        // Unknown
        _ => json!({
            "status": "error",
            "message": format!("Unknown request type: {:?}", req["type"])
        }),
    }
}
