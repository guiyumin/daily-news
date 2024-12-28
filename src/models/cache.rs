use crate::models::place::Place;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub user_id: String,
    pub last_place: Option<String>,
    pub last_display_name: Option<String>,
    pub last_place_id: Option<u32>,
    pub last_lat: Option<String>,
    pub last_lon: Option<String>,
}

impl Cache {
    pub fn load() -> Self {
        let cache_file = Self::get_cache_path();

        if cache_file.exists() {
            let contents = fs::read_to_string(&cache_file).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_else(|_| Self::init())
        } else {
            let cache = Self::init();
            cache.save();
            cache
        }
    }

    pub fn save(&self) {
        let cache_file = Self::get_cache_path();
        if let Ok(contents) = serde_json::to_string_pretty(self) {
            let _ = fs::write(cache_file, contents);
        }
    }

    pub fn update(&mut self, place: &Place) {
        self.last_place = Some(place.name.clone());
        self.last_display_name = Some(place.display_name.clone());
        self.last_place_id = Some(place.place_id);
        self.last_lat = Some(place.lat.clone());
        self.last_lon = Some(place.lon.clone());
        self.save();
        println!("Cache updated: {}", place.display_name);
    }

    fn init() -> Self {
        Self {
            user_id: nanoid!(),
            last_place: None,
            last_display_name: None,
            last_place_id: None,
            last_lat: None,
            last_lon: None,
        }
    }

    fn get_cache_path() -> PathBuf {
        let base_path = if cfg!(windows) {
            // On Windows, use %APPDATA% (typically C:\Users\Username\AppData\Roaming)
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else {
            // On Unix-like systems (Linux/macOS), use ~/.dailynews
            dirs::home_dir()
                .map(|h| h.join(".dailynews"))
                .unwrap_or_else(|| PathBuf::from("."))
        };

        let mut path = base_path;
        if cfg!(windows) {
            path.push("dailynews"); // On Windows: AppData\Roaming\dailynews\
        }
        fs::create_dir_all(&path).unwrap_or_default();
        path.push("cache.json");
        path
    }

    pub fn retrieve_place(&self) -> Option<Place> {
        match (
            &self.last_place,
            &self.last_place_id,
            &self.last_lat,
            &self.last_lon,
            &self.last_display_name,
        ) {
            (Some(name), Some(id), Some(lat), Some(lon), Some(display_name)) => Some(Place {
                place_id: *id,
                name: name.clone(),
                display_name: display_name.clone(),
                lat: lat.clone(),
                lon: lon.clone(),
            }),
            _ => None,
        }
    }
}
