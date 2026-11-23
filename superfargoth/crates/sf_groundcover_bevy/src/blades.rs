//! Grass blades: the `bevy_grass` crate (0.20.6, MIT) behind one plugin.
//!
//! [`BladesPlugin`] adds `bevy_grass`'s `GrassPlugins` without the far-grass terrain material
//! (`FarGrassPlugin`: our terrain shades its own ground) and without `.grass.ron` style files
//! (`GrassStyleFilePlugin`: styles come through our own data). Its settings are the default tier with
//! **blade shadows off**: blades must not cast into views they don't belong to yet. Fields are plain
//! `bevy_grass::GrassField` entities.
//!
//! **Fields (slices GC-3a2b, GC-3b3):** while `groundcover.blades.enabled` is on ([`BLADES_ENABLED`], off by
//! default; registered by [`GroundcoverPlugin`](crate::GroundcoverPlugin)), once a [`GroundcoverSource`], a
//! [`BladeStyles`] and a [`GroundcoverViewer`] exist, the blocks around the viewer ([`blocks_around`]) are planned
//! for every blade species whose style is registered, sampled on the async compute pool ([`sample_block`],
//! nearest first, at most `groundcover.render.max_jobs_in_flight` at once; a block whose ground is not loaded
//! waits), and each block with something to grow becomes one field with a [`BladeBlock`]. The plan follows the
//! viewer's block; a field is despawned only once it is `groundcover.blades.hysteresis_blocks` outside the ring, and
//! the spawns and despawns of one move land in one frame (`bevy_grass` rebuilds its tables on any field change).
//! When the floating origin moves, every field's transform is rewritten from its world centre. Not yet: despawn on
//! disable, trampling, render layers.
//!
//! **Wind (slice GC-3c):** the crate's own wind model is off (`GrassPlugin { wind: false }`); [`sample_blade_wind`]
//! writes `GrassWindFrame` from the weather's wind (`sf_wind_bevy::WindField`, still air without one) through the
//! pure [`blade_wind_frame`]: direction and strength follow the wind, the gust fronts scroll downwind at the wind
//! speed, and the look values are the live `groundcover.blades.*` wind tunables ([`BladeWindSettings`]).
//!
//! Systems: [`BladeSystems::Drive`] (`PostUpdate`, after `WorldOriginSystems`, before
//! `GrassSystems::DetectChanges`, so a field spawned or moved is seen the same frame); [`sample_blade_wind`]
//! (`Update`, in `GrassWindSystems::Sample`).
//!
//! Spec: `superfargoth/docs/specs/groundcover.md` "Blades: integration facts (GC-3R)". No game plugin adds this
//! yet.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use bevy_app::{App, Plugin, PluginGroup, PostUpdate, Update};
use bevy_asset::Assets;
use bevy_camera::Camera3d;
use bevy_ecs::prelude::*;
use bevy_grass::prelude::NoGrass;
use bevy_grass::prelude::{
    FarGrassPlugin, GrassField, GrassPlugin, GrassPlugins, GrassSettings, GrassShadows, GrassStyleFilePlugin,
    GrassSystems, GrassWindFrame, GrassWindSystems,
};
use bevy_grass::GrassWorldOffset;
use bevy_image::Image;
use bevy_math::{DVec2, DVec3, Vec2, Vec3};
use bevy_platform::collections::{HashMap, HashSet};
use bevy_tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use bevy_time::Time;
use bevy_transform::components::{GlobalTransform, Transform};
use sf_coords::WorldPos;
use sf_groundcover::{GroundSample, Groundcover, Overrides, Rect, RenderKind, ScatterSettings, Scatterer, SpeciesIndex};
use sf_tunables_bevy::Tunables;
use sf_wind_bevy::WindField;
use sf_wind_bevy::sf_wind::Wind;
use sf_world_bevy::{WorldOrigin, WorldOriginSystems};

use crate::ground::GroundSource;
use crate::tunables::{
    BLADES_BLOCK, BLADES_CELL, BLADES_ENABLED, BLADES_FLUTTER, BLADES_GUST_CONTRAST, BLADES_GUST_SCALE,
    BLADES_HYSTERESIS, BLADES_RING, BLADES_SWAY, BLADES_SWAY_SPEED, BLADES_WIND_FULL, BLADES_WIND_MAX,
};
use crate::{GroundcoverSource, GroundcoverViewer, RenderSettings, read};

/// The `bevy_grass` style a [`BladeStyles`] entry maps to, for games that don't depend on the crate.
pub use bevy_grass::GrassStyle;

/// Adds `bevy_grass` (blades, its wind and light following) with blade shadows off. Add it after
/// Bevy's render plugins; headless (no render app) it keeps only its main-world part.
pub struct BladesPlugin;

impl Plugin for BladesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            GrassPlugins
                .build()
                .set(GrassPlugin {
                    settings: GrassSettings { shadows: GrassShadows::Off, ..GrassSettings::default() },
                    // The weather's wind drives the blades (`sample_blade_wind`), not the crate's own model.
                    wind: false,
                    ..GrassPlugin::default()
                })
                .disable::<FarGrassPlugin>()
                .disable::<GrassStyleFilePlugin>(),
        )
        .init_resource::<BladeRuntime>()
        .add_systems(PostUpdate, mark_grass_cameras.before(GrassSystems::DetectChanges))
        .add_systems(Update, sample_blade_wind.in_set(GrassWindSystems::Sample).run_if(blades_enabled))
        .add_systems(
            PostUpdate,
            (write_grass_offset, follow_origin, drive_blades)
                .chain()
                .in_set(BladeSystems::Drive)
                .after(WorldOriginSystems)
                .before(GrassSystems::DetectChanges)
                .run_if(blades_enabled),
        );
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Block planning and grid sampling (slice GC-3a2a): pure functions the field spawner calls. Nothing here touches
// the ECS or `bevy_grass`.
// ---------------------------------------------------------------------------------------------------------------

/// Most cells per side of a block's grids: `bevy_grass`'s own cap on a field's baked maps
/// (`MAX_TEXELS`, `bevy_grass-0.20.6/src/ground.rs:29`).
// tunables: const (capacity: the vendored crate's own cap on a field's baked maps)
const MAX_CELLS_PER_SIDE: u32 = 4096;

/// A world-anchored grass block: block `k` covers `[k × block_m, (k + 1) × block_m)` on each axis, in absolute
/// world metres. One blade field per block (and blade species).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockKey {
    pub x: i32,
    pub z: i32,
}

impl BlockKey {
    /// The block holding world point (x, z) (floored: -0.1 is in block -1).
    pub fn containing(x: f64, z: f64, block_m: f32) -> Self {
        let side = f64::from(block_m);
        // Floored first; a world 2³¹ blocks wide is out of range anyway.
        #[allow(clippy::cast_possible_truncation)]
        let floor = |v: f64| (v / side).floor() as i32;
        Self { x: floor(x), z: floor(z) }
    }

    /// The block's world corner (its minimum X and Z), metres.
    pub fn corner(self, block_m: f32) -> [f64; 2] {
        let side = f64::from(block_m);
        [f64::from(self.x) * side, f64::from(self.z) * side]
    }

    /// The block's world rect (XZ, metres).
    pub fn rect(self, block_m: f32) -> Rect {
        let [x, z] = self.corner(block_m);
        let side = f64::from(block_m);
        Rect { min: [x, z], max: [x + side, z + side] }
    }

    /// The block's world centre (XZ, metres): where its field's `Transform` goes (minus the floating origin).
    pub fn centre(self, block_m: f32) -> [f64; 2] {
        let [x, z] = self.corner(block_m);
        let half = 0.5 * f64::from(block_m);
        [x + half, z + half]
    }
}

/// The `ring × ring` blocks centred on the viewer's block (`ring` odd; an even value is rounded up, 0 counts as
/// 1). Nearest first by squared distance in blocks from the viewer's block, ties by Z then X, so the list depends
/// only on the viewer's block.
pub fn blocks_around(viewer_x: f64, viewer_z: f64, block_m: f32, ring: u32) -> Vec<BlockKey> {
    let centre = BlockKey::containing(viewer_x, viewer_z, block_m);
    // `ring` is a small count (its tunable caps it at 15).
    #[allow(clippy::cast_possible_wrap)]
    let half = (ring | 1) as i32 / 2;
    let mut offsets: Vec<(i32, i32)> = (-half..=half).flat_map(|dz| (-half..=half).map(move |dx| (dx, dz))).collect();
    offsets.sort_by_key(|&(dx, dz)| (dx * dx + dz * dz, dz, dx));
    offsets.into_iter().map(|(dx, dz)| BlockKey { x: centre.x + dx, z: centre.z + dz }).collect()
}

