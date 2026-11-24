//! Settlements S2 (`worldgen-derive`): a guide folder derived from another one. The guide is copied; named
//! anchors are dropped from `anchors.json` and their stamps' weights set to 0 (every stamp keeps its index, since
//! other anchors and `layers.json` key stamps by index); stamps from other folders (format v1,
//! `sf_worldgen::stamps`) are appended after the existing ones, refused where they overlap one at weight > 0.
//! `derive.json` (format 1) records the source and what changed:
//!
//! ```json
//! { "format": 1, "source_guide": "<path>", "source_blake3": { "stamps.f32": "<hex>" | null, "anchors.json": … },
//!   "dropped": [{ "name": "Balmora", "stamp": 2, "world_centre_m": [x, y], "lift_m": 12.5 },
//!               { "name": "Balmora 3", "stamp": null, … }],
//!   "appended": [{ "name": "…", "stamp": 412, "from": "<dir>" }] }
//! ```
//!
//! A `--drop-anchor X` value removes the anchor named `X` and those named `X <number>` (the census names later
//! duplicates " 2", " 3", ...; `x10::name_of`), never `Xyz`.
//!
//! SP-1b (`--valley <site dir>`, spec `site-prep.md`): after the drops and appends, the guide's heights as the game
//! shows them (the derived stamps, dropped ones at weight 0; [`crate::site::guide_tiles`]) feed
//! `sf_worldgen::valley_terrace` around the site's centre at the guide's sea level (refused when it differs from the
//! site's `water_level_m` by over [`SEA_MATCH_M`]). The stamp's weights are then clipped to 0 wherever a stamp
//! already in the set samples a weight > 0 (one sample wider, so bilinear lookups never meet either), rising back
//! over the operator's `blend_m` (smootherstep), so no anchor is reshaped and the no-overlap rule holds; the stamp is
//! cropped to its samples of weight > 0 and appended, `valley.json` (the operator's report, before the clip) is
//! written next to it and `derive.json` gains `"valley": { "name", "stamp", "site", "clipped", ... }` (the clip's
//! counts and the cut, fill and grade shares after it).

// Grid maths: sample counts stay far below 2^31 and coordinates within a few hundred km (as in sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names
)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sf_worldgen::anchors::{ANCHORS_FILE, AnchorTransform};
use sf_worldgen::filter::Declared;
use sf_worldgen::stamps::{Stamp, StampSet};
use sf_worldgen::valley_terrace::{self, ValleyInput, ValleyParams, ValleyTerrace};

use crate::DumpError;
use crate::site::{self, SiteJson};

/// The operator's report file, written next to the derived stamps.
pub const VALLEY_FILE: &str = "valley.json";

/// Largest difference between the guide's sea level and the site's water level `--valley` accepts, metres.
// tunables: const (maths: f64 round-trip tolerance of one shared level, not a quality trade)
pub const SEA_MATCH_M: f64 = 0.01;

/// Street grade of the valley report's "share under the grade" (m/m): Magna Urbis' street limit (spec `site-prep.md`).
// tunables: const (report: the spec's measurement, so the numbers compare)
pub const REPORT_GRADE: f64 = 0.12;

/// Distance from the water within which that share counts, metres (spec `site-prep.md` measurement).
// tunables: const (report: the spec's measurement, so the numbers compare)
pub const REPORT_WITHIN_M: f64 = 300.0;

/// The declared derive values (spec `tunables.md` "Definition").
pub fn declared() -> Vec<Declared> {
    vec![Declared {
        name: "derive.valley_core_m",
        v: 300.0,
        unit: "m",
        min: 0.0,
        max: 5000.0,
        source: "Guess".into(),
        help: "radius of the town core of a --valley stamp around the site centre: risers inside are retaining walls, \
               outside earth banks",
    }, Declared {
        name: "derive.valley_reach_m",
        v: 600.0,
        unit: "m",
        min: 50.0,
        max: 5000.0,
        source: "Measured 2026-11-24 (SP-1b): Magna Urbis's town extent for 3000 people is ~271 m                  (settlements/mod.rs:655); 600 m covers it plus the founding crossing's search (300 m) and fits the                  stamp budget (half the 3 km site edge, 1500 m, gave 13.6 M samples over anchor.stamp_samples_max)"
            .into(),
        help: "radius of a --valley stamp's extent around the site centre",
    }]
}

