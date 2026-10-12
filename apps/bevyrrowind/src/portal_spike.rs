//! Scaffold `portal_spike` (`--portals`, 2026-10-12): the M8 de-risking
//! spike for seamless doors (superfargoth `docs/specs/portals.md`).
//!
//! Door links get portal frames at startup (sill frame + opening from the
//! door model's bounds, `mw_world::portal::door_portal`); the first time a
//! portal opens, both ends are grounded on their floors and share one
//! opening (`mw_world::portal::common_opening`). Near a portal-able door,
//! one portal camera renders the other worldspace from the crossed pose
//! (`DoorLink::cross`) into an HDR image, clipped at the far door plane
//! (Bevy's oblique `PerspectiveProjection::near_clip_plane`); the opening
//! shows it through a screen-space material on a thin box at the door's
//! mid-plane. The larger door slab of the pair swings open, the smaller
//! hides. Walking through the opening re-parents the camera into the other
//! worldspace with no fade ([`Gate`]: by position only, both ways).
//!
//! Exposure belongs to the viewer (portals.md "Exposure"): both cameras
//! render at one fixed base exposure ([`mw_assets::SCENE_EV100`]) into
//! linear HDR, and Bevy's `AutoExposure` on the main camera meters the
//! whole frame (opening included) and adapts. `--scheduled-exposure` keeps
//! the spike's first per-space scheduled exposure for A/B.
//!
//! Removal path: M8 replaces this module with a framework portal plugin
//! (frames baked into `LinkEnd`, N portal views, per-view look); delete this
//! file, its `.wesl`, the `--portals`/`--scheduled-exposure`/`--portal-walk`
//! flags and the `PortalSpike` hooks.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use bevy::asset::{UntypedHandle, embedded_asset};
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ClearColorConfig, Exposure, Hdr, RenderTarget};
use bevy::camera_controller::free_camera::FreeCameraState;
use bevy::light::{AmbientLight, NotShadowCaster};
use bevy::curve::cubic_splines::LinearSpline;
use bevy::pbr::{Material, MaterialPlugin};
use bevy::post_process::auto_exposure::{AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, TextureFormat, TextureUsages};
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use bevy::render::view::{DebandDither, Tonemapping};
use bevy::shader::ShaderRef;
use mw_assets::{LightCalibration, SharedVfs};
use mw_world::portal::{self, DoorShape, ModelTris};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId};
use sf_sky_bevy::sf_sky::ExposureRange;
use sf_sky_bevy::{MoonLight, SkyCamera, SunLight, TemporalHistory};
use sf_terrain_bevy::TerrainCamera;
use sf_vfs::Vfs;
use sf_world::{DoorLink, IndexedPlacement, LinkEnd, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::{
    ActivateCx, ActiveWorldspace, DoorLinks, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, StreamingSystems,
    WorldPosOf, WorldspaceRegistry, to_grid,
};

const SHADER: &str = "embedded://bevyrrowind/portal_spike.wesl";
/// A portal opens when the camera is this close to a door, meters (the
/// door prefetch radius, so the other side is resident).
const OPEN_M: f32 = 24.0;
/// The opening's box sits this far in front of the door's mid-plane. The
/// slab rests on a solid wall (no hole behind Morrowind doors), whose face
/// is half a slab behind the mid-plane, so the box stays in front of it.
const FRONT_M: f32 = 0.01;
/// The camera crosses this far in front of the opening's box: before the
/// main camera's near plane (0.1 m) reaches it.
const CROSS_MARGIN_M: f32 = 0.12;
/// The crossing plane of an end whose box sits at [`FRONT_M`].
#[cfg(test)]
pub(crate) const CROSS_AT_M: f32 = FRONT_M + CROSS_MARGIN_M;
/// A slab larger than the common opening by this much (either dimension)
/// stays closed around it (its hole would show the void around the
/// smaller opening), meters.
const STAY_MARGIN_M: f32 = 0.02;
/// The far plane clip sits this far in front of the far door plane, so
/// geometry on the plane (the frame around the opening) stays.
const CLIP_BIAS_M: f32 = 0.02;
/// A door slab whose model origin is this close to an edge swings on it.
const HINGE_MIN: f32 = 0.35;
/// A door whose opening is within this distance opens whatever the camera
/// faces (walking backwards, strafing); farther ones must lie within ~50
/// degrees of the view direction.
const NEAR_M: f32 = 2.0;
const MIN_FACING: f32 = 0.65;
/// Seconds to swing a door fully open.
const SWING_SECS: f32 = 0.5;
/// Frames logged after a crossing (`portal trace`).
const TRACE_FRAMES: u32 = 6;
/// Frames are also logged while the camera is this close in front of the
/// crossing plane (the frames just before a crossing).
const TRACE_BEFORE_M: f32 = 0.25;

/// What a portal link's door does while its portal is open.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DoorMove {
    /// Swings open by this many radians about its origin's vertical axis.
    Swing(f32),
    Hide,
    /// Stays closed around a smaller common opening (its box sits on the
    /// slab's front face); hidden while the camera stands behind that face.
    Stay,
}

/// A door of a portal link and what it does.
#[derive(Clone, Copy, Debug)]
pub struct DoorPlan {
    pub id: PlacementId,
    pub action: DoorMove,
}

/// A door link rendered as a portal: its index in `DoorLinks`, the plans
/// for its `a` and `b` doors, each end's cell and slab frame + opening (as
/// derived, before grounding).
#[derive(Clone, Debug)]
pub struct SpikeLink {
    pub link: usize,
    pub doors: [DoorPlan; 2],
    pub cells: [RecordId; 2],
    pub slab: [(WorldTransform, Vec2); 2],
    /// The opening's box offset in front of each end's mid-plane.
    pub front: [f32; 2],
    /// Both ends grounded on their floors (done when the portal first opens).
    pub grounded: bool,
}

/// The links the spike renders as portals, the solids of their cells (for
/// grounding) and the spike's options.
#[derive(Resource, Clone, Debug, Default)]
pub struct PortalLinks {
    pub links: Vec<SpikeLink>,
    pub solids: HashMap<RecordId, Arc<Vec<(String, WorldTransform)>>>,
    /// `--scheduled-exposure`: per-space scheduled exposure (the spike's
    /// first version) instead of the viewer's `AutoExposure`.
    pub scheduled_exposure: bool,
    /// `--portal-walk`: walk the camera forward through the door in front
    /// and back, twice, logging each crossing (test scene `portal-walk`).
    pub walk: bool,
}

