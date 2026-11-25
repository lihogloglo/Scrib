//! Level actors (TS1 part 2, `docs/specs/test-levels.md` "Actors"): the
//! level's `actors` become actor placements in
//! the level's streaming index, so the streamed world spawns them like any
//! cell's NPCs. [`LevelActorSpawner`] dresses them in their outfit and tags
//! them [`LevelActor`]; `walkers.rs` gives them their `Mover` (and drops the
//! `AI_W` scaffold here: [`script`]); [`drive`] steers them along their
//! scripted path (`sf_controller::PathFollower`: loop, back-and-forth or
//! once, walk or run) and fires their periodic action while they stand.
//! The script is the actor's routine: perception reactions (`mw_walkers_bevy::reaction`, `AI8i`:
//! a search, then the walk back to where it left its script) pre-empt it, and [`drive`] runs
//! after them and after the navmesh corridor's idle write, skipping while a reaction owns the actor.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use mw_graphs::build::params as gp;
use sf_anim_bevy::Animator;
use sf_anim_graph::FidgetPicker;
use sf_character::{CharacterState, WearableRef};
use sf_character_bevy::{Character, CharacterPlacement, CharacterSpawner, Characters};
use sf_controller::{Gait, PathFollower, PathMode, PathNode};
use sf_controller_bevy::{Avoider, ControllerSystems, Dormant, MoveIntent, Mover};
use sf_coords::{WorldPos, WorldTransform};
use sf_level::Level;
use sf_physics_bevy::ImpactOnCharacter;
use sf_records::{RecordDb, RecordId};
use sf_world::{IndexedPlacement, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};
use sf_world_bevy::{PlacementSpawner, PlacementSpawners, SpawnCx};
use tes3::esp::{Creature, Npc};

/// Streaming cells are 128 m: one marker per 64 m keeps every cell under the
/// level in the directory (cells without content count as unloaded, and a
/// mover there would park: walkers' processing range).
const MARKER_SPACING: f32 = 64.0;

/// One actor resolved against the records.
#[derive(Clone)]
pub struct ActorSetup {
    pub def: sf_level::Actor,
    /// Record kind (`NPC_`, `CREA`) of `def.id`.
    pub kind: &'static str,
    pub placement: PlacementId,
}

/// The level's actors with their record kinds; unknown ids are skipped with a warning. The
/// player is the game's own (`player.rs`, F11), not a level actor.
pub fn resolve(level: &Level, db: &RecordDb) -> Vec<ActorSetup> {
    level
        .actors
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, def)| {
            let id = RecordId::new(&def.id);
            let kind = if db.get::<Npc>(&id).is_some() {
                "NPC_"
            } else if db.get::<Creature>(&id).is_some() {
                "CREA"
            } else {
                warn!("level {}: actor {:?} is no NPC or creature record, skipped", level.name, def.id);
                return None;
            };
            let placement = PlacementId::from_key("level", format!("{}/actor/{i}", level.name).as_bytes());
            Some(ActorSetup { def, kind, placement })
        })
        .collect()
}

/// The level's `placed` records as world placements, resolved like a cell reference's base
/// (`mw_world::source::resolve_base`: kind, class and model), so a `LIGH` is a real lamp (its
/// mesh, its light, and the AI light inputs' `LightHook`, MA16) and a `STAT` a real static.
/// Unknown ids are skipped with a warning.
pub fn resolve_placed(level: &Level, db: &RecordDb) -> Vec<Placement> {
    level
        .placed
        .iter()
        .enumerate()
        .filter_map(|(i, def)| {
            let Some(base) = mw_world::source::resolve_base(db, &def.id) else {
                warn!("level {}: placed {:?} is no placeable record, skipped", level.name, def.id);
                return None;
            };
            Some(Placement {
                id: PlacementId::from_key("level", format!("{}/placed/{i}", level.name).as_bytes()),
                base: RecordRef::new(base.tag, def.id.as_str()),
                model: base.model.and_then(|m| sf_vfs::VfsPath::new(&m).ok()),
                transform: WorldTransform {
                    pos: WorldPos::from_f64(def.at.as_dvec3()),
                    // Engine yaw (0 faces -z), as `mw_nif::coords::reference_rotation` turns a reference.
                    rotation: Quat::from_rotation_y(def.yaw.to_radians()),
                    scale: Vec3::ONE,
                },
                class: base.class,
                flags: PlacementFlags::empty(),
            })
        })
        .collect()
}

