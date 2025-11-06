# Next Steps for Casper Development

## 🎯 Current Status

You now have a solid foundation for your JARVIS-like assistant! Here's what's been implemented:

### ✅ Completed (Just Now!)
- **Enhanced Screen Control**: Mouse clicking, scrolling, keyboard keys, position detection
- **Window Management**: Process detection, window focus, maximize/minimize, launch apps
- **Action Recording System**: Record and replay action sequences
- **Action Library**: Save/load sequences from `~/.casper/actions/`
- **Updated Daemon**: All new endpoints integrated
- **Architecture Document**: Complete roadmap for JARVIS vision

### 🚧 Existing (From Before)
- Basic mouse movement and text typing
- Command execution
- Notifications
- Text-to-speech (espeak-ng)
- Daemon-client architecture
- TUI interface

## 🚀 Immediate Action Items (This Week)

### 1. Build and Test New Features (Day 1-2)

```bash
# Build the updated workspace
cd ~/Documents/programming/casper
cargo build --workspace

# If you get errors, install dependencies:
sudo pacman -S wmctrl xdotool espeak-ng libnotify gtk4

# Start the daemon
cd casper-daemon
cargo run

# In another terminal, test the new features
cd tests/daemon/client
cargo run
```

**Create a simple test script** to verify everything works:

```bash
# Create test_new_features.sh
cat > test_new_features.sh << 'EOF'
#!/bin/bash

SOCK="/tmp/casper.sock"

echo "Testing new screen control features..."

# Test mouse click
echo '{"type":"click_mouse","button":"left"}' | nc -U $SOCK

# Test scroll
echo '{"type":"scroll","amount":3,"direction":"down"}' | nc -U $SOCK

# Test key press
echo '{"type":"press_key","key":"enter"}' | nc -U $SOCK

# Test window detection
echo '{"type":"is_process_running","process":"firefox"}' | nc -U $SOCK

# Test list windows
echo '{"type":"list_windows"}' | nc -U $SOCK

# Test mouse position
echo '{"type":"get_mouse_position"}' | nc -U $SOCK

echo "Tests complete!"
EOF

chmod +x test_new_features.sh
./test_new_features.sh
```

### 2. Create Your First Action Sequence (Day 2-3)

**Goal:** Record opening Firefox and navigating to a website

```bash
# Start recording
echo '{"type":"start_recording","name":"open_github","description":"Open Firefox and go to GitHub"}' | nc -U /tmp/casper.sock

# Now manually:
# 1. Open Firefox
echo '{"type":"launch_application","app":"firefox"}' | nc -U /tmp/casper.sock

# 2. Wait for it to open
sleep 3

# 3. Type URL (first focus address bar with Ctrl+L)
echo '{"type":"press_key","key":"ctrl"}' | nc -U /tmp/casper.sock
echo '{"type":"press_key","key":"l"}' | nc -U /tmp/casper.sock
echo '{"type":"type_text","text":"github.com"}' | nc -U /tmp/casper.sock
echo '{"type":"press_key","key":"enter"}' | nc -U /tmp/casper.sock

# 4. Stop recording
echo '{"type":"stop_recording"}' | nc -U /tmp/casper.sock

# 5. Check it was saved
ls ~/.casper/actions/

# 6. Replay it!
echo '{"type":"load_sequence","name":"open_github"}' | nc -U /tmp/casper.sock
echo '{"type":"play_sequence"}' | nc -U /tmp/casper.sock
```

### 3. Try the Spotify Example (Day 3-4)

Follow the guide in `examples/spotify_daily_mix.md`:

1. Install Spotify if you haven't
2. Start Casper daemon
3. Test basic Spotify detection:
   ```bash
   echo '{"type":"is_process_running","process":"spotify"}' | nc -U /tmp/casper.sock
   ```
4. Launch it:
   ```bash
   echo '{"type":"open_or_focus_application","app":"spotify"}' | nc -U /tmp/casper.sock
   ```
5. Record your workflow to Daily Mix
6. Replay it automatically!

### 4. Fix Any Build Issues (Ongoing)

Check for compilation errors:

