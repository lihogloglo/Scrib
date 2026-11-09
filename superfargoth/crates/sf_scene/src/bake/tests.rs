use glam::{IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{Aabb as ModelAabb, CookSettings, CookedLod, CookedSubmesh, MeshData, Model, Node, cook};
use sf_vfs::VfsPath;
use sf_world::{PlacementId as WorldId, RecordRef};

use super::*;
use crate::eligibility::ClassKind;
use crate::table::{LodLevel, StaticCellTableView};

/// Test rules: the class from `PlacementClass`, a script on base ids starting with `scripted`.
struct Rules;

struct Facts<'a>(CellFacts<'a>);

impl SceneEligibility for Facts<'_> {
    fn class_kind(&self, p: u32) -> ClassKind {
        match self.0.placements[p as usize].class {
            PlacementClass::Static => ClassKind::Static,
            PlacementClass::Door => ClassKind::Door,
            _ => ClassKind::Item,
        }
    }
    fn has_script(&self, p: u32) -> bool {
        self.0.placements[p as usize].base.id.as_str().starts_with("scripted")
    }
    fn is_referenced(&self, _: u32) -> bool {
        false
    }
    fn model_cooked(&self, p: u32) -> bool {
        self.0.models[p as usize].cooked
    }
    fn has_animated_controllers(&self, p: u32) -> bool {
        self.0.models[p as usize].animated
    }
    fn part_materials(&self, p: u32) -> &[MaterialKind] {
        &self.0.models[p as usize].parts
    }
}

impl SceneSource for Rules {
    fn fingerprint(&self) -> [u8; 32] {
        [7; 32]
    }
    fn eligibility<'a>(&'a self, cell: CellFacts<'a>) -> Box<dyn SceneEligibility + 'a> {
        Box::new(Facts(cell))
    }
}

fn material(alpha: AlphaMode, double_sided: bool) -> Material {
    Material { alpha, double_sided, ..Material::default() }
}

/// A submesh with counts only (no vertex data: these tests hand [`build_cell_table`] the part
/// spheres; [`part_spheres`] is tested on a real cook).
fn sub(material: u32, vertex_count: u32, index_count: u32) -> CookedSubmesh {
    CookedSubmesh { material, vertex_count, index_count, vertices: Vec::new(), indices: Vec::new() }
}

fn model(materials: Vec<Material>, lods: Vec<(f32, Vec<CookedSubmesh>)>) -> CookedModel {
    CookedModel {
        bounds: ModelAabb { min: [-1.0, 0.0, -1.0], max: [1.0, 2.0, 1.0] },
        has_normals: true,
        has_uvs: true,
        has_colors: false,
        materials,
        lods: lods.into_iter().map(|(error, submeshes)| CookedLod { error, submeshes }).collect(),
    }
}

fn sphere(center: Vec3, radius: f32) -> Sphere {
    Sphere { center, radius }
}

fn placement(k: u128, base: &str, class: PlacementClass, pos: WorldPos, scale: f32) -> Placement {
    Placement {
        id: WorldId(k),
        base: RecordRef::new("BASE", base),
        model: Some(VfsPath::new("meshes/a.mdl").unwrap()),
        transform: WorldTransform { pos, rotation: Quat::from_rotation_y(0.5), scale: Vec3::splat(scale) },
        class,
        flags: PlacementFlags::empty(),
    }
}

#[test]
fn subjects_round_trip() {
    let ws = WorldspaceId::new("ws");
    let s = cell_subject(&ws, IVec3::new(-3, 0, 12));
    assert_eq!(s, "ws/-3,0,12");
    assert_eq!(parse_cell_subject(&s), Some((ws, IVec3::new(-3, 0, 12))));
    assert_eq!(parse_cell_subject("ws/1,2"), None);
    assert_eq!(parse_cell_subject("ws/1,2,3,4"), None);
    assert_eq!(parse_cell_subject("nope"), None);
}

#[test]
fn mesh_keys_are_stable_and_distinct() {
    let a0 = mesh_key("meshes/a.mdl", 0);
    assert_eq!(a0, mesh_key("meshes/a.mdl", 0));
    assert_ne!(a0, mesh_key("meshes/a.mdl", 1));
    assert_ne!(a0, mesh_key("meshes/b.mdl", 0));
}

