# AI Vision with Gemini - Usage Guide

This guide shows how to use Casper's AI vision capabilities powered by Google Gemini to understand what's on screen and interact intelligently with applications.

## Why AI Vision Instead of OCR?

Traditional OCR is limited:
- ❌ Only extracts text, doesn't understand context
- ❌ Struggles with non-standard fonts or overlays
- ❌ Can't identify icons, buttons, or visual elements
- ❌ Doesn't understand layout or hierarchy

AI Vision (Gemini) is powerful:
- ✅ Understands entire UI context
- ✅ Identifies elements by description ("the blue play button")
- ✅ Understands layout and relationships
- ✅ Can suggest actions based on current state
- ✅ Works with any language or visual style

## Setup

### 1. Get a Gemini API Key

1. Go to [Google AI Studio](https://aistudio.google.com/app/apikey)
2. Create a new API key
3. Copy it to your `.env` file:

```bash
# Edit .env
AI_REQUEST_URL=https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent
AI_TOKEN=YOUR_API_KEY_HERE
AI_MODEL=gemini-2.0-flash-exp
```

### 2. Install Screenshot Tools

```bash
# For Wayland (Gnome)
sudo pacman -S grim slurp

# For X11
sudo pacman -S scrot
# OR
sudo pacman -S imagemagick
```

## Basic Usage Examples

### Example 1: Describe What's On Screen

```rust
use casper_core::capture::capture_screen_temp;
use casper_core::ai_vision::AIVision;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Capture current screen
    let screenshot_path = capture_screen_temp()?;
    
    // Initialize AI vision
    let ai_vision = AIVision::from_env()?;
    
    // Ask AI to describe what it sees
    let description = ai_vision.describe_screen(&screenshot_path).await?;
    
    println!("AI sees: {}", description);
    
    Ok(())
}
```

**Example Output:**
```
AI sees: The screen shows a Spotify desktop application. The main content area 
displays "Daily Mix" playlists with album artwork. The left sidebar shows 
navigation options including Home, Search, and Your Library. The playback 
controls are at the bottom with a progress bar.
```

### Example 2: Find a Specific Element

```rust
use casper_core::capture::capture_screen_temp;
use casper_core::ai_vision::AIVision;

#[tokio::main]
async fn main() -> Result<(), String> {
    let screenshot_path = capture_screen_temp()?;
    let ai_vision = AIVision::from_env()?;
    
    // Find the "Daily Mix 1" playlist
    match ai_vision.find_element(&screenshot_path, "Daily Mix 1 playlist").await? {
        Some(element) => {
            println!("Found at position: ({}, {})", element.x, element.y);
            println!("Size: {}x{}", element.width, element.height);
            println!("Confidence: {}%", element.confidence);
            
            // Now you can click it!
            // click_mouse_at(element.x + element.width/2, element.y + element.height/2)?;
        }
        None => {
            println!("Element not found on screen");
        }
    }
    
    Ok(())
}
```

### Example 3: Get Action Suggestions

```rust
use casper_core::capture::capture_screen_temp;
use casper_core::ai_vision::AIVision;

#[tokio::main]
async fn main() -> Result<(), String> {
    let screenshot_path = capture_screen_temp()?;
    let ai_vision = AIVision::from_env()?;
    
    // Tell AI what you want to do
    let task = "play my Daily Mix on Spotify";
    let steps = ai_vision.suggest_actions(&screenshot_path, task).await?;
    
    println!("To {}, you should:", task);
    for (i, step) in steps.iter().enumerate() {
        println!("{}. {}", i + 1, step);
    }
    
    Ok(())
}
```

**Example Output:**
```
To play my Daily Mix on Spotify, you should:
1. Click on the "Home" icon in the left sidebar to ensure you're on the home screen
2. Scroll down to find the "Daily Mix" section
3. Click on "Daily Mix 1" playlist thumbnail
4. Click the green "Play" button at the top of the playlist
```

### Example 4: Check If Element Is Visible

```rust
use casper_core::capture::capture_screen_temp;
use casper_core::ai_vision::AIVision;

#[tokio::main]
async fn main() -> Result<(), String> {
    let screenshot_path = capture_screen_temp()?;
    let ai_vision = AIVision::from_env()?;
    
    // Check if Spotify is showing
    let has_spotify = ai_vision
        .is_element_visible(&screenshot_path, "Spotify logo")
        .await?;
    
    if has_spotify {
        println!("Spotify is open!");
    } else {
        println!("Spotify is not visible");
    }
    
    Ok(())
}
```

## Real-World Workflow: Spotify Daily Mix

Here's a complete example that uses AI vision to play your Daily Mix on Spotify:

```rust
use casper_core::{
    capture::capture_screen_temp,
    ai_vision::AIVision,
    window::open_or_focus_application,
    screen::{move_mouse, click_mouse},
    tts::speak,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("🎵 Opening Spotify...");
    
    // 1. Open or focus Spotify
    open_or_focus_application("spotify", Some("spotify"))?;
    sleep(Duration::from_secs(2)).await;
    
    // 2. Capture screen
    let screenshot = capture_screen_temp()?;
    
    // 3. Initialize AI vision
    let ai = AIVision::from_env()?;
    
    // 4. Check if we're on the right screen
    println!("🔍 Checking current screen...");
    let description = ai.describe_screen(&screenshot).await?;
    println!("AI sees: {}", description);
    
    // 5. Find Daily Mix
    println!("🎯 Looking for Daily Mix...");
    match ai.find_element(&screenshot, "Daily Mix playlist").await? {
        Some(element) => {
            println!("✅ Found Daily Mix at ({}, {})", element.x, element.y);
            
            // 6. Click on it
            let click_x = element.x + element.width / 2;
            let click_y = element.y + element.height / 2;
            
            move_mouse(click_x, click_y)?;
            sleep(Duration::from_millis(200)).await;
            click_mouse("left")?;
            
            // 7. Confirm
            speak("Playing your Daily Mix")?;
            println!("🎶 Done!");
        }
        None => {
            println!("❌ Daily Mix not found on current screen");
            
            // Get AI suggestions for what to do
            let suggestions = ai.suggest_actions(
                &screenshot, 
                "navigate to Daily Mix playlists"
            ).await?;
            
            println!("💡 AI suggests:");
            for step in suggestions {
                println!("   - {}", step);
            }
        }
    }
    
    Ok(())
}
```

## Advanced: Context-Aware Commands

You can use AI vision to make commands context-aware:

```rust
use casper_core::{
    capture::capture_screen_temp,
    ai_vision::AIVision,
};

async fn smart_command(user_command: &str) -> Result<(), String> {
    // Capture what's currently on screen
    let screenshot = capture_screen_temp()?;
    let ai = AIVision::from_env()?;
    
    // Build a context-aware prompt
    let prompt = format!(
        "I'm looking at this screenshot. The user wants to: '{}'\n\
         Based on what you see, should I:\n\
         A) Execute the command directly\n\
         B) First navigate somewhere\n\
         C) Open a different application\n\
         D) Tell the user it's not possible\n\
         \n\
         Respond with just the letter and a brief explanation.",
        user_command
    );
    
    let response = ai.analyze_screenshot(&screenshot, &prompt).await?;
    println!("AI decision: {}", response);
    
    // Parse response and take appropriate action
    // ... implementation ...
    
    Ok(())
}
```

## Daemon Integration

Add AI vision endpoints to the daemon:

```json
// Capture and analyze current screen
{
  "type": "analyze_screen",
  "prompt": "What application is currently open?"
}

// Find element
{
  "type": "find_element",
  "description": "the blue play button"
}

// Get suggestions
{
  "type": "suggest_actions",
  "task": "play my favorite playlist"
}

// Check visibility
{
  "type": "is_visible",
  "element": "Spotify window"
}
```

## Tips & Best Practices

### 1. Be Specific in Descriptions

❌ Bad: "the button"  
✅ Good: "the green play button in the center"

❌ Bad: "text"  
✅ Good: "the 'Daily Mix 1' text below the album artwork"

### 2. Capture Relevant Regions

Instead of full screen, capture specific windows or regions for better accuracy:

```rust
use casper_core::capture::capture_region;

// Capture just the Spotify window area
capture_region(100, 100, 800, 600, "/tmp/spotify_window.png")?;
```

### 3. Add Retry Logic

AI responses can vary, so add retries:

```rust
async fn find_element_with_retry(
    ai: &AIVision,
    screenshot: &str,
    description: &str,
    max_attempts: u32,
) -> Result<Option<ElementPosition>, String> {
    for attempt in 1..=max_attempts {
        match ai.find_element(screenshot, description).await {
            Ok(Some(element)) if element.confidence > 70 => {
                return Ok(Some(element));
            }
            Ok(Some(element)) => {
                println!("Low confidence ({}%), retrying...", element.confidence);
            }
            Ok(None) => {
                println!("Not found, retrying...");
            }
            Err(e) => {
                println!("Error on attempt {}: {}", attempt, e);
            }
        }
        
        if attempt < max_attempts {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    
    Ok(None)
}
```

### 4. Cache Screenshots

Don't capture the screen for every request:

```rust
// Capture once
let screenshot = capture_screen_temp()?;

// Use for multiple queries
let description = ai.describe_screen(&screenshot).await?;
let has_element = ai.is_element_visible(&screenshot, "button").await?;
let suggestions = ai.suggest_actions(&screenshot, "task").await?;
```

### 5. Handle Rate Limits

Gemini API has rate limits. Add delays between requests:

```rust
use tokio::time::{sleep, Duration};

// Wait between API calls
sleep(Duration::from_millis(500)).await;
```

## Cost Considerations

Gemini API pricing (as of 2024):
- **Free tier**: 15 requests per minute
- **Paid tier**: $0.000125 per image

For a typical session:
- 1 screenshot analysis = ~$0.0001
- 100 analyses per day = ~$0.01/day = ~$3/year

Much cheaper than cloud OCR or other vision APIs!

## Troubleshooting

### "AI_TOKEN not set in environment"

Make sure you:
1. Created the `.env` file (copy from `.env.example`)
2. Added your Gemini API key
3. The daemon loads the `.env` file (use `dotenv::dotenv()`)

### "Failed to capture screen"

Install screenshot tools:
```bash
# Wayland
sudo pacman -S grim slurp

# X11
sudo pacman -S scrot
```

### "API error 400"

Check your API key is valid and not expired.

### "API error 429"

You've hit the rate limit. Wait a minute or upgrade your API plan.

### AI Returns Wrong Coordinates

Gemini's coordinate estimation can be approximate. Tips:
1. Use higher resolution screenshots
2. Capture smaller regions (window instead of full screen)
3. Add visual landmarks in your prompt
4. Verify with visual confirmation before clicking

## Next Steps

1. **Add to daemon**: Integrate these functions into casper-daemon
2. **Voice commands**: Combine with voice recognition for "Hey Casper, click the blue button"
3. **Learning**: Record AI's decisions to build a knowledge base
4. **Multi-step workflows**: Chain multiple AI vision queries for complex tasks

## Related Files

- `casper-core/src/ai_vision.rs` - AI vision implementation
- `casper-core/src/capture.rs` - Screen capture utilities
- `.env.example` - Configuration template
- `examples/spotify_daily_mix.md` - Complete workflow example

---

**The power of AI vision is that Casper doesn't just automate clicks—it understands what it's doing!** 🚀