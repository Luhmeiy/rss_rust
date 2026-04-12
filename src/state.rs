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
}

impl SharedState {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        SharedState {
            entries,
            list_state,
            view_mode: ViewMode::List,
            exit: false,
        }
    }
}
