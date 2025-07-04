/// This file provides functions to convert JSON nodes representing windows in a window manager's
/// tree structure. It includes functions to extract visible nodes, available tabs, floating and
/// tiled windows, and to convert these nodes into a structured format for further processing.
/// It also includes functions to determine the layout of nodes and to find focused windows in
/// the tree structure.
///
/// This module is part of a window manager's arrangement system, which allows for
/// manipulating and querying the layout of windows in a graphical user interface.

use crate::linear;
use crate::planar;
use crate::logging::OptionExt;
use crate::types::Windows;

/// Returns a collection of windows that are floating, i.e., those that are not tiled.
pub fn floating(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| w.floating)
        .cloned()
        .collect()
}

/// Returns a collection of windows that are not floating, i.e., those that are tiled.
pub fn tiled(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| !w.floating)
        .cloned()
        .collect()
}

/// Converts a JSON node to a `planar::Arrangement`.
/// This function assumes that the node represents a workspace or root node
/// and contains a list of windows.
pub fn as_arrangement(windows: Windows, relation: planar::Relation) -> planar::Arrangement {
    let current = focused_index(&windows).unwrap_or(0);
    planar::Arrangement::new(windows, Some(current), Some(relation))
}

/// Converts a collection of `Windows` to a `linear::Sequence`.
/// This function creates a sequence of window IDs and marks the focused window by its index.
pub fn as_sequence(windows: &Windows) -> linear::Sequence {
    let focused = focused_index(windows).unwrap_or(0);
    return linear::Sequence::new(windows.iter().map(|w| w.id).collect(), focused);
}

/// Returns whether any window in the provided `Windows` is focused.
pub fn any_focused(windows: &Windows) -> bool {
    windows.iter().any(|w| w.focused)
}

/// Returns the index of the currently focused window, if any.
fn focused_index(windows: &Windows) -> Option<usize> {
    windows.iter().position(|w| w.focused).wanted(
        format!("No focused window found in windows: {:?}", windows).as_str())
}

#[cfg(test)]
mod tests {
    use crate::types::Rect;
    use crate::types::Window;
    use super::*;

    /// Tests for floating and tiled windows.
    #[test]
    fn test_floating_and_tiled() {
        let windows = vec![
            Window { id: 1, rect: Rect {x: 0, y: 0, w: 100, h: 100}, focused: true, floating: false },
            Window { id: 2, rect: Rect {x: 100, y: 100, w: 200, h: 200}, focused: false, floating: true },
        ];
        let floating_windows = floating(&windows);
        let tiled_windows = tiled(&windows);
        assert_eq!(floating_windows.len(), 1);
        assert_eq!(tiled_windows.len(), 1);
    }

    /// Tests for checking if any window is focused.
    #[test]
    fn test_any_focused() {
        let windows = vec![
            Window { id: 1, rect: Rect {x: 0, y: 0, w: 100, h: 100}, focused: true, floating: false },
            Window { id: 2, rect: Rect {x: 100, y: 100, w: 200, h: 200}, focused: false, floating: true },
        ];
        assert!(any_focused(&windows));
    }

    /// Tests for visible nodes extraction.
    #[test]
    fn test_as_arrangement() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        let arrangement = as_arrangement(windows, planar::Relation::Border);
        assert_eq!(arrangement.windows.len(), 2);
        assert_eq!(arrangement.current, 0);
    }

    /// Tests for focused index extraction.
    /// We expect the function to return the index of the first focused window in the provided
    #[test]
    fn test_focused_index() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        assert_eq!(focused_index(&windows), Some(0));

        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: false, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        assert_eq!(focused_index(&windows), None);
    }

    /// Tests sequence conversion.
    /// We expect the function to return a sequence of window IDs and the index of the focused
    /// window.
    #[test]
    fn test_as_sequence() {
        let windows = vec![
            Window { id: 1, rect: Rect { x: 0, y: 0, w: 100, h: 100 }, focused: true, floating: false },
            Window { id: 2, rect: Rect { x: 100, y: 100, w: 200, h: 200 }, focused: false, floating: true },
        ];
        let sequence = as_sequence(&windows);
        assert_eq!(sequence[0], 1);
        assert_eq!(sequence[1], 2);
    }
}
