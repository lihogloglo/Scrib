//! Rigid-body physics (ROADMAP M4; superfargoth physics.md, physics-adapter.md):
//! `sf_physics_bevy` on the streamed world.
//!
//! - R0 statics and doors become `Fixed` colliders from their `collision.cook`
//!   shapes: [`hook`] wraps their spawners after `exterior.rs` builds them.
//!   Terrain heightfield chunks follow the exterior's R0 cells. Interiors are
//!   their own physics worlds (one per worldspace; only the active one steps).
//! - Items (P4; [`MwItems`]): `Dynamic` placements and carryable lights get
//!   their `body.cook` shape as sleeping dynamic bodies, mass from the record
//!   weight (`mw_physics`), held `Fixed` where `items.check` found them
//!   unsupported (and carryable lights always). Moved items persist through
//!   the `PlacementOverlay` when their cell unloads. Statics whose authored
//!   collision is a box around their shelves (`items.check` `supports`)
//!   simulate with their `support.cook` render mesh, so the items rest on
//!   the shelves instead of starting inside the box.
//! - `V` throws a 3 kg ball from the camera (PH-1 without a player
//!   controller); the 8 newest stay.
//! - `--physics-debug` (or `G`) draws collider wireframes around the camera.
//! - Characters (P5, `sf_physics_bevy` feature `controller`): the walkers'
//!   mover probes this physics world (statics, terrain chunks, props), each
//!   mover has a kinematic capsule that thrown and falling bodies hit, and
//!   characters push debris and props with capped impulses.
//! - `--off physics` leaves physics out; the walkers then probe their own
//!   `StaticWorld` (walkers.rs).
//! - Every 10 s the log gets the peaks: step and insert time per frame.
//! - Ragdolls (P7, `sf_physics_bevy` feature `ragdoll`): `I` kills the
//!   character nearest the crosshair ([`kill_key`], PH-2): its graph's `dead`
//!   state plays the death clip and after [`DEATH_HANDOFF`] it goes limp
//!   (the `ragdoll.cook` product of its skeleton). Corpses are `RAGDOLL`
//!   bodies the carry driver drags by the grabbed limb.
//! - Knockdowns (P8b) and hit reactions (P9): capsule impacts of at least
//!   [`KNOCKDOWN_IMPULSE`] knock a walker down (powered ragdoll, get-up from
//!   the `Knockout@tail` clip's first frame); lighter ones kick the graph's
//!   `HitReact` springs. An impact is the incoming body's momentum at first
//!   contact (`sf_physics::impact_impulse`), never the solver's contact
//!   force: walking into a ball lying on the street is no hit.
//! - `BEVYRROWIND_AUTO` ([`Auto`]): scripted kills, throws and screenshots
//!   for agents verifying without a keyboard.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, OnceLock};

use bevy::camera::visibility::RenderLayers;
use bevy::camera_controller::free_camera::{FreeCamera, run_freecamera_controller};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use mw_physics::ItemRecords;
use sf_bake::{Manifest, Store};
use sf_controller::steps::COLLISION_COOK;
use sf_controller::{CollisionMesh, StaticShape};
use sf_coords::WorldTransform;
use sf_physics::steps::{BODY_COOK, Hold, ITEMS_CHECK, ItemChecks, SUPPORT_COOK};
use sf_physics::{BodyDesc, BodyKind, BodyShape, Layer, ShapeRef};
use sf_physics_bevy::{
    CarryFeedback, CarryInput, CharacterStats, Holder, ItemBody, ItemContent, Physics, PhysicsBody, PhysicsBodySource, PhysicsDebug, PhysicsPlugin, PhysicsShapeSource,
    PhysicsSettings, PhysicsStats, PhysicsStreaming, hook_items, hook_physics,
};
use sf_anim_bevy::{Animator, Hit};
use sf_character_bevy::CharacterBody;
use sf_controller_bevy::Dormant;
use sf_controller_bevy::Mover;
use sf_physics_bevy::{ImpactOnCharacter, Ragdoll, RagdollGotUp, RagdollMode, RagdollSource, RagdollStats, clip_root_body};
use sf_ragdoll::{RAGDOLL_COOK, RagdollDef, RagdollPolicy};
use sf_records::RecordDb;
use sf_world::{Placement, PlacementClass, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, PlacementSpawners, WorldPosOf, from_grid, transform_to_grid};

