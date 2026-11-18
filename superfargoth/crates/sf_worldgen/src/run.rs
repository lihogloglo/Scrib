//! The recipe runner: placement (stage 1), macro form (stage 2: stream power, then thermal), previews.
//!
//! Placement: samples whose region name starts with a `mask_to_sea` prefix become sea (the guide's `fill_m`), the
//! grid is cropped to the remaining land's bounding box plus `sea_margin_m`, and the macro grid samples the guide
//! by Catmull-Rom at `S × guide position`, with each sample's height remapped by its region's relief profile
//! (`relief × h`, plus an optional power boost to a target peak), blended across regions by a cubic B-spline over
//! the region grid. Every arm on the same `regional` flag shares one placement.
//!
//! Outputs per arm (`<output>/<arm>/`): `heights.f32` + `meta.json` (guide format v1, so a product reads back as
//! a guide), `overview.png`, `crop-<name>.png`; then `sheet-<crop>.png` and `sheet-overview.png` (arms left to
//! right in recipe order) and `README.md` with the numbers.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

use crate::guide::{Guide, RegionMap};
use crate::recipe::{Recipe, Resolved};
use crate::resample::{bicubic, bspline_weights};
use crate::shade::{Gray, Light, contact_sheet, downsample, hillshade, write_png};
use crate::stream::{StreamParams, StreamStats, erode};
use crate::thermal::{ThermalParams, relax};
use crate::{Audit, Error, Grid, Result, WetDry, hash_f32};

/// Contact-sheet gap, px, and its gray (layout).
const SHEET_GAP_PX: usize = 8; // tunables: const (sheet layout, fixed by the look-test design)
const SHEET_GAP_GRAY: u8 = 255; // tunables: const (sheet layout: white gaps)

/// Command-line overrides.
#[derive(Debug, Clone, Default)]
pub struct Overrides {
    /// Only these arms (by name).
    pub arms: Option<Vec<String>>,
    /// Macro spacing override, metres (coarsening when an arm is too slow).
    pub macro_spacing_m: Option<f64>,
    /// Stream iterations override.
    pub iterations: Option<usize>,
    /// Output folder override.
    pub out: Option<PathBuf>,
    /// Skip previews.
    pub no_images: bool,
}

/// The macro frame: guide → world mapping of the cropped grid.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Frame {
    pub scale: f64,
    pub spacing_m: f64,
    /// World metres of macro sample (0, 0): x east, y north (guide frame × S).
    pub origin_m: [f64; 2],
    pub w: usize,
    pub h: usize,
}

impl Frame {
    /// World position of macro sample (i, j).
    pub fn world(&self, i: f64, j: f64) -> [f64; 2] {
        [
            self.origin_m[0] + i * self.spacing_m,
            self.origin_m[1] - j * self.spacing_m,
        ]
    }
    /// Fractional macro sample of a world position.
    pub fn sample(&self, p: [f64; 2]) -> [f64; 2] {
        [
            (p[0] - self.origin_m[0]) / self.spacing_m,
            (self.origin_m[1] - p[1]) / self.spacing_m,
        ]
    }
}

/// A crop window, resolved.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CropAt {
    pub name: String,
    pub rule: String,
    /// Centre, world metres.
    pub centre_m: [f64; 2],
    /// Centre, guide metres (before scaling).
    pub centre_guide_m: [f64; 2],
    /// Region-grid pixel of the centre.
    pub cell_px: [i64; 2],
}

/// Numbers of one arm.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ArmReport {
    pub name: String,
    pub help: String,
    pub placement_s: f64,
    pub erosion_s: f64,
    pub thermal_s: f64,
    pub render_s: f64,
    pub stream: Option<StreamStats>,
    pub thermal_iterations: usize,
    pub thermal_moved_m3: f64,
    pub max_height_m: f64,
    pub before: WetDry,
    pub after: WetDry,
    pub audit_erosion: Audit,
    pub audit_thermal: Audit,
    pub heights_blake3: String,
}

struct Placement {
    intent: Grid,
    k: Vec<f64>,
    restore: Vec<f64>,
    tan_talus: Vec<f64>,
    seconds: f64,
}

