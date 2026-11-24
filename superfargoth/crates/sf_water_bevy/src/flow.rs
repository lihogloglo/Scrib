//! River flow on mesh bodies (water.md "Rivers": shading): two-phase flow-advected normals
//! (Vlachos, "Water Flow in Portal 2", SIGGRAPH 2010; UE's river water bodies read a per-vertex
//! velocity the same way). A [`crate::WaterMeshSurface`] whose mesh carries
//! [`crate::mesh::ATTRIBUTE_WATER_FLOW`] gets the `WATER_FLOW` shader def (`material.rs`): the
//! fragment samples the cascades' slopes twice, offset upstream by the flow × each phase's time,
//! the phases half a cycle apart and cross-faded by triangle weights (each fades out as it wraps,
//! so the jump never shows), with a smooth noise offset on the phase per surface point so the
//! cross-fade does not pulse in step everywhere. The normal strengthens and foam rises with speed.
//! Zero flow (lakes, a still ribbon) and meshes without the attribute shade exactly as before.
//!
//! [`phases`] is the CPU twin of the shader's `flow_phases`; [`slots`] fills the uniform.

use bevy_ecs::prelude::*;
use bevy_math::Vec4;
use sf_tunables::{def, Def, Group, Source, Unit};

/// `water.flow.cycle_s`: one advection cycle; each phase's offset grows over it, then wraps.
pub const WATER_FLOW_CYCLE_S: Def<f32> = def!(float "water.flow.cycle_s" = 2.0)
    .range(0.25, 16.0)
    .unit(Unit::Seconds)
    .help("River ripples: seconds of one flow-advection cycle (longer: more stretch on bends, slower pulse)")
    .source(Source::Guess)
    .group(Group::Water);

/// `water.flow.advect_scale`: how far the ripples travel per metre the water flows (1 = with the water).
pub const WATER_FLOW_ADVECT_SCALE: Def<f32> = def!(float "water.flow.advect_scale" = 1.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("River ripples: advection distance per metre of flow (1 = the ripples ride with the water, 0 = still)")
    .source(Source::Reference("water.md Rivers: ripples carried at the flow velocity (Vlachos 2010)"))
    .group(Group::Water);

/// `water.flow.phase_noise`: share of a cycle the per-point noise shifts the phase by.
pub const WATER_FLOW_PHASE_NOISE: Def<f32> = def!(float "water.flow.phase_noise" = 1.0)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("River ripples: share of a cycle the phase noise shifts each point by (0 = the cross-fade pulses everywhere at once)")
    .source(Source::Reference("Vlachos 2010: a noise offset on the phase hides the pulsing"))
    .group(Group::Water);

/// `water.flow.noise_cell_m`: size of the phase noise's cells.
pub const WATER_FLOW_NOISE_CELL_M: Def<f32> = def!(float "water.flow.noise_cell_m" = 16.0)
    .range(1.0, 128.0)
    .unit(Unit::Meters)
    .help("River ripples: cell size of the phase noise (small cells stretch the ripples more)")
    .source(Source::Guess)
    .group(Group::Water);

/// `water.flow.normal_gain`: extra normal strength at [`WATER_FLOW_FOAM_FULL_MPS`] (fast water is rougher).
pub const WATER_FLOW_NORMAL_GAIN: Def<f32> = def!(float "water.flow.normal_gain" = 0.5)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("River ripples: extra normal strength at the full foam speed, linear in speed below it")
    .source(Source::Guess)
    .group(Group::Water);

/// `water.flow.foam_start_mps`: flow speed (m/s) where speed foam starts: about Froude 0.8 on a 0.9 m deep
/// channel (`0.8 sqrt(g d)`), where standing waves begin to break.
pub const WATER_FLOW_FOAM_START_MPS: Def<f32> = def!(float "water.flow.foam_start_mps" = 2.4)
    .range(0.0, 20.0)
    .unit(Unit::None)
    .help("River foam: flow speed in m/s where white water starts (about Froude 0.8 on a 0.9 m channel)")
    .source(Source::Guess)
    .group(Group::Water);

/// `water.flow.foam_full_mps`: flow speed (m/s) where speed foam is full: critical flow on a 0.9 m deep channel
/// (Froude 1, `sqrt(g d)`), where rapids break into hydraulic jumps and entrain air.
pub const WATER_FLOW_FOAM_FULL_MPS: Def<f32> = def!(float "water.flow.foam_full_mps" = 3.0)
    .range(0.1, 30.0)
    .unit(Unit::None)
    .help("River foam: flow speed in m/s where white water is full (rapids; Froude 1 on a 0.9 m channel)")
    .source(Source::Industry(
        "Chanson 2004, The Hydraulics of Open Channel Flow ch. 3-4: white water at supercritical flow and hydraulic jumps, Froude 1 = sqrt(g d) = 3 m/s at 0.9 m",
    ))
    .group(Group::Water);

/// `water.flow.foam`: foam cover at full speed.
pub const WATER_FLOW_FOAM: Def<f32> = def!(float "water.flow.foam" = 0.6)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("River foam: cover at the full foam speed (0 = no speed foam)")
    .source(Source::Guess)
    .group(Group::Water);

/// The river flow's look (bound from [`crate::TUNABLES`] by the host; live both ways).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WaterFlowSettings {
    pub cycle_s: f32,
    pub advect_scale: f32,
    pub phase_noise: f32,
    pub noise_cell_m: f32,
    pub normal_gain: f32,
    pub foam_start_mps: f32,
    pub foam_full_mps: f32,
    pub foam: f32,
}

