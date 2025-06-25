use crate::planar;
use crate::linear;
use std::slice::Iter;

macro_rules! die {
    ($code:expr, $format:expr $(, $args:expr)*) => {{
        eprintln!($format $(, $args)*);
        std::process::exit($code);
    }};
}

pub struct Cli {
    pub backend: UseBackend,
    pub command: String,
    pub number: Option<usize>,
    pub wrap: bool,
}

// The help message will be built at runtime, because rust does not support
// macro expansion in const contexts, and we want to use the `#[cfg]` attributes
// to conditionally include backend options based on the features enabled.
// This should not have a significant performance impact, as the help message is only
// used when the user requests it, and it is built once at runtime.
const HELP: &'static [&'static str] = &["
i3switch - A simple command-line utility to switch focus in i3 window manager

Usage: i3switch (<OPTION>|[<BACKEND>] <COMMAND> [wrap])

Backends:\n",
#[cfg(feature = "i3")]
"  -i3           Use i3 backend (default)\n",
#[cfg(feature = "wmctl")]
"  -wm           Use wmctl backend\n",
#[cfg(feature = "xcb")]
"  -xcb          Use xcb backend\n",
"
Commands:
  next          Move focus to next tab/window
  prev          Move focus to previous tab/window
  right         Move focus right
  down          Move focus down
  left          Move focus left
  up            Move focus up
  number NUM    Switch focus to tab/window number NUM

Arguments:
  [wrap]        Wrap around when reaching the edge of the workspace

Options:
  -h, --help    Print help (see a summary with '-h')
  -V, --version Print version
"];


impl Cli {
    pub fn parse(args: Vec<String>) -> Self {
        let mut command = String::new();
        let mut number: Option<usize> = None;
        let mut wrap = false;

        let mut arg_index = 1;

        // Handle the boring help and version flags ahead

        match args.get(arg_index) {
            Some(arg) if arg == "-h" || arg == "--help" => {
                die!(0, "{}", Self::help());
            }
            Some(arg) if arg == "-V" || arg == "--version" => {
                die!(0, "i3switch version {}", env!("CARGO_PKG_VERSION"));
            }
            _ => {}
        }

        // Let's take the happy path first

        let backend_arg = args.get(arg_index).map(|s| s.as_str());
        let backend = match backend_arg {
            #[cfg(feature = "i3")]
            Some("-i3")  => Some(UseBackend::I3),
            #[cfg(feature = "wmctl")]
            Some("-wm")  => Some(UseBackend::WmCtl),
            #[cfg(feature = "xcb")]
            Some("-xcb") => Some(UseBackend::Xcb),
            _            => None,
        };

        if backend.is_some() {
            arg_index += 1;
        }

        let valid_commands = ["left", "right", "up", "down", "next", "prev", "number"];
        if valid_commands.contains(&args.get(arg_index).map(|s| s.as_str()).unwrap_or("")) {
            command = args.get(arg_index).unwrap_or(&String::new()).clone();
            arg_index += 1;
        }

        if args.get(arg_index) == Some(&"wrap".to_string()) {
            wrap = true;
            arg_index += 1;
        }

        if command == "number" {
            number = args.get(arg_index).unwrap_or(&String::new()).parse::<usize>().ok();
            arg_index += 1;
        }

        // Now we can check if there's any issues with what we have got so far

        if args.get(arg_index).is_some() {
            die!(1, "Error: Unexpected argument '{}'. Use -h for help.", args[arg_index]);
        }

        if command.is_empty() {
            die!(1, "Error: No command provided. Use -h for help.");
        }

        if command == "number" {
            if number.is_none() {
                die!(1, "Error: No number provided for 'number' command. Use -h for help.");
            } else if wrap {
                die!(1, "Error: Wrap option is not applicable for 'number' command. Use -h for help.");
            }
        }

        // Any defaults we need to set

        let backend = backend.unwrap_or(*UseBackend::iter().next()
            .expect("At least one backend should be available"));

        Cli {
            backend,
            command,
            number,
            wrap,
        }
    }

    pub fn help() -> String {
        HELP.join("")
    }

    pub fn linear_direction(&self) -> Option<linear::Direction> {
        match self.command.as_str() {
            "next" => Some(linear::Direction::Next),
            "prev" => Some(linear::Direction::Prev),
            _ => None,
        }
    }

    pub fn planar_direction(&self) -> Option<planar::Direction> {
        match self.command.as_str() {
            "left"  => Some(planar::Direction::Left),
            "right" => Some(planar::Direction::Right),
            "up"    => Some(planar::Direction::Up),
            "down"  => Some(planar::Direction::Down),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseBackend {
    #[cfg(feature = "i3")]
    I3,
    #[cfg(feature = "wmctl")]
    WmCtl,
    #[cfg(feature = "xcb")]
    Xcb,
}

impl UseBackend {
    pub fn iter() -> Iter<'static, UseBackend> {
        static BACKENDS: &[UseBackend] = &[
            #[cfg(feature = "i3")]
            UseBackend::I3,
            #[cfg(feature = "wmctl")]
            UseBackend::WmCtl,
            #[cfg(feature = "xcb")]
            UseBackend::Xcb,
        ];
        return BACKENDS.iter();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        let args = "i3switch -i3 next wrap"
            .to_string().split_whitespace().map(String::from).collect();
        let cli = Cli::parse(args);
        assert_eq!(cli.backend, UseBackend::I3);
        assert_eq!(cli.command, "next");
        assert!(cli.wrap);
        assert!(cli.number.is_none());

        let args = "i3switch -wm prev"
            .to_string().split_whitespace().map(String::from).collect();
        let cli = Cli::parse(args);
        assert_eq!(cli.backend, UseBackend::WmCtl);
        assert_eq!(cli.command, "prev");
        assert!(!cli.wrap);
        assert!(cli.number.is_none());

        let args = "i3switch -xcb number 3"
            .to_string().split_whitespace().map(String::from).collect();
        let cli = Cli::parse(args);
        assert_eq!(cli.backend, UseBackend::Xcb);
        assert_eq!(cli.command, "number");
        assert!(!cli.wrap);
        assert_eq!(cli.number, Some(3));

        let args = "i3switch -i3 up wrap"
            .to_string().split_whitespace().map(String::from).collect();
        let cli = Cli::parse(args);
        assert_eq!(cli.backend, UseBackend::I3);
        assert_eq!(cli.command, "up");
        assert!(cli.wrap);
        assert!(cli.number.is_none());

        let args = "i3switch left"
            .to_string().split_whitespace().map(String::from).collect();
        let cli = Cli::parse(args);
        assert_eq!(cli.backend, UseBackend::I3);
        assert_eq!(cli.command, "left");
        assert!(!cli.wrap);
        assert!(cli.number.is_none());
    }
}
