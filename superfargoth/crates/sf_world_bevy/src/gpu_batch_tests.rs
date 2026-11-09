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
use bevy::pbr::gpu_instance_batch::{GpuBatchedMesh3d, GpuInstanceBatchPlugin, GpuMeshInstance};
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

/// Size of the WGSL struct `name` as naga lays it out (storage/uniform buffer rules). Imported
/// structs keep their name as a suffix of the module-mangled one.
fn wgsl_size(module: &naga::Module, name: &str) -> u32 {
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    let (handle, _) = module.types.iter().find(|(_, t)| t.name.as_deref().is_some_and(|n| n == name || n.ends_with(&format!("_{name}")) || n.ends_with(name))).unwrap_or_else(|| panic!("no struct {name}"));
    layouter[handle].size
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
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().reserve_handle();
    app.world_mut().spawn((GpuBatchedMesh3d { mesh, max_capacity: 64 }, InheritedVisibility::VISIBLE)).id()
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
