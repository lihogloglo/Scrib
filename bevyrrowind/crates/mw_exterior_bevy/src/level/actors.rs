//! Level actors (TS1 part 2, `docs/specs/test-levels.md` "Actors"): the
//! level's `actors` become actor placements in
//! the level's streaming index, so the streamed world spawns them like any
//! cell's NPCs. [`LevelActorSpawner`] dresses them in their outfit and tags
//! them [`LevelActor`]; `walkers.rs` gives them their `Mover` (and drops the
//! `AI_W` scaffold here: [`script`]); [`drive`] steers them along their
//! scripted path (`sf_controller::PathFollower`: loop, back-and-forth or
//! once, walk or run) and fires their periodic action while they stand.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use mw_graphs::build::params as gp;
use sf_anim_bevy::Animator;
use sf_anim_graph::FidgetPicker;
use sf_character::{CharacterState, WearableRef};
use sf_character_bevy::{Character, CharacterSpawner, Characters};
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
        world.spawn((character, local, Visibility::Inherited, ChildOf(cx.parent), cx.layers.clone(), LevelActor { index: *index }, name, sf_character_bevy::AnimSeed::from_id(p.id.0)));
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
        .add_systems(Update, (script, drive).chain().in_set(ControllerSystems::Drive));
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
/// `AI_W` scaffold and local avoidance (scripted paths stay on their line).
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

/// Paths write the intent; actions fire while standing.
#[allow(clippy::needless_pass_by_value)]
fn drive(
    time: Res<Time>,
    mut actors: Query<(Entity, &Mover, &mut MoveIntent, &mut ActorScript), Without<Dormant>>,
    mut animators: Query<&mut Animator>,
    mut impacts: MessageWriter<ImpactOnCharacter>,
) {
    let dt = time.delta_secs().min(0.1);
    for (entity, mover, mut intent, mut script) in &mut actors {
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
                impacts.write(ImpactOnCharacter { entity, impulse: param("impulse", 25.0), point, direction });
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
