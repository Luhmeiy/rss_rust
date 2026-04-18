pub mod content_view;
pub mod feed_panel;
pub mod feeds_popup;
pub mod input;
pub mod list_header;
pub mod list_layout;
pub mod lists_panel;
pub mod new_feed_popup;
pub mod new_list_popup;
pub mod popup;
pub mod search;

use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    feed::{Feed, FeedEntry},
    state::{SharedState, ViewMode},
    ui::{
        content_view::ContentView, feeds_popup::FeedPopup, list_layout::ListLayout,
        new_feed_popup::NewFeedPopup, new_list_popup::NewListPopup, popup::Popup,
    },
};

pub struct App {
    shared_state: SharedState,
    content_view: ContentView,
    list_layout: ListLayout,
    feeds_popup: FeedPopup,
    new_feed_popup: NewFeedPopup,
    new_list_popup: NewListPopup,
}

impl App {
    pub fn new(feeds: Vec<Feed>, entries: Vec<FeedEntry>) -> Self {
        App {
            shared_state: SharedState::new(entries, feeds),
            content_view: ContentView::new(),
            list_layout: ListLayout::new(),
            feeds_popup: FeedPopup::new(),
            new_feed_popup: NewFeedPopup::new(),
            new_list_popup: NewListPopup::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.shared_state.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.shared_state.view_mode {
            ViewMode::Content => self.content_view.render(frame, &mut self.shared_state),
            ViewMode::List => self.list_layout.render(frame, &mut self.shared_state),
        }

        if self.shared_state.show_feeds_popup {
            self.feeds_popup.render(frame, &self.shared_state);
        }

        if self.shared_state.show_new_feed_popup {
            self.new_feed_popup.render(frame);
        }

        if self.shared_state.show_new_list_popup {
            self.new_list_popup.render(frame);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if self.list_layout.is_search() {
                    self.list_layout
                        .list_header
                        .search
                        .handle_key_event(key_event);
                } else if self.shared_state.show_feeds_popup {
                    self.feeds_popup
                        .handle_key_event(key_event, &mut self.shared_state);
                } else if self.shared_state.show_new_feed_popup {
                    self.new_feed_popup
                        .handle_key_event(key_event, &mut self.shared_state);
                } else if self.shared_state.show_new_list_popup {
                    self.new_list_popup
                        .handle_key_event(key_event, &mut self.shared_state);
                } else {
                    match self.shared_state.view_mode {
                        ViewMode::Content => self
                            .content_view
                            .handle_key_event(key_event, &mut self.shared_state),
                        ViewMode::List => self
                            .list_layout
                            .handle_key_event(key_event, &mut self.shared_state),
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }
}
