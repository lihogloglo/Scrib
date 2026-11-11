//! Bevy video player over `sf_video` (`superfargoth/docs/specs/video.md`, "Design" and "Build plan" V2).
//!
//! A [`VideoPlayer`] component on a UI entity plays a video into that entity's [`ImageNode`]:
//!
//! - **Open.** The first [`VideoSystems`] run after the component is added probes and opens the bytes with
//!   the [`VideoDecoderRegistry`] resource (Bink 1 by default; games register more formats), creates an
//!   `Rgba8UnormSrgb` [`Image`] of the video's size and sets it on the [`ImageNode`] (required by
//!   [`VideoPlayer`], so one is added if missing).
//! - **Decode thread.** A plain `std::thread` decodes ahead and converts each frame to RGBA8 with
//!   [`sf_video::yuv420_to_rgba8`], into a bounded queue of [`QUEUE_FRAMES`] frames; a full queue blocks the
//!   thread, so it never runs more than that far ahead. Shown frames' buffers go back to it for reuse.
//! - **Clock.** No-audio videos clock on [`Time<Real>`] (menus pause `Time<Virtual>`), starting when the
//!   first frame arrives. Each update shows the newest frame whose presentation time is due; older due frames
//!   are dropped (late), later ones wait (early).
//! - **Upload.** The image is `MAIN_WORLD | RENDER_WORLD`, and the frame swap goes through
//!   `Assets::get_mut`, which queues `AssetEvent::Modified`; Bevy 0.20's render-asset extraction then clones
//!   the image and `GpuImage::prepare_asset` writes the data into the existing texture (same descriptor,
//!   `COPY_DST`) instead of creating a new one.
//! - **End.** With `looped`, the thread calls [`VideoSource::seek_start`](sf_video::VideoSource::seek_start)
//!   at the end of the stream and carries on, timestamps continuing past the first pass. Without it, the
//!   last frame stays up for its duration, then a [`VideoFinished`] message is written and the player stops
//!   (the last frame stays on the image). A probe, open or decode error ends playback the same way, with a
//!   `warn!`, never a panic.
//! - **Stop.** Despawning the entity or removing the component drops the queue's receiver; the thread's
//!   next send fails and it returns, dropping its source. Nothing joins it on the main thread, so a
//!   despawn never waits on a decode.
//!
//! Not yet (V3): the audio track, A/V sync, `Bus::Movie`, ducking, skip, and pausing the thread while the
//! node is hidden or the window minimised.

use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError};
use std::time::Duration;

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_log::warn;
use bevy_time::{Real, Time};
use bevy_ui::widget::ImageNode;
use sf_video::{Frame, VideoDecoders, VideoInfo, VideoSource, yuv420_to_rgba8};
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

/// Decoded RGBA frames the decode thread may queue ahead of the display (video.md "Design": 3-4).
pub const QUEUE_FRAMES: usize = 3;

/// Adds the [`VideoDecoderRegistry`] (unless the app inserted one first), the [`VideoFinished`] message
/// and the player systems in [`VideoSystems`] (`Update`). Needs `Assets<Image>` (Bevy's `ImagePlugin`)
/// and `Time<Real>` (`TimePlugin`).
#[derive(Default)]
pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<VideoDecoderRegistry>() {
            app.insert_resource(VideoDecoderRegistry::default());
        }
        app.add_message::<VideoFinished>()
            .add_systems(Update, advance_players.in_set(VideoSystems));
    }
}

/// The player systems (open, show due frames, finish), for ordering against them.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoSystems;

/// The video formats players probe, as a resource. `Default` registers the built-in formats (Bink 1);
/// register more with `registry.0.register(..)` before players start.
#[derive(Resource, Default)]
pub struct VideoDecoderRegistry(pub VideoDecoders);

/// Written once when a player that doesn't loop reaches the end of its video, or when its video fails
/// to open or decode. Not written for a player despawned or removed mid-play.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoFinished {
    pub entity: Entity,
}

/// Plays a video into this entity's [`ImageNode`]. See the crate docs for the lifecycle.
#[derive(Component)]
#[require(ImageNode)]
pub struct VideoPlayer {
    bytes: Arc<[u8]>,
    looped: bool,
    state: PlayerState,
    image: Option<Handle<Image>>,
    info: Option<VideoInfo>,
    frames_shown: u64,
    last_frame: Option<u32>,
}

