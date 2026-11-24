// sf_water_bevy: what the water draw and the velocity redraw share (water.md "Mesh",
// "Floating origin"): the clipmap's vertex placement and the cascade lookups.

struct WaterParams {
    phase01: vec4<f32>,
    phase23: vec4<f32>,
    sizes: vec4<f32>,
    lambda_min: vec4<f32>,
    absorption: vec4<f32>,
    scattering: vec4<f32>,
    clip: vec4<f32>,
    foam: vec4<f32>,
    misc: vec4<f32>,
    // (swell scale, chop scale, bounded 0/1, shore field on 0/1).
    waves: vec4<f32>,
    // Render-space XZ rectangle (min x, min z, max x, max z) of a bounded body.
    bounds: vec4<f32>,
    // Shore field: render-space XZ of its corner, 1 / its extent (m).
    shore: vec4<f32>,
    // (range m, swell ramp m, chop ramp m, chop floor).
    shore2: vec4<f32>,
    // (time s, shore foam band m, shore normal floor, shore foam of the body's own look).
    misc2: vec4<f32>,
    // Optics map: render-space XZ of cell (0, 0)'s centre, 1 / cell size, on 0/1.
    regions: vec4<f32>,
    // SSPR: (wave distortion; reflection texel scale, 0 = off for this surface; plane level; 0).
    reflection: vec4<f32>,
    // Shore waves (W-GERSTNER, `sf_water::ShoreWaves`): (ω per train rad/s ×3, previous frame's time s);
    // (offshore amplitude m, swash m, beach slope, band m); (land margin m, variation, noise origin
    // x, z m: the floating origin modulo `sf_water::shore_waves::NOISE_PERIOD`).
    swash: vec4<f32>,
    swash2: vec4<f32>,
    swash3: vec4<f32>,
    // A sea-state transition (W-WIND, `sf_water::ShoreWaves::blend`): (ω of the trains crossfaded
    // from ×3, weight of `swash.xyz` against them; 1 = none).
    swash4: vec4<f32>,
    // Shore foam (WA-6): (breaking ratio of a full band, `sf_water::ShoreSettings::foam_full_breaking`;
    // breaker foam on 0/1 (SF-1, `--off surffoam` = 0); its lace's noise origin x, z m: the floating
    // origin modulo `sf_water::shore_waves::NOISE_PERIOD`, set on every surface).
    shore3: vec4<f32>,
    // Breaker foam (SF-1, `sf_water::SurfFoam`): (birth start, birth full, bore decay rad, front rad);
    // (lace cells per noise period, lace width fresh, aged, feather in cells); (lace variation,
    // albedo fresh, aged, aged tint); (aged opacity, glow, relief m, swash residue rad); (contact
    // on calm water, contact line share, swash spread, 0).
    surf: vec4<f32>,
    surf2: vec4<f32>,
    surf3: vec4<f32>,
    surf4: vec4<f32>,
    surf5: vec4<f32>,
    // Rain rings (R1, `sf_water_bevy::WaterRain`): (intensity 0..1, 0 = no rings or switched off;
    // ring clock s; strength; 0); (fade near m, fade far m of pixel footprint; 0, 0).
    rain: vec4<f32>,
    rain2: vec4<f32>,
    // River flow (`sf_water_bevy::WaterFlowSettings`, read only with `WATER_FLOW`): (cycle clock s,
    // cycle s, advection scale, phase noise share); (phase noise cells per noise period, normal
    // gain, foam start m/s, foam full m/s); (foam cover at full speed, 0, 0, 0).
    flow: vec4<f32>,
    flow2: vec4<f32>,
    flow3: vec4<f32>,
    // Surf shape (SF-2, `sf_water::SurfShape`; its height gain is already in `swash2.x`): (break
    // sharpness p when on, 0 = off (`--off surfskew`); breaker γ; steepen start; bore start); (bore full; skew at the end of the
    // steepening; skew of the full bore; bore loop cap); (minimum crest height at the break line m;
    // gap cells per noise period; gap share; gap feather).
    shape: vec4<f32>,
    shape2: vec4<f32>,
    shape3: vec4<f32>,
    // Optics table for the map, 3 vec4 per entry: (absorption, whitecaps), (scattering, contact
    // depth), (roughness, shore foam, 0, 0).
    looks: array<vec4<f32>, 24>,
}

