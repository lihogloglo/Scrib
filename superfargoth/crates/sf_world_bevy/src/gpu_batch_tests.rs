//! Sanity checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for the
//! vendored port of bevyengine/bevy#25978 (`GpuBatchedMesh3d`, static-scene.md S3a), which S3b
//! feeds from the baked scene tables.
//!
//! - The port's WGSL (`gpu_instance_batch/{materialize,gpu_instance}.wesl`,
//!   `render/unpack_ranges.wesl`, the changed `render/mesh_preprocess.wesl` and
//!   `occlusion_culling/mesh_preprocess_types.wesl` with the passes that share it) links with
//!   `wesl` as Bevy's shader cache does and validates with naga, for every def set the 0.20
//!   preprocessing pipelines specialise (resolver as `sf_sky_bevy`'s `shader_tests.rs`).
//! - The Rust and WGSL layouts of `GpuMeshInstance` and `MeshCullingData` (which gained
//!   `is_active`) agree.
//! - An app with `GpuInstanceBatchPlugin` builds and updates headless (no render app), the
//!   batch joins the `Mesh3d` visibility class and the lights' shadow lists: the port's one
//!   deviation from upstream (no `Mesh3dVisibility` class, so no vendored `bevy_mesh`,
//!   `bevy_camera`, `bevy_light`).
//! - A binned phase bins an instance batch and the patch GS counters read it (static-scene.md
//!   S3a2). Only the zero side is checkable here: `instance_batch_draws` is filled by
//!   `batch_and_prepare_binned_render_phase`, which needs a `RenderDevice`.
//! - Patch GS geometry ranges (`gs-patch-plan.md` step 2, tests T2-T5): the Rust and WGSL
//!   layouts of `GpuBatchGeometryRange` agree down to the two `vec3` offsets, the `MeshInput`
//!   fields `materialize` overwrites per instance still exist under those names, `validate_ranges`
//!   accepts exactly the slot partitions, and `range_of` returns the containing range for every
//!   slot of a partition. The WGSL `range_of` is a hand-kept mirror of the Rust one: only its
//!   compilation is machine-checked here (no GPU, so nothing executes it), the equivalence is
//!   reviewed by eye.

use std::any::TypeId;
use std::borrow::Cow;
use std::path::PathBuf;

use bevy::app::App;
use bevy::asset::{AssetApp, AssetPlugin, Assets};
use bevy::camera::visibility::{
    CascadesVisibleEntities, InheritedVisibility, SetViewVisibility, ViewVisibility, VisibilityClass,
    VisibleMeshEntities,
};
use bevy::ecs::entity::{Entity, EntityHashMap};
use bevy::light::{DirectionalLight, NotShadowCaster};
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::gpu_instance_batch::{
    range_of, validate_ranges, GpuBatchGeometryRange, GpuBatchedMesh3d, GpuBatchedMeshRange,
    GpuInstanceBatchPlugin, GpuMeshInstance, RangeError,
};
use bevy::pbr::MeshCullingData;
use bevy::render::render_resource::ShaderType;
use bevy::shader::Shader;
use bevy::MinimalPlugins;
use wesl::syntax::{ModulePath, PathOrigin};

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (only presence and type matter).
const CONSTANTS: &str = "\
const MATERIAL_BIND_GROUP = 3;
const MAX_CASCADES_PER_LIGHT = 4;
const MAX_DIRECTIONAL_LIGHTS = 10;
const MAX_RECT_LIGHTS = 16;
const PER_OBJECT_BUFFER_BATCH_SIZE = 64;
const SAMPLES_PER_SLICE_SIDE = 3;
const SLICE_COUNT = 3;
const SORTED_FRAGMENT_MAX_COUNT = 8;
const SCREEN_SPACE_SPECULAR_TRANSMISSION_BLUR_TAPS = 8;
const TONEMAPPING_LUT_TEXTURE_BINDING_INDEX = 20;
const TONEMAPPING_LUT_SAMPLER_BINDING_INDEX = 21;
";

fn vendor(package: &str) -> PathBuf {
    PathBuf::from(CRATE).join("../../vendor").join(package).join("src")
}

