use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents a single action that can be performed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    MoveMouse { x: i32, y: i32 },
    ClickMouse { button: String },
    MouseDown { button: String },
    MouseUp { button: String },
    Scroll { amount: i32, direction: String },
    TypeText { text: String },
    PressKey { key: String },
    KeyDown { key: String },
    KeyUp { key: String },
    RunCommand { command: String },
    Wait { milliseconds: u64 },
    LaunchApp { app_name: String },
    FocusWindow { window_pattern: String },
    ShowNotification { summary: String, body: String },
    Speak { text: String },
}

/// A sequence of actions that can be recorded and replayed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSequence {
    pub name: String,
    pub description: String,
    pub actions: Vec<ActionWithTimestamp>,
    pub created_at: String,
    pub tags: Vec<String>,
}

/// Action with timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWithTimestamp {
    pub action: Action,
    pub delay_ms: u64, // Delay before this action (from previous action)
}

impl ActionSequence {
    pub fn new(name: String, description: String) -> Self {
        ActionSequence {
            name,
            description,
            actions: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            tags: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Action, delay_ms: u64) {
        self.actions.push(ActionWithTimestamp { action, delay_ms });
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Failed to write file: {}", e))?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
        let sequence: ActionSequence =
            serde_json::from_str(&content).map_err(|e| format!("Failed to deserialize: {}", e))?;
        Ok(sequence)
    }
}

/// Recorder for capturing user actions
pub struct ActionRecorder {
    current_sequence: Option<ActionSequence>,
    is_recording: bool,
    last_action_time: Option<std::time::Instant>,
}

impl ActionRecorder {
    pub fn new() -> Self {
        ActionRecorder {
            current_sequence: None,
            is_recording: false,
            last_action_time: None,
        }
    }

    pub fn start_recording(&mut self, name: String, description: String) -> Result<(), String> {
        if self.is_recording {
            return Err("Already recording".to_string());
        }
        self.current_sequence = Some(ActionSequence::new(name, description));
        self.is_recording = true;
        self.last_action_time = Some(std::time::Instant::now());
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<ActionSequence, String> {
        if !self.is_recording {
            return Err("Not currently recording".to_string());
        }
        self.is_recording = false;
        self.last_action_time = None;
        self.current_sequence
            .take()
            .ok_or_else(|| "No sequence to save".to_string())
    }

    pub fn record_action(&mut self, action: Action) -> Result<(), String> {
        if !self.is_recording {
            return Err("Not currently recording".to_string());
        }

        let delay_ms = if let Some(last_time) = self.last_action_time {
            let now = std::time::Instant::now();
            let delay = now.duration_since(last_time);
            self.last_action_time = Some(now);
            delay.as_millis() as u64
        } else {
            0
        };

        if let Some(ref mut sequence) = self.current_sequence {
            sequence.add_action(action, delay_ms);
            Ok(())
        } else {
            Err("No active sequence".to_string())
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }
}

impl Default for ActionRecorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Player for replaying action sequences
pub struct ActionPlayer {
    current_sequence: Option<ActionSequence>,
    current_index: usize,
    is_playing: bool,
}

impl ActionPlayer {
    pub fn new() -> Self {
        ActionPlayer {
            current_sequence: None,
            current_index: 0,
            is_playing: false,
        }
    }

    pub fn load_sequence(&mut self, sequence: ActionSequence) {
        self.current_sequence = Some(sequence);
        self.current_index = 0;
        self.is_playing = false;
    }

    pub fn start_playback(&mut self) -> Result<(), String> {
        if self.current_sequence.is_none() {
            return Err("No sequence loaded".to_string());
        }
        self.is_playing = true;
        self.current_index = 0;
        Ok(())
    }

    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.current_index = 0;
    }

    pub fn next_action(&mut self) -> Option<&ActionWithTimestamp> {
        if !self.is_playing {
            return None;
        }

        if let Some(ref sequence) = self.current_sequence {
            if self.current_index < sequence.actions.len() {
                let action = &sequence.actions[self.current_index];
                self.current_index += 1;
                return Some(action);
            } else {
                self.is_playing = false;
            }
        }

        None
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn get_progress(&self) -> (usize, usize) {
        if let Some(ref sequence) = self.current_sequence {
            (self.current_index, sequence.actions.len())
        } else {
            (0, 0)
        }
    }
}

impl Default for ActionPlayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for storing and retrieving action sequences
pub struct ActionLibrary {
    sequences: Vec<ActionSequence>,
    library_path: String,
}

impl ActionLibrary {
    pub fn new(library_path: String) -> Self {
        ActionLibrary {
            sequences: Vec::new(),
            library_path,
        }
    }

    pub fn add_sequence(&mut self, sequence: ActionSequence) {
        self.sequences.push(sequence);
    }

    pub fn get_sequence(&self, name: &str) -> Option<&ActionSequence> {
        self.sequences.iter().find(|s| s.name == name)
    }

    pub fn list_sequences(&self) -> Vec<String> {
        self.sequences.iter().map(|s| s.name.clone()).collect()
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&ActionSequence> {
        self.sequences
            .iter()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .collect()
    }

    pub fn save_all(&self) -> Result<(), String> {
        let path = Path::new(&self.library_path);
        if !path.exists() {
            fs::create_dir_all(path).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        for sequence in &self.sequences {
            let file_name = format!("{}.json", sequence.name.replace(' ', "_"));
            let file_path = path.join(file_name);
            sequence.save_to_file(&file_path)?;
        }

        Ok(())
    }

    pub fn load_all(&mut self) -> Result<(), String> {
        let path = Path::new(&self.library_path);
        if !path.exists() {
            return Ok(()); // No library yet
        }

        let entries = fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;

        self.sequences.clear();

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match ActionSequence::load_from_file(&path) {
                    Ok(sequence) => self.sequences.push(sequence),
                    Err(e) => eprintln!("Failed to load sequence from {:?}: {}", path, e),
                }
            }
        }

        Ok(())
    }

    pub fn delete_sequence(&mut self, name: &str) -> Result<(), String> {
        self.sequences.retain(|s| s.name != name);

        let file_name = format!("{}.json", name.replace(' ', "_"));
        let file_path = Path::new(&self.library_path).join(file_name);

        if file_path.exists() {
            fs::remove_file(file_path).map_err(|e| format!("Failed to delete file: {}", e))?;
        }

        Ok(())
    }
}
