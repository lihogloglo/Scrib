//! `AI10c3b2`: a body that crosses into another loaded cell becomes its, on a hand-built hierarchy
//! (worldspace root -> two `StreamCell` cells -> near group -> batch -> body) with a stand-in for
//! `Streaming::rehome` (`AI10c3b1` tests the real bookkeeping in `sf_world_bevy`).

use std::sync::{Arc, PoisonError};

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use bevy_math::{DVec3, IVec3, Quat, Vec3};
use bevy_transform::components::Transform;
use sf_character::{CharacterState, OutfitKey};
use sf_character_bevy::{Character, CharacterBody, CharacterPlacement, OutfitCacheKey, SmoothBodies};
use sf_controller_bevy::{Held, Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{RecordDb, RecordId};
use sf_world::{IndexedPlacement, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, PlacementSpawner, Rehomed, SpawnCx, StreamCell, WorldOverlay, WorldPosOf, WorldspaceRoot};

use super::{BodyCell, CrossingStats, Crossers, crossings, proof_line, rehome_found};
use crate::poses::{ActorPoseHook, ResidentActors};
use crate::walkers::{ActorSpace, WalkerContent};

const ID: PlacementId = PlacementId(42);
const PARTITION: Partition = Partition::STREAMING;

/// Placement spawner that spawns nothing (the pose hook's inner one).
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

/// Home at x = 126 (streaming cell A); x = 131 is cell B.
fn home() -> WorldPos {
    at(126.0, 10.0)
}

fn over() -> WorldPos {
    at(131.0, 10.0)
}

fn actor(pos: WorldPos) -> IndexedPlacement {
    let placement = Placement {
        id: ID,
        base: RecordRef::new("NPC_", "test_npc"),
        model: None,
        transform: WorldTransform { pos, rotation: Quat::IDENTITY, scale: Vec3::ONE },
        class: PlacementClass::Actor,
        flags: PlacementFlags::empty(),
    };
    IndexedPlacement { placement, radius: 0.5 }
}

/// The stand-in for `Streaming::rehome`: what it answers, and every question asked.
#[derive(Resource, Default)]
struct Host {
    answer: Option<Rehomed>,
    asked: Vec<(PlacementId, IVec3, IVec3)>,
}

/// `rehome_crossers` with [`Host`] in place of the streaming runtime.
fn rehome_with_host(mut cx: Crossers, mut host: ResMut<Host>) {
    let found = crossings(&cx, |_| Some(PARTITION));
    rehome_found(&mut cx, found, |_, id, from, to| {
        host.asked.push((id, from, to));
        host.answer
    });
}

/// A streaming cell entity under `root` and its near group.
fn cell(world: &mut World, root: Entity, cell: IVec3) -> Entity {
    let (coord, transform) = sf_world_bevy::to_grid(PARTITION.cell_origin(cell));
    let c = world.spawn((StreamCell { worldspace: ws(), cell }, coord, transform, ChildOf(root))).id();
    world.spawn((Transform::IDENTITY, ChildOf(c))).id()
}

struct Scene {
    app: App,
    root: Entity,
    body: Entity,
    batch_a: Entity,
    group_b: Entity,
    a: IVec3,
    b: IVec3,
}

/// Two loaded cells, a body of placement `ID` under A's batch at its home; `resident`: A's Active
/// list holds it. The host accepts (`listed`) when `accept`.
fn scene(resident: bool, accept: bool, listed: bool) -> Scene {
    let (a, b) = (PARTITION.cell_of(home()), PARTITION.cell_of(over()));
    assert_ne!(a, b, "the 5 m cross a cell border");
    let mut app = App::new();
    app.init_resource::<WorldOverlay>()
        .init_resource::<ResidentActors>()
        .init_resource::<CrossingStats>()
        .init_resource::<Host>()
        .add_systems(Update, rehome_with_host);
    let world = app.world_mut();
    let root = world.spawn((WorldspaceRoot { id: ws(), layer: 0 }, Transform::IDENTITY)).id();
    let group_a = cell(world, root, a);
    let batch_a = world.spawn((Transform::IDENTITY, ChildOf(group_a))).id();
    let group_b = cell(world, root, b);
    let origin_a = PARTITION.cell_origin(a);
    let body = world
        .spawn((
            CharacterPlacement(ID),
            Mover::new(home(), 0.0, MoverParams::default(), origin_a),
            Transform::from_translation(origin_a.delta_f32(home())),
            ActorSpace(ws()),
            BodyCell { worldspace: ws(), cell: a },
            ChildOf(batch_a),
        ))
        .id();
    if resident {
        let cell_entity = world.spawn_empty().id();
        let w = ws();
        ActorPoseHook(Arc::new(Nothing)).activate(world, &ActivateCx { worldspace: &w, cell: a, cell_entity, discard: false }, &actor(home()));
    }
    let origin = PARTITION.cell_origin(b);
    world.resource_mut::<Host>().answer = accept.then_some(Rehomed { parent: group_b, origin, listed });
    Scene { app, root, body, batch_a, group_b, a, b }
}

/// The body's feet go to `pos`, drawn there as `interpolate` writes it (from its frame origin).
fn walk_to(s: &mut Scene, pos: WorldPos) {
    let mut e = s.app.world_mut().entity_mut(s.body);
    let origin = {
        let mut m = e.get_mut::<Mover>().unwrap();
        m.loco.position = pos;
        m.frame_origin
    };
    e.get_mut::<Transform>().unwrap().translation = origin.delta_f32(pos);
}

fn drawn(s: &mut Scene) -> WorldPos {
    let body = s.body;
    s.app.world_mut().run_system_once(move |p: WorldPosOf| p.get(body)).unwrap().expect("a world position")
}

fn parent(s: &Scene) -> Entity {
    s.app.world().get::<ChildOf>(s.body).unwrap().parent()
}

fn stored(s: &Scene) -> Option<Placement> {
    let overlay = s.app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    overlay.placed(ID).map(|(_, p)| p.clone())
}

fn asked(s: &Scene) -> usize {
    s.app.world().resource::<Host>().asked.len()
}

fn rehomed(s: &Scene) -> u32 {
    s.app.world().resource::<CrossingStats>().rehomed
}

#[test]
fn a_body_that_crossed_becomes_the_new_cells_without_a_jump() {
    let mut s = scene(true, true, true);
    s.app.update();
    assert_eq!(asked(&s), 0, "at home: nothing asked");
    walk_to(&mut s, over());
    let before = drawn(&mut s);
    s.app.update();
    assert_eq!(s.app.world().resource::<Host>().asked, vec![(ID, s.a, s.b)]);
    assert_eq!(parent(&s), s.group_b, "a direct child of B's near group");
    let origin_b = PARTITION.cell_origin(s.b);
    let world = s.app.world();
    let mover = world.get::<Mover>(s.body).unwrap();
    assert_eq!(mover.frame_origin, origin_b);
    let local = world.get::<Transform>(s.body).unwrap().translation;
    assert!(local.distance(origin_b.delta_f32(over())) < 1e-4, "{local}");
    assert_eq!(world.get::<BodyCell>(s.body), Some(&BodyCell { worldspace: ws(), cell: s.b }));
    let after = drawn(&mut s);
    assert!(before.delta_f32(after).length() < 1e-4, "no drawn jump: {before:?} -> {after:?}");
    let p = stored(&s).expect("the crossing wrote the overlay");
    assert!(p.transform.pos.delta_f32(over()).length() < 1e-4);
    // Neither cell's next fetch lists it twice: A's reload skips it, B lists it once.
    let overlay = s.app.world().resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner);
    assert!(overlay.apply(&ws(), PARTITION, s.a, &[actor(home())]).is_empty(), "A's reload spawns no twin");
    let in_b = overlay.apply(&ws(), PARTITION, s.b, &[]);
    assert_eq!(in_b.iter().map(|p| p.placement.id).collect::<Vec<_>>(), vec![ID]);
    drop(overlay);
    assert_eq!(s.app.world().resource::<ResidentActors>().len(), 1, "listed in B: still resident");
    assert_eq!(rehomed(&s), 1);
    s.app.update();
    assert_eq!(asked(&s), 1, "home again: asked once");
}

