//! # App
//!
//! `app` is the module providing the traits and types used to create and manage the framework applications.

/// Type used to interact with a Mitrid application.
pub mod command;

/// Types used to communicate with the applications.
pub mod channel;

/// Types and traits used by the applications for logging.
pub mod logger;

/// Trait implemented by applications.
pub mod app;

/// Trait implemented by types used to read the process environment.
pub mod env;

/// Trait implemented by types used to configure applications.
pub mod config;

/// Trait implemented by application managers.
pub mod manager;

/// Types and trait used to manage and interact with the framework applications from the command line.
pub mod cli;

pub use self::command::{Request, Response};
pub use self::channel::*;
pub use self::logger::*;
pub use self::app::App;
pub use self::env::Env;
pub use self::config::Config;
pub use self::manager::Manager;