/// Module paths to files: the vendored Bevy crates.
struct Files;

impl wesl::Resolver for Files {
    fn resolve_source<'a>(&'a self, path: &ModulePath) -> Result<Cow<'a, str>, wesl::error::ResolveError> {
        let missing = |why: &str| wesl::error::ResolveError::ModuleNotFound(path.clone(), why.to_string());
        let PathOrigin::Package(package) = &path.origin else {
            return Err(missing("not a package path"));
        };
        // Imports from inside a package arrive as `importer/package` (bevy_shader canonical_path).
        let package = package.rsplit('/').next().unwrap_or(package);
        if package == "constants" && path.components.is_empty() {
            return Ok(Cow::Borrowed(CONSTANTS));
        }
        if !matches!(package, "bevy_pbr" | "bevy_render" | "bevy_core_pipeline") {
            return Err(missing("unknown package"));
        }
        let mut file = vendor(package);
        for c in &path.components {
            file.push(c);
        }
        for ext in ["wesl", "wgsl"] {
            if let Ok(src) = std::fs::read_to_string(file.with_extension(ext)) {
                return Ok(Cow::Owned(src));
            }
        }
        Err(missing(&format!("no file {}", file.display())))
    }

    /// As `bevy_shader`'s resolver: `importer/package` is `package` (one module, one type).
    fn canonical_path(&self, path: &ModulePath) -> ModulePath {
        match &path.origin {
            PathOrigin::Package(p) if p.contains('/') => ModulePath {
                origin: PathOrigin::Package(p.rsplit('/').next().unwrap_or(p).to_string()),
                components: path.components.clone(),
            },
            _ => path.clone(),
        }
    }
}

/// Links `module` with `defs` on and validates it; the error names the defs.
fn compile(module: &str, defs: &[&str]) -> Result<naga::Module, String> {
    let mut parts = module.split("::");
    let path = ModulePath {
        origin: PathOrigin::Package(parts.next().unwrap_or_default().to_string()),
        components: parts.map(str::to_string).collect(),
    };
    let mut options = wesl::CompileOptions { imports: true, condcomp: true, visibility: false, strip: true, ..Default::default() };
    for d in defs {
        options.features.flags.insert((*d).to_string(), true.into());
    }
    let wgsl = wesl::compile(&path, &options, &Files)
        .map_err(|e| format!("{module} {defs:?}: link: {}", e.diagnostic().render_plain()))?
        .to_string();
    let parsed = naga::front::wgsl::parse_str(&wgsl).map_err(|e| format!("{module} {defs:?}: parse: {}", e.emit_to_string(&wgsl)))?;
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&parsed)
        .map_err(|e| format!("{module} {defs:?}: validate: {}", e.emit_to_string(&wgsl)))?;
    Ok(parsed)
}

/// The def sets `PreprocessPipeline::specialize` (`gpu_preprocess.rs`) builds: direct, frustum
/// culled, and the two occlusion-culling phases.
const PREPROCESS: [&[&str]; 4] = [
    &["WRITE_INDIRECT_PARAMETERS_METADATA"],
    &["WRITE_INDIRECT_PARAMETERS_METADATA", "INDIRECT", "FRUSTUM_CULLING"],
    &["WRITE_INDIRECT_PARAMETERS_METADATA", "INDIRECT", "FRUSTUM_CULLING", "OCCLUSION_CULLING", "EARLY_PHASE"],
    &["WRITE_INDIRECT_PARAMETERS_METADATA", "INDIRECT", "FRUSTUM_CULLING", "OCCLUSION_CULLING", "LATE_PHASE"],
];

/// Some of `BuildIndirectParametersPipeline::specialize`'s def sets (indexed or not, multi-draw
/// count, each culling phase): the pass reads `mesh_preprocess_types`.
const BUILD_INDIRECT: [&[&str]; 5] = [
    &[],
    &["INDEXED", "MULTI_DRAW_INDIRECT_COUNT_SUPPORTED", "MAIN_PHASE"],
    &["INDEXED", "MULTI_DRAW_INDIRECT_COUNT_SUPPORTED", "OCCLUSION_CULLING", "EARLY_PHASE"],
    &["INDEXED", "MULTI_DRAW_INDIRECT_COUNT_SUPPORTED", "OCCLUSION_CULLING", "LATE_PHASE"],
    &["MULTI_DRAW_INDIRECT_COUNT_SUPPORTED", "OCCLUSION_CULLING", "MAIN_PHASE"],
];

