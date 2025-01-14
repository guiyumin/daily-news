use crate::utils::urls::OPEN_METEO_URL;
use crate::{models::place::Place, utils::alphabet::ALPHANUMERIC};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::models::weather_response::{Current, Daily, WeatherResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CurrentWeatherData {
    pub temperature: f64,
    pub wind_speed: f64,
    pub rain: f64,
    pub snow: f64,
}

impl CurrentWeatherData {
    pub fn from(current: Current) -> Self {
        Self {
            temperature: current.temperature_2m,
            wind_speed: current.wind_speed_10m,
            rain: current.rain,
            snow: current.snowfall,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WeeklyWeatherData {
    pub dates: Vec<String>,
    pub min_temperature: Vec<f64>,
    pub max_temperature: Vec<f64>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub max_wind_speed: Vec<f64>,
    pub rain: Vec<f64>,
    pub snow: Vec<f64>,
}

impl WeeklyWeatherData {
    pub fn from(daily: Daily) -> Self {
        Self {
            dates: daily.time,
            min_temperature: daily.temperature_2m_min,
            max_temperature: daily.temperature_2m_max,
            sunrise: daily.sunrise,
            sunset: daily.sunset,
            max_wind_speed: daily.wind_speed_10m_max,
            rain: daily.rain_sum,
            snow: daily.snowfall_sum,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Weather {
    pub current: CurrentWeatherData,
    pub weekly: WeeklyWeatherData,
}

impl Weather {
    pub fn new() -> Self {
        Self {
            current: CurrentWeatherData::default(),
            weekly: WeeklyWeatherData::default(),
        }
    }

    pub fn query_by_place(&self, place: &Place) -> Result<Self, reqwest::Error> {
        let query_params: [(&str, &str); 4] = [
            ("latitude", place.lat.as_ref().unwrap()),
            ("longitude", place.lon.as_ref().unwrap()),
            ("current", "snowfall,temperature_2m,wind_speed_10m,rain"),
            (
                "daily",
                "rain_sum,snowfall_sum,temperature_2m_max,temperature_2m_min,sunrise,sunset,wind_speed_10m_max",
            ),
        ];
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        let url = format!("{}?{}", OPEN_METEO_URL, query_string);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header(
                "User-Agent",
                format!("DailyNews/1.0 User/{}", nanoid!(12, &ALPHANUMERIC)),
            )
            .send()?;

        let weather_response: WeatherResponse = response.json()?;
        let weather = Weather::from(weather_response);
        Ok(weather)
    }

    pub fn from(weather_response: WeatherResponse) -> Self {
        Self {
            current: CurrentWeatherData::from(weather_response.current),
            weekly: WeeklyWeatherData::from(weather_response.daily),
        }
    }
}