/// One block's grids for one blade species.
///
/// **Layout and local coordinates** (`bevy_grass`'s convention): a field is centred on its `Transform` and its
/// closures get the point in metres from the field's centre along its X and Z (`Vec2::x` = X, `Vec2::y` = Z;
/// `bevy_grass-0.20.6/src/api.rs:133-136`), sampled at cell centres `(i + 0.5) × cell − size / 2`, row-major with
/// the first row at −Z (`src/ground.rs:176-184`); cells per side `ceil(size / cell)` (`src/ground.rs:90-95`). The
/// grids here use the same cells, so the field goes at the block's centre with its Y at [`Self::base_y`]
/// (heights are above the field's own Y, `api.rs:137-140`).
#[derive(Clone, Debug, PartialEq)]
pub struct BlockGrids {
    /// Cells per side.
    pub cells: u32,
    /// Cell side, metres: the requested cell snapped to divide the block.
    pub cell_m: f32,
    /// The lowest valid ground height in the block (world Y), metres.
    pub base_y: f32,
    /// Ground height above `base_y` at each cell centre, metres; `NaN` where there is no ground (a hole: no blade
    /// grows there). Row-major, first row at −Z.
    pub heights: Vec<f32>,
    /// Density at each cell centre, 0..1 of [`Self::peak`]. Same layout.
    pub mask: Vec<f32>,
    /// Blades per m² at mask 1: the species' density × the scatter's density scale.
    pub peak: f32,
}

impl BlockGrids {
    /// The field's side, metres (`cells × cell_m`).
    #[allow(clippy::cast_precision_loss)]
    pub fn size_m(&self) -> f32 {
        self.cells as f32 * self.cell_m
    }

    /// The cell index holding field-local point `local` (see the type's docs), clamped at the edges.
    fn index(&self, local: Vec2) -> usize {
        let half = 0.5 * self.size_m();
        let max = self.cells.saturating_sub(1);
        // Floored and clamped into [0, cells - 1] (NaN casts to 0).
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let cell = |v: f32| (((v + half) / self.cell_m).floor().max(0.0) as u32).min(max);
        (cell(local.y) * self.cells + cell(local.x)) as usize
    }

    /// Height above `base_y` of the cell holding field-local `local` (nearest cell, clamped at the edges).
    pub fn height_at(&self, local: Vec2) -> f32 {
        self.heights[self.index(local)]
    }

    /// Mask (0..1) of the cell holding field-local `local` (nearest cell, clamped at the edges).
    pub fn mask_at(&self, local: Vec2) -> f32 {
        self.mask[self.index(local)]
    }
}

/// What [`sample_block`] found.
#[derive(Clone, Debug, PartialEq)]
pub enum BlockSample {
    /// The ground under the block is not loaded yet: ask again later.
    NotReady,
    /// Nothing grows here (every mask cell is 0, no ground, or a species peak of 0): spawn no field.
    Empty,
    Grids(BlockGrids),
}

/// Sample species `species`'s grids over block `key` (`block_m` side, `cell_m` requested cell, snapped the way
/// `bevy_grass` snaps it). `mask` = clamp(`density_at` / `peak`, 0, 1) at each cell centre, with `density_at` the
/// scatter's density twin (rule weights, slope, altitude, water, overrides). A pure function of its inputs. A
/// species with `peak ≤ 0` is `Empty` without asking the ground.
///
/// Cost: two ground samples per cell (one for the height, one inside `density_at`).
pub fn sample_block<G: GroundSource + ?Sized>(
    scatterer: &Scatterer<'_>,
    ground: &G,
    species: SpeciesIndex,
    key: BlockKey,
    block_m: f32,
    cell_m: f32,
) -> BlockSample {
    let peak = scatterer.cover.species(species).density * scatterer.settings.density_scale;
    if peak.is_nan() || peak <= 0.0 {
        return BlockSample::Empty;
    }
    if !ground.ready(key.rect(block_m)) {
        return BlockSample::NotReady;
    }
    // Same count as `bevy_grass`'s `ground::texels` (ceil, at least 1, at most its cap).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let cells = ((block_m / cell_m.max(1e-3)).ceil() as u32).clamp(1, MAX_CELLS_PER_SIDE);
    #[allow(clippy::cast_precision_loss)]
    let cell = block_m / cells as f32;
    let step = f64::from(cell);
    let [x0, z0] = key.corner(block_m);

    let n = cells as usize;
    let mut heights = Vec::with_capacity(n * n);
    let mut mask = Vec::with_capacity(n * n);
    let mut sample = GroundSample::blank();
    let mut base_y = f32::INFINITY;
    let mut any = false;
    for j in 0..cells {
        let z = z0 + (f64::from(j) + 0.5) * step;
        for i in 0..cells {
            let x = x0 + (f64::from(i) + 0.5) * step;
            ground.sample(x, z, &mut sample);
            let valid = sample.valid && sample.height.is_finite();
            let m = if valid { (scatterer.density_at(ground, x, z, species) / peak).clamp(0.0, 1.0) } else { 0.0 };
            // A NaN density is no grass.
            let m = if m.is_nan() { 0.0 } else { m };
            if valid {
                base_y = base_y.min(sample.height);
            }
            heights.push(if valid { sample.height } else { f32::NAN });
            any |= m > 0.0;
            mask.push(m);
        }
    }
    if !any {
        return BlockSample::Empty;
    }
    for h in &mut heights {
        *h -= base_y;
    }
    BlockSample::Grids(BlockGrids { cells, cell_m: cell, base_y, heights, mask, peak })
}

// ---------------------------------------------------------------------------------------------------------------
// Field spawning (slices GC-3a2b, GC-3b3): plan the blocks around the viewer's block, sample them on the async
// compute pool, spawn one `bevy_grass` field per (block, blade species) that has something to grow, in one batch
// per move.
// ---------------------------------------------------------------------------------------------------------------

/// Blade style name (the `style` of `RenderKind::Blade`) → the `bevy_grass` style its fields draw with; each field's
/// `density` is replaced by its block's [`BlockGrids::peak`]. Inserted by the game; a blade species whose style is
/// not here when the blocks are planned is skipped (one warning).
#[derive(Resource, Clone, Debug, Default)]
pub struct BladeStyles(pub HashMap<String, GrassStyle>);

/// On a blade field: which block and species it draws, and where it sits in absolute world metres (its
/// `Transform` is this minus the floating origin).
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct BladeBlock {
    pub key: BlockKey,
    pub species: SpeciesIndex,
    /// The block's world centre (XZ), metres.
    pub centre: [f64; 2],
    /// The field's world Y: its grids' [`BlockGrids::base_y`], metres.
    pub base_y: f64,
}

impl BladeBlock {
    /// The field's translation with the floating origin's frame at `origin` (absolute world metres).
    pub fn translation(&self, origin: DVec3) -> Vec3 {
        (DVec3::new(self.centre[0], self.base_y, self.centre[1]) - origin).as_vec3()
    }
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BladeSystems {
    /// `PostUpdate`, after `WorldOriginSystems`, before `GrassSystems::DetectChanges`: origin follow, then planning,
    /// sampling jobs and spawns.
    Drive,
}

/// Run condition: `groundcover.blades.enabled` (off when there is no store or it is unregistered).
pub fn blades_enabled(tunables: Option<Res<Tunables>>) -> bool {
    tunables.is_some_and(|t| read(&t.0, &BLADES_ENABLED))
}

// ---------------------------------------------------------------------------------------------------------------
// Blade wind (slice GC-3c): the weather's wind as `bevy_grass`'s `GrassWindFrame`.
// ---------------------------------------------------------------------------------------------------------------

/// The live `groundcover.blades.*` wind tunables [`blade_wind_frame`] maps with (defaults in `crate::tunables`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BladeWindSettings {
    /// Wind speed (m/s) that gives strength 1 (`groundcover.blades.wind_full_mps`).
    pub full_mps: f32,
    /// Cap on the strength (`groundcover.blades.wind_max_strength`).
    pub max_strength: f32,
    /// Size of a gust front in the blades, metres (`groundcover.blades.gust_scale_m`).
    pub gust_scale_m: f32,
    /// Gust contrast per unit of the weather's gustiness (`groundcover.blades.gust_contrast_per_gustiness`).
    pub gust_contrast_per_gustiness: f32,
    /// `GrassWindFrame::sway` (`groundcover.blades.sway`).
    pub sway: f32,
    /// `GrassWindFrame::sway_speed` (`groundcover.blades.sway_speed`).
    pub sway_speed: f32,
    /// `GrassWindFrame::flutter` (`groundcover.blades.flutter`).
    pub flutter: f32,
}

impl BladeWindSettings {
    /// The store's values (defaults for anything unregistered).
    pub fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        Self {
            full_mps: read(store, &BLADES_WIND_FULL),
            max_strength: read(store, &BLADES_WIND_MAX),
            gust_scale_m: read(store, &BLADES_GUST_SCALE),
            gust_contrast_per_gustiness: read(store, &BLADES_GUST_CONTRAST),
            sway: read(store, &BLADES_SWAY),
            sway_speed: read(store, &BLADES_SWAY_SPEED),
            flutter: read(store, &BLADES_FLUTTER),
        }
    }
}

