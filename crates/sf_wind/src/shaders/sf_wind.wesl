// The wind field (weather.md "Wind"): global wind with a curl-noise gust
// field, an altitude profile, and local sources. Pure functions over the
// uploaded data, no bindings: each consumer binds `WindParams` (uniform) and
// `array<WindSource>` (storage) in its own group and calls these, e.g.
//
//     var v = wind_global(params, rel, above_ground);
//     for (var i = 0u; i < wind_source_count(params); i++) {
//         v += wind_source(sources[i], rel);
//     }
//
// `rel` is an origin-relative world position (meters). The CPU twin is
// `sf_wind::WindFrame` (`src/frame.rs`, `src/field.rs`); keep both in step,
// `tests/wgsl_twin.rs` runs this file on the GPU and compares (1e-4).
//
// Gust field after the owner's Megalophobia `assets/shaders/grass_wind.wgsl`
// (MIT OR Apache-2.0, ARCHITECTURE §10 2026-10-12): two octaves of
// value-noise curl advected downwind, a calm bias, a swirl capped so nothing
// leans upwind. Changed for us: an integer (PCG) lattice hash so CPU and GPU
// agree bit for bit, lattices that tile a 1024 m period so the field is
// sampled at `rel + (origin - drift) mod 1024`, uploaded as whole cells +
// a fraction (exact across floating-origin shifts), drift integrated on the
// CPU (speed changes do not jump the field),
// velocity output instead of a blade lean.

struct WindParams {
    // Prevailing direction (unit, xz), mean speed (m/s at the reference
    // height), gustiness 0..1.
    axis_speed: vec4<f32>,
    // Per octave: fractional lattice offset (cells, in [0, 1)), 1 / cell size
    // (1/m), cells per period (an integer).
    octave0: vec4<f32>,
    octave1: vec4<f32>,
    // Whole lattice offsets (cells, integers): octave 0 xz, octave 1 xz. The
    // offset is split so the float math only carries `rel / cell + frac`.
    cells: vec4<f32>,
    // swirl (< 1), gust bias, gust band, source count.
    shape: vec4<f32>,
    // Roughness length z0 (m), 1 / ln(reference / z0 + 1), max profile
    // factor, reference height (m above ground).
    profile: vec4<f32>,
}

struct WindSource {
    // Origin-relative center, kind (0 directional, 1 radial, 2 vortex).
    a: vec4<f32>,
    // Direction (directional) or axis (vortex), unit; strength in m/s with
    // the lifetime envelope applied (radial: + outward, vortex: + right-hand
    // around the axis).
    b: vec4<f32>,
    // Inner radius (full strength), outer radius (zero), unused, unused.
    c: vec4<f32>,
}

struct WindGust {
    // Local flow direction (unit, xz): never more than asin(swirl) off the
    // prevailing axis.
    dir: vec2<f32>,
    // The divergence-free swirl before it is folded into `dir` (|curl| <= 1).
    curl: vec2<f32>,
    // Gust envelope: 0 in a lull, 1 under a gust front.
    gust: f32,
}

// Below this a radial or vortex source scales down linearly (no singularity).
const SOURCE_CORE: f32 = 0.5;
const OCTAVE1_SEED: u32 = 0x9e3779b9u;

