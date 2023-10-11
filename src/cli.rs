use std::collections::HashMap;
use std::ffi::OsString;

use clap::{Args, CommandFactory, Parser, Subcommand};

#[cfg(test)]
mod tests;

// See the following clap issues to know why this module is a bit of a mess
// https://github.com/clap-rs/clap/issues/1404
// https://github.com/clap-rs/clap/issues/5055
// tl;dr clap doesn't handle unknown arguments and always escape `--` when it's
// the first argument

/// Parsed command line interface arguments
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cli {
    /// cargo subcommand wrapped by the extension like build, clippy, test, ...
    pub cargo_cmd: &'static str,
    /// cargo arguments understood and processed by the extension
    pub ft_args: FtArgs,
    /// cargo arguments not processed by the extension and passed to cargo as is
    pub cargo_args: Vec<OsString>,
}

impl Cli {
    pub fn parse() -> Self {
        Self::from_cargo_cli(CargoCli::parse())
    }

    #[cfg(test)]
    fn parse_from<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::from_cargo_cli(CargoCli::parse_from(iter))
    }

    fn from_cargo_cli(CargoCli::Ft(FtCli::External(args)): CargoCli) -> Self {
        // args[0] is the subcommand, so the first argument is args[1]
        let start_with_double_dash = args.get(1).map_or(false, |arg| arg == "--");

        let command = Command::parse_from(args);

        let (cargo_cmd, mut args) = command.cmd_and_args();

        if start_with_double_dash {
            args.cargo_args.insert(0, "--".into());
        }

        let (ft_args, cargo_args) = FtArgs::split_ft_and_cargo_args(args.cargo_args);

        Self { cargo_cmd, ft_args, cargo_args }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(no_binary_name = true)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
#[group(skip)]
pub struct FtArgs {
    #[command(flatten)]
    pub manifest: clap_cargo::Manifest,
    /// Require Cargo.lock and cache are up to date
    #[arg(long)]
    pub frozen: bool,
    /// Require Cargo.lock is up to date
    #[arg(long)]
    pub locked: bool,
    /// Run without accessing the network
    #[arg(long)]
    pub offline: bool,

    #[command(flatten)]
    pub workspace: clap_cargo::Workspace,

    /// Build for the given architecture. The default is the host architecture.
    #[arg(long)]
    pub target: Option<String>,
}

impl FtArgs {
    fn split_ft_and_cargo_args(args: Vec<OsString>) -> (Self, Vec<OsString>) {
        let known_args_taking_value = Self::command()
            .get_arguments()
            .flat_map(|arg| {
                assert!(!arg.is_positional());

                let shorts = arg
                    .get_short_and_visible_aliases()
                    .into_iter()
                    .flat_map(|shorts| shorts.into_iter().map(|short| format!("-{short}")));

                let longs = arg
                    .get_long_and_visible_aliases()
                    .into_iter()
                    .flat_map(|longs| longs.into_iter().map(|long| format!("--{long}")));

                shorts.chain(longs).map(|arg_str| (arg_str, arg.get_action().takes_values()))
            })
            .collect::<HashMap<_, _>>();

        let mut double_dash_seen = false;
        let mut next_is_value = false;

        let (ft_args, cargo_args) = args.into_iter().partition::<Vec<_>, _>(|arg| {
            if double_dash_seen || arg == "--" {
                double_dash_seen = true;
                return false;
            }

            let is_value = next_is_value;

            match arg.to_str().map(|arg| (arg, known_args_taking_value.get(arg))) {
                Some((_, Some(takes_value))) => {
                    next_is_value = *takes_value;
                    true
                }
                Some((arg, None)) if is_flag_and_value(arg, &known_args_taking_value) => {
                    next_is_value = false;
                    true
                }
                _ => {
                    next_is_value = false;
                    is_value
                }
            }
        });

        (Self::parse_from(ft_args), cargo_args)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
enum CargoCli {
    #[command(subcommand)]
    Ft(FtCli),
}

#[derive(Debug, Clone, Eq, PartialEq, Subcommand)]
#[command(author, version, about, long_about = None)]
enum FtCli {
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(Debug, Clone, Eq, PartialEq, Parser)]
#[command(no_binary_name = true)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
#[group(skip)]
enum Command {
    /// Compile and execute benchmarks.
    Bench(CommandArgs),
    /// Compile local packages and all of their dependencies.
    Build(CommandArgs),
    /// Check a local package and all of its dependencies for errors.
    Check(CommandArgs),
    /// Checks a package to catch common mistakes and improve your Rust code.
    Clippy(CommandArgs),
    /// Build the documentation for the local package and all dependencies.
    Doc(CommandArgs),
    /// This Cargo subcommand will automatically take rustc’s suggestions from
    /// diagnostics like warnings and apply them to your source code.
    Fix(CommandArgs),
    /// Run a binary or example of the local package.
    Run(CommandArgs),
    /// The specified target for the current package (or package specified by -p
    /// if provided) will be compiled along with all of its dependencies.
    Rustc(CommandArgs),
    /// The specified target for the current package (or package specified by -p
    /// if provided) will be documented with the specified args being passed to
    /// the final rustdoc invocation.
    Rustdoc(CommandArgs),
    /// Compile and execute unit, integration, and documentation tests.
    Test(CommandArgs),
}

impl Command {
    fn cmd_and_args(self) -> (&'static str, CommandArgs) {
        match self {
            Self::Bench(args) => ("bench", args),
            Self::Build(args) => ("build", args),
            Self::Check(args) => ("check", args),
            Self::Clippy(args) => ("clippy", args),
            Self::Doc(args) => ("doc", args),
            Self::Fix(args) => ("fix", args),
            Self::Run(args) => ("run", args),
            Self::Rustc(args) => ("rustc", args),
            Self::Rustdoc(args) => ("rustdoc", args),
            Self::Test(args) => ("test", args),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Args)]
struct CommandArgs {
    /// Same as cargo subcommand, see `cargo help subcommand`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    cargo_args: Vec<OsString>,
}

fn is_flag_and_value(arg: &str, known_args: &HashMap<String, bool>) -> bool {
    arg.split_once('=').map_or(false, |(prefix, _)| known_args.contains_key(prefix))
}