#[test]
fn a_refusal_changes_nothing_and_is_asked_again_next_frame() {
    let mut s = scene(true, false, true);
    walk_to(&mut s, over());
    let translation = s.app.world().get::<Transform>(s.body).unwrap().translation;
    s.app.update();
    let world = s.app.world();
    assert_eq!(parent(&s), s.batch_a);
    assert_eq!(world.get::<Transform>(s.body).unwrap().translation, translation);
    assert_eq!(world.get::<Mover>(s.body).unwrap().frame_origin, PARTITION.cell_origin(s.a));
    assert_eq!(world.get::<BodyCell>(s.body).map(|c| c.cell), Some(s.a));
    assert!(stored(&s).is_none(), "no overlay write on a refusal");
    assert_eq!(rehomed(&s), 0);
    s.app.update();
    assert_eq!(asked(&s), 2, "the comparison asks again");
    let (group_b, origin) = (s.group_b, PARTITION.cell_origin(s.b));
    s.app.world_mut().resource_mut::<Host>().answer = Some(Rehomed { parent: group_b, origin, listed: true });
    s.app.update();
    assert_eq!(parent(&s), s.group_b, "accepted on a later frame");
}

/// A tier-0 promotion (`AI10c2b`) places the body two cells on, as `Mover::snap` leaves it.
#[test]
fn a_promotion_into_another_cell_is_rehomed_the_same_way() {
    let mut s = scene(true, true, true);
    let far = at(126.0 + 2.0 * 128.0, 10.0);
    let c = PARTITION.cell_of(far);
    assert_ne!(c, s.b);
    let origin = PARTITION.cell_origin(c);
    let root = s.root;
    let group_c = cell(s.app.world_mut(), root, c);
    s.app.world_mut().resource_mut::<Host>().answer = Some(Rehomed { parent: group_c, origin, listed: true });
    walk_to(&mut s, far);
    let before = drawn(&mut s);
    s.app.update();
    assert_eq!(s.app.world().resource::<Host>().asked, vec![(ID, s.a, c)]);
    assert_eq!(parent(&s), group_c);
    assert_eq!(s.app.world().get::<BodyCell>(s.body).map(|b| b.cell), Some(c));
    assert_eq!(s.app.world().get::<Mover>(s.body).unwrap().frame_origin, origin);
    let after = drawn(&mut s);
    assert!(before.delta_f32(after).length() < 1e-4, "{before:?} -> {after:?}");
}