```bash
cd ~/Documents/programming/casper
cargo check --workspace

# Common issues and fixes:
# 1. Missing chrono dependency - Already added!
# 2. Missing enigo features - Check Cargo.toml
# 3. Mutex/Arc issues - Already handled in daemon
```

## 📅 Week 1-2: Foundation Solidification

### Week 1: Core Testing & Bug Fixes
- [ ] Test all screen control functions on your system
- [ ] Verify window management works with Gnome/Wayland
- [ ] Create at least 5 action sequences for common tasks
- [ ] Document any issues or limitations
- [ ] Update README.md with new features

**Deliverable:** Working demo video showing:
1. Opening an app
2. Controlling it with mouse/keyboard
3. Recording an action sequence
4. Replaying it successfully

### Week 2: Enhanced TUI Client
- [ ] Update casper-tui to use new features
- [ ] Add menu for different command types
- [ ] Show list of saved action sequences
- [ ] Add recording mode UI
- [ ] Display current mouse position

**Example TUI improvements:**

```rust
// Add to casper-tui/src/main.rs
enum Mode {
    Command,      // Execute single commands
    Recording,    // Record action sequences
    Playback,     // Replay sequences
    WindowMgmt,   // Manage windows
}

// Show mode-specific help text
// Display recording status
// List available sequences
```

## 📅 Month 1: Screen Vision & OCR

### Goals
1. Capture screenshots (window or region)
2. Extract text with Tesseract OCR
3. Find UI elements by text
4. Make action playback adaptive

### Implementation Steps

**Week 3: Screen Capture**

```bash
# Add dependencies to casper-core/Cargo.toml
[dependencies]
image = "0.24"
```

Create `casper-core/src/vision.rs`:

```rust
use std::process::Command;
use image::DynamicImage;

pub fn capture_screen() -> Result<DynamicImage, String> {
    // Use grim for Wayland
    Command::new("grim")
        .arg("/tmp/casper_screenshot.png")
        .status()?;
    
    let img = image::open("/tmp/casper_screenshot.png")?;
    Ok(img)
}

pub fn capture_region(x: i32, y: i32, w: i32, h: i32) -> Result<DynamicImage, String> {
    let geometry = format!("{},{} {}x{}", x, y, w, h);
    Command::new("grim")
        .arg("-g")
        .arg(geometry)
        .arg("/tmp/casper_region.png")
        .status()?;
    
    let img = image::open("/tmp/casper_region.png")?;
    Ok(img)
}
```

**Week 4: OCR Integration**

```bash
# Install Tesseract
sudo pacman -S tesseract tesseract-data-eng

# Add to Cargo.toml
tesseract = "0.14"
```

Add to `casper-core/src/vision.rs`:

```rust
use tesseract::Tesseract;

pub struct TextMatch {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f32,
}

pub fn extract_text(image_path: &str) -> Result<String, String> {
    let tess = Tesseract::new(None, Some("eng"))?;
    let text = tess
        .set_image(image_path)?
        .get_text()?;
    Ok(text)
}

pub fn find_text_in_screen(search_text: &str) -> Result<Option<TextMatch>, String> {
    // 1. Capture screen
    capture_screen()?;
    
    // 2. OCR to find text
    let tess = Tesseract::new(None, Some("eng"))?;
    tess.set_image("/tmp/casper_screenshot.png")?;
    
    // 3. Get bounding boxes
    let boxes = tess.get_component_boxes()?;
    
    // 4. Find matching text
    for b in boxes {
        if b.text.contains(search_text) {
            return Ok(Some(TextMatch {
                text: b.text,
                x: b.x,
                y: b.y,
                width: b.w,
                height: b.h,
                confidence: b.confidence,
            }));
        }
    }
    
    Ok(None)
}
```

## 📅 Month 2: Voice Integration

### Week 5-6: Speech Recognition

```bash
# Install dependencies
sudo pacman -S portaudio

# Add to Cargo.toml
vosk = "0.3"
cpal = "0.15"  # For audio input
```

Update `casper-core/src/voice.rs`:

