# Casper Architecture & Roadmap

## Vision: A True JARVIS for Linux

Casper aims to be a **proactive AI assistant** that doesn't just respond to commands, but actively helps you work by:
- **Understanding natural language** ("select the daily playlist on Spotify")
- **Interacting with GUI applications** (clicking, scrolling, navigating)
- **Learning from your actions** (recording and replaying tasks)
- **Detecting application states** (is Spotify open? where is it?)
- **Scheduling and automating tasks**
- **Speaking and listening** (voice I/O)

### Example Use Case: Spotify Daily Mix

```
You: "Casper, select the daily playlist on Spotify"

Casper's Process:
1. Parse command → Extract intent: "open Spotify" + "navigate to daily mix"
2. Check if Spotify is running → Process detection
3. If not running → Launch Spotify → Wait for window
4. If running but minimized → Focus window
5. Navigate to home screen → Click home button
6. Scroll to find "Daily Mix" → OCR/pattern matching
7. Click on playlist → Mouse click at coordinates
8. Confirm success → Speak: "Playing your Daily Mix"
```

---

## Current Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        Clients                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │   TUI    │  │   Tray   │  │   Web    │  │   Voice  │  │
│  │  (CLI)   │  │  (GUI)   │  │   API    │  │  Daemon  │  │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  │
└───────┼─────────────┼─────────────┼─────────────┼─────────┘
        │             │             │             │
        └─────────────┴─────────────┴─────────────┘
                      │ Unix Socket
                      │ (/tmp/casper.sock)
        ┌─────────────▼──────────────────────────┐
        │      Casper Daemon (Hub)               │
        │  - Request routing                     │
        │  - Session management                  │
        │  - State coordination                  │
        │  - Action recording/playback           │
        └─────────────┬──────────────────────────┘
                      │
        ┌─────────────▼──────────────────────────┐
        │       Casper Core (Library)            │
        │  ┌──────────────────────────────────┐  │
        │  │ Screen Control (mouse, keyboard) │  │
        │  ├──────────────────────────────────┤  │
        │  │ Window Management (focus, detect)│  │
        │  ├──────────────────────────────────┤  │
        │  │ Action Recording/Playback        │  │
        │  ├──────────────────────────────────┤  │
        │  │ AI/NLP (command understanding)   │  │
        │  ├──────────────────────────────────┤  │
        │  │ Voice I/O (recognition, TTS)     │  │
        │  ├──────────────────────────────────┤  │
        │  │ Task Scheduler                   │  │
        │  ├──────────────────────────────────┤  │
        │  │ Screen Reading (OCR, vision)     │  │
        │  └──────────────────────────────────┘  │
        └────────────────────────────────────────┘
```

### Crates Structure

```
casper/
├── casper-core/          # Core functionality library
│   ├── actions.rs        # ✅ Action recording/playback
│   ├── ai.rs             # 🚧 NLP & command understanding
│   ├── commands.rs       # ✅ Shell command execution
│   ├── connections.rs    # ✅ External service integration
│   ├── mcp.rs            # 🚧 Multi-Channel Protocol
│   ├── notifications.rs  # ✅ Desktop notifications
│   ├── scheduler.rs      # ❌ Task scheduling (TODO)
│   ├── screen.rs         # ✅ Mouse/keyboard control
│   ├── tts.rs            # ✅ Text-to-speech
│   ├── vision.rs         # ❌ OCR & image recognition (TODO)
│   ├── voice.rs          # 🚧 Voice recognition
│   └── window.rs         # ✅ Window/process management
├── casper-daemon/        # Background service
├── casper-tui/           # Terminal UI client
├── casper-tray/          # System tray client
└── casper-voice/         # ❌ Voice client (TODO)

