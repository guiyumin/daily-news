use crate::models::cfg::Cfg;
use crate::utils::urls::OPENSTREETMAP_SEARCH_URL;
use colored::Colorize;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Place {
    pub place_id: Option<u32>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
}

impl Place {
    pub fn query_by_name(name: &str) -> Result<Vec<Place>, reqwest::Error> {
        let url = format!("{}?q={}&format=jsonv2", OPENSTREETMAP_SEARCH_URL, name);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&url)
            .header(
                "User-Agent",
                format!("DailyNews/1.0 User/{}", Cfg::new().user_id),
            )
            .send()?;

        let places: Vec<Place> = response.json()?;
        Ok(places)
    }

    pub fn select(places: &[Place]) -> &Place {
        println!(
            "{}",
            "Found multiple places. Please select one:".bright_green()
        );

        for (i, place) in places.iter().enumerate() {
            println!("{}. {}", i + 1, place.display_name.as_deref().unwrap_or(""));
        }

        loop {
            let text = format!("Enter number (1-{})", places.len());
            print!("{}", text.bright_green());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Ok(selection) = input.trim().parse::<usize>() {
                if selection > 0 && selection <= places.len() {
                    return &places[selection - 1];
                }
            }
            println!("{}", "Invalid selection. Please try again.".red());
        }
    }
}
