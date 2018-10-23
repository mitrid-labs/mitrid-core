//! # Client
//!
//! `client` is the module providing the network client types and traits.

/// Type used to represent the client behavior on error responses.
pub mod on_error;

/// Trait implemented by network clients.
pub mod client;

pub use self::on_error::OnError;
pub use self::client::Client;