/// The level's placements: its actors at their spawns, and invisible
/// markers covering the level's footprint (see [`MARKER_SPACING`]).
pub fn placements(level: &Level, actors: &[ActorSetup]) -> Vec<Placement> {
    let mut out: Vec<Placement> = actors
        .iter()
        .filter_map(|a| {
            let spawn = level.spawn(&a.def.spawn)?;
            Some(Placement {
                id: a.placement,
                base: RecordRef::new(a.kind, a.def.id.as_str()),
                model: None,
                transform: WorldTransform {
                    pos: WorldPos::from_f64(spawn.at.as_dvec3()),
                    rotation: Quat::from_rotation_y(spawn.yaw.to_radians()),
                    scale: Vec3::ONE,
                },
                class: PlacementClass::Actor,
                flags: PlacementFlags::empty(),
            })
        })
        .collect();
    if let Some((lo, hi)) = level.bounds() {
        let steps = |a: f32, b: f32| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let n = ((b - a) / MARKER_SPACING).ceil().max(0.0) as u32;
            #[allow(clippy::cast_precision_loss)]
            (0..=n).map(move |i| (a + i as f32 * MARKER_SPACING).min(b))
        };
        for x in steps(lo.x, hi.x) {
            for z in steps(lo.z, hi.z) {
                out.push(Placement {
                    id: PlacementId::from_key("level", format!("{}/marker/{x},{z}", level.name).as_bytes()),
                    base: RecordRef::new("level", "marker"),
                    model: None,
                    transform: WorldTransform::from_pos(WorldPos::from_f64(Vec3::new(x, 0.0, z).as_dvec3())),
                    class: PlacementClass::Marker,
                    flags: PlacementFlags::empty(),
                });
            }
        }
    }
    out
}

/// A level actor (index into the resolved list).
#[derive(Component, Clone, Copy, Debug)]
pub struct LevelActor {
    pub index: usize,
}

/// Spawns the level's actor placements as characters in their outfit,
/// tagged [`LevelActor`]; other actor placements as `CharacterSpawner` does.
struct LevelActorSpawner {
    actors: HashMap<PlacementId, (usize, CharacterState)>,
}

impl PlacementSpawner for LevelActorSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let p = &placement.placement;
        let Some((index, state)) = self.actors.get(&p.id) else { return CharacterSpawner.spawn(world, cx, placement) };
        if cx.level == sf_world::Level::Far {
            return 0;
        }
        let Some(turn) = world.get_resource::<Characters>().map(|c| c.content().placement_rotation()) else { return 0 };
        let mut local = cx.local;
        local.rotation *= turn;
        let character = Character::Record { kind: p.base.kind.clone(), id: p.base.id.clone(), state: state.clone() };
        let name = Name::new(format!("level actor {} ({})", index, p.base.id.as_str()));
        // `CharacterPlacement` as `CharacterSpawner` gives it: the stable id the walkers key the
        // NPC's AI by (`AiActor` + the sight observer, MA14), so level actors perceive (AI47).
        world.spawn((
            character,
            CharacterPlacement(p.id),
            local,
            Visibility::Inherited,
            ChildOf(cx.parent),
            cx.layers.clone(),
            LevelActor { index: *index },
            name,
            sf_character_bevy::AnimSeed::from_id(p.id.0),
        ));
        1
    }

    fn needs_model(&self, _: &IndexedPlacement) -> bool {
        false
    }
}

/// The resolved actors, for the systems.
#[derive(Resource)]
struct LevelActors(Arc<[ActorSetup]>);

