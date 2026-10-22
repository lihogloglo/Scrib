//! Physics impact and slide sounds (audio.md "Impacts"). The host feeds each step's contact
//! reports ([`ContactSample`]: the normal impulse, the speed change it caused, the slide speed and
//! the two surfaces); [`ImpactAudio::tick`] picks a cue by surface pair and impulse band, scales
//! its volume by the impact speed, rate-limits per body and keeps one scrape loop per sliding body.
//!
//! Gating follows the practice of Havok/Source-style impact listeners: a hit needs a speed change
//! above [`ImpactParams::min_speed`] (resting and rolling contacts only hold bodies up against
//! gravity, `g * dt` per step), a body sounds at most once per [`ImpactParams::cooldown_s`] unless
//! hit much harder, and at most [`ImpactParams::max_per_tick`] hits play per tick, loudest first
//! (a collapsing pile is a handful of voices, not dozens).
//!
//! Surfaces are the host's surface ids (`sf_controller::SurfaceId` values: 0 is the default
//! surface); which cue a pair plays is content ([`ImpactMap`]).

use std::collections::HashMap;

use sf_records::RecordId;

use crate::fade_toward;

/// The cues for a surface pair (or one surface against anything).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImpactCues {
    /// `(lowest impulse in N·s, cue)` bands: a hit plays the highest band it reaches; a hit
    /// under every band is silent (a pebble's tick on a crate's cue). Kept sorted.
    pub bands: Vec<(f32, RecordId)>,
    /// Looping scrape while one body slides on the other.
    pub slide: Option<RecordId>,
}

impl ImpactCues {
    /// One impact cue from `min_impulse` up, no slide loop.
    pub fn impact(min_impulse: f32, cue: RecordId) -> Self {
        Self { bands: vec![(min_impulse, cue)], slide: None }
    }

    #[must_use]
    pub fn with_band(mut self, min_impulse: f32, cue: RecordId) -> Self {
        self.bands.push((min_impulse, cue));
        self.bands.sort_by(|a, b| a.0.total_cmp(&b.0));
        self
    }

    #[must_use]
    pub fn with_slide(mut self, cue: RecordId) -> Self {
        self.slide = Some(cue);
        self
    }

    /// The cue of the highest band `impulse` reaches.
    pub fn band(&self, impulse: f32) -> Option<&RecordId> {
        self.bands.iter().rev().find(|(min, _)| impulse >= *min).map(|(_, cue)| cue)
    }
}

/// Surface pair -> cues (adapter data). `None` matches any surface.
#[derive(Clone, Debug, Default)]
pub struct ImpactMap {
    map: HashMap<(Option<u16>, Option<u16>), ImpactCues>,
}

impl ImpactMap {
    /// Sets the cues of `a` against `b` (unordered; `None` = any surface).
    pub fn insert(&mut self, a: Option<u16>, b: Option<u16>, cues: ImpactCues) {
        let mut cues = cues;
        cues.bands.sort_by(|x, y| x.0.total_cmp(&y.0));
        self.map.insert(Self::key(a, b), cues);
    }

    fn key(a: Option<u16>, b: Option<u16>) -> (Option<u16>, Option<u16>) {
        if a <= b { (a, b) } else { (b, a) }
    }

    /// Lookup order for `mover` (the moving body's surface) touching `other`: the exact pair,
    /// the mover against anything (a cup sounds like a cup on most floors), the other against
    /// anything, then the catch-all.
    fn chain(&self, mover: u16, other: u16) -> impl Iterator<Item = &ImpactCues> {
        [(Some(mover), Some(other)), (Some(mover), None), (Some(other), None), (None, None)]
            .into_iter()
            .filter_map(|(a, b)| self.map.get(&Self::key(a, b)))
    }

    /// The impact cue for a hit of `impulse` N·s: the first entry in the lookup order that has
    /// bands decides (its bands may leave a light hit silent).
    pub fn impact(&self, mover: u16, other: u16, impulse: f32) -> Option<&RecordId> {
        self.chain(mover, other).find(|c| !c.bands.is_empty())?.band(impulse)
    }

