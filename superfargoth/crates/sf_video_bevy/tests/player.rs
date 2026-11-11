//! Headless player sanity (video.md "Build plan" V2, V3b): a synthetic format with a few known gray
//! frames, played once, looped, despawned mid-play, skipped, hidden and with a sound track ducking the
//! other buses. Sanity assertions only; the clock is real time, so loops poll with a generous timeout
//! instead of asserting timings.

use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_asset::{AssetApp, AssetPlugin, Assets};
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_ui::widget::ImageNode;
use bevy_window::{PrimaryWindow, Window};
use sf_audio::DuckTrigger;
use sf_audio_bevy::AudioDucking;
use sf_video::{
    AudioInfo, Frame, Plane, Pts, Result, VideoDecoders, VideoFormat, VideoInfo, VideoReader,
    VideoSource,
};
use sf_video_bevy::{
    DUCKED_BUSES, VIDEO_DUCK_HOLD, VideoAudioStream, VideoDecoderRegistry, VideoFinished,
    VideoPauseWhenHidden, VideoPlayer, VideoPlugin, VideoSkipped, VideoSystems,
};

const MAGIC: &[u8; 4] = b"TEST";
const W: u32 = 8;
const H: u32 = 4;
const FPS: u32 = 100;
/// The sound track's rate (mono): one 10-sample packet per frame.
const RATE: u32 = 1000;
const TIMEOUT: Duration = Duration::from_secs(10);

/// Header: `TEST` + frame count (one byte) + sound flag (one byte: 1 = a mono track of silence). Frame
/// `i` is uniform gray, luma `luma(i)`, full range.
struct TestFormat {
    /// Set when a source this format opened is dropped (its decode thread has returned).
    dropped: Arc<AtomicBool>,
}

fn luma(index: u32) -> u8 {
    u8::try_from(10 + 40 * index).unwrap()
}

impl VideoFormat for TestFormat {
    fn name(&self) -> &'static str {
        "test"
    }

    fn probe(&self, header: &[u8]) -> bool {
        header.starts_with(MAGIC)
    }

    fn open(&self, mut reader: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>> {
        let mut header = [0u8; 6];
        reader.read_exact(&mut header)?;
        Ok(Box::new(TestSource {
            frames: u32::from(header[4]),
            sound: header[5] == 1,
            next: 0,
            dropped: Arc::clone(&self.dropped),
        }))
    }
}

struct TestSource {
    frames: u32,
    sound: bool,
    next: u32,
    dropped: Arc<AtomicBool>,
}

impl Drop for TestSource {
    fn drop(&mut self) {
        self.dropped.store(true, Ordering::SeqCst);
    }
}

fn plane(width: u32, height: u32, value: u8) -> Plane {
    Plane {
        width,
        height,
        stride: width as usize,
        data: vec![value; (width * height) as usize],
    }
}

impl VideoSource for TestSource {
    fn info(&self) -> VideoInfo {
        VideoInfo {
            width: W,
            height: H,
            fps_num: FPS,
            fps_den: 1,
            frames: self.frames,
            audio: self.sound.then_some(AudioInfo {
                sample_rate: RATE,
                channels: 1,
            }),
            alpha: false,
            full_range: true,
        }
    }

    fn next_frame(&mut self, out: &mut Frame) -> Result<Option<Pts>> {
        if self.next >= self.frames {
            return Ok(None);
        }
        let index = self.next;
        self.next += 1;
        out.y = plane(W, H, luma(index));
        out.u = plane(W / 2, H / 2, 128);
        out.v = plane(W / 2, H / 2, 128);
        out.alpha = None;
        out.full_range = true;
        Ok(Some(Pts {
            index,
            time_us: self.info().frame_time_us(index),
        }))
    }

    fn next_audio(&mut self, out: &mut Vec<i16>) -> Result<usize> {
        if !self.sound {
            return Ok(0);
        }
        let n = (RATE / FPS) as usize;
        out.extend(std::iter::repeat_n(0, n));
        Ok(n)
    }

