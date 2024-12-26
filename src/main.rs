use daily_news::models::place::Place;
use daily_news::models::cache::Cache;

fn main() {
    let mut cache = Cache::load();
    
    let (name, place_id) = match Place::get_place_info_from_cache(&cache) {
        Some((name, id)) => (name, id),
        None => match Place::get_valid_place(&mut cache) {
            Some(place) => {
                // Update cache with all place information
                let name = place.name.clone();
                let id = place.place_id;
                cache.last_place = Some(place.name);
                cache.last_display_name = Some(place.display_name);
                cache.last_place_id = Some(place.place_id);
                cache.last_lat = Some(place.lat);
                cache.last_lon = Some(place.lon);
                cache.save();
                (name, id)
            }
            None => {
                println!("No place selected. Exiting.");
                return;
            }
        }
    };

    println!("\nUsing place '{}' with ID: {}", name, place_id);
    // Now you can use name and place_id for weather and news queries
}
