//! NPCs search when their awareness reaches Searching (`AI8f2`; `docs/specs/ai.md` "Perception":
//! the Searching band runs the HTN `search` at the last known position). The core is
//! `sf_ai::search` (`AI8f1`): this module owns one [`SearchState`] per walker ([`Searcher`]) and
//! steps its goals through the existing `MoveTo` / `Face` executors (`sf_ai::OperatorExec`), so
//! [`crate::walkers`]' drive feeds the same `NavCorridor` either way.
//!
//! **Arbitration (the simplest that fits until `AI7`'s drives):** a mode beside the wander, chosen
//! every tick from the strongest awareness entry ([`Searcher::arbitrate`]):
//! - its band is **Searching** and no search ran for this episode → start one at its `LastSeen`
//!   (position, confidence), the actor's resolved `SearchParams` (`AiEffectivePerception`, else the
//!   globals) and the `ai.perception.search_*` pattern; the wander is paused where it was;
//! - fresher evidence (a newer `LastSeen` tick) outside the running search's current radius, about
//!   another target, or after the search gave up → restart at the new position (no new tunable:
//!   the search's own radius decides);
//! - the search reached its ttl (`SearchGoal::Done`, on the ttl tick even mid-walk) or the band left Searching (Alerted
//!   up, or another target's entry got stronger) → the wander resumes, re-choosing from where the search left the actor
//!   (`WanderDriver::face` toward its current yaw, which ends the paused step at once).
//!
//! **The search, not the meter, ends Searching (`AI8f3`):** while a search runs, its target's
//! awareness entry is held at the Searching floor (`AwarenessMeter::hold_searching`, synced by
//! [`Searcher::sync_hold`]); when it gives up, aborts or moves to another target the hold is
//! released on that tick and the entry decays out of Searching from there.
//!
//! Not covered: **Alerted** (latched) actors do not search (combat / report / flee drives come
//! later; an Alerted actor keeps wandering, then searches once it drops to the Searching floor);
//! walkers off the navmesh (fliers, swimmers, the pathgrid fallback) do not search; a running
//! search is not saved yet (ai.md "Saves"); no gizmos (`AI26`).

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use sf_ai::{
    AwarenessBand, AwarenessConfig, AwarenessMeter, ExecConfig, ExecContext, ExecGait, ExecOutput, NavMoveState, OperatorExec,
    OperatorKind, OperatorStatus, PerceptClock, PerceptTarget, SearchGoal, SearchParams, SearchPatternConfig,
    SearchState, search_seed,
};
use sf_ai_bevy::{AiEffectivePerception, AiPerceptClock, AiPerceptionSettings};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_world::WorldspaceId;

use crate::walkers::WanderVocab;

/// Operator transitions one tick may run (a finished `Face` or an already-reached point hands over
/// to the next goal in the same tick).
// tunables: const (loop bound: bounds the work per tick; a search step is at most two operators)
const MAX_TRANSITIONS: usize = 4;

/// What a walker's searches did since spawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SearchStats {
    /// Searches started (restarts included).
    pub started: u32,
    /// `GoTo` goals issued (the last known position and ring points).
    pub goals: u32,
    /// `GoTo` goals reached.
    pub reached: u32,
    /// `GoTo` goals given up (no path, no progress).
    pub failed: u32,
    /// Look-arounds finished.
    pub looks: u32,
    /// Searches that ran out their ttl.
    pub gave_up: u32,
    /// Searches ended because the band left Searching.
    pub dropped: u32,
}

/// A walker's search mode (`AI8f2`), required by [`crate::walkers::Walker`].
#[derive(Component, Debug, Default)]
pub struct Searcher {
    run: Option<Running>,
    /// The target whose awareness entry this searcher holds at the Searching floor (`AI8f3`).
    held: Option<PerceptTarget>,
    pub stats: SearchStats,
}

#[derive(Debug)]
struct Running {
    state: SearchState,
    target: PerceptTarget,
    /// The `LastSeen` tick it started from.
    evidence_tick: u64,
    /// The perception tick it started on.
    started_tick: u64,
    /// Whether `active` is a `MoveTo`.
    walking: bool,
    active: Option<OperatorExec>,
    /// Whether `active` has not been stepped yet (the corridor's state belongs to the last goal).
    fresh: bool,
    /// A look-around's second turn.
    next: Option<OperatorKind>,
}

