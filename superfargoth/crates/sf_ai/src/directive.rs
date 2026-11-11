//! Directives: the per-actor stack of content orders, the adapter's [`StackPolicy`] and the
//! completion events scripts poll (ai.md "Directives (the §1b seam)", slice AI4).
//!
//! **Canonical technique:** authored orders on a priority stack with reactive overrides above
//! them (Bethesda AI packages; `OpenMW` `AiSequence`; Unreal's prioritized gameplay tasks that
//! pause and resume lower ones). Content says *what* ([`DirectiveKind`]); drives and the HTN
//! library decide *how*. Bevy-free and exact: every mutation takes the logical clock reading
//! `now` ([`GameTime`]), so the stack's order, its expiries and its events are the same however
//! often the caller steps it (tier invariance, ai.md "Tiers and budget").
//!
//! - **Ids:** [`DirectiveStack::push`] assigns each directive a fresh per-actor [`DirectiveId`]
//!   from a never-reused sequence (saved with the stack, AI5). A repeating directive comes back
//!   as a new instance with a new id, so a script waiting on `done(id)` sees one cycle end.
//! - **Policy:** the adapter's [`StackPolicy`] decides insertion, cancellation and repeat through
//!   a [`StackEdit`]. The modern default, [`ModernPolicy`]: source rank first
//!   ([`DirectiveSource`]: record < schedule < memory < script < quest, so quests and scripts
//!   outrank schedules), then priority, newest first among equals; `repeat` re-pushes the
//!   finished directive through the same rule. Morrowind's (MA2) ports `OpenMW`'s rules over the
//!   same edit API: priority only, cancellation by `canCancel`, repeats sent to the back.
//! - **Limits:** [`Limit::Until`] is an absolute deadline (schedule windows); [`Limit::For`]
//!   is a duration that counts only while the directive is *served* (`OpenMW`'s packages count
//!   their remaining duration only while they execute; a combat interrupt pauses them). The
//!   served directive is the top unless the caller says otherwise ([`Serving`]): AI7's arbiter
//!   sets [`Serving::Suspended`] while a non-directive drive (combat, flee) runs, or names a
//!   lower directive when the top can't be served (Bethesda runs the first package whose
//!   conditions hold).
//! - **Completion:** [`DirectiveStack::finish`] (an executor or script says the order is
//!   fulfilled), expiry and cancellation take the directive off the stack and emit exactly one
//!   [`DirectiveDone`] for it, at the exact logical time it ended (not the time it was noticed).
//!   Every id ever issued ends exactly once; [`DirectiveStack::status`] answers `done(id)`.

use std::collections::VecDeque;
use std::fmt::{self, Write as _};

use serde::{Deserialize, Serialize};
use sf_ids::ActorKey;
use sf_records::RecordId;
use sf_rpg::GameTime;
use sf_world::{Location, PlacementId};

use crate::ids::DirectiveId;

/// Policy reactions one stack operation may cause (an `on_end` that cancels, whose cancellation
/// ends more...). A sane policy needs a handful; past this the operation stops reacting.
pub const MAX_REACTIONS: usize = 256;

/// What a directive orders. Only kinds whose data has a home today; anything else is
/// [`Custom`](DirectiveKind::Custom), a record the adapter's methods understand.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DirectiveKind {
    /// Stay within `radius` metres of `home` (Morrowind's wander).
    StayNear { home: Location, radius: f32 },
    /// Travel to a place, through doors and worldspaces.
    GoTo(Location),
    /// Follow an actor (a player's character is an actor too).
    Follow(ActorKey),
    /// Lead `who` to `to`.
    Escort { who: ActorKey, to: Location },
    /// Use (activate) a placement.
    Use(PlacementId),
    /// Fight an actor (a quest or script order; the Combat drive reads it, AI15).
    Fight(ActorKey),
    /// Anything else: a content record that names the behaviour.
    Custom(RecordId),
}

impl DirectiveKind {
    /// Short name for console lines.
    pub fn name(&self) -> &'static str {
        match self {
            Self::StayNear { .. } => "stay_near",
            Self::GoTo(_) => "go_to",
            Self::Follow(_) => "follow",
            Self::Escort { .. } => "escort",
            Self::Use(_) => "use",
            Self::Fight(_) => "fight",
            Self::Custom(_) => "custom",
        }
    }
}

