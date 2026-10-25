//! Static collision over the real install (`mw_bake::collision`): bakes
//! `model.info`/`mesh.cook` (usually cache hits), then `collision.cook`
//! for every referenced model, and reports sources, triangle totals and
//! the step's time. Merges the products into the profile manifest.
//!
//! `cargo run --release -p mw_bake --example collision_report [-- <Morrowind folder>]`
//!
//! `SF_COLLISION_CACHE=<dir>` bakes into a scratch store instead (a cold
//! first bake of models plus collision).
//!
//! `SF_COLLISION_CELL="<cell>"`: a collision census of one cell instead
//! (bakes only that cell's models). One row per placement that could
//! support the player (statics, activators, doors, containers, lights;
//! no items, actors or editor markers), in world meters: the collision
//! source, triangle counts, the open-edge length of the collision mesh
//! (edges used by one triangle) next to the welded render LOD0's, and how
//! far the visible floor-facing render triangles (normal·up > 0.7, sampled
//! every [`SPACING`]) are from the model's own collision and from every
//! collision mesh in the cell. Sorted by the worst gap against the whole
//! cell: an uncovered visible floor is a hole the player can fall through
//! (physics-fix-plan.md slice 5).

#![allow(clippy::cast_precision_loss)]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use glam::{Affine3A, Vec3};
use mw_bake::collision::{self, Source};
use mw_bake::{VfsInputs, collision::MwCollisionRules};
use mw_install::MorrowindInstall;
use mw_nif::NodeKind;
use mw_world::{BaseKind, Placement};
use sf_bake::{Inputs, Manifest, Store};
use sf_controller::CollisionMesh;
use sf_controller::steps::{COLLISION_COOK, CollisionSettings, DefaultCollisionRules, from_cooked};
use sf_model::Cooked;
use sf_model::steps::MeshCookStep;
use sf_records::RecordDb;
use sf_vfs::Vfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let scratch = std::env::var_os("SF_COLLISION_CACHE").map(std::path::PathBuf::from);
    let root = match &scratch {
        Some(dir) => dir.clone(),
        None => mw_bake::default_cache_root().ok_or("no cache root")?,
    };
    let mut models = mw_bake::referenced_models(&content.db);
    let importers = mw_bake::importers();
    models.retain(|p, _| importers.for_path(p).is_some());
    let census = match std::env::var("SF_COLLISION_CELL") {
        Ok(name) => Some((cell_supports(&content.db, &name)?, name)),
        Err(_) => None,
    };
    let paths: Vec<String> = match &census {
        Some((placements, _)) => {
            let mut paths: Vec<String> = placements.iter().filter_map(|p| p.model.clone()).collect();
            paths.sort();
            paths.dedup();
            paths.retain(|p| models.contains_key(p));
            paths
        }
        None => models.keys().cloned().collect(),
    };
    let store = Store::open(&root)?;
    let profile = mw_bake::profile_name(&install);
    let db = Arc::new(content.db);
    let inputs = Arc::new(VfsInputs::new(Arc::new(vfs), &install, root.join("filehash.memo")).with_records(db));
    let loaded = started.elapsed();

    let baker = mw_bake::model_baker(store.clone(), inputs.clone());
    let t = Instant::now();
    let models_report = baker.bake(&mw_bake::model_keys(&paths));
    let models_time = t.elapsed();
    let t = Instant::now();
    let collision_report = baker.bake(&collision::collision_keys(&paths));
    let collision_time = t.elapsed();
    let manifest = baker.manifest();
    inputs.save()?;
    let mut saved = store.load_manifest(&profile).unwrap_or_default();
    saved.merge(&manifest);
    store.save_manifest(&profile, &saved)?;

    let mesh = models_report.steps.get("mesh.cook").copied().unwrap_or_default();
    println!(
        "load {loaded:.2?}; models {models_time:.2?} (mesh.cook built {} hit {}); collision {collision_time:.2?}",
        mesh.built, mesh.hits
    );
    let t = Instant::now();
    if let Some((placements, name)) = census {
        let step = collision_report.steps.get("collision.cook").copied().unwrap_or_default();
        println!("collision.cook built {} hit {} failed {}", step.built, step.hits, step.failed);
        run_census(&name, &placements, &store, &manifest, inputs.as_ref());
        println!("(census {:.2?})", t.elapsed());
        return Ok(());
    }
    let mut report = collision::report(&store, &manifest, inputs.as_ref(), &models);
    report.step = collision_report.steps.get("collision.cook").map(|s| (*s, collision_time));
    print!("{report}");
    println!("(report classification {:.2?})", t.elapsed());
    Ok(())
}