/// [`declared`]'s value `name`, range-checked.
fn declared_value(name: &str) -> Result<f64, DumpError> {
    let d = declared();
    let p = d.iter().find(|p| p.name == name).ok_or_else(|| DumpError::Invalid(format!("{name}: not declared")))?;
    if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
        Ok(p.v)
    } else {
        Err(DumpError::Invalid(format!("{} = {} outside [{}, {}]", p.name, p.v, p.min, p.max)))
    }
}

/// [`declared`]'s `derive.valley_core_m`, range-checked.
pub fn valley_core_m() -> Result<f64, DumpError> {
    declared_value("derive.valley_core_m")
}

/// [`declared`]'s `derive.valley_reach_m`, range-checked.
pub fn valley_reach_m() -> Result<f64, DumpError> {
    declared_value("derive.valley_reach_m")
}

/// `--valley`: a valley-terrace stamp for a site.
#[derive(Clone, Debug)]
pub struct ValleyArgs {
    /// The site folder (`site.json`: `name`, `centre_m`, `water_level_m`, `dims`, `cell_m`).
    pub site: PathBuf,
    /// Extent radius; default [`valley_reach_m`].
    pub reach_m: Option<f64>,
    /// Core radius; default [`valley_core_m`].
    pub core_m: Option<f64>,
}

/// `valley` of [`DeriveFile`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValleyRecord {
    pub name: String,
    /// Its index in the derived `stamps.json`.
    pub stamp: usize,
    /// The site folder.
    pub site: String,
    pub reach_m: f64,
    pub core_m: f64,
    /// Samples of weight > 0 set to 0 on (and one sample around) existing stamps of weight > 0.
    pub clipped: usize,
    /// Samples of weight > 0 lowered by the rise back over `blend_m`.
    pub tapered: usize,
    /// The appended stamp's dims after cropping to its samples of weight > 0.
    pub dims: [usize; 2],
    /// After the clip, on the operator's grid (the report's `cut_m3`, `fill_m3`, `grade_share_*` are before it).
    pub cut_m3: f64,
    pub fill_m3: f64,
    pub grade_share_before: f64,
    pub grade_share_after: f64,
}

/// The record file of a derived guide.
pub const DERIVE_FILE: &str = "derive.json";

/// Version of [`DERIVE_FILE`].
pub const DERIVE_FORMAT: u32 = 1;

/// What to derive.
#[derive(Clone, Debug)]
pub struct DeriveArgs {
    pub guide: PathBuf,
    pub out: PathBuf,
    /// Anchor names to drop (each also drops `<name> <number>`).
    pub drop_anchors: Vec<String>,
    /// Folders holding a `stamps.json` + `stamps.f32` to append, in order.
    pub add_stamps: Vec<PathBuf>,
    /// Upper bound of the total stamp samples (`anchor.stamp_samples_max`).
    pub stamp_samples_max: usize,
    /// A valley-terrace stamp to append last (module docs).
    pub valley: Option<ValleyArgs>,
    /// The `worldgen.sharp` scale the game shows the guide at (`--valley` samples the guide as shown).
    pub sharp_scale: f32,
}

/// `source_blake3` of [`DeriveFile`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHashes {
    #[serde(rename = "stamps.f32")]
    pub stamps_f32: Option<String>,
    #[serde(rename = "anchors.json")]
    pub anchors_json: Option<String>,
}

