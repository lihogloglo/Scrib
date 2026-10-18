//! The live wind: the weather's [`Wind`], the advected gust field and the
//! local sources, evaluated for a floating origin as a [`WindFrame`].

use glam::{DVec2, Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};
use sf_coords::{GRID_CELL, WorldPos};

use crate::frame::{WindFrame, WindParams, WindSourceGpu};
use crate::source::{MAX_SOURCES, WindSource, WindSourceId, WindSourceKind};
use crate::{PERIOD, Wind};

/// Second gust octave: cell size relative to the first, and its drift
/// (along, across the wind) relative to the gust travel (Megalophobia's
/// constants: it shears across the first so the sum never repeats).
const OCTAVE1_SCALE: f32 = 0.37;
const OCTAVE1_DRIFT: Vec2 = Vec2::new(1.9, 0.6);
/// Grid cells per [`PERIOD`] (the origin phase is computed from the cell).
#[expect(clippy::cast_possible_truncation, reason = "1024 / 256")]
const PERIOD_CELLS: i32 = (PERIOD / GRID_CELL as f64) as i32;
const _: () = assert!(PERIOD_CELLS as f64 * GRID_CELL as f64 == PERIOD, "the period must be whole grid cells");

/// Tuning of the field (framework defaults; an adapter may override).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WindSettings {
    /// How far the swirl turns the flow off the prevailing wind, `[0, 0.99]`:
    /// at most `asin(swirl)`, so nothing ever blows upwind.
    pub swirl: f32,
    /// Gust-field value below which the air lulls; higher = patchier.
    pub gust_bias: f32,
    /// Width of the ramp from lull to gust, in field units.
    pub gust_band: f32,
    /// Speed the gust cells travel downwind, relative to the mean wind.
    pub gust_travel: f32,
    /// Aerodynamic roughness length z0 of the log profile (m): 0.03 open
    /// grass, 0.1 crops and scrub, 0.5-1 forest and towns.
    pub roughness: f32,
    /// Height above ground (m) where the profile is 1 and [`Wind::velocity`]
    /// holds (10 m, the WMO anemometer height).
    pub reference_height: f32,
    /// Cap on the profile factor high above the ground.
    pub max_profile: f32,
}

impl Default for WindSettings {
    fn default() -> Self {
        Self {
            swirl: 0.9,
            gust_bias: 0.42,
            gust_band: 0.3,
            gust_travel: 1.0,
            roughness: 0.1,
            reference_height: 10.0,
            max_profile: 2.0,
        }
    }
}

/// Lattice cells per [`PERIOD`] for a cell size (at least 1).
fn cells_for(size: f32) -> u32 {
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "clamped to 1..=4096")]
    let n = (PERIOD / f64::from(size.max(1.0))).round().clamp(1.0, 4096.0) as u32;
    n
}

/// The wind as it evolves: what weather sets ([`Wind`]), how far the gust
/// field has travelled, and the local sources. Saveable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindState {
    pub wind: Wind,
    /// Wind clock in seconds (source lifetimes).
    pub time: f64,
    /// How far each gust octave has travelled, wrapped to [`PERIOD`].
    drift: [DVec2; 2],
    sources: Vec<(WindSourceId, f64, WindSource)>,
    next_id: u64,
}

impl WindState {
    pub fn new(wind: Wind) -> Self {
        Self {
            wind,
            time: 0.0,
            drift: [DVec2::ZERO; 2],
            sources: Vec::new(),
            next_id: 0,
        }
    }

    /// Advances the clock: the gust field travels downwind at the current
    /// speed (integrated, so a change of speed never jumps the field) and
    /// expired sources go.
    pub fn advance(&mut self, dt: f32, settings: &WindSettings) {
        let dt = f64::from(dt);
        self.time += dt;
        let speed = self.wind.speed();
        if speed > 0.0 {
            let axis = (self.wind.velocity / speed).as_dvec2();
            let travel = f64::from(speed * settings.gust_travel) * dt;
            let slant = DVec2::new(-axis.y, axis.x);
            let drift1 = axis * f64::from(OCTAVE1_DRIFT.x) + slant * f64::from(OCTAVE1_DRIFT.y);
            self.drift[0] = (self.drift[0] + axis * travel).rem_euclid(DVec2::splat(PERIOD));
            self.drift[1] = (self.drift[1] + drift1 * travel).rem_euclid(DVec2::splat(PERIOD));
        }
        let now = self.time;
        #[expect(clippy::cast_possible_truncation, reason = "source ages are short")]
        self.sources.retain(|(_, start, s)| !s.expired((now - start) as f32));
    }