fn masked_source(guide: &Guide, recipe: &Recipe) -> (Grid, usize) {
    let fill = guide
        .fill_m
        .unwrap_or_else(|| f64::from(guide.heights.iter().copied().fold(f32::INFINITY, f32::min)));
    let mut g = Grid {
        w: guide.w,
        h: guide.h,
        spacing: guide.spacing_m,
        data: guide.heights.iter().map(|&v| f64::from(v)).collect(),
    };
    let mut masked = 0;
    if let Some(reg) = &guide.regions {
        let masked_idx: Vec<bool> = reg
            .names
            .iter()
            .map(|n| {
                !n.is_empty()
                    && recipe
                        .place
                        .mask_to_sea
                        .iter()
                        .any(|p| n.starts_with(p.as_str()))
            })
            .collect();
        for row in 0..g.h {
            for col in 0..g.w {
                if masked_idx[usize::from(reg.at_sample(col, row, g.spacing))] {
                    g.data[row * g.w + col] = fill.min(guide.sea_level_m - f64::EPSILON);
                    masked += 1;
                }
            }
        }
    }
    (g, masked)
}

/// Region weights at guide position (col, row) (fractional samples): up to 16 (legend index, weight) pairs.
fn region_weights(reg: &RegionMap, spacing: f64, col: f64, row: f64, out: &mut Vec<(u8, f64)>) {
    out.clear();
    let px = col * spacing / reg.cell_m - 0.5;
    let py = row * spacing / reg.cell_m - 0.5;
    let (fx, fy) = (px.floor(), py.floor());
    let wx = bspline_weights(px - fx);
    let wy = bspline_weights(py - fy);
    for (j, wyj) in wy.iter().enumerate() {
        for (i, wxi) in wx.iter().enumerate() {
            let idx = reg.at_clamped(fx as isize + i as isize - 1, fy as isize + j as isize - 1);
            let wgt = wxi * wyj;
            if let Some(e) = out.iter_mut().find(|e| e.0 == idx) {
                e.1 += wgt;
            } else {
                out.push((idx, wgt));
            }
        }
    }
}

/// Relief profile of one region: `relief × h`, plus `c × (h - base)^power` above `base` so the region's highest
/// guide sample lands on `peak`.
fn profile(r: &Resolved, h_max: f64, h: f64) -> f64 {
    let mut v = r.relief * h;
    if let Some((peak, base, power)) = r.boost
        && h > base
        && h_max > base
    {
        let c = (peak - r.relief * h_max) / (h_max - base).powf(power);
        v += c * (h - base).powf(power);
    }
    v
}

