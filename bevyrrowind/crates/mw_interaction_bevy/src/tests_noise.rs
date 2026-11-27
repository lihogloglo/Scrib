//! The host's noise report (`IN18c`, `noise` module docs): an interaction noise becomes one
//! perception noise, field for field, its kind from the table and its maker as the source.

use bevy_ecs::message::Messages;
use glam::{DVec3, Vec3};
use mw_game_core::player::HOST_KEY;
use sf_ai::{ActorKey, NoiseKindId, PerceptTarget};
use sf_ai_bevy::{AiPerceptClock, NoiseEvent};
use sf_coords::WorldPos;
use sf_interaction::{NoiseCase, TagId, Tick, UserKey};
use sf_interaction_bevy::{InteractionNoise, NOISE_BREAK, NOISE_DOOR};

use crate::noise::{InteractionNoiseKinds, KIND_BASE, to_noise_event};
use crate::tests::{app, tick};
use crate::tests_door::ws;

fn noise(kind: TagId, maker: Option<UserKey>, reveals: bool, case: NoiseCase) -> InteractionNoise {
    InteractionNoise {
        tick: Tick(9),
        at: Vec3::new(300.5, 2.25, -40.0),
        space: Some(ws()),
        kind,
        loudness_m: 12.0,
        maker,
        reveals_maker: reveals,
        case,
    }
}

#[test]
fn an_interaction_noise_is_one_perception_noise_field_for_field() {
    let mut kinds = InteractionNoiseKinds::default();
    let door = to_noise_event(&noise(NOISE_DOOR, Some(UserKey::Player(HOST_KEY)), true, NoiseCase::Door), &kinds, 42);
    assert_eq!(
        door,
        NoiseEvent {
            tick: 42,
            at: WorldPos::from_f64(DVec3::new(300.5, 2.25, -40.0)),
            space: Some(ws()),
            kind: NoiseKindId(KIND_BASE),
            loudness_m: 12.0,
            source: Some(PerceptTarget::Player(HOST_KEY)),
            reveals_source: true,
            notify: None,
        }
    );
    assert_eq!(door.at.cell.x, 1, "300.5 m is in the next grid cell");

    // A thrown landing names its thrower without revealing it; a break with no culprit has no source.
    let thrower = ActorKey(7);
    let landed = to_noise_event(&noise(NOISE_BREAK, Some(UserKey::Actor(thrower)), false, NoiseCase::ThrownLanding), &kinds, 3);
    assert_eq!((landed.kind, landed.source, landed.reveals_source), (NoiseKindId(KIND_BASE + 1), Some(PerceptTarget::Actor(thrower)), false));
    let none = to_noise_event(&noise(NOISE_BREAK, None, false, NoiseCase::Break), &kinds, 3);
    assert_eq!(none.source, None);

    // An unknown tag is `noise.object`; a registered material tag gets the next id, once.
    let wood = TagId::of("noise.wood");
    assert_eq!(kinds.id(wood), NoiseKindId(KIND_BASE + 2));
    assert_eq!(kinds.register(wood), NoiseKindId(KIND_BASE + 3));
    assert_eq!(kinds.register(wood), NoiseKindId(KIND_BASE + 3));
    assert_eq!(kinds.register(NOISE_DOOR), NoiseKindId(KIND_BASE));
    assert_eq!(kinds.id(wood), NoiseKindId(KIND_BASE + 3));
}

#[test]
fn the_game_reports_each_tick_s_interaction_noises_stamped_with_the_perception_clock() {
    let mut app = app();
    // Without perception's messages the reporter does nothing (and never panics).
    app.world_mut().resource_mut::<Messages<InteractionNoise>>().write(noise(NOISE_DOOR, None, true, NoiseCase::Door));
    tick(&mut app, 1);
    app.world_mut().resource_mut::<Messages<InteractionNoise>>().clear();

    app.add_message::<NoiseEvent>().insert_resource(AiPerceptClock { tick: 77, tick_s: 1.0 / 60.0 });
    let written = [
        noise(NOISE_DOOR, Some(UserKey::Player(HOST_KEY)), true, NoiseCase::Door),
        noise(NOISE_BREAK, Some(UserKey::Actor(ActorKey(2))), false, NoiseCase::Break),
    ];
    app.world_mut().resource_mut::<Messages<InteractionNoise>>().write_batch(written.clone());
    tick(&mut app, 1);

    let kinds = InteractionNoiseKinds::default();
    let events = app.world().resource::<Messages<NoiseEvent>>();
    let heard: Vec<NoiseEvent> = events.get_cursor().read(events).cloned().collect();
    let expected: Vec<NoiseEvent> = written.iter().map(|n| to_noise_event(n, &kinds, 77)).collect();
    assert_eq!(heard, expected, "one perception noise per interaction noise, in order, none for the earlier tick's");

    // Read once: the next tick reports nothing new.
    tick(&mut app, 1);
    let events = app.world().resource::<Messages<NoiseEvent>>();
    assert_eq!(events.get_cursor().read(events).count(), 2);
}
