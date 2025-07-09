use notify_rust::Notification;

pub fn show_notification(summary: &str, body: &str) -> Result<(), String> {
    Notification::new()
        .summary(summary)
        .body(body)
        .show()
        .map_err(|e| e.to_string())?;
    Ok(())
}