/// Adds physics to the streamed viewer. `db` gives item weights and the
/// carryable lights (without it, items weigh their volume × 400 kg/m³).
pub fn add(app: &mut App, store: Store, manifest: Manifest, debug: bool, db: Option<Arc<RecordDb>>) {
    let ragdolls = ragdoll_source(store.clone(), manifest.clone());
    let (body_store, body_manifest) = (store.clone(), manifest.clone());
    let bodies = move |model: &str| -> Option<Arc<BodyShape>> {
        let digest = body_manifest.get(BODY_COOK, model)?;
        BodyShape::from_bytes(&body_store.get(&digest).ok()?).ok().map(Arc::new)
    };
    let items = MwItems::new(db.as_deref(), store.clone(), manifest.clone());
    let (support_store, support_manifest) = (store.clone(), manifest.clone());
    let supports = move |model: &str| -> Option<Arc<StaticShape>> {
        let mesh = CollisionMesh::from_bytes(&support_store.get(&support_manifest.get(SUPPORT_COOK, model)?).ok()?).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    };
    let source = move |model: &str| -> Option<Arc<StaticShape>> {
        let digest = manifest.get(COLLISION_COOK, model)?;
        let bytes = store.get(&digest).ok()?;
        let mesh = CollisionMesh::from_bytes(&bytes).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    };
    app.add_plugins(PhysicsPlugin { settings: PhysicsSettings::default(), ..PhysicsPlugin::default() })
        .insert_resource(PhysicsShapeSource(Arc::new(source)))
        .insert_resource(PhysicsBodySource { loader: Arc::new(bodies), items: Arc::new(items), supports: Some(Arc::new(supports)) })
        .insert_resource(PhysicsDebug { enabled: debug, ..PhysicsDebug::default() })
        .insert_resource(ragdolls)
        .init_resource::<Peaks>()
        .init_resource::<Balls>()
        .insert_resource(Auto::from_env())
        .add_systems(PostStartup, hook)
        .init_resource::<FpsCap>()
        .configure_sets(RunFixedMainLoop, sf_physics_bevy::PhysicsSystems::Views.after(run_freecamera_controller))
        .add_systems(
            Update,
            (
                toggle_debug,
                auto.before(throw_ball).before(kill_key),
                throw_ball,
                (kill_key, knockdown, got_up, ragdoll_log).chain(),
                peaks.after(sf_physics_bevy::PhysicsSystems::WriteBack),
                (attach_holder, carry_keys).chain().before(sf_physics_bevy::PhysicsSystems::Carry),
                carry_feedback.after(sf_physics_bevy::PhysicsSystems::Carry),
            ),
        )
        .add_systems(Last, cap_fps);
}

// --- Ragdolls (P7 debug driver) --------------------------------------------------------------

/// Seconds of death clip before the body goes limp (physics.md "Death": the
/// clip, then the powered crumple: `ReactionSet::death`, P8b).
pub const DEATH_HANDOFF: f32 = 0.4;

/// A character killed by [`kill_key`]: its mover stays [`Dormant`].
#[derive(Component)]
pub struct Dead;

/// `ragdoll.cook` products by skeleton subject (cached); skeletons whose
/// policy keeps the death clip get none.
fn ragdoll_source(store: Store, manifest: Manifest) -> RagdollSource {
    let cache: std::sync::Mutex<HashMap<String, Option<Arc<RagdollDef>>>> = std::sync::Mutex::default();
    RagdollSource(Arc::new(move |skeleton: &str| {
        let mut cache = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        cache
            .entry(skeleton.to_owned())
            .or_insert_with(|| {
                let bytes = store.get(&manifest.get(RAGDOLL_COOK, skeleton)?).ok()?;
                let def = RagdollDef::from_bytes(&bytes).ok()?;
                (def.policy == RagdollPolicy::Ragdoll).then(|| Arc::new(def))
            })
            .clone()
    }))
}

/// `I`: kills the character nearest the crosshair (within 40 m and 1 m of
/// the view ray): the graph's `dead` flag starts the death clip, the mover
/// parks, and [`Ragdoll`] hands the body to physics after [`DEATH_HANDOFF`].
#[allow(clippy::needless_pass_by_value)]
fn kill_key(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(Entity, &Transform), With<FreeCamera>>,
    pos_of: WorldPosOf,
    characters: Query<(Entity, &CharacterBody), Without<Dead>>,
    mut animators: Query<&mut Animator>,
    mut commands: Commands,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }
    let Some((cam, view)) = cameras.iter().next() else { return };
    let Some(eye) = pos_of.get(cam) else { return };
    let forward = view.rotation * Vec3::NEG_Z;
    let mut best: Option<(f32, Entity, &CharacterBody)> = None;
    for (e, body) in &characters {
        let Some(feet) = pos_of.get(e) else { continue };
        let d = eye.delta_f32(feet.offset(Vec3::Y));
        let along = d.dot(forward);
        let off = (d - forward * along).length();
        if (0.0..40.0).contains(&along) && off < 1.0 && best.is_none_or(|(b, ..)| off < b) {
            best = Some((off, e, body));
        }
    }
    let Some((_, root, body)) = best else {
        info!("ragdoll: nobody under the crosshair");
        return;
    };
    kill(root, body, &mut animators, &mut commands);
}

