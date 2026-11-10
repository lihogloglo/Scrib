//! Character stats on the player, NPCs and creatures (stats.md S4, stats-adapter.md "Crates"):
//! `sf_stats_bevy` with Morrowind's table and `OpenMW`'s rules (`mw_stats`).
//!
//! - **Setup** ([`add`]): `MwTable::from_db`, `MwGmst::from_db` (missing names logged) and
//!   `openmw_rules` into `StatTableRes` / `StatRules`, plus [`MwStats`] for the glue.
//! - **Player**: the sheet lives on its own entity, [`PlayerCharacter`], not on the body (the
//!   body comes and goes with the free camera; the character stays). Owner
//!   `StatOwner::Player(HOST_KEY)`, built from the `player` record ([`player_start_ops`]: chargen
//!   is skipped) with its start spells learned (the record's race's powers, then the
//!   [`PlayerBirthsign`]'s spells: [`MwStats::player_spells`]). A save restores it
//!   ([`restore_player_sheet`]).
//! - **NPCs and creatures**: every new `Character::Record` of an NPC or creature record (not the
//!   player's avatar) gets `Stats` and `Effects` (same owner, [`crate::effects`]) before the next
//!   effect and stat tick ([`attach_actor_stats`]), owner
//!   `ActorKey` = its placement id ([`crate::dialogue::key`]). A placement with an overlay
//!   [`STATS_BLOB`] loads it ([`load_sheet`]); else the record's init ops. Saved effects come
//!   back as saved; with none saved the character learns its record's start spells
//!   ([`MwStats::actor_spells`]), never both (a save holds the abilities' instances).
//! - **Unload and save** (save.md "One path for unload and save"): [`ActorStatsSpawner`] writes a
//!   changed sheet (`changed_since_init`, stats.md "Saves") and its effects
//!   ([`crate::effects::EFFECTS_BLOB`]) to the overlay when its placement
//!   leaves R0, never on a reload's discard; [`persist_actor_stats`] writes the same into the
//!   save's overlay copy. An untouched NPC re-initialises from its record.
//! - **Inputs**: `is_player`; `carried` is 0 until inventories have weight (TODO(stats S5+):
//!   the loadout's weight; containers carry none yet). `combatant` is kept by the combat vitals glue
//!   (`mw_player_bevy::vitals`, S5): set while a `Combatant` links the sheet.

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
use sf_effects_bevy::sf_effects::EffectsSave;
use sf_effects_bevy::{EffectSystems, Effects};
use sf_stats_bevy::{SfStatsPlugin, StatRules, StatSystems, StatTableRes, StatTick, Stats};
use sf_world::{IndexedPlacement, PlacementOverlay};
use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, WorldOverlay};
use tes3::esp::{Birthsign, Creature, Npc, Race, Spell};

/// Owner name of the per-placement stats blob (`PlacementOverlay::set_state`): a postcard
/// [`SheetSave`].
pub const STATS_BLOB: &str = "stats";
/// Its schema version (`sf_save_bevy::sections::BlobVersions`).
pub const STATS_BLOB_VERSION: u32 = 1;
/// The player record (chargen skipped: the dev and release start).
const PLAYER_RECORD: &str = "player";

