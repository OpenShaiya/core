#![feature(seek_convenience)]
pub mod network;
pub mod client;

/// Error returned by most functions.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for convenience.
pub type Result<T> = std::result::Result<T, Error>;
