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
    models::feed::FeedEntry,
    state::{data::DataState, ui::UIState, ui::ViewMode},
};

pub fn render(
    entries: Vec<FeedEntry>,
    frame: &mut Frame,
    data: &DataState,
    ui: &mut UIState,
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
        .title_bottom(" [↑/↓] Nav  [Tab] Lists  [o] Open  [v] View  [f] Fav  [b] Book  [l] Add to List  [q] Quit ")
        .title_alignment(HorizontalAlignment::Center)
        .border_set(border::THICK)
        .border_style(border_style);

    let items: Vec<ListItem> = entries
        .iter()
        .filter(|item| {
            data.selected_feeds.contains(&item.feed())
                && (item.title().to_lowercase().contains(search)
                    || item.summary().to_lowercase().contains(search))
        })
        .map(|item| {
            let desc = item.summary();
            let title = item.title();

            let fav_indicator = if data.favorites.contains(item) {
                "[★]"
            } else {
                ""
            };

            let saved_indicator = if data.bookmarks.contains(item) {
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

        frame.render_stateful_widget(list, body_area, &mut ui.list_state);
    }
}

pub fn handle_key_event(key_event: KeyEvent, data: &mut DataState, ui: &mut UIState) {
    match key_event.code {
        KeyCode::Down => ui.list_state.select_next(),
        KeyCode::Up => ui.list_state.select_previous(),
        KeyCode::Char('v') => ui.view_mode = ViewMode::Content,
        KeyCode::Char('l') => {
            if let Some(selected) = ui.list_state.selected() {
                if let Some(entry) = data.entries.get(selected) {
                    data.selected_entry = Some(entry.clone());
                }
            }

            ui.toggle_show_list_selector();
        }
        KeyCode::Char('b') => {
            if let Some(selected) = ui.list_state.selected() {
                if let Some(entry) = data.entries.get(selected) {
                    if !data.bookmarks.contains(entry) {
                        data.bookmarks.push(entry.clone());
                    } else {
                        data.bookmarks.retain(|x| x != entry)
                    }
                }
            }
        }
        KeyCode::Char('f') => {
            if let Some(selected) = ui.list_state.selected() {
                if let Some(entry) = data.entries.get(selected) {
                    if !data.favorites.contains(entry) {
                        data.favorites.push(entry.clone());
                    } else {
                        data.favorites.retain(|x| x != entry)
                    }
                }
            }
        }
        KeyCode::Char('o') => {
            if let Some(selected) = ui.list_state.selected() {
                if let Some(entry) = data.entries.get(selected) {
                    if let Some(link) = entry.entry.links.first() {
                        let _ = open::that(&link.href);
                    }
                }
            }
        }
        _ => {}
    }
}