/// Who pushed a directive. Declared lowest to highest: the order [`ModernPolicy`] ranks them in
/// (ai.md: record defaults, schedules, memory-driven goals, scripts, quests).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectiveSource {
    /// The actor's record (Morrowind's AI packages on the NPC).
    Record,
    /// A schedule window (`sf.ai.schedule`, AI11).
    Schedule,
    /// A memory-driven goal (a vendetta, AI13).
    Memory,
    /// A script.
    Script,
    /// A quest.
    Quest,
}

/// When a directive ends by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Limit {
    /// At this game time, served or not (schedule windows).
    Until(GameTime),
    /// After this many game minutes of being served (paused while suspended or not served).
    For { minutes: u32 },
}

/// A directive before it is on a stack: what content pushes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectiveSpec {
    pub kind: DirectiveKind,
    pub source: DirectiveSource,
    /// Within a source under [`ModernPolicy`]; the whole order under Morrowind's.
    pub priority: u8,
    /// Push a fresh instance when this one completes or its [`Limit::For`] runs out.
    pub repeat: bool,
    pub limit: Option<Limit>,
    /// The content that issued it (package, schedule, script or quest record), for cancelling
    /// a source's directives and for the debugger.
    pub origin: Option<RecordId>,
}

impl DirectiveSpec {
    /// Priority 0, no repeat, no limit, no origin.
    pub fn new(kind: DirectiveKind, source: DirectiveSource) -> Self {
        Self { kind, source, priority: 0, repeat: false, limit: None, origin: None }
    }

    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    #[must_use]
    pub fn repeating(mut self) -> Self {
        self.repeat = true;
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: Limit) -> Self {
        self.limit = Some(limit);
        self
    }

    #[must_use]
    pub fn origin(mut self, origin: impl Into<RecordId>) -> Self {
        self.origin = Some(origin.into());
        self
    }
}

/// A directive on a stack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Directive {
    pub id: DirectiveId,
    pub spec: DirectiveSpec,
    /// Game minutes this instance has been served (what [`Limit::For`] counts).
    pub served: u32,
}

/// Why a directive left the stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectiveEnd {
    /// Fulfilled ([`DirectiveStack::finish`]).
    Completed,
    /// Its [`Limit`] ran out.
    Expired,
    /// Removed by a script, a quest or the policy (including a push the policy refused).
    Cancelled,
}

/// Completion event: plain data the shell turns into a Bevy message. One per issued id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectiveDone {
    pub actor: ActorKey,
    pub id: DirectiveId,
    pub end: DirectiveEnd,
    /// The logical time it ended (an expiry's exact deadline, however late it was noticed).
    pub at: GameTime,
}

/// Which directive the actor is serving: what [`Limit::For`] counts and the drives work on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Serving {
    /// The top of the stack (the default).
    #[default]
    Top,
    /// A directive below the top (the top can't be served now). Falls back to `Top` once that
    /// directive leaves the stack.
    Directive(DirectiveId),
    /// None: a non-directive drive (combat, flee, ...) has the actor.
    Suspended,
}

/// `done(id)` for scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectiveStatus {
    /// On the stack and served.
    Serving,
    /// On the stack, not served.
    Waiting,
    /// Issued and gone (its [`DirectiveDone`] was emitted).
    Ended,
    /// Never issued by this stack.
    Unknown,
}

/// The adapter's stack rules (ai.md "Directives"): insertion, cancellation, repeat. Called by
/// [`DirectiveStack`] only; edits go through [`StackEdit`], so ids and events stay exact.
pub trait StackPolicy: Send + Sync {
    /// Places `d` (its id already assigned) on the stack; may cancel others. Not inserting it
    /// refuses it: it ends `Cancelled` at once.
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive);
    /// `ended` just left the stack for `end`: re-queue a repeat ([`StackEdit::renew`]), update
    /// counters, cancel dependants. Called for every directive that leaves, in event order.
    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd);
}

/// Modern default: source rank, then priority, newest first among equals; `repeat` re-pushes
/// on completion and on a [`Limit::For`] running out (never on cancellation, nor on an
/// absolute deadline, which would expire the copy at once).
#[derive(Clone, Copy, Debug, Default)]
pub struct ModernPolicy;

