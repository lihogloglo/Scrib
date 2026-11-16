//! Deformer capture, slice S1a (surface-weather.md "S1 deformer capture"): whole legs, bodies,
//! creatures and held items carve the trail ring, not only the host's foot stamps. The Arkham
//! Origins method: an orthographic depth view looking **up** at the meshes under [`Deformer`]
//! roots near the camera's player; per texel the lowest point; the ring pass (`trail.wgsl`)
//! converts it to a ceiling over the ground and merges it (Min) into the ring's B channel, and
//! the raised cover stands under that ceiling (`layer.wesl` `surface_raise_of`; the CPU twins and
//! the composition rule are in `sf_surface::deform`).
//!
//! - **View**: one more view of Bevy's own shadow pass (`sf_sky_bevy::sun_view`, as the shelter
//!   maps), owned by an anchor of its own, redrawn **every frame** while a deformer is in range
//!   (the shelter redraws rarely): `sun_view` spawns the view per request, so a per-frame redraw
//!   needs no knob. Shadow-pass state checked for a view looking up (Bevy 0.20, vendored
//!   `bevy_pbr`): the prepass/shadow pipeline's depth bias is 0 (plus a material's own
//!   `depth_bias`, normally 0), so no knob either; culling is each material's (`cull_mode`, back
//!   faces by default): a closed mesh's underside faces the view and draws, a double-sided
//!   material (cloth, `cull_mode: None`) draws both sides, a one-sided open sheet seen from its
//!   back does not (the host makes cloth double-sided, as it must for the main view anyway).
//!   Depth clamp (the ortho shadow key's unclipped depth) puts geometry under the slab at 1, a
//!   full carve; `sun_view` keeps casters under the near plane in the frustum.
//! - **Casters** ([`split_deformers`]): the meshes under the nearest [`DeformSettings::max_actors`]
//!   roots within [`DeformSettings::range_m`] of the window centre (skinned or not, held items
//!   included), visible, on the camera's layers, at one fixed coarse LOD
//!   ([`DeformSettings::lod_distance_m`], `bevy_render::view::RenderViewLodDistances`), made
//!   visible to Bevy's extraction ([`mark_casters_visible`]) whatever the main view sees.
//!   `NotShadowCaster` is not a filter here; Bevy's CPU mesh path still drops such meshes in
//!   `queue_shadows`, the GPU path draws them. [`NoDeform`] meshes are left out (S1b: a
//!   first-person view-model copy, drawn with the camera's projection).
//! - **Gate** ([`cover_under`], S1b): no capture and no gaps unless snow or ash lies on a cover
//!   cell under the window (`sf_weather_bevy::WorldSurface`); without a
//!   `SurfaceWeatherCamera` (a host's indoor view) nothing runs at all.
//! - **Window**: [`DeformSettings::window_m`] around the [`Deformer`] root nearest the camera
//!   (the player), world-snapped to whole texels (`sf_surface::deform::capture_min`); the target
//!   is [`MAX_CAPTURE_SIZE`]² `Depth32Float`, the window in its first texels. The view's
//!   projection and the request's `size` span the whole target: Bevy's shadow pass sets no
//!   viewport (`sf_surface::deform::Capture::view`; trail-offset 2026-11-17).
//! - **Diagnostics**: the target is `tex show surface.deform` (reverse Z: 1 at the slab's
//!   bottom), the ring `tex show surface.trail b -0.5 0` for its ceiling channel; every
//!   [`DeformSettings::log_s`] while a capture runs, one `surface deform:` info line (focus,
//!   window and ring corners, slab, ground, casters), and one when the cover gate starts
//!   skipping the capture.
//! - **Ground** ([`DeformGround`]): a host hook (the terrain's heightfield) sampled on a coarse
//!   world-snapped grid ([`DeformSettings::ground_step_m`]) that the ring pass reads as a storage
//!   buffer. Absent, or `None` at a point: a plane through the player's root (exact on flat
//!   ground, wrong on slopes), logged once.
//! - **Timing**: the ring pass runs before the prepass (the terrain displaces from the ring);
//!   the shadow pass, which draws the capture, runs after it. So frame N merges the capture drawn
//!   in frame N-1 with N-1's placement ([`TrailFrame::capture`]); a floating-origin shift drops
//!   that one frame's merge.
//! - **Sweep gaps** ([`DeformGaps`]): a root that moved farther than its [`Deformer::width_m`]
//!   plus [`DeformSettings::gap_texels`] since the last capture is listed with its segment; the
//!   host stamps feet along it (S1b). Counter `surface/deform_gaps`.
//!
//! Counters (`SfDiagPlugin`): `surface/deform_captures`, `surface/deform_casters`,
//! `surface/deform_gaps`. Presentation only (net rule): nothing here feeds gameplay.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_camera::Camera;
use bevy_camera::visibility::{InheritedVisibility, RenderLayers, SetViewVisibility, ViewVisibility, VisibilityRange};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_math::{DVec2, Vec2, Vec3};
use bevy_mesh::Mesh3d;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_render::sync_world::{MainEntity, RenderEntity};
use bevy_time::{Real, Time};
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::texview::{DebugTexture, DebugTextures};
use sf_sky_bevy::sun_view::{SunViewRequest, SunViewSource};
use sf_surface::deform::{Capture, GroundGrid, MAX_CAPTURE_SIZE, capture_min, capture_size, slab, sweep_gap};
use sf_tunables::{Def, Lifetime, Source, Unit, def};
use sf_weather_bevy::WorldSurface;

