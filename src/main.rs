use daily_news::models::cache::Cache;
use daily_news::models::place::Place;
use std::io;
fn main() {
    // load cache from file or create new if not exists
    let mut cache = Cache::load();

    let cached_place = cache.retrieve_place();

    let mut need_input = false;

    // if no place found in cache, we request user input
    if !cached_place.is_some() {
        println!("No place found in cache");
        need_input = true;
    }

    // if place found in cache, we ask user if they want to use it
    if !need_input {
        let cached_place: Place = cached_place.unwrap();
        println!("Place found in cache: {}", cached_place.display_name);
        println!("Would you like to use the cached place? (Enter=yes/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() || !input.trim().eq_ignore_ascii_case("n") {
            println!("Using cached place: {}", cached_place.display_name);
        } else {
            // if user does not want to use cached place, we request user input
            need_input = true;
        }
    }

    if need_input {
        loop {
            println!("Please enter a city name or zip code: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let place_name = input.trim();
            let places = Place::query_by_name(place_name).unwrap();

            if !places.is_empty() {
                let place = Place::select(&places);

                cache.update_place(&place);
                break;
            } else {
                println!("No places found with that name. Please try again.");
            }
        }
    }

    // Now you can use name and place_id for weather and news queries
}