// Shore field sample at render-space `xz`: (signed distance to the shoreline m, openness,
// shoreward direction xz; `sf_water::ShoreField::encode_rgba16f`); open deep water outside the
// field or without one.
fn shore_at(p: WaterParams, tex: texture_2d<f32>, s: sampler, xz: vec2<f32>) -> vec4<f32> {
    if p.waves.w < 0.5 {
        return vec4(1e4, 1.0, 0.0, 0.0);
    }
    let uv = (xz - p.shore.xy) * p.shore.zw;
    if any(uv < vec2(0.0)) || any(uv > vec2(1.0)) {
        return vec4(p.shore2.x, 1.0, 0.0, 0.0);
    }
    return textureSampleLevel(tex, s, uv, 0.0);
}

// Local bed slope β at render-space `xz` (W-GERSTNER-DEPTH; `sf_water::ShoreField::sample_slope`
// is the CPU twin): the baked `R16Float` over the shore field's extent, or one texel of the beach
// slope without bathymetry; the beach slope (`swash2.z`) outside the field.
fn shore_slope(p: WaterParams, tex: texture_2d<f32>, s: sampler, xz: vec2<f32>) -> f32 {
    let uv = (xz - p.shore.xy) * p.shore.zw;
    if p.waves.w < 0.5 || any(uv < vec2(0.0)) || any(uv > vec2(1.0)) {
        return p.swash2.z;
    }
    return textureSampleLevel(tex, s, uv, 0.0).x;
}

// ---- Shore waves (W-GERSTNER): `sf_water::shore_waves` is the CPU twin and the reference for
// every constant and law (Eckart dispersion over the beach slope, Green's-law shoaling capped by
// the McCowan breaker, finite-depth Gerstner orbits under a no-loop limit, Stockdon swash). ----

const SW_SHARES = vec3(0.5, 0.3, 0.2);
const SW_PHASE_SPREAD = vec3(1.0, -0.7, 1.3);
const SW_BREAKER = 0.78;
const SW_MICHE = 0.44610616;
const SW_STEEPNESS = 0.75;
const SW_ORBIT = 0.4;
const SW_MIN_DEPTH = 0.1;
const SW_PHASE_NOISE = 0.8;
const SW_SET_FLOOR = 0.6;
const SW_SKEW = 0.5;
const SW_ZONE = 0.3;
// tunables: const (the twin of sf_water SKEW_RATE_LIMIT, a no-fold bound)
const SW_SKEW_RATE_LIMIT = 0.1;
const SW_ECKART_LINEAR = 4.0;
const SW_GRAVITY = 9.81;
const SW_REPEAT = 256.0;
const SW_TAU = 6.2831853;

fn sw_hash(x: u32, y: u32, seed: u32) -> f32 {
    var h = (x * 0x8da6b343u) ^ (y * 0xd8163841u) ^ (seed * 0xcb1ab31fu);
    h ^= h >> 16u;
    h *= 0x7feb352du;
    h ^= h >> 15u;
    h *= 0x846ca68bu;
    h ^= h >> 16u;
    return f32(h >> 8u) / 16777216.0;
}

fn sw_noise(uv: vec2<f32>, cells: u32, seed: u32) -> f32 {
    let g = uv * f32(cells);
    let i = floor(g);
    let f = g - i;
    let x0 = u32(i.x) % cells;
    let y0 = u32(i.y) % cells;
    let x1 = (x0 + 1u) % cells;
    let y1 = (y0 + 1u) % cells;
    let s = f * f * (vec2(3.0) - 2.0 * f);
    let top = sw_hash(x0, y0, seed) + (sw_hash(x1, y0, seed) - sw_hash(x0, y0, seed)) * s.x;
    let bottom = sw_hash(x0, y1, seed) + (sw_hash(x1, y1, seed) - sw_hash(x0, y1, seed)) * s.x;
    return top + (bottom - top) * s.y;
}

fn sw_eckart_closed(u: f32) -> f32 {
    let v = sqrt(tanh(u));
    return atanh(v) + atan(v);
}

fn sw_eckart(u: f32) -> f32 {
    if u < SW_ECKART_LINEAR {
        return sw_eckart_closed(u);
    }
    return sw_eckart_closed(SW_ECKART_LINEAR) + u - SW_ECKART_LINEAR;
}

