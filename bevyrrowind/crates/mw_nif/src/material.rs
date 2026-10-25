use std::hash::{Hash, Hasher};

use tes3::nif::{
    NiAlphaProperty, NiMaterialProperty, NiStencilProperty, NiStream, NiTexturingProperty, NiVertexColorProperty,
    NiZBufferProperty, SourceVertexMode, TextureMap, TextureSource,
};
use tes3::nif::{AlphaBlendFunction, AlphaTestFunction, DrawMode, NiKey};

use crate::Inherited;

/// Texture slots in `NiTexturingProperty::texture_maps`.
const BASE: usize = 0;
const DARK: usize = 1;
const DETAIL: usize = 2;
const GLOW: usize = 4;

/// How a surface handles transparency.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlphaMode {
    Opaque,
    /// Alpha test: discard below `cutoff` (0..=1).
    Mask { cutoff: f32 },
    /// Alpha blending; `additive` when the destination factor is one.
    Blend { additive: bool },
}

/// What vertex colors mean for this surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VertexColorMode {
    Ignore,
    /// Vertex colors are emitted light.
    Emissive,
    /// Vertex colors tint diffuse/ambient (the NIF default).
    AmbientDiffuse,
}

/// A surface description resolved from the inherited NIF properties.
///
/// Colors are linear RGB in 0..=1 as stored. Texture paths are normalized
/// VFS paths (see [`texture_path`]); resolving the actual file (e.g. `.dds`
/// replacing `.tga`) is the asset layer's job.
#[derive(Clone, Debug)]
pub struct Material {
    pub base_texture: Option<String>,
    pub dark_texture: Option<String>,
    pub detail_texture: Option<String>,
    pub glow_texture: Option<String>,
    pub diffuse: [f32; 3],
    pub ambient: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub glossiness: f32,
    pub alpha: f32,
    pub alpha_mode: AlphaMode,
    /// Whether `alpha_mode` is (or may become) a cutout drawn from an authored blend.
    pub cutout: Cutout,
    pub vertex_colors: VertexColorMode,
    pub double_sided: bool,
    pub depth_test: bool,
    pub depth_write: bool,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_texture: None,
            dark_texture: None,
            detail_texture: None,
            glow_texture: None,
            diffuse: [1.0; 3],
            ambient: [1.0; 3],
            specular: [0.0; 3],
            emissive: [0.0; 3],
            glossiness: 0.0,
            alpha: 1.0,
            alpha_mode: AlphaMode::Opaque,
            cutout: Cutout::No,
            vertex_colors: VertexColorMode::AmbientDiffuse,
            double_sided: false,
            depth_test: true,
            depth_write: true,
        }
    }
}

/// The non-float fields of a [`Material`], for equality and hashing.
type FlagKey<'a> =
    (&'a Option<String>, &'a Option<String>, &'a Option<String>, &'a Option<String>, VertexColorMode, [bool; 3], Cutout);

impl Material {
    /// Every float as raw bits, for exact equality and hashing.
    fn float_bits(&self) -> impl Iterator<Item = u32> + '_ {
        let alpha_mode = match self.alpha_mode {
            AlphaMode::Opaque => [0, 0],
            AlphaMode::Mask { cutoff } => [1, cutoff.to_bits()],
            AlphaMode::Blend { additive } => [2, u32::from(additive)],
        };
        self.diffuse
            .iter()
            .chain(&self.ambient)
            .chain(&self.specular)
            .chain(&self.emissive)
            .chain([&self.glossiness, &self.alpha])
            .map(|f| f.to_bits())
            .chain(alpha_mode)
    }

    fn flags(&self) -> FlagKey<'_> {
        (
            &self.base_texture,
            &self.dark_texture,
            &self.detail_texture,
            &self.glow_texture,
            self.vertex_colors,
            [self.double_sided, self.depth_test, self.depth_write], self.cutout,
        )
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.flags() == other.flags() && self.float_bits().eq(other.float_bits())
    }
}

impl Eq for Material {}

impl Hash for Material {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.flags().hash(state);
        for bits in self.float_bits() {
            bits.hash(state);
        }
    }
}

/// Alpha-test cutoff for authored blend + "alpha > 0" surfaces drawn as
/// cutouts. 0.5 keeps a card's covered area equal to its blended average
/// for symmetric edge ramps (the usual masked-foliage cutoff, glTF's
/// default); lower thickens the foliage, higher thins it.
pub const CUTOUT_CUTOFF: f32 = 0.5;

/// How an authored blend relates to alpha testing (see [`AuthoredAlpha::resolve`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Cutout {
    /// Drawn as authored.
    #[default]
    No,
    /// Blend + "alpha > 0" test, promoted to [`AlphaMode::Mask`] at [`CUTOUT_CUTOFF`].
    Promoted,
    /// Plain blend of a textured, unfaded surface: Morrowind's foliage (`0xED`:
    /// src-alpha / inv-src-alpha, no test) and glass, potions alike. The flags
    /// can't tell them apart; the texture's alpha can (holes vs. soft), so
    /// the consumer decides and draws cutouts as a Mask at [`CUTOUT_CUTOFF`].
    Candidate,
}

