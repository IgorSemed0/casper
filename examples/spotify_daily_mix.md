# Example: Spotify Daily Mix Automation

This example demonstrates how Casper can automate opening Spotify and playing your Daily Mix playlist through a combination of process detection, window management, and screen interaction.

## The Goal

**User Command:** "Casper, select the daily playlist on Spotify"

**What Casper Does:**
1. Check if Spotify is running
2. If not, launch it and wait
3. Focus the Spotify window
4. Navigate to the Daily Mix playlist
5. Click to start playing
6. Confirm with voice feedback

## Implementation Approaches

### Approach 1: GUI Automation (Current Capability)

This approach uses screen control to interact with Spotify's interface:

```json
// Step 1: Check if Spotify is running
{
  "type": "is_process_running",
  "process": "spotify"
}

// Step 2: If not running, launch it
{
  "type": "launch_application",
  "app": "spotify"
}

// Step 3: Wait for application to start
{
  "type": "wait",
  "milliseconds": 2000
}

// Step 4: Focus Spotify window
{
  "type": "open_or_focus_application",
  "app": "spotify",
  "launch_command": "spotify"
}

// Step 5: Click on "Home" (if not already there)
// Coordinates depend on window size - this is where OCR/vision helps
{
  "type": "move_mouse",
  "x": 100,
  "y": 200
}

{
  "type": "click_mouse",
  "button": "left"
}

// Step 6: Wait for page to load
{
  "type": "wait",
  "milliseconds": 1000
}

// Step 7: Scroll down to find Daily Mix
{
  "type": "scroll",
  "amount": 3,
  "direction": "down"
}

// Step 8: Click on Daily Mix
// (Position found via OCR or learned from recording)
{
  "type": "move_mouse",
  "x": 400,
  "y": 500
}

{
  "type": "click_mouse",
  "button": "left"
}

// Step 9: Provide feedback
{
  "type": "speak",
  "text": "Playing your Daily Mix"
}

{
  "type": "show_notification",
  "summary": "Spotify",
  "body": "Now playing: Daily Mix"
}
```

### Approach 2: D-Bus/MPRIS Control (Recommended Future)

Spotify supports the MPRIS D-Bus interface for media control:

```bash
# Check if Spotify is running
dbus-send --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
  org.freedesktop.DBus.ListNames | grep spotify

# Control playback
dbus-send --print-reply --dest=org.mpris.MediaPlayer2.spotify \
  /org/mpris/MediaPlayer2 org.mpris.MediaPlayer2.Player.PlayPause

# Get current track
dbus-send --print-reply --dest=org.mpris.MediaPlayer2.spotify \
  /org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get \
  string:org.mpris.MediaPlayer2.Player string:Metadata
```

**Casper Integration:**
```rust
// casper-core/src/integrations/spotify.rs
pub fn spotify_play_pause() -> Result<(), String> {
    run_command("dbus-send --dest=org.mpris.MediaPlayer2.spotify ...")
}

pub fn spotify_get_current_track() -> Result<String, String> {
    // Parse D-Bus output
}
```

### Approach 3: Spotify API (For Advanced Features)

For playlist navigation, we'd need:
1. Spotify API credentials
2. OAuth authentication
3. Search for "Daily Mix" playlists
4. Start playback on specific device

```rust
// casper-core/src/integrations/spotify_api.rs
use reqwest::Client;

pub async fn search_daily_mix(token: &str) -> Result<Vec<Playlist>, String> {
    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .bearer_auth(token)
        .send()
        .await?;
    
    // Parse and filter for "Daily Mix"
    // ...
}

pub async fn play_playlist(token: &str, playlist_id: &str) -> Result<(), String> {
    // Start playback
    // ...
}
```

## Recording the Workflow

The best approach is to **record the actions once**, then replay them:

### Recording Session

```bash
# Start Casper daemon
cd casper-daemon && cargo run

# In another terminal, use a client to record
```

```json
// Start recording
{
  "type": "start_recording",
  "name": "spotify_daily_mix",
  "description": "Open Spotify and play Daily Mix playlist"
}

// Perform actions manually (Casper watches and records)
// 1. Launch Spotify
// 2. Wait for it to open
// 3. Click on Home
// 4. Scroll to Daily Mix
// 5. Click on playlist

// Stop recording
{
  "type": "stop_recording"
}
```

This creates: `~/.casper/actions/spotify_daily_mix.json`

### Replaying

```json
// Load the recorded sequence
{
  "type": "load_sequence",
  "name": "spotify_daily_mix"
}

// Play it back
{
  "type": "play_sequence"
}
```

## Making It Smarter with OCR (Phase 2)

Instead of hardcoded coordinates, use OCR to find "Daily Mix":

```rust
// Pseudo-code for smart playback
async fn play_spotify_daily_mix() -> Result<(), String> {
    // 1. Open/focus Spotify
    open_or_focus_application("spotify", Some("spotify"))?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // 2. Capture Spotify window
    let window = find_window_by_pattern("spotify")?;
    let screenshot = capture_window(&window.id)?;
    
    // 3. Use OCR to find "Daily Mix" text
    let text_positions = extract_text_positions(&screenshot)?;
    let daily_mix_pos = text_positions
        .iter()
        .find(|t| t.text.contains("Daily Mix"))
        .ok_or("Daily Mix not found on screen")?;
    
    // 4. Click on it
    click_at(daily_mix_pos.x, daily_mix_pos.y)?;
    
    // 5. Confirm
    speak("Playing your Daily Mix")?;
    
    Ok(())
}
```

