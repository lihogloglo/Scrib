//! Work budgets for queries and crowd updates (not in recastnavigation).
//!
//! Upstream budgets only the sliced A* search (`updateSlicedFindPath` takes an
//! iteration count). Here every query that can walk an unbounded amount of
//! navmesh has a budgeted form taking a [`Budget`]: the call stops before it
//! would spend more work than the budget has left and says so with
//! [`Budgeted::Exhausted`] or [`Progress::Exhausted`]. Calls without a budget keep
//! upstream's behaviour and signatures.
//!
//! The unit of work is one *expansion*: a polygon popped from a search's open list,
//! a polygon entered by a raycast, a portal examined by the string-pulling funnel,
//! a tile visited by a scan, or a candidate polygon tested by a box query. The
//! cost of one expansion is bounded by the tile format (at most
//! [`VERTS_PER_POLYGON`](crate::detour::VERTS_PER_POLYGON) edges and the links of
//! one polygon), so a budget of `n` bounds the work of a call by a constant times
//! `n`. `docs/VERIFICATION.md` lists the measured work and wall time per call.

use crate::Limits;

/// A work allowance, spent one expansion at a time (not in recastnavigation).
///
/// A budget never goes over its limit: a charge that would exceed it is refused
/// and the operation stops before doing that work, so after any call
/// [`Budget::used`] is at most [`Budget::limit`]. One budget can be shared by
/// several calls (a frame's allowance) or by the resumed steps of one search.
///
/// ```
/// use navmesh_rs::Budget;
///
/// let mut budget = Budget::new(1_000);
/// assert_eq!(budget.remaining(), 1_000);
/// assert!(!budget.is_exhausted());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Budget {
    limit: u64,
    used: u64,
    refused: bool,
}

impl Budget {
    /// A budget of `max_work` expansions.
    pub const fn new(max_work: u64) -> Self {
        Budget {
            limit: max_work,
            used: 0,
            refused: false,
        }
    }

    /// A budget of [`Limits::max_work`], the per-call work limit of a workspace.
    pub const fn from_limits(limits: &Limits) -> Self {
        Budget::new(limits.max_work)
    }

    /// A budget that never runs out.
    pub const fn unlimited() -> Self {
        Budget::new(u64::MAX)
    }

    /// The most work this budget allows.
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Work spent so far; never above [`Budget::limit`].
    pub const fn used(&self) -> u64 {
        self.used
    }

    /// Work left to spend.
    pub const fn remaining(&self) -> u64 {
        self.limit - self.used
    }

    /// Whether a charge has been refused since the budget was created or last
    /// [reset](Budget::reset).
    pub const fn is_exhausted(&self) -> bool {
        self.refused
    }

    /// Starts over with a fresh allowance of `max_work`.
    pub fn reset(&mut self, max_work: u64) {
        *self = Budget::new(max_work);
    }

    /// Charges `n` units; `false` (and nothing spent) when they do not fit.
    #[inline]
    pub(crate) fn spend(&mut self, n: u64) -> bool {
        match self.used.checked_add(n) {
            Some(total) if total <= self.limit => {
                self.used = total;
                true
            }
            _ => {
                self.refused = true;
                false
            }
        }
    }

    /// Marks the whole allowance spent, for work that ran into the limit.
    pub(crate) fn exhaust(&mut self) {
        self.used = self.limit;
        self.refused = true;
    }

    /// Runs `f` and reports whether a charge was refused during it, leaving the
    /// sticky [`Budget::is_exhausted`] flag as it was or set.
    pub(crate) fn scoped<T>(&mut self, f: impl FnOnce(&mut Budget) -> T) -> (T, bool) {
        let before = self.refused;
        self.refused = false;
        let value = f(self);
        let refused = self.refused;
        self.refused = before || refused;
        (value, refused)
    }
}

/// Charges one unit to an optional budget: `true` when the work may go ahead.
#[inline]
pub(crate) fn spend_one(budget: &mut Option<&mut Budget>) -> bool {
    match budget {
        Some(b) => b.spend(1),
        None => true,
    }
}

/// The outcome of a budgeted call (not in recastnavigation).
///
/// `Exhausted` carries what the call had when the budget ran out. What that means
/// is stated on each budgeted method: a path to the closest polygon found so far,
/// the polygons found so far, or no result at all.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Budgeted<T> {
    /// The call finished within the budget; the value equals the unbudgeted call's.
    Complete(T),
    /// The budget ran out first; the value is the partial progress.
    Exhausted(T),
}

impl<T> Budgeted<T> {
    /// Whether the call finished within the budget.
    pub fn is_complete(&self) -> bool {
        matches!(self, Budgeted::Complete(_))
    }

    /// The value, complete or partial.
    pub fn value(&self) -> &T {
        match self {
            Budgeted::Complete(v) | Budgeted::Exhausted(v) => v,
        }
    }

    /// The value, complete or partial.
    pub fn into_value(self) -> T {
        match self {
            Budgeted::Complete(v) | Budgeted::Exhausted(v) => v,
        }
    }

    pub(crate) fn of(value: T, exhausted: bool) -> Self {
        if exhausted {
            Budgeted::Exhausted(value)
        } else {
            Budgeted::Complete(value)
        }
    }
}

/// The state of a resumable search after a step (not in recastnavigation).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Progress {
    /// The search ran to its end; its results are final.
    Done,
    /// The budget ran out first; resume with a new allowance.
    Exhausted,
}

impl Progress {
    /// Whether the search has finished.
    pub fn is_done(self) -> bool {
        self == Progress::Done
    }
}
