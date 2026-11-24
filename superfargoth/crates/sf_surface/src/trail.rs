//! Trails (surface-weather.md "Trails (W6) and why DLSS needs motion vectors"; slice W6a): the
//! CPU twin of the trail ring's compute pass (`sf_surface_bevy` `trail.wgsl`) and the packing of
//! its parameters, so the GPU math is tested exactly here.
//!
//! - **Ring:** [`RING_SIZE`]² texels of [`RING_TEXEL_M`] (64 m), toroidal and camera-centred like
//!   the height clipmap: the window's first texel is a whole world texel index ([`window_min`]),
//!   a texel's world index follows from its address ([`world_of`]), and a texel that held another
//!   world texel last frame (it entered the ring) starts from 0 ([`kept`]).
//! - **Texel:** R the depression (m, ≤ 0, down to `-depth`: the stamp carves the cover to the
//!   ground), G the lip (m, ≥ 0). The GPU texture is `Rgba16Float` (a core storage format); B the
//!   deformer ceiling (S1a), A the press (GC-T1).
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
//! - **Press** (A; GC-T1, groundcover.md "One trample source"): a press strength 0..1, independent
//!   of the cover depth, that plants and blades read (GC-M3, GC-3d). Each stamp writes
//!   [`press_profile`] around its centre (Max), every pass decays it by `exp(-dt / recovery)`
//!   ([`Press::keep`]) and snaps it to 0 under [`PRESS_FLOOR`] ([`press_update`]). Decay is
//!   released in steps of at least [`PRESS_STEP`] recovery times ([`PressClock`]) so a half float
//!   never rounds a step away, and the pass runs until the last press is under the floor
//!   ([`press_bound`]): a CPU time bound, no readback. The standard world-anchored interaction
//!   target decayed per frame (UE5 interactive foliage, Horizon's grass). The deformer capture
//!   presses it too (GC-T2b, [`crate::deform::press_of`]): whole legs, bodies and creatures.
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

/// A press under this snaps to 0 (A, GC-T1): one 8-bit step, so no reader moves visibly for it,
/// and well inside the half float's normal range (≥ 2⁻¹⁴, 11-bit significand), so its decay
/// steps stay exact. A format constant (the 8-bit step and the half float), not a tunable.
// tunables: const (a format: one 8-bit step, inside the half float's normal range)
pub const PRESS_FLOOR: f32 = 1.0 / 256.0;
/// Press decay is released in steps of at least this many recovery times: each step shrinks the
/// press by ≥ 0.78 %, 16 × a half float's rounding ([`HALF_ROUNDING`]), so no step rounds away.
// tunables: const (a format margin: 16 × the half float's rounding; the decay is exp(-dt / recovery) whatever it is)
pub const PRESS_STEP: f32 = 1.0 / 128.0;
/// A half float's round-to-nearest relative error bound (2⁻¹¹, 11-bit significand).
// tunables: const (a format: the half float's 11-bit significand)
pub const HALF_ROUNDING: f32 = 1.0 / 2048.0;

/// Uniform header slots: window, mins, counts, shape, then the deformer capture's three
/// (`capture`, `slab`, `ground`; S1a), then the press (`press`, GC-T1).
pub const HEADER_SLOTS: usize = 8;
/// Uniform words: the header's `vec4` slots, then two per stamp.
pub const PARAM_SLOTS: usize = HEADER_SLOTS + 2 * MAX_STAMPS;

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

/// The press (A) at distance `r` (m) from a stamp's centre: 1 at the centre, 0 from
/// `radius_m` on, a smoothstep between (0.5 at half the radius). Round, not the print's ellipse:
/// a foot bends the plants around it on every side, whatever its facing.
pub fn press_profile(r: f32, radius_m: f32) -> f32 {
    if radius_m <= 0.0 {
        return 0.0;
    }
    1.0 - smoothstep(0.0, 1.0, r / radius_m)
}

/// The press pass's parameters (GC-T1; uniform slot `press`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Press {
    /// Decay released this pass (s; 0 = none, [`PressClock::advance`]).
    pub dt_s: f32,
    /// `surface.trail.press_recovery_s`: the spring back's e-folding time (s).
    pub recovery_s: f32,
    /// `surface.trail.press_radius_m`: a stamp's press reach (m; 0 = stamps press nothing).
    pub radius_m: f32,
}

