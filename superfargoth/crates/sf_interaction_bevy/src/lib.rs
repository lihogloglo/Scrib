//! Bevy shell for [`sf_interaction`] (`docs/specs/interaction.md` "Point index and focus", "Claims",
//! "Events", "Tunables"). Slice `IN31a`: the host's claim table and slot index on the fixed tick.
//!
//! - [`InteractionPlugin`]: each `FixedUpdate` run, in [`InteractionSystems::Apply`], advances
//!   [`InteractionTick`], then applies the tick's messages in this order:
//!   1. [`IndexChange`]s (inserts, moves, removals of placed objects; the game's cell loading sends
//!      them, IN32), sorted by placement (send order kept per placement). A removal also ends every
//!      claim on the object with its reason (`OwnerUnloaded` for a cell unload). A re-insert of an
//!      indexed object ends the holds on slots it disables or no longer has (`SlotDisabled`). A
//!      move, or a re-insert elsewhere, of a held object is measured from its anchor in
//!      [`InteractionFrames`] (where it was when its first hold was granted, so a slow drift adds
//!      up) and goes to [`Claims::owner_moved`] (`OwnerMoved` beyond the move tolerances).
//!   2. [`InvalidateClaims`], sorted by subject then reason, so send order never matters.
//!   3. [`ClaimRequest`]s as one [`Claims::apply_batch`] (the core sorts it by priority, `UserKey`,
//!      request index). A `Claim` or `Enqueue` on a slot that isn't in the index is refused with
//!      `Refusal::Disabled` without reaching the core (the object isn't resident).
//!   4. [`Claims::tick`] (lease expiry and queue promotion). Then each object the tick's events
//!      touched is anchored at its current frame if it is newly held, or loses its anchor if it no
//!      longer is.
//!
//!   Out: one [`ClaimAnswered`] per request (in send order) and one [`ClaimChanged`] per core
//!   [`InteractionEvent`] (in the core's order).
//! - [`InteractionSettings`]: [`InteractParams`] bound live from [`TUNABLES`] (`interact.*`,
//!   defaults from the struct; a test keeps the two equal). `tick_hz` comes from `Time<Fixed>`
//!   (the struct's default without one). The claim table takes a changed value on the next fixed
//!   run; running leases keep their deadlines.
//! - Hosts order their request writers `.before(InteractionSystems::Apply)` and their event readers
//!   `.after(...)` in `FixedUpdate`.
//!
//! - Activation (`IN31b1`, [`activate`] module docs): [`ActivateCommand`] validated in
//!   [`InteractionSystems::Activate`] (after `Apply`) into `sf_script_bevy::Activated`; after the
//!   script slot, [`ActivationSystems`] runs the gates and writes [`UseApproved`] / [`UseRefused`].
//!   Needs `sf_script_bevy::ScriptsPlugin` (the scripts answer every `Activated`).
//!
//! - Focus (`IN31b2`, [`focus`] module docs): the host's [`FocusCandidates`] per player per tick
//!   become each player's stable [`FocusState`] (target, offered verbs, hold list) in
//!   [`PlayerFocus`], in [`InteractionSystems::Focus`] (after `Apply`, before `Activate`);
//!   [`FocusState::command`] gives the [`ActivateCommand`] the host writes.
//!
//! - Pose slots (`IN2b2`, [`seat`] module docs): the default for an approved use of a
//!   [`sf_interaction::SlotKind::Pose`] slot by an [`InteractionUser`] character: claim, offer the
//!   claimed slot to the action layer (`sf_action_bevy::SeatAim`), occupy on `SeatEntered`, release
//!   on `SeatExited` / `SeatRefused`, clear the aim when the claim is lost. [`SeatSystems`] orders
//!   it around `sf_action_bevy::ActionSystems::Step`; [`EndPoseFree`] is the host's capsule test.
//!
//! Not here: game wiring (`IN32`).

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use sf_interaction::{
    Claims, Frame, Granted, InteractParams, InteractableDef, InteractionEvent, InvalidReason, Op, PointIndex,
    Refusal, Request, SlotRef, Subject, Tick,
};
use sf_script_bevy::{Activated, ActivationOutcome, ScriptSlot};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;
use sf_world::PlacementId;
use smallvec::SmallVec;

