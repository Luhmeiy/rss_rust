use ratatui;
use std::error::Error;

mod feed;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    let content = feed::run().await?;
    ratatui::run(|terminal| ui::App::new(content).run(terminal))?;
    Ok(())
}
