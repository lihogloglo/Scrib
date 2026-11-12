//! `water.shore` (water.md "Shoreline and depth colour"): the signed distance to the shoreline at
//! the rest level, by an exact Euclidean distance transform (Felzenszwalb and Huttenlocher, "Distance
//! Transforms of Sampled Functions", 2012), and **openness**, a fetch proxy that scales the swell:
//! water far (through water, geodesically) from the open sea gets little swell, enclosed water gets
//! chop or ripples only. The GPU reads the same field ([`ShoreField::encode_rg8`]) so the CPU query
//! and the drawn surface attenuate alike.

use glam::DVec2;

/// Tuning of the shore product and of the wave attenuation it drives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShoreSettings {
    /// Distances clamp to ± this, metres.
    pub range: f32,
    /// Water of an open component at least this far from any shore is the "open region".
    pub open_core: f32,
    /// Geodesic distance through water (m) from the open region over which openness falls to 1/e.
    pub open_falloff: f32,
    /// An enclosed component's inscribed diameter (m) at which its openness reaches `enclosed_max`.
    pub enclosed_fetch: f32,
    /// Openness of the largest enclosed water (lakes get chop, never open-sea swell).
    pub enclosed_max: f32,
    /// Swell fades in over this distance from the shore (m).
    pub swell_ramp: f32,
    /// Chop fades in over this distance from the shore (m) ...
    pub chop_ramp: f32,
    /// ... and never drops below this share in enclosed water (wind ripples stay).
    pub chop_floor: f32,
    /// The chop's **shading normal** never drops below this share of its open-water strength near
    /// the shore (W-SHORE): the vertices settle, the wind wavelets keep glinting up to the waterline.
    /// 0 = the normals flatten with the displacement (the old look).
    pub normal_floor: f32,
    /// Seabed slope (rise per metre of shore distance) the analytic shore waves assume for their
    /// depth, `depth = beach_slope · distance` (W-GERSTNER; [`crate::ShoreWaves`]): the field has
    /// no bathymetry. 0.05 = 1:20, a typical sandy foreshore (Dean & Dalrymple 2002).
    pub beach_slope: f32,
    /// Strength of the analytic shore waves and swash (W-GERSTNER): 1 = the sea state's own
    /// amplitude, 0 = off (`--off shorewaves`, the FFT simply fades to flat at the shore).
    pub shore_waves: f32,
}

impl Default for ShoreSettings {
    fn default() -> Self {
        Self {
            range: 256.0,
            open_core: 200.0,
            open_falloff: 800.0,
            enclosed_fetch: 2000.0,
            enclosed_max: 0.35,
            swell_ramp: 40.0,
            chop_ramp: 3.0,
            chop_floor: 0.25,
            normal_floor: 0.5,
            beach_slope: 0.05,
            shore_waves: 1.0,
        }
    }
}

/// Water (true) or land at the rest level on a regular grid in the worldspace's XZ plane.
#[derive(Clone, Debug, PartialEq)]
pub struct WetMask {
    pub width: usize,
    pub height: usize,
    /// Texel size, metres.
    pub texel: f64,
    /// World XZ of texel (0, 0)'s centre; rows run +Z.
    pub origin: DVec2,
    /// `[row * width + column]`.
    pub wet: Vec<bool>,
}

impl WetMask {
    /// All water.
    pub fn new(width: usize, height: usize, texel: f64, origin: DVec2) -> Self {
        Self {
            width,
            height,
            texel,
            origin,
            wet: vec![true; width * height],
        }
    }
}

/// Bake statistics (the bake report).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShoreStats {
    pub wet_texels: usize,
    /// Connected water components (4-connected).
    pub components: usize,
    /// Components touching the grid's edge (the open sea).
    pub open_components: usize,
}