use crate::shelter::lod_casts;
use crate::trail::TrailFrame;
use crate::{SurfaceOrigin, SurfaceStats, SurfaceUniform, SurfaceWeatherCamera};

/// The capture's subview index among its anchor's subviews.
pub const DEFORM_SUBVIEW: u32 = 0xF9;

/// `surface.trail.deform.enabled`: [`DeformSettings::on`].
pub const DEFORM_ENABLED: Def<bool> = def!(bool "surface.trail.deform.enabled" = true)
    .help("Legs, bodies, creatures and held items carve deep snow and ash (a depth view from below); off: foot prints only")
    .source(Source::Guess);

/// `surface.trail.deform.window_m`: [`DeformSettings::window_m`].
pub const DEFORM_WINDOW_M: Def<f32> = def!(float "surface.trail.deform.window_m" = 16.0)
    .range(8.0, 32.0)
    .unit(Unit::Meters)
    .help("The deformer capture covers this square around the player")
    .source(Source::Guess);

/// `surface.trail.deform.texel_m`: [`DeformSettings::texel_m`]; the ring's texel, so one capture
/// texel is one ring texel (no resampling).
pub const DEFORM_TEXEL_M: Def<f32> = def!(float "surface.trail.deform.texel_m" = 0.0625)
    .range(0.0625, 0.25)
    .unit(Unit::Meters)
    .lifetime(Lifetime::Restart)
    .help("The deformer capture's texel")
    .source(Source::Industry("Arkham Origins: <= 512² per surface; the trail ring's texel"));

/// `surface.trail.deform.range_m`: [`DeformSettings::range_m`].
pub const DEFORM_RANGE_M: Def<f32> = def!(float "surface.trail.deform.range_m" = 6.0)
    .range(2.0, 12.0)
    .unit(Unit::Meters)
    .help("Deformers this close to the player carve with their whole meshes; the rest leave foot prints")
    .source(Source::Guess);

/// `surface.trail.deform.max_actors`: [`DeformSettings::max_actors`].
pub const DEFORM_MAX_ACTORS: Def<u32> = def!(count "surface.trail.deform.max_actors" = 8)
    .range(1, 16)
    .unit(Unit::Count)
    .help("At most this many deformers (the nearest) draw into the capture")
    .source(Source::Guess);

