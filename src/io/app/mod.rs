//! # App
//!
//! `app` is the module providing the traits and types used to manage the I/O applications.

/// Type used to interact with an I/O application.
pub mod command;

/// Trait implemented by types retrieving the I/O apps configurations.
pub mod config;

/// Trait implemented by I/O applications.
pub mod app;

/// Trait implemented by types that can manage and interact with the I/O apps from the command line.
pub mod cli;