//! Combat vitals from character stats (stats.md "Consumers" and "Per-tick pipeline", S5;
//! stats-adapter.md "Consumers"): the sheet is the truth, `sf_combat::Vitals` a per-tick copy.
//!
//! - **Link.** A [`Combatant`] gets a [`CombatSheet`] once its sheet exists ([`link_sheets`]): the
//!   player body's is the [`PlayerCharacter`]'s (`mw_rpg_bevy::stats`, [`HOST_KEY`]), any other
//!   combatant's is on a child (the training dummy's dressed `NPC_` character). The linked sheet's
//!   `Inputs::combatant` is set (combat owns its stamina regen: `DerivedModel::regen` skips it), and
//!   cleared when no combatant links it any more (the body despawned with the free camera).
//! - **Refresh** ([`refresh_vitals`], `CombatSystems::Stats`, after `StatSystems::Apply`): health
//!   1:1 (a current below the pool's `depleted_below` is 0, dead to combat as to the sheet),
//!   stamina = fatigue x [`mw_combat::STAMINA_PER_FATIGUE`] (a negative fatigue shows as 0). What
//!   was written is kept.
//! - **Differences back** ([`return_differences`], `StatSystems::Combat`, after the combat step):
//!   each pool's change since the refresh, exactly what combat applied with its clamping, goes to
//!   the sheet as `PoolAdjust` (fatigue / `STAMINA_PER_FATIGUE`), cause `Combat`, synchronously.
//!   Clamps made while writing (above max, below zero) are never sent back.
//! - **Death** ([`deaths`], after `StatSystems::Events`): `Depleted(health)` of a linked sheet is
//!   the death signal; combat's own `CombatantDied` already covers a hit that took its health to
//!   0, so only the rest (a hit that left less than 1, any other cause) writes one.
//!
//! A combatant without a sheet (no stats loaded, tests) keeps its own vitals, as before S5.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use mw_game_core::player::{HOST_KEY, PlayerBody, PlayerCharacter};
use sf_combat::{Pool, Vitals};
use sf_combat_bevy::{CombatSystems, Combatant, CombatantDied};
use sf_stats_bevy::sf_stats::{Cause, CauseKind, Role, SheetRead, StatEvent, StatIdx, StatKind, StatOp, StatOwner, StatTable};
use sf_stats_bevy::{PendingStatEvents, StatIndex, StatMessage, StatRules, StatSystems, StatTableRes, StatTick, Stats};

/// A combatant's character sheet: its owner and the vitals the last refresh wrote.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct CombatSheet {
    pub owner: StatOwner,
    written: Option<Vitals>,
}

/// The glue's places in `FixedUpdate`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VitalsSystems {
    /// After `StatSystems::Apply`, before `CombatSystems::Stats`: new combatants link their sheet.
    Link,
    /// After `StatSystems::Events`: the death hook. Readers of `CombatantDied` that must see a
    /// sheet death in the same tick order after it.
    Deaths,
}

/// Orders the stat sets around the combat sets and adds the glue. Called from `combat::add`.
pub(crate) fn add(app: &mut App) {
    // Without stats loaded the glue idles; the death hook still needs its reader's message.
    app.add_message::<StatMessage>()
        .configure_sets(FixedUpdate, StatSystems::Apply.before(CombatSystems::Stats))
        .configure_sets(FixedUpdate, StatSystems::Combat.after(CombatSystems::Step))
        .configure_sets(FixedUpdate, VitalsSystems::Link.after(StatSystems::Apply).before(CombatSystems::Stats))
        .configure_sets(FixedUpdate, VitalsSystems::Deaths.after(StatSystems::Events))
        .add_systems(FixedUpdate, link_sheets.in_set(VitalsSystems::Link))
        .add_systems(FixedUpdate, refresh_vitals.in_set(CombatSystems::Stats))
        .add_systems(FixedUpdate, return_differences.in_set(StatSystems::Combat))
        .add_systems(FixedUpdate, deaths.in_set(VitalsSystems::Deaths));
}

/// The pools combat sees: (health, fatigue) by role.
fn roles(table: &StatTable) -> Option<(StatIdx, StatIdx)> {
    Some((table.role(Role::Health)?, table.role(Role::Stamina)?))
}

/// A pool's `depleted_below` (0 for anything else).
fn depleted_below(table: &StatTable, pool: StatIdx) -> f32 {
    match table.def(pool).kind {
        StatKind::Pool { depleted_below, .. } => depleted_below,
        _ => 0.0,
    }
}

/// The combat vitals of a sheet (module docs, "Refresh"); `poise` is combat's own.
#[must_use]
pub fn vitals_of(s: &SheetRead<'_>, (health, fatigue): (StatIdx, StatIdx), poise: Option<Pool>) -> Vitals {
    let max = s.max(health).max(0.0);
    let cur = s.current(health);
    let cur = if cur < depleted_below(s.table(), health) { 0.0 } else { cur.min(max) };
    let stamina_max = mw_combat::stamina_from_fatigue(s.max(fatigue));
    let stamina = mw_combat::stamina_from_fatigue(s.current(fatigue)).min(stamina_max);
    Vitals { health: Pool { cur, max }, stamina: Pool { cur: stamina, max: stamina_max }, poise }
}

