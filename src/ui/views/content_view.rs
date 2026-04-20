use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, HorizontalAlignment, Layout},
    symbols::border,
    widgets::{Block, Paragraph, Wrap},
};

use crate::state::{data::DataState, ui::UIState, ui::ViewMode};

pub struct ContentView {
    scroll_offset: u16,
}

impl ContentView {
    pub fn new() -> Self {
        ContentView { scroll_offset: 0 }
    }

    pub fn render(&self, frame: &mut Frame, data: &DataState, ui: &UIState) {
        let (title, feed, date, content_text) = ui
            .list_state
            .selected()
            .and_then(|s| data.entries.get(s))
            .map(|e| (e.title(), e.feed(), e.date(), e.content()))
            .or_else(|| {
                Some((
                    String::from("No title"),
                    String::from("No feed"),
                    String::from("No date"),
                    String::from("No content"),
                ))
            })
            .unwrap_or_default();

        let outer_area = frame.area();

        let outer_block = Block::bordered()
            .title(format!(" {} ", title))
            .title_bottom("[↑/↓] Scroll  [←/→] Next/Prev  [o] Open  [Esc] Back  [q] Quit")
            .title_alignment(HorizontalAlignment::Center)
            .border_set(border::THICK);
        frame.render_widget(outer_block, outer_area);

        let layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
            .horizontal_margin(2)
            .vertical_margin(1)
            .flex(Flex::Center);
        let [header_area, body_area] = outer_area.layout(&layout);

        let metadata_text = format!(" {} | {} ", feed, date);
        let metadata_text_length = metadata_text.len() as u16;
        let metadata = Paragraph::new(metadata_text)
            .block(Block::bordered())
            .centered();
        frame.render_widget(
            metadata,
            header_area.centered_horizontally(Constraint::Length(metadata_text_length + 2)),
        );

        let body = Paragraph::new(content_text.clone())
            .wrap(Wrap { trim: true })
            .scroll((0, self.scroll_offset));
        frame.render_widget(body, body_area);
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        data: &mut DataState,
        ui: &mut UIState,
    ) {
        match key_event.code {
            KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(3);
            }
            KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(3);
            }
            KeyCode::Right => {
                ui.list_state.select_next();
                self.scroll_offset = 0;
            }
            KeyCode::Left => {
                ui.list_state.select_previous();
                self.scroll_offset = 0;
            }
            KeyCode::Char('o') => {
                if let Some(selected) = ui.list_state.selected() {
                    if let Some(entry) = data.entries.get(selected) {
                        if let Some(link) = entry.entry.links.first() {
                            let _ = open::that(&link.href);
                        }
                    }
                }
            }
            KeyCode::Esc => ui.view_mode = ViewMode::List,
            KeyCode::Char('q') => ui.exit = true,
            _ => {}
        }
    }
}