pub mod activate;
pub mod focus;
pub mod seat;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_activate;
#[cfg(test)]
mod tests_focus;
#[cfg(test)]
mod tests_seat;

pub use activate::{
    ActivateCommand, ActivationSystems, InteractionDefs, InteractionStateTags, PendingUse, PendingUses, ScriptActivate,
    UseApproved, UseRefused,
};
pub use focus::{FocusCandidates, FocusState, PlayerFocus};
pub use seat::{EndPoseFree, EndPoseTest, InteractionUser, SeatClips, SeatPhase, SeatSystems, SeatUse, SeatUses, seat_target};

// --- Tunables ---------------------------------------------------------------------------------

/// A gameplay tunable (net `Gameplay`, scope `Dev`, source `Guess`, live).
const fn gp(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

/// A gameplay count (as [`gp`], unit `Count`).
const fn gp_count(d: Def<u32>) -> Def<u32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live).unit(Unit::Count)
}

/// A presentation tunable (net `Presentation`, scope `Dev`, source `Guess`, live).
const fn pres(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Presentation).lifetime(Lifetime::Live)
}

pub const PRIORITY_AMBIENT: Def<u32> = gp_count(def!(count "interact.claim.priority_ambient" = 10)
    .range(0, 255)
    .help("Claim priority of ambient uses (order player = quest > schedule > ambient: UE Smart Objects)"));
pub const PRIORITY_SCHEDULE: Def<u32> = gp_count(def!(count "interact.claim.priority_schedule" = 20)
    .range(0, 255)
    .help("Claim priority of schedule uses"));
pub const PRIORITY_QUEST: Def<u32> = gp_count(def!(count "interact.claim.priority_quest" = 30)
    .range(0, 255)
    .help("Claim priority of quest uses (= player, roadmap decision 1)"));
pub const PRIORITY_PLAYER: Def<u32> = gp_count(def!(count "interact.claim.priority_player" = 30)
    .range(0, 255)
    .help("Claim priority of the player's uses"));
pub const LEASE_S: Def<f32> = gp(def!(float "interact.claim.lease_s" = 10.0)
    .range(1.0, 60.0)
    .unit(Unit::Seconds)
    .help("A Reserved claim's lease without progress (renewed by path progress)"));
pub const MOVE_TOLERANCE_M: Def<f32> = gp(def!(float "interact.claim.move_tolerance_m" = 0.25)
    .range(0.05, 2.0)
    .unit(Unit::Meters)
    .help("Owner move beyond which its claims are invalidated (OwnerMoved)"));
pub const MOVE_TOLERANCE_DEG: Def<f32> = gp(def!(float "interact.claim.move_tolerance_deg" = 20.0)
    .range(1.0, 90.0)
    .unit(Unit::Degrees)
    .help("Owner turn beyond which its claims are invalidated (OwnerMoved)"));
pub const QUEUE_CAPACITY: Def<u32> = gp_count(def!(count "interact.queue.capacity" = 3)
    .range(0, 16)
    .help("Queue tickets of the kinds that queue by default (door, workstation)"));
pub const TICKET_LEASE_S: Def<f32> = gp(def!(float "interact.queue.ticket_lease_s" = 30.0)
    .range(5.0, 300.0)
    .unit(Unit::Seconds)
    .help("A queue ticket's lease without renewal"));
pub const FOCUS_HOLD_S: Def<f32> = def!(float "interact.focus.hold_s" = 0.35)
    .range(0.1, 1.5)
    .unit(Unit::Seconds)
    .source(Source::Guess)
    .scope(Scope::Player)
    .net(Net::Presentation)
    .lifetime(Lifetime::Live)
    .help("Hold of interact after which the focus lists every verb (RDR2-style hold prompt)");
