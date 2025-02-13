use crate::models::cfg::Cfg;
use crate::models::place::Place;
use crate::models::weather::Weather;
use crate::utils::alphabet::ALPHANUMERIC;
use colored::Colorize;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
    pub user_id: String,
    pub place: Place,
    pub weather: Weather,
}

impl Cache {
    fn new() -> Self {
        Self {
            user_id: nanoid!(12, &ALPHANUMERIC),
            place: Place::default(),
            weather: Weather::default(),
        }
    }

    pub fn load() -> Self {
        let cache_file = Self::get_cache_path();

        if cache_file.exists() {
            let contents = fs::read_to_string(&cache_file).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_else(|_| Self::new())
        } else {
            let cache = Self::new();
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

    pub fn update_place(&mut self, place: &Place) {
        self.place.name = place.name.clone();
        self.place.display_name = place.display_name.clone();
        self.place.place_id = place.place_id;
        self.place.lat = place.lat.clone();
        self.place.lon = place.lon.clone();
        self.save();
        println!(
            "{}: \n{}",
            "Cache updated".bright_green(),
            place.display_name.as_deref().unwrap_or("").bright_blue()
        );
    }

    fn get_cache_path() -> PathBuf {
        let mut path = Cfg::new().dir;
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
                place_id: Some(*id),
                name: Some(name.clone()),
                display_name: Some(display_name.clone()),
                lat: Some(lat.clone()),
                lon: Some(lon.clone()),
            }),
            _ => None,
        }
    }
}
