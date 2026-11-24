//! A generated world's lakes and rivers (GW-3): a guide folder's `water.json` (`sf_worldgen::water`, written by
//! scrib-worldgen `worldgen-water`) becomes one water mesh body per lake (`sf_water::lake_mesh`, flat at its level)
//! and per river (`sf_water::river_mesh`, sloped along its levels, flow per vertex in
//! `sf_water_bevy::mesh::ATTRIBUTE_WATER_FLOW`), each a [`WaterMeshSurface`] (UE5 Water bodies: a lake polygon, a
//! river spline mesh). The meshes are built once on a worker thread when the app is built; the bodies spawn as
//! high-precision children of the exterior's worldspace root (`CellCoord` + `Transform` + `Stationary`, like the
//! streamed cells), so `big_space` keeps them in render space under the floating origin, on the exterior's render
//! layer. Each body's origin is its bounding-box centre (mesh XZ in f32 stays small); mesh Y is the absolute level.
//!
//! Frame: guide (x east, y north) to engine (X, Z) = (x, −y) (`sf_worldgen::water::to_engine_xz`). Look: the
//! coastal optics (`mw_water/coastal`, green clearest, 14 m visibility): the adapter has no fresh-water record, and
//! inland lakes and rivers fall in Jerlov's coastal classes (chlorophyll and dissolved organics absorb the blue),
//! the closest of the sea, coastal, swamp, ash, north and interior records.
//!
//! Speed (GW-6, G-W7): each river point flows at the speed its own slope gives, Manning's open-channel estimate on
//! the bed depth Magna Urbis carved ([`river_speeds`]), with Jarrett's (1984) mountain-stream roughness where the
//! channel is steep (a fixed lowland n puts most of a 10x-scaled world's torrents at the cap, all foam) and the cap
//! at critical flow; the flow shading's `water.flow.*` are bound live in `water.rs`.
//!
//! `water.guide.bodies` off (read at start) or `--water` other than `on` draws none.

use std::thread::JoinHandle;
use std::time::Instant;

use bevy_app::{App, Update};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::visibility::RenderLayers;
use bevy_ecs::prelude::*;
use bevy_log::{error, info};
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};
use glam::{DVec2, DVec3};
use mw_world::ids;
use sf_coords::WorldPos;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Group, Lifetime, Source, Unit, def};
use sf_tunables_bevy::{Tunables, TunablesAppExt};
use sf_water::{BodyMesh, LakeMeshParams, RiverMeshParams, RiverPoint, lake_mesh, river_mesh};
use sf_water_bevy::{WaterLook, WaterMeshSurface};
use sf_world_bevy::WorldspaceRegistry;
use sf_world_bevy::big_space::prelude::Stationary;
use sf_worldgen::water::{GuideWater, to_engine_xz};

#[cfg(test)]
#[path = "worldgen_water_tests.rs"]
mod tests;

