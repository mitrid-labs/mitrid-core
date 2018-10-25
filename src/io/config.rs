//! # Config
//!
//! `config` is the module providing the type used to configure the I/O applications.

use futures::Future as BasicFuture;
use tokio_fs::{File, OpenOptions};

use std::path::Path;
use std::io::{Read, Write};

use base::Result;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use base::Evaluable;
use utils::Version;
use utils::Stage;

/// Type used to configure I/O applications.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable
{
    /// Chain of the config.
    pub chain: String,
    /// Version of the config chain.
    pub version: Version,
    /// Stage of the config chain.
    pub stage: Stage,
    /// Hash of the chain node password.
    pub pswd_hash: D,
    /// Buffer of the applications channels.
    pub buffer: u64,
    /// Maximum number of threads per application.
    pub max_threads: u64,
    /// Manager parameters.
    pub manager_params: MnP,
    /// Store parameters.
    pub store_params: StP,
    /// Local network addresses.
    pub addresses: Vec<A>,
    /// Seed network addresses.
    pub seed: Vec<A>,
    /// Network server parameters.
    pub server_params: SvP,
    /// Network client parameters.
    pub client_params: ClP,
    /// Custom parameters.
    pub custom_params: CP,
}

impl<D, MnP, A, StP, SvP, ClP, CP> Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize + Serializable,
            MnP: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            StP: Datable + Serializable,
            SvP: Datable + Serializable,
            ClP: Datable + Serializable,
            CP: Datable + Serializable
{
    /// Creates a new `Config`.
    pub fn new(chain: &str,
               version: &Version,
               stage: &Stage,
               pswd_hash: &D,
               buffer: u64,
               max_threads: u64,
               manager_params: &MnP,
               store_params: &StP,
               addresses: &Vec<A>,
               seed: &Vec<A>,
               server_params: &SvP,
               client_params: &ClP,
               custom_params: &CP)
        -> Result<Self>
    {
        version.check()?;
        stage.check()?;
        pswd_hash.check()?;
        manager_params.check()?;
        store_params.check()?;
        addresses.check()?;
        seed.check()?;
        server_params.check()?;
        client_params.check()?;
        custom_params.check()?;

        let config = Config {
            chain: chain.to_owned(),
            version: version.to_owned(),
            stage: stage.to_owned(),
            pswd_hash: pswd_hash.to_owned(),
            buffer: buffer,
            max_threads: max_threads,
            manager_params: manager_params.to_owned(),
            store_params: store_params.to_owned(),
            addresses: addresses.to_owned(),
            seed: seed.to_owned(),
            server_params: server_params.to_owned(),
            client_params: client_params.to_owned(),
            custom_params: custom_params.to_owned(),
        };

        Ok(config)
    }

    /// Reads a `Config` from a json file.
    pub fn read_from_json_file<P: 'static + Send + AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut json = String::new();
                
                match file.read_to_string(&mut json) {
                    Err(e) => Err(format!("{}", e)),
                    Ok(_) => {
                        Config::from_json(&json)
                    },
                }
            })
            .wait()
    }

    /// Writes the `Config` to a json file.
    pub fn write_json_file<P: 'static +  Send + AsRef<Path>>(&self, path: P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                match self.to_json() {
                    Err(e) => Err(e),
                    Ok(json) => {
                        match file.write_all(json.as_bytes()) {
                            Err(e) => Err(format!("{}", e)),
                            Ok(_) => Ok(()),
                        }
                    }
                }
            })
            .wait()
    }

    /// Reads a `Config` from a binary file.
    pub fn read_from_binary_file<P: 'static + Send + AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut buf = Vec::new();
                
                match file.read_to_end(&mut buf) {
                    Err(e) => Err(format!("{}", e)),
                    Ok(_) => {
                        Config::from_bytes(&buf)
                    },
                }
            })
            .wait()
    }
    
    /// Writes the `Config` to a binary file.
    pub fn write_binary_file<P: 'static + Send + AsRef<Path>>(&self, path: P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                match self.to_bytes() {
                    Err(e) => Err(e),
                    Ok(buf) => {
                        match file.write_all(&buf) {
                            Err(e) => Err(format!("{}", e)),
                            Ok(_) => Ok(()),
                        }
                    }
                }
            })
            .wait()
    }

    /// Reads a `Config` from a hex file.
    pub fn read_from_hex_file<P: 'static + Send + AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                let mut hex = String::new();
                
                match file.read_to_string(&mut hex) {
                    Err(e) => Err(format!("{}", e)),
                    Ok(_) => {
                        Config::from_hex(&hex)
                    },
                }
            })
            .wait()
    }
    
    /// Writes the `Config` to a hex file.
    pub fn write_hex_file<P: 'static +  Send + AsRef<Path>>(&self, path: P) -> Result<()> {
        self.check()?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or_else(|e| {
                Err(format!("{}", e))
            })
            .and_then(|mut file| {
                match self.to_hex() {
                    Err(e) => Err(e),
                    Ok(hex) => {
                        match file.write_all(hex.as_bytes()) {
                            Err(e) => Err(format!("{}", e)),
                            Ok(_) => Ok(()),
                        }
                    }
                }
            })
            .wait()
    }
}

impl<D, MnP, A, StP, SvP, ClP, CP> Sizable for Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable
{
    fn size(&self) -> u64 {
        self.chain.size() +
            self.version.size() +
            self.stage.size() +
            self.pswd_hash.size() +
            self.buffer.size() +
            self.max_threads.size() +
            self.manager_params.size() +
            self.store_params.size() +
            self.addresses.size() +
            self.seed.size() +
            self.server_params.size() +
            self.client_params.size() +
            self.custom_params.size()
    }
}

impl<D, MnP, A, StP, SvP, ClP, CP> Checkable for Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable
{
    fn check(&self) -> Result<()> {
        self.chain.check()?;
        self.version.check()?;
        self.stage.check()?;

        self.pswd_hash.check()?;

        self.buffer.check()?;
        self.max_threads.check()?;
        self.manager_params.check()?;

        self.store_params.check()?;

        self.addresses.check()?;
        self.seed.check()?;
        self.server_params.check()?;
        
        self.client_params.check()?;
        
        self.custom_params.check()?;

        Ok(())
    }
}

impl<D, MnP, A, StP, SvP, ClP, CP> Serializable for Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize + Serializable,
            MnP: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            StP: Datable + Serializable,
            SvP: Datable + Serializable,
            ClP: Datable + Serializable,
            CP: Datable + Serializable
{}

impl<D, MnP, A, StP, SvP, ClP, CP> Datable for Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable
{}

impl<D, MnP, A, StP, SvP, ClP, CP> Evaluable for Config<D, MnP, A, StP, SvP, ClP, CP>
    where   D: Datable + ConstantSize,
            MnP: Datable,
            A: Datable + VariableSize,
            StP: Datable,
            SvP: Datable,
            ClP: Datable,
            CP: Datable
{}