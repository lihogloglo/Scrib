// The floating origin of the active worldspace, for world-stable patterns.
// See `sf_world_bevy::origin` (Rust) and docs/specs/world-coordinates.md.
//
// `rel` below is an origin-relative position: what world-space positions
// derived from `GlobalTransform` hold in every superfargoth shader.

struct WorldOrigin {
    // The floating origin's grid cell (256 m cells).
    cell: vec3<i32>,
    // (cell * 256) mod 1024, per axis, meters.
    phase_1k: vec3<f32>,
    // (cell * 256) mod 16384, per axis, meters.
    phase_16k: vec3<f32>,
};

// World position modulo 1024 m: continuous across origin moves.
fn world_mod_1k(origin: WorldOrigin, rel: vec3<f32>) -> vec3<f32> {
    let p = rel + origin.phase_1k;
    return p - 1024.0 * floor(p / 1024.0);
}

// World position modulo 16384 m: continuous across origin moves.
fn world_mod_16k(origin: WorldOrigin, rel: vec3<f32>) -> vec3<f32> {
    let p = rel + origin.phase_16k;
    return p - 16384.0 * floor(p / 16384.0);
}
