//! Character stats on the player, NPCs and creatures (stats.md S4, stats-adapter.md "Crates"):
//! `sf_stats_bevy` with Morrowind's table and `OpenMW`'s rules (`mw_stats`).
//!
//! - **Setup** ([`add`]): `MwTable::from_db`, `MwGmst::from_db` (missing names logged) and
//!   `openmw_rules` into `StatTableRes` / `StatRules`, plus [`MwStats`] for the glue.
//! - **Player**: the sheet lives on its own entity, [`PlayerCharacter`], not on the body (the
//!   body comes and goes with the free camera; the character stays). Owner
//!   `StatOwner::Player(HOST_KEY)`, built from the `player` record ([`player_start_ops`]: chargen
//!   is skipped). A save restores it ([`restore_player_sheet`]).
//! - **NPCs and creatures**: every new `Character::Record` of an NPC or creature record (not the
//!   player's avatar) gets `Stats` before the next stat tick ([`attach_actor_stats`]), owner
//!   `ActorKey` = its placement id ([`crate::dialogue::key`]). A placement with an overlay
//!   [`STATS_BLOB`] loads it ([`load_sheet`]); else the record's init ops.
//! - **Unload and save** (save.md "One path for unload and save"): [`ActorStatsSpawner`] writes a
//!   changed sheet (`changed_since_init`, stats.md "Saves") to the overlay when its placement
//!   leaves R0, never on a reload's discard; [`persist_actor_stats`] writes the same into the
//!   save's overlay copy. An untouched NPC re-initialises from its record.
//! - **Inputs**: `is_player`; `carried` is 0 until inventories have weight (TODO(stats S5+):
//!   the loadout's weight; containers carry none yet). `combatant` is S5's (combat vitals).

use std::sync::{Arc, PoisonError};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_game_core::player::{HOST_KEY, PlayerAvatar, PlayerCharacter};
use mw_stats::{MwGmst, MwTable, creature_ops, npc_ops, ops_for_missing, player_start_ops};
use sf_character_bevy::{Character, CharacterPlacement, CharacterSpawner};
use sf_records::{RecordDb, RecordId};
use sf_stats_bevy::sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, Inputs, Rules, Settle, Sheet, SheetSave, StatOp, StatOwner,
};
use sf_stats_bevy::{SfStatsPlugin, StatRules, StatSystems, StatTableRes, Stats};
use sf_world::{IndexedPlacement, PlacementOverlay};
use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, WorldOverlay};
use tes3::esp::{Creature, Npc};

/// Owner name of the per-placement stats blob (`PlacementOverlay::set_state`): a postcard
/// [`SheetSave`].
pub const STATS_BLOB: &str = "stats";
/// Its schema version (`sf_save_bevy::sections::BlobVersions`).
pub const STATS_BLOB_VERSION: u32 = 1;
/// The player record (chargen skipped: the dev and release start).
const PLAYER_RECORD: &str = "player";

/// The Morrowind side of the stats: the table with its indices and the GMST snapshot.
#[derive(Resource, Clone)]
pub struct MwStats {
    pub table: Arc<MwTable>,
    pub gmst: MwGmst,
    pub db: Arc<RecordDb>,
}

impl MwStats {
    /// The init ops of an NPC or creature record (`None`: neither, or the NPC can't initialise).
    pub fn actor_ops(&self, id: &RecordId) -> Option<Vec<StatOp>> {
        if let Some(npc) = self.db.get::<Npc>(id) {
            return npc_ops(&self.table, &self.gmst, &self.db, npc).map_err(|e| warn!("stats: {e}")).ok();
        }
        self.db.get::<Creature>(id).map(|c| creature_ops(&self.table, &self.gmst, c))
    }

    /// The player's init ops (chargen skipped); none without a `player` record.
    pub fn player_ops(&self) -> Vec<StatOp> {
        self.db
            .get::<Npc>(&RecordId::new(PLAYER_RECORD))
            .map(|p| player_start_ops(&self.table, &self.gmst, &self.db, p))
            .unwrap_or_default()
    }
}

/// The player's inputs (`carried`: see the module docs).
fn player_inputs() -> Inputs {
    Inputs { is_player: true, ..Inputs::default() }
}

