//! Headless player sanity (video.md "Build plan" V2): a synthetic format with a few known gray frames,
//! played once, looped, and despawned mid-play. Sanity assertions only; the clock is real time, so loops
//! poll with a generous timeout instead of asserting timings.

use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_asset::{AssetApp, AssetPlugin, Assets};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_ui::widget::ImageNode;
use sf_video::{
    Frame, Plane, Pts, Result, VideoDecoders, VideoFormat, VideoInfo, VideoReader, VideoSource,
};
use sf_video_bevy::{VideoDecoderRegistry, VideoFinished, VideoPlayer, VideoPlugin, VideoSystems};

const MAGIC: &[u8; 4] = b"TEST";
const W: u32 = 8;
const H: u32 = 4;
const FPS: u32 = 100;
const TIMEOUT: Duration = Duration::from_secs(10);

/// Header: `TEST` + frame count (one byte). Frame `i` is uniform gray, luma `luma(i)`, full range.
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
        let mut header = [0u8; 5];
        reader.read_exact(&mut header)?;
        Ok(Box::new(TestSource {
            frames: u32::from(header[4]),
            next: 0,
            dropped: Arc::clone(&self.dropped),
        }))
    }
}

struct TestSource {
    frames: u32,
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
            audio: None,
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

    fn next_audio(&mut self, _out: &mut Vec<i16>) -> Result<usize> {
        Ok(0)
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
        .add_systems(Update, collect_finished.after(VideoSystems));
    (app, dropped)
}

fn video(frames: u8) -> Vec<u8> {
    let mut bytes = MAGIC.to_vec();
    bytes.push(frames);
    bytes
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
