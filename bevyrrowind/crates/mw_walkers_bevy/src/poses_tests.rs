//! `AI10c3a`: an actor's moved body keeps its pose across its cell's unload ([`ActorPoseHook`]
//! writes it to the overlay at deactivation; the overlay loads it with the cell it stands in;
//! `attach` respawns the body there, facing that way).

use std::f32::consts::PI;
use std::sync::{Arc, PoisonError};

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec3, Quat, Vec3};
use bevy_transform::components::Transform;
use sf_character::{CharacterState, OutfitKey};
use sf_character_bevy::{Character, CharacterBody, CharacterPlacement, OutfitCacheKey, SmoothBodies};
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId};
use sf_world::{IndexedPlacement, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, PlacementOverlay, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, WorldOverlay, WorldspaceRoot};

use super::{ActorPoseHook, ResidentActors, persist_actor_poses, placement_rotation, placement_yaw};
use crate::walkers::{ActorSpace, WalkerContent};

const ID: u128 = 42;

/// Placement spawner that spawns nothing (the hook's inner one).
struct Nothing;

impl PlacementSpawner for Nothing {
    fn spawn(&self, _world: &mut World, _cx: &SpawnCx<'_>, _placement: &IndexedPlacement) -> u32 {
        0
    }
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_exterior")
}

fn at(x: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, z))
}

/// An actor placement at `pos`, facing `yaw`, scale 1.1.
fn actor(pos: WorldPos, yaw: f32) -> IndexedPlacement {
    let placement = Placement {
        id: PlacementId(ID),
        base: RecordRef::new("NPC_", "test_npc"),
        model: None,
        transform: WorldTransform { pos, rotation: Quat::from_rotation_y(yaw), scale: Vec3::splat(1.1) },
        class: PlacementClass::Actor,
        flags: PlacementFlags::empty(),
    };
    IndexedPlacement { placement, radius: 0.5 }
}

/// The overlay, the resident index and a body for `ID` at `pos` facing `yaw`.
fn world_with_body(pos: WorldPos, yaw: f32) -> (World, Entity) {
    let mut world = World::new();
    world.init_resource::<WorldOverlay>();
    world.init_resource::<ResidentActors>();
    let body = world
        .spawn((CharacterPlacement(PlacementId(ID)), Mover::new(pos, yaw, MoverParams::default(), WorldPos::ORIGIN), ActorSpace(ws())))
        .id();
    (world, body)
}

fn cycle(world: &mut World, placement: &IndexedPlacement, discard: bool) {
    let hook = ActorPoseHook(Arc::new(Nothing));
    let cell_entity = world.spawn_empty().id();
    let w = ws();
    let cx = ActivateCx { worldspace: &w, cell: IVec3::ZERO, cell_entity, discard };
    hook.activate(world, &cx, placement);
    assert_eq!(world.resource::<ResidentActors>().len(), 1, "resident while Active");
    hook.deactivate(world, &cx, placement);
    assert!(world.resource::<ResidentActors>().is_empty(), "gone with its cell");
}

fn stored(world: &World) -> Option<(WorldspaceId, Placement)> {
    let overlay = world.resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.placed(PlacementId(ID)).map(|(w, p)| (w.clone(), p.clone()))
}

fn planar(a: WorldPos, b: WorldPos) -> f32 {
    a.delta_f32(b).with_y(0.0).length()
}

/// Home at x = 126 (streaming cell 0); the body walked 5 m on to x = 131 (cell 1) and turned.
fn moved_cycle() -> (World, IndexedPlacement, WorldPos, f32) {
    let home = actor(at(126.0, 10.0), 0.0);
    let (to, yaw) = (at(131.0, 10.0), 1.0);
    let (mut world, _) = world_with_body(to, yaw);
    cycle(&mut world, &home, false);
    (world, home, to, yaw)
}

#[test]
fn a_moved_and_turned_body_is_written_to_the_overlay_at_deactivation() {
    let (world, home, to, yaw) = moved_cycle();
    let (w, p) = stored(&world).expect("an overlay entry");
    assert_eq!(w, ws());
    assert!(planar(p.transform.pos, to) < 1e-4, "{:?}", p.transform.pos);
    assert!((placement_yaw(p.transform.rotation, Quat::IDENTITY) - yaw).abs() < 1e-4);
    assert_eq!(p.transform.scale, home.placement.transform.scale, "scale unchanged");
    assert_eq!((p.base.clone(), p.class), (home.placement.base.clone(), PlacementClass::Actor));
}

#[test]
fn a_body_that_did_not_move_or_a_discard_writes_nothing() {
    let home = actor(at(126.0, 10.0), 0.5);
    // Settled 4 cm onto the ground at spawn: not a move.
    let (mut world, _) = world_with_body(home.placement.transform.pos.offset(Vec3::new(0.0, -0.04, 0.0)), 0.5);
    cycle(&mut world, &home, false);
    assert!(stored(&world).is_none(), "an actor that never moved adds no entry");
    let (mut world, _) = world_with_body(at(131.0, 10.0), 1.0);
    cycle(&mut world, &home, true);
    assert!(stored(&world).is_none(), "a reload after a save load writes nothing");
}

