use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::feed::FeedEntry;

pub struct App {
    entries: Vec<FeedEntry>,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(area);

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(2)
            .areas(border_area);

        let title = Line::from(" RSS Feed ".bold());
        Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .render(border_area, buf);

        let mut lines: Vec<Line> = Vec::new();

        for item in self.entries.iter() {
            let title = item.entry.title.as_ref().map_or("Untitled", |t| &t.content);
            let link = item.entry.links.first().map_or("N/A", |l| &l.href);
            let date = item.entry.published.or(item.entry.updated).map_or_else(
                || "N/A".to_string(),
                |d| d.format("%d-%m-%Y %H:%M").to_string(),
            );

            let desc = item
                .entry
                .summary
                .as_ref()
                .map(|s| s.content.to_owned())
                .unwrap_or_default();

            lines.push("----------------------------------------------".into());
            lines.push(title.into());

            lines.push(format!("Source: {}", item.source).into());
            lines.push(format!("Link: {}", link).into());
            lines.push(format!("Date: {}", date).into());

            if !desc.is_empty() {
                lines.push(format!("{}", desc).into());
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner_area, buf);
    }
}

impl App {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        App {
            entries,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
