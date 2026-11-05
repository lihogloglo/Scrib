//! The cell window texture (surface-weather.md "GPU resources"): 128² RGBA8 cells of 32 m
//! around the camera, toroidal and world-anchored (`sf_surface::view::CellWindow`). A recenter
//! writes the entering strips; a state change ([`WorldSurface::generation`]: a step, the
//! console, a restore) rewrites the whole window. Writes go straight to the GPU texture through
//! the queue (Bevy re-uploads a whole `Image` when the asset changes), as the terrain clipmaps.

use bevy_asset::AssetId;
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_math::{DVec2, IVec4, UVec2, Vec4};
use bevy_platform::time::Instant;
use bevy_render::MainWorld;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{
    Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
};
use bevy_render::renderer::RenderQueue;
use bevy_render::texture::GpuImage;
use bevy_transform::components::GlobalTransform;
use sf_diag_bevy::DiagCounters;
use sf_surface::view::{CellSites, CellWindow, WINDOW_CELLS, fill_texels};
use sf_surface::{CELL_M, cell_of};
use sf_terrain_bevy::TerrainOrigin;
use sf_weather_bevy::{WeatherSite, WorldSurface};

use crate::{SurfaceGpu, SurfaceStats, SurfaceUniform, SurfaceWeatherView};

/// One rectangle of the cell texture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellWrite {
    pub texel: UVec2,
    pub size: UVec2,
    /// RGBA8, row-major.
    pub data: Vec<u8>,
}

/// Writes queued by the main world this frame (the render world takes them).
#[derive(Resource, Default)]
pub struct CellWrites(pub Vec<CellWrite>);

/// The window's placement and what the texture holds.
#[derive(Default)]
pub(crate) struct WindowState {
    window: CellWindow,
    generation: Option<u64>,
    /// Per-channel maxima of the uploaded texels (layer flags).
    max: [u8; 4],
}

/// The active surface-weather camera: its render-space position.
pub(crate) fn camera_position(
    cameras: &Query<(&Camera, &GlobalTransform), With<SurfaceWeatherView>>,
) -> Option<bevy_math::Vec3> {
    cameras
        .iter()
        .filter(|(c, _)| c.is_active)
        .min_by_key(|(c, _)| c.order)
        .map(|(_, t)| t.translation())
}

/// Keeps the window on the camera's cell and the texture on the state.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update_cells(
    surface: Option<Res<WorldSurface>>,
    site: Option<Res<WeatherSite>>,
    origin: Res<TerrainOrigin>,
    cameras: Query<(&Camera, &GlobalTransform), With<SurfaceWeatherView>>,
    mut window_state: Local<WindowState>,
    mut writes: ResMut<CellWrites>,
    mut uniform: ResMut<SurfaceUniform>,
    mut stats: ResMut<SurfaceStats>,
    counters: Option<Res<DiagCounters>>,
) {
    let (Some(surface), Some(cam)) = (surface, camera_position(&cameras)) else {
        // Interiors and no state: no window (the next exterior frame rewrites it).
        window_state.window.reset();
        uniform.window = Vec4::ZERO;
        uniform.flags = 0;
        return;
    };
    let t0 = Instant::now();
    let o = origin.0.to_f64();
    let world = DVec2::new(o.x + f64::from(cam.x), o.z + f64::from(cam.z));
    let mut strips = window_state.window.recenter(cell_of(world));
    let full = window_state.generation != Some(surface.generation());
    if full {
        strips = window_state.window.rect().into_iter().collect();
        window_state.generation = Some(surface.generation());
        window_state.max = [0; 4];
    }
    let grid = surface.grid();
    let uniform_site: &[_] = site.as_deref().map_or(&[], |s| s.climates.as_slice());
    let sites = grid.map_or(CellSites::Uniform(uniform_site), CellSites::Grid);
    let ws = grid.map(sf_surface::CellGrid::worldspace);
    let mut texels = 0u64;
    for strip in strips {
        for r in CellWindow::texel_rects(strip) {
            let mut data = Vec::new();
            let max = fill_texels(&surface.state, sites, ws, r, &mut data);
            for (m, v) in window_state.max.iter_mut().zip(max) {
                *m = (*m).max(v);
            }
            texels += u64::from(r.size.x * r.size.y);
            writes.0.push(CellWrite {
                texel: r.texel,
                size: r.size,
                data,
            });
        }
    }
    let min = window_state.window.min().expect("placed by recenter");
    #[allow(clippy::cast_possible_truncation)]
    let corner = ((min.as_dvec2() * CELL_M) - DVec2::new(o.x, o.z)).as_vec2();
    #[allow(clippy::cast_possible_truncation)]
    let inv_cell = (1.0 / CELL_M) as f32;
    uniform.window = Vec4::new(corner.x, corner.y, inv_cell, 1.0);
    let t = min.rem_euclid(bevy_math::IVec2::splat(WINDOW_CELLS));
    uniform.cells = IVec4::new(t.x, t.y, WINDOW_CELLS, 0);
    uniform.flags = SurfaceUniform::flags_of(window_state.max);
    if texels > 0 {
        stats.cell_texels += texels;
        stats.full_uploads += u64::from(full);
        if let Some(c) = counters {
            c.add_named(
                "surface/cell_upload_ms",
                sf_diag_bevy::sf_diag::unit::MS,
                t0.elapsed().as_secs_f64() * 1000.0,
            );
        }
    }
}

/// Render world: the writes waiting for their texture.
#[derive(Resource, Default)]
pub(crate) struct PendingCellWrites {
    image: Option<AssetId<Image>>,
    writes: Vec<CellWrite>,
}

pub(crate) fn extract_writes(mut main: ResMut<MainWorld>, mut pending: ResMut<PendingCellWrites>) {
    if pending.image.is_none() {
        pending.image = main.get_resource::<SurfaceGpu>().map(|g| g.cells.id());
    }
    if let Some(mut writes) = main.get_resource_mut::<CellWrites>() {
        pending.writes.append(&mut writes.0);
    }
}

pub(crate) fn write_cells(
    mut pending: ResMut<PendingCellWrites>,
    images: Res<RenderAssets<GpuImage>>,
    queue: Res<RenderQueue>,
) {
    let Some(gpu) = pending.image.and_then(|id| images.get(id)) else {
        return;
    };
    let texture = gpu.texture.clone();
    for w in pending.writes.drain(..) {
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d {
                    x: w.texel.x,
                    y: w.texel.y,
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            &w.data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w.size.x * 4),
                rows_per_image: Some(w.size.y),
            },
            Extent3d {
                width: w.size.x,
                height: w.size.y,
                depth_or_array_layers: 1,
            },
        );
    }
}
