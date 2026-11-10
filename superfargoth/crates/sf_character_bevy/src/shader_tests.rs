//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for the
//! view-model material (`view_model.rs`, first-person.md "View model", FP-2b). The stages are
//! linked by `wesl` against the vendored `bevy_pbr` modules as Bevy's shader cache does
//! (`bevy_shader` `shader_cache.rs`) and validated by naga, per def set (the resolver follows
//! `sf_sky_bevy`'s and `sf_surface_bevy`'s `shader_tests.rs`). Plus: the copies still match the
//! vendored stages except for the documented view-model lines (a Bevy update that changes them
//! fails here: re-sync), the helpers still read as `sf_camera::view_model`'s WESL mirror lines,
//! and the extension's uniform and bindless layout.

use std::borrow::Cow;
use std::path::PathBuf;

use bevy::render::render_resource::{AsBindGroup, ShaderType};
use wesl::syntax::{ModulePath, PathOrigin};

use crate::{ViewModel, ViewModelMaterial, ViewModelUniform};

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (values as the 0.20 pipelines
/// set them on desktop; only their presence and type matter here).
const CONSTANTS: &str = "\
const MATERIAL_BIND_GROUP = 3;
const MAX_CASCADES_PER_LIGHT = 4;
const MAX_DIRECTIONAL_LIGHTS = 10;
const MAX_RECT_LIGHTS = 16;
const PER_OBJECT_BUFFER_BATCH_SIZE = 64;
";

fn vendor(package: &str) -> PathBuf {
    PathBuf::from(CRATE)
        .join("../../vendor")
        .join(package)
        .join("src")
}

fn read(path: PathBuf) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Module paths to files: the vendored Bevy crates and this crate.
struct Files;

impl wesl::Resolver for Files {
    fn resolve_source<'a>(
        &'a self,
        path: &ModulePath,
    ) -> Result<Cow<'a, str>, wesl::error::ResolveError> {
        let missing =
            |why: &str| wesl::error::ResolveError::ModuleNotFound(path.clone(), why.to_string());
        let PathOrigin::Package(package) = &path.origin else {
            return Err(missing("not a package path"));
        };
        // Imports from inside a package arrive as `importer/package` (bevy_shader canonical_path).
        let package = package.rsplit('/').next().unwrap_or(package);
        if package == "constants" && path.components.is_empty() {
            return Ok(Cow::Borrowed(CONSTANTS));
        }
        let mut file = match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => vendor(package),
            "sf_character_bevy" => PathBuf::from(CRATE).join("src"),
            _ => return Err(missing("unknown package")),
        };
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

/// Links `module` with `defs` on and validates the result; the error names the defs.
fn compile(module: &str, defs: &[&str]) -> Result<naga::Module, String> {
    let mut parts = module.split("::");
    let path = ModulePath {
        origin: PathOrigin::Package(parts.next().unwrap_or_default().to_string()),
        components: parts.map(str::to_string).collect(),
    };
    let mut options = wesl::CompileOptions {
        imports: true,
        condcomp: true,
        visibility: false,
        strip: true,
        ..Default::default()
    };
    for d in defs {
        options.features.flags.insert((*d).to_string(), true.into());
    }
    let wgsl = wesl::compile(&path, &options, &Files)
        .map_err(|e| format!("{module} {defs:?}: link: {}", e.diagnostic().render_plain()))?
        .to_string();
    let parsed = naga::front::wgsl::parse_str(&wgsl)
        .map_err(|e| format!("{module} {defs:?}: parse: {}", e.emit_to_string(&wgsl)))?;
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&parsed)
    .map_err(|e| format!("{module} {defs:?}: validate: {}", e.emit_to_string(&wgsl)))?;
    Ok(parsed)
}

const MAIN: &str = "sf_character_bevy::shaders::view_model";
const PREPASS: &str = "sf_character_bevy::shaders::view_model_prepass";

/// What a skinned, normal-mapped character part's pipelines carry (Bevy 0.20 `mesh.rs` /
/// `prepass/mod.rs` vertex defs).
const BASE: &[&str] = &[
    "VERTEX_POSITIONS",
    "VERTEX_NORMALS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_TANGENTS",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
];
const PREPASS_BASE: &[&str] = &[
    "VERTEX_POSITIONS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
];

