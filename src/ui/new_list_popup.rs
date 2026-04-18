use ratatui::{Frame, layout::Rect};

use crate::state::SharedState;
use crate::ui::popup::{Popup, PopupState};

pub struct NewListPopup {
    state: PopupState,
}

impl NewListPopup {
    pub fn new() -> Self {
        NewListPopup {
            state: PopupState::new("Add List"),
        }
    }
}

impl Popup for NewListPopup {
    fn get_state(&mut self) -> &mut PopupState {
        &mut self.state
    }

    fn on_enter(&mut self, _shared_state: &mut SharedState) {
        *self.state.get_status() = Some(true);
        self.state.get_input().reset();
    }

    fn on_esc(&mut self, shared_state: &mut SharedState) {
        shared_state.toggle_show_new_list_popup();
    }
}

pub fn render_new_list_button(frame: &mut Frame, button_area: Rect, show_popup: bool) {
    crate::ui::popup::render_button(frame, button_area, show_popup, "[n] Add List".to_string());
}
