use crate::types::Windows;

trait GetTabs {
    fn get_tabs(&self) -> Windows;
}

trait GetVisible {
    fn get_windows(&self) -> Windows;
}

trait SetFocus {
    fn set_focus(&self, window_id: &u64) -> Result<(), String>;
}
