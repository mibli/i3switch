/// A simple rectangle structure with methods to calculate its extents and middle points in a 2D
/// space.

#[cfg(feature = "i3")]
use serde::Deserialize;

/// This structure is used to represent a rectangle defined by its top-left corner (x, y) and its
/// width (w) and height (h).
#[cfg(feature = "i3")]
#[derive(Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    #[cfg(feature = "i3")]
    #[serde(rename = "width")]
    pub w: i32,
    #[cfg(feature = "i3")]
    #[serde(rename = "height")]
    pub h: i32,
}

impl Rect {
    #[allow(dead_code)]
    pub fn left(&self) -> i32 {
        self.x
    }
    #[allow(dead_code)]
    pub fn right(&self) -> i32 {
        self.x + self.w
    }
    #[allow(dead_code)]
    pub fn top(&self) -> i32 {
        self.y
    }
    #[allow(dead_code)]
    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }
    #[allow(dead_code)]
    pub fn vertical_middle(&self) -> i32 {
        self.y + self.h / 2
    }
    #[allow(dead_code)]
    pub fn horizontal_middle(&self) -> i32 {
        self.x + self.w / 2
    }
}

impl ToString for Rect {
    fn to_string(&self) -> String {
        format!("Rect(x: {}, y: {}, w: {}, h: {})", self.x, self.y, self.w, self.h)
    }
}
