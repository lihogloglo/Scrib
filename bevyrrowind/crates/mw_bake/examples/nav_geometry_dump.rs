//! Scratch (NV0 navmesh spike, `superfargoth/docs/research/navmesh-spike-nv0-2026-11-11.md`):
//! dumps navmesh input geometry from the last bake's `collision.cook` products (+ LAND
//! heights for exteriors) as a raw triangle soup, for an out-of-workspace `navmesh-rs`
//! harness. Doors are left out of the soup (nav.md "Door polygons") and written as
//! world-space boxes instead, for convex area volumes.
//!
//! `cargo run --release -p mw_bake --example nav_geometry_dump -- <out dir> [<Morrowind folder>]`
//!
//! File `<name>.navgeo`: b"NVG1", u32 nverts, u32 ntris, u32 ndoors, then f32 verts (x, y, z;
//! meters, Y up, relative to the block origin), u32 tris, and per door f32 [min xyz, max xyz].
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::too_many_lines
)]

use std::io::Write as _;

use glam::{DVec3, Vec3};
use mw_install::MorrowindInstall;
use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::HeightSampler;
use mw_world::{MwWorldSource, SourceOptions, ids};
use sf_bake::{Manifest, Store};
use sf_controller::CollisionMesh;
use sf_controller::steps::COLLISION_COOK;
use sf_vfs::Vfs;
use sf_world::{Placement, PlacementClass};

const CELL_UNITS: f64 = 8192.0;
/// LAND vertex spacing, Morrowind units.
const LAND_STEP: f64 = 128.0;

#[derive(Default)]
struct Soup {
    verts: Vec<[f32; 3]>,
    tris: Vec<[u32; 3]>,
    doors: Vec<[f32; 6]>,
    missing: u32,
    models: u32,
}

impl Soup {
    fn push_mesh(&mut self, mesh: &CollisionMesh, p: &Placement, origin: DVec3) {
        let base = self.verts.len() as u32;
        let t = &p.transform;
        let pos = (t.pos.to_f64() - origin).as_vec3();
        for v in &mesh.positions {
            let w = pos + t.rotation * (Vec3::from_array(*v) * t.scale);
            self.verts.push(w.to_array());
        }
        self.tris.extend(
            mesh.triangles
                .iter()
                .map(|t| [t[0] + base, t[1] + base, t[2] + base]),
        );
        self.models += 1;
    }