/// The def sets each stage must build with: plain, skinned, morphed, both, compressed vertices,
/// bindless or not, and for the prepass depth only, normals, motion vectors with and without the
/// previous skin and morph.
fn def_sets(prepass: bool) -> Vec<Vec<&'static str>> {
    let mut variants: Vec<Vec<&str>> = vec![
        vec![],
        vec!["SKINNED"],
        vec!["MORPH_TARGETS"],
        vec!["SKINNED", "MORPH_TARGETS"],
        vec![
            "SKINNED",
            "VERTEX_POSITIONS_COMPRESSED",
            "VERTEX_NORMALS_COMPRESSED",
            "VERTEX_UVS_A_COMPRESSED",
            "VERTEX_TANGENTS_COMPRESSED",
        ],
        vec![
            "SKINNED",
            "VERTEX_COLORS",
            "VERTEX_UVS_B",
            "VISIBILITY_RANGE_DITHER",
        ],
    ];
    if prepass {
        let passes: [&[&str]; 4] = [
            &["DEPTH_PREPASS"],
            &[
                "DEPTH_PREPASS",
                "NORMAL_PREPASS",
                "NORMAL_PREPASS_OR_DEFERRED_PREPASS",
                "VERTEX_NORMALS",
                "VERTEX_TANGENTS",
            ],
            &["DEPTH_PREPASS", "MOTION_VECTOR_PREPASS"],
            &[
                "DEPTH_PREPASS",
                "NORMAL_PREPASS",
                "NORMAL_PREPASS_OR_DEFERRED_PREPASS",
                "VERTEX_NORMALS",
                "VERTEX_TANGENTS",
                "MOTION_VECTOR_PREPASS",
                "HAS_PREVIOUS_SKIN",
                "HAS_PREVIOUS_MORPH",
            ],
        ];
        variants = variants
            .iter()
            .flat_map(|v| {
                passes
                    .iter()
                    .map(move |p| v.iter().chain(p.iter()).copied().collect())
            })
            .collect();
        for v in &mut variants {
            v.push("PREPASS_PIPELINE");
        }
    }
    // The prepass declares normals and tangents only with a normal (or deferred) prepass.
    let base: &[&str] = if prepass { PREPASS_BASE } else { BASE };
    let mut sets = Vec::new();
    for v in variants {
        for bindless in [false, true] {
            let mut set: Vec<&str> = base.iter().chain(&v).copied().collect();
            set.sort_unstable();
            set.dedup();
            if bindless {
                set.push("BINDLESS");
            }
            sets.push(set);
        }
    }
    sets
}

