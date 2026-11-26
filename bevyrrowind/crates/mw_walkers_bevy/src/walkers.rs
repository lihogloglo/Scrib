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
//!   Every ground actor that perceives also gets a `NavCorridor` and the perception reactions
//!   ([`crate::reaction`], `AI8i`: search, walk back to its post), wander or not; while a reaction
//!   owns it the wander ([`drive`]) is paused.
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
//! - Ticks (`NV7a`): the scaffold ([`drive`]), ORCA and the mover run in
//!   `FixedUpdate` after the player's move and before the physics step; the
//!   drawn NPC is interpolated between its last two ticks
//!   (`sf_controller_bevy`). Ground, attach, the processing range and the
//!   camera obstacle stay at frame rate (host state); gizmos, the report and
//!   the stride monitor only read.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_time::prelude::*;
use bevy_transform::prelude::*;
use mw_ai::{
    CellKey, PathGrids, Wander, WanderDriver, WanderPlanner, WanderTuning, WanderVocabulary,
    gmst_float, walk_speed,
};
use mw_graphs::build::params as gp;
use sf_ai::{
    ExecConfig, ExecContext, NavCommand, NavMoveState, Tier, TraceEntry,
};
use sf_ai_bevy::{
    AiActor, AiDecideGate, AiExecPlugin, AiExecSettings, AiExecTick, AiLibrary, AiLogicalBody, AiPlugin, AiTier, AiTraces,
    add_exec_tunables, to_controller_gait,
};
use sf_anim_bevy::{AnimLod, AnimStats, Animator};
use sf_bake::{Manifest, Store};
use sf_character_bevy::{Character, CharacterBody, CharacterPlacement};
use sf_controller::steps::COLLISION_COOK;
use sf_controller::{
    CollisionMesh, GroundHit, GroundProbe, LocomotionIntent, Nearest, ProbeQuery, StaticId,
    StaticShape, StaticWorld, TerrainProbe,
};
use sf_controller_bevy::{
    AnimParamNames, AvoidObstacle, Avoider, ControllerGizmos, ControllerSettings,
    ControllerSystems, Dormant, Held, MoveIntent, Mover, MoverMode, MoverParams, SfControllerPlugin,
};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::ActorKey;
use sf_nav::{DEFAULT_EXTENT, QueryFilter};
use sf_nav_bevy::{NavCorridor, NavMeshes, NavQueries, NavSystems};
use sf_records::{RecordDb, RecordId};
use sf_terrain_bevy::TerrainHeightsMap;
use sf_terrain_bevy::sf_terrain::TerrainHeights;
use sf_tunables_bevy::TunablesAppExt;
use sf_tunables_bevy::sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};
use sf_world::{IndexedPlacement, Level, Location, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::coords::WorldPosOf;

use crate::reaction::Reaction;
use sf_world_bevy::{
    ActivateCx, ActiveWorldspace, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx,
    Streaming,
};
use tes3::esp::{Creature, CreatureFlags, Npc};

/// Hover height of flying creatures over the ground, metres (cliff racers glide low).
const FLY_HOVER: f32 = 1.5;
/// Half size of the square around a mover whose streaming cells must all be
/// R0-active for it to simulate, m: a walk step plus the capsule, so a mover
/// parks before it could reach a cell without colliders.
const PROCESS_MARGIN: f32 = 4.0;
/// Animation significance of dormant movers (the budget drops them to a low tier).
const DORMANT_SIGNIFICANCE: f32 = 0.25;
/// Exterior AI processing wake radius around [`mw_game_core::exterior::ExteriorCamera`], m
/// (vanilla Morrowind's AI processing range is 7168 units = 102.4 m; 110 m covers a full
/// cell sightline while parking unseen 3-px movers 122–480 m away). Override with
/// `BEVYRROWIND_AI_RADIUS` (`0` disables the distance cap).
const AI_WAKE_RADIUS_M: f32 = 110.0;
/// Exterior AI processing sleep radius (hysteresis margin past [`AI_WAKE_RADIUS_M`]), m.
const AI_SLEEP_RADIUS_M: f32 = 122.0;

fn ai_radius_m() -> Option<(f32, f32)> {
    static RADIUS: std::sync::OnceLock<Option<(f32, f32)>> = std::sync::OnceLock::new();
    *RADIUS.get_or_init(|| {
        if let Some(val) = std::env::var_os("BEVYRROWIND_AI_RADIUS")
            && let Some(r) = val.to_str().and_then(|s| s.parse::<f32>().ok())
        {
            return (r > 0.0).then_some((r, r * (AI_SLEEP_RADIUS_M / AI_WAKE_RADIUS_M)));
        }
        Some((AI_WAKE_RADIUS_M, AI_SLEEP_RADIUS_M))
    })
}

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
#[derive(bevy_ecs::system::SystemParam)]
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
pub(crate) struct WalkerContent {
    pub(crate) db: Arc<RecordDb>,
    pub(crate) min_walk: f32,
    pub(crate) max_walk: f32,
    /// `fIdleChanceMultiplier` (the `AI_W` idle roll gate, `mw_ai::roll_idle_with`).
    pub(crate) idle_mult: f32,
}

/// A dressed character another layer moves (e.g. `mw_player_bevy`'s training dummy, under an
/// action body): [`attach`] gives it no mover, like the player's avatar.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ActionDriven;

/// The shared Morrowind wander HTN vocabulary (`MA3`).
#[derive(Resource, Clone)]
pub struct WanderVocab(pub Arc<WanderVocabulary>);

/// A wandering character's HTN wander driver (`mw_ai`, `MA3`), a routine: the perception-reaction
/// layer ([`crate::reaction::Reaction`], `AI8i`: the search, `AI8f2`) pauses it while it owns the actor.
#[derive(Component)]
#[require(Reaction)]
pub struct Walker {
    pub driver: WanderDriver,
    variant: Option<u8>,
    /// The pathgrid it walks: its exterior cell at spawn, or its interior.
    key: CellKey,
    /// The intent [`apply_held_intent`] restores after [`NavSystems::Corridor`] (which resets a
    /// goal-less corridor to `IDLE`): the pathgrid fallback's walk, or a navmesh stand's facing.
    held_intent: Option<LocomotionIntent>,
    /// The mode it ran last tick: `Some(true)` HTN over the navmesh (`NavCorridor`), `Some(false)`
    /// the pathgrid fallback (fliers, swimmers, no resident navmesh tile), `None` not yet driven.
    on_navmesh: Option<bool>,
}

impl Walker {
    /// Drops the stand [`apply_held_intent`] would restore (a reaction owns the intent).
    pub(crate) fn release_held_intent(&mut self) {
        self.held_intent = None;
    }
}

/// The worldspace a mover lives in (NPCs never change worldspace in M3).
#[derive(Component, Clone, Debug)]
pub struct ActorSpace(pub WorldspaceId);

/// Mover and walker setup for a dressed character's base record.
struct Setup {
    params: MoverParams,
    mode: MoverMode,
    /// `AI_W` with a distance: the wander routine.
    wander: Option<Wander>,
    /// Walk speed, m/s (the wander's and the reactions').
    speed: f32,
    /// Size the capsule from the dressed body's bind-pose bounds (`params` is the fallback).
    fit_to_body: bool,
}

/// The walk speed `attach` gives a record character (m/s); none for other characters.
pub(crate) fn record_walk_speed(content: &WalkerContent, character: &Character) -> Option<f32> {
    match character {
        Character::Record { id, .. } => Some(setup_of(content, id).speed),
        Character::Recipe(_) => None,
    }
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
        let wander = Wander::of_npc(npc).filter(|w| w.distance > 0.0);
        return Setup {
            params: MoverParams::default(),
            mode: MoverMode::Ground,
            wander,
            speed: speed_of(speed),
            fit_to_body: false,
        };
    }
    if let Some(c) = content.db.get::<Creature>(id) {
        #[allow(clippy::cast_precision_loss)]
        let speed = speed_of(c.data.speed as f32);
        let wander = Wander::of_creature(c).filter(|w| w.distance > 0.0);
        let flies = c.creature_flags.contains(CreatureFlags::FLIES);
        // Scaffold (fallfix): pure swimmers (slaughterfish) hold their depth
        // without gravity, `Fly { hover: None }` with a level intent, until the
        // water queries (M6) feed `MoverMode::Swim` its surface; remove with that
        // wiring. As `Ground` movers they sank into the sea floor at load.
        let swims = c.creature_flags.contains(CreatureFlags::SWIMS)
            && !c.creature_flags.contains(CreatureFlags::WALKS);
        let mode = if flies {
            MoverMode::Fly {
                hover: Some(FLY_HOVER),
            }
        } else if swims {
            MoverMode::Fly { hover: None }
        } else {
            MoverMode::Ground
        };
        // Fallback until the body is dressed: the record's scale on a human-ish column. `attach` fits
        // the capsule to the model's bind-pose bounds (AN-8: a fixed 0.4 m column swallowed no mudcrab).
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
            speed,
            fit_to_body: true,
        };
    }
    Setup {
        params: MoverParams::default(),
        mode: MoverMode::Ground,
        wander: None,
        speed: speed_of(50.0),
        fit_to_body: false,
    }
}

