//! `bevyrrowind clip-preview` (dev builds; motion-generation.md MG5): animation clips played on a
//! real Morrowind NPC on a flat, noon-lit stage, captured frame by frame into one MP4 per clip,
//! for the owner to review generated (and library) clips on the real rig.
//!
//! - The clip is cooked and retargeted in process with the bake's own Bevy-free calls
//!   ([`cook`]): no bake round trip.
//! - The NPC is the `character` viewer's actor (`mw_actors_bevy` content: real body, outfit,
//!   compiled graph; no AI, no physics). The clip plays as a late-bound montage
//!   (`sf_anim_graph::LateClip`) in the graph's full-body slot with root motion applied.
//! - Capture is deterministic: `TimeUpdateStrategy::ManualDuration(1/fps)` steps every clock
//!   by exactly one frame per app update, the actor's animation is pinned to full rate (A0), and
//!   a `Screenshot` per frame is taken in `Last` (the frame's own render), saved as
//!   `<out>/<id>/frames/<i>.png`. Frame `i` shows clip time `i/fps`: the montage starts on an
//!   update with a zero time scale (pose at 0), then advances one step per frame.
//! - `ffmpeg` encodes each clip's frames (H.264, yuv420p); the frames are removed unless
//!   `keep_frames`. The run fails if a clip's PNG count isn't its frame count.

pub mod browse;
pub mod cook;
mod live;

use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bevy::asset::io::AssetSourceId;
use bevy::camera::Exposure;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use bevy::window::{PresentMode, WindowResolution};
use bevy::world_serialization::{WorldAsset, WorldAssetRoot};
use mw_assets::MwAssetsPlugin;
use mw_bake::anim::HumanoidTarget;
use sf_anim_bevy::{AnimLod, AnimRootMotion, AnimTier, Animator, SkeletonBinding};
use sf_anim_graph::{LateClip, MontageClip, SlotRequest, Variety};
use sf_character::{CharacterState, Readiness};
use sf_character_bevy::{Character, CharacterReadiness, CharacterStats};
use sf_records::{RecordDb, RecordId};
use sf_vfs::Vfs;

use cook::{At, ClipJob, PropOffset, PropSpec, Resolved};

/// A plain Dunmer commoner of Morrowind.esm (Seyda Neen; common shirt, pants, shoes).
pub const DEFAULT_NPC: &str = "draren thiralas";

/// Where the camera looks from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PreviewCamera {
    /// In front of the actor (it faces +Z, towards the camera).
    #[default]
    Front,
    /// From its left side.
    Side,
    /// Circling it, one turn per [`ORBIT_PERIOD`] seconds.
    Orbit,
}

impl std::str::FromStr for PreviewCamera {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "front" => Ok(Self::Front),
            "side" => Ok(Self::Side),
            "orbit" => Ok(Self::Orbit),
            _ => Err(format!("camera {s:?}: front, side or orbit")),
        }
    }
}

/// Seconds per orbit turn.
pub const ORBIT_PERIOD: f32 = 8.0;

pub struct ClipPreviewOptions {
    pub jobs: Vec<ClipJob>,
    pub npc: String,
    pub camera: PreviewCamera,
    pub out: PathBuf,
    pub fps: u32,
    pub size: (u32, u32),
    /// The ffmpeg executable (`ffmpeg` on PATH by default).
    pub ffmpeg: PathBuf,
    pub keep_frames: bool,
    /// MG6: a placeable record (`STAT`, `ACTI`, `CONT`, ...) whose model stands in the scene,
    /// e.g. a chair for a seated clip.
    pub furniture: Option<String>,
    /// Where the furniture stands (scene frame: meters, degrees).
    pub at: At,
    /// An item record whose model the actor holds, on a hand bone.
    pub prop: Option<PropSpec>,
    /// The prop under its bone (bone frame: meters, degrees).
    pub prop_offset: PropOffset,
    /// Browse this folder's clips in a live window (no capture) instead of `jobs`.
    pub browse: Option<BrowseOptions>,
}

