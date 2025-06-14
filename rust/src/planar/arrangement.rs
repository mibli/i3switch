use crate::planar::Rect;
use crate::planar::Relation;
use crate::planar::Direction;
use crate::planar::alignment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: u64,
    pub rect: Rect,
}

pub struct Arrangement {
    pub windows:  Vec<Window>,
    pub relation: Relation,
    pub current:  usize,
}

impl Arrangement {
    pub fn new(windows: Vec<Window>, current: Option<usize>, relation: Option<Relation>) -> Self {
        let relation = relation.unwrap_or(Relation::Border);
        let current = current.unwrap_or(0);
        // Ensure current index is within bounds
        let current = if current < windows.len() { current } else { 0 };
        Arrangement {
            windows,
            relation,
            current,
        }
    }

    /// Returns the next window in the specified direction, if it exists.
    pub fn next(&mut self, direction: Direction) -> Option<&Window> {
        let properties = alignment::get_properties(self.relation, direction);
        let rects: Vec<&Rect> = self.windows.iter().map(|w| &w.rect).collect();
        let next_index = alignment::next_in_direction(&rects, rects[self.current], &properties);
        return Some(&self.windows[next_index?]);
    }

    /// Returns the first window on the axis of the specified direction, if it exists.
    pub fn first(&self, direction: Direction) -> Option<&Window> {
        let properties = alignment::get_properties(self.relation, direction);
        let rects: Vec<&Rect> = self.windows.iter().map(|w| &w.rect).collect();
        let first_index = alignment::first_of_direction(&rects, rects[self.current], &properties);
        return Some(&self.windows[first_index?]);
    }
}
