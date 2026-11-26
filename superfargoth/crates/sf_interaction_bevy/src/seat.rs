//! The default for pose slots (`IN2b2`, `docs/specs/interaction.md` "Using a slot", steps 1-4): an
//! approved use of a seat, bench end or bed becomes a claim, the claimed slot is offered to the
//! action layer (`sf_action_bevy::SeatAim`), entering occupies it and standing up, a refused enter
//! or a lost claim releases it. One path for the player and NPCs.
//!
//! - **Users.** The host puts an [`InteractionUser`] on every character entity that uses slots
//!   (an `sf_action_bevy` character: [`ActionCharacter`] + [`ActionBody`]): its claim key, its
//!   script handle (what `Activated.actor` names), its claim priority and its tags.
//! - **Targets.** The action layer names a seat by [`seat_target`]: the placement id's low 64 bits
//!   as a `TargetId` (no other placement-to-target convention exists; the shell only compares it
//!   with the seat it offered that user, so a collision between two placements is never read).
//! - **Space.** Interaction frames are world metres, the action body's `WorldPos` narrowed to f32
//!   (`WorldPos::to_f64`); the senses are relative to the feet, world axes, world yaw (yaw 0 faces
//!   +Z, the slot frame's forward is its +Z).
//!
//! Each step, with its schedule:
//! 1. **Request** (`Update`, [`SeatSystems::Request`], after `ActivationSystems::Default`): a
//!    [`UseApproved`] by a user's actor on a [`SlotKind::Pose`] slot (the approval's slot; with
//!    none, a script's `Activate`, the best offered pose slot for the verb, else the first pose
//!    slot with that verb, which the claim then refuses) writes a `Claim` [`ClaimRequest`]. A slot
//!    the user already holds (`UseSlot`: claim, approach, activate) skips the claim. A user already
//!    in a seat use ignores further approvals (no second slot, no other verb: ai.md "No escalation
//!    ladder").
//! 2. **Offer** (`FixedUpdate`, [`SeatSystems::Offer`], after `InteractionSystems::Apply`, before
//!    `ActionSystems::Step`): a granted claim ([`ClaimAnswered`]) picks the entry side once
//!    ([`pick_entrance`] from the user's feet, `valid` = [`EndPoseFree`]; a slot with no entrances
//!    is entered from anywhere; entrances but none free: released, nothing else). A refused one
//!    ends the use. Then every tick while the user holds the claim, its [`SeatAim`] is the slot's
//!    world frame (enter) and the exit ([`pick_exit`] preferring the entry side, `valid` =
//!    [`EndPoseFree`]; none, or no entry side: where the user stood when the claim was granted),
//!    both relative to this tick's feet, facing the slot's way.
//! 3. **Events** (`FixedUpdate`, [`SeatSystems::Events`], after `ActionSystems::Step`): the body's
//!    `SeatEntered` writes an `Occupy` request, `SeatExited` and `SeatRefused` a `Release` (no
//!    retry) and clear the aim.
//! 4. **Lost claim**: the claim table is the truth. A claim that is no longer this user's at the
//!    offer step (invalidated by `IndexChange::Remove`, `InvalidateClaims`, a move, a lease, a
//!    preemption) clears the aim that tick; the action layer stands a seated body up, and its
//!    later `SeatExited` writes nothing (the hold is gone already).
//!
//! **Latency** at one fixed tick per frame: an `ActivateCommand` validated on tick N is approved
//! in that frame's `Update`, its claim applies on tick N+1 and the enter starts on tick N+1 (the
//! grant is read after `Apply`, before `Step`). `SeatEntered` on tick M occupies on M+1;
//! `SeatExited` or `SeatRefused` on tick K releases on K+1. A lost claim on tick T clears the aim
//! on T (the stand-up starts on T).

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use sf_action::{EventKind as ActionEventKind, TargetId};
use sf_action_bevy::{ActionBody, ActionCharacter, SeatAim, SeatSense};
use sf_anim::RootPose;
use sf_coords::WorldPos;
use sf_interaction::{
    Entrance, Frame, Op, Request, Side, SlotDef, SlotKind, SlotRef, SlotTerms, TagId, UserKey, VerbTarget, compose, pick_entrance,
    pick_exit, to_local,
};
use sf_script_bevy::sf_script::Handle;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::activate::offered_verbs;
use crate::{
    ClaimAnswered, ClaimRequest, InteractionClaims, InteractionDefs, InteractionFrames, InteractionIndex, InteractionStateTags,
    UseApproved,
};