impl ModernPolicy {
    fn rank(spec: &DirectiveSpec) -> (DirectiveSource, u8) {
        (spec.source, spec.priority)
    }
}

impl StackPolicy for ModernPolicy {
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive) {
        let rank = Self::rank(&d.spec);
        let at = stack.iter().position(|e| Self::rank(&e.spec) <= rank).unwrap_or(stack.len());
        stack.insert(at, d);
    }

    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd) {
        // Not after an absolute deadline (the copy would expire at once), nor a zero-length
        // `For` (it would loop at one instant).
        let repeats = ended.spec.repeat
            && end != DirectiveEnd::Cancelled
            && !matches!(ended.spec.limit, Some(Limit::Until(_) | Limit::For { minutes: 0 }));
        if repeats {
            let again = stack.renew(ended);
            self.push(stack, again);
        }
    }
}

/// The stack as a policy sees it, top first. Removing goes through [`cancel`](Self::cancel)
/// (an event follows); new instances through [`renew`](Self::renew) (fresh id).
pub struct StackEdit<'a> {
    entries: &'a mut Vec<Directive>,
    next_id: &'a mut u32,
    cancelled: Vec<Directive>,
    issued: Vec<DirectiveId>,
}

impl StackEdit<'_> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Directive> {
        self.entries.get(index)
    }

    /// Top first.
    pub fn iter(&self) -> std::slice::Iter<'_, Directive> {
        self.entries.iter()
    }

    /// Inserts at `index` (0 = top; clamped to the bottom).
    pub fn insert(&mut self, index: usize, d: Directive) {
        debug_assert!(self.entries.iter().all(|e| e.id != d.id), "{} is already on the stack", d.id);
        let index = index.min(self.entries.len());
        self.entries.insert(index, d);
    }

    /// Inserts at the bottom (`OpenMW` sends repeats to the back of the queue).
    pub fn push_bottom(&mut self, d: Directive) {
        self.insert(usize::MAX, d);
    }

    /// Removes the directive at `index`; it ends `Cancelled` after this policy call.
    pub fn cancel(&mut self, index: usize) -> Option<DirectiveId> {
        if index >= self.entries.len() {
            return None;
        }
        let d = self.entries.remove(index);
        let id = d.id;
        self.cancelled.push(d);
        Some(id)
    }

    /// Cancels every directive `pred` accepts, top first; returns how many.
    pub fn cancel_where(&mut self, mut pred: impl FnMut(&Directive) -> bool) -> usize {
        let mut n = 0;
        let mut i = 0;
        while i < self.entries.len() {
            if pred(&self.entries[i]) {
                self.cancel(i);
                n += 1;
            } else {
                i += 1;
            }
        }
        n
    }

    /// A fresh instance of `d` (new id, nothing served) for the policy to insert.
    pub fn renew(&mut self, d: &Directive) -> Directive {
        let id = issue(self.next_id);
        self.issued.push(id);
        Directive { id, spec: d.spec.clone(), served: 0 }
    }
}

fn issue(next_id: &mut u32) -> DirectiveId {
    let id = DirectiveId(*next_id);
    *next_id = next_id.checked_add(1).expect("directive ids exhausted");
    id
}

fn minutes(t: GameTime) -> u64 {
    t.total_minutes()
}

fn game_time(minutes: u64) -> GameTime {
    GameTime::from_minutes(minutes)
}

/// A stack's saved form (`sf/ai`, [`crate::save`]): everything but the actor, which the section
/// keys by. Rebuilt into a stack only through [`DirectiveStack::from_saved`], which checks it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedStack {
    /// Top first.
    pub entries: Vec<Directive>,
    /// The next id the stack issues: every id below it was issued (`done(id)` answers from it).
    pub next_id: u32,
    pub clock: GameTime,
    pub serving: Serving,
}

/// A [`SavedStack`] that breaks the stack's invariants.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StackError {
    #[error("{id} is on the stack but the id sequence is only at {next}")]
    NotIssued { id: DirectiveId, next: u32 },
    #[error("{id} is on the stack twice")]
    Duplicate { id: DirectiveId },
    #[error("serving {id}, which was never issued (next id {next})")]
    ServingNotIssued { id: DirectiveId, next: u32 },
}

