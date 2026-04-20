use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    state::{data::DataState, ui::UIState},
    ui::{
        components::list_header::ListHeader,
        panels::{feed_panel::FeedPanel, lists_panel::ListsPanel},
        popups::new_list_popup,
    },
};

#[derive(PartialEq)]
pub enum CursorPosition {
    Feed,
    Lists,
}

pub struct ListLayout {
    cursor_position: CursorPosition,
    pub list_header: ListHeader,
    feed_panel: FeedPanel,
    lists_panel: ListsPanel,
    show_lists_panel: bool,
}

impl ListLayout {
    pub fn new() -> Self {
        ListLayout {
            cursor_position: CursorPosition::Feed,
            list_header: ListHeader::new(),
            feed_panel: FeedPanel::new(),
            lists_panel: ListsPanel::new(),
            show_lists_panel: false,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, data: &mut DataState, ui: &mut UIState) {
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).margin(1);
        let [header_area, body_area] = frame.area().layout(&layout);

        self.list_header.render(frame, header_area, ui);

        let search = self.list_header.get_search();
        let panels_focused = !self.list_header.is_search()
            && !ui.show_feeds_popup
            && !ui.show_new_feed_popup
            && !ui.show_new_list_popup
            && !ui.show_list_selector;

        let display_area: Rect;

        if self.show_lists_panel {
            let body_layout =
                Layout::horizontal([Constraint::Length(30), Constraint::Fill(3)]).spacing(1);
            let [lists_area, display_area_layout] = body_area.layout(&body_layout);

            display_area = display_area_layout;

            let lists_content_layout =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
            let [lists_items_area, button_area] = lists_area.layout(&lists_content_layout);

            self.lists_panel.render(
                frame,
                &data.lists,
                lists_items_area,
                self.cursor_position == CursorPosition::Lists && panels_focused,
            );

            new_list_popup::render_new_list_button(frame, button_area, ui.show_new_list_popup);
        } else {
            display_area = body_area;
        }

        let list_name = self.lists_panel.get_list_state(&data.lists);
        let entries = match list_name {
            "All" => data.entries.clone(),
            "Favorites" => data.favorites.clone(),
            "Bookmarks" => data.bookmarks.clone(),
            list_name => data
                .custom_lists
                .get(list_name)
                .cloned()
                .unwrap_or_default(),
        };

        if self
            .feed_panel
            .needs_update(list_name, &data.selected_feeds, search, entries.len())
        {
            self.feed_panel
                .update_entries(entries, list_name, &data.selected_feeds, search);
        }

        self.feed_panel.render(
            frame,
            data,
            display_area,
            self.cursor_position == CursorPosition::Feed && panels_focused,
        );
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        data: &mut DataState,
        ui: &mut UIState,
    ) {
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
            KeyCode::Char('s') => ui.toggle_show_feeds_popup(),
            KeyCode::Char('a') => ui.toggle_show_new_feed_popup(),
            KeyCode::Char('q') => ui.exit = true,
            _ => match self.cursor_position {
                CursorPosition::Feed => self.feed_panel.handle_key_event(key_event, data, ui),
                CursorPosition::Lists => self.lists_panel.handle_key_event(
                    key_event,
                    &mut self.cursor_position,
                    data,
                    ui,
                ),
            },
        }
    }

    pub fn is_search(&self) -> bool {
        self.list_header.is_search()
    }
}
