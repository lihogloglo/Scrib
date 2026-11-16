//! Selection gates (CW-2, ROADMAP 1.25; frame-waste audit 2026-11-16 §2 item 1): a distance-driven
//! selection (streaming rings, terrain CDLOD, far-field residency) re-runs only when one of its sources
//! moved past a location quantum, turned past a rotation quantum, or any other input changed; otherwise
//! the caller keeps last run's result. The industry form is UE5 World Partition's
//! `wp.Runtime.UpdateStreaming.LocationQuantization` / `RotationQuantization`: a streaming update is
//! skipped while the quantised sources hash the same.
//!
//! The gate compares against the sources of the last *run*, not of the last frame, so the result is
//! never staler than one quantum of travel (a slow walk can't creep past it). Whatever the selection
//! must react to besides the sources (data arriving, settings, an origin shift) goes in the `inputs`
//! key; a selection that reads state one frame late (transforms propagated after it) sets
//! [`SelectGate::settle`] so it re-runs that many frames after each input change. Disabled, every
//! call runs and the stored sources are dropped, so enabling it later runs at once.

use glam::{DVec3, Vec3};
use sf_tunables::{AnyDef, Def, Group, Source, def};

/// `select.gate`: the one switch of every CW-2 gate (streaming here, terrain in `sf_terrain_bevy`, the far
/// field in `sf_lod_bevy`; each system's quanta sit next to its settings). Hosts read it for the default.
pub const SELECT_GATE: Def<bool> = def!(bool "select.gate" = false)
    .help("Streaming, terrain and far-field selection re-run only when the camera moved or turned past a quantum or an input changed")
    .source(Source::Owner("2026-11-15: a change stays opt-in, off, until its interleaved A/B wins"))
    .group(Group::ViewDistance);

/// `sf_world`'s tunables (live binding: tunables slice T2).
pub const TUNABLES: &[&dyn AnyDef] =
    &[
        &SELECT_GATE,
        &crate::scheduler::STREAM_SELECT_MOVE_M,
        &crate::scheduler::STREAM_SELECT_TURN_DEG,
        &crate::lod_screen::LOD_BIAS,
        &crate::lod_screen::SIZE_CULL_MIN_PX,
        &crate::lod_screen::SIZE_CULL_MIN_DISTANCE,
    ];

/// The switch and quanta of one gate (each system declares its own tunables and fills this).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GateQuanta {
    pub enabled: bool,
    /// Re-run once a source moved this far from where the last run saw it, meters.
    pub move_m: f32,
    /// Re-run once a source's view direction turned this far, degrees (sources without one ignore it).
    pub turn_deg: f32,
}

impl GateQuanta {
    /// Off: every call runs.
    pub const OFF: Self = Self { enabled: false, move_m: 0.0, turn_deg: 0.0 };
}

/// One source of a selection: a position (any space; a jump, e.g. an origin shift in render space,
/// counts as a move) and its view direction when the selection weighs by it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GateSource {
    pub pos: DVec3,
    pub dir: Option<Vec3>,
}

/// The gate state. `K` is the key of every input besides the sources.
#[derive(Clone, Debug)]
pub struct SelectGate<K = u64> {
    last: Vec<GateSource>,
    inputs: Option<K>,
    /// Extra frames to run after an input change (state read one frame late).
    pub settle: u8,
    pending: u8,
    /// Calls that ran / skipped (tests, diagnostics).
    pub runs: u64,
    pub skips: u64,
}

impl<K> Default for SelectGate<K> {
    fn default() -> Self {
        Self { last: Vec::new(), inputs: None, settle: 0, pending: 0, runs: 0, skips: 0 }
    }
}

impl<K: PartialEq> SelectGate<K> {
    /// A gate that re-runs `settle` extra frames after each input change.
    pub fn with_settle(settle: u8) -> Self {
        Self { settle, ..Self::default() }
    }