/// Fills `frame` and `extent` of every two-way link whose doors are both
/// upright (the bake's next step, world-adapter.md "Portal frames:
/// feasibility", done at startup here) with the pair's common opening (the
/// per-dimension minimum; grounded later, when the portal first opens).
pub fn fill_frames(links: &mut [DoorLink], db: &RecordDb, vfs: &Vfs) -> PortalLinks {
    let started = Instant::now();
    let (ends, _) = mw_world::worldspaces::door_ends(db);
    let by_id: HashMap<PlacementId, &mw_world::worldspaces::DoorEnd> = ends.iter().map(|e| (e.id, e)).collect();
    let models = portal::door_models(db);
    let bounds = portal::load_bounds(vfs, &models);
    let mut out = Vec::new();
    for (i, link) in links.iter_mut().enumerate() {
        if !link.two_way {
            continue;
        }
        let (Some(da), Some(db_)) = (link.a.door.and_then(|d| by_id.get(&d)), link.b.door.and_then(|d| by_id.get(&d))) else {
            continue;
        };
        let geometry = |end: &mw_world::worldspaces::DoorEnd, landing: WorldPos| {
            let m = models.get(&end.id);
            let b = m.and_then(|m| m.model.as_deref()).and_then(|p| bounds.get(p).copied());
            portal::door_portal(&end.pose, m.map_or(1.0, |m| m.scale), b, Some(landing))
        };
        let (ga, gb) = (geometry(da, link.a.arrival.pos), geometry(db_, link.b.arrival.pos));
        let upright = |s: DoorShape| matches!(s, DoorShape::Slab | DoorShape::Thick);
        let ratio = (ga.extent / gb.extent.max(Vec2::splat(1e-3))).max(gb.extent / ga.extent.max(Vec2::splat(1e-3))).max_element();
        if !upright(ga.shape) || !upright(gb.shape) || ratio > portal::ADJUST_MAX_RATIO {
            continue;
        }
        link.a.frame = ga.frame;
        link.a.extent = ga.extent;
        link.b.frame = gb.frame;
        link.b.extent = gb.extent;
        portal::common_opening(&mut link.a, &mut link.b, [None, None]);
        // The larger slab is kept, the smaller hides, so one door shows on
        // either side of the crossing. The kept one swings into its own
        // space when it is the opening's size; when it is bigger, it stays
        // closed around the common opening (its wall hole is the slab's
        // size: open, the margin would show the void around the opening).
        let a_kept = ga.extent.x * ga.extent.y >= gb.extent.x * gb.extent.y;
        let common = link.a.extent;
        let plan = |end: &mw_world::worldspaces::DoorEnd, g: &portal::DoorFrame, kept: bool| {
            let bigger = (g.extent - common).max_element() > STAY_MARGIN_M;
            let action = if !kept {
                DoorMove::Hide
            } else if bigger {
                DoorMove::Stay
            } else if g.shape == DoorShape::Slab && g.hinge.abs() > HINGE_MIN {
                DoorMove::Swing(swing_angle(&end.pose, g))
            } else {
                DoorMove::Hide
            };
            DoorPlan { id: end.id, action }
        };
        let (pa, pb) = (plan(da, &ga, a_kept), plan(db_, &gb, !a_kept));
        let front = |p: &DoorPlan, g: &portal::DoorFrame| {
            if p.action == DoorMove::Stay { (g.thickness * 0.5).min(0.15) + FRONT_M } else { FRONT_M }
        };
        out.push(SpikeLink {
            link: i,
            doors: [pa, pb],
            front: [front(&pa, &ga), front(&pb, &gb)],
            cells: [da.cell.clone(), db_.cell.clone()],
            slab: [(ga.frame, ga.extent), (gb.frame, gb.extent)],
            grounded: false,
        });
    }
    let mut solids = HashMap::new();
    for cell in out.iter().flat_map(|s| s.cells.iter()) {
        solids.entry(cell.clone()).or_insert_with(|| Arc::new(portal::cell_solids(db, cell)));
    }
    println!(
        "portal spike: {} portal links of {} ({} cells' solids) in {:.2} s",
        out.len(),
        links.len(),
        solids.len(),
        started.elapsed().as_secs_f32()
    );
    PortalLinks { links: out, solids, scheduled_exposure: false, walk: false }
}

/// Environment toggle for seamless portals, besides `--portals`.
pub const ENV: &str = "BEVYRROWIND_PORTALS";

/// Seamless portals are an optional setting, off by default (classic
/// fade-teleport doors; owner decision 2026-10-12, parked until M8): on with
/// `--portals` or `BEVYRROWIND_PORTALS=1` (`on`, `true`, `yes`).
pub fn enabled(flag: bool) -> bool {
    flag || std::env::var(ENV).is_ok_and(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "on" | "true" | "yes"))
}

/// The quarter turn about the door's origin that moves its slab into the
/// front (+Z) side of its frame.
fn swing_angle(pose: &WorldTransform, g: &portal::DoorFrame) -> f32 {
    let mut c = pose.pos.delta_f32(g.frame.pos.offset(g.frame.rotation * Vec3::Y * g.extent.y * 0.5));
    c.y = 0.0;
    let n = g.frame.rotation * Vec3::Z;
    let turn = std::f32::consts::FRAC_PI_2;
    if (Quat::from_rotation_y(turn) * c).dot(n) >= (Quat::from_rotation_y(-turn) * c).dot(n) { turn } else { -turn }
}

/// The opening shows the portal camera's image at screen-space UVs.
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct PortalMaterial {
    #[texture(0)]
    #[sampler(1)]
    view: Handle<Image>,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
    fn enable_shadows() -> bool {
        false
    }
}

/// The root entity of a door placement (spawned by [`DoorTagSpawner`]),
/// with its spawn transform.
#[derive(Component, Clone, Copy, Debug)]
pub struct PortalDoor {
    pub id: PlacementId,
    base: Transform,
}

/// Puts each door placement under a [`PortalDoor`] root, so the spike can
/// swing or hide it; otherwise the wrapped spawner's.
struct DoorTagSpawner(Arc<dyn PlacementSpawner>);

impl PlacementSpawner for DoorTagSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        if placement.placement.class != PlacementClass::Door {
            return self.0.spawn(world, cx, placement);
        }
        let tag = world
            .spawn((PortalDoor { id: placement.placement.id, base: cx.local }, cx.local, Visibility::Inherited, ChildOf(cx.parent)))
            .id();
        let inner = SpawnCx { parent: tag, local: Transform::IDENTITY, ..*cx };
        1 + self.0.spawn(world, &inner, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
    }
}

#[derive(Component)]
struct PortalCamera;

#[derive(Component)]
struct PortalBox;

