use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::ListItem};

use crate::{
    fetch,
    state::{data::DataState, ui::UIState},
    ui::{
        components::popup::render_button,
        popups::list_popup::{ListPopup, ListPopupState},
    },
};

pub struct FeedPopup {
    state: ListPopupState,
}

impl FeedPopup {
    pub fn new() -> Self {
        FeedPopup {
            state: ListPopupState::new("Feeds", Some("[d] Delete ".to_string()), "No feeds found."),
        }
    }
}

impl ListPopup for FeedPopup {
    fn get_state(&mut self) -> &mut ListPopupState {
        &mut self.state
    }

    fn render_list(data: &DataState) -> Vec<ListItem<'_>> {
        data.feeds
            .iter()
            .map(|feed| {
                let is_selected = data.selected_feeds.contains(&feed.title);
                Self::render_list_item(is_selected, feed.title.clone())
            })
            .collect()
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, data: &mut DataState, ui: &mut UIState) {
        let list_state = self.state.get_list_state();

        match key_event.code {
            KeyCode::Down => list_state.select_next(),
            KeyCode::Up => list_state.select_previous(),
            KeyCode::Char(' ') => {
                if let Some(selected) = list_state.selected() {
                    if let Some(feed) = data.feeds.get(selected) {
                        if data.selected_feeds.contains(&feed.title) {
                            data.selected_feeds.remove(&feed.title);
                        } else {
                            data.selected_feeds.insert(feed.title.clone());
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected) = list_state.selected() {
                    let feed_to_delete = data.feeds.get(selected).cloned();

                    if let Some(feed_to_delete) = feed_to_delete {
                        {
                            let content = std::fs::read_to_string("feeds.txt").unwrap();
                            let new_content: Vec<&str> = content
                                .lines()
                                .filter(|line| !line.contains(&feed_to_delete.url))
                                .collect();
                            std::fs::write("feeds.txt", new_content.join("\n")).unwrap();
                        }

                        let (mut new_feeds, new_entries) = fetch::loader::run();
                        new_feeds.sort();

                        data.feeds = new_feeds;
                        data.entries = new_entries;
                        data.selected_feeds.remove(&feed_to_delete.title);
                    }
                }
            }
            KeyCode::Char('s') | KeyCode::Esc => ui.toggle_show_feeds_popup(),
            _ => {}
        }
    }
}

pub fn render_feed_button(frame: &mut Frame, button_area: Rect, show_popup: bool) {
    render_button(frame, button_area, show_popup, "[s] Feeds".to_string());
}