/// The alpha state an `NiAlphaProperty` authors.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AuthoredAlpha {
    pub blending: bool,
    pub additive: bool,
    pub testing: bool,
    pub test: AlphaTestFunction,
    pub test_ref: u8,
}

impl AuthoredAlpha {
    /// How to draw it, and whether that is a cutout promoted from a blend
    /// (see [`Cutout`]).
    ///
    /// Alpha testing that does real work (`ref > 0`) wins: no sorting, and
    /// it keeps depth. Morrowind's foliage, hair and banners author blend +
    /// test `GREATER 0` (the test only skips empty texels): drawn blended
    /// they write no depth, so they miss the deferred fog, shadows and
    /// motion vectors (TAA/DLSS smear) and sort per mesh. Modern engines
    /// draw such cards alpha-tested (UE "Masked", Unity "Cutout"), so they
    /// become a [`AlphaMode::Mask`] at [`CUTOUT_CUTOFF`]. True translucents
    /// stay blended: additive (glows, Ghostfence), blends without the test
    /// (glass, smoke), untextured or faded surfaces (`material_alpha` < 1:
    /// ghosts). Animated fades and particles are demoted by the caller,
    /// which knows the controllers.
    pub fn resolve(self, material_alpha: f32, textured: bool) -> (AlphaMode, Cutout) {
        let passes_above = matches!(self.test, AlphaTestFunction::Greater | AlphaTestFunction::GreaterEqual);
        if self.testing && self.test_ref > 0 {
            (AlphaMode::Mask { cutoff: f32::from(self.test_ref) / 255.0 }, Cutout::No)
        } else if !self.blending {
            (AlphaMode::Opaque, Cutout::No)
        } else if self.additive || !textured || material_alpha < 1.0 || (self.testing && !passes_above) {
            (AlphaMode::Blend { additive: self.additive }, Cutout::No)
        } else if self.testing {
            (AlphaMode::Mask { cutoff: CUTOUT_CUTOFF }, Cutout::Promoted)
        } else {
            (AlphaMode::Blend { additive: false }, Cutout::Candidate)
        }
    }
}

impl Material {
    /// Undoes a cutout promotion: back to the authored blend.
    pub fn restore_blend(&mut self) {
        if self.cutout != Cutout::No {
            self.alpha_mode = AlphaMode::Blend { additive: false };
            self.cutout = Cutout::No;
        }
    }
}

/// Normalizes a NIF texture reference to a VFS path under `textures/`.
///
/// NIFs store bare names (`tx_wood.tga`), sometimes with a `textures\`
/// prefix or odd casing.
pub fn texture_path(raw: &str) -> String {
    let normalized = raw.trim().replace('\\', "/").to_ascii_lowercase();
    let normalized = normalized.trim_start_matches('/');
    if normalized.starts_with("textures/") { normalized.to_owned() } else { format!("textures/{normalized}") }
}

fn get<'a, T>(stream: &'a NiStream, key: Option<NiKey>) -> Option<&'a T>
where
    &'a T: TryFrom<&'a tes3::nif::NiType>,
{
    stream.objects.get(key?).and_then(|o| <&T>::try_from(o).ok())
}

fn slot_texture(stream: &NiStream, texturing: &NiTexturingProperty, slot: usize) -> Option<String> {
    let map = match texturing.texture_maps.get(slot)?.as_ref()? {
        TextureMap::Map(map) => map,
        TextureMap::BumpMap(bump) => &bump.base,
    };
    let source = stream.get(map.texture)?;
    match &source.source {
        TextureSource::External(name) if !name.trim().is_empty() => Some(texture_path(name)),
        // Embedded pixel data is rare in Morrowind content; unsupported for now.
        _ => None,
    }
}