/// `water.guide.bodies`: a generated world's lakes and rivers from its guide's `water.json`.
pub const GUIDE_BODIES: Def<bool> = def!(bool "water.guide.bodies" = true)
    .help("A generated world's lakes and rivers (the guide's water.json) as water mesh bodies; read at start")
    .source(Source::Owner("world10x 2026-11-24: rivers and lakes in the generated world"))
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// `water.guide.river_manning_n`: the channel's Manning roughness, s/m^(1/3).
pub const RIVER_MANNING_N: Def<f32> = def!(float "water.guide.river_manning_n" = 0.035)
    .range(0.01, 0.2)
    .unit(Unit::None)
    .help("Generated rivers: Manning roughness n of the bed, s/m^(1/3) (higher = slower water); read at start")
    .source(Source::Industry("Chow 1959, Open-Channel Hydraulics table 5-6: natural stream, clean winding 0.033-0.045"))
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// `water.guide.river_slope_window_m`: the length along the river the slope is measured over (centred).
pub const RIVER_SLOPE_WINDOW_M: Def<f32> = def!(float "water.guide.river_slope_window_m" = 50.0)
    .range(1.0, 1000.0)
    .unit(Unit::Meters)
    .help("Generated rivers: length along the river the level drop is measured over for the speed (longer = smoother speed); read at start")
    .source(Source::Guess)
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// `water.guide.river_speed_min_mps`: the slowest a river flows (flat stretches), m/s.
pub const RIVER_SPEED_MIN_MPS: Def<f32> = def!(float "water.guide.river_speed_min_mps" = 0.2)
    .range(0.0, 5.0)
    .unit(Unit::None)
    .help("Generated rivers: slowest flow in m/s (a flat stretch still drifts); read at start")
    .source(Source::Guess)
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// `water.guide.river_speed_max_mps`: the fastest a river flows (steep rapids), m/s.
pub const RIVER_SPEED_MAX_MPS: Def<f32> = def!(float "water.guide.river_speed_max_mps" = 3.0)
    .range(0.1, 20.0)
    .unit(Unit::None)
    .help("Generated rivers: fastest flow in m/s (steep rapids); read at start")
    .source(Source::Industry(
        "Grant 1997, Critical flow constrains flow hydraulics in mobile-bed streams (WRR 33): steep channels hold near Froude 1, sqrt(g d) = 3 m/s at a 0.9 m bed",
    ))
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// `water.guide.river_steep_n`: Jarrett's mountain-stream roughness on steep reaches ([`jarrett_n`]).
pub const RIVER_STEEP_N: Def<bool> = def!(bool "water.guide.river_steep_n" = true)
    .help("Generated rivers: steep reaches take Jarrett's mountain-stream roughness (off = the fixed Manning n everywhere, the A/B arm); read at start")
    .source(Source::Industry("Jarrett 1984, Hydraulics of high-gradient streams (J. Hydraul. Eng. 110): n = 0.39 S^0.38 R^-0.16, S > 0.002"))
    .group(Group::Water)
    .lifetime(Lifetime::Restart);

/// This module's tunables (the app's catalog).
pub const TUNABLES: &[&dyn AnyDef] =
    &[&GUIDE_BODIES, &RIVER_MANNING_N, &RIVER_STEEP_N, &RIVER_SLOPE_WINDOW_M, &RIVER_SPEED_MIN_MPS, &RIVER_SPEED_MAX_MPS];

/// Magna Urbis's carved bed depth from the river width, `clamp(base + per_width · w, min, max)` m
/// (`terrain/src/vector_water/incision.rs:14`, rev 21e2217): the depth the terrain was carved with.
const MU_BED_DEPTH: [f64; 4] = [0.45, 0.05, 0.6, 2.2]; // tunables: const (port: must match the bed Magna Urbis carved)

/// Jarrett's (1984) fit `n = c · S^a · R^b` (SI; slope m/m, hydraulic radius m): `[c, a, b]`.
const JARRETT: [f64; 3] = [0.39, 0.38, -0.16]; // tunables: const (maths: the published regression's coefficients)

/// Jarrett's (1984) Manning roughness of a high-gradient natural channel, `0.39 S^0.38 R^-0.16` (SI), fitted on
/// Colorado mountain streams at slopes 0.002-0.04 and hydraulic radii 0.15-2.2 m: the bed's boulders and steps
/// slow steep water far more than a lowland n says. It crosses Chow's 0.035 at about S = 0.002, so taking the larger
/// of the two keeps the base n on gentle reaches and is continuous at the switch.
fn jarrett_n(slope: f64, radius: f64) -> f64 {
    let [c, a, b] = JARRETT;
    c * slope.max(0.0).powf(a) * radius.max(1e-3).powf(b)
}

/// How a river's speed follows its slope ([`river_speeds`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RiverFlow {
    pub manning_n: f64,
    /// Steep reaches take [`jarrett_n`] when it is larger.
    pub steep_n: bool,
    pub window_m: f64,
    pub min_mps: f64,
    pub max_mps: f64,
}