impl Default for BladeWindSettings {
    fn default() -> Self {
        Self {
            full_mps: BLADES_WIND_FULL.default_value(),
            max_strength: BLADES_WIND_MAX.default_value(),
            gust_scale_m: BLADES_GUST_SCALE.default_value(),
            gust_contrast_per_gustiness: BLADES_GUST_CONTRAST.default_value(),
            sway: BLADES_SWAY.default_value(),
            sway_speed: BLADES_SWAY_SPEED.default_value(),
            flutter: BLADES_FLUTTER.default_value(),
        }
    }
}

/// This frame's blade wind from the weather's `wind` (`None`: still air), `settings`, last frame's `previous` and
/// the frame's time step `dt` (seconds).
///
/// - `direction`: the mean velocity's direction; a zero velocity (or still air) keeps `previous.direction`.
/// - `strength`: speed / `full_mps`, clamped to `[0, max_strength]`; 0 in still air.
/// - `gust_contrast`: `gust_contrast_per_gustiness` × the weather's gustiness (0 in still air), so steady air shows
///   no fronts.
/// - `gust_scale`, `sway`, `sway_speed`, `flutter`: the settings.
/// - Gust scroll: from `previous.gust_offset`, [`GrassWindFrame::advance`] by `dt` at the wind speed, so the fronts
///   travel downwind at the wind speed and stay continuous when it changes; `advance` makes the offset before the
///   step `previous_gust_offset`, which the crate's motion vectors compare against (its own model does the same in
///   `wind/clock.rs`). The function must therefore run exactly once per frame.
#[must_use]
pub fn blade_wind_frame(wind: Option<&Wind>, settings: &BladeWindSettings, previous: &GrassWindFrame, dt: f32) -> GrassWindFrame {
    let (speed, gustiness) = wind.map_or((0.0, 0.0), |w| (w.speed(), w.gustiness));
    let direction = match wind {
        Some(w) if speed > 1e-6 => w.velocity / speed,
        _ => previous.direction,
    };
    let mut frame = GrassWindFrame {
        direction,
        strength: (speed / settings.full_mps.max(1e-3)).min(settings.max_strength).max(0.0),
        gust_scale: settings.gust_scale_m,
        gust_offset: previous.gust_offset,
        previous_gust_offset: previous.gust_offset,
        sway: settings.sway,
        sway_speed: settings.sway_speed,
        flutter: settings.flutter,
        gust_contrast: (settings.gust_contrast_per_gustiness * gustiness).max(0.0),
    };
    frame.advance(dt, speed);
    frame
}

/// `Update`, in `GrassWindSystems::Sample` (behind [`blades_enabled`]): writes `GrassWindFrame` from the weather's
/// wind ([`WindField`]'s `state.wind`, still air without one) with [`blade_wind_frame`]. `dt` is `Time` in `Update`,
/// the virtual clock, so the gusts pause with the game. The weather writes the wind in `Update` too, in an order
/// this crate does not fix: at worst the blades see last frame's mean wind, which blends over seconds.
#[allow(clippy::needless_pass_by_value)]
pub fn sample_blade_wind(
    time: Res<Time>,
    wind: Option<Res<WindField>>,
    tunables: Option<Res<Tunables>>,
    mut frame: ResMut<GrassWindFrame>,
) {
    let settings = tunables.map_or_else(BladeWindSettings::default, |t| BladeWindSettings::from_tunables(&t.0));
    let next = blade_wind_frame(wind.as_deref().map(|f| &f.state.wind), &settings, &frame, time.delta_secs());
    *frame = next;
}

/// Grass is drawn only by the viewer camera of an enabled blade renderer. Every other 3D camera (and the viewer
/// while blades are off) carries `bevy_grass`'s `NoGrass`, so the crate allocates no per-camera buffers and runs
/// no compute for it: without the marker every `Camera3d` pays for grass, fields or not (smoke bench 2026-11-23).
fn mark_grass_cameras(
    mut commands: Commands,
    tunables: Option<Res<Tunables>>,
    cameras: Query<(Entity, Has<GroundcoverViewer>, Has<NoGrass>), With<Camera3d>>,
) {
    let enabled = tunables.is_some_and(|t| read(&t.0, &BLADES_ENABLED));
    for (entity, viewer, no_grass) in &cameras {
        let wants = enabled && viewer;
        if wants && no_grass {
            commands.entity(entity).try_remove::<NoGrass>();
        } else if !wants && !no_grass {
            commands.entity(entity).try_insert(NoGrass);
        }
    }
}

/// The tunables the planner and the sampling jobs read, fixed when the blocks are planned.
#[derive(Clone, Copy, Debug)]
struct BladeSettings {
    block_m: f32,
    ring: u32,
    /// Blocks a spawned field may lag outside the ring before it is despawned.
    hysteresis: u32,
    cell_m: f32,
    scatter: ScatterSettings,
    max_jobs_in_flight: u32,
}

impl BladeSettings {
    fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        let render = RenderSettings::from_tunables(store);
        Self {
            block_m: read(store, &BLADES_BLOCK),
            ring: read(store, &BLADES_RING),
            hysteresis: read(store, &BLADES_HYSTERESIS),
            cell_m: read(store, &BLADES_CELL),
            scatter: render.scatter,
            max_jobs_in_flight: render.max_jobs_in_flight,
        }
    }

    /// Whether `key` is inside the ring around `centre` grown by the hysteresis on every side: a spawned field
    /// outside it is despawned.
    fn keeps(&self, centre: BlockKey, key: BlockKey) -> bool {
        // Both are small counts (tunable ranges 1..15 and 0..4).
        #[allow(clippy::cast_possible_wrap)]
        let half = ((self.ring | 1) / 2 + self.hysteresis) as i32;
        (key.x - centre.x).abs() <= half && (key.z - centre.z).abs() <= half
    }
}

/// One (block, species) to sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BladeJob {
    key: BlockKey,
    species: SpeciesIndex,
}

/// What every sampling job reads (shared, read-only).
struct BladeInputs {
    cover: Arc<Groundcover>,
    overrides: Arc<Overrides>,
    ground: Arc<dyn GroundSource>,
    settings: BladeSettings,
    /// Blade species with a registered style → that style.
    styles: BTreeMap<SpeciesIndex, GrassStyle>,
}

/// The live state behind [`drive_blades`]: one plan (the ring around the viewer's block) at a time.
///
/// A plan's (block, species) pairs that are neither spawned nor known to be empty go through `queue` (waiting to
/// start; a `NotReady` sample comes back here) → `jobs` (sampling) → `pending` (finished grids). Nothing is spawned
/// or despawned while a job of the plan runs: once `jobs` is empty the **batch** applies, spawning every pending
/// grid and (after a move) despawning every field outside the ring grown by the hysteresis, in one frame, so
/// `bevy_grass` rebuilds its tables once per move. A new viewer block replaces the plan: jobs and pending grids
/// still in the new ring are kept (re-ranked), the rest dropped (dropping a `Task` cancels it).
#[derive(Resource, Default)]
struct BladeRuntime {
    inputs: Option<Arc<BladeInputs>>,
    /// The viewer's block the current plan is for; `None` until the first plan.
    centre: Option<BlockKey>,
    /// Waiting to start, by plan rank (nearest block first).
    queue: BTreeMap<usize, BladeJob>,
    /// Running, with their plan rank.
    jobs: Vec<(usize, BladeJob, Task<BlockSample>)>,
    /// Finished grids of the plan, with their rank, waiting for the batch.
    pending: Vec<(usize, BladeJob, BlockGrids)>,
    /// Sampled `Empty` inside the grown ring: not sampled again until they leave it.
    empty: BTreeSet<BladeJob>,
    /// The plan was replaced since the last batch: the next batch also despawns what left the grown ring.
    moved: bool,
}

impl BladeRuntime {
    /// The blocks were planned at least once.
    #[cfg(test)]
    fn planned(&self) -> bool {
        self.centre.is_some()
    }

    /// Replace the plan with the ring around `eye` (world XZ, in block `centre`); `spawned` = the fields alive now.
    fn replan(&mut self, centre: BlockKey, eye: [f64; 2], spawned: &HashSet<BladeJob>) {
        let Some(inputs) = self.inputs.clone() else { return };
        let settings = inputs.settings;
        let mut rank_of: HashMap<BladeJob, usize> = HashMap::default();
        for key in blocks_around(eye[0], eye[1], settings.block_m, settings.ring) {
            for &species in inputs.styles.keys() {
                let next = rank_of.len();
                rank_of.insert(BladeJob { key, species }, next);
            }
        }
        // Running and finished work still wanted is kept under its new rank; the rest is dropped (cancelled).
        self.jobs.retain_mut(|(rank, job, _)| rank_of.get(job).map(|&r| *rank = r).is_some());
        self.pending.retain_mut(|(rank, job, _)| rank_of.get(job).map(|&r| *rank = r).is_some());
        self.empty.retain(|job| settings.keeps(centre, job.key));
        let busy: HashSet<BladeJob> = self.jobs.iter().map(|(_, j, _)| *j).chain(self.pending.iter().map(|(_, j, _)| *j)).collect();
        self.queue = rank_of
            .into_iter()
            .filter(|(job, _)| !spawned.contains(job) && !self.empty.contains(job) && !busy.contains(job))
            .map(|(job, rank)| (rank, job))
            .collect();
        self.centre = Some(centre);
        self.moved = true;
    }
}

