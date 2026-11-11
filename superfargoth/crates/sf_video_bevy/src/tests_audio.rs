//! The audio track's parts (video.md "Build plan" V3a), headless: `bevy_audio` has no device in tests,
//! so the ring, the source the device would pull and the clock choice are tested directly.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::time::Duration;

use super::audio::{AudioFeed, AudioRing, samples_to_us, us_to_samples};
use super::{Msg, Playback, drain_due};

fn s(v: i16) -> f32 {
    f32::from(v) / 32768.0
}

#[test]
fn ring_counts_every_sample_handed_out() {
    let ring = Arc::new(AudioRing::new(8, 1000, 2));
    assert_eq!(ring.push(&[1, 2, 3, 4, 5, 6]), 6);
    assert!(!ring.is_attached());
    let mut src = ring.consumer();
    assert!(ring.is_attached());
    let pulled: Vec<f32> = src.by_ref().take(4).collect();
    assert_eq!(pulled, [s(1), s(2), s(3), s(4)]);
    assert_eq!(ring.played(), 4);
    // 2 queued, 6 free; 7 offered: whole stereo frames only.
    assert_eq!(ring.push(&[7, 8, 9, 10, 11, 12, 13]), 6);
    assert_eq!(ring.push(&[14, 15]), 0, "full");
    let pulled: Vec<f32> = src.by_ref().take(8).collect();
    assert_eq!(pulled, [5, 6, 7, 8, 9, 10, 11, 12].map(s));
    assert_eq!(ring.played(), 12);
    assert_eq!(ring.played_us(), 6000, "6 stereo frames at 1 kHz");
}

#[test]
fn underrun_yields_whole_frames_of_silence() {
    let ring = Arc::new(AudioRing::new(16, 1000, 2));
    let mut src = ring.consumer();
    assert_eq!(src.next(), Some(0.0), "empty: silence, not the end");
    // The frame finishes in silence even if data lands mid-frame, so channels never swap.
    ring.push(&[3, 4]);
    assert_eq!(src.next(), Some(0.0));
    assert_eq!(src.next(), Some(s(3)));
    assert_eq!(src.next(), Some(s(4)));
    assert_eq!(
        ring.played(),
        4,
        "silence counts: the clock follows the device"
    );
}

#[test]
fn a_closed_ring_ends_after_its_queue() {
    let ring = Arc::new(AudioRing::new(16, 1000, 1));
    let mut src = ring.consumer();
    ring.push(&[1, 2]);
    ring.close();
    assert_eq!(src.by_ref().collect::<Vec<_>>(), [s(1), s(2)]);
    assert_eq!(src.next(), None);
    assert_eq!(ring.played(), 2);
    assert_eq!(ring.consumer().next(), None, "one consumer per stream");
}

#[test]
fn feed_pads_and_drops_overflow_until_attached() {
    let ring = Arc::new(AudioRing::new(4, 1000, 1));
    let mut feed = AudioFeed::new(Arc::clone(&ring));
    feed.feed(&[1, 2, 3, 4, 5, 6]);
    assert!(!feed.has_pending(), "no listener yet: overflow dropped");
    assert_eq!(
        feed.end_us(),
        6000,
        "dropped samples still advance the stream end"
    );
    let _src = ring.consumer();
    feed.feed(&[7]);
    assert!(feed.has_pending(), "attached: kept until there is room");
    feed.pad_to(10_000);
    assert_eq!(feed.end_us(), 10_000);
    feed.pad_to(5_000);
    assert_eq!(feed.end_us(), 10_000, "padding never shortens");
    drop(feed);
    let mut src = ring.consumer();
    assert_eq!(src.next(), None, "a second consumer ends at once");
    assert_eq!(us_to_samples(1, 44_100, 2), 2, "rounds up to a whole frame");
    assert_eq!(samples_to_us(88_200, 44_100, 2), 1_000_000);
}

/// A playback over a channel holding frames at 0, 100 and 200 ms, then the end at 300 ms.
fn playback() -> Playback {
    let (tx, rx) = mpsc::sync_channel(8);
    let (recycle, _recycled) = mpsc::sync_channel(8);
    for index in 0..3u32 {
        let time_us = u64::from(index) * 100_000;
        tx.send(Msg::Frame {
            index,
            time_us,
            rgba: Vec::new(),
        })
        .unwrap();
    }
    tx.send(Msg::End { time_us: 300_000 }).unwrap();
    Playback {
        rx: Mutex::new(rx),
        recycle,
        pending: None,
        clock_start: None,
        paused_at: None,
    }
}

#[test]
fn with_audio_the_due_frame_follows_the_sample_counter() {
    let ring = Arc::new(AudioRing::new(1000, 1000, 1));
    let mut src = ring.consumer();
    let mut pb = playback();
    // Real time is far ahead; the audio clock (0 samples) holds the video at frame 0.
    let late = Duration::from_secs(60);
    let (due, end) = drain_due(&mut pb, late, Some(ring.played_us()));
    assert_eq!(due.map(|(i, _)| i), Some(0));
    assert!(end.is_none());
    // 150 samples at 1 kHz: 150 ms, frame 1 is due, frame 2 waits.
    src.by_ref().take(150).for_each(drop);
    let (due, end) = drain_due(&mut pb, late, Some(ring.played_us()));
    assert_eq!(due.map(|(i, _)| i), Some(1));
    assert!(end.is_none());
    src.by_ref().take(150).for_each(drop);
    let (due, end) = drain_due(&mut pb, late, Some(ring.played_us()));
    assert_eq!(
        due.map(|(i, _)| i),
        Some(2),
        "300 ms: frame 2, then the end"
    );
    assert_eq!(end, Some(Ok(())));
}

#[test]
fn without_audio_the_real_clock_drives() {
    let mut pb = playback();
    let t0 = Duration::from_secs(5);
    let (due, _) = drain_due(&mut pb, t0, None);
    assert_eq!(due.map(|(i, _)| i), Some(0));
    let (due, end) = drain_due(&mut pb, t0 + Duration::from_millis(250), None);
    assert_eq!(due.map(|(i, _)| i), Some(2), "frame 1 dropped as late");
    assert!(end.is_none());
}