/// The crossing rule, by the camera's position only (not its facing or
/// velocity): the camera crosses when its local z in the open end's frame
/// goes from at or above `trigger` to below it, inside the opening. The
/// trigger is the end's approach plane (box + 0.12 m); right after a crossing it is
/// the same physical plane seen from the other side (`-trigger`), so
/// stepping straight back crosses back, and the arrival is never already
/// past it. It returns to the approach plane once the camera stands beyond.
/// No cooldown.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Gate {
    /// The camera's side is the link's `a` end.
    pub from_a: bool,
    pub trigger: f32,
    /// The camera's local z last frame.
    pub last_z: Option<f32>,
    /// Each end's approach trigger (its box offset + [`CROSS_MARGIN_M`]).
    pub approach: [f32; 2],
}

impl Gate {
    pub(crate) fn new(from_a: bool, front: [f32; 2]) -> Self {
        let approach = front.map(|f| f + CROSS_MARGIN_M);
        Self { from_a, trigger: approach[usize::from(!from_a)], last_z: None, approach }
    }

    /// Advances one frame with the camera's pose (in the camera side's
    /// worldspace): the crossed pose when it crossed (the gate then faces
    /// the other end).
    pub(crate) fn step(&mut self, link: &DoorLink, pose: &WorldTransform) -> Option<WorldTransform> {
        let (from, to) = link_ends(link, self.from_a);
        let p = local_in(from, pose.pos);
        let inside = p.x.abs() <= from.extent.x * 0.5 && p.y > -0.5 && p.y < from.extent.y + 0.5;
        if inside && p.z < self.trigger && self.last_z.is_some_and(|z| z >= self.trigger) {
            self.from_a = !self.from_a;
            self.trigger = -self.trigger;
            self.last_z = Some(-p.z);
            return Some(DoorLink::cross(from, to, pose));
        }
        let approach = self.approach[usize::from(!self.from_a)];
        if p.z >= approach {
            self.trigger = approach;
        }
        self.last_z = Some(p.z);
        None
    }
}

/// The portal now open: link (index into [`PortalLinks`]) and its gate.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Open {
    spike: usize,
    gate: Gate,
}

#[derive(Resource)]
struct PortalState {
    camera: Entity,
    portal_box: Entity,
    image: Handle<Image>,
    material: Handle<PortalMaterial>,
    open: Option<Open>,
    /// 0 (closed) to 1 (open) for the door swing.
    openness: f32,
    /// The worldspace whose look the portal camera has.
    look: Option<WorldspaceId>,
    crossings: u32,
    /// The `Stay` door the camera stands behind (hidden).
    behind: Option<PlacementId>,
    /// Frames still to log after the last crossing.
    trace: u32,
}

/// The viewer's eye adaptation: compensation curve for `AutoExposure`.
#[derive(Resource)]
struct EyeAdaptation(Handle<AutoExposureCompensationCurve>);

pub fn add(app: &mut App, links: PortalLinks) {
    embedded_asset!(app, "portal_spike.wesl");
    let auto = !links.scheduled_exposure;
    let walk = links.walk;
    app.add_plugins(MaterialPlugin::<PortalMaterial>::default())
        .insert_resource(links)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, tag_doors)
        .add_systems(
            Update,
            (drive, (look, swing_doors)).chain().after(StreamingSystems::Transition).before(crate::doors::worldspace_look),
        )
        .add_systems(Update, sun_layers.after(crate::doors::worldspace_look))
        .add_systems(Last, trace);
    if auto {
        app.add_plugins(AutoExposurePlugin)
            .add_systems(Update, exposure_keys)
            .add_systems(Last, viewer_exposure.before(trace));
    }
    app.insert_resource(ExposureStrength(DEFAULT_EXPOSURE_STRENGTH))
        .add_systems(Update, terrain_camera.after(crate::doors::worldspace_look));
    if walk {
        app.add_systems(Update, walk_through.before(drive));
    }
}

fn tag_doors(mut spawners: ResMut<PlacementSpawners>) {
    let wrap = |s: &Arc<dyn PlacementSpawner>| -> Arc<dyn PlacementSpawner> { Arc::new(DoorTagSpawner(s.clone())) };
    if let Some(s) = spawners.for_class(PlacementClass::Door).map(wrap) {
        spawners.set(PlacementClass::Door, s);
    }
    let uncooked = spawners.uncooked().map(wrap);
    spawners.set_uncooked(uncooked);
}

fn target_image(images: &mut Assets<Image>, size: UVec2) -> Handle<Image> {
    images.add(Image::new_target_texture(size.x.max(1), size.y.max(1), TextureFormat::Rgba16Float, None))
}

/// Exposure compensation (EV) by the metered average, Bevy's `AutoExposure`
/// axis: log2 of the average linear HDR value at the base exposure
/// [`mw_assets::SCENE_EV100`], so scene EV = x + 7. Full adaptation would
/// map every average to the same grey; the curve keeps a dark scene darker
/// (partial adaptation, as UE's exposure compensation curve and the
/// schedule this replaces): night street (x <= -11) -4.5, lit interior
/// (x = -3) -3.6, daylight (x >= 2) -2.3 (mid grey). Values match the
/// sky schedule's EV100 (noon ~14, night ~0.5) and the interior EV100 7.
const COMPENSATION: [(f32, f32); 5] = [(-16.0, -4.5), (-11.0, -4.5), (-3.0, -3.6), (2.0, -2.3), (10.0, -2.3)];
/// The lit interior's point of [`COMPENSATION`]: the anchor of the strength.
const INTERIOR_X: f32 = -3.0;
const INTERIOR_COMPENSATION: f32 = -3.6;
/// Default adaptation strength (portal3, owner: portal2's 1.0 "too
/// intense"); live keys `-` / `=` change it by 0.1 (0 to 1.5).
pub const DEFAULT_EXPOSURE_STRENGTH: f32 = 0.7;
/// Adaptation speeds, EV/s (portal2: 6 / 4, softened): into a brighter
/// scene, into a darker one.
const SPEED_BRIGHTEN: f32 = 3.0;
const SPEED_DARKEN: f32 = 2.0;

/// How strongly the viewer's eye adapts between scenes (`--portals`): 1 is
/// [`COMPENSATION`] (portal2), 0 no adaptation away from a lit interior's
/// exposure. The exposure correction (relative to the base) at each metered
/// average is scaled about the interior's: `E_s = E_int + s (E - E_int)`,
/// with `E = compensation - x` (Bevy's target). Lower values keep the
/// doorway's swing smaller (inside looking out the street blows out less,
/// from the street the office is less dark), at the cost of brighter days
/// and darker nights than the sky schedule (UE's min/max EV limits make the
/// same trade). Live keys `-` / `=`.
#[derive(Resource, Clone, Copy, Debug)]
pub struct ExposureStrength(pub f32);