impl VideoPlayer {
    /// A player for the whole video file in `bytes` (read through `sf_vfs` by the caller); with `looped`
    /// it restarts at the end and never finishes.
    #[must_use]
    pub fn new(bytes: impl Into<Arc<[u8]>>, looped: bool) -> Self {
        Self {
            bytes: bytes.into(),
            looped,
            state: PlayerState::Pending,
            image: None,
            info: None,
            frames_shown: 0,
            last_frame: None,
        }
    }

    /// Whether the video restarts at its end.
    #[must_use]
    pub fn looped(&self) -> bool {
        self.looped
    }

    /// The image frames are written to, once the video is open.
    #[must_use]
    pub fn image(&self) -> Option<&Handle<Image>> {
        self.image.as_ref()
    }

    /// The stream description, once the video is open.
    #[must_use]
    pub fn info(&self) -> Option<VideoInfo> {
        self.info
    }

    /// Frames written to the image so far (late frames dropped unseen don't count; loops add up).
    #[must_use]
    pub fn frames_shown(&self) -> u64 {
        self.frames_shown
    }

    /// Stream index of the frame on the image now.
    #[must_use]
    pub fn last_frame(&self) -> Option<u32> {
        self.last_frame
    }

    /// Whether playback has ended (end of a non-looped video, or an error).
    #[must_use]
    pub fn is_finished(&self) -> bool {
        matches!(self.state, PlayerState::Finished)
    }
}

enum PlayerState {
    Pending,
    Playing(Playback),
    Finished,
}

/// The main-thread end of a decode thread. Dropping it stops the thread (its next send fails).
struct Playback {
    /// `Mutex` only to make the component `Sync`; the system has `&mut` access, so it never locks.
    rx: Mutex<Receiver<Msg>>,
    /// Shown frames' old buffers, back to the thread for reuse.
    recycle: SyncSender<Vec<u8>>,
    /// The next message, received but not yet due.
    pending: Option<Msg>,
    /// `Time<Real>` elapsed at stream time 0, set by the first message.
    clock_start: Option<Duration>,
}

/// Decode thread to display, in stream order. Times are microseconds from the start of playback
/// (loops continue the count).
enum Msg {
    Frame {
        index: u32,
        time_us: u64,
        rgba: Vec<u8>,
    },
    End {
        time_us: u64,
    },
    Error(String),
}

impl Msg {
    fn time_us(&self) -> u64 {
        match self {
            Msg::Frame { time_us, .. } | Msg::End { time_us } => *time_us,
            Msg::Error(_) => 0,
        }
    }
}

fn advance_players(
    time: Res<Time<Real>>,
    decoders: Res<VideoDecoderRegistry>,
    mut images: ResMut<Assets<Image>>,
    mut players: Query<(Entity, &mut VideoPlayer, &mut ImageNode)>,
    mut finished: MessageWriter<VideoFinished>,
) {
    let now = time.elapsed();
    for (entity, mut player, mut node) in &mut players {
        let player = &mut *player;
        if matches!(player.state, PlayerState::Pending) {
            match start(player, &decoders.0, &mut images) {
                Ok(handle) => node.image = handle,
                Err(error) => {
                    warn!("video on {entity}: {error}");
                    player.state = PlayerState::Finished;
                    finished.write(VideoFinished { entity });
                    continue;
                }
            }
        }
        let PlayerState::Playing(playback) = &mut player.state else {
            continue;
        };
        let (due, outcome) = drain_due(playback, now);
        if let Some((index, rgba)) = due {
            if let Some(handle) = &player.image
                && let Some(mut image) = images.get_mut(handle)
                && let Some(old) = image.data.replace(rgba)
            {
                // Full or disconnected: the thread allocates instead.
                let _ = playback.recycle.try_send(old);
            }
            player.frames_shown += 1;
            player.last_frame = Some(index);
        }
        if let Some(outcome) = outcome {
            if let Err(error) = outcome {
                warn!("video on {entity}: {error}");
            }
            player.state = PlayerState::Finished;
            finished.write(VideoFinished { entity });
        }
    }
}

/// What one update takes from the queue: the frame to show (index, RGBA) and, once due, how playback ended.
type Drained = (Option<(u32, Vec<u8>)>, Option<Result<(), String>>);

