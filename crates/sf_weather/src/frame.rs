//! `evaluate`: the climates around the camera at a time → one
//! [`WeatherFrame`] for the renderer, wind, audio and gameplay.

use std::f32::consts::TAU;

use glam::Vec2;
use smallvec::SmallVec;

use sf_atmos::fog::KOSCHMIEDER;
use sf_atmos::{CloudWeather, FogSettings};
use sf_records::RecordId;
use sf_sky::{KeyCurve, Weather};
use sf_wind::{Wind, WindSource};

use crate::book::{BookPreset, ClimateIx, PrecipIx, PresetIx, WeatherBook};
use crate::hash::{self, salt};
use crate::records::{CloudTerms, FogTerms, SurfaceRates};
use crate::state::{Blend, CHANNELS, CLOUDS, CUES, FOG, PRECIP, SKY, WIND, WeatherState};

/// Strike rates are capped here (the thinning bound of the schedule), per game hour.
pub const LIGHTNING_CAP_PER_HOUR: f32 = 120.0;
/// Candidate strikes are drawn per bucket of this many game hours.
const LIGHTNING_BUCKET_HOURS: f64 = 1.0 / 30.0;
/// A frame step longer than this (sleep, time skip) reports no strikes.
const LIGHTNING_MAX_SPAN_HOURS: f64 = 1.0;
/// Default width of the region border blend, meters.
pub const BORDER_BAND_M: f32 = 256.0;
/// Visibility without fog or precipitation, meters.
pub const MAX_VISIBILITY_M: f32 = 100_000.0;

/// What `evaluate` needs besides the records and the state.
#[derive(Clone, Copy, Debug)]
pub struct WeatherInput<'a> {
    /// Game time, hours since the clock's epoch (`SkyClock::days` × hours per day).
    pub hours: f64,
    /// The previous evaluation's `hours` (lightning reports strikes in `[prev_hours, hours)`).
    pub prev_hours: f64,
    /// Hour of the day (time-of-day curves).
    pub hour_of_day: f32,
    /// Camera altitude above sea level, meters (visibility).
    pub altitude: f32,
    /// Climates at the camera with spatial weights ([`border_blend`]); empty
    /// for no weather (interiors).
    pub climates: &'a [(ClimateIx, f32)],
}

/// One precipitation kind at an intensity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrecipFrame {
    pub kind: PrecipIx,
    pub intensity: f32,
}

/// A sound loop at a volume.
#[derive(Clone, Debug, PartialEq)]
pub struct CueFrame {
    pub sound: RecordId,
    pub volume: f32,
    /// A climate (region) ambient loop rather than a weather loop.
    pub region: bool,
}

/// A lightning strike in this frame's time step.
#[derive(Clone, Debug, PartialEq)]
pub struct LightningStrike {
    /// Game hours.
    pub time: f64,
    pub climate: ClimateIx,
    /// Direction from the camera, radians from +X toward +Z.
    pub azimuth: f32,
    pub distance_m: f32,
    pub flash: f32,
    /// Thunder sound, delayed by `distance_m / 343` real seconds.
    pub thunder: Option<RecordId>,
}

/// A climate's storm wind source and how strongly it is on (0..1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StormFrame {
    pub climate: ClimateIx,
    pub source: WindSource,
    pub strength: f32,
}

/// Everything the weather drives, for one frame.
#[derive(Clone, Debug, PartialEq)]
pub struct WeatherFrame {
    /// Sky medium, cloud cover dimming, and the ground wind target
    /// (`sky.wind`; `sf_wind_bevy::WindField.state.wind`).
    pub sky: Weather,
    pub clouds: CloudWeather,
    /// Weather part of the fog; look fields are `FogSettings::default()`.
    pub fog: FogSettings,
    pub precip: Vec<PrecipFrame>,
    pub surface: SurfaceRates,
    pub lightning: Vec<LightningStrike>,
    /// Current mean strike rate, per game hour.
    pub lightning_per_hour: f32,
    pub cues: Vec<CueFrame>,
    pub tags: Vec<(RecordId, f32)>,
    pub storms: Vec<StormFrame>,
    /// Meteorological visibility at the camera (Koschmieder), meters.
    pub visibility_m: f32,
}

