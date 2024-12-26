use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub place_id: u32,
    pub display_name: String,
    pub name: String,
    #[serde(rename = "lat")]
    pub latitude: String,
    #[serde(rename = "lon")]
    pub longitude: String,
}

impl City {
    pub fn get_by_name(name: String) -> Result<Vec<City>, reqwest::Error> {
        let url = format!("https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2", name);
     
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "DailyNews/1.0")
            .send()?;

        let cities: Vec<City> = response.json()?;
        Ok(cities)
    }

    // pub fn get_by_place_id(place_id: u32) -> Self {
    //     // TODO: Implement this, read from the cache if possible

    // }

}

