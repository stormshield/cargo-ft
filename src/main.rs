use std::borrow::Borrow;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::{env, fmt, path};

use anstream::eprintln;
use error_stack::{ensure, report, Context, Result, ResultExt};

use crate::cli::Cli;
use crate::color::{note, warn};
use crate::package::FtPackage;

mod cli;
mod color;
mod filter;
mod package;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RuntimeError {
    cargo_cmd: &'static str,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not {}", self.cargo_cmd)
    }
}

impl Context for RuntimeError {}

fn main() -> Result<ExitCode, RuntimeError> {
    setup();

    let Cli { cargo_cmd, ft_args, cargo_args } = Cli::parse();
    let context = RuntimeError { cargo_cmd };

    let config = cargo_config2::Config::load()
        .change_context(context)
        .attach_printable("could not load cargo configuration")?;

    let mut targets = config
        .build_target_for_cli(&ft_args.target)
        .change_context(context)
        .attach_printable("could not select target triple")?;

    let target = targets.pop();
    let build_target = target.as_deref().unwrap_or(env!("TARGET_PLATFORM"));
    ensure!(
        targets.is_empty(),
        report!(context).attach_printable("multi-target build is not supported")
    );

    let manifest_options = [
        ft_args.frozen.then_some("--frozen"),
        ft_args.locked.then_some("--locked"),
        ft_args.offline.then_some("--offline"),
    ]
    .into_iter()
    .flatten();

    let cargo_metadata_options = ["--no-deps"]
        .into_iter()
        .chain(manifest_options.clone())
        .map(|s| s.into())
        .collect::<Vec<_>>();

    eprintln!("{:>12} metadata", note("Collecting"));
    let metadata = ft_args
        .manifest
        .metadata()
        .other_options(cargo_metadata_options)
        .verbose(true)
        .exec()
        .change_context(context)
        .attach_printable_lazy(|| {
            format!(
                "could not run initial cargo metadata on {}",
                manifest_path_display(ft_args.manifest.manifest_path.as_deref())
            )
        })?;

    let (selected, _excluded) = ft_args.workspace.partition_packages(&metadata);

    if selected.is_empty() {
        eprintln!("{:>12} nothing", warn("Finished"));
        return Ok(ExitCode::SUCCESS);
    }

    let selected = FtPackage::parse_metadata(&metadata, &selected).change_context(context)?;

    let (selected_supported, selected_unsupported) =
        filter::partition_packages(&selected, build_target);

    ensure!(
        !selected_supported.is_empty(),
        report!(context).attach_printable(format!(
            "all selected packages {:?} are unsupported on {build_target}",
            package_names(&selected)
        ))
    );

    let selected_is_explicit = !ft_args.workspace.package.is_empty()
        || !ft_args.workspace.exclude.is_empty()
        || selected.len() == 1;

    ensure!(
        !selected_is_explicit || selected_unsupported.is_empty(),
        report!(context).attach_printable(format!(
            "selected packages {:?} are unsupported on {build_target}",
            package_names(&selected_unsupported)
        ))
    );

    for package in selected_unsupported {
        let name = &package.package.name;
        eprintln!("{:>12} unsupported {name} on {build_target}", note("Skipping"));
    }

    let mut command = Command::new(config.cargo());
    command
        .arg(cargo_cmd)
        .args(ft_args.manifest.manifest_path.map(|p| {
            let mut arg = OsString::from("--manifest-path=");
            arg.push(p);
            arg
        }))
        .args(manifest_options)
        .args(selected_supported.into_iter().map(|p| format!("--package={}", p.package.name)))
        .args(target.map(|t| format!("--target={t}")))
        .args(cargo_args);

    let status = command
        .status()
        .change_context(context)
        .attach_printable_lazy(|| format!("could not run {command:?}"))?;

    Ok(status.code().map_or(ExitCode::FAILURE, |code| ExitCode::from(code as u8)))
}

fn setup() {
    // disable file location printing when building without debug assertions,
    // i.e. release
    #[cfg(not(debug_assertions))]
    error_stack::Report::install_debug_hook::<std::panic::Location<'_>>(|_, _| {});
}

fn manifest_path_display(manifest_path: Option<&Path>) -> path::Display<'_> {
    manifest_path.unwrap_or(Path::new("./Cargo.toml")).display()
}

fn package_names<'p>(packages: &[impl Borrow<FtPackage<'p>>]) -> Vec<&'p str> {
    packages.iter().map(|p| p.borrow().package.name.as_str()).collect::<Vec<_>>()
}
