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
//!    later `SeatExited` writes nothing (the hold is gone already). A lost def, frame or index row
//!    with the claim still held writes its `Release` at the loss, then goes the same way.
//!
//! **Clips** ([`SeatClips`]): from the offer on, the user carries the slot's clip family, band (cm)
//! and the enter and exit sides, for the animation side to pick its clip set (the anim contract:
//! `<family>.b<band cm>.{enter.<side>, loop, exit.<side>}`); the exit side follows [`pick_exit`]
//! while the use lasts. It is removed when the use ends: the stand-up's `SeatExited`, a refused
//! enter, a lost claim before the body sat, the slot given back. A lost claim while seated keeps it
//! until the stand-up's `SeatExited` (the stand-up plays the exit clip of the last exit side).
//!
//! **Reports** ([`SeatReport`]): every transition of a seat use writes one message, from the step
//! where it happens (never per tick), naming the user, its entity, the slot and the [`SeatStep`]:
//! the request's `Asked` or `Held` (or `Busy`: an approval ignored because the user is already in
//! a use, with that use's slot and phase); the claim's `ClaimRefused`; the offer step's `UserGone`,
//! `Lost` (which of claim, def, frame or index row went; once when the use is dropped or a seated
//! user turns `Leaving`), `NoEntrance` and `Offered` (on the grant's tick: entry side, horizontal
//! feet-to-seat distance, yaw offset); the events step's `Entered`, `Exited` and `EnterRefused`
//! (with that tick's action refusals). An offered use whose enter cannot start reports
//! `Approaching` once, on the tick the action layer starts its approach walk, before that tick's
//! `Entered` or `EnterRefused`. An approval that is not a seat use (no def, no pose slot
//! for the verb) or whose actor is no [`InteractionUser`] reports nothing. The host logs or shows
//! them; every use ends in `ClaimRefused`, `UserGone`, `Lost` (before sitting), `NoEntrance`,
//! `EnterRefused` or `Exited`.
//!
//! **Latency** at one fixed tick per frame: an `ActivateCommand` validated on tick N is approved
//! in that frame's `Update`, its claim applies on tick N+1 and the enter starts on tick N+1 (the
//! grant is read after `Apply`, before `Step`). `SeatEntered` on tick M occupies on M+1;
//! `SeatExited` or `SeatRefused` on tick K releases on K+1. A lost claim on tick T clears the aim
//! on T (the stand-up starts on T).

use std::collections::BTreeMap;
use std::f32::consts::{PI, TAU};
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::{Quat, Vec2, Vec3};
use sf_action::{EventKind as ActionEventKind, RuleKey, TargetId};
use sf_action_bevy::{ActionBody, ActionCharacter, SeatAim, SeatSense};
use sf_anim::RootPose;
use sf_coords::WorldPos;
use sf_interaction::{
    ClipFamily, Entrance, Frame, InteractableDef, Op, Request, Side, SlotDef, SlotKind, SlotRef, SlotTerms, TagId, UserKey, VerbTarget, compose, pick_entrance,
    pick_exit, to_local,
};
use sf_script_bevy::sf_script::Handle;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::activate::offered_verbs;
use crate::{
    ClaimAnswered, ClaimRequest, InteractionClaims, InteractionDefs, InteractionFrames, InteractionIndex, InteractionStateTags,
    SIT_APPROACH, UseApproved,
};

// --- Components and resources -------------------------------------------------------------------

/// A character that uses slots (host-inserted, beside its `sf_action_bevy` character): who it is
/// to the claims and to the scripts.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
#[require(SeatAim, sf_action_bevy::ActionAliases)]
pub struct InteractionUser {
    pub user: UserKey,
    /// What `Activated.actor` names for this character (`sf_script_bevy::PLAYER` for the player).
    pub actor: Handle,
    /// Claim priority (`InteractParams::priority_*`).
    pub priority: u8,
    /// The user's tags, for the slots' `users` filters.
    pub tags: SmallVec<[TagId; 4]>,
}