/// Gives dressed characters their mover (and wanderers their walker); not the player's avatar
/// (the action layer moves it).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn attach(
    mut commands: Commands,
    content: Res<WalkerContent>,
    vocab: Option<Res<WanderVocab>>,
    pos_of: WorldPosOf,
    fresh: Query<
        (
            Entity,
            &Character,
            &CharacterBody,
            &Transform,
            &ChildOf,
            Option<&CharacterPlacement>,
        ),
        (Without<Mover>, Without<mw_game_core::player::PlayerAvatar>, Without<ActionDriven>),
    >,
    bodies: Query<&Transform>,
) {
    for (entity, character, body, transform, parent, placement) in &fresh {
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
        let body_scale = bodies.get(body.body).map_or(Vec3::ONE, |t| t.scale);
        let mut params = setup.params;
        if setup.fit_to_body
            && let Some((lo, hi)) = body.outfit.bind_bounds
        {
            params = MoverParams::from_bounds(lo * body_scale, hi * body_scale);
            // A capsule cannot lie flat: a wide, low body gets an upright cylinder (`BodyDesc::character`).
            let flat = params.radius > 0.5 * params.height;
            if flat {
                info!("creature {id}: wide and low, physics cylinder {:.2} m radius x {:.2} m high", params.radius, params.height);
            } else {
                debug!("creature {id}: capsule {:.2} m radius x {:.2} m high", params.radius, params.height);
            }
        }
        let mut mover = Mover::new(
            position,
            Mover::yaw_of(transform.rotation),
            params,
            frame_origin,
        );
        mover.loco.mode = setup.mode;
        mover.animator = Some(body.body);
        // Race height/weight: the body's scale (foot probes, motion feed).
        mover.body_scale = body_scale;
        let mut e = commands.entity(entity);
        // Parked until `processing_range` finds its ground loaded.
        e.try_insert((
            mover,
            MoveIntent::default(),
            ActorSpace(location.worldspace.clone()),
            Dormant,
        ));
        if let Some(p) = placement {
            // Sight (MA14): the eye on the standing height, the fitted capsule or the default
            // one times the race height.
            let height = if setup.fit_to_body { params.height } else { params.height * body_scale.y };
            e.try_insert((AiActor(ActorKey(p.0.0)), crate::sight::observer(height)));
        }
        let walk_speed = setup.speed;
        let wanders = setup.wander.is_some();
        if let Some(wander) = setup.wander {
            e.insert(new_walker(entity, wander, &location, walk_speed, Some(content.idle_mult), vocab.as_deref()));
        }
        // AI8i: every ground actor that perceives (or wanders) gets the navmesh corridor and the
        // perception reactions (search, walk back to its post), with or without a wander: guards
        // on post and shopkeepers (`AI_W` distance 0) search too.
        if matches!(setup.mode, MoverMode::Ground) && (wanders || placement.is_some()) {
            let actor_key = ActorKey(placement.map_or(u128::from(entity.to_bits()), |p| p.0.0));
            let filter = QueryFilter::for_class(&mw_bake::nav::areas(), mw_bake::nav::HUMANOID);
            e.insert((
                NavCorridor::new(actor_key, mw_bake::nav::HUMANOID, location.worldspace.clone(), walk_speed)
                    .with_filter(filter),
                Reaction::new(walk_speed),
            ));
        }
    }
}

