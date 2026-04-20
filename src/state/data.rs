use std::collections::{HashMap, HashSet};

use crate::models::feed::{Feed, FeedEntry};

pub struct DataState {
    pub entries: Vec<FeedEntry>,
    pub favorites: Vec<FeedEntry>,
    pub bookmarks: Vec<FeedEntry>,
    pub lists: Vec<String>,
    pub custom_lists: HashMap<String, Vec<FeedEntry>>,
    pub selected_entry: Option<FeedEntry>,
    pub feeds: Vec<Feed>,
    pub selected_feeds: HashSet<String>,
}

impl DataState {
    pub fn new(entries: Vec<FeedEntry>, feeds: Vec<Feed>) -> Self {
        DataState {
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
            feeds: feeds.clone(),
            selected_feeds: feeds.into_iter().map(|f| f.title).collect(),
        }
    }

    pub fn add_list(&mut self, name: String) {
        self.lists.push(name.clone());
        self.custom_lists.insert(name, Vec::new());
    }
}
