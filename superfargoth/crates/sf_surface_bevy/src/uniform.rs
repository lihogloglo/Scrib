//! `SurfaceUniform`: the per-view surface weather parameters, uploaded as a flat array of
//! `vec4<u32>` slots (f32/i32 bit-cast, as the terrain's params), so Rust and WGSL can't drift on
//! padding. Slot order matches `struct SurfaceUniform` in `layer.wesl`.

use bevy_ecs::prelude::*;
use bevy_math::{IVec4, Mat4, Vec4};
use sf_surface::view::DebugView;

/// Slots: the shelter matrix (4 columns), window, cells, shelter, misc, time, canopy, cover (snow,
/// ash), cover UV at the origin, the trail ring's placement (W6a).
pub const U_SHELTER_CLIP: usize = 0;
pub const U_WINDOW: usize = 4;
pub const U_CELLS: usize = 5;
pub const U_SHELTER: usize = 6;
pub const U_MISC: usize = 7;
pub const U_TIME: usize = 8;
pub const U_CANOPY: usize = 9;
pub const U_COVER: usize = 10;
pub const U_COVER_UV: usize = 12;
pub const U_TRAIL: usize = 13;
pub const U_TRAIL_CELLS: usize = 14;
pub const U_SLOTS: usize = 15;

/// Layer flags (`misc.x`): which channels are non-zero anywhere in the cell window.
pub const FLAG_WET: u32 = 1;
pub const FLAG_PUDDLE: u32 = 2;
pub const FLAG_SNOW: u32 = 4;
pub const FLAG_ASH: u32 = 8;

/// The CPU mirror of the uniform (main world), written by the plugin's systems and uploaded
/// when it changes.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SurfaceUniform {
    /// Render space → shelter clip (reverse Z: 1 at the top plane).
    pub shelter_clip_from_world: Mat4,
    /// xy: render-space X, Z of the window's first cell's min corner; z: 1 / cell (1/m); w: 1
    /// when placed.
    pub window: Vec4,
    /// xy: the first cell's texel (cell mod 128); z: window edge (cells).
    pub cells: IVec4,
    /// x: 1 when the shelter map is valid; y: depth range (m); z: texel (m); w: half extent (m).
    pub shelter: Vec4,
    /// `FLAG_*`.
    pub flags: u32,
    pub debug: DebugView,
    /// x: animation seconds (wrapped at `sf_surface::layer::RIPPLE_TIME_WRAP_S`, still while no
    /// rings show), y: rain at the camera (0..1; puddle rings, W4).
    pub time: Vec4,
    /// x: 1 when the canopy map is valid; y: share of the rain a full leaf canopy lets through.
    pub canopy: Vec4,
    /// Per cover channel (snow, ash; `crate::cover::cover_slots`): x 1 / tiling (1/m, 0 = no
    /// layer), y depth (m), z sparkle, w contrast.
    pub cover: [Vec4; 2],
    /// The cover's world-aligned UV at the render origin (W5b; `crate::cover::cover_uv`): xy snow,
    /// zw ash.
    pub cover_uv: Vec4,
    /// The trail ring (W6a; `crate::trail`): xy render X, Z of its first texel's min corner, z 1 /
    /// texel (1/m), w its depth (m; 0 = trails off: the shading ignores the ring).
    pub trail: Vec4,
    /// xy: the first texel's address (world texel mod the ring edge); z: the ring edge (texels).
    pub trail_cells: IVec4,
}

impl Default for SurfaceUniform {
    fn default() -> Self {
        Self {
            shelter_clip_from_world: Mat4::IDENTITY,
            window: Vec4::ZERO,
            cells: IVec4::ZERO,
            shelter: Vec4::ZERO,
            flags: 0,
            debug: DebugView::Off,
            time: Vec4::ZERO,
            canopy: Vec4::ZERO,
            cover: [Vec4::ZERO; 2],
            cover_uv: Vec4::ZERO,
            trail: Vec4::ZERO,
            trail_cells: IVec4::ZERO,
        }
    }
}

impl SurfaceUniform {
    /// Layer flags from the window's per-channel maxima (R wet, G puddle, B snow, A ash).
    pub fn flags_of(max: [u8; 4]) -> u32 {
        [FLAG_WET, FLAG_PUDDLE, FLAG_SNOW, FLAG_ASH]
            .iter()
            .zip(max)
            .filter(|(_, m)| *m > 0)
            .fold(0, |f, (b, _)| f | b)
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn slots(&self) -> [[u32; 4]; U_SLOTS] {
        let f = |v: Vec4| v.to_array().map(f32::to_bits);
        let m = self.shelter_clip_from_world;
        let mut s = [[0; 4]; U_SLOTS];
        for c in 0..4 {
            s[U_SHELTER_CLIP + c] = f(m.col(c));
        }
        s[U_WINDOW] = f(self.window);
        s[U_CELLS] = self.cells.to_array().map(|v| v as u32);
        s[U_SHELTER] = f(self.shelter);
        s[U_MISC] = [self.flags, self.debug as u32, 0, 0];
        s[U_TIME] = f(self.time);
        s[U_CANOPY] = f(self.canopy);
        s[U_COVER] = f(self.cover[0]);
        s[U_COVER + 1] = f(self.cover[1]);
        s[U_COVER_UV] = f(self.cover_uv);
        s[U_TRAIL] = f(self.trail);
        s[U_TRAIL_CELLS] = self.trail_cells.to_array().map(|v| v as u32);
        s
    }
}