/// Both stages link and validate for every def set, and keep a `vertex` entry point.
#[test]
fn view_model_stages_compile_for_every_def_set() {
    let mut failures = Vec::new();
    let mut count = 0;
    for (module, prepass) in [(MAIN, false), (PREPASS, true)] {
        for defs in def_sets(prepass) {
            count += 1;
            match compile(module, &defs) {
                Ok(m)
                    if !m
                        .entry_points
                        .iter()
                        .any(|e| e.name == "vertex" && e.stage == naga::ShaderStage::Vertex) =>
                {
                    failures.push(format!("{module} {defs:?}: no `vertex` entry point"));
                }
                Ok(_) => {}
                Err(e) => failures.push(e),
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {count} failed:\n{}",
        failures.len(),
        failures.join("\n\n")
    );
    println!("view model: {count} stage/def-set combinations link and validate");
}

/// Code lines only: comments and blank lines dropped, each line trimmed.
fn code(src: &str) -> String {
    src.lines()
        .map(|l| l.split("//").next().unwrap_or_default().trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// `src` from the first `@if(MORPH_TARGETS)` (the morph helpers) to the end of `fn vertex`.
fn stage(src: &str) -> String {
    let start = src.find("@if(MORPH_TARGETS)").expect("morph helpers");
    let vertex = src.find("@vertex").expect("vertex stage");
    let tail = "\n    return out;\n}\n";
    let end = vertex + src[vertex..].find(tail).expect("end of vertex") + tail.len();
    code(&src[start..end])
}

/// `vendor` with each `(stock, ours)` code substitution applied once (each must be found).
fn substituted(vendor: &str, subs: &[(&str, &str)]) -> String {
    let mut s = vendor.to_owned();
    for (stock, ours) in subs {
        assert!(
            s.contains(stock),
            "the vendored stage no longer has:\n{stock}\n(Bevy update: re-sync the view-model stage)"
        );
        s = s.replacen(stock, ours, 1);
    }
    s
}

/// Re-sync guard: the main-pass copy is the vendored `render/mesh.wesl` stage with only the
/// clip position changed (imports aside: `package::` paths become imports).
#[test]
fn main_stage_is_the_vendored_stage_but_the_clip_position() {
    let vendor = stage(&read(vendor("bevy_pbr").join("render/mesh.wesl")));
    let ours = stage(&read(
        PathBuf::from(CRATE).join("src/shaders/view_model.wesl"),
    ));
    let want = substituted(
        &vendor,
        &[
            ("package::render::morph::layer_count", "morph::layer_count"),
            ("package::render::morph::weight_at", "morph::weight_at"),
            (
                "out.position = position_world_to_clip(out.world_position.xyz);",
                "let vm = view_model_params(vertex_no_morph.instance_index);\n@if(VERTEX_POSITIONS)\n\
                 out.position = view_model_clip(position_world_to_clip(view_model_world(out.world_position.xyz, view.view_from_world, vm.fov_scale)), vm.depth);",
            ),
        ],
    );
    assert_eq!(
        ours, want,
        "view_model.wesl drifted from the vendored render/mesh.wesl vertex stage"
    );
}

/// Re-sync guard: the prepass copy is the vendored `prepass/prepass.wesl` stage with only the
/// world position, the clip position and the previous world position changed.
#[test]
fn prepass_stage_is_the_vendored_stage_but_the_view_model_positions() {
    let vendor = stage(&read(vendor("bevy_pbr").join("prepass/prepass.wesl")));
    let ours = stage(&read(
        PathBuf::from(CRATE).join("src/shaders/view_model_prepass.wesl"),
    ));
    let want = substituted(
        &vendor,
        &[
            (
                "out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));\n\
                 out.position = position_world_to_clip(out.world_position.xyz);",
                "let vm = view_model_params(vertex_no_morph.instance_index);\n\
                 let real_world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));\n\
                 out.world_position = vec4(view_model_world(real_world_position.xyz, view.view_from_world, vm.fov_scale), 1.0);\n\
                 out.position = view_model_clip(position_world_to_clip(out.world_position.xyz), vm.depth);",
            ),
            (
                "out.previous_world_position = mesh_functions::mesh_position_local_to_world(\nprev_model,\nvec4<f32>(prev_vertex.position, 1.0)\n);",
                "let real_previous_world_position = mesh_functions::mesh_position_local_to_world(\nprev_model,\nvec4<f32>(prev_vertex.position, 1.0)\n);\n\
                 out.previous_world_position = vec4(\n\
                 view_model_world(real_previous_world_position.xyz, prepass_bindings::previous_view_uniforms.view_from_world, vm.fov_scale),\n\
                 1.0\n);",
            ),
        ],
    );
    assert_eq!(
        ours, want,
        "view_model_prepass.wesl drifted from the vendored prepass/prepass.wesl vertex stage"
    );
}

/// The two helpers are `sf_camera::view_model`'s tested math: its "WESL mirror" doc lines
/// (`ViewModelProjection::pseudo_world`, `depth_slice`) appear verbatim in their bodies.
#[test]
fn helpers_are_the_sf_camera_mirror_lines() {
    let camera = read(PathBuf::from(CRATE).join("../sf_camera/src/view_model.rs"));
    let common = code(&read(
        PathBuf::from(CRATE).join("src/shaders/view_model_common.wesl"),
    ));
    let body = |name: &str| {
        let start = common
            .find(&format!("fn {name}("))
            .unwrap_or_else(|| panic!("no fn {name}"));
        let end = start + common[start..].find("\n}").expect("end of fn");
        common[start..end].to_owned()
    };
    let world = body("view_model_world");
    let mirror: Vec<&str> = camera
        .lines()
        .map(str::trim)
        .filter_map(|l| l.strip_prefix("/// "))
        .filter(|l| {
            l.starts_with("let v = ")
                || l.starts_with("let right = ")
                || l.starts_with("let up = ")
                || l.starts_with("return p + ")
        })
        .collect();
    assert_eq!(
        mirror.len(),
        4,
        "sf_camera's pseudo_world mirror lines moved"
    );
    for line in mirror {
        assert!(
            world.contains(line),
            "view_model_world lacks sf_camera's line `{line}`:\n{world}"
        );
    }
    let slice = camera
        .lines()
        .find_map(|l| {
            l.trim()
                .strip_prefix("/// WESL mirror: `")
                .and_then(|l| l.split('`').next())
        })
        .expect("sf_camera's depth_slice mirror line");
    assert!(
        body("view_model_clip").contains(&format!("return {slice};")),
        "view_model_clip is not `{slice}`"
    );
}

/// The extension's data is two `f32` in the WGSL `ViewModel` order, at the bindings the WGSL
/// declares, and it is bindless like `StandardMaterial`, so the combined material stays bindless.
#[test]
fn view_model_uniform_and_bindless_layout() {
    assert_eq!(ViewModelUniform::min_size().get(), 8);
    let common = code(&read(
        PathBuf::from(CRATE).join("src/shaders/view_model_common.wesl"),
    ));
    assert!(
        common.contains("struct ViewModel {\nfov_scale: f32,\ndepth: f32,\n}"),
        "WGSL ViewModel layout changed"
    );
    for (binding, decl) in [
        (100, "var<uniform> view_model: ViewModel;"),
        (
            101,
            "var<storage> view_model_indices: array<ViewModelIndices>;",
        ),
        (102, "var<storage> view_model_array: array<ViewModel>;"),
    ] {
        assert!(
            common.contains(&format!("@binding({binding}) {decl}")),
            "binding {binding}: `{decl}` moved"
        );
    }
    let src = read(PathBuf::from(CRATE).join("src/view_model.rs"));
    assert!(
        src.contains("#[data(100, ViewModelUniform, binding_array(102))]")
            && src.contains("#[bindless(index_table(range(100..101), binding(101)))]")
    );
    assert!(
        ViewModel::bindless_slot_count().is_some(),
        "the extension is not bindless"
    );
    assert!(
        ViewModelMaterial::bindless_slot_count().is_some(),
        "ExtendedMaterial<StandardMaterial, ViewModel> is not bindless"
    );
    let descriptor =
        ViewModelMaterial::bindless_descriptor().expect("combined bindless descriptor");
    assert_eq!(
        descriptor.index_tables.len(),
        2,
        "StandardMaterial's index table plus ours"
    );
}
