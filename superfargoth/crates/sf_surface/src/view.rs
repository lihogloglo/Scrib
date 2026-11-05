//! Presentation helpers for the GPU side (surface-weather.md "GPU resources", W2), Bevy-free so
//! they test in seconds: the toroidal cell window and its texels, the vertical shelter view and
//! its redraw triggers, the noise atlas, and the debug views. Nothing here feeds gameplay
//! (design rule 2): `sf_surface_bevy` uploads it.

use glam::camera::rh::{proj::directx::orthographic, view::look_to_mat4};
use glam::{IVec2, Mat4, UVec2, Vec3};
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
    streamed: bool,
    frames: u32,
}

impl ShelterTrigger {
    /// The centre of the last redraw.
    pub fn center(&self) -> Option<Vec3> {
        self.center
    }

    /// The map is off: the next frame redraws fresh.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// One frame with the camera at `camera` (render space); `streamed`: shelter casters were
    /// added this frame. Returns the redraw (and moves the centre) when one is due.
    pub fn frame(&mut self, camera: Vec3, streamed: bool) -> Option<Redraw> {
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
            self.center = Some(shelter_center(camera));
        }
        self.streamed = false;
        self.frames = 0;
        Some(why)
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
