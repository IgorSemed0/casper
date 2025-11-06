use std::process::Command;

/// Check if a process is running by name
pub fn is_process_running(process_name: &str) -> Result<bool, String> {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg(process_name)
        .output()
        .map_err(|e| format!("Failed to execute pgrep: {}", e))?;

    Ok(output.status.success())
}

/// Get list of running processes matching a pattern
pub fn find_processes(pattern: &str) -> Result<Vec<String>, String> {
    let output = Command::new("pgrep")
        .arg("-f")
        .arg(pattern)
        .output()
        .map_err(|e| format!("Failed to execute pgrep: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let pids: Vec<String> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();
        Ok(pids)
    } else {
        Ok(Vec::new())
    }
}

/// Launch an application
pub fn launch_application(app_name: &str) -> Result<(), String> {
    Command::new(app_name)
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", app_name, e))?;
    Ok(())
}

/// Focus a window by application name (using wmctrl)
pub fn focus_window(app_name: &str) -> Result<(), String> {
    let output = Command::new("wmctrl")
        .arg("-a")
        .arg(app_name)
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to focus window: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Get list of all windows with their properties
pub fn list_windows() -> Result<Vec<WindowInfo>, String> {
    let output = Command::new("wmctrl")
        .arg("-l")
        .arg("-p")
        .arg("-x")
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "wmctrl failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut windows = Vec::new();

    for line in stdout.lines() {
        if let Some(window_info) = parse_wmctrl_line(line) {
            windows.push(window_info);
        }
    }

    Ok(windows)
}

/// Get active window information (using xdotool or gdbus for Wayland)
pub fn get_active_window() -> Result<WindowInfo, String> {
    // Try gdbus first for Wayland/Gnome
    if let Ok(window) = get_active_window_gdbus() {
        return Ok(window);
    }

    // Fallback to xdotool for X11
    get_active_window_xdotool()
}

fn get_active_window_gdbus() -> Result<WindowInfo, String> {
    let output = Command::new("gdbus")
        .args(&[
            "call",
            "--session",
            "--dest",
            "org.gnome.Shell",
            "--object-path",
            "/org/gnome/Shell",
            "--method",
            "org.gnome.Shell.Eval",
            "global.display.focus_window.get_wm_class()",
        ])
        .output()
        .map_err(|e| format!("Failed to execute gdbus: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse the output to extract window class
        // Format is usually: (true, '"ClassName"')
        if let Some(class) = extract_window_class(&stdout) {
            return Ok(WindowInfo {
                id: String::from("0"),
                pid: 0,
                desktop: 0,
                class: class.clone(),
                title: class,
                machine: String::from("localhost"),
            });
        }
    }

    Err("Failed to get active window via gdbus".to_string())
}

fn get_active_window_xdotool() -> Result<WindowInfo, String> {
    let output = Command::new("xdotool")
        .args(&["getactivewindow", "getwindowname"])
        .output()
        .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(WindowInfo {
            id: String::from("0"),
            pid: 0,
            desktop: 0,
            class: String::new(),
            title,
            machine: String::from("localhost"),
        })
    } else {
        Err("Failed to get active window via xdotool".to_string())
    }
}

fn extract_window_class(gdbus_output: &str) -> Option<String> {
    // Extract class from gdbus output: (true, '"ClassName"')
    if let Some(start) = gdbus_output.find('"') {
        if let Some(end) = gdbus_output[start + 1..].find('"') {
            return Some(gdbus_output[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

/// Maximize a window
pub fn maximize_window(window_id: &str) -> Result<(), String> {
    let output = Command::new("wmctrl")
        .args(&[
            "-i",
            "-r",
            window_id,
            "-b",
            "add,maximized_vert,maximized_horz",
        ])
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to maximize window: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Minimize a window
pub fn minimize_window(window_id: &str) -> Result<(), String> {
    let output = Command::new("wmctrl")
        .args(&["-i", "-r", window_id, "-b", "add,hidden"])
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to minimize window: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Close a window
pub fn close_window(window_id: &str) -> Result<(), String> {
    let output = Command::new("wmctrl")
        .args(&["-i", "-c", window_id])
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to close window: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Move and resize a window
pub fn move_resize_window(
    window_id: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), String> {
    let geometry = format!("0,{},{},{},{}", x, y, width, height);
    let output = Command::new("wmctrl")
        .args(&["-i", "-r", window_id, "-e", &geometry])
        .output()
        .map_err(|e| format!("Failed to execute wmctrl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to move/resize window: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Window information structure
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: String,
    pub pid: u32,
    pub desktop: i32,
    pub class: String,
    pub title: String,
    pub machine: String,
}

fn parse_wmctrl_line(line: &str) -> Option<WindowInfo> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }

    let id = parts[0].to_string();
    let desktop = parts[1].parse::<i32>().unwrap_or(-1);
    let pid = parts[2].parse::<u32>().unwrap_or(0);
    let class = parts[3].to_string();
    let machine = parts[4].to_string();

    // The title is the rest of the line after the first 5 parts
    let title = if parts.len() > 5 {
        parts[5..].join(" ")
    } else {
        String::new()
    };

    Some(WindowInfo {
        id,
        pid,
        desktop,
        class,
        title,
        machine,
    })
}

/// Check if an application window is visible/open
pub fn is_application_visible(app_pattern: &str) -> Result<bool, String> {
    let windows = list_windows()?;
    Ok(windows.iter().any(|w| {
        w.class.to_lowercase().contains(&app_pattern.to_lowercase())
            || w.title.to_lowercase().contains(&app_pattern.to_lowercase())
    }))
}

/// Find window ID by application name or title pattern
pub fn find_window_by_pattern(pattern: &str) -> Result<Option<WindowInfo>, String> {
    let windows = list_windows()?;
    let pattern_lower = pattern.to_lowercase();

    Ok(windows.into_iter().find(|w| {
        w.class.to_lowercase().contains(&pattern_lower)
            || w.title.to_lowercase().contains(&pattern_lower)
    }))
}

/// Open or focus an application
pub fn open_or_focus_application(
    app_name: &str,
    launch_command: Option<&str>,
) -> Result<(), String> {
    // First, check if the application is already running and visible
    if let Ok(Some(window)) = find_window_by_pattern(app_name) {
        // Application is already open, just focus it
        focus_window(&window.title)?;
        return Ok(());
    }

    // Check if process is running but no window is visible
    if is_process_running(app_name)? {
        // Process exists, try to focus by name
        if focus_window(app_name).is_ok() {
            return Ok(());
        }
    }

    // Application is not running, launch it
    let cmd = launch_command.unwrap_or(app_name);
    launch_application(cmd)?;

    // Wait a bit for the application to start
    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(())
}
