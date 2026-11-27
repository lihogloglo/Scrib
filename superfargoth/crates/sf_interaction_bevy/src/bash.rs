//! Bash shell (`docs/specs/interaction-destruction.md` "Bashing (IN37)", slice `IN37b`): a press of
//! the `bash` verb becomes one blow landing after the swing's wind-up; the core [`bash`] gives its
//! result. Crime owns the lock and reads [`Bashed`] (LW6); this shell owns the blow.
//!
//! - **Inputs, host-filled:** [`InteractionBashRules`] (the core's [`BashRules`]; default neutral)
//!   and [`BashBlows`], the user's blow now (unset or `None`: the use does nothing).
//! - **Press** (`Update`, [`BashSystems::Press`], after `ActivationSystems::Default`): a
//!   [`UseApproved`] with verb `bash` on a placement whose def offers `bash`, by the
//!   [`InteractionUser`] whose `actor` is the activation's (none: nothing), queues one
//!   [`PendingBlow`] landing `ceil(interact.bash.windup_s x tick_hz)` ticks on, the blow read at the
//!   press. A user with a blow pending, or inside `interact.bash.recover_s` of their last landed one,
//!   is dropped.
//! - **Hits** (`FixedUpdate`, [`BashSystems::Hits`]): a [`PropDamage`] with a culprit, not
//!   `DAMAGE_IMPACT`, on a placement that is not a registered destructible and whose def offers
//!   `bash` with its `requires` met, is a blow landing this tick (`force` = the amount; reach not
//!   checked: the swing connected).
//! - **Land** (`FixedUpdate`, [`BashSystems::Land`]): due blows in (land tick, user, placement)
//!   order. Dropped when the target left (no def or frame), its `bash` `requires` fail now, or (a
//!   press) the user left or their [`ActionBody`] feet are over `interact.bash.reach_m` from the
//!   point (a user with no body is in reach). Else one [`Bashed`]; a registered destructible also
//!   takes [`PropDamage`] (amount the force, same kind and culprit, `dir` from the feet to the point);
//!   one [`Noise`] ([`NoiseCase::Bash`], `interact.noise.bash_reach_m` x the material's ratio, the
//!   user as maker, revealed) into [`NoiseInbox`]; [`from_bashed`] into [`ChangeInbox`]. The point is
//!   the def's doorway when it has a door slot, else the object's origin.
//!
//! **Order** (`FixedUpdate`): `FireSystems` -> `Hits` -> `Land` -> `DestroySystems::Damage`: two
//! systems, as one cannot both read and write `PropDamage`; Land's hits name registered
//! destructibles, which `Hits` skips (no loop). Combat writes prop hits `.before(BashSystems::Hits)`.
//!
//! Not saved: a pending blow is under a second, lost on save or unload.

use std::collections::BTreeMap;

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use glam::Vec3;
use sf_action_bevy::ActionBody;
use sf_interaction::{
    BashRules, Blow, DAMAGE_IMPACT, InteractableDef, Noise, NoiseCase, TagId, Tick, UserKey, VERB_BASH, bash,
    bash_seed, from_bashed,
};
use sf_world::PlacementId;

use crate::seat::feet_of;
use crate::{
    ActivationSystems, ChangeInbox, ChangeOwners, DestroySystems, Destructibles, FireSystems, InteractionDefs,
    InteractionFrames, InteractionMaterials, InteractionSettings, InteractionStateTags, InteractionTick,
    InteractionUser, NoiseInbox, PropDamage, UseApproved, effective_params, source_key,
};

/// The kind of a blow's noise.
pub const NOISE_BASH: TagId = TagId::of("noise.bash");

/// The bash rules (the adapter's record), filled by the host; the default is neutral.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionBashRules(pub BashRules);

/// The [`BashBlows`] closure: the user's blow now (strength x readied weapon), `None` = no blow.
pub type BlowOf = dyn Fn(UserKey) -> Option<Blow> + Send + Sync;

