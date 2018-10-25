//! # App
//!
//! `app` is the module providing the traits and types used to manage the I/O applications.

/// Type used to interact with an I/O application.
pub mod command;

/// Types used to communicate with the I/O applications.
pub mod channels;

/// Trait implemented by I/O applications.
pub mod app;

// /// Trait implemented by I/O applications managers.
// pub mod manager;