use std::cmp::Ordering;
use std::fmt;

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use utils::regex;

pub const VERSION: &str = "0.0.1";

pub const NUMERIC_VERSION: &str = "^[0-9]*$";
pub const PRERELEASE_VERSION: &str = "^[A-Za-z-]*$";
pub const BUILDMETA_VERSION: &str = "^[0-9A-Za-z-]*$";
pub const SEMVER_VERSION: &str = "^(?P<major>[0-9]*).(?P<minor>[0-9]*).(?P<patch>[0-9]*)(-(?P<prerelease>[A-Za-z-]*))?(\\+(?P<buildmeta>[0-9A-Za-z-]*))?$";

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub buildmeta: Option<String>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32, pre: Option<String>, build: Option<String>)
        -> Result<Version> {

        let prerelease = pre.clone();
        let buildmeta = build.clone();

        if let Some(ref pre) = prerelease {
            Self::check_prerelease(pre)?;
        }

        if let Some(ref build) = buildmeta {
            Self::check_buildmeta(build)?;
        }

        let ver = Version {major, minor, patch, prerelease, buildmeta};
        Ok(ver)
    }

    pub fn check_numeric(num: &str) -> Result<()> {
        if regex::is_match(num, NUMERIC_VERSION).unwrap() {
            Ok(())
        } else {
            Err(String::from("invalid numeric version"))
        }
    }

    pub fn check_prerelease(pre: &str) -> Result<()> {
        if regex::is_match(pre, PRERELEASE_VERSION).unwrap() {
            Ok(())
        } else {
            Err(String::from("invalid prerelease version"))
        }
    }

    pub fn check_buildmeta(build: &str) -> Result<()> {
        if regex::is_match(build, BUILDMETA_VERSION).unwrap() {
            Ok(())
        } else {
            Err(String::from("invalid buildmeta version"))
        }
    }

    pub fn check_semver(sv: &str) -> Result<()> {
        if regex::is_match(sv, SEMVER_VERSION).unwrap() {
            Ok(())
        } else {
            Err(String::from("invalid semver version"))
        }
    }

    pub fn parse(s: &str) -> Result<Version> {
        let matches = regex::captures(SEMVER_VERSION, s)?;

        let _major = matches.get("minor").unwrap();
        let major = u32::from_str_radix(_major, 10)
                        .map_err(|e| format!("{}", e))?;

        let _minor = matches.get("minor").unwrap();
        let minor = u32::from_str_radix(_minor, 10)
                        .map_err(|e| format!("{}", e))?;

        let _patch = matches.get("patch").unwrap();
        let patch = u32::from_str_radix(_patch, 10)
                        .map_err(|e| format!("{}", e))?;

        let prerelease = matches.get("prerelease").map(|p| p.to_owned());
        let buildmeta = matches.get("buildmeta").map(|b| b.to_owned());

        let ver = Version { major, minor, patch, prerelease, buildmeta };

        Ok(ver)
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        res.push_str(&format!("{}", self.major));
        res.push_str(&format!(".{}", self.minor));
        res.push_str(&format!(".{}", self.patch));

        if self.prerelease.is_some() {
            res.push('-');
            res.push_str(&self.prerelease.clone().unwrap());
        }

        if self.buildmeta.is_some() {
            res.push('+');
            res.push_str(&self.buildmeta.clone().unwrap());
        }

        res
    }

    fn compare_numeric(n: u32, other: &u32) -> Ordering {
        n.cmp(other)
    }

    fn compare_prerelease(a: &Option<String>, b: &Option<String>) -> Ordering {
        if a.is_none() {
            if b.is_none() {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        }

        if b.is_none() {
            return Ordering::Less;
        }

        let _a = a.clone().unwrap();
        let _b = b.clone().unwrap();

        _a.cmp(&_b)
    }

    fn compare(&self, other: &Version) -> Ordering {
        let mut res = Self::compare_numeric(self.major, &other.major);
        if res != Ordering::Equal {
            return res;
        }

        res = Self::compare_numeric(self.minor, &other.minor);
        if res != Ordering::Equal {
            return res;
        }

        res = Self::compare_numeric(self.patch, &other.patch);
        if res != Ordering::Equal {
            return res;
        }

        Self::compare_prerelease(&self.prerelease, &other.prerelease)
    }

    pub fn is_compatible(&self, other: &Version) -> Result<bool> {
        self.check()?;
        other.check()?;

        let compatible = self.major == other.major;
        Ok(compatible)
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::parse(VERSION).unwrap()
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major &&
            self.minor == other.minor &&
            self.patch == other.patch
    }
}

impl Eq for Version {}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        self.compare(other)
    }
}

impl Sizable for Version {
    fn size(&self) -> u64 {
        self.major.size() +
            self.minor.size() +
            self.patch.size() +
            self.prerelease.size() +
            self.buildmeta.size()
    }
}

impl Checkable for Version {
    fn check(&self) -> Result<()> {
        if self.prerelease.is_some() {
            Self::check_prerelease(&self.clone().prerelease.unwrap())?;
        }

        if self.buildmeta.is_some() {
            Self::check_buildmeta(&self.clone().buildmeta.unwrap())?;
        }

        Ok(())
    }
}

impl Datable for Version {}

impl Serializable for Version {}
