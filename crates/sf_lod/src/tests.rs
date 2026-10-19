use std::sync::Arc;

use glam::{DVec3, IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{CookSettings, Cooked, CookedModel, Material, MeshData, Model, Node};
use sf_vfs::VfsPath;
use sf_world::{IndexedPlacement, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};

use crate::build::{FarBakeSettings, FarModel, build_l1, build_parent, node_center};
use crate::levels::{Drawn, FadeRange, FarLevels, coverage};
use crate::node::NodeKey;
use crate::policy::{DefaultFarSource, FarPolicy, ModelSummary};
use crate::NodeBlob;
use crate::octa::decode;

/// Deterministic pseudo-random numbers.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        x
    }
}

#[test]
fn node_keys_nest() {
    let cell = IVec3::new(-9, 0, 37);
    let l1 = NodeKey::of_cell(cell);
    let l2 = l1.parent().expect("L2");
    let l3 = l2.parent().expect("L3");
    assert_eq!(l2, NodeKey::containing(cell, 2));
    assert_eq!(l3, NodeKey::containing(cell, 3));
    assert_eq!(l2.index(), IVec3::new(-2, 0, 4));
    assert_eq!(l3.index(), IVec3::new(-1, 0, 1));
    assert!(l3.parent().is_none());
    assert!(l2.contains_cell(cell) && l3.contains_cell(cell));
    let p = Partition::STREAMING;
    assert!((l1.extent(p) - 128.0).abs() < 1e-6 && (l2.extent(p) - 1024.0).abs() < 1e-3 && (l3.extent(p) - 4096.0).abs() < 1e-3);
    // A child's box lies inside its parent's.
    let (o1, o2) = (l1.origin(p).to_f64(), l2.origin(p).to_f64());
    assert!(o1.cmpge(o2).all() && (o1 + 128.0).cmple(o2 + 1024.0).all());
    assert_eq!(NodeKey::parse_tail(2, &l2.subject_tail()), Some(l2));
}

// -- lead bands: no holes ------------------------------------------------------------

/// A synthetic 4 km world: one object per cell at a random spot, cells
/// 32×32 (one L3 node, 16 L2 nodes).
struct Sim {
    levels: FarLevels,
    objects: Vec<(IVec3, DVec3)>,
    /// Center and lead of every node, computed with the bake's rules.
    nodes: std::collections::BTreeMap<NodeKey, (DVec3, f32)>,
}

impl Sim {
    fn new() -> Self {
        let mut rng = Rng(3);
        let mut objects = Vec::new();
        for x in 0..32 {
            for z in 0..32 {
                let p = DVec3::new(f64::from(x) * 128.0 + rng.next() * 128.0 - 128.0, rng.next() * 3.0, f64::from(z) * 128.0 + rng.next() * 128.0 - 128.0);
                let cell = Partition::STREAMING.cell_of(WorldPos::from_f64(p));
                objects.push((cell, p));
            }
        }
        let mut nodes: std::collections::BTreeMap<NodeKey, (DVec3, f32)> = std::collections::BTreeMap::new();
        for level in 1..=3u8 {
            let mut keys: Vec<NodeKey> = objects.iter().map(|(c, _)| NodeKey::containing(*c, level)).collect();
            keys.sort();
            keys.dedup();
            for key in keys {
                // Children's origins: objects for L1, child centers above.
                let origins: Vec<DVec3> = if level == 1 {
                    objects.iter().filter(|(c, _)| key.contains_cell(*c)).map(|(_, p)| *p).collect()
                } else {
                    nodes.iter().filter(|(k, _)| k.level == level - 1 && k.parent() == Some(key)).map(|(_, (c, _))| *c).collect()
                };
                let center = node_center(key, Partition::STREAMING, origins.iter().map(|o| WorldPos::from_f64(*o))).to_f64();
                #[allow(clippy::cast_possible_truncation)]
                let lead = origins.iter().map(|o| o.distance(center) as f32).fold(0.0, f32::max);
                nodes.insert(key, (center, lead));
            }
        }
        Self { levels: FarLevels::default(), objects, nodes }
    }

