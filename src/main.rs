use daily_news::models::place::{Place, PlaceType};

fn main() {
    match Place::get_by_name("San Diego", PlaceType::City) {
        Ok(cities) => println!("{:#?}", cities),
        Err(e) => println!("Error: {}", e),
    }
}