#[test]
fn port_shaders_link_and_validate() {
    let mut jobs: Vec<(&str, &[&str])> = vec![
        ("bevy_pbr::gpu_instance_batch::materialize", &[]),
        ("bevy_pbr::render::unpack_ranges", &[]),
        ("bevy_pbr::render::unpack_bins", &[]),
        ("bevy_pbr::render::allocate_uniforms", &[]),
        ("bevy_pbr::render::reset_indirect_batch_sets", &[]),
    ];
    jobs.extend(PREPROCESS.iter().map(|defs| ("bevy_pbr::render::mesh_preprocess", *defs)));
    jobs.extend(BUILD_INDIRECT.iter().map(|defs| ("bevy_pbr::render::build_indirect_params", *defs)));
    let failures: Vec<String> = jobs.into_iter().filter_map(|(module, defs)| compile(module, defs).err()).collect();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The WGSL struct `name`. Imported structs keep their name as a suffix of the module-mangled
/// one.
fn wgsl_struct(module: &naga::Module, name: &str) -> naga::Handle<naga::Type> {
    let (handle, _) = module.types.iter().find(|(_, t)| t.name.as_deref().is_some_and(|n| n == name || n.ends_with(&format!("_{name}")) || n.ends_with(name))).unwrap_or_else(|| panic!("no struct {name}"));
    handle
}

/// Size of the WGSL struct `name` as naga lays it out (storage/uniform buffer rules).
fn wgsl_size(module: &naga::Module, name: &str) -> u32 {
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    layouter[wgsl_struct(module, name)].size
}

/// The members of the WGSL struct `name`, in order, as `(field, byte offset)`.
fn wgsl_members(module: &naga::Module, name: &str) -> Vec<(String, u32)> {
    match &module.types[wgsl_struct(module, name)].inner {
        naga::TypeInner::Struct { members, .. } => members
            .iter()
            .map(|m| (m.name.clone().unwrap_or_default(), m.offset))
            .collect(),
        other => panic!("{name} is not a struct: {other:?}"),
    }
}

/// The byte offset of `field` in the WGSL struct `name`.
fn wgsl_offset(module: &naga::Module, name: &str, field: &str) -> u32 {
    wgsl_members(module, name)
        .into_iter()
        .find(|(member, _)| member == field)
        .unwrap_or_else(|| panic!("no field {name}.{field}"))
        .1
}

#[test]
fn instance_and_culling_layouts_agree() {
    // `materialize` binds both structs as storage arrays, so naga keeps them.
    let module = compile("bevy_pbr::gpu_instance_batch::materialize", &[]).expect("materialize links");
    assert_eq!(u64::from(wgsl_size(&module, "GpuMeshInstance")), GpuMeshInstance::min_size().get());
    assert_eq!(u64::from(wgsl_size(&module, "MeshCullingData")), MeshCullingData::min_size().get());
    assert_eq!(size_of::<GpuMeshInstance>() as u64, GpuMeshInstance::min_size().get(), "GpuMeshInstance is Pod-uploadable");
    assert_eq!(size_of::<MeshCullingData>() as u64, MeshCullingData::min_size().get(), "MeshCullingData is Pod-uploadable");
    // Ordinary meshes stay drawn; zeroed slots are inactive (the PR's contract).
    assert_eq!(MeshCullingData::default().is_active, 1);
    assert_eq!(GpuMeshInstance::default().is_active, 0);
}

/// `gs-patch-plan.md` step 2d, test T2: the per-batch geometry range record is one layout in
/// both languages. 64 bytes, 16-aligned, with the two `vec3<f32>`s at 32 and 48 - chosen so
/// WGSL's align-16 rule for `vec3` inserts no implicit padding that `#[repr(C)]` would not.
/// A field reordered on either side moves an offset and fails here instead of drawing garbage.
#[test]
fn geometry_range_layout_agrees() {
    let module = compile("bevy_pbr::gpu_instance_batch::materialize", &[]).expect("materialize links");
    assert_eq!(u64::from(wgsl_size(&module, "GeometryRange")), GpuBatchGeometryRange::min_size().get());
    assert_eq!(size_of::<GpuBatchGeometryRange>() as u64, GpuBatchGeometryRange::min_size().get(), "GpuBatchGeometryRange is Pod-uploadable");
    assert_eq!(size_of::<GpuBatchGeometryRange>(), 64);
    for (field, offset) in [("first_slot", 0), ("index_count", 12), ("flags", 24), ("aabb_center", 32), ("pad1", 44), ("aabb_half_extents", 48), ("pad2", 60)] {
        assert_eq!(wgsl_offset(&module, "GeometryRange", field), offset, "WGSL GeometryRange.{field}");
    }
    assert_eq!(std::mem::offset_of!(GpuBatchGeometryRange, aabb_center), 32);
    assert_eq!(std::mem::offset_of!(GpuBatchGeometryRange, aabb_half_extents), 48);
    // The whole record is zeroable: a batch without ranges binds one zeroed record it never
    // reads (`range_count == 0`), because a storage binding cannot be empty.
    assert_eq!(GpuBatchGeometryRange::default().index_count, 0);
}

/// `gs-patch-plan.md` test T3: `materialize` overwrites these `MeshInput` fields per instance
/// by name. An upstream rename must fail a test rather than a frame.
#[test]
fn materialize_overwrites_named_mesh_input_fields() {
    let module = compile("bevy_pbr::gpu_instance_batch::materialize", &[]).expect("materialize links");
    let members: Vec<String> = wgsl_members(&module, "MeshInput").into_iter().map(|(name, _)| name).collect();
    for field in ["world_from_local", "flags", "tag", "first_vertex_index", "first_index_index", "index_count", "metadata_index", "material_and_lightmap_bind_group_slot"] {
        assert!(members.iter().any(|m| m == field), "MeshInput.{field} is gone; materialize writes it ({members:?})");
    }
    let culling: Vec<String> = wgsl_members(&module, "MeshCullingData").into_iter().map(|(name, _)| name).collect();
    for field in ["aabb_center", "aabb_half_extents", "is_active"] {
        assert!(culling.iter().any(|m| m == field), "MeshCullingData.{field} is gone ({culling:?})");
    }
}

/// A range covering `first_slot..first_slot + slot_count` of a batch, drawing its whole mesh.
fn mesh_range(first_slot: u32, slot_count: u32) -> GpuBatchedMeshRange {
    GpuBatchedMeshRange {
        mesh: Assets::<Mesh>::default().reserve_handle(),
        first_index: 0,
        index_count: 0,
        first_slot,
        slot_count,
        material: None,
    }
}

/// `gs-patch-plan.md` test T5: the slot layout `range_of` (and its WGSL mirror) rests on is
/// exactly "sorted, non-empty, contiguous, filling the capacity". Empty is upstream behaviour
/// and always valid.
///
/// The headless half of T5 - a bad batch extracting with its ranges cleared - is not reachable
/// here: `extract_gpu_batched_meshes` is a render-world `ExtractSchedule` system over the
/// `pub(crate)` `ExtractedGpuBatchedMeshes`, and this app has no render app. The warn-and-clear
/// path is a two-line branch over this function.
#[test]
fn validate_ranges_accepts_exactly_the_partitions() {
    assert_eq!(validate_ranges(&[], 64), Ok(()), "no ranges is upstream #25978, not an error");
    assert_eq!(validate_ranges(&[], 0), Ok(()));
    assert_eq!(validate_ranges(&[mesh_range(0, 64)], 64), Ok(()));
    assert_eq!(validate_ranges(&[mesh_range(0, 4), mesh_range(4, 6), mesh_range(10, 54)], 64), Ok(()));

    assert_eq!(validate_ranges(&[mesh_range(0, 0)], 64), Err(RangeError::EmptySpan { range: 0 }), "an empty span would make two ranges start at one slot");
    assert_eq!(validate_ranges(&[mesh_range(1, 63)], 64), Err(RangeError::NotContiguous { range: 0, expected_first_slot: 0, first_slot: 1 }), "the first range must start at slot 0");
    assert_eq!(validate_ranges(&[mesh_range(0, 4), mesh_range(8, 56)], 64), Err(RangeError::NotContiguous { range: 1, expected_first_slot: 4, first_slot: 8 }), "a gap");
    assert_eq!(validate_ranges(&[mesh_range(0, 8), mesh_range(4, 60)], 64), Err(RangeError::NotContiguous { range: 1, expected_first_slot: 8, first_slot: 4 }), "an overlap");
    assert_eq!(validate_ranges(&[mesh_range(4, 60), mesh_range(0, 4)], 64), Err(RangeError::NotContiguous { range: 0, expected_first_slot: 0, first_slot: 4 }), "unsorted");
    assert_eq!(validate_ranges(&[mesh_range(0, 32)], 64), Err(RangeError::CapacityMismatch { end: 32, max_capacity: 64 }), "the ranges must fill the batch");
    assert_eq!(validate_ranges(&[mesh_range(0, 128)], 64), Err(RangeError::CapacityMismatch { end: 128, max_capacity: 64 }), "and must not run past it");
    assert_eq!(validate_ranges(&[mesh_range(0, u32::MAX), mesh_range(u32::MAX, 2)], 64), Err(RangeError::SlotOverflow { range: 1 }));
}

/// `gs-patch-plan.md` test T4: for every slot of a partition, `range_of` returns the one range
/// containing it. The oracle is the partition itself, so this is exact, not a smoke test.
///
/// `range_of` is the Rust twin of `materialize.wesl`'s; nothing here executes the WGSL (no GPU),
/// so the two staying in lockstep is a review duty, and the test exists to make a change to the
/// Rust one visible.
#[test]
fn range_of_is_a_correct_partition_search() {
    // Boundaries, by hand.
    assert_eq!(range_of(&[], 7), 0, "an empty table has no range; the caller checks the count");
    assert_eq!(range_of(&[0], 0), 0);
    assert_eq!(range_of(&[0], 4095), 0);
    for (slot, expected) in [(0, 0), (3, 0), (4, 1), (9, 1), (10, 2), (63, 2)] {
        assert_eq!(range_of(&[0, 4, 10], slot), expected, "slot {slot}");
    }

    // Randomised partitions of 0..capacity into 1..=16 ranges, exhaustive over their slots.
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut roll = move |n: u32| {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((state >> 33) % u64::from(n)) as u32
    };
    for _ in 0..200 {
        let capacity = 1 + roll(256);
        let wanted = 1 + roll(16);
        let mut cuts: Vec<u32> = (1..wanted).map(|_| 1 + roll(capacity)).collect();
        cuts.push(0);
        cuts.sort_unstable();
        cuts.dedup();
        let first_slots: Vec<u32> = cuts.into_iter().filter(|c| *c < capacity).collect();
        let ranges: Vec<GpuBatchedMeshRange> = first_slots
            .iter()
            .enumerate()
            .map(|(i, &first_slot)| {
                let end = first_slots.get(i + 1).copied().unwrap_or(capacity);
                mesh_range(first_slot, end - first_slot)
            })
            .collect();
        assert_eq!(validate_ranges(&ranges, capacity), Ok(()), "the generator makes partitions: {first_slots:?} of {capacity}");
        for slot in 0..capacity {
            let found = range_of(&first_slots, slot);
            assert!(first_slots[found] <= slot, "range {found} of {first_slots:?} starts after slot {slot}");
            if let Some(next) = first_slots.get(found + 1) {
                assert!(slot < *next, "range {found} of {first_slots:?} ends before slot {slot}");
            }
            // The same answer stated the other way round: exactly one range contains the slot.
            let containing = ranges.iter().filter(|r| r.first_slot <= slot && slot < r.first_slot + r.slot_count).count();
            assert_eq!(containing, 1, "slot {slot} of {first_slots:?}");
        }
    }
}

/// `MinimalPlugins` + assets + the plugin; no render app (no GPU), so only the main-world half runs.
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<Mesh>()
        .init_asset::<Shader>()
        .add_plugins(GpuInstanceBatchPlugin);
    app.finish();
    app.cleanup();
    app
}