```rust
use vosk::{Model, Recognizer};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn recognize_voice() -> Result<String, String> {
    // Load Vosk model
    let model_path = "/usr/share/vosk/model";
    let model = Model::new(model_path)?;
    let mut recognizer = Recognizer::new(&model, 16000.0)?;
    
    // Capture audio from microphone
    let host = cpal::default_host();
    let device = host.default_input_device()?;
    
    // Process audio and return transcription
    // ... implementation details ...
    
    Ok(transcription)
}

pub fn start_listening_for_wake_word() {
    // Listen for "Hey Casper" or "Casper"
    // When detected, start full recognition
}
```

### Week 7-8: Better TTS

```bash
# Try piper-tts for better voice
yay -S piper-tts

# Or use festival
sudo pacman -S festival
```

Update `casper-core/src/tts.rs`:

```rust
pub fn speak(text: &str) -> Result<(), String> {
    speak_with_engine(text, TTSEngine::Piper)
}

pub enum TTSEngine {
    ESpeak,   // Current, robotic
    Piper,    // Neural, natural
    Festival, // Classic
}

pub fn speak_with_engine(text: &str, engine: TTSEngine) -> Result<(), String> {
    match engine {
        TTSEngine::Piper => {
            Command::new("piper")
                .arg("--model")
                .arg("/usr/share/piper/model")
                .arg("--output_file")
                .arg("-")
                .stdin(Stdio::piped())
                .spawn()?
                .stdin
                .unwrap()
                .write_all(text.as_bytes())?;
        }
        TTSEngine::ESpeak => {
            Command::new("espeak-ng").arg(text).spawn()?;
        }
        _ => return Err("Engine not implemented".to_string()),
    }
    Ok(())
}
```

## 📅 Month 3: AI & Natural Language

### Week 9-10: Intent Recognition

Start simple with keyword matching, then add NLP:

```rust
// casper-core/src/ai.rs

pub struct Intent {
    pub action: ActionType,
    pub target: Option<String>,
    pub parameters: HashMap<String, String>,
    pub confidence: f32,
}

pub enum ActionType {
    Launch,
    Open,
    Close,
    Navigate,
    Play,
    Pause,
    Search,
    Type,
    Click,
    Unknown,
}

pub fn parse_command(text: &str) -> Result<Intent, String> {
    let text_lower = text.to_lowercase();
    
    // Simple keyword matching first
    if text_lower.contains("open") || text_lower.contains("launch") {
        let target = extract_target(&text_lower);
        return Ok(Intent {
            action: ActionType::Open,
            target,
            parameters: HashMap::new(),
            confidence: 0.8,
        });
    }
    
    if text_lower.contains("play") && text_lower.contains("spotify") {
        // Extract playlist name
        let playlist = extract_playlist_name(&text_lower);
        return Ok(Intent {
            action: ActionType::Play,
            target: Some("spotify".to_string()),
            parameters: {
                let mut p = HashMap::new();
                p.insert("playlist".to_string(), playlist);
                p
            },
            confidence: 0.9,
        });
    }
    
    // For more complex commands, use LLM
    parse_with_llm(text)
}

fn extract_target(text: &str) -> Option<String> {
    // Extract application name after "open" or "launch"
    // "open firefox" -> Some("firefox")
    // "launch spotify" -> Some("spotify")
    
    let keywords = ["open", "launch", "start"];
    for keyword in keywords {
        if let Some(pos) = text.find(keyword) {
            let after = &text[pos + keyword.len()..].trim();
            if let Some(first_word) = after.split_whitespace().next() {
                return Some(first_word.to_string());
            }
        }
    }
    None
}
```

### Week 11-12: LLM Integration (Optional)

For complex queries, integrate a local LLM:

```bash
# Add to Cargo.toml
llm = "0.1"  # Or use llama-cpp-rs
```

```rust
pub fn parse_with_llm(text: &str) -> Result<Intent, String> {
    // Load model (do this once, not per request)
    let model = load_llm_model()?;
    
    // Create prompt
    let prompt = format!(
        "Extract the intent from this command: '{}'\n\
         Return JSON with: action, target, parameters",
        text
    );
    
    // Get response
    let response = model.generate(&prompt)?;
    
    // Parse JSON response into Intent
    let intent: Intent = serde_json::from_str(&response)?;
    Ok(intent)
}
```

