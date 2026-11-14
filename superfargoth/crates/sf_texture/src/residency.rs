//! Texture residency planner (textures.md §6 "Part B: residency", slice S3a).
//!
//! Decides which mip levels of each streamable texture should be resident, from where the textures
//! are seen and a memory pool. Bevy-free and deterministic: [`Residency::plan`] is a function of its
//! inputs, the time and the hysteresis state the caller owns (the [`Residency`] value); the caller
//! runs it at its own cadence (4 Hz in the spec) and executes the request list (S4).
//!
//! Level numbers are the *file's* levels: 0 is the full-size level of the cooked KTX2, before any
//! load-time cap ([`crate::level_cap`]). A texture whose first resident level is `k` holds levels
//! `k..` (a mip chain is always resident down to its last level).
//!
//! Rules (UE texture streaming's wanted-mip rule and budget, Unity mipmap streaming's budget):
//! - **Wanted level** per demand, the texel-per-pixel rule: `px_per_m = H_out / (2·d·tan(fov_y/2))`,
//!   `texels_per_m = ρ·W` (`W` = larger side of level 0), `level = floor(log2(texels_per_m /
//!   px_per_m)) + bias`, then clamped to the texture's start levels. A texture wants the finest
//!   level any of its demands wants; with no demand it wants its coarsest.
//! - **Start levels**: never finer than the load-time cap's first level, never coarser than the
//!   *mip tail* (first level ≤ [`TAIL_SIZE`]², always resident), only block-aligned levels (wgpu needs
//!   whole blocks at level 0). A texture whose capped level 0 is at most [`STREAMABLE_ABOVE`]² is not
//!   streamed: it has one start level.
//! - **Hysteresis** (state in [`Residency`]): an upgrade waits until the wanted level has stayed finer
//!   for [`Settings::upgrade_stable_s`] (to the coarsest level wanted over that window), a downgrade
//!   until it has stayed coarser for [`Settings::downgrade_wait_s`] (unwanted levels stay cached
//!   meanwhile). A texture seen for the first time takes its wanted level at once.
//! - **Pool**: targets start at the hysteretic wanted level; while their bytes exceed the pool, the
//!   texture outside the reveal radius with the most levels finer than currently wanted (cached
//!   levels first, then the loss spreads one level at a time), then the least recently wanted, then
//!   the most bytes freed, is raised one start level. Only then are textures in the reveal radius
//!   raised, farthest first, and [`Plan::pool_warning`] is set once per episode (the pool is too
//!   small for the preset).
//! - **Requests**: every texture whose target differs from its resident level, downgrades first
//!   (most bytes freed first), then upgrades (reveal radius first, then the largest visible deficit,
//!   then the nearest). Ties always end on the id, so no output depends on input or map order.

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};

use crate::level_cap::{Layout, level_size};

/// A texture is streamed only when its (capped) level 0's larger side is above this.
pub const STREAMABLE_ABOVE: u32 = 512;
/// Levels whose larger side is at most this are the mip tail: always resident.
pub const TAIL_SIZE: u32 = 128;
/// Default upgrade hysteresis: the wanted level must stay finer this long (seconds).
pub const UPGRADE_STABLE_S: f64 = 0.5;
/// Default downgrade hysteresis: the wanted level must stay coarser this long (seconds).
pub const DOWNGRADE_WAIT_S: f64 = 5.0;
/// UV units per metre assumed by a demand with no density (Unity/UE fallback: conservative).
pub const FALLBACK_RHO: f32 = 1.0;

/// A texture, as the caller names it (unique per plan).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureId(pub u64);

/// A texture's level sizes and the levels it may start at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextureLevels {
    /// Larger side of level 0 (texels).
    top: u32,
    /// `resident[k]`: GPU bytes of levels `k..` (one extra 0 at the end).
    resident: Vec<u64>,
    /// Allowed first resident levels, finest first; never empty.
    starts: Vec<u32>,
}

