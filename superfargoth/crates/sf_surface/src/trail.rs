//! Trails (surface-weather.md "Trails (W6) and why DLSS needs motion vectors"; slice W6a): the
//! CPU twin of the trail ring's compute pass (`sf_surface_bevy` `trail.wgsl`) and the packing of
//! its parameters, so the GPU math is tested exactly here.
//!
//! - **Ring:** [`RING_SIZE`]² texels of [`RING_TEXEL_M`] (64 m), toroidal and camera-centred like
//!   the height clipmap: the window's first texel is a whole world texel index ([`window_min`]),
//!   a texel's world index follows from its address ([`world_of`]), and a texel that held another
//!   world texel last frame (it entered the ring) starts from 0 ([`kept`]).
//! - **Texel:** R the depression (m, ≤ 0, down to `-depth`: the stamp carves the cover to the
//!   ground), G the lip (m, ≥ 0). The GPU texture is `Rgba16Float` (a core storage format; B, A
//!   spare for W6b).
//! - **Stamps** ([`Stamp`]): oriented ellipses (feet, later lying bodies), combined with Min on R
//!   and Max on G; the profile is a flat-bottomed print with a soft wall and a quadratic lip
//!   around it (Rise of the Tomb Raider's deferred snow deformation: the trail plus an elevated
//!   rim, Michels and Sikachev, GPU Pro 7). A lip never stands above the depression it sits in
//!   ([`lip_cap`]), so a print over an older print's rim is flat at its bottom.
//! - **Decay** ([`Refill`], [`texel_update`]): while snow or ash falls the print refills toward 0
//!   at [`REFILL`] × the fall rate (cover depth per game hour) and the lip flattens with it; with
//!   nothing falling the prints stay. Refill is released in whole [`REFILL_QUANTUM_M`] steps so a
//!   half-float texel (11-bit mantissa) never rounds a small step away, and the pass runs only on
//!   those frames.
//!
//! Presentation only (design rule "net-ready"): stamps never feed gameplay.

use glam::{DVec2, IVec2, Vec2};

use crate::smoothstep;

/// Ring edge, texels.
pub const RING_SIZE: i32 = 1024;
/// Ring texel, meters (1024 × 6.25 cm = 64 m: the two finest CDLOD levels' reach, W6b).
pub const RING_TEXEL_M: f64 = 0.0625;
/// Actors farther than this from the camera (horizontally) don't stamp.
pub const STAMP_RANGE_M: f32 = 40.0;
/// Stamps per frame the pass takes (its uniform array); the nearest win.
pub const MAX_STAMPS: usize = 64;
/// Lip height, a share of the depth.
pub const LIP_HEIGHT: f32 = 0.25;
/// Lip width, in units of the stamp's normalised radius (the ellipse is radius 1).
pub const LIP_WIDTH: f32 = 0.6;
/// The print's soft wall, a share of its normalised radius inside the rim.
pub const WALL: f32 = 0.3;
/// Prints refill this many times faster than bare ground covers (cover rate per game hour ×
/// depth): Morrowind's snow (0.5 / h) refills a full print in 20 game minutes.
pub const REFILL: f32 = 6.0;
/// Refill is released in steps of this many meters (above a half float's step at 0.45 m).
pub const REFILL_QUANTUM_M: f32 = 0.002;
/// A foot's print: half length (along the facing) and half width, meters, at body scale 1.
/// Wider than the sole: a boot pushes snow aside, and the ring's 6.25 cm texels need ≥ 1.5 texels.
pub const FOOT_HALF_M: Vec2 = Vec2::new(0.16, 0.09);

/// Uniform words: four slots of `vec4` (window, mins, counts, shape), then two per stamp.
pub const PARAM_SLOTS: usize = 4 + 2 * MAX_STAMPS;

