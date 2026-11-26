//! `MA2d1`: record AI packages seed each new actor's directive stack (`directives.rs`); `MA2d3`:
//! the player, `AI_A` objects and targets that spawn later resolve.

use std::sync::Arc;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use mw_game_core::player::HOST_KEY;
use mw_world::ids::exterior;
use sf_ai::{DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, GameTime, Limit};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_character::CharacterState;
use sf_character_bevy::{Character, CharacterPlacement};
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_records::{RecordDb, RecordId};
use sf_world::{IndexedPlacement, Location, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};
use sf_world_bevy::{ActivateCx, PlacementSpawner, SpawnCx};
use tes3::esp::{
    AiActivatePackage, AiFollowPackage, AiPackage, AiTravelPackage, AiWanderPackage, Creature, FixedString, Npc,
};

use super::{
    AiPackageExtras, PendingPackages, PlacedObjects, PlacedObjectsHook, Renewing, retry_pending, seed_directives,
};
use crate::walkers::{ActorSpace, WalkerContent};

const WANDERER: &str = "test_wanderer";
const GUARD: &str = "test_guard";
const CRAB: &str = "test_crab";
/// Follows `LEADER` between a wander and a travel (`MA2d3`: the leader spawns later).
const TRAILER: &str = "test_trailer";
const LEADER: &str = "test_leader";
/// Activates `LEVER`.
const PULLER: &str = "test_puller";
const LEVER: &str = "test_lever";

fn wander() -> AiPackage {
    AiPackage::Wander(AiWanderPackage { distance: 512, duration: 5, game_hour: 6, idle2: 60, reset: 1, ..AiWanderPackage::default() })
}

fn travel() -> AiPackage {
    AiPackage::Travel(AiTravelPackage { location: [1000.0, 2000.0, 0.0], reset: 0 })
}

fn follow(target: &str) -> AiPackage {
    AiPackage::Follow(AiFollowPackage {
        location: [f32::MAX; 3],
        duration: 0,
        target: FixedString(target.to_owned()),
        reset: 0,
        cell: String::new(),
    })
}

fn db() -> Arc<RecordDb> {
    let mut db = RecordDb::new();
    db.register::<Npc>(mw_actors::NPC_KIND).expect("npc kind");
    db.register::<Creature>(mw_actors::CREATURE_KIND).expect("crea kind");
    let src = db.add_source("test", &[]);
    let npc = |id: &str, ai_packages| Npc { id: id.into(), name: id.into(), ai_packages, ..Npc::default() };
    db.define(src, RecordId::new(WANDERER), npc(WANDERER, vec![wander(), travel()])).expect("wanderer");
    // A follower of a spawned NPC, then of the player.
    db.define(src, RecordId::new(GUARD), npc(GUARD, vec![follow(WANDERER), follow("Player")])).expect("guard");
    db.define(src, RecordId::new(TRAILER), npc(TRAILER, vec![wander(), follow(LEADER), travel()])).expect("trailer");
    db.define(src, RecordId::new(LEADER), npc(LEADER, Vec::new())).expect("leader");
    let activate = AiPackage::Activate(AiActivatePackage { target: FixedString(LEVER.to_owned()), reset: 0 });
    db.define(src, RecordId::new(PULLER), npc(PULLER, vec![activate])).expect("puller");
    let crab = Creature { id: CRAB.into(), name: CRAB.into(), ..Creature::default() };
    db.define(src, RecordId::new(CRAB), crab).expect("crab");
    Arc::new(db)
}

fn app() -> App {
    let mut app = App::new();
    let db = db();
    app.insert_resource(PlacedObjects::of_content(&db))
        .insert_resource(WalkerContent { db, min_walk: 1.0, max_walk: 2.0, idle_mult: 1.0 })
        .init_resource::<AiDirectives>()
        .init_resource::<PendingPackages>()
        .add_systems(Update, (seed_directives, retry_pending).chain());
    app
}

fn spawn(app: &mut App, kind: &str, id: &str, placement: u128, at: WorldPos) -> Entity {
    let character = Character::Record { kind: kind.into(), id: RecordId::new(id), state: CharacterState::default() };
    app.world_mut()
        .spawn((
            character,
            CharacterPlacement(PlacementId(placement)),
            AiActor(ActorKey(placement)),
            ActorSpace(exterior()),
            Mover::new(at, 0.0, MoverParams::default(), WorldPos::ORIGIN),
        ))
        .id()
}

fn stack(app: &App, key: u128) -> &DirectiveStack {
    app.world().resource::<AiDirectives>().get(ActorKey(key)).expect("a stack")
}

