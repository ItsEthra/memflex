#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