/// The new player's birthsign (BSGN id), read when the player character is built (a new game,
/// or a save without effects): chargen's choice, none until chargen exists (the dev start
/// skips it). Not saved: the book holds what it gave.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlayerBirthsign(pub Option<RecordId>);

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

    /// The start spells of an NPC (its race's powers, then its record's spells) or creature (its
    /// record's spells); `mw_effects::start_spells`, unknown ids warned. Autocalc spells: A3b.
    pub fn actor_spells(&self, id: &RecordId) -> Vec<RecordId> {
        let lists: Vec<&[String]> = if let Some(npc) = self.db.get::<Npc>(id) {
            let race = self.db.get::<Race>(&RecordId::new(&npc.race));
            vec![race.map_or(&[][..], |r| &r.spells), &npc.spells]
        } else if let Some(c) = self.db.get::<Creature>(id) {
            vec![&c.spells]
        } else {
            return Vec::new();
        };
        self.start_spells(id, lists)
    }

    /// The new player's start spells (`buildPlayer`): the `player` record's race's powers, then
    /// `birthsign`'s spells; never the record's own list. `F_PCStart` autocalc spells: A3b.
    pub fn player_spells(&self, birthsign: Option<&RecordId>) -> Vec<RecordId> {
        let player = RecordId::new(PLAYER_RECORD);
        let race = self.db.get::<Npc>(&player).and_then(|p| self.db.get::<Race>(&RecordId::new(&p.race)));
        let sign = birthsign.and_then(|b| {
            let sign = self.db.get::<Birthsign>(b);
            if sign.is_none() {
                warn!("effects: birthsign {b:?} has no BSGN record");
            }
            sign
        });
        let lists = [race.map_or(&[][..], |r| &r.spells), sign.map_or(&[][..], |s| &s.spells)];
        self.start_spells(&player, lists)
    }

    fn start_spells<'a>(&self, who: &RecordId, lists: impl IntoIterator<Item = &'a [String]>) -> Vec<RecordId> {
        let s = mw_effects::start_spells(lists, |id| self.db.get::<Spell>(id).is_some());
        if !s.missing.is_empty() {
            warn!("effects: {who:?}: start spells without a SPEL record ignored: {:?}", s.missing);
        }
        s.known
    }

    /// The player's init ops (chargen skipped); none without a `player` record.
    pub fn player_ops(&self) -> Vec<StatOp> {
        self.db
            .get::<Npc>(&RecordId::new(PLAYER_RECORD))
            .map(|p| player_start_ops(&self.table, &self.gmst, &self.db, p))
            .unwrap_or_default()
    }
}

/// The new-game player's sheet (chargen skipped) with its table, as the host player's
/// [`mw_rpg::MwSheets`] (no actor sheets): dialogue runs over records without an app (tests,
/// tools). `None`: the records have no stat table.
pub struct NewGameSheets {
    table: Arc<MwTable>,
    player: Sheet,
}

impl NewGameSheets {
    pub fn from_db(db: Arc<RecordDb>) -> Option<Self> {
        let table = Arc::new(MwTable::from_db(&db).ok()?);
        let (gmst, _) = MwGmst::from_db(&db);
        let rules = mw_stats::openmw_rules(&table, &gmst);
        let stats = MwStats { table, gmst, db };
        let player = new_sheet(&stats, &rules, StatOwner::Player(HOST_KEY), &player_inputs(), &stats.player_ops());
        Some(Self { table: stats.table, player })
    }
}

impl mw_rpg::MwSheets for NewGameSheets {
    fn table(&self) -> Option<&MwTable> {
        Some(&self.table)
    }

    fn player(&self, player: sf_rpg::PlayerId) -> Option<&Sheet> {
        (player == sf_rpg::PlayerId::HOST).then_some(&self.player)
    }

    fn actor(&self, _actor: ActorKey) -> Option<&Sheet> {
        None
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
    let fx_db = stats.db.clone();
    if !app.is_plugin_added::<SfStatsPlugin>() {
        app.add_plugins(SfStatsPlugin);
    }
    app.insert_resource(StatTableRes(Arc::new(table.table.clone())))
        .insert_resource(StatRules(rules.clone()))
        .insert_resource(stats.clone())
        .add_systems(FixedUpdate, attach_actor_stats.before(EffectSystems::Commands).before(StatSystems::Apply));
    crate::effects::add(app, &fx_db);
    let owner = StatOwner::Player(HOST_KEY);
    let character = new_player(app.world_mut(), &stats, &rules, owner);
    app.world_mut().spawn((Name::new("player character"), PlayerCharacter { key: HOST_KEY }, character));
}

/// A new player character's sheet and effects: init ops, then the start spells' modifiers, then
/// one settle with full pools.
fn new_player(world: &mut World, stats: &MwStats, rules: &Rules, owner: StatOwner) -> (Stats, Effects) {
    let birthsign = world.get_resource::<PlayerBirthsign>().and_then(|b| b.0.clone());
    let (fx, modifiers) = crate::effects::learn_in_world(world, owner, &stats.player_spells(birthsign.as_ref()));
    let sheet = fresh_sheet(stats, rules, owner, &player_inputs(), &stats.player_ops(), &modifiers);
    (Stats::new(owner, sheet, player_inputs()), fx)
}

/// A new sheet from init ops (`mw_stats::new_sheet`), warning on rejected ops.
fn new_sheet(stats: &MwStats, rules: &Rules, owner: StatOwner, inputs: &Inputs, ops: &[StatOp]) -> Sheet {
    let (sheet, rejected) = mw_stats::new_sheet(&stats.table, rules, owner, inputs, ops);
    if rejected > 0 {
        warn!("stats: {rejected} init ops rejected for {owner:?}");
    }
    sheet
}

/// A new sheet from init ops with `modifiers` (an effect source's, [`crate::effects`]) applied
/// before it settles with full pools, warning on rejected ops.
fn fresh_sheet(stats: &MwStats, rules: &Rules, owner: StatOwner, inputs: &Inputs, ops: &[StatOp], modifiers: &[(StatOwner, StatOp, Cause)]) -> Sheet {
    if modifiers.is_empty() {
        return new_sheet(stats, rules, owner, inputs, ops);
    }
    let t = &stats.table;
    let mut sheet = Sheet::new(&t.table);
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs };
    let rejected = sf_stats_bevy::sf_stats::apply(&t.table, rules, &mut sheet, ops, &ctx, &mut Vec::new());
    if rejected > 0 {
        warn!("stats: {rejected} init ops rejected for {owner:?}");
    }
    apply_modifiers(t, rules, &mut sheet, owner, inputs, modifiers);
    sheet.settle(&t.table, rules, inputs, Settle::NewCharacter);
    sheet
}

