use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::feed::FeedEntry;

#[derive(PartialEq)]
enum ViewMode {
    List,
    Content,
}

pub struct App {
    entries: Vec<FeedEntry>,
    list_state: ListState,
    view_mode: ViewMode,
    scroll_offset: u16,
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
            view_mode: ViewMode::List,
            scroll_offset: 0,
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
        if self.view_mode == ViewMode::List {
            let [border_area] = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .areas(frame.area());

            let border = Block::bordered()
                .title(Line::from(" RSS Feed ").bold().centered())
                .title_bottom(Line::from("[↑/↓] Navigate  [o] Open  [v] View  [q] Quit").centered())
                .border_set(border::THICK);

            let items: Vec<ListItem> = self
                .entries
                .iter()
                .map(|item| {
                    let desc = item.summary();

                    let mut lines = vec![
                        Line::from(item.title()),
                        Line::from(format!("Source: {}", item.source())),
                        Line::from(format!("Date: {}", item.date())),
                    ];

                    if !desc.is_empty() {
                        lines.push(Line::from(desc));
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
        } else {
            let outer_area = frame.area();

            let (title, source, date, content_text) = self
                .list_state
                .selected()
                .and_then(|s| self.entries.get(s))
                .map(|e| (e.title(), e.source(), e.date(), e.content()))
                .or_else(|| {
                    Some((
                        String::from("No title"),
                        String::from("No source"),
                        String::from("No date"),
                        String::from("No content"),
                    ))
                })
                .unwrap_or_default();

            let outer_block = Block::bordered()
                .title(Line::from(format!(" {} ", title)).bold().centered())
                .title_bottom(
                    Line::from("[↑/↓] Scroll  [←/→] Next/Prev  [o] Open  [Esc] Back  [q] Quit")
                        .centered(),
                )
                .border_set(border::THICK);
            frame.render_widget(outer_block, outer_area);

            let inner_layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .flex(Flex::Center)
                .split(frame.area());

            let metadata_text = format!(" {} | {} ", source, date);
            let metadata_text_length = metadata_text.len() as u16;
            let metadata = Paragraph::new(metadata_text)
                .block(Block::bordered())
                .centered();
            frame.render_widget(
                metadata,
                inner_layout[0].centered_horizontally(Constraint::Length(metadata_text_length + 2)),
            );

            let body = Paragraph::new(content_text)
                .wrap(Wrap { trim: true })
                .scroll((0, self.scroll_offset));
            frame.render_widget(body, inner_layout[1]);
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Down => {
                if self.view_mode == ViewMode::Content {
                    self.scroll_offset = self.scroll_offset.saturating_add(3);
                } else {
                    self.list_state.select_next();
                }
            }
            KeyCode::Up => {
                if self.view_mode == ViewMode::Content {
                    self.scroll_offset = self.scroll_offset.saturating_sub(3);
                } else {
                    self.list_state.select_previous();
                }
            }
            KeyCode::Right => {
                if self.view_mode == ViewMode::Content {
                    self.list_state.select_next();
                    self.scroll_offset = 0;
                }
            }
            KeyCode::Left => {
                if self.view_mode == ViewMode::Content {
                    self.list_state.select_previous();
                    self.scroll_offset = 0;
                }
            }
            KeyCode::Char('o') => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(entry) = self.entries.get(selected) {
                        if let Some(link) = entry.entry.links.first() {
                            let _ = open::that(&link.href);
                        }
                    }
                }
            }
            KeyCode::Char('v') => {
                self.view_mode = ViewMode::Content;
                self.scroll_offset = 0;
            }
            KeyCode::Esc => self.view_mode = ViewMode::List,
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