/// Links each new combatant to its sheet (module docs, "Link").
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)] // system parameters
fn link_sheets(
    mut commands: Commands,
    new: Query<(Entity, Has<PlayerBody>, Option<&Children>), (With<Combatant>, Without<CombatSheet>)>,
    players: Query<(&PlayerCharacter, &Stats)>,
    sheets: Query<&Stats>,
) {
    for (e, player, children) in &new {
        let owner = if player {
            players.iter().find(|(p, _)| p.key == HOST_KEY).map(|(_, s)| s.owner)
        } else {
            children.and_then(|c| c.iter().find_map(|child| sheets.get(child).ok().map(|s| s.owner)))
        };
        if let Some(owner) = owner {
            commands.entity(e).try_insert(CombatSheet { owner, written: None });
        }
    }
}

/// Sheet → combat vitals (module docs, "Refresh"); keeps `Inputs::combatant` current.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn refresh_vitals(
    table: Option<Res<StatTableRes>>,
    index: Option<Res<StatIndex>>,
    mut fighters: Query<(&mut Combatant, &mut CombatSheet)>,
    mut sheets: Query<&mut Stats>,
    mut linked: Local<Vec<StatOwner>>,
) {
    let (Some(table), Some(index)) = (table, index) else { return };
    let Some(pools) = roles(&table.0) else { return };
    linked.clear();
    for (mut combatant, mut link) in &mut fighters {
        let Some(mut stats) = index.get(link.owner).and_then(|e| sheets.get_mut(e).ok()) else {
            link.written = None;
            continue;
        };
        linked.push(link.owner);
        if !stats.inputs.combatant {
            stats.inputs.combatant = true;
        }
        let vitals = vitals_of(&stats.view(&table.0), pools, combatant.fighter.vitals.poise);
        combatant.fighter.vitals = vitals;
        link.written = Some(vitals);
    }
    for mut stats in &mut sheets {
        if stats.inputs.combatant && !linked.contains(&stats.owner) {
            stats.inputs.combatant = false;
        }
    }
}

/// The sheet ops for combat's change since `written` (module docs, "Differences back").
#[must_use]
pub fn difference_ops(written: &Vitals, now: &Vitals, (health, fatigue): (StatIdx, StatIdx)) -> Vec<StatOp> {
    let mut ops = Vec::new();
    let dh = now.health.cur - written.health.cur;
    if dh != 0.0 {
        ops.push(StatOp::PoolAdjust { pool: health, delta: dh, below_zero: false, above_max: false });
    }
    let ds = now.stamina.cur - written.stamina.cur;
    if ds != 0.0 {
        let delta = ds / mw_combat::STAMINA_PER_FATIGUE;
        ops.push(StatOp::PoolAdjust { pool: fatigue, delta, below_zero: false, above_max: false });
    }
    ops
}

/// Synchronous ops on a sheet by owner (`Stats::apply`, events through `PendingStatEvents`):
/// idle without stats loaded.
#[derive(SystemParam)]
pub struct SheetOps<'w, 's> {
    table: Option<Res<'w, StatTableRes>>,
    rules: Option<Res<'w, StatRules>>,
    index: Option<Res<'w, StatIndex>>,
    tick: Option<Res<'w, StatTick>>,
    pending: Option<ResMut<'w, PendingStatEvents>>,
    sheets: Query<'w, 's, &'static mut Stats>,
}

impl SheetOps<'_, '_> {
    /// The table, when stats are loaded.
    pub fn table(&self) -> Option<&StatTable> {
        self.table.as_deref().map(|t| &*t.0)
    }

