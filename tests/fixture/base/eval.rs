use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PayloadEvalParams {
    Const,
    IsEmpty,
}

impl Default for PayloadEvalParams {
    fn default() -> PayloadEvalParams {
        PayloadEvalParams::Const
    }
}

impl Sizable for PayloadEvalParams {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for PayloadEvalParams {}

impl Serializable for PayloadEvalParams {}

impl Datable for PayloadEvalParams {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum PayloadEvalResult {
    Const(String),
    IsEmpty(bool),
}

impl Default for PayloadEvalResult {
    fn default() -> PayloadEvalResult {
        PayloadEvalResult::Const(String::new())
    }
}

impl Sizable for PayloadEvalResult {
    fn size(&self) -> u64 {
        match self {
            &PayloadEvalResult::Const(ref s) => s.size(),
            &PayloadEvalResult::IsEmpty(b) => b.size(),
        }
    }
}

impl Checkable for PayloadEvalResult {}

impl Serializable for PayloadEvalResult {}

impl Datable for PayloadEvalResult {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PayloadEvalMutParams {
    ToLowercase,
    ToUppercase,
}

impl Default for PayloadEvalMutParams {
    fn default() -> PayloadEvalMutParams {
        PayloadEvalMutParams::ToLowercase
    }
}

impl Sizable for PayloadEvalMutParams {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for PayloadEvalMutParams {}

impl Serializable for PayloadEvalMutParams {}

impl Datable for PayloadEvalMutParams {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum PayloadEvalMutResult {
    ToLowercase(String),
    ToUppercase(String),
}

impl Default for PayloadEvalMutResult {
    fn default() -> PayloadEvalMutResult {
        PayloadEvalMutResult::ToLowercase(String::new())
    }
}

impl Sizable for PayloadEvalMutResult {
    fn size(&self) -> u64 {
        match self {
            &PayloadEvalMutResult::ToLowercase(ref s) => s.size(),
            &PayloadEvalMutResult::ToUppercase(ref s) => s.size(),
        }
    }
}

impl Checkable for PayloadEvalMutResult {}

impl Serializable for PayloadEvalMutResult {}

impl Datable for PayloadEvalMutResult {}