// (local wave number rad/m, phase Φ rad) of `omega` at depth `h` on slope `beta`.
fn sw_local_wave(omega: f32, h: f32, beta: f32) -> vec2<f32> {
    let k0 = omega * omega / SW_GRAVITY;
    let k = k0 / sqrt(tanh(k0 * max(h, SW_MIN_DEPTH)));
    if h >= SW_MIN_DEPTH {
        return vec2(k, sw_eckart(k0 * h) / beta);
    }
    return vec2(k, (sw_eckart(k0 * SW_MIN_DEPTH) - k * (SW_MIN_DEPTH - h)) / beta);
}

struct ShoreWave {
    // Displacement (x, y, z), m.
    offset: vec3<f32>,
    // (∂y/∂x, ∂y/∂z, ∂Dx/∂x, ∂Dz/∂z), added to the FFT's slopes.
    slope: vec4<f32>,
    // sin of the main train's phase (1 on its crests).
    crest: f32,
    // How close the trains come to breaking, 0-1 (`sf_water::ShoreWave::breaking`).
    breaking: f32,
    // cos of the main train's phase (SF-1: with `crest`, the phase the breaker foam trails).
    crest_cos: f32,
    // The main train's phase θ rad (the dominant set's through a transition; counts the waves).
    phase: f32,
    // Swash (SF-1): (weight 0-1, swash phase g rad; `sf_water::ShoreWave::swash`).
    swash: vec2<f32>,
    // Surf lifecycle (SF-2): (phase skew s, bore weight 0-1 with the break gaps;
    // `sf_water::ShoreWave::lifecycle`).
    lifecycle: vec2<f32>,
}

// Share of the shore foam band at breaking ratio `breaking`, full from `full`: the local wave
// energy, ∝ ratio² (WA-6; `sf_water::shore_foam_weight` is the CPU twin).
fn shore_foam_weight(breaking: f32, full: f32) -> f32 {
    let r = saturate(breaking / max(full, 1e-3));
    return r * r;
}

// ---- Breaker foam (SF-1): `sf_water::surf_foam` is the CPU twin (same terms, same order). ----

const SW_PI = 3.14159265;

// (F2 − F1 in cells, its gradient per cell) of a periodic cellular (Worley) noise over uv ∈ [0, 1)
// with `cells` cells a side: 0 on the cell borders (the foam lace's field).
fn sw_cells(uv: vec2<f32>, cells: u32, seed: u32) -> vec3<f32> {
    let g = uv * f32(cells);
    let i = vec2<i32>(floor(g));
    let f = g - floor(g);
    let n = i32(cells);
    var d1 = 1e9;
    var d2 = 1e9;
    var r1 = vec2(0.0);
    var r2 = vec2(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let c = vec2<u32>(((i + vec2(x, y)) % n + n) % n);
            let r = vec2(f32(x), f32(y)) + vec2(sw_hash(c.x, c.y, seed), sw_hash(c.x, c.y, seed + 1u)) - f;
            let dd = dot(r, r);
            if dd < d1 {
                d2 = d1;
                r2 = r1;
                d1 = dd;
                r1 = r;
            } else if dd < d2 {
                d2 = dd;
                r2 = r;
            }
        }
    }
    let a = sqrt(d1);
    let b = sqrt(d2);
    return vec3(b - a, r1 / max(a, 1e-5) - r2 / max(b, 1e-5));
}

fn surf_birth(p: WaterParams, breaking: f32) -> f32 {
    return smoothstep(p.surf.x, max(p.surf.y, p.surf.x + 1e-3), breaking);
}

fn bore_profile(p: WaterParams, psi: f32) -> f32 {
    let front = max(p.surf.w, 1e-3);
    return smoothstep(0.0, front, psi) * exp(-max(psi - front, 0.0) / max(p.surf.z, 1e-3));
}

fn behind_crest(p: WaterParams, theta: f32) -> f32 {
    let x = 0.5 * SW_PI + max(p.surf.w, 1e-3) - theta;
    return x - SW_TAU * floor(x / SW_TAU);
}