    fn center(&self, key: NodeKey) -> DVec3 {
        self.nodes[&key].0
    }

    fn lead(&self, key: NodeKey) -> f32 {
        self.nodes[&key].1
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Res {
    /// Rings with a load delay in steps (streaming latency).
    Lagging(u32),
}

/// Walks a camera along `path`; residency follows rings with latency; the
/// no-hole fallback forces parents. Asserts every object is covered at
/// every step.
#[allow(clippy::cast_possible_truncation, clippy::too_many_lines)]
fn walk(path: &[DVec3], res: Res, fallback: bool) -> usize {
    let sim = Sim::new();
    let l = &sim.levels;
    let Res::Lagging(delay) = res;
    // R1: objects; R2: L1 nodes by their L2 bounds (here: L2 center distance
    // minus its half diagonal); L2/L3 always resident.
    let (r1_load, r1_unload) = (f64::from(l.object_end()) + 150.0, f64::from(l.object_end()) + 250.0);
    let (r2_load, r2_unload) = (f64::from(l.handoff(2)) + 600.0, f64::from(l.handoff(2)) + 800.0);
    let n = sim.objects.len();
    let mut l0_age = vec![None::<u32>; n]; // steps since the load started; Some(age)
    let mut l1_age: std::collections::BTreeMap<NodeKey, u32> = std::collections::BTreeMap::new();
    let cells: Vec<NodeKey> = {
        let mut k: Vec<NodeKey> = sim.objects.iter().map(|(c, _)| NodeKey::of_cell(*c)).collect();
        k.sort();
        k.dedup();
        k
    };
    let mut checked = 0usize;
    let mut holes = 0usize;
    for cam in path {
        // Residency update.
        for (i, (_, p)) in sim.objects.iter().enumerate() {
            let d = p.distance(*cam);
            l0_age[i] = match l0_age[i] {
                None if d <= r1_load => Some(0),
                Some(_) if d > r1_unload => None,
                Some(a) => Some(a + 1),
                None => None,
            };
        }
        for k in &cells {
            let parent = k.parent().expect("L2");
            let d = (sim.center(parent).distance(*cam) - 724.0).max(0.0);
            match l1_age.get_mut(k) {
                None if d <= r2_load => {
                    l1_age.insert(*k, 0);
                }
                Some(_) if d > r2_unload => {
                    l1_age.remove(k);
                }
                Some(a) => *a += 1,
                None => {}
            }
        }
        let l0_res = |i: usize| l0_age[i].is_some_and(|a| a >= delay);
        let l1_res = |k: &NodeKey| l1_age.get(k).is_some_and(|a| *a >= delay);
        // Forced flags (the residency pass).
        let forced = |k: NodeKey| -> bool {
            let d = sim.center(k).distance(*cam) as f32;
            let lead = sim.lead(k);
            let children_resident = match k.level {
                1 => sim.objects.iter().enumerate().filter(|(_, (c, _))| k.contains_cell(*c)).all(|(i, _)| l0_res(i)),
                2 => cells.iter().filter(|c| k.contains_cell(c.index())).all(&l1_res),
                _ => true,
            };
            fallback && l.forced(k.level, lead, d, children_resident)
        };
        for (i, (cell, p)) in sim.objects.iter().enumerate() {
            let k1 = NodeKey::of_cell(*cell);
            let k2 = k1.parent().expect("L2");
            let k3 = k2.parent().expect("L3");
            let obj = FadeRange { start: 0.0..0.0, end: l.object_end()..l.object_end() + l.band_width(1) };
            let chain = [
                Drawn { origin: *p, range: obj, resident: l0_res(i) },
                Drawn { origin: sim.center(k1), range: l.node_range(1, sim.lead(k1), forced(k1)), resident: l1_res(&k1) },
                Drawn { origin: sim.center(k2), range: l.node_range(2, sim.lead(k2), forced(k2)), resident: true },
                Drawn { origin: sim.center(k3), range: l.node_range(3, sim.lead(k3), forced(k3)), resident: true },
            ];
            holes += usize::from(!coverage(*cam, &chain));
            checked += 1;
        }
    }
    assert!(checked > 0);
    holes
}

fn line(from: DVec3, to: DVec3, step: f64) -> Vec<DVec3> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (from.distance(to) / step).ceil() as usize;
    #[allow(clippy::cast_precision_loss)]
    (0..=n).map(|i| from.lerp(to, i as f64 / n as f64)).collect()
}

#[test]
fn lead_bands_cover_with_instant_residency() {
    // Fly in from 12 km out at 60 m/s (6 m per 0.1 s step), across, and out.
    let mut path = line(DVec3::new(-12_000.0, 40.0, 1_900.0), DVec3::new(2_000.0, 2.0, 1_950.0), 30.0);
    path.extend(line(DVec3::new(2_000.0, 2.0, 1_950.0), DVec3::new(2_100.0, 300.0, 9_000.0), 30.0));
    assert_eq!(walk(&path, Res::Lagging(0), true), 0);
}

#[test]
fn residency_fallback_covers_while_children_load() {
    // Same path, loads take 40 steps; then a teleport into the middle.
    let mut path = line(DVec3::new(-6_000.0, 40.0, 1_000.0), DVec3::new(2_000.0, 2.0, 2_000.0), 30.0);
    path.extend(std::iter::repeat_n(DVec3::new(500.0, 2.0, 3_500.0), 50));
    assert_eq!(walk(&path, Res::Lagging(40), true), 0);
    // The fallback is what closes them: without it, lagging loads leave holes.
    assert!(walk(&path, Res::Lagging(40), false) > 0);
}

// -- cluster build --------------------------------------------------------------------

/// A finely tessellated sphere (lots to simplify), vertex-colored.
fn sphere_model() -> CookedModel {
    let mut mesh = MeshData::default();
    let (rings, segs) = (24u32, 48u32);
    for r in 0..=rings {
        for s in 0..=segs {
            let (t, p) = (f32::from(u16::try_from(r).expect("small")) / 24.0 * std::f32::consts::PI, f32::from(u16::try_from(s).expect("small")) / 48.0 * std::f32::consts::TAU);
            let n = Vec3::new(t.sin() * p.cos(), t.cos(), t.sin() * p.sin());
            mesh.positions.push((n * 3.0 + Vec3::Y * 3.0).to_array());
            mesh.normals.get_or_insert_with(Vec::new).push(n.to_array());
            mesh.colors.get_or_insert_with(Vec::new).push([0.6, 0.5, 0.4, 1.0]);
        }
    }
    for r in 0..rings {
        for s in 0..segs {
            let a = r * (segs + 1) + s;
            let b = a + segs + 1;
            mesh.indices.extend([a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    let mut node = Node::new("sphere");
    node.mesh = Some(0);
    let model = Model { nodes: vec![node], meshes: vec![mesh], materials: vec![Material::default()] };
    match sf_model::cook(&model, &CookSettings::default()).expect("cooks") {
        Cooked::Static(m) => m,
        Cooked::Skipped { .. } | Cooked::Skinned(_) => panic!("static"),
    }
}

fn placement(i: u32, pos: DVec3, class: PlacementClass) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id: PlacementId::from_key("test", &i.to_le_bytes()),
            base: RecordRef::new("test.object", "sphere"),
            model: Some(VfsPath::new("sphere.mdl").expect("path")),
            transform: WorldTransform { pos: WorldPos::from_f64(pos), rotation: Quat::from_rotation_y(0.3 * f32::from(u16::try_from(i).expect("small"))), scale: Vec3::ONE },
            class,
            flags: PlacementFlags::empty(),
        },
        radius: 4.3,
    }
}

fn cell_blob(cell: IVec3, model: &Arc<CookedModel>, settings: &FarBakeSettings) -> NodeBlob {
    let origin = Partition::STREAMING.cell_origin(cell).to_f64();
    let mut placements: Vec<IndexedPlacement> = (0..12)
        .map(|i| placement(i, origin + DVec3::new(10.0 + f64::from(i) * 9.0, 0.0, 20.0 + f64::from(i % 4) * 25.0), PlacementClass::Static))
        .collect();
    placements.push(placement(99, origin + DVec3::new(60.0, 0.0, 60.0), PlacementClass::Light));
    let summary = ModelSummary::of(model);
    let lookup = |_: &VfsPath| Some(FarModel { policy: FarPolicy::Merge, summary, cooked: Some(model.clone()), albedo: Vec::new() });
    build_l1(Partition::STREAMING, cell, &placements, &lookup, &DefaultFarSource, settings).expect("builds")
}

#[test]
fn proxies_stay_within_budget_and_simplify() {
    let model = Arc::new(sphere_model());
    let settings = FarBakeSettings::default();
    let l = &settings.levels;
    let cells: Vec<IVec3> = (0..8).flat_map(|x| (0..2).map(move |z| IVec3::new(x, 1, z))).collect();
    let mut l1 = Vec::new();
    let lod0 = model.lods[0].triangle_count();
    for &cell in &cells {
        let blob = cell_blob(cell, &model, &settings);
        let h = &blob.header;
        assert!(h.error <= l.error_budget(1, h.lead) + 1e-5, "L1 error {} > budget {}", h.error, l.error_budget(1, h.lead));
        assert!(h.triangles > 0 && h.triangles < lod0 * 12, "simplified: {} vs {}", h.triangles, lod0 * 12);
        assert_eq!(h.lights, 1);
        // Round trip.
        assert_eq!(NodeBlob::from_bytes(&blob.to_bytes()).expect("decodes"), blob);
        l1.push(blob);
    }
    let key = NodeKey::of_cell(cells[0]).parent().expect("L2");
    let l2 = build_parent(key, Partition::STREAMING, l1.clone(), &settings).expect("builds");
    let h = &l2.header;
    assert!(h.error <= l.error_budget(2, h.lead) + 1e-5, "L2 error {} > budget {}", h.error, l.error_budget(2, h.lead));
    let l1_tris: u32 = l1.iter().map(|b| b.header.triangles).sum();
    assert!(h.triangles < l1_tris, "L2 simplifies further: {} vs {l1_tris}", h.triangles);
    // Merged lights: 16 lights 128 m apart don't merge at 12 m.
    assert_eq!(h.lights, 16);
    // Order of children doesn't matter.
    let mut reversed = l1;
    reversed.reverse();
    assert_eq!(build_parent(key, Partition::STREAMING, reversed, &settings).expect("builds").to_bytes(), l2.to_bytes());
}

#[test]
fn octahedral_reexport_round_trips() {
    let d = Vec3::new(0.3, 0.8, -0.5).normalize();
    let back = decode(crate::OctaMode::Full, crate::octa::encode(crate::OctaMode::Full, d));
    assert!(back.distance(d) < 1e-4);
}

#[cfg(feature = "bake")]
mod bake;

#[test]
#[allow(clippy::many_single_char_names)]
fn faceted_sources_simplify_and_keep_hard_edges() {
    use crate::cluster::{ClusterBuilder, SimplifySettings, crease_normals};
    use glam::Affine3A;
    use sf_model::Vertex;
    // The sphere, expanded to one vertex triple per face (flat shading).
    let model = sphere_model();
    let (v, i) = crate::cluster::decode_level(&model, 0).expect("decodes");
    let mut flat = Vec::new();
    for t in i.as_chunks::<3>().0 {
        let [a, b, c] = t.map(|k| Vec3::from_array(v[k as usize].position));
        let n = (b - a).cross(c - a).normalize_or_zero().to_array();
        for k in t {
            flat.push(Vertex { normal: n, ..v[*k as usize] });
        }
    }
    let idx: Vec<u32> = (0..u32::try_from(flat.len()).expect("small")).collect();
    let mut b = ClusterBuilder::new();
    b.add(&flat, &idx, Affine3A::IDENTITY, [1.0; 4], 0.0);
    let out = b.build(0.05, &SimplifySettings::default()).expect("builds").expect("non-empty");
    let input = i.len() / 3;
    assert!((out.triangles as usize) * 3 < input, "faceted input simplified: {} of {input}", out.triangles);
    assert!(out.error <= 0.05 + 1e-6);

    // A cube keeps its hard edges: every normal is an axis.
    let mut cube = Vec::new();
    let mut cube_idx = Vec::new();
    for (n, a, bb) in [(Vec3::X, Vec3::Y, Vec3::Z), (Vec3::Y, Vec3::Z, Vec3::X), (Vec3::Z, Vec3::X, Vec3::Y)] {
        for side in [n, -n] {
            let base = u32::try_from(cube.len()).expect("small");
            for (u, w) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                cube.push(Vertex { position: (side + a * u + bb * w).to_array(), normal: [0.0; 3], uv: [0.0; 2], color: [255; 4] });
            }
            let quad = if side == n { [0, 1, 2, 0, 2, 3] } else { [0, 2, 1, 0, 3, 2] };
            cube_idx.extend(quad.map(|k| base + k));
        }
    }
    let (cv, _) = crease_normals(&cube, &cube_idx, 0.77);
    for vtx in &cv {
        let n = Vec3::from_array(vtx.normal);
        assert!((n.abs().max_element() - 1.0).abs() < 1e-4, "hard edge kept: {n}");
    }
}

/// Precombine (precombine.md): members merge per material and LOD level in
/// cell space; actors, blended models and vetoed placements stay out; size
/// classes split.
#[test]
fn near_precombine_merges_statics_per_level() {
    use crate::near::{BUCKET_EDGES, NearCombine, bucket_of, build_near};
    let model = Arc::new(sphere_model());
    let cell = IVec3::new(2, 1, 3);
    let origin = Partition::STREAMING.cell_origin(cell).to_f64();
    let mut placements: Vec<IndexedPlacement> =
        (0..3).map(|i| placement(i, origin + DVec3::new(20.0 + f64::from(i) * 30.0, 0.0, 40.0), PlacementClass::Static)).collect();
    placements.push(placement(10, origin + DVec3::new(60.0, 0.0, 60.0), PlacementClass::Actor));
    let mut small = placement(11, origin + DVec3::new(70.0, 0.0, 70.0), PlacementClass::Static);
    small.radius = 0.3;
    placements.push(small);
    let vetoed = placements[2].placement.id;
    let lookup = |_: &VfsPath| Some(model.clone());
    let near = build_near(Partition::STREAMING, cell, &placements, &lookup, &|p| p.id == vetoed).expect("builds").expect("some");
    assert_eq!(near.member_count(), 3, "two statics + the small one; actor and vetoed out");
    assert_eq!(near.buckets.len(), 2, "size classes split");
    let big = near.buckets.iter().find(|b| b.members.len() == 2).expect("big bucket");
    assert_eq!(big.model.lods.len(), model.lods.len());
    for (k, lod) in big.model.lods.iter().enumerate() {
        assert_eq!(lod.triangle_count(), 2 * model.lods[k].triangle_count(), "level {k}");
        assert!((lod.error - model.lods[k].error).abs() < 1e-6);
    }
    assert_eq!(big.model.materials.len(), 1, "one shared material");
    // The two spheres (radius 3, 30 m apart) span ~36 m in X around the center.
    let w = big.model.bounds.max[0] - big.model.bounds.min[0];
    assert!((w - 36.0).abs() < 0.5, "width {w}");
    assert!(big.reach > 15.0);
    assert_eq!(NearCombine::from_bytes(&near.to_bytes()).expect("decodes"), near);
    // Blended materials keep a placement out entirely.
    let mut glass = (*model).clone();
    glass.materials[0].alpha = sf_model::AlphaMode::Blend;
    let glass = Arc::new(glass);
    let none = build_near(Partition::STREAMING, cell, &placements[..2], &|_: &VfsPath| Some(glass.clone()), &|_| false).expect("builds");
    assert!(none.is_none());
    assert_eq!((bucket_of(0.1), bucket_of(BUCKET_EDGES[0]), bucket_of(3.0), bucket_of(50.0)), (0, 1, 2, 3));
}