/// `--browse <dir>` (MG10a): every `.bvh` under `dir`, cooked on demand with `source`'s rules.
pub struct BrowseOptions {
    pub dir: PathBuf,
    pub source: String,
}

/// Bones tried for a prop with no `@<bone>`: the game's right-hand attach bone (held weapons,
/// `mw_actors::held::bone::WEAPON`), then the hand joint itself (as `mw_combat::WEAPON_BONES`).
const DEFAULT_PROP_BONES: [&str; 2] = [mw_actors::held::bone::WEAPON, "Bip01 R Hand"];

/// The held prop (MG7a): its model loads from `Startup`, it is attached once the actor's rig is
/// bound, and the clip waits for both.
struct Prop {
    id: String,
    model: String,
    handle: Handle<WorldAsset>,
    bone: Option<String>,
    offset: Transform,
    attached: bool,
}

/// One clip through the run.
struct Item {
    job: Resolved,
    /// Cooked onto the actor's skeleton once it is dressed.
    late: Option<Arc<LateClip>>,
    /// Cook + retarget, wall time.
    cook: Duration,
    frames: u32,
    capture: Duration,
}

impl Item {
    fn frames_dir(&self, out: &std::path::Path) -> PathBuf {
        out.join(&self.job.id).join("frames")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// Waiting for the actor, its textures and pipelines; then `settle` more frames.
    Loading { ready_frames: u32 },
    /// Clip `index`, next frame to shoot `frame`.
    Clip { index: usize, frame: u32 },
    /// Waiting for the last screenshots to be written.
    Saving { frames_waited: u32 },
    /// The live browser ([`live::browse_step`]) drives the actor; nothing is captured.
    Browse,
}

#[derive(Resource)]
struct Preview {
    items: Vec<Item>,
    phase: Phase,
    out: PathBuf,
    fps: u32,
    camera: PreviewCamera,
    ffmpeg: PathBuf,
    keep_frames: bool,
    started: Option<Instant>,
    launched: Instant,
    loading_frames: u32,
    failed: bool,
    /// The furniture's model, waited for while loading (MG6).
    furniture: Option<Handle<WorldAsset>>,
    /// The held prop, waited for while loading.
    prop: Option<Prop>,
    /// The browse state; `None` in capture runs.
    browse: Option<live::Browse>,
}

/// Screenshots written so far (both `save_to_disk` and this counter observe the capture).
#[derive(Resource, Default)]
struct Saved(u32);

/// Frames of settle after everything is loaded (pipelines and shadow maps warm).
const SETTLE_FRAMES: u32 = 20;
/// Frames waited for the actor before giving up (at the capture step, not wall time).
const LOAD_LIMIT_FRAMES: u32 = 30 * 120;
/// Frames waited for the last screenshots.
const SAVE_LIMIT_FRAMES: u32 = 600;

/// Checks every job, then opens the window and renders them all. Returns `false` if a clip
/// failed (file, cook, capture or encode).
pub fn run(vfs: Arc<Vfs>, db: Arc<RecordDb>, store: sf_bake::Store, manifest: sf_bake::Manifest, options: ClipPreviewOptions) -> bool {
    let mut items = Vec::new();
    let mut failed = false;
    for job in &options.jobs {
        match cook::resolve(job) {
            Ok(job) => items.push(Item { job, late: None, cook: Duration::ZERO, frames: 0, capture: Duration::ZERO }),
            Err(e) => {
                eprintln!("error: {e}");
                failed = true;
            }
        }
    }
    let browse = match options.browse.as_ref().map(|b| browse::BrowseList::scan(&b.dir).map(|list| live::Browse::new(list, b.source.clone()))).transpose() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: {e}");
            return false;
        }
    };
    if items.is_empty() && browse.is_none() {
        return false;
    }
    let npc = RecordId::new(options.npc.as_str());
    if db.get::<tes3::esp::Npc>(&npc).is_none() {
        eprintln!("error: no NPC_ record {:?}", options.npc);
        return false;
    }
    let furniture = match options.furniture.as_deref().map(|id| placeable_model(&db, id, "--furniture")).transpose() {
        Ok(f) => f.map(|(id, model)| (id, model, options.at)),
        Err(e) => {
            eprintln!("error: {e}");
            return false;
        }
    };
    let prop = match options.prop.as_ref().map(|p| placeable_model(&db, &p.id, "--prop").map(|(id, model)| (id, model, p.bone.clone()))).transpose() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            return false;
        }
    };
    let prop_offset = {
        let o = options.prop_offset;
        let [rx, ry, rz] = o.rotation_deg.map(f32::to_radians);
        Transform::from_translation(Vec3::from_array(o.translation)).with_rotation(Quat::from_euler(EulerRot::XYZ, rx, ry, rz))
    };
    let content = Arc::new(mw_actors_bevy::actors::MwCharacters::new(Arc::clone(&db), Arc::clone(&vfs), store, manifest, mw_physics::ClothMode::Authored));
    let step = Duration::from_secs_f64(1.0 / f64::from(options.fps.max(1)));

    let mut app = App::new();
    app.register_asset_source(AssetSourceId::Default, sf_vfs::bevy::asset_source(vfs.clone()));
    mw_environment_bevy::render::RenderSettings::init(&mut app);
    mw_game_core::shipped_errors(&mut app);
    let browsing = browse.is_some();
    app.add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(window(options.size, browsing)), ..default() }))
    .add_plugins((MwAssetsPlugin { vfs }, sf_anim_bevy::SfAnimPlugin))
    .insert_resource(mw_assets::LightCalibration::default().ambient([150, 150, 160]))
    .insert_resource(sf_character_bevy::CharacterSettings { bodies: sf_character_bevy::SmoothBodies::default() })
    .insert_resource(Preview {
        items,
        phase: Phase::Loading { ready_frames: 0 },
        out: options.out,
        fps: options.fps.max(1),
        camera: options.camera,
        ffmpeg: options.ffmpeg,
        keep_frames: options.keep_frames,
        started: None,
        launched: Instant::now(),
        loading_frames: 0,
        failed,
        furniture: None,
        prop: None,
        browse,
    })
    .init_resource::<Saved>()
    .add_systems(
        Startup,
        move |mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, assets: Res<AssetServer>, mut preview: ResMut<Preview>| {
            stage(&mut commands, &mut meshes, &mut materials, npc.clone());
            if let Some((id, model, at)) = &furniture {
                preview.furniture = Some(place_furniture(&mut commands, &assets, id, model, *at));
            }
            if let Some((id, model, bone)) = &prop {
                let handle = assets.load(model.clone());
                preview.prop = Some(Prop { id: id.clone(), model: model.clone(), handle, bone: bone.clone(), offset: prop_offset, attached: false });
            }
        },
    )
    .add_systems(Update, drive);
    add_mode(&mut app, browsing, step);
    mw_actors_bevy::actors::add_content(&mut app, content);
    app.insert_resource(mw_actors_bevy::held::HeldDarkness::forced(Some(false)));
    mw_devtools_bevy::pipeline_watch::add(&mut app);
    let exit = app.run();
    exit.is_success()
}