#[test]
fn material_keys_follow_contents_not_names() {
    let base = Material { base_color_texture: Some("textures/wall.dds".into()), ..material(AlphaMode::Mask(0.5), false) };
    let key = material_key(&base);
    assert_eq!(key, material_key(&base.clone()));
    assert_eq!(key, material_key(&Material { name: "another model's copy".into(), ..base.clone() }), "the name is a label");
    let negative_zero = Material { emissive: [-0.0, 0.0, 0.0], ..base.clone() };
    assert_eq!(key, material_key(&negative_zero));
    let mut extra = base.clone();
    extra.extras.insert("hint".into(), "x".into());
    let differs = [
        Material { base_color_texture: Some("textures/roof.dds".into()), ..base.clone() },
        Material { base_color_texture: None, ..base.clone() },
        Material { base_color: [1.0, 0.9, 1.0, 1.0], ..base.clone() },
        Material { emissive: [0.1, 0.0, 0.0], ..base.clone() },
        Material { alpha: AlphaMode::Mask(0.6), ..base.clone() },
        Material { alpha: AlphaMode::Opaque, ..base.clone() },
        Material { double_sided: true, ..base.clone() },
        Material { unlit: true, ..base.clone() },
        extra,
    ];
    for (k, m) in differs.iter().enumerate() {
        assert_ne!(material_key(m), key, "variant {k}");
    }
}

/// `SceneSettings { layers: 0 }` asks for an instance table no view can ever draw. It is
/// rejected where the settings enter the bake, so the error names them, and the writer keeps its
/// own check for a table that arrives from disk instead of from these settings.
#[test]
fn layerless_settings_fail_where_they_enter() {
    let settings = SceneSettings { layers: 0, ..SceneSettings::default() };
    assert_eq!(settings.validate(), Err(SettingsError::NoLayers));
    assert_eq!(SceneSettings::default().validate(), Ok(()));
    let err = build_cell_table(&Rules, &settings, IVec3::ZERO, 0.0, &[], &[]).unwrap_err();
    assert_eq!(err, BuildError::Settings(SettingsError::NoLayers));
    assert!(err.to_string().contains("SceneSettings.layers"), "{err}");
    // Defence in depth: a table of valid settings whose mask is cleared afterwards (or read back
    // from disk) is still rejected by the writer.
    let table = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &[], &[]).unwrap();
    assert_eq!(table.layers, 1);
    assert_eq!(StaticCellTable { layers: 0, ..table }.write().unwrap_err(), TableError::NoLayers);
}

#[test]
fn default_material_reading() {
    let m = default_material(&material(AlphaMode::Mask(0.5), true));
    assert_eq!(m, SceneMaterial { kind: MaterialKind::AlphaMasked, traits: MaterialTraits { masked: true, two_sided: true, wind: false } });
    assert_eq!(default_material(&material(AlphaMode::Blend, false)).kind, MaterialKind::Blended);
    assert_eq!(default_material(&material(AlphaMode::Opaque, false)).traits, MaterialTraits::default());
}

