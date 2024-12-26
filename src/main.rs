use daily_news::models::place::Place;

fn main() {
    match Place::get_by_name("San Diego") {
        Ok(cities) => println!("{:#?}", cities),
        Err(e) => println!("Error: {}", e),
    }
}
