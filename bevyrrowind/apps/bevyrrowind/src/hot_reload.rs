//! Hot reload in dev builds (ROADMAP M5 A4; superfargoth `docs/specs/mods.md` "Hot reload").
//!
//! With the `dev` feature the default asset source serves a swappable VFS snapshot
//! (`sf_vfs::SharedVfs`), and one `sf_vfs` watcher covers every directory layer (loose
//! `Data Files` and folder mods). On a change it rescans the touched layers, swaps the
//! snapshot and sends the *winning* changes to Bevy's asset server (loaded textures, sounds
//! and other by-path assets reload) and to [`apply`], which routes the rest
//! (`LoadedProfile::route`): script files reload their contexts with state kept
//! (`Scripts::reload_changed`), record files re-run the record step into [`LiveRecords`]
//! (with a [`RecordsChanged`] message; placed objects keep their data until their cell
//! reloads), and bake inputs are reported (dev re-bake during play: not wired yet, ROADMAP
//! A4 handoff). Release builds use a fixed snapshot and none of this exists.

use std::sync::Arc;

use bevy::asset::io::AssetSourceBuilder;
use bevy::prelude::*;
use sf_vfs::Vfs;

/// The default asset source over `vfs`: watched and swappable with the `dev` feature.
pub fn asset_source(app: &mut App, vfs: Arc<Vfs>) -> AssetSourceBuilder {
    #[cfg(feature = "dev")]
    {
        dev::install(app, vfs)
    }
    #[cfg(not(feature = "dev"))]
    {
        let _ = app;
        sf_vfs::bevy::asset_source(vfs)
    }
}

/// The record DB after the last record hot reload (dev builds); consumers that opt in read it
/// on [`RecordsChanged`].
#[allow(dead_code, reason = "read by record consumers that opt in (none yet); built only in dev")]
#[derive(Resource, Clone)]
pub struct LiveRecords(pub Arc<sf_records::RecordDb>);

/// Record files changed and [`LiveRecords`] holds the rebuilt DB.
#[allow(dead_code, reason = "read by record consumers that opt in (none yet); built only in dev")]
#[derive(Message, Clone, Debug)]
pub struct RecordsChanged {
    /// The changed record files (VFS paths).
    pub files: Vec<String>,
}

/// Keeps the loaded profile (records moved out) for record hot reload (dev builds).
#[cfg(feature = "dev")]
pub fn keep_profile(mods: mw_mods::Mods, loaded: sf_mods::LoadedProfile) {
    let _ = dev::PROFILE.set(std::sync::Mutex::new((mods, loaded)));
}

#[cfg(feature = "dev")]
mod dev {
    use std::sync::mpsc::{Receiver, channel};
    use std::sync::{Mutex, OnceLock, PoisonError};

    use bevy::asset::UntypedAssetId;
    use sf_script::ScriptSource;
    use sf_script_bevy::Scripts;
    use sf_vfs::SharedVfs;
    use sf_vfs::bevy::{AssetEvents, shared_asset_source};
    use sf_vfs::watch::{DEFAULT_DEBOUNCE, VfsUpdate, VfsWatcher, WatchEvent};

    use super::{App, Arc, AssetSourceBuilder, LiveRecords, RecordsChanged, Vfs};
    use bevy::prelude::*;

    /// The loaded profile kept by `mods::load_content` for record reloads.
    pub static PROFILE: OnceLock<Mutex<(mw_mods::Mods, sf_mods::LoadedProfile)>> = OnceLock::new();

    /// Bake inputs: a change re-bakes products (cooked models), which isn't wired during play yet.
    const BAKE_INPUTS: [&str; 2] = ["nif", "kf"];

    #[derive(Resource)]
    struct HotReload {
        updates: Mutex<Receiver<VfsUpdate>>,
        _watcher: Mutex<VfsWatcher>,
    }

    pub fn install(app: &mut App, vfs: Arc<Vfs>) -> AssetSourceBuilder {
        let shared = SharedVfs::new(vfs);
        let events = AssetEvents::default();
        let (tx, rx) = channel();
        let to_bevy = events.clone();
        let watcher = VfsWatcher::start(shared.clone(), DEFAULT_DEBOUNCE, move |event| match event {
            WatchEvent::Updated(update) => {
                to_bevy.send(&update.changes);
                let _ = tx.send(update);
            }
            WatchEvent::Failed(e) => warn!("hot reload: {e}"),
        });
        match watcher {
            Ok(watcher) => {
                // Before the log plugin exists (the source is registered ahead of DefaultPlugins).
                println!("hot reload: watching {} directories", watcher.roots().len());
                app.insert_resource(HotReload { updates: Mutex::new(rx), _watcher: Mutex::new(watcher) })
                    .add_message::<RecordsChanged>()
                    .add_systems(Update, (apply, log_reloaded_images));
            }
            Err(e) => eprintln!("warning: hot reload off: {e}"),
        }
        shared_asset_source(shared, events)
    }