// --- Components and resources -------------------------------------------------------------------

/// A character that uses slots (host-inserted, beside its `sf_action_bevy` character): who it is
/// to the claims and to the scripts.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
#[require(SeatAim)]
pub struct InteractionUser {
    pub user: UserKey,
    /// What `Activated.actor` names for this character (`sf_script_bevy::PLAYER` for the player).
    pub actor: Handle,
    /// Claim priority (`InteractParams::priority_*`).
    pub priority: u8,
    /// The user's tags, for the slots' `users` filters.
    pub tags: SmallVec<[TagId; 4]>,
}

/// The `TargetId` the action layer names placement `placement`'s slots by: its id's low 64 bits.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn seat_target(placement: PlacementId) -> TargetId {
    // Truncation on purpose: a name the shell compares with the seat it offered, never a lookup.
    TargetId(placement.0 as u64)
}

/// The end-pose test (the spec's `EndPoseFree`): does a standing capsule fit at entrance `.0`,
/// placed in the world at `.1` (its approach point, facing the slot's way)? A host seam: the game
/// asks its physics (a capsule overlap against the world minus the seat's own object). The
/// default accepts every entrance.
#[derive(Resource)]
pub struct EndPoseFree(pub Box<EndPoseTest>);

/// The [`EndPoseFree`] test: entrance, its world frame → fits.
pub type EndPoseTest = dyn Fn(&Entrance, &Frame) -> bool + Send + Sync;

impl Default for EndPoseFree {
    fn default() -> Self {
        Self(Box::new(|_, _| true))
    }
}

/// Where a seat use is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeatPhase {
    /// The claim request is out.
    Asked,
    /// The claim is the user's; the entry side is picked at the next offer step.
    Granted,
    /// The slot is in the user's [`SeatAim`]; the enter may be running.
    Offered,
    /// The body sat (`SeatEntered`); the claim is (or turns) Occupied.
    Seated,
    /// The claim was lost while seated: the aim is clear, the stand-up runs, `SeatExited` ends it.
    Leaving,
}

/// One user's seat use.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeatUse {
    pub entity: Entity,
    pub slot: SlotRef,
    pub phase: SeatPhase,
    /// The entrance side picked on the grant (`None`: the slot has no entrances).
    pub entry: Option<Side>,
    /// The feet when the claim was granted (world): the exit when no entrance is free.
    pub from: Vec3,
}

/// Every user's current seat use, at most one each.
#[derive(Resource, Clone, Debug, Default)]
pub struct SeatUses(BTreeMap<UserKey, SeatUse>);

impl SeatUses {
    #[must_use]
    pub fn get(&self, user: UserKey) -> Option<&SeatUse> {
        self.0.get(&user)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&UserKey, &SeatUse)> {
        self.0.iter()
    }
}

/// Where the pose-slot default runs (module docs).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SeatSystems {
    /// `Update`, after `ActivationSystems::Default`: approvals into claim requests.
    Request,
    /// `FixedUpdate`, after `InteractionSystems::Apply`, before `ActionSystems::Step`: grants into
    /// [`SeatAim`].
    Offer,
    /// `FixedUpdate`, after `ActionSystems::Step`: the body's seat events into occupy / release.
    Events,
}

// --- Helpers ------------------------------------------------------------------------------------

/// Facing of a world rotation (yaw 0 faces +Z).
fn yaw_of(rotation: Quat) -> f32 {
    let f = rotation * Vec3::Z;
    f.x.atan2(f.z)
}

fn world_pos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(v.as_dvec3())
}

fn feet_of(body: &ActionBody) -> Vec3 {
    body.loco.position.to_f64().as_vec3()
}

