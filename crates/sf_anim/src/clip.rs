//! Cooked clips: uniformly sampled, quantized joint tracks plus root motion,
//! curves, events and sync markers. The bake product of `anim.cook`.
//!
//! **Layout** (the ACL design, Frechette): every frame is a fixed-size,
//! bit-packed record holding one key per animated track, frames back to
//! back. Sampling at `t` reads two records and interpolates, with no key
//! search and no per-character cache (ozz's sampling cache exists only
//! because its keys are not uniform). Channels that never move are not
//! stored per frame: they live in a constant pose copied into the output
//! first.
//!
//! **Quantization** (per track, chosen at cook against an error budget):
//! - Rotations: ACL's "drop a component" with range reduction. The track's
//!   quaternions are made sign-continuous, one component that never nears
//!   zero is dropped (ACL always drops W; we pick the safest of the four,
//!   which keeps tracks that swing through 180° quantizable), and the other
//!   three are stored over the track's own [min, max] range with 1–24 bits.
//!   The dropped one is rebuilt as `sqrt(1 − Σk²)`. Tracks no component
//!   suits are stored raw (4 × f32).
//! - Translations and scales: per-axis [min, max] range, 1–24 bits, or
//!   raw f32 when the range is too wide for 24 bits.
//!
//! Range reduction is what makes 16 bits or fewer enough: a joint that
//! swings 30° spends its bits on that 30°, not on the whole sphere.

use std::sync::OnceLock;

use glam::{Quat, Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate::{FormatError, JointMask, LocalPose, SkeletonId, SoaTrs, Trs, math};

/// How a clip's root motion moves the character (animation.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootMotionMode {
    /// The root delta moves the character (attacks, rolls, climbs).
    Drive,
    /// The character is capsule-driven; playback rate follows its speed.
    #[default]
    Match,
    /// In place.
    Ignore,
}

/// A point event (`end == None`) or a window `[time, end]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub time: f32,
    pub end: Option<f32>,
    pub name: String,
    pub payload: String,
}

impl Event {
    pub fn point(time: f32, name: impl Into<String>) -> Self {
        Self { time, end: None, name: name.into(), payload: String::new() }
    }

    pub fn window(start: f32, end: f32, name: impl Into<String>) -> Self {
        Self { time: start, end: Some(end), name: name.into(), payload: String::new() }
    }
}

/// A named phase point (e.g. a foot touching down), for sync groups.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncMarker {
    pub time: f32,
    pub name: String,
}

/// A float curve sampled at the clip's rate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveTrack {
    pub name: String,
    pub values: Vec<f32>,
}

/// Root motion relative to frame 0, one entry per frame: model-space
/// position `[x, y, z]` of the character root and its heading `yaw`
/// (radians about +Y, unwrapped so it interpolates).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootMotionTrack {
    pub frames: Vec<[f32; 4]>,
    /// Per frame, the root's attitude in its heading frame: `[pitch, roll]`
    /// (radians about the heading frame's X, then Z; YXZ Euler order).
    /// Empty unless extracted (`RootMotionRules::attitude`: flyers,
    /// swimmers); ground creatures keep yaw only.
    pub attitude: Vec<[f32; 2]>,
}

/// Motion of the character root over an interval, in the root's frame at
/// the start of the interval.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RootDelta {
    pub translation: Vec3,
    /// Radians about +Y.
    pub yaw: f32,
    /// Change of attitude `(pitch, roll)` in the heading frame, radians
    /// (zero for clips without an attitude track).
    pub tilt: Vec2,
}

impl RootDelta {
    pub const ZERO: Self = Self { translation: Vec3::ZERO, yaw: 0.0, tilt: Vec2::ZERO };

    /// A ground delta: translation and yaw.
    pub fn new(translation: Vec3, yaw: f32) -> Self {
        Self { translation, yaw, tilt: Vec2::ZERO }
    }

    /// `self` followed by `next`.
    #[must_use]
    pub fn then(self, next: Self) -> Self {
        Self {
            translation: self.translation + Quat::from_rotation_y(self.yaw) * next.translation,
            yaw: self.yaw + next.yaw,
            tilt: self.tilt + next.tilt,
        }
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }
}