Legend: ✅ Implemented | 🚧 Partial | ❌ Not Started
```

---

## Implementation Phases

### Phase 1: Enhanced Screen Control ✅ (CURRENT)

**Goal:** Complete mouse/keyboard control with window management

**Completed:**
- ✅ Mouse movement, clicking, scrolling
- ✅ Keyboard typing, key presses
- ✅ Window detection (is app running?)
- ✅ Window management (focus, maximize, minimize)
- ✅ Process detection and launching
- ✅ Action recording system

**Next Steps:**
1. Update daemon to handle new screen control commands
2. Add window management endpoints
3. Test with real applications (Spotify, Firefox, etc.)

---

### Phase 2: Screen Vision & Understanding (2-3 weeks)

**Goal:** Enable Casper to "see" and understand what's on screen

**Components to Add:**

#### 2.1 Screen Capture
```rust
// casper-core/src/vision.rs
pub fn capture_screen() -> Result<Image, String>;
pub fn capture_window(window_id: &str) -> Result<Image, String>;
pub fn capture_region(x: i32, y: i32, width: i32, height: i32) -> Result<Image, String>;
```

**Tools:**
- **Wayland:** `grim` (screenshot utility)
- **X11:** `scrot` or `import` (ImageMagick)

#### 2.2 OCR (Text Recognition)
```rust
pub fn extract_text(image: &Image) -> Result<String, String>;
pub fn find_text_position(text: &str) -> Result<(i32, i32), String>;
```

**Tools:**
- **tesseract-rs:** Rust bindings for Tesseract OCR
- Offline, no cloud needed

#### 2.3 Image Recognition (Find UI Elements)
```rust
pub fn find_image(template: &Image) -> Result<Vec<Match>, String>;
pub fn wait_for_image(template: &Image, timeout: Duration) -> Result<Match, String>;
```

**Approach:**
- Template matching for buttons/icons
- Feature detection (OpenCV)
- Consider: `opencv-rust` or simpler `image` crate

**Dependencies:**
```toml
[dependencies]
image = "0.24"
tesseract = "0.14"
# opencv = "0.88" # Optional, heavy dependency
```

---

### Phase 3: AI & Natural Language Processing (2-4 weeks)

**Goal:** Understand complex commands and make intelligent decisions

#### 3.1 Intent Recognition
```rust
// casper-core/src/ai.rs
pub struct Intent {
    pub action: ActionType,
    pub target: String,
    pub parameters: HashMap<String, String>,
}

pub fn parse_command(text: &str) -> Result<Intent, String>;
```

**Example Mappings:**
```
"select the daily playlist on Spotify" →
  Intent {
    action: Navigate,
    target: "Spotify",
    parameters: {
      "destination": "daily playlist",
      "method": "click"
    }
  }
```

#### 3.2 AI Options

**Option A: Local LLM (Recommended for Privacy)**
- **llama-cpp-rs:** Run LLaMA models locally
- **candle:** Rust ML framework (lightweight)
- Models: Phi-3, TinyLlama (for speed)

**Option B: Traditional NLP**
- **rust-bert:** For classification/NER
- Lighter weight, faster
- Pre-trained models for intent classification

**Option C: Hybrid**
- Simple keyword matching for common tasks
- LLM for complex/ambiguous commands
- Best balance of speed and capability

#### 3.3 Context & Memory
```rust
pub struct ConversationContext {
    pub history: Vec<Message>,
    pub current_app: Option<String>,
    pub last_action: Option<Action>,
}
```

Keep track of:
- What app is currently focused
- Last executed command
- User preferences
- Application-specific context

---

### Phase 4: Voice Integration (1-2 weeks)

**Goal:** Natural voice interaction

#### 4.1 Voice Recognition (Speech-to-Text)
```rust
// casper-core/src/voice.rs
pub fn start_listening() -> Result<AudioStream, String>;
pub fn recognize_speech(audio: &AudioData) -> Result<String, String>;
```

**Options:**
- **vosk-rs:** Offline speech recognition (recommended)
- **whisper-rs:** OpenAI Whisper (more accurate, heavier)
- **deepspeech-rs:** Mozilla DeepSpeech (deprecated but works)

**Wake Word Detection:**
- "Casper" or "Hey Casper"
- **porcupine-rust:** Lightweight wake word detection

#### 4.2 Voice Activity Detection (VAD)
- Detect when user starts/stops speaking
- Reduce false triggers
- **webrtc-vad:** Lightweight VAD

#### 4.3 Enhanced TTS
Current: espeak-ng (robotic)

**Better Options:**
- **piper-tts:** Neural TTS, sounds natural
- **coqui-tts:** High quality, configurable
- Keep espeak as fallback

---

### Phase 5: Learning & Task Automation (2-3 weeks)

**Goal:** Learn from user actions and automate repetitive tasks

#### 5.1 Action Recording (Already Implemented! ✅)
```rust
// Start recording
recorder.start_recording("Open Spotify Daily Mix", "Navigate to daily playlist");

