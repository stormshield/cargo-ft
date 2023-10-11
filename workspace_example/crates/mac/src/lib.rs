#[cfg(not(target_os = "macos"))]
compile_error!("This crate compiles only on macOS");