pub const FOCUS_SWITCH_MARGIN: Def<f32> = pres(def!(float "interact.focus.switch_margin" = 0.2)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Score lead, as a fraction of the focused target's, a challenger needs to take focus (hysteresis against prompt flicker, gta6.md 2)"));
pub const FOCUS_SWITCH_S: Def<f32> = pres(def!(float "interact.focus.switch_s" = 0.15)
    .range(0.0, 1.0)
    .unit(Unit::Seconds)
    .help("How long a challenger must keep that lead before focus switches"));
pub const HAND_MAX: Def<u32> = gp_count(def!(count "interact.claim.hand_max" = 2)
    .range(0, 4)
    .help("Hand claims a user may hold while Occupied (two hands)"));

/// The `interact.*` tunables [`InteractParams`] has, bound by [`InteractionPlugin`].
pub const TUNABLES: &[&dyn AnyDef] = &[
    &PRIORITY_AMBIENT,
    &PRIORITY_SCHEDULE,
    &PRIORITY_QUEST,
    &PRIORITY_PLAYER,
    &LEASE_S,
    &MOVE_TOLERANCE_M,
    &MOVE_TOLERANCE_DEG,
    &QUEUE_CAPACITY,
    &TICKET_LEASE_S,
    &FOCUS_HOLD_S,
    &FOCUS_SWITCH_MARGIN,
    &FOCUS_SWITCH_S,
    &HAND_MAX,
];

// --- Resources ----------------------------------------------------------------------------------

/// [`InteractParams`] as bound from [`TUNABLES`]; `tick_hz` here is ignored (the fixed timestep's).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct InteractionSettings(pub InteractParams);

/// The host's claim table.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionClaims(pub Claims);

/// The slots of every resident object, owner = the object's entity.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionIndex(pub PointIndex<Entity>);

/// Where each indexed object is (world), and where each held one was when its first hold (claim or
/// queue ticket) was granted: `OwnerMoved` is measured from that anchor, not frame to frame.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionFrames {
    current: BTreeMap<PlacementId, Frame>,
    anchors: BTreeMap<PlacementId, Frame>,
}

impl InteractionFrames {
    /// The object's frame as last inserted or moved, while it is indexed.
    #[must_use]
    pub fn current(&self, placement: PlacementId) -> Option<&Frame> {
        self.current.get(&placement)
    }

    /// Where the object was when its first current hold was granted, while it has one.
    #[must_use]
    pub fn anchor(&self, placement: PlacementId) -> Option<&Frame> {
        self.anchors.get(&placement)
    }

    /// The object went to `object`: the claims test their tolerances against the anchor, and an
    /// invalidation drops it.
    fn moved(&mut self, claims: &mut Claims, placement: PlacementId, object: &Frame, now: Tick) {
        let Some(slot) = self.current.get_mut(&placement) else { return };
        *slot = *object;
        if let Some(anchor) = self.anchors.get(&placement) {
            let moved_m = anchor.translation.distance(object.translation);
            let turned_deg = anchor.rotation.angle_between(object.rotation).to_degrees();
            if claims.owner_moved(placement, moved_m, turned_deg, now) {
                self.anchors.remove(&placement);
            }
        }
    }

    /// Anchors the objects of this tick's events that are newly held, drops those no longer held.
    fn reconcile(&mut self, claims: &Claims) {
        let mut touched: Vec<PlacementId> = claims.events().iter().map(|e| e.slot.placement).collect();
        touched.sort_unstable();
        touched.dedup();
        for placement in touched {
            if claims.held_slots(placement).next().is_none() {
                self.anchors.remove(&placement);
            } else if let Some(frame) = self.current.get(&placement) {
                self.anchors.entry(placement).or_insert(*frame);
            }
        }
    }
}

/// The fixed tick claims count in: advanced once per `FixedUpdate` run, before the tick's batch.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InteractionTick(pub Tick);

// --- Messages -----------------------------------------------------------------------------------

/// One claim-table request (claim, enqueue, occupy, release, renew, hand claim or release) for this
/// tick's batch.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ClaimRequest(pub Request);

/// Ends holds for `reason` (a dead or knocked-down user, a destroyed object, a disabled slot).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidateClaims {
    pub subject: Subject,
    pub reason: InvalidReason,
}