/// A visible, shadow-mapped directional light with `cascades` cascades for one (fake) view, as
/// `check_dir_light_mesh_visibility` leaves it (that system is not in this app).
fn sun(app: &mut App, view: Entity, cascades: usize, shadows: bool) -> Entity {
    let mut lists = EntityHashMap::default();
    lists.insert(view, vec![VisibleMeshEntities::default(); cascades]);
    let light = app
        .world_mut()
        .spawn((DirectionalLight { shadow_maps_enabled: shadows, ..Default::default() }, CascadesVisibleEntities { entities: lists }))
        .id();
    app.world_mut().get_mut::<ViewVisibility>(light).expect("light visibility").set_visible();
    light
}

fn batch(app: &mut App) -> Entity {
    batch_with(app, Vec::new())
}

/// A batch of 64 slots carrying `ranges` (empty = upstream #25978).
fn batch_with(app: &mut App, ranges: Vec<GpuBatchedMeshRange>) -> Entity {
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().reserve_handle();
    app.world_mut().spawn((GpuBatchedMesh3d { mesh, max_capacity: 64, ranges }, InheritedVisibility::VISIBLE)).id()
}

fn cascade_lists(app: &App, light: Entity, view: Entity) -> Vec<Vec<Entity>> {
    let lists = app.world().get::<CascadesVisibleEntities>(light).expect("cascades");
    lists.entities[&view].iter().map(|l| l.entities.clone()).collect()
}

