use std::time::Duration;

use bevy_app::App;
use bevy_time::{TimePlugin, TimeUpdateStrategy};
use sf_audio::SoundFile;

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn app() -> App {
    let mut bank = SoundBank::default();
    for name in ["rain", "storm", "thunder"] {
        bank.0.insert(id(name), SoundFile { path: format!("Sound/{name}.wav"), volume: 0.5 });
    }
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(500)))
        .add_plugins(SfAudioPlugin { bank, playback: false });
    app.update();
    app
}

fn send(app: &mut App, m: BusSound) {
    app.world_mut().write_message(m);
}

fn events(app: &mut App) -> Vec<AudioEvent> {
    app.world_mut().resource_mut::<Messages<AudioEvent>>().drain().collect()
}

fn lp(sound: &str, volume: f32) -> BusSound {
    BusSound::Loop { sound: id(sound), volume, bus: Bus::Weather }
}

fn level(app: &mut App, sound: &str) -> Option<f32> {
    app.world_mut().query::<&Voice>().iter(app.world()).find(|v| v.sound == id(sound)).map(|v| v.level)
}

#[test]
fn clear_rain_thunderstorm_starts_fades_and_stops_loops() {
    let mut app = app();
    // Clear: nothing plays.
    app.update();
    assert!(events(&mut app).is_empty());
    // Rain begins: starts at silence and fades in over FADE_S (500 ms frames).
    send(&mut app, lp("rain", 1.0));
    app.update();
    assert_eq!(events(&mut app), vec![AudioEvent::LoopStart { sound: id("rain"), target: 1.0 }]);
    for _ in 0..4 {
        app.update();
    }
    assert!((level(&mut app, "rain").unwrap() - 1.0).abs() < 1e-5);
    // Thunderstorm: rain fades out while the storm bed fades in (a crossfade).
    send(&mut app, lp("rain", 0.0));
    send(&mut app, lp("storm", 1.0));
    app.update();
    let ev = events(&mut app);
    assert!(ev.contains(&AudioEvent::LoopStop { sound: id("rain") }));
    assert!(ev.contains(&AudioEvent::LoopStart { sound: id("storm"), target: 1.0 }));
    app.update();
    let (r, s) = (level(&mut app, "rain").unwrap(), level(&mut app, "storm").unwrap());
    assert!(r < 1.0 && r > 0.0 && s > 0.0 && s < 1.0, "{r} {s}");
    // Thunder one-shot plays at once at its volume on the weather bus.
    send(&mut app, BusSound::OneShot { sound: id("thunder"), volume: 0.8, bus: Bus::Weather });
    app.update();
    assert!(events(&mut app).contains(&AudioEvent::OneShot { sound: id("thunder"), volume: 0.8, bus: Bus::Weather }));
    for _ in 0..6 {
        app.update();
    }
    // Rain finished fading and was dropped; the storm holds.
    assert_eq!(level(&mut app, "rain"), None);
    assert!((level(&mut app, "storm").unwrap() - 1.0).abs() < 1e-5);
}

#[test]
fn unknown_sound_is_skipped() {
    let mut app = app();
    send(&mut app, lp("nope", 1.0));
    app.update();
    assert!(events(&mut app).is_empty());
    assert_eq!(level(&mut app, "nope"), None);
}

#[test]
fn indoors_eases_the_weather_bus_out_and_back() {
    let mut app = app();
    let gain = |app: &App| app.world().resource::<AudioMixer>().0.bus_gain(Bus::Weather);
    assert!((gain(&app) - 1.0).abs() < 1e-6);
    app.insert_resource(ListenerOutdoors(false));
    app.update();
    assert!(gain(&app) <= 0.0, "a 500 ms frame covers the 0.5 s swing: {}", gain(&app));
    app.insert_resource(ListenerOutdoors(true));
    app.update();
    assert!((gain(&app) - 1.0).abs() < 1e-6);
}

fn cue_events(app: &mut App) -> Vec<RecordId> {
    events(app)
        .into_iter()
        .filter_map(|e| if let AudioEvent::Cue { sound, .. } = e { Some(sound) } else { None })
        .collect()
}

fn play(app: &mut App, cue: &str, at: SoundAt) {
    app.world_mut().write_message(PlaySound { cue: id(cue), at });
}

#[test]
fn cues_pick_variants_cap_instances_and_cull_by_distance() {
    let mut app = app();
    let mut group = sf_audio::SoundGroup::of([id("rain"), id("storm")]);
    group.attenuation = Some(Attenuation::new(1.0, 10.0));
    group.max_instances = 2;
    app.world_mut().resource_mut::<AudioCues>().0.insert(id("step"), group);
    app.world_mut().spawn((SpatialListener::new(0.2), GlobalTransform::default()));
    // Three plays in a frame: the cap of two holds, and variants do not repeat.
    for _ in 0..3 {
        play(&mut app, "step", SoundAt::Point(Vec3::new(2.0, 0.0, 0.0)));
    }
    app.update();
    let played = cue_events(&mut app);
    assert_eq!(played.len(), 2, "{played:?}");
    assert_ne!(played[0], played[1]);
    // Beyond max distance nothing starts (cue cap aside: a new cue id, the plain bank sound).
    play(&mut app, "thunder", SoundAt::Point(Vec3::new(500.0, 0.0, 0.0)));
    app.update();
    assert!(cue_events(&mut app).is_empty());
    // 2D cues ignore distance.
    play(&mut app, "thunder", SoundAt::Listener);
    app.update();
    assert_eq!(cue_events(&mut app), vec![id("thunder")]);
}

