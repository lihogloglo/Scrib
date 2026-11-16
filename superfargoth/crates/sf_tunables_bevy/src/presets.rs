//! Presets as data in a Bevy app (spec "Presets are bundles of tunables", slice T5b; the core is
//! [`sf_tunables::presets`]).
//!
//! - [`PresetsPlugin`] registers the drivers (`sf.graphics_preset`, `sg.*`) and runs
//!   [`drive_presets`] between [`TunableSystems::Apply`] and [`TunableSystems::Bind`]: a driver set
//!   above the `Preset` layer (console `set sg.shadows low`, BRP `sf.tunables/set`, `--set`, a key) is
//!   a level request: its level is written at the `Preset` layer, the request's layer is cleared, and
//!   every driver then shows the level its members match (or `custom`). Bindings run after it, so a
//!   switch reaches the bound resources the same frame. Each request goes out as [`PresetApplied`]
//!   (also logged: what applies on restart, to new spawns, what stays above the preset).
//! - The host inserts [`Presets`] (a [`PresetBook`] built from its records: [`preset_records`],
//!   [`mod_values`] read them from a [`RecordDb`]); without it the drivers are plain tunables.

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use sf_records::RecordDb;
use sf_tunables::presets::{PresetBook, PresetRecord, TunableRecord};

use crate::{SetBy, TunableSystems, Tunables, TunablesAppExt};

/// The checked presets the drivers apply.
#[derive(Resource, Clone, Debug, Default)]
pub struct Presets(pub PresetBook);

/// A level request that ran: the driver, the level, the layer it came on, and its summary
/// ([`sf_tunables::presets::Applied::summary`]) or why it was refused.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct PresetApplied {
    pub driver: String,
    pub level: String,
    pub by: SetBy,
    pub summary: Result<String, String>,
}

/// Registers the drivers and runs [`drive_presets`].
pub struct PresetsPlugin;

impl Plugin for PresetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_tunables(sf_tunables::presets::TUNABLES)
            .add_message::<PresetApplied>()
            .add_systems(
                PreUpdate,
                drive_presets
                    .after(TunableSystems::Apply)
                    .before(TunableSystems::Bind),
            );
    }
}

/// Applies level requests and keeps the drivers showing their derived level; runs only when a
/// value changed since its last run (or the book did).
pub fn drive_presets(
    presets: Option<Res<Presets>>,
    mut tunables: ResMut<Tunables>,
    mut seen: Local<Option<u64>>,
    mut applied: MessageWriter<PresetApplied>,
) {
    let Some(presets) = presets else { return };
    if *seen == Some(tunables.revision()) && !presets.is_changed() {
        return;
    }
    let book = &presets.0;
    let requests = book.requests(&tunables);
    if !requests.is_empty() {
        let done = book.take_requests(&mut tunables);
        for ((driver, level, result), (_, _, by)) in done.into_iter().zip(requests) {
            let summary = result.map(|a| a.summary(&tunables));
            match &summary {
                Ok(s) => info!("preset {driver} {level}: {s}"),
                Err(e) => warn!("preset {driver} {level}: {e}"),
            }
            applied.write(PresetApplied { driver, level, by, summary });
        }
    } else if book.out_of_sync(&tunables) {
        book.sync(&mut tunables);
    }
    *seen = Some(tunables.revision());
}

/// The source that last wrote record `id` of kind `T`.
fn writer<T: Send + Sync + 'static>(db: &RecordDb, id: &sf_records::RecordId) -> (u32, String) {
    let source = db.entry::<T>(id).and_then(|e| e.history.last()).map(|t| t.source);
    source.map_or((u32::MAX, String::new()), |s| (s.0, db.source(s).name.clone()))
}

/// `sf.tunable` records (`db` must have the kind registered), in load order of their last
/// writer: `(tunable name, record, mod)` for [`sf_tunables::presets::apply_mod_values`].
pub fn mod_values(db: &RecordDb) -> Vec<(String, TunableRecord, String)> {
    let mut out: Vec<(u32, String, TunableRecord, String)> = db
        .iter::<TunableRecord>()
        .map(|(id, r)| {
            let (order, owner) = writer::<TunableRecord>(db, id);
            (order, id.as_str().to_owned(), r.clone(), owner)
        })
        .collect();
    out.sort_by_key(|(order, ..)| *order);
    out.into_iter().map(|(_, name, r, owner)| (name, r, owner)).collect()
}

/// `sf.tunable_preset` records, in load order of their last writer: `(record, mod)` for
/// [`sf_tunables::presets::MergedPresets::merge`].
pub fn preset_records(db: &RecordDb) -> Vec<(PresetRecord, String)> {
    let mut out: Vec<(u32, PresetRecord, String)> = db
        .iter::<PresetRecord>()
        .map(|(id, r)| {
            let (order, owner) = writer::<PresetRecord>(db, id);
            (order, r.clone(), owner)
        })
        .collect();
    out.sort_by_key(|(order, ..)| *order);
    out.into_iter().map(|(_, r, owner)| (r, owner)).collect()
}

/// Registers the two record kinds (`sf.tunable`, `sf.tunable_preset`) in `db`.
pub fn register_kinds(db: &mut RecordDb) -> Result<(), sf_records::RecordError> {
    db.register_serde::<TunableRecord>(sf_tunables::presets::TUNABLE_KIND)?;
    db.register_serde::<PresetRecord>(sf_tunables::presets::PRESET_KIND)
}
