//! The claim table (interaction.md "Claims"): host only, on the fixed tick. Who holds which slot,
//! who waits for it, and every way a hold ends. The core takes distances and moves as inputs from the
//! caller; it never reads physics. Maps are ordered (`BTreeMap`) so every replay iterates alike.

use std::cmp::Reverse;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sf_ids::{ActorKey, PlayerKey};
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::def::{SlotDef, TagFilter, TagId};
use crate::params::InteractParams;

/// The fixed-tick counter claims and leases count in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tick(pub u64);

impl Tick {
    #[must_use]
    pub const fn plus(self, ticks: u64) -> Self {
        Self(self.0.saturating_add(ticks))
    }
}

/// Who claims: an NPC or a player. Ordered (actors first), the tie-breaker of every sort here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UserKey {
    Actor(ActorKey),
    Player(PlayerKey),
}

/// One slot of one placed object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SlotRef {
    pub placement: PlacementId,
    pub slot: u8,
}

/// What a claim needs to know of its slot, resolved from the def by the caller each request (the
/// adapter may switch a slot off: `enabled`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlotTerms {
    pub group: Option<u8>,
    /// Ticket capacity; `None` = no queue.
    pub queue: Option<u8>,
    /// Hand-claim radius while Occupied, m.
    pub reach: f32,
    pub users: TagFilter,
    pub enabled: bool,
}

impl SlotTerms {
    /// The def's terms, its kind's default queue capacity filled in, enabled.
    #[must_use]
    pub fn of(def: &SlotDef, params: &InteractParams) -> Self {
        let queue = def.queue.or_else(|| def.queues_by_default().then_some(params.queue_capacity));
        Self { group: def.group, queue, reach: def.reach, users: def.users.clone(), enabled: true }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Walking to it; leased.
    Reserved,
    /// In the slot; never preempted, no lease.
    Occupied,
}

/// A body claim on a slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub user: UserKey,
    pub phase: Phase,
    pub priority: u8,
    pub since: Tick,
    /// `None` while Occupied.
    pub lease_until: Option<Tick>,
    /// The slot's terms at claim time (group, queue capacity, reach).
    pub group: Option<u8>,
    pub queue: Option<u8>,
    pub reach: f32,
}

/// A place in a slot's queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ticket {
    pub user: UserKey,
    pub priority: u8,
    pub since: Tick,
    pub lease_until: Tick,
    /// Re-queued after preemption: stays ahead of every ordinary ticket.
    pub head: bool,
}

impl Ticket {
    /// Queue order: head first, then priority (descending), `since`, `UserKey`.
    fn key(&self) -> (Reverse<bool>, Reverse<u8>, Tick, UserKey) {
        (Reverse(self.head), Reverse(self.priority), self.since, self.user)
    }
}

/// A user's one body hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hold {
    Claim(SlotRef),
    Ticket(SlotRef),
}

/// Why a request was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refusal {
    /// The slot is held by an equal or higher priority, or is Occupied.
    Taken,
    /// A slot of the same exclusion group is held and may not be preempted.
    GroupTaken,
    /// The user already holds a body claim or a ticket.
    UserBusy,
    /// The slot's user filter refuses this user.
    Filter,
    /// The slot is switched off.
    Disabled,
    /// The queue is at capacity.
    QueueFull,
    /// The slot has no queue, or the user is a player (players never queue).
    NoQueue,
    /// Only the holder may occupy.
    NotHolder,
    /// A hand claim needs an Occupied body claim.
    NotOccupied,
    /// The hand target is beyond the body slot's `reach`.
    OutOfReach,
    /// `hand_max` hand claims held already.
    HandsFull,
    /// Release or renew without a hold.
    NoHold,
}

/// What a granted request produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Granted {
    Claimed,
    /// A ticket at this queue position (0 = head).
    Queued { position: u8 },
    /// Occupy, release, renew and hand requests.
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvalidReason {
    Preempted,
    LeaseExpired,
    OwnerDestroyed,
    OwnerMoved,
    OwnerUnloaded,
    SlotDisabled,
    UserDied,
    UserKnockedDown,
    UserLeftWorldspace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    Claimed,
    Queued,
    Promoted,
    Entered,
    /// Emitted by the shell when the exit clip starts; the core emits `Released` at its end.
    Exited,
    Released,
    Invalidated(InvalidReason),
}