/// The graph's `dead` flag starts the death clip, the mover parks, and
/// [`Ragdoll`] hands the body to physics after [`DEATH_HANDOFF`].
fn kill(root: Entity, body: &CharacterBody, animators: &mut Query<&mut Animator>, commands: &mut Commands) {
    if let Ok(mut animator) = animators.get_mut(body.body) {
        animator.set_bool("dead", true);
    }
    commands.entity(root).try_insert((Dead, Dormant));
    commands.entity(body.body).try_insert(Ragdoll { skeleton: body.skeleton.clone(), delay: DEATH_HANDOFF, mode: RagdollMode::Death { crumple: true } });
    info!("ragdoll: killed {root} ({})", body.skeleton);
}

/// Impact momentum on a character's capsule (N·s) that knocks it down: the
/// P8b debug driver's threshold (a `V` ball at full throw). Removal: the
/// adapter's knockdown rule (fatigue and agility) in `mw_physics`.
pub const KNOCKDOWN_IMPULSE: f32 = 15.0;

/// Smallest impulse (N·s) a character reacts to with the graph's `HitReact`
/// op (P9); below [`KNOCKDOWN_IMPULSE`] the reaction is kinematic. Strength
/// is the impulse over [`KNOCKDOWN_IMPULSE`] (1: the op's full `gain`).
pub const HIT_REACT_IMPULSE: f32 = 1.0;

/// Seconds of knockdown clip before the handoff.
pub const KNOCKDOWN_HANDOFF: f32 = 0.05;

/// The graph flag a knockdown sets and the get-up clears (`mw_graphs` `knocked_out`).
const KNOCKED_OUT: &str = "knocked_out";

/// The get-up clip (`mw_graphs` `knockout_end` state): its first frame places
/// the character's root at the get-up ([`RagdollMode::Knockdown`] `get_up`).
const GET_UP_CLIP: &str = "Knockout@tail";

/// A knocked-down character: its mover stays [`Dormant`] until it gets up;
/// `body_local` is its body's local transform before the ragdoll moved it.
#[derive(Component)]
pub struct KnockedDown {
    body_local: Transform,
}

/// Hard impacts on a living character's capsule knock it down (PH-9);
/// lighter ones kick the graph's `HitReact` springs (P9, PH-10).
#[allow(clippy::needless_pass_by_value)]
fn knockdown(
    mut impacts: MessageReader<ImpactOnCharacter>,
    characters: Query<&CharacterBody, (Without<Dead>, Without<KnockedDown>)>,
    mut bodies: Query<(&mut Animator, &Transform, &GlobalTransform)>,
    pos_of: WorldPosOf,
    source: Res<RagdollSource>,
    mut commands: Commands,
) {
    let mut seen = Vec::new();
    for hit in impacts.read() {
        if hit.impulse < HIT_REACT_IMPULSE || seen.contains(&hit.entity) {
            continue;
        }
        let Ok(body) = characters.get(hit.entity) else { continue };
        let Ok((mut animator, local, global)) = bodies.get_mut(body.body) else { continue };
        seen.push(hit.entity);
        if hit.impulse < KNOCKDOWN_IMPULSE {
            // Into the model frame: the body's world rotation and scale.
            let Some(origin) = pos_of.get(body.body) else { continue };
            let (scale, rotation, _) = global.to_scale_rotation_translation();
            let inverse = rotation.inverse();
            let point = inverse * origin.delta_f32(hit.point) / scale.y.max(1e-6);
            let strength = hit.impulse / KNOCKDOWN_IMPULSE;
            animator.instance.hit(Hit { point, direction: inverse * hit.direction, strength, joint: None });
            info!("hit react: {} ({:.1} N·s, strength {strength:.2})", hit.entity, hit.impulse);
            continue;
        }
        animator.set_bool(KNOCKED_OUT, true);
        commands.entity(hit.entity).try_insert((KnockedDown { body_local: *local }, Dormant));
        let get_up = get_up_body(&source, &body.skeleton, &animator);
        if get_up.is_none() {
            info!("ragdoll: {} has no {GET_UP_CLIP} clip: LYING_REST_GET_UP scaffold", body.skeleton);
        }
        let mode = RagdollMode::Knockdown { point: hit.point, impulse: hit.direction * hit.impulse, get_up, get_up_flag: KNOCKED_OUT.to_owned() };
        commands.entity(body.body).try_insert(Ragdoll { skeleton: body.skeleton.clone(), delay: KNOCKDOWN_HANDOFF, mode });
        info!("ragdoll: knocked down {} ({:.0} N·s)", hit.entity, hit.impulse);
    }
}

