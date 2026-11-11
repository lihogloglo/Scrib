//! A video's audio track through `bevy_audio` (video.md "Design", V3a): the decode thread pushes the
//! stream's PCM into an [`AudioRing`], a [`VideoAudioStream`] asset hands `bevy_audio` a
//! [`VideoAudioSource`] (a rodio `Source`) that drains it, and the samples it hands out are the
//! video's master clock ([`AudioRing::played_us`]).
//!
//! The ring is a single-producer, single-consumer queue of `i16` slots behind two monotonic atomic
//! counters (no locks on the audio thread). The producer only pushes whole sample frames, and an
//! underrun yields whole frames of silence, so channels never swap. Silence is counted as played: the
//! counter follows the device, not the data (an underrun mid-stream would slip the sound behind the
//! clock; the decode thread runs ahead of the display, so it doesn't in practice).

use std::collections::VecDeque;
use std::num::NonZero;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicU64, Ordering};
use std::time::Duration;

use bevy_asset::Asset;
use bevy_audio::{ChannelCount, Decodable, Sample, SampleRate, Source};
use bevy_ecs::prelude::*;
use bevy_reflect::TypePath;

/// Seconds of PCM the ring holds: room for Bink's front-loaded audio, so the decode thread rarely
/// keeps any back.
pub(crate) const RING_SECONDS: usize = 4;

/// The shared audio queue of one player (decode thread to audio thread).
pub(crate) struct AudioRing {
    slots: Box<[AtomicI16]>,
    /// Samples ever written (producer) and read (consumer); `write - read` are queued.
    write: AtomicU64,
    read: AtomicU64,
    /// Samples handed to the device, silence included: the clock.
    played: AtomicU64,
    /// Set when `bevy_audio` built the source (a sink exists): from then on the counter is the clock.
    attached: AtomicBool,
    /// The producer is done: the source ends once the queue is drained.
    closed: AtomicBool,
    /// The player is gone: the decode thread stops waiting for room.
    stopped: AtomicBool,
    rate: u32,
    channels: u16,
}

impl AudioRing {
    pub(crate) fn new(capacity: usize, rate: u32, channels: u16) -> Self {
        let channels = channels.max(1);
        // Whole frames only, so a full ring never splits one.
        let capacity = (capacity.max(1)).div_ceil(usize::from(channels)) * usize::from(channels);
        Self {
            slots: (0..capacity).map(|_| AtomicI16::new(0)).collect(),
            write: AtomicU64::new(0),
            read: AtomicU64::new(0),
            played: AtomicU64::new(0),
            attached: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
            rate: rate.max(1),
            channels,
        }
    }

    /// A ring holding [`RING_SECONDS`] of this stream.
    pub(crate) fn for_stream(rate: u32, channels: u16) -> Self {
        Self::new(
            rate as usize * usize::from(channels) * RING_SECONDS,
            rate,
            channels,
        )
    }

