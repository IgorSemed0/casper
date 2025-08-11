pub fn process_command(command: &str) -> Result<String, String> {
    // Basic keyword matcching, thinking about using use rust-bert, I got interesred º-º
    if command.contains("hello") {
        Ok("I'm an AI response º-º!".to_string())
    } else {
        Err("AI under construction".to_string())
    }
}