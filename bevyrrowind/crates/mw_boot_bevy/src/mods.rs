//! The mod pipeline's library half (`superfargoth/docs/specs/mods.md`): every
//! command loads content through a profile (`--profile`, default `default`)
//! over `<user data>/mods` (`--user-data`). The `bevyrrowind mods ...` CLI
//! (inspect and edit profiles) stays in the binary (`apps/bevyrrowind/src/mods.rs`).

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::Instant;

use mw_install::MorrowindInstall;
use mw_mods::{Mods, MwAdapter, UserDirs};
use mw_records::LoadedContent;
use sf_mods::{LoadedProfile, Severity};
use sf_vfs::{MemSource, Vfs, VfsSource};

use crate::Result;

/// The profile and user data folder every command loads with (set once from
/// the global CLI flags in `run`).
#[derive(Clone, Debug)]
pub struct Selection {
    pub user_data: Option<PathBuf>,
    pub profile: String,
}

static SELECTION: OnceLock<Selection> = OnceLock::new();

/// The profiles folder and the loaded profile, for the keymap (`controls.rs`), set by [`load`].
static BINDINGS_PROFILE: OnceLock<(PathBuf, sf_mods::Profile)> = OnceLock::new();

/// The profiles folder and the profile the last [`load`] resolved (`None` before one): the
/// player's bindings load from it and save back into its file.
pub fn bindings_profile() -> Option<(PathBuf, sf_mods::Profile)> {
    BINDINGS_PROFILE.get().cloned()
}

/// The user data folder (`--user-data`, `BEVYRROWIND_USER_DATA`, else the platform's local data
/// folder): `profiles/` and `saves/` (`saves.rs`) live in it.
pub fn user_data() -> Option<PathBuf> {
    UserDirs::detect(selection().user_data.as_deref()).map(|u| u.root)
}

/// The loaded profile's script contexts (`LoadedProfile::script_contexts`), set by [`load`].
static SCRIPT_PLANS: OnceLock<Vec<sf_mods::ScriptPlan>> = OnceLock::new();

/// Script contexts of the profile the last [`load`] resolved (empty before one).
pub fn script_plans() -> Vec<sf_mods::ScriptPlan> {
    SCRIPT_PLANS.get().cloned().unwrap_or_default()
}

/// The content pack every profile loads (`sf/meta` `content_id`): the Morrowind adapter's name.
pub const CONTENT_ID: &str = mw_mods::ADAPTER;

/// One mod of a resolved load order: id, version and the blake3 of its manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModIdentity {
    pub id: String,
    pub version: String,
    pub manifest_hash: [u8; 32],
}

/// The identity of the content the last [`load_content`] resolved: what a save records in
/// `sf/meta` (save.md "What is saved"; mods.md "Profile fingerprint").
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProfileIdentity {
    pub profile: String,
    /// Loaded mods in load order.
    pub load_order: Vec<ModIdentity>,
    /// The profile fingerprint (`LoadedProfile::fingerprint`).
    pub fingerprint: [u8; 32],
}

impl ProfileIdentity {
    /// The identity of a resolved profile and its fingerprint.
    pub fn of(loaded: &LoadedProfile, fingerprint: [u8; 32]) -> Self {
        let load_order = loaded
            .mods
            .iter()
            .map(|m| ModIdentity {
                id: m.manifest.id.as_str().to_owned(),
                version: m.manifest.version.to_string(),
                // The manifest as the fingerprint hashes it (`sf_mods` `LoadedProfile::fingerprint`).
                manifest_hash: sf_bake::Digest::of(format!("{:?}", m.manifest).as_bytes()).0,
            })
            .collect();
        Self { profile: loaded.profile.name.clone(), load_order, fingerprint }
    }
}

/// The last resolved identity (replaced by every [`load_content`]).
static IDENTITY: Mutex<Option<ProfileIdentity>> = Mutex::new(None);

