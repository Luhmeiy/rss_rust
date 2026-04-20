use ratatui;
use std::error::Error;

mod fetch;
mod models;
mod state;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let (feeds, content) = fetch::loader::run();
    ratatui::run(|terminal| ui::app::App::new(feeds, content).run(terminal))?;
    Ok(())
}