// User performs actions manually
recorder.record_action(Action::LaunchApp { app_name: "spotify" });
recorder.record_action(Action::Wait { milliseconds: 2000 });
recorder.record_action(Action::ClickMouse { button: "left" });

// Save
let sequence = recorder.stop_recording()?;
sequence.save_to_file("~/.casper/actions/spotify_daily_mix.json")?;
```

#### 5.2 Smart Replay
```rust
pub fn replay_sequence(
    sequence: &ActionSequence,
    context: &Context,
) -> Result<(), String> {
    // Adapt to current screen state
    // Handle timing variations
    // Detect and recover from errors
}
```

**Challenges:**
- Screen resolution differences
- UI element positions change
- Application updates
- Network delays

**Solutions:**
- Use OCR to find elements dynamically
- Relative positioning
- Retry logic with timeouts
- Visual verification

#### 5.3 Task Scheduler
```rust
// casper-core/src/scheduler.rs
pub struct ScheduledTask {
    pub name: String,
    pub sequence: ActionSequence,
    pub schedule: Schedule, // Cron-like
    pub enabled: bool,
}

pub fn schedule_task(task: ScheduledTask) -> Result<(), String>;
pub fn list_scheduled_tasks() -> Vec<ScheduledTask>;
```

**Use Cases:**
- "Every day at 9am, open my email"
- "When I connect headphones, open Spotify"
- "If CPU > 80%, notify me"

**Dependencies:**
```toml
[dependencies]
tokio-cron-scheduler = "0.9"
```

---

### Phase 6: Application-Specific Integrations (Ongoing)

**Goal:** Deep integration with popular applications

#### 6.1 Spotify
- Use **librespot** or **spotifyd** for playback control
- D-Bus integration for MPRIS
- API for playlists/search
- GUI automation as fallback

#### 6.2 Browser (Firefox/Chrome)
- WebDriver protocol for control
- Extension for deeper integration
- Tab management, bookmarks
- Form filling

#### 6.3 Terminal
- tmux/screen integration
- Command suggestions
- Auto-completion

#### 6.4 File Manager
- Quick navigation
- File operations
- Search integration

---

## Technical Challenges & Solutions

### Challenge 1: Wayland Limitations

**Problem:** Wayland restricts many screen control operations for security

**Solutions:**
1. **Use Portals:** XDG Desktop Portals for screenshots
2. **Accessibility APIs:** AT-SPI for app control
3. **Compositor Extensions:** Gnome Shell extensions
4. **Fallback to X11:** XWayland for legacy apps

### Challenge 2: UI Element Detection

**Problem:** Finding "Daily Mix" button without hardcoded coordinates

**Solutions:**
1. **OCR:** Find text, click nearby
2. **Template Matching:** Store button images
3. **Accessibility Tree:** Use AT-SPI to query UI elements
4. **ML-based:** Train model to detect common UI patterns

### Challenge 3: Action Reliability

**Problem:** Recorded actions fail due to timing/UI changes

**Solutions:**
1. **Visual Verification:** Check screen before/after action
2. **Retry Logic:** Multiple attempts with exponential backoff
3. **Adaptive Timing:** Learn optimal delays
4. **Fallback Strategies:** Alternative paths to same goal

### Challenge 4: Performance

**Problem:** LLMs and OCR are slow

**Solutions:**
1. **Lazy Loading:** Load models only when needed
2. **Caching:** Cache OCR results for same screens
3. **Background Processing:** Use async for heavy ops
4. **GPU Acceleration:** Use CUDA/ROCm when available
5. **Simple First:** Try keyword matching before LLM

---

## Development Priorities

### Immediate (This Week)
1. ✅ Enhanced screen control (done!)
2. ⏳ Update daemon with new endpoints
3. ⏳ Test window management with real apps
4. ⏳ Create example action sequences

### Short Term (1 Month)
1. Screen capture & OCR integration
2. Basic image recognition
3. Improved AI command parsing
4. Voice recognition with Vosk
5. Action replay system

### Medium Term (2-3 Months)
1. Local LLM integration
2. Task scheduler
3. Application-specific plugins
4. Web interface
5. Mobile companion app

### Long Term (6+ Months)
1. Cross-platform support (other WMs)
2. Multi-user support
3. Cloud sync (optional)
4. Plugin marketplace
5. Visual programming interface

---

## System Requirements

### Minimum
- ArchLinux with Gnome/Wayland
- 8GB RAM (4GB for Casper, 4GB for OS)
- 4 CPU cores
- 2GB disk space

### Recommended
- 16GB RAM (for local LLM)
- 8 CPU cores or GPU
- SSD for fast model loading
- Microphone for voice input

### Dependencies
```bash
# Core
sudo pacman -S rust espeak-ng libnotify gtk4