/// One dropped anchor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dropped {
    pub name: String,
    /// Its stamp (weights now 0), `None` for a point anchor.
    pub stamp: Option<usize>,
    /// Its `anchors.json` `world_centre_m` (ST-2b: `worldgen-site --guide` searches the river from it); `None` in
    /// files written before it was recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_centre_m: Option<[f64; 2]>,
    /// Its `anchors.json` `lift_m`; `None` as `world_centre_m`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lift_m: Option<f64>,
}

/// One appended stamp.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Appended {
    pub name: String,
    /// Its index in the derived `stamps.json`.
    pub stamp: usize,
    /// The folder it came from.
    pub from: String,
}

/// `derive.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeriveFile {
    pub format: u32,
    pub source_guide: String,
    pub source_blake3: SourceHashes,
    pub dropped: Vec<Dropped>,
    /// Every appended stamp, the `--valley` one last (`from` = the site folder).
    pub appended: Vec<Appended>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valley: Option<ValleyRecord>,
}

/// Whether anchor `name` is dropped by the value `drop`: `drop` itself or `drop` + " " + a number.
pub fn drops(drop: &str, name: &str) -> bool {
    name == drop
        || name
            .strip_prefix(drop)
            .and_then(|rest| rest.strip_prefix(' '))
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

fn inv(what: &str) -> impl Fn(sf_worldgen::Error) -> DumpError + '_ {
    move |e| DumpError::Invalid(format!("{what}: {e}"))
}

fn hash(path: &Path) -> Result<Option<String>, DumpError> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(blake3::hash(&std::fs::read(path)?).to_hex().to_string()))
}

/// Copies `from`'s files into `to` (created), recursively; returns (files, bytes).
fn copy_dir(from: &Path, to: &Path) -> Result<(usize, u64), DumpError> {
    std::fs::create_dir_all(to)?;
    let (mut files, mut bytes) = (0, 0);
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let (src, dst) = (e.path(), to.join(e.file_name()));
        if e.file_type()?.is_dir() {
            let (f, b) = copy_dir(&src, &dst)?;
            files += f;
            bytes += b;
        } else {
            bytes += std::fs::copy(&src, &dst)?;
            files += 1;
        }
    }
    Ok((files, bytes))
}

/// Checks the folders, then creates `out` (empty); refuses `out` = `guide`, `out` inside `guide` and a non-empty `out`.
fn prepare(guide: &Path, out: &Path) -> Result<(), DumpError> {
    if !guide.is_dir() {
        return Err(DumpError::Invalid(format!("{}: not a guide folder", guide.display())));
    }
    let existed = out.exists();
    if existed && std::fs::read_dir(out)?.next().is_some() {
        return Err(DumpError::Invalid(format!("{}: exists and is not empty", out.display())));
    }
    std::fs::create_dir_all(out)?;
    let (g, o) = (guide.canonicalize()?, out.canonicalize()?);
    if o.starts_with(&g) {
        if !existed {
            let _ = std::fs::remove_dir(out);
        }
        return Err(DumpError::Invalid(format!("{}: is the guide folder or inside it", out.display())));
    }
    Ok(())
}

/// Derives `args.out` from `args.guide` (module docs); returns the summary. On a refusal after `out` was created
/// (it was absent or empty), everything written to it is removed again.
pub fn run(args: &DeriveArgs) -> Result<String, DumpError> {
    prepare(&args.guide, &args.out)?;
    derive(args).inspect_err(|_| {
        let _ = std::fs::remove_dir_all(&args.out);
    })
}

