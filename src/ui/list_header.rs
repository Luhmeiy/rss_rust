use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    state::SharedState,
    ui::{feeds_popup::render_feed_button, new_feed_popup::render_new_feed_button, search::Search},
};

pub struct ListHeader {
    pub search: Search,
}

impl ListHeader {
    pub fn new() -> Self {
        ListHeader {
            search: Search::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, header_area: Rect, shared_state: &SharedState) {
        let layout = Layout::horizontal([
            Constraint::Length(13),
            Constraint::Length(16),
            Constraint::Fill(1),
        ])
        .spacing(1);
        let [feed_area, new_feed_area, search_bar_area] = header_area.layout(&layout);

        render_feed_button(frame, feed_area, shared_state.show_feeds_popup);
        render_new_feed_button(frame, new_feed_area, shared_state.show_new_feed_popup);
        self.search.render(frame, search_bar_area);
    }

    pub fn get_search(&self) -> &str {
        &self.search.get_search()
    }

    pub fn is_search(&self) -> bool {
        self.search.is_search()
    }

    pub fn toggle_input_mode(&mut self) {
        self.search.toggle_input_mode();
    }
}
