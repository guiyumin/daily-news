use crate::models::place::Place;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedPlace {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub place_id: Option<u32>,
    pub lat: Option<String>,
    pub lon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub user_id: String,
    pub place: CachedPlace,
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
        self.place.name = Some(place.name.clone());
        self.place.display_name = Some(place.display_name.clone());
        self.place.place_id = Some(place.place_id);
        self.place.lat = Some(place.lat.clone());
        self.place.lon = Some(place.lon.clone());
        self.save();
        println!("Cache updated: {}", place.display_name);
    }

    fn init() -> Self {
        Self {
            user_id: nanoid!(),
            place: CachedPlace {
                name: None,
                display_name: None,
                place_id: None,
                lat: None,
                lon: None,
            },
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
            &self.place.name,
            &self.place.place_id,
            &self.place.lat,
            &self.place.lon,
            &self.place.display_name,
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
