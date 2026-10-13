//! NPCs walking the streets (ROADMAP M3 demo): `sf_controller_bevy` on the
//! characters `sf_character_bevy` spawns, driven by `mw_ai`'s
//! `PathWalkScaffold` (PGRD pathgrids + `AI_W` wander) until `sf_ai` exists.
//!
//! - Ground ([`MwGround`]): the resident terrain heights of the exterior
//!   plus a `StaticWorld` of the R0 statics' `collision.cook` shapes
//!   (inserted and removed by the streaming activation hooks,
//!   [`ColliderHook`]). Physics colliders wait for avian (M4).
//! - Every dressed character gets a `Mover` (grounded, an obstacle to the
//!   others); characters with an `AI_W` package also get a [`Walker`]
//!   (the scaffold) and an `Avoider` (ORCA, reactive). The camera is a
//!   non-reactive obstacle (`AvoidObstacle`), as the player will be.
//! - The animation graph gets `moving`/`move`/`running` from the mover's
//!   velocity (the graph's `SpeedMatch` op matches playback to speed), and
//!   `AI_W` idles fire `fidget` with `idle_variant`.
//!
//! - Processing range ([`processing_range`]): a mover simulates only in the
//!   active worldspace where every streaming cell around it is R0-active
//!   (its colliders are loaded); elsewhere it is `Dormant` (parked, scaffold
//!   paused, low animation significance). Interiors walk their own PGRD
//!   (the worldspace id is the cell name). Flying creatures (`FLIES`) use
//!   the mover's `Fly { hover }` mode.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use mw_ai::{CellKey, PathGrids, PathWalkScaffold, Wander, WanderPlanner, gmst_float, walk_speed};
use mw_graphs::build::params as gp;
use sf_anim_bevy::{AnimLod, AnimStats, Animator};
use sf_bake::{Manifest, Store};
use sf_character_bevy::{Character, CharacterBody};
use sf_controller::steps::COLLISION_COOK;
use sf_controller::{
    CollisionMesh, GroundHit, GroundProbe, Nearest, ProbeQuery, StaticId, StaticShape, StaticWorld,
    TerrainProbe,
};
use sf_controller_bevy::{
    AnimParamNames, AvoidObstacle, Avoider, ControllerGizmos, ControllerSettings,
    ControllerSystems, Dormant, MoveIntent, Mover, MoverMode, MoverParams, SfControllerPlugin,
};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId};
use sf_terrain_bevy::TerrainHeightsMap;
use sf_terrain_bevy::sf_terrain::TerrainHeights;
use sf_world::{IndexedPlacement, Level, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::coords::WorldPosOf;
use sf_world_bevy::{
    ActivateCx, ActiveWorldspace, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx,
    Streaming,
};
use tes3::esp::{Creature, CreatureFlags, Npc};

/// Morrowind's default timescale: game seconds per real second.
const TIMESCALE: f32 = 30.0;
/// Hover height of flying creatures over the ground, metres (cliff racers glide low).
const FLY_HOVER: f32 = 1.5;
/// Half size of the square around a mover whose streaming cells must all be
/// R0-active for it to simulate, m: a walk step plus the capsule, so a mover
/// parks before it could reach a cell without colliders.
const PROCESS_MARGIN: f32 = 4.0;
/// Animation significance of dormant movers (the budget drops them to a low tier).
const DORMANT_SIGNIFICANCE: f32 = 0.25;

// --- Ground ---------------------------------------------------------------------------------

/// One worldspace's R0 static collision.
#[derive(Default)]
struct WsStatics {
    world: Option<StaticWorld>,
    placed: HashMap<PlacementId, StaticId>,
    dirty: bool,
}

/// Terrain heights + R0 static collision: the controller's ground.
///
/// Statics are kept per worldspace (an interior's coordinates overlap the
/// exterior's, and door prefetch activates R0 cells of other worldspaces);
/// probes see the active worldspace only, plus the terrain when that is
/// the exterior. Movers elsewhere are [`Dormant`] (the processing range).
#[derive(Resource, Default)]
pub struct MwGround {
    terrain: Option<TerrainProbe<TerrainHeights>>,
    worlds: HashMap<WorldspaceId, WsStatics>,
    shapes: HashMap<String, Option<Arc<StaticShape>>>,
    active: Option<WorldspaceId>,
    exterior_active: bool,
}

impl MwGround {
    fn placed(&self) -> usize {
        self.worlds.values().map(|w| w.placed.len()).sum()
    }
}

impl GroundProbe for MwGround {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        let terrain = self.terrain.as_ref().filter(|_| self.exterior_active);
        let statics = self
            .active
            .as_ref()
            .and_then(|ws| self.worlds.get(ws))
            .and_then(|w| w.world.as_ref());
        match (terrain, statics) {
            (Some(t), Some(s)) => Nearest(&[t, s]).probe(queries, out),
            (Some(t), None) => t.probe(queries, out),
            (None, Some(s)) => s.probe(queries, out),
            (None, None) => out.fill(None),
        }
    }
}