    fn capacity(&self) -> u64 {
        self.slots.len() as u64
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "a position modulo the slot count"
    )]
    fn slot(&self, at: u64) -> &AtomicI16 {
        &self.slots[(at % self.capacity()) as usize]
    }

    /// Producer: queue as many whole sample frames of `samples` as fit; returns how many samples went in.
    pub(crate) fn push(&self, samples: &[i16]) -> usize {
        let write = self.write.load(Ordering::Relaxed);
        let read = self.read.load(Ordering::Acquire);
        let free = self.capacity() - (write - read);
        let ch = u64::from(self.channels);
        let n = (free.min(samples.len() as u64) / ch) * ch;
        for (i, &s) in (0u64..n).zip(samples) {
            self.slot(write + i).store(s, Ordering::Relaxed);
        }
        self.write.store(write + n, Ordering::Release);
        #[expect(clippy::cast_possible_truncation, reason = "n <= samples.len()")]
        let n = n as usize;
        n
    }

    /// Consumer: the next queued sample, if any.
    fn pop(&self) -> Option<i16> {
        let read = self.read.load(Ordering::Relaxed);
        if read == self.write.load(Ordering::Acquire) {
            return None;
        }
        let s = self.slot(read).load(Ordering::Relaxed);
        self.read.store(read + 1, Ordering::Release);
        Some(s)
    }

    /// The producer is done; the source ends after the queued samples.
    pub(crate) fn close(&self) {
        self.closed.store(true, Ordering::Release);
    }

    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    /// The player is gone (despawned or removed): the producer stops waiting for room.
    pub(crate) fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
    }

    pub(crate) fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::Acquire)
    }

    /// Whether a device is pulling from this ring (the audio clock is live).
    pub(crate) fn is_attached(&self) -> bool {
        self.attached.load(Ordering::Acquire)
    }

    /// Samples handed out so far (silence included).
    pub(crate) fn played(&self) -> u64 {
        self.played.load(Ordering::Acquire)
    }

    /// Stream time of the samples handed out, in microseconds.
    pub(crate) fn played_us(&self) -> u64 {
        samples_to_us(self.played(), self.rate, self.channels)
    }

    /// A consumer for this ring. Only the first one is real: later ones end at once (one consumer).
    pub(crate) fn consumer(self: &Arc<Self>) -> VideoAudioSource {
        let first = !self.attached.swap(true, Ordering::AcqRel);
        VideoAudioSource {
            ring: first.then(|| Arc::clone(self)),
            rate: NonZero::new(self.rate).unwrap_or(NonZero::<u32>::MIN),
            channels: NonZero::new(self.channels).unwrap_or(NonZero::<u16>::MIN),
            phase: 0,
            silence_left: 0,
        }
    }
}

/// Stream time of `samples` interleaved samples (floor), in microseconds.
pub(crate) fn samples_to_us(samples: u64, rate: u32, channels: u16) -> u64 {
    let frames = samples / u64::from(channels.max(1));
    u64::try_from(u128::from(frames) * 1_000_000 / u128::from(rate.max(1))).unwrap_or(u64::MAX)
}

/// Interleaved samples covering at least `us` microseconds (whole frames, rounded up).
pub(crate) fn us_to_samples(us: u64, rate: u32, channels: u16) -> u64 {
    let frames = (u128::from(us) * u128::from(rate)).div_ceil(1_000_000);
    u64::try_from(frames * u128::from(channels.max(1))).unwrap_or(u64::MAX)
}

/// The decode thread's end of a ring: PCM the ring had no room for waits here, in stream order, and
/// the produced total (silence padding included) gives the audio's stream end.
pub(crate) struct AudioFeed {
    ring: Arc<AudioRing>,
    pending: VecDeque<i16>,
    scratch: Vec<i16>,
    produced: u64,
}

impl AudioFeed {
    pub(crate) fn new(ring: Arc<AudioRing>) -> Self {
        Self {
            ring,
            pending: VecDeque::new(),
            scratch: Vec::new(),
            produced: 0,
        }
    }

    /// Queue `samples` (interleaved PCM of the stream) and push what fits.
    pub(crate) fn feed(&mut self, samples: &[i16]) {
        self.produced += samples.len() as u64;
        self.pending.extend(samples);
        self.flush();
    }

    /// Pad with silence so the audio covers at least `us` of stream time.
    pub(crate) fn pad_to(&mut self, us: u64) {
        let target = us_to_samples(us, self.ring.rate, self.ring.channels);
        if target > self.produced {
            let n = usize::try_from(target - self.produced).unwrap_or(usize::MAX);
            self.produced = target;
            self.pending.extend(std::iter::repeat_n(0, n));
            self.flush();
        }
    }

    /// Stream time the produced audio reaches, in microseconds.
    pub(crate) fn end_us(&self) -> u64 {
        samples_to_us(self.produced, self.ring.rate, self.ring.channels)
    }