/// The rotation of a heading-frame attitude `(pitch, roll)`.
pub fn attitude_rotation(attitude: Vec2) -> Quat {
    Quat::from_euler(glam::EulerRot::YXZ, 0.0, attitude.x, attitude.y)
}

/// Cook-time facts about a clip.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClipMeta {
    pub root_motion_mode: RootMotionMode,
    /// Stored as a delta over a base pose: play with [`crate::apply_additive`].
    pub additive: bool,
    /// Largest virtual-vertex error measured against the source, meters.
    pub max_error: f32,
    /// Mean root velocity over the loop range (or the clip), model space,
    /// m/s. For in-place clips it is estimated from planted feet.
    pub average_velocity: Vec3,
    /// Mean turn rate, radians per second.
    pub turn_rate: f32,
    /// Root displacement over one loop (or the clip), meters.
    pub cycle_distance: f32,
    /// Source tracks naming joints the skeleton lacks.
    pub unbound_joints: Vec<String>,
}

/// Bit width marking a raw (f32) track.
pub(crate) const RAW: u8 = 32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RotTrack {
    pub joint: u16,
    /// 1–24 bits per stored component, or [`RAW`] for 4 × f32.
    pub bits: u8,
    /// The dropped component (0–3 = x, y, z, w).
    pub drop: u8,
    /// Bit offset inside a frame record.
    pub offset: u32,
    pub min: [f32; 3],
    /// Range / (2^bits − 1).
    pub step: [f32; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum VecChannel {
    Translation,
    Scale,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct VecTrack {
    pub joint: u16,
    pub channel: VecChannel,
    /// 1–24 bits per component, or [`RAW`] for 3 × f32.
    pub bits: u8,
    pub offset: u32,
    pub min: [f32; 3],
    pub step: [f32; 3],
}

/// A cooked clip.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clip {
    pub name: String,
    pub skeleton: SkeletonId,
    pub(crate) joint_count: u16,
    pub duration: f32,
    /// Samples per second; the last frame lands exactly on `duration`.
    pub rate: f32,
    pub frame_count: u32,
    /// Per joint: the value of every channel that has no track.
    pub(crate) constant: Vec<Trs>,
    pub(crate) rotations: Vec<RotTrack>,
    pub(crate) vectors: Vec<VecTrack>,
    pub(crate) frame_bytes: u32,
    /// `frame_count × frame_bytes`, plus 8 bytes of padding for word reads.
    pub(crate) data: Vec<u8>,
    pub root_motion: Option<RootMotionTrack>,
    pub curves: Vec<CurveTrack>,
    /// Sorted by time.
    pub events: Vec<Event>,
    /// Sorted by time.
    pub markers: Vec<SyncMarker>,
    /// The looping section (`[0, duration]` for a plain loop); `None` if the
    /// clip doesn't loop by default.
    pub loop_range: Option<(f32, f32)>,
    pub meta: ClipMeta,
    #[serde(skip)]
    constant_soa: OnceLock<Vec<SoaTrs>>,
}

/// Which edge of an event a playhead crossed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventEdge {
    /// A point event.
    Point,
    /// Entered a window.
    Enter,
    /// Left a window.
    Exit,
}

/// An event crossed during [`Clip::advance`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiredEvent {
    /// Index into [`Clip::events`].
    pub event: u32,
    pub edge: EventEdge,
}

/// The result of [`Clip::advance`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Advance {
    /// New playhead, seconds.
    pub time: f32,
    /// Loop wraps crossed (either direction).
    pub wraps: u32,
    /// Root motion over the update, in the root's frame at its start.
    pub root: RootDelta,
    /// A non-looping playhead reached the end (or the start, in reverse).
    pub finished: bool,
}

const MAGIC: &[u8; 4] = b"SFAN";
/// Version of the cooked clip format.
pub const CLIP_FORMAT: u32 = 2;

