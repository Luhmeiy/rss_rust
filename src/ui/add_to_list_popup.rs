use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

use crate::state::SharedState;

pub struct AddToListPopup {
    list_state: ListState,
}

impl AddToListPopup {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        AddToListPopup { list_state }
    }

    pub fn render(&mut self, frame: &mut Frame, shared_state: &mut SharedState) {
        let popup_block = Block::bordered()
            .title(format!(" Add Entry "))
            .title_bottom(" [Esc] Back  [Enter] Select ")
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        if shared_state.custom_lists.is_empty() {
            let empty = Paragraph::new(" No items.").block(popup_block);
            frame.render_widget(empty, centered_area);
        } else {
            let items: Vec<ListItem> = shared_state
                .custom_lists
                .iter()
                .map(|feed| {
                    let is_selected = shared_state
                        .custom_lists
                        .get(feed.0)
                        .map(|entries| {
                            entries
                                .iter()
                                .any(|e| Some(e) == shared_state.selected_entry.as_ref())
                        })
                        .unwrap_or(false);

                    let marker: Line = if is_selected {
                        Line::from(vec![
                            Span::raw("●").green(),
                            Span::raw(" "),
                            Span::raw(feed.0.clone()),
                        ])
                    } else {
                        Line::from(vec![
                            Span::raw("○"),
                            Span::raw(" "),
                            Span::raw(feed.0.clone()),
                        ])
                    };

                    ListItem::new(marker)
                })
                .collect();

            let list = List::new(items)
                .block(popup_block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");
            frame.render_stateful_widget(list, centered_area, &mut self.list_state);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Esc => shared_state.toggle_show_list_selector(),
            KeyCode::Char(' ') => {
                if let Some(selected_idx) = self.list_state.selected() {
                    let keys: Vec<String> = shared_state.custom_lists.keys().cloned().collect();
                    if let Some(list_name) = keys.get(selected_idx) {
                        if let Some(entry) = &shared_state.selected_entry {
                            if let Some(list_entries) = shared_state.custom_lists.get_mut(list_name)
                            {
                                if !list_entries.contains(entry) {
                                    list_entries.push(entry.clone());
                                } else {
                                    list_entries.retain(|e| e != entry);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
