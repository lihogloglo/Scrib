//! Section versions and the migration runner (save.md "Versioning and migration").
//!
//! Each section family has a current schema version, a minimum it can still convert, and one
//! step per older version (`v → v + 1`). Load runs the steps from the stored version up; saves
//! are only ever written at the current version (one-way, like Minecraft's `DataFixerUpper`). The
//! owning crate keeps frozen copies of its older types (`legacy::v1::Thing`) for typed steps.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::codec::{decode, encode};
use crate::key::SectionKey;

/// One migration step on raw section bytes: version `v` in, `v + 1` out.
pub type StepFn = Arc<dyn Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync>;

/// Which keys a spec covers: an owner (or any owner) and a family (the name up to its first
/// `/`, see [`SectionKey::family`]). `sf/character/<key>` is family `character` of owner `sf`;
/// `<mod id>/script` is family `script` of any owner.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Family {
    /// `None`: any owner.
    pub owner: Option<String>,
    pub family: String,
}

impl Family {
    pub fn matches(&self, key: &SectionKey) -> bool {
        self.owner.as_deref().is_none_or(|o| o == key.owner()) && self.family == key.family()
    }
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner.as_deref().unwrap_or("*"), self.family)
    }
}

/// The versions and steps of one section family.
#[derive(Clone)]
pub struct SectionSpec {
    family: Family,
    current: u32,
    min: u32,
    /// Shown when a save is older than `min` ("load it in 0.4 and save again").
    too_old_hint: String,
    steps: BTreeMap<u32, StepFn>,
}

