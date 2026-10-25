//! `mod.toml`: a mod's identity, requirements and content (mods.md "Manifest additions").

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize, Serializer};

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

impl Serialize for ModId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
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
    /// Framework API versions this mod was written against; a mod whose
    /// requirement the runtime's API version doesn't meet is not loaded.
    #[serde(default = "any_version")]
    pub api: VersionReq,
    #[serde(default)]
    pub description: String,
    /// Required mods and acceptable versions. Loaded before this mod.
    #[serde(default)]
    pub dependencies: BTreeMap<ModId, VersionReq>,
    /// Mods loaded before this one when enabled. Enabled with a version outside
    /// the requirement, they make this mod incompatible (Factorio `?` semantics).
    #[serde(default)]
    pub optional: BTreeMap<ModId, VersionReq>,
    /// Mods (and versions) this one refuses to load with (Factorio `!`, Fabric `breaks`).
    #[serde(default)]
    pub conflicts: BTreeMap<ModId, VersionReq>,
    /// Ordering hints that apply only when the named mods are enabled.
    #[serde(default)]
    pub load_after: Vec<ModId>,
    #[serde(default)]
    pub load_before: Vec<ModId>,
    /// Content roots and files.
    #[serde(default)]
    pub content: Content,
    /// UI contributions (ui.md).
    #[serde(default)]
    pub ui: UiContent,
    /// Folder the manifest was loaded from, if any. Asset roots resolve against it.
    #[serde(skip)]
    pub root: Option<PathBuf>,
    /// Name of the [`ContentAdapter`](crate::ContentAdapter) that synthesized this mod.
    #[serde(skip)]
    pub adapter: Option<String>,
}

/// Where a mod's files live.
///
/// `assets` are folders relative to the mod root, mounted into the VFS in
/// order. Every other path (`records`, `scripts`, `ui.styles`) is a VFS path
/// *inside the mod's own asset roots*: a mod with `assets = ["assets"]` lists
/// `assets/records/x.ron` as `records/x.ron`. Record files are read from the
/// mod's own layers, so two mods may ship the same record file name.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    /// Asset roots mounted into the VFS in order (later wins within the mod).
    #[serde(default)]
    pub assets: Vec<PathBuf>,
    /// Record files (native patches or adapter formats, by extension), applied in order.
    #[serde(default)]
    pub records: Vec<String>,
    /// Script entry points, run in order at context start. Must live under
    /// `scripts/<mod id>/` (`sf_script`'s per-mod namespace).
    #[serde(default)]
    pub scripts: Vec<String>,
}

/// UI contributions: stylesheets imported as `@layer mods.<id>` in load order.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiContent {
    #[serde(default)]
    pub styles: Vec<String>,
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
            optional: BTreeMap::new(),
            conflicts: BTreeMap::new(),
            load_after: Vec::new(),
            load_before: Vec::new(),
            content: Content::default(),
            ui: UiContent::default(),
            root: None,
            adapter: None,
        }
    }
}
