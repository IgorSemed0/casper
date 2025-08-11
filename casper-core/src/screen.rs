use enigo::{Enigo, Settings, Coordinate, Mouse, Keyboard};

pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.move_mouse(x, y, Coordinate::Abs).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn type_text(text: &str) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.fast_text(text).map_err(|e| e.to_string())?;
    Ok(())
}