/// The baked shore product of one worldspace's level-0 water.
#[derive(Clone, Debug, PartialEq)]
pub struct ShoreField {
    pub width: usize,
    pub height: usize,
    pub texel: f64,
    pub origin: DVec2,
    /// Signed distance to the shoreline, metres: + in water, − on land, clamped to ± `range`.
    pub distance: Vec<f32>,
    /// 0 (sheltered) … 1 (open sea); land takes its wettest neighbour's value.
    pub openness: Vec<f32>,
    /// Shoreward direction (XZ, −∇distance), blurred over ~5 texels: unit where the shore is
    /// coherent, shorter where two shores' gradients meet (the medial axis of a channel), 0 beyond
    /// the distance range. The shore waves travel along it (W-GERSTNER).
    pub direction: Vec<[f32; 2]>,
    pub settings: ShoreSettings,
    pub stats: ShoreStats,
}

const BIG: f64 = 1e20;

/// Squared distance transform of one sampled function (FH 2012, the lower envelope of parabolas).
#[allow(clippy::cast_precision_loss, clippy::many_single_char_names)]
fn edt_1d(f: &[f64], d: &mut [f64], v: &mut [usize], z: &mut [f64]) {
    let n = f.len();
    let mut k = 0usize;
    v[0] = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let qf = q as f64;
        loop {
            let p = v[k];
            let pf = p as f64;
            let s = ((f[q] + qf * qf) - (f[p] + pf * pf)) / (2.0 * (qf - pf));
            if s <= z[k] && k > 0 {
                k -= 1;
                continue;
            }
            if s <= z[k] {
                // k == 0 and z[0] = −∞: unreachable for finite s; keep the first parabola.
                break;
            }
            k += 1;
            v[k] = q;
            z[k] = s;
            z[k + 1] = f64::INFINITY;
            break;
        }
    }
    k = 0;
    for (q, out) in d.iter_mut().enumerate().take(n) {
        let qf = q as f64;
        while z[k + 1] < qf {
            k += 1;
        }
        let p = v[k] as f64;
        *out = (qf - p) * (qf - p) + f[v[k]];
    }
}

/// Exact squared Euclidean distance (texels²) from every texel to the nearest texel where
/// `feature` is true (`BIG`-ish where there is none).
#[allow(clippy::many_single_char_names)]
pub fn edt_squared(feature: &[bool], width: usize, height: usize) -> Vec<f64> {
    let mut g: Vec<f64> = feature.iter().map(|f| if *f { 0.0 } else { BIG }).collect();
    let n = width.max(height);
    let (mut f, mut d, mut v, mut z) = (vec![0.0; n], vec![0.0; n], vec![0usize; n], vec![0.0; n + 1]);
    for x in 0..width {
        for y in 0..height {
            f[y] = g[y * width + x];
        }
        edt_1d(&f[..height], &mut d[..height], &mut v, &mut z);
        for y in 0..height {
            g[y * width + x] = d[y];
        }
    }
    for y in 0..height {
        let row = &mut g[y * width..(y + 1) * width];
        f[..width].copy_from_slice(row);
        edt_1d(&f[..width], &mut d[..width], &mut v, &mut z);
        row.copy_from_slice(&d[..width]);
    }
    g
}

/// IEEE half-float bits of `x`, round to nearest even (the shore texture's encoding).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::many_single_char_names)]
pub fn f16_bits(x: f32) -> u16 {
    let b = x.to_bits();
    let sign = ((b >> 16) & 0x8000) as u16;
    let exp = ((b >> 23) & 0xff) as i32;
    let man = b & 0x7f_ffff;
    if exp == 0xff {
        return sign | 0x7c00 | if man == 0 { 0 } else { 0x200 };
    }
    let e = exp - 127 + 15;
    if e >= 0x1f {
        return sign | 0x7c00;
    }
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let m = man | 0x80_0000;
        let shift = (14 - e) as u32;
        let half = 1u32 << (shift - 1);
        let rem = m & ((1u32 << shift) - 1);
        let mut r = m >> shift;
        if rem > half || (rem == half && r & 1 == 1) {
            r += 1;
        }
        return sign | r as u16;
    }
    let mut r = ((e as u32) << 10) | (man >> 13);
    let rem = man & 0x1fff;
    if rem > 0x1000 || (rem == 0x1000 && r & 1 == 1) {
        r += 1;
    }
    sign | r as u16
}

