//! # Version
//!
//! `version` is the module providing the Semver version type and constants.

use std::cmp::Ordering;
use std::fmt;

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Datable;
use base::Serializable;
use util::regex;

/// Current Semver version of the library.
pub const VERSION: &str = "0.5.0";

/// Regex pattern of a numeric version.
pub const NUMERIC_VERSION: &str = "^[0-9]*$";

/// Regex pattern of a prerelease version.
pub const PRERELEASE_VERSION: &str = "^[A-Za-z-]*$";

/// Regex pattern of a buildmeta version.
pub const BUILDMETA_VERSION: &str = "^[0-9A-Za-z-]*$";

/// Regex pattern of a Semver version.
pub const SEMVER_VERSION: &str = "^(?P<major>[0-9]*).(?P<minor>[0-9]*).(?P<patch>[0-9]*)(-(?P<prerelease>[A-Za-z-]+))?(\\+(?P<buildmeta>[0-9A-Za-z-]+))?$";

/// Type used to represent a Semver version.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Version {
    /// Semver version major. Used for API breaking changes.
    pub major: u32,
    /// Semver version minor. Used for backward-compatible API changes.
    pub minor: u32,
    /// Semver version patch. Used for changes not affecting the API.
    pub patch: u32,
    /// Semver version prerelease. Used in prereleases. Optional.
    pub prerelease: String,
    /// Semver version buildmeta. Build information. Optional.
    pub buildmeta: String,
}

impl Version {
    /// Creates a new Semver version.
    pub fn new(major: u32, minor: u32, patch: u32, pre: &str, build: &str)
        -> Result<Version> {

        Self::check_prerelease(pre)?;

        Self::check_buildmeta(build)?;

        let prerelease = String::from(pre);
        let buildmeta = String::from(build);

        let ver = Version {major, minor, patch, prerelease, buildmeta};
        Ok(ver)
    }

    /// Check a Semver numeric version (major, minor or patch).
    pub fn check_numeric(num: &str) -> Result<()> {
        if regex::is_match(NUMERIC_VERSION, num).unwrap() {
            Ok(())
        } else {
            Err("invalid numeric version".into())
        }
    }

    /// Check a Semver prerelease version.
    pub fn check_prerelease(pre: &str) -> Result<()> {
        if regex::is_match(PRERELEASE_VERSION, pre).unwrap() {
            Ok(())
        } else {
            Err("invalid prerelease version".into())
        }
    }

    /// Check a buildmeta prerelease version.
    pub fn check_buildmeta(build: &str) -> Result<()> {
        if regex::is_match(BUILDMETA_VERSION, build).unwrap() {
            Ok(())
        } else {
            Err("invalid buildmeta version".into())
        }
    }

    /// Check a semver version.
    pub fn check_semver(sv: &str) -> Result<()> {
        if regex::is_match(SEMVER_VERSION, sv).unwrap() {
            Ok(())
        } else {
            Err("invalid semver version".into())
        }
    }

    /// Parse a string as a `Version`.
    pub fn parse(s: &str) -> Result<Version> {
        let matches = regex::captures(SEMVER_VERSION, s)?;

        let _major = matches.get("major").unwrap();
        let major = u32::from_str_radix(_major, 10)
                        .map_err(|e| format!("{}", e))?;

        let _minor = matches.get("minor").unwrap();
        let minor = u32::from_str_radix(_minor, 10)
                        .map_err(|e| format!("{}", e))?;

        let _patch = matches.get("patch").unwrap();
        let patch = u32::from_str_radix(_patch, 10)
                        .map_err(|e| format!("{}", e))?;

        let _prerelease = matches.get("prerelease").unwrap();
        let prerelease = _prerelease.to_owned();
        let _buildmeta = matches.get("buildmeta").unwrap();
        let buildmeta = _buildmeta.to_owned();

        let ver = Version { major, minor, patch, prerelease, buildmeta };

        Ok(ver)
    }

    /// Stringify the `Version`.
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        res.push_str(&format!("{}", self.major));
        res.push_str(&format!(".{}", self.minor));
        res.push_str(&format!(".{}", self.patch));

        if !self.prerelease.is_empty() {
            res.push('-');
            res.push_str(&self.prerelease);
        }

        if !self.buildmeta.is_empty() {
            res.push('+');
            res.push_str(&self.buildmeta);
        }

        res
    }

    fn compare_numeric(n: u32, other: &u32) -> Ordering {
        n.cmp(other)
    }

    fn compare_prerelease(a: &String, b: &String) -> Ordering {
        if a.is_empty() {
            if b.is_empty() {
                return Ordering::Equal;
            }

            return Ordering::Greater;
        }

        if b.is_empty() {
            return Ordering::Less;
        }

        let _a = a.clone();
        let _b = b.clone();

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

    /// Returns if this `Version` is compatible to an other.
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
        Self::check_prerelease(&self.prerelease)?;

        Self::check_buildmeta(&self.buildmeta)?;

        Ok(())
    }
}

impl Datable for Version {}

impl Serializable for Version {}
