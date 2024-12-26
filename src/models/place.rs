use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    pub place_id: u32,
    pub display_name: String,
    pub name: String,
    pub lat: String,
    pub lon: String,
}

pub enum PlaceType {
    City,
    Zipcode,
}

impl Place {
    pub fn get_by_name(name: &str, addresstype: PlaceType) -> Result<Vec<Place>, reqwest::Error> {
        let search_string = match addresstype {
            PlaceType::City => {
                name.to_string()
            }
            PlaceType::Zipcode => {
                format!("{},US", name)
            }
        };

        let url = format!("https://nominatim.openstreetmap.org/search.php?q={}&format=jsonv2", search_string);
     
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "DailyNews/1.0")
            .send()?;

        let places: Vec<Place> = response.json()?;
        Ok(places)
    }



    // pub fn get_by_name(name: &str) -> Self {
    //     // TODO: Implement this, read from the cache if possible

    // }

}

