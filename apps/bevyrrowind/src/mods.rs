//! The mod pipeline in the app (`superfargoth/docs/specs/mods.md`): every
//! command loads content through a profile (`--profile`, default `default`)
//! over `<user data>/mods` (`--user-data`), and `bevyrrowind mods ...`
//! inspects and edits profiles.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use clap::Subcommand;
use mw_install::MorrowindInstall;
use mw_mods::{Mods, MwAdapter, UserDirs};
use mw_records::LoadedContent;
use sf_mods::{LoadedProfile, ModId, Severity};
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

/// The loaded profile's script contexts (`LoadedProfile::script_contexts`), set by [`load`].
static SCRIPT_PLANS: OnceLock<Vec<sf_mods::ScriptPlan>> = OnceLock::new();

/// Script contexts of the profile the last [`load`] resolved (empty before one).
pub fn script_plans() -> Vec<sf_mods::ScriptPlan> {
    SCRIPT_PLANS.get().cloned().unwrap_or_default()
}

pub fn select(selection: Selection) {
    let _ = SELECTION.set(selection);
}

fn selection() -> Selection {
    SELECTION.get().cloned().unwrap_or_else(|| Selection { user_data: None, profile: mw_mods::DEFAULT_PROFILE.into() })
}

#[derive(Subcommand)]
pub enum ModsCommand {
    /// Installed mods (folder mods and the install's synthesized ones), enabled ones marked.
    List,
    /// The resolved load order with each mod's VFS layers.
    Order,
    /// Every issue per mod, the file override view and record conflicts.
    Report {
        /// Only this mod's issues.
        #[arg(long = "mod")]
        only: Option<String>,
        /// Record conflicts listed per kind.
        #[arg(long, default_value_t = 5)]
        conflicts: usize,
    },
    /// Enable a mod in the profile (appended last, or after `--after`), writing the profile file.
    Enable {
        id: String,
        #[arg(long)]
        after: Option<String>,
    },
    /// Disable a mod in the profile, writing the profile file.
    Disable { id: String },
}

/// The builtin mod's layers: the shipped animation graphs and the `mw` Luau
/// runtime (scripting-adapter.md "The `mw` runtime library").
fn builtin_layers() -> Vec<Arc<dyn VfsSource>> {
    let table = mw_script::functions::FunctionTable::vanilla();
    let runtime = MemSource::new("builtin:mw_script")
        .with(mw_mods::RUNTIME_PATH, mw_script::emit::RUNTIME)
        .with(mw_mods::RUNTIME_FUNCTIONS_PATH, mw_script::emit::runtime_functions(table));
    vec![Arc::new(mw_graphs::builtin_source()), Arc::new(runtime)]
}

/// Scans mods and the install and selects the profile. `records: false`
/// mounts only (content files aren't parsed).
pub fn open(install: &MorrowindInstall, records: bool) -> Result<Mods> {
    let selection = selection();
    let user = UserDirs::detect(selection.user_data.as_deref()).ok_or("no user data folder; pass --user-data")?;
    let mut adapter = MwAdapter::new(install.clone(), builtin_layers());
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

/// VFS and records of the selected profile, and its fingerprint.
pub fn load_content(install: &MorrowindInstall, verbose: bool) -> Result<(Vfs, LoadedContent, [u8; 32])> {
    let start = Instant::now();
    let mods = open(install, true)?;
    let loaded = load(&mods);
    let fingerprint_start = Instant::now();
    let fingerprint = mods.fingerprint(&loaded);
    let fingerprint_time = fingerprint_start.elapsed();
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
    let vfs = Arc::try_unwrap(loaded.vfs).map_err(|_| "VFS still shared")?;
    Ok((vfs, LoadedContent { db: loaded.records, files, warnings }, fingerprint))
}

fn id(raw: &str) -> Result<ModId> {
    Ok(ModId::new(raw.to_ascii_lowercase())?)
}

/// Whether the last bake ran with this profile fingerprint.
fn bake_status(install: &MorrowindInstall, fingerprint: &[u8; 32]) -> String {
    let Ok(root) = crate::bake::cache_root(None) else { return "no bake cache".into() };
    let Ok(store) = sf_bake::Store::open(&root) else { return "no bake cache".into() };
    match store.profile_fingerprint(&mw_bake::profile_name(install)) {
        Some(stored) if stored.0 == *fingerprint => "up to date".into(),
        Some(_) => "stale (run `bevyrrowind bake`)".into(),
        None => "no fingerprint stored (run `bevyrrowind bake`)".into(),
    }
}

pub fn run(install: &MorrowindInstall, command: ModsCommand) -> Result {
    match command {
        ModsCommand::List => {
            let mods = open(install, false)?;
            println!("profile {} (mods folder {})", mods.profile.name, mods.user.mods().display());
            for m in mods.repo.manifests() {
                let mark = if mods.profile.enabled.contains(&m.id) { "x" } else { " " };
                let invalid = if mods.repo.is_valid(&m.id) { "" } else { "  INVALID" };
                let origin = m.adapter.as_deref().map_or_else(|| "folder".to_owned(), |a| format!("adapter:{a}"));
                println!("[{mark}] {:<28} {:<10} {:<18} {}{invalid}", m.id.as_str(), m.version.to_string(), origin, m.name);
            }
            Ok(())
        }
        ModsCommand::Order => {
            let mods = open(install, false)?;
            let loaded = load(&mods);
            for (i, m) in loaded.mods.iter().enumerate() {
                let layers: Vec<&str> = m.layers.clone().map(|s| loaded.vfs.source(sf_vfs::SourceIndex(s)).label()).collect();
                println!("{i:>3}  {:<28} {}", m.id().as_str(), layers.join(", "));
            }
            Ok(())
        }
        ModsCommand::Report { only, conflicts } => {
            let mods = open(install, true)?;
            let loaded = load(&mods);
            let only = only.as_deref().map(id).transpose()?;
            print!("{}", loaded.render_report(only.as_ref(), conflicts));
            let fingerprint = mods.fingerprint(&loaded);
            let hex = sf_bake::Digest(fingerprint).hex();
            println!("\nprofile fingerprint {}…: bake {}", &hex[..12], bake_status(install, &fingerprint));
            Ok(())
        }
        ModsCommand::Enable { id: raw, after } => {
            let mut mods = open(install, false)?;
            let new = id(&raw)?;
            if mods.repo.get(&new).is_none() {
                return Err(format!("no installed mod {new} (see `bevyrrowind mods list`)").into());
            }
            let profile = &mut mods.profile;
            profile.enabled.retain(|m| *m != new);
            let at = match after.as_deref().map(id).transpose()? {
                Some(after) => profile.enabled.iter().position(|m| *m == after).ok_or(format!("{after} is not enabled"))? + 1,
                None => profile.enabled.len(),
            };
            profile.enabled.insert(at, new);
            let path = profile.save(&mods.user.profiles())?;
            println!("enabled {raw}; wrote {}", path.display());
            Ok(())
        }
        ModsCommand::Disable { id: raw } => {
            let mut mods = open(install, false)?;
            let old = id(&raw)?;
            if old == mw_mods::builtin_id() {
                return Err("the builtin mod is always enabled".into());
            }
            let before = mods.profile.enabled.len();
            mods.profile.enabled.retain(|m| *m != old);
            if mods.profile.enabled.len() == before {
                return Err(format!("{old} is not enabled").into());
            }
            let path = mods.profile.save(&mods.user.profiles())?;
            println!("disabled {raw}; wrote {}", path.display());
            Ok(())
        }
    }
}
