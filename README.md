# Casper: A JARVIS-Inspired Ghost Copilot in Rust

## 🎯 Project Vision

Casper is an open-source, AI-driven personal assistant inspired by JARVIS from Iron Man, designed as a "ghost copilot" for enhanced productivity on Linux systems (initially targeting ArchLinux with Gnome and Wayland). Unlike simple coding assistants or text-based tools, Casper **actually controls your computer** - opening applications, navigating interfaces, learning from your actions, and automating repetitive tasks.

### Key Objectives

- **Modularity and Speed**: Built in Rust for performance, safety, and concurrency
- **Privacy-Focused**: Offline capabilities where possible (local voice recognition, no cloud dependencies)
- **Extensibility**: Client-server architecture for easy addition of interfaces (TUI, tray, voice, web)
- **Session Sharing**: Multiple clients share the same daemon session for consistent state
- **Learning Capability**: Record and replay action sequences, building up a library of automated tasks
- **Initial Scope**: Linux-only (Wayland/Gnome), with plans for cross-platform expansion

### Current Features (v0.2.0)

#### ✅ Screen Control
- **Mouse Control**: Move, click (left/right/middle), drag, scroll, get position
- **Keyboard Control**: Type text, press keys, key combinations, special keys (Enter, Ctrl, Alt, etc.)
- **Precision**: Full coordinate control and timing adjustments

#### ✅ Window Management
- **Process Detection**: Check if applications are running
- **Window Control**: Focus, maximize, minimize, close, move/resize windows
- **Application Management**: Launch apps, detect visibility, smart open-or-focus
- **Window Discovery**: List all windows, find by name/pattern

#### ✅ Action Recording & Playback
- **Record Sequences**: Capture series of actions with timing
- **Action Library**: Save/load sequences from `~/.casper/actions/`
- **Replay Automation**: Execute recorded workflows on demand
- **Learning**: Build up a repertoire of automated tasks over time

#### ✅ Core Capabilities
- **Command Execution**: Run shell commands with output capture
- **Notifications**: Desktop notifications for feedback
- **Text-to-Speech**: Speak responses using espeak-ng
- **External Connections**: HTTP requests to APIs and services
- **Daemon Architecture**: Background service with Unix socket IPC

#### 🚧 In Development
- **AI/NLP**: Natural language command understanding (basic keyword matching implemented)
- **Voice Recognition**: Offline speech-to-text with Vosk (placeholder ready)
- **OCR & Vision**: Screen reading and UI element detection (planned)
- **Task Scheduler**: Cron-like automation and triggers (planned)

### Non-Goals

- No mandatory cloud dependencies
- No telemetry or data collection
- No Windows/macOS support initially (Linux first!)

The project emphasizes **practical utility today** while building towards a true JARVIS-like assistant.
Project Structure
Casper is a Rust monorepo (workspace) with separate crates for modularity:
casper/
├── .gitignore                  # Ignores build artifacts, temp files, etc.
├── Cargo.toml                  # Workspace config
├── README.md                   # Project overview (this file)
├── ARCHITECTURE.md             # ⭐ Complete roadmap and technical design
├── NEXT_STEPS.md               # ⭐ Actionable development guide
├── CONTRIBUTING.md             # Contribution guidelines
├── casper-core/                # Shared library with core logic
│   ├── src/
│   │   ├── lib.rs
│   │   ├── actions.rs          # ⭐ NEW: Action recording & playback
│   │   ├── ai.rs               # AI/NLP command processing
│   │   ├── commands.rs         # Shell command execution
│   │   ├── connections.rs      # External service integration
│   │   ├── mcp.rs              # Multi-Channel Protocol (placeholder)
│   │   ├── notifications.rs    # Desktop notifications
│   │   ├── screen.rs           # ⭐ ENHANCED: Full mouse/keyboard control
│   │   ├── tts.rs              # Text-to-speech
│   │   ├── voice.rs            # Voice recognition (placeholder)
│   │   └── window.rs           # ⭐ NEW: Window & process management
│   └── Cargo.toml
├── casper-daemon/              # ⭐ ENHANCED: Background service with full API
│   ├── src/
│   │   └── main.rs             # Unix socket server with 30+ endpoints
│   └── Cargo.toml
├── casper-tui/                 # Terminal User Interface client using Ratatui
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── casper-tray/                # System tray client (optional, GTK-based)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── examples/                   # ⭐ NEW: Usage examples
│   └── spotify_daily_mix.md    # Complete Spotify automation example
└── tests/                      # Test utilities
    └── daemon/
        └── client/             # Test client for daemon
            ├── src/
            │   └── main.rs
            └── Cargo.toml

