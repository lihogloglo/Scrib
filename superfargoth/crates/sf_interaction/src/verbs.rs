//! Verb ranking and stable focus (interaction.md "Point index and focus", decision 3: RDR2 and
//! Cyberpunk prompts; GTA 6's prompt misfires, gta6.md §2).

use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::claims::SlotRef;
use crate::def::{InteractableDef, SlotKind, TagId};
use crate::params::InteractParams;

/// What a verb acts through: one of the object's slots, or the object itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerbTarget {
    Slot(SlotRef),
    /// Index into the def's slotless `verbs`.
    Slotless { placement: PlacementId, verb: u8 },
}

/// One offered use, as the prompt shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerbChoice {
    pub verb: TagId,
    pub priority: i16,
    pub target: VerbTarget,
}

/// The uses `def` placed as `placement` offers now: slot verbs whose `requires` matches
/// `state_tags` and whose claim would not be refused (`would_claim`, the caller's
/// [`crate::Claims::would_claim`] for this user, tags and priority), then slotless verbs whose
/// `requires` matches. Surface slots are never offered: a put-down point is a claim target, not a
/// use (interaction.md "Errand spots"). Sorted by `priority` descending, then def order (slots, then slotless verbs).
pub fn rank_verbs(
    def: &InteractableDef,
    placement: PlacementId,
    state_tags: &[TagId],
    would_claim: impl Fn(SlotRef) -> bool,
) -> SmallVec<[VerbChoice; 4]> {
    let mut out: SmallVec<[VerbChoice; 4]> = SmallVec::new();
    for (i, slot) in def.slots.iter().enumerate() {
        let Ok(i) = u8::try_from(i) else { break };
        if matches!(slot.kind, SlotKind::Surface { .. }) {
            continue;
        }
        let r = SlotRef { placement, slot: i };
        if slot.verb.requires.matches(state_tags) && would_claim(r) {
            out.push(VerbChoice { verb: slot.verb.verb, priority: slot.verb.priority, target: VerbTarget::Slot(r) });
        }
    }
    for (i, v) in def.verbs.iter().enumerate() {
        let Ok(i) = u8::try_from(i) else { break };
        if v.requires.matches(state_tags) {
            out.push(VerbChoice {
                verb: v.verb,
                priority: v.priority,
                target: VerbTarget::Slotless { placement, verb: i },
            });
        }
    }
    // Stable: equal priorities keep def order.
    out.sort_by_key(|c| std::cmp::Reverse(c.priority));
    out
}

/// One player's focused target, held steady against prompt flicker: a challenger takes focus only
/// after leading the focused target's score by `focus_switch_margin` (a fraction of it) for
/// `focus_switch_s`; while interact is held the focused target is frozen. Pure; the shell feeds it
/// each fixed tick with the focus query's candidates. `T` is the shell's target key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FocusStabilizer<T> {
    current: Option<T>,
    /// The candidate leading by the margin, and for how many consecutive ticks.
    challenger: Option<(T, u64)>,
    held_ticks: u64,
}

impl<T: Copy + Ord> FocusStabilizer<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { current: None, challenger: None, held_ticks: 0 }
    }

    #[must_use]
    pub fn current(&self) -> Option<T> {
        self.current
    }

    /// Whether interact has been held long enough (`focus_hold_s`) to list every verb.
    #[must_use]
    pub fn list_open(&self, params: &InteractParams) -> bool {
        self.held_ticks >= params.focus_hold_ticks()
    }

    /// Drops the focus (the shell's target vanished, the player sat down).
    pub fn clear(&mut self) {
        self.current = None;
        self.challenger = None;
    }

    /// One tick: `candidates` are this tick's targets with scores (higher is better; order free),
    /// `held` whether interact is down. Returns the focused target.
    ///
    /// With nothing focused, or the focused target gone from `candidates`, the best candidate
    /// (ties by `T` order) is focused at once; the dwell guards switches only.
    pub fn update(&mut self, candidates: &[(T, f32)], held: bool, params: &InteractParams) -> Option<T> {
        self.held_ticks = if held { self.held_ticks.saturating_add(1) } else { 0 };
        if held && self.current.is_some() {
            self.challenger = None;
            return self.current;
        }
        let best = candidates
            .iter()
            .filter(|(_, s)| !s.is_nan())
            .copied()
            .max_by(|a, b| a.1.total_cmp(&b.1).then(b.0.cmp(&a.0)));
        let current_score = self.current.and_then(|c| candidates.iter().find(|(t, _)| *t == c).map(|(_, s)| *s));
        let Some(cur_score) = current_score else {
            self.current = best.map(|(t, _)| t);
            self.challenger = None;
            return self.current;
        };
        let Some((best_t, best_s)) = best else { return self.current };
        let leads = Some(best_t) != self.current && best_s > cur_score + cur_score.abs() * params.focus_switch_margin;
        if !leads {
            self.challenger = None;
            return self.current;
        }
        let dwell = match self.challenger {
            Some((t, n)) if t == best_t => n + 1,
            _ => 1,
        };
        if dwell >= params.focus_switch_ticks() {
            self.current = Some(best_t);
            self.challenger = None;
        } else {
            self.challenger = Some((best_t, dwell));
        }
        self.current
    }
}
