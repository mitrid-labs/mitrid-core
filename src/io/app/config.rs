//! # Config
//!
//! `config` is the module providing the trait implemented by types used to configure the I/O applications.

use std::path::Path;

use base::Result;
use base::Serializable;
use base::Datable;

/// Trait implemented by types used to configure I/O applications.
pub trait Config
    where   Self: Datable + Serializable
{
    /// Reads a `Config` from a json file.
    fn read_from_json_file<P: AsRef<Path>>(path: P) -> Result<Self>;

    /// Writes the `Config` to a json file.
    fn write_json_file<P: AsRef<Path>>(path: P) -> Result<()>;

    /// Reads a `Config` from a binary file.
    fn read_from_binary_file<P: AsRef<Path>>(path: P) -> Result<Self>;
    
    /// Writes the `Config` to a binary file.
    fn write_binary_file<P: AsRef<Path>>(path: P) -> Result<()>;

    /// Reads a `Config` from a hex file.
    fn read_from_hex_file<P: AsRef<Path>>(path: P) -> Result<Self>;
    
    /// Writes the `Config` to a hex file.
    fn write_hex_file<P: AsRef<Path>>(path: P) -> Result<()>;
}