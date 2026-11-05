//! Render world: the uniform buffer, the samplers, and the view component of the vendored
//! `bevy_pbr` patch "SW" (`bevy_pbr::SurfaceWeatherView`, mesh-view bindings 43-48). Each frame
//! the cameras marked [`SurfaceWeatherCamera`] get the component on their render-world view
//! (once the textures are on the GPU); views that lost the mark lose it, and with it the
//! `SURFACE_WEATHER` shader def (interiors pay nothing). Same shape as `sf_gi_bevy`'s
//! `GiProbeVolumeView`.

use bevy_app::SubApp;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_pbr::{SurfaceWeatherBindings, SurfaceWeatherView};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{
    AddressMode, Buffer, BufferDescriptor, BufferUsages, FilterMode, Sampler, SamplerDescriptor,
};
use bevy_render::renderer::{RenderDevice, RenderQueue};
use bevy_render::sync_world::RenderEntity;
use bevy_render::texture::GpuImage;
use bevy_render::{Extract, Render, RenderSystems};

use crate::{SurfaceGpu, SurfaceUniform, SurfaceWeatherCamera, U_SLOTS, cells};

pub(crate) fn build(render_app: &mut SubApp) {
    render_app.add_systems(
        Render,
        cells::write_cells.in_set(RenderSystems::PrepareResources),
    );
}

/// The render-world side: made on first use.
pub(crate) struct ViewState {
    uniform: Buffer,
    sampler: Sampler,
    shelter_sampler: Sampler,
    written: Option<[[u32; 4]; U_SLOTS]>,
    marked: Vec<Entity>,
}

impl ViewState {
    fn new(device: &RenderDevice) -> Self {
        Self {
            uniform: device.create_buffer(&BufferDescriptor {
                label: Some("surface_weather_uniform"),
                size: (U_SLOTS * 16) as u64,
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            // Cells and noise: filtered, repeating (the window is a torus, the noise tiles).
            sampler: device.create_sampler(&SamplerDescriptor {
                label: Some("surface_weather_sampler"),
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                ..SamplerDescriptor::default()
            }),
            // The shelter gather: unfiltered depth, clamped at the map's edge.
            shelter_sampler: device.create_sampler(&SamplerDescriptor {
                label: Some("surface_weather_shelter_sampler"),
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                ..SamplerDescriptor::default()
            }),
            written: None,
            marked: Vec::new(),
        }
    }
}

fn view_of(images: &RenderAssets<GpuImage>, image: &bevy_asset::Handle<Image>) -> Option<bevy_render::render_resource::TextureView> {
    images.get(image).map(|g| g.texture_view.clone())
}

/// Writes the uniform when it changed and gives the marked cameras' views the bindings.
#[allow(clippy::too_many_arguments)]
pub(crate) fn extract_view(
    mut commands: Commands,
    cameras: Extract<Query<&RenderEntity, With<SurfaceWeatherCamera>>>,
    gpu: Extract<Option<Res<SurfaceGpu>>>,
    uniform: Extract<Res<SurfaceUniform>>,
    images: Res<RenderAssets<GpuImage>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut state: Local<Option<ViewState>>,
) {
    let state = state.get_or_insert_with(|| ViewState::new(&device));
    let slots = uniform.slots();
    if state.written != Some(slots) {
        let bytes: Vec<u8> = slots.iter().flatten().flat_map(|v| v.to_le_bytes()).collect();
        queue.write_buffer(&state.uniform, 0, &bytes);
        state.written = Some(slots);
    }
    let bindings = gpu.as_deref().and_then(|gpu| {
        Some(SurfaceWeatherBindings {
            uniform: state.uniform.clone(),
            cells: view_of(&images, &gpu.cells)?,
            shelter: view_of(&images, &gpu.shelter)?,
            noise: view_of(&images, &gpu.noise)?,
            sampler: state.sampler.clone(),
            shelter_sampler: state.shelter_sampler.clone(),
        })
    });
    let now: Vec<Entity> = match bindings {
        Some(_) => cameras.iter().map(|r| **r).collect(),
        None => Vec::new(),
    };
    for e in state.marked.drain(..).filter(|e| !now.contains(e)) {
        if let Ok(mut c) = commands.get_entity(e) {
            c.try_remove::<SurfaceWeatherView>();
        }
    }
    if let Some(bindings) = bindings {
        for &e in &now {
            if let Ok(mut c) = commands.get_entity(e) {
                c.try_insert(SurfaceWeatherView(bindings.clone()));
            }
        }
    }
    state.marked = now;
}
