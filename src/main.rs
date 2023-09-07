mod backend;
mod terminal;

use crate::backend::config::Config;
use crate::terminal::{restore_terminal, run, setup_terminal};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get().expect("setup config file");
    let mut terminal = setup_terminal()?;
    run(&mut terminal).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
