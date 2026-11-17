//! superfargoth patch (own: bind group walk once), superfargoth `vendor/README.md` and
//! `docs/research/wgpu-pass-rewalk-2026-11-17.md`.
//!
//! A render pass walks every texture of a bind group each time the group is set
//! ([`set_bind_group`](super::pass::set_bind_group) merges its uses into the pass's usage scope)
//! and each time it is flushed before a draw (`flush_bindings_helper` registers its memory-init
//! actions), which is again whenever a lower-indexed group or the pipeline layout changes. With
//! a bindless group of thousands of textures those repeats dominate `finish()`.
//!
//! With [`set_walk_once`] on, a render pass remembers the bind groups it already merged and the
//! ones whose init actions it already registered, and skips the repeats. Both repeats are no-ops:
//! a bind group is immutable and a render pass's scope only grows, so merging the same uses again
//! changes nothing; discards are only recorded when a pass starts, so a second registration in the
//! same pass finds nothing to clear and only appends duplicate init actions. Off (the default) is
//! the stock code path.
//!
//! The counters are always on (a few relaxed atomic adds per bind group, none per texture).

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::FastHashSet;

static WALK_ONCE: AtomicBool = AtomicBool::new(false);

/// Process-wide: render passes walk each bind group's textures once per pass. Default `false` = stock.
pub fn set_walk_once(on: bool) {
    WALK_ONCE.store(on, Ordering::Relaxed);
}

/// Whether [`set_walk_once`] is on.
#[inline]
pub fn walk_once() -> bool {
    WALK_ONCE.load(Ordering::Relaxed)
}

/// Render-pass bind group walks since the last [`take_counts`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalkCounts {
    /// Bind groups merged into a pass's usage scope (`SetBindGroup` commands that walked).
    pub merges: u64,
    /// Texture entries those merges walked.
    pub merge_textures: u64,
    /// Merges skipped: the pass had already merged that bind group.
    pub merges_skipped: u64,
    /// Texture entries the skipped merges would have walked.
    pub merge_textures_skipped: u64,
    /// Bind groups whose memory-init actions were registered at a flush before a draw.
    pub inits: u64,
    /// Texture entries those registrations walked.
    pub init_textures: u64,
    /// Registrations skipped: the pass had already registered that bind group.
    pub inits_skipped: u64,
    /// Texture entries the skipped registrations would have walked.
    pub init_textures_skipped: u64,
}

static COUNTS: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];

#[inline]
fn count(slot: usize, walked: bool, textures: usize) {
    let slot = if walked { slot } else { slot + 2 };
    COUNTS[slot].fetch_add(1, Ordering::Relaxed);
    COUNTS[slot + 1].fetch_add(textures, Ordering::Relaxed);
}

/// The counts since the last call, reset to zero.
pub fn take_counts() -> WalkCounts {
    let t = |i: usize| COUNTS[i].swap(0, Ordering::Relaxed) as u64;
    WalkCounts {
        merges: t(0),
        merge_textures: t(1),
        merges_skipped: t(2),
        merge_textures_skipped: t(3),
        inits: t(4),
        init_textures: t(5),
        inits_skipped: t(6),
        init_textures_skipped: t(7),
    }
}

/// One render pass's memo, present only while [`walk_once`] is on. Keys are bind group addresses:
/// the encoder's tracker holds every bind group set in the pass, so an address names one bind
/// group for the pass's life.
#[derive(Debug, Default)]
pub(crate) struct PassWalks {
    merged: FastHashSet<usize>,
    init_registered: FastHashSet<usize>,
}

impl PassWalks {
    /// A render pass's memo, or `None` when the switch is off.
    pub(crate) fn for_render_pass() -> Option<Self> {
        walk_once().then(Self::default)
    }
}

/// Whether this `SetBindGroup` must merge the bind group (key `bind_group`, `textures` entries)
/// into the pass's scope. Counts either way.
#[inline]
pub(crate) fn must_merge(walks: &mut Option<PassWalks>, bind_group: usize, textures: usize) -> bool {
    let walk = walks.as_mut().is_none_or(|w| w.merged.insert(bind_group));
    count(0, walk, textures);
    walk
}

/// Whether this flush must register the bind group's memory-init actions. Counts either way.
#[inline]
pub(crate) fn must_register_init(
    walks: &mut Option<PassWalks>,
    bind_group: usize,
    textures: usize,
) -> bool {
    let walk = walks
        .as_mut()
        .is_none_or(|w| w.init_registered.insert(bind_group));
    count(4, walk, textures);
    walk
}
