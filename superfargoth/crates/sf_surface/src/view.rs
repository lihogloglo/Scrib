//! Presentation helpers for the GPU side (surface-weather.md "GPU resources", W2), Bevy-free so
//! they test in seconds: the toroidal cell window and its texels, the vertical shelter view and
//! its redraw triggers, the noise atlas, and the debug views. Nothing here feeds gameplay
//! (design rule 2): `sf_surface_bevy` uploads it.

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};
use glam::{DVec3, IVec2, Mat4, UVec2, Vec3};
use sf_weather::ClimateIx;

use crate::{CellGrid, SurfaceState, Worldspace};

/// Cell window edge, cells (texels): 128 × 32 m = 4 km around the camera.
pub const WINDOW_CELLS: i32 = 128;

/// A half-open rectangle of world cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellRect {
    pub min: IVec2,
    pub size: IVec2,
}

impl CellRect {
    pub fn area(self) -> i32 {
        self.size.x * self.size.y
    }

    pub fn contains(self, c: IVec2) -> bool {
        let l = c - self.min;
        l.x >= 0 && l.y >= 0 && l.x < self.size.x && l.y < self.size.y
    }
}

/// One upload: a texel rectangle of the window texture (no wrap inside) and the world cell at
/// its first texel. Texel `(x, z)` of a cell is `cell.rem_euclid(WINDOW_CELLS)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TexelRect {
    pub texel: UVec2,
    pub size: UVec2,
    pub first_cell: IVec2,
}

/// The toroidal window of cells around the camera, world-anchored ([`crate::cell_of`]): the cell
/// holding the camera sits at `min + WINDOW_CELLS / 2`. A recenter writes only the strips that
/// enter; every texel always holds the cell `≡ texel (mod WINDOW_CELLS)` inside the window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CellWindow {
    min: Option<IVec2>,
}

impl CellWindow {
    /// The first cell, once placed.
    pub fn min(&self) -> Option<IVec2> {
        self.min
    }

    /// The window as a rectangle, once placed.
    pub fn rect(&self) -> Option<CellRect> {
        self.min.map(|min| CellRect {
            min,
            size: IVec2::splat(WINDOW_CELLS),
        })
    }

    /// Forgets the placement (the next [`Self::recenter`] rewrites everything).
    pub fn reset(&mut self) {
        self.min = None;
    }

    /// Centres the window on `center` (the camera's cell). Returns the cells entering it,
    /// disjoint: everything on the first call or a jump of a window or more, else at most one
    /// column strip and one row strip; nothing when the centre stayed.
    pub fn recenter(&mut self, center: IVec2) -> Vec<CellRect> {
        let b = center - IVec2::splat(WINDOW_CELLS / 2);
        let n = WINDOW_CELLS;
        let old = self.min.replace(b);
        let Some(a) = old else {
            return vec![CellRect {
                min: b,
                size: IVec2::splat(n),
            }];
        };
        let d = b - a;
        if d == IVec2::ZERO {
            return Vec::new();
        }
        if d.x.abs() >= n || d.y.abs() >= n {
            return vec![CellRect {
                min: b,
                size: IVec2::splat(n),
            }];
        }
        let mut out = Vec::with_capacity(2);
        // Columns that enter (all rows of the new window).
        if d.x != 0 {
            let x0 = if d.x > 0 { a.x + n } else { b.x };
            out.push(CellRect {
                min: IVec2::new(x0, b.y),
                size: IVec2::new(d.x.abs(), n),
            });
        }
        // Rows that enter, over the columns both windows share.
        if d.y != 0 {
            let x0 = a.x.max(b.x);
            let x1 = a.x.min(b.x) + n;
            let z0 = if d.y > 0 { a.y + n } else { b.y };
            out.push(CellRect {
                min: IVec2::new(x0, z0),
                size: IVec2::new(x1 - x0, d.y.abs()),
            });
        }
        out
    }

