//! Kick-up (surface-weather.md W7): puffs of snow and ash thrown up where a foot plants. The CPU
//! twin of `sf_surface_bevy`'s `kickup.wesl` and the burst ring that feeds it, so the GPU math is
//! tested exactly here.
//!
//! - **Burst pool** ([`BurstRing`]): a fixed ring of [`BURSTS`] bursts (the canonical GPU-instanced
//!   burst pool: one draw of a fixed mesh of [`BURSTS`] × [`SPRITES`] quads, no spawn or despawn).
//!   A new burst takes the next slot, overwriting the oldest; a burst whose sprites have all died
//!   ([`KickLook::max_age_s`]) frees its slot.
//! - **Sprites** (closed form, no per-sprite CPU work): each sprite of a burst gets a launch
//!   velocity from its seeds ([`launch`]: a radial spray biased along the foot's facing), then
//!   flies the exact solution of linear drag toward the wind under gravity ([`arc`]), never below
//!   the burst's floor ([`floor_clamp`]); it fades in, out ([`fade`]) and grows ([`size`]) over its
//!   life.
//! - **Strength** ([`strength`]): the cover under the foot × the pace (the step's speed,
//!   [`StepClock`]: distance to the previous print nearby over the time between) × the foot's
//!   size.
//!
//! The kind looks ([`KickLook::SNOW`], [`KickLook::ASH`]) are start values for the owner's look.
//! Presentation only (design rule "net-ready"): bursts never feed gameplay.

use glam::{DVec3, Vec2, Vec3};

use crate::smoothstep;

/// Bursts alive at once (the ring's slots).
pub const BURSTS: usize = 64;
/// Sprites per burst.
pub const SPRITES: u32 = 12;
/// Feet further than this from the camera throw nothing (m).
pub const RANGE_M: f32 = 30.0;
/// Cover below this share throws nothing.
pub const MIN_COVER: f32 = 0.15;
/// The floor lies this far below the planted foot (m): sprites never sink under it.
pub const FLOOR_BELOW_M: f32 = 0.05;
/// The typical half length of a foot print (m; `trail::FOOT_HALF_M.x`'s order): feet this size throw
/// a full burst.
pub const FOOT_REF_M: f32 = 0.14;

/// What is thrown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KickKind {
    /// Powder snow: short white-blue arcs.
    Snow = 0,
    /// Ash: lighter, grey-brown, drifts with the wind.
    Ash = 1,
}

impl KickKind {
    pub const ALL: [KickKind; 2] = [KickKind::Snow, KickKind::Ash];

    pub fn index(self) -> usize {
        self as usize
    }
}

/// How a kind flies and looks (one entry of the shader's kind table).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KickLook {
    /// Mean sprite life (s); each sprite lives 0.75-1.25 × this.
    pub life_s: f32,
    /// Linear drag rate (1/s): the velocity relaxes toward the terminal one at this rate.
    pub drag: f32,
    /// Downward acceleration (m/s²).
    pub gravity: f32,
    /// Share of the wind the sprites drift with (`[0, 1]`).
    pub wind_follow: f32,
    /// Horizontal launch speed at full strength (m/s).
    pub speed: f32,
    /// Upward launch speed at full strength (m/s).
    pub up: f32,
    /// Bias of the spray along the foot's facing (0: radial).
    pub forward_bias: f32,
    /// Sprites launch over this spread of delays (s).
    pub delay_s: f32,
    /// Sprite radius at birth and at death (m).
    pub size: [f32; 2],
    /// Linear albedo (lit by the scene in the shader).
    pub albedo: Vec3,
    /// Peak opacity at full strength.
    pub opacity: f32,
}

impl KickLook {
    pub const SNOW: KickLook = KickLook {
        life_s: 0.6,
        drag: 4.0,
        gravity: 7.0,
        wind_follow: 0.3,
        speed: 1.7,
        up: 1.9,
        forward_bias: 0.6,
        delay_s: 0.06,
        size: [0.05, 0.2],
        albedo: Vec3::new(0.86, 0.9, 1.0),
        opacity: 0.55,
    };
    pub const ASH: KickLook = KickLook {
        life_s: 1.2,
        drag: 3.0,
        gravity: 1.2,
        wind_follow: 0.8,
        speed: 1.2,
        up: 1.2,
        forward_bias: 0.4,
        delay_s: 0.08,
        size: [0.07, 0.38],
        albedo: Vec3::new(0.3, 0.27, 0.24),
        opacity: 0.4,
    };

    pub fn of(kind: KickKind) -> KickLook {
        match kind {
            KickKind::Snow => Self::SNOW,
            KickKind::Ash => Self::ASH,
        }
    }

    /// The oldest a burst of this kind shows anything (s): the longest delay plus the longest life.
    pub fn max_age_s(&self) -> f32 {
        self.delay_s + 1.25 * self.life_s
    }
}

