use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

use crate::{feed, state::SharedState};

pub struct FeedPopup {
    list_state: ListState,
}

impl FeedPopup {
    pub fn new() -> Self {
        FeedPopup {
            list_state: ListState::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, shared_state: &SharedState) {
        match self.list_state.selected() {
            Some(_) => {}
            None => {
                if !shared_state.feeds.is_empty() {
                    self.list_state.select(Some(0))
                }
            }
        }

        let popup_block = Block::bordered()
            .title(" Feeds ")
            .title_bottom(" [Esc/s] Back  [Space] Select/Deselect [d] Delete ")
            .title_alignment(HorizontalAlignment::Center)
            .on_black();
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        if shared_state.feeds.is_empty() {
            let empty_list = Paragraph::new(" No feeds found.").block(popup_block);
            frame.render_widget(empty_list, centered_area);
        } else {
            let items: Vec<ListItem> = shared_state
                .feeds
                .iter()
                .map(|feed| {
                    let is_selected = shared_state.selected_feeds.contains(&feed.title);

                    let marker: Line = if is_selected {
                        Line::from(vec![
                            Span::raw("●").green(),
                            Span::raw(" "),
                            Span::raw(feed.title.clone()),
                        ])
                    } else {
                        Line::from(vec![
                            Span::raw("○"),
                            Span::raw(" "),
                            Span::raw(feed.title.clone()),
                        ])
                    };

                    ListItem::new(marker)
                })
                .collect();

            let list = List::new(items)
                .block(popup_block)
                .highlight_style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");
            frame.render_stateful_widget(list, centered_area, &mut self.list_state);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Char(' ') => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(feed) = shared_state.feeds.get(selected) {
                        if shared_state.selected_feeds.contains(&feed.title) {
                            shared_state.selected_feeds.remove(&feed.title);
                        } else {
                            shared_state.selected_feeds.insert(feed.title.clone());
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.list_state.selected() {
                    let feed_to_delete = shared_state.feeds.get(selected).cloned();

                    if let Some(feed_to_delete) = feed_to_delete {
                        {
                            let content = std::fs::read_to_string("feeds.txt").unwrap();
                            let new_content: Vec<&str> = content
                                .lines()
                                .filter(|line| !line.contains(&feed_to_delete.url))
                                .collect();
                            std::fs::write("feeds.txt", new_content.join("\n")).unwrap();
                        }

                        let (mut new_feeds, new_entries) = feed::run();
                        new_feeds.sort();

                        shared_state.feeds = new_feeds;
                        shared_state.entries = new_entries;
                        shared_state.selected_feeds.remove(&feed_to_delete.title);
                        shared_state.list_state.select(Some(0));
                    }
                }
            }
            KeyCode::Char('s') | KeyCode::Esc => shared_state.toggle_show_feeds_popup(),
            _ => {}
        }
    }
}

pub fn render_feed_button(frame: &mut Frame, feed_area: Rect, show_popup: bool) {
    let border = Block::bordered()
        .border_set(border::THICK)
        .style(match show_popup {
            true => Style::default().fg(Color::Yellow),
            false => Style::default(),
        });
    let feed = Paragraph::new("[s] Feeds").block(border).centered();

    frame.render_widget(feed, feed_area);
}
