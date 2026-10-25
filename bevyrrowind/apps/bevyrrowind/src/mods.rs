//! `bevyrrowind mods ...`: inspects and edits profiles. The profile loading every command goes
//! through (`--profile`, `--user-data`) lives in `mw_boot_bevy::mods`; re-exported here so
//! `crate::mods::..` paths resolve (app-split.md slice 2).

use clap::Subcommand;
use mw_install::MorrowindInstall;
use sf_mods::ModId;

pub use mw_boot_bevy::mods::{Selection, load, open, script_plans, select, user_data};

use crate::Result;

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
