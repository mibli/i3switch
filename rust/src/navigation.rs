use crate::backend::traits::{GetVisible, GetTabs};
use crate::linear;
use crate::logging::OptionExt;
use crate::logging;
use crate::planar;
use crate::types::Windows;

// --------------------------------------
// Public functions for window navigation
// --------------------------------------

/// Get window to switch to in tabbed, stacked or floating layout.
/// If `wrap` is true, it will wrap around to the first/last window if no next/previous window is
/// available.
pub fn get_window_to_switch_to<B: GetVisible + GetTabs>(backend: &B, direction: linear::Direction, wrap: bool) -> u64 {
    let sequence = get_linear_sequence(backend);
    if let Some(window_id) = sequence.next(direction) {
        window_id
    } else if wrap {
        if let Some(window_id) = sequence.first(direction) {
            window_id
        } else {
            logging::error!("No windows available to switch to.");
            std::process::exit(1);
        }
    } else {
        logging::info!("No windows available to switch to.");
        std::process::exit(0);
    }
}

/// Get window to switch based on their position in the planar arrangement.
/// If `wrap` is true, it will wrap around to the first/last window if no next/previous window is
/// available.
/// If no window is available in the specified direction, it will print an error message and exit
/// the program.
/// If `wrap` is false, it will print an info message and exit the program without switching.
pub fn get_window_in_direction<B: GetVisible>(backend: &B, direction: planar::Direction, wrap: bool) -> u64 {
    let mut arrangement = get_planar_arrangement(backend);
    if let Some(window) = arrangement.next(direction) {
        window.id
    } else if wrap {
        if let Some(window) = arrangement.first(direction) {
            window.id
        } else {
            logging::error!("No windows available to switch to.");
            std::process::exit(1);
        }
    } else {
        logging::info!("No windows available to switch to.");
        std::process::exit(0);
    }
}

/// Get the window ID of a specific window number for tabbed, stacked and floating layouts.
/// If the number is out of bounds, it will print an error message and exit the program.
pub fn get_window_of_number<B: GetVisible + GetTabs>(backend: &B, number: usize) -> u64 {
    let sequence = get_linear_sequence(backend);
    if number >= sequence.size() {
        logging::critical!("Invalid window number: {}. There are only {} windows available.", number, sequence.size());
    }
    sequence[number]
}

// ----------------------------------------------------------
// Helper functions for extracting and converting window data
// ----------------------------------------------------------

/// Get the linear sequence of windows based on the i3 tree structure.
/// If there are focused floating windows, it will return the sequence of those windows.
/// Otherwise, it will return the sequence of available tabs in the current workspace.
fn get_linear_sequence<B: GetVisible + GetTabs>(backend: &B) -> linear::Sequence {
    let windows = backend.get_visible()
        .expect("Failed to get visible windows from backend");
    let mut floating = floating(&windows);

    logging::debug!("Floating windows: {:?}", floating);

    if any_focused(&floating) {
        logging::debug!("Using floating windows for linear sequence.");
        floating.sort_by_key(|w| w.rect.x);
        as_sequence(&floating)
    } else {
        logging::debug!("Using available tabs for linear sequence.");
        let windows = backend.get_tabs()
            .expect("Failed to get tabs from backend");
        as_sequence(&windows)
    }
}

/// Get the planar arrangement of windows based on the i3 tree structure.
/// If there are focused floating windows, it will return the arrangement of those windows.
/// Otherwise, it will return the arrangement of visible windows in the current workspace.
fn get_planar_arrangement<B: GetVisible>(backend: &B) -> planar::Arrangement {
    let windows = backend.get_visible()
        .expect("Failed to get visible windows from backend");
    let floating = floating(&windows);

    if any_focused(&floating) {
        logging::debug!("Using floating windows for planar arrangement.");
        return as_arrangement(floating, planar::Relation::Center);
    } else {
        logging::debug!("Using available tiled for planar arrangement.");
        let tiled = tiled(&windows);
        return as_arrangement(tiled, planar::Relation::Border);
    }
}

/// Returns a collection of windows that are floating, i.e., those that are not tiled.
fn floating(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| w.floating)
        .cloned()
        .collect()
}

/// Returns a collection of windows that are not floating, i.e., those that are tiled.
fn tiled(windows: &Windows) -> Windows {
    windows.iter()
        .filter(|w| !w.floating)
        .cloned()
        .collect()
}

/// Converts a JSON node to a `planar::Arrangement`.
/// This function assumes that the node represents a workspace or root node
/// and contains a list of windows.
fn as_arrangement(windows: Windows, relation: planar::Relation) -> planar::Arrangement {
    let current = focused_index(&windows).unwrap_or(0);
    planar::Arrangement::new(windows, Some(current), Some(relation))
}

/// Converts a collection of `Windows` to a `linear::Sequence`.
/// This function creates a sequence of window IDs and marks the focused window by its index.
fn as_sequence(windows: &Windows) -> linear::Sequence {
    let focused = focused_index(windows).unwrap_or(0);
    return linear::Sequence::new(windows.iter().map(|w| w.id).collect(), focused);
}

/// Returns whether any window in the provided `Windows` is focused.
fn any_focused(windows: &Windows) -> bool {
    windows.iter().any(|w| w.focused)
}

/// Returns the index of the currently focused window, if any.
fn focused_index(windows: &Windows) -> Option<usize> {
    windows.iter().position(|w| w.focused).wanted(
        format!("No focused window found in windows: {:?}", windows).as_str())
}

// -----
// Tests
// -----

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
