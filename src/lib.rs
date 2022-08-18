//! Memflex - Memory hacking library for Rust
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

mod global;
pub use global::*;

#[cfg(feature = "internal")]
/// Module with helper functions for internal memory access.
pub mod internal;