#[test]
fn the_overlay_loads_the_moved_actor_with_the_cell_it_stands_in() {
    let (world, home, to, _) = moved_cycle();
    let partition = Partition::STREAMING;
    let (old, new) = (partition.cell_of(home.placement.transform.pos), partition.cell_of(to));
    assert_ne!(old, new, "the 5 m cross a cell border");
    let overlay = world.resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    let index = [home.clone()];
    assert!(overlay.apply(&ws(), partition, old, &index).is_empty(), "the home cell no longer lists it");
    let there = overlay.apply(&ws(), partition, new, &[]);
    assert_eq!(there.len(), 1);
    assert_eq!(there[0].placement.id, PlacementId(ID));
    assert!(planar(there[0].placement.transform.pos, to) < 1e-4);
}

/// The stored rotation takes the content's spawn turn off again (Morrowind turns placed actors).
#[test]
fn the_stored_rotation_spawns_the_body_facing_its_yaw_under_a_turn() {
    let turn = Quat::from_rotation_y(PI);
    for yaw in [-2.5, 0.0, 1.0, 3.0] {
        let back = placement_yaw(placement_rotation(yaw, turn), turn);
        let d = (back - yaw).rem_euclid(std::f32::consts::TAU);
        assert!(d.min(std::f32::consts::TAU - d) < 1e-4, "{yaw} -> {back}");
    }
}

/// Round trip: a character spawned from the moved placement, as `CharacterSpawner` does (child
/// of its cell, local transform), gets from `attach` a mover at the stored feet and yaw.
#[test]
fn attach_puts_the_respawned_body_at_the_stored_pose() {
    let (world, _, to, yaw) = moved_cycle();
    let (_, p) = stored(&world).unwrap();
    let mut app = App::new();
    let mut db = RecordDb::new();
    db.register::<tes3::esp::Npc>(mw_actors::NPC_KIND).expect("npc kind");
    db.register::<tes3::esp::Creature>(mw_actors::CREATURE_KIND).expect("crea kind");
    app.insert_resource(WalkerContent { db: Arc::new(db), min_walk: 1.0, max_walk: 2.0, idle_mult: 1.0 })
        .add_systems(Update, crate::walkers::attach);
    let world = app.world_mut();
    let root = world.spawn((WorldspaceRoot { id: ws(), layer: 0 }, Transform::IDENTITY)).id();
    let (cell, local) = sf_world_bevy::to_grid(p.transform.pos);
    let cell = world.spawn((cell, Transform::IDENTITY, ChildOf(root))).id();
    let body = world.spawn(Transform::IDENTITY).id();
    let key = OutfitCacheKey { skeleton: "meshes/base_anim.nif".into(), outfit: OutfitKey(0), lod: 0, shape: 0, bodies: SmoothBodies::default(), roles: false };
    let outfit = CharacterBody { body, skeleton: key.skeleton.clone(), key, outfit: Arc::default(), meshes: Vec::new(), cloth_joints: Vec::new() };
    let character = Character::Record { kind: mw_actors::NPC_KIND.into(), id: RecordId::new("test_npc"), state: CharacterState::default() };
    let spawned = Transform { translation: local.translation, rotation: p.transform.rotation, scale: Vec3::ONE };
    let npc = world.spawn((character, outfit, CharacterPlacement(p.id), spawned, ChildOf(cell))).id();
    app.update();
    let mover = app.world().get::<Mover>(npc).expect("attach gave it a mover");
    assert!(mover.loco.position.delta_f32(to).length() < 1e-4, "{:?}", mover.loco.position);
    assert!((mover.loco.motion.yaw - yaw).abs() < 1e-4, "{}", mover.loco.motion.yaw);
    assert_eq!(app.world().get::<ActorSpace>(npc).map(|s| s.0.clone()), Some(ws()));
}

#[test]
fn persist_actor_poses_writes_a_resident_moved_actor_into_a_copy() {
    let home = actor(at(126.0, 10.0), 0.0);
    let (mut world, _) = world_with_body(at(131.0, 10.0), 1.0);
    let hook = ActorPoseHook(Arc::new(Nothing));
    let cell_entity = world.spawn_empty().id();
    let w = ws();
    hook.activate(&mut world, &ActivateCx { worldspace: &w, cell: IVec3::ZERO, cell_entity, discard: false }, &home);
    let mut copy = PlacementOverlay::default();
    persist_actor_poses(&world, &mut copy);
    let (_, p) = copy.placed(PlacementId(ID)).expect("written into the copy");
    assert!(planar(p.transform.pos, at(131.0, 10.0)) < 1e-4);
    assert!(stored(&world).is_none(), "the live overlay untouched");
    assert_eq!(world.resource::<ResidentActors>().len(), 1, "nothing removed");
}
