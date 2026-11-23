// sf_groundcover_bevy: what the groundcover vertex stages share (groundcover.md "Renderers", GC-2;
// `GroundcoverMaterial`, `material.rs`): the extension's uniform, the instance tag, the dithered
// distance fade and the wind bend (main bending from the wind field plus per-instance detail motion, GC-M2).
// The extension is non-bindless: its uniform is material binding 100 and the wind buffer 101
// (`SF_WIND_BINDING`, set in `GroundcoverWind::specialize`).

import sf_wind_bevy::wind::{wind, wind_sample_gusty, wind_sample_prev_gusty};

const TAU: f32 = 6.28318530718;
// Phase multipliers that decorrelate the cross sway and the flutter from the along-wind sway of the same
// instance (any irrational-looking factors away from 1 do).
// tunables: const (phase decorrelation factor, maths)
const CROSS_PHASE: f32 = 1.7;
// tunables: const (phase decorrelation factor, maths)
const FLUTTER_PHASE: f32 = 3.1;
// Flutter phase per metre of a vertex's horizontal offset from the instance root (rad/m, two incommensurate
// axes): the cards of one clump flutter slightly out of step.
// tunables: const (per-vertex phase decorrelation; the flutter's look is tuned by its amount and Hz)
const FLUTTER_SPREAD: vec2<f32> = vec2<f32>(2.3, 3.7);

struct GroundcoverSpecies {
    // Tint at tag position 0 (rgb), wind response (w).
    tint_a: vec4<f32>,
    // Tint at tag position 255 (rgb), bend height in metres (w).
    tint_b: vec4<f32>,
}

