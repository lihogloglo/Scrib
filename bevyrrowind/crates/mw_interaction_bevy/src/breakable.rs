//! Breakables into the destruction shell (crate docs, "Breakables"; superfargoth
//! `docs/specs/interaction-destruction.md` "Game wiring" and "Dev source", slice `IN38b1`).
//!
//! - **Records** ([`BreakablesTable`]): every breakable `CONT` and `MISC` record by id, classified
//!   once from the host's record db ([`insert_records`], `mw_interaction::breakable::classify`): a
//!   record is refused when its own script handles activation or death ([`ACTIVATION_BLOCKS`]) or
//!   when any `SCPT` or dialogue result names it (`mw_world::source::collect_script_tokens`).
//!   Decorative `STAT` urns are counted but not listed: their live removal is `IN38c`.
//! - **Def** ([`attach`]): the index hook (`index.rs`) gives a breakable placement its
//!   `destructible_def` (live [`BreakSettings`]) and material, on top of its furniture def or as a
//!   slotless def of its own; the shell's `Destructibles` registers it from the `Insert`. The
//!   `mw.rules.interaction.break` switch off (Classic) attaches nothing.
//! - **Rules** ([`sync_rules`]): the shell's `DestructionRules` hold `damage_rules` of the live
//!   settings (empty with the switch off).
//! - **Save** ([`save_rest`]): each `DestructChanged` of a standing placement writes its rest bytes
//!   to the overlay (owner `destruct`); a removing state writes none (`IN38b2` removes the
//!   placement). On activation the hook sends `DestructRest` from those bytes after the `Insert`.
//! - **Dev source** ([`dev_break`], the `dev` feature registers it): `Delete` writes one
//!   `PropDamage` of the def's full health, kind blunt, on the player's focused placement.
//!
//! Log: `interact: breakables: N records (...), M refused (...), S decorative statics (IN38c)`.

use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError};

use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_log::info;
use bevy_math::Vec3;
use mw_interaction::breakable::{Source, kind};
use mw_interaction::{BreakClass, BreakFacts, BreakParams, Breakable, Eligibility, Refusal, damage_rules, destructible_def};
use sf_interaction::{DESTRUCT_STATE_OWNER, DestructState, InteractableDef};
use sf_interaction_bevy::{DestructChanged, DestructRest, Destructibles, DestructionRules, PlayerFocus, PropDamage};
use sf_records::{RecordDb, RecordId};
use sf_world::PlacementId;
use sf_world_bevy::WorldOverlay;
use smallvec::SmallVec;
use tes3::esp::{Container, ContainerFlags, MiscItem, Script, Static};

use crate::input::PLAYER_USER;

/// Script blocks that make a record's own script decide what activating or killing it does: such a
/// record stays whole (interaction-adapter.md "Breakables", "Scripts").
pub const ACTIVATION_BLOCKS: &[&str] = &["onactivate", "ondeath", "onmurder", "onknockout", "onpchitme"];

/// The dev source's key (`visual-checks.md` key table).
pub const DEV_BREAK_KEY: KeyCode = KeyCode::Delete;

/// Every breakable `CONT` / `MISC` record, by id.
#[derive(Resource, Clone, Debug, Default)]
pub struct BreakablesTable(pub Arc<BTreeMap<RecordId, Breakable>>);

impl BreakablesTable {
    #[must_use]
    pub fn get(&self, id: &RecordId) -> Option<Breakable> {
        self.0.get(id).copied()
    }
}

/// What [`BreakablesTable::build`] found, for the log line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BreakCounts {
    /// Listed records per class.
    pub breakable: BTreeMap<BreakClass, u32>,
    /// Refused records per reason.
    pub refused: BTreeMap<Refusal, u32>,
    /// Breakable decorative `STAT`s, not listed (`IN38c`).
    pub statics: u32,
}

/// Whether script `name`'s text has one of [`ACTIVATION_BLOCKS`].
fn handles_activation(db: &RecordDb, name: &str) -> bool {
    let name = name.trim();
    if name.is_empty() {
        return false;
    }
    let Some(script) = db.get::<Script>(&RecordId::new(name)) else { return false };
    let mut words = std::collections::BTreeSet::new();
    mw_world::source::script_tokens(&script.text, &mut words);
    ACTIVATION_BLOCKS.iter().any(|b| words.contains(*b))
}

