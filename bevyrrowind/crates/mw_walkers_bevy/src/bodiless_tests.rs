//! `AI10c3c2a`: a bodiless tier-0 actor's kept placement follows its logical position into the
//! cell it walks into (one overlay write per crossing), and the kept map itself (kept at
//! deactivation, dropped at activation, nothing on a discard, pruned without body or state).

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};

use bevy_ecs::prelude::*;
use bevy_math::{DVec3, IVec3, Quat, Vec3};
use sf_ai::{LogicalState, RouteRef};
use sf_ai_bevy::AiLogical;
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::{WorldPos, WorldTransform};
use sf_ids::{ActorKey, GameInstant};
use sf_nav::{DoorState, TraverseTimes};
use sf_world::{
    IndexedPlacement, LinkId, Location, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, PlacementOverlay, RecordRef,
    WorldspaceId,
};
use sf_character_bevy::CharacterPlacement;
use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx, WorldOverlay};

use super::follow;
use crate::logical::WorldGraphEnv;
use crate::poses::{ActorPoseHook, ResidentActors};
use crate::walkers::ActorSpace;

const ID: u128 = 42;
/// Walk speed, m/s.
const SPEED: f32 = 2.0;
/// One test tick, game ms (time scale 1: 0.5 m at [`SPEED`]).
const TICK_MS: u64 = 250;
const PARTITION: Partition = Partition::STREAMING;

fn ws() -> WorldspaceId {
    WorldspaceId::new("test_exterior")
}

fn at(x: f64, z: f64) -> WorldPos {
    WorldPos::from_f64(DVec3::new(x, 0.0, z))
}

fn planar(a: WorldPos, b: WorldPos) -> f32 {
    a.delta_f32(b).with_y(0.0).length()
}

/// An actor placement `id` at `pos`, facing `yaw`.
fn placement(id: u128, pos: WorldPos, yaw: f32) -> Placement {
    Placement {
        id: PlacementId(id),
        base: RecordRef::new("NPC_", "test_npc"),
        model: None,
        transform: WorldTransform { pos, rotation: Quat::from_rotation_y(yaw), scale: Vec3::splat(1.1) },
        class: PlacementClass::Actor,
        flags: PlacementFlags::empty(),
    }
}

/// A state walking from `from` to `to` at [`SPEED`] since instant 0 (time scale 1).
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "test metres and ms")]
fn walking(from: WorldPos, to: WorldPos) -> LogicalState {
    let (a, b) = (Location::new(ws(), from), Location::new(ws(), to));
    let length_m = planar(from, to);
    let mut s = LogicalState::new(a.clone(), GameInstant::ZERO);
    s.route = Some(RouteRef { from: a, to: b, length_m, speed_mps: SPEED });
    s.arrives = Some(GameInstant((f64::from(length_m / SPEED) * 1000.0) as u64));
    s
}

/// Runs [`follow`] at `t` over the env without a graph (straight walks), writing into `overlay`.
fn run(away: &mut HashMap<PlacementId, (WorldspaceId, Placement)>, store: &AiLogical, bodies: &[ActorKey], t: GameInstant, overlay: &mut PlacementOverlay) -> usize {
    let door = |_: LinkId| DoorState::Closed;
    let none_speed = |_: ActorKey| None;
    let nowhere = |_: PlacementId| None;
    let no_body = |_: ActorKey| None;
    let env = WorldGraphEnv::new(None, &door, TraverseTimes::default(), &none_speed, &nowhere, &no_body);
    follow(
        away,
        store,
        |k| bodies.contains(&k),
        |k| store.position_at(k, t, &env, 1.0),
        |w| (*w == ws()).then_some(PARTITION),
        |p, to, rotation| overlay.move_to(p, to, rotation),
    )
}

fn kept_at(pos: WorldPos, yaw: f32) -> HashMap<PlacementId, (WorldspaceId, Placement)> {
    HashMap::from([(PlacementId(ID), (ws(), placement(ID, pos, yaw)))])
}

/// Walks from x = 126 (cell A) to x = 140 (cell B): exactly one write, on the first tick whose
/// position stands in B, at the logical position with the kept rotation; none after.
#[test]
#[expect(clippy::cast_precision_loss, reason = "test ms")]
fn a_bodiless_walker_is_written_once_on_the_tick_it_enters_the_next_cell() {
    let (from, to) = (at(126.0, 10.0), at(140.0, 10.0));
    let (a, b) = (PARTITION.cell_of(from), PARTITION.cell_of(to));
    assert_ne!(a, b, "the walk crosses a cell border");
    let mut store = AiLogical::default();
    store.insert(ActorKey(ID), walking(from, to));
    let mut away = kept_at(from, 0.7);
    let mut overlay = PlacementOverlay::default();
    let mut write_ticks = Vec::new();
    let mut first_in_b = None;
    for i in 0..40u64 {
        let t = GameInstant(i * TICK_MS);
        let x = (126.0 + f64::from(SPEED) * (i * TICK_MS) as f64 / 1000.0).min(140.0);
        if first_in_b.is_none() && PARTITION.cell_of(at(x, 10.0)) == b {
            first_in_b = Some(i);
        }
        if run(&mut away, &store, &[], t, &mut overlay) > 0 {
            write_ticks.push(i);
        }
    }
    assert_eq!(write_ticks, vec![first_in_b.expect("it reaches B")], "one write, on the tick it enters B");
    let (w, p) = overlay.placed(PlacementId(ID)).expect("an overlay entry");
    assert_eq!(*w, ws());
    assert_eq!(PARTITION.cell_of(p.transform.pos), b, "listed in B");
    let crossing_x = 126.0 + f64::from(SPEED) * (first_in_b.unwrap() * TICK_MS) as f64 / 1000.0;
    assert!(planar(p.transform.pos, at(crossing_x, 10.0)) < 1e-3, "at the logical position: {:?}", p.transform.pos);
    assert_eq!(p.transform.rotation, Quat::from_rotation_y(0.7), "rotation kept");
    assert_eq!(p.transform.scale, Vec3::splat(1.1), "scale kept");
    let (kw, kp) = &away[&PlacementId(ID)];
    assert_eq!((kw.clone(), PARTITION.cell_of(kp.transform.pos)), (ws(), b), "the kept copy moved with it");
}

