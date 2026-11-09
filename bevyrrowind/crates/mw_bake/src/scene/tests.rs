use glam::{IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{Aabb, CookedLod, CookedModel, CookedSubmesh};
use sf_records::RecordId;
use sf_scene::bake::{CellModel, ModelFacts, build_cell_table, material_key, mesh_key};
use sf_scene::{Eligibility, EntityReason, InstanceFlags, MaterialClass, Sphere, classify};
use sf_vfs::VfsPath;
use sf_world::{Placement, PlacementId, RecordRef};
use tes3::esp::Static;

use super::*;

fn material(alpha: AlphaMode, extra: &[(&str, &str)]) -> Material {
    let mut m = Material { alpha, ..Material::default() };
    for (k, v) in extra {
        m.extras.insert((*k).to_owned(), (*v).to_owned());
    }
    m
}

/// Records: a static, an activator with a script, a script naming `named_rock`.
fn db() -> RecordDb {
    let mut db = RecordDb::new();
    mw_records::register_kinds(&mut db).unwrap();
    let source = db.add_source("test", &[]);
    for id in ["rock", "named_rock", "window"] {
        db.define(source, RecordId::new(id), Static { id: id.into(), mesh: format!("x\\{id}.nif"), ..Default::default() }).unwrap();
    }
    let lever = Activator { id: "lever".into(), mesh: "x\\lever.nif".into(), script: "lever_script".into(), ..Default::default() };
    db.define(source, RecordId::new("lever"), lever).unwrap();
    let script = Script { id: "quest".into(), text: "if ( GetJournalIndex A1 >= 10 )\n\"Named_Rock\"->Disable\nendif".into(), ..Default::default() };
    db.define(source, RecordId::new("quest"), script).unwrap();
    db
}

fn placement(kind: &str, base: &str, class: PlacementClass, flags: PlacementFlags) -> Placement {
    Placement {
        id: PlacementId(0),
        base: RecordRef::new(kind, base),
        model: Some(VfsPath::new(&format!("meshes/x/{base}.nif")).unwrap()),
        transform: WorldTransform { pos: WorldPos::new(IVec3::ZERO, Vec3::new(1.0, 2.0, 3.0)), rotation: Quat::IDENTITY, scale: Vec3::ONE },
        class,
        flags,
    }
}

fn opaque() -> ModelFacts {
    ModelFacts { cooked: true, animated: false, parts: vec![MaterialKind::Opaque] }
}

#[test]
fn materials_follow_the_nif_blend() {
    let kind = |m: &Material| scene_material(m).kind;
    assert_eq!(kind(&material(AlphaMode::Opaque, &[])), MaterialKind::Opaque);
    assert_eq!(kind(&material(AlphaMode::Mask(0.5), &[(extras::BLEND, "cutout")])), MaterialKind::AlphaMasked);
    assert_eq!(kind(&material(AlphaMode::Blend, &[])), MaterialKind::Blended);
    assert_eq!(kind(&material(AlphaMode::Blend, &[(extras::BLEND, "additive")])), MaterialKind::Additive);
    // A cutout candidate is alpha-tested at runtime, so it is alpha-tested here
    // (`runtime_alpha`; this read `Blended` until 2026-11-09, which kept foliage out of the table).
    assert_eq!(kind(&material(AlphaMode::Blend, &[(extras::BLEND, "cutout_candidate")])), MaterialKind::AlphaMasked);
    assert_eq!(kind(&material(AlphaMode::Opaque, &[(extras::DEPTH_WRITE, "false")])), MaterialKind::Blended);
    // A promotion does not override the depth switches: those parts still sort as entities.
    assert_eq!(
        kind(&material(AlphaMode::Blend, &[(extras::BLEND, "cutout_candidate"), (extras::DEPTH_WRITE, "false")])),
        MaterialKind::Blended
    );
    assert_eq!(kind(&material(AlphaMode::Mask(0.5), &[(extras::DEPTH_TEST, "false")])), MaterialKind::Blended);
    let leaves = scene_material(&Material { alpha: AlphaMode::Mask(0.5), double_sided: true, ..Material::default() });
    assert_eq!(leaves.traits, MaterialTraits { masked: true, two_sided: true, wind: false });
    assert_eq!(MaterialClass::of(leaves.traits), MaterialClass::MaskedTwoSided);
}

#[test]
fn static_scripted_referenced_door_and_marker() {
    use Eligibility::Entity;
    let source = MwSceneSource::build(&db());
    let placements = [
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("ACTI", "Lever", PlacementClass::Custom(2), PlacementFlags::empty()),
        placement("DOOR", "door", PlacementClass::Door, PlacementFlags::empty()),
        placement("STAT", "named_rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "northmarker", PlacementClass::Marker, PlacementFlags::EDITOR_ONLY),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::DISABLED),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("CONT", "chest", mw_world_container(), PlacementFlags::empty()),
    ];
    let mut models = vec![opaque(); placements.len()];
    models[6] = ModelFacts::default(); // `mesh.cook` skipped it (a controller)
    let facts = source.eligibility(CellFacts { placements: &placements, models: &models });
    let got: Vec<Eligibility> = (0..8).map(|p| classify(&*facts, p)).collect();
    assert_eq!(
        got,
        [
            Eligibility::Scene,
            Entity(EntityReason::Class(ClassKind::Activator)),
            Entity(EntityReason::Class(ClassKind::Door)),
            Entity(EntityReason::Referenced),
            Entity(EntityReason::Class(ClassKind::Marker)),
            Entity(EntityReason::Referenced),
            Entity(EntityReason::NotCooked),
            Entity(EntityReason::Class(ClassKind::Container)),
        ]
    );
    // The activator's script is known even though its class already keeps it out.
    assert!(facts.has_script(1));
    assert!(!facts.has_script(0));
}

fn mw_world_container() -> PlacementClass {
    PlacementClass::Custom(1)
}

#[test]
fn fingerprint_follows_scripts_and_rules() {
    let a = MwSceneSource::from_parts(["lever".to_owned()], ["named_rock".to_owned()]);
    let b = MwSceneSource::from_parts(["LEVER".to_owned()], ["Named_Rock".to_owned()]);
    let c = MwSceneSource::from_parts(["lever".to_owned()], ["other".to_owned()]);
    let d = MwSceneSource::from_parts(Vec::new(), ["lever".to_owned(), "named_rock".to_owned()]);
    assert_eq!(a.fingerprint(), b.fingerprint());
    assert_ne!(a.fingerprint(), c.fingerprint());
    assert_ne!(a.fingerprint(), d.fingerprint());
}

/// A window with an opaque frame and blended glass: the frame goes in the table, the glass
/// stays an entity under the same placement id; a plain rock goes in whole.
#[test]
fn mixed_opaque_and_blended_model_splits() {
    let sub = |material, vertex_count, index_count| CookedSubmesh { material, vertex_count, index_count, vertices: Vec::new(), indices: Vec::new() };
    let cooked = |materials, submeshes| CookedModel {
        bounds: Aabb { min: [-1.0; 3], max: [1.0; 3] },
        has_normals: true,
        has_uvs: true,
        has_colors: false,
        materials,
        lods: vec![CookedLod { error: 0.0, submeshes }],
    };
    let window = cooked(vec![material(AlphaMode::Opaque, &[]), material(AlphaMode::Blend, &[])], vec![sub(0, 8, 12), sub(1, 4, 6)]);
    let rock = cooked(vec![material(AlphaMode::Opaque, &[])], vec![sub(0, 3, 3)]);
    let source = MwSceneSource::build(&db());
    let placements = [
        placement("STAT", "window", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
    ];
    // Part spheres by hand (the counts-only submeshes carry no vertices to decode).
    let unit = [Some(Sphere { center: Vec3::ZERO, radius: 1.0 }); 2];
    let models = [
        CellModel::Static { path: "meshes/x/window.nif", model: &window, parts: &unit },
        CellModel::Static { path: "meshes/x/rock.nif", model: &rock, parts: &unit[..1] },
    ];
    let mut facts: Vec<ModelFacts> = Vec::new();
    for m in [&window, &rock] {
        facts.push(ModelFacts { cooked: true, animated: false, parts: m.materials.iter().map(|x| scene_material(x).kind).collect() });
    }
    let split = classify(&*source.eligibility(CellFacts { placements: &placements, models: &facts }), 0);
    assert_eq!(split, Eligibility::Split { scene: vec![0], entity: vec![1] });
    let table = build_cell_table(&source, &SceneSettings::default(), IVec3::ZERO, f32::NEG_INFINITY, &placements, &models).unwrap();
    let got: Vec<(u32, u64)> = table.instances.iter().map(|i| (i.placement, table.chains[i.chain as usize].mesh)).collect();
    assert_eq!(got, [(0, mesh_key("meshes/x/window.nif", 0)), (1, mesh_key("meshes/x/rock.nif", 0))]);
    // The frame and the rock read the same opaque NIF material: one material key, one class.
    let (frame, stone) = (&table.chains[table.instances[0].chain as usize], &table.chains[table.instances[1].chain as usize]);
    assert_eq!(frame.material, stone.material);
    assert_eq!(frame.material, material_key(&material(AlphaMode::Opaque, &[])));
    assert_eq!((frame.material_class, stone.material_class), (MaterialClass::Opaque, MaterialClass::Opaque));
    assert!(table.instances.iter().all(|i| i.material_override == 0));
    assert_eq!(table.layers, SceneSettings::default().layers);
    assert!(table.write().is_ok());
}

/// No files and no records: enough for a [`Baker`] with an empty manifest.
struct NoInputs;

impl sf_bake::Inputs for NoInputs {
    fn file_fingerprint(&self, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        Err(BakeError::MissingInput(sf_bake::Input::File(path.to_owned())))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

#[test]
fn bake_scene_refuses_to_run_before_the_terrain() {
    // Without level-0 terrain tiles every cell would bake with `land_min = -inf` and keep
    // horizon culling off for as long as the cache lives (module docs, "Order matters").
    let dir = tempfile::tempdir().unwrap();
    let store = sf_bake::Store::open(dir.path()).unwrap();
    let mut baker = Baker::new(store, Arc::new(NoInputs));
    let ws = WorldspaceId::new("ws");
    assert!(land_tiles(&baker, &ws).is_empty(), "a fresh baker has no tile products");
    let err = bake_scene(&mut baker, &db(), std::slice::from_ref(&ws)).unwrap_err().to_string();
    assert!(err.contains("no level-0") && err.contains(TILE_STEP), "{err}");
}

/// Alpha-masked foliage: the cooked-model loader promotes an authored blend marked a cutout
/// candidate to an alpha test (`runtime_alpha`), so the bake must put such a part in the table,
/// masked. A genuinely blended part is still kept out, and an opaque one is untouched.
#[test]
fn cutout_candidates_reach_the_table_masked() {
    let sub = |material, vertex_count, index_count| CookedSubmesh { material, vertex_count, index_count, vertices: Vec::new(), indices: Vec::new() };
    let one_part = |m: Material| CookedModel {
        bounds: Aabb { min: [-1.0; 3], max: [1.0; 3] },
        has_normals: true,
        has_uvs: true,
        has_colors: false,
        materials: vec![m],
        lods: vec![CookedLod { error: 0.0, submeshes: vec![sub(0, 8, 12)] }],
    };
    // A leaf card: authored blend, `alpha > 0` test, two-sided (the Morrowind foliage idiom).
    let leaf = one_part(Material {
        double_sided: true,
        ..material(AlphaMode::Blend, &[(extras::BLEND, "cutout_candidate")])
    });
    let glass = one_part(material(AlphaMode::Blend, &[]));
    let stone = one_part(material(AlphaMode::Opaque, &[]));
    let source = MwSceneSource::build(&db());
    let placements = [
        placement("STAT", "window", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
    ];
    let unit = [Some(Sphere { center: Vec3::ZERO, radius: 1.0 })];
    let models = [
        CellModel::Static { path: "meshes/x/leaf.nif", model: &leaf, parts: &unit },
        CellModel::Static { path: "meshes/x/glass.nif", model: &glass, parts: &unit },
        CellModel::Static { path: "meshes/x/stone.nif", model: &stone, parts: &unit },
    ];
    let facts: Vec<ModelFacts> = [&leaf, &glass, &stone]
        .into_iter()
        .map(|m| ModelFacts { cooked: true, animated: false, parts: m.materials.iter().map(|x| scene_material(x).kind).collect() })
        .collect();
    let eligibility = source.eligibility(CellFacts { placements: &placements, models: &facts });
    assert_eq!(classify(&*eligibility, 0), Eligibility::Scene, "a cutout candidate belongs in the table");
    assert_eq!(classify(&*eligibility, 1), Eligibility::Entity(EntityReason::NoScenePart), "a real blend still sorts");
    assert_eq!(classify(&*eligibility, 2), Eligibility::Scene);
    let table = build_cell_table(&source, &SceneSettings::default(), IVec3::ZERO, f32::NEG_INFINITY, &placements, &models).unwrap();
    let got: Vec<(u32, MaterialClass, bool)> = table
        .instances
        .iter()
        .map(|i| (i.placement, table.chains[i.chain as usize].material_class, i.flags.contains(InstanceFlags::ALPHA_MASKED)))
        .collect();
    assert_eq!(got, [(0, MaterialClass::MaskedTwoSided, true), (2, MaterialClass::Opaque, false)]);
    assert!(table.write().is_ok());
}

/// `runtime_alpha` is a copy of the cooker's promotion, because `mw_assets` depends on `mw_bake`
/// and so cannot be a dependency of it. This reads the cooker and fails if its rule moves, which
/// is the only automatic way the two are kept in step.
#[test]
fn cooker_promotion_rule_is_unchanged() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../mw_assets/src/cooked.rs");
    let source = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let squash = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let cooker = squash(&source);
    for rule in [
        r#"let candidate = cutout == Some("cutout_candidate") && crate::texture_is_cutout(vfs, m.base_color_texture.as_deref());"#,
        "SfAlpha::Blend if candidate => AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF),",
        "SfAlpha::Blend => AlphaMode::Blend,",
    ] {
        assert!(
            cooker.contains(&squash(rule)),
            "mw_assets/src/cooked.rs (the source of truth, `surface`) no longer contains `{rule}`: its alpha \
             promotion changed, so `mw_bake::scene::runtime_alpha` must change with it, or this pin updated."
        );
    }
}
