use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Margin, Position, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::{state::SharedState, ui::input::Input};

pub struct NewFeedPopup {
    input: Input,
}

impl NewFeedPopup {
    pub fn new() -> Self {
        NewFeedPopup {
            input: Input::new(),
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

        let feed_bar = Paragraph::new(self.input.get_field()).block(feed_bar_border);
        frame.render_widget(feed_bar, inner_area);

        frame.set_cursor_position(Position::new(
            inner_area.x + self.input.get_character_index() as u16 + 2,
            inner_area.y + 1,
        ))
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Right => self.input.move_cursor_right(),
            KeyCode::Left => self.input.move_cursor_left(),
            KeyCode::Backspace => self.input.delete_char(),
            KeyCode::Enter => self.input.reset(),
            KeyCode::Esc => {
                self.input.reset();
                shared_state.toggle_show_new_feed_popup();
            }
            KeyCode::Char(to_insert) => self.input.enter_char(to_insert),
            _ => {}
        }
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