/// What [`Searcher::arbitrate`] decided for this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SearchMode {
    /// No search: wander.
    Off,
    /// A search runs this tick ([`Searcher::step`]).
    Run,
    /// A running search just ended: resume the wander.
    End,
}

/// What a search start needs besides the meter.
pub(crate) struct SearchSetup<'a> {
    pub params: SearchParams,
    /// The actor's awareness config (the hold settles its entry with it).
    pub awareness: &'a AwarenessConfig,
    pub pattern: &'a SearchPatternConfig,
    pub actor: ActorKey,
    pub now: PerceptClock,
}

/// The drive's search inputs: the perception settings and clock (from `AiSightPlugin`) and the
/// wander vocabulary's move operator ids.
#[derive(SystemParam)]
pub(crate) struct SearchWorld<'w> {
    settings: Option<Res<'w, AiPerceptionSettings>>,
    clock: Option<Res<'w, AiPerceptClock>>,
    vocab: Res<'w, WanderVocab>,
}

impl SearchWorld<'_> {
    /// The setup for `actor` (its resolved profile, else the globals), or `None` without the
    /// perception plugin.
    pub(crate) fn setup<'a>(&'a self, actor: ActorKey, eff: Option<&'a AiEffectivePerception>) -> Option<SearchSetup<'a>> {
        let (settings, clock) = (self.settings.as_deref()?, self.clock.as_deref()?);
        let params = eff.map_or(
            SearchParams {
                ttl_s: settings.search.search_ttl_s,
                error_m: settings.search.search_error_m,
                widen_mps: settings.search.search_widen_mps,
            },
            |e| e.0.search,
        );
        let awareness = eff.map_or(&settings.awareness, |e| &e.0.cfg);
        Some(SearchSetup { params, awareness, pattern: &settings.search_pattern, actor, now: clock.clock() })
    }

    pub(crate) fn vocab(&self) -> &WanderVocab {
        &self.vocab
    }
}

impl Searcher {
    /// Whether a search is running (started and not yet given up).
    pub fn is_searching(&self) -> bool {
        self.run.as_ref().is_some_and(|r| !r.state.is_done())
    }

    /// The running search's last known position.
    pub fn lkp(&self) -> Option<WorldPos> {
        self.run.as_ref().filter(|r| !r.state.is_done()).map(|r| r.state.lkp())
    }

    /// Whether the search is walking to a point this tick.
    pub fn is_walking(&self) -> bool {
        self.run.as_ref().is_some_and(|r| {
            !r.state.is_done() && r.active.is_some() && r.walking
        })
    }

    /// The target a running search holds (`None` once it gave up or ended).
    fn hold_target(&self) -> Option<PerceptTarget> {
        self.run.as_ref().filter(|r| !r.state.is_done()).map(|r| r.target)
    }

    /// Whether [`Self::sync_hold`] has anything to change on `meter` (read-only, so the caller
    /// only takes the meter mutably when it does).
    pub(crate) fn hold_stale(&self, meter: &AwarenessMeter) -> bool {
        let want = self.hold_target();
        want != self.held || want.is_some_and(|t| meter.get(t).is_some_and(|e| !e.search_held()))
    }

    /// Holds the running search's target at the Searching floor and releases a target no
    /// search runs for any more (gave up, aborted, ended, moved on), both at `setup.now`.
    pub(crate) fn sync_hold(&mut self, meter: &mut AwarenessMeter, setup: &SearchSetup<'_>) {
        let want = self.hold_target();
        if let Some(old) = self.held.filter(|&t| Some(t) != want) {
            meter.release_search_hold(setup.awareness, old, setup.now);
        }
        if let Some(t) = want {
            meter.hold_searching(setup.awareness, t, setup.now);
        }
        self.held = want;
    }

    /// Ends a running search without resuming anything (the walker left the navmesh).
    pub(crate) fn abort(&mut self) {
        if self.is_searching() {
            self.stats.dropped += 1;
        }
        self.run = None;
    }

