//! Glue to the mod pipeline (`sf_mods`): Fluent files from each mod's VFS
//! files and setting values from the player profile.
//!
//! - **Strings**: every mod may ship `l10n/<locale>/<mod id>.ftl` in its own
//!   VFS layers. Sources come out in load order, so a later mod's message
//!   overrides an earlier one ([`Localizer::build`]). A file named after a
//!   different mod is reported against the mod that ships it and ignored: a
//!   translation or rewording mod ships its own `<its id>.ftl` with the
//!   message ids it changes, so files never shadow each other.
//! - **Settings**: [`SettingsStore`] values live in `Profile::settings`
//!   (`[settings.<mod id>]` tables).

use std::collections::BTreeMap;

use sf_mods::{ContentAdapter, LoadedProfile, ModId, Profile};
use sf_records::RecordDb;
use sf_vfs::VfsPath;

use crate::l10n::{L10nIssue, L10nSource, LanguageIdentifier, Localizer, negotiate};
use crate::settings::{SettingError, SettingsStore};

/// Registers the UI record kinds ([`register_kinds`](crate::register_kinds))
/// in the mod pipeline; pass it to `sf_mods::load_profile` beside
/// `NativeRecords` and the content pack's adapters.
#[derive(Clone, Copy, Debug, Default)]
pub struct UiRecordKinds;

impl ContentAdapter for UiRecordKinds {
    fn name(&self) -> &'static str {
        "sf_ui"
    }

    fn register_kinds(&self, db: &mut RecordDb) {
        // Fails only when a kind name is taken, a programming error of the app.
        crate::register_kinds(db).expect("UI record kinds registered twice");
    }
}

/// VFS folder of Fluent files: `l10n/<locale>/<mod id>.ftl`.
pub const L10N_DIR: &str = "l10n";

/// Fluent sources of every loaded mod, in load order (within a mod, by path).
/// Unreadable files, non-UTF-8 text, bad locale folders and files named after
/// another mod become issues of the mod whose layer holds them.
pub fn l10n_sources(loaded: &LoadedProfile) -> (Vec<L10nSource>, Vec<L10nIssue>) {
    // (load-order index of the mod whose layer holds it, path) per `l10n/<locale>/<name>.ftl`.
    let mut found: Vec<(usize, VfsPath)> = Vec::new();
    for path in loaded.vfs.paths() {
        let p = path.as_str();
        let Some(rest) = p.strip_prefix(L10N_DIR).and_then(|r| r.strip_prefix('/')) else {
            continue;
        };
        if path.extension() != Some("ftl") || rest.matches('/').count() != 1 {
            continue;
        }
        for layer in loaded.vfs.providers(path) {
            if let Some(index) = loaded.mods.iter().position(|m| m.layers.contains(&layer.0)) {
                found.push((index, path.clone()));
            }
        }
    }
    found.sort();
    found.dedup();

    let mut sources = Vec::new();
    let mut issues = Vec::new();
    for (index, vfs_path) in found {
        let m = &loaded.mods[index];
        let path = vfs_path.as_str().to_owned();
        let mod_id = m.id().as_str().to_owned();
        let issue = |message: String| L10nIssue {
            mod_id: mod_id.clone(),
            path: path.clone(),
            message,
        };
        let rest = &path[L10N_DIR.len() + 1..];
        let (locale_dir, file) = rest.split_once('/').expect("checked one separator");
        let stem = file.strip_suffix(".ftl").expect("checked suffix");
        if stem != mod_id {
            issues.push(issue(format!(
                "ignored: a mod's Fluent file is `{L10N_DIR}/{locale_dir}/{mod_id}.ftl` \
                 (override another mod's messages from your own file)"
            )));
            continue;
        }
        let Ok(locale) = locale_dir.parse::<LanguageIdentifier>() else {
            issues.push(issue(format!("`{locale_dir}` is not a locale tag")));
            continue;
        };
        let bytes = match m.files(&loaded.vfs).read(&vfs_path) {
            Ok(b) => b,
            Err(e) => {
                issues.push(issue(format!("unreadable: {e}")));
                continue;
            }
        };
        match String::from_utf8(bytes) {
            Ok(text) => sources.push(L10nSource {
                mod_id: mod_id.clone(),
                locale,
                path: path.clone(),
                text,
            }),
            Err(_) => issues.push(issue("not UTF-8 text".to_owned())),
        }
    }
    (sources, issues)
}

/// Locales that have at least one source, in first-seen order.
pub fn available_locales(sources: &[L10nSource]) -> Vec<LanguageIdentifier> {
    let mut out: Vec<LanguageIdentifier> = Vec::new();
    for s in sources {
        if !out.contains(&s.locale) {
            out.push(s.locale.clone());
        }
    }
    out
}

impl Localizer {
    /// Strings of a loaded profile: every mod's Fluent files, the chain
    /// negotiated from `requested` (player preference first) over the locales
    /// mods ship, ending with `default`. Issues: file problems (see
    /// [`l10n_sources`]) and Fluent syntax errors, each with its mod.
    pub fn from_profile(
        loaded: &LoadedProfile,
        requested: &[&str],
        default: &LanguageIdentifier,
        use_isolating: bool,
    ) -> (Self, Vec<L10nIssue>) {
        let (sources, mut issues) = l10n_sources(loaded);
        let chain = negotiate(requested, &available_locales(&sources), default);
        let (localizer, syntax) = Self::build(&chain, &sources, use_isolating);
        issues.extend(syntax);
        (localizer, issues)
    }
}

impl SettingsStore {
    /// Reads `profile.settings` (see [`SettingsStore::load_profile`]).
    pub fn load_from(&mut self, profile: &Profile) -> Vec<SettingError> {
        let tables: BTreeMap<String, toml::Table> = profile
            .settings
            .iter()
            .map(|(id, t)| (id.as_str().to_owned(), t.clone()))
            .collect();
        self.load_profile(&tables)
    }

    /// Replaces `profile.settings` with the values to save (non-defaults and
    /// passed-through entries). Returns whether it changed. Owners are mod ids,
    /// so every table key is valid; an invalid one would be dropped.
    pub fn save_to(&self, profile: &mut Profile) -> bool {
        let settings: BTreeMap<ModId, toml::Table> = self
            .to_profile()
            .into_iter()
            .filter(|(_, t)| !t.is_empty())
            .filter_map(|(id, t)| Some((ModId::new(id).ok()?, t)))
            .collect();
        let changed = profile.settings != settings;
        profile.settings = settings;
        changed
    }
}
