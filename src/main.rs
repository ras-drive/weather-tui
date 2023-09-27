mod app;
mod backend;
mod terminal;

use app::run;
use app::App;

use crate::backend::config::Config;
use crate::terminal::{restore_terminal, setup_terminal};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get().expect("setup config file");
    let app = App::default();
    let mut terminal = setup_terminal()?;
    run(&mut terminal, app).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
