//! Thin Bevy shell for `sf_combat` (`docs/specs/combat.md` "C1b"). The decisions live in
//! `sf_combat` (Bevy-free); this crate schedules them beside `sf_action_bevy`.
//!
//! A combatant is an action-driven character (`ActionCharacter` + `ActionBody`) with a
//! [`Combatant`] (its `sf_combat::Fighter` and `FightState`). Per fixed tick:
//! 1. [`CombatSystems::Gate`] (before `ActionSystems::Step`): `sf_combat::gate` plus the host's
//!    [`Combatant::withheld`] classes into the character's `ActionGate` (`Sense::refused`).
//! 2. `sf_action::step` and the mover run (`sf_action_bevy`; Clip moves move the body by their
//!    root track).
//! 3. [`CombatSystems::Stats`]: the content refreshes its rules' stat tables (empty here).
//! 4. [`CombatSystems::Step`]: `sf_combat::step` over every combatant (vitals, sweeps, contact
//!    order, outcomes, reactions, hit-stop), on the bodies' poses before and after the mover, in
//!    one frame anchored at the first combatant (`WorldPos` deltas, never `Entity`).
//! 5. [`CombatSystems::Apply`]: presentation messages ([`HitLanded`], [`StaminaFlash`],
//!    [`CombatantDied`]).
//!
//! The spec's `Vitals`, `Sweep` and `Resolve` sets are one set here ([`CombatSystems::Step`]): the
//! core runs them in one call so contact order stays deterministic across them.
//!
//! Socket tracks are built at load ([`socket_tracks`]) from the content's cooked clips and
//! skeleton into [`CombatData`]; the content also inserts its compiled sheets there and adds
//! their rules and slots to `ActionRules`. Gizmos (feature `debug`): [`CombatGizmos`].

use std::collections::HashMap;
use std::marker::PhantomData;

use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::*;
use sf_action::{ClassSet, Refused, SlotKey, TagId};
use sf_action_bevy::{ActionBody, ActionCharacter, ActionGate, ActionSystems, ActionTick};
use sf_anim::{Clip, Skeleton};
use sf_combat::{Body, CombatCtx, CombatEvent, CombatOutput, CombatRules, FightState, Fighter, Movesets, Side, SocketTracks, Tier, Unit};
use sf_coords::WorldPos;

#[cfg(test)]
mod tests;

/// The combat sets in `FixedUpdate` (see the crate docs).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatSystems {
    /// Before `ActionSystems::Step`: the start gate.
    Gate,
    /// After `ActionSystems::Move`: the content's stat refresh.
    Stats,
    /// Vitals, sweeps, contacts, outcomes, reactions and hit-stop.
    Step,
    /// Presentation messages.
    Apply,
}

/// The combat layer with rules `R` (the content's `CombatRules` resource; one monomorphization, in
/// the game). Needs `sf_action_bevy::SfActionPlugin`.
pub struct SfCombatPlugin<R> {
    _rules: PhantomData<fn() -> R>,
}

impl<R> Default for SfCombatPlugin<R> {
    fn default() -> Self {
        Self { _rules: PhantomData }
    }
}

impl<R: CombatRules + Resource> Plugin for SfCombatPlugin<R> {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatData>()
            .add_message::<HitLanded>()
            .add_message::<StaminaFlash>()
            .add_message::<CombatantDied>()
            .configure_sets(FixedUpdate, CombatSystems::Gate.after(ActionSystems::Command).before(ActionSystems::Step))
            .configure_sets(FixedUpdate, (CombatSystems::Stats, CombatSystems::Step, CombatSystems::Apply).chain().after(ActionSystems::Move))
            .add_systems(FixedUpdate, gate::<R>.in_set(CombatSystems::Gate))
            .add_systems(FixedUpdate, step::<R>.in_set(CombatSystems::Step))
            .add_systems(FixedUpdate, flash.in_set(CombatSystems::Apply));
        #[cfg(feature = "debug")]
        app.init_resource::<CombatGizmos>().add_systems(bevy_app::PostUpdate, draw_gizmos);
    }
}

