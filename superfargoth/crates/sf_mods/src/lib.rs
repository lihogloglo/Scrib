//! Mods: manifests, the repository, profiles, load order, and the load
//! pipeline with its report (mods.md).
//!
//! Everything that adds content is a mod, including a base game. A mod is
//! described by a [`ModManifest`], usually read from a `mod.toml` at the mod
//! root. Adapters ([`ContentAdapter`]) synthesize manifests for content that
//! predates this format and provide its archives and record formats.
//!
//! Pipeline (Bevy-free): [`ModRepository::scan`] discovers and validates
//! installed mods; a [`Profile`] lists the enabled ones in the player's order;
//! [`load_profile`] resolves the load order ([`resolve_load_order`] keeps the
//! profile's order wherever declared constraints allow), mounts the VFS, and
//! applies record files, reporting every issue per mod in a [`ModReport`].

mod adapter;
mod hot;
mod load;
mod manifest;
mod native;
mod order;
mod profile;
mod repo;
mod report;

pub use adapter::{ContentAdapter, ModFiles, RecordCx, RecordFileError};
pub use hot::HotReloadPlan;
pub use load::{LoadTimings, LoadedMod, LoadedProfile, ScriptPlan, load_profile};
pub use manifest::{Content, MANIFEST_FILE, ManifestError, ModId, ModManifest, UiContent};
pub use native::NativeRecords;
pub use order::{LoadOrderError, resolve_load_order};
pub use profile::{InputBindings, PROFILE_EXTENSION, Profile, ProfileError, ScriptLimits, SettingValues};
pub use repo::ModRepository;
pub use report::{Area, IssueKind, ModIssue, ModReport, ModSummary, Severity};

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_pipeline;
#[cfg(test)]
mod tests_records;
