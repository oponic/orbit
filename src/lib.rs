use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::process::Command;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

pub fn keyid() -> Result<String, String> {
    let config_path = std::env::var("CONFIG")
        .map_err(|e| format!("Failed to get CONFIG environment variable: {}", e))?;
    let mut key_path = PathBuf::from(config_path);
    key_path.push("DRM");
    key_path.push("key.xml");
    if !key_path.exists() {
        return Err("Ingenuine copy".to_string()); // TODO: Change this to panic too after DRM stuff is done
    }
    // TODO: Implement XML parsing and platform key extraction
    Ok("".to_string())
}

pub fn jumpscare(message: &str, duration_ms: u32) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let script = format!(
            "Add-Type -AssemblyName System.Windows.Forms\n\
             $form = New-Object System.Windows.Forms.Form\n\
             $form.TopMost = $true\n\
             $form.FormBorderStyle = 'None'\n\
             $form.WindowState = 'Maximized'\n\
             $label = New-Object System.Windows.Forms.Label\n\
             $label.Text = '{}'\n\
             $label.AutoSize = $true\n\
             $label.Font = New-Object System.Drawing.Font('Arial', 72)\n\
             $label.ForeColor = 'Red'\n\
             $form.Controls.Add($label)\n\
             $form.Show()\n\
             Start-Sleep -Milliseconds {}\n\
             $form.Close()",
            message, duration_ms
        );

        let mut temp_file = std::env::temp_dir();
        temp_file.push("jumpscare.ps1");
        fs::write(&temp_file, script)
            .map_err(|e| format!("Failed to write temporary script: {}", e))?;

        Command::new("powershell")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(temp_file)
            .output()
            .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

        fs::remove_file(temp_file)
            .map_err(|e| format!("Failed to remove temporary script: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let script = format!(
            "#!/bin/bash\n\
             xterm -maximized -fa 'Arial' -fs 72 -fg red -T '' -e '\
             echo \"{}\"; sleep {:.3}'\n",
            message,
            duration_ms as f32 / 1000.0
        );

        let mut temp_file = std::env::temp_dir();
        temp_file.push("jumpscare.sh");
        fs::write(&temp_file, script)
            .map_err(|e| format!("Failed to write temporary script: {}", e))?;

        Command::new("chmod")
            .arg("+x")
            .arg(&temp_file)
            .output()
            .map_err(|e| format!("Failed to make script executable: {}", e))?;

        Command::new(&temp_file)
            .output()
            .map_err(|e| format!("Failed to execute shell script: {}", e))?;

        fs::remove_file(temp_file)
            .map_err(|e| format!("Failed to remove temporary script: {}", e))?;
    }

    Ok(())
}

pub fn update_plugins() -> Result<(), String> {
    let config_path = std::env::var("CONFIG")
        .map_err(|e| format!("Failed to get CONFIG environment variable: {}", e))?;
    let plugins_dir = PathBuf::from(config_path).join("plugins");

    if let Ok(entries) = fs::read_dir(&plugins_dir) {
        for entry in entries.flatten() {
            if !entry.path().is_dir() {
                continue;
            }

            let index_path = entry.path().join("index.xml");
            if !index_path.exists() {
                continue;
            }

            // Read and parse the index.xml file
            let xml_content = fs::read_to_string(&index_path)
                .map_err(|e| format!("Failed to read index.xml: {}", e))?;
            
            let mut reader = Reader::from_str(&xml_content);
            reader.trim_text(true);
            
            let mut buf = Vec::new();
            let mut current_element = String::new();
            let mut update_url = None;
            
            // Parse XML to find update URL
            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    },
                    Ok(Event::Text(ref e)) => {
                        if current_element == "update" {
                            update_url = Some(e.unescape().unwrap_or_default().to_string());
                        }
                    },
                    Ok(Event::Eof) => break,
                    Err(_) => break,
                    _ => {}
                }
            }

            // If we found an update URL, try to update the plugin
            if let Some(url) = update_url {
                // Check if there are updates available
                let status = Command::new("git")
                    .current_dir(&entry.path())
                    .args(["remote", "update"])
                    .output()
                    .map_err(|e| format!("Failed to check for updates: {}", e))?;

                if !status.status.success() {
                    // If the directory isn't a git repo yet, clone it
                    let plugin_name = entry.path().file_name()
                        .ok_or("Invalid plugin path")?
                        .to_string_lossy()
                        .to_string();
                    
                    let temp_dir = std::env::temp_dir().join(&plugin_name);
                    
                    // Clone to temporary directory
                    Command::new("git")
                        .args(["clone", &url, temp_dir.to_str().unwrap()])
                        .output()
                        .map_err(|e| format!("Failed to clone repository: {}", e))?;

                    // Remove old plugin directory
                    fs::remove_dir_all(&entry.path())
                        .map_err(|e| format!("Failed to remove old plugin: {}", e))?;

                    // Move new version to plugins directory
                    fs::rename(&temp_dir, &entry.path())
                        .map_err(|e| format!("Failed to move new plugin version: {}", e))?;
                } else {
                    // Check if we need to pull updates
                    let status = Command::new("git")
                        .current_dir(&entry.path())
                        .args(["status", "-uno"])
                        .output()
                        .map_err(|e| format!("Failed to check git status: {}", e))?;

                    if !String::from_utf8_lossy(&status.stdout).contains("Your branch is up to date") {
                        // Pull updates
                        Command::new("git")
                            .current_dir(&entry.path())
                            .args(["pull", "origin", "main"])
                            .output()
                            .map_err(|e| format!("Failed to pull updates: {}", e))?;
                    }
                }
            }
        }
    }

    Ok(())
}
