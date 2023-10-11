# cargo-ft

cargo-ft (cargo filter target) is a cargo extension for specifying supported targets for a crate.

## Description

When your workspace grows, you can have crates targeting different platforms. For example, you can have a crate
for desktop platforms (Linux, Windows and MacOS), a crate for Android and a crate for WASM. When you run
`cargo check/clippy` in your CI, you must manually include/exclude some crates when checking for Linux, for Android
and for WASM. You also need to update the inclusion/exclusion list of crates when adding or removing crates in
your workspace.

This cargo extension allows you to declare the supported targets directly in the `Cargo.toml` so they can be
skipped during build, or error early when asking to build the crate explicitly. Using the previously mentioned
workspace with 3 crates, executing `cargo ft check --target x86_64-unknown-linux-gnu` will only check the `desktop`
crate while executing `cargo ft check --target wasm32-unknown-unknown` will only check the `wasm` crate.

## Installation

Use [cargo](https://doc.rust-lang.org/cargo) to install cargo-ft.

```shell
cargo install --locked cargo-ft
```

## Usage

By default, if the metadata is missing or empty, the tool assumes the crate supports all targets.

If you want to control the supported targets, start by declaring the supported targets of your crates in
their `Cargo.toml` like this :

```toml
[package.metadata.ft]
targets = ["wasm32-unknown-unknown"]
```

Then, prefix your build, check, clippy or run command, or any command where you can specify a `target` with `ft`
to filter unsupported targets.

Some examples :

```shell
# Run cargo build on crates supporting the host target
cargo ft build # Instead of cargo build

# Run cargo clippy on crates supporting wasm32
cargo ft clippy --workspace --all-targets --all-features --target wasm32-unknown-unknown -- -D warnings # Instead of cargo clippy ...
```

## Related issues

This tool was created in response to a need we encountered with multi-targets workspace with lots of binaries. The
following issues shows us we are not alone, even if the needs are probably not exactly the same :

- [Allow specifying a set of supported target platforms in Cargo.toml](https://github.com/rust-lang/cargo/issues/6179)
- [Be able to specify which targets are valid in a workspace](https://github.com/EmbarkStudios/rust-ecosystem/issues/41)
- [Allow workspaces/crates to limit support to an explicit set of targets](https://github.com/rust-lang/cargo/issues/11313)

All those issues mention the need to slim down the `Cargo.lock` with only target relevant dependencies. It is not done
in this extension because either you only use `cargo-ft` for everything, or `cargo` and `cargo-ft` will fight every time
changing the `Cargo.lock`. This feature is probably only doable directly in `cargo`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

Please make sure to use [conventional commits](https://www.conventionalcommits.org)

## License

This project is licensed under the Apache-2.0 license.

See [LICENSE](LICENSE) and [NOTICE](NOTICE) for details.