// (lace 0-1, ∂lace/∂e) of the cellular field `e` for a lace `width` and `feather` (cells).
fn foam_lace(e: f32, width: f32, feather: f32) -> vec2<f32> {
    let f = max(feather, 1e-3);
    let t = saturate((e - (width - f)) / (2.0 * f));
    return vec2(1.0 - t * t * (3.0 - 2.0 * t), -6.0 * t * (1.0 - t) / (2.0 * f));
}

fn swash_edge(p: WaterParams, swash: vec2<f32>) -> f32 {
    let x = 1.5 * SW_PI - swash.y;
    let age = x - SW_TAU * floor(x / SW_TAU);
    var shape = exp(-(age - SW_PI) / max(p.surf4.w, 1e-3));
    if age < SW_PI {
        shape = smoothstep(0.0, max(p.surf.w, 1e-3), age);
    }
    return saturate(swash.x) * shape;
}

// One fragment's inputs (`sf_water::SurfFoamInput`; `vertical` ≥ 1e5 where unknown).
struct SurfInput {
    breaking: f32,
    phase: f32,
    swash: vec2<f32>,
    vertical: f32,
    contact: f32,
    lace: f32,
    variation: f32,
    strength: f32,
    detail: f32,
}

// (`sf_water::SurfFoamTerms`.)
struct SurfTerms {
    bore: f32,
    waterline: f32,
    coverage: f32,
    fresh: f32,
    glow: f32,
    d_lace: f32,
}

fn surf_foam(p: WaterParams, i: SurfInput) -> SurfTerms {
    let detail = saturate(i.detail);
    let birth = surf_birth(p, i.breaking) * saturate(i.strength);
    let profile = bore_profile(p, behind_crest(p, i.phase));
    let v = 1.0 + p.surf3.x * (2.0 * saturate(i.variation) - 1.0);
    let lb = foam_lace(i.lace, mix(p.surf2.z, p.surf2.y, profile) * v, p.surf2.w);
    let bore = birth * profile * mix(profile, lb.x, detail);
    let edge = swash_edge(p, i.swash);
    var waterline = 0.0;
    var fresh_w = 0.0;
    var dl_w = 0.0;
    if i.contact > 1e-3 && i.vertical < 1e5 {
        let tau = max(birth, edge);
        let amount = mix(p.surf5.x, 1.0, tau);
        let x = max(i.vertical, 0.0) / (i.contact * (1.0 + p.surf5.z * edge));
        let line = 1.0 - smoothstep(0.0, max(p.surf5.y, 1e-3), x);
        let out = 1.0 - smoothstep(0.0, 1.0, x);
        let lw = foam_lace(i.lace, mix(p.surf2.z, p.surf2.y, out * tau), p.surf2.w);
        let lace = out * mix(out, lw.x, detail);
        waterline = amount * (1.0 - (1.0 - line) * (1.0 - lace));
        fresh_w = line + (1.0 - line) * tau * out;
        dl_w = amount * (1.0 - line) * out * detail * lw.y;
    }
    let coverage = 1.0 - (1.0 - bore) * (1.0 - waterline);
    let fresh = (bore * profile + waterline * fresh_w) / max(bore + waterline, 1e-6);
    return SurfTerms(bore, waterline, coverage, fresh, p.surf4.y * birth * profile, (1.0 - waterline) * birth * profile * detail * lb.y + (1.0 - bore) * dl_w);
}

// The surf lifecycle (SF-2, `sf_water::surf_life` is the CPU twin) at breaking ratio `b` (before
// the smooth min), its shore-distance rate `db_dd` (1/m), gap mask `open` (1 = breaks) and the orbit
// the skew warps `orbit` (m, Q₀·Σ A·coth): (phase skew s, bore weight with the gaps, |∂s/∂d| of the
// steepening 1/m). The steepening's skew is capped so its peak growth stretches the map by at most
// SW_SKEW_RATE_LIMIT.
fn surf_life(p: WaterParams, b: f32, db_dd: f32, open: f32, orbit: f32) -> vec3<f32> {
    let span = max(p.shape.w - p.shape.z, 1e-3);
    let x = saturate((b - p.shape.z) / span);
    let steep_w = x * x * (3.0 - 2.0 * x);
    let bore_ramp = smoothstep(p.shape.w, max(p.shape2.x, p.shape.w + 1e-3), b);
    let bore = bore_ramp * saturate(open);
    let skew_steepen = clamp(p.shape2.y, 0.0, 0.9);
    let skew_bore = clamp(p.shape2.z, 0.0, 0.9);
    let grow = orbit * skew_steepen * 1.5 / span * abs(db_dd);
    let lead = skew_steepen * min(SW_SKEW_RATE_LIMIT / max(grow, 1e-12), 1.0);
    let skew = lead * steep_w * (1.0 - bore) + skew_bore * bore;
    let rate = lead * 6.0 * x * (1.0 - x) / span * abs(db_dd);
    return vec3(skew, bore, rate);
}