/// A wanderer's [`Walker`] and its [`Avoider`] as [`attach`] gives them at spawn (and the
/// directive drive to an actor a `StayNear` serves without one, `MA2d4`): the wander of `wander`
/// around `at`, at `walk_speed`; `idle_mult` is `fIdleChanceMultiplier` (`None`: the planner's
/// default).
pub(crate) fn new_walker(
    entity: Entity,
    wander: Wander,
    at: &Location,
    walk_speed: f32,
    idle_mult: Option<f32>,
    vocab: Option<&WanderVocab>,
) -> (Walker, Avoider) {
    let seed = entity.to_bits() ^ 0x5741_4C4B;
    let vocab = vocab.map_or_else(mw_ai::wander_library, |v| v.0.clone());
    let mut planner = WanderPlanner::new(wander, at.pos);
    if let Some(mult) = idle_mult {
        planner.idle_mult = mult;
    }
    let driver = WanderDriver::with_vocab(planner, walk_speed, seed, at.worldspace.clone(), vocab);
    let key = if at.worldspace == mw_world::ids::exterior() {
        CellKey::exterior_at(at.pos)
    } else {
        CellKey::interior(at.worldspace.as_str())
    };
    (
        Walker { driver, variant: None, key, held_intent: None, on_navmesh: None },
        Avoider { max_speed: walk_speed * 1.5, ..Avoider::default() },
    )
}

/// Movers an outside system owns ([`Held`]: dead, knocked down, a physics
/// takeover) stay parked until it lets go.
type Unprocessed = Without<Held>;

/// The processing range (streaming.md / locomotion.md "Processing range"):
/// a mover simulates only in the active worldspace and only while every
/// streaming cell within [`PROCESS_MARGIN`] of it is R0-active, i.e. its
/// ground and static colliders are loaded (R0 activation builds them).
/// Elsewhere it is [`Dormant`]: parked where it stands, its scaffold paused,
/// its animation significance lowered so the budget gives it a low tier.
///
/// On physics, R0 activation only queues the statics: they go in over the
/// following frames under the insert budget. A dormant mover wakes only
/// once no static around it is still queued (the collision-ready gate,
/// `PhysicsStreaming::collision_loading`): woken earlier, it fell through
/// the floor that arrived a few frames later (fallfix, 2026-10-18). Exterior
/// movers also wait for the terrain heights around them.
///
/// `Update`, at frame rate (`NV7a`): host state like streaming (which colliders
/// are resident, the camera's AI range), not a gameplay rule; a change lands
/// at the next tick boundary, so a mover never parks or wakes mid-tick.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments, clippy::type_complexity)]
fn processing_range(
    mut commands: Commands,
    streaming: Res<Streaming>,
    active: Res<ActiveWorldspace>,
    ready: GroundReady,
    pos_of: WorldPosOf,
    cams: Query<Entity, With<mw_game_core::exterior::ExteriorCamera>>,
    movers: Query<(Entity, Ref<Mover>, &ActorSpace, Has<Dormant>), Unprocessed>,
    mut lods: Query<&mut AnimLod>,
) {
    let scheduler = streaming.scheduler();
    let exterior_ws = mw_world::ids::exterior();
    let ai_anchor = ai_radius_m().and_then(|(wake, sleep)| {
        let eye = cams.iter().find_map(|e| pos_of.get(e))?;
        Some((eye, wake * wake, sleep * sleep))
    });
    for (entity, mover, space, dormant) in &movers {
        let pos = mover.loco.position;
        let in_ai_range = if space.0 == exterior_ws
            && let Some((anchor, wake_sq, sleep_sq)) = ai_anchor
        {
            let d = anchor.delta_f32(pos);
            let dist_sq = d.x * d.x + d.z * d.z;
            dist_sq <= if dormant { wake_sq } else { sleep_sq }
        } else {
            true
        };
        let awake = active.0.as_ref() == Some(&space.0)
            && in_ai_range
            && scheduler.covers(&space.0, pos, PROCESS_MARGIN, Level::Active)
            && !(dormant && ready.loading(&space.0, pos));
        // A fresh mover starts dormant (`attach`): it still gets its significance.
        if awake != dormant && !mover.is_added() {
            continue;
        }
        // `try_`: streaming may despawn the mover before these commands apply.
        if awake {
            commands.entity(entity).try_remove::<Dormant>();
        } else {
            commands.entity(entity).try_insert(Dormant);
        }
        if let Some(mut lod) = mover.animator.and_then(|a| lods.get_mut(a).ok()) {
            lod.significance = if awake { 1.0 } else { DORMANT_SIGNIFICANCE };
            lod.idle_skip_a3 = !awake;
        }
    }
}

/// The collision-ready gate's inputs (see [`processing_range`]).
#[derive(bevy_ecs::system::SystemParam)]
struct GroundReady<'w> {
    ground: Ground<'w>,
    collision: sf_physics_bevy::CollisionReady<'w>,
    heights: Option<Res<'w, TerrainHeightsMap>>,
}

impl GroundReady<'_> {
    /// Whether the ground around `pos` in `ws` is still streaming in: a
    /// cell not activated or a static queued for physics (`CollisionReady`),
    /// or terrain heights not resident.
    fn loading(&self, ws: &WorldspaceId, pos: WorldPos) -> bool {
        (self.ground.on_physics() && self.collision.loading(ws, pos, PROCESS_MARGIN))
            || self.heights.as_deref().is_some_and(|h| terrain_missing(h, ws, pos))
    }
}

/// Whether `ws` has terrain but its heights around `pos` (the processing
/// square's corners) are not resident yet: the ground probe would find no
/// terrain there.
fn terrain_missing(heights: &TerrainHeightsMap, ws: &WorldspaceId, pos: WorldPos) -> bool {
    let Some(h) = heights.0.get(ws) else { return false };
    let m = PROCESS_MARGIN;
    [(0.0, 0.0), (-m, -m), (m, -m), (-m, m), (m, m)]
        .into_iter()
        .any(|(dx, dz)| h.height(pos.offset(Vec3::new(dx, 0.0, dz))).is_none())
}