    /// Adds a source starting now. `None` when all [`MAX_SOURCES`] are in use.
    pub fn add_source(&mut self, source: WindSource) -> Option<WindSourceId> {
        if self.sources.len() >= MAX_SOURCES {
            return None;
        }
        let id = WindSourceId(self.next_id);
        self.next_id += 1;
        self.sources.push((id, self.time, source));
        Some(id)
    }

    /// Removes a source; false if it was gone already.
    pub fn remove_source(&mut self, id: WindSourceId) -> bool {
        let before = self.sources.len();
        self.sources.retain(|(i, _, _)| *i != id);
        self.sources.len() != before
    }

    pub fn sources(&self) -> impl Iterator<Item = (WindSourceId, &WindSource)> {
        self.sources.iter().map(|(id, _, s)| (*id, s))
    }

    /// Everything the GPU (and the CPU twin) needs to sample the field at
    /// positions relative to `origin`, the floating origin.
    pub fn frame(&self, settings: &WindSettings, origin: WorldPos) -> WindFrame {
        let speed = self.wind.speed();
        let axis = if speed > 0.0 { self.wind.velocity / speed } else { Vec2::X };
        // Origin phase: the origin modulo the period, from the cell index
        // (exact) plus the cell-local offset, minus each octave's drift.
        let phase = |cell: i32, local: f32| f64::from(cell.rem_euclid(PERIOD_CELLS)) * f64::from(GRID_CELL) + f64::from(local);
        let origin_mod = DVec2::new(phase(origin.cell.x, origin.local.x), phase(origin.cell.z, origin.local.z));
        let n0 = cells_for(self.wind.gust_length);
        #[expect(clippy::cast_possible_truncation, reason = "f32 sizes")]
        let size0 = (PERIOD / f64::from(n0)) as f32;
        let n1 = cells_for(size0 * OCTAVE1_SCALE);
        // The offset in lattice cells, split into whole cells (exact integers)
        // and a fraction, so the shader's float math only carries
        // `rel / cell + frac` and keeps full precision near the origin.
        let octave = |k: usize, n: u32| {
            let inv = f64::from(n) / PERIOD;
            let c = (origin_mod - self.drift[k]).rem_euclid(DVec2::splat(PERIOD)) * inv;
            let whole = c.floor();
            let frac = (c - whole).as_vec2().min(Vec2::splat(1.0 - f32::EPSILON));
            #[expect(clippy::cast_possible_truncation, reason = "f32 upload")]
            let row = Vec4::new(frac.x, frac.y, inv as f32, f64::from(n) as f32);
            (row, whole.as_vec2())
        };
        let (octave0, whole0) = octave(0, n0);
        let (octave1, whole1) = octave(1, n1);

        let z0 = settings.roughness.max(1e-3);
        let reference = settings.reference_height.max(0.1);
        let sources: Vec<WindSourceGpu> = self
            .sources
            .iter()
            .filter(|(_, _, s)| s.radius > 0.0)
            .map(|(_, start, s)| {
                #[expect(clippy::cast_possible_truncation, reason = "source ages are short")]
                let age = (self.time - start) as f32;
                let center = origin.delta_f32(s.center);
                let (kind, v) = match s.kind {
                    WindSourceKind::Directional { direction } => (0.0, direction),
                    WindSourceKind::Radial => (1.0, Vec3::ZERO),
                    WindSourceKind::Vortex { axis } => (2.0, axis),
                };
                let radius = s.radius.max(0.01);
                WindSourceGpu {
                    a: center.extend(kind),
                    b: v.extend(s.strength * s.envelope(age)),
                    c: Vec4::new(s.inner_radius.clamp(0.0, radius * 0.999), radius, 0.0, 0.0),
                }
            })
            .collect();
        #[expect(clippy::cast_precision_loss, reason = "<= 32")]
        let count = sources.len() as f32;
        WindFrame {
            params: WindParams {
                axis_speed: Vec4::new(axis.x, axis.y, speed, self.wind.gustiness.clamp(0.0, 1.0)),
                octave0,
                octave1,
                cells: Vec4::new(whole0.x, whole0.y, whole1.x, whole1.y),
                shape: Vec4::new(settings.swirl.clamp(0.0, 0.99), settings.gust_bias, settings.gust_band.max(1e-3), count),
                profile: Vec4::new(z0, 1.0 / (reference / z0 + 1.0).ln(), settings.max_profile, reference),
            },
            sources,
        }
    }

    /// Wind velocity (m/s) at a world position, `above_ground` meters over
    /// the ground. Builds a frame around `pos`'s cell: for many queries per
    /// frame, build one [`WindFrame`] and use [`WindFrame::sample`].
    pub fn velocity_at(&self, settings: &WindSettings, pos: WorldPos, above_ground: f32) -> Vec3 {
        let origin = WorldPos::new(pos.cell, Vec3::ZERO);
        self.frame(settings, origin).sample(pos.local, above_ground)
    }
}
