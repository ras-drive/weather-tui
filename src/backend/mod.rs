use reqwest::Url;
use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    is_day: i64,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    wind_degree: f64,
    pub wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    pub humidity: f64,
    cloud: f64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    icon: String,
    code: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}

impl Weather {
    pub async fn get(api_key: &str, location: (&str, &str)) -> Result<Self, reqwest::Error> {
        let params = [("q", format!("{} {}", location.0, location.1))];
        let url =
            Url::parse_with_params("https://weatherapi-com.p.rapidapi.com/current.json", params)
                .unwrap();
        let client = reqwest::Client::new();
        client
            .get(url)
            .header("X-RapidAPI-Key", api_key)
            .header("X-RapidAPI-Host", "weatherapi-com.p.rapidapi.com")
            .send()
            .await?
            .json::<Weather>()
            .await
    }

    pub fn get_condition(&self) -> &str {
        &self.current.condition.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[tokio::test]
    async fn test_get_weather_object() {
        let weather = Weather::get(
            Config::get().unwrap().get_api_key().unwrap(),
            ("Chicago", "US"),
        )
        .await;
        assert!(weather.is_ok());
    }
}