/// `surface.trail.deform.lod_distance_m`: [`DeformSettings::lod_distance_m`].
pub const DEFORM_LOD_DISTANCE_M: Def<f32> = def!(float "surface.trail.deform.lod_distance_m" = 12.0)
    .range(0.0, 100.0)
    .unit(Unit::Meters)
    .help("Deformers draw into the capture at the LOD level they show at this distance")
    .source(Source::Guess);

/// `surface.trail.deform.slab_below_m`: [`DeformSettings::slab_below_m`].
pub const DEFORM_SLAB_BELOW_M: Def<f32> = def!(float "surface.trail.deform.slab_below_m" = 0.25)
    .range(0.05, 1.0)
    .unit(Unit::Meters)
    .help("The capture's slab starts this far under the lowest ground (anything lower carves to the ground)")
    .source(Source::Industry("Arkham Origins: an ankle-high slab"));

/// `surface.trail.deform.gap_texels`: [`DeformSettings::gap_texels`].
pub const DEFORM_GAP_TEXELS: Def<f32> = def!(float "surface.trail.deform.gap_texels" = 2.0)
    .range(1.0, 8.0)
    .unit(Unit::Texels)
    .help("A deformer that skips more than this between captures also stamps feet along its path")
    .source(Source::Guess);

/// `surface.trail.deform.ground_step_m`: [`DeformSettings::ground_step_m`]. Heightfields (a
/// terrain vertex every ~1-2 m) are bilinear between their vertices, which a 0.5 m grid follows
/// to a few millimetres.
pub const DEFORM_GROUND_STEP_M: Def<f32> = def!(float "surface.trail.deform.ground_step_m" = 0.5)
    .range(0.125, 2.0)
    .unit(Unit::Meters)
    .help("The ground under the capture is sampled on a grid this fine")
    .source(Source::Guess);

/// `surface.trail.deform.log_s`: [`DeformSettings::log_s`].
pub const DEFORM_LOG_S: Def<f32> = def!(float "surface.trail.deform.log_s" = 2.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("While the deformer capture runs, one 'surface deform:' info line this often (0: never)")
    .source(Source::Guess);

/// A root whose mesh descendants (and own mesh) carve deep cover. The host tags actors, mounts
/// and held items (S1b); mods use the same component.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Deformer {
    /// The narrowest part that should leave a continuous trench (a leg, a hoof; m): the sweep
    /// gap check's width ([`DeformGaps`]).
    pub width_m: f32,
}

/// On a mesh under a [`Deformer`] root: left out of the capture. For meshes whose vertex stage
/// doesn't place them in the world: a first-person view-model copy draws with the camera's own
/// projection, so in the capture view it would carve where nothing stands.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoDeform;

/// The ground under a world point (absolute X, Z; m) as an absolute world height (m), or `None`
/// where unknown: the host's heightfield (S1b: the terrain's heights). Absent: a plane through
/// the player's root.
#[derive(Resource, Clone)]
pub struct DeformGround(pub Arc<dyn Fn(DVec2) -> Option<f64> + Send + Sync>);

/// A root that travelled farther than its width (plus [`DeformSettings::gap_texels`]) since its
/// last capture: the host stamps feet along `from`..`to` (S1b).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeformGap {
    pub root: Entity,
    pub from: WorldPos,
    pub to: WorldPos,
}

/// This frame's sweep gaps (cleared each frame).
#[derive(Resource, Clone, Debug, Default)]
pub struct DeformGaps(pub Vec<DeformGap>);

/// The capture's settings (the tunables above).
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct DeformSettings {
    pub on: bool,
    pub window_m: f32,
    pub texel_m: f32,
    pub range_m: f32,
    pub max_actors: u32,
    pub lod_distance_m: f32,
    pub slab_below_m: f32,
    pub gap_texels: f32,
    pub ground_step_m: f32,
    pub log_s: f32,
}