impl BreakablesTable {
    /// The breakable records of `db` and the counts (module docs, "Records").
    #[must_use]
    pub fn build(db: &RecordDb) -> (Self, BreakCounts) {
        let named = mw_world::source::collect_script_tokens(db);
        let mut table = BTreeMap::new();
        let mut counts = BreakCounts::default();
        let mut judge = |id: &RecordId, source: Source, organic: bool, script: &str| {
            let facts = BreakFacts {
                source,
                id: id.as_str(),
                organic,
                activate_script: handles_activation(db, script),
                named_by_script: named.contains(id.as_str()),
            };
            match mw_interaction::breakable::classify(&facts) {
                Eligibility::Not => {}
                Eligibility::Refused(_, r) => *counts.refused.entry(r).or_default() += 1,
                Eligibility::Breakable(_) if source == Source::Static => counts.statics += 1,
                Eligibility::Breakable(b) => {
                    *counts.breakable.entry(b.class).or_default() += 1;
                    table.insert(id.clone(), b);
                }
            }
        };
        for (id, c) in db.iter::<Container>() {
            judge(id, Source::Container, c.container_flags.contains(ContainerFlags::ORGANIC), &c.script);
        }
        for (id, m) in db.iter::<MiscItem>() {
            judge(id, Source::Misc, false, &m.script);
        }
        for (id, _) in db.iter::<Static>() {
            judge(id, Source::Static, false, "");
        }
        (Self(Arc::new(table)), counts)
    }
}

/// Inserts the content's [`BreakablesTable`] (the host's record db) and logs its counts.
pub fn insert_records(app: &mut bevy_app::App, db: &RecordDb) {
    fn list<K: std::fmt::Debug>(m: &BTreeMap<K, u32>) -> String {
        m.iter().map(|(k, n)| format!("{k:?} {n}")).collect::<Vec<_>>().join(", ")
    }
    let (table, counts) = BreakablesTable::build(db);
    info!(
        "interact: breakables: {} records ({}), {} refused ({}), {} decorative statics (IN38c)",
        table.0.len(),
        list(&counts.breakable),
        counts.refused.values().sum::<u32>(),
        list(&counts.refused),
        counts.statics,
    );
    app.insert_resource(table);
}

/// The breakables' tunables (`mw.rules.interaction.break*`), bound live.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct BreakSettings(pub BreakParams);

type FloatDef = sf_tunables_bevy::Def<f32>;
/// The [`BreakParams`] field a float tunable sets.
type FieldOf = fn(&mut BreakParams) -> &mut f32;

/// Binds every `mw.rules.interaction.break*` tunable to [`BreakSettings`].
pub(crate) fn bind_settings(app: &mut bevy_app::App) {
    use mw_interaction::tunables as t;
    use sf_tunables_bevy::TunablesAppExt;
    app.init_resource::<BreakSettings>();
    let _ = app.add_tunable(&t::BREAK).bind_resource(|s: &mut BreakSettings, v| s.0.enabled = v);
    let floats: [(&FloatDef, FieldOf); 11] = [
        (&t::BREAK_CRATE_HP, |p| &mut p.crate_hp),
        (&t::BREAK_BARREL_HP, |p| &mut p.barrel_hp),
        (&t::BREAK_URN_HP, |p| &mut p.urn_hp),
        (&t::BREAK_SACK_HP, |p| &mut p.sack_hp),
        (&t::BREAK_POTTERY_HP, |p| &mut p.pottery_hp),
        (&t::BREAK_GLASS_HP, |p| &mut p.glass_hp),
        (&t::BREAK_BLUNT, |p| &mut p.blunt),
        (&t::BREAK_BLADE, |p| &mut p.blade),
        (&t::BREAK_ARROW, |p| &mut p.arrow),
        (&t::BREAK_HAND, |p| &mut p.hand),
        (&t::BREAK_FIRE_WOOD, |p| &mut p.fire_wood),
    ];
    for (def, field) in floats {
        let _ = app.add_tunable(def).bind_resource(move |s: &mut BreakSettings, v| *field(&mut s.0) = v);
    }
}