/// The clips of the seat a user is entering, sitting in or getting up from (module docs, "Clips"):
/// on the user's entity from the offer to the end of the use, absent otherwise. Inserted and
/// removed rather than a required `Option` (like [`SeatAim`]): its presence is the state, it changes
/// twice per use (two archetype moves, negligible), and readers query it directly.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct SeatClips {
    /// The slot's [`SlotDef::clips`] (data: the game names its families).
    pub family: ClipFamily,
    /// The slot's [`SlotDef::band`] (the surface's height above the floor), centimetres, rounded.
    pub band_cm: u16,
    /// The entrance [`pick_entrance`] chose; `Side::Front` for a slot with no entrances (entered
    /// from where the feet were).
    pub enter: Side,
    /// The exit [`pick_exit`] chooses now; `Side::Front` when none is free or the slot has no
    /// entrances (the user gets up where its feet were when the claim was granted).
    pub exit: Side,
}

/// A slot band in metres as whole centimetres (clamped to `u16`).
fn band_cm(band: f32) -> u16 {
    let cm = (band * 100.0).round();
    if cm.is_nan() || cm <= 0.0 {
        0
    } else if cm >= f32::from(u16::MAX) {
        u16::MAX
    } else {
        // In range and whole: exact.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let cm = cm as u16;
        cm
    }
}

/// Removes `entity`'s [`SeatClips`] if it has them.
fn clear_clips(commands: &mut Commands, entity: Entity, has: bool) {
    if has && let Ok(mut e) = commands.get_entity(entity) {
        e.try_remove::<SeatClips>();
    }
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

/// What a lost seat use no longer has (checked in this order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeatLoss {
    /// The slot's claim is no longer the user's.
    Claim,
    /// The placement has no def, or the def no such slot.
    Def,
    /// The placement has no current frame.
    Frame,
    /// The slot has no index row.
    IndexRow,
}

/// One transition of a seat use ([`SeatReport`], module docs "Reports").
#[derive(Clone, Debug, PartialEq)]
pub enum SeatStep {
    /// The claim request is written.
    Asked,
    /// The slot was already the user's claim: straight to granted.
    Held,
    /// An approval ignored: the user is already in a seat use (the report names that use's slot),
    /// in this phase.
    Busy(SeatPhase),
    /// The claim refused the use.
    ClaimRefused(sf_interaction::Refusal),
    /// The character is gone (or no longer this user): the slot is let go.
    UserGone,
    /// The use lost what it needs (a claim still held is released): dropped, or (seated) the stand-up starts.
    Lost(SeatLoss),
    /// The slot has entrances but none is free: the slot is given back.
    NoEntrance,
    /// The slot is offered to the action layer (the grant's tick only).
    Offered {
        /// The entrance picked (`None`: the slot has no entrances).
        entry: Option<Side>,
        /// Feet to the slot's world translation, horizontal, metres.
        feet_to_seat_m: f32,
        /// The slot's yaw minus the body's, radians in [-pi, pi].
        yaw_off: f32,
    },
    /// The action layer started the approach walk ([`crate::SIT_APPROACH`]): the body walks to the
    /// enter clip's start pose; the use stays offered.
    Approaching,
    /// The body sat (`SeatEntered`).
    Entered,
    /// The body stood up (`SeatExited`): the use ends.
    Exited,
    /// The action layer refused the enter, or the approach ended without sitting (`SeatRefused`),
    /// with that tick's refusals.
    EnterRefused(SmallVec<[sf_action::Refusal; 2]>),
}

/// One seat-use transition of `user` (entity `entity`) on `slot` (module docs "Reports").
#[derive(Message, Clone, Debug, PartialEq)]
pub struct SeatReport {
    pub user: UserKey,
    pub entity: Entity,
    pub slot: SlotRef,
    pub step: SeatStep,
}

fn report(user: UserKey, u: &SeatUse, step: SeatStep) -> SeatReport {
    SeatReport { user, entity: u.entity, slot: u.slot, step }
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

/// An angle wrapped to [-pi, pi).
fn wrap_angle(a: f32) -> f32 {
    (a + PI).rem_euclid(TAU) - PI
}

fn world_pos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(v.as_dvec3())
}