/// The identity of the content the last [`load_content`] resolved (`None` before one).
pub fn profile_identity() -> Option<ProfileIdentity> {
    IDENTITY.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

pub fn select(selection: Selection) {
    let _ = SELECTION.set(selection);
}

fn selection() -> Selection {
    SELECTION.get().cloned().unwrap_or_else(|| Selection { user_data: None, profile: mw_mods::DEFAULT_PROFILE.into() })
}


/// The builtin mod's layers: the shipped animation graphs and the `mw` Luau
/// runtime (scripting-adapter.md "The `mw` runtime library"), and the Morrowind UI theme
/// (`ui_theme.rs`, palette from the install's Morrowind.ini).
fn builtin_layers(install: &MorrowindInstall) -> Vec<Arc<dyn VfsSource>> {
    let table = mw_script::functions::FunctionTable::vanilla();
    let runtime = MemSource::new("builtin:mw_script")
        .with(mw_mods::RUNTIME_PATH, mw_script::emit::RUNTIME)
        .with(mw_mods::RUNTIME_FUNCTIONS_PATH, mw_script::emit::runtime_functions(table));
    vec![
        Arc::new(mw_graphs::builtin_source()),
        Arc::new(runtime),
        Arc::new(crate::ui_theme::source(install.ini.as_ref())),
    ]
}

/// Scans mods and the install and selects the profile. `records: false`
/// mounts only (content files aren't parsed).
pub fn open(install: &MorrowindInstall, records: bool) -> Result<Mods> {
    let selection = selection();
    let user = UserDirs::detect(selection.user_data.as_deref()).ok_or("no user data folder; pass --user-data")?;
    let mut adapter = MwAdapter::new(install.clone(), builtin_layers(install));
    if !records {
        adapter = adapter.without_records();
    }
    Ok(Mods::open(adapter, user, &selection.profile)?)
}

/// Loads the profile: prints errors (always) and the issue count, plus
/// mount warnings.
pub fn load(mods: &Mods) -> LoadedProfile {
    let loaded = mods.load(&sf_script::API_VERSION);
    // Script contexts of this profile, for the streamed view's runtime (`scripts.rs`, M5 L2).
    let _ = SCRIPT_PLANS.set(loaded.script_contexts());
    if let Some(user) = UserDirs::detect(selection().user_data.as_deref()) {
        let _ = BINDINGS_PROFILE.set((user.profiles(), loaded.profile.clone()));
    }
    for warning in mods.adapter.take_mount_warnings() {
        eprintln!("warning: {warning}");
    }
    let errors: Vec<_> = loaded.report.issues.iter().filter(|i| i.severity() == Severity::Error).collect();
    for issue in &errors {
        eprintln!("error: {}{}", issue.mod_id.as_ref().map(|m| format!("{m}: ")).unwrap_or_default(), issue.kind);
    }
    let warnings = loaded.report.issues.iter().filter(|i| i.severity() == Severity::Warning).count();
    if !errors.is_empty() || warnings > 0 {
        eprintln!("mods: {} errors, {warnings} warnings (`bevyrrowind mods report`)", errors.len());
    }
    loaded
}

/// The VFS of the selected profile (no records).
pub fn mount(install: &MorrowindInstall, verbose: bool) -> Result<Vfs> {
    let start = Instant::now();
    let mods = open(install, false)?;
    let loaded = load(&mods);
    let vfs = Arc::try_unwrap(loaded.vfs).map_err(|_| "VFS still shared")?;
    if verbose {
        print_vfs(&vfs, start.elapsed());
    }
    Ok(vfs)
}

fn print_vfs(vfs: &Vfs, elapsed: std::time::Duration) {
    println!("VFS: {} files in {elapsed:.0?}", vfs.len());
    for s in vfs.stats() {
        println!("  {:<20} {:>7} files, {:>7} winning", s.label, s.files, s.winning);
    }
}

/// VFS and records of the selected profile, its fingerprint, and its VFS sources' texture
/// profiles (each mod's `[texture]`, textures-adapter.md §8).
pub fn load_content(
    install: &MorrowindInstall,
    verbose: bool,
) -> Result<(Vfs, LoadedContent, [u8; 32], mw_bake::texture_profiles::TextureSources)> {
    let start = Instant::now();
    let mods = open(install, true)?;
    let mut loaded = load(&mods);
    let fingerprint_start = Instant::now();
    let fingerprint = mods.fingerprint(&loaded);
    let fingerprint_time = fingerprint_start.elapsed();
    *IDENTITY.lock().unwrap_or_else(PoisonError::into_inner) = Some(ProfileIdentity::of(&loaded, fingerprint));
    let (files, warnings) = mods.adapter.take_content();
    if verbose {
        println!(
            "profile {} ({}): {} mods; scan {:.0?}, resolve {:.0?}, mount {:.0?}, records {:.0?}, fingerprint {fingerprint_time:.0?}; total {:.0?}",
            mods.profile.name,
            if mods.profile_from_file { "file" } else { "generated default" },
            loaded.mods.len(),
            mods.repo.scan_time,
            loaded.timings.resolve,
            loaded.timings.mount,
            loaded.timings.records,
            start.elapsed(),
        );
        print_vfs(&loaded.vfs, mods.repo.scan_time + loaded.timings.mount);
        println!("\ncontent: {} files in {:.0?}", files.len(), loaded.timings.records);
        for f in &files {
            println!(
                "  {:<24} {:?} {:>7} objects  read {:>6.0?}  parse {:>6.0?}  apply {:>6.0?}  masters: {}",
                f.name,
                f.file_type,
                f.objects,
                f.read_time,
                f.parse_time,
                f.apply_time,
                if f.masters.is_empty() { "-".to_owned() } else { f.masters.join(", ") }
            );
        }
    }
    let textures = mw_bake::texture_profiles::TextureSources::from_mods(&loaded.mods);
    let vfs = std::mem::replace(&mut loaded.vfs, Arc::new(Vfs::builder().build()));
    let vfs = Arc::try_unwrap(vfs).map_err(|_| "VFS still shared")?;
    let db = std::mem::replace(&mut loaded.records, sf_records::RecordDb::new());
    // Dev builds keep the profile (records and VFS moved out) for record hot reload (M5 A4).
    #[cfg(feature = "dev")]
    crate::hot_reload::keep_profile(mods, loaded);
    Ok((vfs, LoadedContent { db, files, warnings }, fingerprint, textures))
}
