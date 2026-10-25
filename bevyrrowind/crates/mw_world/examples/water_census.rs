//! Water census on the real install (ROADMAP M6 W0, water-adapter.md "Census"): exterior water heights,
//! interiors with water (visible vs below all geometry), boats (moored vs beached), authored water
//! meshes and their textures, connected sea-level water components, regions with water.
//! Prints counts only (no vanilla data is written anywhere).
//!
//! `cargo run --release -p mw_world --example water_census [-- <Morrowind folder>]`
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::items_after_statements,
    clippy::type_complexity
)]

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use glam::{Quat, Vec3};
use mw_install::MorrowindInstall;
use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::{LAND_VERTS, LandData};
use rayon::prelude::*;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Cell, CellFlags, Landscape};

/// Boat statics (water-adapter.md "Boats").
const BOATS: &[&str] = &[
    "x/ex_de_rowboat.nif",
    "x/ex_longboat.nif",
    "x/ex_longboat01.nif",
    "x/ex_longboat02.nif",
    "x/ex_longboatwrk.nif",
    "x/ex_gondola_01.nif",
    "x/ex_de_ship.nif",
    "x/ex_de_shipwreck.nif",
];

/// Meters per LAND vertex step (128 units).
const SAMPLE_M: f64 = 128.0 * UNIT_METERS_F64;

