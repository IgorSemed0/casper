# Casper Quick Start Guide

Get your JARVIS assistant running in 5 minutes! 🚀

## Prerequisites

- ArchLinux (or similar) with Gnome/Wayland
- Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

## 1. Install Dependencies (2 minutes)

```bash
# Core dependencies
sudo pacman -S rust espeak-ng libnotify gtk4 wmctrl xdotool

# Screenshot tools (choose based on your setup)
# For Wayland/Gnome:
sudo pacman -S grim slurp

# For X11:
sudo pacman -S scrot
```

## 2. Clone and Build (2 minutes)

```bash
# Clone the repository
git clone https://github.com/yourusername/casper.git
cd casper

# Build everything
cargo build --workspace --release

# Create actions directory
mkdir -p ~/.casper/actions
```

## 3. Configure AI (1 minute)

```bash
# Copy environment template
cp .env.example .env

# Edit .env and add your Gemini API key
# Get one free at: https://aistudio.google.com/app/apikey
nano .env

# Change this line:
# AI_TOKEN=your_gemini_api_key_here
# To your actual key:
# AI_TOKEN=AIzaSyAbc123...
```

## 4. Start Casper (30 seconds)

```bash
# Terminal 1: Start the daemon
cd casper-daemon
cargo run --release

# Terminal 2: Use the TUI client (or run tests)
cd ../casper-tui
cargo run --release
```

You should see:
```
🤖 Casper Daemon v0.2.0 listening on /tmp/casper.sock
📝 Action library: ~/.casper/actions
✨ Ready to assist!
```

## 5. Test It! (Quick Tests)

Open a new terminal and try these commands:

### Test 1: Check if Firefox is running
```bash
echo '{"type":"is_process_running","process":"firefox"}' | nc -U /tmp/casper.sock
```

### Test 2: Move your mouse
```bash
echo '{"type":"move_mouse","x":500,"y":500}' | nc -U /tmp/casper.sock
```

### Test 3: Get mouse position
```bash
echo '{"type":"get_mouse_position"}' | nc -U /tmp/casper.sock
```

### Test 4: Show a notification
```bash
echo '{"type":"show_notification","summary":"Hello","body":"Casper is working!"}' | nc -U /tmp/casper.sock
```

### Test 5: Speak something
```bash
echo '{"type":"speak","text":"Hello, I am Casper, your assistant"}' | nc -U /tmp/casper.sock
```

### Test 6: List all windows
```bash
echo '{"type":"list_windows"}' | nc -U /tmp/casper.sock
```

## First Real Task: Open Spotify

Let's do something useful - open Spotify and get ready to control it:

```bash
# 1. Check if Spotify is running
echo '{"type":"is_process_running","process":"spotify"}' | nc -U /tmp/casper.sock

# 2. If not, open it (or focus if already open)
echo '{"type":"open_or_focus_application","app":"spotify"}' | nc -U /tmp/casper.sock

# 3. Wait a moment, then list windows to find it
sleep 2
echo '{"type":"find_window","pattern":"spotify"}' | nc -U /tmp/casper.sock
```

## Record Your First Action Sequence

Now let's record a sequence:

```bash
# 1. Start recording
echo '{"type":"start_recording","name":"test_sequence","description":"My first recording"}' | nc -U /tmp/casper.sock

# 2. Do some actions (these get recorded with timing)
echo '{"type":"move_mouse","x":100,"y":100}' | nc -U /tmp/casper.sock
sleep 1
echo '{"type":"click_mouse","button":"left"}' | nc -U /tmp/casper.sock

# 3. Stop recording
echo '{"type":"stop_recording"}' | nc -U /tmp/casper.sock

# 4. List saved sequences
echo '{"type":"list_sequences"}' | nc -U /tmp/casper.sock

# 5. Check the saved file
ls ~/.casper/actions/
cat ~/.casper/actions/test_sequence.json
```

## Use AI Vision (if you configured Gemini)

