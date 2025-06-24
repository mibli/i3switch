use clap::{Parser, ValueEnum, Subcommand, ArgAction};
use crate::planar;
use crate::linear;

// Public interface for the CLI module

/// We cant use Cli::parse() directly in the main function because it requires
/// the command line arguments to be passed in, which is not possible
/// in a library context. Instead, we define this function to be used
/// in the main function to parse the command line arguments.
pub fn get_parsed_command() -> Cli {
    Cli::parse()
}

/// Determine if the wrap option is enabled
pub fn wrap(cli: &Cli) -> bool {
    match cli.wrap {
        WrapOption::Wrap => true,
        WrapOption::NoWrap => false,
    }
}

/// Convert the root command to a planar direction if applicable
pub fn planar_direction(cli: &Cli) -> Option<planar::Direction> {
    match cli.root_command {
        RootCommand::Right => Some(planar::Direction::Right),
        RootCommand::Down  => Some(planar::Direction::Down),
        RootCommand::Left  => Some(planar::Direction::Left),
        RootCommand::Up    => Some(planar::Direction::Up),
        _ => None,
    }
}

/// Convert the root command to a linear direction if applicable
pub fn linear_direction(cli: &Cli) -> Option<linear::Direction> {
    match cli.root_command {
        RootCommand::Next => Some(linear::Direction::Next),
        RootCommand::Prev => Some(linear::Direction::Prev),
        _ => None,
    }
}

/// Get the specific tab/window number to switch focus to if applicable
pub fn number(cli: &Cli) -> Option<usize> {
    match cli.root_command {
        RootCommand::Number { number } => Some(number),
        _ => None,
    }
}

// Define the command-line interface (CLI) structure using Clap

/// i3switch - A simple command-line utility to switch focus in i3 window manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Root command for focus switching
    #[clap(subcommand)]
    root_command: RootCommand,

    /// Wrap around when reaching the edge of the workspace
    #[clap(arg_enum, action=ArgAction::Set, default_value = "nowrap", global = true)]
    wrap: WrapOption,

    /// Wrap around when reaching the edge of the workspace
    #[clap(arg_enum, action=ArgAction::Set, default_value = "i3", global = true)]
    pub backend: BackendOption,
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
        number: usize,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum BackendOption {
    /// Use the i3 IPC backend
    I3,
    /// Use the sway IPC backend
    WmCtrl,
    /// Use the xcb backend
    Xcb,
}
