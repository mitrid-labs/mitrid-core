//! # Command
//!
//! `command` is the module providing the types used to interact with I/O applications.

/// Type used to represent a command request.
pub mod request;

/// Type used to represent a command response.
pub mod response;

pub use self::request::Request;
pub use self::response::Response;