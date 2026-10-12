//! `ModReport`: every problem and every override, attributed to a mod
//! (mods.md "Validation and conflict reports").

use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::path::PathBuf;

use semver::{Version, VersionReq};
use serde::Serialize;

use crate::ModId;

/// How bad an issue is. Errors mean the mod (or one of its files) was not loaded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Pipeline stage an issue belongs to (the report's sections).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Area {
    Manifest,
    Order,
    Files,
    Records,
    Adapter,
}

/// One problem found while scanning or loading.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModIssue {
    /// The mod it belongs to; `None` for folder-level problems before an id is known.
    pub mod_id: Option<ModId>,
    pub kind: IssueKind,
}

/// Every issue type of pipeline steps 1-4 (scripts and UI add theirs in L2/U2).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum IssueKind {
    // Manifest
    /// `mod.toml` failed to parse (includes unknown fields and invalid ids).
    ManifestParse { path: PathBuf, message: String },
    /// A folder in a mods directory has no `mod.toml`.
    NoManifest { path: PathBuf },
    /// Two installed mods share an id; the later one is ignored.
    DuplicateId { first: String, ignored: String },
    MissingDescription,
    // Order
    /// The profile enables a mod that isn't installed.
    UnknownMod,
    DuplicateInProfile,
    /// The profile enables a mod that failed validation (see its other errors).
    InvalidMod,
    /// The profile's settings name a mod that isn't enabled.
    SettingsForDisabledMod,
    MissingDependency { dependency: ModId, not_loaded: bool },
    VersionMismatch { dependency: ModId, required: VersionReq, found: Version },
    OptionalVersionMismatch { dependency: ModId, required: VersionReq, found: Version },
    ConflictPresent { other: ModId, declared: VersionReq, found: Version },
    Cycle { members: Vec<ModId> },
    ApiIncompatible { required: VersionReq, provided: Version },
    OptionalMissing { dependency: ModId },
    // Files
    MissingAssetRoot { root: PathBuf },
    UnreadableAssetRoot { root: PathBuf, message: String },
    EmptyAssetRoot { root: PathBuf },
    MissingRecordFile { path: String },
    MissingEntryPoint { path: String },
    /// An entry point outside `scripts/<mod id>/`.
    EntryPointOutsideNamespace { path: String },
    MissingStyle { path: String },
    InvalidPath { path: String, message: String },
    // Records
    /// No loader handles this record file extension.
    UnknownRecordFormat { path: String },
    /// A record file failed to apply (from the native loader or an adapter).
    RecordFile { path: String, line: Option<u32>, column: Option<u32>, message: String },
    /// A non-fatal record diagnostic from a loader.
    RecordWarning { path: String, message: String },
    // Adapter
    /// The adapter that synthesized this mod isn't registered.
    UnknownAdapter { adapter: String },
    /// The adapter failed to discover or mount something.
    Adapter { adapter: String, message: String },
}

impl IssueKind {
    pub fn severity(&self) -> Severity {
        match self {
            Self::MissingDescription
            | Self::SettingsForDisabledMod
            | Self::DuplicateInProfile
            | Self::EmptyAssetRoot { .. }
            | Self::RecordWarning { .. }
            | Self::NoManifest { .. } => Severity::Warning,
            Self::OptionalMissing { .. } => Severity::Info,
            _ => Severity::Error,
        }
    }