#[test]
fn a_body_with_no_held_placement_stays_and_is_counted() {
    let mut s = scene(false, true, true);
    walk_to(&mut s, over());
    s.app.update();
    assert_eq!(asked(&s), 0, "never asked: nothing could be written for it");
    assert_eq!(parent(&s), s.batch_a);
    assert!(stored(&s).is_none());
    let stats = s.app.world().resource::<CrossingStats>();
    assert_eq!((stats.rehomed, stats.unplaced), (0, 1));
}

/// A body spawned from a moved placement by a never-Active cell: the overlay holds its placement.
#[test]
fn an_overlay_placement_is_enough_and_a_near_only_cell_drops_the_resident_entry() {
    let mut s = scene(false, true, false);
    {
        let overlay = s.app.world().resource::<WorldOverlay>();
        overlay.0.write().unwrap_or_else(PoisonError::into_inner).move_to(&actor(home()).placement, &sf_world::Location::new(ws(), home()), Quat::IDENTITY);
    }
    walk_to(&mut s, over());
    s.app.update();
    assert_eq!(parent(&s), s.group_b);
    assert!(stored(&s).unwrap().transform.pos.delta_f32(over()).length() < 1e-4);
    // Resident in A, re-homed into a Near-only B (`listed` false): it leaves `ResidentActors`.
    let mut s = scene(true, true, false);
    walk_to(&mut s, over());
    s.app.update();
    assert_eq!(parent(&s), s.group_b);
    assert!(s.app.world().resource::<ResidentActors>().is_empty(), "no Active list holds it");
}

#[test]
fn a_held_body_is_left_to_its_owner() {
    let mut s = scene(true, true, true);
    walk_to(&mut s, over());
    s.app.world_mut().entity_mut(s.body).insert(Held);
    s.app.update();
    assert_eq!(asked(&s), 0);
    assert_eq!(parent(&s), s.batch_a);
    s.app.world_mut().entity_mut(s.body).remove::<Held>();
    s.app.update();
    assert_eq!(parent(&s), s.group_b, "re-homed once let go");
}

#[test]
fn the_proof_line() {
    assert_eq!(proof_line(3, 0), "actors: 3 re-homed");
    assert_eq!(proof_line(0, 2), "actors: 0 re-homed, 2 without a held placement");
}

/// `attach` gives a streamed body the cell it hangs under (body -> batch -> near group -> cell).
#[test]
fn attach_gives_a_body_its_ancestor_cell() {
    let mut app = App::new();
    let mut db = RecordDb::new();
    db.register::<tes3::esp::Npc>(mw_actors::NPC_KIND).expect("npc kind");
    db.register::<tes3::esp::Creature>(mw_actors::CREATURE_KIND).expect("crea kind");
    app.insert_resource(WalkerContent { db: Arc::new(db), min_walk: 1.0, max_walk: 2.0, idle_mult: 1.0 })
        .add_systems(Update, crate::walkers::attach);
    let world = app.world_mut();
    let root = world.spawn((WorldspaceRoot { id: ws(), layer: 0 }, Transform::IDENTITY)).id();
    let a = PARTITION.cell_of(home());
    let group = cell(world, root, a);
    let batch = world.spawn((Transform::IDENTITY, ChildOf(group))).id();
    let skeleton = world.spawn(Transform::IDENTITY).id();
    let key = OutfitCacheKey { skeleton: "meshes/base_anim.nif".into(), outfit: OutfitKey(0), lod: 0, shape: 0, bodies: SmoothBodies::default(), roles: false };
    let outfit = CharacterBody { body: skeleton, skeleton: key.skeleton.clone(), key, outfit: Arc::default(), meshes: Vec::new(), cloth_joints: Vec::new() };
    let character = Character::Record { kind: mw_actors::NPC_KIND.into(), id: RecordId::new("test_npc"), state: CharacterState::default() };
    let local = Transform::from_translation(PARTITION.cell_origin(a).delta_f32(home()));
    let npc = world.spawn((character, outfit, CharacterPlacement(ID), local, ChildOf(batch))).id();
    app.update();
    assert_eq!(app.world().get::<BodyCell>(npc), Some(&BodyCell { worldspace: ws(), cell: a }));
    let mover = app.world().get::<Mover>(npc).expect("a mover");
    assert!(mover.loco.position.delta_f32(home()).length() < 1e-4, "{:?}", mover.loco.position);
}