pub(crate) fn nav_move_state(nav: &NavCorridor) -> NavMoveState {
    if nav.failed.is_some() {
        NavMoveState::Failed
    } else if nav.is_arrived() {
        NavMoveState::Arrived
    } else if nav.corridor().is_some() {
        NavMoveState::Following
    } else if nav.pending.is_some() || nav.goal.is_some() {
        NavMoveState::Pending
    } else {
        NavMoveState::Idle
    }
}

pub(crate) fn clear_corridor_goal(nav: &mut NavCorridor, queries: Option<&mut NavQueries>) {
    if let Some(id) = nav.pending.take()
        && let Some(q) = queries
    {
        q.cancel(id);
    }
    nav.clear_goal();
}

/// Steps each awake [`Walker`] (`MA3`): on a resident navmesh tile, steps HTN + [`NavCorridor`]
/// before [`NavSystems::Corridor`]; off the navmesh (fliers or unbaked cells), runs the pathgrid
/// fallback route. While a perception reaction owns the actor ([`Reaction::owns`], `AI8i`: a search
/// or the walk back to its post, [`crate::reaction`], which runs before this) the wander is paused.
///
/// Tiers (`AI9g`, `ai.md` "Tiers and budget"): the HTN planner decides on the actor's [`AiTier`]
/// cadence ([`AiDecideGate::due`]: Full every `ai.alloc.full_decide_s`, Reduced every
/// `ai.alloc.reduced_decide_s`, staggered; every tick before the first allocation), passed as
/// [`ExecContext::decide`]: a walker whose step ran out stands until its next decide tick, while
/// the running operator (steering, idle timers) and events (a dirty key, a failed precondition, a
/// failed walk) run every tick. A tier-0 body ([`AiLogicalBody`], `AI10c2`: its state steps in
/// [`crate::logical`]) stands where it is, before anything else drives it, until its promotion
/// places it; so does a `Logical` walker outside the store (no stack, no game clock). Gap: the
/// pathgrid fallback (`WanderDriver::update_fallback`, off the navmesh) still chooses every tick.
#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
fn drive(
    time: Res<Time>,
    mut tick: ResMut<AiExecTick>,
    exec_settings: Option<Res<AiExecSettings>>,
    wander_settings: Option<Res<WanderSettings>>,
    grids: Res<Pathgrids>,
    meshes: Option<Res<NavMeshes>>,
    mut queries: Option<ResMut<NavQueries>>,
    mut traces: Option<ResMut<AiTraces>>,
    mut walkers: Query<
        (
            Entity,
            &Mover,
            &ActorSpace,
            Option<&AiActor>,
            &mut Walker,
            &mut MoveIntent,
            &mut Avoider,
            Option<&mut NavCorridor>,
            &Reaction,
            Option<&crate::directive_drive::DirectiveDrive>,
            Option<&crate::greeting::Greeting>,
            (Option<&AiTier>, Has<AiLogicalBody>),
        ),
        Without<Dormant>,
    >,
    ground: Ground,
    mut give_ups: ResMut<GiveUps>,
    mut modes: ResMut<WanderModes>,
    mut animators: Query<&mut Animator>,
    mut decide_gate: AiDecideGate,
) {
    tick.0 += 1;
    let current_tick = tick.0;
    let exec_cfg = exec_settings.map_or_else(ExecConfig::default, |s| s.config);
    let wander_tuning = wander_settings.map_or_else(WanderTuning::default, |s| s.tuning);
    let dt = time.delta_secs().min(0.1);
    for (
        entity,
        mover,
        space,
        actor,
        mut walker,
        mut intent,
        mut avoider,
        mut corridor,
        reaction,
        directive,
        greeting,
        (tier, logical),
    ) in &mut walkers
    {
        let walker = &mut *walker;
        if !logical
            && (reaction.owns()
                || directive.is_some_and(crate::directive_drive::DirectiveDrive::owns)
                || greeting.is_some_and(crate::greeting::Greeting::holds))
        {
            // AI8i: the reaction drives the actor (MA2d2: or a directive's walk, a `GoTo`; MA4c2:
            // or a greeting holds it still); the wander stays paused where it was.
            walker.held_intent = None;
            continue;
        }
        if logical
            || directive.is_some_and(crate::directive_drive::DirectiveDrive::stands)
            || tier.is_some_and(|t| t.tier == Tier::Logical)
        {
            // AI10c2: a tier-0 body stands, whatever drove it before (its state walks in
            // `crate::logical` until its promotion places it). MA2d4: no package left on its
            // stack (`OpenMW`: an empty `AiSequence` runs nothing): it stands where it is, facing
            // where it faces; so does a `Logical` walker the store does not hold (no stack, no
            // game clock).
            if let Some(nav) = corridor.as_deref_mut() {
                clear_corridor_goal(nav, queries.as_deref_mut());
            }
            let stand = LocomotionIntent { desired_facing: Some(mover.loco.motion.yaw), ..LocomotionIntent::IDLE };
            intent.0 = stand;
            walker.held_intent = Some(stand);
            avoider.travel = Vec3::ZERO;
            continue;
        }
        let pos = mover.loco.position;
        let loc = Location::new(space.0.clone(), pos);
        walker.driver.tuning = wander_tuning;
        let graph = grids.0.get(&walker.key);

        let on_navmesh = matches!(mover.loco.mode, MoverMode::Ground)
            && corridor.as_ref().is_some_and(|c| {
                meshes.as_ref().is_some_and(|m| {
                    m.0.nearest(&loc, c.class, DEFAULT_EXTENT, &c.filter)
                        .is_some()
                })
            });
        walker.on_navmesh = Some(on_navmesh);
        let arrivals_before = walker.driver.arrival_count();

        let (walking, idle_variant) = if on_navmesh {
            let nav = corridor.as_deref_mut().expect("on_navmesh requires corridor");
            let nav_state = nav_move_state(nav);
            let decide = decide_gate.due(tier);
            let ctx = ExecContext {
                location: loc,
                yaw: mover.loco.motion.yaw,
                dt,
                nav: nav_state,
                decide,
            };
            let out = {
                let wants_trace = actor
                    .zip(traces.as_ref())
                    .is_some_and(|(a, t)| t.wants(a.0, 2));
                let mut entry = wants_trace.then(|| TraceEntry::new(current_tick, 2));
                let (stuck, edge) =
                    (walker.driver.stuck_count(), walker.driver.current_edge(pos));
                let out = walker
                    .driver
                    .step_htn(graph, &ctx, &exec_cfg, entry.as_mut());
                if walker.driver.stuck_count() > stuck {
                    let _ = give_ups.record(entity, mover, edge, ground.probe());
                    modes.navmesh_give_ups += 1;
                }
                if out.plan_outcome.is_some() {
                    decide_gate.record_plan(tier, decide);
                }
                if let (Some(a), Some(t), Some(e)) = (actor, traces.as_mut(), entry)
                    && (out.plan_outcome.is_some() || out.operator.is_some())
                {
                    t.push(a.0, e);
                }
                out
            };
            match out.nav {
                NavCommand::MoveTo { goal, gait, speed } => {
                    let needs_new_goal = nav.goal.as_ref() != Some(&goal)
                        || nav.is_arrived()
                        || nav.failed.is_some();
                    if needs_new_goal {
                        clear_corridor_goal(nav, queries.as_deref_mut());
                        nav.set_goal(goal);
                    }
                    nav.follower.gait = to_controller_gait(gait);
                    nav.follower.speed = speed;
                    walker.held_intent = None;
                }
                NavCommand::Stop | NavCommand::None => {
                    clear_corridor_goal(nav, queries.as_deref_mut());
                    // Keep the executor's facing (`Face`, `Wait { facing }`) as
                    // `sf_ai_bevy::apply_exec_output` does, held past the corridor's reset.
                    let stand = LocomotionIntent {
                        desired_facing: out.facing,
                        ..LocomotionIntent::IDLE
                    };
                    intent.0 = stand;
                    walker.held_intent = Some(stand);
                }
            }
            avoider.travel = nav
                .corridor()
                .and_then(|c| c.corners().first())
                .map_or(Vec3::ZERO, |c| {
                    pos.delta_f32(c.pos).with_y(0.0).normalize_or_zero()
                });
            (walker.driver.is_walking(), out.idle_variant)
        } else {
            if let Some(ref mut nav) = corridor {
                clear_corridor_goal(nav, queries.as_deref_mut());
            }
            let (stuck, edge) = (walker.driver.stuck_count(), walker.driver.current_edge(pos));
            let out = walker.driver.update_fallback(graph, pos, dt, &exec_cfg);
            if walker.driver.stuck_count() > stuck {
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
                    && walker.driver.replan_around(graph, from, to, pos)
                {
                    give_ups.rerouted += 1;
                }
            }
            intent.0 = out.intent;
            walker.held_intent = Some(out.intent);
            avoider.travel = walker.driver.follower().map_or(Vec3::ZERO, |f| {
                let t = f.target();
                let from = if t > 0 { f.nodes[t - 1].pos } else { pos };
                f.nodes.get(t).map_or(Vec3::ZERO, |n| {
                    from.delta_f32(n.pos).with_y(0.0).normalize_or_zero()
                })
            });
            (out.walking, out.idle_variant)
        };
        let arrived = u64::from(walker.driver.arrival_count() - arrivals_before);
        if on_navmesh {
            modes.navmesh_arrivals += arrived;
        } else {
            modes.fallback_arrivals += arrived;
        }

        // AI_W idles: a new IdleN (2..=9) plays once as a fidget.
        let variant = (!walking).then_some(idle_variant.unwrap_or(0));
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

/// Restores [`Walker::held_intent`] after [`NavSystems::Corridor`] for ground walkers whose
/// [`NavCorridor`] had no active goal: pathgrid fallback walks, and navmesh stands holding the
/// executor's facing.
#[allow(clippy::type_complexity)]
fn apply_held_intent(
    mut walkers: Query<(&Walker, &mut MoveIntent), (Without<Dormant>, Without<Held>)>,
) {
    for (walker, mut intent) in &mut walkers {
        if let Some(fb) = walker.held_intent {
            intent.0 = fb;
        }
    }
}

/// The camera as a non-reactive obstacle at its feet (about 1.2 m below the
/// eye), so walkers step around a camera standing in the street.
///
/// Scaffold (`NV7a`): stays in `Update`, at frame rate, because it follows the
/// free camera (presentation); the ticks read the obstacle as last written.
/// Removal path: once every view has a player body (whose obstacle follows
/// its tick state, `mw_player_bevy::obstacle`), the free camera stops being
/// an avoidance participant and this goes.
#[allow(clippy::needless_pass_by_value)]
fn camera_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    pos_of: WorldPosOf,
    mut cams: Query<(Entity, Option<&mut AvoidObstacle>), With<mw_game_core::exterior::ExteriorCamera>>,
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
            commands.entity(entity).try_insert(AvoidObstacle {
                position: feet,
                velocity: Vec3::ZERO,
                radius: 0.4,
            });
        }
    }
}