impl Default for WeatherFrame {
    fn default() -> Self {
        Self {
            sky: Weather::default(),
            clouds: CloudWeather { coverage: 0.0, cirrus: 0.0, ..CloudWeather::default() },
            fog: FogSettings::default(),
            precip: Vec::new(),
            surface: SurfaceRates::default(),
            lightning: Vec::new(),
            lightning_per_hour: 0.0,
            cues: Vec::new(),
            tags: Vec::new(),
            storms: Vec::new(),
            visibility_m: MAX_VISIBILITY_M,
        }
    }
}

impl WeatherFrame {
    /// The ground wind target.
    pub fn wind(&self) -> &Wind {
        &self.sky.wind
    }
}

/// Spatial weight of the camera's own climate at `distance_m` inside it
/// from the nearest border: ½ on the border, 1 from half the band in, a
/// smoothstep between. The neighbor gets the rest, so crossing a border
/// is a continuous crossfade.
pub fn border_weight(distance_m: f32, band_m: f32) -> f32 {
    let half = (0.5 * band_m).max(1e-3);
    let t = (distance_m / half).clamp(0.0, 1.0);
    0.5 + 0.5 * t * t * (3.0 - 2.0 * t)
}

/// The climates at a point `distance_m` inside `own` from its border with `neighbor`.
pub fn border_blend(own: ClimateIx, neighbor: Option<ClimateIx>, distance_m: f32, band_m: f32) -> SmallVec<[(ClimateIx, f32); 2]> {
    let mut out = SmallVec::new();
    match neighbor {
        Some(n) if n != own => {
            let w = border_weight(distance_m, band_m);
            out.push((own, w));
            if w < 1.0 {
                out.push((n, 1.0 - w));
            }
        }
        _ => out.push((own, 1.0)),
    }
    out
}

fn by_hour(curve: &KeyCurve, hour: f32) -> f32 {
    if curve.keys().is_empty() { 1.0 } else { curve.sample(hour) }
}

/// Mean wind of `preset` in `climate` at `t`: speed and direction wander
/// inside the preset's ranges (smooth in time).
#[allow(clippy::many_single_char_names)]
fn preset_wind(state: &WeatherState, climate_key: u64, p: &BookPreset, t: f64) -> Vec2 {
    let w = &p.record.wind;
    let key = climate_key ^ p.key.rotate_left(17);
    let period = crate::records::WindTerms::WANDER_HOURS;
    let s = hash::value_noise(state.seed, key, salt::WIND_SPEED, t, period);
    let d = hash::value_noise(state.seed, key, salt::WIND_DIR, t, period);
    let speed = w.speed[0] + (w.speed[1] - w.speed[0]) * s;
    let angle = (w.direction_deg + w.direction_jitter_deg * (2.0 * d - 1.0)).to_radians();
    Vec2::new(angle.cos(), angle.sin()) * speed
}

/// The weather at the camera (allocating convenience over [`evaluate_into`]).
pub fn evaluate(book: &WeatherBook, state: &WeatherState, input: &WeatherInput) -> WeatherFrame {
    let mut frame = WeatherFrame::default();
    evaluate_into(book, state, input, &mut frame);
    frame
}

#[derive(Default)]
struct Acc {
    clouds: CloudTerms,
    fog: FogTerms,
    haze: f32,
    dust: f32,
    dust_tint: glam::Vec3,
    dimming: f32,
    w_clouds: f32,
    w_sky: f32,
    w_fog: f32,
    wind: Vec2,
    gustiness: f32,
    w_wind: f32,
    gust_length: (f32, f32),
}

