use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EvalParams {
    Const,
    IsEmpty,
    ToLowercase,
    ToUppercase,
}

impl Default for EvalParams {
    fn default() -> EvalParams {
        EvalParams::Const
    }
}

impl Sizable for EvalParams {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for EvalParams {}

impl Serializable for EvalParams {}

impl Datable for EvalParams {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum EvalReturn {
    Const(String),
    IsEmpty(bool),
    ToLowercase(String),
    ToUppercase(String),
}

impl Default for EvalReturn {
    fn default() -> EvalReturn {
        EvalReturn::Const(String::new())
    }
}

impl Sizable for EvalReturn {
    fn size(&self) -> u64 {
        match self {
            &EvalReturn::Const(ref s) => s.size(),
            &EvalReturn::IsEmpty(b) => b.size(),
            &EvalReturn::ToLowercase(ref s) => s.size(),
            &EvalReturn::ToUppercase(ref s) => s.size(),
        }
    }
}

impl Checkable for EvalReturn {}

impl Serializable for EvalReturn {}

impl Datable for EvalReturn {}