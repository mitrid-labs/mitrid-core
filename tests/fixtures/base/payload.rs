use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Payload(String);

impl Payload {
    pub fn new(payload: &str) -> Payload {
        Payload::from_str(payload)
    }

    pub fn from_str(s: &str) -> Payload {
        Payload(s.into())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Sizable for Payload {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl Checkable for Payload {}

impl Serializable for Payload {}

impl Datable for Payload {}