/// Every loaded moveset sheet and the socket tracks of their slots (the content fills it at load).
#[derive(Resource, Default)]
pub struct CombatData {
    pub movesets: Movesets,
    pub sockets: SocketTracks,
    /// The last step's events (tests, overlays); cleared each tick.
    pub events: Vec<CombatEvent>,
}

/// A fighting character: its fighter (vitals, shapes, numbers) and combat state beside its
/// `ActionState`. Requires an `ActionGate` (the gate writes it).
#[derive(Component, Clone, Debug)]
#[require(ActionGate)]
pub struct Combatant {
    pub fighter: Fighter,
    pub fight: FightState,
    /// Classes the host withholds on top of the rules' gate (e.g. attacks while the weapon is
    /// sheathed); a press of one is refused like a gated one.
    pub withheld: ClassSet,
    /// Feet and yaw after the last combat step (the sweep's "before" pose).
    last: Option<(WorldPos, f32)>,
}

impl Combatant {
    pub fn new(fighter: Fighter) -> Self {
        Self { fighter, fight: FightState::default(), withheld: ClassSet::EMPTY, last: None }
    }
}

/// A weapon hit landed (presentation: sound, sparks, camera shake; the reaction and hit-stop
/// already run in gameplay).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct HitLanded {
    pub attacker: Entity,
    pub defender: Entity,
    /// The hit row of the attacker's move.
    pub hit: u8,
    pub point: WorldPos,
    pub kind: TagId,
    pub tier: Tier,
    pub side: Side,
}

/// A press was refused by the gate (the HUD flashes the stamina bar).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaminaFlash {
    pub who: Entity,
}

/// A combatant's health reached 0.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CombatantDied {
    pub who: Entity,
}

/// Bakes the weapon socket track of each slot clip: the socket joint `bone` in root space at
/// 60 Hz (`sf_anim::sample_socket_track`), with its reach bound. Run once at load.
pub fn socket_tracks<'a>(skeleton: &Skeleton, bone: usize, clips: impl IntoIterator<Item = (SlotKey, &'a Clip)>) -> SocketTracks {
    let mut out = SocketTracks::default();
    for (slot, clip) in clips {
        #[allow(clippy::cast_precision_loss)]
        let hz = sf_action::TICK_HZ as f32;
        out.insert(slot, sf_anim::sample_socket_track(skeleton, clip, bone, hz));
    }
    out
}