    /// The texel rectangles of `r` (at most a window wide), split where the torus wraps.
    #[allow(clippy::cast_sign_loss)]
    pub fn texel_rects(r: CellRect) -> Vec<TexelRect> {
        let n = WINDOW_CELLS;
        let split = |start: i32, len: i32| -> Vec<(i32, i32)> {
            let t = start.rem_euclid(n);
            if t + len <= n {
                vec![(start, len)]
            } else {
                vec![(start, n - t), (start + n - t, len - (n - t))]
            }
        };
        let mut out = Vec::with_capacity(4);
        for &(z, h) in &split(r.min.y, r.size.y) {
            for &(x, w) in &split(r.min.x, r.size.x) {
                let first = IVec2::new(x, z);
                out.push(TexelRect {
                    texel: first.rem_euclid(IVec2::splat(n)).as_uvec2(),
                    size: UVec2::new(w as u32, h as u32),
                    first_cell: first,
                });
            }
        }
        out
    }
}

/// Where a cell's wetness and puddles come from.
#[derive(Clone, Copy)]
pub enum CellSites<'a> {
    /// Each cell's own climates (the worldspace's [`CellGrid`]); cells outside it are dry.
    Grid(&'a CellGrid),
    /// One site for every cell (no grid: the camera's climates).
    Uniform(&'a [(ClimateIx, f32)]),
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn unorm8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// One cell's texel, RGBA8 unorm: R wet, G puddle (the cell's climate blend), B snow, A ash.
#[allow(clippy::implicit_hasher)]
pub fn cell_texel(
    state: &SurfaceState,
    sites: CellSites<'_>,
    cover: Option<&std::collections::HashMap<IVec2, crate::CellSurface>>,
    cell: IVec2,
) -> [u8; 4] {
    let (mut wet, mut puddle) = (0.0, 0.0);
    let mut add = |c: ClimateIx, w: f32| {
        let s = state.climate(c);
        wet += s.wet * w;
        puddle += s.puddle * w;
    };
    match sites {
        CellSites::Grid(g) => {
            if let Some(ix) = g.index(cell) {
                g.site(ix).iter().for_each(|e| add(e.climate, e.weight));
            }
        }
        CellSites::Uniform(site) => site.iter().for_each(|&(c, w)| add(c, w)),
    }
    let cs = cover
        .and_then(|m| m.get(&cell))
        .copied()
        .unwrap_or_default();
    [unorm8(wet), unorm8(puddle), unorm8(cs.snow), unorm8(cs.ash)]
}

/// The texels of `r` (row-major from `r.first_cell`), appended to `out`; returns the per-channel
/// maximum (layer flags).
#[allow(clippy::cast_possible_wrap)]
pub fn fill_texels(
    state: &SurfaceState,
    sites: CellSites<'_>,
    ws: Option<&Worldspace>,
    r: TexelRect,
    out: &mut Vec<u8>,
) -> [u8; 4] {
    let cover = ws.and_then(|ws| state.cells.get(ws));
    let mut max = [0u8; 4];
    out.reserve((r.size.x * r.size.y * 4) as usize);
    for z in 0..r.size.y as i32 {
        for x in 0..r.size.x as i32 {
            let t = cell_texel(state, sites, cover, r.first_cell + IVec2::new(x, z));
            for i in 0..4 {
                max[i] = max[i].max(t[i]);
            }
            out.extend_from_slice(&t);
        }
    }
    max
}

// ---------------------------------------------------------------------------------------------
// Shelter view: a vertical depth map of everything above, around the camera.

/// Shelter map edge, texels.
pub const SHELTER_SIZE: u32 = 1024;
/// Half the covered square, meters (128 m across: 0.125 m texels).
pub const SHELTER_HALF_M: f32 = 64.0;
/// Shelter texel, meters.
pub const SHELTER_TEXEL_M: f32 = 2.0 * SHELTER_HALF_M / 1024.0;
/// Depth range above the map's centre (the top plane; casters above clamp to it) and below it.
pub const SHELTER_UP_M: f32 = 200.0;
pub const SHELTER_DOWN_M: f32 = 150.0;
/// Redraw when the camera moved this far from the last centre, horizontally / vertically.
pub const SHELTER_RECENTER_M: f32 = 16.0;
pub const SHELTER_RECENTER_Y_M: f32 = 8.0;
/// Fewest frames between redraws forced by casters streaming in.
pub const SHELTER_REFRESH_FRAMES: u32 = 30;
/// Share of the rain a full leaf canopy lets through (owner 2026-11-05: "~40% passes
/// through"; roofs let none). Casters the game marks as foliage draw into the canopy map, and
/// the layer scales exposure by `1 − (1 − pass)·leaf cover` (Clockwork `LEAF_HOLD`, its
/// `roofLeaves`, holds back at most 0.4; ours holds back 0.6, the owner's number).
pub const SHELTER_LEAF_PASS: f32 = 0.4;

/// The shelter centre for a camera at `p` (render space): snapped to whole texels so a
/// redraw never shifts the texel grid against the world (no shimmer between redraws).
pub fn shelter_center(p: Vec3) -> Vec3 {
    (p / SHELTER_TEXEL_M).round() * SHELTER_TEXEL_M
}

/// The shelter centre for a camera at render `p` with the floating origin at world `origin`:
/// X and Z snapped to whole texels of the *world* grid (Y in render space, depth only), as
/// `(render centre, world texel of the centre)`. Two redraws then differ by a whole number of
/// texels (the exposure pack's shift, [`ExposureMode::Carry`]) across a floating-origin shift
/// too. With the origin at zero it is [`shelter_center`].
#[allow(clippy::cast_possible_truncation)]
pub fn shelter_center_world(origin: DVec3, p: Vec3) -> (Vec3, IVec2) {
    let t = f64::from(SHELTER_TEXEL_M);
    let world = origin + p.as_dvec3();
    let tx = (world.x / t).round();
    let tz = (world.z / t).round();
    let center = Vec3::new(
        (tx * t - origin.x) as f32,
        (p.y / SHELTER_TEXEL_M).round() * SHELTER_TEXEL_M,
        (tz * t - origin.z) as f32,
    );
    (center, IVec2::new(tx as i32, tz as i32))
}

/// The vertical view centred on `center`: `(view_from_world, clip_from_world)`, orthographic,
/// looking down −Y with −Z up (view X = world X), reverse Z: depth 1 at the top plane
/// ([`SHELTER_UP_M`] above), 0 at the bottom ([`SHELTER_DOWN_M`] below).
pub fn shelter_view(center: Vec3) -> (Mat4, Mat4) {
    let eye = center + Vec3::Y * SHELTER_UP_M;
    let view = look_to_mat4(eye, Vec3::NEG_Y, Vec3::NEG_Z);
    let h = SHELTER_HALF_M;
    let proj = orthographic(-h, h, -h, h, SHELTER_UP_M + SHELTER_DOWN_M, 0.0);
    (view, proj * view)
}

/// Why a redraw happens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Redraw {
    /// First draw, or the map was off.
    Fresh,
    /// The camera moved past [`SHELTER_RECENTER_M`] / [`SHELTER_RECENTER_Y_M`].
    Moved,
    /// Casters streamed in (at most every [`SHELTER_REFRESH_FRAMES`]).
    Streamed,
}

/// The shelter map's redraw triggers.
#[derive(Clone, Copy, Debug, Default)]
pub struct ShelterTrigger {
    center: Option<Vec3>,
    texel: Option<IVec2>,
    streamed: bool,
    frames: u32,
}

impl ShelterTrigger {
    /// The centre of the last redraw.
    pub fn center(&self) -> Option<Vec3> {
        self.center
    }

    /// The world texel of the last redraw's centre ([`shelter_center_world`]).
    pub fn center_texel(&self) -> Option<IVec2> {
        self.texel
    }

    /// The map is off: the next frame redraws fresh.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// One frame with the camera at `camera` (render space); `streamed`: shelter casters were
    /// added this frame. Returns the redraw (and moves the centre) when one is due.
    pub fn frame(&mut self, camera: Vec3, streamed: bool) -> Option<Redraw> {
        self.frame_world(DVec3::ZERO, camera, streamed)
    }

    /// [`Self::frame`] with the floating origin at world `origin`: the centre snaps to the world
    /// texel grid ([`shelter_center_world`]).
    pub fn frame_world(&mut self, origin: DVec3, camera: Vec3, streamed: bool) -> Option<Redraw> {
        self.streamed |= streamed;
        self.frames = self.frames.saturating_add(1);
        let why = match self.center {
            None => Some(Redraw::Fresh),
            Some(c)
                if (c - camera).with_y(0.0).length() > SHELTER_RECENTER_M
                    || (c.y - camera.y).abs() > SHELTER_RECENTER_Y_M =>
            {
                Some(Redraw::Moved)
            }
            Some(_) if self.streamed && self.frames >= SHELTER_REFRESH_FRAMES => {
                Some(Redraw::Streamed)
            }
            Some(_) => None,
        }?;
        if why != Redraw::Streamed {
            let (center, texel) = shelter_center_world(origin, camera);
            self.center = Some(center);
            self.texel = Some(texel);
        }
        self.streamed = false;
        self.frames = 0;
        Some(why)
    }
}

// ---------------------------------------------------------------------------------------------
// Exposure pack (SW-5/SW-6): the shelter result eased over time instead of swapped per redraw.
//
// The layer reads one `Rgba16Float` texture in the shelter map's texels (`sf_surface_bevy`
// `exposure.rs` / `exposure.wgsl`): R and G the solid and leaf casters' heights over the map
// centre (copied from the two depth maps), B the *lag* (eased exposure minus the fresh one), A
// the fresh ground exposure (the layer's tent for a receiver below every caster, edge fade
// included). The fragment keeps its exact 3D test on R and G and adds the lag: steady, the lag
// is 0 and the layer is unchanged; after a redraw the ground keeps what it showed (lag = shown -
// fresh) and the lag decays to 0 over ~2.4 s. The lag is the ground's: a receiver on its
// texel's top surface (a roof, a leaf, a static floor) keeps the exact test, only what lies under
// the top is eased (`layer.wesl` `SHELTER_LAG_TOP_M`). The pack is world-anchored: a redraw's pass reads
// last pack's texel at `texel + shift` (the centres' world-texel difference), so a world point
// keeps its eased value however the map moves; a texel the old map did not cover showed open sky.
// Like persistent snow-accumulation maps (RDR2, Horizon) and stable shadow cascades: the map
// never pops with the camera, it fades.

/// The lag's time constant, seconds: 95 % of a change shows after `3 × TAU` (2.4 s).
pub const EXPOSURE_TAU_S: f32 = 0.8;
/// Easing passes run this long after a redraw is taken in (the lag is then under e⁻⁵ ≈ 0.7 %),
/// the last one snaps it to 0; steady frames run no pass.
pub const EXPOSURE_RUN_S: f32 = 5.0 * EXPOSURE_TAU_S;

/// The lag's factor over `dt` seconds: frame-rate independent (`decay(a) · decay(b) =
/// decay(a + b)`).
pub fn ease_decay(dt: f32) -> f32 {
    (-dt.max(0.0) / EXPOSURE_TAU_S).exp()
}

/// What one exposure pass does to the pack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExposureMode {
    /// A map with no history (first draw, the map was off): lag 0.
    Fresh,
    /// Take in a redraw whose centre is `shift` world texels from the held one: each texel keeps
    /// what it showed (last pack's texel at `texel + shift`; open sky outside it).
    Carry(IVec2),
    /// Decay the lag only (the maps did not change).
    Ease,
}