/// The host's blow of each user (module docs, "Inputs"); the default gives none.
#[derive(Resource)]
pub struct BashBlows(pub Box<BlowOf>);

impl Default for BashBlows {
    fn default() -> Self {
        Self(Box::new(|_| None))
    }
}

/// One blow waiting to land.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingBlow {
    pub user: UserKey,
    pub placement: PlacementId,
    pub blow: Blow,
    /// The tick it lands at.
    pub land: Tick,
    /// A press's swing (reach checked at land); `false` = a hit that already connected.
    pub pressed: bool,
}

/// The blows waiting to land and each user's last landed tick.
#[derive(Resource, Clone, Debug, Default)]
pub struct BashPending {
    blows: Vec<PendingBlow>,
    landed: BTreeMap<UserKey, Tick>,
}

impl BashPending {
    /// The blows waiting, in queue order.
    #[must_use]
    pub fn blows(&self) -> &[PendingBlow] {
        &self.blows
    }
}

/// One landed blow (the core's [`sf_interaction::Bashed`]): crime, the game's costs, presentation.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct Bashed(pub sf_interaction::Bashed);

/// Where blows are queued (`Update`) and landed (`FixedUpdate`).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BashSystems {
    /// Approved `bash` uses into pending blows, after `ActivationSystems::Default`.
    Press,
    /// Prop hits on bashable non-destructibles into blows landing now.
    Hits,
    /// Due blows into [`Bashed`], destructible damage, noise and change.
    Land,
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<InteractionBashRules>()
        .init_resource::<BashBlows>()
        .init_resource::<BashPending>()
        .add_message::<Bashed>()
        .configure_sets(Update, BashSystems::Press.after(ActivationSystems::Default))
        .configure_sets(
            FixedUpdate,
            (FireSystems, BashSystems::Hits, BashSystems::Land, DestroySystems::Damage).chain(),
        )
        .add_systems(Update, press.in_set(BashSystems::Press))
        .add_systems(FixedUpdate, hits.in_set(BashSystems::Hits))
        .add_systems(FixedUpdate, land.in_set(BashSystems::Land));
}

/// Whether `def` offers `bash` with its `requires` met by `tags` (`None`: not offered).
fn bash_open(def: &InteractableDef, tags: &[TagId]) -> Option<bool> {
    def.verbs.iter().find(|v| v.verb == VERB_BASH).map(|v| v.requires.matches(tags))
}

/// The tick's approved `bash` uses into pending blows (module docs, "Press").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn press(
    mut approved: MessageReader<UseApproved>,
    users: Query<&InteractionUser>,
    defs: Res<InteractionDefs>,
    blows: Res<BashBlows>,
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    mut pending: ResMut<BashPending>,
) {
    let params = effective_params(&settings, fixed.as_deref());
    for a in approved.read() {
        if a.verb != Some(VERB_BASH) {
            continue;
        }
        let placement = PlacementId(a.activation.target.0);
        let Some(def) = defs.get(placement) else { continue };
        if bash_open(def, &[]).is_none() {
            continue;
        }
        let Some(user) = users.iter().find(|u| u.actor == a.activation.actor).map(|u| u.user) else { continue };
        let busy = pending.blows.iter().any(|b| b.user == user && b.pressed);
        let recovering =
            pending.landed.get(&user).is_some_and(|t| tick.0.0 < t.0.saturating_add(params.bash_recover_ticks()));
        if busy || recovering {
            continue;
        }
        let Some(blow) = (blows.0)(user) else { continue };
        let land = tick.0.plus(params.bash_windup_ticks());
        pending.blows.push(PendingBlow { user, placement, blow, land, pressed: true });
    }
}