/// One claim-table change (net rule 8): scripts, audio, barks and crime read them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub tick: Tick,
    pub user: UserKey,
    pub slot: SlotRef,
    pub kind: EventKind,
}

/// What an invalidation covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Subject {
    /// The user's hold (and hand claims) ends; its slot's queue promotes.
    User(UserKey),
    /// The slot's claim and every ticket on it end (no promotion: the slot is gone or off).
    Slot(SlotRef),
    /// Every slot of the object, as `Slot`, and hand claims on it.
    Owner(PlacementId),
}

/// One request of a tick's batch.
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub user: UserKey,
    pub priority: u8,
    pub op: Op,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Claim { slot: SlotRef, terms: SlotTerms, tags: SmallVec<[TagId; 4]> },
    Enqueue { slot: SlotRef, terms: SlotTerms, tags: SmallVec<[TagId; 4]> },
    Occupy { slot: SlotRef },
    Release,
    Renew,
    HandClaim { target: PlacementId, distance_m: f32 },
    HandRelease { target: PlacementId },
}

#[derive(Clone, Debug, PartialEq)]
struct Queue {
    capacity: u8,
    group: Option<u8>,
    reach: f32,
    tickets: SmallVec<[Ticket; 4]>,
}

/// The claim table.
#[derive(Clone, Debug, PartialEq)]
pub struct Claims {
    params: InteractParams,
    by_slot: BTreeMap<SlotRef, Claim>,
    by_user: BTreeMap<UserKey, Hold>,
    queues: BTreeMap<SlotRef, Queue>,
    hands: BTreeMap<UserKey, SmallVec<[PlacementId; 2]>>,
    hand_owner: BTreeMap<PlacementId, UserKey>,
    events: Vec<InteractionEvent>,
}

impl Default for Claims {
    fn default() -> Self {
        Self::new(InteractParams::default())
    }
}

