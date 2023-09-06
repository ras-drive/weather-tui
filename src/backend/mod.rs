
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

use dotenv::dotenv;

fn get_api_key() -> Result<String, dotenv::Error> {
    dotenv().ok();

    for (key, value) in env::vars() {
        if key == "WEATHER_API_KEY" {
            return Ok(value);
        }
    }

    Err(dotenv::Error::EnvVar(env::VarError::NotPresent))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    location: Location,
    current: Current,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i64,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: f64,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: f64,
    cloud: f64,
    feelslike_c: f64,
    feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    text: String,
    icon: String,
    code: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}

impl Weather {
    pub async fn get(location: (&str, &str)) -> Result<Self, reqwest::Error> {
        let params = [
        ("q", format!("{} {}", location.0, location.1)),
        ];
        let url = Url::parse_with_params("https://weatherapi-com.p.rapidapi.com/current.json", params).unwrap();
        let client = reqwest::Client::new();
        client
        .get(url)
        .header("X-RapidAPI-Key", get_api_key().unwrap())
        .header("X-RapidAPI-Host", "weatherapi-com.p.rapidapi.com")
        .send()
        .await?.json::<Weather>().await

    }

    pub fn get_location(&self) -> (&str, &str, &str) {
        (&self.location.country, &self.location.region, &self.location.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_key() {
        assert!(get_api_key().is_ok())
    }

    #[tokio::test]
    async fn test_get_weather_object() {
        let weather = Weather::get(("Chicago", "US")).await;
        assert!(weather.is_ok());
    }
}
