#[cfg(not(target_os = "linux"))]
compile_error!("This crate compiles only on Linux");