macro_rules! add_scaled {
    ($acc:expr, $src:expr, $w:expr; $($f:ident),*) => { $( $acc.$f += $src.$f * $w; )* };
}

macro_rules! scale {
    ($acc:expr, $s:expr; $($f:ident),*) => { $( $acc.$f *= $s; )* };
}

/// Evaluates into `frame`, reusing its allocations.
#[allow(clippy::too_many_lines)]
pub fn evaluate_into(book: &WeatherBook, state: &WeatherState, input: &WeatherInput, frame: &mut WeatherFrame) {
    frame.precip.clear();
    frame.lightning.clear();
    frame.cues.clear();
    frame.tags.clear();
    frame.storms.clear();

    // Per-climate blends, spatially weighted.
    let mut blends: SmallVec<[(ClimateIx, f32, Blend); 4]> = SmallVec::new();
    for &(c, w) in input.climates {
        if w <= 0.0 {
            continue;
        }
        let blend = state.blend(book, c, input.hours);
        if !blend.is_empty() {
            blends.push((c, w, blend));
        }
    }
    let total: f32 = blends.iter().map(|b| b.1).sum();
    if total <= 0.0 {
        let d = WeatherFrame::default();
        frame.sky = d.sky;
        frame.clouds = d.clouds;
        frame.fog = d.fog;
        frame.surface = d.surface;
        frame.lightning_per_hour = 0.0;
        frame.visibility_m = d.visibility_m;
        return;
    }

    let mut acc = Acc { clouds: zero_clouds(), fog: zero_fog(), ..Acc::default() };
    acc.gust_length = (0.0, Wind::default().gust_length);
    let mut surface = SurfaceRates::default();
    let mut lightning_rate = 0.0;
    for (c, wc, blend) in &mut blends {
        *wc /= total;
        let climate = book.c(*c);
        let mut storm = 0.0;
        for (p, w) in blend.iter() {
            let bp = book.p(*p);
            let rec = &bp.record;
            let w = w.map(|x| x * *wc);
            // Continuous channels.
            add_scaled!(acc.clouds, rec.clouds, w[CLOUDS]; coverage, cloud_type, density, wetness, storm, erosion, base, thickness, speed, cirrus, cirrus_height);
            acc.w_clouds += w[CLOUDS];
            acc.haze += rec.sky.haze * by_hour(&rec.sky.haze_by_hour, input.hour_of_day) * w[SKY];
            acc.dust += rec.sky.dust * w[SKY];
            acc.dust_tint += rec.sky.dust_tint * w[SKY];
            acc.dimming += rec.sky.overcast_dimming * w[SKY];
            acc.w_sky += w[SKY];
            let f = &rec.fog;
            let hf = by_hour(&f.height_by_hour, input.hour_of_day);
            let bf = by_hour(&f.banks_by_hour, input.hour_of_day);
            acc.fog.height_density += f.height_density * hf * w[FOG];
            acc.fog.bank_density += f.bank_density * bf * w[FOG];
            add_scaled!(acc.fog, f, w[FOG]; height_falloff, height_base, bank_falloff, bank_coverage, albedo, phase_g, ambient);
            acc.w_fog += w[FOG];
            acc.wind += preset_wind(state, climate.key, bp, input.hours) * w[WIND];
            acc.gustiness += rec.wind.gustiness * w[WIND];
            acc.w_wind += w[WIND];
            if w[WIND] > acc.gust_length.0 {
                acc.gust_length = (w[WIND], rec.wind.gust_length);
            }
            storm += rec.wind.storm * w[WIND];
            // Discrete channels: each preset's contribution on its own.
            for &(kind, intensity) in &bp.precip {
                let x = intensity * w[PRECIP];
                match frame.precip.iter_mut().find(|e| e.kind == kind) {
                    Some(e) => e.intensity += x,
                    None => frame.precip.push(PrecipFrame { kind, intensity: x }),
                }
            }
            add_scaled!(surface, rec.surface, w[PRECIP]; wet, puddle, snow, ash, drying);
            if let Some(l) = &rec.lightning {
                lightning_rate += l.per_hour.min(LIGHTNING_CAP_PER_HOUR) * w[PRECIP];
            }
            for (tag, strength) in &rec.tags {
                add_named(&mut frame.tags, tag, strength * w[PRECIP]);
            }
            for cue in &rec.cues {
                add_cue(&mut frame.cues, &cue.sound, cue.volume * w[CUES], false);
            }
        }
        for cue in &climate.record.cues {
            add_cue(&mut frame.cues, &cue.sound, cue.volume * *wc, true);
        }
        if let Some(source) = climate.record.storm_source
            && storm > 0.0
        {
            frame.storms.push(StormFrame { climate: *c, source, strength: storm });
        }
    }
    frame.precip.retain(|e| e.intensity > 1e-6);
    frame.cues.retain(|e| e.volume > 1e-6);
    frame.tags.retain(|e| e.1 > 1e-6);

    let norm = |w: f32| if w > 0.0 { 1.0 / w } else { 0.0 };
    let (nc, ns, nf, nw) = (norm(acc.w_clouds), norm(acc.w_sky), norm(acc.w_fog), norm(acc.w_wind));
    let wind = Wind { velocity: acc.wind * nw, gustiness: acc.gustiness * nw, gust_length: acc.gust_length.1 };
    let direction = wind.velocity.try_normalize().unwrap_or(Vec2::X);
    let mut clouds = acc.clouds;
    scale!(clouds, nc; coverage, cloud_type, density, wetness, storm, erosion, base, thickness, speed, cirrus, cirrus_height);
    frame.clouds = clouds.to_weather(direction);
    let mut fog = acc.fog;
    scale!(fog, nf; height_density, bank_density, height_falloff, height_base, bank_falloff, bank_coverage, albedo, phase_g, ambient);
    frame.fog = fog.to_settings(wind.velocity);
    frame.sky = Weather {
        cloud_cover: frame.clouds.mean_cover(),
        overcast_dimming: acc.dimming * ns,
        haze: acc.haze * ns,
        dust: acc.dust * ns,
        dust_tint: acc.dust_tint * ns,
        wind,
    };
    frame.surface = surface;
    frame.lightning_per_hour = lightning_rate;

    let mut extinction = frame.fog.extinction_at(input.altitude, 0.0);
    for e in &frame.precip {
        extinction += book.precip_kind(e.kind).extinction.max(0.0) * e.intensity;
    }
    frame.visibility_m = if extinction > 0.0 { (KOSCHMIEDER / extinction).min(MAX_VISIBILITY_M) } else { MAX_VISIBILITY_M };

    for (c, wc, _) in &blends {
        strikes(book, state, *c, *wc, input.prev_hours, input.hours, &mut frame.lightning);
    }
    frame.lightning.sort_by(|a, b| a.time.total_cmp(&b.time));
}

