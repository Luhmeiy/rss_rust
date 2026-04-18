use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{HorizontalAlignment, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::{
    feed::FeedEntry,
    state::{SharedState, ViewMode},
};

pub fn render(
    entries: Vec<FeedEntry>,
    frame: &mut Frame,
    shared_state: &mut SharedState,
    body_area: Rect,
    search: &str,
    is_active: bool,
) {
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let border = Block::bordered()
        .title(" RSS Feed ")
        .title_bottom(" [↑/↓] Nav  [Tab] Lists  [o] Open  [v] View  [f] Fav  [b] Book  [q] Quit ")
        .title_alignment(HorizontalAlignment::Center)
        .border_set(border::THICK)
        .border_style(border_style);

    let items: Vec<ListItem> = entries
        .iter()
        .filter(|item| {
            shared_state.selected_feeds.contains(&item.feed())
                && (item.title().to_lowercase().contains(search)
                    || item.summary().to_lowercase().contains(search))
        })
        .map(|item| {
            let desc = item.summary();
            let title = item.title();

            let fav_indicator = if shared_state.favorites.contains(item) {
                "[★]"
            } else {
                ""
            };

            let saved_indicator = if shared_state.bookmarks.contains(item) {
                "[→]"
            } else {
                ""
            };

            let formatted_title = match (fav_indicator.is_empty(), saved_indicator.is_empty()) {
                (false, false) => format!("{} {} {}", fav_indicator, saved_indicator, title),
                (false, true) => format!("{} {}", fav_indicator, title),
                (true, false) => format!("{} {}", saved_indicator, title),
                (true, true) => title,
            };

            let mut lines = vec![
                Line::from(formatted_title),
                Line::from(format!("Feed: {}", item.feed())),
                Line::from(format!("Date: {}", item.date())),
            ];

            if !desc.is_empty() {
                lines.push(Line::from(desc));
            }

            lines.push(Line::from(""));
            ListItem::new(lines)
        })
        .collect();

    if items.is_empty() {
        let empty_list = Paragraph::new(" No entries found.").block(border);
        frame.render_widget(empty_list, body_area);
    } else {
        let list = List::new(items)
            .block(border)
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, body_area, &mut shared_state.list_state);
    }
}

pub fn handle_key_event(key_event: KeyEvent, shared_state: &mut SharedState) {
    match key_event.code {
        KeyCode::Down => shared_state.list_state.select_next(),
        KeyCode::Up => shared_state.list_state.select_previous(),
        KeyCode::Char('v') => shared_state.view_mode = ViewMode::Content,
        KeyCode::Char('b') => {
            if let Some(selected) = shared_state.list_state.selected() {
                if let Some(entry) = shared_state.entries.get(selected) {
                    if !shared_state.bookmarks.contains(entry) {
                        shared_state.bookmarks.push(entry.clone());
                    } else {
                        shared_state.bookmarks.retain(|x| x != entry)
                    }
                }
            }
        }
        KeyCode::Char('f') => {
            if let Some(selected) = shared_state.list_state.selected() {
                if let Some(entry) = shared_state.entries.get(selected) {
                    if !shared_state.favorites.contains(entry) {
                        shared_state.favorites.push(entry.clone());
                    } else {
                        shared_state.favorites.retain(|x| x != entry)
                    }
                }
            }
        }
        KeyCode::Char('o') => {
            if let Some(selected) = shared_state.list_state.selected() {
                if let Some(entry) = shared_state.entries.get(selected) {
                    if let Some(link) = entry.entry.links.first() {
                        let _ = open::that(&link.href);
                    }
                }
            }
        }
        _ => {}
    }
}
