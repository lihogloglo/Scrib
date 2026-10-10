use std::hash::{Hash, Hasher};

use tes3::nif::{
    NiAlphaProperty, NiMaterialProperty, NiStencilProperty, NiStream, NiTexturingProperty, NiVertexColorProperty,
    NiZBufferProperty, SourceVertexMode, TextureMap, TextureSource,
};
use tes3::nif::{AlphaBlendFunction, DrawMode, NiKey};

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
            vertex_colors: VertexColorMode::AmbientDiffuse,
            double_sided: false,
            depth_test: true,
            depth_write: true,
        }
    }
}

/// The non-float fields of a [`Material`], for equality and hashing.
type FlagKey<'a> =
    (&'a Option<String>, &'a Option<String>, &'a Option<String>, &'a Option<String>, VertexColorMode, [bool; 3]);

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
            [self.double_sided, self.depth_test, self.depth_write],
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
        // Prefer alpha testing when it does real work: it needs no sorting
        // and keeps depth for foliage. Blending covers the rest (glass,
        // smoke, glows).
        material.alpha_mode = if alpha.alpha_testing() && alpha.test_ref > 0 {
            AlphaMode::Mask { cutoff: f32::from(alpha.test_ref) / 255.0 }
        } else if alpha.alpha_blending() {
            AlphaMode::Blend { additive: alpha.dst_blend_mode() == AlphaBlendFunction::One }
        } else {
            AlphaMode::Opaque
        };
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
}