## 🎯 Quick Wins (Do These Anytime)

### Documentation
- [ ] Add more examples to `examples/`
- [ ] Create video tutorials
- [ ] Write blog post about the project
- [ ] Update README with screenshots

### Testing
- [ ] Write unit tests for screen.rs
- [ ] Write unit tests for window.rs
- [ ] Write unit tests for actions.rs
- [ ] Add integration tests

### Developer Experience
- [ ] Add `cargo clippy` to CI
- [ ] Add `cargo fmt` checks
- [ ] Create development Docker container
- [ ] Add debug logging with `tracing`

### Community
- [ ] Post on Reddit r/rust, r/linux
- [ ] Share on HackerNews
- [ ] Create Discord server
- [ ] Set up discussions on GitHub

## 🛠️ Tools & Resources

### Essential Tools
```bash
# Development
sudo pacman -S rust-analyzer code  # VS Code with Rust

# Testing
sudo pacman -S wmctrl xdotool xwininfo xprop

# Screenshots
sudo pacman -S grim slurp wl-clipboard

# OCR
sudo pacman -S tesseract tesseract-data-eng

# Media
sudo pacman -S espeak-ng festival piper-tts

# Voice
sudo pacman -S portaudio pulseaudio
```

### Learning Resources
- **Rust Async**: https://tokio.rs/tokio/tutorial
- **Enigo (Input)**: https://docs.rs/enigo/latest/enigo/
- **Ratatui (TUI)**: https://ratatui.rs/
- **Tesseract OCR**: https://github.com/tesseract-ocr/tesseract
- **MPRIS/D-Bus**: https://specifications.freedesktop.org/mpris-spec/latest/

### Inspiration
- **Talon Voice**: Voice control for coding
- **Hammerspoon**: macOS automation
- **AutoHotkey**: Windows automation
- **i3wm**: Keyboard-driven window manager

## 🐛 Known Issues to Fix

1. **Action Recording** - Currently manual, needs to auto-capture mouse/keyboard
2. **Wayland Limitations** - Some operations need XWayland or portal
3. **Timing** - Fixed delays aren't reliable, need adaptive waiting
4. **Error Handling** - More graceful failures needed
5. **Multi-monitor** - Coordinates need screen awareness

## 📊 Success Metrics

Track your progress:

```bash
# Create a progress tracker
cat > PROGRESS.md << 'EOF'
# Casper Development Progress

## Week 1
- [ ] Built project successfully
- [ ] Tested all new features
- [ ] Created 3 action sequences
- [ ] Demo video recorded

## Week 2
- [ ] Enhanced TUI client
- [ ] Fixed bugs: ___
- [ ] Added features: ___

## Week 3
- [ ] Screen capture working
- [ ] OCR integrated
- [ ] Found UI element by text

(Continue weekly...)
EOF
```

## 🎉 First Milestone Goal

**By end of Month 1, you should have:**

1. ✅ Casper running stably on your system
2. ✅ 10+ recorded action sequences
3. ✅ OCR finding UI elements
4. ✅ Spotify Daily Mix automation working
5. ✅ A demo video showing it all

**Then you can say:**
> "I have a working JARVIS that can control my computer with voice commands, understand what's on screen, and learn from my actions!"

## 🚀 Remember

- **Start simple**: Don't try to implement everything at once
- **Test frequently**: Run your code after every change
- **Document as you go**: Future you will thank present you
- **Share your progress**: Community feedback is valuable
- **Have fun**: This is YOUR assistant, make it useful for YOUR needs!

## 📞 Need Help?

1. Check `ARCHITECTURE.md` for design decisions
2. Look at `examples/` for usage patterns
3. Read the code - it's well-commented
4. Open an issue on GitHub
5. Join the Discord (create it!)

---

**NOW GO BUILD YOUR JARVIS! 🤖✨**

Start with step 1 (build and test), then tackle the Spotify example. That will give you a complete working demo to show off and build upon.

Good luck, and may your assistant be ever helpful! 🎯