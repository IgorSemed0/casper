# Changelog

All notable changes to Casper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-01-XX - "The Vision Update"

This is a major release that transforms Casper from a basic automation tool into a true JARVIS-like assistant with AI vision capabilities, comprehensive screen control, and intelligent action recording.

### Added

#### AI & Vision
- **AI Vision Module** (`ai_vision.rs`) - Gemini API integration for screen understanding
  - Analyze screenshots and describe what's on screen
  - Find UI elements by natural language description ("the blue play button")
  - Get element coordinates with confidence scores
  - Suggest actions based on current screen state
  - Check if specific elements are visible
- **Environment Configuration** - Flexible AI provider setup via `.env`
  - Support for Google Gemini, OpenAI, Anthropic, and local LLMs
  - Configurable: `AI_REQUEST_URL`, `AI_TOKEN`, `AI_MODEL`
  - Optional settings for temperature, max tokens, timeout
- **Screen Capture Module** (`capture.rs`) - Multi-backend screenshot support
  - Wayland support with `grim` and `slurp`
  - X11 support with `scrot` and ImageMagick `import`
  - Auto-detect display server and available tools
  - Capture full screen, regions, windows, or active window
  - Interactive region selection
  - Temporary file capture for AI processing

#### Screen Control Enhancements
- **Mouse Control** - Complete mouse interaction capabilities
  - Click (left, right, middle buttons)
  - Mouse press/release for drag operations
  - Scroll (vertical and horizontal)
  - Get current mouse position
- **Keyboard Control** - Full keyboard simulation
  - Press individual keys
  - Key down/up for combinations (Ctrl+C, etc.)
  - Support for special keys (Enter, Escape, Arrows, F-keys)
  - Support for modifier keys (Ctrl, Alt, Shift, Meta/Super)

#### Window & Process Management
- **Window Management Module** (`window.rs`) - Comprehensive window control
  - Detect if processes are running (`pgrep`)
  - Launch applications
  - Focus, maximize, minimize, close windows (`wmctrl`)
  - Move and resize windows with pixel precision
  - List all windows with detailed properties
  - Find windows by name or pattern matching
  - Get active window (Wayland via `gdbus`, X11 via `xdotool`)
  - Smart `open_or_focus` that checks if app is already running

#### Action Recording & Automation
- **Action Recording System** (`actions.rs`) - Learn and replay tasks
  - Record sequences of user actions with timing information
  - Save/load action sequences as JSON files
  - Action library manager for organizing sequences
  - Support for all action types (mouse, keyboard, window, app launch)
  - Tag and search sequences by category
  - Playback with preserved timing
  - Action library stored in `~/.casper/actions/`

#### Daemon Improvements
- **30+ New Endpoints** - Comprehensive API for all features
  - Screen control: `click_mouse`, `scroll`, `press_key`, `get_mouse_position`
  - Window management: `is_process_running`, `launch_application`, `focus_window`, `list_windows`, `find_window`, `maximize_window`, `minimize_window`, `close_window`
  - Action recording: `start_recording`, `stop_recording`, `record_action`, `is_recording`
  - Action playback: `load_sequence`, `play_sequence`, `list_sequences`, `delete_sequence`
  - Status: `ping` endpoint for health checks
- **State Management** - Daemon maintains session state
  - Action recorder state
  - Action player state  
  - Action library loaded from disk on startup
  - Thread-safe state access with proper locking
- **Larger Buffer** - Increased to 4096 bytes for complex requests
- **Better Error Handling** - Detailed JSON error responses

#### Documentation
- **ARCHITECTURE.md** - Complete technical design and roadmap
  - System architecture with component diagrams
  - 6-phase implementation roadmap
  - Technical challenges and solutions
  - Security and privacy considerations
- **NEXT_STEPS.md** - Actionable development guide
  - Weekly task breakdown
  - Month-by-month development plan
  - Code examples for each feature
  - Success metrics and milestones
- **QUICKSTART.md** - Get running in 5 minutes
  - Step-by-step installation
  - Quick tests to verify functionality
  - Common issues and solutions
  - Helpful shell aliases