/// The curve sampled every half EV and kept inside the exposure range
/// (`LightCalibration::exposure_range`: the night floor, sky-lighting.md
/// "Exposure range"), UE's Min/Max EV100 on Bevy's curve.
fn compensation_points(strength: f32, range: ExposureRange) -> Vec<Vec2> {
    let e_int = INTERIOR_COMPENSATION - INTERIOR_X;
    let scaled = COMPENSATION.map(|(x, c)| Vec2::new(x, e_int + strength * ((c - x) - e_int) + x));
    let (lo, hi) = (scaled[0].x, scaled[scaled.len() - 1].x);
    std::iter::successors(Some(lo), |x| Some(x + 0.5))
        .take_while(|x| *x <= hi + 1e-3)
        .map(|x| Vec2::new(x, range.clamp_compensation(mw_assets::SCENE_EV100, x, sample(&scaled, x))))
        .collect()
}

/// A piecewise-linear curve's value at `x` (flat past its ends).
fn sample(points: &[Vec2], x: f32) -> f32 {
    points
        .windows(2)
        .find(|w| x <= w[1].x)
        .map_or(points[points.len() - 1].y, |w| w[0].y + (w[1].y - w[0].y) * ((x - w[0].x) / (w[1].x - w[0].x)).clamp(0.0, 1.0))
}

/// The exposure range of the lighting calibration (default without one).
fn exposure_range(calibration: Option<&LightCalibration>) -> ExposureRange {
    calibration.copied().unwrap_or_default().exposure_range()
}

fn compensation_curve(strength: f32, range: ExposureRange) -> Option<AutoExposureCompensationCurve> {
    AutoExposureCompensationCurve::from_curve(LinearSpline::new(compensation_points(strength, range)))
        .map_err(|e| warn!("portal spike: compensation curve: {e:?}"))
        .ok()
}

/// `-` / `=`: adaptation strength down / up by 0.1; prints the value and
/// the resulting EV100 of a night street, a lit interior and noon.
fn exposure_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut strength: ResMut<ExposureStrength>,
    adaptation: Option<Res<EyeAdaptation>>,
    mut curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    calibration: Option<Res<LightCalibration>>,
) {
    let range = exposure_range(calibration.as_deref());
    let delta = if input.just_pressed(KeyCode::Equal) {
        0.1
    } else if input.just_pressed(KeyCode::Minus) {
        -0.1
    } else {
        return;
    };
    strength.0 = ((strength.0 + delta) * 10.0).round().clamp(0.0, 15.0) / 10.0;
    if let (Some(a), Some(curve)) = (adaptation, compensation_curve(strength.0, range))
        && let Some(mut c) = curves.get_mut(&a.0)
    {
        *c = curve;
    }
    // Steady-state EV100 = base - (compensation - x), at the curve's
    // night (-11), interior (-3) and daylight (5, noon) points.
    let points = compensation_points(strength.0, range);
    let ev = |x: f32| mw_assets::SCENE_EV100 - (sample(&points, x) - x);
    let msg = format!(
        "portal exposure strength {:.1} (default {DEFAULT_EXPOSURE_STRENGTH}): EV100 night {:.1}, interior {:.1}, noon {:.1}",
        strength.0,
        ev(-11.0),
        ev(INTERIOR_X),
        ev(5.0)
    );
    info!("{msg}");
    println!("{msg}");
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PortalMaterial>>,
    curves: Option<ResMut<Assets<AutoExposureCompensationCurve>>>,
    strength: Option<Res<ExposureStrength>>,
    calibration: Option<Res<LightCalibration>>,
) {
    let image = target_image(&mut images, UVec2::ONE);
    let material = materials.add(PortalMaterial { view: image.clone() });
    let portal_box = commands
        .spawn((PortalBox, Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))), MeshMaterial3d(material.clone()), NotShadowCaster, Visibility::Hidden))
        .id();
    // Linear HDR out, no tonemapping or dithering: the main camera tonemaps
    // (and adapts) the opening together with the rest of its frame.
    let camera = commands
        .spawn((
            PortalCamera,
            // The atmosphere pass samples depth when the far side is the exterior
            // (Bevy sets this only when a camera's `Camera3d` changes).
            Camera3d {
                depth_texture_usages: (TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING).into(),
                ..Camera3d::default()
            },
            Camera { order: -1, is_active: false, ..Camera::default() },
            Hdr,
            Tonemapping::None,
            DebandDither::Disabled,
            // Sampled per pixel by the opening; the atmosphere pass (when the
            // far side is the exterior) also needs single-sampled targets.
            Msaa::Off,
            Exposure { ev100: mw_assets::SCENE_EV100 },
            RenderTarget::Image(image.clone().into()),
            Visibility::Visible,
        ))
        .id();
    if let Some(mut curves) = curves
        && let Some(curve) = compensation_curve(strength.map_or(DEFAULT_EXPOSURE_STRENGTH, |s| s.0), exposure_range(calibration.as_deref()))
    {
        commands.insert_resource(EyeAdaptation(curves.add(curve)));
    }
    commands.insert_resource(PortalState {
        camera,
        portal_box,
        image,
        material,
        open: None,
        openness: 0.0,
        look: None,
        crossings: 0,
        behind: None,
        trace: 0,
    });
}

/// A link's ends: (camera side, far side).
fn link_ends(link: &DoorLink, from_a: bool) -> (&LinkEnd, &LinkEnd) {
    if from_a { (&link.a, &link.b) } else { (&link.b, &link.a) }
}

/// The ends of a spike link: (camera side, far side).
fn ends<'a>(links: &'a [DoorLink], spike: &SpikeLink, from_a: bool) -> (&'a LinkEnd, &'a LinkEnd) {
    link_ends(&links[spike.link], from_a)
}

/// The camera's position in a link end's frame (x across, y up from the
/// sill, z out of the front).
fn local_in(end: &LinkEnd, pos: WorldPos) -> Vec3 {
    end.frame.rotation.inverse() * end.frame.pos.delta_f32(pos)
}

/// Distance from a local position to the opening's rectangle.
fn to_opening(end: &LinkEnd, p: Vec3) -> f32 {
    let half = end.extent.x * 0.5;
    Vec3::new(p.x - p.x.clamp(-half, half), p.y - p.y.clamp(0.0, end.extent.y), p.z).length()
}

/// Grounds both ends of a spike link on their floors and gives them the
/// common opening (`mw_world::portal::common_opening`), once.
fn ground(spike: &mut SpikeLink, link: &mut DoorLink, solids: &HashMap<RecordId, Arc<Vec<(String, WorldTransform)>>>, vfs: &Vfs, cache: &mut HashMap<String, Option<Arc<ModelTris>>>) {
    let started = Instant::now();
    let mut floor = [None, None];
    for (i, end) in [&mut link.a, &mut link.b].into_iter().enumerate() {
        (end.frame, end.extent) = spike.slab[i];
        let Some(solids) = solids.get(&spike.cells[i]) else { continue };
        floor[i] = portal::floor_offset(&end.frame, solids, |path| {
            cache.entry(path.to_owned()).or_insert_with(|| ModelTris::load(vfs, path).map(Arc::new)).clone()
        });
    }
    portal::common_opening(&mut link.a, &mut link.b, floor);
    spike.grounded = true;
    info!(
        "portal spike: grounded {} / {}: floor {:?} / {:?} m above the slab sills, slabs {:.2?} / {:.2?}, opening {:.2} x {:.2} m ({:.0} ms)",
        link.a.worldspace.as_str(),
        link.b.worldspace.as_str(),
        floor[0],
        floor[1],
        spike.slab[0].1,
        spike.slab[1].1,
        link.a.extent.x,
        link.a.extent.y,
        started.elapsed().as_secs_f64() * 1000.0
    );
}