pub(crate) fn from_properties(stream: &NiStream, inherited: &Inherited) -> Material {
    let mut material = Material::default();

    if let Some(texturing) = get::<NiTexturingProperty>(stream, inherited.texturing) {
        material.base_texture = slot_texture(stream, texturing, BASE);
        material.dark_texture = slot_texture(stream, texturing, DARK);
        material.detail_texture = slot_texture(stream, texturing, DETAIL);
        material.glow_texture = slot_texture(stream, texturing, GLOW);
    }

    if let Some(props) = get::<NiMaterialProperty>(stream, inherited.material) {
        material.diffuse = props.diffuse_color.to_array();
        material.ambient = props.ambient_color.to_array();
        material.specular = props.specular_color.to_array();
        material.emissive = props.emissive_color.to_array();
        material.glossiness = props.shine;
        material.alpha = props.alpha;
    }

    if let Some(alpha) = get::<NiAlphaProperty>(stream, inherited.alpha) {
        let authored = AuthoredAlpha {
            blending: alpha.alpha_blending(),
            additive: alpha.dst_blend_mode() == AlphaBlendFunction::One,
            testing: alpha.alpha_testing(),
            test: alpha.test_mode(),
            test_ref: alpha.test_ref,
        };
        (material.alpha_mode, material.cutout) = authored.resolve(material.alpha, material.base_texture.is_some());
    }

    if let Some(vertex) = get::<NiVertexColorProperty>(stream, inherited.vertex_color) {
        material.vertex_colors = match vertex.source_vertex_mode {
            SourceVertexMode::Ignore => VertexColorMode::Ignore,
            SourceVertexMode::Emissive => VertexColorMode::Emissive,
            SourceVertexMode::AmbientDiffuse => VertexColorMode::AmbientDiffuse,
        };
    }

    if let Some(stencil) = get::<NiStencilProperty>(stream, inherited.stencil) {
        material.double_sided = stencil.draw_mode == DrawMode::Both;
    }

    if let Some(zbuffer) = get::<NiZBufferProperty>(stream, inherited.zbuffer) {
        material.depth_test = zbuffer.z_buffer_test();
        material.depth_write = zbuffer.z_buffer_write();
    }

    material
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_paths_normalize() {
        assert_eq!(texture_path("Tx_Wood.TGA"), "textures/tx_wood.tga");
        assert_eq!(texture_path("textures\\a\\b.dds"), "textures/a/b.dds");
        assert_eq!(texture_path(" \\Textures\\x.bmp"), "textures/x.bmp");
    }

    #[test]
    fn equal_materials_hash_equal() {
        use std::collections::HashSet;
        let a = Material { base_texture: Some("textures/a.dds".into()), ..Material::default() };
        let b = a.clone();
        let c = Material { alpha_mode: AlphaMode::Mask { cutoff: 0.5 }, ..a.clone() };
        let set: HashSet<Material> = [a, b, c].into_iter().collect();
        assert_eq!(set.len(), 2);
    }

    fn authored(blending: bool, additive: bool, testing: bool, test: AlphaTestFunction, test_ref: u8) -> AuthoredAlpha {
        AuthoredAlpha { blending, additive, testing, test, test_ref }
    }

    #[test]
    fn foliage_blend_with_zero_test_becomes_cutout() {
        let leaves = authored(true, false, true, AlphaTestFunction::Greater, 0);
        assert_eq!(leaves.resolve(1.0, true), (AlphaMode::Mask { cutoff: CUTOUT_CUTOFF }, Cutout::Promoted));
    }

    #[test]
    fn real_alpha_test_keeps_its_ref() {
        let tested = authored(true, false, true, AlphaTestFunction::Greater, 128);
        assert_eq!(tested.resolve(1.0, true), (AlphaMode::Mask { cutoff: 128.0 / 255.0 }, Cutout::No));
    }

    #[test]
    fn true_translucents_stay_blended() {
        let blend = (AlphaMode::Blend { additive: false }, Cutout::No);
        let glow = authored(true, true, true, AlphaTestFunction::Greater, 0);
        assert_eq!(glow.resolve(1.0, true), (AlphaMode::Blend { additive: true }, Cutout::No));
        let glass = authored(true, false, false, AlphaTestFunction::Greater, 0);
        // Plain blend (foliage `0xED`, glass): the texture decides.
        assert_eq!(glass.resolve(1.0, true), (AlphaMode::Blend { additive: false }, Cutout::Candidate));
        assert_eq!(glass.resolve(0.5, true), blend);
        // Faded (ghosts) or untextured: the alpha is the material's, not holes.
        let leaves = authored(true, false, true, AlphaTestFunction::Greater, 0);
        assert_eq!(leaves.resolve(0.5, true), blend);
        assert_eq!(leaves.resolve(1.0, false), blend);
        let always = authored(true, false, true, AlphaTestFunction::Always, 0);
        assert_eq!(always.resolve(1.0, true), blend);
        let opaque = authored(false, false, false, AlphaTestFunction::Always, 0);
        assert_eq!(opaque.resolve(1.0, true), (AlphaMode::Opaque, Cutout::No));
    }

    #[test]
    fn restore_blend_undoes_only_cutouts() {
        let mut leaves =
            Material { alpha_mode: AlphaMode::Mask { cutoff: CUTOUT_CUTOFF }, cutout: Cutout::Promoted, ..Material::default() };
        leaves.restore_blend();
        assert_eq!(leaves.alpha_mode, AlphaMode::Blend { additive: false });
        let mut tested = Material { alpha_mode: AlphaMode::Mask { cutoff: 0.5 }, ..Material::default() };
        tested.restore_blend();
        assert_eq!(tested.alpha_mode, AlphaMode::Mask { cutoff: 0.5 });
    }
}