/// Capture frames keep one size and don't wait for vsync; the browser is a normal window.
fn window((w, h): (u32, u32), browsing: bool) -> Window {
    Window {
        title: "Bevyrrowind - clip preview".into(),
        resolution: WindowResolution::new(w, h).with_scale_factor_override(1.0),
        resizable: browsing,
        present_mode: if browsing { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync },
        ..default()
    }
}

/// Capture: one fixed step per update, the follow camera, a screenshot per frame. Browse
/// (MG10a): real time, the orbit camera, the overlay, keys and on-demand cooks.
fn add_mode(app: &mut App, browsing: bool, step: Duration) {
    if browsing {
        app.init_resource::<live::OrbitCam>()
            .add_systems(Startup, live::spawn_overlay)
            .add_systems(Update, live::browse_step.after(drive))
            .add_systems(PostUpdate, live::orbit.after(sf_anim_bevy::AnimSystems::Apply).before(TransformSystems::Propagate));
    } else {
        app.insert_resource(TimeUpdateStrategy::ManualDuration(step))
            .add_systems(PostUpdate, follow.after(sf_anim_bevy::AnimSystems::Apply).before(TransformSystems::Propagate))
            .add_systems(Last, shoot);
    }
}

/// A floor of 1 m checker tiles (foot sliding and root travel read against it), the noon sun
/// straight above-ish, the actor at the origin facing +Z, the camera.
fn stage(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, npc: RecordId) {
    let tile = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    let shades = [0.36, 0.30].map(|v| materials.add(StandardMaterial { base_color: Color::srgb(v, v * 0.95, v * 0.88), perceptual_roughness: 0.9, ..default() }));
    for x in -12..12_i16 {
        for z in -12..12_i16 {
            let shade = &shades[usize::from((x + z).rem_euclid(2) == 1)];
            commands.spawn((Mesh3d(tile.clone()), MeshMaterial3d(shade.clone()), Transform::from_xyz(f32::from(x) + 0.5, 0.0, f32::from(z) + 0.5)));
        }
    }
    commands.spawn((
        DirectionalLight { illuminance: 600.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(2.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Character::Record { kind: "NPC_".into(), id: npc.clone(), state: CharacterState::default() },
        Transform::IDENTITY,
        Name::new(npc.as_str().to_owned()),
        CharacterReadiness(Readiness::Sheathed),
    ));
    commands.spawn((Camera3d::default(), Exposure { ev100: mw_assets::SCENE_EV100 }, Transform::from_xyz(0.0, 1.3, 3.6).looking_at(Vec3::new(0.0, 0.95, 0.0), Vec3::Y)));
}

/// The model of a placeable record (`mw_world::resolve_base`: every kind a cell reference can
/// place, except actors), or an error naming the closest ids; `arg` names the option.
fn placeable_model(db: &RecordDb, id: &str, arg: &str) -> Result<(String, String), String> {
    use tes3::esp::{Activator, Container, Door, Light, MiscItem, Static};
    match mw_world::resolve_base(db, id) {
        Some((mw_world::BaseKind::Actor, ..)) => Err(format!("{arg} {id:?} is an actor, not a placeable object")),
        Some((_, Some(model), _)) => Ok((id.to_owned(), model)),
        Some((_, None, _)) => Err(format!("{arg} {id:?} has no model")),
        None => {
            let ids = db
                .iter::<Static>()
                .map(|(k, _)| k)
                .chain(db.iter::<Activator>().map(|(k, _)| k))
                .chain(db.iter::<Container>().map(|(k, _)| k))
                .chain(db.iter::<Door>().map(|(k, _)| k))
                .chain(db.iter::<Light>().map(|(k, _)| k))
                .chain(db.iter::<MiscItem>().map(|(k, _)| k))
                .map(RecordId::as_str);
            let near = cook::near_matches(id, ids, 8);
            Err(format!("{arg} {id:?}: no placeable record; closest: {}", near.join(", ")))
        }
    }
}

/// The furniture's model (the runtime NIF path, as the `view` viewer spawns an uncooked
/// reference) at `at`.
fn place_furniture(commands: &mut Commands, assets: &AssetServer, id: &str, model: &str, at: At) -> Handle<WorldAsset> {
    let handle: Handle<WorldAsset> = assets.load(model.to_owned());
    let transform = Transform::from_xyz(at.x, at.y, at.z).with_rotation(Quat::from_rotation_y(at.yaw_deg.to_radians()));
    commands.spawn((WorldAssetRoot(handle.clone()), transform, Visibility::Inherited, Name::new(id.to_owned())));
    println!("furniture {id} ({model}) at {:.2} {:.2} {:.2}, yaw {:.0}", at.x, at.y, at.z, at.yaw_deg);
    handle
}

/// The prop while loading: attaches it under its bone once the actor's rig is bound (`rig`:
/// its skeleton, body entity and joint entities). `Ok(true)` once attached and its model is
/// loaded; an error if the model failed or the skeleton lacks the bone.
fn prop_step(
    commands: &mut Commands,
    prop: &mut Prop,
    server: &AssetServer,
    rig: Option<(&sf_anim::Skeleton, Entity, &SkeletonBinding)>,
    joints: &Query<(Option<&ChildOf>, Has<Visibility>)>,
) -> Result<bool, String> {
    let loaded = match server.get_recursive_dependency_load_state(prop.handle.id()) {
        Some(bevy::asset::RecursiveDependencyLoadState::Failed(e)) => return Err(format!("the prop model failed to load: {e}")),
        load => matches!(load, Some(bevy::asset::RecursiveDependencyLoadState::Loaded)),
    };
    if !prop.attached
        && let Some((skeleton, body, binding)) = rig
        && !binding.joints.is_empty()
    {
        let names: Vec<&str> = prop.bone.as_deref().map_or_else(|| DEFAULT_PROP_BONES.to_vec(), |b| vec![b]);
        let Some((bone, joint)) = names.iter().find_map(|n| binding.find(skeleton, n).map(|j| (*n, j))) else {
            let hands: Vec<&str> = skeleton.names().iter().map(String::as_str).filter(|n| n.contains("Hand") || n.contains("Bone")).collect();
            return Err(format!("--prop bone {names:?} is not in the NPC's skeleton; hand bones: {}", hands.join(", ")));
        };
        visible_chain(commands, joints, joint, body);
        commands.spawn((WorldAssetRoot(prop.handle.clone()), prop.offset, Visibility::Inherited, Name::new(prop.id.clone()), ChildOf(joint)));
        let (t, (rx, ry, rz)) = (prop.offset.translation, prop.offset.rotation.to_euler(EulerRot::XYZ));
        println!(
            "prop {} ({}) on {bone:?}, offset {:.3} {:.3} {:.3} m, {:.0} {:.0} {:.0} deg",
            prop.id,
            prop.model,
            t.x,
            t.y,
            t.z,
            rx.to_degrees(),
            ry.to_degrees(),
            rz.to_degrees()
        );
        prop.attached = true;
    }
    Ok(loaded && prop.attached)
}

/// Gives the joints from `joint` up to `body` visibility components, so the prop under them
/// inherits the actor's visibility (rig joints carry none; as `sf_character_bevy`'s held props).
fn visible_chain(commands: &mut Commands, joints: &Query<(Option<&ChildOf>, Has<Visibility>)>, joint: Entity, body: Entity) {
    let mut at = joint;
    while at != body {
        let Ok((parent, has)) = joints.get(at) else { return };
        if !has {
            commands.entity(at).try_insert(Visibility::Inherited);
        }
        let Some(parent) = parent else { return };
        at = parent.parent();
    }
}

type ActorQuery<'a> = (&'a mut Animator, &'a mut AnimLod, &'a mut Transform, Option<&'a mut AnimRootMotion>);

/// The run's state machine, before the animation update: wait for the actor, then start each
/// clip on its frame 0 (time scale 0: the pose at clip time 0), then advance at rate 1.
/// `shoot` (in `Last`) moves to the next clip, so each clip starts on the update whose frame 0
/// it then shoots.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
fn drive(
    mut commands: Commands,
    mut preview: ResMut<Preview>,
    mut actors: Query<ActorQuery>,
    rigs: Query<(Entity, &SkeletonBinding), With<Animator>>,
    joints: Query<(Option<&ChildOf>, Has<Visibility>)>,
    stats: Res<CharacterStats>,
    pending: Res<mw_devtools_bevy::pipeline_watch::PendingDraws>,
    server: Res<AssetServer>,
    materials_q: Query<&MeshMaterial3d<StandardMaterial>>,
    materials: Res<Assets<StandardMaterial>>,
    mut exit: MessageWriter<AppExit>,
) {
    let p = &mut *preview;
    match p.phase {
        Phase::Loading { ready_frames } => {
            p.loading_frames += 1;
            let textures_pending = materials_q.iter().filter_map(|m| materials.get(&m.0)).filter_map(|m| m.base_color_texture.as_ref()).any(|t| {
                !matches!(server.get_load_state(t.id()), Some(bevy::asset::LoadState::Loaded | bevy::asset::LoadState::Failed(_)) | None)
            });
            let actor = actors.iter().next().is_some();
            let furniture_ready = match &p.furniture {
                None => true,
                Some(h) => match server.get_recursive_dependency_load_state(h.id()) {
                    Some(bevy::asset::RecursiveDependencyLoadState::Failed(e)) => {
                        eprintln!("error: the furniture model failed to load: {e}");
                        exit.write(AppExit::error());
                        return;
                    }
                    load => matches!(load, Some(bevy::asset::RecursiveDependencyLoadState::Loaded)),
                },
            };
            if stats.failed > 0 {
                eprintln!("error: the NPC failed to build (see the log)");
                exit.write(AppExit::error());
                return;
            }
            let rig = (stats.spawned > 0).then(|| actors.iter().next().zip(rigs.iter().next())).flatten();
            let prop_ready = match p.prop.as_mut().map(|prop| prop_step(&mut commands, prop, &server, rig.map(|((a, ..), (body, b))| (&*a.asset.skeleton, body, b)), &joints)) {
                None => true,
                Some(Ok(ready)) => ready,
                Some(Err(e)) => {
                    eprintln!("error: {e}");
                    exit.write(AppExit::error());
                    return;
                }
            };
            let ready = actor && furniture_ready && prop_ready && stats.spawned > 0 && !textures_pending && pending.gpu().pipelines == 0 && pending.get() == 0;
            if p.loading_frames > LOAD_LIMIT_FRAMES {
                eprintln!("error: the NPC is still loading after {LOAD_LIMIT_FRAMES} frames");
                exit.write(AppExit::error());
                return;
            }
            let ready_frames = if ready { ready_frames + 1 } else { 0 };
            p.phase = Phase::Loading { ready_frames };
            if ready_frames < SETTLE_FRAMES {
                return;
            }
            let Some((animator, mut lod, ..)) = actors.iter_mut().next() else { return };
            lod.forced = Some(AnimTier::A0);
            println!("loaded in {} frames ({:.1} s wall)", p.loading_frames, p.launched.elapsed().as_secs_f32());
            let asset = Arc::clone(&animator.asset);
            if let Some(browse) = p.browse.as_mut() {
                // MG10a: clips are cooked one at a time, on demand, from here on.
                match HumanoidTarget::new((*asset.skeleton).clone()) {
                    Ok(target) => browse.target = Some(target),
                    Err(e) => {
                        eprintln!("error: the NPC's skeleton is not humanoid ({e})");
                        exit.write(AppExit::error());
                    }
                }
                p.phase = Phase::Browse;
                return;
            }
            if let Err(e) = cook_all(p, &asset) {
                eprintln!("error: {e}");
                exit.write(AppExit::error());
                return;
            }
            // Falls through: the first clip starts on this same update, because `shoot` takes
            // its frame 0 at the end of it.
            p.phase = next_clip(&p.items, 0);
        }
        Phase::Clip { .. } | Phase::Saving { .. } | Phase::Browse => {}
    }
    let Phase::Clip { index, frame } = p.phase else { return };
    let Some((mut animator, _, mut transform, root)) = actors.iter_mut().next() else { return };
    if frame > 0 {
        animator.time_scale = 1.0;
        return;
    }
    let item = &mut p.items[index];
    let Some(late) = item.late.clone() else { return };
    let dir = item.frames_dir(&p.out);
    let _ = std::fs::remove_dir_all(&dir);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("error: {}: {e}", dir.display());
        exit.write(AppExit::error());
        return;
    }
    item.capture = Duration::ZERO;
    p.started = Some(Instant::now());
    if let Err(e) = play_full(&mut animator, &mut transform, root, late) {
        eprintln!("error: {}: {e}", item.job.id);
        exit.write(AppExit::error());
        return;
    }
    animator.time_scale = 0.0;
}

/// The actor back on the mark (origin, facing +Z) with root motion applied, and `late` from its
/// first pose in the graph's full-body slot (no blend in). Capture and browse both start clips here.
fn play_full(animator: &mut Animator, transform: &mut Transform, root: Option<Mut<AnimRootMotion>>, late: Arc<LateClip>) -> Result<(), String> {
    transform.translation = Vec3::ZERO;
    transform.rotation = Quat::IDENTITY;
    if let Some(mut root) = root {
        root.apply = true;
    }
    let asset = Arc::clone(&animator.asset);
    animator.instance.set_variety(&asset.binding(), Variety::default());
    let slot = asset.graph.slot(mw_graphs::build::slots::FULL).ok_or_else(|| format!("the actor's graph has no {:?} slot", mw_graphs::build::slots::FULL))?;
    let req = SlotRequest { blend_in: 0.0, priority: 9, ..SlotRequest::new(MontageClip::Late(late)) };
    animator.instance.play_slot(&asset.binding(), slot, req).map(|_| ()).map_err(|e| format!("the slot refused the clip ({e:?})"))
}

/// Every clip cooked onto the dressed actor's skeleton (Bevy-free, [`cook::cook`]) and bound
/// late to its graph. A clip that fails is reported and skipped.
fn cook_all(p: &mut Preview, asset: &sf_anim_bevy::AnimAsset) -> Result<(), String> {
    let target = HumanoidTarget::new((*asset.skeleton).clone()).map_err(|e| format!("the NPC's skeleton is not humanoid ({e})"))?;
    for item in &mut p.items {
        let t = Instant::now();
        match cook::cook(&item.job, &target)
            .and_then(|clip| LateClip::new(&asset.graph, &asset.skeleton, Arc::new(clip)).map_err(|e| format!("late clip: {e:?}")))
        {
            Ok(late) => {
                item.cook = t.elapsed();
                item.frames = cook::frame_count(late.clip.duration, p.fps);
                println!(
                    "clip {}: {} {:?} (source {}), {:.2} s, {} frames, cooked + retargeted in {:.0} ms",
                    item.job.id,
                    item.job.path.display(),
                    late.clip.name,
                    item.job.source,
                    late.clip.duration,
                    item.frames,
                    item.cook.as_secs_f32() * 1000.0
                );
                item.late = Some(Arc::new(late));
            }
            Err(e) => {
                eprintln!("error: {}: {e}", item.job.id);
                p.failed = true;
            }
        }
    }
    Ok(())
}

/// The first clip from `from` that retargeted, else saving.
fn next_clip(items: &[Item], from: usize) -> Phase {
    items.iter().enumerate().skip(from).find(|(_, i)| i.late.is_some()).map_or(Phase::Saving { frames_waited: 0 }, |(index, _)| Phase::Clip { index, frame: 0 })
}

/// The camera follows the actor (root motion moves it), after the animation applied this frame.
#[allow(clippy::needless_pass_by_value)]
fn follow(preview: Res<Preview>, time: Res<Time>, actors: Query<&Transform, With<Animator>>, mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Animator>)>) {
    let Some(at) = actors.iter().next().map(|t| t.translation) else { return };
    let target = Vec3::new(at.x, 0.95, at.z);
    let offset = match preview.camera {
        PreviewCamera::Front => Vec3::new(0.0, 0.35, 3.6),
        PreviewCamera::Side => Vec3::new(3.6, 0.35, 0.0),
        PreviewCamera::Orbit => {
            let angle = time.elapsed_secs() * std::f32::consts::TAU / ORBIT_PERIOD;
            Vec3::new(3.6 * angle.sin(), 0.35, 3.6 * angle.cos())
        }
    };
    for mut t in &mut cameras {
        *t = Transform::from_translation(target + offset).looking_at(target, Vec3::Y);
    }
}

