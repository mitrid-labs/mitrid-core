//! # Mitrid Core
//!
//! `mitrid_core` provides the core traits and types used by the Mitrid framework, a framework
//! for building blockchains and other distributed ledgers using block-based authenticated
//! data structures (authenticated trees, authenticated graphs, etc).

extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate serde_cbor as cbor;
extern crate hex;
extern crate regex;
#[macro_use]
extern crate bitflags;
extern crate rand;

/// Traits and types used across the library.
pub mod base;

/// Types and functionalities commonly used but not foundational.
pub mod utils;

/// Traits used to implement cryptographical operations.
pub mod crypto;

/// Types used to implement block-based authenticated data structures.
pub mod models;

/// Traits and types used to implement I/O operations.
pub mod io;