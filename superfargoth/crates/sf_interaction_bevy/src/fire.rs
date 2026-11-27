//! Fire step shell (`docs/specs/interaction-world-rules.md` "Rules and fire shell", its `IN39b` fire
//! step), slice `IN39b`: the core's [`step_fire`] on the fixed tick over the resident flammable
//! placements, on the same [`InteractionObjectStates`] as the rules. The heat, fuel, rain, budget
//! and round-robin maths are the core's ([`Fire`]).
//!
//! - **Candidates** ([`FireCandidates`]): every [`IndexChange::Insert`] whose def material is
//!   `flammable` in [`InteractionMaterials`] (a re-insert without one, or a removal, drops it),
//!   rebuilt from [`InteractionDefs`] when the materials change. Each keeps its shelter and zone
//!   answers while it rests: asked once, forgotten on its next `Insert` or `Move`.
//! - **Host inputs**, all optional (unset = dry, open sky, wild): [`FireWeather`] (mean wind, the
//!   active space's interior flag), [`FireWetness`] (surface wetness at a point), [`FireShelter`]
//!   (a roof above a point), [`FireZones`] (a point inside a settlement). Wetness is 0 indoors or
//!   sheltered; shelter is asked only when a wetness closure is set and outdoors, the zone only for
//!   a `structure` material.
//! - **Step** (`FixedUpdate`, [`FireSystems`]): bodies at [`InteractionFrames::current`]; a body that
//!   is not burning and whose wetness is at least `interact.rules.wet_threshold` gets `Wet` refreshed
//!   (`interact.rules.wet_ttl_s`) first (a burning one is the core's: `interact.fire.rain_douse`
//!   puts it out); then [`step_fire`] with the tick's params. The `Burning` / `Wet` / `Charred` tags
//!   of every placement refreshed or with an event are mirrored into [`InteractionStateTags`].
//! - **Culprits** ([`FireCulprits`]): a [`RuleApplied`] `Added(Burning)` stores the rule's culprit
//!   and its source's first tag as the cause; a heat ignition inherits the entry of the nearest
//!   `spread` body burning at the tick's start within `interact.fire.spread_radius_m` (distance,
//!   then placement; none in reach, or one without an entry: none). `Removed(Burning)`, a douse, a
//!   burn-out and an index removal forget the entry (the douse and burn-out after their event).
//! - **Changes**: a rule's `Added(Burning)` (as an `Ignited` [`FireEvent`]), then the tick's heat
//!   ignitions and burn-outs, into [`ChangeInbox`] through [`from_fire`] (site from
//!   [`ChangeOwners`]; a douse is no change kind).
//! - **Outputs** (`IN39c`, after the step, same set): every transition of the tick (a rule's
//!   `Added(Burning)` as `Ignited`, its `Removed`/`Expired(Burning)` as `Doused`, then the step's)
//!   is one [`FireChanged`] for the presenter. A burn-out of a registered destructible is one
//!   [`PropStage`] of kind [`DAMAGE_FIRE`] (`burnt_stage`, else the def's last stage) with the
//!   fire's culprit; the world-change step drops a fire [`crate::Destroyed`] on a `Charred`
//!   placement (its `Burnt` said it). Each placement burning after the step is in
//!   [`BurningObjects`] (frame, burn start, fire region, `light`); a burning destructible not
//!   `gone` takes one [`PropDamage`] of `interact.fire.damage_per_s / tick_hz` (kind fire, `+Y`,
//!   the fire's culprit). `light`: per region, the lit ones keep their slot (the latest-started
//!   past `interact.fire.lights_max` lose it), then free slots go to the earliest-started (then
//!   placement), so a new fire never takes a lit one's slot and a fire going out frees one. A
//!   burning placement that is not a registered light is a [`GameplayLights`] entry
//!   (`interact.fire.light_radius_m`, intensity 1) until it stops burning or leaves the index.
//!
//! Not yet: saving states and fire (`IN39d`).

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::{Vec2, Vec3};
use sf_interaction::{
    BURNING, CHARRED, DAMAGE_FIRE, Fire, FireBody, FireEnv, FireEvent, FireOutput, RuleOutput, TagId, Tick, Ttl,
    UserKey, WET, fire_region, from_fire, step_fire,
};
use sf_world::PlacementId;

