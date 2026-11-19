// The GPU twin of `tiles.rs` (worldgen.md "Pipeline" stage 7, slice G2): `field_height` is
// `GuideTiles::height` (the guide's Catmull-Rom surface plus `filter::detail_offset_banded`, band-limited for
// the height level), `field_albedo` the macro look: the ground layers' mean albedos (`layers::LayerField`, slice
// WG-X3) or, without them, the fallback look. The host (`sf_terrain_bevy` gpu fill, or the twin test) declares
// `field_data: array<u32>`, filled with `GuideTiles::gpu_data`; `tiles.rs` prepends the header slot constants
// (`FH_*`) and the guide's first word (`FH_HEIGHTS`); the stamps follow the heights (`FH_STAMPS` holds their
// first word, 0 = none), then the layer pyramids (`FH_LAYERS`, 0 = none).
//
// Precision: the CPU reference splits positions in f64; here the level-0 global sample index stays an
// integer and every split into (cell, fraction) is integer-exact where the cell is a whole number of eighth
// metres (guide spacing a multiple of 0.5 m, octave wavelengths a multiple of 0.125 m), else one f32
// rounding. The base sums use the nearest tap as reference (as `resample::bicubic_with`), so only f32
// rounding and the GPU's sin / cos / atan2 differ from the CPU (tests_tiles.rs "gpu twin").

fn wg_u(i: u32) -> u32 {
    return field_data[i];
}

fn wg_f(i: u32) -> f32 {
    return bitcast<f32>(field_data[i]);
}

fn wg_i(i: u32) -> i32 {
    return bitcast<i32>(field_data[i]);
}

fn wg_v3(i: u32) -> vec3<f32> {
    return vec3<f32>(wg_f(i), wg_f(i + 1u), wg_f(i + 2u));
}

/// Guide sample (col, row), edges extended.
fn wg_tap(c: i32, r: i32) -> f32 {
    let w = i32(wg_u(FH_W));
    let h = i32(wg_u(FH_H));
    let x = u32(clamp(c, 0, w - 1));
    let y = u32(clamp(r, 0, h - 1));
    return wg_f(FH_HEIGHTS + y * u32(w) + x);
}

struct WgCell {
    i: i32,
    t: f32,
}

/// floor(n / d) and its remainder in [0, d), d > 0.
fn wg_floor_div(n: i32, d: i32) -> vec2<i32> {
    var q = n / d;
    if (n % d != 0) && (n < 0) {
        q -= 1;
    }
    return vec2<i32>(q, n - q * d);
}

/// (n + off) / s as cell + fraction for an integer cell size `s` > 0 (`off` = `ci` + `cf`, cf in [0, 1));
/// else (`s` = 0) the f32 affine map `f32(n) * a + c`.
fn wg_split(n: i32, ci: i32, cf: f32, s: i32, a: f32, c: f32) -> WgCell {
    if s > 0 {
        let qr = wg_floor_div(n + ci, s);
        var q = qr.x;
        var t = (f32(qr.y) + cf) / f32(s);
        if t >= 1.0 {
            q += 1;
            t -= 1.0;
        }
        return WgCell(q, t);
    }
    let u = f32(n) * a + c;
    let fl = floor(u);
    return WgCell(i32(fl), u - fl);
}

fn wg_cr(t: f32) -> vec4<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    return vec4<f32>(-0.5 * t3 + t2 - 0.5 * t, 1.5 * t3 - 2.5 * t2 + 1.0, -1.5 * t3 + 2.0 * t2 + 0.5 * t, 0.5 * t3 - 0.5 * t2);
}

fn wg_cr_d(t: f32) -> vec4<f32> {
    let t2 = t * t;
    return vec4<f32>(-1.5 * t2 + 2.0 * t - 0.5, 4.5 * t2 - 5.0 * t, -4.5 * t2 + 4.0 * t + 0.5, 1.5 * t2 - t);
}