**Communication**: Clients connect to the daemon via Unix sockets (`/tmp/casper.sock`) for IPC, ensuring session sharing.

**Dependencies**: Rust 2024 edition; crates include:
- `enigo` (screen control)
- `notify-rust` (notifications)
- `tokio` (async runtime)
- `serde_json` (messaging)
- `reqwest` (HTTP connections)
- `ratatui` & `crossterm` (TUI)
- `chrono` (timestamps)

**Build/Run**: Use `cargo build --workspace` to build all crates. Daemon must run first for clients to connect.

## 🚀 Quick Start

### Installation

```bash
# 1. Install system dependencies (ArchLinux)
sudo pacman -S rust espeak-ng libnotify gtk4 wmctrl xdotool

# 2. Clone the repository
git clone <repo-url>
cd casper

# 3. Build the workspace
cargo build --workspace

# 4. Create actions directory
mkdir -p ~/.casper/actions
```

### Running Casper

```bash
# Terminal 1: Start the daemon
cd casper-daemon
cargo run

# Terminal 2: Use the TUI client
cd casper-tui
cargo run

# Or use the test client
cd tests/daemon/client
cargo run
```

## 📚 Usage Examples

### Example 1: Basic Screen Control

```bash
# Move mouse to position (500, 300)
echo '{"type":"move_mouse","x":500,"y":300}' | nc -U /tmp/casper.sock

# Click left mouse button
echo '{"type":"click_mouse","button":"left"}' | nc -U /tmp/casper.sock

# Type some text
echo '{"type":"type_text","text":"Hello, World!"}' | nc -U /tmp/casper.sock

# Press Enter key
echo '{"type":"press_key","key":"enter"}' | nc -U /tmp/casper.sock

# Scroll down
echo '{"type":"scroll","amount":3,"direction":"down"}' | nc -U /tmp/casper.sock
```

### Example 2: Window Management

```bash
# Check if Spotify is running
echo '{"type":"is_process_running","process":"spotify"}' | nc -U /tmp/casper.sock

# Launch Spotify if not running
echo '{"type":"launch_application","app":"spotify"}' | nc -U /tmp/casper.sock

# Focus Spotify window
echo '{"type":"focus_window","window":"Spotify"}' | nc -U /tmp/casper.sock

# List all open windows
echo '{"type":"list_windows"}' | nc -U /tmp/casper.sock

# Find a specific window
echo '{"type":"find_window","pattern":"firefox"}' | nc -U /tmp/casper.sock
```

### Example 3: Recording Actions

```bash
# Start recording a sequence
echo '{"type":"start_recording","name":"open_github","description":"Open browser and go to GitHub"}' | nc -U /tmp/casper.sock

# Perform your actions...
echo '{"type":"launch_application","app":"firefox"}' | nc -U /tmp/casper.sock
# Wait, type URL, etc...

# Stop recording
echo '{"type":"stop_recording"}' | nc -U /tmp/casper.sock

# List saved sequences
echo '{"type":"list_sequences"}' | nc -U /tmp/casper.sock

# Replay the sequence
echo '{"type":"load_sequence","name":"open_github"}' | nc -U /tmp/casper.sock
echo '{"type":"play_sequence"}' | nc -U /tmp/casper.sock
```

### Example 4: The Spotify Daily Mix (Full Workflow)

See `examples/spotify_daily_mix.md` for a complete guide on automating Spotify!

```bash
# The goal: "Casper, play my daily mix on Spotify"

# 1. Check if Spotify is running, launch if needed
echo '{"type":"open_or_focus_application","app":"spotify"}' | nc -U /tmp/casper.sock

# 2. Record your manual navigation to Daily Mix
echo '{"type":"start_recording","name":"spotify_daily_mix"}' | nc -U /tmp/casper.sock
# ... click through Spotify UI ...
echo '{"type":"stop_recording"}' | nc -U /tmp/casper.sock

# 3. Now replay it anytime with one command!
echo '{"type":"load_sequence","name":"spotify_daily_mix"}' | nc -U /tmp/casper.sock
echo '{"type":"play_sequence"}' | nc -U /tmp/casper.sock
```

## 💻 Code Examples

### Screen Control (casper-core/src/screen.rs)

