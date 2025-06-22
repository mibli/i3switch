#![recursion_limit = "256"] // Required for tests with older serde_json
use clap::{Parser, ValueEnum, Subcommand, ArgAction};
use std::process;

mod backend;
mod converters;
mod linear;
mod logging;
mod navigation;
mod planar;
mod types;

use crate::backend::i3;
use crate::backend::traits::SetFocus;

use clap::{Parser, ValueEnum, Subcommand};

/// i3switch - A simple command-line utility to switch focus in i3 window manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Root command for focus switching
    #[clap(subcommand)]
    root_command: RootCommand,

    /// Wrap around when reaching the edge of the workspace
    #[clap(arg_enum, action=ArgAction::Set, default_value = "nowrap", global = true)]
    wrap: WrapOption,
}

/// Define the subcommand for switching focus to a specific tab/window number
#[derive(Subcommand, Debug)]
enum RootCommand {
    // Move focus to next/prev tab/window
    /// Move focus to next tab/window
    Next,
    /// Move focus to previous tab/window
    Prev,

    // Move focus in a specific direction
    /// Move focus right
    Right,
    /// Move focus down
    Down,
    /// Move focus left
    Left,
    /// Move focus up
    Up,

    // Move focus to a specific tab/window number
    /// Switch focus to a specific tab/window number
    Number {
        /// The tab/window number to switch focus to
        #[clap(value_parser, value_name="num")]
        number: u32,
    },
}

/// Define the wrap option for focus switching
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
#[clap(rename_all = "lower")]
enum WrapOption {
    /// Enable wrap around
    Wrap,
    /// Disable wrap around
    NoWrap,
}

/// Implement conversion from WrapOption to bool
impl From<WrapOption> for bool {
    fn from(option: WrapOption) -> Self {
        match option {
            WrapOption::Wrap => true,
            WrapOption::NoWrap => false,
        }
    }
}

/// Get the linear direction based on the command
fn get_linear_direction(command: &RootCommand) -> Option<linear::Direction> {
    match command {
        RootCommand::Next => Some(linear::Direction::Next),
        RootCommand::Prev => Some(linear::Direction::Prev),
        _ => None,
    }
}

/// Get the planar direction based on the command
fn get_planar_direction(command: &RootCommand) -> Option<planar::Direction> {
    match command {
        RootCommand::Right => Some(planar::Direction::Right),
        RootCommand::Down  => Some(planar::Direction::Down),
        RootCommand::Left  => Some(planar::Direction::Left),
        RootCommand::Up    => Some(planar::Direction::Up),
        _ => None,
    }
}

fn main() {
    // Initialize logging
    logging::init();

    let cli = Cli::parse();

    let wrap = bool::from(cli.wrap);

    let mut backend = i3::Backend::new();

    // Determine the window ID to switch focus to based on the command
    let window_id: u64;
    if let Some(direction) = get_linear_direction(&cli.root_command) {
        logging::info!("Switching focus in linear direction: {:?}", direction);
        window_id = navigation::get_window_to_switch_to(&backend, direction, wrap);
    } else if let Some(direction) = get_planar_direction(&cli.root_command) {
        logging::info!("Switching focus in planar direction: {:?}", direction);
        window_id = navigation::get_window_in_direction(&backend, direction, wrap);
    } else if let RootCommand::Number { number } = &cli.root_command {
        logging::info!("Switching focus to window number: {}", number);
        if wrap {
            logging::warning!("Wrap option is ignored for number switching.");
        }
        window_id = navigation::get_window_of_number(&backend, *number as usize);
    } else {
        logging::critical!("Invalid command provided: {:?}", cli.root_command);
    }

    backend.set_focus(&window_id);

    std::process::exit(0);
}