impl Press {
    /// The share of the press kept this pass: `exp(-dt / recovery)` (no recovery time: none).
    pub fn keep(&self) -> f32 {
        if self.dt_s <= 0.0 {
            1.0
        } else if self.recovery_s <= 0.0 {
            0.0
        } else {
            (-self.dt_s / self.recovery_s).exp()
        }
    }
}

/// One texel's press: its last value (0 if it entered the ring) decayed, stamped (Max) at its
/// centre `q` (render XZ), merged (Max) with the deformer capture's press `captured` there
/// (GC-T2b; spread, GC-T3: [`crate::deform::Capture::spread_press_at`]; 0 = none), snapped to 0 under [`PRESS_FLOOR`].
/// `trail.wgsl` `main`.
pub fn press_update(prev: f32, q: Vec2, stamps: &[Stamp], press: &Press, captured: f32) -> f32 {
    let mut a = prev * press.keep();
    for s in stamps {
        a = a.max(press_profile(q.distance(s.center), press.radius_m));
    }
    a = a.max(captured);
    if a < PRESS_FLOOR { 0.0 } else { a }
}

/// Recovery times of decay after the last press stamp that bring any press (≤ 1) under
/// [`PRESS_FLOOR`] even when every stored step rounds up by [`HALF_ROUNDING`] (at most one
/// rounding per [`PRESS_STEP`]): `ln(1 / floor) / (1 − rounding / step)` ≈ 5.9.
pub fn press_bound() -> f32 {
    (1.0 / PRESS_FLOOR).ln() / (1.0 - HALF_ROUNDING / PRESS_STEP)
}

/// The CPU side of the press decay (GC-T1): accumulates frame time and releases it in steps of
/// at least [`PRESS_STEP`] recovery times while a press may still stand above the floor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PressClock {
    /// Time not released yet (s).
    pub pending_s: f32,
    /// Recovery times released since the last press stamp (`None`: every press is under the
    /// floor, the pass has no press work).
    pub decayed: Option<f32>,
}

impl PressClock {
    /// Advances by `dt_s` (negative: ignored) at this frame's recovery time; `pressed`: a stamp
    /// writes a press this frame (after this pass's decay). Returns the decay this pass applies
    /// ([`Press::dt_s`]; 0 = none): non-zero means the pass must run.
    pub fn advance(&mut self, dt_s: f32, recovery_s: f32, pressed: bool) -> f32 {
        let mut out = 0.0;
        if let Some(done) = self.decayed {
            self.pending_s += dt_s.max(0.0);
            if self.pending_s > 0.0 && (recovery_s <= 0.0 || self.pending_s >= PRESS_STEP * recovery_s) {
                out = std::mem::take(&mut self.pending_s);
                let x = if recovery_s > 0.0 { done + out / recovery_s } else { f32::INFINITY };
                self.decayed = (x < press_bound()).then_some(x);
            }
        }
        if pressed {
            self.decayed = Some(0.0);
        }
        if self.decayed.is_none() {
            self.pending_s = 0.0;
        }
        out
    }

    /// Whether a press may still stand above the floor.
    pub fn live(&self) -> bool {
        self.decayed.is_some()
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
    /// The deformer capture drawn last frame (S1a, `crate::deform`), merged into B.
    pub capture: Option<crate::deform::Capture>,
    /// The press channel's decay and stamp reach (A, GC-T1).
    pub press: Press,
}

impl TrailPass {
    /// The uniform as `vec4<u32>` slots (f32 / i32 bit-cast).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn slots(&self) -> Vec<[u32; 4]> {
        let f = |v: [f32; 4]| v.map(f32::to_bits);
        let old = self.old_min.unwrap_or(IVec2::ZERO);
        let n = self.stamps.len().min(MAX_STAMPS);
        let mut s = vec![[0; 4]; PARAM_SLOTS];
        s[0] = f([self.corner.x, self.corner.y, RING_TEXEL_M as f32, self.depth_m]);
        s[1] = [self.new_min.x, self.new_min.y, old.x, old.y].map(|v| v as u32);
        let size = self.capture.as_ref().map_or(0, |c| c.size);
        s[2] = [n as u32, u32::from(self.old_min.is_some()), RING_SIZE as u32, size];
        s[3] = f([self.refill_m, LIP_HEIGHT, LIP_WIDTH, WALL]);
        if let Some(c) = &self.capture {
            let g = &c.ground;
            s[4] = f([c.corner.x, c.corner.y, c.texel_m, c.near_y]);
            s[5] = f([c.range_m, crate::deform::ceiling_ref(self.depth_m), g.corner.x, g.corner.y]);
            s[6] = f([g.step_m, g.edge as f32, c.press_m, c.spread_m]);
        }
        s[7] = f([self.press.dt_s, self.press.recovery_s, self.press.radius_m, PRESS_FLOOR]);
        for (i, st) in self.stamps.iter().take(MAX_STAMPS).enumerate() {
            s[HEADER_SLOTS + 2 * i] = f([st.center.x, st.center.y, st.axis.x, st.axis.y]);
            s[HEADER_SLOTS + 1 + 2 * i] = f([st.half.x, st.half.y, st.depth, 0.0]);
        }
        s
    }

