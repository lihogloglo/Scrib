//! Fog for transparent surfaces (weather.md "Fog", F3 step 4): the froxels'
//! integrated volume as a shared [`Image`] ([`FroxelVolume`]) that blended
//! materials sample in their fragment shader, plus a ready-made
//! [`FroxelFogMaterial`] (`StandardMaterial` + the fog) for hosts that draw
//! glass, particles or water with Bevy's PBR.
//!
//! Transparents can't be fogged by the composite: it runs on the depth buffer
//! before the transparent pass, and blended surfaces don't write depth. The
//! industry answer (UE Volumetric Fog, Frostbite, Decima) is that each
//! translucent material fetches the integrated froxel volume at its own
//! screen position and distance: `froxel_apply.wesl`'s
//! `froxel_fog_apply`. Past the froxel range the volume's last slice is used
//! (the analytic far layers are not applied to transparents; a named limit,
//! W2's water draws its own far fog).

use bevy_asset::{Asset, Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler, ImageSamplerDescriptor};
use bevy_math::Vec4;
use bevy_pbr::{ExtendedMaterial, MaterialExtension, StandardMaterial};
use bevy_reflect::TypePath;
use bevy_render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_shader::ShaderRef;

use crate::Fog;

const SHADER: &str = "embedded://sf_atmos_bevy/shaders/transparent_fog.wesl";

/// The froxels' integrated volume (rgb: in-scattered light, pre-exposed; a:
/// transmittance; texel `s` holds the integral to slice `s`'s far boundary)
/// and how to look it up. Written by the fog pass every frame for the
/// exterior camera, before the transparent pass.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct FroxelVolume {
    pub image: Handle<Image>,
    /// (near, far, slices, 1 when the froxels run else 0): `sf_atmos::FroxelSettings`.
    pub params: Vec4,
    /// The image's size (tiles across, down, slices).
    pub size: [u32; 3],
}

/// The fog binding of [`FroxelFogMaterial`] (bindings 110-112 of the material group).
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
pub struct FroxelFogExt {
    /// [`FroxelVolume::params`]; kept current by the plugin.
    #[uniform(110)]
    pub params: Vec4,
    #[texture(111, dimension = "3d")]
    #[sampler(112)]
    pub volume: Handle<Image>,
}

impl FroxelFogExt {
    /// The extension for the current volume.
    #[must_use]
    pub fn new(volume: &FroxelVolume) -> Self {
        Self { params: volume.params, volume: volume.image.clone() }
    }
}

impl MaterialExtension for FroxelFogExt {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
}

/// Bevy's `StandardMaterial` fogged by the froxels: use it for blended
/// surfaces (`AlphaMode::Blend`, `Premultiplied`, `Add`, `Multiply`) in fog.
pub type FroxelFogMaterial = ExtendedMaterial<StandardMaterial, FroxelFogExt>;

/// A 3D storage image the froxel integrate pass writes into.
fn volume_image(size: [u32; 3]) -> Image {
    let mut image = Image::new_uninit(
        Extent3d { width: size[0], height: size[1], depth_or_array_layers: size[2] },
        TextureDimension::D3,
        TextureFormat::Rgba16Float,
        // Kept in the main world too (no data: cheap), so the handle stays valid.
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage = TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    // Trilinear, clamped: one fetch blends neighbouring tiles and slices.
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    image
}

/// Keeps [`FroxelVolume`] matching the fog's froxel settings (a new image when
/// the grid changes) and every [`FroxelFogMaterial`] pointing at it.
pub(crate) fn update_volume(
    fog: Res<Fog>,
    mut volume: ResMut<FroxelVolume>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<FroxelFogMaterial>>,
    mut applied: Local<Option<FroxelVolume>>,
) {
    let settings = fog.froxels.filter(|_| fog.enabled);
    #[allow(clippy::cast_precision_loss)]
    let wanted = settings.map(|f| (f.size, Vec4::new(f.near, f.far, f.size[2] as f32, 1.0)));
    let (size, params) = wanted.unwrap_or(([1, 1, 1], Vec4::new(0.5, 1.0, 1.0, 0.0)));
    if volume.size != size || !images.contains(&volume.image) {
        volume.image = images.add(volume_image(size));
        volume.size = size;
    }
    if volume.params != params {
        volume.params = params;
    }
    if applied.as_ref() == Some(&*volume) {
        return;
    }
    *applied = Some(volume.clone());
    let ids: Vec<_> = materials.ids().collect();
    for id in ids {
        if let Some(mut m) = materials.get_mut(id) {
            m.extension = FroxelFogExt::new(&volume);
        }
    }
}

/// New materials made before the volume existed (or with a default
/// extension) get the current one.
pub(crate) fn fix_new_materials(volume: Res<FroxelVolume>, mut events: MessageReader<bevy_asset::AssetEvent<FroxelFogMaterial>>, mut materials: ResMut<Assets<FroxelFogMaterial>>) {
    for event in events.read() {
        let bevy_asset::AssetEvent::Added { id } = event else { continue };
        let stale = materials.get(*id).is_some_and(|m| m.extension.volume != volume.image || m.extension.params != volume.params);
        if stale && let Some(mut m) = materials.get_mut(*id) {
            m.extension = FroxelFogExt::new(&volume);
        }
    }
}
