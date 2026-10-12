//! Render-world side: partial clipmap texture writes. Bevy re-uploads a
//! whole `Image` when the asset changes, so the toroidal strips go straight
//! to the GPU texture through the queue instead (a few KiB per strip).

use bevy_asset::AssetId;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_render::MainWorld;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect};
use bevy_render::renderer::RenderQueue;
use bevy_render::texture::GpuImage;

/// One rectangle of one array layer.
#[derive(Clone, Debug)]
pub struct TextureWrite {
    pub image: AssetId<Image>,
    pub layer: u32,
    pub origin: [u32; 2],
    pub size: [u32; 2],
    pub bytes_per_texel: u32,
    pub data: Vec<u8>,
}

/// Writes queued by the main world this frame.
#[derive(Resource, Default)]
pub struct ClipmapWrites(pub Vec<TextureWrite>);

/// Writes waiting in the render world (until their texture exists).
#[derive(Resource, Default)]
pub struct PendingWrites(pub Vec<TextureWrite>);

pub fn extract_writes(mut main: ResMut<MainWorld>, mut pending: ResMut<PendingWrites>) {
    if let Some(mut writes) = main.get_resource_mut::<ClipmapWrites>() {
        pending.0.append(&mut writes.0);
    }
}

pub fn write_clipmaps(
    mut pending: ResMut<PendingWrites>,
    images: Res<RenderAssets<GpuImage>>,
    queue: Res<RenderQueue>,
) {
    pending.0.retain(|w| {
        let Some(gpu) = images.get(w.image) else { return true };
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &gpu.texture,
                mip_level: 0,
                origin: Origin3d { x: w.origin[0], y: w.origin[1], z: w.layer },
                aspect: TextureAspect::All,
            },
            &w.data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w.size[0] * w.bytes_per_texel),
                rows_per_image: Some(w.size[1]),
            },
            Extent3d { width: w.size[0], height: w.size[1], depth_or_array_layers: 1 },
        );
        false
    });
}