type Bounds = HashMap<String, (Vec3, Vec3, Vec<String>)>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let t = Instant::now();
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let db = &content.db;
    println!(
        "loaded {} files in {:.2?}",
        content.files.len(),
        t.elapsed()
    );

    // Every placed model's bounds and textures, once.
    let mut models: Vec<String> = Vec::new();
    for (_, cell) in db.iter::<Cell>() {
        let (placements, _) = mw_world::cell_placements(db, cell);
        models.extend(placements.into_iter().filter_map(|p| p.model));
    }
    models.sort();
    models.dedup();
    let bounds: Bounds = models
        .par_iter()
        .filter_map(|m| {
            let bytes = vfs.read(&VfsPath::new(m).ok()?).ok()?;
            let model = mw_nif::load(&bytes).ok()?;
            let (lo, hi) = mw_nif::visible_bounds(&model)?;
            let mut textures: Vec<String> = model
                .materials
                .iter()
                .filter_map(|mat| mat.base_texture.clone())
                .map(|t| t.to_ascii_lowercase())
                .collect();
            textures.sort();
            textures.dedup();
            Some((
                m.clone(),
                (Vec3::from_array(lo), Vec3::from_array(hi), textures),
            ))
        })
        .collect();
    println!(
        "model bounds: {} of {} placed models",
        bounds.len(),
        models.len()
    );

    let world_aabb = |model: &str, trs: &mw_nif::coords::Trs| -> Option<(Vec3, Vec3)> {
        let (lo, hi, _) = bounds.get(model)?;
        let rot = Quat::from_array(trs.rotation);
        let pos = Vec3::from_array(trs.translation);
        let mut out: Option<(Vec3, Vec3)> = None;
        for i in 0..8 {
            let c = Vec3::new(
                if i & 1 == 0 { lo.x } else { hi.x },
                if i & 2 == 0 { lo.y } else { hi.y },
                if i & 4 == 0 { lo.z } else { hi.z },
            );
            let p = pos + rot * (c * trs.scale);
            out = Some(out.map_or((p, p), |(a, b)| (a.min(p), b.max(p))));
        }
        out
    };

    // 1. Exterior cells: water height subrecords and flags.
    let mut ext = 0;
    let mut ext_whgt = 0;
    let mut ext_flag = 0;
    let mut ext_values: BTreeMap<i64, usize> = BTreeMap::new();
    let mut region_of: HashMap<(i32, i32), String> = HashMap::new();
    for (_, cell) in db
        .iter::<Cell>()
        .filter(|(_, c)| !c.data.flags.contains(CellFlags::IS_INTERIOR))
    {
        ext += 1;
        ext_flag += usize::from(cell.data.flags.contains(CellFlags::HAS_WATER));
        if let Some(h) = cell.water_height {
            ext_whgt += 1;
            *ext_values.entry(h.round() as i64).or_default() += 1;
        }
        region_of.insert(cell.data.grid, cell.region.clone().unwrap_or_default());
    }
    println!(
        "\n[exterior cells] {ext}: with a water height subrecord {ext_whgt}, HAS_WATER flag {ext_flag}"
    );
    println!("  water height values (units, rounded: cells): {ext_values:?}");

    // 2. Interiors with water.
    struct Interior {
        name: String,
        level: f32,
        geo_min: f32,
        wet_area: f32,
    }
    let mut interiors = Vec::new();
    let mut int_total = 0;
    let mut int_no_geometry = 0;
    for (_, cell) in db
        .iter::<Cell>()
        .filter(|(_, c)| c.data.flags.contains(CellFlags::IS_INTERIOR))
    {
        int_total += 1;
        if !cell.data.flags.contains(CellFlags::HAS_WATER) {
            continue;
        }
        let level = (f64::from(cell.water_height.unwrap_or(0.0)) * UNIT_METERS_F64) as f32;
        let (placements, _) = mw_world::cell_placements(db, cell);
        let boxes: Vec<(Vec3, Vec3)> = placements
            .iter()
            .filter(|p| !p.editor_only)
            .filter_map(|p| world_aabb(p.model.as_deref()?, &p.transform))
            .collect();
        let Some(geo_min) = boxes.iter().map(|b| b.0.y).min_by(f32::total_cmp) else {
            int_no_geometry += 1;
            continue;
        };
        // Wet area proxy: XZ extent of the geometry reaching below the level.
        let wet: Vec<_> = boxes.iter().filter(|b| b.0.y < level).collect();
        let wet_area = if wet.is_empty() {
            0.0
        } else {
            let lo = wet.iter().fold(Vec3::splat(f32::MAX), |a, b| a.min(b.0));
            let hi = wet.iter().fold(Vec3::splat(f32::MIN), |a, b| a.max(b.1));
            (hi.x - lo.x) * (hi.z - lo.z)
        };
        interiors.push(Interior {
            name: cell.name.clone(),
            level,
            geo_min,
            wet_area,
        });
    }
    let visible: Vec<&Interior> = interiors.iter().filter(|i| i.level > i.geo_min).collect();
    println!(
        "\n[interiors] {int_total}; HAS_WATER {} (+{int_no_geometry} without geometry): visible {} , below all geometry {}",
        interiors.len(),
        visible.len(),
        interiors.len() - visible.len()
    );
    let mut levels: Vec<f32> = visible.iter().map(|i| i.level).collect();
    levels.sort_by(f32::total_cmp);
    if !levels.is_empty() {
        println!(
            "  visible levels (m): min {:.1}, median {:.1}, max {:.1}; at exactly 0: {}",
            levels[0],
            levels[levels.len() / 2],
            levels[levels.len() - 1],
            levels.iter().filter(|l| **l == 0.0).count()
        );
    }
    let mut by_area: Vec<&&Interior> = visible.iter().collect();
    by_area.sort_by(|a, b| b.wet_area.total_cmp(&a.wet_area));
    println!("  10 largest wet extents (XZ box of geometry below the level, m²):");
    for i in by_area.iter().take(10) {
        println!(
            "    {:>8.0}  level {:>7.1} m  {}",
            i.wet_area, i.level, i.name
        );
    }

    // 3. Boats and 4. authored water meshes, over all cells.
    let water_model = |m: &str| {
        let (_, _, tex) = &bounds[m];
        m.contains("water") || m.contains("waterfall") || tex.iter().any(|t| t.contains("water"))
    };
    let mut boats: BTreeMap<&str, [usize; 4]> = BTreeMap::new(); // total, moored, above, sunk
    let mut water_meshes: BTreeMap<String, (usize, usize)> = BTreeMap::new(); // exterior, interior
    for (_, cell) in db.iter::<Cell>() {
        let interior = cell.data.flags.contains(CellFlags::IS_INTERIOR);
        let level = if interior {
            if cell.data.flags.contains(CellFlags::HAS_WATER) {
                (f64::from(cell.water_height.unwrap_or(0.0)) * UNIT_METERS_F64) as f32
            } else {
                f32::NEG_INFINITY
            }
        } else {
            0.0
        };
        let (placements, _) = mw_world::cell_placements(db, cell);
        for p in &placements {
            let Some(model) = p.model.as_deref() else {
                continue;
            };
            let short = model.trim_start_matches("meshes/");
            if let Some(boat) = BOATS.iter().find(|b| **b == short)
                && let Some((lo, hi)) = world_aabb(model, &p.transform)
            {
                let e = boats.entry(boat).or_default();
                e[0] += 1;
                if lo.y < level && hi.y > level {
                    e[1] += 1;
                } else if lo.y >= level {
                    e[2] += 1;
                } else {
                    e[3] += 1;
                }
            }
            if bounds.contains_key(model) && water_model(model) {
                let e = water_meshes.entry(short.to_owned()).or_default();
                if interior { e.1 += 1 } else { e.0 += 1 }
            }
        }
    }
    println!("\n[boats] model: placements (moored = hull crosses the level, above, sunk)");
    for (m, [n, moored, above, sunk]) in &boats {
        println!("  {m:<24} {n:>4}  moored {moored:>3}  above {above:>3}  sunk {sunk:>3}");
    }
    println!(
        "\n[water meshes] models whose path or base texture names water: {}",
        water_meshes.len()
    );
    let mut water_textures: BTreeMap<String, usize> = BTreeMap::new();
    for (m, (e, i)) in &water_meshes {
        let tex = &bounds[&format!("meshes/{m}")].2;
        let wt: Vec<&String> = tex.iter().filter(|t| t.contains("water")).collect();
        for t in &wt {
            *water_textures.entry((*t).clone()).or_default() += 1;
        }
        println!("  {m:<40} ext {e:>4} int {i:>4}  water textures {wt:?}");
    }
    println!(
        "  distinct water textures: {} {:?}",
        water_textures.len(),
        water_textures.keys().collect::<Vec<_>>()
    );

    // 5. Sea-level water components on the LAND vertex grid (1.79 m), 4-connected; cells without
    // LAND count as water (deep: the engine's fill height), the outer ring is open sea.
    let lands: HashMap<(i32, i32), LandData> = db
        .iter::<Landscape>()
        .map(|(_, l)| (l.grid, LandData::decode(l)))
        .collect();
    let (mut gx0, mut gy0, mut gx1, mut gy1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for &(x, y) in lands.keys() {
        (gx0, gy0, gx1, gy1) = (gx0.min(x), gy0.min(y), gx1.max(x), gy1.max(y));
    }
    let (gx0, gy0) = (gx0 - 1, gy0 - 1);
    let (gx1, gy1) = (gx1 + 1, gy1 + 1);
    let step = LAND_VERTS - 1;
    let w = ((gx1 - gx0 + 1) as usize) * step + 1;
    let h = ((gy1 - gy0 + 1) as usize) * step + 1;
    let mut wet = vec![true; w * h];
    for (&(cx, cy), land) in &lands {
        let ox = ((cx - gx0) as usize) * step;
        let oy = ((cy - gy0) as usize) * step;
        for j in 0..LAND_VERTS {
            for i in 0..LAND_VERTS {
                wet[(oy + j) * w + ox + i] = land.heights[j * LAND_VERTS + i] < 0.0;
            }
        }
    }
    let mut label = vec![u32::MAX; w * h];
    let mut comps: Vec<(usize, (i32, i32))> = Vec::new(); // samples, a cell in it
    let mut stack = Vec::new();
    for start in 0..w * h {
        if !wet[start] || label[start] != u32::MAX {
            continue;
        }
        let id = comps.len() as u32;
        label[start] = id;
        stack.push(start);
        let mut n = 0;
        while let Some(i) = stack.pop() {
            n += 1;
            let (x, y) = (i % w, i / w);
            let mut visit = |j: usize| {
                if wet[j] && label[j] == u32::MAX {
                    label[j] = id;
                    stack.push(j);
                }
            };
            if x > 0 {
                visit(i - 1);
            }
            if x + 1 < w {
                visit(i + 1);
            }
            if y > 0 {
                visit(i - w);
            }
            if y + 1 < h {
                visit(i + w);
            }
        }
        let cell = (
            (start % w / step) as i32 + gx0,
            (start / w / step) as i32 + gy0,
        );
        comps.push((n, cell));
    }
    let area = |n: usize| n as f64 * SAMPLE_M * SAMPLE_M;
    let sea = label[0] as usize;
    println!(
        "\n[sea-level components] grid {w}×{h} at {SAMPLE_M:.2} m, cells {gx0}..{gx1} × {gy0}..{gy1}: {} components",
        comps.len()
    );
    println!(
        "  open sea (touches the outer ring): {:.1} km²",
        area(comps[sea].0) / 1e6
    );
    let mut enclosed: Vec<(usize, &(usize, (i32, i32)))> = comps
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != sea)
        .collect();
    enclosed.sort_by_key(|c| std::cmp::Reverse(c.1.0));
    let mut hist = [0usize; 5];
    for (_, (n, _)) in &enclosed {
        let a = area(*n);
        hist[if a < 100.0 {
            0
        } else if a < 1e3 {
            1
        } else if a < 1e4 {
            2
        } else if a < 1e5 {
            3
        } else {
            4
        }] += 1;
    }
    println!(
        "  enclosed: {} (< 100 m² {}, 100 m²-0.1 ha {}, 0.1-1 ha {}, 1-10 ha {}, > 10 ha {}), total {:.2} km²",
        enclosed.len(),
        hist[0],
        hist[1],
        hist[2],
        hist[3],
        hist[4],
        enclosed.iter().map(|(_, c)| area(c.0)).sum::<f64>() / 1e6
    );
    println!("  10 largest enclosed (area, a cell in it, its region):");
    for (_, (n, cell)) in enclosed.iter().take(10) {
        println!(
            "    {:>9.0} m²  {:?} {}",
            area(*n),
            cell,
            region_of.get(cell).map_or("", String::as_str)
        );
    }

    // 6. Regions with water: wet samples per region (cells with LAND only), open sea vs enclosed.
    let mut per_region: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new(); // cells, sea, enclosed
    for &(cx, cy) in lands.keys() {
        let region = region_of.get(&(cx, cy)).cloned().unwrap_or_default();
        let e = per_region
            .entry(if region.is_empty() {
                "(none)".into()
            } else {
                region
            })
            .or_default();
        e.0 += 1;
        let ox = ((cx - gx0) as usize) * step;
        let oy = ((cy - gy0) as usize) * step;
        for j in 0..step {
            for i in 0..step {
                let l = label[(oy + j) * w + ox + i];
                if l == u32::MAX {
                    continue;
                }
                if l as usize == sea {
                    e.1 += 1;
                } else {
                    e.2 += 1;
                }
            }
        }
    }
    println!(
        "\n[regions] region: LAND cells, open-sea water km², enclosed water km², wet fraction"
    );
    for (r, (cells, s, en)) in &per_region {
        let total = (*cells * step * step) as f64;
        println!(
            "  {r:<28} {cells:>4}  sea {:>7.3}  enclosed {:>6.3}  wet {:>5.1} %",
            area(*s) / 1e6,
            area(*en) / 1e6,
            100.0 * (*s + *en) as f64 / total
        );
    }
    let swamp: Vec<_> = enclosed
        .iter()
        .filter(|(_, (_, c))| {
            region_of
                .get(c)
                .is_some_and(|r| r.to_ascii_lowercase().contains("bitter"))
        })
        .collect();
    println!(
        "  Bitter Coast enclosed components (by a cell in each): {}, {:.3} km²",
        swamp.len(),
        swamp.iter().map(|(_, c)| area(c.0)).sum::<f64>() / 1e6
    );
    println!("\ncensus in {:.2?}", t.elapsed());
    Ok(())
}