impl TextureLevels {
    /// A texture of `width`×`height` texels at level 0, texel blocks of `block`, GPU bytes per
    /// level in `level_bytes` (level 0 first; not empty) and `finest` the first level the load-time
    /// cap keeps ([`crate::level_cap::CapPlan::skip`]; 0 when uncapped).
    pub fn new(width: u32, height: u32, block: (u32, u32), level_bytes: &[u64], finest: u32) -> Self {
        assert!(!level_bytes.is_empty(), "a texture has at least one level");
        let last = u32::try_from(level_bytes.len() - 1).unwrap_or(u32::MAX);
        let (width, height) = (width.max(1), height.max(1));
        let finest = finest.min(last);
        let side = |k: u32| level_size(width, k).max(level_size(height, k));
        let starts = if side(finest) <= STREAMABLE_ABOVE {
            vec![finest]
        } else {
            let tail = (finest..=last).find(|&k| side(k) <= TAIL_SIZE).unwrap_or(last);
            let aligned = |k: u32| {
                level_size(width, k).is_multiple_of(block.0.max(1))
                    && level_size(height, k).is_multiple_of(block.1.max(1))
            };
            (finest..=tail).filter(|&k| k == finest || aligned(k)).collect()
        };
        let mut resident = vec![0; level_bytes.len() + 1];
        for k in (0..level_bytes.len()).rev() {
            resident[k] = resident[k + 1] + level_bytes[k];
        }
        Self { top: width.max(height), resident, starts }
    }

    /// From a KTX2 header and level index, under the load-time `cap` (S1's [`Layout::plan`]).
    /// A file the cap can't trim (unknown block size) is not streamed.
    pub fn from_layout(layout: &Layout, cap: Option<u32>) -> Self {
        let bytes: Vec<u64> = layout.levels.iter().map(|l| l.uncompressed_byte_length).collect();
        let bytes = if bytes.is_empty() { vec![0] } else { bytes };
        let (width, height) = (layout.header.pixel_width, layout.header.pixel_height);
        let plan = layout.plan(cap);
        if let Ok((block, _)) = layout.block {
            Self::new(width, height, block, &bytes, plan.skip)
        } else {
            let mut whole = Self::new(width, height, (1, 1), &bytes, plan.skip);
            whole.starts.truncate(1);
            whole
        }
    }

    /// Finest allowed start level (the cap's first level).
    pub fn finest(&self) -> u32 {
        self.starts[0]
    }

    /// Coarsest allowed start level (the mip tail).
    pub fn coarsest(&self) -> u32 {
        self.starts[self.starts.len() - 1]
    }

    /// Allowed start levels, finest first.
    pub fn starts(&self) -> &[u32] {
        &self.starts
    }

    /// Whether the planner ever moves this texture.
    pub fn is_streamable(&self) -> bool {
        self.starts.len() > 1
    }

    /// GPU bytes resident when `level` is the first resident level.
    pub fn bytes_at(&self, level: u32) -> u64 {
        let k = (level as usize).min(self.resident.len() - 1);
        self.resident[k]
    }

    /// The coarsest allowed start level at least as fine as `level` (the finest when `level` is
    /// finer than every start).
    pub fn snap(&self, level: u32) -> u32 {
        match self.starts.partition_point(|&s| s <= level) {
            0 => self.starts[0],
            i => self.starts[i - 1],
        }
    }

    /// The next allowed start level coarser than `level`.
    fn coarser(&self, level: u32) -> Option<u32> {
        self.starts.get(self.starts.partition_point(|&s| s <= level)).copied()
    }
}

/// One place a texture is drawn from (a cluster of the static scene, an actor, an item).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Demand {
    /// Effective distance from the camera to the demand's bounding sphere, metres (0 inside;
    /// [`effective_distance`]).
    pub distance: f32,
    /// UV units per metre of the mesh drawing it (`mesh.cook` UV density); `None` uses
    /// [`View::fallback_rho`].
    pub rho: Option<f32>,
    /// Inside the reveal radius: dropped last under pool pressure, counted by the reveal gate.
    pub in_reveal_radius: bool,
}

/// One texture's input to a plan.
#[derive(Clone, Copy, Debug)]
pub struct TextureInput<'a> {
    pub id: TextureId,
    pub levels: &'a TextureLevels,
    /// First resident level now.
    pub resident: u32,
    pub demands: &'a [Demand],
}

/// The view the wanted levels are computed for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    /// Output height in pixels (DLSS samples at output resolution with a negative view mip bias).
    pub h_out: f32,
    /// Vertical field of view, radians.
    pub fov_y: f32,
    /// Preset bias added to every wanted level (positive: coarser).
    pub bias: i32,
    /// ρ of a demand with no UV density.
    pub fallback_rho: f32,
}

/// Pool and hysteresis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// GPU bytes all planned textures may hold together.
    pub pool_bytes: u64,
    pub upgrade_stable_s: f64,
    pub downgrade_wait_s: f64,
}

impl Settings {
    /// `pool_bytes` with the default hysteresis.
    pub const fn with_pool(pool_bytes: u64) -> Self {
        Self { pool_bytes, upgrade_stable_s: UPGRADE_STABLE_S, downgrade_wait_s: DOWNGRADE_WAIT_S }
    }
}

