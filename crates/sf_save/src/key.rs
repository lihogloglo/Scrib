//! Section keys: `owner/name` (save.md "File format").

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Longest owner, bytes.
pub const MAX_OWNER_LEN: usize = 64;
/// Longest name, bytes.
pub const MAX_NAME_LEN: usize = 128;

/// Identifies a section: its **owner** (`sf` for the framework, the adapter's name, or a mod id)
/// and an owner-local **name** (`overlay`, `script`, `character/<player key>`). Owners map to
/// mods, which drives dormancy (S3).
///
/// Owners are ASCII letters, digits, `_`, `-` and `.`; names also allow `/` between non-empty
/// segments. Keys order by owner, then name: the canonical section order of a file.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "KeyRepr", into = "KeyRepr")]
pub struct SectionKey {
    owner: String,
    name: String,
}

/// Why a key is invalid.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum KeyError {
    #[error("section owner {0:?} must be 1-{MAX_OWNER_LEN} ASCII letters, digits, '_', '-' or '.'")]
    Owner(String),
    #[error("section name {0:?} must be 1-{MAX_NAME_LEN} bytes of non-empty '/'-separated ASCII segments")]
    Name(String),
    #[error("section key {0:?} is not 'owner/name'")]
    Form(String),
}

fn word_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.')
}

fn valid_owner(owner: &str) -> bool {
    !owner.is_empty() && owner.len() <= MAX_OWNER_LEN && owner.bytes().all(word_char)
}

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_NAME_LEN && name.split('/').all(|seg| !seg.is_empty() && seg.bytes().all(word_char))
}

impl SectionKey {
    pub fn new(owner: impl Into<String>, name: impl Into<String>) -> Result<Self, KeyError> {
        let (owner, name) = (owner.into(), name.into());
        if !valid_owner(&owner) {
            return Err(KeyError::Owner(owner));
        }
        if !valid_name(&name) {
            return Err(KeyError::Name(name));
        }
        Ok(Self { owner, name })
    }

    /// Parses `owner/name` (the name may contain further `/`).
    pub fn parse(text: &str) -> Result<Self, KeyError> {
        let (owner, name) = text.split_once('/').ok_or_else(|| KeyError::Form(text.to_owned()))?;
        Self::new(owner, name)
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// The name up to its first `/`: `character` for `character/00ab...`. Registry entries
    /// match on it ([`crate::SectionSpec`]).
    pub fn family(&self) -> &str {
        self.name.split_once('/').map_or(self.name.as_str(), |(head, _)| head)
    }

    /// The name after the family and its `/` (`00ab...` for `character/00ab...`), if any.
    pub fn instance(&self) -> Option<&str> {
        self.name.split_once('/').map(|(_, tail)| tail)
    }
}

impl fmt::Display for SectionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

impl fmt::Debug for SectionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Serialize, Deserialize)]
struct KeyRepr {
    owner: String,
    name: String,
}

impl TryFrom<KeyRepr> for SectionKey {
    type Error = KeyError;

    fn try_from(r: KeyRepr) -> Result<Self, KeyError> {
        Self::new(r.owner, r.name)
    }
}

impl From<SectionKey> for KeyRepr {
    fn from(k: SectionKey) -> Self {
        Self { owner: k.owner, name: k.name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_validate_parse_and_split() {
        let k = SectionKey::parse("sf/character/00ff").unwrap();
        assert_eq!((k.owner(), k.name(), k.family(), k.instance()), ("sf", "character/00ff", "character", Some("00ff")));
        assert_eq!(k.to_string(), "sf/character/00ff");
        assert_eq!(SectionKey::new("better_lanterns", "script").unwrap().family(), "script");
        for bad in ["", "sf", "/x", "sf/", "sf//x", "sf/x/", "s f/x", "sf/x y"] {
            assert!(SectionKey::parse(bad).is_err(), "{bad:?}");
        }
        assert!(SectionKey::new("a".repeat(MAX_OWNER_LEN + 1), "x").is_err());
        assert!(SectionKey::new("a", "x".repeat(MAX_NAME_LEN + 1)).is_err());
    }

    #[test]
    fn keys_order_by_owner_then_name_and_validate_on_decode() {
        let mut keys = [SectionKey::parse("sf/rpg").unwrap(), SectionKey::parse("mw/globals").unwrap(), SectionKey::parse("sf/clock").unwrap()];
        keys.sort();
        let names: Vec<String> = keys.iter().map(ToString::to_string).collect();
        assert_eq!(names, ["mw/globals", "sf/clock", "sf/rpg"]);
        let bytes = postcard::to_stdvec(&KeyRepr { owner: "sf".into(), name: "bad name".into() }).unwrap();
        assert!(postcard::from_bytes::<SectionKey>(&bytes).is_err());
    }
}
