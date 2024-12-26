mod models;

use models::place::Place;
use models::place::PlaceType;

fn main() {
    match Place::get_by_name("San Diego", PlaceType::City) {
        Ok(cities) => println!("{:#?}", cities),
        Err(e) => println!("Error: {}", e),
    }
}