// PCG hash (Jarzynski and Olano, "Hash Functions for GPU Rendering", 2020).
fn wind_pcg(v: u32) -> u32 {
    let state = v * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

// Lattice value in [0, 1): exact (24 bits), the same on CPU and GPU.
fn wind_lattice(ix: i32, iy: i32, seed: u32) -> f32 {
    let h = wind_pcg(bitcast<u32>(ix) + wind_pcg(bitcast<u32>(iy) + seed));
    return f32(h >> 8u) * (1.0 / 16777216.0);
}

fn wind_wrap(i: i32, n: i32) -> i32 {
    return ((i % n) + n) % n;
}

// Value noise with its analytic gradient, `(value, d/dx, d/dy)`, on a lattice
// that repeats every `n` cells.
fn wind_noise_d(p: vec2<f32>, base: vec2<i32>, n: i32, seed: u32) -> vec3<f32> {
    let cell = floor(p);
    let f = p - cell;
    let u = f * f * (3.0 - 2.0 * f);
    let du = 6.0 * f * (1.0 - f);
    let ix = i32(cell.x) + base.x;
    let iy = i32(cell.y) + base.y;
    let x0 = wind_wrap(ix, n);
    let x1 = wind_wrap(ix + 1, n);
    let y0 = wind_wrap(iy, n);
    let y1 = wind_wrap(iy + 1, n);
    let a = wind_lattice(x0, y0, seed);
    let b = wind_lattice(x1, y0, seed);
    let c = wind_lattice(x0, y1, seed);
    let d = wind_lattice(x1, y1, seed);
    let k1 = b - a;
    let k2 = c - a;
    let k3 = a - b - c + d;
    return vec3<f32>(
        a + k1 * u.x + k2 * u.y + k3 * u.x * u.y,
        du.x * (k1 + k3 * u.y),
        du.y * (k2 + k3 * u.x),
    );
}

fn wind_smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = clamp((x - e0) / (e1 - e0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

// The gust field at an origin-relative point (xz). Grass and trees use `dir`
// and `gust` directly; `wind_global` turns it into a velocity.
fn wind_gust(w: WindParams, rel_xz: vec2<f32>) -> WindGust {
    let axis = w.axis_speed.xy;
    let p0 = rel_xz * w.octave0.z + w.octave0.xy;
    let p1 = rel_xz * w.octave1.z + w.octave1.xy;
    let n0 = wind_noise_d(p0, vec2<i32>(w.cells.xy), i32(w.octave0.w), 0u);
    let n1 = wind_noise_d(p1, vec2<i32>(w.cells.zw), i32(w.octave1.w), OCTAVE1_SEED);
    // curl(psi) = (dpsi/dz, -dpsi/dx): divergence-free, so the flow closes
    // into cells. Capped to unit length: with swirl < 1 the flow turns at most
    // asin(swirl) off the prevailing wind.
    let raw = vec2<f32>(n0.z, -n0.y) + vec2<f32>(n1.z, -n1.y) * 0.5;
    let curl = raw / max(1.0, length(raw));
    let turned = axis + curl * w.shape.x;
    let dir = turned / max(length(turned), 1e-4);
    let potential = n0.x * 0.7 + n1.x * 0.3;
    let gust = wind_smoothstep(w.shape.y, w.shape.y + w.shape.z, potential);
    return WindGust(dir, curl, gust);
}

// Log wind profile: 0 at the ground, 1 at the reference height, capped.
fn wind_profile(w: WindParams, above_ground: f32) -> f32 {
    let h = max(above_ground, 0.0);
    return min(log(h / w.profile.x + 1.0) * w.profile.y, w.profile.z);
}

// The weather's wind at an origin-relative point, `above_ground` meters over
// the ground (pass `w.profile.w`, the reference height, when unknown).
fn wind_global(w: WindParams, rel: vec3<f32>, above_ground: f32) -> vec3<f32> {
    let speed = w.axis_speed.z;
    if speed <= 0.0 {
        return vec3<f32>(0.0);
    }
    let g = wind_gust(w, rel.xz);
    let s = speed * (1.0 + w.axis_speed.w * (2.0 * g.gust - 1.0)) * wind_profile(w, above_ground);
    return vec3<f32>(g.dir.x * s, 0.0, g.dir.y * s);
}

fn wind_source_count(w: WindParams) -> u32 {
    return u32(w.shape.w);
}

// One local source's velocity at an origin-relative point.
fn wind_source(s: WindSource, rel: vec3<f32>) -> vec3<f32> {
    let r = rel - s.a.xyz;
    let d = length(r);
    let fall = 1.0 - wind_smoothstep(s.c.x, s.c.y, d);
    let k = s.b.w * fall;
    let kind = u32(s.a.w);
    if kind == 1u {
        return r * (k / max(d, SOURCE_CORE));
    }
    if kind == 2u {
        let t = cross(s.b.xyz, r);
        return t * (k / max(length(t), SOURCE_CORE));
    }
    return s.b.xyz * k;
}
