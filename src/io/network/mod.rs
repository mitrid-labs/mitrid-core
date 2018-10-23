//! # Network
//!
//! `network` is the module providing the networking types and traits.

/// Trait implemented by network transports.
pub mod transport;

/// Type used to represent a node in the distributed ledger network.
pub mod node;

/// Type used to represent a network message.
pub mod message;

/// Trait implemented by network clients.
pub mod client;

pub use self::node::Node;
pub use self::message::*;
pub use self::client::*;