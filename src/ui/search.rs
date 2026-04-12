use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Padding, Paragraph},
};

#[derive(Clone, PartialEq)]
enum InputMode {
    Normal,
    Searching,
}

pub struct Search {
    search: String,
    input_mode: InputMode,
    character_index: usize,
}

impl Search {
    pub fn new() -> Self {
        Search {
            search: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame, search_bar_area: Rect) {
        let search_bar_border = Block::bordered()
            .title(" Search ")
            .padding(Padding::left(1))
            .border_set(border::THICK);

        let search_bar = Paragraph::new(self.search.clone())
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
                search_bar_area.x + self.character_index as u16 + 2,
                search_bar_area.y + 1,
            )),
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Esc => self.toggle_input_mode(),
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            _ => {}
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.search
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.search.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;

        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.search.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.search.chars().skip(current_index);

            self.search = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search.chars().count())
    }

    pub fn get_search(&self) -> &str {
        &self.search
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