impl Default for DeformSettings {
    fn default() -> Self {
        Self {
            on: DEFORM_ENABLED.default_value(),
            window_m: DEFORM_WINDOW_M.default_value(),
            texel_m: DEFORM_TEXEL_M.default_value(),
            range_m: DEFORM_RANGE_M.default_value(),
            max_actors: DEFORM_MAX_ACTORS.default_value(),
            lod_distance_m: DEFORM_LOD_DISTANCE_M.default_value(),
            slab_below_m: DEFORM_SLAB_BELOW_M.default_value(),
            gap_texels: DEFORM_GAP_TEXELS.default_value(),
            ground_step_m: DEFORM_GROUND_STEP_M.default_value(),
            log_s: DEFORM_LOG_S.default_value(),
        }
    }
}

/// This frame's capture request, both worlds.
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct DeformerFrame(pub Option<SunViewRequest>);

impl SunViewSource for DeformerFrame {
    const PASS_NAME: &'static str = "surface_deform";
    fn request(&self) -> Option<&SunViewRequest> {
        self.0.as_ref()
    }
}

/// The capture's anchor and target (main world).
#[derive(Resource, Clone, Debug)]
pub(crate) struct DeformTarget {
    pub anchor: Entity,
    pub image: Handle<Image>,
}

/// State across frames.
#[derive(Default)]
pub(crate) struct DeformState {
    /// Last frame's capture (drawn by last frame's shadow pass) and the origin it was placed in.
    last: Option<(Capture, WorldPos)>,
    /// Each captured root's position at its last capture.
    roots: HashMap<Entity, WorldPos>,
    warned_flat: bool,
    /// Real time (s) of the next `surface deform:` line.
    next_log: f64,
    /// The cover gate skipped last frame's capture (its line is logged once per skip run).
    gated: bool,
}

fn target_image() -> Image {
    let mut image = Image::new_uninit(
        Extent3d { width: MAX_CAPTURE_SIZE, height: MAX_CAPTURE_SIZE, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Depth32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    image.sampler = ImageSampler::nearest();
    image
}

/// Creates the target and the anchor (`Startup`), and lists the target in the `tex` viewer.
pub(crate) fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, textures: Option<ResMut<DebugTextures>>) {
    let anchor = commands.spawn(sf_sky_bevy::sun_view::depth_view_anchor()).id();
    let image = target_image();
    let handle = images.add(image.clone());
    if let Some(mut textures) = textures {
        textures.register(DebugTexture::image(
            "surface.deform",
            "deformer capture: depth looking up at legs, bodies and held items (reverse Z: 1 at the slab's bottom, 0 cleared), the window in the first texels",
            handle.clone(),
            &image,
        ));
    }
    commands.insert_resource(DeformTarget { anchor, image: handle });
}

/// A deformer root query item.
pub(crate) type RootItem<'a> = (Entity, &'a GlobalTransform, &'a Deformer);

/// A mesh query item: render entity, visibility, layers, LOD range.
pub(crate) type MeshItem<'a> =
    (Option<&'a RenderEntity>, &'a InheritedVisibility, Option<&'a RenderLayers>, Option<&'a VisibilityRange>);

/// The roots that draw into the capture: within `range_m` of `center` (render XZ), the nearest
/// `max_actors`, nearest first.
pub(crate) fn pick_roots<'a>(roots: impl Iterator<Item = RootItem<'a>>, center: Vec2, range_m: f32, max_actors: u32) -> Vec<(Entity, Vec3, f32)> {
    let mut near: Vec<(f32, (Entity, Vec3, f32))> = roots
        .filter_map(|(e, tf, d)| {
            let p = tf.translation();
            let dist = Vec2::new(p.x, p.z).distance(center);
            (dist <= range_m).then_some((dist, (e, p, d.width_m)))
        })
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.0.cmp(&b.1.0)));
    near.truncate(max_actors as usize);
    near.into_iter().map(|(_, r)| r).collect()
}