impl Clip {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        name: String,
        skeleton: SkeletonId,
        constant: Vec<Trs>,
        duration: f32,
        rate: f32,
        frame_count: u32,
        rotations: Vec<RotTrack>,
        vectors: Vec<VecTrack>,
        frame_bytes: u32,
        data: Vec<u8>,
    ) -> Self {
        Self {
            name,
            skeleton,
            joint_count: u16::try_from(constant.len()).expect("joint count fits u16"),
            duration,
            rate,
            frame_count,
            constant,
            rotations,
            vectors,
            frame_bytes,
            data,
            root_motion: None,
            curves: Vec::new(),
            events: Vec::new(),
            markers: Vec::new(),
            loop_range: None,
            meta: ClipMeta::default(),
            constant_soa: OnceLock::new(),
        }
    }

    pub fn joint_count(&self) -> usize {
        usize::from(self.joint_count)
    }

    /// Bytes of per-frame track data (the part that scales with length).
    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    /// Number of animated (stored per frame) rotation, translation and
    /// scale tracks, and how many of them are raw.
    pub fn track_counts(&self) -> (usize, usize, usize, usize) {
        let t = self.vectors.iter().filter(|v| v.channel == VecChannel::Translation).count();
        let raw = self.rotations.iter().filter(|r| r.bits == RAW).count() + self.vectors.iter().filter(|v| v.bits == RAW).count();
        (self.rotations.len(), t, self.vectors.len() - t, raw)
    }

    fn constant_soa(&self) -> &[SoaTrs] {
        self.constant_soa.get_or_init(|| LocalPose::from_trs(&self.constant).groups().to_vec())
    }

    /// Frame indices around `time` and the blend factor between them.
    #[inline]
    pub fn frame_pair(&self, time: f32) -> (usize, usize, f32) {
        if self.frame_count < 2 {
            return (0, 0, 0.0);
        }
        let last = self.frame_count as usize - 1;
        let x = time.clamp(0.0, self.duration) * self.rate;
        // Truncation is the floor here: x ≥ 0.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let i = (x as usize).min(last - 1);
        #[allow(clippy::cast_precision_loss)]
        let alpha = (x - i as f32).clamp(0.0, 1.0);
        (i, i + 1, alpha)
    }

    /// The local pose at `time` (clamped to the clip). With a `mask`, only
    /// joints in it are sampled; the others get the constant pose.
    pub fn sample(&self, time: f32, out: &mut LocalPose, mask: Option<&JointMask>) {
        let constant = self.constant_soa();
        if out.len() != self.joint_count() {
            *out = LocalPose::new(self.joint_count());
        }
        out.groups_mut().copy_from_slice(constant);
        if self.frame_count == 0 {
            return;
        }
        let (f0, f1, alpha) = self.frame_pair(time);
        let stride = self.frame_bytes as usize * 8;
        let (b0, b1) = (f0 * stride, f1 * stride);
        let data = &self.data;
        let skip = |joint: u16| mask.is_some_and(|m| !m.contains(usize::from(joint)));
        for tr in &self.rotations {
            if skip(tr.joint) {
                continue;
            }
            let q0 = decode_rotation(data, b0, tr);
            let q1 = decode_rotation(data, b1, tr);
            out.set_rotation(usize::from(tr.joint), math::nlerp(q0, q1, alpha));
        }
        for tr in &self.vectors {
            if skip(tr.joint) {
                continue;
            }
            let v = if tr.bits == RAW {
                let v0 = read_raw3(data, b0 + tr.offset as usize);
                let v1 = read_raw3(data, b1 + tr.offset as usize);
                v0.lerp(v1, alpha)
            } else {
                // Lerp in quantized space, then dequantize once.
                let q0 = read_int3(data, b0 + tr.offset as usize, tr.bits);
                let q1 = read_int3(data, b1 + tr.offset as usize, tr.bits);
                Vec3::from(tr.min) + Vec3::from(tr.step) * q0.lerp(q1, alpha)
            };
            match tr.channel {
                VecChannel::Translation => out.set_translation(usize::from(tr.joint), v),
                VecChannel::Scale => out.set_scale(usize::from(tr.joint), v),
            }
        }
    }

    /// Root motion position and yaw at `time`, relative to frame 0.
    pub fn root_at(&self, time: f32) -> (Vec3, f32) {
        let Some(rm) = &self.root_motion else {
            return (Vec3::ZERO, 0.0);
        };
        let (f0, f1, a) = self.frame_pair(time);
        let (p, q) = (rm.frames[f0], rm.frames[f1]);
        let lerp = |i: usize| p[i] + (q[i] - p[i]) * a;
        (Vec3::new(lerp(0), lerp(1), lerp(2)), lerp(3))
    }

    /// Root attitude `(pitch, roll)` at `time` (zero without an attitude track).
    pub fn attitude_at(&self, time: f32) -> Vec2 {
        let Some(rm) = &self.root_motion else {
            return Vec2::ZERO;
        };
        if rm.attitude.len() != rm.frames.len() || rm.attitude.is_empty() {
            return Vec2::ZERO;
        }
        let (f0, f1, a) = self.frame_pair(time);
        Vec2::from(rm.attitude[f0]).lerp(Vec2::from(rm.attitude[f1]), a)
    }

    /// Root motion from `t0` to `t1` (no wrap), in the root's frame at `t0`.
    pub fn root_delta(&self, t0: f32, t1: f32) -> RootDelta {
        if self.root_motion.is_none() {
            return RootDelta::ZERO;
        }
        let (p0, y0) = self.root_at(t0);
        let (p1, y1) = self.root_at(t1);
        RootDelta { translation: Quat::from_rotation_y(-y0) * (p1 - p0), yaw: y1 - y0, tilt: self.attitude_at(t1) - self.attitude_at(t0) }
    }

    /// Value of curve `index` at `time`.
    pub fn curve(&self, index: usize, time: f32) -> f32 {
        let values = &self.curves[index].values;
        let (f0, f1, a) = self.frame_pair(time);
        match values.len() {
            0 => 0.0,
            1 => values[0],
            _ => values[f0] + (values[f1] - values[f0]) * a,
        }
    }

    pub fn find_curve(&self, name: &str) -> Option<usize> {
        self.curves.iter().position(|c| c.name == name)
    }

    /// Windows containing `time`.
    pub fn active_windows(&self, time: f32) -> impl Iterator<Item = usize> + '_ {
        self.events.iter().enumerate().filter(move |(_, e)| e.end.is_some_and(|end| e.time <= time && time <= end)).map(|(i, _)| i)
    }

    /// The markers around `time` and the phase between them, in a loop.
    /// `None` without markers.
    pub fn marker_phase(&self, time: f32) -> Option<(usize, usize, f32)> {
        let n = self.markers.len();
        if n == 0 {
            return None;
        }
        let next = self.markers.partition_point(|m| m.time <= time) % n;
        let prev = (next + n - 1) % n;
        let (t0, t1) = (self.markers[prev].time, self.markers[next].time);
        let span = if t1 > t0 { t1 - t0 } else { t1 - t0 + self.duration };
        let since = (time - t0).rem_euclid(self.duration.max(f32::EPSILON));
        Some((prev, next, if span > 0.0 { (since / span).clamp(0.0, 1.0) } else { 0.0 }))
    }

    /// Moves a playhead from `time` by `delta` seconds of clip time
    /// (negative plays in reverse), wrapping inside the loop range when
    /// `looping`. Collects the events crossed and the root motion.
    ///
    /// A point event fires when the playhead crosses its time in the play
    /// direction, once per crossing, including across a wrap. Pass `first`
    /// on the update that starts playback so events exactly at `time` fire.
    /// An update spanning more than a whole loop (a hitch) fires each event
    /// at most once per loop crossed in full, and only the partial loops'
    /// events; root motion still accounts for every loop.
    pub fn advance(&self, time: f32, delta: f32, looping: bool, first: bool, mut events: Option<&mut Vec<FiredEvent>>) -> Advance {
        let d = self.duration;
        let (a, b) = self.loop_range.unwrap_or((0.0, d));
        let span = b - a;
        let looping = looping && span > 1e-6;
        let mut t = time.clamp(0.0, d);
        let mut remaining = delta.abs();
        let forward = delta >= 0.0;
        let mut root = RootDelta::ZERO;
        let mut wraps = 0;
        let mut inclusive = first;
        let mut fire = |t0: f32, t1: f32, inclusive: bool| {
            if let Some(out) = events.as_deref_mut() {
                self.collect_events(t0, t1, inclusive, out);
            }
        };
        loop {
            if forward {
                // An intro before the loop plays into it; an outro after it
                // (reached only by seeking) runs to the end.
                let wraps_here = looping && t <= b;
                let end = if wraps_here { b } else { d };
                let target = t + remaining;
                if target <= end || !wraps_here {
                    let t1 = target.min(d);
                    fire(t, t1, inclusive);
                    root = root.then(self.root_delta(t, t1));
                    let finished = !wraps_here && target >= d;
                    return Advance { time: t1, wraps, root, finished };
                }
                fire(t, end, inclusive);
                root = root.then(self.root_delta(t, end));
                remaining -= end - t;
                t = a;
            } else {
                let wraps_here = looping && t >= a;
                let start = if wraps_here { a } else { 0.0 };
                let target = t - remaining;
                if target >= start || !wraps_here {
                    let t1 = target.max(0.0);
                    fire(t, t1, inclusive);
                    root = root.then(self.root_delta(t, t1));
                    let finished = !wraps_here && target <= 0.0;
                    return Advance { time: t1, wraps, root, finished };
                }
                fire(t, start, inclusive);
                root = root.then(self.root_delta(t, start));
                remaining -= t - start;
                t = b;
            }
            wraps += 1;
            inclusive = true;
            if remaining > span {
                // A hitch: whole loops move the root but fire nothing.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let skipped = (remaining / span).floor() as u32;
                let cycle = if forward { self.root_delta(a, b) } else { self.root_delta(b, a) };
                for _ in 0..skipped.min(64) {
                    root = root.then(cycle);
                }
                #[allow(clippy::cast_precision_loss)]
                {
                    remaining -= skipped as f32 * span;
                }
                wraps += skipped;
            }
        }
    }

    /// Events crossed moving from `t0` to `t1` (either direction, no wrap).
    /// The interval excludes `t0` unless `inclusive`, and includes `t1`.
    #[allow(clippy::float_cmp)] // Exact: an event at exactly the start time.
    pub fn collect_events(&self, t0: f32, t1: f32, inclusive: bool, out: &mut Vec<FiredEvent>) {
        let forward = t1 >= t0;
        let crossed = |x: f32| {
            if forward { (x > t0 || (inclusive && x == t0)) && x <= t1 } else { (x < t0 || (inclusive && x == t0)) && x >= t1 }
        };
        let mut push = |i: usize, edge| out.push(FiredEvent { event: u32::try_from(i).expect("event count fits u32"), edge });
        // Events are sorted by time; emit in play order.
        let order: Box<dyn Iterator<Item = usize>> = if forward { Box::new(0..self.events.len()) } else { Box::new((0..self.events.len()).rev()) };
        for i in order {
            let e = &self.events[i];
            match e.end {
                None => {
                    if crossed(e.time) {
                        push(i, EventEdge::Point);
                    }
                }
                Some(end) => {
                    let (enter, exit) = if forward { (e.time, end) } else { (end, e.time) };
                    if crossed(enter) {
                        push(i, EventEdge::Enter);
                    }
                    if crossed(exit) {
                        push(i, EventEdge::Exit);
                    }
                }
            }
        }
    }

    /// The cooked form: magic, format version, postcard.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.extend(CLIP_FORMAT.to_le_bytes());
        out.extend(postcard::to_stdvec(self).expect("in-memory serialization"));
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        let body = crate::check_header(bytes, *MAGIC, CLIP_FORMAT)?;
        let clip: Self = postcard::from_bytes(body).map_err(|e| FormatError::Decode(e.to_string()))?;
        clip.validate()?;
        Ok(clip)
    }

    fn validate(&self) -> Result<(), FormatError> {
        let bad = |m: &str| Err(FormatError::Decode(m.to_owned()));
        let n = self.joint_count();
        if self.constant.len() != n {
            return bad("constant pose size");
        }
        let frames = self.frame_count as usize;
        if self.data.len() != frames * self.frame_bytes as usize + 8 {
            return bad("frame data size");
        }
        let bits_ok = |bits: u8| (1..=24).contains(&bits) || bits == RAW;
        let frame_bits = self.frame_bytes as usize * 8;
        for r in &self.rotations {
            let size = if r.bits == RAW { 128 } else { 3 * usize::from(r.bits) };
            if usize::from(r.joint) >= n || !bits_ok(r.bits) || r.drop > 3 || r.offset as usize + size > frame_bits {
                return bad("rotation track");
            }
        }
        for v in &self.vectors {
            let size = if v.bits == RAW { 96 } else { 3 * usize::from(v.bits) };
            if usize::from(v.joint) >= n || !bits_ok(v.bits) || v.offset as usize + size > frame_bits {
                return bad("vector track");
            }
        }
        if self.root_motion.as_ref().is_some_and(|r| r.frames.len() != frames || !(r.attitude.is_empty() || r.attitude.len() == frames))
            || self.curves.iter().any(|c| c.values.len() != frames)
        {
            return bad("per-frame track length");
        }
        Ok(())
    }
}