/// The walkers' ground runs on physics (`PhysicsProbe` over the active
/// world: statics, terrain chunks and props, physics.md "Queries"); false
/// with `--off physics`, where [`MwGround`]'s `StaticWorld` stays the ground.
#[derive(Resource, Clone, Copy)]
struct GroundOnPhysics(bool);

/// The ground the mover uses, for the scaffold's own probes and stats.
#[derive(bevy::ecs::system::SystemParam)]
struct Ground<'w> {
    mw: Res<'w, MwGround>,
    physics: Option<Res<'w, sf_physics_bevy::Physics>>,
    on: Res<'w, GroundOnPhysics>,
}

impl Ground<'_> {
    fn on_physics(&self) -> bool {
        self.on.0 && self.physics.is_some()
    }

    fn probe(&self) -> &dyn GroundProbe {
        match &self.physics {
            Some(p) if self.on.0 => &**p,
            _ => &*self.mw,
        }
    }
}

/// A collider change from the activation hooks: (worldspace, placement,
/// model and transform to add, or `None` to remove).
type ColliderOp = (WorldspaceId, PlacementId, Option<(String, WorldTransform)>);

/// Collider changes from the activation hooks, applied once per frame.
#[derive(Resource, Default)]
struct ColliderQueue(Vec<ColliderOp>);

/// Where `collision.cook` products come from.
#[derive(Resource)]
struct CollisionSource {
    store: Store,
    manifest: Manifest,
}

impl CollisionSource {
    fn shape(&self, model: &str) -> Option<Arc<StaticShape>> {
        let digest = self.manifest.get(COLLISION_COOK, model)?;
        let bytes = self.store.get(&digest).ok()?;
        let mesh = CollisionMesh::from_bytes(&bytes).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    }
}

/// Wraps a placement spawner: R0 activation adds the placement's static
/// collision shape to [`MwGround`], deactivation removes it.
pub struct ColliderHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for ColliderHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(
        &self,
        assets: &AssetServer,
        placement: &IndexedPlacement,
        model: Option<&LoadedModel>,
    ) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        if let (Some(model), Some(mut queue)) =
            (&p.model, world.get_resource_mut::<ColliderQueue>())
        {
            queue.0.push((
                cx.worldspace.clone(),
                p.id,
                Some((model.as_str().to_owned(), p.transform)),
            ));
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        if let Some(mut queue) = world.get_resource_mut::<ColliderQueue>() {
            queue
                .0
                .push((cx.worldspace.clone(), placement.placement.id, None));
        }
    }
}