impl Default for RiverFlow {
    fn default() -> Self {
        Self {
            manning_n: f64::from(RIVER_MANNING_N.default_value()),
            steep_n: RIVER_STEEP_N.default_value(),
            window_m: f64::from(RIVER_SLOPE_WINDOW_M.default_value()),
            min_mps: f64::from(RIVER_SPEED_MIN_MPS.default_value()),
            max_mps: f64::from(RIVER_SPEED_MAX_MPS.default_value()),
        }
    }
}

/// The bed depth (m) Magna Urbis carves for a river `width_m` wide ([`MU_BED_DEPTH`]).
fn bed_depth(width_m: f64) -> f64 {
    let [base, per_width, lo, hi] = MU_BED_DEPTH;
    (base + per_width * width_m.max(0.0)).clamp(lo, hi)
}

/// Each `water.json` river point's (`[x, y, width_m, level_m]`) downstream speed, m/s: Manning's open-channel
/// estimate `v = (1/n) R^(2/3) S^(1/2)` (SI), with the rectangular hydraulic radius `R = w·d / (w + 2d)` from the
/// width and [`bed_depth`], `n` the base roughness or, with [`RiverFlow::steep_n`], the larger of it and
/// [`jarrett_n`] (so `v` grows only as `S^0.12` on steep reaches), and the slope `S` the level drop over a
/// [`RiverFlow::window_m`] centred on the point
/// (shortened at the ends), the levels linear between points; clamped to `[min_mps, max_mps]`, then averaged over
/// the points within the same window, so a sharp slope break (where `S^(1/2)` alone would leap) ramps over the
/// window instead of jumping between neighbouring points.
pub(crate) fn river_speeds(points: &[[f64; 4]], flow: &RiverFlow) -> Vec<f64> {
    let mut along = Vec::with_capacity(points.len());
    let mut length = 0.0;
    for (i, p) in points.iter().enumerate() {
        if i > 0 {
            let q = points[i - 1];
            length += (p[0] - q[0]).hypot(p[1] - q[1]);
        }
        along.push(length);
    }
    // The level at arc length `at` (linear between points).
    let level = |at: f64| -> f64 {
        let i = along.partition_point(|&d| d <= at).clamp(1, points.len().max(2) - 1);
        let (d0, d1) = (along[i - 1], along.get(i).copied().unwrap_or(along[i - 1]));
        let (l0, l1) = (points[i - 1][3], points.get(i).map_or(points[i - 1][3], |p| p[3]));
        if d1 - d0 <= f64::EPSILON { l1 } else { l0 + (l1 - l0) * ((at - d0) / (d1 - d0)).clamp(0.0, 1.0) }
    };
    let (lo, hi) = (flow.min_mps.max(0.0), flow.max_mps.max(flow.min_mps.max(0.0)));
    let half = flow.window_m.max(0.0) * 0.5;
    let raw: Vec<f64> = points
        .iter()
        .zip(&along)
        .map(|(p, &at)| {
            let (up, down) = ((at - half).max(0.0), (at + half).min(length));
            let slope = if down - up > f64::EPSILON { ((level(up) - level(down)) / (down - up)).max(0.0) } else { 0.0 };
            let (width, depth) = (p[2].max(0.0), bed_depth(p[2]));
            let radius = if width > 0.0 { width * depth / (width + 2.0 * depth) } else { depth };
            let base = flow.manning_n.max(1e-4);
            let n = if flow.steep_n { base.max(jarrett_n(slope, radius)) } else { base };
            let v = radius.powf(2.0 / 3.0) * slope.sqrt() / n;
            if v.is_finite() { v.clamp(lo, hi) } else { lo }
        })
        .collect();
    #[allow(clippy::cast_precision_loss)] // point counts
    along
        .iter()
        .map(|&at| {
            let (first, end) = (along.partition_point(|&d| d < at - half), along.partition_point(|&d| d <= at + half));
            raw[first..end].iter().sum::<f64>() / (end - first) as f64
        })
        .collect()
}

