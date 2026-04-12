use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    widgets::{Block, Clear, List, ListState, Paragraph},
};

use crate::state::SharedState;

pub struct SourcePopup {
    list_state: ListState,
}

impl SourcePopup {
    pub fn new() -> Self {
        SourcePopup {
            list_state: ListState::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, sources: &Vec<String>) {
        match self.list_state.selected() {
            Some(_) => {}
            None => {
                if !sources.is_empty() {
                    self.list_state.select(Some(0))
                }
            }
        }

        let popup_block = Block::bordered()
            .title(" Sources ")
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        let list = List::new(sources.clone())
            .block(popup_block)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, centered_area, &mut self.list_state);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Char('s') | KeyCode::Esc => shared_state.toggle_show_popup(),
            _ => {}
        }
    }
}

pub fn render_source_button(frame: &mut Frame, source_area: Rect, show_popup: bool) {
    let border = Block::bordered()
        .border_set(border::THICK)
        .style(match show_popup {
            true => Style::default().fg(Color::Yellow),
            false => Style::default(),
        });
    let source = Paragraph::new("[s] Sources").block(border).centered();

    frame.render_widget(source, source_area);
}