/// A change to the slot index, applied before the tick's claims.
#[derive(Message, Clone, Debug, PartialEq)]
pub enum IndexChange {
    /// A resident object (cell load, a spawned or dropped one): its def's slots at `object`
    /// (world), except the `disabled` slot indices. Replaces the placement's earlier entries.
    Insert {
        placement: PlacementId,
        def: Arc<InteractableDef>,
        object: Frame,
        owner: Entity,
        disabled: SmallVec<[u8; 4]>,
    },
    /// A moved owner (a mount, a vehicle, a settled body): its slots follow `object` (world).
    Move { placement: PlacementId, object: Frame },
    /// The object left (cell unload, destroyed): its slots leave the index and its claims end with
    /// `reason`.
    Remove { placement: PlacementId, reason: InvalidReason },
}

impl IndexChange {
    /// The object's cell unloaded.
    #[must_use]
    pub const fn unload(placement: PlacementId) -> Self {
        Self::Remove { placement, reason: InvalidReason::OwnerUnloaded }
    }

    #[must_use]
    pub const fn placement(&self) -> PlacementId {
        match self {
            Self::Insert { placement, .. } | Self::Move { placement, .. } | Self::Remove { placement, .. } => {
                *placement
            }
        }
    }
}

/// One claim-table change from the core (net rule 8): scripts, audio, barks, crime and the AI
/// executor read them.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClaimChanged(pub InteractionEvent);

/// The answer to one [`ClaimRequest`] of the tick (the AI picks another slot on a refusal).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ClaimAnswered {
    pub tick: Tick,
    pub request: Request,
    pub result: Result<Granted, Refusal>,
}

// --- Plugin -------------------------------------------------------------------------------------

/// Where the claim table steps, in `FixedUpdate`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionSystems {
    /// Settings sync, index changes, invalidations, the batch, leases; events out.
    Apply,
    /// Each player's [`FocusCandidates`] into [`PlayerFocus`] (target, verbs, hold list).
    Focus,
    /// [`ActivateCommand`]s validated against this tick's claims; `Activated` out.
    Activate,
}

/// Hosts [`InteractionClaims`] and [`InteractionIndex`] on the fixed tick and binds [`TUNABLES`]
/// (module docs). With `sf_tunables_bevy::TunablesPlugin` the values follow the console, files and
/// presets live.
#[derive(Default)]
pub struct InteractionPlugin;

macro_rules! bind {
    ($app:ident, $def:expr, |$p:ident, $v:ident| $body:expr) => {
        let _ = $app.add_tunable(&$def).bind_resource(|s: &mut InteractionSettings, $v| {
            let $p = &mut s.0;
            $body;
        });
    };
}