/// The optics record the bodies use (module doc "Look").
pub const LOOK: &str = mw_water::optics::COASTAL;

/// One spawned lake or river (despawned with the world, `water::despawn_world`).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideWaterBody {
    Lake,
    River,
}

/// A built body: where its mesh origin is (engine metres, Y 0) and its mesh.
struct Built {
    kind: GuideWaterBody,
    origin: DVec3,
    mesh: Mesh,
}

/// What the bake hands back: the bodies and the log line's numbers.
struct Baked {
    bodies: Vec<Built>,
    lakes: usize,
    rivers: usize,
    km: f64,
    /// River speed over every river point, m/s: p10, p50, p90.
    speeds: [f64; 3],
    vertices: usize,
    ms: f64,
}

/// The bake, then the bodies ready to spawn (again for each new world, as the ocean).
#[derive(Resource)]
enum GuideWaterBodies {
    Baking(Option<JoinHandle<Baked>>),
    Ready(Vec<(GuideWaterBody, WorldPos, Handle<Mesh>)>),
}

/// Starts the bake of `guide`'s bodies and the system that spawns them; nothing without a guide or with
/// [`GUIDE_BODIES`] off.
pub(crate) fn add(app: &mut App, guide: Option<GuideWater>, look: WaterLook) {
    let on = app.add_tunable(&GUIDE_BODIES).id();
    let steep_n = app.add_tunable(&RIVER_STEEP_N).id();
    let ids = [&RIVER_MANNING_N, &RIVER_SLOPE_WINDOW_M, &RIVER_SPEED_MIN_MPS, &RIVER_SPEED_MAX_MPS].map(|d| app.add_tunable(d).id());
    let tunables = app.world().resource::<Tunables>();
    let (on, steep_n) = (tunables.read(on), tunables.read(steep_n));
    let [manning_n, window_m, min_mps, max_mps] = ids.map(|id| f64::from(tunables.read(id)));
    let flow = RiverFlow { manning_n, steep_n, window_m, min_mps, max_mps };
    let Some(guide) = guide.filter(|_| on) else { return };
    let job = std::thread::Builder::new()
        .name("guide-water-bake".into())
        .spawn(move || bake(&guide, &flow))
        .expect("spawn the guide water bake thread");
    app.insert_resource(GuideWaterBodies::Baking(Some(job))).insert_resource(GuideWaterLook(look)).add_systems(Update, spawn);
}

#[derive(Resource)]
struct GuideWaterLook(WaterLook);

/// Every lake and river mesh, engine frame.
fn bake(guide: &GuideWater, flow: &RiverFlow) -> Baked {
    let t = Instant::now();
    let engine = |p: [f64; 2]| DVec2::from_array(to_engine_xz(p));
    let (lake_params, river_params) = (LakeMeshParams::default(), RiverMeshParams::default());
    let mut bodies = Vec::new();
    let mut speeds = Vec::new();
    for lake in &guide.lakes {
        let outline: Vec<DVec2> = lake.outline.iter().copied().map(engine).collect();
        let holes: Vec<Vec<DVec2>> = lake.holes.iter().map(|h| h.iter().copied().map(engine).collect()).collect();
        let origin = centre(&outline);
        let body = lake_mesh(&outline, &holes, lake.level_m, origin, &lake_params);
        push(&mut bodies, GuideWaterBody::Lake, origin, &body, false);
    }
    for river in &guide.rivers {
        let speed = river_speeds(&river.points, flow);
        #[allow(clippy::cast_possible_truncation)] // widths are metres, speeds m/s
        let spline: Vec<RiverPoint> = river
            .points
            .iter()
            .zip(&speed)
            .map(|(&[x, y, w, level], &v)| RiverPoint { center: engine([x, y]), width: w as f32, height: level, flow_speed: v as f32 })
            .collect();
        speeds.extend(speed);
        let origin = centre(&spline.iter().map(|p| p.center).collect::<Vec<_>>());
        let body = river_mesh(&spline, &river_params, origin);
        push(&mut bodies, GuideWaterBody::River, origin, &body, true);
    }
    let count = |k| bodies.iter().filter(|b| b.kind == k).count();
    Baked {
        lakes: count(GuideWaterBody::Lake),
        rivers: count(GuideWaterBody::River),
        km: guide.river_length_m() / 1000.0,
        speeds: percentiles(&mut speeds),
        vertices: bodies.iter().map(|b| b.mesh.count_vertices()).sum(),
        ms: t.elapsed().as_secs_f64() * 1000.0,
        bodies,
    }
}

