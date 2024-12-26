use serde::{Deserialize, Serialize};
use reqwest;
use nanoid::nanoid;
use crate::utils::alphabet::ALPHANUMERIC;
use std::io::{self, Write};
use crate::models::cache::Cache;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Place {
    pub place_id: u32,
    pub display_name: String,
    pub name: String,
    pub lat: String,
    pub lon: String,
}

impl Place {

    pub fn get_place_name(cache: &Cache) -> Option<(String, u32)> {
        if let Some(last_display_name) = &cache.last_display_name {
            println!("Last used place was '{}'.", last_display_name);
            print!("Press Enter to use it again, or type a new place: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                return Some((
                    cache.last_place.as_ref().unwrap().clone(),
                    cache.last_place_id.unwrap()
                ));
            }
            None
        } else {
            print!("Enter a place name: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            None
        }
    }

    pub fn query_by_name(name: &str) -> Result<Vec<Place>, reqwest::Error> {
        let url = format!("https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2", name);
     
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", format!("DailyNews/1.0 User/{}", nanoid!(12, &ALPHANUMERIC)))
            .send()?;

        let places: Vec<Place> = response.json()?;
        Ok(places)
    }

   
    pub fn select(places: &[Place]) -> Option<&Place> {
        println!("\nFound multiple places. Please select one:");
        
        for (i, place) in places.iter().enumerate() {
            println!("{}. {}", i + 1, place.display_name);
        }

        print!("\nEnter number (1-{}): ", places.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(selection) = input.trim().parse::<usize>() {
            if selection > 0 && selection <= places.len() {
                return Some(&places[selection - 1]);
            }
        }
        
        println!("Invalid selection. Please try again.");
        None
    }

    pub fn get_valid_place(cache: &mut Cache) -> Option<Place> {
        loop {
            let place_name = match Self::get_place_name(cache) {
                Some((name, _)) => name,
                None => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    input.trim().to_string()
                }
            };

            match Self::query_by_name(&place_name) {
                Ok(places) => {
                    if places.is_empty() {
                        println!("No places found with that name. Please try again.");
                        continue;
                    }

                    let selected_place = if places.len() == 1 {
                        places[0].clone()
                    } else {
                        match Self::select(&places) {
                            Some(place) => place.clone(),
                            None => continue,
                        }
                    };

                    return Some(selected_place);
                }
                Err(e) => {
                    println!("Error searching for place: {}. Please try again.", e);
                    continue;
                }
            }
        }
    }

    pub fn get_place_info_from_cache(cache: &Cache) -> Option<(String, u32)> {
        if let Some(cached_place) = cache.to_place() {
            println!("Using cached place: {}", cached_place.display_name);
            print!("Would you like to use the cached place? (Enter=yes/n): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            if input.trim().is_empty() || !input.trim().eq_ignore_ascii_case("n") {
                return Some((cached_place.name, cached_place.place_id));
            }
        }
        None
    }
}