/// Crosses the open portal ([`Gate`]); opens the nearest portal in front
/// (grounding it the first time); places the portal camera (crossed pose,
/// oblique clip at the far plane) and the opening's box.
#[allow(clippy::too_many_arguments, clippy::type_complexity, clippy::too_many_lines, clippy::many_single_char_names)]
fn drive(
    mut commands: Commands,
    mut state: ResMut<PortalState>,
    (mut spikes, mut links, vfs, mut history): (ResMut<PortalLinks>, ResMut<DoorLinks>, Option<Res<SharedVfs>>, Option<ResMut<TemporalHistory>>),
    mut cache: Local<HashMap<String, Option<Arc<ModelTris>>>>,
    mut registry: ResMut<WorldspaceRegistry>,
    mut active: ResMut<ActiveWorldspace>,
    positions: WorldPosOf,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<PortalMaterial>>,
    mut main: Query<(Entity, &Camera, &Transform, &Projection, Option<&mut FreeCameraState>), (With<crate::doors::DoorUser>, Without<PortalCamera>)>,
    mut portal_cam: Query<(&mut Camera, &mut Projection, &mut RenderTarget), (With<PortalCamera>, Without<crate::doors::DoorUser>)>,
) {
    let Ok((cam, camera, transform, projection, free)) = main.single_mut() else { return };
    let Some(mut here) = positions.location(cam) else { return };
    let mut rotation = transform.rotation;

    // 1. Crossing: by position against the open portal's gate.
    if let Some(mut open) = state.open {
        let link = &links.0[spikes.links[open.spike].link];
        let (from, to) = link_ends(link, open.gate.from_a);
        if from.worldspace == here.worldspace
            && let Some(out) = open.gate.step(link, &WorldTransform { pos: here.pos, rotation, scale: Vec3::ONE })
        {
            let entry = registry.ensure(&mut commands, &to.worldspace);
            let (cell, mut t) = to_grid(out.pos);
            t.rotation = out.rotation;
            commands.entity(cam).insert((ChildOf(entry.root), cell, t, RenderLayers::from_layers(&[0, entry.layer])));
            active.0 = Some(to.worldspace.clone());
            // The free camera keeps flying: its cached yaw/pitch turn with
            // the crossing and so does its velocity (`TravelArrived` would
            // zero it, as a fade wants).
            if let Some(mut f) = free {
                let (yaw, pitch, _) = out.rotation.to_euler(EulerRot::YXZ);
                f.yaw = yaw;
                f.pitch = pitch;
                f.velocity = DoorLink::cross_direction(from, to, f.velocity);
                f.rotation_curve = None;
            }
            // A camera cut for temporal upscalers (DLSS/TAA): the history
            // and motion vectors belong to the other worldspace.
            if let Some(h) = history.as_mut() {
                h.reset = true;
            }
            state.crossings += 1;
            state.trace = TRACE_FRAMES;
            info!(
                "portal spike: crossed {} -> {} (#{}), arrival local z {:.3}, return trigger {:.2}",
                from.worldspace.as_str(),
                to.worldspace.as_str(),
                state.crossings,
                open.gate.last_z.unwrap_or_default(),
                open.gate.trigger
            );
            here = sf_world::Location::new(to.worldspace.clone(), out.pos);
            rotation = out.rotation;
        } else if open.gate.last_z.is_some_and(|z| z >= open.gate.trigger && z < open.gate.trigger + TRACE_BEFORE_M) {
            // The frames just before a crossing, for comparison.
            state.trace = state.trace.max(1);
        }
        state.open = Some(open);
    }

    // 2. The open portal stays while the camera is near and not behind its
    //    gate; otherwise the nearest portal in front (near ones whatever the
    //    camera faces, so walking backwards or strafing opens them too).
    let keep = state.open.filter(|o| {
        let (from, _) = ends(&links.0, &spikes.links[o.spike], o.gate.from_a);
        from.worldspace == here.worldspace && {
            let p = local_in(from, here.pos);
            p.z > o.gate.trigger.min(0.0) - 0.05 && p.length() < OPEN_M
        }
    });
    let mut best: Option<(f32, Open)> = None;
    if keep.is_none() {
        for (spike, s) in spikes.links.iter().enumerate() {
            for from_a in [true, false] {
                let (from, _) = ends(&links.0, s, from_a);
                if from.worldspace != here.worldspace {
                    continue;
                }
                let p = local_in(from, here.pos);
                let d = to_opening(from, p);
                let center = from.frame.pos.offset(from.frame.rotation * Vec3::Y * from.extent.y * 0.5);
                let to_door = here.pos.delta_f32(center);
                let facing = (rotation * Vec3::NEG_Z).dot(to_door / to_door.length().max(1e-3));
                if d < OPEN_M && p.z > 0.0 && (d < NEAR_M || facing > MIN_FACING) && best.is_none_or(|(b, _)| d < b) {
                    let mut gate = Gate::new(from_a, s.front);
                    gate.last_z = Some(p.z);
                    best = Some((d, Open { spike, gate }));
                }
            }
        }
    }
    let open = keep.or(best.map(|(_, o)| o));
    if let Some(o) = open
        && state.open.is_none_or(|s| s.spike != o.spike || s.gate.from_a != o.gate.from_a)
    {
        if !spikes.links[o.spike].grounded
            && let Some(vfs) = &vfs
        {
            let spikes = &mut *spikes;
            let spike = &mut spikes.links[o.spike];
            ground(spike, &mut links.0[spike.link], &spikes.solids, &vfs.0, &mut cache);
        }
        let (from, to) = ends(&links.0, &spikes.links[o.spike], o.gate.from_a);
        info!(
            "portal spike: open {} -> {} (opening {:.2} x {:.2} m) sill {:.2?} camera local {:.2?}",
            from.worldspace.as_str(),
            to.worldspace.as_str(),
            from.extent.x,
            from.extent.y,
            from.frame.pos.to_f64(),
            local_in(from, here.pos)
        );
    }
    state.open = open;
    let Ok((mut pcam, mut pproj, mut target)) = portal_cam.single_mut() else { return };
    let (from, to) = match open {
        Some(o) => ends(&links.0, &spikes.links[o.spike], o.gate.from_a),
        None => (&links.0[0].a, &links.0[0].b),
    };
    // Only in front of the opening's box: right after arriving, the camera
    // stands just behind the far door's plane (the gate's band).
    let front = open.map_or(FRONT_M, |o| spikes.links[o.spike].front[usize::from(!o.gate.from_a)]);
    let z = local_in(from, here.pos).z;
    // A closed (`Stay`) door hides while the camera stands behind its face.
    state.behind = open.filter(|_| z < front).map(|o| spikes.links[o.spike].doors[usize::from(!o.gate.from_a)].id);
    let in_front = open.is_some() && z > front + 0.02;
    if !in_front {
        if pcam.is_active {
            pcam.is_active = false;
            commands.entity(state.portal_box).insert(Visibility::Hidden);
        }
        return;
    }

    // 3. The portal camera: the crossed pose in the far worldspace.
    let virt = DoorLink::cross(from, to, &WorldTransform { pos: here.pos, rotation, scale: Vec3::ONE });
    let to_entry = registry.ensure(&mut commands, &to.worldspace);
    let (cell, mut t) = to_grid(virt.pos);
    t.rotation = virt.rotation;
    commands.entity(state.camera).insert((ChildOf(to_entry.root), cell, t, RenderLayers::layer(to_entry.layer)));
    // Oblique near plane on the far door plane (Lengyel 2005, as Bevy's
    // `mirror` example): nothing behind the far door draws.
    let n_world = to.frame.rotation * Vec3::Z;
    let view_from_world = virt.rotation.inverse();
    let n = view_from_world * n_world;
    let far_front = open.map_or(FRONT_M, |o| spikes.links[o.spike].front[usize::from(o.gate.from_a)]);
    // In front of a closed far slab's face (it would block the view), else
    // just behind the mid-plane so the frame around the opening stays.
    let clip = if far_front > FRONT_M { far_front - 0.005 } else { -CLIP_BIAS_M };
    let q = view_from_world * virt.pos.delta_f32(to.frame.pos.offset(n_world * clip));
    if let Projection::Perspective(main_p) = projection {
        let mut p = main_p.clone();
        // Behind or on the far plane the oblique matrix degenerates: keep
        // the plain near plane for that frame.
        if n.dot(q) > 1e-3 {
            p.near_clip_plane = n.extend(-n.dot(q));
        }
        *pproj = Projection::Perspective(p);
    }
    pcam.is_active = true;
    // Same pixel grid as the main view.
    if let Some(size) = camera.physical_target_size()
        && images.get(&state.image).is_none_or(|i| i.size() != size)
    {
        state.image = target_image(&mut images, size);
        *target = RenderTarget::Image(state.image.clone().into());
        if let Some(mut m) = materials.get_mut(&state.material) {
            m.view = state.image.clone();
        }
    }

    // 4. The opening: a thin box just in front of the door's mid-plane,
    //    the size of the common opening, standing on the grounded sill.
    let from_entry = registry.ensure(&mut commands, &from.worldspace);
    let (cell, mut t) = to_grid(from.frame.pos);
    t.rotation = from.frame.rotation;
    t.translation += from.frame.rotation * Vec3::new(0.0, from.extent.y * 0.5, front);
    t.scale = from.extent.extend(0.002);
    commands
        .entity(state.portal_box)
        .insert((ChildOf(from_entry.root), cell, t, RenderLayers::layer(from_entry.layer), Visibility::Visible));
}

