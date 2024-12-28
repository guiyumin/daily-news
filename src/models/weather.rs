use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Weather {
    pub temperature: Vec<f64>,
    pub wind_speed: Vec<f64>,
    pub humidity: Vec<f64>,
    pub description: String,
}
