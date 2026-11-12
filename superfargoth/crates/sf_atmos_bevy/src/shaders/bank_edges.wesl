// The depth-aware ground-fog bank (FOG-BANK, weather.md "Fog"): (distance, bank) pairs packed
// into one f32 of the cloud `dist` target, the trace's near/far split and the composite's
// log-depth blend and bilateral depth weight. The CPU twin is `sf_atmos::bank_edges` (change
// both together; the constants match).

const LOG_STEPS: f32 = 32.0;
const OCTAVES: f32 = 24.0;
const DEPTH_SIGMA: f32 = 0.25;
const DEPTH_FLOOR: f32 = 1e-4;

// `2k + bank`: k the distance's log2 in 1/LOG_STEPS octaves (from 1 m), bank in [0, 1].
fn bank_encode(t: f32, bank: f32) -> f32 {
    let k = round(clamp(log2(max(t, 1.0)), 0.0, OCTAVES) * LOG_STEPS);
    return 2.0 * k + saturate(bank);
}

// (distance m, bank).
fn bank_decode(v: f32) -> vec2<f32> {
    let k = min(floor(v * 0.5), OCTAVES * LOG_STEPS);
    return vec2(exp2(k / LOG_STEPS), saturate(v - 2.0 * k));
}

// The share of the march interval [a, b] before the near depth `d`.
fn bank_near_share(a: f32, b: f32, d: f32) -> f32 {
    if b <= a {
        return select(0.0, 1.0, d >= b);
    }
    return saturate((d - a) / (b - a));
}

// Whether two pairs' distances are the same surface (the resolve's cut-depth tolerance).
fn bank_same_depth(a: f32, b: f32) -> bool {
    return abs(a - b) <= 0.1 * b + 5.0;
}

// A tap's (near, far) pairs at a pixel of depth `t`: x the log-depth blend of its banks,
// y its bilateral depth weight (1 inside [near, far], Gaussian in octaves outside, floored).
fn bank_tap(t: f32, near: vec2<f32>, far: vec2<f32>) -> vec2<f32> {
    let ln = log2(max(near.x, 1.0));
    let lf = log2(max(far.x, 1.0));
    let lt = log2(max(t, 1.0));
    var s = 1.0;
    if lf - ln > 1e-4 {
        s = saturate((lt - ln) / (lf - ln));
    }
    let e = max(max(ln - lt, lt - lf), 0.0) / DEPTH_SIGMA;
    return vec2(mix(near.y, far.y, s), exp(-0.5 * e * e) + DEPTH_FLOOR);
}
