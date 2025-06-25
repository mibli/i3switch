use crate::types::Windows;

pub trait GetTabs {
    fn get_tabs(&self) -> Result<Windows, String>;
}

pub trait GetVisible {
    fn get_visible(&self) -> Result<Windows, String>;
}

pub trait SetFocus {
    fn set_focus(& mut self, window_id: &u64);
}
