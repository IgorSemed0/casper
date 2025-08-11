use std::process::Command;

pub fn speak(text: &str) -> Result<(), String> {
    Command::new("espeak-ng")
        .arg(text)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}