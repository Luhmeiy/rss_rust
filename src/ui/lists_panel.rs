use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::border,
    widgets::{Block, List, ListState},
};

use crate::ui::list_layout::CursorPosition;

pub struct ListsPanel {
    lists: Vec<String>,
    list_state: ListState,
}

impl ListsPanel {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        ListsPanel {
            lists: Vec::from(["All".to_string(), "Favorites".to_string()]),
            list_state,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, lists_area: Rect, is_active: bool) {
        let border_style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let border = Block::bordered()
            .title(" Lists ")
            .border_set(border::THICK)
            .border_style(border_style);

        let list = List::new(self.lists.clone())
            .block(border)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, lists_area, &mut self.list_state);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, cursor_position: &mut CursorPosition) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Enter => *cursor_position = CursorPosition::Feed,
            _ => {}
        }
    }

    pub fn get_list_state(&self) -> &str {
        let index = self.list_state.selected().unwrap();
        &self.lists[index]
    }
}