    /// Routes each published snapshot's changes (see the module docs).
    fn apply(world: &mut World) {
        let updates: Vec<VfsUpdate> = {
            let hot = world.resource::<HotReload>();
            hot.updates.lock().unwrap_or_else(PoisonError::into_inner).try_iter().collect()
        };
        for update in updates {
            let list: Vec<String> = update.changes.iter().map(|c| format!("{} ({:?})", c.path, c.kind)).collect();
            info!("hot reload: {} winning changes, rebuilt in {:.1?}: {}", list.len(), update.rebuild, list.join(", "));
            let profile = PROFILE.get().map(|p| p.lock().unwrap_or_else(PoisonError::into_inner));
            let plan = match &profile {
                Some(guard) => guard.1.route(&update.changes),
                None => sf_mods::HotReloadPlan { assets: update.changes.clone(), ..default() },
            };
            drop(profile);

            if !plan.scripts.is_empty()
                && let Some(mut scripts) = world.get_resource_mut::<Scripts>()
            {
                let source: Arc<dyn ScriptSource> = update.vfs.clone();
                let reloaded = scripts.reload_changed(plan.scripts.iter().map(sf_vfs::VfsPath::as_str), &source);
                for (id, result) in reloaded {
                    match result {
                        Ok(()) => info!("hot reload: script context {id} reloaded (state kept)"),
                        Err(e) => warn!("hot reload: script context {id} kept its old code: {e}"),
                    }
                }
            }
            if !plan.records.is_empty() {
                reload_records(world, &update, &plan.records);
            }
            for (owner, path) in &plan.styles {
                info!("hot reload: stylesheet {path} of {owner} changed (UI composer: M5 U2)");
            }
            for change in &plan.assets {
                if change.path.extension().is_some_and(|e| BAKE_INPUTS.contains(&e)) {
                    warn!("hot reload: bake input {} changed: run `bevyrrowind bake` (dev re-bake during play not wired)", change.path);
                }
            }
        }
    }

    fn reload_records(world: &mut World, update: &VfsUpdate, files: &[(sf_mods::ModId, sf_vfs::VfsPath)]) {
        let Some(profile) = PROFILE.get() else { return };
        let mut guard = profile.lock().unwrap_or_else(PoisonError::into_inner);
        let (mods, loaded) = &mut *guard;
        let time = loaded.reload_records(update.vfs.clone(), &mods.adapters());
        let _ = mods.adapter.take_content();
        let errors: Vec<String> = loaded
            .report
            .issues
            .iter()
            .filter(|i| i.kind.area() == sf_mods::Area::Records && i.severity() == sf_mods::Severity::Error)
            .map(|i| format!("{}: {}", i.mod_id.as_ref().map(ToString::to_string).unwrap_or_default(), i.kind))
            .collect();
        let db = Arc::new(std::mem::replace(&mut loaded.records, sf_records::RecordDb::new()));
        drop(guard);
        let names: Vec<String> = files.iter().map(|(owner, path)| format!("{path} ({owner})")).collect();
        info!(
            "hot reload: records rebuilt in {time:.0?} for {}; {} live; placed objects keep their data until their cell reloads",
            names.join(", "),
            db.conflict_report().total_live()
        );
        for e in errors {
            warn!("hot reload: record error {e}");
        }
        world.insert_resource(LiveRecords(db));
        world.write_message(RecordsChanged { files: files.iter().map(|(_, p)| p.as_str().to_owned()).collect() });
    }

    /// Logs images the asset server reloaded (the visible half of a texture hot reload).
    fn log_reloaded_images(mut events: MessageReader<AssetEvent<Image>>, assets: Res<AssetServer>) {
        for event in events.read() {
            if let AssetEvent::Modified { id } = event {
                let id: UntypedAssetId = (*id).into();
                if let Some(path) = assets.get_path(id) {
                    info!("hot reload: texture reloaded: {path}");
                }
            }
        }
    }
}
