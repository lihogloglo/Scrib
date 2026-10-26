//! Montage slots (animation.md "Slots and montages"): one-shot clips that
//! gameplay, AI or scripts play at runtime over the authored graph, as UE's
//! Animation Montages through Slot nodes.
//!
//! - A `Slot` node names a slot (and a slot group). Until something plays
//!   in the slot it passes its input through; then it shows the montage,
//!   over the whole body or through the node's mask.
//! - [`GraphInstance::play_slot`](crate::GraphInstance::play_slot) takes a
//!   [`SlotRequest`]: a clip of the binding (by clip slot, e.g. from
//!   [`CompiledGraph::slot_clip`]) or a late-bound clip ([`LateClip`], any
//!   cooked clip on the skeleton), a start rule (interrupt or queue), rate,
//!   start time, loop count, blend times, root-motion mode and priority.
//! - Blends are inertialized (the graph's post-process inertializer): the
//!   switch into a montage, between queued montages and back to the input
//!   each request one blend. Only one pose is evaluated per slot.
//! - The montage clock always runs (UE: montages tick even when their slot
//!   is not evaluated), so gameplay timing and events don't depend on
//!   which branch of the graph shows the slot. Its point events always fire,
//!   once per crossing, outside the per-layer highest-weight selection.
//!   Its warp windows and `Drive` root motion apply where the slot is
//!   evaluated.
//! - Lifecycle notices ([`SlotNotice`]: started, ended or interrupted) come
//!   out of the next update.

use std::collections::VecDeque;
use std::sync::Arc;

use sf_anim::{Clip, FiredEvent, RootDelta, RootMotionMode, Skeleton};

use crate::compiled::{CompiledGraph, WarpWindow, parse_warp_window};

/// A montage slot of a compiled graph ([`CompiledGraph::slot`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotId(pub u16);

/// One accepted [`SlotRequest`], unique per graph instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MontageId(pub u32);

/// [`GraphEvent::clip`](crate::GraphEvent::clip) of events from a late-bound clip.
pub const LATE_CLIP: u16 = u16::MAX;

/// [`SlotRequest::loops`]: repeat the loop range until stopped or interrupted.
pub const LOOP_FOREVER: u32 = u32::MAX;

/// A clip outside the binding (not named by the graph), mapped onto the
/// graph at runtime: its events and curves by name onto the graph's tables,
/// its `warp` windows onto the graph's warp targets. Build it once per
/// (graph, clip) and share it.
#[derive(Clone, Debug)]
pub struct LateClip {
    pub clip: Arc<Clip>,
    /// Graph event id per clip event (`u16::MAX`: a name the graph doesn't use).
    events: Vec<u16>,
    /// `(clip curve, graph curve)` for the curves the graph knows.
    curves: Vec<(u16, u16)>,
    warp: Vec<WarpWindow>,
}

impl LateClip {
    /// # Errors
    /// The clip is bound to another skeleton, is additive, or has a `warp`
    /// window that doesn't parse or names a target the graph doesn't have.
    pub fn new(graph: &CompiledGraph, skeleton: &Skeleton, clip: Arc<Clip>) -> Result<Self, SlotError> {
        if clip.skeleton != graph.skeleton || clip.skeleton != skeleton.id() {
            return Err(SlotError::Skeleton);
        }
        if clip.meta.additive {
            return Err(SlotError::Additive);
        }
        let events = clip.events.iter().map(|e| graph.event(&e.name).unwrap_or(u16::MAX)).collect();
        let curves =
            clip.curves.iter().enumerate().filter_map(|(i, c)| Some((u16::try_from(i).ok()?, u16::try_from(graph.curve(&c.name)?).ok()?))).collect();
        let mut target =
            |name: &str| graph.warp_target(name).and_then(|t| u16::try_from(t).ok()).ok_or_else(|| format!("the graph has no warp target {name:?}"));
        let warp = clip
            .events
            .iter()
            .filter(|e| e.name == "warp")
            .map(|e| parse_warp_window(skeleton, &clip, e, &mut target))
            .collect::<Result<Vec<_>, String>>()
            .map_err(SlotError::Warp)?;
        Ok(Self { clip, events, curves, warp })
    }
}

/// What a slot plays.
#[derive(Clone, Debug)]
pub enum MontageClip {
    /// A clip of the binding, by clip slot ([`CompiledGraph::slot_clip`]
    /// for the graph's `slot_clips`, or any slot of [`CompiledGraph::clips`]).
    Bound(u16),
    /// A clip the graph doesn't name.
    Late(Arc<LateClip>),
}

/// When a request starts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SlotStart {
    /// Now: what plays in the slot (and in its group's other slots) ends
    /// interrupted, and the queue is dropped. Refused while a montage of
    /// higher priority plays.
    #[default]
    Interrupt,
    /// After the montages playing and queued in the slot (at once when the
    /// slot is free).
    Queue,
}

