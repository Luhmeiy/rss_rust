use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::ui::{search::Search, source_popup::render_source_button};

pub struct ListHeader {
    pub search: Search,
}

impl ListHeader {
    pub fn new() -> Self {
        ListHeader {
            search: Search::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, header_area: Rect, show_popup: bool) {
        let layout = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)]);
        let [source_area, search_bar_area] = header_area.layout(&layout);

        render_source_button(frame, source_area, show_popup);
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
