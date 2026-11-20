//! `worldgen-x10`: the same map at 10× as a complete guide folder (spec `worldgen-adapter.md` "Same map at 10×
//! (WG-X)", steps 1, 3, 4; WG-X2).
//!
//! Writes `heights.f32` + `meta.json` (guide v1; the dump's heights × S, spacing × S, origin × S, or a reshaped
//! guide of the same grid given by `--heights`), `regions.png`/`regions.json`, `markers.json` (source metres, as
//! the dump writes it), `stamps.json`/`stamps.f32` (`sf_worldgen::stamps`), `anchors.json`
//! (`sf_worldgen::anchors`) and `anchors-census.md`.

// Grid maths: sample counts stay far below 2^31 and heights within f32 range (as in sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names
)]

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::fill_height;
use mw_world::{MwWorldSource, SourceOptions};
use serde::Serialize;
use sf_records::RecordDb;
use sf_worldgen::anchors::AnchorTransform;
use sf_worldgen::stamps::StampSet;

use crate::anchor::{self, Built, Field, Member, Params};
use crate::classify::{self, Class, RefInfo, Rule};
use crate::{DumpError, MarkersFile, Mosaic, dump};

/// Model bounds lookup: model VFS path → model-space `(min, max)` at unit scale (engine axes, metres).
pub type Bounds<'a> = &'a dyn Fn(&str) -> Option<([f32; 3], [f32; 3])>;

/// The command's arguments.
#[derive(Clone, Debug)]
pub struct X10Args {
    pub out: PathBuf,
    /// A guide of the same grid (dims, spacing, origin) whose heights replace the plain scale.
    pub heights: Option<PathBuf>,
    pub scale: f64,
    pub params: Params,
}

/// `meta.json` of the scaled guide (guide v1 keys + provenance).
#[derive(Clone, Debug, Serialize)]
pub struct X10Meta {
    pub format: u32,
    pub dims: [usize; 2],
    pub sample_spacing_m: f64,
    pub sea_level_m: f64,
    pub fill_m: f64,
    /// `[x east, y north]` of sample (0, 0), metres.
    pub origin_m: [f64; 2],
    pub cell_size_m: f64,
    pub producer: String,
    pub scale: f64,
    pub source_heights_blake3: String,
    pub heights_blake3: String,
    /// The guide whose heights were taken (`--heights`), if any.
    pub heights_from: Option<String>,
    pub anchors: usize,
}

/// The source mosaic's heights × S (f32, rounded once; equals an f32 multiply since the f64 product is exact).
pub fn scale_heights(heights: &[f32], scale: f64) -> Vec<f32> {
    heights.iter().map(|&h| (f64::from(h) * scale) as f32).collect()
}

fn le_bytes(h: &[f32]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(h.len() * 4);
    for v in h {
        raw.extend_from_slice(&v.to_le_bytes());
    }
    raw
}

/// The source frame origin (metres, x east, y north) of mosaic sample (0, 0).
pub fn source_origin_m(m: &Mosaic) -> [f64; 2] {
    let (ox, oy) = m.mw_units(0, 0);
    [ox * UNIT_METERS_F64, oy * UNIT_METERS_F64]
}

/// Every exterior reference of `source` as a [`RefInfo`].
pub fn exterior_refs(source: &MwWorldSource, bounds: Bounds<'_>) -> Vec<RefInfo> {
    source
        .all_placements(&mw_world::ids::exterior())
        .map(|p| {
            let e = p.transform.pos.to_f64();
            let model = p.model.as_ref().map(|m| m.as_str().to_ascii_lowercase().replace('\\', "/"));
            RefInfo {
                key: p.id.to_string(),
                kind: p.base.kind.to_string(),
                base: p.base.id.as_str().to_owned(),
                bounds: p.model.as_ref().and_then(|m| bounds(m.as_str())),
                model,
                pos: [e.x, -e.z, e.y],
                scale: p.transform.scale.x,
            }
        })
        .collect()
}