pub fn add(app: &mut App, actors: Arc<[ActorSetup]>) {
    app.insert_resource(LevelActors(actors))
        .add_message::<ImpactOnCharacter>()
        .add_systems(PostStartup, install)
        .add_systems(Update, script)
        // The path follower is NPC gameplay: on the fixed tick, with the walkers (NV7a); after the
        // reactions and the corridor (which writes `IDLE` for a goal-less corridor), so the path's
        // intent is the last word on the tick and the two never fight (`AI8i`).
        .add_systems(
            FixedUpdate,
            drive
                .in_set(ControllerSystems::Drive)
                .after(sf_nav_bevy::NavSystems::Corridor)
                .after(mw_walkers_bevy::reaction::ReactionSystems::Apply),
        );
}

/// Replaces the viewer's actor spawner (when characters are on).
fn install(actors: Res<LevelActors>, spawners: Option<ResMut<PlacementSpawners>>, characters: Option<Res<Characters>>) {
    let (Some(mut spawners), Some(_)) = (spawners, characters) else { return };
    let map = actors
        .0
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let worn = a.def.outfit.as_ref().map(|o| o.iter().map(|item| WearableRef::new(item)).collect());
            (a.placement, (i, CharacterState { worn, held: None }))
        })
        .collect();
    spawners.set(PlacementClass::Actor, Arc::new(LevelActorSpawner { actors: map }));
}

/// A level actor's script state.
#[derive(Component)]
pub struct ActorScript {
    follower: Option<PathFollower>,
    action: Option<(sf_level::Action, f32)>,
    /// `@fidget` actions: which idle to play and when (idle and crowd variety).
    fidgets: Option<FidgetPicker>,
}

/// Once `walkers.rs` gave a level actor its mover: its script replaces the
/// `AI_W` scaffold and local avoidance (scripted paths stay on their line). The perception
/// reactions (`Reaction`, the navmesh corridor and the searcher) stay: a scripted actor still
/// searches and walks back (`AI8i`).
fn script(mut commands: Commands, actors: Res<LevelActors>, fresh: Query<(Entity, &LevelActor, &Mover), Added<Mover>>) {
    for (entity, actor, mover) in &fresh {
        let Some(setup) = actors.0.get(actor.index) else { continue };
        let follower = setup.def.path.as_ref().map(|p| {
            let n = p.points.len();
            let nodes = p
                .points
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let stop = match p.mode {
                        sf_level::PathMode::Loop => i == 0,
                        sf_level::PathMode::PingPong => i == 0 || i + 1 == n,
                        sf_level::PathMode::Once => i + 1 == n,
                    };
                    PathNode { wait: if stop { p.wait } else { 0.0 }, ..PathNode::at(WorldPos::from_f64(v.as_dvec3())) }
                })
                .collect();
            let mode = match p.mode {
                sf_level::PathMode::Loop => PathMode::Loop,
                sf_level::PathMode::PingPong => PathMode::PingPong,
                sf_level::PathMode::Once => PathMode::Once,
            };
            // Brakes into stops like this mover (arrival deceleration, velocity spring).
            let mut f = PathFollower::new(nodes, mode, p.speed).for_mover(&mover.params);
            f.gait = if p.run { Gait::Run } else { Gait::Walk };
            f
        });
        let (action, fidgets) = match setup.def.action.clone() {
            Some(a) if a.trigger == FIDGET_TRIGGER => (None, Some(fidget_picker(&a, actor.index))),
            a => (a.map(|a| (a, 0.0)), None),
        };
        let mut e = commands.entity(entity);
        e.try_remove::<(mw_walkers_bevy::walkers::Walker, Avoider)>().try_insert(ActorScript { follower, action, fidgets });
    }
}

/// An action with this trigger is no graph trigger: it hits the actor's capsule like a thrown
/// body (`ImpactOnCharacter`; params `impulse` N·s, default 25, and `dir` degrees, 0 = -z), so
/// physics.rs knocks it down (PH-9) or reacts (PH-10) and it gets up again.
const IMPACT_TRIGGER: &str = "@impact";