# Window management
sudo pacman -S wmctrl xdotool

# Screen capture
sudo pacman -S grim slurp

# OCR
sudo pacman -S tesseract tesseract-data-eng

# Voice (optional)
sudo pacman -S portaudio
```

---

## Security & Privacy

### Principles
1. **Offline First:** All core features work without internet
2. **Local Processing:** No data sent to cloud by default
3. **User Control:** Explicit permission for sensitive operations
4. **Encrypted Storage:** Sensitive data encrypted at rest
5. **Audit Log:** Track all actions for transparency

### Permissions
- Screen recording (for vision)
- Input simulation (for control)
- Process monitoring (for detection)
- File system access (for storage)
- Microphone (for voice)

### Sensitive Operations
Require explicit confirmation:
- Running shell commands
- Accessing passwords
- Making purchases
- Sending emails/messages
- Deleting files

---

## Testing Strategy

### Unit Tests
- Each module has comprehensive tests
- Mock external dependencies
- Test error handling

### Integration Tests
- Test daemon-client communication
- Test action recording/playback
- Test window management

### End-to-End Tests
- Automated UI testing
- Real application scenarios
- Performance benchmarks

### User Testing
- Beta program for feedback
- Dogfooding (use Casper to develop Casper!)
- Community contributions

---

## Documentation

### User Docs
- Quick start guide
- Command examples
- Troubleshooting
- FAQ

### Developer Docs
- API reference (rustdoc)
- Architecture overview
- Plugin development guide
- Contributing guidelines

### Examples
- Common workflows
- Action sequence templates
- Integration examples

---

## Community & Contribution

### Open Source
- MIT/Apache 2.0 license
- GitHub for code hosting
- Discord/Matrix for chat
- Reddit/Discourse for discussions

### Contribution Areas
- Core development
- Plugin development
- Documentation
- Testing
- Translation
- Design

---

## Success Metrics

### Phase 1-2 (Foundation)
- ✅ All basic screen controls work
- ✅ Can detect and control windows
- ✅ Can record and replay simple actions

### Phase 3-4 (Intelligence)
- Can understand 80% of common commands
- Voice recognition accuracy > 90%
- Response time < 2 seconds

### Phase 5-6 (Automation)
- 10+ application integrations
- Can automate 50+ common workflows
- Task success rate > 95%

### Long Term (Adoption)
- 1000+ active users
- 100+ community plugins
- Featured in Linux magazines
- Other distros package Casper

---

## Next Steps

1. **Review this architecture** with the team
2. **Set up development environment** with all tools
3. **Update daemon** to support new screen control APIs
4. **Create first real-world demo:** "Open Spotify and play Daily Mix"
5. **Document the demo** with video for community
6. **Start Phase 2** (screen vision) planning

---

**Remember:** Start simple, iterate quickly, listen to users, and build something truly useful!

The goal is not to build the perfect AI assistant overnight, but to create a solid foundation that grows with your needs and the community's contributions.

**Let's make JARVIS a reality, one feature at a time! 🚀**