/// The portal camera's look follows its worldspace: an interior's ambient
/// and fog color, or the sky's (`SkyCamera`: ambient, environment light;
/// its scheduled exposure only with `--scheduled-exposure`).
fn look(
    mut commands: Commands,
    mut state: ResMut<PortalState>,
    spikes: Res<PortalLinks>,
    links: Res<DoorLinks>,
    envs: Res<crate::doors::Environments>,
    calibration: Res<LightCalibration>,
    mut cams: Query<&mut Camera, With<PortalCamera>>,
) {
    let Some(open) = state.open else { return };
    let (_, to) = ends(&links.0, &spikes.links[open.spike], open.gate.from_a);
    if state.look.as_ref() == Some(&to.worldspace) {
        return;
    }
    let Ok(mut camera) = cams.get_mut(state.camera) else { return };
    state.look = Some(to.worldspace.clone());
    let mut e = commands.entity(state.camera);
    if let Some((ambient, clear)) = crate::doors::interior_look(envs.0.get(&to.worldspace), &calibration) {
        e.remove::<SkyCamera>().insert((ambient, Exposure { ev100: mw_assets::SCENE_EV100 }));
        camera.clear_color = ClearColorConfig::Custom(clear);
    } else {
        e.remove::<AmbientLight>().insert(SkyCamera { exposure: spikes.scheduled_exposure, ..SkyCamera::default() });
        camera.clear_color = ClearColorConfig::Default;
    }
}

/// The terrain streams and draws around the portal camera while it looks
/// out at the exterior from another worldspace (the main camera drops
/// `TerrainCamera` indoors); the terrain's render origin follows it into
/// the exterior root (`exterior::sync_origin`). Owner finding, portal3.
fn terrain_camera(
    mut commands: Commands,
    state: Res<PortalState>,
    active: Res<ActiveWorldspace>,
    cams: Query<Has<TerrainCamera>, With<PortalCamera>>,
) {
    let exterior = mw_world::ids::exterior();
    let want = active.0.as_ref().is_some_and(|ws| *ws != exterior) && state.look.as_ref() == Some(&exterior);
    let Ok(has) = cams.get(state.camera) else { return };
    if want && !has {
        commands.entity(state.camera).insert(TerrainCamera);
    } else if !want && has {
        commands.entity(state.camera).remove::<TerrainCamera>();
    }
}

/// Viewer exposure (default): the main and portal cameras keep the base
/// exposure in every worldspace (one physical light scale, no per-space
/// switch at a crossing), and the main camera's `AutoExposure` adapts the
/// whole frame. Runs in `Last`, after the sky and the worldspace look wrote
/// their exposures, so the rendered frame always has the base.
#[allow(clippy::type_complexity)]
fn viewer_exposure(
    mut commands: Commands,
    curve: Option<Res<EyeAdaptation>>,
    mut cams: Query<
        (Entity, &mut Exposure, Option<&mut SkyCamera>, Has<AutoExposure>, Has<crate::doors::DoorUser>),
        Or<(With<crate::doors::DoorUser>, With<PortalCamera>)>,
    >,
) {
    for (entity, mut exposure, sky, has_auto, main) in &mut cams {
        if exposure.ev100 != mw_assets::SCENE_EV100 {
            exposure.ev100 = mw_assets::SCENE_EV100;
        }
        if let Some(mut sky) = sky
            && sky.exposure
        {
            sky.exposure = false;
        }
        if main
            && !has_auto
            && let Some(curve) = &curve
        {
            commands.entity(entity).insert(AutoExposure {
                range: -16.0..=10.0,
                // Light adaptation (into a brighter scene) is quicker than
                // dark adaptation; ~1.5-2.5 s for a noon doorway's swing.
                speed_brighten: SPEED_BRIGHTEN,
                speed_darken: SPEED_DARKEN,
                compensation_curve: curve.0.clone(),
                ..AutoExposure::default()
            });
        }
    }
}