/// `base`'s def with its destructible attached when it is a breakable and the switch is on:
/// `furniture` with `destructible` and `material` set, else a slotless def of its own.
#[must_use]
pub fn attach(world: &World, base: &RecordId, furniture: Option<&Arc<InteractableDef>>) -> Option<Arc<InteractableDef>> {
    let b = world.get_resource::<BreakablesTable>()?.get(base)?;
    let params = world.get_resource::<BreakSettings>().map_or_else(BreakParams::default, |s| s.0);
    let destructible = destructible_def(b, &params)?;
    let mut def = furniture.map_or_else(|| bare(base), |d| InteractableDef::clone(d));
    def.material = Some(b.class.material());
    def.destructible = Some(destructible);
    Some(Arc::new(def))
}

/// A def with no slots, verbs or adverts.
fn bare(base: &RecordId) -> InteractableDef {
    InteractableDef {
        id: base.clone(),
        slots: Vec::new(),
        verbs: SmallVec::new(),
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    }
}

/// `def` without its furniture: its id, material and destructible only (a breakable chair while
/// sitting is off).
pub(crate) fn bare_of(def: &InteractableDef) -> InteractableDef {
    InteractableDef { material: def.material, destructible: def.destructible.clone(), ..bare(&def.id) }
}

/// The rest state the overlay holds for `id`, if it decodes.
pub(crate) fn saved_state(world: &World, id: PlacementId) -> Option<DestructState> {
    let overlay = world.get_resource::<WorldOverlay>()?;
    let overlay = overlay.0.read().unwrap_or_else(PoisonError::into_inner);
    DestructState::from_rest_bytes(overlay.state(id, DESTRUCT_STATE_OWNER)?)
}

/// After the `Insert` of a breakable def: its saved state back into the shell.
pub(crate) fn restore(world: &mut World, id: PlacementId) {
    if let Some(state) = saved_state(world, id) {
        world.write_message(DestructRest { placement: id, state });
    }
}

/// The shell's `DestructionRules` from the live settings (module docs, "Rules").
pub(crate) fn sync_rules(settings: Res<BreakSettings>, mut rules: ResMut<DestructionRules>) {
    let mut fresh = DestructionRules::default();
    for (k, m, f) in damage_rules(&settings.0) {
        fresh.set(k, m, f);
    }
    if *rules != fresh {
        *rules = fresh;
    }
}

/// Each `DestructChanged` of a standing placement into its overlay bytes (module docs, "Save").
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn save_rest(mut changed: MessageReader<DestructChanged>, registry: Res<Destructibles>, overlay: Option<Res<WorldOverlay>>) {
    let standing: Vec<DestructChanged> =
        changed.read().filter(|c| registry.get(c.placement).is_some_and(|d| !d.gone)).copied().collect();
    let Some(overlay) = overlay.filter(|_| !standing.is_empty()) else { return };
    let mut overlay = overlay.0.write().unwrap_or_else(PoisonError::into_inner);
    for c in standing {
        overlay.set_state(c.placement, DESTRUCT_STATE_OWNER, c.state.rest_bytes().to_vec());
    }
}

/// The dev source (module docs): [`DEV_BREAK_KEY`] breaks the player's focused destructible.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn dev_break(
    keys: Option<Res<ButtonInput<KeyCode>>>,
    focus: Res<PlayerFocus>,
    registry: Res<Destructibles>,
    mut out: MessageWriter<PropDamage>,
) {
    if !keys.is_some_and(|k| k.just_pressed(DEV_BREAK_KEY)) {
        return;
    }
    let Some(placement) = focus.get(PLAYER_USER).and_then(sf_interaction_bevy::FocusState::target) else { return };
    let Some(d) = registry.get(placement).filter(|d| !d.gone) else {
        info!("interact: dev break: the focused placement is not breakable");
        return;
    };
    let Some(health) = d.destructible().map(|def| def.health) else { return };
    info!("interact: dev break: {health} blunt on {}", d.def.id.as_str());
    out.write(PropDamage { placement, amount: health, kind: kind::BLUNT, point: d.object.translation, dir: Vec3::NEG_Y, culprit: Some(PLAYER_USER) });
}