/// The get-up clip's first-frame root body, from the character's graph.
fn get_up_body(source: &RagdollSource, skeleton: &str, animator: &Animator) -> Option<(Quat, Vec3)> {
    let def = (source.0)(skeleton)?;
    let slot = animator.asset.graph.clips.iter().position(|c| c.reference == GET_UP_CLIP)?;
    clip_root_body(&def, &animator.asset.skeleton, animator.asset.clips.get(slot)?, 0.0)
}

/// After a get-up the ragdoll moved the body; the walker root takes the move
/// (its mover too, heading included, or the mover would turn it back) and
/// the body goes back to its local transform.
#[allow(clippy::needless_pass_by_value)]
fn got_up(
    mut events: MessageReader<RagdollGotUp>,
    mut roots: Query<(Entity, &CharacterBody, &KnockedDown, &mut Transform, Option<&mut Mover>)>,
    mut locals: Query<&mut Transform, Without<KnockedDown>>,
    mut commands: Commands,
) {
    for e in events.read() {
        let Some((root, _, down, mut t, mover)) = roots.iter_mut().find(|(_, b, ..)| b.body == e.entity) else { continue };
        let Ok(mut local) = locals.get_mut(e.entity) else { continue };
        let l0 = down.body_local;
        let yaw = Mover::yaw_of(t.rotation * local.rotation * l0.rotation.inverse());
        let rotation = Quat::from_rotation_y(yaw);
        let shift = t.rotation * (t.scale * local.translation) - rotation * (t.scale * l0.translation);
        t.rotation = rotation;
        t.translation += shift;
        if let Some(mut m) = mover {
            m.loco.position = m.loco.position.offset(shift);
            m.loco.motion.yaw = yaw;
            m.loco.motion.facing_goal = yaw;
            m.loco.motion.angular_velocity = 0.0;
        }
        *local = l0;
        commands.entity(root).try_remove::<(KnockedDown, Dormant)>();
        info!("ragdoll: {root} got up (heading {:.0}°)", yaw.to_degrees());
    }
}

/// Logs ragdoll spawns, failures and the largest joint stretch.
#[allow(clippy::needless_pass_by_value)]
fn ragdoll_log(stats: Res<RagdollStats>, mut last: Local<(usize, usize)>, mut peak: Local<f32>) {
    if (stats.spawned, stats.failed) != *last {
        info!("ragdoll: {} live, {} spawned, {} failed (no ragdoll.cook product or a death-clip policy)", stats.live, stats.spawned, stats.failed);
        *last = (stats.spawned, stats.failed);
    }
    if stats.max_stretch > *peak + 0.002 {
        *peak = stats.max_stretch;
        info!("ragdoll: joint stretch peak {:.2} cm ({} awake)", stats.max_stretch * 100.0, stats.awake);
    }
}

// --- Carry, drag and throw (P6 debug driver) ---------------------------------------------