/// `portal trace`: per frame after a crossing, what decides the frame's
/// look: active worldspace, the main camera's exposure, sky camera,
/// adaptation, clear color, sun layers and illuminance.
/// Also: the main camera's render-space position, terrain/cloud camera,
/// the portal camera and box, and (both pipelines) the presented frame's
/// mean sRGB luma with its white (> 245) and black (< 6) pixel shares, read
/// back from a screenshot of that frame (`portal luma`, logged when it
/// arrives, a frame or two later).
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn trace(
    mut commands: Commands,
    mut state: ResMut<PortalState>,
    frame: Res<bevy::diagnostic::FrameCount>,
    active: Res<ActiveWorldspace>,
    cams: Query<
        (&Camera, &GlobalTransform, Option<&Exposure>, Has<SkyCamera>, Has<AutoExposure>, Option<&AmbientLight>, Has<TerrainCamera>, Has<sf_atmos_bevy::CloudCamera>),
        With<crate::doors::DoorUser>,
    >,
    portal: Query<(&Camera, Has<TerrainCamera>), (With<PortalCamera>, Without<crate::doors::DoorUser>)>,
    boxes: Query<&Visibility, With<PortalBox>>,
    sun: Query<(&DirectionalLight, Option<&RenderLayers>), With<SunLight>>,
) {
    if state.trace == 0 {
        return;
    }
    state.trace -= 1;
    let Ok((camera, global, exposure, sky, auto, ambient, terrain, clouds)) = cams.single() else { return };
    let sun = sun.single().ok().map(|(l, layers)| (l.illuminance, layers.map(|l| l.iter().collect::<Vec<_>>())));
    let portal = portal.get(state.camera).ok().map(|(c, t)| (c.is_active, t));
    info!(
        "portal trace: frame {} in {:?}: ev100 {:?} sky_camera {sky} auto_exposure {auto} ambient {:?} clear {:?} sun {:?} \
         cam {:.2?} terrain {terrain} clouds {clouds} portal (active, terrain) {portal:?} box {:?}",
        frame.0,
        active.0.as_ref().map(WorldspaceId::as_str),
        exposure.map(|e| e.ev100),
        ambient.map(|a| a.brightness),
        camera.clear_color,
        sun,
        global.translation(),
        boxes.get(state.portal_box).ok(),
    );
    let f = frame.0;
    commands.spawn(Screenshot::primary_window()).observe(move |shot: On<ScreenshotCaptured>| {
        let Some(data) = shot.image.data.as_ref() else { return };
        let (mut sum, mut white, mut black, mut n) = (0u64, 0u32, 0u32, 0u32);
        // Every 7th pixel (RGBA or BGRA 8-bit: the mean of the three channels).
        for px in data.as_chunks::<4>().0.iter().step_by(7) {
            let l = (u32::from(px[0]) + u32::from(px[1]) + u32::from(px[2])) / 3;
            sum += u64::from(l);
            white += u32::from(l > 245);
            black += u32::from(l < 6);
            n += 1;
        }
        let n = n.max(1);
        #[allow(clippy::cast_precision_loss)]
        let (mean, w, b) = (sum as f32 / n as f32, white as f32 / n as f32 * 100.0, black as f32 / n as f32 * 100.0);
        info!("portal luma: frame {f}: mean {mean:.1} white {w:.1}% black {b:.1}%");
    });
}

/// `--portal-walk`: once the start has had time to stream in, walks the
/// camera forward 4 m (through the door in front), back 4 m, twice, at
/// 1.5 m/s, with pauses (crossings are logged).
fn walk_through(time: Res<Time>, mut started: Local<Option<f32>>, mut cams: Query<&mut Transform, With<crate::doors::DoorUser>>) {
    const START_S: f32 = 8.0;
    // (seconds, direction): +1 forward, -1 backward, 0 pause.
    const SCRIPT: [(f32, f32); 8] = [(1.0, 0.0), (8.0 / 3.0, 1.0), (1.0, 0.0), (8.0 / 3.0, -1.0), (1.0, 0.0), (8.0 / 3.0, 1.0), (1.0, 0.0), (8.0 / 3.0, -1.0)];
    let now = time.elapsed_secs();
    if now < START_S {
        return;
    }
    let t0 = *started.get_or_insert(now);
    let mut t = now - t0;
    let dir = SCRIPT.iter().find_map(|&(d, dir)| if t < d { Some(dir) } else { t -= d; None }).unwrap_or(0.0);
    if dir == 0.0 {
        return;
    }
    let Ok(mut transform) = cams.single_mut() else { return };
    let mut forward = transform.rotation * Vec3::NEG_Z;
    forward.y = 0.0;
    transform.translation += forward.normalize_or_zero() * dir * 1.5 * time.delta_secs();
}

/// Swings the kept door of the open portal (hides the other) and closes
/// doors of portals no longer open.
fn swing_doors(
    time: Res<Time>,
    mut state: ResMut<PortalState>,
    spikes: Res<PortalLinks>,
    mut doors: Query<(&PortalDoor, &mut Transform, &mut Visibility)>,
    mut seen: Local<(usize, usize)>,
) {
    let target = if state.open.is_some() { 1.0 } else { 0.0 };
    let step = time.delta_secs() / SWING_SECS;
    state.openness = if target > state.openness { (state.openness + step).min(1.0) } else { (state.openness - step).max(0.0) };
    let plans: Vec<DoorPlan> = state.open.map(|o| spikes.links[o.spike].doors.to_vec()).unwrap_or_default();
    let eased = state.openness * state.openness * (3.0 - 2.0 * state.openness);
    let found = doors.iter().filter(|(d, _, _)| plans.iter().any(|p| p.id == d.id)).count();
    if (found, plans.len()) != *seen {
        *seen = (found, plans.len());
        info!("portal spike: {found} of {} doors spawned ({} door roots)", plans.len(), doors.iter().len());
    }
    for (door, mut transform, mut visibility) in &mut doors {
        let plan = plans.iter().find(|p| p.id == door.id);
        let (rotation, vis) = match plan.map(|p| p.action) {
            Some(DoorMove::Swing(angle)) => (Quat::from_rotation_y(angle * eased) * door.base.rotation, Visibility::Inherited),
            Some(DoorMove::Hide) => (door.base.rotation, Visibility::Hidden),
            Some(DoorMove::Stay) if state.behind == Some(door.id) => (door.base.rotation, Visibility::Hidden),
            Some(DoorMove::Stay) | None => (door.base.rotation, Visibility::Inherited),
        };
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
        if *visibility != vis {
            *visibility = vis;
        }
    }
}