/// A request to play a clip in a slot.
#[derive(Clone, Debug)]
pub struct SlotRequest {
    pub clip: MontageClip,
    pub start: SlotStart,
    /// Playback rate (≥ 0).
    pub rate: f32,
    /// Clip time to start at, seconds (e.g. the loop start).
    pub from: f32,
    /// Times the clip's loop range repeats before it plays out to the end
    /// (0: straight through; [`LOOP_FOREVER`]). A clip without a loop range
    /// plays once.
    pub loops: u32,
    /// Inertialization time into the montage, seconds.
    pub blend_in: f32,
    /// Inertialization time back to the slot's input when it ends, seconds.
    pub blend_out: f32,
    /// The montage ends this many seconds before the clip's end (UE's blend
    /// out trigger time). 0: it plays to the end.
    pub end_early: f32,
    pub root_motion: RootMotionMode,
    /// An `Interrupt` request is refused while a higher-priority montage plays.
    pub priority: u8,
}

impl SlotRequest {
    /// Interrupt, rate 1, from the start, no loops, 0.2 s blends, `Drive`.
    pub fn new(clip: MontageClip) -> Self {
        Self {
            clip,
            start: SlotStart::Interrupt,
            rate: 1.0,
            from: 0.0,
            loops: 0,
            blend_in: 0.2,
            blend_out: 0.2,
            end_early: 0.0,
            root_motion: RootMotionMode::Drive,
            priority: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SlotError {
    #[error("the graph has no such slot")]
    NoSlot,
    #[error("the binding has no such clip")]
    NoClip,
    #[error("slots play full poses, not additive clips")]
    Additive,
    #[error("the clip is bound to another skeleton")]
    Skeleton,
    #[error("a montage of higher priority is playing")]
    Busy,
    #[error("warp window: {0}")]
    Warp(String),
}

/// A montage's lifecycle step, reported by the update after it happened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoticeKind {
    Started,
    /// Played out (`interrupted == false`), or was stopped, replaced or
    /// dropped from the queue.
    Ended {
        interrupted: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotNotice {
    pub slot: SlotId,
    pub montage: MontageId,
    pub kind: NoticeKind,
}

/// What a slot is playing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MontageStatus {
    pub id: MontageId,
    /// Clip time, seconds.
    pub time: f32,
    pub loops_left: u32,
    /// Requests waiting behind it.
    pub queued: usize,
    /// Seconds since the chain started (montages queued back to back, each played from `from`
    /// to its end): the progress a `Progress` montage was set to (B1).
    pub chain_time: f32,
}

// --- Runtime ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Montage {
    pub id: MontageId,
    pub req: SlotRequest,
    pub time: f32,
    pub prev: f32,
    /// Chain time at which this montage's `from` plays (0 for a montage that starts a chain;
    /// a queued one continues where the previous one ended).
    pub offset: f32,
    /// Clip seconds per second over the last advance.
    pub speed: f32,
    pub root: RootDelta,
    pub loops_left: u32,
    pub fresh: bool,
    /// Reached its end this update: shown once more, then leaves.
    pub ended: bool,
    pub wrapped: bool,
    pub events: Vec<FiredEvent>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SlotState {
    pub playing: Option<Montage>,
    pub queue: VecDeque<(MontageId, SlotRequest)>,
    /// Weight the slot's nodes were reached with this update, and the
    /// top-level layer of the heaviest.
    pub weight: f32,
    pub layer: u16,
    /// An inertialization this slot asks for at the next update.
    pub blend: Option<f32>,
    pub notices: Vec<SlotNotice>,
    /// Window exits of an interrupted montage, for the next update's events.
    pub exits: Vec<crate::GraphEvent>,
    /// Chain time for the next update's `Progress` montages
    /// ([`crate::GraphInstance::set_slot_progress`]).
    pub progress: Option<f32>,
}

pub(crate) fn clip_of<'a>(clips: &'a [Arc<Clip>], c: &'a MontageClip) -> &'a Clip {
    match c {
        MontageClip::Bound(i) => &clips[usize::from(*i)],
        MontageClip::Late(l) => &l.clip,
    }
}

pub(crate) fn curve_map<'a>(g: &'a CompiledGraph, c: &'a MontageClip) -> &'a [(u16, u16)] {
    match c {
        MontageClip::Bound(i) => &g.clip_curves[usize::from(*i)],
        MontageClip::Late(l) => &l.curves,
    }
}

pub(crate) fn warp_windows<'a>(g: &'a CompiledGraph, c: &'a MontageClip) -> &'a [WarpWindow] {
    match c {
        MontageClip::Bound(i) => &g.warp_windows[usize::from(*i)],
        MontageClip::Late(l) => &l.warp,
    }
}

/// `(clip slot or LATE_CLIP, graph event id)` of a montage clip's event.
pub(crate) fn event_ids(g: &CompiledGraph, c: &MontageClip, event: u32) -> (u16, u16) {
    match c {
        MontageClip::Bound(i) => (*i, g.clip_events[usize::from(*i)].get(event as usize).copied().unwrap_or(u16::MAX)),
        MontageClip::Late(l) => (LATE_CLIP, l.events.get(event as usize).copied().unwrap_or(u16::MAX)),
    }
}

impl Montage {
    pub fn new(id: MontageId, req: SlotRequest, clip: &Clip) -> Self {
        let time = req.from.clamp(0.0, clip.duration);
        let loops_left = req.loops;
        Self {
            id,
            req,
            time,
            prev: time,
            offset: 0.0,
            speed: 0.0,
            root: RootDelta::ZERO,
            loops_left,
            fresh: true,
            ended: false,
            wrapped: false,
            events: Vec::new(),
        }
    }

    /// Where the montage ends on its last pass.
    pub fn end(&self, clip: &Clip) -> f32 {
        (clip.duration - self.req.end_early.max(0.0)).max(0.0)
    }

    /// Chain time at which a montage queued behind this one starts.
    pub fn chain_end(&self, clip: &Clip) -> f32 {
        self.offset + (self.end(clip) - self.req.from).max(0.0)
    }

    /// Chain time of the playhead.
    pub fn chain_time(&self) -> f32 {
        self.offset + (self.time - self.req.from)
    }

    /// `Progress` playback: moves the playhead to `to` (clip seconds, clamped to the montage;
    /// either direction), firing the events crossed. Reaching the end ends the montage.
    pub fn seek(&mut self, clip: &Clip, to: f32, dt: f32) {
        let end = self.end(clip);
        let to = to.clamp(0.0, end);
        self.events.clear();
        self.prev = self.time;
        self.wrapped = false;
        let adv = clip.advance(self.time, to - self.time, false, self.fresh, Some(&mut self.events));
        // The clip's advance clamps to its duration, not `end`; land on `to` exactly.
        self.time = to;
        self.root = adv.root;
        if dt > 0.0 {
            self.speed = (self.time - self.prev) / dt;
        }
        self.ended = to >= end;
        self.fresh = false;
    }

    /// `Progress` playback without a progress this update: the playhead holds.
    pub fn hold(&mut self) {
        self.events.clear();
        self.prev = self.time;
        self.root = RootDelta::ZERO;
        self.speed = 0.0;
        self.wrapped = false;
    }

    fn loop_range(&self, clip: &Clip, t: f32) -> Option<(f32, f32)> {
        clip.loop_range.filter(|&(a, b)| self.loops_left > 0 && b - a > 1e-6 && t <= b)
    }

    /// Moves the playhead by `dt` seconds: through the loop range while
    /// loops are left (each pass to the loop end wraps once), then to the
    /// end (or `end_early` before it), where it ends. Events and root
    /// motion come from [`Clip::advance`] over each piece, so a straight
    /// pass is exactly a non-looping player's advance.
    pub fn advance(&mut self, clip: &Clip, dt: f32) {
        let mut rem = dt * self.req.rate.max(0.0);
        let delta = rem;
        let mut t = self.time;
        let mut first = self.fresh;
        let mut root = RootDelta::ZERO;
        self.events.clear();
        self.prev = t;
        self.wrapped = false;
        for _ in 0..64 {
            if let Some((a, b)) = self.loop_range(clip, t) {
                if rem <= 0.0 {
                    break;
                }
                let to_end = b - t;
                if rem < to_end {
                    let adv = clip.advance(t, rem, false, first, Some(&mut self.events));
                    root = root.then(adv.root);
                    t = adv.time;
                    break;
                }
                let adv = clip.advance(t, to_end, false, first, Some(&mut self.events));
                root = root.then(adv.root);
                rem -= to_end;
                t = a;
                first = true;
                self.wrapped = true;
                if self.loops_left != LOOP_FOREVER {
                    self.loops_left -= 1;
                }
                continue;
            }
            let end = self.end(clip).max(t.min(clip.duration));
            let to_end = end - t;
            if rem < to_end {
                let adv = clip.advance(t, rem, false, first, Some(&mut self.events));
                root = root.then(adv.root);
                t = adv.time;
            } else {
                let adv = clip.advance(t, to_end, false, first, Some(&mut self.events));
                root = root.then(adv.root);
                t = end;
                self.ended = true;
            }
            break;
        }
        self.time = t;
        self.root = root;
        if dt > 0.0 {
            self.speed = delta / dt;
        }
        self.fresh = false;
    }

    /// The playhead `shift` seconds ahead (inertialization's lookahead).
    pub fn sample_time(&self, clip: &Clip, shift: f32) -> f32 {
        if shift == 0.0 {
            return self.time;
        }
        let t = self.time + shift * self.speed;
        match self.loop_range(clip, self.time) {
            Some((a, b)) if t > b => a + (t - a).rem_euclid(b - a),
            _ => t.clamp(0.0, self.end(clip).max(self.time)),
        }
    }
}
