use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    state::SharedState,
    ui::{feed_panel, list_header::ListHeader, lists_panel::ListsPanel, new_list_popup},
};

#[derive(PartialEq)]
pub enum CursorPosition {
    Feed,
    Lists,
}

pub struct ListLayout {
    cursor_position: CursorPosition,
    pub list_header: ListHeader,
    lists_panel: ListsPanel,
    show_lists_panel: bool,
}

impl ListLayout {
    pub fn new() -> Self {
        ListLayout {
            cursor_position: CursorPosition::Feed,
            list_header: ListHeader::new(),
            lists_panel: ListsPanel::new(),
            show_lists_panel: false,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, shared_state: &mut SharedState) {
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).margin(1);
        let [header_area, body_area] = frame.area().layout(&layout);

        self.list_header.render(frame, header_area, shared_state);

        let search = self.list_header.get_search();
        let panels_focused = !self.list_header.is_search()
            && !shared_state.show_feeds_popup
            && !shared_state.show_new_feed_popup
            && !shared_state.show_new_list_popup
            && !shared_state.show_list_selector;

        let display_area: Rect;

        if self.show_lists_panel {
            let body_layout =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).spacing(1);
            let [lists_area, display_area_layout] = body_area.layout(&body_layout);

            display_area = display_area_layout;

            let lists_content_layout =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
            let [lists_items_area, button_area] = lists_area.layout(&lists_content_layout);

            self.lists_panel.render(
                frame,
                &shared_state.lists,
                lists_items_area,
                self.cursor_position == CursorPosition::Lists && panels_focused,
            );

            new_list_popup::render_new_list_button(
                frame,
                button_area,
                shared_state.show_new_list_popup,
            );
        } else {
            display_area = body_area;
        }

        let entries = match self.lists_panel.get_list_state(&shared_state.lists) {
            "All" => shared_state.entries.clone(),
            "Favorites" => shared_state.favorites.clone(),
            "Bookmarks" => shared_state.bookmarks.clone(),
            list_name => shared_state
                .custom_lists
                .get(list_name)
                .cloned()
                .unwrap_or_default(),
        };

        feed_panel::render(
            entries,
            frame,
            shared_state,
            display_area,
            search,
            self.cursor_position == CursorPosition::Feed && panels_focused,
        );
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Tab => {
                if self.cursor_position == CursorPosition::Lists && self.show_lists_panel == true {
                    self.cursor_position = CursorPosition::Feed
                } else if self.cursor_position == CursorPosition::Feed
                    && self.show_lists_panel == false
                {
                    self.cursor_position = CursorPosition::Lists
                }

                self.show_lists_panel = !self.show_lists_panel;
            }
            KeyCode::Esc => match self.cursor_position {
                CursorPosition::Feed => {
                    self.cursor_position = CursorPosition::Lists;

                    if !self.show_lists_panel {
                        self.show_lists_panel = true;
                    }
                }
                CursorPosition::Lists => {
                    self.cursor_position = CursorPosition::Feed;

                    if self.show_lists_panel {
                        self.show_lists_panel = false;
                    }
                }
            },
            KeyCode::Char('/') => self.list_header.toggle_input_mode(),
            KeyCode::Char('s') => shared_state.toggle_show_feeds_popup(),
            KeyCode::Char('a') => shared_state.toggle_show_new_feed_popup(),
            KeyCode::Char('q') => shared_state.exit = true,
            _ => match self.cursor_position {
                CursorPosition::Feed => feed_panel::handle_key_event(key_event, shared_state),
                CursorPosition::Lists => self.lists_panel.handle_key_event(
                    key_event,
                    &mut self.cursor_position,
                    shared_state,
                ),
            },
        }
    }

    pub fn is_search(&self) -> bool {
        self.list_header.is_search()
    }
}
