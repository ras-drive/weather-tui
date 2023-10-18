use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::Paragraph, Frame, Terminal};

use crate::backend::Weather;

pub mod weather_condition;

use self::weather_condition::WeatherCondition;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    weather_condition: &Option<WeatherCondition>,
    weather: &Option<Weather>,
    celcius_flag: bool,
    kph_flag: bool,
) {
    let size = f.size();

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(size);

    let art = match &weather_condition {
        Some(wc) => wc.as_ref(),
        None => "",
    };

    let p = Paragraph::new(art);
    f.render_widget(p, chunks[0]);

    let temp = if celcius_flag {
        format!("{}C", &weather.as_ref().unwrap().current.temp_c)
    } else {
        format!("{}F", &weather.as_ref().unwrap().current.temp_f)
    };

    let wind = if kph_flag {
        format!("{}KPH", &weather.as_ref().unwrap().current.wind_kph)
    } else {
        format!("{}MPH", &weather.as_ref().unwrap().current.wind_mph)
    };

    let p: Paragraph<'_> = Paragraph::new(format!("\nWeather in {} {}\n\nTodays weather is {}\nThe temperature is {}\n\n {}% Humidity\n {} Winds",
        &weather.as_ref().unwrap().location.name,
        &weather.as_ref().unwrap().location.region,
        &weather.as_ref().unwrap().current.condition.text,
        temp,
        &weather.as_ref().unwrap().current.humidity,
        wind));
    f.render_widget(p, chunks[1]);
}