/// Two models in one cell: `a` (opaque + masked two-sided, two levels, the masked part dropped
/// at level 1) placed twice; `b` (opaque + blended glass) once, which splits; a door and a
/// scripted static with model `a` stay entities. `b`'s opaque material equals `a`'s but for
/// its name.
#[test]
fn two_model_cell_round_trips() {
    let a = model(
        vec![material(AlphaMode::Opaque, false), material(AlphaMode::Mask(0.5), true)],
        vec![(0.0, vec![sub(0, 100, 300), sub(1, 20, 30)]), (0.05, vec![sub(0, 40, 90)])],
    );
    let b_opaque = Material { name: "frame".into(), ..material(AlphaMode::Opaque, false) };
    let b = model(vec![b_opaque, material(AlphaMode::Blend, false)], vec![(0.0, vec![sub(0, 8, 12), sub(1, 4, 6)])]);
    let a_parts = [Some(sphere(Vec3::new(0.0, 1.0, 0.0), 1.5)), Some(sphere(Vec3::new(0.5, 1.5, 0.0), 0.25))];
    let b_parts = [Some(sphere(Vec3::ZERO, 1.0)); 2];
    let origin_cell = IVec3::new(2, 0, -1);
    let at = |x: f32| WorldPos::new(origin_cell, Vec3::new(x, 1.0, 3.0));
    let placements = vec![
        placement(10, "rock", PlacementClass::Static, at(5.0), 1.0),
        placement(11, "window", PlacementClass::Static, at(-20.0), 1.0),
        placement(12, "door", PlacementClass::Door, at(0.0), 1.0),
        placement(13, "rock", PlacementClass::Static, at(40.0), 2.0),
        placement(14, "scripted_rock", PlacementClass::Static, at(0.0), 1.0),
    ];
    let pa = CellModel::Static { path: "meshes/a.mdl", model: &a, parts: &a_parts };
    let pb = CellModel::Static { path: "meshes/b.mdl", model: &b, parts: &b_parts };
    let models = [pa, pb, CellModel::None, pa, pa];
    let settings = SceneSettings { layers: 0b101, ..SceneSettings::default() };
    let table = build_cell_table(&Rules, &settings, origin_cell, -3.0, &placements, &models).unwrap();

    let (a0, a1, b0) = (mesh_key("meshes/a.mdl", 0), mesh_key("meshes/a.mdl", 1), mesh_key("meshes/b.mdl", 0));
    let chain = |i: &StaticInstance| &table.chains[i.chain as usize];
    let got: Vec<(u32, u64, InstanceFlags)> = table.instances.iter().map(|i| (i.placement, chain(i).mesh, i.flags)).collect();
    let opaque = InstanceFlags::CASTS_SHADOW;
    let masked = InstanceFlags::CASTS_SHADOW | InstanceFlags::ALPHA_MASKED;
    assert_eq!(got, [(0, a0, opaque), (0, a1, masked), (1, b0, opaque), (3, a0, opaque), (3, a1, masked)]);
    // Layers are the table's; no instance overrides its material.
    assert_eq!(table.layers, 0b101);
    assert!(table.instances.iter().all(|i| i.material_override == 0));
    // Three chains, dense, sorted by mesh key, each with the shared layout, its class and its
    // material key; every instance's index is in range and every chain is used.
    assert_eq!(table.chains.len(), 3);
    assert!(table.chains.windows(2).all(|w| w[0].mesh < w[1].mesh));
    let mut used: Vec<u32> = table.instances.iter().map(|i| i.chain).collect();
    used.sort_unstable();
    used.dedup();
    assert_eq!(used, [0, 1, 2]);
    let by_mesh = |mesh: u64| table.chains.iter().find(|c| c.mesh == mesh).unwrap();
    let lod = |first_index, index_count, error| LodLevel { first_index, index_count, error };
    assert_eq!(by_mesh(a0).levels, [lod(0, 300, 0.0), lod(300, 90, 0.05)]);
    assert_eq!(by_mesh(a1).levels, [lod(0, 30, 0.0), lod(0, 30, 0.0)]);
    assert_eq!(by_mesh(b0).levels, [lod(0, 12, 0.0)]);
    assert_eq!(by_mesh(a0).material_class, MaterialClass::Opaque);
    assert_eq!(by_mesh(a1).material_class, MaterialClass::MaskedTwoSided);
    assert_eq!(by_mesh(b0).material_class, MaterialClass::Opaque);
    // Equal materials of two models share a key; different materials don't.
    assert_eq!(by_mesh(a0).material, by_mesh(b0).material);
    assert_eq!(by_mesh(a0).material, material_key(&a.materials[0]));
    assert_ne!(by_mesh(a0).material, by_mesh(a1).material);
    // Transforms are cell-relative; each part has its own sphere, following the scale.
    let rock2 = &table.instances[3];
    assert!(rock2.affine().translation.abs_diff_eq(Vec3::new(40.0, 1.0, 3.0).into(), 1e-6));
    assert!((rock2.sphere.radius - 3.0).abs() < 1e-6);
    assert!(rock2.sphere.center.abs_diff_eq(Vec3::new(40.0, 3.0, 3.0), 1e-5), "{:?}", rock2.sphere.center);
    let rock2_detail = &table.instances[4];
    assert!((rock2_detail.sphere.radius - 0.5).abs() < 1e-6);
    let detail = Vec3::new(40.0, 1.0, 3.0) + Quat::from_rotation_y(0.5) * Vec3::new(1.0, 3.0, 0.0);
    assert!(rock2_detail.sphere.center.abs_diff_eq(detail, 1e-5), "{:?}", rock2_detail.sphere.center);
    assert!(table.aabb.max.x >= 40.0 + rock2.sphere.radius - 1e-5);
    // Bytes round trip, and the reader finds a placement's first part and a chain by key.
    let bytes = table.write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert_eq!(view.to_table(), table);
    assert_eq!(view.find(3), Some(3));
    assert_eq!(view.find_all(0), 0..2);
    assert_eq!(view.find(2), None);
    assert_eq!(view.chain_index(b0), Some(table.instances[2].chain));
    assert_eq!(view.land_min().to_bits(), (-3.0f32).to_bits());
    assert_eq!(view.layers(), 0b101);
    // A static model without bounds for a drawn part is an error, not a guess.
    let unbounded = [CellModel::Static { path: "meshes/a.mdl", model: &a, parts: &a_parts[..1] }];
    assert_eq!(
        build_cell_table(&Rules, &settings, origin_cell, 0.0, &placements[..1], &unbounded).unwrap_err(),
        BuildError::PartBounds { model: "meshes/a.mdl".into(), part: 1 }
    );
}

