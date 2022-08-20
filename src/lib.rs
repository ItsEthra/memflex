#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
#![cfg_attr(not(feature = "std"), no_std)]

mod macros;
pub use macros::*;

mod pattern;
pub use pattern::*;

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

/// Some handy external API for interacting with system
#[cfg(feature = "external")]
pub mod external;

mod error;
pub use error::*;