fn derive(args: &DeriveArgs) -> Result<String, DumpError> {
    let (guide, out) = (&args.guide, &args.out);
    let source_blake3 =
        SourceHashes { stamps_f32: hash(&guide.join("stamps.f32"))?, anchors_json: hash(&guide.join(ANCHORS_FILE))? };
    let (files, bytes) = copy_dir(guide, out)?;
    let mut s = String::new();
    #[allow(clippy::cast_precision_loss)]
    let mb = bytes as f64 / 1e6;
    let _ = writeln!(s, "worldgen-derive: {} -> {}: {files} files copied ({mb:.1} MB)", guide.display(), out.display());

    let mut dropped = Vec::new();
    if !args.drop_anchors.is_empty() {
        let t = AnchorTransform::read(out)
            .map_err(inv(ANCHORS_FILE))?
            .ok_or_else(|| DumpError::Invalid(format!("{}: no {ANCHORS_FILE} to drop anchors from", guide.display())))?;
        for d in &args.drop_anchors {
            if !t.anchors.iter().any(|a| drops(d, &a.name)) {
                return Err(DumpError::Invalid(format!("--drop-anchor {d:?}: no anchor of that name")));
            }
        }
        let (gone, kept): (Vec<_>, Vec<_>) =
            t.anchors.into_iter().partition(|a| args.drop_anchors.iter().any(|d| drops(d, &a.name)));
        dropped = gone
            .into_iter()
            .map(|a| Dropped { name: a.name, stamp: a.stamp, world_centre_m: Some(a.world_centre_m), lift_m: Some(a.lift_m) })
            .collect();
        let _ = writeln!(s, "  anchors: {} dropped, {} kept", dropped.len(), kept.len());
        AnchorTransform::new(t.scale, kept).write(out).map_err(inv(ANCHORS_FILE))?;
        for d in &dropped {
            let st = d.stamp.map_or_else(|| "no stamp".to_owned(), |k| format!("stamp {k} weights zeroed"));
            let _ = writeln!(s, "    {:?}: {st}", d.name);
        }
    }

    let zero: Vec<usize> = dropped.iter().filter_map(|d| d.stamp).collect();
    let mut appended = Vec::new();
    let mut valley = None;
    if !zero.is_empty() || !args.add_stamps.is_empty() || args.valley.is_some() {
        let mut set = StampSet::read(out).map_err(inv("stamps"))?;
        if set.is_none() && !zero.is_empty() {
            return Err(DumpError::Invalid(format!("{}: anchors name stamps but there is no stamps.json", guide.display())));
        }
        let set = set.get_or_insert_with(StampSet::default);
        for &k in &zero {
            set.zero_weights(k).map_err(inv("stamps"))?;
        }
        for dir in &args.add_stamps {
            let add = StampSet::read(dir)
                .map_err(inv(&dir.display().to_string()))?
                .ok_or_else(|| DumpError::Invalid(format!("--add-stamp {}: no stamps.json", dir.display())))?;
            let first = set.stamps.len();
            let names: Vec<String> = add.stamps.iter().map(|st| st.name.clone()).collect();
            set.append(add).map_err(inv(&format!("--add-stamp {}", dir.display())))?;
            for (i, name) in names.into_iter().enumerate() {
                appended.push(Appended { name, stamp: first + i, from: dir.display().to_string() });
            }
        }
        if let Some(v) = &args.valley {
            // The guide as the game shows it with these stamps: write them first, then open the derived folder.
            set.write(out).map_err(inv("stamps"))?;
            let (stamp, rec) = add_valley(v, args.sharp_scale, out, set, &mut s)?;
            let index = set.stamps.len();
            set.append(StampSet { stamps: vec![stamp] }).map_err(inv("--valley"))?;
            debug_assert_eq!(index, rec.stamp);
            appended.push(Appended { name: rec.name.clone(), stamp: index, from: rec.site.clone() });
            valley = Some(rec);
        }
        let samples: usize = set.stamps.iter().map(|st| st.heights.len()).sum();
        if samples > args.stamp_samples_max {
            return Err(DumpError::Invalid(format!(
                "{samples} stamp samples exceed the budget of {} (anchor.stamp_samples_max)",
                args.stamp_samples_max
            )));
        }
        set.write(out).map_err(inv("stamps"))?;
        let _ = writeln!(
            s,
            "  stamps: {} total, {} zeroed, {} appended, {samples} samples",
            set.stamps.len(),
            zero.len(),
            appended.len()
        );
        for a in &appended {
            let _ = writeln!(s, "    + {:?} as stamp {} from {}", a.name, a.stamp, a.from);
        }
    }

    let file = DeriveFile {
        format: DERIVE_FORMAT,
        source_guide: guide.display().to_string(),
        source_blake3,
        dropped,
        appended,
        valley,
    };
    std::fs::write(out.join(DERIVE_FILE), serde_json::to_string_pretty(&file)?)?;
    let _ = writeln!(s, "  wrote {}", out.join(DERIVE_FILE).display());
    Ok(s)
}

