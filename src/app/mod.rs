//! # App
//!
//! `app` is the module providing the traits and types used to create and manage the framework applications.

/// Type used to interact with a Mitrid application.
pub mod command;

/// Types used to communicate with the applications.
pub mod channels;

/// Types and traits used by the applications for logging.
pub mod logger;

/// Trait implemented by applications.
pub mod app;

/// Trait implemented by types used to configure applications.
pub mod config;

/// Trait implemented by application managers.
pub mod manager;

pub use self::command::{Request, Response};
pub use self::channels::*;
pub use self::logger::*;
pub use self::app::App;
pub use self::config::Config;
pub use self::manager::Manager;