//! `surface probe [x z] [half_m]` (the command and its request live in `sf_weather_bevy`,
//! [`SurfaceProbe`]): reads the four shelter depth maps back from the GPU and prints them as
//! world-anchored text grids (`sf_surface::probe`), the numeric twin of the texture viewer. An
//! instrument: what the GPU captured for a world area, comparable between two redraws.
//!
//! Bevy's own readback (`bevy_render::gpu_readback`), one [`ReadbackOnce`] entity per held map:
//! the copy is taken by the render of the frame the entity is spawned on (after the graph, so
//! after that frame's shadow passes), once, and its bytes come back in a later frame's extract
//! (`ReadbackComplete`, usually 1-3 frames on). `Depth32Float` copies as 4-byte floats, rows of
//! `1024 × 4` bytes (already a multiple of wgpu's 256-byte row alignment, so no padding; the
//! decoder takes a padded row all the same). The maps need `TextureUsages::COPY_SRC`
//! (`shelter::shelter_image`).
//!
//! Each map is paired with the clip it was drawn with. A redraw requested on main frame N is
//! drawn by frame N's render, and the uniform switches to its clip on a later frame
//! ([`ShelterPending`]); so the readback is spawned only on a frame where neither level has a
//! redraw requested or pending (the maps then are what the uniform's clips describe, and the
//! clips, origin and camera are taken on that same frame), and a result is thrown away and asked
//! again if a level was redrawn before it came back. No such frame, or no result, within
//! [`PROBE_TIMEOUT_FRAMES`]: the probe says so instead.

use bevy_asset::Handle;
use bevy_camera::Camera;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_log::info;
use bevy_math::{DVec2, DVec3, Mat4};
use bevy_render::gpu_readback::{ReadbackComplete, ReadbackOnce};
use bevy_transform::components::GlobalTransform;
use sf_surface::probe::{PROBE_FAR_CELL_M, PROBE_LEGEND, PROBE_NEAR_CELL_M, ProbeMap, ProbeWindow, map_center, probe_grid};
use sf_surface::view::{NEAR_LEVEL, SHELTER_SIZE, ShelterLevel};
use sf_weather_bevy::{ProbeRequest, ProbeStatus, SurfaceProbe};

use crate::cells::camera_position;
use crate::shelter::ShelterPending;
use crate::{ShelterFarSettings, SurfaceGpu, SurfaceOrigin, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera};

/// Frames a probe may take, waiting for a settled frame and for the readback together.
// tunables: const (a dev command's give-up bound, about a second: no cost or look)
pub(crate) const PROBE_TIMEOUT_FRAMES: u32 = 120;

/// What every line starts with (the game log is searched for it).
const PREFIX: &str = "surface probe:";

/// One map being read back.
struct MapRead {
    name: &'static str,
    level: ShelterLevel,
    /// The map's centre, world ([`map_center`] of the clip it was drawn with).
    center: DVec3,
    cell_m: f32,
    /// The readback entity (despawned when its bytes arrive).
    entity: Entity,
    depth: Option<Vec<f32>>,
}

/// A readback in flight: the maps and what they were captured with.
struct Capture {
    request: ProbeRequest,
    /// The window's centre, world X and Z.
    center: DVec2,
    /// The camera, world.
    camera: DVec3,
    /// `SurfaceStats::{shelter_redraws, far_redraws}` when the readback was spawned.
    redraws: (u64, u64),
    /// Which levels hold a map (near, far).
    held: (bool, bool),
    maps: Vec<MapRead>,
}

#[derive(Default)]
enum Phase {
    #[default]
    Idle,
    /// Until a frame with no redraw requested or pending.
    Waiting(ProbeRequest),
    Reading(Capture),
}

/// The probe in progress.
#[derive(Resource, Default)]
pub(crate) struct ProbeState {
    phase: Phase,
    /// Frames since the request.
    frames: u32,
    /// Readbacks that came back since the last frame.
    inbox: Vec<(Entity, Vec<u8>)>,
}

impl ProbeState {
    /// The readback entities in flight.
    #[cfg(test)]
    pub(crate) fn reading(&self) -> Vec<Entity> {
        match &self.phase {
            Phase::Reading(c) => c.maps.iter().filter(|m| m.depth.is_none()).map(|m| m.entity).collect(),
            _ => Vec::new(),
        }
    }
}

