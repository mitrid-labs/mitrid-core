use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::utils::Version;
use mitrid_core::base::Stage;
use mitrid_core::app::Config as BasicConfig;

use fixtures::crypto::Digest;
use fixtures::io::network::Address;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Config {
    pub chain: String,
    pub version: Version,
    pub stage: Stage,
    pub pswd_hash: Digest,
    pub buffer_size: u64,
    pub max_threads: u64,
    pub addresses: Vec<Address>,
    pub seed: Vec<Address>,
}

impl Config {
    pub fn new(chain: &str,
               version: &Version,
               stage: &Stage,
               pswd_hash: &Digest,
               buffer_size: u64,
               max_threads: u64,
               addresses: &Vec<Address>,
               seed: &Vec<Address>)
        -> Result<Config>
    {
        version.check()?;
        stage.check()?;
        pswd_hash.check()?;
        addresses.check()?;
        seed.check()?;

        let config = Config {
            chain: chain.to_owned(),
            version: version.to_owned(),
            stage: stage.to_owned(),
            pswd_hash: pswd_hash.to_owned(),
            buffer_size: buffer_size,
            max_threads: max_threads,
            addresses: addresses.to_owned(),
            seed: seed.to_owned(),
        };

        Ok(config)
    }
}

impl BasicConfig<Digest, (), Address, (), (), (), ()> for Config {
    /// Returs the chain of the config.
    fn chain(&self) -> String {
        self.chain.clone()
    }
    
    /// Returs the version of the config chain.
    fn version(&self) -> Version {
        self.version.clone()
    }
    
    /// Returs the stage of the config chain.
    fn stage(&self) -> Stage {
        self.stage.clone()
    }
    
    /// Returs the hash of the chain node password.
    fn pswd_hash(&self) -> Digest {
        self.pswd_hash.clone()
    }
    
    /// Returs the buffer size of the applications channels.
    fn buffer_size(&self) -> u64 {
        self.buffer_size
    }
    
    /// Returs the maximum number of threads per application.
    fn max_threads(&self) -> u64 {
        self.max_threads
    }
    
    /// Returs the manager parameters.
    fn manager_params(&self) -> () {
        ()
    }
    
    /// Returs the store parameters.
    fn store_params(&self) -> () {
        ()
    }
    
    /// Returs the local network addresses.
    fn addresses(&self) -> Vec<Address> {
        self.addresses.clone()
    }
    
    /// Returs the seed network addresses.
    fn seed(&self) -> Vec<Address> {
        self.seed.clone()
    }
    
    /// Returs the network server parameters.
    fn server_params(&self) -> () {
        ()
    }
    
    /// Returs the network client parameters.
    fn client_params(&self) -> () {
        ()
    }

    /// Returs the custom parameters.
    fn custom_params(&self) -> () {
        ()
    }
}

impl Sizable for Config {
    fn size(&self) -> u64 {
        self.chain.size() +
            self.version.size() +
            self.stage.size() +
            self.pswd_hash.size() +
            self.buffer_size.size() +
            self.max_threads.size() +
            self.addresses.size() +
            self.seed.size()
    }
}

impl Checkable for Config {
    fn check(&self) -> Result<()> {
        self.chain.check()?;
        self.version.check()?;
        self.stage.check()?;
        self.pswd_hash.check()?;
        self.buffer_size.check()?;
        self.max_threads.check()?;
        self.addresses.check()?;
        self.seed.check()
    }
}

impl Serializable for Config {}

impl Datable for Config {}