use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use utils::Version;
use models::Stage;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub chain: String,
    pub version: Version,
    pub stage: Stage,
    pub size: u64,
}

impl Meta {
    pub fn new(name: String,
               chain: String,
               version: Version,
               stage: Stage,
               size: u64)
        -> Result<Meta>
    {
        version.check()?;
        stage.check()?;

        let meta = Meta { name, chain, version, stage, size };
        Ok(meta)
    }
}

impl Sizable for Meta {
    fn size(&self) -> u64 {
        self.name.size() +
            self.chain.size() +
            self.version.size() +
            self.stage.size() +
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