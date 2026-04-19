use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListItem;

use crate::{
    state::SharedState,
    ui::list_popup::{ListPopup, ListPopupState},
};

pub struct AddToListPopup {
    state: ListPopupState,
}

impl AddToListPopup {
    pub fn new() -> Self {
        AddToListPopup {
            state: ListPopupState::new("Add to List", None, "No user created lists found."),
        }
    }
}

impl ListPopup for AddToListPopup {
    fn get_state(&mut self) -> &mut ListPopupState {
        &mut self.state
    }

    fn render_list(shared_state: &SharedState) -> Vec<ListItem<'_>> {
        shared_state
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

                Self::render_list_item(is_selected, feed.0.clone())
            })
            .collect()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        let list_state = self.state.get_list_state();

        match key_event.code {
            KeyCode::Down => list_state.select_next(),
            KeyCode::Up => list_state.select_previous(),
            KeyCode::Esc => shared_state.toggle_show_list_selector(),
            KeyCode::Char(' ') => {
                if let Some(selected_idx) = list_state.selected() {
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