#[allow(clippy::needless_pass_by_value)]
fn gate<R: CombatRules + Resource>(rules: Res<R>, mut fighters: Query<(&Combatant, &mut ActionGate)>) {
    for (c, mut g) in &mut fighters {
        let refused = sf_combat::gate(&*rules, &c.fighter, &c.fight).union(c.withheld);
        g.set_if_neq(ActionGate(refused));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step<R: CombatRules + Resource>(
    tick: Res<ActionTick>,
    mut data: ResMut<CombatData>,
    rules: Res<R>,
    mut fighters: Query<(Entity, &mut Combatant, &mut ActionCharacter, &ActionBody)>,
    mut hits: MessageWriter<HitLanded>,
    mut deaths: MessageWriter<CombatantDied>,
) {
    let data = &mut *data;
    let mut rows: Vec<_> = fighters.iter_mut().collect();
    // Deterministic order (co-op, replays): by character id, not by archetype iteration.
    rows.sort_by_key(|(_, _, c, _)| c.id);
    let Some(anchor) = rows.first().map(|(_, _, _, b)| b.loco.position) else { return };
    let entities: Vec<Entity> = rows.iter().map(|(e, ..)| *e).collect();
    let mut units: Vec<Unit<'_>> = rows
        .iter_mut()
        .map(|(_, combatant, character, body)| {
            let (pos, yaw) = (body.loco.position, body.loco.motion.yaw);
            let (prev_pos, prev_yaw) = combatant.last.unwrap_or((pos, yaw));
            let body = Body { pos: anchor.delta_f32(pos), yaw, prev_pos: anchor.delta_f32(prev_pos), prev_yaw };
            combatant.last = Some((pos, yaw));
            let (combatant, character) = (&mut **combatant, &mut **character);
            Unit { fighter: &mut combatant.fighter, fight: &mut combatant.fight, action: &mut character.state, output: &character.output, body }
        })
        .collect();
    let ctx = CombatCtx { tick: tick.0, movesets: &data.movesets, sockets: &data.sockets };
    let mut out = CombatOutput::default();
    sf_combat::step(&ctx, &mut units, &*rules, &mut out);
    let ids: HashMap<_, _> = units.iter().zip(&entities).map(|(u, &e)| (u.fighter.id, e)).collect();
    drop(units);
    for e in &out.events {
        match *e {
            CombatEvent::HitLanded { attacker, defender, hit, point, kind, tier, side } => {
                if let (Some(&attacker), Some(&defender)) = (ids.get(&attacker), ids.get(&defender)) {
                    hits.write(HitLanded { attacker, defender, hit, point: anchor.offset(point), kind, tier, side });
                }
            }
            CombatEvent::Died { who } => {
                if let Some(&who) = ids.get(&who) {
                    deaths.write(CombatantDied { who });
                }
            }
            CombatEvent::Evaded { .. } | CombatEvent::Missed { .. } => {}
        }
    }
    data.events = out.events;
}

/// A gate refusal this tick flashes the stamina bar.
fn flash(fighters: Query<(Entity, &ActionCharacter), With<Combatant>>, mut out: MessageWriter<StaminaFlash>) {
    for (who, c) in &fighters {
        if c.output.refusals.iter().any(|r| r.reason == Refused::Gate) {
            out.write(StaminaFlash { who });
        }
    }
}

// --- Gizmos ---------------------------------------------------------------------------------

/// Combat gizmos: the weapon segment of a playing combat move (red while a hit window is open,
/// yellow otherwise) and every combatant's hurt capsule and head (green; cyan in i-frames).
/// Toggle `enabled` (the content binds it to its dev overlay).
#[cfg(feature = "debug")]
#[derive(Resource, Clone, Debug, Default)]
pub struct CombatGizmos {
    pub enabled: bool,
}

#[cfg(feature = "debug")]
#[allow(clippy::needless_pass_by_value)]
fn draw_gizmos(
    settings: Res<CombatGizmos>,
    data: Res<CombatData>,
    mut gizmos: bevy_gizmos::gizmos::Gizmos,
    fighters: Query<(&Combatant, &ActionCharacter, &bevy_transform::components::GlobalTransform)>,
) {
    use bevy_color::Color;
    use sf_combat::SocketSource;
    if !settings.enabled {
        return;
    }
    for (c, character, gt) in &fighters {
        let spec = data.movesets.spec(c.fighter.moveset, character.state.mov);
        let t = character.output.clock.1;
        let iframes = spec.is_some_and(|s| s.iframes_at(t) != sf_combat::HitMask::NONE);
        let hurt = c.fighter.hurt;
        let color = if iframes { Color::srgb(0.2, 0.9, 1.0) } else { Color::srgb(0.2, 1.0, 0.3) };
        let (bottom, top) = (gt.transform_point(hurt.bottom), gt.transform_point(hurt.top));
        for p in [bottom, top] {
            gizmos.sphere(bevy_math::Isometry3d::from_translation(p), hurt.radius, color);
        }
        let side = gt.right() * hurt.radius;
        gizmos.line(bottom + side, top + side, color);
        gizmos.line(bottom - side, top - side, color);
        gizmos.sphere(bevy_math::Isometry3d::from_translation(gt.transform_point(hurt.head)), hurt.head_radius, color);
        let Some(spec) = spec else { continue };
        let Some((slot, clip_t)) = spec.slot_at(t) else { continue };
        let Some(track) = data.sockets.socket(slot) else { continue };
        let open = spec.hits.iter().any(|h| h.start <= t && t < h.end);
        let blade = if open { Color::srgb(1.0, 0.15, 0.1) } else { Color::srgb(1.0, 0.9, 0.2) };
        let w = c.fighter.weapon;
        let (a, b) = (gt.transform_point(track.point(clip_t, w.a)), gt.transform_point(track.point(clip_t, w.b)));
        gizmos.line(a, b, blade);
        gizmos.sphere(bevy_math::Isometry3d::from_translation(b), w.radius, blade);
    }
}