impl SavedStack {
    /// The invariants every live stack keeps: ids on it were issued (below `next_id`) and are
    /// unique; a named served directive was issued (it may have left: serving falls back to the
    /// top).
    pub fn check(&self) -> Result<(), StackError> {
        let next = self.next_id;
        for (i, d) in self.entries.iter().enumerate() {
            if d.id.0 >= next {
                return Err(StackError::NotIssued { id: d.id, next });
            }
            if self.entries[..i].iter().any(|e| e.id == d.id) {
                return Err(StackError::Duplicate { id: d.id });
            }
        }
        match self.serving {
            Serving::Directive(id) if id.0 >= next => Err(StackError::ServingNotIssued { id, next }),
            _ => Ok(()),
        }
    }
}

/// One actor's directives, top first, with its id sequence and logical clock. Saved as a
/// [`SavedStack`] (AI5); serializes whole for debug dumps. Mutations take `now`, the policy and
/// an event sink.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DirectiveStack {
    actor: ActorKey,
    entries: Vec<Directive>,
    next_id: u32,
    /// Last clock reading: served time is accrued up to here.
    clock: GameTime,
    serving: Serving,
}

impl DirectiveStack {
    /// An empty stack whose clock starts at `now`.
    pub fn new(actor: ActorKey, now: GameTime) -> Self {
        Self { actor, entries: Vec::new(), next_id: 0, clock: now, serving: Serving::Top }
    }

    /// `actor`'s stack from its saved form, refused if it breaks the invariants
    /// ([`SavedStack::check`]).
    pub fn from_saved(actor: ActorKey, saved: SavedStack) -> Result<Self, StackError> {
        saved.check()?;
        let SavedStack { entries, next_id, clock, serving } = saved;
        Ok(Self { actor, entries, next_id, clock, serving })
    }

    /// The saved form (everything but the actor).
    pub fn to_saved(&self) -> SavedStack {
        SavedStack { entries: self.entries.clone(), next_id: self.next_id, clock: self.clock, serving: self.serving }
    }

    pub fn actor(&self) -> ActorKey {
        self.actor
    }

    /// The id the next push or repeat gets.
    pub fn next_id(&self) -> DirectiveId {
        DirectiveId(self.next_id)
    }

