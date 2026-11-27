//! Rules shell (`docs/specs/interaction-world-rules.md` "Rules and fire shell", its rules shell and
//! "Order"), slice `IN39a2`: the host's causes into the core's [`step_rules`] on the fixed
//! tick, and its events routed to the systems that own their outputs. The table, the states and the
//! maths are the core's ([`Rules`], [`ObjectStates`], [`RuleQueue`]).
//!
//! - **Causes** ([`RuleCause`], any writer `.before(RuleSystems)`: hits, spell areas, water contact,
//!   a torch): the target's material from [`InteractionDefs`] (an unindexed placement is dropped), its
//!   host-owned tags ([`HOST_OWNED`]) now set from [`InteractionStateTags`], into the core queue at
//!   the tick. A deferred cause (past `interact.rules.evals_per_tick`) keeps that snapshot.
//! - **Step** ([`RuleSystems`]): [`step_rules`] on [`InteractionObjectStates`] with the tick's params
//!   (`tick_hz` from `Time<Fixed>`); every event is one [`RuleApplied`] (culprit and source inside).
//!   Added / Removed / Expired of a rule tag (not host-owned) mirror into [`InteractionStateTags`], so
//!   the verbs' `requires` gate on `Burning` or `Wet`.
//! - **Routes**, same set, in event order: Added / Removed of [`LIT`] on a registered light set its
//!   state (`LightState::set`: an event only on a change), its tag and its [`GameplayLights`] entry,
//!   and write [`LightChanged`] with the source's first tag (id order, a stable but arbitrary pick)
//!   as the rule cause; on anything else nothing (a `Lit` tag lives only in the light registry).
//!   The rest need the placement's frame ([`InteractionFrames::current`]; gone: dropped), `at` its
//!   translation: `Damage` -> one [`PropDamage`] (`dir` up); `Stage(i)` -> one [`PropStage`];
//!   `Change(k)` -> one [`WorldChange`] into [`ChangeInbox`] (owner and home from [`ChangeOwners`],
//!   cause the source's first tag); `Noise(k)` -> [`NoiseInbox`] at the material's impact reach
//!   ([`NoiseCase::Impact`]), the culprit as maker, not revealed; `Effect(e)` -> [`RuleEffect`] for
//!   the host's presentation. The damage and stage kind is the source's first tag ([`RULE_SOURCE`]
//!   for a cause without tags).
//!
//! **Order** (one chain in `FixedUpdate`): `InteractionSystems::Apply` -> [`RuleSystems`] ->
//! [`FireSystems`] (the fire step, `fire` module) -> `DestroySystems::Damage` (reads the tick's rule
//! [`PropDamage`] and [`PropStage`]) -> `Release` -> `WorldChangeSystems` (drains [`ChangeInbox`]
//! with the door, light and break changes, one stream folded per tick).
//!
//! Not yet: unloaded placements keep their states (`IN39d` drops them after their save write); door
//! tags as host-owned tags (a later row routes them to the door registry).

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_interaction::{
    Cause, LIT, Noise, NoiseCase, ObjectStates, RuleEvent, RuleOutput, RuleQueue, Rules, TagId, Tick, UserKey,
    WorldChange, step_rules,
};
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::{
    ChangeInbox, ChangeOwners, DestroySystems, GameplayLights, InteractionDefs, InteractionFrames, InteractionMaterials,
    InteractionSettings, InteractionStateTags, InteractionSystems, InteractionTick, LightChanged, Lights, NoiseInbox,
    PropDamage, PropStage, effective_params, light, source_key,
};

/// The state tags the host's registries own (never stored in [`ObjectStates`]): a light's [`LIT`].
pub const HOST_OWNED: &[TagId] = &[LIT];

/// The damage or stage kind of a rule whose cause has no source tag.
pub const RULE_SOURCE: TagId = TagId::of("rule");

/// The rules table (`sf.interact.rule`, record order), filled by the host.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionRules(pub Rules);

/// The rule states of the resident placements (`Burning`, `Wet`, `Charred`).
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionObjectStates(pub ObjectStates);

/// The causes waiting for evaluation (the core queue, oldest first).
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionRuleQueue(pub RuleQueue);

/// One cause on one placement this tick; ordered `.before(RuleSystems)`.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct RuleCause {
    pub placement: PlacementId,
    /// The cause's tags: element, tool, projectile, impact, spell.
    pub source: SmallVec<[TagId; 4]>,
    /// Who made it; `None` = the world (rain, a rule).
    pub culprit: Option<UserKey>,
}

/// One rules event of the tick (the core's [`RuleEvent`]).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct RuleApplied {
    pub tick: Tick,
    pub event: RuleEvent,
}

/// A rule's effect to play at the placement (the host's presentation, seeded per client).
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct RuleEffect {
    pub tick: Tick,
    pub placement: PlacementId,
    pub effect: TagId,
    /// The placement's position, world metres.
    pub at: Vec3,
}

/// Where the tick's causes are gathered, stepped and routed: `FixedUpdate`, after
/// `InteractionSystems::Apply`, before [`FireSystems`].
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RuleSystems;

