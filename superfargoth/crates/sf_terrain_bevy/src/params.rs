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
/// RO-16 veil diagnosis (scaffold: remove with the veil fix): no specular on the terrain
/// (reflectance 0, so sun, lamp, env-map and flat-ambient specular all drop).
pub const DEBUG_NO_SPECULAR: u32 = 8;
/// RO-16: no indirect light on the terrain (diffuse and specular occlusion 0, so the env map,
/// the flat ambient and the GI probes drop; sun, moon and lamps stay).
pub const DEBUG_NO_INDIRECT: u32 = 16;
/// RO-16: the raw splat layer albedo (no land-colour tint from the macro map, no macro variation).
pub const DEBUG_RAW_LAYERS: u32 = 32;
/// RO-16: no env-map specular on the terrain alone (specular occlusion 0; diffuse, sun, lamp and
/// flat-ambient specular stay).
pub const DEBUG_NO_ENV_SPECULAR: u32 = 64;

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

/// The shader's morph zone of `lod`: [`CdlodSettings::morph_zone`] moved `slack_m` closer to the camera
/// (at most down to the band's inner edge). A selection gate (CW-2) that keeps a selection while the camera
/// moves less than `slack_m` stays crack-free with it: a finer node's edge against a coarser one lies at
/// least `range(lod) - slack_m` from the camera, where the morph has finished (Strugar's condition, with
/// the staleness folded into the zone).
pub fn morph_zone(cdlod: &CdlodSettings, lod: i8, slack_m: f32) -> (f32, f32) {
    let (start, end) = cdlod.morph_zone(lod);
    let slack = slack_m.clamp(0.0, (start - cdlod.range(lod) * 0.5).max(0.0));
    (start - slack, end - slack)
}

impl Params {
    pub fn set_f(&mut self, slot: usize, v: [f32; 4]) {
        self.0[slot] = v.map(f);
    }

    pub fn set_i(&mut self, slot: usize, v: [i32; 4]) {
        self.0[slot] = v.map(i);
    }

    /// Morph zones per LOD from `MIN_LOD`: (start, 1 / length, spacing), each moved `slack_m` closer
    /// ([`morph_zone`]).
    pub fn set_morph(&mut self, cdlod: &CdlodSettings, slack_m: f32) {
        for n in 0..MORPH_LODS {
            let lod = MIN_LOD + i8::try_from(n).expect("small");
            let (start, end) = morph_zone(cdlod, lod, slack_m);
            self.set_f(P_MORPH + n, [start, 1.0 / (end - start).max(1e-3), NodeKey::spacing(lod), 0.0]);
        }
    }

    pub fn as_slice(&self) -> &[[u32; 4]] {
        &self.0
    }
}

impl Params {
    /// Layer `n`'s two slots (`common.wesl` `layer_tiling` .. `layer_top`): (tiling, displacement,
    /// triplanar, p90 height) and (mean albedo, mean height).
    pub fn set_layer(&mut self, n: usize, l: &sf_terrain::LayerInfo) {
        self.0[P_LAYERS + 2 * n] = [f(l.tiling_m), f(l.displacement_m), u32::from(l.triplanar), f(l.p90_height)];
        self.set_f(P_LAYERS + 2 * n + 1, [l.mean_albedo[0], l.mean_albedo[1], l.mean_albedo[2], l.mean_height]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// W6b: a layer's slots carry its p90 height (the bury raise's high point) beside its mean.
    #[test]
    fn layer_slots_carry_mean_and_p90_height() {
        let l = sf_terrain::LayerInfo {
            record: "l".into(),
            id: sf_terrain::layer_id("l"),
            tiling_m: 4.0,
            triplanar: true,
            displacement_m: 0.05,
            mean_albedo: [0.1, 0.2, 0.3],
            mean_height: 0.45,
            p90_height: 0.8,
            source_edge: 512,
        };
        let mut p = Params::default();
        p.set_layer(3, &l);
        assert_eq!(p.0[P_LAYERS + 6], [4.0f32.to_bits(), 0.05f32.to_bits(), 1, 0.8f32.to_bits()]);
        assert_eq!(p.0[P_LAYERS + 7], [0.1f32, 0.2, 0.3, 0.45].map(f32::to_bits));
    }
}
