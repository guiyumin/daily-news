use crate::models::cache::Cache;

use crate::models::weather::Weather;

pub fn serve_weather() -> Result<Weather, reqwest::Error> {
    let place = Cache::load().retrieve_place().unwrap();
    let weather_instance = Weather::new();
    weather_instance.query_by_place(&place)
}
