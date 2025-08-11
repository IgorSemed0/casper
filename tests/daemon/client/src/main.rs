use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn send_request(request: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect("/tmp/casper.sock").await?;
    stream.write_all(request.as_bytes()).await?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tests = vec![
        r#"{"type": "run_command", "command": "echo Hello, World!"}"#,
        r#"{"type": "move_mouse", "x": 100, "y": 200}"#,
        r#"{"type": "type_text", "text": "Hello from Casper"}"#,
        r#"{"type": "show_notification", "summary": "Test", "body": "Hello from Casper!"}"#,
        r#"{"type": "connect_to_service", "service": "example_api", "action": "get"}"#,
        r#"{"type": "process_mcp", "data": "test"}"#,
        r#"{"type": "process_command", "command": "hello"}"#,
        r#"{"type": "recognize_voice"}"#,
        r#"{"type": "speak", "text": "Hello, noah, how are you? this is Casper speaking, is evething ok with you?"}"#,
    ];

    for request in tests {
        let response = send_request(request).await?;
        println!("Request: {}\nResponse: {}\n", request, response);
    }

    Ok(())
}