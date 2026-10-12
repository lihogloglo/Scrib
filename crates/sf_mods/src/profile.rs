//! Profiles: `<user data>/profiles/<name>.toml` (mods.md "Mods directory and profiles").
//!
//! A profile is what a player picks: enabled mods in preferred order, per-mod
//! setting values, and script resource limits. The CLI and the future mod
//! manager write the same file (MO2/Vortex profiles).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::ModId;

/// File extension of profile files.
pub const PROFILE_EXTENSION: &str = "toml";

/// Per-mod setting values, keyed by setting name. The settings registry
/// (ui.md) gives them types and defaults; the profile only stores values.
pub type SettingValues = BTreeMap<ModId, toml::Table>;

/// One player profile.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    /// The file stem; not stored in the file.
    #[serde(skip)]
    pub name: String,
    /// Enabled mods in the player's preferred order (load order keeps it
    /// wherever dependencies and hints allow).
    #[serde(default)]
    pub enabled: Vec<ModId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub settings: SettingValues,
    #[serde(default, skip_serializing_if = "ScriptLimits::is_unset")]
    pub limits: ScriptLimits,
}

/// Script resource limits for every mod context of this profile. Unset values
/// use the runtime's defaults (scripting.md "Error isolation and limits").
///
/// Limits belong to the player, not the mod: a manifest can't raise its own.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptLimits {
    /// Allocator limit per mod context, in MiB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mib: Option<u32>,
    /// Hard per-call limit, in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_timeout_ms: Option<u32>,
    /// Soft per-frame budget per context, in microseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame_soft_us: Option<u32>,
}

impl ScriptLimits {
    pub fn is_unset(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("failed to access profile {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("invalid profile {path}: {source}")]
    Parse { path: PathBuf, source: toml::de::Error },
    #[error("failed to write profile {name}: {source}")]
    Serialize { name: String, source: toml::ser::Error },
}

impl Profile {
    /// A profile enabling `mods` in this order, with no settings or limits.
    pub fn new(name: impl Into<String>, mods: impl IntoIterator<Item = ModId>) -> Self {
        Self { name: name.into(), enabled: mods.into_iter().collect(), ..Self::default() }
    }

    /// `<profiles_dir>/<name>.toml`.
    pub fn path_in(profiles_dir: &Path, name: &str) -> PathBuf {
        profiles_dir.join(name).with_extension(PROFILE_EXTENSION)
    }

    /// Parses profile text; `name` is the profile's name (normally the file stem).
    pub fn from_toml(name: impl Into<String>, text: &str) -> Result<Self, toml::de::Error> {
        let mut profile: Self = toml::from_str(text)?;
        profile.name = name.into();
        Ok(profile)
    }

    pub fn to_toml(&self) -> Result<String, ProfileError> {
        toml::to_string_pretty(self).map_err(|source| ProfileError::Serialize { name: self.name.clone(), source })
    }

    /// Loads a profile file; its name is the file stem.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ProfileError> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path).map_err(|source| ProfileError::Io { path: path.into(), source })?;
        let name = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        Self::from_toml(name, &text).map_err(|source| ProfileError::Parse { path: path.into(), source })
    }

    /// Writes `<profiles_dir>/<name>.toml`, creating the folder. Returns the path.
    pub fn save(&self, profiles_dir: &Path) -> Result<PathBuf, ProfileError> {
        let path = Self::path_in(profiles_dir, &self.name);
        let io = |source| ProfileError::Io { path: path.clone(), source };
        std::fs::create_dir_all(profiles_dir).map_err(io)?;
        std::fs::write(&path, self.to_toml()?).map_err(io)?;
        Ok(path)
    }

    /// Names of the profiles in a folder, sorted.
    pub fn list(profiles_dir: &Path) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(profiles_dir) else { return Vec::new() };
        let mut names: Vec<String> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().is_some_and(|e| e.eq_ignore_ascii_case(PROFILE_EXTENSION)))
            .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
            .collect();
        names.sort();
        names
    }
}
