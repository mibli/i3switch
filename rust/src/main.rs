#![recursion_limit = "256"] // Required for tests with older serde_json

mod backend;
mod converters;
mod linear;
mod logging;
mod navigation;
mod planar;
mod types;
mod cli;

use crate::backend::*;

fn main() {
    // Initialize logging
    logging::init();

    let cli = cli::get_parsed_command();

    let wrap = cli::wrap(&cli);

    let mut backend = Backend::new(UsedBackend::I3(I3Backend::new()));

    // Determine the window ID to switch focus to based on the command
    let window_id: u64;
    if let Some(direction) = cli::linear_direction(&cli) {
        logging::info!("Switching focus in linear direction: {:?}", direction);
        window_id = navigation::get_window_to_switch_to(&backend, direction, wrap);
    } else if let Some(direction) = cli::planar_direction(&cli) {
        logging::info!("Switching focus in planar direction: {:?}", direction);
        window_id = navigation::get_window_in_direction(&backend, direction, wrap);
    } else if let Some(number) = cli::number(&cli) {
        logging::info!("Switching focus to window number: {}", number);
        if wrap {
            logging::warning!("Wrap option is ignored for number switching.");
        }
        window_id = navigation::get_window_of_number(&backend, number);
    } else {
        logging::critical!("Invalid command provided: {:?}", cli);
    }

    backend.set_focus(&window_id);

    std::process::exit(0);
}
