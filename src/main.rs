use ratatui;
use std::error::Error;

mod feed;
mod state;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let (sources, content) = feed::run();
    ratatui::run(|terminal| ui::App::new(sources, content).run(terminal))?;
    Ok(())
}