impl ExposureMode {
    /// The mode's code in the pass's params (`exposure.wgsl` `MODE_*`).
    pub fn code(self) -> i32 {
        match self {
            ExposureMode::Fresh => 0,
            ExposureMode::Carry(_) => 1,
            ExposureMode::Ease => 2,
        }
    }

    /// The shift a [`ExposureMode::Carry`] reads at (zero otherwise).
    pub fn shift(self) -> IVec2 {
        match self {
            ExposureMode::Carry(s) => s,
            _ => IVec2::ZERO,
        }
    }
}

/// One frame's exposure pass: the mode and the lag's factor this pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExposurePass {
    pub mode: ExposureMode,
    pub decay: f32,
}

/// The pack texel a new texel `texel` takes its history from after a shift: `texel + shift`,
/// when last pack held it.
pub fn exposure_source(texel: IVec2, shift: IVec2) -> Option<IVec2> {
    let s = texel + shift;
    let n = SHELTER_SIZE.cast_signed();
    (s.x >= 0 && s.y >= 0 && s.x < n && s.y < n).then_some(s)
}

/// One pack texel's `(lag, fresh)` after a pass (the GPU twin is `exposure.wgsl` `main`): `held`
/// is last pack's `(lag, fresh)` at the source texel (`None`: outside it), `fresh` this pass's
/// ground exposure (ignored by [`ExposureMode::Ease`], which keeps last pack's).
pub fn exposure_texel(pass: ExposurePass, held: Option<(f32, f32)>, fresh: f32) -> (f32, f32) {
    match pass.mode {
        ExposureMode::Fresh => (0.0, fresh),
        ExposureMode::Carry(_) => {
            // What the texel showed: eased = lag + fresh; open sky where the old map ended.
            let shown = held.map_or(1.0, |(lag, f)| lag + f);
            ((shown - fresh) * pass.decay, fresh)
        }
        ExposureMode::Ease => {
            let (lag, f) = held.unwrap_or((0.0, fresh));
            (lag * pass.decay, f)
        }
    }
}

