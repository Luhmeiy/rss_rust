use ratatui;
use std::error::Error;

mod feed;
mod state;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let (feeds, content) = feed::run();
    ratatui::run(|terminal| ui::App::new(feeds, content).run(terminal))?;
    Ok(())
}