/// Floor samples per render triangle: one per sub-triangle of this edge, meters.
const SPACING: f32 = 0.25;
/// A floor sample farther than this from collision is "missed", meters.
const GAP: f32 = 0.05;
/// Distances are searched up to this, meters ("no collision near").
const CAP: f32 = 5.0;
/// Floor-facing: the triangle normal's up component.
const FLOOR_UP: f32 = 0.7;

/// The cell's placements whose collision could hold the player up.
fn cell_supports(db: &RecordDb, name: &str) -> Result<Vec<Placement>, String> {
    let cell = mw_world::find_cell(db, name).ok_or_else(|| format!("no cell {name:?}"))?;
    let (placements, _) = mw_world::cell_placements(db, cell);
    Ok(placements
        .into_iter()
        .filter(|p| p.model.is_some() && !p.editor_only && !matches!(p.kind, BaseKind::Item | BaseKind::Actor))
        .collect())
}

/// One model's render and collision geometry (model space).
struct ModelData {
    source: String,
    /// Visible render triangles of the NIF (not collision, billboard or
    /// particle geometry): what the player sees.
    render: Vec<[Vec3; 3]>,
    /// Hidden render triangles outside a `RootCollisionNode` (`OpenMW`
    /// collides with them when the model has none).
    hidden: usize,
    lods: usize,
    /// LOD1's error over the model's bounding radius, percent.
    lod1_error: f32,
    lod0: CollisionMesh,
    collision: CollisionMesh,
}