#[test]
fn chains_name_the_model_and_part_their_mesh_key_came_from() {
    // Format v3: the mesh key is a hash, so the table carries the (model, part) it was made
    // from — the chain's `scene.chains` subject, and the only way the runtime can find its
    // geometry.
    let a = model(
        vec![material(AlphaMode::Opaque, false), material(AlphaMode::Mask(0.5), true)],
        vec![(0.0, vec![sub(0, 100, 300), sub(1, 20, 30)])],
    );
    let b = model(vec![material(AlphaMode::Opaque, false)], vec![(0.0, vec![sub(0, 8, 12)])]);
    let a_parts = [Some(sphere(Vec3::ZERO, 1.0)); 2];
    let b_parts = [Some(sphere(Vec3::ZERO, 1.0))];
    let origin_cell = IVec3::ZERO;
    let at = WorldPos::new(origin_cell, Vec3::ZERO);
    // `b.mdl` is placed first, so the model section is sorted by name, not by first use.
    let placements = vec![
        placement(10, "frame", PlacementClass::Static, at, 1.0),
        placement(11, "rock", PlacementClass::Static, at, 1.0),
    ];
    let models = [
        CellModel::Static { path: "meshes/b.mdl", model: &b, parts: &b_parts },
        CellModel::Static { path: "meshes/a.mdl", model: &a, parts: &a_parts },
    ];
    let table =
        build_cell_table(&Rules, &SceneSettings::default(), origin_cell, 0.0, &placements, &models).unwrap();
    assert_eq!(table.models, ["meshes/a.mdl", "meshes/b.mdl"]);
    let sources: Vec<(&str, u32)> =
        table.chains.iter().map(|c| (table.models[c.model as usize].as_str(), c.part)).collect();
    let mut expected = vec![("meshes/a.mdl", 0), ("meshes/a.mdl", 1), ("meshes/b.mdl", 0)];
    expected.sort_by_key(|&(m, p)| mesh_key(m, p));
    assert_eq!(sources, expected, "chains stay sorted by mesh key, whatever their source");
    // Every chain's mesh key is the key of its own source, and the bake says so before storing.
    for c in &table.chains {
        assert_eq!(mesh_key(&table.models[c.model as usize], c.part), c.mesh);
    }
    table.verify_chain_sources().unwrap();
    // The reader gives the runtime the same pair back, as the chain's product subject.
    let bytes = table.write().unwrap();
    let view = StaticCellTableView::parse(&bytes).unwrap();
    assert_eq!(view.to_table(), table);
    for (index, &(model, part)) in (0u32..).zip(&expected) {
        assert_eq!(view.chain_source(index), Some((model, part)));
        assert_eq!(crate::product::chain_subject(model, part), format!("{model}/{part}"));
    }
    // A table whose key does not match its source is a stale product, and the bake refuses it.
    let mut stale = table.clone();
    stale.chains[0].part += 100;
    assert_eq!(stale.verify_chain_sources(), Err(TableError::MeshKey(0)));
    let mut stale = table;
    stale.chains[0].model = 9;
    assert_eq!(stale.verify_chain_sources(), Err(TableError::MissingModel { chain: 0, model: 9 }));
}

/// A 10 m floor (material 0) and a small sign high in one corner (material 1), cooked for real.
fn floor_and_sign() -> CookedModel {
    let floor = MeshData {
        positions: vec![[-5.0, 0.0, -5.0], [5.0, 0.0, -5.0], [5.0, 0.0, 5.0], [-5.0, 0.0, 5.0]],
        normals: Some(vec![[0.0, 1.0, 0.0]; 4]),
        indices: vec![0, 2, 1, 0, 3, 2],
        material: 0,
        ..MeshData::default()
    };
    let sign = MeshData {
        positions: vec![[4.0, 3.0, 4.0], [4.5, 3.0, 4.0], [4.0, 3.5, 4.0]],
        normals: Some(vec![[0.0, 0.0, 1.0]; 3]),
        indices: vec![0, 1, 2],
        material: 1,
        ..MeshData::default()
    };
    let node = |name: &str, mesh| Node { mesh: Some(mesh), ..Node::new(name) };
    let source = Model {
        nodes: vec![node("floor", 0), node("sign", 1)],
        meshes: vec![floor, sign],
        materials: vec![material(AlphaMode::Opaque, false), material(AlphaMode::Mask(0.5), true)],
    };
    match cook(&source, &CookSettings::default()).unwrap() {
        Cooked::Static(m) => m,
        other => panic!("not a static cook: {other:?}"),
    }
}

#[test]
fn part_spheres_are_each_parts_own_bounds() {
    let m = floor_and_sign();
    let spheres = part_spheres(&m).unwrap();
    assert_eq!(spheres.len(), 2);
    let (floor, sign) = (spheres[0].unwrap(), spheres[1].unwrap());
    assert!(floor.center.abs_diff_eq(Vec3::ZERO, 1e-6), "{floor:?}");
    assert!((floor.radius - 200f32.sqrt() / 2.0).abs() < 1e-5, "{floor:?}");
    assert!(sign.center.abs_diff_eq(Vec3::new(4.25, 3.25, 4.0), 1e-6), "{sign:?}");
    assert!((sign.radius - 0.5f32.sqrt() / 2.0).abs() < 1e-5, "{sign:?}");
    // A material no LOD0 submesh uses has no sphere.
    let mut unused = m.clone();
    unused.materials.push(Material::default());
    assert_eq!(part_spheres(&unused).unwrap()[2], None);
}

