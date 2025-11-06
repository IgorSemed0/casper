use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

async fn send_request(request: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect("/tmp/casper.sock").await?;
    stream.write_all(request.as_bytes()).await?;
    let mut buf = vec![0; 4096];
    let n = stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Casper v0.2.0 Comprehensive Test Suite");
    println!("{}", "=".repeat(60));

    // Original features
    println!("\n📦 Testing Original Features:");
    println!("{}", "-".repeat(60));

    let basic_tests = vec![
        r#"{"type": "run_command", "command": "echo Hello, World!"}"#,
        r#"{"type": "move_mouse", "x": 100, "y": 200}"#,
        r#"{"type": "type_text", "text": "Hello from Casper"}"#,
        r#"{"type": "show_notification", "summary": "Test", "body": "Hello from Casper!"}"#,
        r#"{"type": "connect_to_service", "service": "example_api", "action": "get"}"#,
        r#"{"type": "process_mcp", "data": "test"}"#,
        r#"{"type": "process_command", "command": "hello"}"#,
        r#"{"type": "recognize_voice"}"#,
        r#"{"type": "speak", "text": "Hello, this is Casper speaking!"}"#,
    ];

    for request in &basic_tests {
        let response = send_request(request).await?;
        println!("Request: {}\nResponse: {}\n", request, response);
    }

    // New v0.2.0 features
    println!("\n✨ Testing New v0.2.0 Features:");
    println!("{}", "-".repeat(60));

    let new_tests = vec![
        // Daemon status
        ("Ping", r#"{"type":"ping"}"#),
        // Enhanced screen control
        ("Get Mouse Position", r#"{"type":"get_mouse_position"}"#),
        ("Click Mouse", r#"{"type":"click_mouse","button":"left"}"#),
        ("Mouse Down", r#"{"type":"mouse_down","button":"left"}"#),
        ("Mouse Up", r#"{"type":"mouse_up","button":"left"}"#),
        (
            "Scroll",
            r#"{"type":"scroll","amount":3,"direction":"down"}"#,
        ),
        ("Press Key", r#"{"type":"press_key","key":"escape"}"#),
        ("Key Down", r#"{"type":"key_down","key":"shift"}"#),
        ("Key Up", r#"{"type":"key_up","key":"shift"}"#),
        // Window management
        (
            "Is Process Running",
            r#"{"type":"is_process_running","process":"systemd"}"#,
        ),
        (
            "Is Application Visible",
            r#"{"type":"is_application_visible","app":"terminal"}"#,
        ),
        ("List Windows", r#"{"type":"list_windows"}"#),
        ("Find Window", r#"{"type":"find_window","pattern":"zed"}"#),
    ];

    for (name, request) in &new_tests {
        println!("\n🔹 Testing: {}", name);
        let response = send_request(request).await?;
        println!("   Response: {}", response);
    }

    // Action recording tests
    println!("\n\n🎬 Testing Action Recording:");
    println!("{}", "-".repeat(60));

    println!("\n▶️  Starting recording...");
    let response = send_request(r#"{"type":"start_recording","name":"test_sequence_v2","description":"Testing v0.2.0 recording"}"#).await?;
    println!("   Response: {}", response);

    println!("\n▶️  Checking recording status...");
    let response = send_request(r#"{"type":"is_recording"}"#).await?;
    println!("   Response: {}", response);

    println!("\n▶️  Recording some actions...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    send_request(r#"{"type":"move_mouse","x":500,"y":500}"#).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    send_request(r#"{"type":"click_mouse","button":"left"}"#).await?;
    println!("   Recorded 2 actions");

    println!("\n▶️  Stopping recording...");
    let response = send_request(r#"{"type":"stop_recording"}"#).await?;
    println!("   Response: {}", response);

    println!("\n▶️  Listing saved sequences...");
    let response = send_request(r#"{"type":"list_sequences"}"#).await?;
    println!("   Response: {}", response);

    println!("\n{}", "=".repeat(60));
    println!("✅ All tests completed!");
    println!("\n💡 Check ~/.casper/actions/ for saved sequences");
    println!("📖 See examples/ directory for real-world workflows");

    Ok(())
}
