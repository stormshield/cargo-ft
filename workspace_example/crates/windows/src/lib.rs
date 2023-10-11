#[cfg(not(target_os = "windows"))]
compile_error!("This crate compiles only on Windows");