#[test]
fn per_part_spheres_are_smaller_than_the_model_sphere() {
    let m = floor_and_sign();
    let parts = part_spheres(&m).unwrap();
    let pos = WorldPos::new(IVec3::ZERO, Vec3::new(10.0, 0.0, -10.0));
    let placements = [placement(1, "ruin", PlacementClass::Static, pos, 2.0)];
    let models = [CellModel::Static { path: "meshes/ruin.mdl", model: &m, parts: &parts }];
    let table = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &placements, &models).unwrap();
    assert_eq!(table.instances.len(), 2);
    let (lo, hi) = (Vec3::from_array(m.bounds.min), Vec3::from_array(m.bounds.max));
    let model_radius = (hi - lo).length() * 0.5 * 2.0;
    let radii: Vec<f32> = table.instances.iter().map(|i| i.sphere.radius).collect();
    assert!(radii.iter().all(|&r| r < model_radius), "{radii:?} vs {model_radius}");
    assert!(radii[1] < model_radius / 10.0, "the sign culls as a sign, not as the building");
    // Every placed vertex of a part lies in its sphere.
    for (instance, lod0) in table.instances.iter().zip(&m.lods[0].submeshes) {
        let affine = instance.affine();
        for v in lod0.decode().unwrap().0 {
            let p = affine.transform_point3(Vec3::from_array(v.position));
            assert!(p.distance(instance.sphere.center) <= instance.sphere.radius + 1e-4);
        }
    }
    assert!(table.write().is_ok());
}

#[test]
fn uncooked_skinned_editor_only_and_all_blended_are_left_out() {
    let glass = model(vec![material(AlphaMode::Blend, false)], vec![(0.0, vec![sub(0, 4, 6)])]);
    let rock = model(vec![material(AlphaMode::Opaque, false)], vec![(0.0, vec![sub(0, 4, 6)])]);
    let one = [Some(sphere(Vec3::ZERO, 1.0))];
    let pos = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let mut hidden = placement(4, "rock", PlacementClass::Static, pos, 1.0);
    hidden.flags = PlacementFlags::EDITOR_ONLY;
    let placements: Vec<Placement> = (0..4).map(|k| placement(k, "rock", PlacementClass::Static, pos, 1.0)).chain([hidden]).collect();
    let models = [
        CellModel::Static { path: "glass", model: &glass, parts: &one },
        CellModel::Skinned,
        CellModel::Skipped,
        CellModel::Static { path: "rock", model: &rock, parts: &one },
        CellModel::Static { path: "rock", model: &rock, parts: &one },
    ];
    let table = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, f32::NEG_INFINITY, &placements, &models).unwrap();
    assert_eq!(table.instances.iter().map(|i| i.placement).collect::<Vec<_>>(), [3]);
    assert!(table.write().is_ok());
    // An empty cell is a valid, empty table.
    let empty = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &[], &[]).unwrap();
    assert!(empty.instances.is_empty() && empty.chains.is_empty());
    assert!(StaticCellTableView::parse(&empty.write().unwrap()).unwrap().is_empty());
}

fn bounds(default: f32, low: &[(usize, usize, f32)]) -> BlockBounds {
    let mut min = vec![default; BLOCKS * BLOCKS];
    for &(bx, bz, h) in low {
        min[bz * BLOCKS + bx] = h;
    }
    BlockBounds { max: vec![100.0; BLOCKS * BLOCKS], min }
}

#[test]
#[allow(clippy::float_cmp, reason = "exact: the same f32 operations")]
fn land_minimum_of_a_streaming_cell() {
    let p = Partition::STREAMING; // 128 m cells: four per 256 m level-0 tile
    let t = bounds(10.0, &[(3, 5, -4.0), (20, 20, -50.0)]);
    let origin = |cell: IVec3| p.cell_origin(cell).cell;
    assert_eq!(land_tiles(p, IVec3::ZERO), [(0, 0)]);
    assert_eq!(land_tiles(p, IVec3::new(2, 0, -1)), [(1, -1)]);
    let min = |cell: IVec3| land_min(p, cell, origin(cell), &[(land_tiles(p, cell)[0], Some(&t))]);
    // Cell (0, 0) is the tile's (-x, -z) quarter: blocks 0..16 on both axes.
    assert_eq!(min(IVec3::ZERO), -4.0 - MAX_QUANT_STEP);
    assert_eq!(min(IVec3::new(1, 0, 1)), -50.0 - MAX_QUANT_STEP);
    assert_eq!(min(IVec3::new(0, 0, 1)), 10.0 - MAX_QUANT_STEP);
    // Cell-relative: a cell in the grid cell above sees the land 256 m lower.
    assert_eq!(min(IVec3::new(0, 2, 0)), -260.0 - MAX_QUANT_STEP);
    // No terrain tile, or no grid: no land.
    assert_eq!(land_min(p, IVec3::ZERO, IVec3::ZERO, &[((0, 0), None)]), f32::NEG_INFINITY);
    assert!(land_tiles(Partition::Single, IVec3::ZERO).is_empty());
    assert_eq!(land_min(Partition::Single, IVec3::ZERO, IVec3::ZERO, &[]), f32::NEG_INFINITY);
    // A 512 m cell spans four tiles.
    assert_eq!(land_tiles(Partition::Grid { shift: 1 }, IVec3::ZERO), [(0, 0), (1, 0), (0, 1), (1, 1)]);
}

