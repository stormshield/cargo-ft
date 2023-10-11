use clap::CommandFactory;
use clap_cargo::{Manifest, Workspace};

use super::*;

#[test]
fn verify_cargo_cli() {
    CargoCli::command().debug_assert();
}

#[test]
fn verify_command() {
    Command::command().debug_assert();
}

#[test]
fn verify_ft_args() {
    let cmd = FtArgs::command();

    for arg in cmd.get_arguments() {
        assert!(!arg.is_positional());
    }

    cmd.debug_assert();
}

#[test]
fn build_no_arg() {
    check(["build"], Cli {
        cargo_cmd: "build",
        ft_args: FtArgs {
            manifest: Manifest::default(),
            frozen: false,
            locked: false,
            offline: false,
            workspace: Workspace::default(),
            target: None,
        },
        cargo_args: vec![],
    });
}

#[test]
fn build_known_args() {
    let mut workspace = Workspace::default();
    workspace.workspace = true;

    check(["build", "--workspace", "--target=wasm32-unknown-unknown"], Cli {
        cargo_cmd: "build",
        ft_args: FtArgs {
            manifest: Manifest::default(),
            frozen: false,
            locked: false,
            offline: false,
            workspace,
            target: Some("wasm32-unknown-unknown".to_owned()),
        },
        cargo_args: vec![],
    });
}

#[test]
fn build_unknown_args() {
    check(["build", "--all-targets", "--all-features", "--profile=release"], Cli {
        cargo_cmd: "build",
        ft_args: FtArgs {
            manifest: Manifest::default(),
            frozen: false,
            locked: false,
            offline: false,
            workspace: Workspace::default(),
            target: None,
        },
        cargo_args: vec![
            "--all-targets".into(),
            "--all-features".into(),
            "--profile=release".into(),
        ],
    });
}

#[test]
fn build_known_and_unknown_args() {
    let mut workspace = Workspace::default();
    workspace.workspace = true;

    check(
        [
            "build",
            "--workspace",
            "--target",
            "wasm32-unknown-unknown",
            "--all-targets",
            "--all-features",
            "--profile",
            "release",
        ],
        Cli {
            cargo_cmd: "build",
            ft_args: FtArgs {
                manifest: Manifest::default(),
                frozen: false,
                locked: false,
                offline: false,
                workspace,
                target: Some("wasm32-unknown-unknown".to_owned()),
            },
            cargo_args: vec![
                "--all-targets".into(),
                "--all-features".into(),
                "--profile".into(),
                "release".into(),
            ],
        },
    );
}

#[test]
fn build_unknown_and_known_args() {
    let mut workspace = Workspace::default();
    workspace.workspace = true;

    check(
        [
            "build",
            "--all-targets",
            "--all-features",
            "--profile=release",
            "--workspace",
            "--target=wasm32-unknown-unknown",
        ],
        Cli {
            cargo_cmd: "build",
            ft_args: FtArgs {
                manifest: Manifest::default(),
                frozen: false,
                locked: false,
                offline: false,
                workspace,
                target: Some("wasm32-unknown-unknown".to_owned()),
            },
            cargo_args: vec![
                "--all-targets".into(),
                "--all-features".into(),
                "--profile=release".into(),
            ],
        },
    );
}

#[test]
fn build_mixed_known_and_unknown_args() {
    let mut workspace = Workspace::default();
    workspace.workspace = true;

    check(
        [
            "build",
            "--profile=release",
            "--workspace",
            "--all-targets",
            "--target",
            "wasm32-unknown-unknown",
            "--all-features",
        ],
        Cli {
            cargo_cmd: "build",
            ft_args: FtArgs {
                manifest: Manifest::default(),
                frozen: false,
                locked: false,
                offline: false,
                workspace,
                target: Some("wasm32-unknown-unknown".to_owned()),
            },
            cargo_args: vec![
                "--profile=release".into(),
                "--all-targets".into(),
                "--all-features".into(),
            ],
        },
    );
}

#[test]
fn run_binary_args() {
    check(["run", "--", "arg1", "--arg2"], Cli {
        cargo_cmd: "run",
        ft_args: FtArgs {
            manifest: Manifest::default(),
            frozen: false,
            locked: false,
            offline: false,
            workspace: Workspace::default(),
            target: None,
        },
        cargo_args: vec!["--".into(), "arg1".into(), "--arg2".into()],
    });
}

#[test]
fn run_known_and_binary_args() {
    let mut workspace = Workspace::default();
    workspace.workspace = true;

    check(
        ["run", "--workspace", "--target", "wasm32-unknown-unknown", "--", "arg1", "--arg2"],
        Cli {
            cargo_cmd: "run",
            ft_args: FtArgs {
                manifest: Manifest::default(),
                frozen: false,
                locked: false,
                offline: false,
                workspace,
                target: Some("wasm32-unknown-unknown".to_owned()),
            },
            cargo_args: vec!["--".into(), "arg1".into(), "--arg2".into()],
        },
    );
}

#[test]
fn run_unknown_and_binary_args() {
    check(
        ["run", "--all-targets", "--all-features", "--profile=release", "--", "arg1", "--arg2"],
        Cli {
            cargo_cmd: "run",
            ft_args: FtArgs {
                manifest: Manifest::default(),
                frozen: false,
                locked: false,
                offline: false,
                workspace: Workspace::default(),
                target: None,
            },
            cargo_args: vec![
                "--all-targets".into(),
                "--all-features".into(),
                "--profile=release".into(),
                "--".into(),
                "arg1".into(),
                "--arg2".into(),
            ],
        },
    );
}

fn check<I, T>(args: I, expected_cli: Cli)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::parse_from(
        ["cargo-ft", "ft"].map(Into::into).into_iter().chain(args.into_iter().map(Into::into)),
    );

    assert_eq!(cli, expected_cli);
}