// The envelope at the break line (SF-2's minimum visible crest; `sf_water::surf_envelope`): the
// smooth min `soft` raised to the sharper min of exponent `shape.x` up to half `shape3.x`.
fn surf_envelope(p: WaterParams, shoal: f32, breaker: f32, soft: f32) -> f32 {
    let lo = min(shoal, breaker);
    let hi = max(shoal, breaker);
    let e = clamp(p.shape.x, 1.0, 32.0);
    let sharp = lo / pow(1.0 + pow(max(lo / max(hi, 1e-20), 1e-12), e), 1.0 / e);
    let raised = min(sharp, 0.5 * max(p.shape3.x, 0.0));
    return select(soft, raised, raised > soft);
}

// The shore waves at Lagrangian render-space `xz` with shore sample `sh` (`shore_at`) over bed
// slope `bed` (`shore_slope`) at time `t` (s mod 256). `lod_k`: trains whose wave number is above it
// are faded out (the clipmap's or the pixel's resolution; a very large value keeps all). Zero
// outside the band and when off (`ShoreWaves::eval_on`).
fn shore_waves(p: WaterParams, sh: vec4<f32>, bed: f32, xz: vec2<f32>, t: f32, lod_k: f32) -> ShoreWave {
    let to = shore_trains(p, p.swash.xyz, sh, bed, xz, t, lod_k);
    if p.swash4.w >= 1.0 || all(p.swash4.xyz == p.swash.xyz) {
        return to;
    }
    // A sea-state transition (W-WIND): crossfade the trains it comes from (`ShoreWaves::eval`).
    let prior = shore_trains(p, p.swash4.xyz, sh, bed, xz, t, lod_k);
    let w = max(p.swash4.w, 0.0);
    let late = w >= 0.5;
    return ShoreWave(mix(prior.offset, to.offset, w), mix(prior.slope, to.slope, w), mix(prior.crest, to.crest, w), mix(prior.breaking, to.breaking, w), mix(prior.crest_cos, to.crest_cos, w), select(prior.phase, to.phase, late), vec2(mix(prior.swash.x, to.swash.x, w), select(prior.swash.y, to.swash.y, late)), mix(prior.lifecycle, to.lifecycle, w));
}

