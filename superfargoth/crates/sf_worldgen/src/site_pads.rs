//! **Site pads** (spec `sites.md` "WG-P2 pads", flow step 4, slice WG-P2): for every site of a judged list
//! (`sites.valid.json`) with `pad = true` and not dropped, a height stamp (format v1, [`crate::stamps`]) that
//! flattens its footprint to the median ground and fades to nothing across `pad.blend_m`, appended to the guide's
//! `stamps.json`. The UE Landscape Patch / Valheim location flatten technique (spec "Industry comparison").
//!
//! **Shape.** The footprint is the site's ring (three points or more), else its disc ([`crate::reach`]'s `disc_m`).
//! A stamp sample at distance `d` outside the footprint has weight `smootherstep(1 − d / blend)` (1 inside, 0 from
//! `blend` on); every height is the median. The stamp grid sits on multiples of `pad.spacing_m` and spans the
//! footprint's box grown by `blend`, so its border samples weigh 0 and the stamp joins the ground without a seam.
//! **Median:** of the ground at the samples inside the footprint (the lower middle of an even count, a real sample),
//! else the ground at the centre. Heights are the guide's runtime terrain ([`GuideTiles`]) at the coarsest level
//! whose spacing is at most `pad.spacing_m`, with the guide's stamps but none of its pads.
//!
//! **Refusals** (the site stays as judged, with no pad: reachability judged it on the unpadded ground), checked in
//! this order, the first that holds is counted:
//!
//! - [`Refusal::Anchor`]: a sample of weight > 0 of the pad lies where an existing stamp (anchors and every other
//!   stamp of the guide) has weight > 0, or the other way round: anchor interiors stay bit-exact.
//! - [`Refusal::Overlap`]: the same against a pad accepted earlier (sites in file order).
//! - [`Refusal::Water`]: a sample of weight > 0 was water (sea, lake or river band, no margin), lies within
//!   `pad.water_margin_m` of a lake shore or river bank, or ends below the sea level + `pad.water_margin_m`: the pad
//!   neither adds nor removes water (the wet/dry audit).
//! - [`Refusal::Cut`]: the cut or fill (ground after − before) at a sample exceeds `pad.max_cut_m`.
//! - [`Refusal::Band`]: the ground slope after the pad (central differences at `pad.spacing_m`) at a sample of
//!   weight strictly between 0 and 1 exceeds `pad.max_band_deg` (a wall the controller could not walk).
//!
//! **Re-runs.** Pad stamps are named `pad:<site key>` and always sit after every other stamp; a run first drops the
//! trailing pads of the guide (other stamps keep their index: other files key stamps by index), so a second run
//! writes the same bytes. A pad stamp followed by another stamp is an error.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

use sf_world::WorldspaceId;

use super::{SitesFile, Status};
use crate::filter::{Declared, set};
use crate::reach::run::ground_level;
use crate::reach::{disc_m, in_ring};
use crate::scatter::WaterMask;
use crate::sites::Site;
use crate::stamps::{Stamp, StampSet};
use crate::tiles::GuideTiles;
use crate::water::GuideWater;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_site_pads.rs"]
mod tests;

/// Name prefix of a pad stamp.
// tunables: const (format: stamp naming convention)
pub const PAD_PREFIX: &str = "pad:";
/// Water is the water itself when judging what was water before the pad (a definition, not a value).
// tunables: const (definition: the water edge itself)
const WATER_MARGIN_M: f64 = 0.0;

/// The pad values (spec "Tunables", `pad.*`).
#[must_use]
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("pad.blend_m", 8.0, "m", 2.0, 30.0, "Guess", "a pad fades to the natural ground across this band"),
        d("pad.max_cut_m", 3.0, "m", 0.0, 10.0, "Guess", "a pad cutting or filling more than this is refused"),
        d("pad.water_margin_m", 1.0, "m", 0.0, 5.0, "Guess", "a pad keeps this far from water, in height and from shores"),
        d("pad.spacing_m", 2.0, "m", 0.5, 10.0, "Guess(4 samples across the default blend; quality against cost)",
          "sample spacing of a pad stamp"),
        d("pad.max_band_deg", 46.0, "deg", 20.0, 70.0, "Reference(sf_controller MoverParams::max_slope_deg default 46)",
          "a pad whose fade band is steeper than this is refused"),
    ]
}

/// The pad values, checked against their ranges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PadValues {
    pub blend_m: f64,
    pub max_cut_m: f64,
    pub water_margin_m: f64,
    pub spacing_m: f64,
    pub max_band_deg: f64,
}

