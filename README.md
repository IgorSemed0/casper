Casper: A JARVIS-Inspired Ghost Copilot in Rust
Project Goal
Casper is an open-source, AI-driven personal assistant inspired by JARVIS from Iron Man, designed as a "ghost copilot" for enhanced productivity on Linux systems (initially targeting ArchLinux with Gnome and Wayland). Unlike simple coding assistants, Casper performs real-world actions such as controlling the screen (mouse/keyboard), executing shell commands, connecting to external software/services, supporting MCP (Multi-Channel Protocol, placeholder for custom protocol integration), processing AI-driven natural language commands, responding to voice inputs, speaking responses via text-to-speech, and sending desktop notifications.
Key objectives:

Modularity and Speed: Built in Rust for performance, safety, and concurrency.
Privacy-Focused: Offline capabilities where possible (e.g., voice recognition with Vosk).
Extensibility: Client-server architecture for easy addition of interfaces (TUI, tray, future GUI).
Session Sharing: Multiple clients (TUI, tray) share the same daemon session for consistent state.
Initial Scope: Linux-only (Wayland/Gnome), with plans for cross-platform expansion.
Features:
Screen Interactions: Move mouse, click, type text.
Command Execution: Run shell commands (e.g., echo Hello, World!).
Software Connections: Integrate with APIs or local apps (e.g., HTTP requests via reqwest).
MCP Support: Placeholder for multi-channel protocol (clarification needed for full implementation).
AI-Driven: Basic keyword processing, expandable to NLP with rust-bert.
Voice Commands: Offline recognition (placeholder, to use vosk-rust).
Text-to-Speech: Speak responses using espeak-ng.
Notifications: Desktop pop-ups via notify-rust.


Non-Goals: No cloud dependencies; avoid external APIs unless specified; no Windows/macOS support initially.

The project emphasizes rapid development, learning Rust in the process, and starting with a TUI interface backed by a daemon.
Project Structure
Casper is a Rust monorepo (workspace) with separate crates for modularity:
casper/
├── .gitignore                  # Ignores build artifacts, temp files, etc.
├── Cargo.toml                  # Workspace config
├── README.md                   # Project overview (this file)
├── casper-core/                # Shared library with core logic (commands, screen, etc.)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── commands.rs
│   │   ├── screen.rs
│   │   ├── notifications.rs
│   │   ├── connections.rs
│   │   ├── mcp.rs
│   │   ├── ai.rs
│   │   ├── voice.rs
│   │   └── tts.rs
│   └── Cargo.toml
├── casper-daemon/              # Background service handling requests via Unix sockets
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── casper-tui/                 # Terminal User Interface client using Ratatui
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── casper-tray/                # System tray client (optional, GTK-based, Wayland-limited)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
└── tests/                      # Test utilities
    └── daemon/
        └── client/             # Test client for daemon
            ├── src/
            │   └── main.rs
            └── Cargo.toml


Communication: Clients connect to the daemon via Unix sockets (/tmp/casper.sock) for IPC, ensuring session sharing.
Dependencies: Rust 2024 edition; crates like enigo (screen control), notify-rust (notifications), tokio (async), serde_json (messaging), reqwest (connections), ratatui & crossterm (TUI), gtk4 (tray).
Build/Run: Use cargo run in each crate directory. Daemon must run first for clients to connect.

Code Examples
Core Library (casper-core/src/screen.rs)
Handles screen interactions using enigo:
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard};

pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.move_mouse(x, y, Coordinate::Abs).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn type_text(text: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.fast_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

Daemon (casper-daemon/src/main.rs)
Background service listening for JSON requests:
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use casper_core::commands::run_command;
// ... other imports for features ...

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
                // ... match arms for other features ...
                _ => json!({ "status": "error", "message": "Unknown request type" }),
            };

            let response_str = response.to_string();
            socket.write_all(response_str.as_bytes()).await.unwrap_or(());
        });
    }
}

TUI Client (casper-tui/src/main.rs)
Interactive terminal interface:
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
// ... other imports ...

fn main() -> io::Result<()> {
    // Setup terminal ...
    let mut app = App::new();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        loop {
            // Draw TUI layout ...
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => {
                        app.input.pop();
                    },
                    KeyCode::Enter => {
                        // Send request to daemon ...
                    },
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        Ok::<(), io::Error>(())
    })?;
    // Cleanup terminal ...
    Ok(())
}

Installation and Setup

Install Rust (2024 edition) and dependencies (e.g., sudo pacman -S espeak-ng libnotify gtk4 on ArchLinux).
Clone the repo: git clone <repo-url>.
Build: cargo build in root.
Run Daemon: cd casper-daemon && cargo run.
Run TUI: cd casper-tui && cargo run.
Test: Use the test client in tests/daemon/client.

Roadmap

Implement voice recognition with vosk-rust.
Enhance AI with rust-bert for NLP.
Add MCP protocol (pending clarification).
Develop tray client.
Expand to other platforms.

Contributions welcome! See CONTRIBUTING.md for details.