/// The floating origin's frame in absolute world metres (zero without one), as the mesh renderer's `drive`.
fn origin_frame(origin: Option<&WorldOrigin>) -> DVec3 {
    origin.map_or(DVec3::ZERO, |o| WorldPos::new(o.cell, Vec3::ZERO).to_f64())
}

/// What the sampling jobs read: the blade species with a registered style (one warning per species without), the
/// cover, overrides and ground, and the settings.
fn blade_inputs(source: &GroundcoverSource, styles: &BladeStyles, settings: BladeSettings) -> BladeInputs {
    let cover = &source.cover;
    let mut species_styles = BTreeMap::new();
    for s in cover.indices() {
        let RenderKind::Blade { style } = &cover.species(s).render else { continue };
        if let Some(found) = styles.0.get(style) {
            species_styles.insert(s, found.clone());
        } else {
            log::warn!(
                "groundcover blades: species `{}` asks for blade style `{style}`, which is not in BladeStyles; skipped",
                cover.species(s).id
            );
        }
    }
    BladeInputs {
        cover: cover.clone(),
        overrides: source.overrides.clone(),
        ground: source.ground.clone(),
        settings,
        styles: species_styles,
    }
}

/// `PostUpdate` (in [`BladeSystems::Drive`], before [`follow_origin`]): tell `bevy_grass` where the render-space
/// origin is in the world (XZ of [`origin_frame`]; zero without a `WorldOrigin`), so its field seeds come from world
/// positions and an origin shift keeps every blade (slice GC-3b1b). Writes only on a change.
fn write_grass_offset(origin: Option<Res<WorldOrigin>>, mut offset: ResMut<GrassWorldOffset>) {
    let frame = origin_frame(origin.as_deref());
    offset.set_if_neq(GrassWorldOffset(DVec2::new(frame.x, frame.z)));
}

/// `PostUpdate` (in [`BladeSystems::Drive`], before [`drive_blades`]): when the floating origin changes, rewrite
/// every field's transform from its world centre and `base_y`, so fields stay on the ground.
fn follow_origin(origin: Option<Res<WorldOrigin>>, mut blocks: Query<(&BladeBlock, &mut Transform, &mut GlobalTransform)>) {
    let Some(origin) = origin else { return };
    if !origin.is_changed() {
        return;
    }
    let frame = origin_frame(Some(&origin));
    for (block, mut transform, mut global) in &mut blocks {
        transform.translation = block.translation(frame);
        // Root entities: set the global too, so `bevy_grass` sees the new place this frame.
        *global = GlobalTransform::from(*transform);
    }
}

