use ratatui::widgets::ListState;

use crate::models::feed::FeedEntry;

#[derive(PartialEq)]
pub enum ViewMode {
    List,
    Content,
}

pub struct UIState {
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub exit: bool,
    pub show_feeds_popup: bool,
    pub show_new_feed_popup: bool,
    pub show_new_list_popup: bool,
    pub show_list_selector: bool,
}

impl UIState {
    pub fn new(entries: Vec<FeedEntry>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        UIState {
            list_state,
            view_mode: ViewMode::List,
            exit: false,
            show_feeds_popup: false,
            show_new_feed_popup: false,
            show_new_list_popup: false,
            show_list_selector: false,
        }
    }

    pub fn toggle_show_feeds_popup(&mut self) {
        self.show_feeds_popup = !self.show_feeds_popup;
    }

    pub fn toggle_show_new_feed_popup(&mut self) {
        self.show_new_feed_popup = !self.show_new_feed_popup;
    }

    pub fn toggle_show_new_list_popup(&mut self) {
        self.show_new_list_popup = !self.show_new_list_popup;
    }

    pub fn toggle_show_list_selector(&mut self) {
        self.show_list_selector = !self.show_list_selector;
    }
}