/// Shoreward unit direction (−∇distance, central differences), box-blurred twice over 5 texels so
/// crests bend smoothly round headlands; where opposite gradients meet the blurred vector shortens.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
fn shoreward(distance: &[f32], w: usize, h: usize, texel: f64) -> Vec<[f32; 2]> {
    let at = |x: usize, y: usize| distance[y * w + x];
    let mut dir: Vec<[f32; 2]> = (0..w * h)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            let (xl, xr) = (x.saturating_sub(1), (x + 1).min(w - 1));
            let (yl, yr) = (y.saturating_sub(1), (y + 1).min(h - 1));
            let gx = (at(xr, y) - at(xl, y)) / (((xr - xl).max(1)) as f32 * texel as f32);
            let gz = (at(x, yr) - at(x, yl)) / (((yr - yl).max(1)) as f32 * texel as f32);
            let g = glam::Vec2::new(-gx, -gz);
            // |∇d| ≈ 1 on a distance field; flat (clamped) or degenerate texels carry no direction.
            if g.length() > 0.25 { g.normalize().into() } else { [0.0, 0.0] }
        })
        .collect();
    let mut tmp = dir.clone();
    for _ in 0..2 {
        box_pass(&dir, &mut tmp, w, h, true);
        box_pass(&tmp, &mut dir, w, h, false);
    }
    dir
}