/// Wraps the static classes' spawners with [`ColliderHook`] (call after the
/// spawners are final).
pub fn hook_colliders(spawners: &mut PlacementSpawners) {
    for class in [
        PlacementClass::Static,
        PlacementClass::Dynamic,
        PlacementClass::Door,
    ] {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(ColliderHook(inner)));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn apply_colliders(
    mut queue: ResMut<ColliderQueue>,
    source: Res<CollisionSource>,
    active: Res<ActiveWorldspace>,
    mut ground: ResMut<MwGround>,
    on: Res<GroundOnPhysics>,
    counters: Option<Res<sf_diag_bevy::DiagCounters>>,
) {
    if active.0 != ground.active {
        ground.exterior_active = active.0.as_ref() == Some(&mw_world::ids::exterior());
        ground.active.clone_from(&active.0);
    }
    if on.0 {
        // On physics: its activation hooks build the statics; none here.
        queue.0.clear();
    }
    if queue.0.is_empty() {
        return;
    }
    let started = std::time::Instant::now();
    let MwGround { worlds, shapes, .. } = &mut *ground;
    for (ws, id, op) in queue.0.drain(..) {
        let w = worlds.entry(ws).or_default();
        if let (Some(old), Some(s)) = (w.placed.remove(&id), w.world.as_mut()) {
            s.remove(old);
            w.dirty = true;
        }
        let Some((model, transform)) = op else {
            continue;
        };
        let Some(shape) = shapes
            .entry(model)
            .or_insert_with_key(|m| source.shape(m))
            .clone()
        else {
            continue;
        };
        let world = w
            .world
            .get_or_insert_with(|| StaticWorld::new(transform.pos));
        w.placed.insert(id, world.insert(&shape, transform));
        w.dirty = true;
    }
    worlds.retain(|_, w| {
        if let (true, Some(s)) = (std::mem::take(&mut w.dirty), w.world.as_mut()) {
            s.rebuild();
        }
        !w.placed.is_empty()
    });
    if let Some(c) = counters {
        c.add_named(
            "controller/colliders/ms",
            sf_diag_bevy::sf_diag::unit::MS,
            started.elapsed().as_secs_f64() * 1e3,
        );
        #[allow(clippy::cast_precision_loss)]
        c.set_named(
            "controller/statics/count",
            sf_diag_bevy::sf_diag::unit::COUNT,
            ground.placed() as f64,
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn refresh_terrain(map: Res<TerrainHeightsMap>, mut ground: ResMut<MwGround>) {
    if !map.is_changed() && ground.terrain.is_some() {
        return;
    }
    if let Some(h) = map.0.get(&mw_world::ids::exterior()) {
        ground.terrain = Some(TerrainProbe(h.clone()));
    }
}

// --- Walkers --------------------------------------------------------------------------------

/// The pathgrids of every cell.
#[derive(Resource)]
struct Pathgrids(PathGrids);

/// Records and game settings the walkers read.
#[derive(Resource)]
struct WalkerContent {
    db: Arc<RecordDb>,
    min_walk: f32,
    max_walk: f32,
}

/// A wandering character's scaffold driver (`mw_ai`, named scaffold).
#[derive(Component)]
pub struct Walker {
    scaffold: PathWalkScaffold,
    variant: Option<u8>,
    /// The pathgrid it walks: its exterior cell at spawn, or its interior.
    key: CellKey,
}

/// The worldspace a mover lives in (NPCs never change worldspace in M3).
#[derive(Component, Clone, Debug)]
pub struct ActorSpace(pub WorldspaceId);

/// Mover and walker setup for a dressed character's base record.
struct Setup {
    params: MoverParams,
    mode: MoverMode,
    wander: Option<(Wander, f32)>,
}

fn setup_of(content: &WalkerContent, id: &RecordId) -> Setup {
    // A leveled list (LEVC) walks as the creature (or NPC) it shows: the same
    // pick `MwActors::recipe` dresses it with.
    let picked = mw_actors::leveled_pick(&content.db, id).map(|(_, picked)| picked);
    let id = picked.as_ref().unwrap_or(id);
    let speed_of = |speed: f32| walk_speed(speed, content.min_walk, content.max_walk);
    if let Some(npc) = content.db.get::<Npc>(id) {
        let speed = npc
            .data
            .stats
            .as_ref()
            .map_or(50.0, |s| f32::from(s.attributes[4]));
        let wander = Wander::of_npc(npc)
            .filter(|w| w.distance > 0.0)
            .map(|w| (w, speed_of(speed)));
        return Setup {
            params: MoverParams::default(),
            mode: MoverMode::Ground,
            wander,
        };
    }
    if let Some(c) = content.db.get::<Creature>(id) {
        #[allow(clippy::cast_precision_loss)]
        let speed = speed_of(c.data.speed as f32);
        let wander = Wander::of_creature(c)
            .filter(|w| w.distance > 0.0)
            .map(|w| (w, speed));
        let flies = c.creature_flags.contains(CreatureFlags::FLIES);
        let mode = if flies {
            MoverMode::Fly {
                hover: Some(FLY_HOVER),
            }
        } else {
            MoverMode::Ground
        };
        // Creature capsules scale with the record's scale; size by model bounds is the next step.
        let s = c.scale.unwrap_or(1.0).clamp(0.3, 3.0);
        let params = MoverParams {
            radius: 0.4 * s,
            height: 1.2 * s,
            ..MoverParams::default()
        };
        return Setup {
            params,
            mode,
            wander,
        };
    }
    Setup {
        params: MoverParams::default(),
        mode: MoverMode::Ground,
        wander: None,
    }
}

/// Gives dressed characters their mover (and wanderers their walker).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn attach(
    mut commands: Commands,
    content: Res<WalkerContent>,
    pos_of: WorldPosOf,
    fresh: Query<(Entity, &Character, &CharacterBody, &Transform, &ChildOf), Without<Mover>>,
    bodies: Query<&Transform>,
) {
    for (entity, character, body, transform, parent) in &fresh {
        let Character::Record { id, .. } = character else {
            continue;
        };
        let (Some(location), Some(frame_origin)) =
            (pos_of.location(entity), pos_of.get(parent.parent()))
        else {
            continue;
        };
        let position = location.pos;
        let setup = setup_of(&content, id);
        let mut mover = Mover::new(
            position,
            Mover::yaw_of(transform.rotation),
            setup.params,
            frame_origin,
        );
        mover.loco.mode = setup.mode;
        mover.animator = Some(body.body);
        // Race height/weight: the body's scale (foot probes, motion feed).
        if let Ok(t) = bodies.get(body.body) {
            mover.body_scale = t.scale;
        }
        let mut e = commands.entity(entity);
        e.insert((
            mover,
            MoveIntent::default(),
            ActorSpace(location.worldspace.clone()),
        ));
        if let Some((wander, speed)) = setup.wander {
            let seed = entity.to_bits() ^ 0x5741_4C4B;
            let scaffold = PathWalkScaffold::new(WanderPlanner::new(wander, position), speed, seed);
            let key = if location.worldspace == mw_world::ids::exterior() {
                CellKey::exterior_at(position)
            } else {
                CellKey::interior(location.worldspace.as_str())
            };
            e.insert((
                Walker {
                    scaffold,
                    variant: None,
                    key,
                },
                Avoider {
                    max_speed: speed * 1.5,
                    ..Avoider::default()
                },
            ));
        }
    }
}

/// Dead and knocked-down movers stay parked (physics.rs).
type Unprocessed = (Without<crate::physics::Dead>, Without<crate::physics::KnockedDown>);

/// The processing range (streaming.md / locomotion.md "Processing range"):
/// a mover simulates only in the active worldspace and only while every
/// streaming cell within [`PROCESS_MARGIN`] of it is R0-active, i.e. its
/// ground and static colliders are loaded (R0 activation builds them).
/// Elsewhere it is [`Dormant`]: parked where it stands, its scaffold paused,
/// its animation significance lowered so the budget gives it a low tier.
#[allow(clippy::needless_pass_by_value)]
fn processing_range(
    mut commands: Commands,
    streaming: Res<Streaming>,
    active: Res<ActiveWorldspace>,
    movers: Query<(Entity, &Mover, &ActorSpace, Has<Dormant>), Unprocessed>,
    mut lods: Query<&mut AnimLod>,
) {
    let scheduler = streaming.scheduler();
    for (entity, mover, space, dormant) in &movers {
        let awake = active.0.as_ref() == Some(&space.0)
            && scheduler.covers(&space.0, mover.loco.position, PROCESS_MARGIN, Level::Active);
        if awake != dormant {
            continue;
        }
        if awake {
            commands.entity(entity).remove::<Dormant>();
        } else {
            commands.entity(entity).insert(Dormant);
        }
        if let Some(mut lod) = mover.animator.and_then(|a| lods.get_mut(a).ok()) {
            lod.significance = if awake { 1.0 } else { DORMANT_SIGNIFICANCE };
        }
    }
}

/// The scaffold writes intents (and route directions for ORCA's yield rule).
#[allow(clippy::needless_pass_by_value)]
fn drive(
    time: Res<Time>,
    grids: Res<Pathgrids>,
    mut walkers: Query<(Entity, &Mover, &mut Walker, &mut MoveIntent, &mut Avoider), Without<Dormant>>,
    ground: Ground,
    mut give_ups: ResMut<GiveUps>,
    mut animators: Query<&mut Animator>,
) {
    let dt = time.delta_secs().min(0.1);
    let hours = dt * TIMESCALE / 3600.0;
    for (entity, mover, mut walker, mut intent, mut avoider) in &mut walkers {
        let pos = mover.loco.position;
        let walker = &mut *walker;
        let graph = grids.0.get(&walker.key);
        let (stuck, edge) = (walker.scaffold.stuck_count(), current_edge(&walker.scaffold, pos));
        let out = walker.scaffold.update(graph, pos, dt, hours);
        if walker.scaffold.stuck_count() > stuck {
            let kind = give_ups.record(entity, mover, edge, ground.probe());
            if let Some((from, to)) = edge {
                match graph.filter(|g| g.node_at(to).is_some()) {
                    None => give_ups.off_grid += 1,
                    Some(g) if g.node_at(from).is_none() => give_ups.first_leg += 1,
                    Some(_) => {}
                }
            }
            // The mover found the edge unwalkable: route around it, not idle.
            if kind != "other"
                && let Some((from, to)) = edge
                && walker.scaffold.replan_around(graph, from, to, pos)
            {
                give_ups.rerouted += 1;
            }
        }
        intent.0 = out.intent;
        avoider.travel = walker.scaffold.follower().map_or(Vec3::ZERO, |f| {
            let t = f.target();
            let from = if t > 0 { f.nodes[t - 1].pos } else { pos };
            f.nodes.get(t).map_or(Vec3::ZERO, |n| {
                from.delta_f32(n.pos).with_y(0.0).normalize_or_zero()
            })
        });
        // AI_W idles: a new IdleN (2..=9) plays once as a fidget.
        let variant = (!out.walking).then_some(out.idle_variant.unwrap_or(0));
        if variant != walker.variant {
            walker.variant = variant;
            if let (Some(v @ 2..=9), Some(mut animator)) = (
                variant,
                mover.animator.and_then(|a| animators.get_mut(a).ok()),
            ) {
                animator.set_float(gp::IDLE_VARIANT, f32::from(v));
                animator.fire(gp::FIDGET);
            }
        }
    }
}

/// The camera as a non-reactive obstacle at its feet (about 1.2 m below the
/// eye), so walkers step around a camera standing in the street.
#[allow(clippy::needless_pass_by_value)]
fn camera_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    pos_of: WorldPosOf,
    mut cams: Query<(Entity, Option<&mut AvoidObstacle>), With<crate::exterior::ExteriorCamera>>,
) {
    let dt = time.delta_secs().max(1e-4);
    for (entity, existing) in &mut cams {
        let Some(eye) = pos_of.get(entity) else {
            continue;
        };
        let feet = eye.offset(Vec3::Y * -1.2);
        if let Some(mut o) = existing {
            let v = o.position.delta_f32(feet) / dt;
            o.velocity = if v.length() < 15.0 {
                v.with_y(0.0)
            } else {
                Vec3::ZERO
            };
            o.position = feet;
        } else {
            commands.entity(entity).insert(AvoidObstacle {
                position: feet,
                velocity: Vec3::ZERO,
                radius: 0.4,
            });
        }
    }
}

/// `J` toggles the mover gizmos (capsule ring, velocity, trajectory), `K` foot placement.
#[allow(clippy::needless_pass_by_value)]
fn gizmos(
    keys: Res<ButtonInput<KeyCode>>,
    mut g: ResMut<ControllerGizmos>,
    mut settings: ResMut<ControllerSettings>,
    grid: Query<&sf_world_bevy::big_space::prelude::Grid>,
) {
    if keys.just_pressed(KeyCode::KeyJ) {
        g.enabled = !g.enabled;
        info!("walker gizmos {}", if g.enabled { "on" } else { "off" });
    }
    // A/B for foot placement: K drops the per-leg ground samples.
    if keys.just_pressed(KeyCode::KeyK) {
        settings.foot_probes = !settings.foot_probes;
        info!(
            "walker foot placement {}",
            if settings.foot_probes { "on" } else { "off" }
        );
    }
    if let Some(grid) = grid.iter().next() {
        let c = grid.local_floating_origin().cell();
        g.origin = WorldPos::new(IVec3::new(c.x, c.y, c.z), Vec3::ZERO);
    }
}

/// Adds the walkers: controller stage, ground, pathgrids, scaffold driver.
pub fn add(app: &mut App, db: Arc<RecordDb>, store: Store, manifest: Manifest, physics: bool) {
    let started = std::time::Instant::now();
    let grids = PathGrids::build(&db);
    let min_walk = gmst_float(&db, "fMinWalkSpeed", 100.0);
    let max_walk = gmst_float(&db, "fMaxWalkSpeed", 200.0);
    info!(
        "walkers: {} pathgrids in {:.2} s",
        grids.grids.len(),
        started.elapsed().as_secs_f32()
    );
    let params = AnimParamNames {
        moving: Some(gp::MOVING.into()),
        direction: Some(gp::MOVE.into()),
        running: Some(gp::RUNNING.into()),
        turn: Some(gp::TURN.into()),
        speed: None,
    };
    // The mover probes physics when it runs (P5), else the `StaticWorld`.
    if physics {
        app.add_plugins(SfControllerPlugin::<sf_physics_bevy::Physics>::default());
    } else {
        app.add_plugins(SfControllerPlugin::<MwGround>::default());
    }
    app.insert_resource(GroundOnPhysics(physics))
        .insert_resource(ControllerSettings {
            params,
            // Measured-speed thresholds against the humanoid cycles (walk
            // 2.2 m/s, run 3.2 m/s, `SpeedMatch` 0.5-1.5, stride warp 0.6+):
            // moving from 0.5 m/s down to 0.25; running from 2.9 down to 2.6.
            moving_speed: 0.5,
            idle_speed: 0.25,
            run_speed: 2.9,
            run_exit: 2.6,
            ..ControllerSettings::default()
        })
        .init_resource::<MwGround>()
        .init_resource::<ColliderQueue>()
        .insert_resource(CollisionSource { store, manifest })
        .insert_resource(Pathgrids(grids))
        .insert_resource(WalkerContent {
            db,
            min_walk,
            max_walk,
        })
        .add_systems(
            Update,
            (
                apply_colliders,
                refresh_terrain,
                attach,
                processing_range,
                camera_obstacle,
                drive,
            )
                .chain()
                .in_set(ControllerSystems::Drive),
        )
        .insert_resource(GiveUps {
            trace: std::env::var_os("BEVYRROWIND_STRIDE_TRACE").is_some(),
            ..GiveUps::default()
        })
        .insert_resource(StrideStats {
            trace: std::env::var_os("BEVYRROWIND_STRIDE_TRACE").is_some(),
            ..StrideStats::default()
        })
        .add_systems(
            PostUpdate,
            stride_monitor.after(sf_anim_bevy::AnimSystems::Evaluate),
        )
        .add_systems(Update, (gizmos, report));
}

/// Every 5 s: what the walkers do (log only; counters carry the timings).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn report(
    time: Res<Time>,
    mut next: Local<f32>,
    stats: Res<sf_controller_bevy::ControllerStats>,
    anim: Option<Res<AnimStats>>,
    strides: Res<StrideStats>,
    give_ups: Res<GiveUps>,
    ground: Ground,
    movers: Query<(&Mover, Option<&Walker>, Has<Dormant>)>,
) {
    let now = time.elapsed_secs();
    if now < *next {
        return;
    }
    *next = now + 5.0;
    let (mut walkers, mut walking, mut grounded, mut dormant, mut stuck, mut blocked) = (0, 0, 0, 0, 0, 0);
    for (m, w, d) in &movers {
        walkers += usize::from(w.is_some());
        dormant += usize::from(d);
        stuck += w.map_or(0, |w| w.scaffold.stuck_count());
        walking += usize::from(m.gait.moving);
        // Pushing without progress (blocked, crowded): the graph is told the
        // measured speed, so these stand instead of walking in place.
        blocked += usize::from(!d && m.applied.desired_velocity.length() > 0.5 && m.measured.length() < 0.25);
        grounded +=
            usize::from(m.loco.grounded.is_some() || !matches!(m.loco.mode, MoverMode::Ground));
    }
    info!(
        "walkers: {} movers + {dormant} dormant ({walkers} wander, {walking} moving, {grounded} grounded, {stuck} walks given up: {} steep, {} wall, {} other; {} on a first leg, {} off the pathgrid; {} rerouted around a blocked edge), {} statics, terrain {}; last frame avoid {:.0} us, move {:.0} us, feed {:.0} us",
        stats.movers,
        give_ups.steep,
        give_ups.wall,
        give_ups.other,
        give_ups.first_leg,
        give_ups.off_grid,
        give_ups.rerouted,
        ground.mw.placed(),
        if ground.on_physics() { "on physics".into() } else { (ground.mw.terrain.is_some() && ground.mw.exterior_active).to_string() },
        stats.avoid_us,
        stats.move_us,
        stats.feed_us
    );
    // Foot placement guards since start (animfix): rare clamps are normal;
    // a steady climb in `reach`/`pelvis` points at a speed or ground fault.
    if let Some(f) = anim.map(|a| a.feet_total) {
        info!(
            "walkers feet: {blocked} blocked; IK guards since start: {} samples rejected, {} unplanted, {} reach clamps, {} pelvis clamps, {} teleports, {} non-finite",
            f.rejected, f.unplanted, f.reach_clamped, f.pelvis_clamped, f.teleports, f.non_finite
        );
    }
    info!(
        "walkers strides since start: {} stances, {} skated (planted foot moved > 5 cm), {} legs without a plant for 2 s while walking, {} contacts held 1.5 s above 1 m/s",
        strides.stances, strides.skated, strides.frozen, strides.stuck
    );
}

// --- Stride monitor (slopefix) -------------------------------------------------------------

/// Planted feet that skate, and legs that stop cycling, counted over every
/// simulated walker (owner check WK-4: "one leg won't do its normal
/// movement, the other a diminished version, the body slides"). The report
/// logs the totals; `BEVYRROWIND_STRIDE_TRACE=1` logs each case with the
/// mover and graph state around it.
#[derive(Resource, Default)]
struct StrideStats {
    /// Stances (contact above 0.5 for at least 0.2 s) seen, and those whose
    /// foot moved more than 5 cm in the world mid-stance.
    stances: u64,
    skated: u64,
    /// Walking legs that went 2 s without planting.
    frozen: u64,
    /// Walking legs whose contact curve held one value for 1.5 s (a walk
    /// cycle's contacts change every half stride; walkers3).
    stuck: u64,
    trace: bool,
}

#[derive(Clone, Copy, Default)]
struct FootTrack {
    last: Option<WorldPos>,
    stance: f32,
    slide: f32,
    since_plant: f32,
    /// Last contact value and how long it has held while walking.
    contact: f32,
    held: f32,
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
fn stride_monitor(
    time: Res<Time>,
    mut stats: ResMut<StrideStats>,
    mut tracks: Local<HashMap<Entity, Vec<FootTrack>>>,
    mut slow: Local<HashMap<Entity, f32>>,
    ground: Ground,
    movers: Query<(Entity, &Mover, Option<&Character>), Without<Dormant>>,
    animators: Query<(&Animator, Option<&AnimLod>)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    let mut seen = Vec::new();
    for (e, m, character) in &movers {
        let asked = m.applied.desired_velocity.with_y(0.0);
        if stats.trace && asked.length() > 1.5 && (0.1..1.0).contains(&m.measured.length()) {
            let t = slow.entry(e).or_insert(0.0);
            *t += dt;
            if *t >= 1.0 {
                *t = 0.0;
                let dir = asked.normalize();
                let ahead: Vec<String> = [0.15f32, 0.3, 0.5]
                    .iter()
                    .map(|&s| {
                        let q = ProbeQuery::down(m.loco.position.offset(dir * s + Vec3::Y * 0.45), 1.2);
                        ground.probe().probe_one(q).map_or("-".into(), |h| format!("{s}: rise {:.2} n.y {:.2}", 0.45 - h.distance, h.normal.y))
                    })
                    .collect();
                info!(
                    "stride-slow: {e} at {:.2?} asked {:.2?} vel {:.2?} measured {:.2} grounded n {:.2?} h_off {:.2} ahead {ahead:?}",
                    m.loco.position.local,
                    asked,
                    m.loco.motion.velocity,
                    m.measured.length(),
                    m.loco.grounded.map(|g| g.normal),
                    m.loco.height_offset,
                );
            }
        } else {
            slow.remove(&e);
        }
        let Some((a, lod)) = m.animator.and_then(|x| animators.get(x).ok()) else {
            continue;
        };
        seen.push(e);
        let legs = &a.asset.graph.legs;
        let feet = tracks.entry(e).or_insert_with(|| vec![FootTrack::default(); legs.len()]);
        if feet.len() != legs.len() || !a.posed() {
            feet.fill(FootTrack::default());
            continue;
        }
        let frame = m.loco.frame();
        let walking = m.gait.moving;
        for (k, leg) in legs.iter().enumerate() {
            let t = &mut feet[k];
            let model = a.output.model.translation(usize::from(leg.effector())) * m.body_scale;
            let w = frame.to_world(model);
            let contact = leg
                .contact
                .and_then(|c| a.output.curves.get(usize::from(c)).copied())
                .unwrap_or(0.0);
            let step = t.last.map_or(0.0, |p| p.delta_f32(w).with_y(0.0).length());
            t.last = Some(w);
            let mut why = None;
            if contact > 0.5 && walking {
                if t.stance == 0.0 {
                    t.since_plant = 0.0;
                }
                t.stance += dt;
                if t.stance > 0.1 {
                    t.slide += step;
                }
            } else {
                if t.stance >= 0.2 {
                    stats.stances += 1;
                    if t.slide > 0.05 {
                        stats.skated += 1;
                        why = Some(format!("skated {:.2} m over a {:.2} s stance", t.slide, t.stance));
                    }
                }
                t.stance = 0.0;
                t.slide = 0.0;
            }
            if stats.trace && walking && k == 0 && (2.0..3.0).contains(&t.stance) {
                let c: Vec<String> = legs
                    .iter()
                    .map(|l| format!("{:.2}", l.contact.and_then(|c| a.output.curves.get(usize::from(c)).copied()).unwrap_or(-1.0)))
                    .collect();
                info!(
                    "stride-long: {e} t {:.2} pos {:.2?} measured {:.2?} vel {:.2?} asked {:.2?} yaw {:.2} moving {} contacts {c:?} drive.v {:.2?} moved {:.3?}",
                    t.stance,
                    m.loco.position.local,
                    m.measured,
                    m.loco.motion.velocity,
                    m.applied.desired_velocity,
                    m.loco.motion.yaw,
                    m.gait.moving,
                    a.drive.velocity,
                    a.drive.moved.translation,
                );
            }
            t.since_plant = if walking { t.since_plant + dt } else { 0.0 };
            if t.since_plant > 2.0 {
                stats.frozen += 1;
                t.since_plant = 0.0;
                why = Some(format!("no plant for 2 s (contact {contact:.2})"));
            }
            let fast = m.measured.length() > 1.0;
            t.held = if walking && fast && (contact - t.contact).abs() < 0.02 { t.held + dt } else { 0.0 };
            t.contact = contact;
            if t.held > 1.5 {
                stats.stuck += 1;
                t.held = 0.0;
                if stats.trace {
                    info!("stride-stuck: {e} leg {k} contact {contact:.2} held 1.5 s at {:.2} m/s: {}", m.measured.length(), graph_state(character, a, lod));
                }
            }
            if let (Some(why), true) = (why, stats.trace) {
                let g = &a.asset.graph;
                let locomotion = g
                    .node(mw_graphs::build::LOCOMOTION)
                    .and_then(|n| Some((n, a.instance.current_state(n)?)))
                    .and_then(|(n, i)| g.state_names(n).get(i).map(|s| (*s).to_owned()));
                let ground: Vec<String> = a
                    .drive
                    .ground
                    .iter()
                    .map(|s| s.map_or("-".into(), |s| format!("{:.2}/{:.2}", s.point.y, s.normal.y)))
                    .collect();
                info!(
                    "stride: {e} leg {k} {why}: at {:?} ground n.y {:?}, measured {:.2} m/s ({:?}), asked {:.2}, moving {}, state {locomotion:?}, \
                     samples {ground:?}, feet {:?}, scale {:.2}; {}",
                    m.loco.position,
                    m.loco.grounded.map(|g| g.normal.y),
                    m.measured.length(),
                    m.measured,
                    m.applied.desired_velocity.length(),
                    m.gait.moving,
                    a.output.feet,
                    m.body_scale.y,
                    graph_state(character, a, lod),
                );
            }
        }
    }
    tracks.retain(|e, _| seen.contains(e));
}

/// What drives a walker's graph, for the stride trace: base record,
/// stance/torch/move parameters, slot montages, the weighted players (node,
/// lane times and weights), the LOD tier and whether this frame evaluated.
fn graph_state(character: Option<&Character>, a: &Animator, lod: Option<&AnimLod>) -> String {
    let g = &a.asset.graph;
    let param = |name: &str| g.param(name).map(|id| a.instance.params.get(id));
    let record = match character {
        Some(Character::Record { id, .. }) => format!("{id:?}"),
        _ => "-".into(),
    };
    let slots: Vec<String> = g
        .slots
        .iter()
        .filter_map(|s| {
            let m = a.instance.montage(g.slot(&s.name)?)?;
            Some(format!("{} t{:.2}", s.name, m.time))
        })
        .collect();
    let players: Vec<String> = (0..g.nodes.len())
        .filter_map(|n| {
            let (lanes, w) = a.instance.player(u32::try_from(n).ok()?)?;
            (w > 0.01).then(|| {
                let lanes: Vec<String> = lanes
                    .iter()
                    .filter(|l| l.1 > 0.01)
                    .map(|l| format!("{:.3}@{:.2}", l.0, l.1))
                    .collect();
                format!("{} w{w:.2} [{}]", g.nodes[n].name, lanes.join(" "))
            })
        })
        .collect();
    format!(
        "record {record} stance {:?} torch {:?} move {:?} slots {slots:?} players {players:?} tier {:?} posed {} time_scale {:.2}",
        param(gp::STANCE).map(|v| v[0]),
        param(gp::TORCH).map(|v| v[0]),
        param(gp::MOVE),
        lod.map(|l| l.tier),
        a.posed(),
        a.time_scale,
    )
}

// --- Walks given up (walkers3) --------------------------------------------------------------

/// The path edge a walker is on: where it comes from (the previous node, or
/// where it stands when heading for the first) and the node it walks to.
fn current_edge(scaffold: &PathWalkScaffold, pos: WorldPos) -> Option<(WorldPos, WorldPos)> {
    let f = scaffold.follower()?;
    let t = f.target();
    let to = f.nodes.get(t)?.pos;
    Some((if t > 0 { f.nodes[t - 1].pos } else { pos }, to))
}

/// Walks the scaffold gave up for lack of progress, by what stopped them:
/// ground too steep to climb ahead on the edge, a wall (any other surface
/// that turned the motion aside), or nothing the mover hit (crowding, ORCA).
/// `BEVYRROWIND_STRIDE_TRACE=1` logs each with the ground along the edge.
#[derive(Resource, Default)]
struct GiveUps {
    steep: u64,
    wall: u64,
    other: u64,
    /// Steep or walled give-ups resumed at once around the blocked edge.
    rerouted: u64,
    /// Give-ups on a pathgrid walk's first leg (from where the walker stood
    /// to its first node), any kind.
    first_leg: u64,
    /// Give-ups on walks to a point off the pathgrid (cells without one).
    off_grid: u64,
    trace: bool,
}

impl GiveUps {
    fn record(&mut self, e: Entity, m: &Mover, edge: Option<(WorldPos, WorldPos)>, ground: &dyn GroundProbe) -> &'static str {
        let min_ny = m.params.min_normal_y();
        let pos = m.loco.position;
        let dir = edge.map_or(Vec3::ZERO, |(_, to)| pos.delta_f32(to).with_y(0.0).normalize_or_zero());
        let lift = m.params.step_up;
        let profile: Vec<Option<GroundHit>> = (1..=5)
            .map(|k| {
                #[allow(clippy::cast_precision_loss)]
                let s = k as f32 * 0.4;
                ground.probe_one(ProbeQuery::down(pos.offset(dir * s + Vec3::Y * (lift + 1.0)), lift + 3.0))
            })
            .collect();
        let deflected = m.loco.deflect != Vec3::ZERO;
        let steep = profile[..3].iter().flatten().any(|h| !h.walkable(min_ny));
        let kind = match (deflected, steep) {
            (true, true) => {
                self.steep += 1;
                "steep"
            }
            (true, false) => {
                self.wall += 1;
                "wall"
            }
            _ => {
                self.other += 1;
                "other"
            }
        };
        if self.trace {
            let rows: Vec<String> = profile
                .iter()
                .map(|h| h.map_or("-".into(), |h| format!("rise {:.2} n.y {:.2}", lift + 1.0 - h.distance, h.normal.y)))
                .collect();
            info!(
                "give-up ({kind}): {e} at {:.2?} edge {:?} dist {:.2} deflect {:.2?} ground n {:.2?} ahead every 0.4 m {rows:?}",
                pos.local,
                edge.map(|(a, b)| (a.local, b.local)),
                edge.map_or(0.0, |(_, to)| pos.delta_f32(to).with_y(0.0).length()),
                m.loco.deflect,
                m.loco.grounded.map(|g| g.normal),
            );
        }
        kind
    }
}
