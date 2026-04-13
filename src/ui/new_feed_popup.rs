use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Margin, Position, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::state::SharedState;

pub struct NewFeedPopup {
    feed: String,
    character_index: usize,
}

impl NewFeedPopup {
    pub fn new() -> Self {
        NewFeedPopup {
            feed: String::new(),
            character_index: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let popup_block = Block::bordered()
            .title(" Add Source ")
            .title_bottom(" [Esc] Back  [Enter] Add Source ")
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);
        frame.render_widget(popup_block, centered_area);

        let inner_area = centered_area.inner(Margin::new(2, 3));

        let feed_bar_border = Block::bordered()
            .padding(Padding::left(1))
            .border_set(border::THICK);

        let feed_bar = Paragraph::new(self.feed.clone()).block(feed_bar_border);
        frame.render_widget(feed_bar, inner_area);

        frame.set_cursor_position(Position::new(
            inner_area.x + self.character_index as u16 + 2,
            inner_area.y + 1,
        ))
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Enter => {
                self.feed.clear();
                self.character_index = 0;
            }
            KeyCode::Esc => {
                self.feed.clear();
                self.character_index = 0;
                shared_state.toggle_show_new_feed_popup();
            }
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
        self.feed.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.feed
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.feed.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;

        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.feed.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.feed.chars().skip(current_index);

            self.feed = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.feed.chars().count())
    }
}

pub fn render_new_feed_button(frame: &mut Frame, new_feed_area: Rect, show_popup: bool) {
    let border = Block::bordered()
        .border_set(border::THICK)
        .style(match show_popup {
            true => Style::default().fg(Color::Yellow),
            false => Style::default(),
        });
    let source = Paragraph::new("[a] Add Feed").block(border).centered();

    frame.render_widget(source, new_feed_area);
}