    /// Top first.
    pub fn iter(&self) -> std::slice::Iter<'_, Directive> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn top(&self) -> Option<&Directive> {
        self.entries.first()
    }

    pub fn get(&self, id: DirectiveId) -> Option<&Directive> {
        self.entries.iter().find(|d| d.id == id)
    }

    /// The last clock reading the stack was advanced to.
    pub fn clock(&self) -> GameTime {
        self.clock
    }

    /// The serving mode as set ([`serving`](Self::serving) resolves it).
    pub fn serving_mode(&self) -> Serving {
        self.serving
    }

    /// The directive being served: what fills `TraceEntry::directive`.
    pub fn serving(&self) -> Option<&Directive> {
        self.serving_index().map(|i| &self.entries[i])
    }

    /// `done(id)`.
    pub fn status(&self, id: DirectiveId) -> DirectiveStatus {
        match self.entries.iter().position(|d| d.id == id) {
            Some(i) if Some(i) == self.serving_index() => DirectiveStatus::Serving,
            Some(_) => DirectiveStatus::Waiting,
            None if id.0 < self.next_id => DirectiveStatus::Ended,
            None => DirectiveStatus::Unknown,
        }
    }

    fn serving_index(&self) -> Option<usize> {
        match self.serving {
            Serving::Suspended => None,
            Serving::Directive(id) => {
                self.entries.iter().position(|d| d.id == id).or((!self.entries.is_empty()).then_some(0))
            }
            Serving::Top => (!self.entries.is_empty()).then_some(0),
        }
    }

    /// Pushes a new directive through `policy` at `now`; returns its id (ended at once, with an
    /// event, if the policy refused it).
    pub fn push<P: StackPolicy + ?Sized>(
        &mut self,
        spec: DirectiveSpec,
        now: GameTime,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
    ) -> DirectiveId {
        self.advance(now, policy, events);
        let id = issue(&mut self.next_id);
        let d = Directive { id, spec, served: 0 };
        self.react(policy, events, vec![id], |policy, edit| policy.push(edit, d));
        id
    }

    /// Marks `id` fulfilled at `now`: it leaves the stack (`Completed`), the policy may re-queue
    /// it. False if it isn't on the stack (already ended, possibly by expiring before `now`).
    pub fn finish<P: StackPolicy + ?Sized>(
        &mut self,
        id: DirectiveId,
        now: GameTime,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
    ) -> bool {
        self.advance(now, policy, events);
        let Some(i) = self.entries.iter().position(|d| d.id == id) else { return false };
        let d = self.entries.remove(i);
        self.ended(policy, events, d, DirectiveEnd::Completed);
        true
    }

    /// Cancels `id` at `now`. False if it isn't on the stack.
    pub fn cancel<P: StackPolicy + ?Sized>(
        &mut self,
        id: DirectiveId,
        now: GameTime,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
    ) -> bool {
        self.cancel_where(now, policy, events, |d| d.id == id) > 0
    }

    /// Cancels every directive `pred` accepts at `now` (a quest stopping: its origin), top
    /// first; returns how many.
    pub fn cancel_where<P: StackPolicy + ?Sized>(
        &mut self,
        now: GameTime,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
        pred: impl FnMut(&Directive) -> bool,
    ) -> usize {
        self.advance(now, policy, events);
        let mut n = 0;
        self.react(policy, events, Vec::new(), |_, edit| n = edit.cancel_where(pred));
        n
    }

    /// Sets what the actor serves from `now` on (time before `now` counts for the old one).
    pub fn serve<P: StackPolicy + ?Sized>(
        &mut self,
        serving: Serving,
        now: GameTime,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
    ) {
        self.advance(now, policy, events);
        self.serving = serving;
    }

    /// Moves the logical clock to `now`: counts served time and ends what expired on the way, in
    /// time order (ties top first), each at its exact deadline. One call or many smaller ones
    /// give the same stack and events. A `now` before the clock is ignored (the clock never
    /// goes back).
    pub fn advance<P: StackPolicy + ?Sized>(&mut self, now: GameTime, policy: &P, events: &mut Vec<DirectiveDone>) {
        let now = minutes(now);
        // Any number of expiries over a long span (tier 0 fast-forwarding a day of short
        // repeats) but at most MAX_REACTIONS at one instant, so a zero-length loop can't hang.
        let mut same_instant = 0;
        while let Some((at, i)) = self.next_expiry(now) {
            same_instant = if at == minutes(self.clock) { same_instant + 1 } else { 1 };
            if same_instant > MAX_REACTIONS {
                debug_assert!(false, "directive expiries did not settle at {at}");
                break;
            }
            self.accrue(at);
            let d = self.entries.remove(i);
            self.ended(policy, events, d, DirectiveEnd::Expired);
        }
        self.accrue(now);
    }

    /// The earliest deadline at or before `now`, and its index (ties: nearest the top).
    fn next_expiry(&self, now: u64) -> Option<(u64, usize)> {
        let clock = minutes(self.clock);
        let served = self.serving_index();
        let mut best: Option<(u64, usize)> = None;
        for (i, d) in self.entries.iter().enumerate() {
            let at = match d.spec.limit {
                Some(Limit::Until(t)) => minutes(t).max(clock),
                Some(Limit::For { minutes: total }) if Some(i) == served => {
                    clock + u64::from(total.saturating_sub(d.served))
                }
                _ => continue,
            };
            if at <= now && best.is_none_or(|(b, _)| at < b) {
                best = Some((at, i));
            }
        }
        best
    }

    /// Counts the minutes up to `to` for the served directive and moves the clock.
    fn accrue(&mut self, to: u64) {
        let clock = minutes(self.clock);
        if to <= clock {
            return;
        }
        if let Some(i) = self.serving_index() {
            let d = &mut self.entries[i];
            d.served = d.served.saturating_add(u32::try_from(to - clock).unwrap_or(u32::MAX));
        }
        self.clock = game_time(to);
    }

    /// `d` has left the stack: its event, then the policy's reactions.
    fn ended<P: StackPolicy + ?Sized>(
        &mut self,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
        d: Directive,
        end: DirectiveEnd,
    ) {
        events.push(DirectiveDone { actor: self.actor, id: d.id, end, at: self.clock });
        self.react(policy, events, Vec::new(), |policy, edit| policy.on_end(edit, &d, end));
    }

    /// Runs one policy call, then ends what it cancelled or refused (events in order) and lets
    /// the policy react to each of those, breadth first, at most [`MAX_REACTIONS`] times.
    fn react<P: StackPolicy + ?Sized>(
        &mut self,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
        issued: Vec<DirectiveId>,
        call: impl FnOnce(&P, &mut StackEdit<'_>),
    ) {
        let mut pending = VecDeque::new();
        self.edit(policy, events, issued, &mut pending, call);
        for _ in 0..MAX_REACTIONS {
            let Some((d, end)) = pending.pop_front() else { return };
            self.edit(policy, events, Vec::new(), &mut pending, |policy, edit| policy.on_end(edit, &d, end));
        }
        debug_assert!(pending.is_empty(), "directive policy reactions did not settle");
    }

    fn edit<P: StackPolicy + ?Sized>(
        &mut self,
        policy: &P,
        events: &mut Vec<DirectiveDone>,
        issued: Vec<DirectiveId>,
        pending: &mut VecDeque<(Directive, DirectiveEnd)>,
        call: impl FnOnce(&P, &mut StackEdit<'_>),
    ) {
        let mut edit =
            StackEdit { entries: &mut self.entries, next_id: &mut self.next_id, cancelled: Vec::new(), issued };
        call(policy, &mut edit);
        let StackEdit { cancelled, issued, .. } = edit;
        // Issued but neither placed nor cancelled: the policy refused it. It still ends once.
        let refused: Vec<DirectiveId> = issued
            .into_iter()
            .filter(|id| self.entries.iter().all(|d| d.id != *id) && cancelled.iter().all(|d| d.id != *id))
            .collect();
        for d in cancelled {
            events.push(DirectiveDone { actor: self.actor, id: d.id, end: DirectiveEnd::Cancelled, at: self.clock });
            pending.push_back((d, DirectiveEnd::Cancelled));
        }
        for id in refused {
            events.push(DirectiveDone { actor: self.actor, id, end: DirectiveEnd::Cancelled, at: self.clock });
        }
    }
}

impl fmt::Display for GameTimeText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d{} {:02}:{:02}", self.0.day, self.0.minute / 60, self.0.minute % 60)
    }
}

