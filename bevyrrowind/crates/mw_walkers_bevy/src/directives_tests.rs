//! `MA2d1`: record AI packages seed each new actor's directive stack (`directives.rs`).

use std::sync::Arc;

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use mw_world::ids::exterior;
use sf_ai::{DirectiveKind, DirectiveSource, DirectiveSpec, DirectiveStack, GameTime};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_character::CharacterState;
use sf_character_bevy::{Character, CharacterPlacement};
use sf_controller_bevy::{Mover, MoverParams};
use sf_coords::WorldPos;
use sf_ids::ActorKey;
use sf_records::{RecordDb, RecordId};
use sf_world::{Location, PlacementId};
use tes3::esp::{AiFollowPackage, AiPackage, AiTravelPackage, AiWanderPackage, Creature, FixedString, Npc};

use super::{AiPackageExtras, seed_directives};
use crate::walkers::{ActorSpace, WalkerContent};

const WANDERER: &str = "test_wanderer";
const GUARD: &str = "test_guard";
const CRAB: &str = "test_crab";

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
    // A follower of a spawned NPC, then of the player (no player actor key yet: skipped).
    db.define(src, RecordId::new(GUARD), npc(GUARD, vec![follow(WANDERER), follow("player")])).expect("guard");
    let crab = Creature { id: CRAB.into(), name: CRAB.into(), ..Creature::default() };
    db.define(src, RecordId::new(CRAB), crab).expect("crab");
    Arc::new(db)
}

fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WalkerContent { db: db(), min_walk: 1.0, max_walk: 2.0, idle_mult: 1.0 })
        .init_resource::<AiDirectives>()
        .add_systems(Update, seed_directives);
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
/// resolves to the spawned NPC of that record, the player's is skipped.
#[test]
fn empty_records_get_an_empty_stack_and_targets_resolve_to_spawned_actors() {
    let mut app = app();
    spawn(&mut app, mw_actors::CREATURE_KIND, CRAB, 21, WorldPos::ORIGIN);
    spawn(&mut app, mw_actors::NPC_KIND, WANDERER, 11, WorldPos::ORIGIN);
    let guard = spawn(&mut app, mw_actors::NPC_KIND, GUARD, 31, WorldPos::ORIGIN);
    app.update();
    assert!(stack(&app, 21).is_empty());
    let s = stack(&app, 31);
    assert_eq!(s.len(), 1, "the player follow is skipped");
    assert_eq!(s.top().map(|d| &d.spec.kind), Some(&DirectiveKind::Follow(ActorKey(11))));
    assert!(app.world().get::<AiPackageExtras>(guard).expect("extras").0.is_empty());
}