#[test]
fn voice_gain_follows_the_distance_curve() {
    let mixer = Mixer::default();
    let v = Voice { sound: id("x"), bus: Bus::Sfx, base: 0.5, level: 1.0, target: 1.0, looping: false };
    let s = SpatialVoice(Attenuation { min_m: 1.0, max_m: 11.0, rolloff: sf_audio::Rolloff::Linear });
    let ear = Some(Vec3::ZERO);
    assert!((voice_gain(&mixer, &v, None, ear) - 0.5).abs() < 1e-6);
    assert!((voice_gain(&mixer, &v, Some((&s, Vec3::new(6.0, 0.0, 0.0))), ear) - 0.25).abs() < 1e-6);
    assert!(voice_gain(&mixer, &v, Some((&s, Vec3::new(0.0, 0.0, 20.0))), ear).abs() < 1e-6);
    // No listener yet: unattenuated.
    assert!((voice_gain(&mixer, &v, Some((&s, Vec3::new(6.0, 0.0, 0.0))), None) - 0.5).abs() < 1e-6);
}

#[test]
fn ambience_beds_follow_layers_and_the_door() {
    let mut app = app();
    app.world_mut().resource_mut::<AmbienceLoops>().0.set_layer(
        "cell",
        vec![(id("rain"), 1.0, sf_audio::Space::Outdoor), (id("storm"), 0.5, sf_audio::Space::Indoor)],
    );
    app.update();
    assert_eq!(events(&mut app), vec![AudioEvent::LoopStart { sound: id("rain"), target: 1.0 }]);
    for _ in 0..4 {
        app.update();
    }
    assert!((level(&mut app, "rain").unwrap() - 1.0).abs() < 1e-5);
    // Go inside: the outdoor bed fades out and the indoor bed fades in.
    app.insert_resource(ListenerOutdoors(false));
    app.update();
    let ev = events(&mut app);
    for _ in 0..7 {
        app.update();
    }
    assert!(ev.contains(&AudioEvent::LoopStop { sound: id("rain") }), "{ev:?}");
    assert!(ev.contains(&AudioEvent::LoopStart { sound: id("storm"), target: 0.5 }), "{ev:?}");
    assert_eq!(level(&mut app, "rain"), None);
    assert!((level(&mut app, "storm").unwrap() - 0.5).abs() < 1e-5);
}

#[test]
fn music_requests_start_tracks() {
    let mut app = app();
    {
        let mut music = app.world_mut().resource_mut::<Music>();
        music.0.set_playlist("explore", vec!["Music/a.mp3".into()]);
        music.0.request(Some("explore"));
    }
    app.update();
    assert!(events(&mut app).contains(&AudioEvent::MusicStart { path: "Music/a.mp3".into() }));
    assert_eq!(app.world_mut().query::<&MusicVoice>().iter(app.world()).count(), 1);
    app.world_mut().resource_mut::<Music>().0.request(None);
    for _ in 0..8 {
        app.update();
    }
    assert_eq!(app.world_mut().query::<&MusicVoice>().iter(app.world()).count(), 0);
}

#[test]
fn one_shot_sets_draw_on_the_ambience_bus_outdoors() {
    let mut app = app();
    app.world_mut().resource_mut::<OneShotSets>().0 =
        vec![OneShotSet { key: 7, interval_s: [0.4, 0.6], sounds: vec![(id("thunder"), 1.0)], volume: 0.5 }];
    let mut drawn = 0;
    for _ in 0..8 {
        app.update();
        drawn += events(&mut app).iter().filter(|e| **e == AudioEvent::OneShot { sound: id("thunder"), volume: 0.5, bus: Bus::Ambience }).count();
    }
    assert!(drawn >= 2, "{drawn}");
}

#[test]
fn volumes_reach_the_mixer_and_voices() {
    let mut app = app();
    {
        let mut volumes = app.world_mut().resource_mut::<AudioVolumes>();
        volumes.0.set_level(Bus::Ambience, 0.5);
        volumes.0.set_muted(Bus::Music, true);
    }
    app.update();
    let mixer = app.world().resource::<AudioMixer>().0.clone();
    assert!((mixer.bus_gain(Bus::Ambience) - sf_audio::slider_to_gain(0.5)).abs() < 1e-7);
    assert!((mixer.bus_gain(Bus::Weather) - sf_audio::slider_to_gain(0.5)).abs() < 1e-7, "weather under ambience");
    assert!(mixer.bus_gain(Bus::Music) <= 0.0);
    assert!((mixer.bus_gain(Bus::Sfx) - 1.0).abs() < 1e-7);
}

#[test]
fn speech_ducks_the_music_bus_and_releases() {
    let mut app = app();
    app.world_mut().resource_mut::<AudioDucking>().0.add_rule(sf_audio::DuckRule {
        trigger: sf_audio::DuckTrigger::Bus(Bus::Voice),
        target: Bus::Music,
        depth_db: 8.0,
        attack_s: 0.25,
        release_s: 1.0,
    });
    let music = |app: &App| app.world().resource::<AudioMixer>().0.bus_gain(Bus::Music);
    app.update();
    assert!((music(&app) - 1.0).abs() < 1e-7, "nothing speaks");
    // A line of speech (a voice on the Voice bus): full duck within one 500 ms frame.
    let line = app
        .world_mut()
        .spawn(Voice { sound: id("rain"), bus: Bus::Voice, base: 1.0, level: 1.0, target: 1.0, looping: false })
        .id();
    app.update();
    assert!((music(&app) - sf_audio::db_to_gain(-8.0)).abs() < 1e-6, "{}", music(&app));
    // The line ends: released over 1 s (two frames).
    app.world_mut().despawn(line);
    app.update();
    assert!((music(&app) - sf_audio::db_to_gain(-4.0)).abs() < 1e-5, "{}", music(&app));
    app.update();
    assert!((music(&app) - 1.0).abs() < 1e-7);
}