    /// Push waiting PCM into the ring. Until a device attaches, what doesn't fit is dropped: nobody is
    /// listening, and holding it would grow without bound when no output device exists.
    pub(crate) fn flush(&mut self) {
        // One slice, so a sample frame split across the deque's wrap can't stall the push.
        let n = self.ring.push(self.pending.make_contiguous());
        self.pending.drain(..n);
        if !self.ring.is_attached() {
            self.pending.clear();
        }
    }

    /// Whether PCM is still waiting for room.
    pub(crate) fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Reusable buffer for `VideoSource::next_audio`; hand it back with [`Self::feed_scratch`].
    pub(crate) fn take_scratch(&mut self) -> Vec<i16> {
        let mut buf = std::mem::take(&mut self.scratch);
        buf.clear();
        buf
    }

    /// Feed and keep the buffer for the next call.
    pub(crate) fn feed_scratch(&mut self, buf: Vec<i16>) {
        self.feed(&buf);
        self.scratch = buf;
    }

    /// Wait (polling) until everything waiting is in the ring, or the player is gone.
    pub(crate) fn drain_blocking(&mut self) {
        while self.has_pending() && !self.ring.is_stopped() {
            std::thread::sleep(Duration::from_millis(10));
            self.flush();
        }
    }
}

impl Drop for AudioFeed {
    fn drop(&mut self) {
        self.ring.close();
    }
}

/// The audio track of one playing video, as a `bevy_audio` source asset. Made by the player; play it
/// with `AudioPlayer::<VideoAudioStream>`. Its decoder is the live stream, so build it once.
#[derive(Asset, TypePath)]
pub struct VideoAudioStream {
    ring: Arc<AudioRing>,
}

impl VideoAudioStream {
    pub(crate) fn new(ring: Arc<AudioRing>) -> Self {
        Self { ring }
    }
}

impl Decodable for VideoAudioStream {
    type Decoder = VideoAudioSource;

    fn decoder(&self) -> Self::Decoder {
        self.ring.consumer()
    }
}

/// The rodio source draining a video's [`AudioRing`] on the audio thread: `f32` samples at the
/// stream's rate and channels, silence on underrun, ends when the decode thread is done and the
/// queue is empty.
pub struct VideoAudioSource {
    /// `None` for a second consumer of the same stream (ends at once).
    ring: Option<Arc<AudioRing>>,
    rate: SampleRate,
    channels: ChannelCount,
    /// Sample index within the current frame.
    phase: u16,
    /// Silence still owed to finish a frame after an underrun.
    silence_left: u16,
}

impl Iterator for VideoAudioSource {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        let ring = self.ring.as_deref()?;
        let out = if self.silence_left > 0 {
            self.silence_left -= 1;
            0.0
        } else {
            // Empty: read the flag, then look once more (everything pushed before `close` is visible then).
            let sample = ring.pop().ok_or(()).or_else(|()| {
                let closed = ring.is_closed();
                ring.pop().ok_or(closed)
            });
            match sample {
                Ok(s) => f32::from(s) / 32768.0,
                Err(true) => {
                    self.ring = None;
                    return None;
                }
                Err(false) => {
                    self.silence_left = self.channels.get() - 1 - self.phase;
                    0.0
                }
            }
        };
        ring.played.fetch_add(1, Ordering::AcqRel);
        self.phase = (self.phase + 1) % self.channels.get();
        Some(out)
    }
}

impl Source for VideoAudioSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        self.channels
    }

    fn sample_rate(&self) -> SampleRate {
        self.rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Marks a video's audio entity (an `AudioPlayer<VideoAudioStream>` on [`sf_audio::Bus::Movie`]);
/// `owner` is the [`VideoPlayer`](crate::VideoPlayer) entity it belongs to.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoAudio {
    pub owner: Entity,
}
