use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::Paragraph, Frame, Terminal};

use crate::app::App;

pub mod weather_condition;

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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(size);

    let art = match &app.weather_condition {
        Some(wc) => wc.as_ref(),
        None => "",
    };

    let p = Paragraph::new(art);
    f.render_widget(p, chunks[0]);

    let p: Paragraph<'_> = Paragraph::new("Hello, World!");
    f.render_widget(p, chunks[1]);
}