/// One stamp in render space (meters, X and Z).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stamp {
    pub center: Vec2,
    /// Unit facing (the ellipse's long axis).
    pub axis: Vec2,
    /// Half length along `axis`, half width across it (m).
    pub half: Vec2,
    /// Depth as a share of the ring's depth (1: down to the ground).
    pub depth: f32,
}

/// The first texel (whole world texel index, X and Z) of the ring centred on `camera_xz`
/// (absolute world meters).
#[allow(clippy::cast_possible_truncation)]
pub fn window_min(camera_xz: DVec2) -> IVec2 {
    let t = (camera_xz / RING_TEXEL_M).floor();
    IVec2::new(t.x as i32, t.y as i32) - IVec2::splat(RING_SIZE / 2)
}

/// The world texel index the texel at `addr` (0..n) holds in the window starting at `min`.
pub fn world_of(addr: IVec2, min: IVec2, n: i32) -> IVec2 {
    min + (addr - min).rem_euclid(IVec2::splat(n))
}

/// Whether the texel at `addr` keeps its value when the window moves from `old` (`None`: the
/// ring holds nothing valid) to `new`: it held the same world texel.
pub fn kept(addr: IVec2, old: Option<IVec2>, new: IVec2, n: i32) -> bool {
    old.is_some_and(|old| world_of(addr, old, n) == world_of(addr, new, n))
}

/// The print's profile at normalised elliptic radius `r`: (depression share 0..1, lip share 0..1).
pub fn profile(r: f32) -> (f32, f32) {
    let dep = 1.0 - smoothstep(1.0 - WALL, 1.0, r);
    let x = (r - 1.0) / LIP_WIDTH;
    let lip = if x > 0.0 && x < 1.0 { 4.0 * x * (1.0 - x) } else { 0.0 };
    (dep, lip)
}

impl Stamp {
    /// Its reach from the centre (m): the ellipse plus the lip.
    pub fn reach(&self) -> f32 {
        self.half.max_element() * (1.0 + LIP_WIDTH)
    }

    /// (R, G) of this stamp at render XZ `q` with the ring's depth `depth_m`.
    pub fn at(&self, q: Vec2, depth_m: f32) -> [f32; 2] {
        let d = q - self.center;
        if d.length_squared() > self.reach() * self.reach() {
            return [0.0, 0.0];
        }
        let along = d.dot(self.axis) / self.half.x;
        let across = d.dot(self.axis.perp()) / self.half.y;
        let (dep, lip) = profile(Vec2::new(along, across).length());
        let h = depth_m * self.depth;
        [-h * dep, LIP_HEIGHT * h * lip]
    }
}

/// Min on R, Max on G (the stamps' blend).
pub fn combine(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [a[0].min(b[0]), a[1].max(b[1])]
}

/// A lip never stands above the depression under it: G ≤ the lip height × the cover left.
pub fn lip_cap(v: [f32; 2], depth_m: f32) -> [f32; 2] {
    let left = if depth_m > 0.0 { (1.0 + v[0] / depth_m).clamp(0.0, 1.0) } else { 0.0 };
    [v[0], v[1].min(LIP_HEIGHT * depth_m * left)]
}

/// The refill step: R rises toward 0 by `refill_m`, G sinks by its share.
pub fn decay(v: [f32; 2], refill_m: f32) -> [f32; 2] {
    [(v[0] + refill_m).min(0.0), (v[1] - refill_m * LIP_HEIGHT).max(0.0)]
}

/// One texel of the pass: its last value (0 if it entered the ring), refilled, stamped at its
/// centre `q` (render XZ), lip capped. `trail.wgsl` `main`.
pub fn texel_update(prev: [f32; 2], q: Vec2, stamps: &[Stamp], refill_m: f32, depth_m: f32) -> [f32; 2] {
    let mut v = decay(prev, refill_m);
    for s in stamps {
        v = combine(v, s.at(q, depth_m));
    }
    lip_cap(v, depth_m)
}