    fn seek_start(&mut self) -> Result<()> {
        self.next = 0;
        Ok(())
    }
}

#[derive(Resource, Default)]
struct Finished(Vec<Entity>);

fn collect_finished(mut reader: MessageReader<VideoFinished>, mut finished: ResMut<Finished>) {
    finished.0.extend(reader.read().map(|m| m.entity));
}

#[derive(Resource, Default)]
struct Skipped(Vec<Entity>);

fn collect_skipped(mut reader: MessageReader<VideoSkipped>, mut skipped: ResMut<Skipped>) {
    skipped.0.extend(reader.read().map(|m| m.entity));
}

/// An app with the test format registered first, and the flag its sources set when dropped.
fn app() -> (App, Arc<AtomicBool>) {
    let dropped = Arc::new(AtomicBool::new(false));
    let mut decoders = VideoDecoders::default();
    decoders.register(Box::new(TestFormat {
        dropped: Arc::clone(&dropped),
    }));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<Image>()
        .insert_resource(VideoDecoderRegistry(decoders))
        .add_plugins(VideoPlugin)
        .init_resource::<Finished>()
        .init_resource::<Skipped>()
        .add_systems(
            Update,
            (collect_finished, collect_skipped).after(VideoSystems),
        );
    (app, dropped)
}

fn video(frames: u8) -> Vec<u8> {
    let mut bytes = MAGIC.to_vec();
    bytes.extend([frames, 0]);
    bytes
}

/// A video with a (silent) mono sound track.
fn sounded_video(frames: u8) -> Vec<u8> {
    let mut bytes = MAGIC.to_vec();
    bytes.extend([frames, 1]);
    bytes
}

