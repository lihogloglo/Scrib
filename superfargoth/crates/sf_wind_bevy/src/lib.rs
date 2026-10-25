//! The wind field for Bevy (weather.md "Wind"): the thin plugin over
//! [`sf_wind`].
//!
//! - [`WindField`] holds the live [`WindState`] (weather writes
//!   `state.wind`, gameplay adds sources) and, after [`WindSystems::Advance`]
//!   in `PostUpdate`, this frame's [`WindFrame`] (the CPU twin: cloth,
//!   particles, audio) and last frame's.
//! - [`WindOrigin`]: the app writes the floating origin (absolute position of
//!   the origin-relative frame) before `PostUpdate`; without a floating
//!   origin leave it at zero.
//! - [`WindBuffer`]: one storage buffer (a Bevy [`ShaderBuffer`] asset, so
//!   materials bind it with `#[storage(n, read_only)]`) holding
//!   `WindData { params, prev_params, sources[32], prev_sources[32] }`
//!   (3 264 bytes, rewritten in place each frame).
//! - WESL: `sf_wind::field` (the pure field, [`sf_wind::WESL`]) and
//!   `sf_wind_bevy::wind` (the binding + `wind_sample`, `wind_sample_prev`).
//!   The consumer sets the slot with [`wind_shader_defs`].
//!
//! Motion vectors: the buffer carries last frame's field **re-expressed
//! around this frame's origin** (the previous [`WindState`] framed at the
//! current origin), so prepass shaders call `wind_sample_prev` with the same
//! origin-relative position. Bevy keeps previous-frame data for skinning and
//! morph weights the same way (a second copy, not a second sample time).

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{uuid_handle, Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_platform::time::Instant;
use bevy_render::storage::ShaderBuffer;
use bevy_shader::{load_shader_library, Shader, ShaderDefVal};
use bevy_time::Time;
use sf_coords::WorldPos;
use sf_wind::{Wind, WindFrame, WindSettings, WindState, MAX_SOURCES};

pub use sf_wind;

/// The pure field module, imported as `sf_wind::field`.
pub const FIELD_SHADER: Handle<Shader> = uuid_handle!("19a298c1-f3e3-47fc-a309-bf3a061065f1");

/// `vec4`s per frame block: params (6) + sources (3 each).
const PARAMS_VEC4S: usize = 6;
const SOURCE_VEC4S: usize = 3;
/// `vec4`s in `WindData` (current + previous frame).
pub const DATA_VEC4S: usize = 2 * (PARAMS_VEC4S + SOURCE_VEC4S * MAX_SOURCES);
/// Size of [`WindBuffer`] in bytes.
pub const DATA_BYTES: usize = DATA_VEC4S * 16;

/// Adds [`WindField`], [`WindOrigin`], [`WindStats`] and the advance system;
/// with rendering (`Assets<Shader>` / `Assets<ShaderBuffer>` present, so add
/// it after `DefaultPlugins`) also the WESL modules and [`WindBuffer`].
#[derive(Clone, Debug, Default)]
pub struct WindPlugin {
    pub wind: Wind,
    pub settings: WindSettings,
}

/// The live wind and its frames.
#[derive(Resource, Clone, Debug)]
pub struct WindField {
    /// What weather and gameplay write.
    pub state: WindState,
    pub settings: WindSettings,
    frame: WindFrame,
    prev: WindFrame,
    /// `state` as it was rendered last frame.
    prev_state: WindState,
}

impl WindField {
    pub fn new(wind: Wind, settings: WindSettings) -> Self {
        let state = WindState::new(wind);
        Self {
            frame: state.frame(&settings, WorldPos::default()),
            prev: state.frame(&settings, WorldPos::default()),
            prev_state: state.clone(),
            state,
            settings,
        }
    }

    /// This frame's field around [`WindOrigin`] (valid after
    /// [`WindSystems::Advance`]).
    pub fn frame(&self) -> &WindFrame {
        &self.frame
    }

    /// Last frame's field, around this frame's origin.
    pub fn prev_frame(&self) -> &WindFrame {
        &self.prev
    }

    /// Velocity (m/s) at an origin-relative position, `above_ground` meters
    /// over the ground (CPU twin of `wind_sample`).
    pub fn sample(&self, rel: Vec3, above_ground: f32) -> Vec3 {
        self.frame.sample(rel, above_ground)
    }

    /// One frame: advance the state by `dt`, frame it (and last frame's
    /// state) around `origin`.
    pub fn step(&mut self, dt: f32, origin: WorldPos) {
        self.state.advance(dt, &self.settings);
        self.frame = self.state.frame(&self.settings, origin);
        self.prev = self.prev_state.frame(&self.settings, origin);
        self.prev_state.clone_from(&self.state);
    }

    /// `WindData` as uploaded: `vec4`s, current then previous.
    pub fn pack(&self, out: &mut [[f32; 4]; DATA_VEC4S]) {
        // Params of both frames first (WGSL field order), then both source arrays.
        out[..PARAMS_VEC4S].copy_from_slice(&self.frame.params.to_array());
        out[PARAMS_VEC4S..2 * PARAMS_VEC4S].copy_from_slice(&self.prev.params.to_array());
        let sources = 2 * PARAMS_VEC4S;
        for (base, frame) in [(sources, &self.frame), (sources + SOURCE_VEC4S * MAX_SOURCES, &self.prev)] {
            let block = &mut out[base..base + SOURCE_VEC4S * MAX_SOURCES];
            block.fill([0.0; 4]);
            for (slot, s) in block.as_chunks_mut::<SOURCE_VEC4S>().0.iter_mut().zip(&frame.sources) {
                *slot = s.to_array();
            }
        }
    }
}

/// The absolute position of the origin-relative frame (the floating
/// origin). Write it before `PostUpdate` in the frame the origin moves.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct WindOrigin(pub WorldPos);

/// The GPU copy of the field: bind as `var<storage, read>` (`#[storage(n,
/// read_only)]` in a material) and import `sf_wind_bevy::wind`.
#[derive(Resource, Clone, Debug)]
pub struct WindBuffer(pub Handle<ShaderBuffer>);

/// CPU cost of [`WindSystems::Advance`] (advance, frame both states, pack,
/// write the buffer asset), milliseconds.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WindStats {
    pub last_ms: f32,
    /// Exponential mean (1/64 per frame).
    pub mean_ms: f32,
}