// One set of trains with angular frequencies `om` (rad/s) for `shore_waves`.
fn shore_trains(p: WaterParams, om: vec3<f32>, sh: vec4<f32>, bed: f32, xz: vec2<f32>, t: f32, lod_k: f32) -> ShoreWave {
    var out = ShoreWave(vec3(0.0), vec4(0.0), 0.0, 0.0, 0.0, 0.0, vec2(0.0), vec2(0.0));
    let d = sh.x;
    let band = p.swash2.w;
    let margin = p.swash3.x;
    if !(d < band && d > -margin) || (p.swash2.x <= 0.0 && p.swash2.y <= 0.0) {
        return out;
    }
    let o = sh.y;
    let dir = sh.zw;
    let beta = max(bed, 1e-3);
    let h = beta * max(d, 0.0);
    let h_edge = beta * band;
    let w = o * (1.0 - smoothstep(0.0, band, d));
    let noise = fract((p.swash3.zw + xz) / 1024.0);
    let fade = 0.5 - 0.5 * cos(SW_TAU * t / SW_REPEAT);
    let n1 = sw_noise(noise, 32u, 1u);
    let sets = n1 + (sw_noise(noise, 32u, 2u) - n1) * fade;
    let variation = p.swash3.y;
    let group = 1.0 - variation * (1.0 - SW_SET_FLOOR) * (1.0 - sets);
    let psi = variation * SW_PHASE_NOISE * (2.0 * sw_noise(noise, 16u, 3u) - 1.0);
    // γ: SW_BREAKER with `--off surfskew`, the tunable `breaker_gamma` otherwise (SF-2).
    let on = p.shape.x > 0.5;
    let gamma = select(SW_BREAKER, clamp(p.shape.y, 0.1, 2.0), on);
    let shoal = p.swash2.x * pow(max(h_edge / max(h, SW_MIN_DEPTH), 1.0), 0.25);
    let breaker = 0.5 * gamma * h;
    let env = shoal * breaker / pow(shoal * shoal * shoal * shoal + breaker * breaker * breaker * breaker + 1e-20, 0.25);
    // env / breaker, the breaker held at SW_MIN_DEPTH so the beach and the swash keep the ratio.
    let breaker_r = 0.5 * gamma * max(h, SW_MIN_DEPTH);
    out.breaking = w * group * shoal / pow(shoal * shoal * shoal * shoal + breaker_r * breaker_r * breaker_r * breaker_r + 1e-20, 0.25);
    var crest_env = env;
    if on {
        crest_env = surf_envelope(p, shoal, breaker, env);
    }
    // The minimum visible crest at the break line (SF-2).
    let amp = crest_env * w * group;
    var a = vec3(0.0);
    var k = vec3(0.0);
    var coth = vec3(0.0);
    var theta = vec3(0.0);
    var steep = 0.0;
    let shares = SW_SHARES;
    let spread = SW_PHASE_SPREAD;
    // The orbit's envelope term held at SW_ORBIT on any bed (W-GERSTNER-DEPTH).
    let coth_cap = SW_ORBIT / (0.5 * gamma * beta);
    var orbit = 0.0;
    for (var i = 0u; i < 3u; i++) {
        let lw = sw_local_wave(om[i], h, beta);
        let th = tanh(lw.x * max(h, SW_MIN_DEPTH));
        a[i] = min(shares[i] * amp, SW_MICHE * th / lw.x);
        k[i] = lw.x;
        coth[i] = min(1.0 / th, coth_cap);
        theta[i] = -(lw.y + om[i] * t + psi * spread[i]);
        steep += lw.x * a[i] * coth[i];
        orbit += a[i] * coth[i];
    }
    // The surf lifecycle (SF-2, `ShoreWaves::life`): the breaking ratio before the smooth min, its
    // rate with shore distance (the depth part held at SW_MIN_DEPTH's: continuous), the break gaps
    // at the point's shoreline foot, the skew capped by the unskewed orbit Q₀·Σ A·coth.
    var life = vec3(0.0);
    if on {
        let ratio = shoal / max(breaker_r, 1e-20);
        let b = w * group * ratio;
        let xb = saturate(d / band);
        let depth_rate = 1.25 * beta / max(h, SW_MIN_DEPTH);
        let db_dd = b * depth_rate + o * group * ratio * 6.0 * xb * (1.0 - xb) / band;
        let foot = fract(noise + d * dir / 1024.0);
        let cells = u32(p.shape3.y);
        let g1 = sw_noise(foot, cells, 4u);
        let gaps = g1 + (sw_noise(foot, cells, 5u) - g1) * fade;
        let feather = max(p.shape3.w, 1e-3);
        let open = 1.0 - variation * (1.0 - smoothstep(p.shape3.z - feather, p.shape3.z + feather, gaps));
        life = surf_life(p, b, db_dd, open, orbit * min(SW_STEEPNESS / max(steep, 1e-6), 1.0));
    }
    // The loop budget (SF-2): the skew steepens the horizontal map by up to 1 + s (its growth is held
    // by the cap above); the bore raises the cap to `shape2.w`. The old SW_STEEPNESS / steep when off.
    let s = life.x;
    let limit = SW_STEEPNESS + (p.shape2.w - SW_STEEPNESS) * life.y;
    let budget = steep * (1.0 + s * (1.0 - life.y));
    let q = min(limit / max(budget, 1e-6), 1.0);
    var y = 0.0;
    var along = 0.0;
    var dy = 0.0;
    var dalong = 0.0;
    for (var i = 0u; i < 3u; i++) {
        // Resolution fade (the CPU twin keeps every train: lod_k = ∞).
        let r = 1.0 - smoothstep(0.5 * lod_k, lod_k, k[i]);
        // The skewed phase θ' = θ − s·sin θ (SF-2; θ when s = 0), its derivative 1 − s·cos θ.
        let warped = theta[i] - s * sin(theta[i]);
        let m = 1.0 - s * cos(theta[i]);
        let sn = sin(warped);
        let cs = cos(warped);
        y += r * a[i] * sn;
        along += r * q * a[i] * coth[i] * cs;
        dy += r * a[i] * k[i] * cs * m;
        dalong -= r * q * a[i] * coth[i] * k[i] * sn * m;
    }
    let phi0 = sw_local_wave(om.x, 0.0, beta).y;
    let theta0 = -(phi0 + om.x * t + psi * SW_PHASE_SPREAD.x);
    // Stockdon's swash ∝ β: `swash2.y` is at the beach slope `swash2.z`.
    let swash_w = o * group * (1.0 - smoothstep(0.0, SW_ZONE * band, d)) * smoothstep(-margin, -0.5 * margin, d);
    let swash = p.swash2.y * (beta / max(p.swash2.z, 1e-3)) * swash_w;
    let g = theta0 - SW_SKEW * sin(theta0);
    y += swash * sin(g);
    out.offset = vec3(dir.x * along, y, dir.y * along);
    out.slope = vec4(dy * dir.x, dy * dir.y, dalong * dir.x * dir.x, dalong * dir.y * dir.y);
    // The main train's skewed phase: the visible crest the foam follows.
    let crest_phase = theta.x - s * sin(theta.x);
    out.crest = sin(crest_phase);
    out.crest_cos = cos(crest_phase);
    out.phase = crest_phase;
    out.lifecycle = life.xy;
    out.swash = vec2(select(0.0, swash_w, p.swash2.y > 0.0), g);
    return out;
}

