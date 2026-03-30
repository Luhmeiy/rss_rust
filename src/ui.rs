use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

use crate::feed::FeedEntry;

pub struct App {
    entries: Vec<FeedEntry>,
    list_state: ListState,
    exit: bool,
}

impl App {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        App {
            entries,
            list_state,
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

    fn draw(&mut self, frame: &mut Frame) {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let border = Block::bordered()
            .title(Line::from(" RSS Feed ").bold().centered())
            .border_set(border::THICK);

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|item| {
                let title = item.entry.title.as_ref().map_or("Untitled", |t| &t.content);
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

                let mut lines = vec![
                    Line::from(title.to_string()),
                    Line::from(format!("Source: {}", item.source)),
                    Line::from(format!("Date: {}", date)),
                ];

                if !desc.is_empty() {
                    lines.push(Line::from(format!("{}", desc)));
                }

                lines.push(Line::from(""));
                ListItem::new(lines)
            })
            .collect();

        let list = List::new(items)
            .block(border)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, border_area, &mut self.list_state);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Enter => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(entry) = self.entries.get(selected) {
                        if let Some(link) = entry.entry.links.first() {
                            let _ = open::that(&link.href);
                        }
                    }
                }
            }
            KeyCode::Char('q') => self.exit = true,
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
}