    /// The slide loop: the first entry in the lookup order with one.
    pub fn slide(&self, mover: u16, other: u16) -> Option<&RecordId> {
        self.chain(mover, other).find_map(|c| c.slide.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// One contact report of a physics step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactSample {
    /// Stable keys of the two bodies when they move (rate limits and slide loops are per moving
    /// body); `None` for world geometry. The first is the reporting (moving) body.
    pub bodies: [Option<u64>; 2],
    /// Surface ids, in `bodies` order.
    pub surfaces: [u16; 2],
    /// Normal impulse this step, N·s (picks the band).
    pub impulse: f32,
    /// Normal speed change the impulse caused, m/s (the impact speed: gates and scales).
    pub speed: f32,
    /// Tangential slide speed at the contact, m/s.
    pub slide: f32,
    /// Where, in the host's playback frame.
    pub at: [f32; 3],
}

/// Tuning (defaults: audio.md "Impacts").
#[derive(Clone, Debug, PartialEq)]
pub struct ImpactParams {
    /// No hit below this speed change, m/s (holding a body up is `g * dt`, 0.16 m/s at 60 Hz).
    pub min_speed: f32,
    /// Full volume from this impact speed, m/s.
    pub full_speed: f32,
    /// Volume of a hit at `min_speed` (linear up to 1 at `full_speed`).
    pub min_volume: f32,
    /// A body sounds at most once per this many seconds...
    pub cooldown_s: f32,
    /// ...unless hit at least this many times harder (impulse) than its last sound.
    pub override_ratio: f32,
    /// Hits played per tick at most, loudest first.
    pub max_per_tick: usize,
    /// Slide loop from this slide speed (m/s), full level at `slide_full`.
    pub slide_min: f32,
    pub slide_full: f32,
    /// Level of a slide at `slide_min`.
    pub slide_min_level: f32,
    /// A slide loop fades out once its body has reported no slide for this long, s.
    pub slide_hold_s: f32,
    /// Slide loop fade time (silence to full), s: fast, a scrape starts and stops with the motion.
    pub slide_fade_s: f32,
}

impl Default for ImpactParams {
    fn default() -> Self {
        Self {
            min_speed: 0.5,
            full_speed: 6.0,
            min_volume: 0.15,
            cooldown_s: 0.1,
            override_ratio: 2.0,
            max_per_tick: 8,
            slide_min: 0.4,
            slide_full: 3.0,
            slide_min_level: 0.3,
            slide_hold_s: 0.12,
            slide_fade_s: 0.1,
        }
    }
}

/// A hit to play once.
#[derive(Clone, Debug, PartialEq)]
pub struct ImpactPlay {
    pub cue: RecordId,
    /// 0..1, scales the cue's own volume.
    pub volume: f32,
    pub at: [f32; 3],
}

/// What a body's slide loop does this tick. A loop is keyed by its body.
#[derive(Clone, Debug, PartialEq)]
pub enum SlideChange {
    /// Start looping `cue` at `level` (0..1).
    Start { body: u64, cue: RecordId, level: f32, at: [f32; 3] },
    /// New level and position.
    Update { body: u64, level: f32, at: [f32; 3] },
    /// Faded out: stop the loop.
    Stop { body: u64 },
}

/// One tick's output.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImpactFrame {
    pub impacts: Vec<ImpactPlay>,
    pub slides: Vec<SlideChange>,
}

#[derive(Clone, Debug)]
struct Slide {
    cue: RecordId,
    level: f32,
    target: f32,
    seen_s: f64,
    at: [f32; 3],
}

/// The impact and slide state: the cue map, per-body cooldowns and the playing slide loops.
#[derive(Clone, Debug, Default)]
pub struct ImpactAudio {
    pub map: ImpactMap,
    pub params: ImpactParams,
    now_s: f64,
    /// Body -> (time, impulse) of its last hit sound.
    last_hit: HashMap<u64, (f64, f32)>,
    slides: HashMap<u64, Slide>,
}

/// Maps `x` from `[lo, hi]` to `[floor, 1]`, clamped.
fn ramp(x: f32, lo: f32, hi: f32, floor: f32) -> f32 {
    let t = if hi > lo { ((x - lo) / (hi - lo)).clamp(0.0, 1.0) } else { 1.0 };
    floor + (1.0 - floor) * t
}

impl ImpactAudio {
    pub fn new(map: ImpactMap, params: ImpactParams) -> Self {
        Self { map, params, ..Self::default() }
    }