/// One 5-tap box blur along rows (`horizontal`) or columns.
#[allow(clippy::cast_precision_loss, clippy::many_single_char_names)]
fn box_pass(src: &[[f32; 2]], dst: &mut [[f32; 2]], w: usize, h: usize, horizontal: bool) {
    for y in 0..h {
        for x in 0..w {
            let (c, len) = if horizontal { (x, w) } else { (y, h) };
            let (lo, hi) = (c.saturating_sub(2), (c + 2).min(len - 1));
            let mut s = [0.0f32; 2];
            for t in lo..=hi {
                let v = if horizontal { src[y * w + t] } else { src[t * w + x] };
                s[0] += v[0];
                s[1] += v[1];
            }
            let n = (hi - lo + 1) as f32;
            dst[y * w + x] = [s[0] / n, s[1] / n];
        }
    }
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0).max(1e-6)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl ShoreField {
    /// Bakes distance and openness from a wet mask. Water connected to the grid's edge is the open
    /// sea (give the mask a margin of water around the land).
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        clippy::too_many_lines,
        clippy::many_single_char_names,
        clippy::cast_possible_wrap,
        clippy::needless_range_loop
    )]
    pub fn bake(mask: &WetMask, settings: ShoreSettings) -> Self {
        let (w, h) = (mask.width, mask.height);
        let texel = mask.texel;
        let land: Vec<bool> = mask.wet.iter().map(|x| !x).collect();
        let to_land = edt_squared(&land, w, h);
        let to_water = edt_squared(&mask.wet, w, h);
        let range = settings.range;
        let distance: Vec<f32> = (0..w * h)
            .map(|i| {
                let d = if mask.wet[i] {
                    (to_land[i].sqrt() - 0.5) * texel
                } else {
                    -(to_water[i].sqrt() - 0.5) * texel
                };
                (d as f32).clamp(-range, range)
            })
            .collect();
        drop((to_land, to_water));

        // 4-connected water components; open = touching the edge.
        let mut label = vec![u32::MAX; w * h];
        let mut open: Vec<bool> = Vec::new();
        let mut inscribed: Vec<f32> = Vec::new();
        let mut stack = Vec::new();
        for start in 0..w * h {
            if !mask.wet[start] || label[start] != u32::MAX {
                continue;
            }
            let id = u32::try_from(open.len()).expect("component count");
            let (mut edge, mut rmax) = (false, 0.0f32);
            label[start] = id;
            stack.push(start);
            while let Some(i) = stack.pop() {
                let (x, y) = (i % w, i / w);
                edge |= x == 0 || y == 0 || x + 1 == w || y + 1 == h;
                rmax = rmax.max(distance[i]);
                let mut visit = |j: usize| {
                    if mask.wet[j] && label[j] == u32::MAX {
                        label[j] = id;
                        stack.push(j);
                    }
                };
                if x > 0 {
                    visit(i - 1);
                }
                if x + 1 < w {
                    visit(i + 1);
                }
                if y > 0 {
                    visit(i - w);
                }
                if y + 1 < h {
                    visit(i + w);
                }
            }
            open.push(edge);
            inscribed.push(rmax);
        }

        // Geodesic distance through water from the open region: Dial's bucket queue over an
        // 8-neighbour 5-7 chamfer (within 2 % of Euclidean).
        let cap = (f64::from(settings.open_falloff) * 6.0 / texel * 5.0).min(f64::from(u32::MAX - 16)) as u32;
        let mut cost = vec![u32::MAX; w * h];
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); 8];
        for i in 0..w * h {
            if !mask.wet[i] || !open[label[i] as usize] {
                continue;
            }
            let (x, y) = (i % w, i / w);
            if distance[i] >= settings.open_core || x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                cost[i] = 0;
                buckets[0].push(i);
            }
        }
        let mut current = 0u32;
        let mut pending: usize = buckets[0].len();
        while pending > 0 {
            let slot = (current % 8) as usize;
            let items = std::mem::take(&mut buckets[slot]);
            pending -= items.len();
            for i in items {
                if cost[i] != current {
                    continue;
                }
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                for (dx, dy, step) in [(-1, 0, 5), (1, 0, 5), (0, -1, 5), (0, 1, 5), (-1, -1, 7), (1, -1, 7), (-1, 1, 7), (1, 1, 7)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                        continue;
                    }
                    let j = ny as usize * w + nx as usize;
                    let c = current + step;
                    if mask.wet[j] && c < cost[j] && c <= cap {
                        cost[j] = c;
                        buckets[(c % 8) as usize].push(j);
                        pending += 1;
                    }
                }
            }
            current += 1;
        }

        let mut openness: Vec<f32> = (0..w * h)
            .map(|i| {
                if !mask.wet[i] {
                    return 0.0;
                }
                let comp = label[i] as usize;
                if open[comp] {
                    if cost[i] == u32::MAX {
                        0.0
                    } else {
                        let g = f64::from(cost[i]) / 5.0 * texel;
                        (-g / f64::from(settings.open_falloff.max(1e-3))).exp() as f32
                    }
                } else {
                    settings.enclosed_max * (2.0 * inscribed[comp] / settings.enclosed_fetch.max(1e-3)).min(1.0)
                }
            })
            .collect();
        // Land takes its wettest 8-neighbour's openness, so bilinear filtering at the shoreline
        // doesn't halve it.
        let source = openness.clone();
        for i in 0..w * h {
            if mask.wet[i] {
                continue;
            }
            let (x, y) = (i % w, i / w);
            let mut best = 0.0f32;
            for ny in y.saturating_sub(1)..(y + 2).min(h) {
                for nx in x.saturating_sub(1)..(x + 2).min(w) {
                    let j = ny * w + nx;
                    if mask.wet[j] {
                        best = best.max(source[j]);
                    }
                }
            }
            openness[i] = best;
        }

        let stats = ShoreStats {
            wet_texels: mask.wet.iter().filter(|x| **x).count(),
            components: open.len(),
            open_components: open.iter().filter(|x| **x).count(),
        };
        let direction = shoreward(&distance, w, h, texel);
        Self {
            width: w,
            height: h,
            texel,
            origin: mask.origin,
            distance,
            openness,
            direction,
            settings,
            stats,
        }
    }

    /// Bilinear (distance m, openness) at world XZ; outside the field: open deep water.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn sample(&self, x: f64, z: f64) -> (f32, f32) {
        let u = (x - self.origin.x) / self.texel;
        let v = (z - self.origin.y) / self.texel;
        if u < 0.0 || v < 0.0 || u > (self.width - 1) as f64 || v > (self.height - 1) as f64 {
            return (self.settings.range, 1.0);
        }
        let (x0, y0) = (u.floor() as usize, v.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(self.width - 1), (y0 + 1).min(self.height - 1));
        let (fx, fy) = ((u - x0 as f64) as f32, (v - y0 as f64) as f32);
        let lerp2 = |a: &[f32]| {
            let top = a[y0 * self.width + x0] * (1.0 - fx) + a[y0 * self.width + x1] * fx;
            let bottom = a[y1 * self.width + x0] * (1.0 - fx) + a[y1 * self.width + x1] * fx;
            top * (1.0 - fy) + bottom * fy
        };
        (lerp2(&self.distance), lerp2(&self.openness))
    }

    /// Bilinear shoreward direction ([`Self::direction`]) at world XZ; 0 outside the field.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn sample_direction(&self, x: f64, z: f64) -> glam::Vec2 {
        let u = (x - self.origin.x) / self.texel;
        let v = (z - self.origin.y) / self.texel;
        if u < 0.0 || v < 0.0 || u > (self.width - 1) as f64 || v > (self.height - 1) as f64 {
            return glam::Vec2::ZERO;
        }
        let (x0, y0) = (u.floor() as usize, v.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(self.width - 1), (y0 + 1).min(self.height - 1));
        let (fx, fy) = ((u - x0 as f64) as f32, (v - y0 as f64) as f32);
        let at = |x: usize, y: usize| glam::Vec2::from(self.direction[y * self.width + x]);
        let top = at(x0, y0).lerp(at(x1, y0), fx);
        let bottom = at(x0, y1).lerp(at(x1, y1), fx);
        top.lerp(bottom, fy)
    }

    /// Amplitude scale of the swell (long cascades) at shore distance `d` and openness `o`
    /// (`water_common.wesl` `shore_scales` is the GPU twin).
    pub fn swell_scale(&self, d: f32, o: f32) -> f32 {
        o * smoothstep(0.0, self.settings.swell_ramp, d)
    }

    /// Amplitude scale of the chop (short cascades).
    pub fn chop_scale(&self, d: f32, o: f32) -> f32 {
        let s = &self.settings;
        (s.chop_floor + (1.0 - s.chop_floor) * o) * smoothstep(0.0, s.chop_ramp, d)
    }

    /// Scale of the chop cascades' slopes in the shading normal (W-SHORE; `water_common.wesl`
    /// `shore_normal_scales` is the GPU twin): the chop's shore ramp held at `normal_floor`, so the
    /// wavelets stay where the displacement has settled; never above the open-water chop here.
    pub fn normal_chop_scale(&self, d: f32, o: f32) -> f32 {
        let s = &self.settings;
        (s.chop_floor + (1.0 - s.chop_floor) * o) * smoothstep(0.0, s.chop_ramp, d).max(s.normal_floor)
    }

    /// The GPU texture (`Rgba16Float`, rows +Z, little-endian): (distance m, openness, shoreward
    /// x, shoreward z). Half floats keep the distance to ≤ 3 cm inside the 40 m wave band, which
    /// the shore waves' phase needs (W-GERSTNER; the old `Rg8Unorm` √-encoding was ~1 m there),
    /// and carry the baked direction so the shaders need no finite differences.
    pub fn encode_rgba16f(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.distance.len() * 8);
        for ((d, o), dir) in self.distance.iter().zip(&self.openness).zip(&self.direction) {
            for v in [*d, *o, dir[0], dir[1]] {
                out.extend_from_slice(&f16_bits(v).to_le_bytes());
            }
        }
        out
    }

    /// Texels that are land by more than `margin` metres (the surface is culled there).
    pub fn land_texels(&self, margin: f32) -> usize {
        self.distance.iter().filter(|d| **d < -margin).count()
    }
}

/// Optics records by area on a coarse grid (water.md "Optics by region"): `index` into a table
/// the host owns, blended bilinearly between cell centres on the GPU.
#[derive(Clone, Debug, PartialEq)]
pub struct OpticsMap {
    pub width: usize,
    pub height: usize,
    /// Cell size, metres.
    pub cell: f64,
    /// World XZ of cell (0, 0)'s centre; rows run +Z.
    pub origin: DVec2,
    /// `[row * width + column]`; cells outside the map use index 0.
    pub index: Vec<u8>,
}