    pub fn area(&self) -> Area {
        match self {
            Self::ManifestParse { .. } | Self::NoManifest { .. } | Self::DuplicateId { .. } | Self::MissingDescription => {
                Area::Manifest
            }
            Self::UnknownMod
            | Self::DuplicateInProfile
            | Self::InvalidMod
            | Self::SettingsForDisabledMod
            | Self::MissingDependency { .. }
            | Self::VersionMismatch { .. }
            | Self::OptionalVersionMismatch { .. }
            | Self::ConflictPresent { .. }
            | Self::Cycle { .. }
            | Self::ApiIncompatible { .. }
            | Self::OptionalMissing { .. } => Area::Order,
            Self::MissingAssetRoot { .. }
            | Self::UnreadableAssetRoot { .. }
            | Self::EmptyAssetRoot { .. }
            | Self::MissingRecordFile { .. }
            | Self::MissingEntryPoint { .. }
            | Self::EntryPointOutsideNamespace { .. }
            | Self::MissingStyle { .. }
            | Self::InvalidPath { .. } => Area::Files,
            Self::UnknownRecordFormat { .. } | Self::RecordFile { .. } | Self::RecordWarning { .. } => Area::Records,
            Self::UnknownAdapter { .. } | Self::Adapter { .. } => Area::Adapter,
        }
    }
}

impl fmt::Display for IssueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManifestParse { path, message } => write!(f, "invalid manifest {}: {message}", path.display()),
            Self::NoManifest { path } => write!(f, "{} has no mod.toml; skipped", path.display()),
            Self::DuplicateId { first, ignored } => write!(f, "installed twice ({first}); ignoring {ignored}"),
            Self::MissingDescription => f.write_str("manifest has no description"),
            Self::UnknownMod => f.write_str("enabled in the profile but not installed"),
            Self::DuplicateInProfile => f.write_str("enabled more than once in the profile; first position kept"),
            Self::InvalidMod => f.write_str("not loaded: the mod has errors"),
            Self::SettingsForDisabledMod => f.write_str("profile has settings for this mod, but it is not enabled"),
            Self::MissingDependency { dependency, not_loaded: false } => {
                write!(f, "not loaded: requires {dependency}, which is not enabled")
            }
            Self::MissingDependency { dependency, not_loaded: true } => {
                write!(f, "not loaded: requires {dependency}, which was not loaded")
            }
            Self::VersionMismatch { dependency, required, found } => {
                write!(f, "not loaded: requires {dependency} {required}, found {found}")
            }
            Self::OptionalVersionMismatch { dependency, required, found } => {
                write!(f, "not loaded: supports {dependency} {required} when enabled, found {found}")
            }
            Self::ConflictPresent { other, declared, found } => {
                write!(f, "not loaded: declares a conflict with {other} {declared}, and {other} {found} is enabled")
            }
            Self::Cycle { members } => {
                let ids: Vec<&str> = members.iter().map(ModId::as_str).collect();
                write!(f, "not loaded: load order cycle among {}", ids.join(", "))
            }
            Self::ApiIncompatible { required, provided } => {
                write!(f, "not loaded: needs framework API {required}, this build provides {provided}")
            }
            Self::OptionalMissing { dependency } => write!(f, "optional {dependency} is not enabled"),
            Self::MissingAssetRoot { root } => write!(f, "asset root {} does not exist", root.display()),
            Self::UnreadableAssetRoot { root, message } => {
                write!(f, "asset root {} unreadable: {message}", root.display())
            }
            Self::EmptyAssetRoot { root } => write!(f, "asset root {} is empty", root.display()),
            Self::MissingRecordFile { path } => write!(f, "record file {path} not found in the mod's assets"),
            Self::MissingEntryPoint { path } => write!(f, "script entry point {path} not found in the mod's assets"),
            Self::EntryPointOutsideNamespace { path } => {
                write!(f, "script entry point {path} must be under scripts/<mod id>/")
            }
            Self::MissingStyle { path } => write!(f, "stylesheet {path} not found in the mod's assets"),
            Self::InvalidPath { path, message } => write!(f, "invalid path {path:?}: {message}"),
            Self::UnknownRecordFormat { path } => write!(f, "no loader for record file {path}"),
            Self::RecordFile { path, line, column, message } => {
                write!(f, "record file {path}")?;
                if let Some(line) = line {
                    write!(f, ":{line}")?;
                    if let Some(column) = column {
                        write!(f, ":{column}")?;
                    }
                }
                write!(f, ": {message}")
            }
            Self::RecordWarning { path, message } => write!(f, "record file {path}: {message}"),
            Self::UnknownAdapter { adapter } => write!(f, "synthesized by adapter {adapter:?}, which is not registered"),
            Self::Adapter { adapter, message } => write!(f, "adapter {adapter}: {message}"),
        }
    }
}

