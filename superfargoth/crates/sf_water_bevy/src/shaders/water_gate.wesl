// sf_water_bevy: the water chain's visibility gate (CW-10a, `gate.rs`).
// `probe`: the camera's water boxes (render space, from the CPU frustum term), drawn depth-only
// against the view's depth after the opaque pass inside an occlusion query; one 14-vertex
// triangle strip per box, both faces (the CPU runs every pass when the camera is inside a box).
// `args`: the resolved query (samples that passed) -> the indirect arguments of the snapshot,
// SSPR and velocity passes: theirs when any sample passed, zero otherwise. No readback.

import bevy_render::view::View;

struct GateBox {
    lo: vec4<f32>,
    hi: vec4<f32>,
}

struct GateCounts {
    // (velocity vertices, velocity instances, SSPR groups x, SSPR groups y)
    counts: vec4<u32>,
}

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<storage, read> boxes: array<GateBox>;

@group(0) @binding(2) var<storage, read> occlusion: array<u32, 2>;
@group(0) @binding(3) var<storage, read_write> args: array<u32, 12>;
@group(0) @binding(4) var<uniform> gate: GateCounts;

// The unit cube as one triangle strip of 14 vertices (bit masks over the vertex index).
fn strip_corner(v: u32) -> vec3<f32> {
    let b = 1u << v;
    return vec3(f32((0x287au & b) != 0u), f32((0x02afu & b) != 0u), f32((0x31e3u & b) != 0u));
}

@vertex
fn probe(@builtin(vertex_index) v: u32, @builtin(instance_index) i: u32) -> @builtin(position) vec4<f32> {
    let b = boxes[i];
    let p = mix(b.lo.xyz, b.hi.xyz, strip_corner(v));
    return view.clip_from_world * vec4(p, 1.0);
}

// Layout of `args` (u32): snapshot draw [0..4), velocity draw [4..8), SSPR dispatch [8..11).
@compute @workgroup_size(1)
fn gate_args() {
    let k = select(0u, 1u, (occlusion[0] | occlusion[1]) != 0u);
    args[0] = 3u;
    args[1] = k;
    args[2] = 0u;
    args[3] = 0u;
    args[4] = gate.counts.x;
    args[5] = gate.counts.y * k;
    args[6] = 0u;
    args[7] = 0u;
    args[8] = gate.counts.z * k;
    args[9] = gate.counts.w * k;
    args[10] = k;
    args[11] = 0u;
}
