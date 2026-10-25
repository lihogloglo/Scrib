//! The per-frame parameter buffer read by the terrain shaders: a flat
//! array of `vec4<u32>` slots (f32/i32 bit-cast), so the layout can't drift
//! between Rust and WGSL padding rules. Slot indices match `common.wesl`.

use sf_terrain::cdlod::{CdlodSettings, MIN_LOD, NodeKey};

pub const P_CAMERA: usize = 0;
pub const P_ORIGIN: usize = 1;
pub const P_MISC: usize = 2;
pub const P_VALID: usize = 3;
pub const P_MORPH: usize = 4;
pub const P_HEIGHT: usize = 16;
pub const P_LAYER: usize = 26;
pub const P_MACRO: usize = 29;
pub const P_DETAIL: usize = 39;
pub const P_LAYERS: usize = 40;
/// Most layers the buffer holds (texture arrays allow 256 slices).
pub const MAX_LAYERS: usize = 256;
/// The far shadow map: (min x, min z, 1 / extent, base height), render
/// space; extent 0 = none.
pub const P_FAR: usize = P_LAYERS + 2 * MAX_LAYERS;
pub const SLOTS: usize = P_FAR + 1;

pub const HEIGHT_LEVELS: usize = 10;
pub const LAYER_LEVELS: usize = 3;
pub const MACRO_LEVELS: usize = 10;
pub const MORPH_LODS: usize = 12;

pub const DEBUG_LOD: u32 = 1;
pub const DEBUG_WIREFRAME: u32 = 2;
pub const DEBUG_NO_FAR_SHADOWS: u32 = 4;

/// Builder over the slot array.
pub struct Params(pub Vec<[u32; 4]>);

impl Default for Params {
    fn default() -> Self {
        Self(vec![[0; 4]; SLOTS])
    }
}

fn f(v: f32) -> u32 {
    v.to_bits()
}

#[allow(clippy::cast_sign_loss)]
fn i(v: i32) -> u32 {
    v as u32
}

impl Params {
    pub fn set_f(&mut self, slot: usize, v: [f32; 4]) {
        self.0[slot] = v.map(f);
    }

    pub fn set_i(&mut self, slot: usize, v: [i32; 4]) {
        self.0[slot] = v.map(i);
    }

    /// Morph zones per LOD from `MIN_LOD`: (start, 1 / length, spacing).
    pub fn set_morph(&mut self, cdlod: &CdlodSettings) {
        for n in 0..MORPH_LODS {
            let lod = MIN_LOD + i8::try_from(n).expect("small");
            let (start, end) = cdlod.morph_zone(lod);
            self.set_f(P_MORPH + n, [start, 1.0 / (end - start).max(1e-3), NodeKey::spacing(lod), 0.0]);
        }
    }

    pub fn as_slice(&self) -> &[[u32; 4]] {
        &self.0
    }
}