- **AI Vision Usage Guide** (`examples/ai_vision_usage.md`)
  - Complete AI vision tutorial
  - Real-world examples with code
  - Best practices and tips
  - Troubleshooting guide
- **Spotify Daily Mix Example** (`examples/spotify_daily_mix.md`)
  - Complete workflow demonstration
  - Multiple implementation approaches
  - Recording and playback instructions
- **Updated README.md** - Comprehensive project overview
  - Current features and status
  - Usage examples
  - API documentation

### Changed
- **casper-core/Cargo.toml** - Added dependencies
  - `chrono` for timestamps
  - `dotenv` for environment configuration
  - `base64` for image encoding
- **Project Structure** - Better organization
  - New `examples/` directory for tutorials
  - Separate modules for distinct functionality

### Fixed
- Removed unused imports from `ai_vision.rs` and `capture.rs`
- Fixed borrow checker issues in daemon state management

### Dependencies
- `chrono = "0.4"` - Timestamp management
- `dotenv = "0.15"` - Environment configuration
- `base64 = "0.21"` - Image encoding for AI APIs
- Existing: `enigo`, `notify-rust`, `tokio`, `serde_json`, `reqwest`, `ratatui`, `crossterm`

### Breaking Changes
None - This is the first major release with these features. All existing functionality remains backward compatible.

---

## [0.1.0] - Initial Release

### Added
- Basic daemon-client architecture via Unix sockets
- Simple screen control (mouse movement, text typing)
- Command execution
- Desktop notifications
- Text-to-speech with espeak-ng
- Terminal UI (TUI) client with ratatui
- System tray client (basic)
- Basic AI module (keyword matching)
- Voice recognition placeholder
- External service connections via HTTP

---

## Roadmap

### [0.3.0] - "The Voice Update" (Planned)
- Voice recognition with Vosk
- Wake word detection
- Better TTS with Piper or Coqui
- Voice activity detection
- Voice command processing

### [0.4.0] - "The Intelligence Update" (Planned)  
- Enhanced AI with local LLM support
- Natural language command understanding
- Context-aware command processing
- Conversation memory
- Multi-step task planning

### [0.5.0] - "The Automation Update" (Planned)
- Task scheduler with cron-like syntax
- Trigger-based automation (on event X, do Y)
- Application-specific plugins (Spotify, Firefox, etc.)
- Visual workflow builder
- Smart action replay with adaptation

### [1.0.0] - "JARVIS Release" (Future)
- Complete JARVIS-like capabilities
- Multi-monitor support
- Remote control via mobile app
- Plugin marketplace
- Cross-platform support (X11, other WMs)
- Production-ready stability

---

## Contributing

We welcome contributions! Each commit should:
1. Focus on a single feature or fix
2. Include descriptive commit messages
3. Follow conventional commits format (`feat:`, `fix:`, `docs:`, etc.)
4. Update relevant documentation
5. Include tests where appropriate

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## Git Workflow

This project follows a clean git history with:
- One feature per commit
- Descriptive commit messages
- Logical grouping of changes
- Documentation commits separate from feature commits

Recent commits:
```
* 6ab25ff docs: add AI vision usage guide and quick start guide
* 9137929 fix: remove unused imports from ai_vision and capture modules
* 1c716b5 docs: comprehensive documentation for JARVIS vision
* 20daaab feat: enhance daemon with 30+ new endpoints
* ffb310a feat: add action recording and playback system
* 55a1436 feat: add window and process management module
* 05d6639 feat: enhance screen control with comprehensive mouse and keyboard actions
* ee37c5c feat: add AI vision module with Gemini API integration
* ebf79a6 feat: add screen capture module
* c50ba2a feat: add environment configuration for AI providers
```

---

**For detailed installation and usage instructions, see [QUICKSTART.md](QUICKSTART.md)**

**For the complete technical roadmap, see [ARCHITECTURE.md](ARCHITECTURE.md)**

**For development guidance, see [NEXT_STEPS.md](NEXT_STEPS.md)**