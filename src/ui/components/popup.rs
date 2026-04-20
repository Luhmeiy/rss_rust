use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Margin, Position, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::{
    state::{data::DataState, ui::UIState},
    ui::components::input::Input,
};

pub struct PopupState {
    input: Input,
    status: Option<bool>,
    title: String,
}

impl PopupState {
    pub fn new(title: &str) -> Self {
        PopupState {
            input: Input::new(),
            status: None,
            title: title.to_string(),
        }
    }

    pub fn get_input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn get_status(&mut self) -> &mut Option<bool> {
        &mut self.status
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn reset(&mut self) {
        self.input.reset();
        self.status = None;
    }
}

pub trait Popup {
    fn get_state(&mut self) -> &mut PopupState;

    fn render(&mut self, frame: &mut Frame) {
        let state = self.get_state();

        let title = state.get_title().to_string();
        let status = state.get_status().clone();

        let popup_block = Block::bordered()
            .title(format!(" {} ", title))
            .title_bottom(format!(" [Esc] Back  [Enter] {} ", title))
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);
        frame.render_widget(popup_block, centered_area);

        let inner_area = centered_area.inner(Margin::new(2, 3));

        let border_style = match status {
            Some(true) => Style::default().fg(Color::Green),
            Some(false) => Style::default().fg(Color::Red),
            None => Style::default(),
        };

        let feed_bar_border = Block::bordered()
            .padding(Padding::left(1))
            .border_set(border::THICK)
            .style(border_style);

        let input = state.get_input();
        let feed_bar = Paragraph::new(input.get_field()).block(feed_bar_border);
        frame.render_widget(feed_bar, inner_area);

        frame.set_cursor_position(Position::new(
            inner_area.x + input.get_character_index() as u16 + 2,
            inner_area.y + 1,
        ))
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, data: &mut DataState, ui: &mut UIState) {
        let state = self.get_state();

        match key_event.code {
            KeyCode::Right => state.get_input().move_cursor_right(),
            KeyCode::Left => state.get_input().move_cursor_left(),
            KeyCode::Backspace => state.get_input().delete_char(),
            KeyCode::Enter => {
                let input = state.get_input().get_field();

                if input.is_empty() {
                    return;
                }

                self.on_enter(data, ui);
            }
            KeyCode::Esc => {
                state.reset();
                self.on_esc(data, ui);
            }
            KeyCode::Char(to_insert) => {
                *state.get_status() = None;
                state.get_input().enter_char(to_insert);
            }
            _ => {}
        }
    }

    fn on_enter(&mut self, data: &mut DataState, ui: &mut UIState);
    fn on_esc(&mut self, data: &mut DataState, ui: &mut UIState);
}

pub fn render_button(frame: &mut Frame, button_area: Rect, show_popup: bool, text: String) {
    let border = Block::bordered()
        .border_set(border::THICK)
        .style(match show_popup {
            true => Style::default().fg(Color::Yellow),
            false => Style::default(),
        });
    let button = Paragraph::new(format!("{}", text)).block(border).centered();

    frame.render_widget(button, button_area);
}