/// A count tunable as the core's `u8` (the ranges keep it in bounds).
fn to_u8(v: u32) -> u8 {
    u8::try_from(v).unwrap_or(u8::MAX)
}

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionSettings>()
            .init_resource::<InteractionClaims>()
            .init_resource::<InteractionIndex>()
            .init_resource::<InteractionFrames>()
            .init_resource::<InteractionTick>()
            .add_message::<ClaimRequest>()
            .add_message::<InvalidateClaims>()
            .add_message::<IndexChange>()
            .add_message::<ClaimChanged>()
            .add_message::<ClaimAnswered>()
            .init_resource::<InteractionDefs>()
            .init_resource::<InteractionStateTags>()
            .init_resource::<PendingUses>()
            .init_resource::<activate::ActivationSlots>()
            .init_resource::<SeatUses>()
            .init_resource::<EndPoseFree>()
            .add_message::<ActivateCommand>()
            .add_message::<ScriptActivate>()
            .add_message::<UseApproved>()
            .add_message::<UseRefused>()
            .add_message::<Activated>()
            .add_message::<ActivationOutcome>()
            .init_resource::<PlayerFocus>()
            .add_message::<FocusCandidates>()
            .configure_sets(
                FixedUpdate,
                (InteractionSystems::Apply, InteractionSystems::Focus, InteractionSystems::Activate).chain(),
            )
            .configure_sets(
                Update,
                (ActivationSystems::Collect, ActivationSystems::Gates, ActivationSystems::Default)
                    .chain()
                    .after(ScriptSlot::Update),
            )
            .add_systems(FixedUpdate, (sync_params, step).chain().in_set(InteractionSystems::Apply))
            .add_systems(FixedUpdate, focus::focus.in_set(InteractionSystems::Focus))
            .add_systems(FixedUpdate, activate::activate.in_set(InteractionSystems::Activate))
            .add_systems(Update, activate::collect.in_set(ActivationSystems::Collect))
            .add_systems(Update, activate::finish.in_set(ActivationSystems::Default))
            .configure_sets(Update, SeatSystems::Request.after(ActivationSystems::Default))
            .configure_sets(
                FixedUpdate,
                SeatSystems::Offer.after(InteractionSystems::Apply).before(sf_action_bevy::ActionSystems::Step),
            )
            .configure_sets(FixedUpdate, SeatSystems::Events.after(sf_action_bevy::ActionSystems::Step))
            .add_systems(Update, seat::seat_requests.in_set(SeatSystems::Request))
            .add_systems(FixedUpdate, seat::seat_offers.in_set(SeatSystems::Offer))
            .add_systems(FixedUpdate, seat::seat_events.in_set(SeatSystems::Events));
        bind!(app, PRIORITY_AMBIENT, |p, v| p.priority_ambient = to_u8(v));
        bind!(app, PRIORITY_SCHEDULE, |p, v| p.priority_schedule = to_u8(v));
        bind!(app, PRIORITY_QUEST, |p, v| p.priority_quest = to_u8(v));
        bind!(app, PRIORITY_PLAYER, |p, v| p.priority_player = to_u8(v));
        bind!(app, LEASE_S, |p, v| p.lease_s = v);
        bind!(app, MOVE_TOLERANCE_M, |p, v| p.move_tolerance_m = v);
        bind!(app, MOVE_TOLERANCE_DEG, |p, v| p.move_tolerance_deg = v);
        bind!(app, QUEUE_CAPACITY, |p, v| p.queue_capacity = to_u8(v));
        bind!(app, TICKET_LEASE_S, |p, v| p.ticket_lease_s = v);
        bind!(app, FOCUS_HOLD_S, |p, v| p.focus_hold_s = v);
        bind!(app, FOCUS_SWITCH_MARGIN, |p, v| p.focus_switch_margin = v);
        bind!(app, FOCUS_SWITCH_S, |p, v| p.focus_switch_s = v);
        bind!(app, HAND_MAX, |p, v| p.hand_max = to_u8(v));
    }
}

/// The settings with `tick_hz` from the fixed timestep (the settings' own without one).
#[allow(clippy::cast_possible_truncation)]
fn effective_params(settings: &InteractionSettings, fixed: Option<&Time<Fixed>>) -> InteractParams {
    let mut p = settings.0;
    if let Some(step) = fixed.map(|f| f.timestep().as_secs_f64()).filter(|s| *s > 0.0) {
        p.tick_hz = (1.0 / step) as f32;
    }
    p
}

/// Rebinds the claim table when the settings or the fixed rate changed.
fn sync_params(
    settings: Res<InteractionSettings>,
    fixed: Option<Res<Time<Fixed>>>,
    mut claims: ResMut<InteractionClaims>,
) {
    let p = effective_params(&settings, fixed.as_deref());
    if *claims.0.params() != p {
        claims.0.set_params(p);
    }
}

/// Orders invalidations by subject then reason (neither is `Ord` in the core).
fn invalidation_key(m: &InvalidateClaims) -> (u8, u128, u128, u16, u8) {
    let (kind, a, b, c) = match m.subject {
        Subject::User(sf_interaction::UserKey::Actor(k)) => (0, 0, k.0, 0),
        Subject::User(sf_interaction::UserKey::Player(k)) => (0, 1, k.0, 0),
        Subject::Slot(s) => (1, 0, s.placement.0, u16::from(s.slot)),
        Subject::Owner(p) => (2, 0, p.0, 0),
    };
    let reason = match m.reason {
        InvalidReason::Preempted => 0,
        InvalidReason::LeaseExpired => 1,
        InvalidReason::OwnerDestroyed => 2,
        InvalidReason::OwnerMoved => 3,
        InvalidReason::OwnerUnloaded => 4,
        InvalidReason::SlotDisabled => 5,
        InvalidReason::UserDied => 6,
        InvalidReason::UserKnockedDown => 7,
        InvalidReason::UserLeftWorldspace => 8,
    };
    (kind, a, b, c, reason)
}

