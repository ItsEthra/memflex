//! Memflex - Memory hacking library for Rust
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

/// Module with support for various memory patterns
pub mod pattern;

mod global;
pub use global::*;

#[cfg(feature = "internal")]
/// Module with helper functions for internal memory access.
pub mod internal;