/// Census of the classification.
#[derive(Clone, Debug, Default)]
pub struct ClassCensus {
    pub by_class: BTreeMap<Class, usize>,
    pub by_rule: BTreeMap<(Class, Rule), usize>,
    /// Model folder groups of the "other" and the human-made classes (`rule: folder`).
    pub other_folders: BTreeMap<String, usize>,
    pub structure_folders: BTreeMap<String, usize>,
}

/// Classifies `refs` and returns the human-made ones as anchor members, plus the census.
pub fn members_of(refs: &[RefInfo], src: &Field<'_>, sea: &[bool], p: &Params) -> (Vec<Member>, ClassCensus) {
    let mut census = ClassCensus::default();
    let mut members = Vec::new();
    for r in refs {
        let (class, rule) = classify::classify(r, p.rock_min_m);
        *census.by_class.entry(class).or_default() += 1;
        *census.by_rule.entry((class, rule)).or_default() += 1;
        match (class, rule) {
            (Class::Other, _) => *census.other_folders.entry(format!("{rule}: {}", r.folder())).or_default() += 1,
            (Class::HumanMade, _) => *census.structure_folders.entry(format!("{rule}: {}", r.folder())).or_default() += 1,
            _ => {}
        }
        if class == Class::HumanMade {
            let k = src.nearest(r.pos[0], r.pos[1]);
            let on_sea = sea[k];
            members.push(Member {
                key: r.key.clone(),
                xy: [r.pos[0], r.pos[1]],
                r: r.bounds_radius_m().unwrap_or(p.default_radius_m).max(0.0),
                on_sea,
                group: format!("{rule}: {}", r.stem().unwrap_or(&r.kind)),
            });
        }
    }
    (members, census)
}

/// Names anchors after the nearest marker (by centroid); later duplicates get " 2", " 3", ...
fn name_of(markers: &MarkersFile, c: [f64; 2], used: &mut BTreeMap<String, usize>) -> String {
    let base = markers
        .markers
        .iter()
        .min_by(|a, b| {
            let da = (a.centroid_m[0] - c[0]).hypot(a.centroid_m[1] - c[1]);
            let db = (b.centroid_m[0] - c[0]).hypot(b.centroid_m[1] - c[1]);
            da.total_cmp(&db)
        })
        .map_or_else(|| "anchor".to_owned(), |m| m.name.clone());
    let n = used.entry(base.clone()).or_default();
    *n += 1;
    if *n == 1 { base } else { format!("{base} {n}") }
}

/// Member groups as (label, count), largest first.
type Groups = Vec<(String, usize)>;

/// The member groups of anchor `b` (top `n`), and those of its members on the open sea.
fn member_groups(b: &Built, members: &BTreeMap<&str, &Member>, n: usize) -> (Groups, Groups) {
    let mut all: BTreeMap<String, usize> = BTreeMap::new();
    let mut sea: BTreeMap<String, usize> = BTreeMap::new();
    for k in &b.place.members {
        if let Some(m) = members.get(k.as_str()) {
            *all.entry(m.group.clone()).or_default() += 1;
            if m.on_sea {
                *sea.entry(m.group.clone()).or_default() += 1;
            }
        }
    }
    let own = |m: BTreeMap<String, usize>| top(&m, n).into_iter().map(|(k, c)| (k.clone(), c)).collect();
    (own(all), own(sea))
}

/// Clusters `members` and builds every anchor, the largest member count first.
pub fn build_anchors(
    members: &[Member],
    src: &Field<'_>,
    fin: &Field<'_>,
    scale: f64,
    p: &Params,
    markers: &MarkersFile,
) -> Vec<Built> {
    let mut clusters = anchor::cluster(members, p, scale, src.spacing_m);
    // Largest first; ties by the smallest key (clusters come ordered by their smallest key, sort is stable).
    clusters.sort_by_key(|g| std::cmp::Reverse(g.len()));
    let mut used = BTreeMap::new();
    let mut stamps = 0;
    clusters
        .iter()
        .map(|g| {
            let ms: Vec<&Member> = g.iter().map(|&k| &members[k]).collect();
            let name = name_of(markers, anchor::centre(&ms, p).0, &mut used);
            let mut b = anchor::build_anchor(name, &ms, src, fin, scale, p);
            if b.stamp.is_some() {
                b.place.stamp = Some(stamps);
                stamps += 1;
            }
            b
        })
        .collect()
}

