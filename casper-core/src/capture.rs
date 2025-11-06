use std::process::Command;

/// Screen capture utility for Wayland and X11
pub struct ScreenCapture {
    backend: CaptureBackend,
}

#[derive(Debug, Clone)]
enum CaptureBackend {
    Grim,   // Wayland (grim + slurp)
    Scrot,  // X11
    Import, // X11 (ImageMagick)
}

impl ScreenCapture {
    /// Create a new screen capture instance, auto-detecting the backend
    pub fn new() -> Result<Self, String> {
        let backend = Self::detect_backend()?;
        Ok(ScreenCapture { backend })
    }

    /// Detect which capture backend to use
    fn detect_backend() -> Result<CaptureBackend, String> {
        // Check if we're on Wayland
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // Try grim for Wayland
            if Command::new("which").arg("grim").output().is_ok() {
                return Ok(CaptureBackend::Grim);
            }
        }

        // Check for X11 tools
        if Command::new("which").arg("scrot").output().is_ok() {
            return Ok(CaptureBackend::Scrot);
        }

        if Command::new("which").arg("import").output().is_ok() {
            return Ok(CaptureBackend::Import);
        }

        Err(
            "No screenshot tool found. Install: grim (Wayland) or scrot/imagemagick (X11)"
                .to_string(),
        )
    }

    /// Capture the entire screen
    pub fn capture_screen(&self, output_path: &str) -> Result<(), String> {
        match self.backend {
            CaptureBackend::Grim => {
                let output = Command::new("grim")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute grim: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "grim failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Scrot => {
                let output = Command::new("scrot")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute scrot: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "scrot failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Import => {
                let output = Command::new("import")
                    .arg("-window")
                    .arg("root")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute import: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "import failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
        }
    }

    /// Capture a specific region of the screen
    pub fn capture_region(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        output_path: &str,
    ) -> Result<(), String> {
        match self.backend {
            CaptureBackend::Grim => {
                let geometry = format!("{},{} {}x{}", x, y, width, height);
                let output = Command::new("grim")
                    .arg("-g")
                    .arg(geometry)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute grim: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "grim failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Scrot => {
                let geometry = format!("{}x{}+{}+{}", width, height, x, y);
                let output = Command::new("scrot")
                    .arg("-a")
                    .arg(geometry)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute scrot: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "scrot failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Import => {
                let geometry = format!("{}x{}+{}+{}", width, height, x, y);
                let output = Command::new("import")
                    .arg("-window")
                    .arg("root")
                    .arg("-crop")
                    .arg(geometry)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute import: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "import failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
        }
    }

    /// Capture a specific window by its ID
    pub fn capture_window(&self, window_id: &str, output_path: &str) -> Result<(), String> {
        match self.backend {
            CaptureBackend::Grim => {
                // For grim, we need to get window geometry first using swaymsg or similar
                Err("Window capture with grim requires window geometry. Use capture_region instead.".to_string())
            }
            CaptureBackend::Scrot => {
                let output = Command::new("scrot")
                    .arg("-u")
                    .arg("-i")
                    .arg(window_id)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute scrot: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "scrot failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Import => {
                let output = Command::new("import")
                    .arg("-window")
                    .arg(window_id)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute import: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "import failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
        }
    }

    /// Capture the active window
    pub fn capture_active_window(&self, output_path: &str) -> Result<(), String> {
        match self.backend {
            CaptureBackend::Grim => {
                // For Wayland/grim, we need a different approach
                // This is a simplified version that captures the full screen
                // In a real implementation, you'd use compositor-specific commands
                self.capture_screen(output_path)
            }
            CaptureBackend::Scrot => {
                let output = Command::new("scrot")
                    .arg("-u")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute scrot: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "scrot failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Import => {
                // Get active window ID
                let xdotool_output = Command::new("xdotool")
                    .arg("getactivewindow")
                    .output()
                    .map_err(|e| format!("Failed to get active window: {}", e))?;

                if !xdotool_output.status.success() {
                    return Err("Failed to get active window ID".to_string());
                }

                let window_id = String::from_utf8_lossy(&xdotool_output.stdout)
                    .trim()
                    .to_string();

                self.capture_window(&window_id, output_path)
            }
        }
    }

    /// Capture to a temporary file and return the path
    pub fn capture_to_temp(&self) -> Result<String, String> {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let temp_path = temp_dir.join(format!("casper_screenshot_{}.png", timestamp));
        let temp_path_str = temp_path.to_str().ok_or("Invalid temp path")?;

        self.capture_screen(temp_path_str)?;

        Ok(temp_path_str.to_string())
    }

    /// Interactive region selection (for Wayland with slurp)
    pub fn select_region(&self, output_path: &str) -> Result<(), String> {
        match self.backend {
            CaptureBackend::Grim => {
                // Use slurp to select region, then grim to capture
                let slurp_output = Command::new("slurp")
                    .output()
                    .map_err(|e| format!("Failed to execute slurp: {}", e))?;

                if !slurp_output.status.success() {
                    return Err("Region selection cancelled or slurp not available".to_string());
                }

                let geometry = String::from_utf8_lossy(&slurp_output.stdout)
                    .trim()
                    .to_string();

                let output = Command::new("grim")
                    .arg("-g")
                    .arg(geometry)
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute grim: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "grim failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Scrot => {
                let output = Command::new("scrot")
                    .arg("-s")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute scrot: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "scrot failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            CaptureBackend::Import => {
                // Interactive selection is default for import without -window
                let output = Command::new("import")
                    .arg(output_path)
                    .output()
                    .map_err(|e| format!("Failed to execute import: {}", e))?;

                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "import failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
        }
    }
}

impl Default for ScreenCapture {
    fn default() -> Self {
        Self::new().expect("Failed to initialize screen capture")
    }
}

/// Convenience function to capture screen to a file
pub fn capture_screen(output_path: &str) -> Result<(), String> {
    let capture = ScreenCapture::new()?;
    capture.capture_screen(output_path)
}

/// Convenience function to capture region
pub fn capture_region(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    output_path: &str,
) -> Result<(), String> {
    let capture = ScreenCapture::new()?;
    capture.capture_region(x, y, width, height, output_path)
}

/// Convenience function to capture to temp file
pub fn capture_screen_temp() -> Result<String, String> {
    let capture = ScreenCapture::new()?;
    capture.capture_to_temp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_detection() {
        // This test will pass if at least one backend is available
        let result = ScreenCapture::detect_backend();
        // We can't assert success because it depends on the system
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_screen_capture_creation() {
        // Try to create a screen capture instance
        let result = ScreenCapture::new();
        // This might fail on systems without capture tools, which is okay
        let _ = result;
    }
}
