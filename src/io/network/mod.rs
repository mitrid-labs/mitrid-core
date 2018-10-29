//! # Network
//!
//! `network` is the module providing the networking types and traits.

/// Type used to represent a node in the distributed ledger network.
pub mod node;

/// Trait implemented by network transports.
pub mod transport;

/// Type used to represent a network message.
pub mod message;

/// Trait implemented by network clients.
pub mod client;

/// Trait implemented by network servers.
pub mod server;

pub use self::node::Node;
pub use self::transport::*;
pub use self::message::*;
pub use self::client::*;
pub use self::server::*;