impl fmt::Debug for SectionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionSpec")
            .field("family", &self.family)
            .field("current", &self.current)
            .field("min", &self.min)
            .field("steps", &self.steps.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

impl SectionSpec {
    /// Family `family` of `owner` at schema version `current` (≥ 1). With no steps, only
    /// `current` loads.
    pub fn new(owner: impl Into<String>, family: impl Into<String>, current: u32) -> Self {
        Self::of(Family { owner: Some(owner.into()), family: family.into() }, current)
    }

    /// Family `family` of every owner (`<mod id>/script`).
    pub fn any_owner(family: impl Into<String>, current: u32) -> Self {
        Self::of(Family { owner: None, family: family.into() }, current)
    }

    fn of(family: Family, current: u32) -> Self {
        assert!(current >= 1, "section versions start at 1");
        Self { family, current, min: current, too_old_hint: String::new(), steps: BTreeMap::new() }
    }

    /// Adds the step from `from` to `from + 1` on raw bytes. Steps must cover every version from
    /// the oldest one given up to `current`.
    #[must_use]
    pub fn step(mut self, from: u32, f: impl Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync + 'static) -> Self {
        assert!(from < self.current, "a step from {from} goes past current version {}", self.current);
        self.steps.insert(from, Arc::new(f));
        self.min = self.min.min(from);
        self
    }

    /// Adds a typed step: decode the frozen old type, convert, encode the next one.
    #[must_use]
    pub fn typed_step<Old, New>(self, from: u32, f: impl Fn(Old) -> New + Send + Sync + 'static) -> Self
    where
        Old: DeserializeOwned,
        New: Serialize,
    {
        self.step(from, move |bytes| decode::<Old>(bytes).map(|old| encode(&f(old))).map_err(|e| e.0))
    }

    /// The message for saves older than the oldest step (`OpenMW` names the last release that
    /// converts them).
    #[must_use]
    pub fn too_old_hint(mut self, hint: impl Into<String>) -> Self {
        self.too_old_hint = hint.into();
        self
    }

    pub fn family(&self) -> &Family {
        &self.family
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    /// The oldest version that still loads.
    pub fn min(&self) -> u32 {
        self.min
    }
}

/// A migration that can't run.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum MigrateError {
    #[error("no registered owner for section {0}")]
    Unknown(SectionKey),
    #[error("section {key} is version {version}, newer than this build's {current}: update the game")]
    TooNew { key: SectionKey, version: u32, current: u32 },
    #[error("section {key} is version {version}, older than the oldest this build converts ({min}){hint}")]
    TooOld { key: SectionKey, version: u32, min: u32, hint: String },
    #[error("section {key}: migrating from version {from}: {message}")]
    Step { key: SectionKey, from: u32, message: String },
}

/// Why a spec can't be registered.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RegistryError {
    #[error("section family {0} registered twice")]
    Duplicate(Family),
    #[error("section family {family} lacks the step from version {missing}")]
    Gap { family: Family, missing: u32 },
}

/// Every known section family. Exact-owner specs win over any-owner ones.
#[derive(Clone, Debug, Default)]
pub struct Registry {
    specs: BTreeMap<Family, SectionSpec>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, spec: SectionSpec) -> Result<(), RegistryError> {
        if let Some(missing) = (spec.min..spec.current).find(|v| !spec.steps.contains_key(v)) {
            return Err(RegistryError::Gap { family: spec.family, missing });
        }
        if self.specs.contains_key(&spec.family) {
            return Err(RegistryError::Duplicate(spec.family));
        }
        self.specs.insert(spec.family.clone(), spec);
        Ok(())
    }

    /// The spec covering `key`.
    pub fn spec(&self, key: &SectionKey) -> Option<&SectionSpec> {
        let exact = Family { owner: Some(key.owner().to_owned()), family: key.family().to_owned() };
        self.specs.get(&exact).or_else(|| self.specs.get(&Family { owner: None, family: exact.family }))
    }

    pub fn specs(&self) -> impl Iterator<Item = &SectionSpec> {
        self.specs.values()
    }

    /// Brings `raw` (stored at `version`) to the current version. Returns the bytes and whether
    /// any step ran.
    pub fn migrate(&self, key: &SectionKey, version: u32, raw: Vec<u8>) -> Result<(Vec<u8>, bool), MigrateError> {
        let spec = self.spec(key).ok_or_else(|| MigrateError::Unknown(key.clone()))?;
        if version > spec.current {
            return Err(MigrateError::TooNew { key: key.clone(), version, current: spec.current });
        }
        if version < spec.min {
            let hint = if spec.too_old_hint.is_empty() { String::new() } else { format!(": {}", spec.too_old_hint) };
            return Err(MigrateError::TooOld { key: key.clone(), version, min: spec.min, hint });
        }
        let mut bytes = raw;
        for from in version..spec.current {
            bytes = spec.steps[&from](&bytes).map_err(|message| MigrateError::Step { key: key.clone(), from, message })?;
        }
        Ok((bytes, version < spec.current))
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct V1 {
        hp: u16,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct V2 {
        hp: u32,
        max: u32,
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct V3 {
        health: f32,
        max: f32,
    }

    fn spec() -> SectionSpec {
        SectionSpec::new("game", "body", 3)
            .typed_step(1, |v: V1| V2 { hp: u32::from(v.hp), max: 100 })
            .typed_step(2, |v: V2| V3 { health: f32::from(u16::try_from(v.hp).unwrap()), max: f32::from(u16::try_from(v.max).unwrap()) })
            .too_old_hint("load it in 0.4 first")
    }

    fn key(text: &str) -> SectionKey {
        SectionKey::parse(text).unwrap()
    }

    #[test]
    fn migrates_stepwise_from_every_supported_version() {
        let mut r = Registry::new();
        r.register(spec()).unwrap();
        let k = key("game/body");
        let (b, ran) = r.migrate(&k, 1, encode(&V1 { hp: 7 })).unwrap();
        assert!(ran);
        assert_eq!(decode::<V3>(&b).unwrap(), V3 { health: 7.0, max: 100.0 });
        let (b, _) = r.migrate(&k, 2, encode(&V2 { hp: 3, max: 9 })).unwrap();
        assert_eq!(decode::<V3>(&b).unwrap(), V3 { health: 3.0, max: 9.0 });
        let current = encode(&V3 { health: 1.0, max: 2.0 });
        assert_eq!(r.migrate(&k, 3, current.clone()).unwrap(), (current, false));
    }

    #[test]
    fn refuses_too_new_too_old_unknown_and_bad_bytes() {
        let mut r = Registry::new();
        r.register(spec().clone()).unwrap();
        let k = key("game/body");
        assert!(matches!(r.migrate(&k, 4, vec![]), Err(MigrateError::TooNew { current: 3, .. })));
        assert!(matches!(r.migrate(&k, 0, vec![]), Err(MigrateError::TooOld { min: 1, .. })));
        let msg = r.migrate(&k, 0, vec![]).unwrap_err().to_string();
        assert!(msg.contains("load it in 0.4 first"), "{msg}");
        assert!(matches!(r.migrate(&key("game/other"), 1, vec![]), Err(MigrateError::Unknown(_))));
        assert!(matches!(r.migrate(&k, 1, vec![1, 2, 3, 4, 5]), Err(MigrateError::Step { from: 1, .. })));
    }

    #[test]
    fn exact_owner_wins_and_gaps_and_duplicates_are_refused() {
        let mut r = Registry::new();
        r.register(SectionSpec::any_owner("script", 1)).unwrap();
        r.register(SectionSpec::new("special", "script", 2).step(1, |b| Ok(b.to_vec()))).unwrap();
        assert_eq!(r.spec(&key("some_mod/script")).unwrap().current(), 1);
        assert_eq!(r.spec(&key("special/script")).unwrap().current(), 2);
        assert_eq!(r.spec(&key("sf/character/00ff")).map(SectionSpec::current), None);
        assert!(matches!(r.register(SectionSpec::any_owner("script", 1)), Err(RegistryError::Duplicate(_))));
        let gap = SectionSpec::new("x", "y", 3).step(1, |b| Ok(b.to_vec()));
        assert!(matches!(r.register(gap), Err(RegistryError::Gap { missing: 2, .. })));
    }
}
