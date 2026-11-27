//! Destruction shell, tier 1 (`docs/specs/interaction-destruction.md` "Shell and game"), slice
//! `IN1b`: the resident destructibles, damage in, the host's paced break release and the removal of
//! a destroyed object. The maths is the core's ([`apply_damage`], [`BreakQueue`], [`break_seed`]);
//! every model, item, sound and save byte is the host's (pattern: the light shell, [`crate::light`]).
//!
//! - **Registry** ([`Destructibles`], `FixedUpdate`, in `InteractionSystems::Apply` after the light
//!   registry): an [`IndexChange::Insert`] whose def has a `destructible` registers it
//!   ([`DestructState::intact`]); a re-insert keeps the state, clamped to the new def (a stage swap
//!   re-sends the broken model's def); an insert without one, or a removal, drops it; a move
//!   refreshes its frame. Then, after all index changes, a [`DestructRest`] (a restored save or a
//!   streamed-in cell) sets the state, clamped to the def (stage at most the stage count, health in
//!   `0 ..= def.health`; `gone` if a reached stage removes). No event: the host that wrote it shows
//!   its own stage.
//! - **Damage** ([`DestroySystems::Damage`], after `Apply`): the tick's [`PropDamage`]s sorted by a
//!   total key (placement, kind, culprit, then the bits of amount, point and dir: writer systems run
//!   unordered, so send order is not deterministic). Each is scaled by [`DestructionRules`]
//!   (`(kind, material) -> factor`, default 1) and applied at once: health is authoritative now.
//!   Dropped: an unknown placement, a `gone` one, a factor `<= 0`, and every kind but
//!   [`DAMAGE_SCRIPT`] on an `ignore_external` def. Every crossed stage queues one break (the hit's
//!   tick, its seed, the overkill on the last crossed stage only); a crossed stage without a model
//!   sets `gone`. One [`DestructChanged`] per placement whose state changed this tick (its last
//!   state, by placement): the host's save write.
//! - **Release** ([`DestroySystems::Release`]): at most `interact.debris.breaks_per_tick` queued
//!   breaks, oldest first, each one [`Destroyed`]. A removing break on a still registered placement
//!   also writes [`IndexChange::Remove`] with `OwnerDestroyed` (its claims end in the next tick's
//!   `Apply`); one whose placement left meanwhile (its cell unloaded) still releases, without a
//!   removal: its health was saved at the hit, the host applies it to its overlay alone.
//!
//! The cap delays presentation and removal, never health: the state saved is always the true one.

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_interaction::{
    BreakQueue, DAMAGE_SCRIPT, DestructState, DestructibleDef, Frame, InteractableDef, InvalidReason, TagId, UserKey,
    apply_damage, break_seed,
};
use sf_world::PlacementId;

use crate::{IndexChange, InteractionSettings, InteractionTick};

/// One resident destructible object.
#[derive(Clone, Debug)]
pub struct Destructible {
    pub def: Arc<InteractableDef>,
    /// The object's world frame, as last inserted or moved (area sources: fire, explosion radii).
    pub object: Frame,
    pub state: DestructState,
    /// A reached stage removes the object: later hits drop.
    pub gone: bool,
}

impl Destructible {
    /// The def's destruction data (always present for a registered object).
    #[must_use]
    pub fn destructible(&self) -> Option<&DestructibleDef> {
        self.def.destructible.as_ref()
    }
}

/// The resident destructibles, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct Destructibles(BTreeMap<PlacementId, Destructible>);