/// Update for about `span` of real time.
fn run_for(app: &mut App, span: Duration) {
    let start = Instant::now();
    while start.elapsed() < span {
        app.update();
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Update until `done` holds or the timeout passes; returns whether it held.
fn run_until(app: &mut App, mut done: impl FnMut(&mut App) -> bool) -> bool {
    let start = Instant::now();
    while start.elapsed() < TIMEOUT {
        app.update();
        if done(app) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    false
}

fn player(app: &App, entity: Entity) -> &VideoPlayer {
    app.world().get::<VideoPlayer>(entity).unwrap()
}

#[test]
fn plays_once_then_finishes() {
    const N: u8 = 5;
    let (mut app, _) = app();
    let entity = app
        .world_mut()
        .spawn(VideoPlayer::new(video(N), false))
        .id();

    let mut shown = Vec::new();
    let finished = run_until(&mut app, |app| {
        let p = player(app, entity);
        if let Some(frame) = p.last_frame()
            && shown.last() != Some(&frame)
        {
            shown.push(frame);
        }
        !app.world().resource::<Finished>().0.is_empty()
    });
    assert!(finished, "no VideoFinished within {TIMEOUT:?}");
    // A few more updates: the message must not repeat.
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(app.world().resource::<Finished>().0, vec![entity]);

    let p = player(&app, entity);
    assert!(p.is_finished());
    assert!(shown.is_sorted(), "frames went backwards: {shown:?}");
    assert_eq!(
        p.last_frame(),
        Some(u32::from(N) - 1),
        "the last frame stays up"
    );
    assert!(p.frames_shown() >= 1 && p.frames_shown() <= u64::from(N));

    // The node shows the player's image, at the video's size, holding the last frame's gray.
    let handle = p.image().unwrap().clone();
    assert_eq!(app.world().get::<ImageNode>(entity).unwrap().image, handle);
    let images = app.world().resource::<Assets<Image>>();
    let image = images.get(&handle).unwrap();
    assert_eq!((image.width(), image.height()), (W, H));
    let data = image.data.as_ref().unwrap();
    assert_eq!(data.len(), (W * H * 4) as usize);
    let expected = luma(u32::from(N) - 1);
    assert!(
        data.as_chunks::<4>()
            .0
            .iter()
            .all(|px| px[0].abs_diff(expected) <= 1 && px[3] == 255),
        "pixels are not the last frame's gray {expected}: {:?}",
        &data[..4]
    );
}

#[test]
fn looped_plays_past_the_end() {
    const N: u8 = 3;
    let (mut app, _) = app();
    let entity = app.world_mut().spawn(VideoPlayer::new(video(N), true)).id();
    let looped = run_until(&mut app, |app| {
        player(app, entity).frames_shown() > 2 * u64::from(N)
    });
    assert!(
        looped,
        "looped video stopped at {} frames",
        player(&app, entity).frames_shown()
    );
    assert!(!player(&app, entity).is_finished());
    assert!(app.world().resource::<Finished>().0.is_empty());
}

#[test]
fn despawn_mid_play_stops_the_thread() {
    let (mut app, dropped) = app();
    let entity = app.world_mut().spawn(VideoPlayer::new(video(4), true)).id();
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 1));
    assert!(!dropped.load(Ordering::SeqCst));

    app.world_mut().despawn(entity);
    app.update();
    let start = Instant::now();
    while !dropped.load(Ordering::SeqCst) && start.elapsed() < TIMEOUT {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(
        dropped.load(Ordering::SeqCst),
        "decode thread still running after despawn"
    );
    assert!(app.world().resource::<Finished>().0.is_empty());
}

#[test]
fn removing_the_player_stops_the_thread() {
    let (mut app, dropped) = app();
    let entity = app.world_mut().spawn(VideoPlayer::new(video(4), true)).id();
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 1));

    app.world_mut().entity_mut(entity).remove::<VideoPlayer>();
    app.update();
    let start = Instant::now();
    while !dropped.load(Ordering::SeqCst) && start.elapsed() < TIMEOUT {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(
        dropped.load(Ordering::SeqCst),
        "decode thread still running after removal"
    );
    assert!(app.world().resource::<Finished>().0.is_empty());
}

#[test]
fn unknown_format_finishes_with_a_warning() {
    let (mut app, _) = app();
    let entity = app
        .world_mut()
        .spawn(VideoPlayer::new(b"not a video".to_vec(), true))
        .id();
    app.update();
    assert!(player(&app, entity).is_finished());
    assert_eq!(app.world().resource::<Finished>().0, vec![entity]);
}

/// An app with a keyboard (`ButtonInput<KeyCode>`, no input plugin: presses stay until cleared).
fn app_with_keys() -> (App, Arc<AtomicBool>) {
    let (mut app, dropped) = app();
    app.init_resource::<ButtonInput<KeyCode>>();
    (app, dropped)
}

fn press(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
}

#[test]
fn a_skip_key_ends_a_skippable_video_once() {
    let (mut app, dropped) = app_with_keys();
    let entity = app
        .world_mut()
        .spawn(VideoPlayer::new(video(4), true).skippable(true))
        .id();
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 1));

    press(&mut app, KeyCode::Escape);
    app.update();
    assert!(player(&app, entity).is_finished());
    assert!(
        !app.world()
            .resource::<ButtonInput<KeyCode>>()
            .just_pressed(KeyCode::Escape),
        "the skip press is consumed"
    );
    for _ in 0..10 {
        app.update();
    }
    assert_eq!(app.world().resource::<Skipped>().0, vec![entity]);
    assert_eq!(app.world().resource::<Finished>().0, vec![entity]);

    let start = Instant::now();
    while !dropped.load(Ordering::SeqCst) && start.elapsed() < TIMEOUT {
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(
        dropped.load(Ordering::SeqCst),
        "decode thread still running after the skip"
    );
}

