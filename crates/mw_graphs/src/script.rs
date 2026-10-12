//! `PlayGroup` / `LoopGroup` → montage slot requests (animation.md "Slots
//! and montages", adapter hook).
//!
//! UESP: flag 0 lets the current animation finish its cycle and then starts
//! the group from its first frame; 1 starts it at once from its first
//! frame; 2 at once from its loop start. `LoopGroup <group> <count>` plays
//! the group `count` times (the original plays exactly that many; `OpenMW`
//! issue 7641 overshoots by one). Mapping: flag 0 → `Queue`, 1 and 2 →
//! `Interrupt`, flag 2 → `from` = the loop start, `loops` = count − 1.
//!
//! Adapter rules: scripted groups play in the full-body slot at
//! [`SCRIPT_PRIORITY`] (`OpenMW`'s "Scripted" priority over everything), so
//! gameplay montages can't interrupt them; flag 0 queues behind the slot's
//! montages, not behind the locomotion cycle (the framework's `Queue`
//! starts at once in a free slot).

use sf_anim_graph::{CompiledGraph, MontageClip, SlotRequest, SlotStart};

use crate::build::slots;

/// Slot scripted groups play in.
pub const SCRIPT_SLOT: &str = slots::FULL;
/// Priority of scripted groups.
pub const SCRIPT_PRIORITY: u8 = 100;

/// A `PlayGroup` / `LoopGroup` flag.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupStart {
    /// 0: after the current animation.
    #[default]
    Queued,
    /// 1: now, from the first frame.
    Now,
    /// 2: now, from the loop start.
    NowAtLoop,
}

impl GroupStart {
    /// From the script flag (unknown values act as 0, like the original).
    pub fn from_flag(flag: i32) -> Self {
        match flag {
            1 => Self::Now,
            2 => Self::NowAtLoop,
            _ => Self::Queued,
        }
    }
}

/// A scripted group request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupRequest {
    pub group: String,
    pub start: GroupStart,
    /// `LoopGroup`'s count; `None` for `PlayGroup` (plays once).
    pub count: Option<u32>,
}

impl GroupRequest {
    pub fn play(group: impl Into<String>, flag: i32) -> Self {
        Self { group: group.into(), start: GroupStart::from_flag(flag), count: None }
    }

    pub fn looped(group: impl Into<String>, count: u32, flag: i32) -> Self {
        Self { group: group.into(), start: GroupStart::from_flag(flag), count: Some(count) }
    }

    /// The slot request for `clip` (the group's cooked clip, whose loop range is `loop_range`).
    pub fn slot_request(&self, clip: MontageClip, loop_range: Option<(f32, f32)>) -> SlotRequest {
        let mut r = SlotRequest::new(clip);
        r.start = if self.start == GroupStart::Queued { SlotStart::Queue } else { SlotStart::Interrupt };
        if self.start == GroupStart::NowAtLoop {
            r.from = loop_range.map_or(0.0, |(a, _)| a);
        }
        r.loops = self.count.map_or(0, |n| n.saturating_sub(1));
        r.priority = SCRIPT_PRIORITY;
        r
    }
}

/// The graph's clip slot that plays a whole group (case-insensitive), if
/// the graph has it; else the caller cooks the group and binds it late
/// (`sf_anim_graph::LateClip`).
pub fn bound_group(graph: &CompiledGraph, group: &str) -> Option<u16> {
    graph.clips.iter().position(|c| c.reference.eq_ignore_ascii_case(group)).and_then(|i| u16::try_from(i).ok())
}