/// `PostUpdate` (in [`BladeSystems::Drive`]): when the viewer's block changes (or on the first frame with a
/// source, styles, tunables and a viewer), replace the plan ([`BladeRuntime::replan`]); then poll the sampling jobs
/// (holding a finished `Grids` for the batch, requeueing a `NotReady` for a later frame), start new ones, nearest
/// first, within `groundcover.render.max_jobs_in_flight`, and once none runs apply the batch: spawn the pending
/// fields and, after a move, despawn those outside the grown ring, all in this frame.
#[allow(clippy::too_many_arguments)]
fn drive_blades(
    mut runtime: ResMut<BladeRuntime>,
    source: Option<Res<GroundcoverSource>>,
    styles: Option<Res<BladeStyles>>,
    tunables: Option<Res<Tunables>>,
    origin: Option<Res<WorldOrigin>>,
    viewers: Query<&GlobalTransform, With<GroundcoverViewer>>,
    blocks: Query<(Entity, &BladeBlock)>,
    mut grass_styles: ResMut<Assets<GrassStyle>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let frame = origin_frame(origin.as_deref());
    if runtime.inputs.is_none() {
        let (Some(source), Some(styles), Some(tunables)) = (source, styles, tunables) else { return };
        if viewers.is_empty() {
            return;
        }
        runtime.inputs = Some(Arc::new(blade_inputs(&source, &styles, BladeSettings::from_tunables(&tunables.0))));
    }
    let Some(inputs) = runtime.inputs.clone() else { return };
    let settings = inputs.settings;

    // The plan follows the viewer's block.
    if let Some(viewer) = viewers.iter().next() {
        let eye = frame + viewer.translation().as_dvec3();
        let centre = BlockKey::containing(eye.x, eye.z, settings.block_m);
        if runtime.centre != Some(centre) {
            let spawned: HashSet<BladeJob> =
                blocks.iter().map(|(_, b)| BladeJob { key: b.key, species: b.species }).collect();
            runtime.replan(centre, [eye.x, eye.z], &spawned);
        }
    }
    let BladeRuntime { centre, queue, jobs, pending, empty, moved, .. } = &mut *runtime;
    let Some(centre) = *centre else { return };

    // Finished jobs.
    let mut retry = Vec::new();
    jobs.retain_mut(|(rank, job, task)| {
        check_ready(task).is_none_or(|sample| {
            match sample {
                BlockSample::NotReady => retry.push((*rank, *job)),
                BlockSample::Empty => {
                    empty.insert(*job);
                }
                BlockSample::Grids(grids) => pending.push((*rank, *job, grids)),
            }
            false
        })
    });

    // New jobs, nearest first, for blocks whose ground is loaded.
    let room = (settings.max_jobs_in_flight as usize).saturating_sub(jobs.len());
    if room > 0 && !queue.is_empty() {
        let start: Vec<usize> = queue
            .iter()
            .filter(|(_, job)| inputs.ground.ready(job.key.rect(settings.block_m)))
            .map(|(&rank, _)| rank)
            .take(room)
            .collect();
        let pool = AsyncComputeTaskPool::get();
        for rank in start {
            let Some(job) = queue.remove(&rank) else { continue };
            let inputs = inputs.clone();
            let task = pool.spawn(async move {
                let scatterer = Scatterer::new(&inputs.cover, &inputs.overrides, inputs.settings.scatter);
                sample_block(&scatterer, &*inputs.ground, job.species, job.key, inputs.settings.block_m, inputs.settings.cell_m)
            });
            jobs.push((rank, job, task));
        }
    }
    // Retried from the next frame on; a block waiting for its ground does not hold the batch back.
    for (rank, job) in retry {
        queue.insert(rank, job);
    }

    // The batch: every change of this plan in one frame, once none of its jobs runs.
    if !jobs.is_empty() || (!*moved && pending.is_empty()) {
        return;
    }
    if std::mem::take(moved) {
        for (entity, block) in &blocks {
            if !settings.keeps(centre, block.key) {
                commands.entity(entity).despawn();
            }
        }
    }
    pending.sort_by_key(|&(rank, ..)| rank);
    for (_, job, grids) in pending.drain(..) {
        let style = GrassStyle { density: grids.peak, ..inputs.styles[&job.species].clone() };
        let [x, z] = job.key.centre(settings.block_m);
        let block = BladeBlock { key: job.key, species: job.species, centre: [x, z], base_y: f64::from(grids.base_y) };
        let field = GrassField::new(Vec2::splat(grids.size_m()), style, &mut grass_styles)
            .with_ground_from_fn(&mut images, grids.cell_m, |p| grids.height_at(p))
            .with_density_from_fn(&mut images, grids.cell_m, |p| grids.mask_at(p));
        let transform = Transform::from_translation(block.translation(frame));
        commands.spawn((field, transform, GlobalTransform::from(transform), block));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_grass::prelude::{GrassField, GrassGround, GrassSettings, GrassShadows, GrassStyle};

    use super::BladesPlugin;

    /// A 64 m field sampled every 0.5 m: 128 texels a side.
    const FIELD_M: f32 = 64.0;
    const CELL_M: f32 = 0.5;
    const TEXELS: UVec2 = UVec2::splat(128);

    fn style() -> GrassStyle {
        GrassStyle { density: 40.0, ..GrassStyle::MEADOW }
    }

    /// Headless (no window, no GPU, no render app): the plugin builds and a field spawns with its
    /// style, ground and density images.
    #[test]
    fn a_field_spawns_headless_with_its_style_ground_and_density() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>()
            .init_asset::<Mesh>()
            .init_asset::<Shader>()
            .add_plugins(BladesPlugin);

        let world = app.world_mut();
        let field = world.resource_scope(|world, mut styles: Mut<Assets<GrassStyle>>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            GrassField::new(Vec2::splat(FIELD_M), style(), &mut styles)
                .with_ground_from_fn(&mut images, CELL_M, |_| 0.0)
                .with_density_from_fn(&mut images, CELL_M, |_| 1.0)
        });
        let entity = world.spawn(field).id();

        app.update();
        app.update();

        let world = app.world();
        assert_eq!(world.resource::<GrassSettings>().shadows, GrassShadows::Off);
        let entity = world.entity(entity);
        let field = entity.get::<GrassField>().expect("the field entity keeps its GrassField");
        assert!(entity.contains::<Transform>() && entity.contains::<Visibility>(), "required components");
        assert_eq!(field.size, Vec2::splat(FIELD_M));
        assert_eq!(world.resource::<Assets<GrassStyle>>().get(&field.style), Some(&style()));

        let images = world.resource::<Assets<Image>>();
        let GrassGround::Heightmap { image, .. } = &field.ground else {
            panic!("a ground from a function is a heightmap, got {:?}", field.ground);
        };
        let ground = images.get(image).expect("the ground image exists");
        assert_eq!(ground.size(), TEXELS);
        let density = images
            .get(field.density_map.as_ref().expect("a density from a function sets the density map"))
            .expect("the density image exists");
        assert_eq!(density.size(), TEXELS);
    }

    /// Block planning and grid sampling (GC-3a2a). Exact float asserts: the values are exactly representable.
    #[allow(clippy::float_cmp)]
    mod plan {
        use bevy_math::{Vec2, Vec3};
        use sf_groundcover::{
            ChannelRule, GroundSample, GroundSampler, Groundcover, Overrides, RangeClass, Rect, RenderKind, RuleTable,
            ScatterSettings, Scatterer, ShadowClass, Span, Species, SpeciesIndex, WeightedSpecies,
        };

        use super::super::{BlockGrids, BlockKey, BlockSample, blocks_around, sample_block};
        use crate::ground::{FnGround, GroundSource};

        const BLOCK_M: f32 = 16.0;
        const CELL_M: f32 = 0.5;
        const DENSITY: f32 = 40.0;
        const SCALE: f32 = 0.5;

        /// One blade species (style "meadow") on channel 0 (weight 1), slopes up to 40 degrees.
        pub(super) fn cover() -> Groundcover {
            let grass = Species {
                id: "grass".into(),
                render: RenderKind::Blade { style: "meadow".into() },
                footprint_m: 0.5,
                density: DENSITY,
                slope_deg: Some(Span::new(0.0, 40.0)),
                altitude_m: None,
                water_m: None,
                scale: Span::new(0.8, 1.2),
                yaw_deg: Span::new(0.0, 360.0),
                tint: [[0.9, 1.0, 0.9], [1.0, 1.0, 0.8]],
                align_to_surface: Span::new(0.0, 0.5),
                wind: 1.0,
                trample: 1.0,
                z_offset_m: 0.0,
                shadow: ShadowClass::None,
                range: RangeClass::Near,
            };
            let rules = RuleTable {
                rules: vec![ChannelRule {
                    channel: 0,
                    species: vec![WeightedSpecies { species: "grass".into(), weight: 1.0 }],
                }],
            };
            Groundcover::new(vec![grass], &rules).expect("valid cover")
        }

        fn settings() -> ScatterSettings {
            ScatterSettings { density_scale: SCALE, ..ScatterSettings::default() }
        }

        /// A tilted plane whose channel-0 weight varies with X (0.1 to 0.9), so the mask varies.
        fn sloped(x: f64, z: f64, out: &mut GroundSample) {
            #[allow(clippy::cast_possible_truncation)]
            {
                out.height = (100.0 + 0.2 * x + 0.1 * z) as f32;
                out.channels.clear();
                out.channels.push((0.5 + 0.4 * (x * 0.3).sin()) as f32);
            }
            out.normal = Vec3::new(-0.2, 1.0, -0.1).normalize();
            out.water_distance = f32::MAX;
            out.valid = true;
        }

        /// The same plane with no weight on channel 0.
        fn bare(x: f64, z: f64, out: &mut GroundSample) {
            sloped(x, z, out);
            out.channels[0] = 0.0;
        }

        /// A ground that is never loaded.
        struct Unloaded;
        impl GroundSampler for Unloaded {
            fn sample(&self, x: f64, z: f64, out: &mut GroundSample) {
                sloped(x, z, out);
            }
        }
        impl GroundSource for Unloaded {
            fn ready(&self, _: Rect) -> bool {
                false
            }
        }

        fn grids(sample: BlockSample) -> BlockGrids {
            match sample {
                BlockSample::Grids(g) => g,
                other => panic!("expected grids, got {other:?}"),
            }
        }

        #[test]
        fn blocks_around_is_the_ring_nearest_first_and_floors_negatives() {
            let list = blocks_around(10.0, 20.0, 64.0, 5);
            assert_eq!(list.len(), 25);
            assert_eq!(list[0], BlockKey { x: 0, z: 0 });
            // The four edge neighbours next, in (z, x) order, then the diagonals.
            assert_eq!(
                list[1..5],
                [BlockKey { x: 0, z: -1 }, BlockKey { x: -1, z: 0 }, BlockKey { x: 1, z: 0 }, BlockKey { x: 0, z: 1 }]
            );
            assert_eq!(list[5], BlockKey { x: -1, z: -1 });
            assert_eq!(list[24], BlockKey { x: 2, z: 2 });
            // Every block once.
            let mut sorted = list.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), 25);
            // The same list anywhere inside the viewer's block, and on every call.
            assert_eq!(blocks_around(0.0, 0.0, 64.0, 5), list);
            assert_eq!(blocks_around(63.99, 0.5, 64.0, 5), list);
            assert_eq!(blocks_around(10.0, 20.0, 64.0, 5), list);
            // An even ring rounds up; 0 is 1.
            assert_eq!(blocks_around(10.0, 20.0, 64.0, 4), list);
            assert_eq!(blocks_around(10.0, 20.0, 64.0, 0), vec![BlockKey { x: 0, z: 0 }]);
            assert_eq!(blocks_around(10.0, 20.0, 64.0, 3).len(), 9);
            // Negative coordinates floor.
            assert_eq!(blocks_around(-0.1, -64.0, 64.0, 1), vec![BlockKey { x: -1, z: -1 }]);
            assert_eq!(BlockKey::containing(-64.1, 64.0, 64.0), BlockKey { x: -2, z: 1 });
            let k = BlockKey { x: -1, z: 2 };
            assert_eq!(k.rect(64.0), Rect { min: [-64.0, 128.0], max: [0.0, 192.0] });
            assert_eq!(k.centre(64.0), [-32.0, 160.0]);
        }

        #[test]
        fn sample_block_matches_the_density_twin_and_the_ground() {
            let cover = cover();
            let overrides = Overrides::none();
            let scatterer = Scatterer::new(&cover, &overrides, settings());
            let ground = FnGround(sloped);
            let s = SpeciesIndex(0);
            let key = BlockKey { x: -1, z: 2 };
            let g = grids(sample_block(&scatterer, &ground, s, key, BLOCK_M, CELL_M));

            assert_eq!(g.cells, 32);
            assert_eq!(g.cell_m, CELL_M);
            assert_eq!(g.peak, DENSITY * SCALE);
            assert_eq!(g.heights.len(), 32 * 32);
            assert_eq!(g.mask.len(), 32 * 32);
            // The lowest cell centre is the (-X, -Z) corner one: (-15.75, 32.25).
            #[allow(clippy::cast_possible_truncation)]
            let lowest = (100.0 + 0.2 * -15.75 + 0.1 * 32.25_f64) as f32;
            assert_eq!(g.base_y, lowest);

            let [x0, z0] = key.corner(BLOCK_M);
            let mut sample = GroundSample::blank();
            for (i, j) in [(0_u32, 0_u32), (31, 0), (0, 31), (31, 31), (7, 19), (20, 3)] {
                let x = x0 + f64::from(i) * 0.5 + 0.25;
                let z = z0 + f64::from(j) * 0.5 + 0.25;
                let cell = (j * 32 + i) as usize;
                let expected = scatterer.density_at(&ground, x, z, s) / g.peak;
                assert!(expected > 0.0 && expected < 1.0, "the test ground gives a partial mask, got {expected}");
                assert!((g.mask[cell] - expected).abs() <= 1e-6, "mask at ({i}, {j}): {} vs {expected}", g.mask[cell]);
                ground.sample(x, z, &mut sample);
                assert!((g.heights[cell] - (sample.height - g.base_y)).abs() <= 1e-5, "height at ({i}, {j})");
            }
        }

        #[test]
        fn sample_block_is_empty_without_weight_or_peak_and_waits_for_the_ground() {
            let cover = cover();
            let overrides = Overrides::none();
            let scatterer = Scatterer::new(&cover, &overrides, settings());
            let s = SpeciesIndex(0);
            let key = BlockKey { x: 3, z: -4 };
            assert_eq!(sample_block(&scatterer, &FnGround(bare), s, key, BLOCK_M, CELL_M), BlockSample::Empty);
            assert_eq!(sample_block(&scatterer, &Unloaded, s, key, BLOCK_M, CELL_M), BlockSample::NotReady);
            let off = Scatterer::new(&cover, &overrides, ScatterSettings { density_scale: 0.0, ..settings() });
            assert_eq!(sample_block(&off, &Unloaded, s, key, BLOCK_M, CELL_M), BlockSample::Empty);
        }

        #[test]
        fn lookups_read_the_cell_holding_the_point_and_clamp_outside() {
            // 4 x 4 cells of 1 m: local -2..2 on each axis; value = cell index.
            #[allow(clippy::cast_precision_loss)]
            let values: Vec<f32> = (0..16).map(|c| c as f32).collect();
            let g = BlockGrids {
                cells: 4,
                cell_m: 1.0,
                base_y: 0.0,
                heights: values.clone(),
                mask: values.iter().map(|v| v / 16.0).collect(),
                peak: 1.0,
            };
            assert_eq!(g.size_m(), 4.0);
            // Cell (i, j) = (0, 0) is the (-X, -Z) corner; X is the column, Z the row.
            assert_eq!(g.height_at(Vec2::new(-1.5, -1.5)), 0.0);
            assert_eq!(g.height_at(Vec2::new(-1.9, -1.1)), 0.0);
            assert_eq!(g.height_at(Vec2::new(0.5, -1.5)), 2.0);
            assert_eq!(g.height_at(Vec2::new(-1.5, 0.5)), 8.0);
            assert_eq!(g.height_at(Vec2::new(1.2, 1.7)), 15.0);
            assert_eq!(g.height_at(Vec2::ZERO), 10.0);
            assert_eq!(g.mask_at(Vec2::new(0.5, -1.5)), 2.0 / 16.0);
            // Outside clamps to the edge cells.
            assert_eq!(g.height_at(Vec2::new(-50.0, -50.0)), 0.0);
            assert_eq!(g.height_at(Vec2::new(50.0, -50.0)), 3.0);
            assert_eq!(g.height_at(Vec2::new(-50.0, 50.0)), 12.0);
            assert_eq!(g.height_at(Vec2::new(0.5, 7.0)), 14.0);
            assert_eq!(g.mask_at(Vec2::new(9.0, 9.0)), 15.0 / 16.0);
        }
    }

    /// Field spawning (GC-3a2b), headless. Exact float asserts: the values are exactly representable.
    #[allow(clippy::float_cmp)]
    mod fields {
        use std::sync::Arc;
        use std::time::{Duration, Instant};

        use bevy::prelude::*;
        use bevy_grass::GrassWorldOffset;
        use bevy_grass::prelude::{GrassField, GrassStyle, NoGrass};
        use bevy_math::{DVec2, DVec3};
        use sf_coords::WorldPos;
        use sf_groundcover::{GroundSample, Overrides, SpeciesIndex};
        use sf_tunables::{SetBy, Value};
        use sf_tunables_bevy::{Tunables, TunablesAppExt};
        use sf_world_bevy::WorldOrigin;

        use super::super::{BladeBlock, BladeRuntime, BladeStyles, BladesPlugin, BlockKey};
        use super::plan::cover;
        use crate::{FnGround, GroundcoverSource, GroundcoverViewer, tunables};

        const BLOCK_M: f32 = 16.0;
        const ORIGIN_CELL: IVec3 = IVec3::new(1, 0, -2);

        /// A plane rising along Z (50 m + Z / 4), level normals; grass (channel 0) only at X ≥ 0.
        fn half_grass(x: f64, z: f64, out: &mut GroundSample) {
            #[allow(clippy::cast_possible_truncation)]
            {
                out.height = (50.0 + 0.25 * z) as f32;
            }
            out.normal = Vec3::Y;
            out.water_distance = f32::MAX;
            out.valid = true;
            out.channels.clear();
            out.channels.push(if x >= 0.0 { 1.0 } else { 0.0 });
        }

        fn frame(cell: IVec3) -> DVec3 {
            WorldPos::new(cell, Vec3::ZERO).to_f64()
        }

        /// Grass everywhere on the same plane as [`half_grass`].
        fn all_grass(x: f64, z: f64, out: &mut GroundSample) {
            half_grass(x.abs(), z, out);
        }

        /// 16 m blocks, a 3 × 3 ring, the viewer at world (8, 60, 8) (block (0, 0)), the origin at `ORIGIN_CELL`.
        fn app(enabled: bool, style: &str) -> App {
            app_with(enabled, style, half_grass, BLOCK_M, 3, 1)
        }

        /// [`app`] with its ground, ring and hysteresis.
        fn app_with(enabled: bool, style: &str, ground: fn(f64, f64, &mut GroundSample), block_m: f32, ring: i64, hysteresis: i64) -> App {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, AssetPlugin::default()))
                .init_asset::<Image>()
                .init_asset::<Mesh>()
                .init_asset::<Shader>()
                .add_plugins(BladesPlugin)
                .register_tunables(tunables::TUNABLES);
            {
                let mut res = app.world_mut().resource_mut::<Tunables>();
                let store = &mut res.0;
                let values = [
                    (store.id_of(&tunables::BLADES_ENABLED).expect("registered").untyped(), Value::Bool(enabled)),
                    (store.id_of(&tunables::BLADES_BLOCK).expect("registered").untyped(), Value::Float(f64::from(block_m))),
                    (store.id_of(&tunables::BLADES_RING).expect("registered").untyped(), Value::Int(ring)),
                    (store.id_of(&tunables::BLADES_HYSTERESIS).expect("registered").untyped(), Value::Int(hysteresis)),
                ];
                for (id, v) in values {
                    store.set_by(id, v, SetBy::Console).expect("valid value");
                }
            }
            let mut styles = BladeStyles::default();
            styles.0.insert(style.into(), GrassStyle::MEADOW);
            app.insert_resource(styles)
                .insert_resource(WorldOrigin::new(None, ORIGIN_CELL))
                .insert_resource(GroundcoverSource {
                    cover: Arc::new(cover()),
                    overrides: Arc::new(Overrides::none()),
                    ground: Arc::new(FnGround(ground)),
                });
            let eye = (DVec3::new(8.0, 60.0, 8.0) - frame(ORIGIN_CELL)).as_vec3();
            app.world_mut().spawn((Transform::from_translation(eye), GlobalTransform::from_translation(eye), GroundcoverViewer));
            app
        }

        /// Runs frames until the plan exists and every job finished (bounded).
        fn settle(app: &mut App) {
            let deadline = Instant::now() + Duration::from_secs(30);
            loop {
                app.update();
                let rt = app.world().resource::<BladeRuntime>();
                if rt.planned() && rt.queue.is_empty() && rt.jobs.is_empty() && rt.pending.is_empty() && !rt.moved {
                    return;
                }
                assert!(Instant::now() < deadline, "the blade jobs did not finish");
                std::thread::yield_now();
            }
        }

        fn fields(app: &mut App) -> Vec<(BladeBlock, GrassField, Transform, GlobalTransform)> {
            let mut list: Vec<_> = app
                .world_mut()
                .query::<(&BladeBlock, &GrassField, &Transform, &GlobalTransform)>()
                .iter(app.world())
                .map(|(b, f, t, g)| (*b, f.clone(), *t, *g))
                .collect();
            list.sort_by_key(|(b, ..)| b.key);
            list
        }

        #[test]
        fn one_field_per_grown_block_placed_on_the_ground_and_following_the_origin() {
            let mut app = app(true, "meadow");
            settle(&mut app);
            let spawned = fields(&mut app);

            // Blocks x = -1 lie at X < 0 (no grass): no field. x = 0, 1 each get one.
            let expected: Vec<BlockKey> = (0..=1).flat_map(|x| (-1..=1).map(move |z| BlockKey { x, z })).collect();
            let mut keys: Vec<BlockKey> = spawned.iter().map(|(b, ..)| b.key).collect();
            keys.sort();
            let mut sorted = expected.clone();
            sorted.sort();
            assert_eq!(keys, sorted);
            assert_eq!(app.world_mut().query::<&GrassField>().iter(app.world()).count(), 6, "no field without a block");

            let f0 = frame(ORIGIN_CELL);
            let styles = app.world().resource::<Assets<GrassStyle>>();
            let mut before = Vec::new();
            for (block, field, transform, global) in &spawned {
                assert_eq!(block.species, SpeciesIndex(0));
                assert_eq!(block.centre, block.key.centre(BLOCK_M));
                // The lowest cell centre is the -Z edge row: Z = corner + 0.25.
                let z0 = f64::from(block.key.z) * f64::from(BLOCK_M) + 0.25;
                #[allow(clippy::cast_possible_truncation)]
                let lowest = (50.0 + 0.25 * z0) as f32;
                assert_eq!(block.base_y, f64::from(lowest));
                assert_eq!(field.size, Vec2::splat(BLOCK_M));
                #[allow(clippy::cast_possible_truncation)]
                let want = Vec3::new(
                    (block.centre[0] - f0.x) as f32,
                    (block.base_y - f0.y) as f32,
                    (block.centre[1] - f0.z) as f32,
                );
                assert_eq!(transform.translation, want);
                assert_eq!(global.translation(), want);
                let style = styles.get(&field.style).expect("the field's style exists");
                assert_eq!(style.blade, GrassStyle::MEADOW.blade);
                assert!(style.density > 0.0, "the style's density is the block's peak");
                before.push(want);
            }

            // The origin moves: every field moves by minus the shift.
            let cell = ORIGIN_CELL + IVec3::new(2, 0, -1);
            app.insert_resource(WorldOrigin::new(None, cell));
            app.update();
            let delta = (frame(cell) - f0).as_vec3();
            assert_ne!(delta, Vec3::ZERO);
            let moved = fields(&mut app);
            assert_eq!(moved.len(), 6, "an origin shift spawns nothing");
            for ((block, _, transform, global), old) in moved.iter().zip(&before) {
                assert_eq!(transform.translation, block.translation(frame(cell)));
                assert_eq!(global.translation(), transform.translation);
                assert!((transform.translation - (*old - delta)).abs().max_element() <= 1e-3, "{block:?}");
            }
        }

        #[test]
        fn nothing_spawns_when_disabled_or_when_the_style_is_not_registered() {
            let mut off = app(false, "meadow");
            for _ in 0..20 {
                off.update();
            }
            assert!(!off.world().resource::<BladeRuntime>().planned());
            assert_eq!(off.world_mut().query::<&GrassField>().iter(off.world()).count(), 0);

            let mut unknown = app(true, "other");
            settle(&mut unknown);
            assert_eq!(unknown.world_mut().query::<&GrassField>().iter(unknown.world()).count(), 0);
            assert_eq!(unknown.world_mut().query::<&BladeBlock>().iter(unknown.world()).count(), 0);
        }

        // ---- Following the viewer (slice GC-3b3) ----

        /// Puts the viewer at world (x, 60, z) (its `GlobalTransform` too: no transform propagation headless).
        fn walk_to(app: &mut App, x: f64, z: f64) {
            let eye = (DVec3::new(x, 60.0, z) - frame(ORIGIN_CELL)).as_vec3();
            let mut viewers = app.world_mut().query_filtered::<(&mut Transform, &mut GlobalTransform), With<GroundcoverViewer>>();
            for (mut transform, mut global) in viewers.iter_mut(app.world_mut()) {
                transform.translation = eye;
                *global = GlobalTransform::from_translation(eye);
            }
        }

        /// The blade fields, sorted by key.
        fn blades(app: &mut App) -> Vec<(BlockKey, Entity)> {
            let mut list: Vec<_> =
                app.world_mut().query::<(Entity, &BladeBlock)>().iter(app.world()).map(|(e, b)| (b.key, e)).collect();
            list.sort();
            list
        }

        fn keys(app: &mut App) -> Vec<BlockKey> {
            let keys: Vec<BlockKey> = blades(app).into_iter().map(|(k, _)| k).collect();
            let mut unique = keys.clone();
            unique.dedup();
            assert_eq!(unique, keys, "a block has one field");
            keys
        }

        /// Blocks `x0..=x1` × `z0..=z1`, sorted as [`blades`] sorts them.
        fn square(x0: i32, x1: i32, z0: i32, z1: i32) -> Vec<BlockKey> {
            let mut list: Vec<BlockKey> = (x0..=x1).flat_map(|x| (z0..=z1).map(move |z| BlockKey { x, z })).collect();
            list.sort();
            list
        }

        /// Runs frames until the runtime settles, returning how many of them changed the set of blade fields.
        fn frames_with_changes(app: &mut App) -> usize {
            let mut before = blades(app);
            let mut changed = 0;
            let deadline = Instant::now() + Duration::from_secs(30);
            loop {
                app.update();
                let now = blades(app);
                if now != before {
                    changed += 1;
                    before = now;
                }
                let rt = app.world().resource::<BladeRuntime>();
                if rt.queue.is_empty() && rt.jobs.is_empty() && rt.pending.is_empty() && !rt.moved {
                    return changed;
                }
                assert!(Instant::now() < deadline, "the blade jobs did not finish");
                std::thread::yield_now();
            }
        }

        #[test]
        fn one_block_along_x_adds_the_new_column_and_keeps_the_old_inside_the_margin() {
            let mut app = app_with(true, "meadow", all_grass, BLOCK_M, 3, 1);
            settle(&mut app);
            assert_eq!(keys(&mut app), square(-1, 1, -1, 1));
            // Block (1, 0): the ring is x 0..=2; x = -1 is one block outside it, inside the margin.
            walk_to(&mut app, 24.0, 8.0);
            settle(&mut app);
            assert_eq!(keys(&mut app), square(-1, 2, -1, 1));
        }

        #[test]
        fn three_blocks_along_x_drop_what_left_the_grown_ring() {
            let mut app = app_with(true, "meadow", all_grass, BLOCK_M, 3, 1);
            settle(&mut app);
            for x in [24.0, 40.0, 56.0] {
                walk_to(&mut app, x, 8.0);
                settle(&mut app);
            }
            // Block (3, 0): ring x 2..=4, grown ring x 1..=5; x = 1 is the trail kept by the margin.
            assert_eq!(keys(&mut app), square(1, 4, -1, 1));
        }

        #[test]
        fn the_spawns_and_despawns_of_one_move_land_in_one_frame() {
            // No margin, so the move both spawns (x = 2) and despawns (x = -1).
            let mut app = app_with(true, "meadow", all_grass, BLOCK_M, 3, 0);
            assert_eq!(frames_with_changes(&mut app), 1, "the first plan is one batch too");
            walk_to(&mut app, 24.0, 8.0);
            assert_eq!(frames_with_changes(&mut app), 1);
            assert_eq!(keys(&mut app), square(0, 2, -1, 1));
        }

        #[test]
        fn pacing_across_a_block_edge_churns_nothing() {
            let mut app = app_with(true, "meadow", all_grass, BLOCK_M, 3, 1);
            settle(&mut app);
            walk_to(&mut app, 17.0, 8.0);
            settle(&mut app);
            let settled = blades(&mut app);
            assert_eq!(settled.iter().map(|(k, _)| *k).collect::<Vec<_>>(), square(-1, 2, -1, 1));
            for x in [15.0, 17.0, 15.0] {
                walk_to(&mut app, x, 8.0);
                assert_eq!(frames_with_changes(&mut app), 0, "x = {x}");
                assert_eq!(blades(&mut app), settled, "the same entities, x = {x}");
            }
        }

        /// `bevy_grass` rebuilds its field tables in the main world; headless it reads the fields' visibility, so
        /// mark them visible as the visibility plugin would.
        fn show(mut fields: Query<&mut InheritedVisibility, With<GrassField>>) {
            for mut visible in &mut fields {
                *visible = InheritedVisibility::VISIBLE;
            }
        }

        #[test]
        #[ignore = "measurement: rebuild time of the grass tables after a one-block move"]
        fn rebuild_time_after_a_one_block_move() {
            let mut app = app_with(true, "meadow", all_grass, 64.0, 5, 1);
            app.add_systems(PostUpdate, show.in_set(bevy::camera::visibility::VisibilitySystems::VisibilityPropagate));
            settle(&mut app);
            let first = app.world().resource::<bevy_grass::GrassStats>().tables;
            walk_to(&mut app, 72.0, 8.0);
            settle(&mut app);
            let moved = app.world().resource::<bevy_grass::GrassStats>().tables;
            println!(
                "grass tables, 5 x 5 ring of 64 m blocks: first plan {} fields / {} chunks in {:?}; after a one-block \
                 move {} fields / {} chunks in {:?} ({} rebuilds in all)",
                first.fields, first.chunks, first.last_rebuild, moved.fields, moved.chunks, moved.last_rebuild, moved.rebuilds
            );
        }

        /// Slice GC-3b1b: `bevy_grass` learns the origin's world XZ on the update after `WorldOrigin` changes.
        #[test]
        fn the_grass_world_offset_follows_the_origin() {
            let mut app = app(true, "meadow");
            app.update();
            let f0 = frame(ORIGIN_CELL);
            assert_eq!(app.world().resource::<GrassWorldOffset>().0, DVec2::new(f0.x, f0.z));

            let cell = ORIGIN_CELL + IVec3::new(-3, 0, 4);
            app.insert_resource(WorldOrigin::new(None, cell));
            app.update();
            let f1 = frame(cell);
            assert_ne!(f1, f0);
            assert_eq!(app.world().resource::<GrassWorldOffset>().0, DVec2::new(f1.x, f1.z));
        }

        /// `mark_grass_cameras`: only the viewer camera of an enabled renderer draws grass; flipping
        /// `groundcover.blades.enabled` on takes `NoGrass` off the viewer, never off another camera.
        #[test]
        fn only_the_viewer_camera_of_enabled_blades_draws_grass() {
            fn cameras(app: &mut App) -> (Entity, Entity) {
                let viewer = app.world_mut().spawn((Camera3d::default(), GroundcoverViewer)).id();
                let other = app.world_mut().spawn(Camera3d::default()).id();
                app.update();
                (viewer, other)
            }
            fn no_grass(app: &App, entity: Entity) -> bool {
                app.world().entity(entity).contains::<NoGrass>()
            }

            let mut on = app(true, "meadow");
            let (viewer, other) = cameras(&mut on);
            assert!(!no_grass(&on, viewer), "the viewer draws grass while blades are enabled");
            assert!(no_grass(&on, other), "another camera never draws grass");

            let mut off = app(false, "meadow");
            let (viewer, other) = cameras(&mut off);
            assert!(no_grass(&off, viewer), "the viewer draws no grass while blades are disabled");
            assert!(no_grass(&off, other));

            {
                let mut res = off.world_mut().resource_mut::<Tunables>();
                let store = &mut res.0;
                let id = store.id_of(&tunables::BLADES_ENABLED).expect("registered").untyped();
                store.set_by(id, Value::Bool(true), SetBy::Console).expect("valid value");
            }
            off.update();
            assert!(!no_grass(&off, viewer), "enabling the blades takes `NoGrass` off the viewer");
            assert!(no_grass(&off, other), "and leaves it on another camera");
        }
    }

    /// Slice GC-3b1b: the field seed is a function of the field corner's world position.
    mod seed {
        use bevy_grass::field_seed;
        use bevy_math::{DVec2, Vec2};

        #[test]
        fn the_seed_survives_an_origin_shift_and_tells_corners_apart() {
            let corner = Vec2::new(-24.0, 40.0);
            let offset = DVec2::new(512.0, -256.0);
            let seed = field_seed(corner, offset);
            for d in [Vec2::new(256.0, 0.0), Vec2::new(-512.0, 768.0), Vec2::new(37.5, -1024.25)] {
                assert_eq!(seed, field_seed(corner - d, offset + d.as_dvec2()), "shift {d}");
            }
            assert_ne!(seed, field_seed(corner + Vec2::new(64.0, 0.0), offset));
            assert_ne!(seed, field_seed(corner + Vec2::new(0.0, 64.0), offset));
            assert_ne!(seed, field_seed(corner, offset + DVec2::new(-64.0, 0.0)));
            assert_eq!(field_seed(corner, DVec2::ZERO), field_seed(corner, DVec2::ZERO));
        }
    }

    /// Blade wind (GC-3c). Exact float asserts: each expected value is the same f32 operation as the mapping's.
    #[allow(clippy::float_cmp)]
    mod wind {
        use bevy::prelude::*;
        use bevy_grass::prelude::GrassWindFrame;
        use sf_tunables::{SetBy, Value};
        use sf_tunables_bevy::{Tunables, TunablesAppExt};
        use sf_wind_bevy::WindField;
        use sf_wind_bevy::sf_wind::{Wind, WindSettings};

        use super::super::{BladeWindSettings, BladesPlugin, blade_wind_frame};
        use crate::tunables;

        fn wind(x: f32, z: f32, gustiness: f32) -> Wind {
            Wind { velocity: Vec2::new(x, z), gustiness, gust_length: 60.0 }
        }

        /// A previous frame with a known direction and gust scroll.
        fn previous() -> GrassWindFrame {
            GrassWindFrame {
                direction: Vec2::new(-1.0, 0.0),
                gust_offset: Vec2::new(10.0, 255.9),
                previous_gust_offset: Vec2::new(9.0, 255.0),
                ..GrassWindFrame::default()
            }
        }

        #[test]
        fn defaults_are_the_tunables_and_the_breeze_preset() {
            let s = BladeWindSettings::default();
            assert_eq!(
                s,
                BladeWindSettings {
                    full_mps: 12.0,
                    max_strength: 1.5,
                    gust_scale_m: 20.0,
                    gust_contrast_per_gustiness: 3.3,
                    sway: 1.0,
                    sway_speed: 1.0,
                    flutter: 1.0,
                }
            );
            let breeze = bevy_grass::prelude::GrassWind::BREEZE;
            assert_eq!((s.sway, s.sway_speed, s.flutter), (breeze.sway, breeze.sway_speed, breeze.flutter));
        }

        #[test]
        fn direction_and_strength_follow_the_wind_and_clamp() {
            let s = BladeWindSettings::default();
            let p = previous();
            // Below full speed: (3, 4) is 5 m/s.
            let f = blade_wind_frame(Some(&wind(3.0, 4.0, 0.3)), &s, &p, 0.0);
            assert_eq!(f.direction, Vec2::new(0.6, 0.8));
            assert_eq!(f.strength, 5.0 / 12.0);
            assert_eq!((f.gust_scale, f.sway, f.sway_speed, f.flutter), (20.0, 1.0, 1.0, 1.0));
            // At full speed, at the cap, above it.
            assert_eq!(blade_wind_frame(Some(&wind(0.0, -12.0, 0.3)), &s, &p, 0.0).strength, 1.0);
            assert_eq!(blade_wind_frame(Some(&wind(0.0, -12.0, 0.3)), &s, &p, 0.0).direction, Vec2::new(0.0, -1.0));
            assert_eq!(blade_wind_frame(Some(&wind(18.0, 0.0, 0.3)), &s, &p, 0.0).strength, 1.5);
            assert_eq!(blade_wind_frame(Some(&wind(30.0, 0.0, 0.3)), &s, &p, 0.0).strength, 1.5);
            let capped = BladeWindSettings { max_strength: 0.25, ..s };
            assert_eq!(blade_wind_frame(Some(&wind(3.0, 4.0, 0.3)), &capped, &p, 0.0).strength, 0.25);
        }

        #[test]
        fn gust_contrast_follows_the_gustiness() {
            let s = BladeWindSettings::default();
            let p = previous();
            let contrast = |g: f32| blade_wind_frame(Some(&wind(3.0, 4.0, g)), &s, &p, 0.0).gust_contrast;
            assert_eq!(contrast(0.0), 0.0);
            assert_eq!(contrast(0.3), 3.3 * 0.3);
            assert!((contrast(0.3) - 1.0).abs() < 0.02, "the default gustiness gives about the crate's contrast 1");
            assert_eq!(contrast(1.0), 3.3);
        }

        #[test]
        fn still_air_and_zero_velocity_keep_the_last_direction_and_stop_the_gusts() {
            let s = BladeWindSettings::default();
            let p = previous();
            for f in [blade_wind_frame(None, &s, &p, 0.5), blade_wind_frame(Some(&wind(0.0, 0.0, 0.3)), &s, &p, 0.5)] {
                assert_eq!(f.direction, p.direction);
                assert_eq!(f.strength, 0.0);
                assert_eq!(f.gust_offset, p.gust_offset, "no speed, no scroll");
                assert_eq!(f.previous_gust_offset, p.gust_offset);
            }
            assert_eq!(blade_wind_frame(None, &s, &p, 0.5).gust_contrast, 0.0);
        }

        #[test]
        fn the_gusts_scroll_downwind_as_the_crates_advance() {
            let s = BladeWindSettings::default();
            let p = previous();
            // (6, 8): 10 m/s toward (0.6, 0.8); half a second over 20 m gusts is a quarter repeat.
            let f = blade_wind_frame(Some(&wind(6.0, 8.0, 0.3)), &s, &p, 0.5);
            let mut expected = GrassWindFrame {
                direction: Vec2::new(0.6, 0.8),
                gust_scale: 20.0,
                gust_offset: p.gust_offset,
                previous_gust_offset: p.previous_gust_offset,
                ..GrassWindFrame::default()
            };
            expected.advance(0.5, 10.0);
            assert_eq!(f.gust_offset, expected.gust_offset);
            assert_eq!(f.previous_gust_offset, p.gust_offset, "the offset before the step");
            // Downwind by (0.15, 0.2) repeats, the Z offset wrapping past 256.
            assert!((f.gust_offset - Vec2::new(10.15, 0.1)).length() < 1e-3, "{:?}", f.gust_offset);
        }

        /// Headless app with blades enabled, optionally a wind field.
        fn wind_app(field: Option<Wind>) -> App {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, AssetPlugin::default()))
                .init_asset::<Image>()
                .init_asset::<Mesh>()
                .init_asset::<Shader>()
                .add_plugins(BladesPlugin)
                .register_tunables(tunables::TUNABLES);
            {
                let mut res = app.world_mut().resource_mut::<Tunables>();
                let id = res.0.id_of(&tunables::BLADES_ENABLED).expect("registered").untyped();
                res.0.set_by(id, Value::Bool(true), SetBy::Console).expect("valid value");
            }
            if let Some(w) = field {
                app.insert_resource(WindField::new(w, WindSettings::default()));
            }
            app
        }

        #[test]
        fn the_app_writes_the_weather_wind_into_the_grass_frame() {
            let mut app = wind_app(Some(wind(0.0, 6.0, 0.5)));
            app.update();
            app.update();
            let f = *app.world().resource::<GrassWindFrame>();
            assert_eq!(f.direction, Vec2::new(0.0, 1.0));
            assert_eq!(f.strength, 0.5);
            assert_eq!(f.gust_contrast, 3.3 * 0.5);
            assert_eq!(f.gust_scale, 20.0);

            let mut still = wind_app(None);
            still.update();
            still.update();
            let f = *still.world().resource::<GrassWindFrame>();
            assert_eq!(f.strength, 0.0);
            assert_eq!(f.gust_contrast, 0.0);
        }
    }
}