impl PadValues {
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| Error::Invalid(format!("site-pads: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self {
            blend_m: get("pad.blend_m")?,
            max_cut_m: get("pad.max_cut_m")?,
            water_margin_m: get("pad.water_margin_m")?,
            spacing_m: get("pad.spacing_m")?,
            max_band_deg: get("pad.max_band_deg")?,
        })
    }
}

impl Default for PadValues {
    fn default() -> Self {
        Self::from_declared(&declared()).expect("defaults are valid")
    }
}

/// Why a pad was refused (module docs, "Refusals").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Refusal {
    Anchor,
    Overlap,
    Water,
    Cut,
    Band,
}

impl Refusal {
    /// Every refusal, in check order.
    pub const ALL: [Self; 5] = [Self::Anchor, Self::Overlap, Self::Water, Self::Cut, Self::Band];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Anchor => "anchor",
            Self::Overlap => "overlap",
            Self::Water => "water",
            Self::Cut => "cut",
            Self::Band => "band",
        }
    }
}

/// What a pad run did.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PadReport {
    /// Sites asking for a pad (`pad = true`, not dropped).
    pub asked: usize,
    pub written: usize,
    /// Refused sites by reason.
    pub refused: BTreeMap<Refusal, usize>,
    /// `(site key, reason)` of every refused site, in file order.
    pub refusals: Vec<(String, Refusal)>,
    /// Largest cut or fill of a written pad, m.
    pub cut_max_m: f64,
}

impl fmt::Display for PadReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pads: {} asked, {} written (cut/fill max {:.2} m); refused:", self.asked, self.written, self.cut_max_m)?;
        for r in Refusal::ALL {
            write!(f, " {} {}", r.name(), self.refused.get(&r).copied().unwrap_or(0))?;
        }
        writeln!(f)
    }
}

/// `smootherstep(t)` on [0, 1].
fn smootherstep(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (6.0f64.mul_add(t, -15.0)) + 10.0)
}

/// Distance from `p` to segment `ab`.
fn seg_distance(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let l2 = d[0].mul_add(d[0], d[1] * d[1]);
    let t = if l2 > 0.0 { (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / l2).clamp(0.0, 1.0) } else { 0.0 };
    (p[0] - t.mul_add(d[0], a[0])).hypot(p[1] - t.mul_add(d[1], a[1]))
}

/// A site's footprint: its ring (three points or more) or its disc.
enum Shape {
    Ring(Vec<[f64; 2]>),
    Disc([f64; 2], f64),
}

impl Shape {
    fn of(s: &Site) -> Self {
        if s.footprint.len() >= 3 { Self::Ring(s.footprint.clone()) } else { Self::Disc(s.centre_m, disc_m(s)) }
    }

    /// `[x0, y0, x1, y1]`, x0 west, y0 south.
    fn bbox(&self) -> [f64; 4] {
        match self {
            Self::Ring(r) => r.iter().fold([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY], |b, q| {
                [b[0].min(q[0]), b[1].min(q[1]), b[2].max(q[0]), b[3].max(q[1])]
            }),
            Self::Disc(c, r) => [c[0] - r, c[1] - r, c[0] + r, c[1] + r],
        }
    }

    /// Distance outside the footprint (0 inside).
    fn outside(&self, p: [f64; 2]) -> f64 {
        match self {
            Self::Ring(r) => {
                if in_ring(r, p) {
                    return 0.0;
                }
                (0..r.len()).map(|i| seg_distance(r[i], r[(i + 1) % r.len()], p)).fold(f64::INFINITY, f64::min)
            }
            Self::Disc(c, rad) => ((p[0] - c[0]).hypot(p[1] - c[1]) - rad).max(0.0),
        }
    }
}

/// Guide-frame position of stamp sample `k`.
fn sample_pos(s: &Stamp, k: usize) -> [f64; 2] {
    let w = s.dims[0];
    [s.origin_m[0] + (k % w) as f64 * s.spacing_m, s.origin_m[1] - (k / w) as f64 * s.spacing_m]
}