pub(crate) fn feet_of(body: &ActionBody) -> Vec3 {
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

/// What the offer step needs of `key`'s held `slot`: its def, its object's frame and its world
/// frame; else the first of them that is missing.
fn placed(
    key: UserKey,
    slot: SlotRef,
    claims: &InteractionClaims,
    defs: &InteractionDefs,
    frames: &InteractionFrames,
    index: &InteractionIndex,
) -> Result<(Arc<InteractableDef>, Frame, Frame), SeatLoss> {
    if claims.0.claim_on(slot).is_none_or(|c| c.user != key) {
        return Err(SeatLoss::Claim);
    }
    let def = defs.get(slot.placement).filter(|d| usize::from(slot.slot) < d.slots.len()).ok_or(SeatLoss::Def)?;
    let object = *frames.current(slot.placement).ok_or(SeatLoss::Frame)?;
    let slot_world = index.0.get(slot).ok_or(SeatLoss::IndexRow)?.world;
    Ok((Arc::clone(def), object, slot_world))
}

/// The `Offered` report: horizontal feet-to-seat distance, the slot's yaw minus the body's (as the
/// action step senses it).
fn offered_step(entry: Option<Side>, feet: Vec3, body: &ActionBody, slot_world: &Frame) -> SeatStep {
    let to_seat = slot_world.translation - feet;
    SeatStep::Offered {
        entry,
        feet_to_seat_m: Vec2::new(to_seat.x, to_seat.z).length(),
        yaw_off: wrap_angle(yaw_of(slot_world.rotation) - body.loco.motion.yaw),
    }
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
    mut reports: MessageWriter<SeatReport>,
) {
    for a in approved.read() {
        let Some((entity, user)) = users.iter().find(|(_, u)| u.actor == a.activation.actor) else { continue };
        let placement = PlacementId(a.activation.target.0);
        let Some(def) = defs.get(placement) else { continue };
        let Some(slot) = resolve_slot(a, user, placement, &def.slots, &defs, &state, &claims, &index) else { continue };
        if let Some(busy) = seats.0.get(&user.user) {
            reports.write(report(user.user, busy, SeatStep::Busy(busy.phase)));
            continue;
        }
        let slot = SlotRef { placement, slot };
        let held = claims.0.claim_on(slot).is_some_and(|c| c.user == user.user);
        let (phase, step) = if held {
            (SeatPhase::Granted, SeatStep::Held)
        } else {
            let mut terms = SlotTerms::of(&def.slots[usize::from(slot.slot)], claims.0.params());
            terms.enabled = index.0.get(slot).is_some();
            out.write(request(user, Op::Claim { slot, terms, tags: user.tags.clone() }));
            (SeatPhase::Asked, SeatStep::Asked)
        };
        let u = SeatUse { entity, slot, phase, entry: None, from: Vec3::ZERO };
        reports.write(report(user.user, &u, step));
        seats.0.insert(user.user, u);
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
    mut bodies: Query<(&ActionBody, &mut SeatAim, Option<&mut SeatClips>)>,
    mut seats: ResMut<SeatUses>,
    mut out: MessageWriter<ClaimRequest>,
    mut reports: MessageWriter<SeatReport>,
    mut commands: Commands,
) {
    for a in answered.read() {
        let Op::Claim { slot, .. } = a.request.op else { continue };
        let Some(u) = seats.0.get_mut(&a.request.user).filter(|u| u.phase == SeatPhase::Asked && u.slot == slot) else {
            continue;
        };
        match a.result {
            Ok(_) => u.phase = SeatPhase::Granted,
            Err(refusal) => {
                reports.write(report(a.request.user, u, SeatStep::ClaimRefused(refusal)));
                seats.0.remove(&a.request.user);
            }
        }
    }

    let free = &*free.0;
    seats.0.retain(|key, u| {
        let user = users.get(u.entity).ok().filter(|i| i.user == *key);
        let Some(((body, mut aim, mut clips), user)) = bodies.get_mut(u.entity).ok().zip(user) else {
            // The character is gone (or no longer this user): let the slot go.
            out.write(ClaimRequest(Request { user: *key, priority: u8::MAX, op: Op::Release }));
            clear_clips(&mut commands, u.entity, true);
            reports.write(report(*key, u, SeatStep::UserGone));
            return false;
        };
        if u.phase == SeatPhase::Asked {
            return true;
        }
        let (def, object, slot_world) = match placed(*key, u.slot, &claims, &defs, &frames, &index) {
            Ok(placed) => placed,
            Err(loss) => {
                aim.0 = None;
                // Only a lost claim is gone already: a lost def / frame / row gives the slot back here.
                if loss != SeatLoss::Claim && u.phase != SeatPhase::Leaving {
                    out.write(request(user, Op::Release));
                }
                return match u.phase {
                    SeatPhase::Seated | SeatPhase::Leaving => {
                        // The stand-up still plays this seat's exit clip: the clips stay to `SeatExited`.
                        if u.phase == SeatPhase::Seated {
                            reports.write(report(*key, u, SeatStep::Lost(loss)));
                        }
                        u.phase = SeatPhase::Leaving;
                        true
                    }
                    _ => {
                        clear_clips(&mut commands, u.entity, clips.is_some());
                        reports.write(report(*key, u, SeatStep::Lost(loss)));
                        false
                    }
                };
            }
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
                    clear_clips(&mut commands, u.entity, clips.is_some());
                    reports.write(report(*key, u, SeatStep::NoEntrance));
                    return false;
                };
                u.entry = Some(e.side);
            }
            u.phase = SeatPhase::Offered;
            reports.write(report(*key, u, offered_step(u.entry, feet, body, &slot_world)));
        }
        let exit_entrance = u.entry.and_then(|side| pick_exit(slot_def, side, valid));
        let exit_side = exit_entrance.map_or(Side::Front, |e| e.side);
        match clips.as_deref_mut() {
            Some(c) if c.exit != exit_side => c.exit = exit_side,
            Some(_) => {}
            None => {
                commands.entity(u.entity).insert(SeatClips {
                    family: slot_def.clips.clone(),
                    band_cm: band_cm(slot_def.band),
                    enter: u.entry.unwrap_or(Side::Front),
                    exit: exit_side,
                });
            }
        }
        let exit = exit_entrance
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
    mut reports: MessageWriter<SeatReport>,
    mut commands: Commands,
) {
    seats.0.retain(|key, seat| {
        let Ok((c, user, mut aim)) = characters.get_mut(seat.entity) else { return true };
        // Always queued (no presence check): ordered after a same-tick insert from the offer step.
        if user.user != *key {
            return true;
        }
        // The approach's `MoveStart` carries the focus target, not the seat: tested unfiltered,
        // written before the loop so a same-tick `Entered` / `EnterRefused` comes after it.
        let approach = ActionEventKind::MoveStart(RuleKey::of(SIT_APPROACH));
        if seat.phase == SeatPhase::Offered && c.output.events.iter().any(|e| e.kind == approach) {
            reports.write(report(*key, seat, SeatStep::Approaching));
        }
        let target = Some(seat_target(seat.slot.placement));
        for e in c.output.events.iter().filter(|e| e.target == target) {
            match e.kind {
                ActionEventKind::SeatEntered(s) if s == seat.slot.slot && seat.phase == SeatPhase::Offered => {
                    out.write(request(user, Op::Occupy { slot: seat.slot }));
                    seat.phase = SeatPhase::Seated;
                    reports.write(report(*key, seat, SeatStep::Entered));
                }
                ActionEventKind::SeatExited(s) if s == seat.slot.slot => {
                    // Leaving: the hold ended already, nothing to release.
                    if seat.phase != SeatPhase::Leaving {
                        out.write(request(user, Op::Release));
                    }
                    aim.0 = None;
                    clear_clips(&mut commands, seat.entity, true);
                    reports.write(report(*key, seat, SeatStep::Exited));
                    return false;
                }
                ActionEventKind::SeatRefused(s) if s == seat.slot.slot && seat.phase == SeatPhase::Offered => {
                    out.write(request(user, Op::Release));
                    aim.0 = None;
                    clear_clips(&mut commands, seat.entity, true);
                    reports.write(report(*key, seat, SeatStep::EnterRefused(c.output.refusals.iter().copied().collect())));
                    return false;
                }
                _ => {}
            }
        }
        true
    });
}