#[test]
fn per_placement_switches_bar_a_placement_whatever_the_adapter_says() {
    // `Rules` answers only the record questions (class, script, reference): `NO_LOD`,
    // `NEAR_ONLY` and `DISABLED` are `sf_world` data, so the framework applies them itself.
    assert_eq!(
        BARRED,
        PlacementFlags::EDITOR_ONLY | PlacementFlags::NO_LOD | PlacementFlags::NEAR_ONLY | PlacementFlags::DISABLED
    );
    let pos = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let plain = placement(1, "rock", PlacementClass::Static, pos, 1.0);
    let facts = Facts(CellFacts { placements: std::slice::from_ref(&plain), models: &[ModelFacts { cooked: true, animated: false, parts: vec![MaterialKind::Opaque] }] });
    assert_eq!(classify_placement(&facts, 0, &plain), Eligibility::Scene);
    for flag in [
        PlacementFlags::EDITOR_ONLY,
        PlacementFlags::NO_LOD,
        PlacementFlags::NEAR_ONLY,
        PlacementFlags::DISABLED,
    ] {
        let barred = Placement { flags: flag, ..plain.clone() };
        assert_eq!(classify_placement(&facts, 0, &barred), Eligibility::Entity(EntityReason::Barred), "{flag:?}");
    }
    // A switch that says nothing about the scene does not bar it.
    let grounded = Placement { flags: PlacementFlags::GROUNDED | PlacementFlags::NO_SIZE_CULL, ..plain.clone() };
    assert_eq!(classify_placement(&facts, 0, &grounded), Eligibility::Scene);
    // And the bake leaves every barred placement out of the table.
    let rock = model(vec![material(AlphaMode::Opaque, false)], vec![(0.0, vec![sub(0, 4, 6)])]);
    let one = [Some(sphere(Vec3::ZERO, 1.0))];
    let placements: Vec<Placement> = [PlacementFlags::NO_LOD, PlacementFlags::NEAR_ONLY, PlacementFlags::DISABLED, PlacementFlags::empty()]
        .into_iter()
        .zip(0u128..)
        .map(|(flags, k)| Placement { flags, ..placement(k, "rock", PlacementClass::Static, pos, 1.0) })
        .collect();
    let models: Vec<CellModel<'_>> =
        placements.iter().map(|_| CellModel::Static { path: "rock", model: &rock, parts: &one }).collect();
    let table = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &placements, &models).unwrap();
    assert_eq!(table.instances.iter().map(|i| i.placement).collect::<Vec<_>>(), [3]);
}

#[test]
fn a_worldspace_with_no_declared_land_is_a_wiring_error() {
    // A cell baked before the terrain would validate, cache, and keep horizon culling off for
    // as long as the cache lives: it must fail in `plan`, not bake with `land_min = -inf`.
    let undeclared = CellsStep(Arc::new(SceneBake::new(Arc::new(Rules), SceneSettings::default())));
    let err = undeclared.plan("ws/0,0,0", &mut PlanCx::default()).unwrap_err().to_string();
    assert!(err.contains("no land coverage declared"), "{err}");
    // Declaring the coverage plans, and so does declaring an empty one (a landless worldspace:
    // the caller said so, instead of the bake guessing).
    let ws = WorldspaceId::new("ws");
    let with_land = SceneBake::new(Arc::new(Rules), SceneSettings::default()).with_land_tiles(ws.clone(), [(0, 0), (1, 0)]);
    CellsStep(Arc::new(with_land)).plan("ws/0,0,0", &mut PlanCx::default()).unwrap();
    let landless = SceneBake::new(Arc::new(Rules), SceneSettings::default()).with_land_tiles(ws, std::iter::empty());
    let step = CellsStep(Arc::new(landless));
    step.plan("ws/0,0,0", &mut PlanCx::default()).unwrap();
    // Another worldspace is still undeclared.
    assert!(step.plan("other/0,0,0", &mut PlanCx::default()).is_err());
    // The land set is part of the key, so a different coverage is a different product.
    let digest = |tiles: &[(i32, i32)]| {
        SceneBake::new(Arc::new(Rules), SceneSettings::default())
            .with_land_tiles(WorldspaceId::new("ws"), tiles.to_vec())
            .coverage(&WorldspaceId::new("ws"))
            .unwrap()
            .digest
    };
    assert_ne!(digest(&[(0, 0)]), digest(&[(0, 0), (1, 0)]));
    assert_eq!(digest(&[(0, 0), (1, 0)]), digest(&[(1, 0), (0, 0)]), "order must not matter");
}