/// Entrance `e` placed in the world: its approach point, facing the slot's way.
fn entrance_frame(object: &Frame, slot_world: &Frame, e: &Entrance) -> Frame {
    Frame { translation: object.translation + object.rotation * e.approach, rotation: slot_world.rotation }
}

/// The slot an approval acts through: its own, else the best offered pose slot for its verb, else
/// the first pose slot with that verb.
#[allow(clippy::too_many_arguments)]
fn resolve_slot(
    approved: &UseApproved,
    user: &InteractionUser,
    placement: PlacementId,
    slots: &[SlotDef],
    defs: &InteractionDefs,
    state: &InteractionStateTags,
    claims: &InteractionClaims,
    index: &InteractionIndex,
) -> Option<u8> {
    let is_pose = |i: u8| slots.get(usize::from(i)).is_some_and(|s| matches!(s.kind, SlotKind::Pose { .. }));
    if let Some(slot) = approved.slot {
        return is_pose(slot).then_some(slot);
    }
    let verb_ok = |v: TagId| approved.verb.is_none_or(|w| w == v);
    let offered = offered_verbs(user.user, &user.tags, user.priority, placement, defs, state, claims, index)
        .into_iter()
        .flatten()
        .find_map(|c| match c.target {
            VerbTarget::Slot(r) if verb_ok(c.verb) && is_pose(r.slot) => Some(r.slot),
            _ => None,
        });
    offered.or_else(|| {
        slots
            .iter()
            .position(|s| matches!(s.kind, SlotKind::Pose { .. }) && verb_ok(s.verb.verb))
            .and_then(|i| u8::try_from(i).ok())
    })
}

fn request(user: &InteractionUser, op: Op) -> ClaimRequest {
    ClaimRequest(Request { user: user.user, priority: user.priority, op })
}

// --- Systems ------------------------------------------------------------------------------------

/// Step 1: approvals on pose slots into claim requests.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
pub(crate) fn seat_requests(
    mut approved: MessageReader<UseApproved>,
    users: Query<(Entity, &InteractionUser)>,
    defs: Res<InteractionDefs>,
    state: Res<InteractionStateTags>,
    claims: Res<InteractionClaims>,
    index: Res<InteractionIndex>,
    mut seats: ResMut<SeatUses>,
    mut out: MessageWriter<ClaimRequest>,
) {
    for a in approved.read() {
        let Some((entity, user)) = users.iter().find(|(_, u)| u.actor == a.activation.actor) else { continue };
        if seats.0.contains_key(&user.user) {
            continue;
        }
        let placement = PlacementId(a.activation.target.0);
        let Some(def) = defs.get(placement) else { continue };
        let Some(slot) = resolve_slot(a, user, placement, &def.slots, &defs, &state, &claims, &index) else { continue };
        let slot = SlotRef { placement, slot };
        let held = claims.0.claim_on(slot).is_some_and(|c| c.user == user.user);
        let phase = if held {
            SeatPhase::Granted
        } else {
            let mut terms = SlotTerms::of(&def.slots[usize::from(slot.slot)], claims.0.params());
            terms.enabled = index.0.get(slot).is_some();
            out.write(request(user, Op::Claim { slot, terms, tags: user.tags.clone() }));
            SeatPhase::Asked
        };
        seats.0.insert(user.user, SeatUse { entity, slot, phase, entry: None, from: Vec3::ZERO });
    }
}