    /// Starts, keeps, restarts or ends the search from `meter`'s strongest entry (module docs).
    pub(crate) fn arbitrate(&mut self, meter: &AwarenessMeter, setup: &SearchSetup<'_>) -> SearchMode {
        let was_searching = self.is_searching();
        let Some(s) = meter.strongest().filter(|s| s.band == AwarenessBand::Searching) else {
            // The episode is over: a later Searching entry starts afresh.
            self.run = None;
            if was_searching {
                self.stats.dropped += 1;
                return SearchMode::End;
            }
            return SearchMode::Off;
        };
        let restart = self.run.as_ref().is_none_or(|r| {
            let fresher = s.last_seen.tick > r.evidence_tick;
            #[allow(clippy::cast_precision_loss)] // tick deltas are small
            let elapsed_s = setup.now.tick.saturating_sub(r.started_tick) as f32 * setup.now.tick_s;
            fresher
                && (r.state.is_done()
                    || s.target != r.target
                    || r.state.lkp().delta_f32(s.last_seen.pos).length() > r.state.radius_at(elapsed_s))
        });
        if restart {
            let tick = setup.now.tick;
            match SearchState::start(
                s.last_seen.pos,
                s.last_seen.confidence,
                setup.params,
                setup.pattern,
                search_seed(setup.actor, tick),
                tick,
            ) {
                Ok(state) => {
                    self.stats.started += 1;
                    self.run = Some(Running {
                        state,
                        target: s.target,
                        evidence_tick: s.last_seen.tick,
                        started_tick: tick,
                        walking: false,
                        active: None,
                        fresh: false,
                        next: None,
                    });
                }
                Err(e) => {
                    bevy_log::warn_once!("AI8f2: search not started: {e}");
                    self.run = None;
                    return if was_searching { SearchMode::End } else { SearchMode::Off };
                }
            }
        }
        if self.is_searching() {
            SearchMode::Run
        } else if was_searching {
            SearchMode::End
        } else {
            SearchMode::Off
        }
    }

    /// Steps the running search by one tick through the move executors; `None` once it gave up
    /// (ttl): resume the wander. `reachable` filters points (a navmesh polygon near them).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn step(
        &mut self,
        ctx: &ExecContext,
        cfg: &ExecConfig,
        vocab: &WanderVocab,
        worldspace: &WorldspaceId,
        speed: f32,
        now: PerceptClock,
        reachable: &mut dyn FnMut(WorldPos) -> bool,
    ) -> Option<ExecOutput> {
        let ops = vocab.0.operators;
        let run = self.run.as_mut().filter(|r| !r.state.is_done())?;
        // Give up at the ttl itself, not when the current walk or turn ends (the core only checks
        // the ttl when asked for the next goal).
        if now.tick.saturating_sub(run.started_tick) >= run.state.ttl_ticks(now.tick_s) {
            run.active = None;
            run.next = None;
        }
        let mut last = ExecOutput::default();
        for _ in 0..MAX_TRANSITIONS {
            if run.active.is_none() {
                let kind = if let Some(k) = run.next.take() {
                    k
                } else {
                    let goal = run.state.next_goal(now, reachable);
                    match goal {
                        SearchGoal::Done => {
                            self.stats.gave_up += 1;
                            return None;
                        }
                        SearchGoal::GoTo(_) => self.stats.goals += 1,
                        SearchGoal::LookAround { .. } => {}
                    }
                    let [first, second] = goal.operators(worldspace, ExecGait::Walk, speed, cfg.face_tolerance_rad);
                    run.next = second;
                    first.expect("a GoTo or LookAround goal has an operator")
                };
                let id = match kind {
                    OperatorKind::MoveTo { .. } => ops.move_to,
                    OperatorKind::Face { .. } => ops.face,
                    OperatorKind::Wait { .. } => ops.wait,
                };
                run.walking = id == ops.move_to;
                run.active = Some(OperatorExec::new(id, kind));
                run.fresh = true;
            }
            let active = run.active.as_mut().expect("set above");
            // A new operator's first step: the corridor's arrival or failure is the last goal's.
            let step_ctx;
            let ctx = if run.fresh && matches!(ctx.nav, NavMoveState::Arrived | NavMoveState::Failed) {
                step_ctx = ExecContext { nav: NavMoveState::Idle, ..ctx.clone() };
                &step_ctx
            } else {
                ctx
            };
            run.fresh = false;
            let out = active.step(ctx, cfg);
            let status = out.operator.map_or(OperatorStatus::Succeeded, |o| o.status);
            if status == OperatorStatus::Running {
                return Some(out);
            }
            match (run.walking, status) {
                (true, OperatorStatus::Succeeded) => self.stats.reached += 1,
                (true, _) => self.stats.failed += 1,
                (false, _) if run.next.is_none() => self.stats.looks += 1,
                (false, _) => {}
            }
            run.active = None;
            last = out;
        }
        Some(last)
    }
}
