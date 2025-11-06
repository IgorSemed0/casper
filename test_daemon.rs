use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let command = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        r#"{"type":"ping"}"#.to_string()
    };

    let mut stream = UnixStream::connect("/tmp/casper.sock")
        .expect("Failed to connect to daemon");
    
    stream.write_all(command.as_bytes())
        .expect("Failed to send command");
    
    let mut response = String::new();
    stream.read_to_string(&mut response)
        .expect("Failed to read response");
    
    println!("{}", response);
}
