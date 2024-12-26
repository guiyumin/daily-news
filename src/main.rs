use daily_news::models::place::Place;
use daily_news::models::cache::Cache;
use std::io::{self, Write};

fn get_place_input(cache: &Cache) -> String {
    if let Some(last_display_name) = &cache.last_display_name {
        println!("Last used place was '{}'.", last_display_name);
        print!("Press Enter to use it again, or type a new place: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            return cache.last_place.as_ref().unwrap().clone();
        }
        input.to_string()
    } else {
        print!("Enter a place name: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

fn select_place(places: &[Place]) -> Option<&Place> {
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

fn get_valid_place(cache: &mut Cache) -> Option<Place> {
    loop {
        let place_name = get_place_input(cache);

        match Place::get_by_name(&place_name) {
            Ok(places) => {
                if places.is_empty() {
                    println!("No places found with that name. Please try again.");
                    continue;
                }

                let selected_place = if places.len() == 1 {
                    places[0].clone()
                } else {
                    match select_place(&places) {
                        Some(place) => place.clone(),
                        None => continue,  // If selection was invalid, try again
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

fn main() {
    let mut cache = Cache::load();
    
    // If we have a cached place, try to use that first
    if let Some(cached_place) = cache.to_place() {
        println!("Using cached place: {}", cached_place.display_name);
        print!("Would you like to use the cached place? (Enter=yes/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().is_empty() || !input.trim().eq_ignore_ascii_case("n") {
            return; // Use cached place and exit
        }
    }

    match get_valid_place(&mut cache) {
        Some(selected_place) => {
            println!("\nSelected: {}", selected_place.display_name);
            
            // Update cache with all place information
            cache.last_place = Some(selected_place.name);
            cache.last_display_name = Some(selected_place.display_name);
            cache.last_place_id = Some(selected_place.place_id);
            cache.last_lat = Some(selected_place.lat);
            cache.last_lon = Some(selected_place.lon);
            cache.save();
        }
        None => println!("No place selected. Exiting."),
    }
}