/// Stats for the game: the framework plugin, the Morrowind table and rules, the player's
/// character. `db`: the loaded records (`None`: no stats).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>) {
    let Some(db) = db else { return };
    let table = match MwTable::from_db(&db) {
        Ok(t) => Arc::new(t),
        Err(e) => {
            warn!("stats off: {e}");
            return;
        }
    };
    let (gmst, missing) = MwGmst::from_db(&db);
    if !missing.is_empty() {
        warn!("stats: {} GMSTs missing, vanilla values used: {}", missing.len(), missing.join(", "));
    }
    let rules = Arc::new(mw_stats::openmw_rules(&table, &gmst));
    let stats = MwStats { table: table.clone(), gmst, db };
    let player = new_sheet(&stats, &rules, StatOwner::Player(HOST_KEY), &player_inputs(), &stats.player_ops());
    if !app.is_plugin_added::<SfStatsPlugin>() {
        app.add_plugins(SfStatsPlugin);
    }
    app.insert_resource(StatTableRes(Arc::new(table.table.clone())))
        .insert_resource(StatRules(rules))
        .insert_resource(stats)
        .add_systems(FixedUpdate, attach_actor_stats.before(StatSystems::Apply));
    app.world_mut().spawn((
        Name::new("player character"),
        PlayerCharacter { key: HOST_KEY },
        Stats::new(StatOwner::Player(HOST_KEY), player, player_inputs()),
    ));
}

/// A new sheet from init ops (`mw_stats::new_sheet`), warning on rejected ops.
fn new_sheet(stats: &MwStats, rules: &Rules, owner: StatOwner, inputs: &Inputs, ops: &[StatOp]) -> Sheet {
    let (sheet, rejected) = mw_stats::new_sheet(&stats.table, rules, owner, inputs, ops);
    if rejected > 0 {
        warn!("stats: {rejected} init ops rejected for {owner:?}");
    }
    sheet
}

/// A saved sheet back (stats.md "Saves", load order): keys the save lacks take their init
/// values (`ops_for_missing` of `init`), unknown keys are dropped and reported, sources re-add
/// their modifiers (none yet: equipment and the effect runtime come later), then one settle.
pub fn load_sheet(t: &MwTable, rules: &Rules, owner: StatOwner, save: &SheetSave, init: &[StatOp], inputs: &Inputs) -> Sheet {
    let (mut sheet, report) = Sheet::from_save(&t.table, save);
    if !report.dropped.is_empty() {
        warn!("stats: {owner:?}: {} saved stats unknown to this mod list, dropped", report.dropped.len());
    }
    let missing = ops_for_missing(t, init, &report.missing);
    if !missing.is_empty() {
        let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Load, source: None, tick: 0 }, inputs };
        sf_stats_bevy::sf_stats::apply(&t.table, rules, &mut sheet, &missing, &ctx, &mut Vec::new());
    }
    sheet.settle(&t.table, rules, inputs, Settle::Loaded);
    sheet
}

/// `Stats` for every new NPC or creature character (see the module docs).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
pub fn attach_actor_stats(
    mut commands: Commands,
    stats: Res<MwStats>,
    rules: Res<StatRules>,
    overlay: Option<Res<WorldOverlay>>,
    added: Query<(Entity, &Character, Option<&CharacterPlacement>), (Added<Character>, Without<Stats>, Without<PlayerAvatar>)>,
) {
    for (e, character, placement) in &added {
        let Character::Record { id, .. } = character else { continue };
        let Some(init) = stats.actor_ops(id) else { continue };
        let owner = StatOwner::Actor(crate::dialogue::key(e, placement));
        let inputs = Inputs::default();
        let saved = placement.zip(overlay.as_deref()).and_then(|(p, o)| saved_sheet(&o.0.read().unwrap_or_else(PoisonError::into_inner), p));
        let sheet = match saved {
            Some(save) => load_sheet(&stats.table, &rules.0, owner, &save, &init, &inputs),
            None => new_sheet(&stats, &rules.0, owner, &inputs, &init),
        };
        commands.entity(e).try_insert(Stats::new(owner, sheet, inputs));
    }
}