/// Nothing is written for a key with a body, a stored key with no kept placement, or a state
/// without a route.
#[test]
fn bodies_unkept_keys_and_unrouted_states_are_not_written() {
    let (from, to) = (at(126.0, 10.0), at(140.0, 10.0));
    let late = GameInstant(7_000);
    let mut overlay = PlacementOverlay::default();

    let mut store = AiLogical::default();
    store.insert(ActorKey(ID), walking(from, to));
    let mut away = kept_at(from, 0.0);
    assert_eq!(run(&mut away, &store, &[ActorKey(ID)], late, &mut overlay), 0, "a body is AI10c3a/b2's");
    assert!(away.contains_key(&PlacementId(ID)), "a body's entry is kept");

    let mut none = HashMap::new();
    assert_eq!(run(&mut none, &store, &[], late, &mut overlay), 0, "no kept placement, nothing to write");

    let mut standing = AiLogical::default();
    standing.insert(ActorKey(ID), LogicalState::new(Location::new(ws(), to), GameInstant::ZERO));
    let mut away = kept_at(from, 0.0);
    assert_eq!(run(&mut away, &standing, &[], late, &mut overlay), 0, "no route, no move");
    assert!(away.contains_key(&PlacementId(ID)), "a stored state's entry is kept");
    assert!(overlay.placed(PlacementId(ID)).is_none(), "the overlay untouched");
}

/// Placement spawner that spawns nothing (the hook's inner one).
struct Nothing;

impl PlacementSpawner for Nothing {
    fn spawn(&self, _world: &mut World, _cx: &SpawnCx<'_>, _placement: &IndexedPlacement) -> u32 {
        0
    }
}

/// The overlay, the resident index and a body for `ID` at `pos` facing `yaw`.
fn world_with_body(pos: WorldPos, yaw: f32) -> World {
    let mut world = World::new();
    world.init_resource::<WorldOverlay>();
    world.init_resource::<ResidentActors>();
    world.spawn((CharacterPlacement(PlacementId(ID)), Mover::new(pos, yaw, MoverParams::default(), WorldPos::ORIGIN), ActorSpace(ws())));
    world
}

fn activate(world: &mut World, p: &IndexedPlacement, discard: bool, deactivate: bool) {
    let hook = ActorPoseHook(Arc::new(Nothing));
    let cell_entity = world.spawn_empty().id();
    let w = ws();
    let cx = ActivateCx { worldspace: &w, cell: IVec3::ZERO, cell_entity, discard };
    if deactivate {
        hook.deactivate(world, &cx, p);
    } else {
        hook.activate(world, &cx, p);
    }
}

fn kept(world: &World) -> Option<(WorldspaceId, Placement)> {
    world.resource::<ResidentActors>().kept_placement(PlacementId(ID)).cloned()
}

#[test]
fn deactivation_keeps_the_placement_activation_drops_it_a_discard_keeps_nothing() {
    let home = IndexedPlacement { placement: placement(ID, at(126.0, 10.0), 0.0), radius: 0.5 };

    // Moved: kept as the overlay now holds it.
    let mut world = world_with_body(at(131.0, 10.0), 1.0);
    activate(&mut world, &home, false, false);
    assert!(kept(&world).is_none(), "nothing kept while Active");
    activate(&mut world, &home, false, true);
    let (w, p) = kept(&world).expect("kept at deactivation");
    let overlay = world.resource::<WorldOverlay>().0.read().unwrap_or_else(PoisonError::into_inner).placed(PlacementId(ID)).map(|(w, p)| (w.clone(), p.clone()));
    assert_eq!(Some((w, p.clone())), overlay, "the moved pose, as the overlay holds it");
    assert!(planar(p.transform.pos, at(131.0, 10.0)) < 1e-4);
    activate(&mut world, &home, false, false);
    assert!(kept(&world).is_none(), "activation drops it");

    // Unmoved: kept as listed.
    let mut world = world_with_body(at(126.0, 10.0), 0.0);
    activate(&mut world, &home, false, false);
    activate(&mut world, &home, false, true);
    assert_eq!(kept(&world), Some((ws(), home.placement.clone())), "the listed placement");

    // Discard: nothing kept, and an earlier entry is dropped.
    activate(&mut world, &home, true, true);
    assert!(kept(&world).is_none(), "a discard keeps nothing");
}

#[test]
fn an_entry_with_neither_body_nor_state_is_pruned_on_the_next_pass() {
    let mut overlay = PlacementOverlay::default();
    let store = AiLogical::default();
    let mut away = kept_at(at(126.0, 10.0), 0.0);
    run(&mut away, &store, &[ActorKey(ID)], GameInstant::ZERO, &mut overlay);
    assert!(away.contains_key(&PlacementId(ID)), "a body keeps it");
    run(&mut away, &store, &[], GameInstant::ZERO, &mut overlay);
    assert!(away.is_empty(), "no body, no state: pruned");
    assert!(overlay.placed(PlacementId(ID)).is_none());
}