    /// Slide loops playing (or fading out).
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Advances by `dt` seconds with the contacts reported since the last tick.
    pub fn tick(&mut self, dt: f32, samples: &[ContactSample]) -> ImpactFrame {
        self.now_s += f64::from(dt.max(0.0));
        let mut frame = ImpactFrame::default();
        self.impacts(samples, &mut frame.impacts);
        self.slide(dt, samples, &mut frame.slides);
        frame
    }

    fn impacts(&mut self, samples: &[ContactSample], out: &mut Vec<ImpactPlay>) {
        let p = &self.params;
        let now = self.now_s;
        let cooldown = f64::from(p.cooldown_s);
        self.last_hit.retain(|_, (t, _)| now - *t < cooldown);
        let mut hits: Vec<(f32, &ContactSample, &RecordId)> = samples
            .iter()
            .filter(|s| s.speed >= p.min_speed)
            .filter_map(|s| {
                let cue = self.map.impact(s.surfaces[0], s.surfaces[1], s.impulse)?;
                Some((ramp(s.speed, p.min_speed, p.full_speed, p.min_volume), s, cue))
            })
            .collect();
        hits.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (volume, s, cue) in hits {
            if out.len() >= p.max_per_tick {
                break;
            }
            let allowed = |body: &u64| self.last_hit.get(body).is_none_or(|(_, last)| s.impulse >= last * p.override_ratio);
            if !s.bodies.iter().flatten().all(allowed) {
                continue;
            }
            for body in s.bodies.iter().flatten() {
                self.last_hit.insert(*body, (now, s.impulse));
            }
            out.push(ImpactPlay { cue: cue.clone(), volume, at: s.at });
        }
    }

    fn slide(&mut self, dt: f32, samples: &[ContactSample], out: &mut Vec<SlideChange>) {
        let p = &self.params;
        let now = self.now_s;
        // The fastest slide of each moving body this tick, with its cue.
        let mut wanted: HashMap<u64, (&ContactSample, &RecordId)> = HashMap::new();
        for s in samples.iter().filter(|s| s.slide >= p.slide_min) {
            let Some(body) = s.bodies[0].or(s.bodies[1]) else { continue };
            let Some(cue) = self.map.slide(s.surfaces[0], s.surfaces[1]) else { continue };
            if wanted.get(&body).is_none_or(|(w, _)| s.slide > w.slide) {
                wanted.insert(body, (s, cue));
            }
        }
        let mut started = Vec::new();
        for (body, (s, cue)) in wanted {
            let target = ramp(s.slide, p.slide_min, p.slide_full, p.slide_min_level);
            match self.slides.get_mut(&body) {
                Some(slide) if slide.cue == *cue => {
                    slide.target = target;
                    slide.seen_s = now;
                    slide.at = s.at;
                }
                current => {
                    // A new loop, or another surface under the same body: the old cue stops at once.
                    if current.is_some() {
                        out.push(SlideChange::Stop { body });
                    }
                    let level = fade_toward(0.0, target, dt, p.slide_fade_s);
                    out.push(SlideChange::Start { body, cue: cue.clone(), level, at: s.at });
                    started.push(body);
                    self.slides.insert(body, Slide { cue: cue.clone(), level, target, seen_s: now, at: s.at });
                }
            }
        }
        let hold = f64::from(p.slide_hold_s);
        let mut stopped = Vec::new();
        for (&body, slide) in &mut self.slides {
            if started.contains(&body) {
                continue;
            }
            if now - slide.seen_s > hold {
                slide.target = 0.0;
            }
            slide.level = fade_toward(slide.level, slide.target, dt, p.slide_fade_s);
            if slide.level <= 0.0 && slide.target <= 0.0 {
                stopped.push(body);
                out.push(SlideChange::Stop { body });
            } else {
                out.push(SlideChange::Update { body, level: slide.level, at: slide.at });
            }
        }
        for body in stopped {
            self.slides.remove(&body);
        }
    }
}

#[cfg(test)]
#[path = "impacts_tests.rs"]
mod tests;