/// The stamps of `built`, in stamp-index order.
pub fn stamp_set(built: &[Built]) -> StampSet {
    StampSet { stamps: built.iter().filter_map(|b| b.stamp.clone()).collect() }
}

/// Weight > 0 samples of one stamp that another stamp also covers with weight > 0 (should be 0).
pub fn overlap_samples(set: &StampSet) -> usize {
    let rects: Vec<[f64; 4]> = set.stamps.iter().map(sf_worldgen::stamps::Stamp::rect_m).collect();
    let mut n = 0;
    for (a, sa) in set.stamps.iter().enumerate() {
        for (b, sb) in set.stamps.iter().enumerate() {
            let (ra, rb) = (rects[a], rects[b]);
            if a == b || ra[2] < rb[0] || rb[2] < ra[0] || ra[3] < rb[1] || rb[3] < ra[1] {
                continue;
            }
            let [w, h] = sa.dims;
            for k in 0..w * h {
                if sa.weights[k] > 0.0 {
                    let x = sa.origin_m[0] + (k % w) as f64 * sa.spacing_m;
                    let y = sa.origin_m[1] - (k / w) as f64 * sa.spacing_m;
                    n += usize::from(sb.sample(x, y).is_some_and(|(_, wb)| wb > 0.0));
                }
            }
        }
    }
    n
}

fn histogram(built: &[Built]) -> Vec<(&'static str, usize)> {
    let bins: [(&str, usize, usize); 7] =
        [("1", 1, 1), ("2-4", 2, 4), ("5-9", 5, 9), ("10-49", 10, 49), ("50-199", 50, 199), ("200-999", 200, 999), ("1000+", 1000, usize::MAX)];
    bins.iter().map(|&(l, lo, hi)| (l, built.iter().filter(|b| (lo..=hi).contains(&b.stats.members)).count())).collect()
}

