use serde::{Deserialize, Serialize};
use reqwest;
use nanoid::nanoid;
use crate::utils::alphabet::ALPHANUMERIC;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Place {
    pub place_id: u32,
    pub display_name: String,
    pub name: String,
    pub lat: String,
    pub lon: String,
}



impl Place {
    pub fn get_by_name(name: &str) -> Result<Vec<Place>, reqwest::Error> {

        let url = format!("https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2", name);
     
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", format!("DailyNews/1.0 User/{}", nanoid!(12, &ALPHANUMERIC)))
            .send()?;

        let places: Vec<Place> = response.json()?;
        Ok(places)
    }



    // pub fn get_by_name(name: &str) -> Self {
    //     // TODO: Implement this, read from the cache if possible

    // }

}