/// After this frame's pose: its screenshot; then the next frame, clip or the encode.
fn shoot(mut commands: Commands, mut preview: ResMut<Preview>, saved: Res<Saved>, mut exit: MessageWriter<AppExit>) {
    let p = &mut *preview;
    match p.phase {
        Phase::Clip { index, frame } => {
            let item = &mut p.items[index];
            let path = item.frames_dir(&p.out).join(format!("{frame:05}.png"));
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path)).observe(|_: On<ScreenshotCaptured>, mut saved: ResMut<Saved>| saved.0 += 1);
            if frame + 1 < item.frames {
                p.phase = Phase::Clip { index, frame: frame + 1 };
            } else {
                item.capture = p.started.map_or(Duration::ZERO, |s| s.elapsed());
                p.phase = next_clip(&p.items, index + 1);
            }
        }
        Phase::Saving { frames_waited } => {
            let expected: u32 = p.items.iter().filter(|i| i.late.is_some()).map(|i| i.frames).sum();
            if saved.0 < expected && frames_waited < SAVE_LIMIT_FRAMES {
                p.phase = Phase::Saving { frames_waited: frames_waited + 1 };
                return;
            }
            let ok = encode(p);
            exit.write(if ok { AppExit::Success } else { AppExit::error() });
        }
        Phase::Loading { .. } | Phase::Browse => {}
    }
}