fn on_readback(event: On<ReadbackComplete>, mut state: ResMut<ProbeState>) {
    state.inbox.push((event.entity, event.data.clone()));
}

/// The readback's bytes as [`SHELTER_SIZE`]² depths: little-endian `f32`s, rows possibly padded.
fn decode(data: &[u8]) -> Option<Vec<f32>> {
    let size = SHELTER_SIZE as usize;
    let row = data.len() / size;
    if row < 4 * size || row * size != data.len() {
        return None;
    }
    let mut depth = Vec::with_capacity(size * size);
    for r in data.chunks_exact(row) {
        depth.extend(r[..4 * size].as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)));
    }
    Some(depth)
}

/// Logs the probe's lines and hands them to the console.
fn finish(probe: &mut SurfaceProbe, state: &mut ProbeState, lines: Vec<String>) {
    let lines: Vec<String> = lines.into_iter().map(|l| format!("{PREFIX} {l}")).collect();
    for line in &lines {
        info!("{line}");
    }
    probe.finish(lines);
    state.phase = Phase::Idle;
    state.inbox.clear();
}

/// The finished capture as text: a summary, the legend, then each map's header and grid (a
/// level holding no map says so).
fn print(capture: &Capture, frames: u32) -> Vec<String> {
    let mut lines = vec![
        format!(
            "window +-{} m around x {:.2} z {:.2}, camera x {:.2} y {:.2} z {:.2}, redraws so far near {} far {}, read back in {frames} frames",
            capture.request.half_m,
            capture.center.x,
            capture.center.y,
            capture.camera.x,
            capture.camera.y,
            capture.camera.z,
            capture.redraws.0,
            capture.redraws.1,
        ),
        PROBE_LEGEND.to_owned(),
    ];
    for (held, name) in [(capture.held.0, "near"), (capture.held.1, "far")] {
        if !held {
            lines.push(format!("{name} solid, {name} leaves: no map held (level off or not drawn yet)"));
        }
    }
    for map in &capture.maps {
        let Some(depth) = map.depth.as_deref() else { continue };
        let window = ProbeWindow { center: capture.center, half_m: capture.request.half_m, cell_m: map.cell_m };
        lines.extend(probe_grid(&ProbeMap { name: map.name, level: map.level, center: map.center, depth }, capture.camera, &window));
    }
    lines
}

