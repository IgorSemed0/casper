use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect("/tmp/casper.sock").await?;
    let request = r#"{"type": "run_command", "command": "echo Hello, World!"}"#;
    stream.write_all(request.as_bytes()).await?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("{}", response);
    Ok(())
}