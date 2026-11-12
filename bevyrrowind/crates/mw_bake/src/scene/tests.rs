use glam::{IVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_model::{Aabb, CookedLod, CookedModel, CookedSubmesh};
use sf_records::RecordId;
use sf_scene::bake::{CellModel, ModelFacts, build_cell_table, material_key, mesh_key};
use sf_scene::{Eligibility, EntityReason, MaterialClass, Sphere, classify};
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

/// [`material`] with a base-colour texture (what the alpha class is keyed by).
fn textured(alpha: AlphaMode, texture: &str, extra: &[(&str, &str)]) -> Material {
    Material { base_color_texture: Some(texture.to_owned()), ..material(alpha, extra) }
}

/// The texture bake's alpha classes: a leaf card (holes), a glass pane (soft alpha).
fn leaf_and_glass() -> AlphaClasses {
    AlphaClasses::new(BTreeMap::from([
        ("textures/leaf.dds".to_owned(), AlphaClass::Cutout),
        ("textures/glass.dds".to_owned(), AlphaClass::Soft),
        ("textures/broken.dds".to_owned(), AlphaClass::Undecodable),
    ]))
}

/// A scene source with the test records and [`leaf_and_glass`].
fn source() -> MwSceneSource {
    MwSceneSource::build(&db(), leaf_and_glass())
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
    let classes = leaf_and_glass();
    let kind = |m: &Material| scene_material(m, &classes).kind;
    assert_eq!(kind(&material(AlphaMode::Opaque, &[])), MaterialKind::Opaque);
    assert_eq!(kind(&material(AlphaMode::Mask(0.5), &[(extras::BLEND, "cutout")])), MaterialKind::AlphaMasked);
    assert_eq!(kind(&material(AlphaMode::Blend, &[])), MaterialKind::Blended);
    assert_eq!(kind(&material(AlphaMode::Blend, &[(extras::BLEND, "additive")])), MaterialKind::Additive);
    assert_eq!(kind(&material(AlphaMode::Opaque, &[(extras::DEPTH_WRITE, "false")])), MaterialKind::Blended);
    assert_eq!(kind(&material(AlphaMode::Mask(0.5), &[(extras::DEPTH_TEST, "false")])), MaterialKind::Blended);
    // A promotion does not override the depth switches: those parts still sort as entities.
    let candidate = &[(extras::BLEND, "cutout_candidate")][..];
    let no_depth = &[(extras::BLEND, "cutout_candidate"), (extras::DEPTH_WRITE, "false")][..];
    assert_eq!(kind(&textured(AlphaMode::Blend, "textures/leaf.dds", no_depth)), MaterialKind::Blended);
    // The texture's alpha class decides, as it does at load (`runtime_alpha`): a cutout is
    // alpha-tested (and so can enter the table), everything else stays the authored blend.
    assert_eq!(kind(&textured(AlphaMode::Blend, "textures/leaf.dds", candidate)), MaterialKind::AlphaMasked);
    assert_eq!(kind(&textured(AlphaMode::Blend, "textures/glass.dds", candidate)), MaterialKind::Blended);
    assert_eq!(kind(&textured(AlphaMode::Blend, "textures/broken.dds", candidate)), MaterialKind::Blended);
    assert_eq!(kind(&textured(AlphaMode::Blend, "textures/unseen.dds", candidate)), MaterialKind::Blended, "no class");
    assert_eq!(kind(&material(AlphaMode::Blend, candidate)), MaterialKind::Blended, "no texture");
    // An authored alpha test (`Cutout::Promoted`) needs no class: it is already alpha-tested.
    assert_eq!(kind(&textured(AlphaMode::Mask(0.5), "textures/glass.dds", &[(extras::BLEND, "cutout")])), MaterialKind::AlphaMasked);
    let leaves = scene_material(&Material { alpha: AlphaMode::Mask(0.5), double_sided: true, ..Material::default() }, &classes);
    assert_eq!(leaves.traits, MaterialTraits { masked: true, two_sided: true, wind: false });
    assert_eq!(MaterialClass::of(leaves.traits), MaterialClass::MaskedTwoSided);
}

#[test]
fn static_scripted_referenced_door_and_marker() {
    use Eligibility::Entity;
    let source = source();
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
fn fingerprint_follows_scripts_rules_and_texture_classes() {
    let parts = |scripted: Vec<String>, tokens: Vec<String>, textures| MwSceneSource::from_parts(scripted, tokens, textures);
    let lever = || vec!["lever".to_owned()];
    let rock = || vec!["named_rock".to_owned()];
    let a = parts(lever(), rock(), leaf_and_glass());
    let b = parts(vec!["LEVER".to_owned()], vec!["Named_Rock".to_owned()], leaf_and_glass());
    let c = parts(lever(), vec!["other".to_owned()], leaf_and_glass());
    let d = parts(Vec::new(), vec!["lever".to_owned(), "named_rock".to_owned()], leaf_and_glass());
    assert_eq!(a.fingerprint(), b.fingerprint());
    assert_ne!(a.fingerprint(), c.fingerprint());
    assert_ne!(a.fingerprint(), d.fingerprint());
    // A texture's class is part of the key: it decides what the material rule answers, so every
    // cell's table re-bakes when one changes, is added, or is dropped (`AlphaClasses`).
    let reclassified = AlphaClasses::new(BTreeMap::from([("textures/leaf.dds".to_owned(), AlphaClass::Soft)]));
    assert_ne!(a.fingerprint(), parts(lever(), rock(), reclassified).fingerprint());
    assert_ne!(a.fingerprint(), parts(lever(), rock(), AlphaClasses::default()).fingerprint());
    let added = AlphaClasses::new(BTreeMap::from([("textures/extra.dds".to_owned(), AlphaClass::Cutout)]));
    assert_ne!(parts(lever(), rock(), AlphaClasses::default()).fingerprint(), parts(lever(), rock(), added).fingerprint());
}

/// A window with an opaque frame and blended glass: the frame goes in the table, the glass
/// stays an entity under the same placement id; a plain rock goes in whole.
#[test]
fn mixed_opaque_and_blended_model_splits() {
    let sub = |material, vertex_count, index_count| CookedSubmesh { material, vertex_count, index_count, vertices: Vec::new(), indices: Vec::new(), tangents: Vec::new() };
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
    let source = source();
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
        facts.push(ModelFacts { cooked: true, animated: false, parts: m.materials.iter().map(|x| scene_material(x, &leaf_and_glass()).kind).collect() });
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

/// A baker with no inputs, and the texture bake's empty alpha-class index registered (so the
/// scene bake gets past [`alpha_classes`]).
fn textureless_baker(store: sf_bake::Store) -> Baker {
    let mut baker = Baker::new(store, Arc::new(NoInputs));
    baker.register(crate::delight::AlphaClassStep);
    baker.register(crate::delight::AlphaIndexStep { textures: Arc::new(BTreeMap::new()) });
    baker
}

#[test]
fn bake_scene_refuses_to_run_before_the_terrain() {
    // Without level-0 terrain tiles every cell would bake with `land_min = -inf` and keep
    // horizon culling off for as long as the cache lives (module docs, "Order matters").
    let dir = tempfile::tempdir().unwrap();
    let mut baker = textureless_baker(sf_bake::Store::open(dir.path()).unwrap());
    let ws = WorldspaceId::new("ws");
    assert!(land_tiles(&baker, &ws).is_empty(), "a fresh baker has no tile products");
    let err = bake_scene(&mut baker, &db(), std::slice::from_ref(&ws)).unwrap_err().to_string();
    assert!(err.contains("no level-0") && err.contains(TILE_STEP), "{err}");
}

/// `bake_chains` reads the cells' products, so called before `bake_scene` it must say so (one
/// failure) rather than report an empty, successful chain bake that would leave the runtime with
/// tables and no geometry.
#[test]
fn bake_chains_before_bake_scene_is_a_failure() {
    let dir = tempfile::tempdir().unwrap();
    let baker = textureless_baker(sf_bake::Store::open(dir.path()).unwrap());
    let ws = WorldspaceId::new("ws");
    let report = bake_chains(&baker, std::slice::from_ref(&ws));
    assert_eq!(report.products.len(), 1, "{:?}", report.products);
    assert_eq!(report.failures().count(), 1);
}

/// Without the texture bake's alpha classes no cutout candidate can be told from a translucent
/// blend, so every leaf card in the world would quietly stay out of the table: the scene bake
/// says so instead of baking that.
#[test]
fn bake_scene_refuses_to_run_before_the_textures() {
    let dir = tempfile::tempdir().unwrap();
    let store = sf_bake::Store::open(dir.path()).unwrap();
    let mut bare = Baker::new(store.clone(), Arc::new(NoInputs));
    let ws = WorldspaceId::new("ws");
    let err = bake_scene(&mut bare, &db(), std::slice::from_ref(&ws)).unwrap_err().to_string();
    assert!(err.contains(ALPHA_INDEX_STEP) && err.contains("bake_delight"), "{err}");
    // With the index baked (here: empty), the scene bake gets to its own ordering rule.
    let mut baker = textureless_baker(store);
    assert_eq!(alpha_classes(&baker).unwrap(), AlphaClasses::default());
    let err = bake_scene(&mut baker, &db(), std::slice::from_ref(&ws)).unwrap_err().to_string();
    assert!(err.contains("no level-0"), "{err}");
}

/// The texture's alpha class decides which cutout candidates reach the table. Four one-part
/// models, all `STAT`: a leaf card (candidate, cutout texture) lands `Masked`; a gauze pane
/// (candidate, soft texture) and a card whose texture the texture bake never classified are kept
/// out, as blends; an opaque rock is untouched. With every candidate promoted — the bake's
/// assumption before the classes existed — the gauze would be in the table drawn alpha-tested.
#[test]
fn the_texture_class_decides_which_candidates_reach_the_table() {
    let sub = |material, vertex_count, index_count| CookedSubmesh { material, vertex_count, index_count, vertices: Vec::new(), indices: Vec::new(), tangents: Vec::new() };
    let one_part = |m: Material| CookedModel {
        bounds: Aabb { min: [-1.0; 3], max: [1.0; 3] },
        has_normals: true,
        has_uvs: true,
        has_colors: false,
        materials: vec![m],
        lods: vec![CookedLod { error: 0.0, submeshes: vec![sub(0, 8, 12)] }],
    };
    let candidate = &[(extras::BLEND, "cutout_candidate")][..];
    // Two-sided authored blends with no alpha test: the Morrowind foliage and gauze idiom.
    let card = |texture: &str| one_part(Material { double_sided: true, ..textured(AlphaMode::Blend, texture, candidate) });
    let leaf = card("textures/leaf.dds");
    let gauze = card("textures/glass.dds");
    let unseen = card("textures/unseen.dds");
    let stone = one_part(material(AlphaMode::Opaque, &[]));
    let source = source();
    let placements = [
        placement("STAT", "window", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
        placement("STAT", "rock", PlacementClass::Static, PlacementFlags::empty()),
    ];
    let unit = [Some(Sphere { center: Vec3::ZERO, radius: 1.0 })];
    let models = [
        CellModel::Static { path: "meshes/x/leaf.nif", model: &leaf, parts: &unit },
        CellModel::Static { path: "meshes/x/gauze.nif", model: &gauze, parts: &unit },
        CellModel::Static { path: "meshes/x/unseen.nif", model: &unseen, parts: &unit },
        CellModel::Static { path: "meshes/x/stone.nif", model: &stone, parts: &unit },
    ];
    let facts: Vec<ModelFacts> = [&leaf, &gauze, &unseen, &stone]
        .into_iter()
        .map(|m| ModelFacts {
            cooked: true,
            animated: false,
            parts: m.materials.iter().map(|x| scene_material(x, &leaf_and_glass()).kind).collect(),
        })
        .collect();
    let eligibility = source.eligibility(CellFacts { placements: &placements, models: &facts });
    assert_eq!(classify(&*eligibility, 0), Eligibility::Scene, "a cutout texture belongs in the table");
    assert_eq!(classify(&*eligibility, 1), Eligibility::Entity(EntityReason::NoScenePart), "soft alpha sorts");
    assert_eq!(classify(&*eligibility, 2), Eligibility::Entity(EntityReason::NoScenePart), "no class: left out");
    assert_eq!(classify(&*eligibility, 3), Eligibility::Scene);
    let table = build_cell_table(&source, &SceneSettings::default(), IVec3::ZERO, f32::NEG_INFINITY, &placements, &models).unwrap();
    let got: Vec<(u32, MaterialClass, bool)> = table
        .instances
        .iter()
        .map(|i| {
            let class = table.chains[i.chain as usize].material_class;
            (i.placement, class, class.traits().masked)
        })
        .collect();
    assert_eq!(got, [(0, MaterialClass::MaskedTwoSided, true), (3, MaterialClass::Opaque, false)]);
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
    // The three locals and the whole match, arm for arm and in order: `runtime_alpha` mirrors it.
    for rule in [
        r#"let additive = m.extras.get(extras::BLEND).is_some_and(|mode| mode == "additive");"#,
        "let cutout = m.extras.get(extras::BLEND).map(String::as_str);",
        r#"let blended = !crate::cutout_foliage() && cutout == Some("cutout");"#,
        r#"let candidate = cutout == Some("cutout_candidate") && crate::texture_is_cutout(vfs, m.base_color_texture.as_deref());"#,
        "SfAlpha::Opaque => AlphaMode::Opaque,
         SfAlpha::Mask(_) if blended => AlphaMode::Blend,
         SfAlpha::Mask(cutoff) => AlphaMode::Mask(cutoff),
         SfAlpha::Blend if additive => AlphaMode::Add,
         SfAlpha::Blend if candidate => AlphaMode::Mask(mw_nif::CUTOUT_CUTOFF),
         SfAlpha::Blend => AlphaMode::Blend,",
    ] {
        assert!(
            cooker.contains(&squash(rule)),
            "mw_assets/src/cooked.rs (the source of truth, `surface`) no longer contains `{rule}`: its alpha \
             promotion changed, so `mw_bake::scene::runtime_alpha` must change with it, or this pin updated."
        );
    }
    // And the loader's texture test is the classifier the bake bakes (`delight::AlphaClass::of`),
    // run on the file under the de-lit layer: the same question on the same bytes.
    let loader = squash(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../mw_assets/src/lib.rs")).unwrap());
    for rule in [
        "vfs.resolve_preferring_below(&p, TEXTURE_ALTERNATES, layer)",
        ".is_some_and(|image| mw_bake::textures::alpha_is_cutout(&image));",
    ] {
        assert!(
            loader.contains(&squash(rule)),
            "mw_assets/src/lib.rs (`texture_is_cutout`) no longer contains `{rule}`: the loader classifies a \
             candidate's texture differently from the bake (mw_bake::delight::AlphaClass::of), so the two no \
             longer agree. Change both, or update this pin."
        );
    }
}
