use crate::interface::pow::{PowHashComponent, St};
use rsa::signature::digest::Digest;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Represents a valid h-mail address - same as email addresses but with a '#' replacing the '@'
#[cfg_attr(feature = "gen_docs", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HmailAddress(String);

impl HmailAddress {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, &'static str> {
        let s = s.as_ref();
        let mut split = s.split("#");

        let Some(username) = split.next() else {
            return Err("h-mail address can not be empty")
        };
        let Some(domain) = split.next() else {
            return Err("h-mail address must have a '#' between the username and domain")
        };
        if split.next().is_some() {
            return Err("h-mail addresses may only have one '#'")
        }
        if username.is_empty() {
            return Err("h-mail addresses must have a username before the '#'")
        }
        if domain.is_empty() {
            return Err("h-mail addresses must have a domain after the '#'")
        }

        Ok(HmailAddress(s.to_string()))
    }

    pub fn from_username_domain<S1: AsRef<str>, S2: AsRef<str>>(username: S1, domain: S2) -> Result<Self, &'static str> {
        Self::new(&format!("{}#{}", username.as_ref(), domain.as_ref()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn username(&self) -> &str {
        self.0.split('#').next().unwrap()
    }

    pub fn domain(&self) -> &str {
        self.0.split('#').next_back().unwrap()
    }
}

impl FromStr for HmailAddress {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for HmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for HmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for HmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HmailAddress::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl PowHashComponent for HmailAddress {
    fn update_hash(&self, sha256: &mut St) {
        sha256.update(self.0.as_bytes())
    }
}