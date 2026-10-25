//! The fog every translucent applies to itself (render-order.md rule 1).
//!
//! The opaque fog is a composite over the depth buffer, which blended and
//! additive surfaces don't write; so each translucent material fetches the fog
//! owner's integrated froxel volume at its own screen position and distance
//! (UE Volumetric Fog, Frostbite): `translucent_fog.wesl`'s `froxel_fog_lookup`
//! then `froxel_fog_apply` (additive and multiplicative: transmittance only).
//! The fog owner (`sf_atmos_bevy`) writes [`TranslucentFog`]; a translucent
//! material binds it (texture, sampler, params) and keeps it current. With no
//! fog owner the resource holds a 1³ clear volume and `params.w == 0`.

use bevy_app::App;
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler, ImageSamplerDescriptor};
use bevy_math::Vec4;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// The integrated fog volume translucents sample (rgb in-scatter, pre-exposed;
/// a transmittance) and its lookup params (near, far, slices, 1 when written
/// this frame else 0: a reader must then skip it).
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct TranslucentFog {
    pub volume: Handle<Image>,
    pub params: Vec4,
}

/// `params` of a fogless view: the lookup returns clear.
pub const NO_FOG: Vec4 = Vec4::new(0.5, 1.0, 1.0, 0.0);

/// Inserts [`TranslucentFog`] (a clear 1³ volume) unless present, and loads the
/// `sf_render_order_bevy::translucent_fog` shader module.
pub(crate) fn init(app: &mut App) {
    if app.world().contains_resource::<bevy_asset::io::embedded::EmbeddedAssetRegistry>() {
        bevy_shader::load_shader_library!(app, "translucent_fog.wesl");
    }
    if app.world().contains_resource::<TranslucentFog>() {
        return;
    }
    let volume = app.world_mut().get_resource_mut::<Assets<Image>>().map(|mut images| images.add(clear_volume())).unwrap_or_default();
    app.insert_resource(TranslucentFog { volume, params: NO_FOG });
}

/// One texel of no fog: no in-scatter, transmittance 1 (`Rgba16Float`).
fn clear_volume() -> Image {
    // f16 1.0 = 0x3C00, little-endian.
    let mut image = Image::new_fill(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        TextureDimension::D3,
        &[0, 0, 0, 0, 0, 0, 0x00, 0x3C],
        TextureFormat::Rgba16Float,
        RenderAssetUsages::all(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::linear());
    image
}
