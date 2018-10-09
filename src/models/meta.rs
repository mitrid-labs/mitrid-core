//! # Meta
//!
//! `meta` is the module providing the type defining the distributed ledger types metadata.

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use utils::Version;
use utils::Timestamp;
use models::Stage;

/// Type used to convey the distributed ledger models metadata.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Meta {
    /// Code of the metadata owner.
    pub code: u64,
    /// Chain of the metadata owner.
    pub chain: String,
    /// Version of the chain.
    pub version: Version,
    /// Stage of the chain.
    pub stage: Stage,
    /// Timestamp of the metadata owner.
    pub timestamp: Timestamp,
    /// Size of the metadata owner.
    pub size: u64,
}

impl Meta {
    /// Creates a new `Meta`.
    pub fn new(code: u64,
               chain: String,
               version: Version,
               stage: Stage,
               size: u64)
        -> Result<Meta>
    {
        version.check()?;
        stage.check()?;

        let timestamp = Timestamp::now()?;

        let meta = Meta { code, chain, version, stage, timestamp, size };
        Ok(meta)
    }
}

impl Sizable for Meta {
    fn size(&self) -> u64 {
        self.code.size() +
            self.chain.size() +
            self.version.size() +
            self.stage.size() +
            self.timestamp.size() +
            self.size.size()
    }
}

impl Checkable for Meta {
    fn check(&self) -> Result<()> {
        self.version.check()?;
        self.stage.check()
    }
}

impl Datable for Meta {}

impl Serializable for Meta{}