    fn add_placements<'a>(
        &mut self,
        store: &Store,
        manifest: &Manifest,
        placements: impl Iterator<Item = &'a Placement>,
        origin: DVec3,
    ) {
        for p in placements {
            let Some(model) = &p.model else { continue };
            let mesh = manifest
                .get(COLLISION_COOK, model.as_str())
                .and_then(|d| store.get(&d).ok())
                .and_then(|b| CollisionMesh::from_bytes(&b).ok());
            let Some(mesh) = mesh else {
                self.missing += 1;
                continue;
            };
            if mesh.is_empty() {
                continue;
            }
            if p.class == PlacementClass::Door {
                let mut tmp = Self::default();
                tmp.push_mesh(&mesh, p, origin);
                let (lo, hi) = tmp
                    .verts
                    .iter()
                    .fold((Vec3::MAX, Vec3::MIN), |(lo, hi), v| {
                        (lo.min(Vec3::from_array(*v)), hi.max(Vec3::from_array(*v)))
                    });
                self.doors.push([lo.x, lo.y, lo.z, hi.x, hi.y, hi.z]);
            } else {
                self.push_mesh(&mesh, p, origin);
            }
        }
    }

    fn write(&self, path: &std::path::Path) -> std::io::Result<()> {
        let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
        f.write_all(b"NVG1")?;
        for n in [self.verts.len(), self.tris.len(), self.doors.len()] {
            f.write_all(&(n as u32).to_le_bytes())?;
        }
        for x in self.verts.iter().flatten() {
            f.write_all(&x.to_le_bytes())?;
        }
        for i in self.tris.iter().flatten() {
            f.write_all(&i.to_le_bytes())?;
        }
        for x in self.doors.iter().flatten() {
            f.write_all(&x.to_le_bytes())?;
        }
        f.flush()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("usage: <out dir> [<Morrowind folder>]")?,
    );
    let explicit = std::env::args().nth(2).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let db = &content.db;
    let store = Store::open(mw_bake::default_cache_root().ok_or("no cache root")?)?;
    let manifest = store.load_manifest(&mw_bake::profile_name(&install))?;
    let source = MwWorldSource::build(db, &SourceOptions::default());
    std::fs::create_dir_all(&out)?;
    let keep = |p: &&Placement| {
        matches!(
            p.class,
            PlacementClass::Static | PlacementClass::Dynamic | PlacementClass::Door
        )
    };

    // Interior: the Census and Excise Office.
    let census = ids::interior("Seyda Neen, Census and Excise Office");
    let mut soup = Soup::default();
    soup.add_placements(
        &store,
        &manifest,
        source.all_placements(&census).filter(keep),
        DVec3::ZERO,
    );
    println!(
        "census: {} models, {} missing products, {} verts, {} tris, {} doors",
        soup.models,
        soup.missing,
        soup.verts.len(),
        soup.tris.len(),
        soup.doors.len()
    );
    soup.write(&out.join("census.navgeo"))?;

    // Exterior: the 3x3 cells around Balmora's named cells.
    let named: Vec<(i32, i32)> = db
        .iter::<tes3::esp::Cell>()
        .map(|(_, c)| c)
        .filter(|c| {
            !c.data.flags.contains(tes3::esp::CellFlags::IS_INTERIOR) && c.name == "Balmora"
        })
        .map(|c| c.data.grid)
        .collect();
    let n = named.len().max(1) as f64;
    let cx = (named.iter().map(|g| f64::from(g.0)).sum::<f64>() / n).round() as i32;
    let cy = (named.iter().map(|g| f64::from(g.1)).sum::<f64>() / n).round() as i32;
    println!("balmora named cells {named:?}; block centre ({cx},{cy})");
    let ext = ids::exterior();
    let mut blocks = vec![("balmora3x3".to_owned(), cx, cy)];
    for b in std::env::var("NV_BLOCKS")
        .unwrap_or_default()
        .split(';')
        .filter(|s| !s.is_empty())
    {
        let (x, y) = b.split_once(',').ok_or("NV_BLOCKS: x,y;x,y")?;
        let (x, y): (i32, i32) = (x.parse()?, y.parse()?);
        blocks.push((format!("block_{x}_{y}"), x, y));
    }
    for (name, cx, cy) in blocks {
        let (lo_x, lo_y) = (
            f64::from(cx - 1) * CELL_UNITS,
            f64::from(cy - 1) * CELL_UNITS,
        );
        let (hi_x, hi_y) = (lo_x + 3.0 * CELL_UNITS, lo_y + 3.0 * CELL_UNITS);
        let k = UNIT_METERS_F64;
        // Block origin: the south-west corner at height 0 (world x = mw x, world z = -mw y).
        let origin = DVec3::new(lo_x * k, 0.0, -lo_y * k);
        let inside = |p: &&Placement| {
            let w = p.transform.pos.to_f64();
            let (mx, my) = (w.x / k, -w.z / k);
            (lo_x..hi_x).contains(&mx) && (lo_y..hi_y).contains(&my)
        };
        let mut soup = Soup::default();
        soup.add_placements(
            &store,
            &manifest,
            source.all_placements(&ext).filter(keep).filter(inside),
            origin,
        );
        let objects = soup.tris.len();
        let mut sampler = HeightSampler::new(db);
        let steps = (3.0 * CELL_UNITS / LAND_STEP) as u32;
        let base = soup.verts.len() as u32;
        for j in 0..=steps {
            for i in 0..=steps {
                let (mx, my) = (
                    lo_x + f64::from(i) * LAND_STEP,
                    lo_y + f64::from(j) * LAND_STEP,
                );
                let h = sampler.height(mx as f32, my as f32).unwrap_or(-2048.0);
                let w = DVec3::new(mx * k, f64::from(h) * k, -my * k) - origin;
                soup.verts.push(w.as_vec3().to_array());
            }
        }
        let row = steps + 1;
        for j in 0..steps {
            for i in 0..steps {
                let a = base + j * row + i;
                // Winding with +Y normals (z = -mw y flips handedness).
                soup.tris.push([a, a + 1, a + row]);
                soup.tris.push([a + 1, a + row + 1, a + row]);
            }
        }
        println!(
            "{name}: {} models, {} missing products, {} object tris + {} terrain tris, {} doors; extent {:.1} m",
            soup.models,
            soup.missing,
            objects,
            soup.tris.len() - objects,
            soup.doors.len(),
            3.0 * CELL_UNITS * k
        );
        soup.write(&out.join(format!("{name}.navgeo")))?;
    }

    // Totals for extrapolating a full bake: exterior cells with LAND, interiors and their
    // collision triangle counts (models counted per placement, doors left out).
    let land_cells = db.iter::<tes3::esp::Landscape>().count();
    let mut interiors = 0u32;
    let mut tris = Vec::new();
    for ws in source
        .worldspaces()
        .filter(|w| **w != ext)
        .cloned()
        .collect::<Vec<_>>()
    {
        let mut s = Soup::default();
        let placed = source
            .all_placements(&ws)
            .filter(keep)
            .filter(|p| p.class != PlacementClass::Door);
        s.add_placements(&store, &manifest, placed, DVec3::ZERO);
        if std::env::var_os("NV_INTERIORS").is_some() {
            std::fs::create_dir_all(out.join("interiors"))?;
            s.write(&out.join("interiors").join(format!("{interiors:04}.navgeo")))?;
        }
        interiors += 1;
        tris.push(s.tris.len());
    }
    tris.sort_unstable();
    let total: usize = tris.iter().sum();
    println!(
        "totals: {land_cells} LAND records; {interiors} interior worldspaces, {total} collision tris \
         (median {}, p90 {}, max {})",
        tris[tris.len() / 2],
        tris[tris.len() * 9 / 10],
        tris.last().unwrap_or(&0)
    );
    Ok(())
}