fn place(guide: &Guide, recipe: &Recipe, src: &Grid, frame: &Frame, regional: bool) -> Placement {
    let t = Instant::now();
    let p = &recipe.params;
    let s = frame.scale;
    let default = recipe.resolve("default");
    let reg = guide.regions.as_ref().filter(|_| regional);
    // Resolved values and highest guide sample per legend index.
    let resolved: Vec<Resolved> = (0..256)
        .map(|i| {
            reg.map_or(default, |r| {
                if r.names[i].is_empty() {
                    default
                } else {
                    recipe.resolve(&r.names[i])
                }
            })
        })
        .collect();
    let mut h_max = vec![f64::NEG_INFINITY; 256];
    if let Some(r) = reg {
        for row in 0..src.h {
            for col in 0..src.w {
                let i = usize::from(r.at_sample(col, row, src.spacing));
                h_max[i] = h_max[i].max(src.data[row * src.w + col]);
            }
        }
    }
    let n = frame.w * frame.h;
    let mut intent = vec![0.0; n];
    let mut k = vec![0.0; n];
    let mut restore = vec![0.0; n];
    let mut tan_talus = vec![0.0; n];
    let (k0, r0) = (p.stream_k_dt.v, p.stream_restore.v);
    intent
        .par_chunks_mut(frame.w)
        .zip(k.par_chunks_mut(frame.w))
        .zip(restore.par_chunks_mut(frame.w))
        .zip(tan_talus.par_chunks_mut(frame.w))
        .enumerate()
        .for_each(|(j, (((ir, kr), rr), tr))| {
            let mut wts = Vec::with_capacity(16);
            for x in 0..frame.w {
                let [wx, wy] = frame.world(x as f64, j as f64);
                let col = (wx / s - guide.origin_m[0]) / src.spacing;
                let row = (guide.origin_m[1] - wy / s) / src.spacing;
                let h = bicubic(src, col, row);
                if let Some(r) = reg {
                    region_weights(r, src.spacing, col, row, &mut wts);
                } else {
                    wts.clear();
                    wts.push((0, 1.0));
                }
                let (mut hi, mut ki, mut ri, mut ti) = (0.0, 0.0, 0.0, 0.0);
                for &(idx, wgt) in &wts {
                    let rv = &resolved[usize::from(idx)];
                    hi += wgt * profile(rv, h_max[usize::from(idx)], h);
                    ki += wgt * rv.erodibility;
                    ri += wgt * rv.uplift;
                    ti += wgt * rv.talus_deg;
                }
                // Keep the wet/dry class of the resampled guide exactly (a blend of same-sign profiles keeps
                // the sign; this guards the rounding at zero).
                if h < guide.sea_level_m {
                    hi = hi.min(guide.sea_level_m - f64::EPSILON);
                } else {
                    hi = hi.max(guide.sea_level_m);
                }
                ir[x] = hi;
                kr[x] = k0 * ki;
                rr[x] = (r0 * ri).clamp(0.0, 1.0);
                tr[x] = ti.to_radians().tan();
            }
        });
    Placement {
        intent: Grid {
            w: frame.w,
            h: frame.h,
            spacing: frame.spacing_m,
            data: intent,
        },
        k,
        restore,
        tan_talus,
        seconds: t.elapsed().as_secs_f64(),
    }
}

fn frame_for(guide: &Guide, recipe: &Recipe, src: &Grid, spacing_m: f64) -> Result<Frame> {
    let (mut c0, mut r0, mut c1, mut r1) = (usize::MAX, usize::MAX, 0, 0);
    for row in 0..src.h {
        for col in 0..src.w {
            if src.data[row * src.w + col] >= guide.sea_level_m {
                c0 = c0.min(col);
                r0 = r0.min(row);
                c1 = c1.max(col);
                r1 = r1.max(row);
            }
        }
    }
    if c0 > c1 {
        return Err(Error::Invalid("no land left after masking".into()));
    }
    let s = recipe.params.scale.v;
    let margin = recipe.params.sea_margin_m.v;
    let x0 = s * (guide.origin_m[0] + c0 as f64 * src.spacing) - margin;
    let y0 = s * (guide.origin_m[1] - r0 as f64 * src.spacing) + margin;
    let x1 = s * (guide.origin_m[0] + c1 as f64 * src.spacing) + margin;
    let y1 = s * (guide.origin_m[1] - r1 as f64 * src.spacing) - margin;
    let w = ((x1 - x0) / spacing_m).floor() as usize + 1;
    let h = ((y0 - y1) / spacing_m).floor() as usize + 1;
    Ok(Frame {
        scale: s,
        spacing_m,
        origin_m: [x0, y0],
        w,
        h,
    })
}