/// One burst: where and when a foot planted, what it threw.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Burst {
    /// World position of the planted foot.
    pub at: DVec3,
    /// The foot's facing on the ground (world X, Z; unit, or zero).
    pub forward: Vec2,
    pub kind: KickKind,
    /// `[0, 1]` ([`strength`]).
    pub strength: f32,
    /// Birth time (s, the host's presentation clock).
    pub born_s: f64,
    /// `[0, 1)`: varies the sprites' seeds between bursts that reuse a slot.
    pub seed: f32,
}

impl Burst {
    /// Seconds since birth.
    pub fn age_s(&self, now_s: f64) -> f32 {
        #[expect(clippy::cast_possible_truncation, reason = "ages are a few seconds")]
        let age = (now_s - self.born_s) as f32;
        age
    }
}

/// The fixed burst pool (see the module docs).
#[derive(Clone, Debug, PartialEq)]
pub struct BurstRing {
    slots: Vec<Option<Burst>>,
    next: usize,
    spawned: u64,
}

impl Default for BurstRing {
    fn default() -> Self {
        Self::new(BURSTS)
    }
}

impl BurstRing {
    pub fn new(capacity: usize) -> Self {
        Self { slots: vec![None; capacity.max(1)], next: 0, spawned: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Bursts spawned so far (the seed counter).
    pub fn spawned(&self) -> u64 {
        self.spawned
    }

    /// Puts `burst` in the next slot (the oldest one once the ring is full) and returns that slot.
    pub fn spawn(&mut self, burst: Burst) -> usize {
        let slot = self.next;
        self.slots[slot] = Some(burst);
        self.next = (slot + 1) % self.slots.len();
        self.spawned += 1;
        slot
    }

    /// Frees the slots whose bursts show nothing any more at `now_s` (older than their kind's
    /// [`KickLook::max_age_s`]); returns how many are left.
    pub fn expire(&mut self, now_s: f64, looks: &[KickLook; 2]) -> usize {
        for s in &mut self.slots {
            if s.is_some_and(|b| b.age_s(now_s) > looks[b.kind.index()].max_age_s()) {
                *s = None;
            }
        }
        self.live_count()
    }

    pub fn get(&self, slot: usize) -> Option<&Burst> {
        self.slots.get(slot).and_then(Option::as_ref)
    }

    pub fn live_count(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    /// The live bursts with their slots.
    pub fn live(&self) -> impl Iterator<Item = (usize, &Burst)> {
        self.slots.iter().enumerate().filter_map(|(i, s)| s.as_ref().map(|b| (i, b)))
    }

    pub fn clear(&mut self) {
        self.slots.iter_mut().for_each(|s| *s = None);
    }
}

/// A recent print, for the pace.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Print {
    at: DVec3,
    time_s: f64,
}

/// The pace of the feet: a new print's speed is its distance to the most recent print within a
/// step's reach (the other foot of the same walker) over the time between. A first step, or a
/// foot that stands, reads 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepClock {
    recent: Vec<Print>,
}

impl StepClock {
    /// Prints kept (the most recent; enough for a few walkers near the camera).
    pub const KEEP: usize = 16;
    /// A step is this long at least and at most (m).
    pub const STEP_M: [f64; 2] = [0.2, 2.6];
    /// A step takes this long at least and at most (s).
    pub const STEP_S: [f64; 2] = [0.12, 1.2];

    /// Records a print at `at`, `time_s`, and returns the pace it implies (m/s, 0 when unknown).
    pub fn step(&mut self, at: DVec3, time_s: f64) -> f32 {
        let pace = self
            .recent
            .iter()
            .rev()
            .find_map(|p| {
                let d = (at - p.at).length();
                let dt = time_s - p.time_s;
                let fits = (Self::STEP_M[0]..=Self::STEP_M[1]).contains(&d) && (Self::STEP_S[0]..=Self::STEP_S[1]).contains(&dt);
                fits.then(|| d / dt)
            })
            .unwrap_or(0.0);
        self.recent.retain(|p| time_s - p.time_s <= Self::STEP_S[1]);
        if self.recent.len() >= Self::KEEP {
            self.recent.remove(0);
        }
        self.recent.push(Print { at, time_s });
        #[expect(clippy::cast_possible_truncation, reason = "a walking speed")]
        let pace = pace as f32;
        pace
    }

    pub fn clear(&mut self) {
        self.recent.clear();
    }

