use std::collections::HashSet;

use ratatui::widgets::ListState;

use crate::feed::FeedEntry;

#[derive(PartialEq)]
pub enum ViewMode {
    List,
    Content,
}

pub struct SharedState {
    pub entries: Vec<FeedEntry>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub exit: bool,
    pub show_popup: bool,
    pub sources: Vec<String>,
    pub selected_sources: HashSet<String>,
}

impl SharedState {
    pub fn new(entries: Vec<FeedEntry>, mut sources: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        sources.sort();

        SharedState {
            entries,
            list_state,
            view_mode: ViewMode::List,
            exit: false,
            show_popup: false,
            sources: sources.clone(),
            selected_sources: sources.into_iter().collect(),
        }
    }

    pub fn toggle_show_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }
}
