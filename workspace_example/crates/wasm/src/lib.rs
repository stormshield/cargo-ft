#[cfg(not(target_arch = "wasm32"))]
compile_error!("This crate compiles only on WASM");
