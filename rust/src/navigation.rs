use crate::backend::traits::{GetVisible, GetTabs};
use crate::converters;
use crate::linear;
use crate::logging;
use crate::planar;

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
            eprintln!("Error: No windows available to switch to.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Info: No windows available to switch to.");
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
            eprintln!("Error: No windows available to switch to.");
            std::process::exit(1);
        }
    } else {
        eprintln!("Info: No windows available to switch to.");
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

/// Get the linear sequence of windows based on the i3 tree structure.
/// If there are focused floating windows, it will return the sequence of those windows.
/// Otherwise, it will return the sequence of available tabs in the current workspace.
fn get_linear_sequence<B: GetVisible + GetTabs>(backend: &B) -> linear::Sequence {
    let windows = backend.get_visible()
        .expect("Failed to get visible windows from backend");
    let mut floating = converters::floating(&windows);

    logging::debug!("Floating windows: {:?}", floating);

    if converters::any_focused(&floating) {
        logging::debug!("Using floating windows for linear sequence.");
        floating.sort_by_key(|w| w.rect.x);
        converters::as_sequence(&floating)
    } else {
        logging::debug!("Using available tabs for linear sequence.");
        let windows = backend.get_tabs()
            .expect("Failed to get tabs from backend");
        converters::as_sequence(&windows)
    }
}

/// Get the planar arrangement of windows based on the i3 tree structure.
/// If there are focused floating windows, it will return the arrangement of those windows.
/// Otherwise, it will return the arrangement of visible windows in the current workspace.
fn get_planar_arrangement<B: GetVisible>(backend: &B) -> planar::Arrangement {
    let windows = backend.get_visible()
        .expect("Failed to get visible windows from backend");
    let floating = converters::floating(&windows);

    if converters::any_focused(&floating) {
        logging::debug!("Using floating windows for planar arrangement.");
        return converters::as_arrangement(&floating, planar::Relation::Center);
    } else {
        logging::debug!("Using available tiled for planar arrangement.");
        let tiled = converters::tiled(&windows);
        return converters::as_arrangement(&tiled, planar::Relation::Border);
    }
}