// The shore waves' vertex displacement at Lagrangian `x0`: faded with the camera distance as the
// cascades are (`displacement_weight`: a train fades where the clipmap's ≈ d/48 spacing can no longer
// carry it, at 10-16 wavelengths; `displaced`'s far fade).
fn shore_wave_offset(p: WaterParams, sh: vec4<f32>, bed: f32, x0: vec2<f32>, camera: vec2<f32>, t: f32) -> vec3<f32> {
    let d = distance(x0, camera);
    let lod_k = SW_TAU * 16.0 / max(d, 1e-3);
    let fade = 1.0 - smoothstep(p.misc.z * 0.75, p.misc.z, d);
    return shore_waves(p, sh, bed, x0, t, lod_k).offset * fade;
}

// (swell, chop) amplitude scales at a shore sample (`sf_water::ShoreField::swell_scale` and
// `chop_scale` are the CPU twins), times the body's own scales.
fn shore_scales(p: WaterParams, sh: vec2<f32>) -> vec2<f32> {
    let swell = sh.y * smoothstep(0.0, p.shore2.y, sh.x);
    let chop = (p.shore2.w + (1.0 - p.shore2.w) * sh.y) * smoothstep(0.0, p.shore2.z, sh.x);
    return vec2(swell, chop) * p.waves.xy;
}

// The same for the shading normal (W-SHORE; `sf_water::ShoreField::normal_chop_scale` is the CPU
// twin): the swell fades as above, the chop's shore ramp holds at the normal floor (`misc2.z`) so
// wind wavelets stay where the vertices have settled. Floor 0 = `shore_scales`.
fn shore_normal_scales(p: WaterParams, sh: vec2<f32>) -> vec2<f32> {
    let swell = sh.y * smoothstep(0.0, p.shore2.y, sh.x);
    let chop = (p.shore2.w + (1.0 - p.shore2.w) * sh.y) * max(smoothstep(0.0, p.shore2.z, sh.x), p.misc2.z);
    return vec2(swell, chop) * p.waves.xy;
}

// The amplitude scale of cascade `c` (0-1: swell, 2-3: chop).
fn cascade_scale(c: u32, scales: vec2<f32>) -> f32 {
    return select(scales.y, scales.x, c < 2u);
}

fn ring_spacing(p: WaterParams, ring: u32) -> f32 {
    return p.clip.x * f32(1u << ring);
}