/// `Shift+J` toggles the mover gizmos (capsule ring, velocity, trajectory; `J` is the journal),
/// `K` foot placement.
#[allow(clippy::needless_pass_by_value)]
fn gizmos(
    keys: Res<ButtonInput<KeyCode>>,
    mut g: ResMut<ControllerGizmos>,
    mut settings: ResMut<ControllerSettings>,
    grid: Query<&sf_world_bevy::big_space::prelude::Grid>,
) {
    if keys.just_pressed(KeyCode::KeyJ) && keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
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

/// Adds the walkers: controller stage, ground, pathgrids, HTN wander driver.
pub fn add(app: &mut App, db: Arc<RecordDb>, store: Store, manifest: Manifest, physics: bool) {
    let started = std::time::Instant::now();
    let grids = PathGrids::build(&db);
    let vocab = mw_ai::wander_library();
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
        // The humanoid gait set's walk / jog lanes (creature graphs have no such param).
        speed: Some(gp::SPEED.into()),
    };
    // The mover probes physics when it runs (P5), else the `StaticWorld`.
    // NPC sight (MA14) probes the same ground.
    if physics {
        app.add_plugins(SfControllerPlugin::<sf_physics_bevy::Physics>::default());
        crate::sight::add::<sf_physics_bevy::Physics>(app);
    } else {
        app.add_plugins(SfControllerPlugin::<MwGround>::default());
        crate::sight::add::<MwGround>(app);
    }
    crate::light_inputs::insert_records(app, &db);
    // NPCs turn their heads to the player (AN8b): the look target of their `AnimDrive`.
    crate::head_track::add(app, &db);
    // The tier allocator's `AiInView` from every player's view frustum (`MA20a`).
    crate::in_view::add(app);
    let refused = sf_ai_bevy::load_perception_states(app.world_mut(), &db);
    if refused > 0 {
        warn!("walkers: {refused} perception records refused (out of range)");
    }
    add_drive(app, vocab);
    crate::greeting::insert_records(app, &db);
    let idle_mult = gmst_float(&db, "fIdleChanceMultiplier", mw_ai::wander::IDLE_CHANCE_MULTIPLIER);
    app.insert_resource(GroundOnPhysics(physics))
        .insert_resource(ControllerSettings {
            params,
            // Measured-speed thresholds against the humanoid cycles (walk
            // 2.2 m/s, run 3.2 m/s, `SpeedMatch` 0.5-1.5, stride warp 0.6+):
            // moving from 0.5 m/s down to 0.25 (0.45 when braking to a stop,
            // so idle blends in over the last of the stop); running from 2.9 down to 2.6.
            moving_speed: 0.5,
            idle_speed: 0.25,
            stop_speed: 0.45,
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
            idle_mult,
        })
        // Host state at frame rate: ground (streamed colliders, terrain), new movers, the
        // processing range, the camera obstacle (scaffold). The ticks see it as last written.
        .add_systems(
            Update,
            (
                apply_colliders,
                refresh_terrain,
                attach,
                processing_range,
                camera_obstacle,
            )
                .chain(),
        )
        // MA2d1: record AI packages onto the directive stack of each new `AiActor`; MA2d3: the
        // `AI_A` placement index and the packages waiting for a target to spawn.
        .init_resource::<crate::directives::PendingPackages>()
        .add_systems(PostStartup, crate::directives::hook_placed_objects)
        // AI10c3a: the resident actor placements and the poses-kept proof line.
        .init_resource::<crate::poses::ResidentActors>()
        .add_systems(Update, crate::poses::report_kept)
        .add_systems(Update, (crate::directives::seed_directives, crate::directives::retry_pending).chain().after(attach))
        // Gameplay on the fixed tick (NV7a): the physics step first (IN11b: characters ride the
        // bases it moved), then the player (its tick state is what the NPCs avoid), then the NPC
        // intents and their mover.
        .configure_sets(
            FixedUpdate,
            (
                sf_physics_bevy::PhysicsSystems::Step.before(sf_action_bevy::ActionSystems::Sample),
                ControllerSystems::Drive.after(sf_action_bevy::ActionSystems::Move),
            ),
        )
        .insert_resource(StrideStats {
            trace: std::env::var_os("BEVYRROWIND_STRIDE_TRACE").is_some(),
            ..StrideStats::default()
        })
        .add_systems(
            PostUpdate,
            stride_monitor.after(sf_anim_bevy::AnimSystems::Evaluate),
        )
        .add_systems(Update, (gizmos, report))
        .add_systems(
            Update,
            log_fell_out.run_if(resource_exists::<Messages<sf_physics_bevy::CharacterFellOut>>),
        );
}