## Natural Language Processing (Phase 3)

With AI/NLP, the command becomes flexible:

**User says any of these:**
- "Casper, play my daily mix"
- "Open Spotify and select the daily playlist"
- "I want to hear my Daily Mix on Spotify"
- "Put on my Spotify daily recommendations"

**Casper extracts:**
```rust
Intent {
    action: PlayMusic,
    target: "Spotify",
    playlist: "Daily Mix",
    method: Auto  // Figure out the best way
}
```

**Execution strategy:**
1. Try D-Bus control (fast, reliable)
2. If no D-Bus, try API (requires auth)
3. If no API, fall back to GUI automation
4. If all fail, report error

## Full Example Script

Here's a Python-like pseudocode showing the complete flow:

```python
async def spotify_daily_mix():
    # Parse command
    intent = parse_command("select the daily playlist on Spotify")
    
    # Check if Spotify is available
    if not is_process_running("spotify"):
        speak("Launching Spotify")
        launch_application("spotify")
        await wait_for_window("spotify", timeout=10)
    
    # Focus window
    focus_window("spotify")
    await sleep(1)
    
    # Try intelligent methods first
    try:
        # Method 1: API (if authenticated)
        if has_spotify_token():
            playlists = await spotify_api.search("Daily Mix")
            await spotify_api.play(playlists[0].id)
            speak("Playing your Daily Mix")
            return
    except Exception as e:
        log(f"API failed: {e}")
    
    try:
        # Method 2: D-Bus
        spotify_dbus.play_pause()
        speak("Playing your Daily Mix")
        return
    except Exception as e:
        log(f"D-Bus failed: {e}")
    
    # Method 3: GUI automation (fallback)
    try:
        # Use recorded sequence or OCR
        if action_library.has("spotify_daily_mix"):
            play_sequence("spotify_daily_mix")
        else:
            # Use OCR to find and click
            screenshot = capture_window_by_name("spotify")
            daily_mix_button = find_text_in_image(screenshot, "Daily Mix")
            click_at(daily_mix_button.x, daily_mix_button.y)
        
        speak("Playing your Daily Mix")
    except Exception as e:
        speak("Sorry, I couldn't find the Daily Mix playlist")
        show_notification("Error", str(e))
```

## Testing the Example

### Prerequisites

```bash
# Install Spotify
yay -S spotify

# Install dependencies for Casper
sudo pacman -S wmctrl xdotool espeak-ng

# Build Casper
cd casper
cargo build --workspace
```

### Manual Test

1. **Start Casper daemon:**
   ```bash
   cd casper-daemon
   cargo run
   ```

2. **In another terminal, send commands:**
   ```bash
   # Check if Spotify is running
   echo '{"type":"is_process_running","process":"spotify"}' | nc -U /tmp/casper.sock
   
   # Launch Spotify
   echo '{"type":"launch_application","app":"spotify"}' | nc -U /tmp/casper.sock
   
   # Wait a bit, then focus
   sleep 3
   echo '{"type":"focus_window","window":"Spotify"}' | nc -U /tmp/casper.sock
   ```

3. **Record your workflow:**
   ```bash
   # Start recording
   echo '{"type":"start_recording","name":"spotify_daily_mix","description":"Play Daily Mix"}' | nc -U /tmp/casper.sock
   
   # Now manually interact with Spotify while recording
   # (Future version will capture mouse/keyboard automatically)
   
   # Stop recording
   echo '{"type":"stop_recording"}' | nc -U /tmp/casper.sock
   ```

### Automated Test

Once recorded, replay anytime:

```bash
# Load sequence
echo '{"type":"load_sequence","name":"spotify_daily_mix"}' | nc -U /tmp/casper.sock

# Play it
echo '{"type":"play_sequence"}' | nc -U /tmp/casper.sock
```

## Next Steps

1. **Implement OCR** to find UI elements dynamically
2. **Add D-Bus integration** for Spotify control
3. **Create Spotify plugin** with API support
4. **Add voice command** "Hey Casper, play my daily mix"
5. **Smart retry logic** if elements not found
6. **Multi-monitor support** for coordinate translation

## Related Files

- `casper-core/src/window.rs` - Window management
- `casper-core/src/screen.rs` - Mouse/keyboard control
- `casper-core/src/actions.rs` - Recording/playback
- `casper-daemon/src/main.rs` - Request handling

## Contributing

If you implement Spotify integration, please:
1. Add it to `casper-core/src/integrations/spotify.rs`
2. Document the API usage
3. Handle authentication securely
4. Provide fallback methods
5. Submit a PR!

---

**Remember:** This is a learning project. Start simple, iterate, and gradually add intelligence. The goal is to make Casper useful TODAY while building towards JARVIS TOMORROW! 🚀