fn resolve_crops(guide: &Guide, recipe: &Recipe, src: &Grid) -> Result<Vec<CropAt>> {
    let s = recipe.params.scale.v;
    let reg = guide
        .regions
        .as_ref()
        .ok_or_else(|| Error::Invalid("crops need a region grid".into()))?;
    let mut out = Vec::new();
    for c in &recipe.crops {
        let (rule, gx, gy) = if let Some(name) = &c.peak_of {
            let idx = reg
                .find(name)
                .ok_or_else(|| Error::Invalid(format!("no region named {name:?}")))?;
            let mut best = (f64::NEG_INFINITY, 0, 0);
            for row in 0..src.h {
                for col in 0..src.w {
                    let v = src.data[row * src.w + col];
                    if reg.at_sample(col, row, src.spacing) == idx && v > best.0 {
                        best = (v, col, row);
                    }
                }
            }
            (
                format!("highest sample of {name:?} ({:.1} m guide)", best.0),
                guide.origin_m[0] + best.1 as f64 * src.spacing,
                guide.origin_m[1] - best.2 as f64 * src.spacing,
            )
        } else {
            let name = c.region.as_deref().unwrap_or_default();
            let idx = reg
                .find(name)
                .ok_or_else(|| Error::Invalid(format!("no region named {name:?}")))?;
            // Window of the crop's size on the region grid holding the most pixels of the region; first in
            // row-major order on ties.
            let win = ((recipe.params.crop_size_m.v / s / reg.cell_m).round() as usize).max(1);
            let mut best = (0usize, 0usize, 0usize);
            for py in 0..=reg.h.saturating_sub(win) {
                for px in 0..=reg.w.saturating_sub(win) {
                    let mut cnt = 0;
                    for y in py..py + win {
                        for &v in &reg.index[y * reg.w + px..y * reg.w + px + win] {
                            cnt += usize::from(v == idx);
                        }
                    }
                    if cnt > best.0 {
                        best = (cnt, px, py);
                    }
                }
            }
            let cx = (best.1 as f64 + win as f64 / 2.0) * reg.cell_m;
            let cy = (best.2 as f64 + win as f64 / 2.0) * reg.cell_m;
            (
                format!(
                    "{win}x{win}-pixel window with the most {name:?} pixels ({} of {})",
                    best.0,
                    win * win
                ),
                guide.origin_m[0] + cx,
                guide.origin_m[1] - cy,
            )
        };
        let cell_px = [
            ((gx - guide.origin_m[0]) / reg.cell_m).floor() as i64,
            ((guide.origin_m[1] - gy) / reg.cell_m).floor() as i64,
        ];
        out.push(CropAt {
            name: c.name.clone(),
            rule,
            centre_m: [gx * s, gy * s],
            centre_guide_m: [gx, gy],
            cell_px,
        });
    }
    Ok(out)
}

fn render_crop(
    g: &Grid,
    frame: &Frame,
    crop: &CropAt,
    recipe: &Recipe,
    sea: f64,
    light: Light,
) -> Gray {
    let px = recipe.params.crop_px.v as usize;
    let size = recipe.params.crop_size_m.v;
    let step = size / px as f64;
    let mut data = vec![0.0; px * px];
    data.par_chunks_mut(px).enumerate().for_each(|(j, row)| {
        for (i, v) in row.iter_mut().enumerate() {
            let wx = crop.centre_m[0] - size / 2.0 + (i as f64 + 0.5) * step;
            let wy = crop.centre_m[1] + size / 2.0 - (j as f64 + 0.5) * step;
            let [sx, sy] = frame.sample([wx, wy]);
            *v = bicubic(g, sx, sy);
        }
    });
    hillshade(
        &Grid {
            w: px,
            h: px,
            spacing: step,
            data,
        },
        sea,
        light,
    )
}

fn write_heights(
    dir: &Path,
    g: &Grid,
    frame: &Frame,
    sea: f64,
    extra: &serde_json::Value,
) -> Result<()> {
    let mut bytes = Vec::with_capacity(g.len() * 4);
    for &v in &g.data {
        bytes.extend_from_slice(&(v as f32).to_le_bytes());
    }
    std::fs::write(dir.join("heights.f32"), bytes)?;
    let meta = serde_json::json!({
        "format": 1,
        "dims": [g.w, g.h],
        "sample_spacing_m": g.spacing,
        "sea_level_m": sea,
        "origin_m": frame.origin_m,
        "producer": "sf_worldgen scrib-worldgen",
        "report": extra,
    });
    std::fs::write(dir.join("meta.json"), serde_json::to_string_pretty(&meta)?)?;
    Ok(())
}