/// The dev camera carries (no player yet): hold the left mouse button on an
/// item under the crosshair to carry it (or drag it when it is heavier than
/// the carry mass), release to drop; `V` throws what is carried; `[` and `]`
/// turn it. Limits from Strength [`mw_physics::DEFAULT_STRENGTH`]
/// (`BEVYRROWIND_STRENGTH` overrides it). PH-4.
fn attach_holder(mut commands: Commands, cameras: Query<Entity, (With<FreeCamera>, Without<Holder>)>) {
    for e in &cameras {
        let strength = std::env::var("BEVYRROWIND_STRENGTH").ok().and_then(|s| s.parse().ok()).unwrap_or(mw_physics::DEFAULT_STRENGTH);
        let limits = mw_physics::holder_limits(strength);
        info!("carry: Strength {strength} -> {limits:?}");
        commands.entity(e).try_insert(Holder::new(limits));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn carry_keys(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    holders: Query<(Entity, &Holder)>,
    mut input: MessageWriter<CarryInput>,
) {
    const TURN: f32 = 0.26;
    for (e, h) in &holders {
        if mouse.just_pressed(MouseButton::Left) {
            input.write(CarryInput::Grab(e));
        }
        if h.carrier.held().is_none() {
            continue;
        }
        if mouse.just_released(MouseButton::Left) {
            input.write(CarryInput::Drop(e));
        } else if keys.just_pressed(KeyCode::KeyV) {
            input.write(CarryInput::Throw(e));
        }
        for (key, yaw) in [(KeyCode::BracketLeft, TURN), (KeyCode::BracketRight, -TURN)] {
            if keys.just_pressed(key) {
                input.write(CarryInput::Rotate { holder: e, yaw, pitch: 0.0 });
            }
        }
    }
}

/// Logs grab results; a drag slows the camera by the holder's speed scale.
#[allow(clippy::needless_pass_by_value)]
fn carry_feedback(
    mut feedback: MessageReader<CarryFeedback>,
    mut cameras: Query<(&Holder, &mut FreeCamera)>,
    mut base: Local<Option<(f32, f32)>>,
) {
    for f in feedback.read() {
        info!("carry: {:?}", f.outcome);
    }
    for (h, mut cam) in &mut cameras {
        let (walk, run) = *base.get_or_insert((cam.walk_speed, cam.run_speed));
        let s = h.speed_scale();
        if (cam.walk_speed - walk * s).abs() > 1e-4 {
            cam.walk_speed = walk * s;
            cam.run_speed = run * s;
        }
    }
}

/// `BEVYRROWIND_FPS_CAP=<fps>`: a dev frame limiter (sleep, then spin) for
/// the PH-4 carry check at 30, 60 and 144 fps. Use a non-vsync present mode
/// (`BEVYRROWIND_PRESENT=mailbox`) for caps above the display rate.
#[derive(Resource)]
struct FpsCap {
    frame: Option<std::time::Duration>,
    last: Option<std::time::Instant>,
}

impl Default for FpsCap {
    fn default() -> Self {
        let fps: Option<f64> = std::env::var("BEVYRROWIND_FPS_CAP").ok().and_then(|s| s.parse().ok()).filter(|f| *f > 0.0);
        Self { frame: fps.map(|f| std::time::Duration::from_secs_f64(1.0 / f)), last: None }
    }
}

fn cap_fps(mut cap: ResMut<FpsCap>) {
    let Some(frame) = cap.frame else { return };
    if let Some(last) = cap.last {
        let until = last + frame;
        let now = std::time::Instant::now();
        let wait = until.saturating_duration_since(now).saturating_sub(std::time::Duration::from_millis(2));
        if !wait.is_zero() {
            std::thread::sleep(wait);
        }
        while std::time::Instant::now() < until {
            std::hint::spin_loop();
        }
        cap.last = Some(if until > now { until } else { now });
    } else {
        cap.last = Some(std::time::Instant::now());
    }
}

/// The Morrowind item rules for the runtime (physics-adapter.md "Items").
struct MwItems {
    records: ItemRecords,
    /// `items.check` holds by placement id (held items only), read from the
    /// bake on a thread at startup; the first activation waits for it.
    holds: Arc<OnceLock<ItemsCheck>>,
}

/// What the runtime reads from every `items.check` product.
#[derive(Default)]
struct ItemsCheck {
    /// Held items by placement id.
    holds: HashMap<u128, Hold>,
    /// Statics simulated with their `support.cook` mesh.
    supports: HashSet<u128>,
}

impl MwItems {
    fn new(db: Option<&RecordDb>, store: Store, manifest: Manifest) -> Self {
        let holds = Arc::new(OnceLock::new());
        let fill = holds.clone();
        std::thread::spawn(move || {
            let mut read = ItemsCheck::default();
            for key in manifest.products.keys().filter(|k| k.step == ITEMS_CHECK) {
                let checks = manifest.get(&key.step, &key.subject).and_then(|d| store.get(&d).ok());
                let Some(Ok(checks)) = checks.map(|b| ItemChecks::from_bytes(&b)) else { continue };
                read.holds.extend(checks.items.iter().filter(|c| c.hold != Hold::No).map(|c| (c.id, c.hold)));
                read.supports.extend(checks.supports.iter().copied());
            }
            info!(
                "physics: items.check holds {} items until grabbed; {} statics with support meshes",
                read.holds.len(),
                read.supports.len()
            );
            let _ = fill.set(read);
        });
        Self { records: db.map(ItemRecords::new).unwrap_or_default(), holds }
    }
}

impl ItemContent for MwItems {
    fn is_item(&self, placement: &Placement) -> bool {
        self.records.is_item(placement)
    }

    fn item_body(&self, _: &WorldspaceId, placement: &Placement, volume: f32) -> ItemBody {
        let held = mw_physics::always_hold(placement) || self.holds.wait().holds.contains_key(&placement.id.0);
        ItemBody { mass: Some(self.records.mass(placement, volume)), hold: held }
    }

    fn is_support(&self, _: &WorldspaceId, placement: &Placement) -> bool {
        self.holds.wait().supports.contains(&placement.id.0)
    }
}

/// Wraps the spawners `exterior.rs` built (Startup) with the physics hook:
/// statics, doors and items; among the lights, the carryable ones.
fn hook(spawners: Option<ResMut<PlacementSpawners>>) {
    if let Some(mut spawners) = spawners {
        hook_physics(&mut spawners, &[PlacementClass::Static, PlacementClass::Door, PlacementClass::Dynamic]);
        hook_items(&mut spawners, &[PlacementClass::Light]);
    }
}

/// Debug balls thrown with `V`, oldest first.
#[derive(Resource, Default)]
struct Balls {
    live: VecDeque<Entity>,
    /// A throw asked for by the dev hook ([`auto`]): from, velocity.
    request: Option<(sf_coords::WorldPos, Vec3)>,
    look: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

const BALL_RADIUS: f32 = 0.12;
const BALL_MASS: f32 = 3.0;
const BALL_SPEED: f32 = 12.0;
const BALLS_KEPT: usize = 8;

/// `V`: a ball from the camera along the view, a PH-1 push without a player
/// controller (physics-adapter.md test scenes).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn throw_ball(
    keys: Res<ButtonInput<KeyCode>>,
    holders: Query<&Holder>,
    mut commands: Commands,
    camera: Query<(&CellCoord, &Transform, &ChildOf, Option<&RenderLayers>), With<Camera3d>>,
    active: Res<ActiveWorldspace>,
    mut physics: ResMut<Physics>,
    mut balls: ResMut<Balls>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // `V` throws the carried item instead while something is held.
    let requested = balls.request.take();
    if requested.is_none() && (!keys.just_pressed(KeyCode::KeyV) || holders.iter().any(|h| h.carrier.held().is_some())) {
        return;
    }
    let (Some((cell, t, parent, layers)), Some(ws)) = (camera.iter().next(), active.0.clone()) else { return };
    let eye = from_grid(cell, t);
    let forward = t.forward().as_vec3();
    let (from, velocity) = requested.unwrap_or((eye.offset(forward * 0.6), forward * BALL_SPEED));
    let pose = WorldTransform { pos: from, ..WorldTransform::IDENTITY };
    let ball = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Sphere(BALL_RADIUS), Layer::PROP);
    let id = physics.insert(&ws, &BodyDesc { mass: Some(BALL_MASS), ccd: true, ..ball });
    if let Some(w) = physics.world_mut(&ws) {
        w.set_velocity(id, velocity, Vec3::ZERO);
    }
    let (mesh, material) = balls
        .look
        .get_or_insert_with(|| {
            let material = StandardMaterial { base_color: Color::srgb(0.9, 0.25, 0.1), perceptual_roughness: 0.6, ..default() };
            (meshes.add(Sphere::new(BALL_RADIUS)), materials.add(material))
        })
        .clone();
    let (grid, local) = transform_to_grid(&pose);
    let body = PhysicsBody { worldspace: ws, id };
    let mut entity = commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), grid, local, ChildOf(parent.parent()), body));
    if let Some(layers) = layers {
        entity.insert(layers.clone());
    }
    balls.live.push_back(entity.id());
    while balls.live.len() > BALLS_KEPT {
        if let Some(old) = balls.live.pop_front() {
            commands.entity(old).try_despawn();
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn toggle_debug(keys: Res<ButtonInput<KeyCode>>, mut draw: ResMut<PhysicsDebug>) {
    if keys.just_pressed(KeyCode::KeyG) {
        draw.enabled = !draw.enabled;
        let on = draw.enabled;
        info!("physics colliders {}", if on { "on" } else { "off" });
    }
}

/// Worst frames since the last report.
#[derive(Resource, Default)]
struct Peaks {
    since: f32,
    frames: u32,
    step_ms: f32,
    step_frame_ms: f32,
    insert_ms: f32,
    inserts: u32,
    over_budget: u32,
    dropped: u32,
    rebases: u32,
    /// Most bodies awake at once (PH-7: 0 with nobody near the items).
    awake: usize,
    /// Character capsules (most at once), pushes, the largest push (N·s) and
    /// impacts on characters (P5).
    capsules: usize,
    pushes: u32,
    max_push: f32,
    impacts: u32,
}

/// physics.md budgets: an idle R0 step ≤ 0.3 ms, insertion ≤ 0.5 ms per frame.
const STEP_BUDGET_MS: f32 = 0.3;
const INSERT_BUDGET_MS: f32 = 0.5;

#[allow(clippy::needless_pass_by_value)]
fn peaks(
    time: Res<Time>,
    stats: Res<PhysicsStats>,
    chars: Option<Res<CharacterStats>>,
    mut p: ResMut<Peaks>,
    physics: Res<Physics>,
    streaming: Res<PhysicsStreaming>,
) {
    if p.since + time.delta_secs() >= 10.0 {
        // Items that never sleep (PH-7: none with nobody near).
        for (id, start, moved, v, w) in streaming.awake_items(&physics).into_iter().take(6) {
            let at = start.pos.to_f64();
            info!(
                "physics: awake item {:032x} placed at {:.2} {:.2} {:.2}: moved {moved:.3} m, {v:.3} m/s, {w:.2} rad/s",
                id.0, at.x, at.y, at.z
            );
        }
    }
    let s = *stats;
    p.frames += 1;
    p.step_ms = p.step_ms.max(s.max_step_ms);
    p.step_frame_ms = p.step_frame_ms.max(s.step_ms);
    p.insert_ms = p.insert_ms.max(s.insert_ms);
    p.inserts = p.inserts.max(s.inserts);
    p.over_budget += u32::from(s.max_step_ms > STEP_BUDGET_MS || s.insert_ms > INSERT_BUDGET_MS);
    p.dropped += s.dropped_steps;
    p.rebases += s.rebases;
    p.awake = p.awake.max(s.awake);
    if let Some(c) = chars.as_deref() {
        p.capsules = p.capsules.max(c.capsules);
        p.pushes += c.pushed;
        p.max_push = p.max_push.max(c.max_push);
        p.impacts += c.impacts;
    }
    p.since += time.delta_secs();
    if p.since < 10.0 {
        return;
    }
    info!(
        "physics: {} bodies ({} items, {} terrain chunks, {} pending, {} awake, peak {}) | peaks over {} frames: step {:.3} ms \
         ({:.3} ms/frame), insert {:.3} ms/frame ({} bodies) | {} frames over budget, {} dropped steps, {} rebases | {} capsules, \
         {} pushes (max {:.2} N·s), {} impacts",
        s.bodies,
        s.items,
        s.terrain_chunks,
        s.pending,
        s.awake,
        p.awake,
        p.frames,
        p.step_ms,
        p.step_frame_ms,
        p.insert_ms,
        p.inserts,
        p.over_budget,
        p.dropped,
        p.rebases,
        p.capsules,
        p.pushes,
        p.max_push,
        p.impacts
    );
    *p = Peaks::default();
}

// --- Dev hook: scripted actions (agents verify without a keyboard) -------------------------

/// Seconds `ball_at_nearest` waits after framing a new target before it throws.
const AUTO_FRAME_DELAY: f32 = 0.5;

/// `BEVYRROWIND_AUTO="kill_nearest@3,ball_at_nearest@6,shot:shots/a.png@7,exit@12"`: actions at
/// seconds after the start settled. `kill_nearest` is `I` on the living character nearest the
/// camera; `ball_at_nearest[:speed]` throws the `V` ball (default 12 m/s, 3 kg) on a ballistic
/// arc at the chest of the nearest living, standing one (`:2` stays under the knockdown
/// impulse: a `HitReact`); both frame that character from 4 m (a throw at a new target waits
/// [`AUTO_FRAME_DELAY`] for the framing). `look` re-frames the last
/// target, `shot:<png>` saves a screenshot, `exit` quits.
#[derive(Resource, Default)]
struct Auto {
    actions: Vec<(f32, AutoAction)>,
    start: Option<f32>,
    /// The last target: the camera frames it.
    focus: Option<Entity>,
}

#[derive(Clone, Debug, PartialEq)]
enum AutoAction {
    KillNearest,
    BallAtNearest(f32),
    Look,
    Shot(std::path::PathBuf),
    Exit,
}

impl Auto {
    fn from_env() -> Self {
        let Ok(spec) = std::env::var("BEVYRROWIND_AUTO") else { return Self::default() };
        let mut actions = Vec::new();
        for item in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let Some((what, at)) = item.rsplit_once('@') else { continue };
            let Ok(at) = at.parse::<f32>() else { continue };
            let (name, arg) = what.split_once(':').unwrap_or((what, ""));
            let action = match name {
                "kill_nearest" => AutoAction::KillNearest,
                "ball_at_nearest" => AutoAction::BallAtNearest(arg.parse().unwrap_or(BALL_SPEED)),
                "look" => AutoAction::Look,
                "shot" if !arg.is_empty() => AutoAction::Shot(arg.into()),
                "exit" => AutoAction::Exit,
                _ => {
                    warn!("BEVYRROWIND_AUTO: unknown action {item:?}");
                    continue;
                }
            };
            actions.push((at, action));
        }
        actions.sort_by(|a, b| a.0.total_cmp(&b.0));
        info!("BEVYRROWIND_AUTO: {actions:?}");
        Self { actions, start: None, focus: None }
    }
}

/// Runs the due [`Auto`] actions and keeps the camera on the target.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn auto(
    time: Res<Time<Real>>,
    settled: Option<Res<crate::exterior::Settled>>,
    mut auto: ResMut<Auto>,
    // Read-only: `WorldPosOf` reads every Transform; the framed pose is written back through `commands`.
    cameras: Query<(Entity, &Transform), With<FreeCamera>>,
    pos_of: WorldPosOf,
    characters: Query<(Entity, &CharacterBody, Has<Dead>, Has<KnockedDown>)>,
    movers: Query<&Mover>,
    mut animators: Query<&mut Animator>,
    mut balls: ResMut<Balls>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    if auto.actions.is_empty() || settled.is_none() {
        return;
    }
    let now = time.elapsed_secs();
    let start = *auto.start.get_or_insert(now);
    let Some((cam, current)) = cameras.iter().next() else { return };
    let mut view = *current;
    let Some(eye) = pos_of.get(cam) else { return };
    let nearest = |alive_only: bool| {
        characters
            .iter()
            .filter(|(_, _, dead, down)| !alive_only || (!dead && !down))
            .filter_map(|(e, b, ..)| Some((pos_of.get(e)?.delta_f32(eye).length(), e, b)))
            .filter(|(d, ..)| *d < 60.0)
            .min_by(|a, b| a.0.total_cmp(&b.0))
    };
    while auto.actions.first().is_some_and(|(at, _)| now - start >= *at) {
        let (at, action) = auto.actions.remove(0);
        info!("BEVYRROWIND_AUTO @{at}: {action:?}");
        match action {
            AutoAction::KillNearest => {
                if let Some((_, root, body)) = nearest(true) {
                    kill(root, body, &mut animators, &mut commands);
                    auto.focus = Some(root);
                } else {
                    info!("BEVYRROWIND_AUTO: nobody within 60 m");
                }
            }
            AutoAction::BallAtNearest(speed) => {
                let Some((_, root, _)) = nearest(true) else { continue };
                let Some(feet) = pos_of.get(root) else { continue };
                if auto.focus != Some(root) {
                    // A new target: frame it first and throw once the camera is there (a throw
                    // in the frame the camera jumps missed in the 2026-10-18 runs).
                    auto.focus = Some(root);
                    auto.actions.insert(0, (at + AUTO_FRAME_DELAY, AutoAction::BallAtNearest(speed)));
                    continue;
                }
                // Thrown from the framing camera: frame first, then aim from there.
                let to_eye = frame_offset(eye, feet);
                let from = feet.offset(to_eye);
                let chest = feet.offset(Vec3::Y * 1.2);
                let aim = from.delta_f32(chest).normalize_or_zero();
                // Released on the camera-chest line, at most a quarter second out: a slow ball
                // lobbed from 4 m lands at ~6 m/s (≥ 15 N·s, a knockdown), so slow throws start
                // closer (0.6 m minimum) and stay under the knockdown impulse.
                let reach = (speed.max(0.5) * 0.25).min(from.delta_f32(chest).length() - 0.4).max(0.6);
                let flight = reach / speed.max(0.5);
                // Ballistic: v = d/t + g·t/2 up, so it lands on the chest after `flight`; plus the
                // target's walking velocity, so it meets the chest where it will be (lead).
                let lead = movers.get(root).map_or(Vec3::ZERO, |m| m.loco.motion.velocity);
                let velocity = aim * speed.max(0.5) + lead + Vec3::Y * (9.81 * flight / 2.0);
                view.translation += eye.delta_f32(from);
                balls.request = Some((chest.offset(-aim * reach), velocity));
                info!("BEVYRROWIND_AUTO: ball at {root}, {speed} m/s over {reach:.1} m");
            }
            AutoAction::Look => {}
            AutoAction::Shot(path) => {
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
            }
            AutoAction::Exit => {
                exit.write(AppExit::Success);
            }
        }
    }
    // Frame the target: 4 m off, 1.8 m up, looking at its chest.
    if let Some(feet) = auto.focus.and_then(|e| pos_of.get(e)) {
        let eye_now = pos_of.get(cam).unwrap_or(eye);
        let want = feet.offset(frame_offset(eye_now, feet));
        view.translation += eye_now.delta_f32(want);
        let look = want.delta_f32(feet.offset(Vec3::Y * 1.0));
        if look.length_squared() > 1e-4 {
            view.rotation = Transform::IDENTITY.looking_to(look, Vec3::Y).rotation;
        }
    }
    if view != *current {
        commands.entity(cam).insert(view);
    }
}

/// Where the framing camera sits relative to a character's feet: 4 m back
/// toward the current eye, 1.8 m up.
fn frame_offset(eye: sf_coords::WorldPos, feet: sf_coords::WorldPos) -> Vec3 {
    let back = feet.delta_f32(eye);
    let flat = Vec3::new(back.x, 0.0, back.z).normalize_or(Vec3::Z);
    flat * 4.0 + Vec3::Y * 1.8
}
