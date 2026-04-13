use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::{
    state::{SharedState, ViewMode},
    ui::list_header::ListHeader,
};

pub struct ListView {
    pub list_header: ListHeader,
}

impl ListView {
    pub fn new() -> Self {
        ListView {
            list_header: ListHeader::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, shared_state: &mut SharedState) {
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).margin(1);
        let [header_area, body_area] = frame.area().layout(&layout);

        self.list_header
            .render(frame, header_area, shared_state.show_popup);

        let border = Block::bordered()
            .title(" RSS Feed ")
            .title_bottom(" [↑/↓] Navigate  [o] Open  [v] View  [/] Search  [q] Quit ")
            .title_alignment(HorizontalAlignment::Center)
            .border_set(border::THICK);

        let search = &self.list_header.get_search().to_lowercase();

        let items: Vec<ListItem> = shared_state
            .entries
            .iter()
            .filter(|item| {
                shared_state.selected_sources.contains(&item.source())
                    && (item.title().to_lowercase().contains(search)
                        || item.summary().to_lowercase().contains(search))
            })
            .map(|item| {
                let desc = item.summary();

                let mut lines = vec![
                    Line::from(item.title()),
                    Line::from(format!("Source: {}", item.source())),
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

    pub fn handle_key_event(&mut self, key_event: KeyEvent, shared_state: &mut SharedState) {
        match key_event.code {
            KeyCode::Down => shared_state.list_state.select_next(),
            KeyCode::Up => shared_state.list_state.select_previous(),
            KeyCode::Char('/') => self.list_header.toggle_input_mode(),
            KeyCode::Char('s') => shared_state.toggle_show_popup(),
            KeyCode::Char('o') => {
                if let Some(selected) = shared_state.list_state.selected() {
                    if let Some(entry) = shared_state.entries.get(selected) {
                        if let Some(link) = entry.entry.links.first() {
                            let _ = open::that(&link.href);
                        }
                    }
                }
            }
            KeyCode::Char('v') => shared_state.view_mode = ViewMode::Content,
            KeyCode::Char('q') => shared_state.exit = true,
            _ => {}
        }
    }

    pub fn is_search(&self) -> bool {
        self.list_header.is_search()
    }
}
