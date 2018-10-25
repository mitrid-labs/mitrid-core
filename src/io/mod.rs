//! # IO
//!
//! `io` is the module providing the traits and types used to implement I/O operations.

/// Type used to represent read/write permissions.
pub mod permission;

/// Type used to represent I/O sessions.
pub mod session;

/// Traits implemented by stores and storable types.
pub mod store;

/// Traits implemented by networking facilities and types that can use them.
pub mod network;

/// Types and traits used by the I/O applications for logging.
pub mod logger;

/// Trait implemented by types used to configure I/O applications.
pub mod config;

/// Types and traits used to manage the I/O applications.
pub mod app;

// /// Trait implemented by types that can manage and interact with the I/O apps from the command line.
// pub mod cli;

pub use self::permission::Permission;
pub use self::session::Session;
pub use self::store::{Store, Storable};
pub use self::network::*;
pub use self::logger::*;
pub use self::config::Config;
pub use self::app::*;