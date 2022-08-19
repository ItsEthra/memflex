//! Memflex - Memory hacking library for Rust
#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

mod pattern;
pub use pattern::*;

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

#[cfg(feature = "nightly")]
mod ptr;
#[cfg(feature = "nightly")]
pub use ptr::*;
