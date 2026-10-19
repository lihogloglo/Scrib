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
        .init_resource::<CurrentWeather>()
        .add_plugins(SfAudioPlugin { bank, playback: false });
    app.update();
    app
}

fn send(app: &mut App, m: WeatherSound) {
    app.world_mut().write_message(m);
}

fn events(app: &mut App) -> Vec<AudioEvent> {
    app.world_mut().resource_mut::<Messages<AudioEvent>>().drain().collect()
}

fn lp(sound: &str, volume: f32) -> WeatherSound {
    WeatherSound::Loop { sound: id(sound), volume, region: false }
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
    send(&mut app, WeatherSound::OneShot { sound: id("thunder"), volume: 0.8, azimuth: 0.0, distance_m: 900.0 });
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
