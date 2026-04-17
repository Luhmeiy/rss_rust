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
    pub favorites: Vec<FeedEntry>,
    pub bookmarks: Vec<FeedEntry>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub exit: bool,
    pub show_feeds_popup: bool,
    pub show_new_feed_popup: bool,
    pub feeds: Vec<String>,
    pub selected_feeds: HashSet<String>,
}

impl SharedState {
    pub fn new(entries: Vec<FeedEntry>, mut feeds: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        feeds.sort();

        SharedState {
            entries,
            favorites: Vec::new(),
            bookmarks: Vec::new(),
            list_state,
            view_mode: ViewMode::List,
            exit: false,
            show_feeds_popup: false,
            show_new_feed_popup: false,
            feeds: feeds.clone(),
            selected_feeds: feeds.into_iter().collect(),
        }
    }

    pub fn toggle_show_feeds_popup(&mut self) {
        self.show_feeds_popup = !self.show_feeds_popup;
    }

    pub fn toggle_show_new_feed_popup(&mut self) {
        self.show_new_feed_popup = !self.show_new_feed_popup;
    }
}