/// Runs the operator for `v`'s site on the derived folder `out` (its stamps = `set`, already written), clips and
/// crops the stamp against `set`, writes `valley.json`; returns the stamp to append and its record.
fn add_valley(
    v: &ValleyArgs,
    sharp_scale: f32,
    out: &Path,
    set: &StampSet,
    s: &mut String,
) -> Result<(Stamp, ValleyRecord), DumpError> {
    let site_file = v.site.join(site::SITE_FILE);
    let raw = std::fs::read(&site_file).map_err(|e| DumpError::Invalid(format!("--valley {}: {e}", site_file.display())))?;
    let json: SiteJson = serde_json::from_slice(&raw)?;
    if out.join(VALLEY_FILE).exists() {
        return Err(DumpError::Invalid(format!("the guide already has a {VALLEY_FILE} (one valley per derived guide)")));
    }
    let (tiles, _) = site::guide_tiles(out, None, sharp_scale)?;
    let sea = tiles.guide().sea_level_m;
    if (sea - json.water_level_m).abs() > SEA_MATCH_M {
        return Err(DumpError::Invalid(format!(
            "--valley: the guide's sea level {sea} m differs from the site's water_level_m {} m",
            json.water_level_m
        )));
    }
    let reach_m = match v.reach_m {
        Some(r) => r,
        None => valley_reach_m()?,
    };
    let core_m = match v.core_m {
        Some(c) => c,
        None => valley_core_m()?,
    };
    let params = ValleyParams::defaults();
    let name = format!("{} valley", json.name);
    let mut seed = [0u8; 8];
    seed.copy_from_slice(&blake3::hash(name.as_bytes()).as_bytes()[..8]);
    let guide = |p: [f64; 2]| f64::from(tiles.height(p[0], -p[1], 0));
    let inp = ValleyInput {
        name: name.clone(),
        guide: &guide,
        sea_level_m: sea,
        centre_m: json.centre_m,
        reach_m,
        core_radius_m: core_m,
        seed: u64::from_le_bytes(seed),
        street_grade: REPORT_GRADE,
        share_within_m: REPORT_WITHIN_M,
    };
    let started = std::time::Instant::now();
    let t = valley_terrace::valley_terrace(&inp, &params).map_err(inv("valley_terrace"))?;
    let secs = started.elapsed().as_secs_f64();
    let r = &t.report;
    let [cx, cy] = json.centre_m;
    let _ = writeln!(
        s,
        "  valley {name:?} (site {}): centre [{cx:.1}, {cy:.1}] m, reach {reach_m:.1} m, core {core_m:.1} m, sea {sea} m; \
         grid {} x {} in {secs:.1} s",
        v.site.display(),
        t.stamp.dims[0],
        t.stamp.dims[1]
    );
    let _ = writeln!(
        s,
        "    operator: cut {:.3} M m3, fill {:.3} M m3, tread share {:.3}, dry ground under {REPORT_GRADE} within \
         {REPORT_WITHIN_M} m of the water {:.3} -> {:.3}",
        r.cut_m3 / 1e6,
        r.fill_m3 / 1e6,
        r.tread_share,
        r.grade_share_before,
        r.grade_share_after
    );

    let mut stamp = t.stamp.clone();
    let (clipped, tapered) = clip(&mut stamp, &set.stamps, params.blend_m);
    let after = metrics(&t, &stamp, &inp);
    let _ = writeln!(
        s,
        "    clip: {clipped} samples to 0 on existing stamps, {tapered} lowered over {} m; after it: cut {:.3} M m3, \
         fill {:.3} M m3, under {REPORT_GRADE}: {:.3} -> {:.3}",
        params.blend_m,
        after[0] / 1e6,
        after[1] / 1e6,
        after[2],
        after[3]
    );
    let stamp =
        crop(&stamp).ok_or_else(|| DumpError::Invalid(format!("--valley: {name:?} has no weight left after the clip")))?;
    std::fs::write(out.join(VALLEY_FILE), serde_json::to_string_pretty(&t.report)?)?;
    let _ = writeln!(
        s,
        "    stamp {}: {} x {} after cropping to weight > 0; wrote {}",
        set.stamps.len(),
        stamp.dims[0],
        stamp.dims[1],
        out.join(VALLEY_FILE).display()
    );
    let rec = ValleyRecord {
        name,
        stamp: set.stamps.len(),
        site: v.site.display().to_string(),
        reach_m,
        core_m,
        clipped,
        tapered,
        dims: stamp.dims,
        cut_m3: after[0],
        fill_m3: after[1],
        grade_share_before: after[2],
        grade_share_after: after[3],
    };
    Ok((stamp, rec))
}

