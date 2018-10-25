//! # App
//!
//! `app` is the module providing the traits and types used by I/O applications.

/// Type used to interact with an I/O application.
pub mod command;

/// Types used to communicate with the I/O applications.
pub mod channels;

/// Types and traits used by the I/O applications for logging.
pub mod logger;

/// Trait implemented by I/O applications.
pub mod app;

/// Trait implemented by types used to configure I/O applications.
pub mod config;

/// Trait implemented by I/O applications managers.
pub mod manager;

pub use self::command::{Request, Response};
pub use self::channels::*;
pub use self::logger::*;
pub use self::app::App;
pub use self::config::Config;
pub use self::manager::Manager;