/// The first sample of weight > 0 of `a` inside `b`'s rectangle where `b` samples a weight > 0 (only `a`'s samples
/// in `b`'s rectangle are visited).
fn overlap_in(a: &Stamp, b: &Stamp) -> bool {
    let (ra, rb) = (a.rect_m(), b.rect_m());
    if ra[2] < rb[0] || rb[2] < ra[0] || ra[3] < rb[1] || rb[3] < ra[1] {
        return false;
    }
    let [w, h] = a.dims;
    let s = a.spacing_m;
    let col = |x: f64| ((x - a.origin_m[0]) / s).clamp(0.0, (w - 1) as f64);
    let row = |y: f64| ((a.origin_m[1] - y) / s).clamp(0.0, (h - 1) as f64);
    let (c0, c1) = (col(rb[0]).floor() as usize, col(rb[2]).ceil() as usize);
    let (r0, r1) = (row(rb[3]).floor() as usize, row(rb[1]).ceil() as usize);
    (r0..=r1).any(|r| {
        (c0..=c1).any(|c| {
            let k = r * w + c;
            let p = sample_pos(a, k);
            a.weights[k] > 0.0 && b.sample(p[0], p[1]).is_some_and(|(_, wb)| wb > 0.0)
        })
    })
}

fn overlaps(a: &Stamp, b: &Stamp) -> bool {
    overlap_in(a, b) || overlap_in(b, a)
}

/// The pad stamp of site `s` with every height 0 (heights come once the median is known).
fn shape_stamp(s: &Site, v: &PadValues) -> Result<(Stamp, Vec<bool>)> {
    let shape = Shape::of(s);
    let b = shape.bbox();
    let (sp, bl) = (v.spacing_m, v.blend_m);
    let x0 = ((b[0] - bl) / sp).floor() * sp;
    let x1 = ((b[2] + bl) / sp).ceil() * sp;
    let y0 = ((b[1] - bl) / sp).floor() * sp;
    let y1 = ((b[3] + bl) / sp).ceil() * sp;
    if !(x0.is_finite() && x1.is_finite() && y0.is_finite() && y1.is_finite()) {
        return Err(Error::Invalid(format!("site {:?}: footprint not finite", s.key)));
    }
    let w = ((x1 - x0) / sp).round() as usize + 1;
    let h = ((y1 - y0) / sp).round() as usize + 1;
    let mut st = Stamp {
        name: format!("{PAD_PREFIX}{}", s.key),
        origin_m: [x0, y1],
        dims: [w, h],
        spacing_m: sp,
        heights: vec![0.0; w * h],
        weights: vec![0.0; w * h],
    };
    let dist: Vec<f64> = (0..w * h).map(|k| shape.outside(sample_pos(&st, k))).collect();
    st.weights = dist.iter().map(|&d| if d <= 0.0 { 1.0 } else { smootherstep(1.0 - d / bl) as f32 }).collect();
    Ok((st, dist.iter().map(|&d| d <= 0.0).collect()))
}

/// Pads for the sites of `file` on `tiles` (the guide with its stamps, no pads) and `water` (no margin): the pad
/// stamps in site order and the report (module docs).
pub fn pads(tiles: &GuideTiles, water: &WaterMask, file: &SitesFile, v: &PadValues) -> Result<(StampSet, PadReport)> {
    let level = ground_level(v.spacing_m);
    let sea = tiles.guide().sea_level_m;
    let max_tan = v.max_band_deg.to_radians().tan();
    let existing: &[Stamp] = tiles.stamps().map_or(&[], |i| &i.set().stamps);
    let mut out = StampSet::default();
    let mut rep = PadReport::default();
    let ground = |t: &GuideTiles, p: [f64; 2]| f64::from(t.height(p[0], -p[1], level));
    for s in file.sites.iter().filter(|s| s.pad && s.status != Some(Status::Dropped)) {
        rep.asked += 1;
        let (mut st, inside) = shape_stamp(s, v)?;
        let n = st.weights.len();
        let refuse = |rep: &mut PadReport, r: Refusal| {
            *rep.refused.entry(r).or_insert(0) += 1;
            rep.refusals.push((s.key.clone(), r));
        };
        if existing.iter().any(|e| overlaps(&st, e)) {
            refuse(&mut rep, Refusal::Anchor);
            continue;
        }
        if out.stamps.iter().any(|e| overlaps(&st, e)) {
            refuse(&mut rep, Refusal::Overlap);
            continue;
        }
        let before: Vec<f64> = (0..n).map(|k| ground(tiles, sample_pos(&st, k))).collect();
        let mut inner: Vec<f64> = (0..n).filter(|&k| inside[k]).map(|k| before[k]).collect();
        inner.sort_by(f64::total_cmp);
        let median = match inner.len() {
            0 => ground(tiles, s.centre_m),
            m => inner[(m - 1) / 2],
        } as f32;
        st.heights.fill(median);
        let solo = tiles.with_params(*tiles.params()).with_stamps(StampSet { stamps: vec![st.clone()] });
        let live = |k: usize| st.weights[k] > 0.0;
        let after: Vec<f64> = (0..n).map(|k| if live(k) { ground(&solo, sample_pos(&st, k)) } else { before[k] }).collect();
        let wet = (0..n).filter(|&k| live(k)).any(|k| {
            let p = sample_pos(&st, k);
            water.wet(p[0], p[1], before[k])
                || water.distance_within_m(p[0], p[1], v.water_margin_m).is_some()
                || after[k] < sea + v.water_margin_m
        });
        if wet {
            refuse(&mut rep, Refusal::Water);
            continue;
        }
        let cut = (0..n).filter(|&k| live(k)).map(|k| (after[k] - before[k]).abs()).fold(0.0, f64::max);
        if cut > v.max_cut_m {
            refuse(&mut rep, Refusal::Cut);
            continue;
        }
        let steep = (0..n).filter(|&k| live(k) && st.weights[k] < 1.0).any(|k| band_tan(&solo, level, sample_pos(&st, k), v.spacing_m) > max_tan);
        if steep {
            refuse(&mut rep, Refusal::Band);
            continue;
        }
        rep.cut_max_m = rep.cut_max_m.max(cut);
        rep.written += 1;
        out.stamps.push(st);
    }
    Ok((out, rep))
}

