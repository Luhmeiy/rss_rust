use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

use crate::state::{data::DataState, ui::UIState};

pub struct ListPopupState {
    list_state: ListState,
    title: String,
    title_bottom: Option<String>,
    empty_message: String,
}

impl ListPopupState {
    pub fn new(title: &str, title_bottom: Option<String>, empty_message: &str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        ListPopupState {
            list_state,
            title: title.to_string(),
            title_bottom,
            empty_message: empty_message.to_string(),
        }
    }

    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_title_bottom(&self) -> &str {
        &self.title_bottom.as_deref().unwrap_or("")
    }
}

pub trait ListPopup {
    fn get_state(&mut self) -> &mut ListPopupState;
    fn render_list(data: &DataState) -> Vec<ListItem<'_>>;
    fn handle_key_event(&mut self, key_event: KeyEvent, data: &mut DataState, ui: &mut UIState);

    fn render_list_item(is_selected: bool, title: String) -> ListItem<'static> {
        let marker: Line = if is_selected {
            Line::from(vec![
                Span::raw("●").green(),
                Span::raw(" "),
                Span::raw(title),
            ])
        } else {
            Line::from(vec![Span::raw("○"), Span::raw(" "), Span::raw(title)])
        };

        ListItem::new(marker)
    }

    fn render(&mut self, frame: &mut Frame, data: &DataState) {
        let state = self.get_state();

        let popup_block = Block::bordered()
            .title(format!(" {} ", state.get_title()))
            .title_bottom(format!(
                " [Esc] Back  [Space] Select/Deselect {}",
                state.get_title_bottom()
            ))
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        let items = Self::render_list(data);
        if items.is_empty() {
            let empty = Paragraph::new(format!(" {}", state.empty_message)).block(popup_block);
            return frame.render_widget(empty, centered_area);
        }

        let list = List::new(items)
            .block(popup_block)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, centered_area, &mut state.get_list_state())
    }
}