impl Claims {
    #[must_use]
    pub fn new(params: InteractParams) -> Self {
        Self {
            params,
            by_slot: BTreeMap::new(),
            by_user: BTreeMap::new(),
            queues: BTreeMap::new(),
            hands: BTreeMap::new(),
            hand_owner: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    #[must_use]
    pub fn params(&self) -> &InteractParams {
        &self.params
    }

    /// Rebinds the tunables (live edits); running leases keep their deadlines.
    pub fn set_params(&mut self, params: InteractParams) {
        self.params = params;
    }

    #[must_use]
    pub fn claim_on(&self, slot: SlotRef) -> Option<&Claim> {
        self.by_slot.get(&slot)
    }

    #[must_use]
    pub fn hold(&self, user: UserKey) -> Option<Hold> {
        self.by_user.get(&user).copied()
    }

    /// The slot's tickets in queue order.
    #[must_use]
    pub fn queue(&self, slot: SlotRef) -> &[Ticket] {
        self.queues.get(&slot).map_or(&[], |q| q.tickets.as_slice())
    }

    #[must_use]
    pub fn hands(&self, user: UserKey) -> &[PlacementId] {
        self.hands.get(&user).map_or(&[], SmallVec::as_slice)
    }

    #[must_use]
    pub fn hand_holder(&self, target: PlacementId) -> Option<UserKey> {
        self.hand_owner.get(&target).copied()
    }

    #[must_use]
    pub fn events(&self) -> &[InteractionEvent] {
        &self.events
    }

    pub fn drain_events(&mut self) -> std::vec::Drain<'_, InteractionEvent> {
        self.events.drain(..)
    }

    /// Reserve `slot` for `user`: free slot and group, filter passes, user holds nothing. Preempts
    /// Reserved claims of lower priority (on the slot and across its group); never an Occupied one.
    ///
    /// # Errors
    /// `Disabled`, `Filter`, `UserBusy`, `Taken`, `GroupTaken`.
    pub fn claim(
        &mut self,
        slot: SlotRef,
        terms: &SlotTerms,
        user: UserKey,
        tags: &[TagId],
        priority: u8,
        now: Tick,
    ) -> Result<(), Refusal> {
        if !terms.enabled {
            return Err(Refusal::Disabled);
        }
        if !terms.users.matches(tags) {
            return Err(Refusal::Filter);
        }
        if self.by_user.contains_key(&user) {
            return Err(Refusal::UserBusy);
        }
        let mut preempt: SmallVec<[SlotRef; 4]> = SmallVec::new();
        if let Some(c) = self.by_slot.get(&slot) {
            if c.phase == Phase::Occupied || c.priority >= priority {
                return Err(Refusal::Taken);
            }
            preempt.push(slot);
        } else if self.queues.get(&slot).and_then(|q| q.tickets.first()).is_some_and(|t| t.priority >= priority) {
            // A free slot whose queue waits on its group: the head goes first.
            return Err(Refusal::Taken);
        }
        if let Some(g) = terms.group {
            for (s, c) in self.group_claims(slot, g) {
                if c.phase == Phase::Occupied || c.priority >= priority {
                    return Err(Refusal::GroupTaken);
                }
                preempt.push(s);
            }
        }
        for s in preempt {
            self.preempt(s, now);
        }
        self.reserve(slot, terms.group, terms.queue, terms.reach, user, priority, now);
        self.emit(now, user, slot, EventKind::Claimed);
        Ok(())
    }

    /// Claim, or when the slot is taken take a ticket in its queue (NPCs only).
    ///
    /// # Errors
    /// Those of [`Self::claim`]; for a taken slot `NoQueue` (no queue, or a player) or `QueueFull`.
    pub fn enqueue(
        &mut self,
        slot: SlotRef,
        terms: &SlotTerms,
        user: UserKey,
        tags: &[TagId],
        priority: u8,
        now: Tick,
    ) -> Result<Granted, Refusal> {
        match self.claim(slot, terms, user, tags, priority, now) {
            Ok(()) => return Ok(Granted::Claimed),
            Err(Refusal::Taken | Refusal::GroupTaken) => {}
            Err(e) => return Err(e),
        }
        let Some(capacity) = terms.queue else { return Err(Refusal::NoQueue) };
        if matches!(user, UserKey::Player(_)) {
            return Err(Refusal::NoQueue);
        }
        let queue = self.queues.entry(slot).or_insert_with(|| Queue {
            capacity,
            group: terms.group,
            reach: terms.reach,
            tickets: SmallVec::new(),
        });
        queue.capacity = capacity;
        queue.group = terms.group;
        queue.reach = terms.reach;
        if queue.tickets.len() >= usize::from(capacity) {
            if queue.tickets.is_empty() {
                self.queues.remove(&slot);
            }
            return Err(Refusal::QueueFull);
        }
        let ticket = Ticket {
            user,
            priority,
            since: now,
            lease_until: now.plus(self.params.ticket_lease_ticks()),
            head: false,
        };
        let position = insert_ticket(&mut queue.tickets, ticket);
        self.by_user.insert(user, Hold::Ticket(slot));
        self.emit(now, user, slot, EventKind::Queued);
        Ok(Granted::Queued { position: u8::try_from(position).unwrap_or(u8::MAX) })
    }

    /// Reserved → Occupied on entering; holder only. Occupying again is a no-op.
    ///
    /// # Errors
    /// `NotHolder`.
    pub fn occupy(&mut self, slot: SlotRef, user: UserKey, now: Tick) -> Result<(), Refusal> {
        let Some(c) = self.by_slot.get_mut(&slot).filter(|c| c.user == user) else {
            return Err(Refusal::NotHolder);
        };
        if c.phase == Phase::Reserved {
            c.phase = Phase::Occupied;
            c.lease_until = None;
            self.emit(now, user, slot, EventKind::Entered);
        }
        Ok(())
    }

    /// Ends the user's hold (claim or ticket) and its hand claims; a freed slot promotes its queue.
    /// Returns whether there was a hold.
    pub fn release(&mut self, user: UserKey, now: Tick) -> bool {
        self.end_hold(user, EventKind::Released, now)
    }

    /// Progress: a Reserved claim's lease restarts at `interact.claim.lease_s`, a ticket's at
    /// `interact.queue.ticket_lease_s`; Occupied has none. Returns whether there was a hold.
    pub fn renew(&mut self, user: UserKey, now: Tick) -> bool {
        match self.by_user.get(&user).copied() {
            Some(Hold::Claim(slot)) => {
                let until = now.plus(self.params.lease_ticks());
                if let Some(c) = self.by_slot.get_mut(&slot).filter(|c| c.phase == Phase::Reserved) {
                    c.lease_until = Some(until);
                }
                true
            }
            Some(Hold::Ticket(slot)) => {
                let until = now.plus(self.params.ticket_lease_ticks());
                if let Some(t) = self
                    .queues
                    .get_mut(&slot)
                    .and_then(|q| q.tickets.iter_mut().find(|t| t.user == user))
                {
                    t.lease_until = until;
                }
                true
            }
            None => false,
        }
    }

    /// A hand claim on a take/use target `distance_m` from the user's Occupied body slot.
    /// Claiming a target the user already holds is a no-op.
    ///
    /// # Errors
    /// `NotOccupied`, `Taken` (another user's hand), `OutOfReach`, `HandsFull`.
    pub fn hand_claim(&mut self, user: UserKey, target: PlacementId, distance_m: f32) -> Result<(), Refusal> {
        let Some(Hold::Claim(slot)) = self.by_user.get(&user).copied() else {
            return Err(Refusal::NotOccupied);
        };
        let Some(c) = self.by_slot.get(&slot).filter(|c| c.phase == Phase::Occupied) else {
            return Err(Refusal::NotOccupied);
        };
        match self.hand_owner.get(&target) {
            Some(&u) if u == user => return Ok(()),
            Some(_) => return Err(Refusal::Taken),
            None => {}
        }
        if c.reach <= 0.0 || distance_m > c.reach {
            return Err(Refusal::OutOfReach);
        }
        let held = self.hands.entry(user).or_default();
        if held.len() >= usize::from(self.params.hand_max) {
            return Err(Refusal::HandsFull);
        }
        held.push(target);
        self.hand_owner.insert(target, user);
        Ok(())
    }

    /// Put-back. Returns whether the user held it.
    pub fn hand_release(&mut self, user: UserKey, target: PlacementId) -> bool {
        if self.hand_owner.get(&target) != Some(&user) {
            return false;
        }
        self.hand_owner.remove(&target);
        if let Some(held) = self.hands.get_mut(&user) {
            held.retain(|t| *t != target);
            if held.is_empty() {
                self.hands.remove(&user);
            }
        }
        true
    }

    /// Ends holds for `reason`: each ended claim or ticket releases once and emits one event.
    pub fn invalidate(&mut self, subject: Subject, reason: InvalidReason, now: Tick) {
        let kind = EventKind::Invalidated(reason);
        match subject {
            Subject::User(user) => {
                self.end_hold(user, kind, now);
            }
            Subject::Slot(slot) => self.end_slot(slot, kind, now),
            Subject::Owner(placement) => {
                let mut slots: Vec<SlotRef> =
                    self.by_slot.keys().chain(self.queues.keys()).copied().filter(|s| s.placement == placement).collect();
                slots.sort_unstable();
                slots.dedup();
                for slot in slots {
                    self.end_slot(slot, kind, now);
                }
                if let Some(user) = self.hand_owner.get(&placement).copied() {
                    self.hand_release(user, placement);
                }
            }
        }
    }

    /// The owner moved `moved_m` and turned `turned_deg` since the claims were made: beyond
    /// `interact.claim.move_tolerance_m` or `_deg`, its claims end with `OwnerMoved`. Returns whether
    /// they did.
    pub fn owner_moved(&mut self, placement: PlacementId, moved_m: f32, turned_deg: f32, now: Tick) -> bool {
        if moved_m > self.params.move_tolerance_m || turned_deg > self.params.move_tolerance_deg {
            self.invalidate(Subject::Owner(placement), InvalidReason::OwnerMoved, now);
            true
        } else {
            false
        }
    }

    /// Lease expiry at `now`: stale tickets first, then Reserved claims (their queues promote).
    pub fn tick(&mut self, now: Tick) {
        let kind = EventKind::Invalidated(InvalidReason::LeaseExpired);
        let mut stale: Vec<(SlotRef, UserKey)> = Vec::new();
        for (slot, q) in &self.queues {
            stale.extend(q.tickets.iter().filter(|t| t.lease_until <= now).map(|t| (*slot, t.user)));
        }
        for (_, user) in stale {
            self.end_hold(user, kind, now);
        }
        let expired: Vec<UserKey> = self
            .by_slot
            .values()
            .filter(|c| c.lease_until.is_some_and(|t| t <= now))
            .map(|c| c.user)
            .collect();
        for user in expired {
            self.end_hold(user, kind, now);
        }
    }

    /// Applies one tick's requests sorted by (priority descending, `UserKey`, request index), so
    /// co-op conflicts and NPC races resolve identically on every replay. Results in input order.
    pub fn apply_batch(&mut self, batch: &[Request], now: Tick) -> Vec<Result<Granted, Refusal>> {
        let mut order: Vec<usize> = (0..batch.len()).collect();
        order.sort_by_key(|&i| (Reverse(batch[i].priority), batch[i].user, i));
        let mut out = vec![Err(Refusal::NoHold); batch.len()];
        for i in order {
            let r = &batch[i];
            out[i] = match &r.op {
                Op::Claim { slot, terms, tags } => {
                    self.claim(*slot, terms, r.user, tags, r.priority, now).map(|()| Granted::Claimed)
                }
                Op::Enqueue { slot, terms, tags } => self.enqueue(*slot, terms, r.user, tags, r.priority, now),
                Op::Occupy { slot } => self.occupy(*slot, r.user, now).map(|()| Granted::Done),
                Op::Release => done_or(self.release(r.user, now), Refusal::NoHold),
                Op::Renew => done_or(self.renew(r.user, now), Refusal::NoHold),
                Op::HandClaim { target, distance_m } => {
                    self.hand_claim(r.user, *target, *distance_m).map(|()| Granted::Done)
                }
                Op::HandRelease { target } => done_or(self.hand_release(r.user, *target), Refusal::NoHold),
            };
        }
        out
    }

    // --- internals ---

    fn emit(&mut self, tick: Tick, user: UserKey, slot: SlotRef, kind: EventKind) {
        self.events.push(InteractionEvent { tick, user, slot, kind });
    }

    /// Claims on the other slots of `slot`'s object in group `g`, in slot order.
    fn group_claims(&self, slot: SlotRef, g: u8) -> Vec<(SlotRef, &Claim)> {
        self.by_slot
            .iter()
            .filter(|(s, c)| s.placement == slot.placement && **s != slot && c.group == Some(g))
            .map(|(s, c)| (*s, c))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn reserve(
        &mut self,
        slot: SlotRef,
        group: Option<u8>,
        queue: Option<u8>,
        reach: f32,
        user: UserKey,
        priority: u8,
        now: Tick,
    ) {
        let claim = Claim {
            user,
            phase: Phase::Reserved,
            priority,
            since: now,
            lease_until: Some(now.plus(self.params.lease_ticks())),
            group,
            queue,
            reach,
        };
        self.by_slot.insert(slot, claim);
        self.by_user.insert(user, Hold::Claim(slot));
    }

    /// A Reserved claim loses its slot: a ticket at the head of the slot's queue when it has room,
    /// else `Invalidated(Preempted)`.
    fn preempt(&mut self, slot: SlotRef, now: Tick) {
        let Some(c) = self.by_slot.remove(&slot) else { return };
        self.drop_hands(c.user);
        let capacity = c.queue.unwrap_or(0);
        let queue = self.queues.entry(slot).or_insert_with(|| Queue {
            capacity,
            group: c.group,
            reach: c.reach,
            tickets: SmallVec::new(),
        });
        if queue.tickets.len() < usize::from(queue.capacity) {
            let ticket = Ticket {
                user: c.user,
                priority: c.priority,
                since: c.since,
                lease_until: now.plus(self.params.ticket_lease_ticks()),
                head: true,
            };
            insert_ticket(&mut queue.tickets, ticket);
            self.by_user.insert(c.user, Hold::Ticket(slot));
            self.emit(now, c.user, slot, EventKind::Queued);
        } else {
            if queue.tickets.is_empty() {
                self.queues.remove(&slot);
            }
            self.by_user.remove(&c.user);
            self.emit(now, c.user, slot, EventKind::Invalidated(InvalidReason::Preempted));
        }
    }

    /// Ends `user`'s hold with one `kind` event; a freed slot promotes. Returns whether there was one.
    fn end_hold(&mut self, user: UserKey, kind: EventKind, now: Tick) -> bool {
        match self.by_user.remove(&user) {
            Some(Hold::Claim(slot)) => {
                let group = self.by_slot.remove(&slot).and_then(|c| c.group);
                self.drop_hands(user);
                self.emit(now, user, slot, kind);
                self.after_free(slot, group, now);
                true
            }
            Some(Hold::Ticket(slot)) => {
                if let Some(q) = self.queues.get_mut(&slot) {
                    q.tickets.retain(|t| t.user != user);
                    if q.tickets.is_empty() {
                        self.queues.remove(&slot);
                    }
                }
                self.emit(now, user, slot, kind);
                true
            }
            None => false,
        }
    }

    /// Ends the slot's claim and every ticket on it; the group's other queues may then promote.
    fn end_slot(&mut self, slot: SlotRef, kind: EventKind, now: Tick) {
        if let Some(q) = self.queues.remove(&slot) {
            for t in q.tickets {
                self.by_user.remove(&t.user);
                self.emit(now, t.user, slot, kind);
            }
        }
        if let Some(c) = self.by_slot.remove(&slot) {
            self.by_user.remove(&c.user);
            self.drop_hands(c.user);
            self.emit(now, c.user, slot, kind);
            self.after_free(slot, c.group, now);
        }
    }

    fn drop_hands(&mut self, user: UserKey) {
        if let Some(held) = self.hands.remove(&user) {
            for t in held {
                self.hand_owner.remove(&t);
            }
        }
    }

    /// A slot was freed: its queue's head, then the queues of its group's other slots, promote.
    fn after_free(&mut self, slot: SlotRef, group: Option<u8>, now: Tick) {
        self.promote(slot, now);
        if let Some(g) = group {
            let others: Vec<SlotRef> = self
                .queues
                .iter()
                .filter(|(s, q)| s.placement == slot.placement && **s != slot && q.group == Some(g))
                .map(|(s, _)| *s)
                .collect();
            for s in others {
                self.promote(s, now);
            }
        }
    }

    /// The head ticket becomes Reserved with a fresh lease, when its slot and group are free.
    fn promote(&mut self, slot: SlotRef, now: Tick) {
        if self.by_slot.contains_key(&slot) {
            return;
        }
        let Some(q) = self.queues.get(&slot) else { return };
        if q.tickets.is_empty() {
            return;
        }
        if q.group.is_some_and(|g| !self.group_claims(slot, g).is_empty()) {
            return;
        }
        let Some(q) = self.queues.get_mut(&slot) else { return };
        let head = q.tickets.remove(0);
        let (group, capacity, reach) = (q.group, q.capacity, q.reach);
        if q.tickets.is_empty() {
            self.queues.remove(&slot);
        }
        self.reserve(slot, group, Some(capacity), reach, head.user, head.priority, now);
        self.emit(now, head.user, slot, EventKind::Promoted);
    }
}

/// Inserts in queue order; returns the position.
fn insert_ticket(tickets: &mut SmallVec<[Ticket; 4]>, ticket: Ticket) -> usize {
    let key = ticket.key();
    let at = tickets.iter().position(|t| t.key() > key).unwrap_or(tickets.len());
    tickets.insert(at, ticket);
    at
}

fn done_or(ok: bool, refusal: Refusal) -> Result<Granted, Refusal> {
    if ok { Ok(Granted::Done) } else { Err(refusal) }
}
