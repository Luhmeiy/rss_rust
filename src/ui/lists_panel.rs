use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::border,
    widgets::{Block, List, ListState},
};

use crate::state::SharedState;
use crate::ui::list_layout::CursorPosition;

pub struct ListsPanel {
    list_state: ListState,
}

impl ListsPanel {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        ListsPanel { list_state }
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        lists: &[String],
        lists_area: Rect,
        is_active: bool,
    ) {
        let border_style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let border = Block::bordered()
            .title(" Lists ")
            .border_set(border::THICK)
            .border_style(border_style);

        let list = List::new(lists.to_vec())
            .block(border)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, lists_area, &mut self.list_state);
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        cursor_position: &mut CursorPosition,
        shared_state: &mut SharedState,
    ) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Enter => *cursor_position = CursorPosition::Feed,
            KeyCode::Char('n') => shared_state.toggle_show_new_list_popup(),
            _ => {}
        }
    }

    pub fn get_list_state<'a>(&self, lists: &'a [String]) -> &'a str {
        let index = self.list_state.selected().unwrap();
        &lists[index]
    }
}
