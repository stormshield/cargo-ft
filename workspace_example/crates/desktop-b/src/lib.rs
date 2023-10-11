#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "mac")))]
compile_error!("This crate compiles only on Linux, Windows or macOS");