```bash
# The daemon needs to be updated to include AI vision endpoints
# For now, you can use it in your own Rust code:
```

Create a file `test_vision.rs`:

```rust
use casper_core::capture::capture_screen_temp;
use casper_core::ai_vision::AIVision;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Capture screen
    let screenshot = capture_screen_temp()?;
    println!("📸 Screenshot saved to: {}", screenshot);
    
    // Analyze with AI
    let ai = AIVision::from_env()?;
    let description = ai.describe_screen(&screenshot).await?;
    
    println!("🤖 AI sees: {}", description);
    
    Ok(())
}
```

Run it:
```bash
cargo run --bin test_vision
```

## What's Next?

### Beginner Tasks (Today)
1. ✅ Read `examples/spotify_daily_mix.md` for a complete workflow
2. ✅ Record 3 different action sequences
3. ✅ Test window management commands
4. ✅ Make Casper speak your name!

### Intermediate Tasks (This Week)
1. 📚 Read `ARCHITECTURE.md` to understand the design
2. 🔨 Add AI vision endpoints to the daemon
3. 🎵 Implement the Spotify Daily Mix automation
4. 📝 Create your own action sequences for daily tasks

### Advanced Tasks (This Month)
1. 🧠 Integrate voice recognition
2. 👁️ Add OCR or AI vision to all workflows
3. ⚙️ Build application-specific plugins
4. 🤝 Contribute back to the project!

## Common Issues

### "No such file or directory: /tmp/casper.sock"
The daemon isn't running. Start it first:
```bash
cd casper-daemon && cargo run --release
```

### "Failed to execute grim/scrot"
Install screenshot tools:
```bash
sudo pacman -S grim slurp  # Wayland
# OR
sudo pacman -S scrot       # X11
```

### "AI_TOKEN not set in environment"
Create and configure `.env`:
```bash
cp .env.example .env
# Edit .env and add your Gemini API key
```

### "Permission denied" on socket
Remove old socket:
```bash
rm /tmp/casper.sock
```

### Mouse/keyboard not working
Make sure you're running with appropriate permissions. Some systems require the user to be in specific groups for input simulation.

## Helpful Aliases

Add these to your `~/.bashrc`:

```bash
# Casper shortcuts
alias casper-start='cd ~/casper/casper-daemon && cargo run --release'
alias casper-tui='cd ~/casper/casper-tui && cargo run --release'
alias casper-test='cd ~/casper/tests/daemon/client && cargo run'
alias casper-cmd='nc -U /tmp/casper.sock'

# Quick commands
casper-ping() {
    echo '{"type":"ping"}' | nc -U /tmp/casper.sock
}

casper-speak() {
    echo "{\"type\":\"speak\",\"text\":\"$1\"}" | nc -U /tmp/casper.sock
}

casper-launch() {
    echo "{\"type\":\"open_or_focus_application\",\"app\":\"$1\"}" | nc -U /tmp/casper.sock
}
```

Then use them:
```bash
casper-speak "Hello World"
casper-launch firefox
casper-ping
```

## Getting Help

- 📖 **Documentation**: See `README.md`, `ARCHITECTURE.md`, `NEXT_STEPS.md`
- 💬 **Examples**: Check `examples/` directory
- 🐛 **Issues**: Open an issue on GitHub
- 💡 **Ideas**: Check `CONTRIBUTING.md`

## Development Mode

For development with auto-rebuild:

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild daemon on changes
cd casper-daemon
cargo watch -x 'run'

# In another terminal, test
cd tests/daemon/client
cargo watch -x 'run'
```

## Resources

- **Gemini API**: https://aistudio.google.com/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial
- **Enigo Docs**: https://docs.rs/enigo/latest/enigo/

---

**You're all set! Welcome to your JARVIS journey! 🎉**

Start with simple tasks and gradually build up to more complex automation. Remember: Casper learns as you teach it!

Questions? Check the docs or open an issue. Happy automating! 🚀