/// An action with this trigger plays idle fidgets (`FidgetPicker`): every `every` to `max` seconds
/// while the actor stands, one of `Idle2`..`Idle9` weighted by the params `idle2`..`idle9` (the AI
/// package's idle chances, 0 to 100), never the same one twice in a row.
const FIDGET_TRIGGER: &str = "@fidget";

/// The picker an `@fidget` action describes; `index` seeds it, so level actors fidget out of step.
fn fidget_picker(action: &sf_level::Action, index: usize) -> FidgetPicker {
    let param = |n: &str, d: f32| action.params.iter().find(|p| p.0 == n).map_or(d, |p| p.1);
    let weights: Vec<f32> = (2..=9).map(|v| param(&format!("idle{v}"), 0.0)).collect();
    FidgetPicker::new(&weights, action.every, param("max", action.every * 2.0), sf_anim_graph::variety::mix64(index as u64))
}

/// Paths write the intent; actions fire while standing (`FixedUpdate`: `dt` is the tick). Paused
/// (path, actions, fidgets) while a perception reaction owns the actor.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn drive(
    time: Res<Time>,
    mut actors: Query<
        (Entity, &Mover, &mut MoveIntent, &mut ActorScript, Option<&mw_walkers_bevy::reaction::Reaction>),
        Without<Dormant>,
    >,
    mut animators: Query<&mut Animator>,
    mut impacts: MessageWriter<ImpactOnCharacter>,
) {
    let dt = time.delta_secs().min(0.1);
    for (entity, mover, mut intent, mut script, reaction) in &mut actors {
        if reaction.is_some_and(mw_walkers_bevy::reaction::Reaction::owns) {
            continue;
        }
        let script = &mut *script;
        let mut standing = true;
        if let Some(f) = &mut script.follower {
            let out = f.update(mover.loco.position, dt);
            standing = out.intent.desired_velocity.length_squared() < 1e-6;
            intent.0 = out.intent;
        }
        if let Some(picker) = &mut script.fidgets {
            if !standing {
                picker.restart();
            } else if let Some(i) = picker.update(dt)
                && let Some(mut animator) = mover.animator.and_then(|a| animators.get_mut(a).ok())
            {
                animator.set_float(gp::IDLE_VARIANT, f32::from(u8::try_from(i + 2).unwrap_or(2)));
                animator.fire(gp::FIDGET);
            }
            continue;
        }
        let Some((action, t)) = &mut script.action else { continue };
        if !standing {
            *t = 0.0;
            continue;
        }
        *t += dt;
        if *t >= action.every {
            *t = 0.0;
            if action.trigger == IMPACT_TRIGGER {
                let param = |n: &str, d: f32| action.params.iter().find(|p| p.0 == n).map_or(d, |p| p.1);
                let dir = param("dir", 0.0).to_radians();
                let direction = Vec3::new(-dir.sin(), 0.0, -dir.cos());
                let point = mover.loco.position.offset(Vec3::Y * 1.2);
                impacts.write(ImpactOnCharacter { entity, impulse: param("impulse", 25.0), point, direction, shape: None });
                continue;
            }
            if let Some(mut animator) = mover.animator.and_then(|a| animators.get_mut(a).ok()) {
                for (name, v) in &action.params {
                    animator.set_float(name, *v);
                }
                animator.fire(&action.trigger);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! The level path drive against the navmesh corridor (`AI8i`): a level path actor keeps its
    //! goal-less `NavCorridor` (the reactions' corridor), which writes `IDLE` every tick; [`drive`]
    //! runs after it, so the path's intent is what the mover gets.

    use std::time::Duration;

    use bevy::app::TaskPoolPlugin;
    use bevy::time::{TimePlugin, TimeUpdateStrategy};
    use sf_controller::{GroundHit, GroundProbe, HeightSurface, ProbeQuery, TerrainProbe};
    use sf_controller_bevy::{MoverParams, SfControllerPlugin};
    // `sf_ids` through a dependency that re-exports it (no new dev-dependency for one key).
    use sf_sky_bevy::sf_ids::ActorKey;
    use sf_nav::{AgentClassId, GridBackend, GridConfig};
    use sf_nav_bevy::{NavCorridor, NavCorridorPlugin, NavMeshes, NavPlugin, NavQueryPlugin};
    use sf_world::WorldspaceId;

    use super::*;

    struct Flat;

    impl HeightSurface for Flat {
        fn height(&self, _p: WorldPos) -> Option<f32> {
            Some(0.0)
        }
        fn normal(&self, _p: WorldPos) -> Option<Vec3> {
            Some(Vec3::Y)
        }
        fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
            if dir.y.abs() < 1e-6 {
                return None;
            }
            let t = -WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
            (0.0..=max).contains(&t).then_some((t, Vec3::Y))
        }
    }

    #[derive(Resource)]
    struct Floor(TerrainProbe<Flat>);

    impl GroundProbe for Floor {
        fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
            self.0.probe(queries, out);
        }
    }

    #[test]
    fn a_level_path_actor_walks_its_path_beside_its_idle_corridor() {
        let mut app = App::new();
        app.add_plugins((
            TaskPoolPlugin::default(),
            TimePlugin,
            SfControllerPlugin::<Floor>::default(),
            NavPlugin,
            NavQueryPlugin,
            NavCorridorPlugin,
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)))
        .insert_resource(Floor(TerrainProbe(Flat)))
        .insert_resource(NavMeshes(Box::new(GridBackend::new(GridConfig { cell_size: 1.0, tile_cells: 8 }).unwrap())));
        add(&mut app, Arc::from(Vec::<ActorSetup>::new()));
        app.update();
        let start = WorldPos::from_f64(bevy::math::DVec3::new(10.0, 0.0, 10.0));
        let end = start.offset(Vec3::new(5.0, 0.0, 0.0));
        let params = MoverParams::default();
        let speed = 1.5;
        let follower = PathFollower::new(vec![PathNode::at(start), PathNode::at(end)], PathMode::Once, speed).for_mover(&params);
        let ws = WorldspaceId::new("level_path");
        let e = app
            .world_mut()
            .spawn((
                Mover::new(start, 0.0, params, WorldPos::ORIGIN),
                MoveIntent::default(),
                NavCorridor::new(ActorKey(1), AgentClassId(0), ws, speed),
                mw_walkers_bevy::reaction::Reaction::new(speed),
                ActorScript { follower: Some(follower), action: None, fidgets: None },
            ))
            .id();
        let (mut walking_ticks, mut idle_ticks, mut done) = (0_u32, 0_u32, None);
        for tick in 0..(10 * 60) {
            app.update();
            let world = app.world();
            let pos = world.get::<Mover>(e).unwrap().loco.position;
            let off = start.delta_f32(pos);
            assert!(off.is_finite() && off.y.abs() < 0.5, "tick {tick}: {off:?}");
            if done.is_none() {
                // Short of the end's braking (the follower's own stop coasts to rest on its spring).
                let to_end = pos.delta_f32(end).with_y(0.0).length();
                if world.get::<MoveIntent>(e).unwrap().0.desired_velocity.length_squared() > 1e-6 {
                    walking_ticks += 1;
                } else if to_end > 0.5 {
                    idle_ticks += 1;
                }
                if to_end < 0.2 {
                    done = Some(tick);
                }
            }
        }
        let done = done.expect("the level path actor never reached the end of its path");
        println!("level path: reached the end on tick {done}, {walking_ticks} walking / {idle_ticks} idle intents short of its brake");
        // 5 m at 1.5 m/s (plus the start and the brake): the corridor's idle never took the intent.
        assert_eq!(idle_ticks, 0, "{idle_ticks} ticks of IDLE intent while walking the path");
        assert!(done < 8 * 60, "took {done} ticks for 5 m");
    }
}
