use crate::utils::alphabet::ALPHANUMERIC;
use nanoid::nanoid;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct Cfg {
    pub user_id: String,
    pub dir: PathBuf,
}

fn get_dir() -> PathBuf {
    let path = if cfg!(windows) {
        // On Windows, use %APPDATA%/dailynews (typically C:\Users\Username\AppData\Roaming\dailynews)
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dailynews")
    } else {
        // On Unix-like systems (Linux/macOS/BSD), use ~/.dailynews
        dirs::home_dir()
            .map(|h| h.join(".dailynews"))
            .unwrap_or_else(|| PathBuf::from("."))
    };

    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| {
                eprintln!("Failed to create directory {}: {}", path.display(), e);
            })
            .ok();
    }
    path
}

fn get_user_id() -> String {
    let mut path = get_dir();
    path.push("cfg.json");

    // Read existing file or create new ID
    let user_id = fs::read_to_string(&path)
        .map_err(|e| serde_json::Error::io(e))
        .and_then(|content| {
            serde_json::from_str::<serde_json::Value>(&content)
                .map(|json| json["user_id"].as_str().unwrap_or("").to_string())
        })
        .unwrap_or_else(|_| {
            // Create new ID and save to file
            let id = nanoid!(12, &ALPHANUMERIC);
            let json = serde_json::json!({ "user_id": id });
            fs::write(&path, serde_json::to_string_pretty(&json).unwrap())
                .map_err(|e| eprintln!("Failed to write config file: {}", e))
                .ok();
            id
        });

    user_id
}

// TODO: make this a singleton
impl Cfg {
    pub fn new() -> Self {
        Self {
            user_id: get_user_id(),
            dir: get_dir(),
        }
    }
}