/// `modifiers` of `owner` on a building sheet, each with its own cause; others' are skipped.
fn apply_modifiers(t: &MwTable, rules: &Rules, sheet: &mut Sheet, owner: StatOwner, inputs: &Inputs, modifiers: &[(StatOwner, StatOp, Cause)]) {
    for (_, op, cause) in modifiers.iter().filter(|(o, _, _)| *o == owner) {
        let ctx = ApplyCtx { owner, cause: *cause, inputs };
        if sf_stats_bevy::sf_stats::apply(&t.table, rules, sheet, std::slice::from_ref(op), &ctx, &mut Vec::new()) > 0 {
            warn!("stats: {owner:?}: an effect modifier was rejected: {op:?}");
        }
    }
}

/// A saved sheet back (stats.md "Saves", load order): keys the save lacks take their init
/// values (`ops_for_missing` of `init`), unknown keys are dropped and reported, sources re-add
/// their modifiers (`modifiers`: the effect runtime's, [`crate::effects::restore`]; ops for other
/// owners are skipped; equipment comes later), then one settle.
pub fn load_sheet(
    t: &MwTable,
    rules: &Rules,
    owner: StatOwner,
    save: &SheetSave,
    init: &[StatOp],
    inputs: &Inputs,
    modifiers: &[(StatOwner, StatOp, Cause)],
) -> Sheet {
    let (mut sheet, report) = Sheet::from_save(&t.table, save);
    if !report.dropped.is_empty() {
        warn!("stats: {owner:?}: {} saved stats unknown to this mod list, dropped", report.dropped.len());
    }
    let missing = ops_for_missing(t, init, &report.missing);
    if !missing.is_empty() {
        let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Load, source: None, tick: 0 }, inputs };
        sf_stats_bevy::sf_stats::apply(&t.table, rules, &mut sheet, &missing, &ctx, &mut Vec::new());
    }
    apply_modifiers(t, rules, &mut sheet, owner, inputs, modifiers);
    sheet.settle(&t.table, rules, inputs, Settle::Loaded);
    sheet
}