#[test]
fn batch_draws_as_a_mesh3d_class_and_casts_sun_shadows_headless() {
    let mut app = app();
    let view = app.world_mut().spawn_empty().id();
    let lit = sun(&mut app, view, 4, true);
    let unlit = sun(&mut app, view, 4, false);
    let caster = batch(&mut app);
    let not_caster = batch(&mut app);
    app.world_mut().entity_mut(not_caster).insert(NotShadowCaster);
    for _ in 0..3 {
        app.update();
    }

    let class = app.world().get::<VisibilityClass>(caster).expect("required VisibilityClass");
    assert!(class.iter().any(|c| *c == TypeId::of::<Mesh3d>()), "the batch pushes the Mesh3d class");
    assert!(app.world().get::<Mesh3d>(caster).is_none(), "the batch never carries Mesh3d");

    let lists = cascade_lists(&app, lit, view);
    assert_eq!(lists.len(), 4);
    for list in &lists {
        assert_eq!(list, &vec![caster], "every cascade, once, without the NotShadowCaster batch");
    }
    assert!(app.world().get::<ViewVisibility>(caster).expect("view visibility").get());
    assert!(cascade_lists(&app, unlit, view).iter().all(Vec::is_empty), "shadow maps off: no casters");
}

/// A batch carrying geometry ranges is still an ordinary batch to the main world: the plugin
/// updates, the batch joins the lights' shadow lists, and the ranges it carries are the ones it
/// keeps. `ranges` empty is upstream #25978 and stays valid.
///
/// What the ranges then *draw* needs a GPU: the slabs, the material bindings and the range
/// upload all live in the render world (`prepare_gpu_batched_mesh_draws`,
/// `gpu_instance_batch::materialize`), which this app does not have. Logged in
/// `bevyrrowind/docs/test-pass-handoffs/gs-slice-b1.md`.
#[test]
fn a_ranged_batch_is_an_ordinary_batch_headless() {
    let mut app = app();
    let view = app.world_mut().spawn_empty().id();
    let lit = sun(&mut app, view, 2, true);
    let ranges = vec![mesh_range(0, 4), mesh_range(4, 6), mesh_range(10, 54)];
    assert_eq!(validate_ranges(&ranges, 64), Ok(()));
    let ranged = batch_with(&mut app, ranges);
    let plain = batch(&mut app);
    for _ in 0..3 {
        app.update();
    }

    for list in cascade_lists(&app, lit, view) {
        assert_eq!(list, vec![ranged, plain].into_iter().min_by_key(|e| e.index()).map_or_else(Vec::new, |_| { let mut both = vec![ranged, plain]; both.sort(); both }), "both batches cast, ranges or not");
    }
    assert_eq!(app.world().get::<GpuBatchedMesh3d>(ranged).expect("the batch").ranges.len(), 3);
    assert!(app.world().get::<GpuBatchedMesh3d>(plain).expect("the batch").ranges.is_empty(), "no ranges = upstream behaviour");
}