impl Destructibles {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&Destructible> {
        self.0.get(&placement)
    }

    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &Destructible)> {
        self.0.iter().map(|(p, d)| (*p, d))
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

/// The adapter's damage scaling: `(kind, material) -> factor`, 1 when absent (the framework never
/// names a stat). A def without a material takes the factor 1.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct DestructionRules(BTreeMap<(TagId, TagId), f32>);

impl DestructionRules {
    /// Sets the factor of `kind` on `material`.
    pub fn set(&mut self, kind: TagId, material: TagId, factor: f32) {
        self.0.insert((kind, material), factor);
    }

    /// The factor of `kind` on `material` (1 when absent or without a material).
    #[must_use]
    pub fn factor(&self, kind: TagId, material: Option<TagId>) -> f32 {
        material.and_then(|m| self.0.get(&(kind, m)).copied()).unwrap_or(1.0)
    }
}

/// Damage to one placed object, from any source (combat, impacts, rules, a dev key); ordered
/// `.before(DestroySystems::Damage)`. `amount` is raw: [`DestructionRules`] scales it.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct PropDamage {
    pub placement: PlacementId,
    pub amount: f32,
    /// The source's damage tag (adapter data: blunt, blade, arrow, hand, fire, impact).
    pub kind: TagId,
    /// Where it landed, world metres (interaction frames).
    pub point: Vec3,
    pub dir: Vec3,
    /// Who; `None` = a rule (fire) or a script.
    pub culprit: Option<UserKey>,
}

/// Puts a registered destructible in this saved state (a restored save or a streamed-in cell),
/// clamped to its def. No event.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct DestructRest {
    pub placement: PlacementId,
    pub state: DestructState,
}

/// A destructible's state changed this tick (its last state): the host writes its rest bytes.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct DestructChanged {
    pub placement: PlacementId,
    pub state: DestructState,
}

/// One released break (the core's event): the host's stage hook, debris, crime and noise read it.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct Destroyed(pub sf_interaction::Destroyed);

/// The breaks waiting for their release (`interact.debris.breaks_per_tick` per tick).
#[derive(Resource, Clone, Debug)]
pub struct PendingBreaks(pub BreakQueue<sf_interaction::Destroyed>);

impl Default for PendingBreaks {
    fn default() -> Self {
        Self(BreakQueue::new())
    }
}

/// Where damage applies and breaks release: `FixedUpdate`, chained after `InteractionSystems::Apply`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DestroySystems {
    /// The tick's [`PropDamage`] into health, queued breaks and [`DestructChanged`].
    Damage,
    /// Queued breaks into [`Destroyed`] (and the removal of a destroyed object).
    Release,
}

/// The destruction resources, messages and the Damage / Release systems (the registry runs in the
/// Apply chain, `lib.rs`).
pub(crate) fn build(app: &mut bevy_app::App) {
    use bevy_app::FixedUpdate;
    app.init_resource::<Destructibles>()
        .init_resource::<DestructionRules>()
        .init_resource::<PendingBreaks>()
        .add_message::<PropDamage>()
        .add_message::<DestructRest>()
        .add_message::<DestructChanged>()
        .add_message::<Destroyed>()
        .configure_sets(
            FixedUpdate,
            (DestroySystems::Damage, DestroySystems::Release).chain().after(crate::InteractionSystems::Apply),
        )
        .add_systems(FixedUpdate, damage.in_set(DestroySystems::Damage))
        .add_systems(FixedUpdate, release.in_set(DestroySystems::Release));
    // Physics contacts as damage in (IN4b).
    #[cfg(feature = "physics")]
    crate::impact::build(app);
}

/// `state` clamped to `def`, and whether a reached stage removes the object.
fn clamp(def: &DestructibleDef, state: DestructState) -> (DestructState, bool) {
    let count = u8::try_from(def.stages.len()).unwrap_or(u8::MAX);
    let reached = state.stage.min(count);
    let health = state.health.clamp(0.0, def.health.max(0.0));
    let gone = def.stages[..usize::from(reached)].iter().any(|s| s.model.is_none());
    (DestructState { health, stage: reached }, gone)
}

