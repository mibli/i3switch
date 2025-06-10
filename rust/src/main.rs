use clap::{Parser, Args, ValueEnum, Subcommand};

/// i3switch - A simple command-line utility to switch focus in i3 window manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Root command for focus switching
    #[command(subcommand)]
    root_command: RootCommand,

    /// Wrap around when reaching the edge of the workspace
    #[arg(
        value_enum,
        action = clap::ArgAction::Set,
        default_value_t = WrapOption::NoWrap,
        required = false,
        global = true
    )]
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
        #[arg(value_name = "num", required = true)]
        number: u32,
    },
}

/// Define the wrap option for focus switching
#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum WrapOption {
    /// Enable wrap around
    Wrap,
    /// Disable wrap around
    #[value(name = "nowrap")]
    NoWrap,
}

#[derive(Debug)]
enum LinearDirection {
    Next,
    Prev,
}

#[derive(Debug)]
enum PlanarDirection {
    Right,
    Down,
    Left,
    Up,
}

impl From<WrapOption> for bool {
    fn from(option: WrapOption) -> Self {
        match option {
            WrapOption::Wrap => true,
            WrapOption::NoWrap => false,
        }
    }
}

fn get_linear_direction(command: &RootCommand) -> Option<LinearDirection> {
    match command {
        RootCommand::Next => Some(LinearDirection::Next),
        RootCommand::Prev => Some(LinearDirection::Prev),
        _ => None,
    }
}

fn get_planar_direction(command: &RootCommand) -> Option<PlanarDirection> {
    match command {
        RootCommand::Right => Some(PlanarDirection::Right),
        RootCommand::Down => Some(PlanarDirection::Down),
        RootCommand::Left => Some(PlanarDirection::Left),
        RootCommand::Up => Some(PlanarDirection::Up),
        _ => None,
    }
}

/// Handle the focus switching based on the provided command and options
fn handle_direction1d(direction: LinearDirection, wrap: bool) {
    println!("Switching focus in direction: {:?}, wrap: {}", direction, wrap);
}

/// Handle the focus switching in a 2D plane based on the provided command and options
fn handle_direction2d(direction: PlanarDirection, wrap: bool) {
    println!("Switching focus in direction: {:?}, wrap: {}", direction, wrap);
}

/// Handle switching focus to a specific tab/window number
fn handle_number(number: u32) {
    // Placeholder for handling number focus switching
    println!("Switching focus to tab/window number: {}", number);
}

fn main() {
    let cli = Cli::parse();

    let wrap = bool::from(cli.wrap);

    if let Some(direction) = get_linear_direction(&cli.root_command) {
        handle_direction1d(direction, wrap);
    } else if let Some(direction) = get_planar_direction(&cli.root_command) {
        handle_direction2d(direction, wrap);
    } else if let RootCommand::Number { number } = &cli.root_command {
        if wrap {
            eprintln!("Error: Wrap option is not applicable for this command.");
        } else {
            handle_number(*number);
        }
    } else {
        eprintln!("Error: Invalid command provided.");
        std::process::exit(1);
    }
}
