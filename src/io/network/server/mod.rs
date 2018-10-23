//! # Server
//!
//! `server` is the module providing the network server types and traits.

/// Trait implemented by the server handler.
pub mod handler;

/// Trait implemented by the server router.
pub mod router;

pub mod server;

pub use self::handler::Handler;
pub use self::router::Router;