//! # IO
//!
//! `io` is the module providing the traits and types used to implement I/O operations.

/// Type used to represent read/write permissions.
pub mod permission;

/// Type used to represent I/O sessions.
pub mod session;

/// Traits implemented by stores and storable types.
pub mod store;

/// Type used to represent a node in the distributed ledger network.
pub mod node;

/// Traits implemented by networking facilities and types that can use them.
pub mod network;

pub use self::permission::Permission;
pub use self::session::Session;
pub use self::store::{Store, Storable};
pub use self::node::Node;
pub use self::network::{Network, Networkable};
