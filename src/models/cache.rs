use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::models::place::Place;

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
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("daily-news");
        fs::create_dir_all(&path).unwrap_or_default();
        path.push("cache.json");
        path
    }

    pub fn to_place(&self) -> Option<Place> {
        match (&self.last_place, &self.last_place_id, &self.last_lat, &self.last_lon, &self.last_display_name) {
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