impl ModIssue {
    pub fn new(mod_id: Option<&ModId>, kind: IssueKind) -> Self {
        Self { mod_id: mod_id.cloned(), kind }
    }

    pub fn severity(&self) -> Severity {
        self.kind.severity()
    }
}

impl fmt::Display for ModIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity() {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        match &self.mod_id {
            Some(id) => write!(f, "{severity} [{id}] {}", self.kind),
            None => write!(f, "{severity} {}", self.kind),
        }
    }
}

/// What a loaded mod contributes (the MO2 conflict view, as counts per mod).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ModSummary {
    pub id: ModId,
    pub version: Version,
    /// Folder path, or `adapter:<name>` for synthesized mods.
    pub origin: String,
    /// Labels of the VFS layers mounted for this mod, lowest first.
    pub layers: Vec<String>,
    /// Distinct paths the mod provides.
    pub files: usize,
    /// Paths where the mod's file wins.
    pub winning: usize,
    /// Paths no other loaded mod provides.
    pub unique: usize,
    /// Mods whose files this mod's winning files override, with path counts.
    pub overrides: BTreeMap<ModId, usize>,
    /// Mods whose winning files override this mod's, with path counts.
    pub overridden_by: BTreeMap<ModId, usize>,
    pub record_files: usize,
    pub script_entry_points: usize,
    pub styles: usize,
}

/// Everything the pipeline found, per mod, in load order.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ModReport {
    pub issues: Vec<ModIssue>,
    /// Loaded mods in load order.
    pub mods: Vec<ModSummary>,
}

impl ModReport {
    pub fn count(&self, severity: Severity) -> usize {
        self.issues.iter().filter(|i| i.severity() == severity).count()
    }

    pub fn has_errors(&self) -> bool {
        self.count(Severity::Error) > 0
    }

    /// Issues for one mod (or the folder-level ones with `None`).
    pub fn issues_for<'a>(&'a self, id: Option<&'a ModId>) -> impl Iterator<Item = &'a ModIssue> + 'a {
        self.issues.iter().filter(move |i| i.mod_id.as_ref() == id)
    }

    /// Text rendering for logs and the CLI; `only` restricts it to one mod.
    /// The record section is `sf_records::ConflictReport::render`, appended by the caller.
    pub fn render(&self, only: Option<&ModId>) -> String {
        let wanted = |id: Option<&ModId>| only.is_none() || id == only;
        let mut out = format!(
            "mods: {} loaded, {} errors, {} warnings\n",
            self.mods.len(),
            self.count(Severity::Error),
            self.count(Severity::Warning)
        );
        for (i, m) in self.mods.iter().enumerate().filter(|(_, m)| wanted(Some(&m.id))) {
            let _ = writeln!(
                out,
                "{:>3}. {} {} ({}): files {} (winning {}, unique {}), records {}, scripts {}, styles {}",
                i + 1,
                m.id,
                m.version,
                m.origin,
                m.files,
                m.winning,
                m.unique,
                m.record_files,
                m.script_entry_points,
                m.styles
            );
            for (other, n) in &m.overrides {
                let _ = writeln!(out, "       overrides {n} files of {other}");
            }
            for (other, n) in &m.overridden_by {
                let _ = writeln!(out, "       {n} files overridden by {other}");
            }
        }
        let mut issues: Vec<&ModIssue> = self.issues.iter().filter(|i| wanted(i.mod_id.as_ref())).collect();
        issues.sort_by_key(|i| (std::cmp::Reverse(i.severity()), i.kind.area()));
        for issue in issues {
            let _ = writeln!(out, "{issue}");
        }
        out
    }
}
