mod terminal;
mod backend;

use std::error::Error;
use crate::terminal::{setup_terminal, restore_terminal, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