/// static-scene.md S3a2 (`gs-patch-plan.md` step 3, test T7): a GPU-authored instance batch is
/// binned in `instance_batches` and counted as a bin and an entity, the three patch GS draw
/// counters exist and read 0 before batching, `batch_set_count` is then exactly the entity
/// path's count (so the slice S0 baseline is unmoved), and `prepare_for_new_frame` resets the
/// draw side while keeping the retained bins.
///
/// The non-zero half of T7 needs a GPU: `instance_batch_draws` is only ever filled by
/// `batch_and_prepare_binned_render_phase`, which allocates in a `RenderDevice` buffer. Logged
/// for the owner in `bevyrrowind/docs/test-pass-handoffs/gs-slice-a.md`.
#[test]
fn instance_batch_bins_and_patch_gs_counters() {
    use bevy::core_pipeline::core_3d::{Opaque3d, Opaque3dBatchSetKey, Opaque3dBinKey};
    use bevy::render::batching::gpu_preprocessing::GpuPreprocessingMode;
    use bevy::render::mesh::allocator::MeshSlabs;
    use bevy::render::render_phase::{
        BinnedRenderPhaseType, DrawFunctionId, InputUniformIndex, ViewBinnedRenderPhases,
    };
    use bevy::render::render_resource::CachedRenderPipelineId;
    use bevy::render::sync_world::MainEntity;
    use bevy::render::view::RetainedViewEntity;

    let mut app = app();
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().reserve_handle();
    let batch_entity = MainEntity::from(app.world_mut().spawn_empty().id());
    let view_owner = MainEntity::from(app.world_mut().spawn_empty().id());
    let view = RetainedViewEntity::new(view_owner, None, 0);

    let mut phases: ViewBinnedRenderPhases<Opaque3d> = ViewBinnedRenderPhases::default();
    phases.prepare_for_new_frame(view, GpuPreprocessingMode::Culling);
    let phase = phases.get_mut(&view).expect("the phase was just created");
    phase.add(
        Opaque3dBatchSetKey {
            pipeline: CachedRenderPipelineId::new(7),
            draw_function: DrawFunctionId(0),
            material_bind_group_index: Some(0),
            slabs: MeshSlabs::default(),
            lightmap_slab: None,
        },
        Opaque3dBinKey { asset_id: mesh.id().untyped() },
        (Entity::PLACEHOLDER, batch_entity),
        InputUniformIndex(0),
        BinnedRenderPhaseType::InstanceBatch,
    );

    // The bin is public, which is what `sf_diag_bevy` chains into the pipeline sets so a
    // pipeline used only by statics is still counted.
    assert_eq!(phase.instance_batches.len(), 1, "one bin per (batch set key, bin key)");
    assert_eq!(phase.bin_count(), 1);
    assert_eq!(phase.entity_count(), 1, "one entity per batch");
    assert!(!phase.is_empty(), "the bin exists, so phase-empty early-outs do not fire");
    // Nothing is batched without a GPU, so every draw-side counter is 0 and `batch_set_count`
    // keeps reporting only `self.batch_sets`.
    assert_eq!(phase.batch_set_count(), 0);
    assert_eq!(phase.instance_batch_draw_count(), 0);
    assert_eq!(phase.instance_batch_indirect_draw_count(), 0);
    assert_eq!(phase.instance_batch_slot_count(), 0);

    phase.prepare_for_new_frame();
    assert_eq!(phase.batch_set_count(), 0);
    assert_eq!(phase.instance_batch_draw_count(), 0, "cleared with the batch sets");
    assert_eq!(phase.bin_count(), 1, "bins are retained across frames");
}
