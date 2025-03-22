//! WebAssembly bindings for the Obadh Engine.
//!
//! This module provides WebAssembly bindings for using the engine in web browsers.

mod bindings;

pub use bindings::*;

/// Set up panic hook for better error handling in WebAssembly
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}