/// `AI_W` then `AI_T`: two record directives in record order (top = the wander, `OpenMW`'s
/// `AiSequence::fill`), the wander home at the spawn feet, its idle table on the actor.
#[test]
fn record_packages_seed_the_stack_in_record_order() {
    let mut app = app();
    let at = mw_world::convert::world_pos([100.0, 200.0, 300.0]);
    let e = spawn(&mut app, mw_actors::NPC_KIND, WANDERER, 11, at);
    app.update();
    let s = stack(&app, 11);
    assert_eq!(s.len(), 2);
    let kinds: Vec<_> = s.iter().map(|d| d.spec.kind.name()).collect();
    assert_eq!(kinds, ["stay_near", "go_to"]);
    let top = s.top().expect("top");
    assert!(top.spec.source == DirectiveSource::Record && top.spec.repeat);
    let DirectiveKind::StayNear { home, .. } = &top.spec.kind else { panic!("top is the wander") };
    assert_eq!(home, &Location::new(exterior(), at));
    assert_eq!(top.spec.origin, Some(RecordId::new(WANDERER)));
    let extras = app.world().get::<AiPackageExtras>(e).expect("extras");
    assert_eq!(extras.0.len(), 1, "only the wander carries extra data");
    assert!(matches!(extras.get(top.id), Some(mw_ai::packages::PackageExtra::Wander(w)) if w.idle_chances[0] == 60));
}

/// A stack already in the store (a loaded save) is kept as it is, and its record directive
/// still finds its extras.
#[test]
fn a_stored_stack_wins_over_the_record() {
    let mut app = app();
    let now = GameTime { day: 3, minute: 600 };
    let mut saved = DirectiveStack::new(ActorKey(11), now);
    let home = Location::new(exterior(), WorldPos::ORIGIN);
    let spec = DirectiveSpec::new(DirectiveKind::StayNear { home, radius: 3.0 }, DirectiveSource::Record);
    let id = saved.push(spec, now, &mw_ai::MorrowindPolicy, &mut Vec::new());
    app.world_mut().resource_mut::<AiDirectives>().insert(saved.clone());
    let e = spawn(&mut app, mw_actors::NPC_KIND, WANDERER, 11, WorldPos::ORIGIN);
    app.update();
    assert_eq!(stack(&app, 11), &saved, "the saved stack is untouched");
    let extras = app.world().get::<AiPackageExtras>(e).expect("extras");
    assert!(matches!(extras.get(id), Some(mw_ai::packages::PackageExtra::Wander(_))));
}

/// A creature without packages gets an empty stack (the key means "seeded"); a follow target
/// resolves to the spawned NPC of that record, `Player` (any case) to the host's character.
#[test]
fn empty_records_get_an_empty_stack_and_targets_resolve_to_spawned_actors() {
    let mut app = app();
    spawn(&mut app, mw_actors::CREATURE_KIND, CRAB, 21, WorldPos::ORIGIN);
    spawn(&mut app, mw_actors::NPC_KIND, WANDERER, 11, WorldPos::ORIGIN);
    let guard = spawn(&mut app, mw_actors::NPC_KIND, GUARD, 31, WorldPos::ORIGIN);
    app.update();
    assert!(stack(&app, 21).is_empty());
    let kinds: Vec<_> = stack(&app, 31).iter().map(|d| d.spec.kind.clone()).collect();
    assert_eq!(kinds, [DirectiveKind::Follow(ActorKey(11)), DirectiveKind::Follow(ActorKey::player(HOST_KEY))]);
    assert_eq!(ActorKey::player(HOST_KEY).as_player(), Some(HOST_KEY));
    assert!(app.world().get::<AiPackageExtras>(guard).expect("extras").0.is_empty());
    assert!(app.world().resource::<PendingPackages>().is_empty(), "nothing waits");
}

/// `MA2d3`: a follow whose leader isn't spawned waits (not dropped); when the leader spawns it
/// goes on the stack at its record position, between the wander and the travel.
#[test]
fn a_follow_target_that_spawns_later_resolves_at_its_record_position() {
    let mut app = app();
    let trailer = spawn(&mut app, mw_actors::NPC_KIND, TRAILER, 41, WorldPos::ORIGIN);
    app.update();
    let kinds: Vec<_> = stack(&app, 41).iter().map(|d| d.spec.kind.name()).collect();
    assert_eq!(kinds, ["stay_near", "go_to"], "the follow waits");
    assert_eq!(app.world().resource::<PendingPackages>().waiting(ActorKey(41)), 1);
    let wander_id = stack(&app, 41).top().map(|d| d.id).expect("the wander");
    app.update();
    assert_eq!(stack(&app, 41).len(), 2, "no spawn, no change");
    spawn(&mut app, mw_actors::NPC_KIND, LEADER, 51, WorldPos::ORIGIN);
    app.update();
    let s = stack(&app, 41);
    let kinds: Vec<_> = s.iter().map(|d| d.spec.kind.clone()).collect();
    assert_eq!(kinds.len(), 3);
    assert_eq!(kinds[1], DirectiveKind::Follow(ActorKey(51)));
    assert!(matches!(kinds[0], DirectiveKind::StayNear { .. }) && matches!(kinds[2], DirectiveKind::GoTo(_)));
    let follow = s.iter().nth(1).expect("the follow");
    assert_eq!((follow.spec.source, follow.spec.origin.clone()), (DirectiveSource::Record, Some(RecordId::new(TRAILER))));
    assert!(app.world().resource::<PendingPackages>().is_empty(), "resolved once");
    let extras = app.world().get::<AiPackageExtras>(trailer).expect("extras");
    assert!(extras.get(wander_id).is_some(), "the wander keeps its idle table");
    assert_eq!(extras.0.len(), 1, "the follow has no end point");
}