use crate::{
    ChangeInbox, ChangeOwners, Destructibles, FireSystems, GameplayLight, GameplayLights, IndexChange,
    InteractionDefs, InteractionFrames, InteractionMaterials, InteractionObjectStates, InteractionSettings,
    InteractionStateTags, InteractionTick, Lights, PropDamage, PropStage, RuleApplied, effective_params,
};

/// A host sample at a world point (wetness, ratio).
pub type SampleAt = dyn Fn(Vec3) -> f32 + Send + Sync;
/// A host yes/no at a world point (sheltered, inside a settlement).
pub type ProbeAt = dyn Fn(Vec3) -> bool + Send + Sync;

/// The fire's weather, filled by the host.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FireWeather {
    /// Mean horizontal wind `(x, z)` at the player, m/s.
    pub wind: Vec2,
    /// The active space is an interior: wetness 0, no shelter asked.
    pub indoors: bool,
}

/// Surface wetness at a point (`sf_surface`'s wet channel); unset = dry.
#[derive(Resource, Clone, Default)]
pub struct FireWetness(pub Option<Arc<SampleAt>>);

/// Whether a point has a roof above it (the host's probe); unset = open sky.
#[derive(Resource, Clone, Default)]
pub struct FireShelter(pub Option<Arc<ProbeAt>>);

/// Whether a point is inside a settlement zone (the adapter's); unset = wild.
#[derive(Resource, Clone, Default)]
pub struct FireZones(pub Option<Arc<ProbeAt>>);

/// The core fire table of the resident placements.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionFire(pub Fire);

/// One resident flammable placement and its answers cached while it rests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FireCandidate {
    pub material: TagId,
    /// Under a roof, once asked.
    pub sheltered: Option<bool>,
    /// Inside a settlement zone, once asked.
    pub in_settlement: Option<bool>,
}

/// The resident placements whose def material is flammable (module docs, "Candidates").
#[derive(Resource, Clone, Debug, Default)]
pub struct FireCandidates(BTreeMap<PlacementId, FireCandidate>);

impl FireCandidates {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&FireCandidate> {
        self.0.get(&placement)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Who set a fire burning and with what.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FireCulprit {
    pub culprit: Option<UserKey>,
    /// The source tag (the rule cause's first; inherited along the spread).
    pub cause: Option<TagId>,
}

/// The culprit of each burning placement that has one (module docs, "Culprits").
#[derive(Resource, Clone, Debug, Default)]
pub struct FireCulprits(BTreeMap<PlacementId, FireCulprit>);

impl FireCulprits {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&FireCulprit> {
        self.0.get(&placement)
    }
}

/// One burning placement (module docs, "Outputs").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BurningObject {
    /// World metres (interaction frames).
    pub at: Vec3,
    /// The tick it started burning.
    pub since: Tick,
    /// Its fire region (`interact.fire.region_m` squares).
    pub region: (i32, i32),
    /// One of its region's `interact.fire.lights_max` fire lights (the rest: flames without a light).
    pub light: bool,
}

/// The placements burning after this tick's fire step, for the presenter (module docs, "Outputs").
#[derive(Resource, Clone, Debug, Default)]
pub struct BurningObjects(BTreeMap<PlacementId, BurningObject>);

impl BurningObjects {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&BurningObject> {
        self.0.get(&placement)
    }