/// One level change for the streamer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub id: TextureId,
    /// First resident level now.
    pub from: u32,
    /// First resident level wanted.
    pub to: u32,
}

impl Request {
    /// Coarser target: frees memory.
    pub fn is_downgrade(&self) -> bool {
        self.to > self.from
    }
}

/// Totals of one plan.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counters {
    pub textures: u32,
    /// Textures with more than one start level.
    pub streamable: u32,
    pub pool_bytes: u64,
    /// Bytes resident now, at the hysteretic wanted levels, and at the targets.
    pub bytes_resident: u64,
    pub bytes_wanted: u64,
    pub bytes_target: u64,
    /// The hysteretic wanted levels did not fit the pool.
    pub over_budget: bool,
    /// Textures whose target is coarser than they currently want.
    pub limited_by_pool: u32,
    /// Of those, textures in the reveal radius.
    pub limited_in_radius: u32,
    /// Demands inside the reveal radius whose texture is resident coarser than the demand wants.
    pub mip_deficit_visible: u32,
    pub downgrades: u32,
    pub upgrades: u32,
}

/// The planner's output.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    /// Target first resident level, parallel to the inputs.
    pub targets: Vec<u32>,
    /// Level changes, downgrades first, then upgrades by priority.
    pub requests: Vec<Request>,
    pub counters: Counters,
    /// Set on the first plan of an episode where the pool had to coarsen textures in the reveal
    /// radius: those textures, by id (the caller logs one `warn!`).
    pub pool_warning: Option<Vec<TextureId>>,
}

/// Per-texture hysteresis memory.
#[derive(Clone, Copy, Debug)]
struct Track {
    /// Hysteretic wanted level.
    wanted: u32,
    /// Pending upgrade: coarsest level wanted since, and since when.
    up: Option<(u32, f64)>,
    /// Pending downgrade: finest level wanted since, and since when.
    down: Option<(u32, f64)>,
    /// Last time a demand wanted a level finer than the coarsest.
    last_wanted: f64,
    /// Plan generation that last saw this texture.
    seen: u64,
}

impl Track {
    fn step(&mut self, raw: u32, now: f64, settings: &Settings) {
        match raw.cmp(&self.wanted) {
            Ordering::Less => {
                self.down = None;
                let (level, since) = self.up.map_or((raw, now), |(l, s)| (l.max(raw), s));
                if now - since >= settings.upgrade_stable_s {
                    self.wanted = level;
                    self.up = None;
                } else {
                    self.up = Some((level, since));
                }
            }
            Ordering::Greater => {
                self.up = None;
                let (level, since) = self.down.map_or((raw, now), |(l, s)| (l.min(raw), s));
                if now - since >= settings.downgrade_wait_s {
                    self.wanted = level;
                    self.down = None;
                } else {
                    self.down = Some((level, since));
                }
            }
            Ordering::Equal => {
                self.up = None;
                self.down = None;
            }
        }
    }
}

/// Multiplicative hash for ids (the map is only looked up, never iterated for output).
#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        }
    }
    fn write_u64(&mut self, x: u64) {
        self.0 = (self.0.rotate_left(5) ^ x).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
}

/// The residency planner and its hysteresis state (owned by the caller, one per view).
#[derive(Default)]
pub struct Residency {
    tracks: HashMap<TextureId, Track, BuildHasherDefault<IdHasher>>,
    generation: u64,
    /// An in-radius pool episode is running (its warning was given).
    episode: bool,
}

/// `f64` bits that order like the values (total order, `-0 < +0`).
fn ord_bits(x: f64) -> u64 {
    let bits = x.to_bits();
    if bits >> 63 == 1 { !bits } else { bits | 1 << 63 }
}

/// Pool raise priority (max-heap): packed most levels finer than wanted (8 bits), then (outside
/// the radius) least recently wanted or (inside) farthest (64 bits), then most bytes freed (56
/// bits); then the smallest id; then the row.
type RaiseKey = (u128, Reverse<TextureId>, usize);

/// Per-input facts derived once per plan.
struct Row {
    raw: u32,
    target: u32,
    in_radius: bool,
    /// Nearest demand distance (all demands), and nearest in-radius one.
    near: f32,
    near_in_radius: f32,
    last_wanted: f64,
}

impl Residency {
    pub fn new() -> Self {
        Self::default()
    }

    /// Textures with hysteresis state.
    pub fn tracked(&self) -> usize {
        self.tracks.len()
    }

