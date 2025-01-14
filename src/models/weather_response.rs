use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current: Current,
    pub daily: Daily,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Current {
    pub time: String,
    pub interval: i32,
    pub snowfall: f64,
    pub temperature_2m: f64,
    pub wind_speed_10m: f64,
    pub rain: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Daily {
    pub time: Vec<String>,
    pub rain_sum: Vec<f64>,
    pub snowfall_sum: Vec<f64>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub wind_speed_10m_max: Vec<f64>,
}

// {

//   "current_units": {
//     "time": "iso8601",
//     "interval": "seconds",
//     "snowfall": "cm",
//     "temperature_2m": "°C",
//     "wind_speed_10m": "km/h",
//     "rain": "mm"
//   },

//   "daily_units": {
//     "time": "iso8601",
//     "rain_sum": "mm",
//     "snowfall_sum": "cm",
//     "temperature_2m_max": "°C",
//     "temperature_2m_min": "°C",
//     "sunrise": "iso8601",
//     "sunset": "iso8601",
//     "wind_speed_10m_max": "km/h"
//   },

// }