/// Where fire steps (`IN39b`, `fire` module), between [`RuleSystems`] and `DestroySystems::Damage`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FireSystems;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<InteractionRules>()
        .init_resource::<InteractionObjectStates>()
        .init_resource::<InteractionRuleQueue>()
        .add_message::<RuleCause>()
        .add_message::<RuleApplied>()
        .add_message::<RuleEffect>()
        .configure_sets(
            FixedUpdate,
            (RuleSystems, FireSystems, DestroySystems::Damage).chain().after(InteractionSystems::Apply),
        )
        .add_systems(FixedUpdate, (gather, step, route).chain().in_set(RuleSystems));
}

/// The placement's host-owned tags now set.
fn host_tags(tags: &InteractionStateTags, placement: PlacementId) -> SmallVec<[TagId; 4]> {
    tags.tags(placement).iter().copied().filter(|t| HOST_OWNED.contains(t)).collect()
}

/// The tick's [`RuleCause`]s into the core queue (module docs, "Causes").
#[allow(clippy::needless_pass_by_value)]
fn gather(
    mut causes: MessageReader<RuleCause>,
    defs: Res<InteractionDefs>,
    tags: Res<InteractionStateTags>,
    tick: Res<InteractionTick>,
    mut queue: ResMut<InteractionRuleQueue>,
) {
    for c in causes.read() {
        let Some(def) = defs.get(c.placement) else { continue };
        queue.0.push(
            tick.0,
            Cause {
                placement: c.placement,
                material: def.material,
                source: c.source.clone(),
                culprit: c.culprit,
                host: host_tags(&tags, c.placement),
            },
        );
    }
}

/// One core rules tick; its events out, rule tags mirrored (module docs, "Step").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn step(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    rules: Res<InteractionRules>,
    mut states: ResMut<InteractionObjectStates>,
    mut queue: ResMut<InteractionRuleQueue>,
    mut tags: ResMut<InteractionStateTags>,
    mut out: MessageWriter<RuleApplied>,
) {
    if queue.0.is_empty() && states.0.is_empty() {
        return;
    }
    let params = effective_params(&settings, fixed.as_deref());
    let events = step_rules(&mut states.0, &mut queue.0, &rules.0, tick.0, &params, HOST_OWNED);
    for e in &events {
        match e.output {
            RuleOutput::Added(t) if !HOST_OWNED.contains(&t) => tags.insert(e.placement, t),
            RuleOutput::Removed(t) | RuleOutput::Expired(t) if !HOST_OWNED.contains(&t) => tags.remove(e.placement, t),
            _ => {}
        }
    }
    out.write_batch(events.into_iter().map(|event| RuleApplied { tick: tick.0, event }));
}

/// The tick's [`RuleApplied`] into the owners of their outputs (module docs, "Routes").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn route(
    mut applied: MessageReader<RuleApplied>,
    settings: Res<InteractionSettings>,
    defs: Res<InteractionDefs>,
    frames: Res<InteractionFrames>,
    owners: Res<ChangeOwners>,
    materials: Res<InteractionMaterials>,
    mut lights: ResMut<Lights>,
    mut tags: ResMut<InteractionStateTags>,
    mut gameplay: ResMut<GameplayLights>,
    mut changes: ResMut<ChangeInbox>,
    mut noises: ResMut<NoiseInbox>,
    mut light_changed: MessageWriter<LightChanged>,
    mut damage: MessageWriter<PropDamage>,
    mut stages: MessageWriter<PropStage>,
    mut effects: MessageWriter<RuleEffect>,
) {
    let p = settings.0;
    for RuleApplied { tick, event: e } in applied.read() {
        let cause = e.source.first().copied();
        let kind = cause.unwrap_or(RULE_SOURCE);
        let lit = match e.output {
            RuleOutput::Added(t) if t == LIT => Some(true),
            RuleOutput::Removed(t) if t == LIT => Some(false),
            _ => None,
        };
        if let Some(lit) = lit {
            let set = light::set_lit(e.placement, lit, e.culprit, *tick, &mut lights, &mut tags, &mut gameplay);
            if let Some(changed) = set {
                light_changed.write(LightChanged(changed, cause));
            }
            continue;
        }
        let Some(at) = frames.current(e.placement).map(|f| f.translation) else { continue };
        match e.output {
            RuleOutput::Damage { kind: k, amount } => {
                damage.write(PropDamage { placement: e.placement, amount, kind: k, point: at, dir: Vec3::Y, culprit: e.culprit });
            }
            RuleOutput::Stage(stage) => {
                stages.write(PropStage { placement: e.placement, stage, kind, culprit: e.culprit, point: at });
            }
            RuleOutput::Change(k) => {
                let site = owners.site(e.placement, at);
                changes.push(WorldChange {
                    tick: *tick,
                    kind: k,
                    object: e.placement,
                    at,
                    culprit: e.culprit,
                    owner: site.owner,
                    home: site.home,
                    cause,
                    detail: sf_interaction::ChangeDetail::None,
                });
            }
            RuleOutput::Noise(k) => {
                let material = defs.get(e.placement).and_then(|d| d.material);
                noises.push(Noise {
                    source: source_key(e.placement),
                    at,
                    kind: k,
                    loudness_m: materials.reach_m(material, &p),
                    maker: e.culprit,
                    reveals_maker: false,
                    case: NoiseCase::Impact,
                });
            }
            RuleOutput::Effect(effect) => {
                effects.write(RuleEffect { tick: *tick, placement: e.placement, effect, at });
            }
            RuleOutput::Added(_) | RuleOutput::Removed(_) | RuleOutput::Expired(_) => {}
        }
    }
}