fn zero_clouds() -> CloudTerms {
    CloudTerms {
        coverage: 0.0,
        cloud_type: 0.0,
        density: 0.0,
        wetness: 0.0,
        storm: 0.0,
        erosion: 0.0,
        base: 0.0,
        thickness: 0.0,
        speed: 0.0,
        cirrus: 0.0,
        cirrus_height: 0.0,
    }
}

fn zero_fog() -> FogTerms {
    FogTerms {
        height_density: 0.0,
        height_falloff: 0.0,
        height_base: 0.0,
        bank_density: 0.0,
        bank_falloff: 0.0,
        bank_coverage: 0.0,
        albedo: glam::Vec3::ZERO,
        phase_g: 0.0,
        ambient: 0.0,
        ..FogTerms::default()
    }
}

fn add_named(list: &mut Vec<(RecordId, f32)>, id: &RecordId, x: f32) {
    match list.iter_mut().find(|e| e.0 == *id) {
        Some(e) => e.1 += x,
        None => list.push((id.clone(), x)),
    }
}

fn add_cue(list: &mut Vec<CueFrame>, sound: &RecordId, volume: f32, region: bool) {
    match list.iter_mut().find(|e| e.sound == *sound && e.region == region) {
        Some(e) => e.volume += volume,
        None => list.push(CueFrame { sound: sound.clone(), volume, region }),
    }
}