fn wg_bs(t: f32) -> vec4<f32> {
    let u = 1.0 - t;
    let t2 = t * t;
    let t3 = t2 * t;
    return vec4<f32>(u * u * u / 6.0, (3.0 * t3 - 6.0 * t2 + 4.0) / 6.0, (-3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0) / 6.0, t3 / 6.0);
}

fn wg_bs_d(t: f32) -> vec4<f32> {
    let u = 1.0 - t;
    let t2 = t * t;
    return vec4<f32>(-0.5 * u * u, 1.5 * t2 - 2.0 * t, -1.5 * t2 + t + 0.5, 0.5 * t2);
}

fn wg_bs_dd(t: f32) -> vec4<f32> {
    return vec4<f32>(1.0 - t, 3.0 * t - 2.0, 1.0 - 3.0 * t, t);
}

/// `filter::BaseSample`: height, gradient (east, north), plan curvature.
struct WgBase {
    h: f32,
    grad: vec2<f32>,
    curv: f32,
}

/// `filter::BaseField::sample` at the level-0 global sample `g`.
fn wg_base(g: vec2<i32>) -> WgBase {
    let cx = wg_split(g.x, wg_i(FH_COL_CI), wg_f(FH_COL_CF), wg_i(FH_CELL_S), wg_f(FH_COL_A), wg_f(FH_COL_C));
    let cy = wg_split(g.y, wg_i(FH_ROW_CI), wg_f(FH_ROW_CF), wg_i(FH_CELL_S), wg_f(FH_ROW_A), wg_f(FH_ROW_C));
    // The nearest tap is the reference: every weight set below sums to 1 or 0.
    let h0 = wg_tap(cx.i + select(0, 1, cx.t >= 0.5), cy.i + select(0, 1, cy.t >= 0.5));
    let wx = wg_cr(cx.t);
    let wy = wg_cr(cy.t);
    let dx = wg_cr_d(cx.t);
    let dy = wg_cr_d(cy.t);
    let bx = wg_bs(cx.t);
    let by = wg_bs(cy.t);
    let b1x = wg_bs_d(cx.t);
    let b1y = wg_bs_d(cy.t);
    let b2x = wg_bs_dd(cx.t);
    let b2y = wg_bs_dd(cy.t);
    var acc = 0.0;
    var d_col = 0.0;
    var d_row = 0.0;
    var sc = 0.0;
    var sr = 0.0;
    var scc = 0.0;
    var scr = 0.0;
    var srr = 0.0;
    for (var j = 0; j < 4; j++) {
        var row = 0.0;
        for (var i = 0; i < 4; i++) {
            let t = wg_tap(cx.i + i - 1, cy.i + j - 1) - h0;
            row += wx[i] * t;
            d_col += dx[i] * wy[j] * t;
            d_row += wx[i] * dy[j] * t;
            sc += b1x[i] * by[j] * t;
            sr += bx[i] * b1y[j] * t;
            scc += b2x[i] * by[j] * t;
            scr += b1x[i] * b1y[j] * t;
            srr += bx[i] * b2y[j] * t;
        }
        acc += wy[j] * row;
    }
    let s = wg_f(FH_SPACING);
    let grad = vec2<f32>(d_col / s, -d_row / s);
    let hx = sc / s;
    let hy = -sr / s;
    let hxx = scc / (s * s);
    let hxy = -scr / (s * s);
    let hyy = srr / (s * s);
    let g2 = hx * hx + hy * hy;
    var curv = 0.0;
    if g2 > 1e-18 {
        curv = clamp((hxx * hy * hy - 2.0 * hxy * hx * hy + hyy * hx * hx) / (g2 * sqrt(g2)), -1e6, 1e6);
    }
    return WgBase(h0 + acc, grad, curv);
}