/// The exposure pack's clock (main world): which pass runs this frame. A redraw requested on
/// frame N is drawn by frame N's render (Bevy's shadow pass, after the prepass), so frame N + 1's
/// pass (before the prepass) takes it in, and the uniform switches to its clip on that frame:
/// every pass of a frame sees one consistent map.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExposureClock {
    held: Option<IVec2>,
    pending: Option<IVec2>,
    easing: Option<f32>,
}

impl ExposureClock {
    /// The map is off: the next redraw starts fresh.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// The world texel of the map the pack holds.
    pub fn held(&self) -> Option<IVec2> {
        self.held
    }

    /// Whether a redraw waits for next frame's pass.
    pub fn pending(&self) -> bool {
        self.pending.is_some()
    }

    /// A redraw centred on world texel `center` was requested this frame.
    pub fn redraw(&mut self, center: IVec2) {
        self.pending = Some(center);
    }

    /// This frame's pass, `dt` seconds after the last frame; call before this frame's
    /// [`Self::redraw`]. A pending redraw is taken in (lag carried, no decay this frame: it
    /// shows exactly what the old map showed), then [`EXPOSURE_RUN_S`] of easing passes, the
    /// last with factor 0; `None` on steady frames.
    pub fn frame(&mut self, dt: f32) -> Option<ExposurePass> {
        if let Some(center) = self.pending.take() {
            let mode = self.held.map_or(ExposureMode::Fresh, |h| ExposureMode::Carry(center - h));
            self.held = Some(center);
            self.easing = (mode != ExposureMode::Fresh).then_some(0.0);
            return Some(ExposurePass { mode, decay: 1.0 });
        }
        let elapsed = {
            let t = self.easing.as_mut()?;
            *t += dt.max(0.0);
            *t
        };
        let decay = if elapsed >= EXPOSURE_RUN_S {
            self.easing = None;
            0.0
        } else {
            ease_decay(dt)
        };
        Some(ExposurePass { mode: ExposureMode::Ease, decay })
    }
}