/// `Stats` and `Effects` for every new NPC or creature character (see the module docs). A
/// placement with saved stats loads them, an untouched one builds its sheet from the record;
/// either way saved effects re-register their modifiers before the sheet settles, and without
/// saved effects (none yet, or a save from before abilities) the record's start spells are
/// learned.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
pub fn attach_actor_stats(
    mut commands: Commands,
    stats: Res<MwStats>,
    rules: Res<StatRules>,
    overlay: Option<Res<WorldOverlay>>,
    mut fx: crate::effects::EffectLoad,
    added: Query<(Entity, &Character, Option<&CharacterPlacement>), (Added<Character>, Without<Stats>, Without<PlayerAvatar>)>,
) {
    for (e, character, placement) in &added {
        let Character::Record { id, .. } = character else { continue };
        let Some(init) = stats.actor_ops(id) else { continue };
        let owner = StatOwner::Actor(crate::dialogue::key(e, placement));
        let inputs = Inputs::default();
        let (saved, saved_fx) = placement.zip(overlay.as_deref()).map_or((None, None), |(p, o)| {
            let o = o.0.read().unwrap_or_else(PoisonError::into_inner);
            (saved_sheet(&o, p), crate::effects::saved_effects(&o, p))
        });
        let (effects, modifiers) = match saved_fx {
            Some(saved_fx) => fx.restore(owner, Some(saved_fx)),
            None => fx.learn(owner, &stats.actor_spells(id)),
        };
        let sheet = match saved {
            Some(save) => load_sheet(&stats.table, &rules.0, owner, &save, &init, &inputs, &modifiers),
            None => fresh_sheet(&stats, &rules.0, owner, &inputs, &init, &modifiers),
        };
        commands.entity(e).try_insert((Stats::new(owner, sheet, inputs), effects));
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

/// Saving (save.md "One path for unload and save"): every resident actor's changed sheet and
/// its effects into `overlay` (the snapshot's copy), as its deactivation would, without removing
/// anything.
pub fn persist_actor_stats(world: &World, overlay: &mut PlacementOverlay) {
    let Some(table) = world.get_resource::<StatTableRes>() else { return };
    let tick = world.get_resource::<StatTick>().map_or(0, |t| t.0);
    let Some(mut q) = world.try_query::<(&CharacterPlacement, &Stats, Option<&Effects>)>() else { return };
    for (placement, stats, effects) in q.iter(world) {
        write_sheet(overlay, table, placement, stats);
        if let Some(effects) = effects {
            crate::effects::write_effects(overlay, placement, effects, tick);
        }
    }
}

/// The player's sheet for its `CharacterSave` (`None` without stats).
pub fn player_sheet_save(world: &World, key: sf_rpg::PlayerKey) -> Option<SheetSave> {
    let table = world.get_resource::<StatTableRes>()?;
    let mut q = world.try_query::<(&PlayerCharacter, &Stats)>()?;
    q.iter(world).find(|(p, _)| p.key == key).map(|(_, s)| s.sheet.to_save(&table.0))
}

/// The player's sheet and effects from a loaded `CharacterSave`: the saved sheet with the saved
/// effects' modifiers re-registered before it settles, or (a save from before stats, `None`) a
/// new sheet from the `player` record, the new-game path (stats.md "Saves"); no saved effects
/// (a v3 save, a save from before abilities, or nothing was active and the book was empty)
/// learns the new player's start spells (effects-adapter.md "Saves": the init hook), never on
/// top of saved effects (their abilities are in the save).
pub fn restore_player_sheet(world: &mut World, key: sf_rpg::PlayerKey, save: Option<&SheetSave>, effects: Option<&EffectsSave>) {
    let (Some(stats), Some(rules)) = (world.get_resource::<MwStats>().cloned(), world.get_resource::<StatRules>().cloned()) else { return };
    let owner = StatOwner::Player(key);
    let init = stats.player_ops();
    let inputs = player_inputs();
    let character = match (save, effects) {
        (Some(save), Some(effects)) => {
            let (fx, modifiers) = crate::effects::restore_in_world(world, owner, Some(effects.clone()));
            (Stats::new(owner, load_sheet(&stats.table, &rules.0, owner, save, &init, &inputs, &modifiers), inputs), fx)
        }
        (Some(save), None) => {
            let birthsign = world.get_resource::<PlayerBirthsign>().and_then(|b| b.0.clone());
            let (fx, modifiers) = crate::effects::learn_in_world(world, owner, &stats.player_spells(birthsign.as_ref()));
            (Stats::new(owner, load_sheet(&stats.table, &rules.0, owner, save, &init, &inputs, &modifiers), inputs), fx)
        }
        (None, _) => new_player(world, &stats, &rules.0, owner),
    };
    let found = world.try_query::<(Entity, &PlayerCharacter)>().and_then(|mut q| q.iter(world).find(|(_, p)| p.key == key).map(|(e, _)| e));
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
        let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
        let p = CharacterPlacement(placement.placement.id);
        write_sheet(&mut overlay, table, &p, stats);
        if let Some(effects) = world.get::<Effects>(e) {
            let tick = world.get_resource::<StatTick>().map_or(0, |t| t.0);
            crate::effects::write_effects(&mut overlay, &p, effects, tick);
        }
    }
}