/// Take every due message: the newest due frame (older due frames are dropped as late) and, if the end
/// or an error is due, the outcome.
fn drain_due(playback: &mut Playback, now: Duration) -> Drained {
    let rx = playback
        .rx
        .get_mut()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut due = None;
    loop {
        let msg = match playback.pending.take() {
            Some(msg) => msg,
            None => match rx.try_recv() {
                Ok(msg) => msg,
                Err(TryRecvError::Empty) => return (due, None),
                Err(TryRecvError::Disconnected) => {
                    return (due, Some(Err("decode thread stopped".to_owned())));
                }
            },
        };
        let time_us = msg.time_us();
        let start = *playback
            .clock_start
            .get_or_insert_with(|| now.saturating_sub(Duration::from_micros(time_us)));
        let elapsed_us = u64::try_from(now.saturating_sub(start).as_micros()).unwrap_or(u64::MAX);
        if time_us > elapsed_us {
            playback.pending = Some(msg);
            return (due, None);
        }
        match msg {
            Msg::Frame { index, rgba, .. } => {
                if let Some((_, late)) = due.replace((index, rgba)) {
                    let _ = playback.recycle.try_send(late);
                }
            }
            Msg::End { .. } => return (due, Some(Ok(()))),
            Msg::Error(error) => return (due, Some(Err(error))),
        }
    }
}

/// Open the video, make its image and start the decode thread.
fn start(
    player: &mut VideoPlayer,
    decoders: &VideoDecoders,
    images: &mut Assets<Image>,
) -> Result<Handle<Image>, String> {
    let reader = Box::new(Cursor::new(Arc::clone(&player.bytes)));
    let source = decoders
        .probe_and_open(reader)
        .map_err(|e| format!("open failed: {e}"))?;
    let info = source.info();
    if info.width == 0 || info.height == 0 {
        return Err(format!("empty frame size {}x{}", info.width, info.height));
    }
    let size = Extent3d {
        width: info.width,
        height: info.height,
        depth_or_array_layers: 1,
    };
    let black = [0, 0, 0, 255].repeat(info.width as usize * info.height as usize);
    let image = Image::new(
        size,
        TextureDimension::D2,
        black,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let handle = images.add(image);

    let (tx, rx) = mpsc::sync_channel(QUEUE_FRAMES);
    let (recycle, recycled) = mpsc::sync_channel(QUEUE_FRAMES + 1);
    let looped = player.looped;
    std::thread::Builder::new()
        .name("sf_video decode".to_owned())
        .spawn(move || decode_loop(source, looped, &tx, &recycled))
        .map_err(|e| format!("decode thread failed to start: {e}"))?;

    player.info = Some(info);
    player.image = Some(handle.clone());
    player.state = PlayerState::Playing(Playback {
        rx: Mutex::new(rx),
        recycle,
        pending: None,
        clock_start: None,
    });
    Ok(handle)
}

/// The decode thread: decode, convert, send; rewind at the end when looping. Returns when the display end
/// is dropped (a send fails), at the end of a non-looped stream, or on an error.
fn decode_loop(
    mut source: Box<dyn VideoSource>,
    looped: bool,
    tx: &SyncSender<Msg>,
    recycled: &Receiver<Vec<u8>>,
) {
    let info = source.info();
    let mut frame = Frame::default();
    // Stream time of this pass's start: loops keep the clock running.
    let mut base_us = 0u64;
    loop {
        let mut pass_end_us = base_us;
        let mut pass_frames = 0u32;
        loop {
            match source.next_frame(&mut frame) {
                Ok(Some(pts)) => {
                    if frame.size() != (info.width, info.height) {
                        let (w, h) = frame.size();
                        let _ = tx.send(Msg::Error(format!(
                            "frame {} is {w}x{h}, the stream says {}x{}",
                            pts.index, info.width, info.height
                        )));
                        return;
                    }
                    let mut rgba = recycled.try_recv().unwrap_or_default();
                    yuv420_to_rgba8(&frame, &mut rgba);
                    pass_frames += 1;
                    pass_end_us = base_us + info.frame_time_us(pts.index + 1);
                    let msg = Msg::Frame {
                        index: pts.index,
                        time_us: base_us + pts.time_us,
                        rgba,
                    };
                    if tx.send(msg).is_err() {
                        return;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = tx.send(Msg::Error(format!("decode failed: {e}")));
                    return;
                }
            }
        }
        // A pass with no frames would loop forever without showing anything: end it.
        if !looped || pass_frames == 0 {
            let _ = tx.send(Msg::End {
                time_us: pass_end_us,
            });
            return;
        }
        if let Err(e) = source.seek_start() {
            let _ = tx.send(Msg::Error(format!("rewind failed: {e}")));
            return;
        }
        base_us = pass_end_us;
    }
}