    /// Applies the ops `make` builds from the table and `owner`'s sheet, cause `kind` at this
    /// tick. False: no stats or no such sheet (nothing applied).
    pub fn apply(&mut self, owner: StatOwner, kind: CauseKind, make: impl FnOnce(&SheetRead<'_>) -> Vec<StatOp>) -> bool {
        let (Some(table), Some(rules), Some(index), Some(pending)) = (&self.table, &self.rules, &self.index, &mut self.pending) else {
            return false;
        };
        let Some(mut stats) = index.get(owner).and_then(|e| self.sheets.get_mut(e).ok()) else { return false };
        let ops = make(&stats.view(&table.0));
        if ops.is_empty() {
            return true;
        }
        let cause = Cause { kind, source: None, tick: self.tick.as_ref().map_or(0, |t| t.0) };
        let rejected = stats.apply(&table.0, &rules.0, &ops, cause, &mut pending.0);
        if rejected > 0 {
            bevy_log::warn!("stats: {rejected} of {} {kind:?} ops for {owner:?} rejected", ops.len());
        }
        true
    }
}

/// Combat vitals → sheet (module docs, "Differences back").
fn return_differences(mut fighters: Query<(&Combatant, &mut CombatSheet)>, mut sheets: SheetOps) {
    let Some(pools) = sheets.table().and_then(roles) else { return };
    for (combatant, mut link) in &mut fighters {
        let Some(written) = link.written.take() else { continue };
        let ops = difference_ops(&written, &combatant.fighter.vitals, pools);
        if !ops.is_empty() {
            sheets.apply(link.owner, CauseKind::Combat, |_| ops);
        }
    }
}

/// `Depleted(health)` of a linked sheet → `CombatantDied`, unless combat already sent it (module
/// docs, "Death").
#[allow(clippy::needless_pass_by_value)] // system parameters
fn deaths(
    table: Option<Res<StatTableRes>>,
    mut events: MessageReader<StatMessage>,
    fighters: Query<(Entity, &Combatant, &CombatSheet)>,
    mut out: MessageWriter<CombatantDied>,
) {
    let Some(health) = table.as_ref().and_then(|t| t.0.role(Role::Health)) else {
        events.clear();
        return;
    };
    for StatMessage(e) in events.read() {
        let StatEvent::Depleted { owner, pool, cause } = e else { continue };
        if *pool != health {
            continue;
        }
        let Some((who, combatant, _)) = fighters.iter().find(|(_, _, l)| l.owner == *owner) else { continue };
        // Combat's own death: a hit took its health to 0 this tick and it wrote `CombatantDied`.
        if cause.kind == CauseKind::Combat && combatant.fighter.vitals.health.cur <= 0.0 {
            continue;
        }
        out.write(CombatantDied { who });
    }
}

/// Test helpers: a three-pool table (health, magicka, fatigue with their roles) and sheets on it.
#[cfg(test)]
pub(crate) mod testing {
    use std::sync::Arc;

    use bevy_app::App;
    use bevy_ecs::entity::Entity;
    use sf_stats_bevy::sf_stats::{
        ApplyCtx, Cause, CauseKind, Inputs, Rebase, Role, Settle, Sheet, StatDef, StatIdx, StatKind, StatOp, StatOwner, StatTable, TestModel,
    };
    use sf_stats_bevy::{SfStatsPlugin, StatRules, StatTableRes, Stats};

    pub const HEALTH: StatIdx = StatIdx(0);
    pub const MAGICKA: StatIdx = StatIdx(1);
    pub const FATIGUE: StatIdx = StatIdx(2);
    /// The stats model's fatigue regen, per second (combat's regen must replace it for a combatant).
    pub const FATIGUE_REGEN: f32 = 10.0;

    fn pool(key: &str, depleted_below: f32, rebase: Rebase) -> StatDef {
        StatDef { key: key.into(), kind: StatKind::Pool { depleted_below, rebase }, group: "test".into(), public: false, label: String::new() }
    }

    /// The stats plugin with the three-pool table and a `TestModel` regenerating fatigue.
    pub fn add_stats(app: &mut App) {
        let defs = vec![pool("health", 1.0, Rebase::KeepCurrent), pool("magicka", 0.0, Rebase::KeepRatio), pool("fatigue", 0.0, Rebase::KeepRatio)];
        let roles = [(Role::Health, "health".into()), (Role::Mana, "magicka".into()), (Role::Stamina, "fatigue".into())];
        let table = StatTable::new(defs, &roles).expect("table builds");
        let model = TestModel { regen: vec![(FATIGUE, FATIGUE_REGEN)], ..TestModel::default() }.rules();
        app.add_plugins(SfStatsPlugin).insert_resource(StatTableRes(Arc::new(table))).insert_resource(StatRules(Arc::new(model)));
    }

    /// A full sheet with these maxima (health, magicka, fatigue).
    pub fn sheet(app: &App, owner: StatOwner, [health, magicka, fatigue]: [f32; 3]) -> Stats {
        let table = &app.world().resource::<StatTableRes>().0;
        let rules = &app.world().resource::<StatRules>().0;
        let inputs = Inputs::default();
        let mut s = Sheet::new(table);
        let ops = [
            StatOp::SetBase { stat: HEALTH, value: health },
            StatOp::SetBase { stat: MAGICKA, value: magicka },
            StatOp::SetBase { stat: FATIGUE, value: fatigue },
        ];
        let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs: &inputs };
        assert_eq!(sf_stats_bevy::sf_stats::apply(table, rules, &mut s, &ops, &ctx, &mut Vec::new()), 0);
        s.settle(table, rules, &inputs, Settle::NewCharacter);
        Stats::new(owner, s, inputs)
    }

    /// A pool's (current, max) on `e`'s sheet.
    pub fn pool_of(app: &App, e: Entity, pool: StatIdx) -> (f32, f32) {
        let table = &app.world().resource::<StatTableRes>().0;
        let v = app.world().get::<Stats>(e).expect("a sheet");
        let r = v.view(table);
        (r.current(pool), r.max(pool))
    }
}
