use daily_news::services::{place, weather};

fn main() {
    place::serve_place();
    let weather = weather::serve_weather();
    match weather {
        Ok(weather) => println!("{:?}", weather),
        Err(e) => println!("Error: {}", e),
    }
}
