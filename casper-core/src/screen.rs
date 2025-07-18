use enigo::{Enigo, Settings, Coordinate, Mouse};

pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).map_err(|e| e.to_string())?;
    enigo.move_mouse(x, y, Coordinate::Abs).map_err(|e| e.to_string())?;
    Ok(())
}