// A ring's centre: the camera snapped to twice the ring's spacing (so a ring's vertices never
// swim, and each finer ring's centre lies on the coarser ring's grid).
fn ring_centre(p: WaterParams, ring: u32, camera: vec2<f32>) -> vec2<f32> {
    let s2 = 2.0 * ring_spacing(p, ring);
    return floor(camera / s2 + 0.5) * s2;
}

// Lagrangian (undisplaced) render-space XZ of clipmap vertex `g` (grid units) of `ring_in`
// (`rings` / `rings + 1`: the skirt's inner / outer vertices).
fn clipmap_xz(p: WaterParams, g: vec2<f32>, ring_in: f32, camera: vec2<f32>) -> vec2<f32> {
    let rings = u32(p.clip.z);
    let half = p.clip.y * 0.5;
    let r = u32(ring_in + 0.5);
    if r >= rings {
        let last = rings - 1u;
        let c = ring_centre(p, last, camera);
        if r == rings {
            return c + g * ring_spacing(p, last);
        }
        return c + normalize(g) * p.misc.y;
    }
    let c = ring_centre(p, r, camera);
    // CDLOD morph: over the outer cells, odd vertices slide onto the next ring's grid, so the
    // edge matches the coarser ring exactly (no cracks, no T-junction pops).
    let edge = max(abs(g.x), abs(g.y));
    let m = saturate((edge - (half - 12.0)) / 10.0);
    let gm = g - fract(g * 0.5) * 2.0 * m;
    return c + gm * ring_spacing(p, r);
}

// True where a finer ring covers this point of `ring` (the fragment is drawn by that one).
fn covered_by_finer(p: WaterParams, ring: u32, xz: vec2<f32>, camera: vec2<f32>) -> bool {
    if ring == 0u || ring >= u32(p.clip.z) {
        return false;
    }
    let inner = ring - 1u;
    let q = abs(xz - ring_centre(p, inner, camera));
    // One fine cell short of its edge: there the finer ring is fully morphed onto this ring's
    // grid (identical surface), so the overlap hides rasterisation seams between the two.
    return max(q.x, q.y) < (p.clip.y * 0.5 - 1.0) * ring_spacing(p, inner);
}

fn cascade_phase(p: WaterParams, c: u32) -> vec2<f32> {
    switch c {
        case 0u: { return p.phase01.xy; }
        case 1u: { return p.phase01.zw; }
        case 2u: { return p.phase23.xy; }
        default: { return p.phase23.zw; }
    }
}

// Texture coordinates of render-space `xz` in cascade `c`: exact under the floating origin
// (`sf_water::origin_phase` holds the integer part of the origin).
fn cascade_uv(p: WaterParams, c: u32, xz: vec2<f32>) -> vec2<f32> {
    return fract((cascade_phase(p, c) + xz) / p.sizes[c]);
}

// Vertex displacement weight of cascade `c` at horizontal distance `d`: a cascade fades out
// where the clipmap's spacing (≈ d/48) can no longer carry its shortest waves. A function of
// distance, not of the ring, so two rings meeting at an edge displace it identically.
fn displacement_weight(p: WaterParams, c: u32, d: f32) -> f32 {
    let l = p.lambda_min[c];
    return 1.0 - smoothstep(10.0 * l, 16.0 * l, d);
}

// The displaced render-space position of Lagrangian point `x0`.
// `scales`: the shore's (swell, chop) scales at x0 (`shore_scales`).
fn displaced(p: WaterParams, tex: texture_2d_array<f32>, s: sampler, x0: vec2<f32>, camera: vec2<f32>, scales: vec2<f32>) -> vec4<f32> {
    let d = distance(x0, camera);
    var disp = vec4(0.0);
    let n = u32(p.misc.x);
    for (var c = 0u; c < n; c++) {
        let w = displacement_weight(p, c, d) * cascade_scale(c, scales);
        if w > 0.0 {
            disp += w * textureSampleLevel(tex, s, cascade_uv(p, c, x0), i32(c), 0.0);
        }
    }
    let fade = 1.0 - smoothstep(p.misc.z * 0.75, p.misc.z, d);
    // xyz: position; w: displaced height (crest scattering).
    return vec4(x0.x + disp.x * fade, p.clip.w + disp.y * fade, x0.y + disp.z * fade, disp.y * fade);
}