fn wg_smoothstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = clamp((x - lo) / (hi - lo), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

/// `filter::pcg3d` (Jarzynski & Olano 2020).
fn wg_pcg3d(v: vec3<u32>) -> vec3<u32> {
    var x = v.x * 1664525u + 1013904223u;
    var y = v.y * 1664525u + 1013904223u;
    var z = v.z * 1664525u + 1013904223u;
    x += y * z;
    y += z * x;
    z += x * y;
    x ^= x >> 16u;
    y ^= y >> 16u;
    z ^= z >> 16u;
    x += y * z;
    y += z * x;
    z += x * y;
    return vec3<u32>(x, y, z);
}

fn wg_unit(v: u32) -> f32 {
    return f32(v >> 8u) * (1.0 / 16777216.0);
}

/// Cell and fraction of `n8 / (8 λ)`, `n8` in eighth metres: integer-exact when 8 λ is a whole number.
fn wg_cell(n8: i32, lambda: f32) -> WgCell {
    let lq = lambda * 8.0;
    if lq == floor(lq) && lq >= 1.0 && lq < 16777216.0 {
        let l = i32(lq);
        let qr = wg_floor_div(n8, l);
        return WgCell(qr.x, f32(qr.y) / f32(l));
    }
    let u = f32(n8) / lq;
    let fl = floor(u);
    return WgCell(i32(fl), u - fl);
}

/// `filter::phasor` at the level-0 global sample `g`: (phase, coherence).
fn wg_phasor(g: vec2<i32>, lambda: f32, dir: vec2<f32>, seed: u32) -> vec2<f32> {
    // Guide-frame point (x east, y north) in eighth metres: x = g.x / 2 − 128, y = −z = 128 − g.y / 2.
    let cx = wg_cell(4 * (g.x - 256), lambda);
    let cy = wg_cell(4 * (256 - g.y), lambda);
    let inv_r2 = 1.0 / (FILTER_KERNEL_RADIUS * FILTER_KERNEL_RADIUS);
    var re = 0.0;
    var im = 0.0;
    var wsum = 0.0;
    for (var dy = -2; dy <= 2; dy++) {
        for (var dx = -2; dx <= 2; dx++) {
            let hsh = wg_pcg3d(vec3<u32>(bitcast<u32>(cx.i + dx), bitcast<u32>(cy.i + dy), seed));
            let ox = cx.t - (f32(dx) + wg_unit(hsh.x));
            let oy = cy.t - (f32(dy) + wg_unit(hsh.y));
            let r2 = (ox * ox + oy * oy) * inv_r2;
            if r2 >= 1.0 {
                continue;
            }
            let b = 1.0 - r2;
            let a = b * b * b;
            let th = 6.2831855 * (ox * dir.x + oy * dir.y + wg_unit(hsh.z));
            re += a * cos(th);
            im += a * sin(th);
            wsum += a;
        }
    }
    let mag = sqrt(re * re + im * im);
    var coherence = 0.0;
    if wsum > 0.0 {
        coherence = min(mag / wsum, 1.0);
    }
    return vec2<f32>(atan2(im, re), coherence);
}

/// `filter::profile`: value and derivative.
fn wg_profile(th: f32, sharp: f32, carve: f32) -> vec2<f32> {
    let s2 = sin(0.5 * th);
    let c2 = cos(0.5 * th);
    let smooth_v = cos(th);
    let d_smooth = -sin(th);
    let edge = 1.0 - 2.0 * abs(s2);
    let d_edge = -select(-1.0, 1.0, s2 >= 0.0) * c2;
    let k = 1.0 / (1.0 + carve);
    return vec2<f32>((smooth_v + (edge - smooth_v) * sharp - carve) * k, (d_smooth + (d_edge - d_smooth) * sharp) * k);
}

/// The band limit `[lo, hi]` (m of wavelength) of height level `level` (`GuideTiles::band_m`).
fn wg_band(level: u32) -> vec2<f32> {
    let s = 0.5 * exp2(f32(level));
    return vec2<f32>(wg_f(FH_BAND_LO) * s, wg_f(FH_BAND_HI) * s);
}

/// `filter::detail_offset_banded` at the level-0 global sample `g` over `b`.
fn wg_detail(g: vec2<i32>, b: WgBase, band: vec2<f32>) -> f32 {
    let octaves = wg_u(FH_OCTAVES);
    let wavelength = wg_f(FH_WAVELENGTH);
    let lacunarity = wg_f(FH_LACUNARITY);
    let gain = wg_f(FH_GAIN);
    let slope0 = sqrt(b.grad.x * b.grad.x + b.grad.y * b.grad.y);
    let shore = max(b.h - wg_f(FH_SEA), 0.0) * wg_f(FH_SHORE_RATIO);
    let amp = min(min(wg_f(FH_RELIEF) * slope0 * wavelength, wg_f(FH_MAX_AMP)), shore);
    if octaves == 0u || !(amp > 0.0) {
        return 0.0;
    }
    var wsum = 0.0;
    var wk = 1.0;
    for (var k = 0u; k < octaves; k++) {
        wsum += wk;
        wk *= gain;
    }
    let curv = abs(b.curv);
    let cascade = wg_u(FH_CASCADE) != 0u;
    let seed0 = wg_u(FH_SEED);
    var g2 = b.grad;
    var dh = 0.0;
    var lambda = wavelength;
    var w = 1.0 / wsum;
    for (var k = 0u; k < octaves; k++) {
        let s = sqrt(g2.x * g2.x + g2.y * g2.y);
        let fade = wg_smoothstep(wg_f(FH_SLOPE_LO), wg_f(FH_SLOPE_HI), s)
            * (1.0 - wg_smoothstep(wg_f(FH_CURV_LO), wg_f(FH_CURV_HI), curv * lambda))
            * wg_smoothstep(band.x, band.y, lambda);
        if fade > 0.0 && s > 0.0 {
            let dir = vec2<f32>(-g2.y / s, g2.x / s);
            let seed = seed0 ^ (k * 0x9e3779b9u);
            let pc = wg_phasor(g, lambda, dir, seed);
            let a = amp * w * fade * wg_smoothstep(0.0, wg_f(FH_PHASE_FADE), pc.y);
            let p = wg_profile(pc.x, wg_f(FH_SHARP), wg_f(FH_CARVE));
            dh += a * p.x;
            if cascade {
                let slope = a * p.y * 6.2831855 / lambda;
                g2 = vec2<f32>(g2.x + slope * dir.x, g2.y + slope * dir.y);
            }
        }
        lambda /= lacunarity;
        w *= gain;
    }
    return dh;
}

/// Bilinear in one stamp grid at word `p` (`stamps::bilinear`): cell (i, j) + t, `a + (b − a)·t`.
fn wg_stamp_lerp(p: u32, w: u32, h: u32, i: u32, j: u32, t: vec2<f32>) -> f32 {
    let i1 = min(i + 1u, w - 1u);
    let j1 = min(j + 1u, h - 1u);
    let a = wg_f(p + j * w + i);
    let b = wg_f(p + j * w + i1);
    let c = wg_f(p + j1 * w + i);
    let d = wg_f(p + j1 * w + i1);
    let top = a + (b - a) * t.x;
    let bot = c + (d - c) * t.x;
    return top + (bot - top) * t.y;
}

/// `stamps::StampIndex::sample` at the level-0 global sample `g`: (height, weight) of the covering stamp of
/// largest weight, (0, 0) outside every stamp. Layout: `StampIndex::gpu_words`.
fn wg_stamp(g: vec2<i32>) -> vec2<f32> {
    let s0 = wg_u(FH_STAMPS);
    if s0 == 0u {
        return vec2<f32>(0.0, 0.0);
    }
    let nx = i32(wg_u(s0 + 1u));
    let ny = i32(wg_u(s0 + 2u));
    let edge = i32(wg_u(s0 + 5u));
    let bx = wg_floor_div(g.x - wg_i(s0 + 3u), edge).x;
    let by = wg_floor_div(g.y - wg_i(s0 + 4u), edge).x;
    if bx < 0 || by < 0 || bx >= nx || by >= ny {
        return vec2<f32>(0.0, 0.0);
    }
    let table = wg_u(s0 + 6u);
    let starts = wg_u(s0 + 7u);
    let items = wg_u(s0 + 8u);
    let bin = u32(by * nx + bx);
    var best = vec2<f32>(0.0, 0.0);
    var found = false;
    for (var k = wg_u(starts + bin); k < wg_u(starts + bin + 1u); k++) {
        let r = table + 8u * wg_u(items + k);
        let a = wg_f(r + 4u);
        let u = f32(g.x - wg_i(r)) * a + wg_f(r + 2u);
        let v = f32(g.y - wg_i(r + 1u)) * a + wg_f(r + 3u);
        let w = wg_u(r + 5u);
        let h = wg_u(r + 6u);
        if !(u >= 0.0 && v >= 0.0 && u <= f32(w - 1u) && v <= f32(h - 1u)) {
            continue;
        }
        let i = min(u32(floor(u)), w - 1u);
        let j = min(u32(floor(v)), h - 1u);
        let t = vec2<f32>(u - f32(i), v - f32(j));
        let p = wg_u(r + 7u);
        let s = vec2<f32>(wg_stamp_lerp(p, w, h, i, j, t), wg_stamp_lerp(p + w * h, w, h, i, j, t));
        if !found || s.y > best.y {
            best = s;
            found = true;
        }
    }
    return best;
}

/// The height at level-0 global sample `g` as height level `level` sees it (`GuideTiles::height`): the stamp
/// blend `mix(base + detail · (1 − w), stamp, w)`.
fn field_height(g: vec2<i32>, level: u32) -> f32 {
    let st = wg_stamp(g);
    if st.y >= 1.0 {
        return st.x;
    }
    let b = wg_base(g);
    let band = wg_band(level);
    var h = b.h;
    if !(wg_u(FH_OCTAVES) == 0u || wg_f(FH_WAVELENGTH) <= band.x) {
        h = b.h + wg_detail(g, b, band) * (1.0 - st.y);
    }
    if st.y > 0.0 {
        h = h + (st.x - h) * st.y;
    }
    return h;
}

fn wg_mix3(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

/// `layers::pick_stamp` at the level-0 global sample `g`: the index of the covering stamp of largest weight over
/// one half (the first on a tie), -1 where there is none.
fn wg_stamp_pick(g: vec2<i32>) -> i32 {
    let s0 = wg_u(FH_STAMPS);
    if s0 == 0u {
        return -1;
    }
    let nx = i32(wg_u(s0 + 1u));
    let ny = i32(wg_u(s0 + 2u));
    let edge = i32(wg_u(s0 + 5u));
    let bx = wg_floor_div(g.x - wg_i(s0 + 3u), edge).x;
    let by = wg_floor_div(g.y - wg_i(s0 + 4u), edge).x;
    if bx < 0 || by < 0 || bx >= nx || by >= ny {
        return -1;
    }
    let table = wg_u(s0 + 6u);
    let starts = wg_u(s0 + 7u);
    let items = wg_u(s0 + 8u);
    let bin = u32(by * nx + bx);
    var best = -1;
    var best_w = 0.5;
    for (var k = wg_u(starts + bin); k < wg_u(starts + bin + 1u); k++) {
        let index = wg_u(items + k);
        let r = table + 8u * index;
        let a = wg_f(r + 4u);
        let u = f32(g.x - wg_i(r)) * a + wg_f(r + 2u);
        let v = f32(g.y - wg_i(r + 1u)) * a + wg_f(r + 3u);
        let w = wg_u(r + 5u);
        let h = wg_u(r + 6u);
        if !(u >= 0.0 && v >= 0.0 && u <= f32(w - 1u) && v <= f32(h - 1u)) {
            continue;
        }
        let i = min(u32(floor(u)), w - 1u);
        let j = min(u32(floor(v)), h - 1u);
        let t = vec2<f32>(u - f32(i), v - f32(j));
        let weight = wg_stamp_lerp(wg_u(r + 7u) + w * h, w, h, i, j, t);
        if weight > best_w {
            best = i32(index);
            best_w = weight;
        }
    }
    return best;
}

/// One 10-bit RGB word (`layers::unpack10`).
fn wg_unpack10(c: u32) -> vec3<f32> {
    return vec3<f32>(f32(c & 1023u), f32((c >> 10u) & 1023u), f32((c >> 20u) & 1023u)) / 1023.0;
}

/// `layers::Pyramid::albedo` of the record at word `r` (`Pyramid::gpu_words`): the coarsest level whose texels
/// are not wider than a macro sample of `level` (2^level m), bilinear between texel centres.
fn wg_layer_albedo(r: u32, g: vec2<i32>, level: u32) -> vec3<f32> {
    let mips = wg_u(r);
    let s = wg_f(r + 1u);
    let sp = f32(1u << min(level, 30u));
    var m = 0u;
    loop {
        if m + 1u >= mips || s * f32(1u << (m + 1u)) > sp {
            break;
        }
        m += 1u;
    }
    let q = r + 2u + 6u * m;
    let p = wg_u(q);
    let w = wg_u(q + 1u);
    let h = wg_u(q + 2u);
    let a = wg_f(q + 3u);
    let u = clamp(f32(g.x) * a + wg_f(q + 4u), 0.0, f32(w - 1u));
    let v = clamp(f32(g.y) * a + wg_f(q + 5u), 0.0, f32(h - 1u));
    let i = min(u32(floor(u)), w - 1u);
    let j = min(u32(floor(v)), h - 1u);
    let i1 = min(i + 1u, w - 1u);
    let j1 = min(j + 1u, h - 1u);
    let tx = u - f32(i);
    let ty = v - f32(j);
    let c00 = wg_unpack10(wg_u(p + j * w + i));
    let c10 = wg_unpack10(wg_u(p + j * w + i1));
    let c01 = wg_unpack10(wg_u(p + j1 * w + i));
    let c11 = wg_unpack10(wg_u(p + j1 * w + i1));
    let top = c00 + (c10 - c00) * tx;
    let bot = c01 + (c11 - c01) * tx;
    return top + (bot - top) * ty;
}

/// The macro albedo (linear) at the level-0 global sample `g` for macro level `level`: the ground layers'
/// (`layers::LayerField::albedo`: a stamp's own grid where its weight is over one half, else the global grid),
/// or without them `tiles::fallback_look` (scaffold FALLBACK_LOOK) by height above sea and slope.
fn field_albedo(g: vec2<i32>, level: u32) -> vec3<f32> {
    let l0 = wg_u(FH_LAYERS);
    if l0 != 0u {
        var rec = wg_u(l0 + 2u);
        let k = wg_stamp_pick(g);
        if k >= 0 && u32(k) < wg_u(l0) {
            let sr = wg_u(wg_u(l0 + 1u) + u32(k));
            if sr != 0u {
                rec = sr;
            }
        }
        return wg_layer_albedo(rec, g, level);
    }
    let b = wg_base(g);
    let h = b.h - wg_f(FH_SEA);
    let slope = sqrt(b.grad.x * b.grad.x + b.grad.y * b.grad.y);
    let p = FH_LOOK;
    if h < 0.0 {
        return wg_mix3(wg_v3(p), wg_v3(p + 3u), wg_smoothstep(wg_f(p + 18u), wg_f(p + 19u), h));
    }
    var land = wg_mix3(wg_v3(p + 6u), wg_v3(p + 9u), wg_smoothstep(wg_f(p + 22u), wg_f(p + 23u), h));
    land = wg_mix3(land, wg_v3(p + 12u), wg_smoothstep(wg_f(p + 24u), wg_f(p + 25u), h));
    let c = wg_mix3(wg_v3(p + 3u), land, wg_smoothstep(wg_f(p + 20u), wg_f(p + 21u), h));
    return wg_mix3(c, wg_v3(p + 15u), wg_smoothstep(wg_f(p + 26u), wg_f(p + 27u), slope));
}