/// The wander drive (`MA3`): the HTN vocabulary as the AI library, the exec settings and tick,
/// and [`drive`] on the fixed tick before [`NavSystems::Corridor`] (which turns its `MoveTo` goals
/// into `MoveIntent`), then [`apply_held_intent`] after it. Shared by [`add`] and the headless
/// drive tests, so they run the game's schedule.
fn add_drive(app: &mut App, vocab: Arc<WanderVocabulary>) {
    if !app.is_plugin_added::<AiPlugin>() {
        app.add_plugins(AiPlugin::default());
    }
    app.insert_resource(AiLibrary {
        library: vocab.library.clone(),
        report: None,
    })
    .insert_resource(WanderVocab(vocab))
    .init_resource::<AiExecSettings>()
    .init_resource::<WanderSettings>()
    .init_resource::<AiExecTick>()
    .init_resource::<WanderModes>()
    .init_resource::<crate::script_ai::AiRuns>()
    .add_message::<crate::script_ai::ScriptAiPush>()
    .insert_resource(GiveUps {
        trace: std::env::var_os("BEVYRROWIND_STRIDE_TRACE").is_some(),
        ..GiveUps::default()
    })
    .add_systems(
        FixedUpdate,
        (
            // MA2c2: script AI pushes onto the stacks, before the drive serves them.
            crate::script_ai::apply_script_pushes.before(crate::directive_drive::drive_directives),
            // MA2d2: the directive stack's serving directive, between the reactions and the wander.
            crate::directive_drive::drive_directives
                .in_set(ControllerSystems::Drive)
                .after(NavSystems::Publish)
                .after(crate::reaction::ReactionSystems::React)
                .before(drive)
                .before(NavSystems::Corridor),
            drive
                .in_set(ControllerSystems::Drive)
                .after(NavSystems::Publish)
                .after(crate::reaction::ReactionSystems::React)
                .before(NavSystems::Corridor),
            apply_held_intent
                .in_set(ControllerSystems::Drive)
                .after(NavSystems::Corridor)
                .before(crate::reaction::ReactionSystems::Apply),
        ),
    )
    .add_systems(Update, report_modes);
    // AI10c2: tier 0 in the game (enter the store, step, promote and place the bodies).
    crate::logical::add(app);
    crate::reaction::add(app);
    // MA4c2: NPCs greet the player (decide before the routines, the stand after the corridor).
    crate::greeting::add(app);
    // The drive reads `AiExecSettings` without `AiExecPlugin` (it steps its own runners): bind
    // the `ai.exec.*` tunables here unless the plugin already does.
    if !app.is_plugin_added::<AiExecPlugin>() {
        add_exec_tunables(app);
    }
    add_wander_tunables(app);
    crate::directive_drive::add_travel_tunables(app);
}