/// The capture's casters as `(render, main)` pairs, sorted, deduplicated (a held item tagged
/// under a tagged actor): each root and its descendants with a mesh, visible, on the camera's
/// layers, at the fixed LOD distance.
pub(crate) fn split_deformers(
    roots: &[Entity],
    descendants: impl Fn(Entity) -> Vec<Entity>,
    mesh: impl Fn(Entity) -> Option<MeshItemOwned>,
    lod_distance: f32,
    camera_layers: &RenderLayers,
) -> Vec<(Entity, MainEntity)> {
    let layer_zero = RenderLayers::default();
    let mut out = Vec::new();
    for &root in roots {
        for e in std::iter::once(root).chain(descendants(root)) {
            let Some(m) = mesh(e) else { continue };
            if !m.visible || !m.layers.as_ref().unwrap_or(&layer_zero).intersects(camera_layers) {
                continue;
            }
            if !lod_casts(m.range.as_ref(), lod_distance) {
                continue;
            }
            out.push((m.render.unwrap_or(Entity::PLACEHOLDER), MainEntity::from(e)));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// A mesh's caster facts, owned (so the split is testable without a world).
#[derive(Clone, Default)]
pub(crate) struct MeshItemOwned {
    pub render: Option<Entity>,
    pub visible: bool,
    pub layers: Option<RenderLayers>,
    pub range: Option<VisibilityRange>,
}

impl From<MeshItem<'_>> for MeshItemOwned {
    fn from((render, visible, layers, range): MeshItem<'_>) -> Self {
        Self { render: render.map(|r| **r), visible: visible.get(), layers: layers.cloned(), range: range.cloned() }
    }
}

/// The ground grid for a window whose min corner is `min_xz` (absolute world m): the hook's
/// heights (render Y) where it knows them, else `flat_y`. Returns the grid and whether any sample
/// fell back.
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub(crate) fn ground_grid(
    hook: Option<&DeformGround>,
    origin: WorldPos,
    min_xz: DVec2,
    window_m: f64,
    step_m: f64,
    flat_y: f32,
) -> (GroundGrid, bool) {
    let (first, edge) = GroundGrid::placement(min_xz, window_m, step_m);
    let o = origin.to_f64();
    let corner = (first.as_dvec2() * step_m - DVec2::new(o.x, o.z)).as_vec2();
    let Some(hook) = hook else {
        return (GroundGrid::flat(corner, step_m as f32, edge, flat_y), true);
    };
    let mut fell_back = false;
    let mut heights = Vec::with_capacity((edge * edge) as usize);
    for z in 0..edge as i32 {
        for x in 0..edge as i32 {
            let p = (first + bevy_math::IVec2::new(x, z)).as_dvec2() * step_m;
            heights.push(hook.0(p).map_or_else(
                || {
                    fell_back = true;
                    flat_y
                },
                |y| (y - o.y) as f32,
            ));
        }
    }
    (GroundGrid { corner, step_m: step_m as f32, edge, heights }, fell_back)
}

/// Whether snow or ash lies on a cover cell under the square window of half side `half_m` around
/// `center` (absolute X, Z), counting only channels with a cover look. Cells are 32 m and the
/// window at most 32 m, so its corners reach every cell it overlaps. Without the world's surface
/// state (a host without cover cells): `true`, the capture runs.
pub(crate) fn cover_under(surface: Option<&WorldSurface>, uniform: &SurfaceUniform, center: DVec2, half_m: f64) -> bool {
    let Some((surface, grid)) = surface.and_then(|s| s.grid().map(|g| (s, g))) else { return true };
    let looked = [uniform.cover[0].x > 0.0, uniform.cover[1].x > 0.0];
    [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)].into_iter().any(|(sx, sz)| {
        let cell = surface.state.cell(grid.worldspace(), sf_surface::cell_of(center + DVec2::new(sx, sz) * half_m));
        (looked[0] && cell.snow > 0.0) || (looked[1] && cell.ash > 0.0)
    })
}

/// Picks the deformers, places the window and the slab, requests this frame's capture and hands
/// last frame's to the ring pass (see the module docs). After `trail::update_trails`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value, clippy::too_many_lines, clippy::type_complexity)]
pub(crate) fn update_deform(
    settings: Res<DeformSettings>,
    origin: Res<SurfaceOrigin>,
    target: Res<DeformTarget>,
    ground_hook: Option<Res<DeformGround>>,
    time: Option<Res<Time<Real>>>,
    cameras: Query<(Entity, &Camera, &GlobalTransform, Option<&RenderLayers>), With<SurfaceWeatherCamera>>,
    roots: Query<RootItem>,
    children: Query<&Children>,
    meshes: Query<MeshItem, (With<Mesh3d>, Without<NoDeform>)>,
    cover: (Option<Res<WorldSurface>>, Res<SurfaceUniform>),
    mut trail: ResMut<TrailFrame>,
    mut frame: ResMut<DeformerFrame>,
    mut gaps: ResMut<DeformGaps>,
    mut stats: ResMut<SurfaceStats>,
    mut memory: Local<DeformState>,
    counters: Option<Res<DiagCounters>>,
) {
    frame.0 = None;
    gaps.0.clear();
    let origin = origin.0;
    // Last frame's capture is what the shadow pass drew: this frame's ring pass merges it, in the
    // render space it was placed in.
    let last = memory.last.take();
    trail.capture = last.filter(|(_, o)| *o == origin && trail.on).map(|(c, _)| (c, target.image.id()));
    if !settings.on || !trail.on {
        memory.roots.clear();
        return;
    }
    let Some((camera, cam_pos, camera_layers)) = cameras
        .iter()
        .filter(|(_, c, ..)| c.is_active)
        .min_by_key(|(_, c, ..)| c.order)
        .map(|(e, _, tf, layers)| (e, tf.translation(), layers.cloned().unwrap_or_default()))
    else {
        memory.roots.clear();
        return;
    };
    // The player: the deformer root nearest the camera.
    let cam_xz = Vec2::new(cam_pos.x, cam_pos.z);
    let Some((_, focus)) = roots
        .iter()
        .map(|(_, tf, _)| tf.translation())
        .map(|p| (Vec2::new(p.x, p.z).distance(cam_xz), p))
        .min_by(|a, b| a.0.total_cmp(&b.0))
    else {
        memory.roots.clear();
        return;
    };
    let center = Vec2::new(focus.x, focus.z);
    // No snow or ash under the window: nothing to carve, no capture (the ring still runs).
    let focus_world = origin.offset(focus).to_f64();
    if !cover_under(cover.0.as_deref(), &cover.1, DVec2::new(focus_world.x, focus_world.z), f64::from(settings.window_m) * 0.5) {
        if !memory.gated && settings.log_s > 0.0 {
            bevy_log::info!(
                "surface deform: no capture, no snow or ash on the cover cells under the window (focus {:.2} {:.2} {:.2}); foot prints only",
                focus_world.x,
                focus_world.y,
                focus_world.z
            );
        }
        memory.gated = true;
        memory.roots.clear();
        return;
    }
    memory.gated = false;
    let picked = pick_roots(roots.iter(), center, settings.range_m, settings.max_actors);
    // Sweep gaps: travel since the last capture past the width.
    let (dt, now_s) = time.map_or((0.0, 0.0), |t| (t.delta_secs(), t.elapsed_secs_f64()));
    let texel = settings.texel_m;
    let mut seen = HashMap::with_capacity(picked.len());
    for &(root, p, width) in &picked {
        let now = origin.offset(p);
        if let Some(&before) = memory.roots.get(&root) {
            let d = before.delta_f32(now);
            let travel = Vec2::new(d.x, d.z).length();
            // Per frame: no clock (headless), one frame of travel.
            let (speed, dt) = if dt > 0.0 { (travel / dt, dt) } else { (travel, 1.0) };
            if sweep_gap(speed, dt, width, texel) > settings.gap_texels {
                gaps.0.push(DeformGap { root, from: before, to: now });
            }
        }
        seen.insert(root, now);
    }
    memory.roots = seen;
    let ids: Vec<Entity> = picked.iter().map(|r| r.0).collect();
    let casters = split_deformers(
        &ids,
        |root| children.iter_descendants(root).collect(),
        |e| meshes.get(e).ok().map(MeshItemOwned::from),
        settings.lod_distance_m,
        &camera_layers,
    );
    if let Some(c) = counters.as_deref() {
        #[allow(clippy::cast_precision_loss)]
        c.add_named("surface/deform_gaps", "count", gaps.0.len() as f64);
    }
    if casters.is_empty() {
        return;
    }
    // The window: world-snapped texels around the player.
    let texel_m = f64::from(texel);
    let size = capture_size(settings.window_m, texel_m);
    let world = origin.offset(focus).to_f64();
    let min = capture_min(DVec2::new(world.x, world.z), texel_m, size);
    let o = origin.to_f64();
    let min_xz = min.as_dvec2() * texel_m;
    let corner = (min_xz - DVec2::new(o.x, o.z)).as_vec2();
    let (ground, fell_back) =
        ground_grid(ground_hook.as_deref(), origin, min_xz, f64::from(size) * texel_m, f64::from(settings.ground_step_m), focus.y);
    if fell_back && !memory.warned_flat {
        memory.warned_flat = true;
        bevy_log::warn!(
            "surface deform: no ground height hook (DeformGround) or no height under the capture; carving over a plane through the player's root (wrong on slopes)"
        );
    }
    let (near_y, range_m) = slab(ground.range(), settings.slab_below_m, trail.depth_m);
    let capture = Capture { corner, texel_m: texel, size, near_y, range_m, ground };
    let (view_from_world, clip_from_world) = capture.view();
    stats.deform_captures += 1;
    stats.deform_casters = casters.len();
    if let Some(c) = counters.as_deref() {
        c.add_named("surface/deform_captures", "count", 1.0);
        #[allow(clippy::cast_precision_loss)]
        c.add_named("surface/deform_casters", "count", casters.len() as f64);
    }
    if settings.log_s > 0.0 && now_s >= memory.next_log {
        memory.next_log = now_s + f64::from(settings.log_s);
        bevy_log::info!(
            "surface deform: focus {:.2} {:.2} {:.2} window corner {:.2} {:.2} slab {:.2}..{:.2} ground@focus {:.2} casters {} ring corner {:.2} {:.2}",
            world.x,
            world.y,
            world.z,
            corner.x,
            corner.y,
            near_y,
            near_y + range_m,
            capture.ground.at(center),
            casters.len(),
            trail.corner.x,
            trail.corner.y
        );
    }
    frame.0 = Some(SunViewRequest {
        light: target.anchor,
        camera,
        subview: DEFORM_SUBVIEW,
        image: target.image.id(),
        // The target's edge: the shadow pass draws over all of it (`Capture::view`).
        size: MAX_CAPTURE_SIZE,
        view_from_world,
        clip_from_world,
        casters,
    });
    memory.last = Some((capture, origin));
}

/// `VisibilitySystems::CheckVisibility`: this frame's capture casters are visible to a view (the
/// capture), so Bevy extracts them even when the main view doesn't see them (`shelter`'s twin).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn mark_casters_visible(frame: Res<DeformerFrame>, mut casters: Query<&mut ViewVisibility>) {
    if let Some(req) = &frame.0 {
        for (_, main) in &req.casters {
            if let Ok(mut visibility) = casters.get_mut(main.id()) {
                visibility.set_visible();
            }
        }
    }
}

/// Render world (extract): the capture view resolves every caster's LOD range at the fixed
/// distance (`RenderViewLodDistances`, vendored patch SW-5d), undithered.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn extract_lod_distance(
    frame: bevy_render::Extract<Res<DeformerFrame>>,
    settings: bevy_render::Extract<Res<DeformSettings>>,
    distances: Option<ResMut<bevy_render::view::RenderViewLodDistances>>,
) {
    let (Some(req), Some(mut distances)) = (frame.0.as_ref(), distances) else { return };
    distances.0.insert(req.retained(), settings.lod_distance_m);
}
