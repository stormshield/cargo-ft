#[cfg(not(any(
    target_os = "linux",
    target_os = "windows",
    target_os = "mac",
    target_arch = "wasm32"
)))]
compile_error!("This crate compiles only on Linux, Windows, macOS or WASM");