/// `mw.ai.wander.walk_backstop_mult`: [`WanderTuning::walk_backstop_mult`].
pub const WANDER_WALK_BACKSTOP_MULT: Def<f32> = def!(float "mw.ai.wander.walk_backstop_mult" = 2.0)
    .range(1.0, 10.0)
    .unit(Unit::Ratio)
    .help("Pathgrid fallback walks are given up after this multiple of their nominal time + slack")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.wander.walk_slack_s`: [`WanderTuning::walk_slack_s`].
pub const WANDER_WALK_SLACK_S: Def<f32> = def!(float "mw.ai.wander.walk_slack_s" = 5.0)
    .range(0.0, 60.0)
    .unit(Unit::Seconds)
    .help("Seconds added to the pathgrid fallback walk backstop")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.wander.same_goal_m`: [`WanderTuning::same_goal_m`].
pub const WANDER_SAME_GOAL_M: Def<f32> = def!(float "mw.ai.wander.same_goal_m" = 0.5)
    .range(0.0, 10.0)
    .unit(Unit::Meters)
    .help("A new wander goal this close to the goal just given up is rolled once more")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.wander.give_up_pause_min_s`: [`WanderTuning::give_up_pause_min_s`].
pub const WANDER_GIVE_UP_PAUSE_MIN_S: Def<f32> =
    def!(float "mw.ai.wander.give_up_pause_min_s" = 1.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("Seconds a wanderer stands after giving up a walk, before the random extra")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// `mw.ai.wander.give_up_pause_jitter_s`: [`WanderTuning::give_up_pause_jitter_s`].
pub const WANDER_GIVE_UP_PAUSE_JITTER_S: Def<f32> =
    def!(float "mw.ai.wander.give_up_pause_jitter_s" = 1.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("Random extra seconds (uniform) a wanderer stands after giving up a walk")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The wander drive's live tunables (`mw.ai.wander.*`) and its `GoTo` executor's (`mw.ai.travel.*`).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &WANDER_WALK_BACKSTOP_MULT,
    &WANDER_WALK_SLACK_S,
    &WANDER_SAME_GOAL_M,
    &WANDER_GIVE_UP_PAUSE_MIN_S,
    &WANDER_GIVE_UP_PAUSE_JITTER_S,
    // MA2d2: the `GoTo` executor (`crate::directive_drive`).
    &crate::directive_drive::TRAVEL_END_TOLERANCE_M,
    &crate::directive_drive::TRAVEL_FINISH_S,
    &crate::directive_drive::TRAVEL_MAX_RANGE_M,
    &crate::directive_drive::TRAVEL_RETRY_S,
];

/// Live wander feel values bound from [`TUNABLES`]; [`drive`] copies them into every
/// [`WanderDriver::tuning`].
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WanderSettings {
    pub tuning: WanderTuning,
}

impl Default for WanderSettings {
    fn default() -> Self {
        Self {
            tuning: WanderTuning {
                walk_backstop_mult: WANDER_WALK_BACKSTOP_MULT.default_value(),
                walk_slack_s: WANDER_WALK_SLACK_S.default_value(),
                same_goal_m: WANDER_SAME_GOAL_M.default_value(),
                give_up_pause_min_s: WANDER_GIVE_UP_PAUSE_MIN_S.default_value(),
                give_up_pause_jitter_s: WANDER_GIVE_UP_PAUSE_JITTER_S.default_value(),
            },
        }
    }
}

fn add_wander_tunables(app: &mut App) {
    let _mult = app
        .add_tunable(&WANDER_WALK_BACKSTOP_MULT)
        .bind_resource(|s: &mut WanderSettings, v| s.tuning.walk_backstop_mult = v);
    let _slack = app
        .add_tunable(&WANDER_WALK_SLACK_S)
        .bind_resource(|s: &mut WanderSettings, v| s.tuning.walk_slack_s = v);
    let _goal = app
        .add_tunable(&WANDER_SAME_GOAL_M)
        .bind_resource(|s: &mut WanderSettings, v| s.tuning.same_goal_m = v);
    let _pause = app
        .add_tunable(&WANDER_GIVE_UP_PAUSE_MIN_S)
        .bind_resource(|s: &mut WanderSettings, v| s.tuning.give_up_pause_min_s = v);
    let _jitter = app
        .add_tunable(&WANDER_GIVE_UP_PAUSE_JITTER_S)
        .bind_resource(|s: &mut WanderSettings, v| s.tuning.give_up_pause_jitter_s = v);
}

// tunables: const (log cadence only: how often the report lines re-check; gates no behaviour)
const REPORT_INTERVAL_S: f32 = 5.0;

/// Which mode the awake wanderers run (`MA3`) and what their walks did, since start.
#[derive(Resource, Debug, Default)]
pub struct WanderModes {
    /// Awake wanderers on the HTN + navmesh path / on the pathgrid fallback, as last logged.
    pub navmesh: usize,
    pub fallback: usize,
    /// Walks that reached their goal, by mode.
    pub navmesh_arrivals: u64,
    pub fallback_arrivals: u64,
    /// Navmesh walks given up (no path, or 3 s without progress).
    pub navmesh_give_ups: u64,
    next: f32,
}

/// Logs `ai: N wanderers on navmesh, M on pathgrid fallback ...` when the split changes, checked
/// every 5 s (not per frame): one line after a cell load settles, so a run's log shows MA3 live.
#[allow(clippy::needless_pass_by_value)]
fn report_modes(time: Res<Time>, mut modes: ResMut<WanderModes>, walkers: Query<&Walker, Without<Dormant>>) {
    let now = time.elapsed_secs();
    if now < modes.next {
        return;
    }
    modes.next = now + REPORT_INTERVAL_S;
    let (mut navmesh, mut fallback) = (0, 0);
    for w in &walkers {
        match w.on_navmesh {
            Some(true) => navmesh += 1,
            Some(false) => fallback += 1,
            None => {}
        }
    }
    if (navmesh, fallback) == (modes.navmesh, modes.fallback) {
        return;
    }
    modes.navmesh = navmesh;
    modes.fallback = fallback;
    info!(
        "ai: {navmesh} wanderers on navmesh, {fallback} on pathgrid fallback (since start: {} navmesh walks arrived, {} given up; {} fallback walks arrived)",
        modes.navmesh_arrivals, modes.navmesh_give_ups, modes.fallback_arrivals
    );
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
    *next = now + REPORT_INTERVAL_S;
    let (mut walkers, mut walking, mut grounded, mut dormant, mut stuck, mut blocked) = (0, 0, 0, 0, 0, 0);
    for (m, w, d) in &movers {
        walkers += usize::from(w.is_some());
        dormant += usize::from(d);
        stuck += w.map_or(0, |w| w.driver.stuck_count());
        walking += usize::from(m.gait.moving);
        // Pushing without progress (blocked, crowded): the graph is told the
        // measured speed, so these stand instead of walking in place.
        blocked += usize::from(!d && m.applied.desired_velocity.length() > 0.5 && m.measured.length() < 0.25);
        // Of the simulating movers (dormant ones keep their last state).
        grounded += usize::from(
            !d && (m.loco.grounded.is_some() || !matches!(m.loco.mode, MoverMode::Ground)),
        );
    }
    info!(
        "walkers: {} movers + {dormant} dormant ({walkers} wander, {walking} moving, {grounded} awake grounded, {stuck} walks given up: {} steep, {} wall, {} other; {} on a first leg, {} off the pathgrid; {} rerouted around a blocked edge), {} statics, terrain {}; last frame avoid {:.0} us, move {:.0} us, feed {:.0} us",
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
        // Feet as drawn (the interpolated root), never the tick state.
        let frame = m.drawn_frame();
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
                    m.loco.moved.translation,
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

/// Walks the wander driver gave up for lack of progress, by what stopped them:
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

/// The physics fell-out safety net caught a walking character under the
/// kill height and put it back on its last ground: collision was missing
/// under it (a streaming gap the collision-ready gate should have held).
fn log_fell_out(mut fell: MessageReader<sf_physics_bevy::CharacterFellOut>, names: Query<&Character>) {
    for f in fell.read() {
        let who = match names.get(f.entity) {
            Ok(Character::Record { id, .. }) => id.as_str().to_owned(),
            _ => format!("{}", f.entity),
        };
        warn!(
            "walkers: {who} fell out of the world at {:.1?}, returned to its last ground {:.1?}",
            f.from.to_f64(),
            f.to.to_f64()
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, Update};
    use sf_controller_bevy::{Dormant, Held, Mover, MoverParams};
    use sf_coords::WorldPos;
    use sf_world::WorldspaceId;
    use sf_world_bevy::{ActiveWorldspace, Streaming, StreamingConfig};

    use super::{ActorSpace, GroundOnPhysics, MwGround, processing_range};

    /// The processing range never touches a [`Held`] mover (a Delirium
    /// takeover, a ragdoll): out of range it doesn't park it, so in range it
    /// can't wake it either; the same filter. Lowest level: `processing_range`
    /// alone (an in-range cell needs a driven streaming scheduler), so the held
    /// mover's `Dormant` is taken off by hand to see the range leave it be.
    #[test]
    fn processing_range_leaves_held_movers_alone() {
        let ws = WorldspaceId::new("test");
        let mut app = App::new();
        app.insert_resource(Streaming::new(StreamingConfig::default()))
            .insert_resource(ActiveWorldspace(None))
            .init_resource::<MwGround>()
            .insert_resource(GroundOnPhysics(false))
            .add_systems(Update, processing_range);
        let mover = || Mover::new(WorldPos::ORIGIN, 0.0, MoverParams::default(), WorldPos::ORIGIN);
        let free = app.world_mut().spawn((mover(), ActorSpace(ws.clone()))).id();
        let held = app.world_mut().spawn((mover(), ActorSpace(ws), Held)).id();
        assert!(app.world().get::<Dormant>(held).is_some(), "Held brings Dormant");
        app.world_mut().entity_mut(held).remove::<Dormant>();
        for _ in 0..30 {
            app.update();
        }
        assert!(app.world().get::<Dormant>(free).is_some(), "out of range: parked");
        assert!(app.world().get::<Dormant>(held).is_none(), "the range touched a held mover");
        assert!(app.world().get::<Held>(held).is_some());
    }

    /// The `mw.ai.wander.*` tunable defaults and [`mw_ai::WanderTuning::default`] (the Bevy-free
    /// twin `mw_ai` uses headless) are the same values, field by field.
    #[test]
    fn wander_tunable_defaults_match_wander_tuning_default() {
        assert_eq!(
            super::WanderSettings::default().tuning,
            mw_ai::WanderTuning::default()
        );
    }
}

#[cfg(test)]
#[path = "drive_tests.rs"]
mod drive_tests;

/// The drive tests' harness for the tier-0 tests (`crate::logical`): the game's fixed-tick drive
/// over a flat 40 m navmesh room in `nav_room` ([`drive_tests::app_with`]) and a wanderer as
/// `attach` makes it ([`drive_tests::spawn_wanderer`]).
#[cfg(test)]
pub(crate) mod test_harness {
    use bevy_app::App;
    use bevy_ecs::entity::Entity;
    use sf_coords::WorldPos;
    use sf_world::WorldspaceId;

    pub(crate) fn app_with(extra: impl FnOnce(&mut App)) -> App {
        super::drive_tests::app_with(extra)
    }

    pub(crate) fn spawn_wanderer(app: &mut App, ws: &WorldspaceId, home: WorldPos, key: u128, seed: u64) -> Entity {
        super::drive_tests::spawn_wanderer(app, ws, home, key, seed)
    }
}

#[cfg(test)]
#[path = "sight_tests.rs"]
mod sight_tests;

#[cfg(test)]
#[path = "search_tests.rs"]
mod search_tests;

#[cfg(test)]
#[path = "directive_drive_tests.rs"]
mod directive_drive_tests;

#[cfg(test)]
#[path = "script_ai_tests.rs"]
mod script_ai_tests;

#[cfg(test)]
#[path = "greeting_tests.rs"]
mod greeting_tests;

#[cfg(test)]
#[path = "greeting_sight_tests.rs"]
mod greeting_sight_tests;
