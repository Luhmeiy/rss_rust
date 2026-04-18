use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::feed::{Feed, FeedEntry};

#[derive(PartialEq)]
pub enum ViewMode {
    List,
    Content,
}

pub struct SharedState {
    pub entries: Vec<FeedEntry>,
    pub favorites: Vec<FeedEntry>,
    pub bookmarks: Vec<FeedEntry>,
    pub lists: Vec<String>,
    pub custom_lists: HashMap<String, Vec<FeedEntry>>,
    pub selected_entry: Option<FeedEntry>,
    pub list_state: ListState,
    pub view_mode: ViewMode,
    pub exit: bool,
    pub show_feeds_popup: bool,
    pub show_new_feed_popup: bool,
    pub show_new_list_popup: bool,
    pub show_list_selector: bool,
    pub feeds: Vec<Feed>,
    pub selected_feeds: HashSet<String>,
}

impl SharedState {
    pub fn new(entries: Vec<FeedEntry>, feeds: Vec<Feed>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }

        SharedState {
            entries,
            favorites: Vec::new(),
            bookmarks: Vec::new(),
            lists: Vec::from([
                "All".to_string(),
                "Favorites".to_string(),
                "Bookmarks".to_string(),
            ]),
            custom_lists: HashMap::new(),
            selected_entry: None,
            list_state,
            view_mode: ViewMode::List,
            exit: false,
            show_feeds_popup: false,
            show_new_feed_popup: false,
            show_new_list_popup: false,
            show_list_selector: false,
            feeds: feeds.clone(),
            selected_feeds: feeds.into_iter().map(|f| f.title).collect(),
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

    pub fn add_list(&mut self, name: String) {
        self.lists.push(name.clone());
        self.custom_lists.insert(name, Vec::new());
    }
}
