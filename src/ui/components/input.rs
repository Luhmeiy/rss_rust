pub struct Input {
    character_index: usize,
    field: String,
}

impl Input {
    pub fn new() -> Self {
        Input {
            character_index: 0,
            field: String::new(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.field.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.field
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.field.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;

        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.field.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.field.chars().skip(current_index);

            self.field = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.field.chars().count())
    }

    pub fn reset(&mut self) {
        self.field.clear();
        self.character_index = 0;
    }

    pub fn get_character_index(&self) -> usize {
        self.character_index
    }

    pub fn get_field(&self) -> &str {
        &self.field
    }
}