/// The plugin's systems.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindSystems {
    /// `PostUpdate`: step [`WindField`] and upload it.
    Advance,
}

/// `SF_WIND_GROUP` / `SF_WIND_BINDING` for a shader importing
/// `sf_wind_bevy::wind`: where the consumer binds [`WindBuffer`].
pub fn wind_shader_defs(group: u32, binding: u32) -> [ShaderDefVal; 2] {
    [
        ShaderDefVal::UInt("SF_WIND_GROUP".into(), group),
        ShaderDefVal::UInt("SF_WIND_BINDING".into(), binding),
    ]
}

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindField::new(self.wind, self.settings))
            .init_resource::<WindOrigin>()
            .init_resource::<WindStats>()
            .add_systems(PostUpdate, advance.in_set(WindSystems::Advance));
        if let Some(mut shaders) = app.world_mut().get_resource_mut::<Assets<Shader>>() {
            // `embedded://<package>/<module>.wesl` → module path `sf_wind::field`.
            shaders
                .insert(&FIELD_SHADER, Shader::from_wesl(sf_wind::WESL, "embedded://sf_wind/field.wesl"))
                .expect("uuid handles always insert");
            load_shader_library!(app, "wind.wesl");
        }
        if let Some(mut buffers) = app.world_mut().get_resource_mut::<Assets<ShaderBuffer>>() {
            let mut buffer = ShaderBuffer::new(vec![[0.0_f32; 4]; DATA_VEC4S], RenderAssetUsages::default());
            buffer.label = "sf_wind".into();
            let handle = buffers.add(buffer);
            app.insert_resource(WindBuffer(handle));
        }
    }
}

fn advance(
    time: Res<Time>,
    origin: Res<WindOrigin>,
    mut field: ResMut<WindField>,
    buffer: Option<Res<WindBuffer>>,
    buffers: Option<ResMut<Assets<ShaderBuffer>>>,
    mut stats: ResMut<WindStats>,
) {
    let start = Instant::now();
    field.step(time.delta_secs(), origin.0);
    if let (Some(buffer), Some(mut buffers)) = (buffer, buffers) {
        let mut data = [[0.0_f32; 4]; DATA_VEC4S];
        field.pack(&mut data);
        if let Some(mut gpu) = buffers.get_mut(&buffer.0) {
            gpu.clear();
            gpu.extend_from_slice(&data);
        }
    }
    let ms = start.elapsed().as_secs_f32() * 1e3;
    stats.last_ms = ms;
    stats.mean_ms += (ms - stats.mean_ms) / 64.0;
}

#[cfg(test)]
mod tests;