    /// Plans `inputs` at time `now` (seconds, monotonic). Textures absent from `inputs` lose their
    /// state.
    pub fn plan(&mut self, view: &View, settings: &Settings, inputs: &[TextureInput<'_>], now: f64) -> Plan {
        self.generation += 1;
        let generation = self.generation;
        let mut counters = Counters { textures: len_u32(inputs.len()), pool_bytes: settings.pool_bytes, ..Counters::default() };
        let mut rows = Vec::with_capacity(inputs.len());
        let scale = Scale::new(view);
        for input in inputs {
            let levels = input.levels;
            let streamable = levels.is_streamable();
            let mut raw = levels.coarsest();
            let (mut near, mut near_in_radius, mut in_radius) = (f32::INFINITY, f32::INFINITY, false);
            for d in input.demands {
                let wanted = if streamable { scale.wanted(levels, d) } else { raw };
                raw = raw.min(wanted);
                near = near.min(d.distance);
                if d.in_reveal_radius {
                    in_radius = true;
                    near_in_radius = near_in_radius.min(d.distance);
                    counters.mip_deficit_visible += u32::from(input.resident > wanted);
                }
            }
            // A texture with one start level has no choice to remember.
            let (wanted, last_wanted) = if streamable {
                let track = self.tracks.entry(input.id).or_insert(Track {
                    wanted: raw,
                    up: None,
                    down: None,
                    last_wanted: f64::NEG_INFINITY,
                    seen: generation,
                });
                track.wanted = levels.snap(track.wanted);
                track.step(raw, now, settings);
                track.seen = generation;
                if raw < levels.coarsest() {
                    track.last_wanted = now;
                }
                (track.wanted, track.last_wanted)
            } else {
                (raw, now)
            };
            counters.streamable += u32::from(streamable);
            counters.bytes_resident += levels.bytes_at(input.resident);
            counters.bytes_wanted += levels.bytes_at(wanted);
            rows.push(Row { raw, target: wanted, in_radius, near, near_in_radius, last_wanted });
        }
        self.tracks.retain(|_, t| t.seen == generation);

        let mut total = counters.bytes_wanted;
        counters.over_budget = total > settings.pool_bytes;
        let mut raised_in_radius = Vec::new();
        if counters.over_budget {
            total = Self::squeeze(inputs, &mut rows, total, settings.pool_bytes, false, &mut raised_in_radius);
        }
        if total > settings.pool_bytes {
            total = Self::squeeze(inputs, &mut rows, total, settings.pool_bytes, true, &mut raised_in_radius);
        }
        counters.bytes_target = total;

        let pool_warning = if raised_in_radius.is_empty() {
            self.episode = false;
            None
        } else if self.episode {
            None
        } else {
            self.episode = true;
            raised_in_radius.sort_unstable();
            raised_in_radius.dedup();
            Some(raised_in_radius)
        };

        let mut downs = Vec::new();
        let mut ups = Vec::new();
        for (input, row) in inputs.iter().zip(&rows) {
            let levels = input.levels;
            if row.target > row.raw {
                counters.limited_by_pool += 1;
                counters.limited_in_radius += u32::from(row.in_radius);
            }
            let request = Request { id: input.id, from: input.resident, to: row.target };
            let id = u128::from(input.id.0);
            match row.target.cmp(&input.resident) {
                Ordering::Greater => {
                    // Most bytes freed first, then the smallest id.
                    let freed = levels.bytes_at(input.resident) - levels.bytes_at(row.target);
                    downs.push((u128::from(u64::MAX - freed) << 64 | id, request));
                }
                Ordering::Less => {
                    // Reveal radius first, then the largest visible deficit, the nearest, the smallest id
                    // (a non-negative `f32`'s bits order like its value; NaN sorts as infinity).
                    let deficit = input.resident.saturating_sub(row.raw).min(0x7fff_ffff);
                    let near = if row.near.is_nan() { f32::INFINITY } else { row.near.max(0.0) };
                    let key = u128::from(!row.in_radius) << 127
                        | u128::from(0x7fff_ffff - deficit) << 96
                        | u128::from(near.to_bits()) << 64
                        | id;
                    ups.push((key, request));
                }
                Ordering::Equal => {}
            }
        }
        downs.sort_unstable_by_key(|d| d.0);
        ups.sort_unstable_by_key(|u| u.0);
        counters.downgrades = len_u32(downs.len());
        counters.upgrades = len_u32(ups.len());
        let requests = downs.into_iter().chain(ups).map(|(_, r)| r).collect();

        Plan { targets: rows.iter().map(|r| r.target).collect(), requests, counters, pool_warning }
    }

    /// Raises targets of textures outside (`in_radius` false) or inside the reveal radius, one
    /// start level at a time by [`RaiseKey`], until `total` fits `pool` or none can be raised.
    fn squeeze(
        inputs: &[TextureInput<'_>],
        rows: &mut [Row],
        mut total: u64,
        pool: u64,
        in_radius: bool,
        raised_in_radius: &mut Vec<TextureId>,
    ) -> u64 {
        let key = |i: usize, row: &Row| -> Option<RaiseKey> {
            let levels = inputs[i].levels;
            let next = levels.coarser(row.target)?;
            let excess = (i64::from(row.raw) - i64::from(row.target)).clamp(-127, 127) + 128;
            let order = if in_radius { f64::from(row.near_in_radius) } else { -row.last_wanted };
            let freed = (levels.bytes_at(row.target) - levels.bytes_at(next)).min((1 << 56) - 1);
            let packed = u128::from(excess.unsigned_abs()) << 120 | u128::from(ord_bits(order)) << 56 | u128::from(freed);
            Some((packed, Reverse(inputs[i].id), i))
        };
        let mut heap: BinaryHeap<RaiseKey> = rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.in_radius == in_radius)
            .filter_map(|(i, r)| key(i, r))
            .collect();
        while total > pool {
            let Some((_, _, i)) = heap.pop() else { break };
            let levels = inputs[i].levels;
            let row = &mut rows[i];
            let next = levels.coarser(row.target).expect("keyed rows can be raised");
            total -= levels.bytes_at(row.target) - levels.bytes_at(next);
            row.target = next;
            if in_radius {
                raised_in_radius.push(inputs[i].id);
            }
            if let Some(k) = key(i, row) {
                heap.push(k);
            }
        }
        total
    }
}

fn len_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The unclamped wanted level (bias included) for a texture whose level 0's larger side is
/// `top` texels, drawn at `rho` UV units per metre, `distance` metres away.
pub fn texel_level(view: &View, top: u32, rho: f32, distance: f32) -> i32 {
    Scale::new(view).level(top, rho, distance)
}

/// A demand's wanted start level for `levels` (clamped and snapped to its start levels).
pub fn wanted_level(view: &View, levels: &TextureLevels, demand: &Demand) -> u32 {
    Scale::new(view).wanted(levels, demand)
}

/// The view's per-plan constants.
struct Scale {
    /// `2·tan(fov_y/2) / H_out`: texels per pixel per (texel per metre · metre).
    k: f64,
    bias: i32,
    fallback_rho: f32,
}

impl Scale {
    fn new(view: &View) -> Self {
        let k = 2.0 * (f64::from(view.fov_y) * 0.5).tan() / f64::from(view.h_out);
        Self { k, bias: view.bias, fallback_rho: view.fallback_rho }
    }