/// Sets `v`'s weights to 0 wherever a stamp of `existing` samples a weight > 0, and one sample around (so bilinear
/// lookups from either side never meet a weight > 0 of the other), then multiplies the rest by
/// `smootherstep(distance / blend_m)`; returns (samples of weight > 0 set to 0, samples of weight > 0 lowered).
pub fn clip(v: &mut Stamp, existing: &[Stamp], blend_m: f64) -> (usize, usize) {
    let [w, h] = v.dims;
    let sp = v.spacing_m;
    let [vx, vy] = v.origin_m;
    let vr = v.rect_m();
    let mut hit = vec![false; w * h];
    for st in existing {
        let r = st.rect_m();
        if r[2] < vr[0] || vr[2] < r[0] || r[3] < vr[1] || vr[3] < r[1] || !st.weights.iter().any(|&x| x > 0.0) {
            continue;
        }
        // The valley samples covering the stamp's rectangle, one wider on each side.
        let lo = |a: f64| ((a / sp).floor() - 1.0).max(0.0) as usize;
        let hi = |a: f64, n: usize| (((a / sp).ceil() + 1.0).max(0.0) as usize).min(n - 1);
        let (c0, c1) = (lo(r[0] - vx), hi(r[2] - vx, w));
        let (r0, r1) = (lo(vy - r[3]), hi(vy - r[1], h));
        for j in r0..=r1 {
            for i in c0..=c1 {
                let p = [vx + i as f64 * sp, vy - j as f64 * sp];
                if st.sample(p[0], p[1]).is_some_and(|(_, wt)| wt > 0.0) {
                    hit[j * w + i] = true;
                }
            }
        }
    }
    // One sample wider (8-neighbourhood).
    let mut zero = hit.clone();
    for (k, _) in hit.iter().enumerate().filter(|(_, x)| **x) {
        let (i, j) = (k % w, k / w);
        for jj in j.saturating_sub(1)..=(j + 1).min(h - 1) {
            for ii in i.saturating_sub(1)..=(i + 1).min(w - 1) {
                zero[jj * w + ii] = true;
            }
        }
    }
    let dist = sf_worldgen::measure::distance_transform(&zero, w, h);
    let (mut clipped, mut tapered) = (0, 0);
    for (k, wt) in v.weights.iter_mut().enumerate() {
        if *wt <= 0.0 {
            continue;
        }
        if zero[k] {
            *wt = 0.0;
            clipped += 1;
            continue;
        }
        let f = sf_worldgen::volcano::smootherstep(dist[k] * sp / blend_m);
        if f < 1.0 {
            *wt = (f64::from(*wt) * f) as f32;
            tapered += 1;
        }
    }
    (clipped, tapered)
}

