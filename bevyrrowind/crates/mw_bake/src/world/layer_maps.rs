//! Terrain layer companions (T5, `superfargoth/docs/specs/textures.md` §9; textures-adapter.md
//! §7-8): an `LTEX` texture's normal map and ARM come from the same pack companions statics use,
//! found by the cook's own lookups ([`normal_companion`], [`orm_companion`]) and read with the
//! profile of the companion's own VFS source. [`Convert`] turns each into `TerrainLayer`'s
//! canonical layout (normal +Y up; ARM = R occlusion, G roughness) in `LayerSource::decode`, so
//! `sf_terrain` names no pack.

use sf_terrain::bake::LayerFile;
use sf_texture::{ChannelMap, PackRole, Role, canonical, specular_to_orm};
use sf_vfs::{SourceIndex, Vfs, VfsPath};

use crate::cook::{normal_companion, orm_companion};
use crate::delight::stem;
use crate::texture_profiles::TextureSources;
use crate::textures::TEXTURE_ALTERNATES;

/// How one companion file becomes its canonical map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Convert {
    /// Canonical normal X, Y ← shipped (the rule's layout, then the profile's Y flip).
    Normal(ChannelMap),
    /// Canonical occlusion, roughness, metallic ← shipped (a packed ORM; the rule's layout).
    Orm(ChannelMap),
    /// A legacy specular map, laid out by its rule then converted ([`specular_to_orm`]:
    /// roughness from the exponent, no occlusion).
    Specular(ChannelMap),
    /// A terrain albedo + roughness file (`_diffusespec` in PBR packs): roughness from its
    /// alpha (after the rule's layout), no occlusion.
    AlbedoRoughness(ChannelMap),
}

impl Convert {
    /// The layer map this companion fills.
    pub const fn file(self) -> LayerFile {
        match self {
            Self::Normal(_) => LayerFile::Normal,
            Self::Orm(_) | Self::Specular(_) | Self::AlbedoRoughness(_) => LayerFile::Arm,
        }
    }

    /// Decoded RGBA8 texels into the canonical map, in place.
    ///
    /// # Panics
    /// If `rgba.len()` isn't a multiple of 4.
    pub fn apply(self, rgba: &mut [u8]) {
        match self {
            Self::Normal(map) | Self::Orm(map) => map.apply_rgba(rgba),
            Self::Specular(map) => {
                map.apply_rgba(rgba);
                specular_to_orm(rgba);
            }
            Self::AlbedoRoughness(map) => {
                map.apply_rgba(rgba);
                for t in rgba.as_chunks_mut::<4>().0 {
                    *t = [u8::MAX, t[3], 0, u8::MAX];
                }
            }
        }
    }
}

/// A layer's companions: each winning file and its conversion.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LayerCompanions {
    pub normal: Option<(String, Convert)>,
    pub arm: Option<(String, Convert)>,
}

/// The companions of layer texture `albedo` (a resolved VFS path). Normal: the cook's
/// [`normal_companion`] (`_nh` before `_n`, then `_nm`, then the suffixes only some profiles
/// have), Y converted by the companion source's profile. ARM: the cook's [`orm_companion`]
/// (`_spec`: a packed ORM or, for the `openmw` profile, a specular map), else a terrain albedo +
/// roughness rule of the companion's source profile. `resolve`: a `.tga` reference to its
/// winning file and VFS source ([`vfs_resolver`]).
pub fn companions(
    sources: &TextureSources,
    albedo: &str,
    resolve: impl Fn(&str) -> Option<(String, SourceIndex)>,
) -> LayerCompanions {
    let normal = normal_companion(sources, albedo, &resolve).map(|(file, profile, rule)| {
        let (to_canonical, _) = canonical(Role::Normal { source_y: profile.normal_y });
        (file, Convert::Normal(rule.layout.then(to_canonical)))
    });
    let orm = orm_companion(sources, albedo, &resolve).map(|(file, _, rule)| {
        let convert =
            if rule.role == PackRole::SpecularShininess { Convert::Specular(rule.layout) } else { Convert::Orm(rule.layout) };
        (file, convert)
    });
    let arm = orm.or_else(|| {
        let base = stem(albedo);
        sources.terrain_roughness_suffixes().into_iter().find_map(|suffix| {
            let (file, source) = resolve(&format!("{base}{suffix}.tga"))?;
            sources.terrain_roughness_rule(source, suffix).map(|(_, rule)| (file, Convert::AlbedoRoughness(rule.layout)))
        })
    });
    LayerCompanions { normal, arm }
}

/// The runtime's texture rule over `vfs` ([`TEXTURE_ALTERNATES`]): a reference to its winning
/// file and VFS source, as the texture cook resolves companions.
pub fn vfs_resolver(vfs: &Vfs) -> impl Fn(&str) -> Option<(String, SourceIndex)> + '_ {
    move |r: &str| {
        let path = VfsPath::new(r).ok()?;
        vfs.resolve_preferring(&path, TEXTURE_ALTERNATES).map(|(p, s)| (p.as_str().to_owned(), s))
    }
}

#[cfg(test)]
mod tests;