#[test]
fn a_video_that_is_not_skippable_ignores_input() {
    let (mut app, _) = app_with_keys();
    let entity = app.world_mut().spawn(VideoPlayer::new(video(3), true)).id();
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 1));
    let before = player(&app, entity).frames_shown();

    press(&mut app, KeyCode::Escape);
    press(&mut app, KeyCode::Enter);
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        > before + 3));
    assert!(!player(&app, entity).is_finished());
    assert!(
        app.world()
            .resource::<ButtonInput<KeyCode>>()
            .just_pressed(KeyCode::Escape),
        "not consumed without a skippable video"
    );
    assert!(app.world().resource::<Skipped>().0.is_empty());
    assert!(app.world().resource::<Finished>().0.is_empty());
}

#[test]
fn a_sounded_video_holds_the_duck_until_it_ends() {
    let (mut app, _) = app();
    // The track's asset store (normally registered with `bevy_audio`'s plugin; no device here, so the
    // clock stays real time) and `sf_audio_bevy`'s ducking resource.
    app.init_asset::<VideoAudioStream>()
        .init_resource::<AudioDucking>();
    let entity = app
        .world_mut()
        .spawn(VideoPlayer::new(sounded_video(20), false))
        .id();

    let held = |app: &App| {
        app.world()
            .resource::<AudioDucking>()
            .0
            .is_held(VIDEO_DUCK_HOLD)
    };
    assert!(
        run_until(&mut app, |app| held(app)),
        "no duck while sounding"
    );
    assert!(player(&app, entity).audio_entity().is_some());
    let trigger = DuckTrigger::Hold(VIDEO_DUCK_HOLD.to_owned());
    let mut targets: Vec<_> = app
        .world()
        .resource::<AudioDucking>()
        .0
        .rules()
        .iter()
        .filter(|r| r.trigger == trigger)
        .map(|r| r.target)
        .collect();
    targets.dedup();
    assert_eq!(targets, DUCKED_BUSES);

    assert!(run_until(&mut app, |app| !app
        .world()
        .resource::<Finished>()
        .0
        .is_empty()));
    app.update();
    assert!(!held(&app), "the duck is released once the video ends");
    assert_eq!(
        app.world().resource::<AudioDucking>().0.rules().len(),
        DUCKED_BUSES.len(),
        "rules added once"
    );
}

#[test]
fn a_hidden_player_stops_and_resumes() {
    let (mut app, _) = app();
    app.insert_resource(VideoPauseWhenHidden(true));
    let entity = app
        .world_mut()
        .spawn((
            VideoPlayer::new(video(4), true),
            InheritedVisibility::HIDDEN,
        ))
        .id();

    run_for(&mut app, Duration::from_millis(150));
    assert!(player(&app, entity).is_paused());
    assert_eq!(
        player(&app, entity).frames_shown(),
        0,
        "hidden from the start"
    );

    let show = |app: &mut App, visible: bool| {
        let v = if visible {
            InheritedVisibility::VISIBLE
        } else {
            InheritedVisibility::HIDDEN
        };
        app.world_mut().entity_mut(entity).insert(v);
    };
    show(&mut app, true);
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 3));
    assert!(!player(&app, entity).is_paused());

    show(&mut app, false);
    app.update();
    let held = player(&app, entity).frames_shown();
    run_for(&mut app, Duration::from_millis(150));
    assert_eq!(
        player(&app, entity).frames_shown(),
        held,
        "frames advanced while hidden"
    );

    show(&mut app, true);
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        > held + 2));
    assert!(!player(&app, entity).is_finished());
}

#[test]
fn a_hidden_primary_window_pauses_players() {
    let (mut app, _) = app();
    let window = app
        .world_mut()
        .spawn((
            Window {
                visible: false,
                ..Window::default()
            },
            PrimaryWindow,
        ))
        .id();
    let entity = app.world_mut().spawn(VideoPlayer::new(video(4), true)).id();
    run_for(&mut app, Duration::from_millis(100));
    assert!(player(&app, entity).is_paused());
    assert_eq!(player(&app, entity).frames_shown(), 0);

    app.world_mut().get_mut::<Window>(window).unwrap().visible = true;
    assert!(run_until(&mut app, |app| player(app, entity)
        .frames_shown()
        >= 2));
}
