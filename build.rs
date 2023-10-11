// Cargo sets the target env var for build scripts, but not for crates
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
// So just reexport it with a different name

use std::env;

fn main() {
    let original_key = "TARGET";
    let new_key = "TARGET_PLATFORM";

    let value = env::var(original_key).expect("env var `TARGET` should be set for build scripts");

    println!("cargo:rustc-env={new_key}={value}");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={original_key}");
}
