//! `swing_door_sounds` (IN33c3): one swing door toggle (`DoorChanged`) plays the door's open or
//! close cue at the door, placed as the camera plus the door's offset from the player.

use bevy_app::{App, Update};
use bevy_ecs::message::Messages;
use bevy_math::Vec3;
use bevy_transform::components::GlobalTransform;
use mw_audio::DoorSounds;
use sf_audio::FootstepMap;
use sf_audio_bevy::{PlaySound, SoundAt};
use sf_interaction_bevy::DoorChanged;
use sf_interaction_bevy::sf_interaction::{self as core, Tick};
use sf_records::RecordId;
use sf_script::Handle;
use sf_script_bevy::{Placed, ScriptPlacements};
use sf_world::PlacementId;

use crate::{MwAudio, swing_door_sounds};

/// A door with both sounds, 3 m in front of the player (-Z).
const DOOR: u128 = 7;
/// A door whose record has no sounds.
const SILENT: u128 = 8;
/// A door with an open sound and no close sound.
const HALF: u128 = 9;
const CAM: Vec3 = Vec3::new(1.0, 1.6, 0.0);

fn placed(record: &str, position: [f64; 3]) -> Placed {
    Placed { record: record.into(), worldspace: "test".into(), position, radius: 0.0 }
}

fn app(with_placements: bool) -> App {
    let mut app = App::new();
    let doors = [
        ("in_door", DoorSounds { open: Some(RecordId::new("door open")), close: Some(RecordId::new("door close")) }),
        ("half_door", DoorSounds { open: Some(RecordId::new("half open")), close: None }),
    ]
    .into_iter()
    .map(|(record, sounds)| (RecordId::new(record), sounds))
    .collect();
    app.add_message::<PlaySound>()
        .add_message::<DoorChanged>()
        .insert_resource(MwAudio { doors, footsteps: FootstepMap::default() })
        .add_systems(Update, swing_door_sounds);
    if with_placements {
        let mut placements = ScriptPlacements { player: Some(placed("player", [100.0, 5.0, -40.0])), ..Default::default() };
        placements.loaded.insert(Handle(DOOR), placed("in_door", [100.0, 5.0, -43.0]));
        placements.loaded.insert(Handle(SILENT), placed("plain_door", [100.0, 5.0, -43.0]));
        placements.loaded.insert(Handle(HALF), placed("half_door", [100.0, 5.0, -43.0]));
        app.insert_resource(placements);
    }
    app.world_mut().spawn((mw_doors_bevy::DoorUser, GlobalTransform::from_translation(CAM)));
    app
}

/// One toggle of `door` (no user: close-behind or a script), one frame; the sounds it played.
fn toggle(app: &mut App, door: u128, open: bool) -> Vec<PlaySound> {
    let change = core::DoorChanged { placement: PlacementId(door), open, user: None, tick: Tick(3) };
    app.world_mut().write_message(DoorChanged(change));
    app.update();
    app.world_mut().resource_mut::<Messages<PlaySound>>().drain().collect()
}

fn at_door(cue: &str) -> PlaySound {
    PlaySound { cue: RecordId::new(cue), at: SoundAt::Point(CAM + Vec3::new(0.0, 0.0, -3.0)) }
}

#[test]
fn open_then_close_play_the_doors_cues_at_the_door() {
    let mut app = app(true);
    assert_eq!(toggle(&mut app, DOOR, true), vec![at_door("door open")]);
    assert_eq!(toggle(&mut app, DOOR, false), vec![at_door("door close")]);
    assert_eq!(toggle(&mut app, DOOR, true), vec![at_door("door open")]);
}

#[test]
fn a_missing_cue_plays_nothing() {
    let mut app = app(true);
    assert_eq!(toggle(&mut app, SILENT, true), vec![]);
    assert_eq!(toggle(&mut app, SILENT, false), vec![]);
    assert_eq!(toggle(&mut app, HALF, true), vec![at_door("half open")]);
    assert_eq!(toggle(&mut app, HALF, false), vec![]);
    // A placement that is not loaded.
    assert_eq!(toggle(&mut app, 99, true), vec![]);
}

#[test]
fn no_script_placements_plays_nothing_and_does_not_panic() {
    let mut app = app(false);
    assert_eq!(toggle(&mut app, DOOR, true), vec![]);
    assert_eq!(toggle(&mut app, DOOR, false), vec![]);
}