/// Lightning strike rate of a blend, per game hour (precipitation channel).
fn strike_rate(book: &WeatherBook, blend: &[(PresetIx, [f32; CHANNELS])]) -> (f32, Option<PresetIx>) {
    let mut rate = 0.0;
    let mut heaviest = (0.0, None);
    for (p, w) in blend {
        if let Some(l) = &book.p(*p).record.lightning {
            let r = l.per_hour.min(LIGHTNING_CAP_PER_HOUR) * w[PRECIP];
            rate += r;
            if r > heaviest.0 {
                heaviest = (r, Some(*p));
            }
        }
    }
    (rate, heaviest.1)
}

/// Strikes of `climate` in `[from, to)`: a non-homogeneous Poisson process
/// by thinning (Lewis and Shedler 1979). Candidates arrive at the cap rate,
/// drawn per fixed time bucket from the hash (Poisson count, uniform
/// times); each is kept with probability `rate(t) / cap`. Deterministic and
/// independent of frame boundaries.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
#[allow(clippy::many_single_char_names)]
fn strikes(book: &WeatherBook, state: &WeatherState, climate: ClimateIx, weight: f32, from: f64, to: f64, out: &mut Vec<LightningStrike>) {
    if to <= from || to - from > LIGHTNING_MAX_SPAN_HOURS {
        return;
    }
    let key = book.c(climate).key;
    let cap = f64::from(LIGHTNING_CAP_PER_HOUR);
    let mean = cap * LIGHTNING_BUCKET_HOURS;
    let first = (from / LIGHTNING_BUCKET_HOURS).floor() as i64;
    let last = (to / LIGHTNING_BUCKET_HOURS).floor() as i64;
    for b in first..=last {
        let n = poisson(mean, hash::unit(hash::hash(state.seed, key, salt::STRIKE_COUNT, b as u64)));
        for i in 0..n {
            let h = hash::hash(state.seed, key, salt::STRIKE, (b as u64).wrapping_mul(64).wrapping_add(i));
            let time = (b as f64 + hash::unit(h)) * LIGHTNING_BUCKET_HOURS;
            if time < from || time >= to {
                continue;
            }
            let blend = state.blend(book, climate, time);
            let (rate, heaviest) = strike_rate(book, &blend);
            let Some(p) = heaviest else { continue };
            if hash::unit(hash::mix(h ^ 1)) * cap >= f64::from(rate * weight) {
                continue;
            }
            let l = book.p(p).record.lightning.as_ref().expect("heaviest has lightning");
            let thunder = (!l.thunder.is_empty()).then(|| {
                let j = (hash::mix(h ^ 4) % l.thunder.len() as u64) as usize;
                l.thunder[j].clone()
            });
            let d = hash::unit(hash::mix(h ^ 3)) as f32;
            out.push(LightningStrike {
                time,
                climate,
                azimuth: hash::unit(hash::mix(h ^ 2)) as f32 * TAU,
                distance_m: l.distance_m[0] + (l.distance_m[1] - l.distance_m[0]) * d,
                flash: l.flash,
                thunder,
            });
        }
    }
}

/// Poisson-distributed count with mean `lambda` by CDF inversion (capped at 63).
fn poisson(lambda: f64, u: f64) -> u64 {
    let mut p = (-lambda).exp();
    let mut cdf = p;
    let mut k = 0;
    while u > cdf && k < 63 {
        k += 1;
        #[allow(clippy::cast_precision_loss)]
        let kf = k as f64;
        p *= lambda / kf;
        cdf += p;
    }
    k
}