/// Step 2: grants and refusals of this tick's claims, then each held seat into its user's aim.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
pub(crate) fn seat_offers(
    mut answered: MessageReader<ClaimAnswered>,
    claims: Res<InteractionClaims>,
    index: Res<InteractionIndex>,
    frames: Res<InteractionFrames>,
    defs: Res<InteractionDefs>,
    free: Res<EndPoseFree>,
    users: Query<&InteractionUser>,
    mut bodies: Query<(&ActionBody, &mut SeatAim)>,
    mut seats: ResMut<SeatUses>,
    mut out: MessageWriter<ClaimRequest>,
) {
    for a in answered.read() {
        let Op::Claim { slot, .. } = a.request.op else { continue };
        let Some(u) = seats.0.get_mut(&a.request.user).filter(|u| u.phase == SeatPhase::Asked && u.slot == slot) else {
            continue;
        };
        if a.result.is_ok() {
            u.phase = SeatPhase::Granted;
        } else {
            seats.0.remove(&a.request.user);
        }
    }

    let free = &*free.0;
    seats.0.retain(|key, u| {
        let user = users.get(u.entity).ok().filter(|i| i.user == *key);
        let Some(((body, mut aim), user)) = bodies.get_mut(u.entity).ok().zip(user) else {
            // The character is gone: let the slot go.
            out.write(ClaimRequest(Request { user: *key, priority: u8::MAX, op: Op::Release }));
            return false;
        };
        if u.phase == SeatPhase::Asked {
            return true;
        }
        let holds = claims.0.claim_on(u.slot).is_some_and(|c| c.user == *key);
        let placed = (|| {
            let def = defs.get(u.slot.placement).filter(|d| usize::from(u.slot.slot) < d.slots.len())?;
            Some((Arc::clone(def), *frames.current(u.slot.placement)?, index.0.get(u.slot)?.world))
        })();
        let Some((def, object, slot_world)) = placed.filter(|_| holds) else {
            aim.0 = None;
            return match u.phase {
                SeatPhase::Seated | SeatPhase::Leaving => {
                    u.phase = SeatPhase::Leaving;
                    true
                }
                _ => false,
            };
        };
        if u.phase == SeatPhase::Leaving {
            aim.0 = None;
            return true;
        }
        let slot_def = &def.slots[usize::from(u.slot.slot)];
        let feet = feet_of(body);
        let valid = |e: &Entrance| free(e, &entrance_frame(&object, &slot_world, e));
        if u.phase == SeatPhase::Granted {
            u.from = feet;
            if !slot_def.entrances.is_empty() {
                let Some(e) = pick_entrance(slot_def, to_local(&object, feet), valid) else {
                    // No way in: give the slot back, nothing else (no other verb).
                    out.write(request(user, Op::Release));
                    aim.0 = None;
                    return false;
                };
                u.entry = Some(e.side);
            }
            u.phase = SeatPhase::Offered;
        }
        let exit = u
            .entry
            .and_then(|side| pick_exit(slot_def, side, valid))
            .map_or(u.from, |e| compose(&object, &Frame { translation: e.approach, rotation: Quat::IDENTITY }).translation);
        let feet = body.loco.position;
        let yaw = yaw_of(slot_world.rotation);
        aim.0 = Some(SeatSense {
            target: seat_target(u.slot.placement),
            slot: u.slot.slot,
            enter: RootPose::new(feet.delta_f32(world_pos(slot_world.translation)), yaw),
            exit: RootPose::new(feet.delta_f32(world_pos(exit)), yaw),
        });
        true
    });
}

/// Step 3: the body's seat events into occupy and release.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn seat_events(
    mut characters: Query<(&ActionCharacter, &InteractionUser, &mut SeatAim)>,
    mut seats: ResMut<SeatUses>,
    mut out: MessageWriter<ClaimRequest>,
) {
    seats.0.retain(|key, seat| {
        let Ok((c, user, mut aim)) = characters.get_mut(seat.entity) else { return true };
        if user.user != *key {
            return true;
        }
        let target = Some(seat_target(seat.slot.placement));
        for e in c.output.events.iter().filter(|e| e.target == target) {
            match e.kind {
                ActionEventKind::SeatEntered(s) if s == seat.slot.slot && seat.phase == SeatPhase::Offered => {
                    out.write(request(user, Op::Occupy { slot: seat.slot }));
                    seat.phase = SeatPhase::Seated;
                }
                ActionEventKind::SeatExited(s) if s == seat.slot.slot => {
                    // Leaving: the hold ended already, nothing to release.
                    if seat.phase != SeatPhase::Leaving {
                        out.write(request(user, Op::Release));
                    }
                    aim.0 = None;
                    return false;
                }
                ActionEventKind::SeatRefused(s) if s == seat.slot.slot && seat.phase == SeatPhase::Offered => {
                    out.write(request(user, Op::Release));
                    aim.0 = None;
                    return false;
                }
                _ => {}
            }
        }
        true
    });
}