fn model_data(path: &str, store: &Store, manifest: &Manifest, inputs: &dyn Inputs) -> Result<ModelData, String> {
    let read = |step: &str| -> Result<Vec<u8>, String> {
        let digest = manifest.get(step, path).ok_or_else(|| format!("no {step} product"))?;
        store.get(&digest).map_err(|e| e.to_string())
    };
    let collision = CollisionMesh::from_bytes(&read(COLLISION_COOK)?).map_err(|e| e.to_string())?;
    let cooked = Cooked::from_bytes(&read(MeshCookStep::ID)?).map_err(|e| e.to_string())?;
    let settings = CollisionSettings { lod: 0, ..CollisionSettings::default() };
    let welded = from_cooked(&cooked, &settings, &DefaultCollisionRules)?;
    let lods = match &cooked {
        Cooked::Static(m) => m.lods.len(),
        _ => 0,
    };
    let bytes = inputs.read_file(path).map_err(|e| e.to_string())?;
    let nif = mw_nif::load(&bytes).map_err(|e| e.to_string())?;
    let source = match collision::collision_of(&nif, MwCollisionRules::default().weld).0 {
        Source::RenderLod if lods <= 1 => "fallback LOD0 (1 LOD)".to_owned(),
        Source::RenderLod => "fallback LOD1".to_owned(),
        Source::Authored => "authored RCN".to_owned(),
        Source::RestPose => "rest pose (animated)".to_owned(),
        other @ Source::None(_) => other.label().to_owned(),
    };
    let world = mw_nif::world_transforms(&nif);
    let mut facing = vec![false; nif.nodes.len()];
    let (mut render, mut hidden) = (Vec::new(), 0);
    for (i, node) in nif.nodes.iter().enumerate() {
        facing[i] = matches!(node.kind, NodeKind::Billboard | NodeKind::Particles) || node.parent.is_some_and(|p| facing[p]);
        let Some(mesh) = node.mesh.and_then(|m| nif.meshes.get(m)) else { continue };
        if mesh.collision || facing[i] {
            continue;
        }
        let transform = Affine3A::from_cols_array(&world[i].to_cols_array());
        let corner = |k: u32| mesh.positions.get(k as usize).map(|p| transform.transform_point3(Vec3::from_array(*p)));
        for c in mesh.indices.as_chunks::<3>().0 {
            let (Some(a), Some(b), Some(d)) = (corner(c[0]), corner(c[1]), corner(c[2])) else { continue };
            if node.visible {
                render.push([a, b, d]);
            } else {
                hidden += 1;
            }
        }
    }
    let (lo, hi) = render.iter().flatten().fold((Vec3::MAX, Vec3::MIN), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
    let radius = if render.is_empty() { 0.0 } else { (hi - lo).length() * 0.5 };
    let lod1_error = match &cooked {
        Cooked::Static(m) if m.lods.len() > 1 && radius > 0.0 => m.lods[1].error / radius * 100.0,
        _ => 0.0,
    };
    Ok(ModelData { source, render, hidden, lods, lod1_error, lod0: welded, collision })
}

/// A world-space triangle with its bounds.
struct Tri {
    v: [Vec3; 3],
    lo: Vec3,
    hi: Vec3,
}

impl Tri {
    fn new(v: [Vec3; 3]) -> Self {
        Self { v, lo: v[0].min(v[1]).min(v[2]), hi: v[0].max(v[1]).max(v[2]) }
    }
}

fn world_tris(mesh: &CollisionMesh, transform: &Affine3A) -> Vec<Tri> {
    let p = |i: u32| transform.transform_point3(Vec3::from_array(mesh.positions[i as usize]));
    mesh.triangles.iter().map(|t| Tri::new([p(t[0]), p(t[1]), p(t[2])])).collect()
}

/// Closest point of triangle `abc` to `p` (Ericson, Real-Time Collision
/// Detection, 5.1.5).
#[allow(clippy::many_single_char_names)]
fn closest(p: Vec3, [a, b, c]: [Vec3; 3]) -> Vec3 {
    let (ab, ac, ap) = (b - a, c - a, p - a);
    let (d1, d2) = (ab.dot(ap), ac.dot(ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = p - b;
    let (d3, d4) = (ab.dot(bp), ac.dot(bp));
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return a + ab * (d1 / (d1 - d3));
    }
    let cp = p - c;
    let (d5, d6) = (ab.dot(cp), ac.dot(cp));
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return a + ac * (d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && d4 - d3 >= 0.0 && d5 - d6 >= 0.0 {
        return b + (c - b) * ((d4 - d3) / ((d4 - d3) + (d5 - d6)));
    }
    let q = a + ab * (vb / (va + vb + vc)) + ac * (vc / (va + vb + vc));
    if q.is_finite() { q } else { a }
}

/// Distance from `p` to the nearest triangle, capped at [`CAP`].
fn distance(tris: &[Tri], p: Vec3) -> f32 {
    let mut best = CAP;
    for t in tris {
        if (t.lo - p).max(p - t.hi).max(Vec3::ZERO).length() < best {
            best = best.min(closest(p, t.v).distance(p));
        }
    }
    best
}

/// Sub-triangle centroids of `t` (about [`SPACING`] apart) and the area each stands for.
fn samples(t: [Vec3; 3]) -> (Vec<Vec3>, f32) {
    let longest = t[0].distance(t[1]).max(t[1].distance(t[2])).max(t[2].distance(t[0]));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = ((longest / SPACING).ceil() as u32).clamp(1, 40);
    let (u, v) = (t[1] - t[0], t[2] - t[0]);
    let nf = n as f32;
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n - i {
            let (fi, fj) = (i as f32, j as f32);
            out.push(t[0] + u * ((fi + 1.0 / 3.0) / nf) + v * ((fj + 1.0 / 3.0) / nf));
            if i + j + 1 < n {
                out.push(t[0] + u * ((fi + 2.0 / 3.0) / nf) + v * ((fj + 2.0 / 3.0) / nf));
            }
        }
    }
    (out, u.cross(v).length() * 0.5 / (nf * nf))
}

/// Total length of edges used by one triangle, in the mesh's units.
fn open_edges(mesh: &CollisionMesh) -> f32 {
    let mut edges: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    for t in &mesh.triangles {
        for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            *edges.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    let p = |i: u32| Vec3::from_array(mesh.positions[i as usize]);
    edges.iter().filter(|(_, n)| **n == 1).map(|((a, b), _)| p(*a).distance(p(*b))).sum()
}

/// Heights of the highest face-up and face-down (by winding) collision
/// triangles on the vertical line through `p`, at most `reach` above it.
#[allow(clippy::many_single_char_names)]
fn ground_below(tris: &[Tri], p: Vec3, reach: f32) -> [Option<f32>; 2] {
    let top = p.y + reach;
    let mut best = [None; 2];
    for t in tris {
        if p.x < t.lo.x || p.x > t.hi.x || p.z < t.lo.z || p.z > t.hi.z || t.lo.y > top {
            continue;
        }
        let [a, b, c] = t.v;
        let det = (b.z - c.z) * (a.x - c.x) + (c.x - b.x) * (a.z - c.z);
        if det.abs() < 1e-9 {
            continue;
        }
        let wa = ((b.z - c.z) * (p.x - c.x) + (c.x - b.x) * (p.z - c.z)) / det;
        let wb = ((c.z - a.z) * (p.x - c.x) + (a.x - c.x) * (p.z - c.z)) / det;
        let wc = 1.0 - wa - wb;
        if wa < -1e-5 || wb < -1e-5 || wc < -1e-5 {
            continue;
        }
        let y = wa * a.y + wb * b.y + wc * c.y;
        let side = usize::from((b - a).cross(c - a).y < 0.0);
        if y <= top && best[side].is_none_or(|h| y > h) {
            best[side] = Some(y);
        }
    }
    best
}

/// Collision up to this far above a floor sample still holds it, meters.
const REACH: f32 = 0.3;
/// A floor sample with no collision within this below it is a hole, meters.
const HOLE: f32 = 0.5;
/// ... and no collision this close (a capsule's radius): render floor
/// inside a wall's thickness or under a collision box's edge is no hole.
const RADIUS: f32 = 0.3;

/// One placement's census row (world meters).
struct Row {
    base: String,
    model: String,
    source: String,
    render_tris: usize,
    hidden_tris: usize,
    lods: usize,
    lod1_error: f32,
    lod0_tris: usize,
    collision_tris: usize,
    open: f32,
    lod0_open: f32,
    floor_area: f32,
    /// Floor area farther than [`GAP`] from the model's own collision.
    own_missing: f32,
    /// ... and from every collision mesh in the cell.
    cell_missing: f32,
    own_gap: f32,
    cell_gap: f32,
    /// Floor area with no cell collision from [`REACH`] above to [`HOLE`]
    /// below, and none within [`RADIUS`]: where the player falls through.
    hole: f32,
    /// Floor area held up only by collision triangles wound face-down.
    flipped: f32,
    /// Bounds of those samples.
    flip_range: [Vec3; 2],
    /// The deepest drop under a hole sample (capped at [`CAP`]).
    drop: f32,
    drop_at: Vec3,
    pos: Vec3,
}

fn run_census(name: &str, placements: &[Placement], store: &Store, manifest: &Manifest, inputs: &dyn Inputs) {
    let mut data: BTreeMap<String, Result<ModelData, String>> = BTreeMap::new();
    for model in placements.iter().filter_map(|p| p.model.as_deref()) {
        if !data.contains_key(model) {
            data.insert(model.to_owned(), model_data(model, store, manifest, inputs));
        }
    }
    let transforms: Vec<Affine3A> = placements
        .iter()
        .map(|p| Affine3A::from_cols_array(&p.transform.to_affine().to_cols_array()))
        .collect();
    // Every collision triangle of the cell, world space.
    let mut cell: Vec<Tri> = Vec::new();
    for (p, t) in placements.iter().zip(&transforms) {
        if let Some(Ok(d)) = p.model.as_ref().and_then(|m| data.get(m)) {
            cell.extend(world_tris(&d.collision, t));
        }
    }
    let mut rows = Vec::new();
    for (p, transform) in placements.iter().zip(&transforms) {
        let model = p.model.clone().unwrap_or_default();
        let d = match data.get(&model) {
            Some(Ok(d)) => d,
            Some(Err(e)) => {
                println!("  {model} ({}): {e}", p.base_id);
                continue;
            }
            None => continue,
        };
        let own = world_tris(&d.collision, transform);
        let scale = p.transform.scale;
        let mut row = Row {
            base: p.base_id.clone(),
            model,
            source: d.source.clone(),
            render_tris: d.render.len(),
            hidden_tris: d.hidden,
            lods: d.lods,
            lod1_error: d.lod1_error,
            lod0_tris: d.lod0.triangles.len(),
            collision_tris: d.collision.triangles.len(),
            open: open_edges(&d.collision) * scale,
            lod0_open: open_edges(&d.lod0) * scale,
            floor_area: 0.0,
            own_missing: 0.0,
            cell_missing: 0.0,
            own_gap: 0.0,
            cell_gap: 0.0,
            hole: 0.0,
            flipped: 0.0,
            flip_range: [Vec3::MAX, Vec3::MIN],
            drop: 0.0,
            drop_at: transform.translation.into(),
            pos: transform.translation.into(),
        };
        for t in &d.render {
            let w = t.map(|v| transform.transform_point3(v));
            let normal = (w[1] - w[0]).cross(w[2] - w[0]).normalize_or_zero();
            if normal.y <= FLOOR_UP {
                continue;
            }
            let (points, area) = samples(w);
            for s in points {
                row.floor_area += area;
                let [up, down] = ground_below(&cell, s, REACH);
                let drop = up.into_iter().chain(down).reduce(f32::max).map_or(CAP, |h| (s.y - h).min(CAP));
                // Only face-down triangles within the hole depth hold this sample up.
                if drop <= HOLE && up.is_none_or(|h| s.y - h > HOLE) {
                    row.flipped += area;
                    row.flip_range = [row.flip_range[0].min(s), row.flip_range[1].max(s)];
                }
                if drop > HOLE && distance(&cell, s) > RADIUS {
                    row.hole += area;
                    if drop > row.drop {
                        row.drop = drop;
                        row.drop_at = s;
                    }
                }
                let gap = distance(&own, s);
                row.own_gap = row.own_gap.max(gap);
                if gap <= GAP {
                    continue;
                }
                row.own_missing += area;
                let gap = distance(&cell, s);
                if gap > GAP {
                    row.cell_missing += area;
                }
                row.cell_gap = row.cell_gap.max(gap);
            }
        }
        rows.push(row);
    }
    rows.sort_by(|a, b| {
        (b.hole, b.drop, b.cell_missing)
            .partial_cmp(&(a.hole, a.drop, a.cell_missing))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.model.cmp(&b.model))
            .then_with(|| a.base.cmp(&b.base))
    });
    print_census(name, &rows, cell.len());
}

/// Engine meters to Morrowind units (`mw_nif::coords` inverted).
fn to_mw(p: Vec3) -> Vec3 {
    Vec3::new(p.x, -p.z, p.y) / mw_nif::coords::UNIT_METERS
}

fn print_census(name: &str, rows: &[Row], cell_tris: usize) {
    println!(
        "census {name:?}: {} placements, {cell_tris} collision triangles; floor = render normal.y > {FLOOR_UP}, \
         sampled every {SPACING} m",
        rows.len()
    );
    println!(
        "  hole = floor m2 with no cell collision from {REACH} m above to {HOLE} m below and none within \
         {RADIUS} m; drop = deepest fall under a hole sample (cap {CAP}); cgap/ogap = max distance to cell/own collision, \
         cmiss/omiss = floor m2 > {GAP} m from it"
    );
    println!(
        "  {:>6} {:>5} {:>5} {:>5} {:>5} {:>6} {:>6} {:>6} | {:<22} {:>5} {:>4} {:>3} {:>5} {:>5} {:>5} {:>6} {:>6} | model (base) at engine x,y,z",
        "hole", "flip", "drop", "cgap", "ogap", "cmiss", "omiss", "floor", "source", "rend", "hid", "lod", "err%", "lod0", "coll", "open", "l0open"
    );
    for r in rows {
        println!(
            "  {:6.2} {:5.2} {:5.2} {:5.2} {:5.2} {:6.2} {:6.2} {:6.1} | {:<22} {:5} {:4} {:3} {:5.1} {:5} {:5} {:6.1} {:6.1} | {} ({}) at {:.2},{:.2},{:.2}",
            r.hole,
            r.flipped,
            r.drop,
            r.cell_gap,
            r.own_gap,
            r.cell_missing,
            r.own_missing,
            r.floor_area,
            r.source,
            r.render_tris,
            r.hidden_tris,
            r.lods,
            r.lod1_error,
            r.lod0_tris,
            r.collision_tris,
            r.open,
            r.lod0_open,
            r.model,
            r.base,
            r.pos.x,
            r.pos.y,
            r.pos.z
        );
    }
    let mut sources: BTreeMap<&str, usize> = BTreeMap::new();
    for r in rows {
        *sources.entry(r.source.as_str()).or_default() += 1;
    }
    println!("summary: placements by source {sources:?}");
    let none: Vec<&str> = rows.iter().filter(|r| r.collision_tris == 0).map(|r| r.base.as_str()).collect();
    println!("  no collision ({}): {}", none.len(), none.join(", "));
    for r in rows.iter().filter(|r| r.hole > 0.01) {
        let mw = to_mw(r.drop_at);
        println!(
            "  HOLE: {} ({}) [{}]: {:.2} m2 of visible floor over no collision, drop {:.2} m at engine {:.2},{:.2},{:.2} (MW {:.0},{:.0},{:.0})",
            r.model, r.base, r.source, r.hole, r.drop, r.drop_at.x, r.drop_at.y, r.drop_at.z, mw.x, mw.y, mw.z
        );
    }
    let coarse: f32 = rows.iter().map(|r| r.cell_missing).sum();
    println!("  visible floor > {GAP} m from any collision (coarse but supported): {coarse:.1} m2 in total");
    let flipped: f32 = rows.iter().map(|r| r.flipped).sum();
    println!("  visible floor held up only by face-down collision triangles: {flipped:.1} m2 in total");
    for r in rows.iter().filter(|r| r.flipped > 0.5) {
        let (lo, hi) = (r.flip_range[0], r.flip_range[1]);
        println!(
            "  face-down support {:.2} m2: {} ({}) [{}], samples in engine {:.2},{:.2},{:.2} .. {:.2},{:.2},{:.2}",
            r.flipped, r.model, r.base, r.source, lo.x, lo.y, lo.z, hi.x, hi.y, hi.z
        );
    }
}
