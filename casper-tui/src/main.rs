use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use std::io::{self, Stdout};

struct App {
    input: String,
    output: String,
}

impl App {
    fn new() -> Self {
        App {
            input: String::new(),
            output: String::new(),
        }
    }
}

async fn send_request(request: &str) -> Result<String, String> {
    let mut stream = UnixStream::connect("/tmp/casper.sock")
        .await
        .map_err(|e| e.to_string())?;
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run TUI
    let mut app = App::new();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                    .split(f.size());

                let input_block = Block::default().title("Input").borders(Borders::ALL);
                let input = Paragraph::new(app.input.as_str()).block(input_block);
                f.render_widget(input, chunks[0]);

                let output_block = Block::default().title("Output").borders(Borders::ALL);
                let output = Paragraph::new(app.output.as_str()).block(output_block);
                f.render_widget(output, chunks[1]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => app.input.pop(),
                    KeyCode::Enter => {
                        let request = json!({
                            "type": "run_command",
                            "command": app.input.clone()
                        });
                        app.output = match send_request(&request.to_string()).await {
                            Ok(resp) => resp,
                            Err(e) => format!("Error: {}", e),
                        };
                        app.input.clear();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        Ok::<(), io::Error>(())
    })?;

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}