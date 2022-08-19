//! Memflex - Memory hacking library for Rust
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

/// Module with support for various memory patterns
pub mod pattern;

#[cfg(feature = "internal")]
mod global;
#[cfg(feature = "internal")]
pub use global::*;

#[cfg(feature = "internal")]
/// Module with helper functions for internal apis.
pub mod internal;

/// Useful types for interacting with C
pub mod types;

pub use memoffset;

mod memory;
pub use memory::*;