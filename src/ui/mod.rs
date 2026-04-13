pub mod content_view;
pub mod list_header;
pub mod list_view;
pub mod search;
pub mod source_popup;

use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    feed::FeedEntry,
    state::{SharedState, ViewMode},
    ui::{content_view::ContentView, list_view::ListView, source_popup::SourcePopup},
};

pub struct App {
    shared_state: SharedState,
    content_view: ContentView,
    list_view: ListView,
    source_popup: SourcePopup,
}

impl App {
    pub fn new(sources: Vec<String>, entries: Vec<FeedEntry>) -> Self {
        App {
            shared_state: SharedState::new(entries, sources),
            content_view: ContentView::new(),
            list_view: ListView::new(),
            source_popup: SourcePopup::new(),
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
        if self.shared_state.view_mode == ViewMode::List {
            self.list_view.render(frame, &mut self.shared_state);
        } else {
            self.content_view.render(frame, &mut self.shared_state);
        }

        if self.shared_state.show_popup {
            self.source_popup.render(frame, &self.shared_state);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if self.list_view.is_search() {
                    self.list_view
                        .list_header
                        .search
                        .handle_key_event(key_event);
                } else if self.shared_state.show_popup {
                    self.source_popup
                        .handle_key_event(key_event, &mut self.shared_state);
                } else {
                    match self.shared_state.view_mode {
                        ViewMode::List => self
                            .list_view
                            .handle_key_event(key_event, &mut self.shared_state),
                        ViewMode::Content => self
                            .content_view
                            .handle_key_event(key_event, &mut self.shared_state),
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }
}
