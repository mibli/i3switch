#![recursion_limit = "256"] // Required for tests with older serde_json

mod backend;
mod linear;
mod logging;
mod navigation;
mod planar;
mod types;
mod cli;

use crate::backend::*;

fn main() {
    let cli = cli::Cli::parse(std::env::args().collect());

    let wrap = cli.wrap;

    let mut backend: Backend;
    match cli.backend {
        #[cfg(feature = "i3")]
        cli::UseBackend::I3 => {
            logging::info!("Using I3 backend.");
            backend = Backend::new(UsedBackend::I3(I3Backend::new()));
        }
        #[cfg(feature = "wmctl")]
        cli::UseBackend::WmCtl => {
            logging::info!("Using WmCtl backend.");
            backend = Backend::new(UsedBackend::WmCtl(WmctlBackend::new()));
        }
        #[cfg(feature = "xcb")]
        cli::UseBackend::Xcb => {
            logging::info!("Using XCB backend.");
            backend = Backend::new(UsedBackend::Xcb(XcbBackend::new()));
        }
    }

    // Determine the window ID to switch focus to based on the command
    let window_id: u64;
    if let Some(direction) = cli.linear_direction() {
        logging::info!("Switching focus in linear direction: {:?}", direction);
        window_id = navigation::get_window_to_switch_to(&backend, direction, wrap);
    } else if let Some(direction) = cli.planar_direction() {
        logging::info!("Switching focus in planar direction: {:?}", direction);
        window_id = navigation::get_window_in_direction(&backend, direction, wrap);
    } else if let Some(number) = cli.number {
        logging::info!("Switching focus to window number: {}", number);
        if wrap {
            logging::warning!("Wrap option is ignored for number switching.");
        }
        window_id = navigation::get_window_of_number(&backend, number);
    } else {
        unreachable!("No valid command provided. This should not happen.");
    }

    backend.set_focus(&window_id);

    std::process::exit(0);
}
