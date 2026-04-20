use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    models::feed::{Feed, FeedEntry},
    state::{data::DataState, ui::UIState, ui::ViewMode},
    ui::{
        components::popup::Popup,
        popups::{
            add_to_list_popup::AddToListPopup, feeds_popup::FeedPopup, list_popup::ListPopup,
            new_feed_popup::NewFeedPopup, new_list_popup::NewListPopup,
        },
        views::{content_view::ContentView, list_layout::ListLayout},
    },
};

pub struct App {
    data: DataState,
    ui: UIState,
    content_view: ContentView,
    list_layout: ListLayout,
    feeds_popup: FeedPopup,
    new_feed_popup: NewFeedPopup,
    new_list_popup: NewListPopup,
    add_to_list_popup: AddToListPopup,
}

impl App {
    pub fn new(feeds: Vec<Feed>, entries: Vec<FeedEntry>) -> Self {
        App {
            data: DataState::new(entries.clone(), feeds),
            ui: UIState::new(entries),
            content_view: ContentView::new(),
            list_layout: ListLayout::new(),
            feeds_popup: FeedPopup::new(),
            new_feed_popup: NewFeedPopup::new(),
            new_list_popup: NewListPopup::new(),
            add_to_list_popup: AddToListPopup::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.ui.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.ui.view_mode {
            ViewMode::Content => self
                .content_view
                .render(frame, &mut self.data, &mut self.ui),
            ViewMode::List => self.list_layout.render(frame, &mut self.data, &mut self.ui),
        }

        if self.ui.show_feeds_popup {
            self.feeds_popup.render(frame, &self.data);
        }

        if self.ui.show_new_feed_popup {
            self.new_feed_popup.render(frame);
        }

        if self.ui.show_new_list_popup {
            self.new_list_popup.render(frame);
        }

        if self.ui.show_list_selector {
            self.add_to_list_popup.render(frame, &mut self.data);
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
                } else if self.ui.show_feeds_popup {
                    self.feeds_popup
                        .handle_key_event(key_event, &mut self.data, &mut self.ui);
                } else if self.ui.show_new_feed_popup {
                    self.new_feed_popup
                        .handle_key_event(key_event, &mut self.data, &mut self.ui);
                } else if self.ui.show_new_list_popup {
                    self.new_list_popup
                        .handle_key_event(key_event, &mut self.data, &mut self.ui);
                } else if self.ui.show_list_selector {
                    self.add_to_list_popup.handle_key_event(
                        key_event,
                        &mut self.data,
                        &mut self.ui,
                    );
                } else {
                    match self.ui.view_mode {
                        ViewMode::Content => self.content_view.handle_key_event(
                            key_event,
                            &mut self.data,
                            &mut self.ui,
                        ),
                        ViewMode::List => self.list_layout.handle_key_event(
                            key_event,
                            &mut self.data,
                            &mut self.ui,
                        ),
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }
}