// --- Bit-packed reads --------------------------------------------------------

#[inline]
pub(crate) fn read_bits(data: &[u8], bit: usize, bits: u8) -> u32 {
    let byte = bit >> 3;
    let word = u64::from_le_bytes(data[byte..byte + 8].try_into().expect("8 bytes"));
    let mask = (1u64 << bits) - 1;
    // `bits ≤ 32`, so the masked value fits.
    #[allow(clippy::cast_possible_truncation)]
    let v = ((word >> (bit & 7)) & mask) as u32;
    v
}

#[inline]
#[allow(clippy::cast_precision_loss)]
fn read_int3(data: &[u8], bit: usize, bits: u8) -> Vec3 {
    let b = usize::from(bits);
    Vec3::new(read_bits(data, bit, bits) as f32, read_bits(data, bit + b, bits) as f32, read_bits(data, bit + 2 * b, bits) as f32)
}

#[inline]
fn read_raw3(data: &[u8], bit: usize) -> Vec3 {
    Vec3::new(
        f32::from_bits(read_bits(data, bit, RAW)),
        f32::from_bits(read_bits(data, bit + 32, RAW)),
        f32::from_bits(read_bits(data, bit + 64, RAW)),
    )
}

#[inline]
pub(crate) fn decode_rotation(data: &[u8], frame_bit: usize, tr: &RotTrack) -> Quat {
    let bit = frame_bit + tr.offset as usize;
    if tr.bits == RAW {
        let v = read_raw3(data, bit);
        let w = f32::from_bits(read_bits(data, bit + 96, RAW));
        return Quat::from_xyzw(v.x, v.y, v.z, w);
    }
    let k = Vec3::from(tr.min) + Vec3::from(tr.step) * read_int3(data, bit, tr.bits);
    let dropped = (1.0 - k.length_squared()).max(0.0).sqrt();
    rebuild(k, dropped, tr.drop)
}

/// Reassembles a quaternion from its three kept components (in x, y, z, w
/// order without `drop`) and the dropped one.
#[inline]
pub(crate) fn rebuild(k: Vec3, dropped: f32, drop: u8) -> Quat {
    match drop {
        0 => Quat::from_xyzw(dropped, k.x, k.y, k.z),
        1 => Quat::from_xyzw(k.x, dropped, k.y, k.z),
        2 => Quat::from_xyzw(k.x, k.y, dropped, k.z),
        _ => Quat::from_xyzw(k.x, k.y, k.z, dropped),
    }
}
