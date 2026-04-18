use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;

use ratatui::{Frame, layout::Rect};

use crate::{
    feed,
    state::SharedState,
    ui::popup::{Popup, PopupState},
};

pub struct NewFeedPopup {
    state: PopupState,
}

impl NewFeedPopup {
    pub fn new() -> Self {
        NewFeedPopup {
            state: PopupState::new("Add Feed"),
        }
    }
}

impl Popup for NewFeedPopup {
    fn get_state(&mut self) -> &mut PopupState {
        &mut self.state
    }

    fn on_enter(&mut self, shared_state: &mut SharedState) {
        let feed_url = self.state.get_input().get_field().to_string();

        {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("feeds.txt")
                .unwrap();
            writeln!(file, "{feed_url}").unwrap();
        }

        let old_count = shared_state.feeds.len();
        let old_feed_titles: HashSet<String> =
            shared_state.feeds.iter().map(|f| f.title.clone()).collect();
        let (mut new_feeds, new_entries) = feed::run();

        new_feeds.sort();

        for feed in &new_feeds {
            if !old_feed_titles.contains(&feed.title) {
                shared_state.selected_feeds.insert(feed.title.clone());
            }
        }

        shared_state.feeds = new_feeds;
        shared_state.entries = new_entries;
        shared_state.list_state.select(Some(0));

        let success = shared_state.feeds.len() > old_count;
        *self.state.get_status() = Some(success);

        if success {
            self.state.get_input().reset();
        }
    }

    fn on_esc(&mut self, shared_state: &mut SharedState) {
        shared_state.toggle_show_new_feed_popup();
    }
}

pub fn render_new_feed_button(frame: &mut Frame, button_area: Rect, show_popup: bool) {
    crate::ui::popup::render_button(frame, button_area, show_popup, "[a] Add Feed".to_string());
}
