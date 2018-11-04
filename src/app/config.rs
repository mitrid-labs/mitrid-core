//! # Config
//!
//! `config` is the module providing the type used to configure the Mitrid applications.

use std::fs::{File, OpenOptions};

use std::path::Path;
use std::io::{Read, Write};

use base::Result;
use base::{ConstantSize, VariableSize};
use base::Serializable;
use base::Datable;
use utils::Version;
use base::Stage;

pub trait Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable,
            Self: Datable + Serializable
{
    /// Returs the chain of the config.
    fn chain(&self) -> String;
    
    /// Returs the version of the config chain.
    fn version(&self) -> Version;
    
    /// Returs the stage of the config chain.
    fn stage(&self) -> Stage;
    
    /// Returs the hash of the chain node password.
    fn pswd_hash(&self) -> D;
    
    /// Returs the buffer size of the applications channels.
    fn buffer_size(&self) -> u64;
    
    /// Returs the maximum number of threads per application.
    fn max_threads(&self) -> u64;
    
    /// Returs the manager parameters.
    fn manager_params(&self) -> MnP;
    
    /// Returs the store parameters.
    fn store_params(&self) -> StP;
    
    /// Returs the local network addresses.
    fn addresses(&self) -> Vec<A>;
    
    /// Returs the seed network addresses.
    fn seed(&self) -> Vec<A>;
    
    /// Returs the network server parameters.
    fn server_params(&self) -> SvP;
    
    /// Returs the network client parameters.
    fn client_params(&self) -> ClP;

    /// Returs the custom parameters.
    fn custom_params(&self) -> CP;

    /// Reads a `Config` from a json file.
    fn read_json_file<P: 'static + Send + AsRef<Path>>(path: &P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut json = String::new();
                
                file.read_to_string(&mut json)
                    .map_err(|e| format!("{:?}", e))?;

                let config = Self::from_json(&json)?;
                config.check()?;

                Ok(config)
            })
    }

    /// Writes the `Config` to a json file.
    fn write_json_file<P: 'static +  Send + AsRef<Path>>(&self, path: &P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let json = self.to_json()?;
                file.write_all(json.as_bytes())
                    .map_err(|e| format!("{:?}", e))
            })
    }

    /// Reads a `Config` from a binary file.
    fn read_binary_file<P: 'static + Send + AsRef<Path>>(path: &P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut buf = Vec::new();
                
                file.read_to_end(&mut buf)
                    .map_err(|e| format!("{:?}", e))?;

                let config = Self::from_bytes(&buf)?;
                config.check()?;

                Ok(config)
            })
    }
    
    /// Writes the `Config` to a binary file.
    fn write_binary_file<P: 'static + Send + AsRef<Path>>(&self, path: &P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let buf = self.to_bytes()?;
                file.write_all(&buf)
                    .map_err(|e| format!("{:?}", e))
            })
    }

    /// Reads a `Config` from a hex file.
    fn read_hex_file<P: 'static + Send + AsRef<Path>>(path: &P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut hex = String::new();
                
                file.read_to_string(&mut hex)
                    .map_err(|e| format!("{:?}", e))?;

                let config = Self::from_hex(&hex)?;
                config.check()?;

                Ok(config)
            })
    }
    
    /// Writes the `Config` to a hex file.
    fn write_hex_file<P: 'static +  Send + AsRef<Path>>(&self, path: &P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let hex = self.to_hex()?;
                file.write_all(hex.as_bytes())
                    .map_err(|e| format!("{:?}", e))

            })
    }
}