/// The slot a request needs resident, if any.
fn needs_slot(op: &Op) -> Option<SlotRef> {
    match op {
        Op::Claim { slot, .. } | Op::Enqueue { slot, .. } => Some(*slot),
        _ => None,
    }
}

/// One fixed tick of the claim table (module docs, steps 1-4).
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
fn step(
    mut tick: ResMut<InteractionTick>,
    mut claims: ResMut<InteractionClaims>,
    mut index: ResMut<InteractionIndex>,
    mut frames: ResMut<InteractionFrames>,
    mut defs: ResMut<InteractionDefs>,
    mut index_in: MessageReader<IndexChange>,
    mut invalidate_in: MessageReader<InvalidateClaims>,
    mut requests_in: MessageReader<ClaimRequest>,
    mut changed: MessageWriter<ClaimChanged>,
    mut answered: MessageWriter<ClaimAnswered>,
) {
    tick.0 = tick.0.plus(1);
    let now = tick.0;
    let claims = &mut claims.0;
    let index = &mut index.0;

    // 1. Index changes, by placement (stable: send order per placement).
    let mut edits: Vec<&IndexChange> = index_in.read().collect();
    edits.sort_by_key(|c| c.placement());
    for change in edits {
        match change {
            IndexChange::Insert { placement, def, object, owner, disabled } => {
                // A re-insert: holds on slots it switches off or drops end, then it counts as a move.
                let off: Vec<SlotRef> = claims
                    .held_slots(*placement)
                    .filter(|s| disabled.contains(&s.slot) || usize::from(s.slot) >= def.slots.len())
                    .collect();
                for slot in off {
                    claims.invalidate(Subject::Slot(slot), InvalidReason::SlotDisabled, now);
                }
                frames.moved(claims, *placement, object, now);
                frames.current.insert(*placement, *object);
                index.insert_placement(*placement, def, object, owner, |i| !disabled.contains(&i));
                defs.0.insert(*placement, Arc::clone(def));
            }
            IndexChange::Move { placement, object } => {
                frames.moved(claims, *placement, object, now);
                index.move_placement(*placement, object);
            }
            IndexChange::Remove { placement, reason } => {
                frames.current.remove(placement);
                frames.anchors.remove(placement);
                index.remove_placement(*placement);
                defs.0.remove(placement);
                claims.invalidate(Subject::Owner(*placement), *reason, now);
            }
        }
    }

    // 2. Invalidations, by subject then reason.
    let mut ends: Vec<InvalidateClaims> = invalidate_in.read().copied().collect();
    ends.sort_by_key(invalidation_key);
    for end in ends {
        claims.invalidate(end.subject, end.reason, now);
    }

    // 3. The batch; claims on slots that aren't resident are refused here.
    let requests: Vec<Request> = requests_in.read().map(|r| r.0.clone()).collect();
    let mut results: Vec<Result<Granted, Refusal>> = vec![Err(Refusal::Disabled); requests.len()];
    let (live, batch): (Vec<usize>, Vec<Request>) = requests
        .iter()
        .enumerate()
        .filter(|(_, r)| needs_slot(&r.op).is_none_or(|s| index.get(s).is_some()))
        .map(|(i, r)| (i, r.clone()))
        .unzip();
    for (i, result) in live.into_iter().zip(claims.apply_batch(&batch, now)) {
        results[i] = result;
    }

    // 4. Leases; then the anchors follow the holds the tick changed.
    claims.tick(now);
    frames.reconcile(claims);

    answered.write_batch(
        requests.into_iter().zip(results).map(|(request, result)| ClaimAnswered { tick: now, request, result }),
    );
    changed.write_batch(claims.drain_events().map(ClaimChanged));
}