/// `SurfaceSystems`, after `shelter::update_shelter` (this frame's redraw requests, pending
/// clips and uniform are final): takes a request, spawns the readbacks on a settled frame and
/// prints the grids when all came back.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value, clippy::too_many_lines)]
pub(crate) fn run_probe(
    mut commands: Commands,
    mut probe: ResMut<SurfaceProbe>,
    mut state: ResMut<ProbeState>,
    pending: Res<ShelterPending>,
    uniform: Res<SurfaceUniform>,
    surface_stats: Res<SurfaceStats>,
    gpu: Option<Res<SurfaceGpu>>,
    origin: Res<SurfaceOrigin>,
    far_settings: Res<ShelterFarSettings>,
    cameras: Query<(&Camera, &GlobalTransform), With<SurfaceWeatherCamera>>,
) {
    if probe.status() == ProbeStatus::Pending
        && let Some(request) = probe.take_request()
    {
        // A new request replaces the one in flight.
        if let Phase::Reading(capture) = std::mem::take(&mut state.phase) {
            for map in &capture.maps {
                commands.entity(map.entity).try_despawn();
            }
        }
        state.inbox.clear();
        state.frames = 0;
        state.phase = Phase::Waiting(request);
    }
    if matches!(state.phase, Phase::Idle) {
        // A stray result (its probe was replaced): nothing waits for it.
        if !state.inbox.is_empty() {
            state.inbox.clear();
        }
        return;
    }
    let state = &mut *state;
    state.frames += 1;
    let redraws = (surface_stats.shelter_redraws, surface_stats.far_redraws);
    match std::mem::take(&mut state.phase) {
        Phase::Idle => {}
        Phase::Waiting(request) => {
            let settled = !pending.near && !pending.far;
            let (Some(gpu), true) = (gpu, settled) else {
                if state.frames >= PROBE_TIMEOUT_FRAMES {
                    let why = format!(
                        "gave up after {} frames: no frame without a shelter redraw requested or pending (near pending {}, far pending {})",
                        state.frames, pending.near, pending.far
                    );
                    finish(&mut probe, state, vec![why]);
                } else {
                    state.phase = Phase::Waiting(request);
                }
                return;
            };
            let held = (uniform.shelter.x > 0.0, uniform.far_shelter.x > 0.0);
            let Some(camera) = camera_position(&cameras).filter(|_| held.0 || held.1) else {
                let why = "no shelter map held: the shelter is off (surface.shelter, no surface layer on and no debug view) or there is no surface camera";
                finish(&mut probe, state, vec![why.to_owned()]);
                return;
            };
            let origin = origin.0.to_f64();
            let camera = origin + camera.as_dvec3();
            let mut maps = Vec::with_capacity(4);
            let mut read = |name, level: ShelterLevel, clip: Mat4, cell_m, image: &Handle<Image>| {
                let entity = commands.spawn(ReadbackOnce::texture(image.clone())).observe(on_readback).id();
                maps.push(MapRead { name, level, center: map_center(&level, clip, origin), cell_m, entity, depth: None });
            };
            if held.0 {
                read("near solid", NEAR_LEVEL, uniform.shelter_clip_from_world, PROBE_NEAR_CELL_M, &gpu.shelter);
                read("near leaves", NEAR_LEVEL, uniform.shelter_clip_from_world, PROBE_NEAR_CELL_M, &gpu.canopy);
            }
            if held.1 {
                let far = far_settings.level();
                read("far solid", far, uniform.far_clip_from_world, PROBE_FAR_CELL_M, &gpu.far_shelter);
                read("far leaves", far, uniform.far_clip_from_world, PROBE_FAR_CELL_M, &gpu.far_canopy);
            }
            let center = request.center.unwrap_or(DVec2::new(camera.x, camera.z));
            state.phase = Phase::Reading(Capture { request, center, camera, redraws, held, maps });
        }
        Phase::Reading(mut capture) => {
            let mut bad = None;
            for (entity, data) in state.inbox.drain(..) {
                if let Some(map) = capture.maps.iter_mut().find(|m| m.entity == entity && m.depth.is_none()) {
                    commands.entity(entity).try_despawn();
                    map.depth = decode(&data);
                    if map.depth.is_none() {
                        bad = Some(format!("{}: the readback came back as {} bytes, expected rows of {SHELTER_SIZE} f32", map.name, data.len()));
                    }
                }
            }
            let despawn = |commands: &mut Commands, capture: &Capture| {
                for map in capture.maps.iter().filter(|m| m.depth.is_none()) {
                    commands.entity(map.entity).try_despawn();
                }
            };
            if let Some(why) = bad {
                despawn(&mut commands, &capture);
                finish(&mut probe, state, vec![why]);
            } else if capture.maps.iter().all(|m| m.depth.is_some()) {
                let overtaken = (capture.held.0 && redraws.0 != capture.redraws.0) || (capture.held.1 && redraws.1 != capture.redraws.1);
                if overtaken && state.frames < PROBE_TIMEOUT_FRAMES {
                    // A level was redrawn while its bytes were on their way: ask again.
                    state.phase = Phase::Waiting(capture.request);
                } else if overtaken {
                    finish(&mut probe, state, vec![format!("gave up after {} frames: every readback was overtaken by a shelter redraw", state.frames)]);
                } else {
                    let lines = print(&capture, state.frames);
                    finish(&mut probe, state, lines);
                }
            } else if state.frames >= PROBE_TIMEOUT_FRAMES {
                let back = capture.maps.iter().filter(|m| m.depth.is_some()).count();
                despawn(&mut commands, &capture);
                let why = format!("gave up after {} frames: {back} of {} readbacks came back (no render app, or the maps are not on the GPU)", state.frames, capture.maps.len());
                finish(&mut probe, state, vec![why]);
            } else {
                state.phase = Phase::Reading(capture);
            }
        }
    }
}