    /// Whether the selection must run this frame; records the sources and inputs when it does.
    pub fn check(&mut self, q: &GateQuanta, sources: &[GateSource], inputs: K) -> bool {
        if !q.enabled {
            self.last.clear();
            self.inputs = None;
            self.runs += 1;
            return true;
        }
        let inputs_changed = self.inputs.as_ref() != Some(&inputs);
        if inputs_changed {
            self.pending = self.settle;
        }
        let run = inputs_changed || self.pending > 0 || self.moved(q, sources);
        if !inputs_changed && self.pending > 0 {
            self.pending -= 1;
        }
        if run {
            self.last.clear();
            self.last.extend_from_slice(sources);
            self.inputs = Some(inputs);
            self.runs += 1;
        } else {
            self.skips += 1;
        }
        run
    }

    /// Forget the last run (the next call runs).
    pub fn reset(&mut self) {
        self.last.clear();
        self.inputs = None;
    }

    fn moved(&self, q: &GateQuanta, sources: &[GateSource]) -> bool {
        if sources.len() != self.last.len() {
            return true;
        }
        let move_m = f64::from(q.move_m.max(0.0));
        let cos_turn = q.turn_deg.max(0.0).to_radians().cos();
        sources.iter().zip(&self.last).any(|(now, was)| {
            now.pos.distance(was.pos) >= move_m
                || match (now.dir, was.dir) {
                    (None, None) => false,
                    (Some(a), Some(b)) => a.normalize_or_zero().dot(b.normalize_or_zero()) < cos_turn,
                    _ => true,
                }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: GateQuanta = GateQuanta { enabled: true, move_m: 1.0, turn_deg: 10.0 };

    fn at(x: f64) -> [GateSource; 1] {
        [GateSource { pos: DVec3::new(x, 0.0, 0.0), dir: None }]
    }

    #[test]
    fn still_source_skips_and_a_quantum_of_travel_runs() {
        let mut g = SelectGate::<u64>::default();
        assert!(g.check(&Q, &at(0.0), 0), "first call runs");
        assert!(!g.check(&Q, &at(0.0), 0), "still: skipped");
        // Creeping: compared with the last run, not the last frame.
        assert!(!g.check(&Q, &at(0.5), 0));
        assert!(!g.check(&Q, &at(0.9), 0));
        assert!(g.check(&Q, &at(1.0), 0), "a quantum from the last run");
        assert!(!g.check(&Q, &at(1.5), 0));
        assert_eq!((g.runs, g.skips), (2, 4));
    }

    #[test]
    fn inputs_turns_count_and_disabled_always_runs() {
        let mut g = SelectGate::<u64>::default();
        assert!(g.check(&Q, &at(0.0), 0));
        assert!(g.check(&Q, &at(0.0), 1), "an input changed");
        assert!(!g.check(&Q, &at(0.0), 1));
        let dir = |deg: f32| [GateSource { pos: DVec3::ZERO, dir: Some(Vec3::new(deg.to_radians().cos(), 0.0, deg.to_radians().sin())) }];
        assert!(g.check(&Q, &dir(0.0), 1), "a direction appeared");
        assert!(!g.check(&Q, &dir(9.0), 1));
        assert!(g.check(&Q, &dir(11.0), 1), "turned past the quantum");
        assert!(g.check(&Q, &[], 1), "a source left");
        for _ in 0..3 {
            assert!(g.check(&GateQuanta::OFF, &at(0.0), 1), "off: runs every time");
        }
        assert!(g.check(&Q, &at(0.0), 1), "re-enabled: runs at once");
    }

    #[test]
    fn settle_runs_extra_frames_after_an_input_change() {
        let mut g = SelectGate::<u64>::with_settle(1);
        assert!(g.check(&Q, &at(0.0), 0));
        assert!(g.check(&Q, &at(0.0), 0), "settle frame");
        assert!(!g.check(&Q, &at(0.0), 0));
        assert!(g.check(&Q, &at(0.0), 7));
        assert!(g.check(&Q, &at(0.0), 7), "settle frame");
        assert!(!g.check(&Q, &at(0.0), 7));
    }
}
