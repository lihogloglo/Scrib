//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for the rock material
//! (`shaders/rock.wesl`): the fragment stage is linked by `wesl` against the vendored `bevy_pbr` modules as
//! Bevy's shader cache does (`bevy_shader` `shader_cache.rs`) and validated by naga, per def set (the resolver
//! follows `sf_character_bevy`'s and `sf_sky_bevy`'s `shader_tests.rs`). Plus: the WGSL uniform matches
//! [`RockParams`] and the flag bits match the Rust ones.

use std::borrow::Cow;
use std::path::PathBuf;

use wesl::syntax::{ModulePath, PathOrigin};

use crate::material::{FLAG_ALBEDO, FLAG_FLIP_Y, FLAG_HEX, FLAG_LOD_TINT, FLAG_NORMAL, FLAG_OCCLUSION, FLAG_ORM};

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (values as the 0.20 pipelines set them on
/// desktop; only their presence and type matter here; `sf_sky_bevy`'s list).
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

/// The file as text with LF line ends (a Windows checkout with `core.autocrlf` has CRLF).
fn read(path: PathBuf) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())).replace("\r\n", "\n")
}

/// Module paths to files: the vendored Bevy crates and this crate.
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
        let mut file = match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => vendor(package),
            "sf_rockgen_bevy" => PathBuf::from(CRATE).join("src"),
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

const MAIN: &str = "sf_rockgen_bevy::shaders::rock";

/// What a rock's main-pass pipeline carries: positions, normals, no UVs or tangents (Bevy 0.20 `mesh.rs`
/// always sets the instance index output).
const BASE: &[&str] = &["MESH_PIPELINE", "VERTEX_POSITIONS", "VERTEX_NORMALS", "VERTEX_OUTPUT_INSTANCE_INDEX"];

/// The view features the game turns on (`sf_sky_bevy`'s HIGH set, minus the `StandardMaterial` texture
/// defs a rock's base never has), with and without the LOD crossfade dither.
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

#[test]
fn rock_fragment_compiles_for_every_def_set() {
    let high: Vec<&str> = BASE.iter().chain(HIGH).copied().collect();
    let dither = |s: &[&'static str]| s.iter().copied().chain(["VISIBILITY_RANGE_DITHER"]).collect::<Vec<_>>();
    let sets: [Vec<&str>; 4] = [BASE.to_vec(), dither(BASE), high.clone(), dither(&high)];
    let mut failures = Vec::new();
    for defs in &sets {
        match compile(MAIN, defs) {
            Ok(m) if !m.entry_points.iter().any(|e| e.name == "fragment" && e.stage == naga::ShaderStage::Fragment) => {
                failures.push(format!("{defs:?}: no `fragment` entry point"));
            }
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{} of {} failed:\n{}", failures.len(), sets.len(), failures.join("\n\n"));
}

/// The WGSL `RockParams` has the Rust struct's fields in its order, and the flag bits agree.
#[test]
fn uniform_and_flags_match_rust() {
    let src = read(PathBuf::from(CRATE).join("src/shaders/rock.wesl"));
    let start = src.find("struct RockParams {").expect("RockParams in rock.wesl");
    let body = &src[start..start + src[start..].find('}').expect("end of RockParams")];
    let fields: Vec<&str> = body.lines().skip(1).filter_map(|l| l.trim().split(':').next()).filter(|f| !f.is_empty()).collect();
    assert_eq!(fields, ["inv_tiling", "blend_sharpness", "hex_contrast", "hex_rotation", "hex_scale", "flags", "lod", "pad"]);
    for (name, bit) in [
        ("FLAG_ALBEDO", FLAG_ALBEDO),
        ("FLAG_NORMAL", FLAG_NORMAL),
        ("FLAG_ORM", FLAG_ORM),
        ("FLAG_OCCLUSION", FLAG_OCCLUSION),
        ("FLAG_FLIP_Y", FLAG_FLIP_Y),
        ("FLAG_HEX", FLAG_HEX),
        ("FLAG_LOD_TINT", FLAG_LOD_TINT),
    ] {
        let line = format!("const {name}: u32 = {bit}u;");
        assert!(src.contains(&line), "rock.wesl lacks `{line}`");
    }
}