/// The registry follows the tick's index changes, then its rest states (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn register(
    mut changes: MessageReader<IndexChange>,
    mut rests: MessageReader<DestructRest>,
    mut registry: ResMut<Destructibles>,
) {
    for change in changes.read() {
        match change {
            IndexChange::Insert { placement, def, object, .. } => {
                let Some(d) = def.destructible.as_ref() else {
                    registry.0.remove(placement);
                    continue;
                };
                let (state, gone) = registry
                    .0
                    .get(placement)
                    .map_or_else(|| (DestructState::intact(d), false), |old| clamp(d, old.state));
                registry.0.insert(*placement, Destructible { def: Arc::clone(def), object: *object, state, gone });
            }
            IndexChange::Move { placement, object } => {
                if let Some(d) = registry.0.get_mut(placement) {
                    d.object = *object;
                }
            }
            IndexChange::Remove { placement, .. } => {
                registry.0.remove(placement);
            }
        }
    }
    for rest in rests.read() {
        let Some(d) = registry.0.get_mut(&rest.placement) else { continue };
        let Some(def) = d.def.destructible.as_ref() else { continue };
        (d.state, d.gone) = clamp(def, rest.state);
    }
}

/// A total order over a tick's hits (module docs): f32 fields by their bits.
fn damage_key(d: &PropDamage) -> (PlacementId, TagId, Option<UserKey>, u32, [u32; 3], [u32; 3]) {
    let bits = |v: Vec3| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
    (d.placement, d.kind, d.culprit, d.amount.to_bits(), bits(d.point), bits(d.dir))
}

/// The tick's hits into health, queued breaks and one [`DestructChanged`] per changed placement.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn damage(
    mut hits: MessageReader<PropDamage>,
    rules: Res<DestructionRules>,
    tick: Res<InteractionTick>,
    mut registry: ResMut<Destructibles>,
    mut pending: ResMut<PendingBreaks>,
    mut changed: MessageWriter<DestructChanged>,
) {
    let mut tick_hits: Vec<PropDamage> = hits.read().copied().collect();
    if tick_hits.is_empty() {
        return;
    }
    tick_hits.sort_by_key(damage_key);
    let now = tick.0;
    let mut last: BTreeMap<PlacementId, DestructState> = BTreeMap::new();
    for h in tick_hits {
        let Some(d) = registry.0.get_mut(&h.placement) else { continue };
        if d.gone {
            continue;
        }
        let Some(def) = d.def.destructible.as_ref() else { continue };
        if def.ignore_external && h.kind != DAMAGE_SCRIPT {
            continue;
        }
        let factor = rules.factor(h.kind, d.def.material);
        if factor.is_nan() || factor <= 0.0 {
            continue;
        }
        let hit = apply_damage(def, d.state, h.amount * factor);
        if hit.state == d.state {
            continue;
        }
        let final_stage = hit.crossed.end;
        for stage in hit.crossed.clone() {
            let removes = def.stages[usize::from(stage)].model.is_none();
            d.gone |= removes;
            pending.0.push(sf_interaction::Destroyed {
                tick: now,
                placement: h.placement,
                stage,
                point: h.point,
                dir: h.dir,
                seed: break_seed(h.placement, now, stage),
                culprit: h.culprit,
                overkill: if stage + 1 == final_stage { hit.overkill } else { 0.0 },
                removes,
            });
        }
        d.state = hit.state;
        last.insert(h.placement, hit.state);
    }
    changed.write_batch(last.into_iter().map(|(placement, state)| DestructChanged { placement, state }));
}

/// This tick's share of the queued breaks into [`Destroyed`] (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn release(
    settings: Res<InteractionSettings>,
    registry: Res<Destructibles>,
    mut pending: ResMut<PendingBreaks>,
    mut out: MessageWriter<Destroyed>,
    mut index: MessageWriter<IndexChange>,
) {
    for b in pending.0.take_tick(settings.0.debris_breaks_per_tick) {
        out.write(Destroyed(b));
        if b.removes && registry.0.contains_key(&b.placement) {
            index.write(IndexChange::Remove { placement: b.placement, reason: InvalidReason::OwnerDestroyed });
        }
    }
}
