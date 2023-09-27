use crate::terminal::{ui, weather_condition::WeatherCondition};
use std::{
    error::Error,
    io::Stdout,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, Terminal};

#[derive(Default)]
pub struct App {
    pub weather_condition: Option<WeatherCondition>,
    pub should_update: bool,
}

pub async fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }
    Ok(())
}