/// Tan of the ground slope at `p` on `t` (central differences across `ds` either side).
#[must_use]
pub fn band_tan(t: &GuideTiles, level: u8, p: [f64; 2], ds: f64) -> f64 {
    let h = |x: f64, y: f64| f64::from(t.height(x, -y, level));
    let gx = (h(p[0] + ds, p[1]) - h(p[0] - ds, p[1])) / (2.0 * ds);
    let gy = (h(p[0], p[1] + ds) - h(p[0], p[1] - ds)) / (2.0 * ds);
    gx.hypot(gy)
}

/// `set` without its trailing pad stamps; a pad stamp followed by another stamp is an error.
pub fn strip_pads(mut set: StampSet) -> Result<StampSet> {
    let keep = set.stamps.iter().rposition(|s| !s.name.starts_with(PAD_PREFIX)).map_or(0, |i| i + 1);
    if let Some(p) = set.stamps[..keep].iter().find(|s| s.name.starts_with(PAD_PREFIX)) {
        return Err(Error::Invalid(format!("stamp {:?} is a pad followed by other stamps (pads go last)", p.name)));
    }
    set.stamps.truncate(keep);
    Ok(set)
}

/// One `site-pads` run.
#[derive(Debug, Clone)]
pub struct PadRequest {
    /// Guide folder (v1): heights, stamps, `water.json`; the pads are written into its `stamps.json`.
    pub guide: PathBuf,
    /// Judged site list (format 2), by convention [`crate::reach::VALID_FILE`] in the guide folder.
    pub sites: PathBuf,
    /// `pad.*` overrides `(name, value)`, source Guess.
    pub sets: Vec<(String, f64)>,
}

/// Reads the guide and the site list, drops the guide's old pads, writes the new ones after its other stamps.
pub fn run_pads(req: &PadRequest) -> Result<PadReport> {
    let mut d = declared();
    for (k, val) in &req.sets {
        if !k.starts_with("pad.") {
            return Err(Error::Invalid(format!("--set {k}: not a pad value")));
        }
        set(&mut d, k, *val, None)?;
    }
    let v = PadValues::from_declared(&d)?;
    let file = SitesFile::read(&req.sites)?;
    let had = StampSet::read(&req.guide)?;
    let had_file = had.is_some();
    let base = strip_pads(had.unwrap_or_default())?;
    let tiles = GuideTiles::open(&req.guide, WorldspaceId::new("site-pads"))?.with_stamps(base.clone());
    let water = match GuideWater::read(&req.guide)? {
        Some(w) => WaterMask::new(&w, WATER_MARGIN_M),
        None => WaterMask::sea(tiles.guide().sea_level_m, WATER_MARGIN_M),
    };
    let (pads, rep) = pads(&tiles, &water, &file, &v)?;
    // A guide with no stamps and no pad stays without a stamps file.
    if had_file || !pads.stamps.is_empty() {
        let mut all = base;
        all.append(pads)?;
        all.write(&req.guide)?;
    }
    Ok(rep)
}