/// The placement's saved sheet, if its blob decodes.
fn saved_sheet(overlay: &PlacementOverlay, placement: &CharacterPlacement) -> Option<SheetSave> {
    let bytes = overlay.state(placement.0, STATS_BLOB)?;
    postcard::from_bytes(bytes).map_err(|e| warn!("stats: placement {:032x}: bad {STATS_BLOB} blob: {e}", placement.0.0)).ok()
}

/// Writes a changed actor sheet into `overlay` as its placement's [`STATS_BLOB`].
fn write_sheet(overlay: &mut PlacementOverlay, table: &StatTableRes, placement: &CharacterPlacement, stats: &Stats) {
    if !stats.sheet.changed_since_init() {
        return;
    }
    match postcard::to_stdvec(&stats.sheet.to_save(&table.0)) {
        Ok(bytes) => overlay.set_state(placement.0, STATS_BLOB, bytes),
        Err(e) => warn!("stats: {:?} not saved: {e}", stats.owner),
    }
}

/// Saving (save.md "One path for unload and save"): every resident actor's changed sheet into
/// `overlay` (the snapshot's copy), as its deactivation would, without removing anything.
pub fn persist_actor_stats(world: &World, overlay: &mut PlacementOverlay) {
    let Some(table) = world.get_resource::<StatTableRes>() else { return };
    let Some(mut q) = world.try_query::<(&CharacterPlacement, &Stats)>() else { return };
    for (placement, stats) in q.iter(world) {
        write_sheet(overlay, table, placement, stats);
    }
}

/// The player's sheet for its `CharacterSave` (`None` without stats).
pub fn player_sheet_save(world: &World, key: sf_rpg::PlayerKey) -> Option<SheetSave> {
    let table = world.get_resource::<StatTableRes>()?;
    let mut q = world.try_query::<(&PlayerCharacter, &Stats)>()?;
    q.iter(world).find(|(p, _)| p.key == key).map(|(_, s)| s.sheet.to_save(&table.0))
}

/// The player's sheet from a loaded `CharacterSave`: the saved one, or (a save from before
/// stats, `None`) a new one from the `player` record, the new-game path (stats.md "Saves").
pub fn restore_player_sheet(world: &mut World, key: sf_rpg::PlayerKey, save: Option<&SheetSave>) {
    let (Some(stats), Some(rules)) = (world.get_resource::<MwStats>().cloned(), world.get_resource::<StatRules>().cloned()) else { return };
    let owner = StatOwner::Player(key);
    let init = stats.player_ops();
    let inputs = player_inputs();
    let sheet = match save {
        Some(save) => load_sheet(&stats.table, &rules.0, owner, save, &init, &inputs),
        None => new_sheet(&stats, &rules.0, owner, &inputs, &init),
    };
    let found = world.try_query::<(Entity, &PlayerCharacter)>().and_then(|mut q| q.iter(world).find(|(_, p)| p.key == key).map(|(e, _)| e));
    let character = Stats::new(owner, sheet, inputs);
    match found {
        Some(e) => {
            world.entity_mut(e).insert(character);
        }
        None => {
            world.spawn((Name::new("player character"), PlayerCharacter { key }, character));
        }
    }
}

/// The actor `PlacementSpawner`: [`CharacterSpawner`] plus the stats write-back when the
/// placement leaves R0 (register it for `PlacementClass::Actor`).
pub struct ActorStatsSpawner;

impl PlacementSpawner for ActorStatsSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        CharacterSpawner.spawn(world, cx, placement)
    }

    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        CharacterSpawner.needs_model(placement)
    }

    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        if cx.discard {
            return;
        }
        let owner = StatOwner::Actor(ActorKey(placement.placement.id.0));
        let Some(e) = world.get_resource::<sf_stats_bevy::StatIndex>().and_then(|i| i.get(owner)) else { return };
        let (Some(table), Some(overlay), Some(stats)) = (world.get_resource::<StatTableRes>(), world.get_resource::<WorldOverlay>(), world.get::<Stats>(e))
        else {
            return;
        };
        write_sheet(&mut overlay.0.write().unwrap_or_else(PoisonError::into_inner), table, &CharacterPlacement(placement.placement.id), stats);
    }
}
