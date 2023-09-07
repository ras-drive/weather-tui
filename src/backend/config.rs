use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    api_key: String,
}

impl Config {
    pub fn get() -> io::Result<Config> {
        let toml_string = fs::read_to_string("config.toml")?;

        let toml: Config = toml::from_str(toml_string.as_str()).unwrap();
        Ok(toml)
    }

    pub fn get_api_key(&self) -> Result<&str, Box<dyn std::error::Error>> {
        match self.api_key.is_empty() {
            true => todo!(),
            false => Ok(self.api_key.as_str()),
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