/// The tick's culprit prop hits on bashable non-destructibles (module docs, "Hits").
#[allow(clippy::needless_pass_by_value)]
fn hits(
    mut damage: MessageReader<PropDamage>,
    registry: Res<Destructibles>,
    defs: Res<InteractionDefs>,
    tags: Res<InteractionStateTags>,
    rules: Res<InteractionBashRules>,
    tick: Res<InteractionTick>,
    mut pending: ResMut<BashPending>,
) {
    for d in damage.read() {
        let Some(user) = d.culprit else { continue };
        if d.kind == DAMAGE_IMPACT || registry.get(d.placement).is_some() {
            continue;
        }
        let Some(def) = defs.get(d.placement) else { continue };
        if bash_open(def, tags.tags(d.placement)) != Some(true) {
            continue;
        }
        let blow = Blow { force: d.amount, kind: d.kind, bare_handed: Some(d.kind) == rules.0.bare_kind };
        pending.blows.push(PendingBlow { user, placement: d.placement, blow, land: tick.0, pressed: false });
    }
}

/// The tick's due blows, landed or dropped (module docs, "Land").
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn land(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    tick: Res<InteractionTick>,
    defs: Res<InteractionDefs>,
    tags: Res<InteractionStateTags>,
    frames: Res<InteractionFrames>,
    registry: Res<Destructibles>,
    rules: Res<InteractionBashRules>,
    materials: Res<InteractionMaterials>,
    owners: Res<ChangeOwners>,
    users: Query<(&InteractionUser, Option<&ActionBody>)>,
    mut pending: ResMut<BashPending>,
    mut noises: ResMut<NoiseInbox>,
    mut changes: ResMut<ChangeInbox>,
    mut bashed: MessageWriter<Bashed>,
    mut damage: MessageWriter<PropDamage>,
) {
    let now = tick.0;
    if pending.blows.iter().all(|b| b.land > now) {
        return;
    }
    let p = effective_params(&settings, fixed.as_deref());
    let recover = p.bash_recover_ticks();
    pending.landed.retain(|_, t| now.0 < t.0.saturating_add(recover));
    let (mut due, waiting): (Vec<_>, Vec<_>) = pending.blows.drain(..).partition(|b| b.land <= now);
    pending.blows = waiting;
    due.sort_by_key(|b| (b.land, b.user, b.placement));
    for b in due {
        let Some(def) = defs.get(b.placement) else { continue };
        let Some(object) = frames.current(b.placement) else { continue };
        if bash_open(def, tags.tags(b.placement)) != Some(true) {
            continue;
        }
        let point = def
            .door()
            .map_or(object.translation, |(_, slot, _)| object.translation + object.rotation * slot.local.translation);
        let feet = match users.iter().find(|(u, _)| u.user == b.user) {
            Some((_, body)) => body.map(feet_of),
            None if b.pressed => continue,
            None => None,
        };
        if b.pressed && feet.is_some_and(|f| f.distance(point) > p.bash_reach_m) {
            continue;
        }
        let result = bash(&rules.0, &b.blow, def.material);
        let event = sf_interaction::Bashed {
            tick: now,
            placement: b.placement,
            culprit: b.user,
            kind: b.blow.kind,
            force: b.blow.force,
            result,
            point,
            seed: bash_seed(b.placement, b.user, now),
        };
        if registry.get(b.placement).is_some() {
            let dir = feet.map_or(Vec3::ZERO, |f| (point - f).normalize_or_zero());
            damage.write(PropDamage {
                placement: b.placement,
                amount: b.blow.force,
                kind: b.blow.kind,
                point,
                dir,
                culprit: Some(b.user),
            });
        }
        noises.push(Noise {
            source: source_key(b.placement),
            at: point,
            kind: NOISE_BASH,
            loudness_m: p.noise_bash_reach_m * materials.ratio(def.material, &p),
            maker: Some(b.user),
            reveals_maker: true,
            case: NoiseCase::Bash,
        });
        changes.push(from_bashed(&event, &owners.site(b.placement, object.translation)));
        pending.landed.insert(b.user, now);
        bashed.write(Bashed(event));
    }
}
