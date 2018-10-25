//! # App
//!
//! `app` is the module providing the traits and types used to manage the I/O applications.

/// Trait implemented by types used to configure I/O applications.
pub mod config;

/// Type used to interact with an I/O application.
pub mod command;

/// Types used to communicate with the I/O applications.
pub mod channels;

/// Types and traits used by the I/O applications for logging.
pub mod logger;

/// Trait implemented by I/O applications.
pub mod app;

// /// Trait implemented by I/O applications managers.
// pub mod manager;

// /// Trait implemented by types that can manage and interact with the I/O apps from the command line.
// pub mod cli;