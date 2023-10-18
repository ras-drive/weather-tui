use crate::{
    backend::{config::Config, Weather},
    terminal::{ui, weather_condition::WeatherCondition},
};
use std::{error::Error, io::Stdout, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, Terminal};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub weather: Option<Weather>,
    pub should_update: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            weather: None,
            should_update: true,
        }
    }
}

pub async fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    loop {
        match app.should_update {
            true => {
                let weather =
                    Weather::get(app.config.get_api_key()?, app.config.get_location()?).await;

                match weather {
                    Ok(weather) => {
                        app.weather = Some(weather);
                    }
                    Err(e) => return Err(Box::new(e)),
                }

                app.should_update = false;
            }
            false => {
                if event::poll(Duration::from_millis(250))? {
                    if let Event::Key(key) = event::read()? {
                        if KeyCode::Char('q') == key.code {
                            break;
                        }
                    }
                }

                terminal.draw(|f| {
                    ui(
                        f,
                        &WeatherCondition::parse(app.weather.as_ref().unwrap().get_condition()),
                        &app.weather,
                        false, // celcius defaults to false for now, will be grabbed from the config file
                        false, // kph defaults to false for now, will be grabbed from the config file
                    )
                })?;
            }
        }
    }

    Ok(())
}