/// p10, p50 and p90 of `values` (nearest rank; zeros for none).
fn percentiles(values: &mut [f64]) -> [f64; 3] {
    if values.is_empty() {
        return [0.0; 3];
    }
    values.sort_by(f64::total_cmp);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    [0.1, 0.5, 0.9].map(|q| values[((values.len() - 1) as f64 * q).round() as usize])
}

/// The bounding-box centre of `points` (zero for none).
fn centre(points: &[DVec2]) -> DVec2 {
    let Some(first) = points.first() else { return DVec2::ZERO };
    let (lo, hi) = points.iter().fold((*first, *first), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
    (lo + hi) * 0.5
}

/// `body` as a GPU mesh at `origin` (skipped when empty: a degenerate outline).
fn push(out: &mut Vec<Built>, kind: GuideWaterBody, origin: DVec2, body: &BodyMesh, flow: bool) {
    if body.is_empty() {
        return;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, body.positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, body.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, body.uvs.clone());
    if flow {
        mesh.insert_attribute(sf_water_bevy::mesh::ATTRIBUTE_WATER_FLOW, body.flow.clone());
    }
    mesh.insert_indices(Indices::U32(body.indices.clone()));
    out.push(Built { kind, origin: DVec3::new(origin.x, 0.0, origin.y), mesh });
}

/// Takes the finished bake (meshes into the assets, one log line), then spawns the bodies under the exterior's
/// root once it exists and none are spawned.
fn spawn(
    mut commands: Commands,
    mut state: ResMut<GuideWaterBodies>,
    mut meshes: ResMut<Assets<Mesh>>,
    look: Res<GuideWaterLook>,
    registry: Res<WorldspaceRegistry>,
    spawned: Query<(), With<GuideWaterBody>>,
) {
    if let GuideWaterBodies::Baking(job) = &mut *state {
        if !job.as_ref().is_some_and(JoinHandle::is_finished) {
            return;
        }
        let Some(Ok(baked)) = job.take().map(JoinHandle::join) else {
            error!("worldgen water: the bake thread panicked; no lakes or rivers");
            *state = GuideWaterBodies::Ready(Vec::new());
            return;
        };
        info!(
            "worldgen water: {} lakes, {} rivers, {:.1} km, river speed p10/p50/p90 {:.2}/{:.2}/{:.2} m/s, {} vertices, built in {:.0} ms",
            baked.lakes, baked.rivers, baked.km, baked.speeds[0], baked.speeds[1], baked.speeds[2], baked.vertices, baked.ms
        );
        let ready = baked.bodies.into_iter().map(|b| (b.kind, WorldPos::from_f64(b.origin), meshes.add(b.mesh))).collect();
        *state = GuideWaterBodies::Ready(ready);
    }
    let GuideWaterBodies::Ready(bodies) = &*state else { return };
    if bodies.is_empty() || !spawned.is_empty() {
        return;
    }
    let Some(entry) = registry.get(&ids::exterior()) else { return };
    for (kind, pos, mesh) in bodies {
        let (coord, transform) = sf_world_bevy::to_grid(*pos);
        commands.spawn((
            *kind,
            WaterMeshSurface::still(mesh.clone(), look.0),
            coord,
            transform,
            Stationary,
            RenderLayers::layer(entry.layer),
            ChildOf(entry.root),
        ));
    }
}