type SkyLights = Or<(With<SunLight>, With<MoonLight>)>;

/// The sun and moon light only the exterior's layer: with a portal camera
/// in the exterior they stay lit while the main camera is inside, and a
/// light on the shared layer 0 would reach every camera.
fn sun_layers(registry: Res<WorldspaceRegistry>, mut lights: Query<&mut RenderLayers, SkyLights>) {
    let Some(entry) = registry.get(&mw_world::ids::exterior()) else { return };
    let want = RenderLayers::layer(entry.layer);
    for mut layers in &mut lights {
        if *layers != want {
            *layers = want.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A street door (exterior, facing +X at x = 0) and an office door
    /// (interior, facing -Z at the origin), both 1.6 x 2.8 m.
    fn pair() -> DoorLink {
        let end = |ws: &str, pos: Vec3, rotation: Quat| {
            let frame = WorldTransform { pos: WorldPos::from_f64(pos.as_dvec3()), rotation, scale: Vec3::ONE };
            LinkEnd { worldspace: WorldspaceId::new(ws), frame, arrival: frame, door: None, extent: Vec2::new(1.6, 2.8) }
        };
        DoorLink {
            a: end("street", Vec3::new(0.0, 2.0, 100.0), Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            b: end("office", Vec3::new(16.0, 2.3, 10.0), Quat::from_rotation_y(std::f32::consts::PI)),
            two_way: true,
        }
    }

    /// Moves the camera up to `steps` times by `step` (camera space: -z
    /// forward, +x right), stopping after `stop_after` crossings; returns
    /// the crossings. `ws` follows the camera's worldspace.
    fn walk(link: &DoorLink, gate: &mut Gate, pose: &mut WorldTransform, ws: &mut String, step: Vec3, steps: u32, stop_after: u32) -> u32 {
        let mut crossings = 0;
        for _ in 0..steps {
            pose.pos = pose.pos.offset(pose.rotation * step);
            if let Some(out) = gate.step(link, pose) {
                *pose = out;
                *ws = link_ends(link, gate.from_a).0.worldspace.as_str().to_owned();
                crossings += 1;
                if crossings == stop_after {
                    break;
                }
            }
        }
        crossings
    }

    #[test]
    fn crossings_by_position_both_ways_forward_backward_and_strafing() {
        let link = pair();
        // Facing the street door (camera -Z = the frame's -Z).
        let street = |local: Vec3, turn: f32| WorldTransform {
            pos: link.a.frame.pos.offset(link.a.frame.rotation * local),
            rotation: link.a.frame.rotation * Quat::from_rotation_y(turn),
            scale: Vec3::ONE,
        };
        // 3 cm steps (1.8 m/s at 60 fps).
        let (fwd, back) = (Vec3::new(0.0, 0.0, -0.03), Vec3::new(0.0, 0.0, 0.03));
        let mut ws = "street".to_owned();
        for round in 0..3 {
            let mut gate = Gate::new(true, [FRONT_M; 2]);
            let mut pose = street(Vec3::new(0.1, 1.6, 2.0), 0.0);
            gate.step(&link, &pose);
            // Forward in, on into the office, then backwards out.
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, fwd, 80, 9), 1, "round {round}: in");
            assert_eq!(ws, "office");
            assert!(local_in(&link.b, pose.pos).z > CROSS_AT_M);
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, back, 80, 9), 1, "round {round}: backwards out");
            assert_eq!(ws, "street");
            // Just through, then straight back at once (the arrival is not
            // past the return trigger, no cooldown), and again.
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, fwd, 80, 1), 1);
            assert_eq!(ws, "office");
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, back, 3, 1), 1, "round {round}: step back out");
            assert_eq!(ws, "street");
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, fwd, 3, 1), 1, "round {round}: step back in");
            assert_eq!(ws, "office");
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, back, 3, 1), 1);
            assert_eq!(ws, "street");
        }
        // Strafing: facing along the door (turned 90 degrees), right is in.
        let turn = std::f32::consts::FRAC_PI_2;
        let (right, left) = (Vec3::new(0.03, 0.0, 0.0), Vec3::new(-0.03, 0.0, 0.0));
        for round in 0..3 {
            let mut gate = Gate::new(true, [FRONT_M; 2]);
            let mut pose = street(Vec3::new(0.0, 1.6, 1.0), turn);
            gate.step(&link, &pose);
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, right, 60, 9), 1, "round {round}: strafe in");
            assert_eq!(ws, "office");
            assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, left, 60, 9), 1, "round {round}: strafe out");
            assert_eq!(ws, "street");
        }
        // Passing beside the opening (into the wall) never crosses.
        let mut gate = Gate::new(true, [FRONT_M; 2]);
        let mut pose = street(Vec3::new(1.5, 1.6, 1.0), 0.0);
        assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, fwd, 60, 9), 0);
        // Walking along the door's plane, in front of the trigger: no crossing.
        let mut gate = Gate::new(true, [FRONT_M; 2]);
        let mut pose = street(Vec3::new(-3.0, 1.6, 0.5), std::f32::consts::FRAC_PI_2);
        assert_eq!(walk(&link, &mut gate, &mut pose, &mut ws, Vec3::new(0.0, 0.0, 0.03), 200, 9), 0);
    }
}

#[cfg(test)]
mod exposure_floor_tests {
    use super::*;

    /// The viewer's steady-state EV100 at metered average `x`.
    fn ev(points: &[Vec2], x: f32) -> f32 {
        mw_assets::SCENE_EV100 - (sample(points, x) - x)
    }

    #[test]
    fn auto_exposure_stops_at_the_night_floor() {
        let open = compensation_points(DEFAULT_EXPOSURE_STRENGTH, ExposureRange::default());
        let floored = compensation_points(DEFAULT_EXPOSURE_STRENGTH, LightCalibration::default().exposure_range());
        // A dark street opens past the floor without it, stops at it with it.
        assert!(ev(&open, -14.0) < 3.0);
        assert!((ev(&floored, -14.0) - LightCalibration::default().exposure_floor_ev100).abs() < 1e-3);
        // A lit interior and daylight are untouched.
        for x in [INTERIOR_X, 5.0] {
            assert!((ev(&open, x) - ev(&floored, x)).abs() < 1e-3, "x {x}");
        }
        assert!(compensation_curve(DEFAULT_EXPOSURE_STRENGTH, LightCalibration::default().exposure_range()).is_some());
    }
}
