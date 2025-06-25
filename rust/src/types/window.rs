use crate::types::Rect;

/// A collection of windows, represented as a vector of `Window` structs.
pub type Windows = Vec<Window>;

/// This enum represents a window in a window manager's tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: u64,
    pub rect: Rect,
    pub focused: bool,
    pub floating: bool,
}

impl From<&Window> for Rect {
    fn from(window: &Window) -> Self {
        window.rect
    }
}