    /// In placement order.
    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &BurningObject)> {
        self.0.iter().map(|(p, o)| (*p, o))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One fire transition of one placement, for the presenter (flames, smoke, sound).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct FireChanged {
    pub tick: Tick,
    pub placement: PlacementId,
    pub output: FireOutput,
    /// World metres.
    pub at: Vec3,
}

/// One transition of this tick with its culprit and place, for [`outputs`].
#[derive(Clone, Copy, Debug)]
struct TickFire {
    event: FireEvent,
    who: FireCulprit,
    at: Vec3,
}

/// The tick's transitions, filled by [`rule_lit`] and [`step`], drained by [`outputs`].
#[derive(Resource, Clone, Debug, Default)]
struct FireTick(Vec<TickFire>);

pub(crate) fn build(app: &mut App) {
    app.init_resource::<FireWeather>()
        .init_resource::<FireWetness>()
        .init_resource::<FireShelter>()
        .init_resource::<FireZones>()
        .init_resource::<InteractionFire>()
        .init_resource::<FireCandidates>()
        .init_resource::<FireCulprits>()
        .init_resource::<BurningObjects>()
        .init_resource::<FireTick>()
        .add_message::<FireChanged>()
        .add_systems(FixedUpdate, (track, rule_lit, step, outputs).chain().in_set(FireSystems));
}

fn flammable(materials: &InteractionMaterials, material: Option<TagId>) -> Option<TagId> {
    material.filter(|m| materials.get(*m).is_some_and(|d| d.flammable.is_some()))
}

/// The tick's index changes into [`FireCandidates`] (module docs, "Candidates").
#[allow(clippy::needless_pass_by_value)]
fn track(
    mut changes: MessageReader<IndexChange>,
    defs: Res<InteractionDefs>,
    materials: Res<InteractionMaterials>,
    mut candidates: ResMut<FireCandidates>,
    mut culprits: ResMut<FireCulprits>,
) {
    let fresh = |material| FireCandidate { material, sheltered: None, in_settlement: None };
    for change in changes.read() {
        match change {
            IndexChange::Insert { placement, def, .. } => match flammable(&materials, def.material) {
                Some(m) => {
                    candidates.0.insert(*placement, fresh(m));
                }
                None => {
                    candidates.0.remove(placement);
                }
            },
            IndexChange::Move { placement, .. } => {
                if let Some(c) = candidates.0.get_mut(placement) {
                    *c = fresh(c.material);
                }
            }
            IndexChange::Remove { placement, .. } => {
                candidates.0.remove(placement);
                culprits.0.remove(placement);
            }
        }
    }
    if materials.is_changed() {
        let mut next = BTreeMap::new();
        for (p, def) in &defs.0 {
            if let Some(m) = flammable(&materials, def.material) {
                let kept = candidates.0.get(p).filter(|c| c.material == m).copied();
                next.insert(*p, kept.unwrap_or_else(|| fresh(m)));
            }
        }
        candidates.0 = next;
    }
}

/// A fire change into the inbox, at the placement's frame (gone: none).
fn push_change(
    e: &FireEvent,
    tick: Tick,
    who: FireCulprit,
    frames: &InteractionFrames,
    owners: &ChangeOwners,
    changes: &mut ChangeInbox,
) {
    let Some(at) = frames.current(e.placement).map(|f| f.translation) else { return };
    if let Some(change) = from_fire(e, tick, &owners.site(e.placement, at), who.culprit, who.cause) {
        changes.push(change);
    }
}

/// The tick's rule `Burning` changes: culprits kept, a rule ignition is an `Ignited` change.
#[allow(clippy::needless_pass_by_value)]
fn rule_lit(
    mut applied: MessageReader<RuleApplied>,
    frames: Res<InteractionFrames>,
    owners: Res<ChangeOwners>,
    mut culprits: ResMut<FireCulprits>,
    mut changes: ResMut<ChangeInbox>,
    mut tick_fires: ResMut<FireTick>,
) {
    for RuleApplied { tick, event: e } in applied.read() {
        let at = frames.current(e.placement).map(|f| f.translation);
        match e.output {
            RuleOutput::Added(t) if t == BURNING => {
                let who = FireCulprit { culprit: e.culprit, cause: e.source.first().copied() };
                culprits.0.insert(e.placement, who);
                let ignited = FireEvent { placement: e.placement, output: FireOutput::Ignited };
                push_change(&ignited, *tick, who, &frames, &owners, &mut changes);
                tick_fires.0.extend(at.map(|at| TickFire { event: ignited, who, at }));
            }
            RuleOutput::Removed(t) | RuleOutput::Expired(t) if t == BURNING => {
                let who = culprits.0.remove(&e.placement).unwrap_or_default();
                let doused = FireEvent { placement: e.placement, output: FireOutput::Doused };
                tick_fires.0.extend(at.map(|at| TickFire { event: doused, who, at }));
            }
            _ => {}
        }
    }
}

/// The nearest `spread` source of `pos` among `sources` within `radius` (distance, then placement).
fn nearest(sources: &[(PlacementId, Vec3)], target: PlacementId, pos: Vec3, radius: f32) -> Option<PlacementId> {
    sources
        .iter()
        .filter(|(p, s)| *p != target && s.distance(pos) <= radius)
        .min_by(|(pa, a), (pb, b)| a.distance(pos).total_cmp(&b.distance(pos)).then(pa.cmp(pb)))
        .map(|(p, _)| *p)
}

/// One fire tick (module docs, "Step", "Culprits", "Changes").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn step(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    materials: Res<InteractionMaterials>,
    frames: Res<InteractionFrames>,
    owners: Res<ChangeOwners>,
    (weather, wetness, shelter, zones): (Res<FireWeather>, Res<FireWetness>, Res<FireShelter>, Res<FireZones>),
    mut candidates: ResMut<FireCandidates>,
    mut fire: ResMut<InteractionFire>,
    mut states: ResMut<InteractionObjectStates>,
    mut tags: ResMut<InteractionStateTags>,
    mut culprits: ResMut<FireCulprits>,
    mut changes: ResMut<ChangeInbox>,
    mut tick_fires: ResMut<FireTick>,
) {
    if candidates.0.is_empty() {
        return;
    }
    let params = effective_params(&settings, fixed.as_deref());
    let now = tick.0;
    let mut bodies = Vec::with_capacity(candidates.0.len());
    let mut touched = Vec::new();
    for (&p, c) in &mut candidates.0 {
        let Some(pos) = frames.current(p).map(|f| f.translation) else { continue };
        let Some(flammable) = materials.get(c.material).and_then(|m| m.flammable) else { continue };
        let wet = match (&wetness.0, weather.indoors) {
            (Some(sample), false) => {
                let roofed = *c.sheltered.get_or_insert_with(|| shelter.0.as_ref().is_some_and(|s| s(pos)));
                if roofed { 0.0 } else { sample(pos) }
            }
            _ => 0.0,
        };
        let in_settlement =
            flammable.structure && *c.in_settlement.get_or_insert_with(|| zones.0.as_ref().is_some_and(|z| z(pos)));
        if wet >= params.rules_wet_threshold && !states.0.has(p, BURNING) {
            states.0.add(p, WET, Ttl::Secs(params.rules_wet_ttl_s), now, &params);
            touched.push(p);
        }
        bodies.push(FireBody { placement: p, pos, flammable, wetness: wet, in_settlement });
    }
    let sources: Vec<(PlacementId, Vec3)> = bodies
        .iter()
        .filter(|b| b.flammable.spread && states.0.has(b.placement, BURNING))
        .map(|b| (b.placement, b.pos))
        .collect();

    let events = step_fire(&mut fire.0, &mut states.0, &bodies, &FireEnv { wind: weather.wind }, now, &params);

    touched.extend(events.iter().map(|e| e.placement));
    touched.sort_unstable();
    touched.dedup();
    for p in touched {
        for t in [BURNING, WET, CHARRED] {
            if states.0.has(p, t) {
                tags.insert(p, t);
            } else {
                tags.remove(p, t);
            }
        }
    }

    let pos_of = |p: PlacementId| bodies.iter().find(|b| b.placement == p).map(|b| b.pos);
    for e in &events {
        let who = match e.output {
            FireOutput::Ignited => {
                let source = pos_of(e.placement)
                    .and_then(|pos| nearest(&sources, e.placement, pos, params.fire_spread_radius_m));
                let who = source.and_then(|s| culprits.0.get(&s).copied());
                match who {
                    Some(w) => culprits.0.insert(e.placement, w),
                    None => culprits.0.remove(&e.placement),
                };
                who.unwrap_or_default()
            }
            FireOutput::Burnt { .. } | FireOutput::Doused => culprits.0.remove(&e.placement).unwrap_or_default(),
        };
        push_change(e, now, who, &frames, &owners, &mut changes);
        if let Some(at) = pos_of(e.placement) {
            tick_fires.0.push(TickFire { event: *e, who, at });
        }
    }
}

/// The fire lights per region: the lit keep their slot up to `max` (earliest first), free slots go
/// to the earliest-started, then placement (module docs, "Outputs").
fn assign_lights(objects: &mut BTreeMap<PlacementId, BurningObject>, max: usize) {
    let mut regions: BTreeMap<(i32, i32), Vec<(Tick, PlacementId)>> = BTreeMap::new();
    for (p, o) in &*objects {
        regions.entry(o.region).or_default().push((o.since, *p));
    }
    for list in regions.values_mut() {
        list.sort_unstable();
        let mut lit = 0;
        for (_, p) in list.iter() {
            let Some(o) = objects.get_mut(p) else { continue };
            if o.light {
                if lit < max {
                    lit += 1;
                } else {
                    o.light = false;
                }
            }
        }
        for (_, p) in list.iter() {
            if lit >= max {
                break;
            }
            let Some(o) = objects.get_mut(p) else { continue };
            if !o.light {
                o.light = true;
                lit += 1;
            }
        }
    }
}

/// The tick's fire outputs (module docs, "Outputs").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn outputs(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    (frames, states, fire): (Res<InteractionFrames>, Res<InteractionObjectStates>, Res<InteractionFire>),
    (candidates, culprits): (Res<FireCandidates>, Res<FireCulprits>),
    (destructibles, lights): (Res<Destructibles>, Res<Lights>),
    mut burning: ResMut<BurningObjects>,
    mut gameplay: ResMut<GameplayLights>,
    mut tick_fires: ResMut<FireTick>,
    mut changed: MessageWriter<FireChanged>,
    mut damage: MessageWriter<PropDamage>,
    mut forced: MessageWriter<PropStage>,
) {
    if tick_fires.0.is_empty() && burning.0.is_empty() && !candidates.0.keys().any(|p| states.0.has(*p, BURNING)) {
        return;
    }
    let params = effective_params(&settings, fixed.as_deref());
    let now = tick.0;
    for TickFire { event: e, who, at } in tick_fires.0.drain(..) {
        changed.write(FireChanged { tick: now, placement: e.placement, output: e.output, at });
        let FireOutput::Burnt { stage } = e.output else { continue };
        let Some(def) = destructibles.get(e.placement).and_then(|d| d.destructible()) else { continue };
        let last = def.stages.len().checked_sub(1).and_then(|i| u8::try_from(i).ok());
        if let Some(stage) = stage.or(last) {
            forced.write(PropStage { placement: e.placement, stage, kind: DAMAGE_FIRE, culprit: who.culprit, point: at });
        }
    }

    let mut next = BTreeMap::new();
    for &p in candidates.0.keys() {
        if !states.0.has(p, BURNING) {
            continue;
        }
        let Some(at) = frames.current(p).map(|f| f.translation) else { continue };
        let since = fire.0.burning_since(p).unwrap_or(now);
        let light = burning.0.get(&p).is_some_and(|o| o.light);
        next.insert(p, BurningObject { at, since, region: fire_region(at, &params), light });
    }
    assign_lights(&mut next, usize::from(params.fire_lights_max));

    let amount = params.fire_damage_per_s / params.tick_hz.max(f32::EPSILON);
    for (&p, o) in &next {
        let culprit = culprits.0.get(&p).and_then(|w| w.culprit);
        if amount > 0.0 && destructibles.get(p).is_some_and(|d| !d.gone && d.destructible().is_some()) {
            damage.write(PropDamage { placement: p, amount, kind: DAMAGE_FIRE, point: o.at, dir: Vec3::Y, culprit });
        }
        if lights.get(p).is_none() {
            let radius_m = params.fire_light_radius_m;
            gameplay.insert(GameplayLight { placement: p, pos: o.at, radius_m, intensity: 1.0, lit: true });
        }
    }
    for p in burning.0.keys() {
        if !next.contains_key(p) && lights.get(*p).is_none() {
            gameplay.remove(*p);
        }
    }
    burning.0 = next;
}