/// Print refill in cover meters per game hour from the fall rates (cover per game hour) and the
/// channels' depths (m; 0 = no look on that channel).
pub fn refill_rate_m_per_h(snow_rate: f32, ash_rate: f32, snow_depth_m: f32, ash_depth_m: f32) -> f32 {
    REFILL * (snow_rate.max(0.0) * snow_depth_m + ash_rate.max(0.0) * ash_depth_m)
}

/// Accumulates refill and releases it in whole quanta.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Refill {
    pub pending_m: f32,
}

impl Refill {
    /// Adds `rate_m_per_h` over `dh` game hours (negative: the clock went back, ignored); returns
    /// the meters to apply this frame (0 below a quantum).
    #[allow(clippy::cast_possible_truncation)]
    pub fn add(&mut self, rate_m_per_h: f32, dh: f64) -> f32 {
        if dh > 0.0 && rate_m_per_h > 0.0 {
            self.pending_m += (f64::from(rate_m_per_h) * dh) as f32;
        }
        if self.pending_m < REFILL_QUANTUM_M {
            return 0.0;
        }
        std::mem::take(&mut self.pending_m)
    }
}

/// One frame of the pass, as the uniform `TrailParams` in `trail.wgsl` reads it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrailPass {
    /// Render X, Z of the new window's first texel's min corner.
    pub corner: Vec2,
    /// Ring depth (m).
    pub depth_m: f32,
    pub new_min: IVec2,
    /// The window the ring holds (`None`: clear it all).
    pub old_min: Option<IVec2>,
    pub refill_m: f32,
    pub stamps: Vec<Stamp>,
}

impl TrailPass {
    /// The uniform as `vec4<u32>` slots (f32 / i32 bit-cast).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn slots(&self) -> Vec<[u32; 4]> {
        let f = |v: [f32; 4]| v.map(f32::to_bits);
        let old = self.old_min.unwrap_or(IVec2::ZERO);
        let n = self.stamps.len().min(MAX_STAMPS);
        let mut s = vec![[0; 4]; PARAM_SLOTS];
        s[0] = f([self.corner.x, self.corner.y, RING_TEXEL_M as f32, self.depth_m]);
        s[1] = [self.new_min.x, self.new_min.y, old.x, old.y].map(|v| v as u32);
        s[2] = [n as u32, u32::from(self.old_min.is_some()), RING_SIZE as u32, 0];
        s[3] = f([self.refill_m, LIP_HEIGHT, LIP_WIDTH, WALL]);
        for (i, st) in self.stamps.iter().take(MAX_STAMPS).enumerate() {
            s[4 + 2 * i] = f([st.center.x, st.center.y, st.axis.x, st.axis.y]);
            s[5 + 2 * i] = f([st.half.x, st.half.y, st.depth, 0.0]);
        }
        s
    }

    /// The render X, Z of texel `addr`'s centre in the new window.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn texel_center(&self, addr: IVec2) -> Vec2 {
        let rel = (addr - self.new_min).rem_euclid(IVec2::splat(RING_SIZE)).as_vec2() + 0.5;
        self.corner + rel * RING_TEXEL_M as f32
    }

    /// The pass on a CPU ring (`RING_SIZE`², row-major by Z then X): the GPU's result.
    pub fn run(&self, prev: &[[f32; 2]]) -> Vec<[f32; 2]> {
        let n = RING_SIZE;
        let stamps = &self.stamps[..self.stamps.len().min(MAX_STAMPS)];
        let mut out = vec![[0.0; 2]; prev.len()];
        for z in 0..n {
            for x in 0..n {
                let addr = IVec2::new(x, z);
                #[allow(clippy::cast_sign_loss)]
                let i = (z * n + x) as usize;
                let last = if kept(addr, self.old_min, self.new_min, n) { prev[i] } else { [0.0; 2] };
                out[i] = texel_update(last, self.texel_center(addr), stamps, self.refill_m, self.depth_m);
            }
        }
        out
    }
}
