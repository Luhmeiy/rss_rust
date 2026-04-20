use ratatui::{Frame, layout::Rect};

use crate::{
    state::{data::DataState, ui::UIState},
    ui::components::popup::{Popup, PopupState, render_button},
};

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

    fn on_enter(&mut self, data: &mut DataState, _ui: &mut UIState) {
        let list_name = self.state.get_input().get_field().to_string();
        data.add_list(list_name);
        *self.state.get_status() = Some(true);
        self.state.get_input().reset();
    }

    fn on_esc(&mut self, _data: &mut DataState, ui: &mut UIState) {
        ui.toggle_show_new_list_popup();
    }
}

pub fn render_new_list_button(frame: &mut Frame, button_area: Rect, show_popup: bool) {
    render_button(frame, button_area, show_popup, "[n] Add List".to_string());
}