// ---------------------------------------------------------------------------------------------
// Noise atlas.

/// Noise atlas edge, texels.
pub const NOISE_SIZE: u32 = 256;

fn hash(x: u32, y: u32, seed: u32) -> u32 {
    let mut h =
        x.wrapping_mul(0x8da6_b343) ^ y.wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297a_2d39);
    h ^ (h >> 15)
}

#[allow(clippy::cast_precision_loss)]
fn lattice(x: u32, y: u32, seed: u32) -> f32 {
    (hash(x, y, seed) >> 8) as f32 / 16_777_215.0
}

/// Tileable value noise at texel `(x, y)`: `cells` lattice cells across the atlas in x, `rows`
/// in y (both divide [`NOISE_SIZE`]), smoothstep-interpolated.
#[allow(clippy::cast_precision_loss, clippy::many_single_char_names)]
fn value_noise(x: u32, y: u32, cells: u32, rows: u32, seed: u32) -> f32 {
    let (cw, ch) = (NOISE_SIZE / cells, NOISE_SIZE / rows);
    let (fx, fy) = ((x % cw) as f32 / cw as f32, (y % ch) as f32 / ch as f32);
    let (x0, y0) = (x / cw, y / ch);
    let (x1, y1) = ((x0 + 1) % cells, (y0 + 1) % rows);
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sy) = (s(fx), s(fy));
    let a = lattice(x0, y0, seed) + (lattice(x1, y0, seed) - lattice(x0, y0, seed)) * sx;
    let b = lattice(x0, y1, seed) + (lattice(x1, y1, seed) - lattice(x0, y1, seed)) * sx;
    a + (b - a) * sy
}