/// Placement spawner that spawns nothing (the hook's inner one).
struct Nothing;

impl PlacementSpawner for Nothing {
    fn spawn(&self, _world: &mut World, _cx: &SpawnCx<'_>, _placement: &IndexedPlacement) -> u32 {
        0
    }
}

fn lever(id: u128, flags: PlacementFlags) -> IndexedPlacement {
    let placement = Placement {
        id: PlacementId(id),
        base: RecordRef::new("ACTI", LEVER),
        model: None,
        transform: sf_coords::WorldTransform::IDENTITY,
        class: PlacementClass::Custom(2),
        flags,
    };
    IndexedPlacement { placement, radius: 0.0 }
}

/// `MA2d3`: an `AI_A` package waits until its object enters R0 ([`PlacedObjectsHook`]: enabled
/// placements only, lowest id first), then is `Use` of that placement.
#[test]
fn an_activate_package_resolves_to_its_placed_object_when_it_enters_r0() {
    let mut app = app();
    spawn(&mut app, mw_actors::NPC_KIND, PULLER, 61, WorldPos::ORIGIN);
    app.update();
    assert!(stack(&app, 61).is_empty());
    assert_eq!(app.world().resource::<PendingPackages>().waiting(ActorKey(61)), 1);
    let hook = PlacedObjectsHook(Arc::new(Nothing));
    let ws = exterior();
    let cell_entity = app.world_mut().spawn_empty().id();
    let cx = ActivateCx { worldspace: &ws, cell: IVec3::ZERO, cell_entity, discard: false };
    hook.activate(app.world_mut(), &cx, &lever(90, PlacementFlags::DISABLED));
    hook.activate(app.world_mut(), &cx, &lever(80, PlacementFlags::empty()));
    hook.activate(app.world_mut(), &cx, &lever(70, PlacementFlags::empty()));
    assert_eq!(app.world().resource::<PlacedObjects>().first(&RecordId::new(LEVER)), Some(PlacementId(70)));
    app.update();
    let s = stack(&app, 61);
    assert_eq!(s.top().map(|d| &d.spec.kind), Some(&DirectiveKind::Use(PlacementId(70))));
    assert!(app.world().resource::<PendingPackages>().is_empty());
    hook.deactivate(app.world_mut(), &cx, &lever(70, PlacementFlags::empty()));
    assert_eq!(app.world().resource::<PlacedObjects>().first(&RecordId::new(LEVER)), Some(PlacementId(80)));
}

/// `MA2d4`: [`Renewing`] notes each repeat's new id (one that is not a repeat, none) and the
/// extras follow it.
#[test]
fn renewing_notes_each_repeat_and_the_extras_follow() {
    let t0 = GameTime { day: 1, minute: 0 };
    let mut s = DirectiveStack::new(ActorKey(1), t0);
    let home = Location::new(exterior(), WorldPos::ORIGIN);
    let kind = DirectiveKind::StayNear { home, radius: 3.0 };
    let policy = Renewing::default();
    let once = DirectiveSpec::new(kind.clone(), DirectiveSource::Record).limit(Limit::For { minutes: 10 });
    let first = s.push(once, t0, &policy, &mut Vec::new());
    let again = DirectiveSpec::new(kind, DirectiveSource::Record).repeating().limit(Limit::For { minutes: 30 });
    let second = s.push(again, t0, &policy, &mut Vec::new());
    let wander = mw_ai::packages::PackageExtra::Wander(mw_ai::Wander {
        distance: 3.0,
        duration_hours: 0.5,
        game_hour: 0,
        idle_chances: [5; 8],
        repeat: true,
    });
    let mut extras = AiPackageExtras(vec![(first, wander.clone()), (second, wander.clone())]);
    s.advance(GameTime { day: 1, minute: 50 }, &policy, &mut Vec::new());
    let renewed = policy.take();
    assert_eq!(renewed.len(), 1, "only the repeating one came back: {renewed:?}");
    let (old, new) = renewed[0];
    assert_eq!(old, second);
    assert_eq!(s.top().map(|d| d.id), Some(new));
    assert!(policy.take().is_empty(), "taken once");
    extras.follow(&renewed);
    assert_eq!(extras.get(new), Some(&wander));
    assert_eq!(extras.get(second), None);
}