impl Default for WaterFlowSettings {
    fn default() -> Self {
        Self {
            cycle_s: WATER_FLOW_CYCLE_S.default_value(),
            advect_scale: WATER_FLOW_ADVECT_SCALE.default_value(),
            phase_noise: WATER_FLOW_PHASE_NOISE.default_value(),
            noise_cell_m: WATER_FLOW_NOISE_CELL_M.default_value(),
            normal_gain: WATER_FLOW_NORMAL_GAIN.default_value(),
            foam_start_mps: WATER_FLOW_FOAM_START_MPS.default_value(),
            foam_full_mps: WATER_FLOW_FOAM_FULL_MPS.default_value(),
            foam: WATER_FLOW_FOAM.default_value(),
        }
    }
}

/// The shortest cycle the slots carry (s): the tunable's floor, so a bad bind never divides by zero.
const MIN_CYCLE_S: f32 = 0.25; // tunables: const (maths: the range floor of `water.flow.cycle_s`)

/// The two phases at cycle position `t` (cycles; any real): `[(phase, weight); 2]`, each phase in
/// `[0, 1)`, half a cycle apart, weighted by a triangle that is 0 where the phase wraps and 1
/// half-way, so the weights sum to 1 and the blend of the two offsets is continuous in `t`. The
/// shader's `flow_phases` (`water_material.wesl`) is this.
#[must_use]
pub fn phases(t: f32) -> [(f32, f32); 2] {
    let fract = |x: f32| x - x.floor();
    let weight = |p: f32| 1.0 - (1.0 - 2.0 * p).abs();
    let a = fract(t);
    let b = fract(t + 0.5);
    [(a, weight(a)), (b, weight(b))]
}

/// The uniform's `flow`, `flow2` and `flow3` slots (`water_common.wesl` `WaterParams`): (cycle
/// clock s, wrapped at the cycle; cycle s; advection scale; phase noise share), (phase noise cells
/// per noise period; normal gain; foam start m/s; foam full m/s), (foam cover; 0, 0, 0). The clock
/// wraps on whole cycles, so the shader's phase (`clock / cycle`) never jumps.
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn slots(time: f64, s: &WaterFlowSettings) -> [Vec4; 3] {
    let cycle = if s.cycle_s.is_finite() { s.cycle_s.max(MIN_CYCLE_S) } else { MIN_CYCLE_S };
    let clock = time.rem_euclid(f64::from(cycle)) as f32;
    let start = s.foam_start_mps.max(0.0);
    let full = s.foam_full_mps.max(start + 1e-3);
    [
        Vec4::new(clock.min(cycle), cycle, s.advect_scale.max(0.0), s.phase_noise.clamp(0.0, 1.0)),
        Vec4::new(crate::lace_cells(s.noise_cell_m), s.normal_gain.max(0.0), start, full),
        Vec4::new(s.foam.clamp(0.0, 1.0), 0.0, 0.0, 0.0),
    ]
}
