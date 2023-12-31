use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Api key for WeatherApi.com
    api_key: String,
    /// Location in "City Region/State" format
    location: String,
    /// Time is stored as minutes between updates
    update_interval: u64,
}

impl Config {
    pub fn get() -> io::Result<Config> {
        let toml_string = fs::read_to_string("config.toml")?;

        let toml: Config = toml::from_str(toml_string.as_str()).unwrap();
        Ok(toml)
    }

    pub fn get_api_key(&self) -> Result<&str, Box<dyn std::error::Error>> {
        match self.api_key.is_empty() {
            true => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "api key not found in config file",
            ))),
            false => Ok(self.api_key.as_str()),
        }
    }

    pub fn get_location(&self) -> Result<(&str, &str), Box<dyn std::error::Error>> {
        let split = self.location.split_once(' ');
        match split {
            None => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "config file location is in invalid format, please use a \"CITY STATE\" format",
            ))),
            Some(split) => Ok((split.0, split.1)),
        }
    }

    pub fn get_update_interval(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match self.update_interval {
            0 => {
                Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "config file location is in invalid format, please use a \"CITY STATE\" format",
                )))
            }
            _ => {
                Ok(self.update_interval)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grab_api_key() {
        assert!(Config::get().unwrap().get_api_key().is_ok())
    }
}
