use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Padding, Paragraph},
};

use crate::ui::components::input::Input;

#[derive(Clone, PartialEq)]
enum InputMode {
    Normal,
    Searching,
}

pub struct Search {
    input: Input,
    input_mode: InputMode,
}

impl Search {
    pub fn new() -> Self {
        Search {
            input: Input::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn render(&self, frame: &mut Frame, search_bar_area: Rect) {
        let search_bar_border = Block::bordered()
            .title(" [/] Search ")
            .padding(Padding::left(1))
            .border_set(border::THICK);

        let search_bar = Paragraph::new(self.input.get_field())
            .block(search_bar_border)
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Searching => Style::default().fg(Color::Yellow),
            });

        frame.render_widget(search_bar, search_bar_area);

        match self.input_mode {
            InputMode::Normal => {}
            #[expect(clippy::cast_possible_truncation)]
            InputMode::Searching => frame.set_cursor_position(Position::new(
                search_bar_area.x + self.input.get_character_index() as u16 + 2,
                search_bar_area.y + 1,
            )),
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right => self.input.move_cursor_right(),
            KeyCode::Left => self.input.move_cursor_left(),
            KeyCode::Backspace => self.input.delete_char(),
            KeyCode::Esc => self.toggle_input_mode(),
            KeyCode::Char(to_insert) => self.input.enter_char(to_insert),
            _ => {}
        }
    }

    pub fn get_search(&self) -> &str {
        &self.input.get_field()
    }

    pub fn is_search(&self) -> bool {
        self.input_mode == InputMode::Searching
    }

    pub fn toggle_input_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Searching,
            InputMode::Searching => InputMode::Normal,
        };
    }
}