/// The static noise atlas, RGBA8, [`NOISE_SIZE`]², tiling: R and G two-octave value noise
/// (patches, edges; G at a finer scale), B and A wall streaks (narrow in x, long in y).
#[allow(clippy::many_single_char_names)]
pub fn noise_atlas() -> Vec<[u8; 4]> {
    let n = NOISE_SIZE;
    let mut out = Vec::with_capacity((n * n) as usize);
    for y in 0..n {
        for x in 0..n {
            let r = 0.65 * value_noise(x, y, 8, 8, 1) + 0.35 * value_noise(x, y, 32, 32, 2);
            let g = 0.65 * value_noise(x, y, 16, 16, 3) + 0.35 * value_noise(x, y, 64, 64, 4);
            let b = value_noise(x, y, 64, 4, 5) * value_noise(x, y, 128, 2, 6);
            let a = value_noise(x, y, 32, 8, 7) * value_noise(x, y, 128, 4, 8);
            out.push([unorm8(r), unorm8(g), unorm8(b), unorm8(a)]);
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Debug views.

/// `surface debug <...>`: what the layer shows instead of its look (like terrain's LOD tint).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DebugView {
    #[default]
    Off = 0,
    /// Shelter exposure: open sky warm, sheltered blue.
    Exposure = 1,
    /// The cell texture: wet, puddle, snow, ash per 32 m cell, with cell borders.
    Cells = 2,
    /// Lagarde porosity per pixel: sealed black (metal, polish) to white (drinks); far red.
    Porosity = 3,
    /// The trail ring (W6a): prints blue (deeper brighter), lips red, outside the ring purple.
    Trails = 4,
}

impl DebugView {
    pub const ALL: [DebugView; 5] = [
        DebugView::Off,
        DebugView::Exposure,
        DebugView::Cells,
        DebugView::Porosity,
        DebugView::Trails,
    ];

    pub fn name(self) -> &'static str {
        match self {
            DebugView::Off => "off",
            DebugView::Exposure => "exposure",
            DebugView::Cells => "cells",
            DebugView::Porosity => "porosity",
            DebugView::Trails => "trails",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|v| v.name().eq_ignore_ascii_case(s))
    }
}