    /// The ground grid's storage words (`trail.wgsl` `ground`; at least one: a binding is never
    /// empty).
    pub fn ground_words(&self) -> Vec<f32> {
        match &self.capture {
            Some(c) if !c.ground.heights.is_empty() => c.ground.heights.clone(),
            _ => vec![0.0],
        }
    }

    /// The pass's B channel (the deformer ceiling, S1a) on a CPU ring (`RING_SIZE`², row-major
    /// by Z then X) with the drawn capture target `depth` (all of it, `deform::MAX_CAPTURE_SIZE`²,
    /// ignored without a capture): the GPU's result.
    pub fn run_ceiling(&self, prev: &[f32], depth: &[f32]) -> Vec<f32> {
        let n = RING_SIZE;
        let mut out = vec![0.0; prev.len()];
        for z in 0..n {
            for x in 0..n {
                let addr = IVec2::new(x, z);
                #[allow(clippy::cast_sign_loss)]
                let i = (z * n + x) as usize;
                let last = if kept(addr, self.old_min, self.new_min, n) { prev[i] } else { 0.0 };
                let captured = self.capture.as_ref().map_or(0.0, |c| c.value_at(self.texel_center(addr), depth, self.depth_m));
                out[i] = crate::deform::ceiling_update(last, self.refill_m, captured);
            }
        }
        out
    }

    /// The pass's A channel (the press, GC-T1; the capture's, GC-T2b, spread GC-T3) on a CPU ring (`RING_SIZE`²,
    /// row-major by Z then X) with the drawn capture target `depth` (as [`TrailPass::run_ceiling`];
    /// ignored without a capture): the GPU's result.
    pub fn run_press(&self, prev: &[f32], depth: &[f32]) -> Vec<f32> {
        let n = RING_SIZE;
        let stamps = &self.stamps[..self.stamps.len().min(MAX_STAMPS)];
        let mut out = vec![0.0; prev.len()];
        for z in 0..n {
            for x in 0..n {
                let addr = IVec2::new(x, z);
                #[allow(clippy::cast_sign_loss)]
                let i = (z * n + x) as usize;
                let last = if kept(addr, self.old_min, self.new_min, n) { prev[i] } else { 0.0 };
                let q = self.texel_center(addr);
                let captured = self.capture.as_ref().map_or(0.0, |c| c.spread_press_at(q, depth));
                out[i] = press_update(last, q, stamps, &self.press, captured);
            }
        }
        out
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

/// The trail's height (m) in a cover of depth `d` (`layer.wesl` `surface_trail_height`): the
/// print down to the ground (`-d` at most) plus the lip scaled to this cover (`ring` the ring
/// depth, > 0).
pub fn trail_height(v: [f32; 2], d: f32, ring: f32) -> f32 {
    v[0].clamp(-d, 0.0) + v[1] * d / ring
}

/// Raised cover (W6b; `layer.wesl` `surface_cover_raise`): how high a cover of depth `d` stands
/// over the ground at `amount` (cell × slope × exposure) over a relief whose high point is `top`
/// (`layer::cover_height` of the surface's p90 height, the look and the drift; 0 = flat) and the
/// ring value `v` (`ring` the ring depth, 0 = no trail): the bury depth's share of the depth
/// (`layer::cover_bury`), prints cut to the ground, lips on top. Zero until the cover buries the
/// relief: snow in a road's grout is shading only, the stones never rise with it. The terrain's
/// visual height is ground + this (summed over the channels).
pub fn cover_raise(amount: f32, top: f32, d: f32, v: [f32; 2], ring: f32) -> f32 {
    let bury = crate::layer::cover_bury(amount, top);
    if bury <= 0.0 || d <= 0.0 {
        return 0.0;
    }
    let h = if ring > 0.0 { trail_height(v, d, ring) } else { 0.0 };
    bury * (d + h).max(0.0)
}