    fn level(&self, top: u32, rho: f32, distance: f32) -> i32 {
        let texels_per_px = f64::from(rho) * f64::from(top) * f64::from(distance.max(0.0)) * self.k;
        floor_log2(texels_per_px).saturating_add(self.bias)
    }

    fn wanted(&self, levels: &TextureLevels, demand: &Demand) -> u32 {
        let rho = demand.rho.filter(|r| r.is_finite() && *r > 0.0).unwrap_or(self.fallback_rho);
        let level = self.level(levels.top, rho, demand.distance).max(0);
        levels.snap(u32::try_from(level).unwrap_or(u32::MAX).min(levels.coarsest()))
    }
}

/// `floor(log2(x))` read from the exponent (exact), clamped to ±64; 0, subnormal or NaN: −64.
fn floor_log2(x: f64) -> i32 {
    if x.is_nan() || x < f64::MIN_POSITIVE {
        return -64;
    }
    if x.is_infinite() {
        return 64;
    }
    let exponent = i32::try_from((x.to_bits() >> 52) & 0x7ff).unwrap_or(0) - 1023;
    exponent.clamp(-64, 64)
}

/// Distance from `eye` to a sphere (0 inside).
pub fn sphere_distance(center: [f32; 3], radius: f32, eye: [f32; 3]) -> f32 {
    let d = [center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]];
    ((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() - radius).max(0.0)
}

/// The streaming scheduler's effective distance: the nearer of now and `lookahead_s` ahead at
/// `velocity` (streaming.md "Priority"), so upgrades run ahead of travel.
pub fn effective_distance(center: [f32; 3], radius: f32, eye: [f32; 3], velocity: [f32; 3], lookahead_s: f32) -> f32 {
    let ahead = [eye[0] + velocity[0] * lookahead_s, eye[1] + velocity[1] * lookahead_s, eye[2] + velocity[2] * lookahead_s];
    sphere_distance(center, radius, eye).min(sphere_distance(center, radius, ahead))
}

#[cfg(test)]
mod tests;