/// Runs a recipe. Returns the arm reports.
#[allow(clippy::too_many_lines)]
pub fn run(recipe_path: &Path, ov: &Overrides) -> Result<Vec<ArmReport>> {
    let recipe = Recipe::read(recipe_path)?;
    let p = &recipe.params;
    let mut guide = Guide::read(&recipe.guide.dir)?;
    if let Some(o) = recipe.guide.origin_m {
        guide.origin_m = o;
    }
    if let Some(reg) = &guide.regions {
        for name in recipe.regions.keys().filter(|k| *k != "default") {
            if reg.find(name).is_none() {
                return Err(Error::Invalid(format!(
                    "[regions.{name:?}]: no such name in the guide's legend"
                )));
            }
        }
    }
    let sea = guide.sea_level_m;
    let out_dir = ov.out.clone().unwrap_or_else(|| recipe.output.dir.clone());
    std::fs::create_dir_all(&out_dir)?;
    let (src, masked) = masked_source(&guide, &recipe);
    let spacing = ov.macro_spacing_m.unwrap_or(p.macro_spacing_m.v);
    let frame = frame_for(&guide, &recipe, &src, spacing)?;
    let crops = resolve_crops(&guide, &recipe, &src)?;
    let light = Light {
        azimuth_deg: p.light_azimuth_deg.v,
        altitude_deg: p.light_altitude_deg.v,
        sea_gray: p.sea_gray.v as u8,
    };
    let iterations = ov.iterations.unwrap_or(p.stream_iterations.v as usize);
    println!(
        "scrib-worldgen: guide {}x{} @ {} m, masked {masked} samples; macro {}x{} @ {spacing} m ({:.1} M samples), S = {}",
        guide.w,
        guide.h,
        guide.spacing_m,
        frame.w,
        frame.h,
        (frame.w * frame.h) as f64 * 1e-6,
        frame.scale
    );
    let mut placements: [Option<Placement>; 2] = [None, None];
    let mut reports = Vec::new();
    let mut overviews: Vec<Gray> = Vec::new();
    let mut crop_imgs: Vec<Vec<Gray>> = vec![Vec::new(); crops.len()];
    let mut arm_names = Vec::new();
    for arm in &recipe.arms {
        if ov.arms.as_ref().is_some_and(|a| !a.contains(&arm.name)) {
            continue;
        }
        let slot = usize::from(arm.regional);
        let mut place_s = 0.0;
        if placements[slot].is_none() {
            let pl = place(&guide, &recipe, &src, &frame, arm.regional);
            place_s = pl.seconds;
            placements[slot] = Some(pl);
        }
        let pl = placements[slot].as_ref().expect("placed above");
        let mut g = pl.intent.clone();
        let before = WetDry::count(&g.data, sea);
        let t = Instant::now();
        let mut stream = None;
        let mut audit_erosion = Audit::default();
        if arm.erosion {
            let start = g.data.clone();
            stream = Some(erode(
                &mut g,
                &pl.intent.data,
                &StreamParams {
                    m: p.stream_m.v,
                    iterations,
                    k_dt: &pl.k,
                    restore: &pl.restore,
                    sea_level: sea,
                },
            )?);
            audit_erosion = Audit::compare(&start, &g.data, sea);
        }
        let erosion_s = t.elapsed().as_secs_f64();
        let t = Instant::now();
        let mut audit_thermal = Audit::default();
        let (mut thermal_iterations, mut moved) = (0, 0.0);
        if arm.thermal {
            let start = g.data.clone();
            thermal_iterations = p.thermal_iterations.v as usize;
            moved = relax(
                &mut g,
                &ThermalParams {
                    iterations: thermal_iterations,
                    rate: p.thermal_rate.v,
                    tan_talus: &pl.tan_talus,
                    sea_level: sea,
                },
            )?;
            audit_thermal = Audit::compare(&start, &g.data, sea);
        }
        let thermal_s = t.elapsed().as_secs_f64();
        let after = WetDry::count(&g.data, sea);
        let max_height_m = g.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mut rep = ArmReport {
            name: arm.name.clone(),
            help: arm.help.clone(),
            placement_s: place_s,
            erosion_s,
            thermal_s,
            render_s: 0.0,
            stream,
            thermal_iterations,
            thermal_moved_m3: moved,
            max_height_m,
            before,
            after,
            audit_erosion,
            audit_thermal,
            heights_blake3: hash_f32(&g.data),
        };
        let dir = out_dir.join(&arm.name);
        std::fs::create_dir_all(&dir)?;
        write_heights(&dir, &g, &frame, sea, &serde_json::to_value(&rep)?)?;
        if !ov.no_images {
            let t = Instant::now();
            let full = hillshade(&g, sea, light);
            let factor = full.w.div_ceil(p.overview_max_px.v as usize);
            let ovw = downsample(&full, factor);
            write_png(&dir.join("overview.png"), &ovw)?;
            overviews.push(ovw);
            for (ci, crop) in crops.iter().enumerate() {
                let img = render_crop(&g, &frame, crop, &recipe, sea, light);
                write_png(&dir.join(format!("crop-{}.png", crop.name)), &img)?;
                crop_imgs[ci].push(img);
            }
            rep.render_s = t.elapsed().as_secs_f64();
        }
        println!(
            "  {}: place {:.1} s, erosion {:.1} s, thermal {:.1} s, render {:.1} s; max {:.1} m; land {} -> {}, sea {} -> {}; pits after routing {}",
            rep.name,
            rep.placement_s,
            rep.erosion_s,
            rep.thermal_s,
            rep.render_s,
            rep.max_height_m,
            rep.before.land,
            rep.after.land,
            rep.before.sea,
            rep.after.sea,
            rep.stream.as_ref().map_or(0, |s| s.pits_after_routing)
        );
        arm_names.push(arm.name.clone());
        reports.push(rep);
    }
    if !ov.no_images && !overviews.is_empty() {
        let refs: Vec<&Gray> = overviews.iter().collect();
        write_png(
            &out_dir.join("sheet-overview.png"),
            &contact_sheet(&refs, SHEET_GAP_PX, SHEET_GAP_GRAY),
        )?;
        for (ci, crop) in crops.iter().enumerate() {
            let refs: Vec<&Gray> = crop_imgs[ci].iter().collect();
            write_png(
                &out_dir.join(format!("sheet-{}.png", crop.name)),
                &contact_sheet(&refs, SHEET_GAP_PX, SHEET_GAP_GRAY),
            )?;
        }
    }
    std::fs::write(
        out_dir.join("README.md"),
        readme(
            &recipe,
            recipe_path,
            &guide,
            &frame,
            masked,
            iterations,
            &crops,
            &reports,
            overviews.first(),
        ),
    )?;
    Ok(reports)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn readme(
    recipe: &Recipe,
    recipe_path: &Path,
    guide: &Guide,
    frame: &Frame,
    masked: usize,
    iterations: usize,
    crops: &[CropAt],
    reports: &[ArmReport],
    overview: Option<&Gray>,
) -> String {
    let p = &recipe.params;
    let census = recipe.census();
    let mut s = String::new();
    let _ = writeln!(s, "# scrib-worldgen run\n");
    let _ = writeln!(
        s,
        "Recipe `{}` ({} declared values, {} `Guess`).",
        recipe_path.display(),
        census.values,
        census.guesses
    );
    let _ = writeln!(
        s,
        "Guide `{}`: {}x{} samples at {} m; {masked} samples masked to sea (`mask_to_sea` = {:?}).",
        recipe.guide.dir.display(),
        guide.w,
        guide.h,
        guide.spacing_m,
        recipe.place.mask_to_sea
    );
    let _ = writeln!(
        s,
        "Macro grid: {}x{} samples ({:.2} M) at {} m, S = {}, origin (NW sample) at ({:.1}, {:.1}) m world, {:.1} x {:.1} km.",
        frame.w,
        frame.h,
        (frame.w * frame.h) as f64 * 1e-6,
        frame.spacing_m,
        frame.scale,
        frame.origin_m[0],
        frame.origin_m[1],
        (frame.w - 1) as f64 * frame.spacing_m / 1000.0,
        (frame.h - 1) as f64 * frame.spacing_m / 1000.0
    );
    let _ = writeln!(
        s,
        "Stream power: m = {}, n = 1, K dt = {} (x erodibility), restore = {} per step (x uplift), {iterations} steps. Thermal: {} steps, rate {}.",
        p.stream_m.v, p.stream_k_dt.v, p.stream_restore.v, p.thermal_iterations.v, p.thermal_rate.v
    );
    let _ = writeln!(
        s,
        "Light: azimuth {} deg, altitude {} deg, true vertical scale; sea drawn at gray {}. World frame = guide frame x S (x east, y north).\n",
        p.light_azimuth_deg.v, p.light_altitude_deg.v, p.sea_gray.v
    );
    let _ = writeln!(s, "## Arms (left to right on every sheet)\n");
    for (i, r) in reports.iter().enumerate() {
        let _ = writeln!(s, "{}. **{}**: {}", i + 1, r.name, r.help);
    }
    if let Some(o) = overview {
        let _ = writeln!(
            s,
            "\nSheets: `sheet-overview.png` (each panel {}x{} px), and one `sheet-<crop>.png` per crop ({} px per panel), {} px gaps.",
            o.w, o.h, p.crop_px.v, SHEET_GAP_PX
        );
    }
    let _ = writeln!(
        s,
        "\n## Crops ({:.0} x {:.0} km, {} px)\n",
        p.crop_size_m.v / 1000.0,
        p.crop_size_m.v / 1000.0,
        p.crop_px.v
    );
    let label = recipe
        .report
        .as_ref()
        .map_or("region-grid pixel", |r| r.cell_label.as_str());
    let _ = writeln!(
        s,
        "| Crop | Centre, world m (x, y) | Centre, guide m | {label} | Rule |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|");
    for c in crops {
        let cell = recipe.report.as_ref().map_or(c.cell_px, |r| {
            [
                r.cell_origin[0] + c.cell_px[0] * r.cell_step[0],
                r.cell_origin[1] + c.cell_px[1] * r.cell_step[1],
            ]
        });
        let _ = writeln!(
            s,
            "| {} | ({:.0}, {:.0}) | ({:.1}, {:.1}) | ({}, {}) | {} |",
            c.name,
            c.centre_m[0],
            c.centre_m[1],
            c.centre_guide_m[0],
            c.centre_guide_m[1],
            cell[0],
            cell[1],
            c.rule
        );
    }
    let _ = writeln!(s, "\n## Numbers per arm\n");
    let _ = writeln!(
        s,
        "| Arm | Wall s (place / erosion / thermal / render) | Iterations (stream / thermal) | Max height m | Land before -> after | Sea before -> after | Pits after routing | Raw closed pits | Incision m (mean / p90 / p99 / max) | Max drainage km² | Wet/dry audit (new wet, new dry, sea moved) | heights blake3 |"
    );
    let _ = writeln!(s, "|---|---|---|---|---|---|---|---|---|---|---|---|");
    for r in reports {
        let st = r.stream.clone().unwrap_or_default();
        let a = (
            r.audit_erosion.new_wet + r.audit_thermal.new_wet,
            r.audit_erosion.new_dry + r.audit_thermal.new_dry,
            r.audit_erosion.sea_moved + r.audit_thermal.sea_moved,
        );
        let _ = writeln!(
            s,
            "| {} | {:.1} / {:.1} / {:.1} / {:.1} | {} / {} | {:.1} | {} -> {} | {} -> {} | {} | {} | {:.2} / {:.1} / {:.1} / {:.1} | {:.1} | {}, {}, {} | `{}` |",
            r.name,
            r.placement_s,
            r.erosion_s,
            r.thermal_s,
            r.render_s,
            r.stream.as_ref().map_or(0, |s| s.iterations),
            r.thermal_iterations,
            r.max_height_m,
            r.before.land,
            r.after.land,
            r.before.sea,
            r.after.sea,
            if r.stream.is_some() {
                st.pits_after_routing.to_string()
            } else {
                "-".into()
            },
            if r.stream.is_some() {
                st.raw_pits_final.to_string()
            } else {
                "-".into()
            },
            st.mean_incision_m,
            st.p90_incision_m,
            st.p99_incision_m,
            st.max_incision_m,
            st.max_area_km2,
            a.0,
            a.1,
            a.2,
            &r.heights_blake3[..16]
        );
    }
    let _ = writeln!(
        s,
        "\nPlacement time is paid once per `regional` flag (shared by the arms after the first). \"Pits after routing\": dry interior samples without a lower receiver on the Priority-Flood+ε surface (0 = everything drains). \"Raw closed pits\": dry samples lower than all 8 dry neighbours in the final heights (depressions are routed, not filled). Incision = intent - final height over dry samples, measured after the stream stage (before thermal)."
    );
    s
}
