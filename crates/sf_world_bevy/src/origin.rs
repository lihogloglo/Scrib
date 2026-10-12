//! The `WorldOrigin` uniform: stable world coordinates for shaders
//! (world-coordinates.md, "Shaders").
//!
//! Shaders only see origin-relative positions (`GlobalTransform` is
//! relative to the floating origin's cell). Patterns that must stay fixed
//! in the world (tiling, noise, wind, wave phase, scatter seeds) add a
//! *phase*: for a period `P`, `(origin_cell · 256) mod P`, computed exactly
//! in integers on the CPU. `rel_pos + phase_P` is then the world position
//! modulo `P`: continuous across origin moves, and precise near the camera.
//! Periods that divide 256 m need no phase. The WESL module
//! `sf_world_bevy::world_origin` declares the struct and helpers; render
//! code binds [`WorldOriginBuffer`].

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{IVec3, Vec3};
use bevy_render::render_resource::{ShaderType, UniformBuffer};
use bevy_render::renderer::{RenderDevice, RenderQueue};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shader::load_shader_library;
use bevy_transform::TransformSystems;
use big_space::prelude::Grid;
use sf_coords::GRID_CELL;
use sf_world::WorldspaceId;

use crate::{ActiveWorldspace, WorldspaceRoot};
use crate::coords::cell_ivec;

/// Phase periods carried by the uniform (meters).
pub const PHASE_PERIODS: [i64; 2] = [1024, 16384];

/// The active worldspace's floating-origin cell, with phases. Updated in
/// `PostUpdate` after transform propagation.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct WorldOrigin {
    pub worldspace: Option<WorldspaceId>,
    pub cell: IVec3,
    /// `(cell · 256) mod 1024` per axis, meters.
    pub phase_1k: Vec3,
    /// `(cell · 256) mod 16384` per axis, meters.
    pub phase_16k: Vec3,
}

impl WorldOrigin {
    pub fn new(worldspace: Option<WorldspaceId>, cell: IVec3) -> Self {
        Self {
            worldspace,
            cell,
            phase_1k: Self::phase(cell, PHASE_PERIODS[0]),
            phase_16k: Self::phase(cell, PHASE_PERIODS[1]),
        }
    }

    /// `(cell · GRID_CELL) mod period`, exact (integer arithmetic).
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)] // < period, exact in f32
    pub fn phase(cell: IVec3, period: i64) -> Vec3 {
        let edge = GRID_CELL as i64;
        let p = |c: i32| (i64::from(c) * edge).rem_euclid(period) as f32;
        Vec3::new(p(cell.x), p(cell.y), p(cell.z))
    }

    /// What a shader computes: the world position modulo `period` of an
    /// origin-relative position (`period` must be one of [`PHASE_PERIODS`]).
    pub fn wrapped(&self, rel: Vec3, period: i64) -> Vec3 {
        let phase = if period == PHASE_PERIODS[0] { self.phase_1k } else { self.phase_16k };
        #[allow(clippy::cast_precision_loss)]
        let p = period as f32;
        (rel + phase).rem_euclid(Vec3::splat(p))
    }
}

/// GPU layout of [`WorldOrigin`] (`sf_world_bevy::world_origin::WorldOrigin`).
#[derive(Resource, Clone, Copy, Debug, Default, ShaderType)]
pub struct WorldOriginUniform {
    pub cell: IVec3,
    pub phase_1k: Vec3,
    pub phase_16k: Vec3,
}

impl From<&WorldOrigin> for WorldOriginUniform {
    fn from(o: &WorldOrigin) -> Self {
        Self { cell: o.cell, phase_1k: o.phase_1k, phase_16k: o.phase_16k }
    }
}

/// The uniform buffer in the render world, rewritten when the origin
/// moves. Bind it in any pipeline that needs world-stable patterns.
#[derive(Resource, Default)]
pub struct WorldOriginBuffer {
    pub buffer: UniformBuffer<WorldOriginUniform>,
}

pub(crate) struct WorldOriginPlugin;

impl Plugin for WorldOriginPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldOrigin>()
            .add_systems(PostUpdate, update_world_origin.after(TransformSystems::Propagate));
        if app.get_sub_app(RenderApp).is_some() {
            load_shader_library!(app, "world_origin.wesl");
            let render_app = app.sub_app_mut(RenderApp);
            render_app
                .init_resource::<WorldOriginBuffer>()
                .init_resource::<WorldOriginUniform>()
                .add_systems(ExtractSchedule, extract_world_origin)
                .add_systems(Render, prepare_world_origin.in_set(RenderSystems::PrepareResources));
        }
    }
}

fn update_world_origin(
    active: Res<ActiveWorldspace>,
    roots: Query<(&WorldspaceRoot, &Grid)>,
    mut origin: ResMut<WorldOrigin>,
) {
    let Some(ws) = &active.0 else { return };
    let Some((_, grid)) = roots.iter().find(|(r, _)| r.id == *ws) else { return };
    let cell = cell_ivec(&grid.local_floating_origin().cell());
    if origin.worldspace.as_ref() != Some(ws) || origin.cell != cell {
        *origin = WorldOrigin::new(Some(ws.clone()), cell);
    }
}

fn extract_world_origin(mut uniform: ResMut<WorldOriginUniform>, origin: Extract<Res<WorldOrigin>>) {
    if origin.is_changed() {
        *uniform = WorldOriginUniform::from(&**origin);
    }
}

fn prepare_world_origin(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    uniform: Res<WorldOriginUniform>,
    mut buffer: ResMut<WorldOriginBuffer>,
) {
    if uniform.is_changed() || buffer.buffer.buffer().is_none() {
        buffer.buffer.set(*uniform);
        buffer.buffer.write_buffer(&device, &queue);
    }
}