#[test]
fn tables_are_stored_uncompressed() {
    // The runtime mmap-reads a table and uploads its instance section straight to the GPU
    // (static-scene.md §2): an LZ4'd product would have to be unpacked into a heap buffer.
    let step = CellsStep(Arc::new(SceneBake::new(Arc::new(Rules), SceneSettings::default())));
    assert!(!step.compress());
    assert_eq!(step.id(), CELLS_STEP);
    assert_eq!(step.version(), 3, "format v3: the chain source and the model section");
    assert_eq!(EXTENSION, "sfscene");
}

#[test]
fn chain_subject_round_trips() {
    let s = chain_subject("meshes/x/flora_tree_01.nif", 3);
    assert_eq!(s, "meshes/x/flora_tree_01.nif/3");
    // The split is at the LAST '/', so a model path with slashes round-trips.
    assert_eq!(parse_chain_subject(&s), Some(("meshes/x/flora_tree_01.nif", 3)));
    assert_eq!(parse_chain_subject("m/0"), Some(("m", 0)));
    assert_eq!(parse_chain_subject("m/-1"), None);
    assert_eq!(parse_chain_subject("m/"), None);
    assert_eq!(parse_chain_subject("nope"), None);
    assert_eq!(CHAINS_STEP, "scene.chains");
    assert_eq!(crate::product::CHAINS_EXT, "sfmesh");
}

/// One triangle of `material`, offset along X, with real meshopt-encoded buffers.
fn tri(material: u32, x: f32) -> CookedSubmesh {
    let v = |p: [f32; 3]| Vertex { position: p, ..Vertex::default() };
    let vertices = [v([x, 0.0, 0.0]), v([x + 1.0, 0.0, 0.0]), v([x, 1.0, 0.0])];
    CookedSubmesh::encode(material, &vertices, &[0, 1, 2]).unwrap()
}

#[test]
fn chain_product_levels_match_the_table() {
    // A real cook, so the submeshes carry real meshopt buffers.
    let m = floor_and_sign();
    let parts = part_spheres(&m).unwrap();
    let at = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let placements = vec![placement(10, "frame", PlacementClass::Static, at, 1.0)];
    let models = [CellModel::Static { path: "meshes/a.mdl", model: &m, parts: &parts }];
    let table = build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &placements, &models).unwrap();
    for chain in &table.chains {
        let product = build_chain_mesh(&m, chain.part).unwrap();
        // One material (the chain's), and one LOD level per level of the table's chain.
        assert_eq!(product.materials, vec![m.materials[chain.part as usize].clone()]);
        assert_eq!(product.lods.len(), chain.levels.len());
        for (level, (lod, range)) in product.lods.iter().zip(&chain.levels).enumerate() {
            // One submesh, of the product's own single material, as long as the table's range.
            assert_eq!(lod.submeshes.len(), 1, "level {level}");
            assert_eq!(lod.submeshes[0].material, 0, "level {level}");
            assert_eq!(lod.submeshes[0].index_count, range.index_count, "level {level}");
            assert_eq!(lod.error.to_bits(), range.error.to_bits(), "level {level}");
            // And it decodes: every index addresses the level's own vertices.
            let (vertices, indices) = lod.submeshes[0].decode().unwrap();
            assert_eq!(indices.len(), range.index_count as usize);
            assert!(indices.iter().all(|&i| (i as usize) < vertices.len()));
        }
    }
    // A part the model does not have is an error, not a panic.
    assert!(matches!(build_chain_mesh(&m, 7), Err(ChainMeshError::NoPart(7))));
    // The step's identity and its key inputs.
    let step = ChainsStep;
    assert_eq!((step.id(), step.version()), (CHAINS_STEP, 1));
    step.plan("meshes/a.mdl/0", &mut PlanCx::default()).unwrap();
    assert!(step.plan("meshes/a.mdl", &mut PlanCx::default()).is_err(), "a subject with no part");
}