/// `d3 08:30` (day, hour, minute).
struct GameTimeText(GameTime);

/// The stack for `ai.directives`, top first, one line each:
/// `dir4 quest p10 go_to until d2 08:00 [serving]`, then the clock and mode.
pub fn format_directives(stack: &DirectiveStack) -> String {
    let mut out = String::new();
    let served = stack.serving().map(|d| d.id);
    for d in stack {
        let s = &d.spec;
        let source = match s.source {
            DirectiveSource::Record => "record",
            DirectiveSource::Schedule => "schedule",
            DirectiveSource::Memory => "memory",
            DirectiveSource::Script => "script",
            DirectiveSource::Quest => "quest",
        };
        let _ = write!(out, "{} {source} p{} {}", d.id, s.priority, s.kind.name());
        if let DirectiveKind::Custom(id) = &s.kind {
            let _ = write!(out, " {id}");
        }
        match s.limit {
            Some(Limit::Until(t)) => {
                let _ = write!(out, " until {}", GameTimeText(t));
            }
            Some(Limit::For { minutes }) => {
                let _ = write!(out, " for {}/{minutes} min", d.served);
            }
            None => {}
        }
        if s.repeat {
            out.push_str(" repeat");
        }
        if let Some(origin) = &s.origin {
            let _ = write!(out, " from {origin}");
        }
        if Some(d.id) == served {
            out.push_str(" [serving]");
        }
        out.push('\n');
    }
    let mode = match stack.serving_mode() {
        Serving::Top => "top".to_owned(),
        Serving::Directive(id) => id.to_string(),
        Serving::Suspended => "suspended".to_owned(),
    };
    let _ = writeln!(out, "clock {}, serving {mode}, {} directive(s)", GameTimeText(stack.clock()), stack.len());
    out
}

impl<'s> IntoIterator for &'s DirectiveStack {
    type Item = &'s Directive;
    type IntoIter = std::slice::Iter<'s, Directive>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'s> IntoIterator for &'s StackEdit<'_> {
    type Item = &'s Directive;
    type IntoIter = std::slice::Iter<'s, Directive>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
