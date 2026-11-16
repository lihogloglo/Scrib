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

/// The outer part of a level's map (fraction of its half extent) where it hands over to the next
/// level out, or fades to open sky past the last: `layer.wesl` / `exposure.wgsl`
/// `SHELTER_FADE_START` (a test checks they match).
// tunables: const (twin of the shader's SHELTER_FADE_START, counted there)
pub const SHELTER_FADE_START: f32 = 0.875;

/// One level of the shelter clipmap (SW-5e): a vertical depth capture of [`SHELTER_SIZE`]²
/// texels over `2 × half_m`, its centre snapped to the world texel grid, redrawn when the camera
/// moved `recenter_m` / `recenter_y_m` from it or casters streamed in (at most every
/// `refresh_frames`). The near level is [`NEAR_LEVEL`] (128 m at 0.125 m); the far level
/// ([`ShelterLevel::far`]) is the same capture over a wider square at a coarser texel, drawn at a
/// coarser fixed caster LOD, like a stable shadow cascade.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShelterLevel {
    /// Half the covered square, meters.
    pub half_m: f32,
    /// Depth range above the centre (the top plane; casters above clamp to it) and below it.
    pub up_m: f32,
    pub down_m: f32,
    /// Redraw when the camera moved this far from the centre, horizontally / vertically.
    pub recenter_m: f32,
    pub recenter_y_m: f32,
    /// Fewest frames between redraws forced by casters streaming in.
    pub refresh_frames: u32,
}

/// The near level: the shelter map as W2-SW-5d built it.
pub const NEAR_LEVEL: ShelterLevel = ShelterLevel {
    half_m: SHELTER_HALF_M,
    up_m: SHELTER_UP_M,
    down_m: SHELTER_DOWN_M,
    recenter_m: SHELTER_RECENTER_M,
    recenter_y_m: SHELTER_RECENTER_Y_M,
    refresh_frames: SHELTER_REFRESH_FRAMES,
};

impl ShelterLevel {
    /// The far level over `half_m` meters each way, redrawn every `recenter_m`. Derived, no
    /// value of its own: the depth range is the half extent up and down (terrain within a 45°
    /// average slope of the camera out to the edge stays inside it), the vertical recentre is
    /// half the horizontal one and the streamed-redraw interval grows with the recentre step,
    /// both as the near level's ratios.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn far(half_m: f32, recenter_m: f32) -> Self {
        let ratio = (recenter_m / SHELTER_RECENTER_M).max(1.0);
        Self {
            half_m,
            up_m: half_m,
            down_m: half_m,
            recenter_m,
            recenter_y_m: recenter_m * (SHELTER_RECENTER_Y_M / SHELTER_RECENTER_M),
            refresh_frames: (SHELTER_REFRESH_FRAMES as f32 * ratio).round() as u32,
        }
    }

    /// Texel edge, meters.
    #[allow(clippy::cast_precision_loss)]
    pub fn texel_m(&self) -> f32 {
        2.0 * self.half_m / SHELTER_SIZE as f32
    }

    /// Depth range, meters.
    pub fn depth_m(&self) -> f32 {
        self.up_m + self.down_m
    }

    /// How far from the camera the level's unfaded part reaches whatever the camera's place
    /// inside one recentre step: the hand-over band starts at [`SHELTER_FADE_START`] × the half
    /// extent from the centre (the inscribed circle of the square), and the camera is at most
    /// `recenter_m` from the centre (horizontally; the raise's 3D distance is at least that).
    pub fn known_m(&self) -> f32 {
        (self.half_m * SHELTER_FADE_START - self.recenter_m).max(0.0)
    }

    /// The level's centre for a camera at render `p` with the floating origin at world `origin`:
    /// X and Z snapped to whole texels of the *world* grid (Y in render space, depth only), as
    /// `(render centre, world texel of the centre)`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn center_world(&self, origin: DVec3, p: Vec3) -> (Vec3, IVec2) {
        let texel = self.texel_m();
        let t = f64::from(texel);
        let world = origin + p.as_dvec3();
        let tx = (world.x / t).round();
        let tz = (world.z / t).round();
        let center = Vec3::new(
            (tx * t - origin.x) as f32,
            (p.y / texel).round() * texel,
            (tz * t - origin.z) as f32,
        );
        (center, IVec2::new(tx as i32, tz as i32))
    }

    /// The vertical view centred on `center`: `(view_from_world, clip_from_world)`, orthographic,
    /// looking down −Y with −Z up (view X = world X), reverse Z: depth 1 at the top plane
    /// (`up_m` above), 0 at the bottom (`down_m` below).
    pub fn view(&self, center: Vec3) -> (Mat4, Mat4) {
        let eye = center + Vec3::Y * self.up_m;
        let view = look_to_mat4(eye, Vec3::NEG_Y, Vec3::NEG_Z);
        let h = self.half_m;
        let proj = orthographic(-h, h, -h, h, self.depth_m(), 0.0);
        (view, proj * view)
    }

    /// Where render point `p` lies in the level centred at `center`: the larger of |dx|, |dz|
    /// over the half extent (≥ 1: outside the map).
    pub fn edge(&self, center: Vec3, p: Vec3) -> f32 {
        let d = (p - center).abs();
        d.x.max(d.z) / self.half_m
    }
}

