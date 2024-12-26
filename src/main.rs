mod models;

use models::city::City  ;

fn main() {
    match City::get_by_name("San Diego".to_string()) {
        Ok(cities) => println!("{:#?}", cities),
        Err(e) => println!("Error: {}", e),
    }
}
