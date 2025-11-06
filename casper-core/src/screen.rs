use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};

pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo
        .move_mouse(x, y, Coordinate::Abs)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn click_mouse(button: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let btn = match button {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => return Err(format!("Unknown button: {}", button)),
    };

    enigo
        .button(btn, enigo::Direction::Click)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn mouse_down(button: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let btn = match button {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => return Err(format!("Unknown button: {}", button)),
    };

    enigo
        .button(btn, Direction::Press)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn mouse_up(button: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let btn = match button {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => return Err(format!("Unknown button: {}", button)),
    };

    enigo
        .button(btn, Direction::Release)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn scroll(amount: i32, direction: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    match direction {
        "up" | "down" => {
            let scroll_amount = if direction == "down" { -amount } else { amount };
            enigo
                .scroll(scroll_amount, enigo::Axis::Vertical)
                .map_err(|e| e.to_string())?;
        }
        "left" | "right" => {
            let scroll_amount = if direction == "left" { -amount } else { amount };
            enigo
                .scroll(scroll_amount, enigo::Axis::Horizontal)
                .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Unknown scroll direction: {}", direction)),
    }

    Ok(())
}

pub fn type_text(text: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.fast_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn press_key(key: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let k = parse_key(key)?;
    enigo.key(k, Direction::Click).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn key_down(key: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let k = parse_key(key)?;
    enigo.key(k, Direction::Press).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn key_up(key: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;

    let k = parse_key(key)?;
    enigo
        .key(k, Direction::Release)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn parse_key(key: &str) -> Result<Key, String> {
    match key.to_lowercase().as_str() {
        "return" | "enter" => Ok(Key::Return),
        "escape" | "esc" => Ok(Key::Escape),
        "backspace" => Ok(Key::Backspace),
        "tab" => Ok(Key::Tab),
        "space" => Ok(Key::Space),
        "delete" | "del" => Ok(Key::Delete),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" => Ok(Key::PageUp),
        "pagedown" => Ok(Key::PageDown),
        "left" | "leftarrow" => Ok(Key::LeftArrow),
        "right" | "rightarrow" => Ok(Key::RightArrow),
        "up" | "uparrow" => Ok(Key::UpArrow),
        "down" | "downarrow" => Ok(Key::DownArrow),
        "shift" => Ok(Key::Shift),
        "control" | "ctrl" => Ok(Key::Control),
        "alt" => Ok(Key::Alt),
        "meta" | "super" | "windows" | "command" => Ok(Key::Meta),
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),
        _ => Err(format!("Unknown key: {}", key)),
    }
}

pub fn get_mouse_position() -> Result<(i32, i32), String> {
    let settings = Settings::default();
    let enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    let (x, y) = enigo.location().map_err(|e| e.to_string())?;
    Ok((x, y))
}