/// Encodes each captured clip with ffmpeg and prints its line; false if any failed.
fn encode(p: &Preview) -> bool {
    let mut ok = !p.failed;
    for item in p.items.iter().filter(|i| i.late.is_some()) {
        let dir = item.frames_dir(&p.out);
        let pngs = std::fs::read_dir(&dir).map_or(0, |d| d.filter_map(Result::ok).filter(|e| e.path().extension().is_some_and(|x| x == "png")).count());
        if pngs != item.frames as usize {
            eprintln!("error: {}: {pngs} PNGs for {} frames", item.job.id, item.frames);
            ok = false;
            continue;
        }
        let mp4 = p.out.join(format!("{}.mp4", item.job.id));
        let t = Instant::now();
        let status = Command::new(&p.ffmpeg)
            .args(["-y", "-loglevel", "error", "-framerate", &p.fps.to_string(), "-i"])
            .arg(dir.join("%05d.png"))
            .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
            .arg(&mp4)
            .status();
        let encode = t.elapsed();
        match status {
            Ok(s) if s.success() => {
                println!(
                    "{}: {} frames -> {} (cook + retarget {:.0} ms, capture {:.1} s, encode {:.1} s)",
                    item.job.id,
                    item.frames,
                    mp4.display(),
                    item.cook.as_secs_f32() * 1000.0,
                    item.capture.as_secs_f32(),
                    encode.as_secs_f32()
                );
                if !p.keep_frames {
                    let _ = std::fs::remove_dir_all(dir.parent().unwrap_or(&dir));
                }
            }
            Ok(s) => {
                eprintln!("error: {}: ffmpeg exited with {s}", item.job.id);
                ok = false;
            }
            Err(e) => {
                eprintln!("error: {}: ffmpeg ({}): {e}", item.job.id, p.ffmpeg.display());
                ok = false;
            }
        }
    }
    ok
}