#[test]
fn a_chain_level_concatenates_rebases_and_repeats() {
    // Material 0 has two submeshes at level 0 and one at level 1; material 1 only at level 0.
    let m = model(
        vec![material(AlphaMode::Opaque, false), material(AlphaMode::Mask(0.5), true)],
        vec![(0.0, vec![tri(0, 0.0), tri(1, 5.0), tri(0, 10.0)]), (0.25, vec![tri(0, 0.0)])],
    );
    // Part 0: the level's submeshes are concatenated in submesh order and rebased *within the
    // level* (the level is its own mesh allocation, so the base starts at 0 again).
    let product = build_chain_mesh(&m, 0).unwrap();
    assert_eq!(product.lods.len(), 2);
    let (vertices, indices) = product.lods[0].submeshes[0].decode().unwrap();
    assert_eq!(vertices.len(), 6);
    assert_eq!(indices, [0, 1, 2, 3, 4, 5]);
    let third = Vec3::from_array(vertices[3].position);
    assert!(third.abs_diff_eq(Vec3::new(10.0, 0.0, 0.0), 1e-6), "the second submesh follows the first: {third:?}");
    assert_eq!(product.lods[0].error.to_bits(), 0.0f32.to_bits());
    let (vertices, indices) = product.lods[1].submeshes[0].decode().unwrap();
    assert_eq!((vertices.len(), indices.len()), (3, 3));
    assert_eq!(product.lods[1].error.to_bits(), 0.25f32.to_bits());
    // Part 1 has no geometry at level 1, so that level repeats the next finer one — byte for
    // byte, and with its error, exactly as the table's level range for it does.
    let product = build_chain_mesh(&m, 1).unwrap();
    let levels = chain_layout(&[
        LevelSource { error: 0.0, submeshes: vec![(3, 3)] },
        LevelSource { error: 0.25, submeshes: Vec::new() },
    ])
    .unwrap()
    .levels;
    assert_eq!(levels[1], levels[0], "the layout rule repeats the finer level");
    assert_eq!(product.lods.len(), 2);
    assert_eq!(product.lods[1].submeshes, product.lods[0].submeshes);
    assert_eq!(product.lods[1].error.to_bits(), levels[1].error.to_bits());
    // The product is an `.sfmesh` and reads back as one.
    let bytes = Cooked::Static(product.clone()).to_bytes();
    assert_eq!(Cooked::from_bytes(&bytes).unwrap(), Cooked::Static(product));
}

#[test]
fn chain_keys_deduplicates_across_cells() {
    // Two cells sharing one model: `chain_keys` reads the baked tables' model sections, so the
    // shared chains are named once. Nothing of the adapter or of any game enters the subject.
    let a = model(
        vec![material(AlphaMode::Opaque, false), material(AlphaMode::Mask(0.5), true)],
        vec![(0.0, vec![sub(0, 100, 300), sub(1, 20, 30)])],
    );
    let b = model(vec![material(AlphaMode::Opaque, false)], vec![(0.0, vec![sub(0, 8, 12)])]);
    let a_parts = [Some(sphere(Vec3::ZERO, 1.0)); 2];
    let b_parts = [Some(sphere(Vec3::ZERO, 1.0))];
    let at = WorldPos::new(IVec3::ZERO, Vec3::ZERO);
    let subjects = |models: &[CellModel<'_>]| {
        let placements: Vec<Placement> = (0..models.len())
            .map(|k| placement(10 + k as u128, "base", PlacementClass::Static, at, 1.0))
            .collect();
        let table =
            build_cell_table(&Rules, &SceneSettings::default(), IVec3::ZERO, 0.0, &placements, models).unwrap();
        let bytes = table.write().unwrap();
        let view = StaticCellTableView::parse(&bytes).unwrap();
        chain_subjects(&view)
    };
    let first = subjects(&[CellModel::Static { path: "meshes/a.mdl", model: &a, parts: &a_parts }]);
    let second = subjects(&[
        CellModel::Static { path: "meshes/a.mdl", model: &a, parts: &a_parts },
        CellModel::Static { path: "meshes/b.mdl", model: &b, parts: &b_parts },
    ]);
    // Chain order is mesh-key order, so sort to compare the sets.
    let sorted = |mut v: Vec<String>| {
        v.sort();
        v
    };
    assert_eq!(sorted(first.clone()), ["meshes/a.mdl/0", "meshes/a.mdl/1"]);
    assert_eq!(sorted(second.clone()), ["meshes/a.mdl/0", "meshes/a.mdl/1", "meshes/b.mdl/0"]);
    // What `chain_keys` does with them: one key per distinct subject over every cell.
    let keys: BTreeSet<String> = first.into_iter().chain(second).collect();
    assert_eq!(keys.len(), 3, "the two cells' shared chains are baked once");
    for subject in &keys {
        let (model, part) = parse_chain_subject(subject).unwrap();
        assert_eq!(chain_subject(model, part), *subject);
    }
}