/// The hand-over weight at a level's `edge` ([`ShelterLevel::edge`]): 0 inside
/// [`SHELTER_FADE_START`], a smoothstep to 1 at the edge (the next level out, or open sky).
pub fn level_fade(edge: f32) -> f32 {
    let t = ((edge - SHELTER_FADE_START) / (1.0 - SHELTER_FADE_START)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// What the layer shows at a point (CPU twin of `layer.wesl` `surface_exposure`'s level
/// selection): `near` and `far` are each level's `(edge, exposure)` there when its map is valid.
/// The near level inside its range, blended over its band ([`level_fade`]) into what the far
/// level shows; the far level beyond, blended over its own band into open sky (1); open beyond
/// it and with no map.
pub fn blended_exposure(near: Option<(f32, f32)>, far: Option<(f32, f32)>) -> f32 {
    let outer = |level: Option<(f32, f32)>, beyond: f32| match level {
        Some((edge, v)) if edge < 1.0 => v + (beyond - v) * level_fade(edge),
        _ => beyond,
    };
    outer(near, outer(far, 1.0))
}

/// The pack uv mapping from an inner level's map to an outer level's (`exposure.wgsl`
/// `outer`): `uv_outer = uv_inner × scale + offset`, both maps axis-aligned the same way (view X
/// = world X, uv y = world Z), from their clips and half extents.
pub fn outer_uv_map(inner_clip: Mat4, inner_half_m: f32, outer_clip: Mat4, outer_half_m: f32) -> (f32, glam::Vec2) {
    // The inner map's centre in render space (its clip's xy = 0 at y = 0).
    let x = -inner_clip.w_axis.x / inner_clip.x_axis.x;
    let z = -inner_clip.w_axis.y / inner_clip.z_axis.y;
    let c = outer_clip.project_point3(Vec3::new(x, 0.0, z));
    let scale = inner_half_m / outer_half_m;
    let center_uv = glam::Vec2::new(c.x * 0.5 + 0.5, -c.y * 0.5 + 0.5);
    (scale, center_uv - glam::Vec2::splat(0.5 * scale))
}

/// The raised cover's reach (SW-5e): how far from the camera the raise may stand so no texel's
/// cover or raise changes with camera distance. `known_m`: how far shelter is known whatever the
/// camera's place (the outermost valid level's [`ShelterLevel::known_m`]; `None`: no shelter
/// map, exposure 1 everywhere, nothing to know); `caster_reach_m`: how far shelter casters exist
/// (the host's near-field handoff less its fade band: past it only far proxies stand, and they
/// never cast). The terrain fades the raise to exactly 0 by this distance
/// (`sf_terrain::cdlod::CdlodSettings::raise_fade_range`).
pub fn raise_reach_m(known_m: Option<f32>, caster_reach_m: f32) -> f32 {
    known_m.map_or(f32::INFINITY, |k| k.min(caster_reach_m))
}

/// The shelter centre for a camera at `p` (render space): snapped to whole texels so a
/// redraw never shifts the texel grid against the world (no shimmer between redraws).
pub fn shelter_center(p: Vec3) -> Vec3 {
    (p / SHELTER_TEXEL_M).round() * SHELTER_TEXEL_M
}

/// [`ShelterLevel::center_world`] of the near level. Two redraws then differ by a whole number of
/// texels (the exposure pack's shift, [`ExposureMode::Carry`]) across a floating-origin shift
/// too. With the origin at zero it is [`shelter_center`].
pub fn shelter_center_world(origin: DVec3, p: Vec3) -> (Vec3, IVec2) {
    NEAR_LEVEL.center_world(origin, p)
}

/// [`ShelterLevel::view`] of the near level.
pub fn shelter_view(center: Vec3) -> (Mat4, Mat4) {
    NEAR_LEVEL.view(center)
}

/// Why a redraw happens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Redraw {
    /// First draw, or the map was off.
    Fresh,
    /// The camera moved past the level's recentre distances.
    Moved,
    /// Casters streamed in (at most every `refresh_frames`).
    Streamed,
}

/// A shelter level's redraw triggers.
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

    /// The world texel of the last redraw's centre ([`ShelterLevel::center_world`]).
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
        self.frame_level(&NEAR_LEVEL, origin, camera, streamed)
    }

    /// [`Self::frame_world`] for `level`'s recentre distances, refresh interval and texel grid.
    pub fn frame_level(&mut self, level: &ShelterLevel, origin: DVec3, camera: Vec3, streamed: bool) -> Option<Redraw> {
        self.streamed |= streamed;
        self.frames = self.frames.saturating_add(1);
        let why = match self.center {
            None => Some(Redraw::Fresh),
            Some(c)
                if (c - camera).with_y(0.0).length() > level.recenter_m
                    || (c.y - camera.y).abs() > level.recenter_y_m =>
            {
                Some(Redraw::Moved)
            }
            Some(_) if self.streamed && self.frames >= level.refresh_frames => {
                Some(Redraw::Streamed)
            }
            Some(_) => None,
        }?;
        if why != Redraw::Streamed {
            let (center, texel) = level.center_world(origin, camera);
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
// Each shelter level (SW-5e: near and far) has one `Rgba16Float` pack in its map's texels
// (`sf_surface_bevy` `exposure.rs` / `exposure.wgsl`): R and G the solid and leaf casters'
// heights over the map centre (copied from the two depth maps), B the *lag* (eased exposure minus
// the fresh one), A the fresh ground exposure (the layer's test for a receiver below every
// caster), the level's own value: the band where it hands over to the next level out (or to open
// sky past the far one, [`level_fade`]) blends live in the layer and the pass. The fragment keeps
// its exact 3D test on R and G and adds the lag: steady, the lag is 0 and the layer is unchanged;
// after a redraw the ground keeps what it showed (lag = shown - fresh, [`exposure_texel_in`]) and
// the lag decays to 0 over ~2.4 s. The lag is the ground's: a receiver on its texel's top surface
// (a roof, a leaf, a static floor) keeps the exact test, only what lies under the top is eased
// (`layer.wesl` `SHELTER_LAG_TOP_M`). The pack is world-anchored: a redraw's pass reads last
// pack's texel at `texel + shift` (the centres' world-texel difference), so a world point keeps
// its eased value however the map moves; a texel the old map did not cover showed the next level
// out (open sky past the far level). Like persistent snow-accumulation maps (RDR2, Horizon) and
// stable shadow cascades: the map never pops with the camera, it fades.

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
/// ground exposure (ignored by [`ExposureMode::Ease`], which keeps last pack's). One level, no
/// hand-over: [`exposure_texel_in`] with no band and open sky outside.
pub fn exposure_texel(pass: ExposurePass, held: Option<(f32, f32)>, fresh: f32) -> (f32, f32) {
    exposure_texel_in(pass, held.map(|(lag, f)| (lag, f, 0.0)), fresh, 0.0, 1.0)
}

/// What a level's pack shows for the ground at a texel (SW-5e): its own fresh ground exposure
/// handed over to the next level out's (`outer`, open sky 1 past the last) over its band
/// ([`level_fade`] of the texel's edge: `fade`), plus the lag. The pack's A is the level's own
/// value, unfaded; the band blends live, so the outer level's own easing shows through it.
pub fn shown_exposure(lag: f32, fresh: f32, fade: f32, outer: f32) -> f32 {
    fresh + (outer - fresh) * fade + lag
}

/// [`exposure_texel`] for a level inside an outer one (SW-5e; GPU twin `exposure.wgsl` `main`):
/// `held` is last pack's `(lag, fresh, fade)` at the source texel (its band weight in the old
/// map), `fade` the texel's band weight in the new map, `outer` what the next level out shows
/// there now (1 past the last). A carried texel keeps exactly what it showed
/// ([`shown_exposure`]; the outer level's where the old map did not reach), whatever the band
/// moved: the lag is taken against the new map's shown value.
pub fn exposure_texel_in(pass: ExposurePass, held: Option<(f32, f32, f32)>, fresh: f32, fade: f32, outer: f32) -> (f32, f32) {
    match pass.mode {
        ExposureMode::Fresh => (0.0, fresh),
        ExposureMode::Carry(_) => {
            let shown = held.map_or(outer, |(lag, f, old_fade)| shown_exposure(lag, f, old_fade, outer));
            ((shown - shown_exposure(0.0, fresh, fade, outer)) * pass.decay, fresh)
        }
        ExposureMode::Ease => {
            let (lag, f, _) = held.unwrap_or((0.0, fresh, 0.0));
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

/// The atlas's full mip chain (SW-6: the raised cover reads the drift prefiltered to a world
/// width, `layer::prefilter_level`): each level the 2x2 box average of the one above (rounded),
/// [`NOISE_SIZE`]² down to 1x1, levels concatenated (mip 0 first, as the GPU image holds them).
/// Every texel stays inside the range of the four it averages, so each level's neighbour steps
/// stay within the atlas's 0..1. Level 0 is [`noise_atlas`] unchanged.
pub fn noise_atlas_mips() -> Vec<[u8; 4]> {
    let mut level = noise_atlas();
    let mut out = level.clone();
    let mut n = NOISE_SIZE as usize;
    while n > 1 {
        let h = n / 2;
        let mut next = Vec::with_capacity(h * h);
        for y in 0..h {
            for x in 0..h {
                let at = |dx: usize, dy: usize| level[(2 * y + dy) * n + 2 * x + dx];
                let texels = [at(0, 0), at(1, 0), at(0, 1), at(1, 1)];
                next.push(std::array::from_fn(|c| {
                    let sum: u16 = texels.iter().map(|t| u16::from(t[c])).sum();
                    u8::try_from((sum + 2) / 4).expect("the average of four bytes")
                }));
            }
        }
        out.extend_from_slice(&next);
        level = next;
        n = h;
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