struct GroundcoverParams {
    // Range end (m), fade band (m), shrink distance (m), 1 = fade on.
    fade: vec4<f32>,
    // Wind scale, tip displacement per m/s (fraction of height), phase sample offset (m), minimum
    // gustiness (the gust contrast).
    wind: vec4<f32>,
    // Sway amount (fraction of the wind speed), sway Hz, flutter amount (fraction), flutter Hz. Frequencies
    // are whole cycles per hour (`globals.time` wraps every 3600 s).
    sway: vec4<f32>,
    // Cross-wind sway Hz (whole cycles per hour), unused, unused, unused.
    sway_cross: vec4<f32>,
    species: array<GroundcoverSpecies, 16>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> groundcover: GroundcoverParams;

// `sf_groundcover::Tag`: bits 0-7 wind phase, 8-15 tint, 16-23 fade seed, 24-31 batch slot.
struct GroundcoverTag {
    phase: f32,
    tint: f32,
    fade: f32,
    slot: u32,
}

fn unpack_tag(t: u32) -> GroundcoverTag {
    var out: GroundcoverTag;
    out.phase = f32(t & 0xffu) / 255.0;
    out.tint = f32((t >> 8u) & 0xffu) / 255.0;
    out.fade = f32((t >> 16u) & 0xffu) / 255.0;
    out.slot = min(t >> 24u, 15u);
    return out;
}

fn species_of(tag: GroundcoverTag) -> GroundcoverSpecies {
    return groundcover.species[tag.slot];
}

// The instance's scale factor from the dithered fade: 1 near, shrinking to 0 over the shrink distance
// before the instance's own cut, which its fade seed places in the band before the range end.
fn fade_scale(root: vec3<f32>, view_position: vec3<f32>, seed: f32) -> f32 {
    let f = groundcover.fade;
    if f.w == 0.0 {
        return 1.0;
    }
    let cut = f.x - f.y * seed;
    let d = distance(root, view_position);
    return clamp((cut - d) / max(f.z, 0.001), 0.0, 1.0);
}

// Where the instance reads the gust field: its root, shifted along the prevailing direction by its phase,
// so neighbours sway out of step.
fn wind_point(root: vec3<f32>, tag: GroundcoverTag) -> vec3<f32> {
    let shift = (tag.phase * 2.0 - 1.0) * groundcover.wind.z;
    let axis = wind.params.axis_speed.xy;
    return root + vec3<f32>(axis.x, 0.0, axis.y) * shift;
}

// World-space displacement of a vertex `height` metres above the instance root under wind `v` (m/s):
// tip displacement `bend_per_mps × speed × bend height`, weighted by (height / bend height)², capped at 0.8
// of the height, and lowered on an arc so the plant does not stretch. The bend height is the model's
// (species row) times the instance's scale.
fn bend(height: f32, v: vec3<f32>, sp: GroundcoverSpecies, scale: f32) -> vec3<f32> {
    let ref_h = max(sp.tint_b.w * scale, 0.05);
    let w = clamp(height / ref_h, 0.0, 1.0);
    var off = v.xz * (groundcover.wind.x * groundcover.wind.y * sp.tint_a.w * ref_h * w * w);
    let cap = 0.8 * max(height, 0.0);
    let l = length(off);
    if l > cap {
        off = off * (cap / max(l, 0.0001));
    }
    let l2 = dot(off, off);
    return vec3<f32>(off.x, -l2 / (2.0 * max(height, 0.05)), off.y);
}

// The instance's uniform scale (length of its model matrix's first axis).
fn instance_scale(world_from_local: mat4x4<f32>) -> f32 {
    return length(world_from_local[0].xyz);
}

// Detail motion on the sampled wind `v` (m/s) at time `t` (s): an along-wind sway of the speed, a slower
// cross-wind sway and a fast cross-wind flutter, all scaled by the speed (calm air: no motion). Per-instance
// phase from the tag; `k` is the vertex's own flutter phase. Only the horizontal part feeds `bend`.
fn detail_wind(v: vec3<f32>, tag: GroundcoverTag, t: f32, k: f32) -> vec3<f32> {
    let speed = length(v.xz);
    let dir = v.xz / max(speed, 0.0001);
    let side = vec2<f32>(-dir.y, dir.x);
    let ph = tag.phase * TAU;
    let s = groundcover.sway;
    let along = 1.0 + s.x * sin(TAU * s.y * t + ph);
    let cross = 0.5 * s.x * sin(TAU * groundcover.sway_cross.x * t + CROSS_PHASE * ph);
    let flutter = s.z * sin(TAU * s.w * t + FLUTTER_PHASE * ph + k);
    let xz = v.xz * along + side * (speed * (cross + flutter));
    return vec3<f32>(xz.x, 0.0, xz.y);
}

// The vertex's flutter phase from its horizontal offset from the instance root.
fn vertex_phase(world: vec3<f32>, root: vec3<f32>) -> f32 {
    return dot(world.xz - root.xz, FLUTTER_SPREAD);
}

// This frame's bend of a vertex at `world` (instance root `root`, scale `scale`) at time `t`
// (`globals.time`).
fn bent(world: vec3<f32>, root: vec3<f32>, scale: f32, tag: GroundcoverTag, t: f32) -> vec3<f32> {
    let h = world.y - root.y;
    let v = wind_sample_gusty(wind_point(root, tag), max(h, 0.0), groundcover.wind.w);
    let d = detail_wind(v, tag, t, vertex_phase(world, root));
    return world + bend(h, d, species_of(tag), scale);
}

// Last frame's bend (motion vectors): last frame's field, around this frame's origin, at last frame's time
// `t_prev` (`globals.time - globals.delta_time`; exact across the hourly wrap since every frequency is whole
// cycles per hour).
fn bent_prev(world: vec3<f32>, root: vec3<f32>, scale: f32, tag: GroundcoverTag, t_prev: f32) -> vec3<f32> {
    let h = world.y - root.y;
    let v = wind_sample_prev_gusty(wind_point(root, tag), max(h, 0.0), groundcover.wind.w);
    let d = detail_wind(v, tag, t_prev, vertex_phase(world, root));
    return world + bend(h, d, species_of(tag), scale);
}
