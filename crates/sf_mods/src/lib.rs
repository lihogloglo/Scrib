//! Mod manifests and load-order resolution.
//!
//! Everything that adds content is a mod, including a base game. A mod is
//! described by a [`ModManifest`], usually read from a `mod.toml` at the mod
//! root. Adapters can also synthesize manifests (for example, for legacy
//! plugins that predate this format).
//!
//! The user's profile lists enabled mods in their preferred order.
//! [`resolve_load_order`] keeps that order wherever the mods' declared
//! constraints allow it.

mod order;

pub use order::{LoadOrderError, resolve_load_order};

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};
use serde::Deserialize;

/// Manifest file name at the root of a mod folder.
pub const MANIFEST_FILE: &str = "mod.toml";

/// Stable mod identifier: lowercase ASCII letters, digits, `_`, `-`, `.`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct ModId(String);

impl ModId {
    pub fn new(id: impl Into<String>) -> Result<Self, ManifestError> {
        let id = id.into();
        let valid = !id.is_empty()
            && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.'));
        if valid { Ok(Self(id)) } else { Err(ManifestError::InvalidId(id)) }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ModId {
    type Error = ManifestError;
    fn try_from(id: String) -> Result<Self, Self::Error> {
        Self::new(id)
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A mod's declared identity, requirements, and content.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModManifest {
    pub id: ModId,
    pub name: String,
    pub version: Version,
    /// Framework API versions this mod was written against.
    #[serde(default = "any_version")]
    pub api: VersionReq,
    #[serde(default)]
    pub description: String,
    /// Required mods and acceptable versions. Loaded before this mod.
    #[serde(default)]
    pub dependencies: BTreeMap<ModId, VersionReq>,
    /// Ordering hints that apply only when the named mods are enabled.
    #[serde(default)]
    pub load_after: Vec<ModId>,
    #[serde(default)]
    pub load_before: Vec<ModId>,
    /// Content roots, relative to the mod folder.
    #[serde(default)]
    pub content: Content,
    /// Folder the manifest was loaded from, if any.
    #[serde(skip)]
    pub root: Option<PathBuf>,
}

/// Where a mod's files live, relative to its root.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    /// Asset roots mounted into the VFS in order (later wins within the mod).
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    /// Record files (native patches or adapter-specific formats), in order.
    #[serde(default)]
    pub records: Vec<PathBuf>,
    /// Script entry points.
    #[serde(default)]
    pub scripts: Vec<PathBuf>,
}

fn any_version() -> VersionReq {
    VersionReq::STAR
}

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("invalid mod id {0:?}: use lowercase ASCII letters, digits, '_', '-', '.'")]
    InvalidId(String),
    #[error("failed to read {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("invalid manifest {path}: {source}")]
    Parse { path: PathBuf, source: toml::de::Error },
}

impl ModManifest {
    /// Parses a manifest from TOML text.
    pub fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(text)
    }

    /// Loads `mod.toml` from a mod folder.
    pub fn load_dir(dir: impl AsRef<Path>) -> Result<Self, ManifestError> {
        let dir = dir.as_ref();
        let path = dir.join(MANIFEST_FILE);
        let text = std::fs::read_to_string(&path).map_err(|source| ManifestError::Io { path: path.clone(), source })?;
        let mut manifest = Self::from_toml(&text).map_err(|source| ManifestError::Parse { path, source })?;
        manifest.root = Some(dir.to_path_buf());
        Ok(manifest)
    }

    /// A manifest with no constraints, for adapters that synthesize mods.
    pub fn synthesized(id: ModId, name: impl Into<String>, version: Version) -> Self {
        Self {
            id,
            name: name.into(),
            version,
            api: any_version(),
            description: String::new(),
            dependencies: BTreeMap::new(),
            load_after: Vec::new(),
            load_before: Vec::new(),
            content: Content::default(),
            root: None,
        }
    }
}

#[cfg(test)]
mod tests;