    /// Prints held.
    pub fn recent_len(&self) -> usize {
        self.recent.len()
    }
}

/// A burst's strength, `[0, 1]`: the cover share under the foot, the pace (m/s; a stand still
/// throws a third, a sprint all of it) and the foot's half length (m; a scamp's paw throws less
/// than a giant's foot, capped at full).
pub fn strength(cover: f32, pace: f32, foot_half_m: f32) -> f32 {
    let cover = smoothstep(MIN_COVER, 0.6, cover);
    let pace = 0.35 + 0.65 * smoothstep(1.0, 5.0, pace);
    let foot = (foot_half_m / FOOT_REF_M).clamp(0.4, 1.0);
    (cover * pace * foot).clamp(0.0, 1.0)
}

/// Which cover a foot throws: the larger of snow and ash (each counted only while its look is
/// drawn), or none under [`MIN_COVER`].
pub fn kind_at(snow: f32, ash: f32) -> Option<(KickKind, f32)> {
    let (kind, cover) = if snow >= ash { (KickKind::Snow, snow) } else { (KickKind::Ash, ash) };
    (cover >= MIN_COVER).then_some((kind, cover))
}

/// The golden-ratio steps that turn a burst seed into per-sprite seeds (`kickup.wesl` `SEED_STEP`).
pub const SEED_STEP: Vec3 = Vec3::new(0.618_034, 0.754_877_7, 0.569_840_3);

/// A sprite's seeds in a burst: its fixed mesh seeds moved by the burst's seed.
pub fn sprite_seeds(mesh: Vec3, burst_seed: f32) -> Vec3 {
    (mesh + burst_seed * SEED_STEP).fract()
}

/// A sprite's delay and life (s) from its seeds.
pub fn timing(look: &KickLook, s: Vec3) -> (f32, f32) {
    (look.delay_s * s.z, look.life_s * (0.75 + 0.5 * s.x))
}

/// A sprite's launch velocity (m/s): a radial spray at azimuth `2π s.x`, biased along `forward`,
/// scaled by `strength` and the seeds.
pub fn launch(look: &KickLook, forward: Vec2, strength: f32, s: Vec3) -> Vec3 {
    let az = std::f32::consts::TAU * s.x;
    let h = Vec2::new(az.cos(), az.sin()) + forward * look.forward_bias;
    let hs = look.speed * strength * (0.35 + 0.65 * s.y);
    let vs = look.up * strength * (0.45 + 0.55 * s.z);
    Vec3::new(h.x * hs, vs, h.y * hs)
}

/// Position at `t` s of a point launched from `p0` at `v0` under linear drag `k` (1/s) toward the
/// air's velocity `wind`, with gravity `g` (m/s², downward): the exact solution of
/// `dv/dt = k (wind - v) - g ŷ`, i.e. `p0 + v∞ t + (v0 - v∞)(1 - e^{-kt})/k` with the terminal
/// velocity `v∞ = wind - (g/k) ŷ`.
pub fn arc(p0: Vec3, v0: Vec3, wind: Vec3, k: f32, g: f32, t: f32) -> Vec3 {
    let k = k.max(1e-3);
    let v_inf = wind - Vec3::Y * (g / k);
    p0 + v_inf * t + (v0 - v_inf) * ((1.0 - (-k * t).exp()) / k)
}

/// `p` kept a sprite's radius above the floor (`floor_y`).
pub fn floor_clamp(p: Vec3, floor_y: f32, radius: f32) -> Vec3 {
    Vec3::new(p.x, p.y.max(floor_y + 0.5 * radius), p.z)
}

/// Opacity share over life (`u` = age / life): a quick fade in, a long fade out.
pub fn fade(u: f32) -> f32 {
    if !(0.0..=1.0).contains(&u) {
        return 0.0;
    }
    smoothstep(0.0, 0.06, u) * (1.0 - smoothstep(0.35, 1.0, u))
}

/// Sprite radius (m) over life: grows as the powder spreads.
pub fn size(look: &KickLook, u: f32, s: Vec3) -> f32 {
    let u = u.clamp(0.0, 1.0);
    (look.size[0] + (look.size[1] - look.size[0]) * u.sqrt()) * (0.7 + 0.6 * s.y)
}

/// A sprite's world position at burst age `age` (s), or `None` before its delay or after its life:
/// the shader's vertex path, end to end.
pub fn sprite_at(look: &KickLook, burst: &Burst, origin: DVec3, wind: Vec3, mesh_seed: Vec3, age: f32) -> Option<(Vec3, f32, f32)> {
    let seeds = sprite_seeds(mesh_seed, burst.seed);
    let (delay, life) = timing(look, seeds);
    let flown = age - delay;
    if flown < 0.0 || flown > life {
        return None;
    }
    let share = flown / life;
    let p0 = (burst.at - origin).as_vec3();
    let radius = size(look, share, seeds);
    let v0 = launch(look, burst.forward, burst.strength, seeds);
    let pos = arc(p0, v0, wind * look.wind_follow, look.drag, look.gravity, flown);
    let pos = floor_clamp(pos, p0.y - FLOOR_BELOW_M, radius);
    Some((pos, radius, look.opacity * burst.strength * fade(share)))
}
