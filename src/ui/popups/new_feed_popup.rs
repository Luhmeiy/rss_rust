use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;

use ratatui::{Frame, layout::Rect};

use crate::{
    fetch,
    state::{data::DataState, ui::UIState},
    ui::components::popup::{Popup, PopupState, render_button},
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

    fn on_enter(&mut self, data: &mut DataState, _ui: &mut UIState) {
        let feed_url = self.state.get_input().get_field().to_string();

        {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("feeds.txt")
                .unwrap();
            writeln!(file, "{feed_url}").unwrap();
        }

        let old_count = data.feeds.len();
        let old_feed_titles: HashSet<String> = data.feeds.iter().map(|f| f.title.clone()).collect();
        let (mut new_feeds, new_entries) = fetch::loader::run();

        new_feeds.sort();

        for feed in &new_feeds {
            if !old_feed_titles.contains(&feed.title) {
                data.selected_feeds.insert(feed.title.clone());
            }
        }

        data.feeds = new_feeds;
        data.entries = new_entries;

        let success = data.feeds.len() > old_count;
        *self.state.get_status() = Some(success);

        if success {
            self.state.get_input().reset();
        }
    }

    fn on_esc(&mut self, _data: &mut DataState, ui: &mut UIState) {
        ui.toggle_show_new_feed_popup();
    }
}

pub fn render_new_feed_button(frame: &mut Frame, button_area: Rect, show_popup: bool) {
    render_button(frame, button_area, show_popup, "[a] Add Feed".to_string());
}