```rust
use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard, Button, Direction};

// Move mouse
pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.move_mouse(x, y, Coordinate::Abs).map_err(|e| e.to_string())?;
    Ok(())
}

// Click mouse
pub fn click_mouse(button: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    let btn = match button {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => return Err(format!("Unknown button: {}", button)),
    };
    enigo.button(btn, Direction::Click).map_err(|e| e.to_string())?;
    Ok(())
}

// Type text
pub fn type_text(text: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.fast_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

// Scroll
pub fn scroll(amount: i32, direction: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    match direction {
        "up" | "down" => {
            let scroll_amount = if direction == "down" { -amount } else { amount };
            enigo.scroll(scroll_amount, enigo::Axis::Vertical)
                .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Unknown direction: {}", direction)),
    }
    Ok(())
}
```

### Window Management (casper-core/src/window.rs)

```rust
use std::process::Command;

// Check if process is running
pub fn is_process_running(process_name: &str) -> Result<bool, String> {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg(process_name)
        .output()
        .map_err(|e| format!("Failed to execute pgrep: {}", e))?;
    Ok(output.status.success())
}

// Launch application
pub fn launch_application(app_name: &str) -> Result<(), String> {
    Command::new(app_name)
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", app_name, e))?;
    Ok(())
}

// Focus window
pub fn focus_window(app_name: &str) -> Result<(), String> {
    let output = Command::new("wmctrl")
        .arg("-a")
        .arg(app_name)
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Failed to focus window: {}", 
            String::from_utf8_lossy(&output.stderr)))
    }
}
```

### Action Recording (casper-core/src/actions.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    MoveMouse { x: i32, y: i32 },
    ClickMouse { button: String },
    TypeText { text: String },
    PressKey { key: String },
    Wait { milliseconds: u64 },
    LaunchApp { app_name: String },
    // ... more action types
}

pub struct ActionRecorder {
    current_sequence: Option<ActionSequence>,
    is_recording: bool,
}

impl ActionRecorder {
    pub fn start_recording(&mut self, name: String, description: String) {
        self.current_sequence = Some(ActionSequence::new(name, description));
        self.is_recording = true;
    }
    
    pub fn record_action(&mut self, action: Action) -> Result<(), String> {
        if let Some(ref mut sequence) = self.current_sequence {
            sequence.add_action(action, delay_ms);
            Ok(())
        } else {
            Err("Not recording".to_string())
        }
    }
}
```

### Daemon (casper-daemon/src/main.rs)

Background service with 30+ endpoints:
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

## 🗺️ Roadmap

### ✅ Phase 1: Enhanced Screen Control (COMPLETED)
- Full mouse control (move, click, scroll, position)
- Complete keyboard control (type, keys, combinations)
- Window management (detect, focus, launch, control)
- Action recording and playback system

### 🚧 Phase 2: Screen Vision & Understanding (1-2 months)
- Screen capture (Wayland/X11)
- OCR text extraction (Tesseract)
- UI element detection
- Adaptive action playback

### 📅 Phase 3: AI & Natural Language (2-3 months)
- Intent recognition from natural language
- Local LLM integration (optional)
- Context-aware command processing
- Smart fallback strategies

### 📅 Phase 4: Voice Integration (1 month)
- Speech-to-text with Vosk (offline)
- Wake word detection ("Hey Casper")
- Better TTS (piper, coqui)
- Voice activity detection

### 📅 Phase 5: Task Automation (1-2 months)
- Task scheduler (cron-like)
- Trigger-based automation
- Application-specific plugins
- Learning from repeated patterns

### 📅 Phase 6: Advanced Features (Ongoing)
- Multi-monitor support
- Remote control (mobile app)
- Plugin marketplace
- Cross-platform expansion

## 📖 Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete technical design and vision
- **[NEXT_STEPS.md](NEXT_STEPS.md)** - Actionable development guide with weekly goals
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute to the project
- **[examples/](examples/)** - Real-world usage examples

## 🤝 Contributing

Contributions are welcome! Whether you're:
- Fixing bugs
- Adding features
- Improving documentation
- Creating examples
- Testing on different systems

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

[Add your license here]

## 🙏 Acknowledgments

Inspired by:
- JARVIS from Iron Man
- Talon Voice
- Hammerspoon
- AutoHotkey

Built with amazing Rust crates:
- `enigo` for input control
- `tokio` for async runtime
- `ratatui` for TUI
- `notify-rust` for notifications

---

**Start building YOUR JARVIS today!** 🚀

Check out [NEXT_STEPS.md](NEXT_STEPS.md) to begin your journey.