/// `st` cropped to the rectangle of its samples of weight > 0 (lossless: outside a stamp the guide stands, as at
/// weight 0); `None` without any.
pub fn crop(st: &Stamp) -> Option<Stamp> {
    let w = st.dims[0];
    let (mut c0, mut c1, mut r0, mut r1) = (usize::MAX, 0, usize::MAX, 0);
    for (k, _) in st.weights.iter().enumerate().filter(|(_, x)| **x > 0.0) {
        let (i, j) = (k % w, k / w);
        (c0, c1, r0, r1) = (c0.min(i), c1.max(i), r0.min(j), r1.max(j));
    }
    if c0 == usize::MAX {
        return None;
    }
    let (cw, ch) = (c1 - c0 + 1, r1 - r0 + 1);
    let pick = |v: &[f32]| -> Vec<f32> { (r0..=r1).flat_map(|j| v[j * w + c0..j * w + c0 + cw].iter().copied()).collect() };
    Some(Stamp {
        name: st.name.clone(),
        origin_m: [st.origin_m[0] + c0 as f64 * st.spacing_m, st.origin_m[1] - r0 as f64 * st.spacing_m],
        dims: [cw, ch],
        spacing_m: st.spacing_m,
        heights: pick(&st.heights),
        weights: pick(&st.weights),
    })
}

/// The operator's report numbers for `stamp` (same grid as `t.stamp`): [cut m3, fill m3, grade share before, after],
/// counted as `valley_terrace` counts them (dry samples; shares inside the reach and within the report distance of
/// the water).
fn metrics(t: &ValleyTerrace, stamp: &Stamp, inp: &ValleyInput) -> [f64; 4] {
    let [w, h] = stamp.dims;
    let sp = stamp.spacing_m;
    let d = &t.diag;
    let fin = valley_terrace::final_surface(stamp, &d.guide_m);
    let g: Vec<f64> = d.guide_m.iter().map(|&v| f64::from(v)).collect();
    let slope = |z: &[f64], k: usize| -> f64 {
        let (i, j) = (k % w, k / w);
        if i == 0 || j == 0 || i + 1 >= w || j + 1 >= h {
            return f64::NAN;
        }
        let gx = (z[k + 1] - z[k - 1]) / (2.0 * sp);
        let gy = (z[k + w] - z[k - w]) / (2.0 * sp);
        gx.hypot(gy)
    };
    let (mut cut, mut fill, mut cnt, mut ok0, mut ok1) = (0.0, 0.0, 0u64, 0u64, 0u64);
    for k in 0..w * h {
        if d.bank[k] == u32::MAX {
            continue;
        }
        let dz = fin[k] - g[k];
        cut += (-dz).max(0.0) * sp * sp;
        fill += dz.max(0.0) * sp * sp;
        let pos = [stamp.origin_m[0] + (k % w) as f64 * sp, stamp.origin_m[1] - (k / w) as f64 * sp];
        let near = (pos[0] - inp.centre_m[0]).hypot(pos[1] - inp.centre_m[1]) <= inp.reach_m;
        let s0 = slope(&g, k);
        if near && f64::from(d.d_m[k]) <= inp.share_within_m && s0.is_finite() {
            cnt += 1;
            ok0 += u64::from(s0 <= inp.street_grade);
            ok1 += u64::from(slope(&fin, k) <= inp.street_grade);
        }
    }
    let share = |a: u64, b: u64| if b == 0 { 0.0 } else { a as f64 / b as f64 };
    [cut, fill, share(ok0, cnt), share(ok1, cnt)]
}