fn top(m: &BTreeMap<String, usize>, n: usize) -> Vec<(&String, usize)> {
    let mut v: Vec<(&String, usize)> = m.iter().map(|(k, &c)| (k, c)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    v.truncate(n);
    v
}

/// The census header and the reference classes.
fn references_md(s: &mut String, args: &X10Args, refs: usize, census: &ClassCensus) {
    let p = &args.params;
    let _ = writeln!(s, "# worldgen-x10 anchors census\n");
    let _ = writeln!(
        s,
        "scale {}, heights from {}; link gap {} m, margin {} m, band {} x R in [{}, {}] m, point anchors <= {} \
         members or R < {} m, default radius {} m, max radius {} m, rock min {} m\n",
        args.scale,
        args.heights.as_ref().map_or_else(|| "the plain scale".to_owned(), |h| h.display().to_string()),
        p.link_gap_m,
        p.margin_m,
        p.band_k,
        p.band_min_m,
        p.band_max_m,
        p.point_max_members,
        p.point_max_radius_m,
        p.default_radius_m,
        p.max_radius_m,
        p.rock_min_m
    );
    let _ = writeln!(s, "## References ({refs} exterior)\n\n| class | rule | count |\n|---|---|---|");
    for ((class, rule), n) in &census.by_rule {
        let _ = writeln!(s, "| {class} | {rule} | {n} |");
    }
    for (class, n) in &census.by_class {
        let _ = writeln!(s, "| **{class}** | (all) | **{n}** |");
    }
    let _ = writeln!(s, "\n### Ambiguous: class \"other\", top model folders\n\n| rule: folder | count |\n|---|---|");
    for (k, n) in top(&census.other_folders, 25) {
        let _ = writeln!(s, "| {k} | {n} |");
    }
    let _ = writeln!(s, "\n### Human-made, top model folders\n\n| rule: folder | count |\n|---|---|");
    for (k, n) in top(&census.structure_folders, 40) {
        let _ = writeln!(s, "| {k} | {n} |");
    }
}

/// The census as Markdown.
#[allow(clippy::too_many_arguments)]
fn census_md(
    args: &X10Args,
    members: &[Member],
    refs: usize,
    census: &ClassCensus,
    built: &[Built],
    overlap: usize,
    stamp_bytes: u64,
    seconds: f64,
) -> String {
    let mut s = String::new();
    let p = &args.params;
    references_md(&mut s, args, refs, census);
    let zero = built.iter().filter(|b| b.stats.lift_m.abs() < 1e-9).count();
    let on_sea = built.iter().filter(|b| b.stats.on_sea).count();
    let points = built.iter().filter(|b| b.stats.point).count();
    let samples: usize = built.iter().map(|b| b.stats.stamp_samples).sum();
    let _ = writeln!(
        s,
        "\n## Anchors: {} ({} stamped, {points} point anchors without a stamp)\n\nTotal stamp samples (sum of w x h): \
         {samples} of the {} budget{}. stamps.f32 {:.1} MB. L = 0: {zero} ({on_sea} with a member on open \
         sea). Stamp samples covered by two stamps at weight > 0: {overlap}.\n\n| members | anchors |\n|---|---|",
        built.len(),
        built.len() - points,
        p.stamp_samples_max,
        if samples > p.stamp_samples_max { " **(OVER BUDGET: stamps not written)**" } else { "" },
        stamp_bytes as f64 / 1e6
    );
    for (l, n) in histogram(built) {
        let _ = writeln!(s, "| {l} | {n} |");
    }
    let mut largest: Vec<&Built> = built.iter().collect();
    largest.sort_by(|a, b| b.stats.footprint_m2.total_cmp(&a.stats.footprint_m2).then(a.place.stamp.cmp(&b.place.stamp)));
    let _ = writeln!(
        s,
        "\n### The 15 largest (footprint)\n\n| anchor | members | footprint m² | extent m | band m | stamp samples | L m | \
         ring p95 m | ring samples | new wet | new dry |\n|---|---|---|---|---|---|---|---|---|---|---|"
    );
    for b in largest.iter().take(15) {
        let t = &b.stats;
        let _ = writeln!(
            s,
            "| {} | {} | {:.0} | {:.0} | {:.0} | {} | {:.1}{} | {:.1} | {} | {} | {} |",
            b.place.name,
            t.members,
            t.footprint_m2,
            t.extent_m,
            t.band_m,
            t.stamp_samples,
            t.lift_m,
            if t.on_sea { " (sea)" } else { "" },
            t.ring_p95_m,
            t.ring_samples,
            t.new_wet,
            t.new_dry
        );
    }
    let mut worst: Vec<&Built> = built.iter().filter(|b| b.stats.members >= 5 && !b.stats.point).collect();
    worst.sort_by(|a, b| b.stats.ring_p95_m.total_cmp(&a.stats.ring_p95_m));
    let _ = writeln!(s, "\n### Worst ring mismatch (anchors of 5+ members)\n\n| anchor | members | L m | ring p95 m |\n|---|---|---|---|");
    for b in worst.iter().take(10) {
        let _ = writeln!(s, "| {} | {} | {:.1} | {:.1} |", b.place.name, b.stats.members, b.stats.lift_m, b.stats.ring_p95_m);
    }
    let by_key: BTreeMap<&str, &Member> = members.iter().map(|m| (m.key.as_str(), m)).collect();
    let mut biggest: Vec<&Built> = built.iter().collect();
    biggest.sort_by_key(|b| std::cmp::Reverse(b.stats.members));
    let _ = writeln!(s, "
### What the 5 largest anchors are made of (top member groups; on the open sea)
");
    for b in biggest.iter().take(5) {
        let (all, sea) = member_groups(b, &by_key, 10);
        let list = |v: &[(String, usize)]| v.iter().map(|(k, c)| format!("{k} ({c})")).collect::<Vec<_>>().join(", ");
        let _ = writeln!(s, "- **{}** ({} members): {}; on sea: {}", b.place.name, b.stats.members, list(&all), list(&sea));
    }
    let mut lifted: Vec<&Built> = built.iter().collect();
    lifted.sort_by(|a, b| b.stats.lift_m.total_cmp(&a.stats.lift_m).then(a.place.name.cmp(&b.place.name)));
    let _ = writeln!(
        s,
        "\n### Largest lifts (Red Mountain and the high ground)\n\n| anchor | members | stamp | L m | ring p95 m | new wet | \
         new dry |\n|---|---|---|---|---|---|---|"
    );
    for b in lifted.iter().take(15) {
        let t = &b.stats;
        let _ = writeln!(
            s,
            "| {} | {} | {} | {:.1} | {:.1} | {} | {} |",
            b.place.name,
            t.members,
            if t.point { "point" } else { "stamp" },
            t.lift_m,
            t.ring_p95_m,
            t.new_wet,
            t.new_dry
        );
    }
    let dry: usize = built.iter().map(|b| b.stats.new_dry).sum();
    let wet: usize = built.iter().map(|b| b.stats.new_wet).sum();
    let _ = writeln!(s, "\nFootprint wet/dry vs. source (all anchors): {wet} new wet, {dry} new dry samples.\n");
    let _ = writeln!(s, "Run time {seconds:.1} s (after content load).");
    s
}

/// Reads a `--heights` guide and checks it is the scaled grid.
fn read_heights(dir: &Path, dims: [usize; 2], spacing: f64, origin: [f64; 2]) -> Result<Vec<f32>, DumpError> {
    let g = sf_worldgen::guide::Guide::read(dir).map_err(|e| DumpError::Invalid(format!("{}: {e}", dir.display())))?;
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-6 * b.abs().max(1.0);
    if [g.w, g.h] != dims || !close(g.spacing_m, spacing) || !close(g.origin_m[0], origin[0]) || !close(g.origin_m[1], origin[1])
    {
        return Err(DumpError::Invalid(format!(
            "{}: grid {}x{} @ {} m origin {:?} differs from the scaled grid {}x{} @ {spacing} m origin {origin:?}",
            dir.display(),
            g.w,
            g.h,
            g.spacing_m,
            g.origin_m,
            dims[0],
            dims[1]
        )));
    }
    Ok(g.heights)
}

/// Rasterises Morrowind's dropped `terrain_rock*` cliff-fake references into a `[0, 1]` steering mask on the
/// scaled guide grid (`cliff_mask.f32`), so `scrib-worldgen cliffs --mask` can carve sharp outcrops and rock
/// faces wherever original Morrowind authored rocky relief without scaling low-poly cliff meshes 10x.
fn write_cliff_mask(
    refs: &[RefInfo],
    dims: [usize; 2],
    spacing_m: f64,
    origin_m: [f64; 2],
    scale: f64,
    out: &Path,
) -> Result<usize, DumpError> {
    let [w, h] = dims;
    let mut mask = vec![0.0f32; w * h];
    let mut count = 0usize;
    for r in refs {
        if !r.stem().is_some_and(|st| st.starts_with("terrain_rock")) {
            continue;
        }
        count += 1;
        let wx = r.pos[0] * scale;
        let wy = r.pos[1] * scale;
        let rad = (r.bounds_radius_m().unwrap_or(10.0) * scale).clamp(spacing_m * 1.5, 450.0);
        let c0 = ((wx - rad - origin_m[0]) / spacing_m).floor().max(0.0) as usize;
        let c1 = ((wx + rad - origin_m[0]) / spacing_m).ceil().min((w - 1) as f64) as usize;
        let r0 = ((origin_m[1] - (wy + rad)) / spacing_m).floor().max(0.0) as usize;
        let r1 = ((origin_m[1] - (wy - rad)) / spacing_m).ceil().min((h - 1) as f64) as usize;
        for row in r0..=r1 {
            let py = origin_m[1] - row as f64 * spacing_m;
            for col in c0..=c1 {
                let px = origin_m[0] + col as f64 * spacing_m;
                let d = (px - wx).hypot(py - wy) / rad;
                if d < 1.0 {
                    let t = 1.0 - d;
                    let v = (t * t * (3.0 - 2.0 * t)) as f32;
                    let slot = &mut mask[row * w + col];
                    if v > *slot {
                        *slot = v;
                    }
                }
            }
        }
    }
    std::fs::write(out.join("cliff_mask.f32"), le_bytes(&mask))?;
    Ok(count)
}

/// The `worldgen-x10` command body; returns the printed report (the census).
pub fn run(db: &RecordDb, bounds: Bounds<'_>, args: &X10Args) -> Result<String, DumpError> {
    let started = Instant::now();
    let s = args.scale;
    let d = dump(db, 100.0)?;
    let m = &d.mosaic;
    let src_origin = source_origin_m(m);
    let sp = f64::from(mw_world::terrain::VERTEX_UNITS) * UNIT_METERS_F64;
    let dims = [m.width, m.height];
    let origin = [src_origin[0] * s, src_origin[1] * s];
    let final_heights = match &args.heights {
        Some(dir) => read_heights(dir, dims, sp * s, origin)?,
        None => scale_heights(&m.heights, s),
    };
    std::fs::create_dir_all(&args.out)?;
    let raw = le_bytes(&final_heights);
    std::fs::write(args.out.join("heights.f32"), &raw)?;
    let heights_blake3 = blake3::hash(&raw).to_hex().to_string();
    drop(raw);
    crate::output::write_regions(&d.regions, &args.out)?;
    std::fs::write(args.out.join("markers.json"), serde_json::to_vec_pretty(&d.markers)?)?;

    let src = Field { w: m.width, h: m.height, origin_m: src_origin, spacing_m: sp, data: &m.heights };
    let fin = Field { w: m.width, h: m.height, origin_m: origin, spacing_m: sp * s, data: &final_heights };
    let sea = anchor::open_sea(&src, args.params.open_water_radius_m);
    let source = MwWorldSource::build(db, &SourceOptions::default());
    let refs = exterior_refs(&source, bounds);
    let _rock_mask_count = write_cliff_mask(&refs, dims, sp * s, origin, s, &args.out)?;
    let (members, census) = members_of(&refs, &src, &sea, &args.params);
    let built = build_anchors(&members, &src, &fin, s, &args.params, &d.markers);

    let samples: usize = built.iter().map(|b| b.stats.stamp_samples).sum();
    let over_budget = samples > args.params.stamp_samples_max;
    let set = stamp_set(&built);
    let overlap = overlap_samples(&set);
    if over_budget {
        // No stamps.json: a guide over the GPU stamp budget must not load.
        let _ = std::fs::remove_file(args.out.join("stamps.json"));
        let _ = std::fs::remove_file(args.out.join("stamps.f32"));
    } else {
        set.write(&args.out).map_err(|e| DumpError::Invalid(format!("stamps: {e}")))?;
    }
    drop(set);
    let transform = AnchorTransform::new(s, built.iter().map(|b| b.place.clone()).collect());
    transform.write(&args.out).map_err(|e| DumpError::Invalid(format!("anchors: {e}")))?;

    let meta = X10Meta {
        format: 1,
        dims,
        sample_spacing_m: sp * s,
        sea_level_m: anchor::SEA_LEVEL_M,
        fill_m: f64::from((f64::from(fill_height()) * s) as f32),
        origin_m: origin,
        cell_size_m: crate::markers::cell_size_m() * s,
        producer: "bevyrrowind worldgen-x10 (mw_worldgen::x10, WG-X2)".into(),
        scale: s,
        source_heights_blake3: blake3::hash(&le_bytes(&m.heights)).to_hex().to_string(),
        heights_blake3,
        heights_from: args.heights.as_ref().map(|h| h.display().to_string()),
        anchors: built.len(),
    };
    std::fs::write(args.out.join("meta.json"), serde_json::to_vec_pretty(&meta)?)?;

    let stamp_bytes = std::fs::metadata(args.out.join("stamps.f32")).map_or(0, |f| f.len());
    let md = census_md(args, &members, refs.len(), &census, &built, overlap, stamp_bytes, started.elapsed().as_secs_f64());
    std::fs::write(args.out.join("anchors-census.md"), &md)?;
    if over_budget {
        print!("{md}");
        return Err(DumpError::Invalid(format!(
            "worldgen-x10: {samples} stamp samples exceed the budget of {} (anchor.stamp_samples_max); stamps not written",
            args.params.stamp_samples_max
        )));
    }
    let mut report = md;
    let _ = writeln!(report, "wrote {}", args.out.display());
    Ok(report)
}
