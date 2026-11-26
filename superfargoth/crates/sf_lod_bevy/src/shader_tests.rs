//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for
//! [`ImpostorMaterial`]: the main-pass and prepass stages are linked by `wesl` against the
//! vendored `bevy_pbr` modules as Bevy's shader cache does, and validated by naga, per def set
//! (the resolver follows `sf_groundcover_bevy`'s `shader_tests.rs`). Plus: where the alpha test
//! is (`depth_equal.rs`).
//!
//! [`ImpostorMaterial`]: crate::ImpostorMaterial

use std::borrow::Cow;
use std::path::PathBuf;

use naga::ShaderStage::{Fragment, Vertex};
use wesl::syntax::{ModulePath, PathOrigin};

use crate::DEPTH_EQUAL_MAIN_DEF;

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (values as the 0.20 pipelines set
/// them on desktop; only their presence and type matter here).
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

/// Module paths to files: the vendored Bevy crates, the surface weather crate and this crate.
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
        let root = PathBuf::from(CRATE);
        let mut file = match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => root.join("../../vendor").join(package).join("src"),
            "sf_lod_bevy" => root.join("src"),
            "sf_surface_bevy" => root.join("../sf_surface_bevy/src"),
            // The rain rings `sf_surface_bevy::view` imports.
            "sf_render_order_bevy" => root.join("../sf_render_order_bevy/src"),
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
    let mut options = wesl::CompileOptions { imports: true, condcomp: true, visibility: false, strip: true, ..wesl::CompileOptions::default() };
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

const MAIN: &str = "sf_lod_bevy::shaders::impostor";
const PREPASS: &str = "sf_lod_bevy::shaders::impostor_prepass";

/// The main-pass pipeline of an alpha-masked material (Bevy 0.20 `mesh.rs`).
const BASE: &[&str] = &["MESH_PIPELINE", "MAY_DISCARD"];

/// The view features the game turns on (`sf_groundcover_bevy`'s HIGH set).
const HIGH: &[&str] = &[
    "DEPTH_PREPASS",
    "NORMAL_PREPASS",
    "MOTION_VECTOR_PREPASS",
    "SCREEN_SPACE_AMBIENT_OCCLUSION",
    "CONTACT_SHADOWS",
    "ENVIRONMENT_MAP",
    "DISTANCE_FOG",
    "ATMOSPHERE",
    "SHADOW_FILTER_METHOD_TEMPORAL",
    "VIEW_PROJECTION_PERSPECTIVE",
    "TONEMAP_IN_SHADER",
];

const DITHER: &[&str] = &["VISIBILITY_RANGE_DITHER"];
const EQUAL: &[&str] = &[DEPTH_EQUAL_MAIN_DEF];

/// Prepass kinds: depth only, the directional shadow's clamp, normals, motion vectors, both.
const PASSES: &[&[&str]] = &[
    &["DEPTH_PREPASS"],
    &["DEPTH_PREPASS", "UNCLIPPED_DEPTH_ORTHO_EMULATION"],
    &["DEPTH_PREPASS", "NORMAL_PREPASS", "PREPASS_FRAGMENT"],
    &["DEPTH_PREPASS", "MOTION_VECTOR_PREPASS", "PREPASS_FRAGMENT"],
    &["DEPTH_PREPASS", "NORMAL_PREPASS", "MOTION_VECTOR_PREPASS", "PREPASS_FRAGMENT"],
];

fn set(parts: &[&[&'static str]]) -> Vec<&'static str> {
    let mut s: Vec<&str> = parts.iter().flat_map(|p| p.iter().copied()).collect();
    s.sort_unstable();
    s.dedup();
    s
}

/// Whether any function of the linked module (unused ones are stripped) can discard.
fn discards(module: &naga::Module) -> bool {
    fn block(b: &naga::Block) -> bool {
        b.iter().any(|s| match s {
            naga::Statement::Kill => true,
            naga::Statement::Block(b) => block(b),
            naga::Statement::If { accept, reject, .. } => block(accept) || block(reject),
            naga::Statement::Switch { cases, .. } => cases.iter().any(|c| block(&c.body)),
            naga::Statement::Loop { body, continuing, .. } => block(body) || block(continuing),
            _ => false,
        })
    }
    module.functions.iter().any(|(_, f)| block(&f.body)) || module.entry_points.iter().any(|e| block(&e.function.body))
}

/// The main pass and the prepass link and validate for every def set, with both entry points.
#[test]
fn impostor_stages_compile_for_every_def_set() {
    let mut main = Vec::new();
    for dither in [&[][..], DITHER] {
        main.push(set(&[BASE, dither]));
        for equal in [&[][..], EQUAL] {
            main.push(set(&[BASE, HIGH, dither, equal]));
            main.push(set(&[BASE, HIGH, dither, equal, &["SURFACE_WEATHER"]]));
        }
    }
    let mut prepass = Vec::new();
    for dither in [&[][..], DITHER] {
        for p in PASSES {
            prepass.push(set(&[&["PREPASS_PIPELINE", "MAY_DISCARD"], dither, p]));
        }
    }
    let mut failures = Vec::new();
    for (module, sets) in [(MAIN, &main), (PREPASS, &prepass)] {
        for defs in sets {
            match compile(module, defs) {
                Ok(m) => {
                    for (name, stage) in [("vertex", Vertex), ("fragment", Fragment)] {
                        if !m.entry_points.iter().any(|e| e.name == name && e.stage == stage) {
                            failures.push(format!("{module} {defs:?}: no `{name}` entry point"));
                        }
                    }
                }
                Err(e) => failures.push(e),
            }
        }
    }
    let count = main.len() + prepass.len();
    assert!(failures.is_empty(), "{} of {count} failed:\n{}", failures.len(), failures.join("\n\n"));
    println!("impostor: {count} stage/def-set combinations link and validate");
}

/// Fill rule 5: under the depth-equal pipeline state the main pass never discards (not the
/// alpha test, not the range dither); without it it does; the prepass always does.
#[test]
fn the_alpha_test_is_in_the_prepass_only_under_depth_equal() {
    let module = |name: &str, defs: &[&[&'static str]]| compile(name, &set(defs)).unwrap_or_else(|e| panic!("{e}"));
    for dither in [&[][..], DITHER] {
        assert!(!discards(&module(MAIN, &[BASE, HIGH, dither, EQUAL])), "main pass, depth-equal, {dither:?}");
        assert!(discards(&module(MAIN, &[BASE, HIGH, dither])), "main pass, stock state, {dither:?}");
        assert!(discards(&module(MAIN, &[BASE, dither])), "main pass, no depth prepass, {dither:?}");
        for p in PASSES {
            assert!(discards(&module(PREPASS, &[&["PREPASS_PIPELINE", "MAY_DISCARD"], dither, p])), "prepass {p:?} {dither:?}");
        }
    }
}

/// The equal test needs bit-identical clip positions from both vertex stages: the same
/// expression, `@invariant`, and a camera prepass that writes the rasterised depth (`frag_depth`
/// only under the shadow views' unclipped-depth emulation).
#[test]
fn both_vertex_stages_place_the_card_the_same_way() {
    let read = |file: &str| std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders").join(file)).unwrap().replace("\r\n", "\n");
    let (main, prepass) = (read("impostor.wesl"), read("impostor_prepass.wesl"));
    for src in [&main, &prepass] {
        assert!(src.contains("@builtin(position) @invariant position: vec4<f32>,"));
        assert!(src.contains("let imp = impostor_common::impostor_vertex(v);"));
        assert!(src.contains("out.position = position_world_to_clip(imp.world_position);"));
    }
    assert!(!main.contains("frag_depth"));
    // Declared and written once, both under the emulation def.
    assert_eq!((prepass.matches("@builtin(frag_depth)").count(), prepass.matches("out.frag_depth").count()), (1, 1));
    assert!(prepass.contains("    @if(UNCLIPPED_DEPTH_ORTHO_EMULATION)\n    @builtin(frag_depth) frag_depth: f32,"));
    assert!(prepass.contains("@if(UNCLIPPED_DEPTH_ORTHO_EMULATION)\n    out.frag_depth = in.unclipped_depth;"));
}
