//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for [`GroundcoverMaterial`]:
//! the main-pass and prepass stages are linked by `wesl` against the vendored `bevy_pbr` modules and
//! `sf_wind_bevy`'s wind module as Bevy's shader cache does, and validated by naga, per def set (the resolver
//! follows `sf_character_bevy`'s and `sf_rockgen_bevy`'s `shader_tests.rs`). Plus: the WGSL uniform matches the
//! Rust struct, and the tag unpacking matches `sf_groundcover::Tag`.
//!
//! [`GroundcoverMaterial`]: crate::GroundcoverMaterial

use std::borrow::Cow;
use std::path::PathBuf;

use wesl::syntax::{ModulePath, PathOrigin};

use crate::material::{PARAMS_BINDING, WIND_BINDING};
use crate::residency::MAX_SPECIES_PER_BATCH;

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (values as the 0.20 pipelines set them on desktop;
/// only their presence and type matter here), and the wind slot `GroundcoverWind::specialize` sets.
fn constants() -> String {
    format!(
        "\
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
const SF_WIND_GROUP = 3;
const SF_WIND_BINDING = {WIND_BINDING};
"
    )
}

fn crates() -> PathBuf {
    PathBuf::from(CRATE).join("..")
}

fn vendor(package: &str) -> PathBuf {
    PathBuf::from(CRATE).join("../../vendor").join(package).join("src")
}

/// The file as text with LF line ends (a Windows checkout with `core.autocrlf` has CRLF).
fn read(path: PathBuf) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())).replace("\r\n", "\n")
}

/// Module paths to files: the vendored Bevy crates, the wind crates and this crate.
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
            return Ok(Cow::Owned(constants()));
        }
        // `sf_wind::field` is `sf_wind::WESL` (registered under that path by `sf_wind_bevy`).
        if package == "sf_wind" && path.components == ["field"] {
            return Ok(Cow::Owned(read(crates().join("sf_wind/src/shaders/sf_wind.wesl"))));
        }
        let mut file = match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" | "bevy_grass" => vendor(package),
            "sf_groundcover_bevy" => PathBuf::from(CRATE).join("src"),
            "sf_wind_bevy" => crates().join("sf_wind_bevy/src"),
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

const MAIN: &str = "sf_groundcover_bevy::shaders::groundcover";
const PREPASS: &str = "sf_groundcover_bevy::shaders::groundcover_prepass";

/// A textured card's main-pass pipeline (Bevy 0.20 `mesh.rs` vertex defs; the instance index output is always on).
const BASE: &[&str] = &[
    "MESH_PIPELINE",
    "VERTEX_POSITIONS",
    "VERTEX_NORMALS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
    "STANDARD_MATERIAL_BASE_COLOR_TEXTURE",
    "MAY_DISCARD",
];

/// The view features the game turns on (`sf_rockgen_bevy`'s HIGH set).
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

const PREPASS_BASE: &[&str] = &[
    "PREPASS_PIPELINE",
    "VERTEX_POSITIONS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
];

/// Mesh variants every stage must take: plain, compressed vertices, colours + second UVs + range dither, tangents.
const VARIANTS: &[&[&str]] = &[
    &[],
    &["VERTEX_POSITIONS_COMPRESSED", "VERTEX_NORMALS_COMPRESSED", "VERTEX_UVS_A_COMPRESSED"],
    &["VERTEX_COLORS", "VERTEX_UVS_B", "VISIBILITY_RANGE_DITHER"],
    &["VERTEX_TANGENTS", "STANDARD_MATERIAL_NORMAL_MAP"],
];

/// Prepass kinds: depth only (and the directional shadow's clamp), normals, motion vectors, both.
const PASSES: &[&[&str]] = &[
    &["DEPTH_PREPASS"],
    &["DEPTH_PREPASS", "UNCLIPPED_DEPTH_ORTHO_EMULATION"],
    &["DEPTH_PREPASS", "NORMAL_PREPASS", "NORMAL_PREPASS_OR_DEFERRED_PREPASS", "VERTEX_NORMALS", "VERTEX_TANGENTS"],
    &["DEPTH_PREPASS", "MOTION_VECTOR_PREPASS"],
    &["DEPTH_PREPASS", "NORMAL_PREPASS", "NORMAL_PREPASS_OR_DEFERRED_PREPASS", "VERTEX_NORMALS", "MOTION_VECTOR_PREPASS"],
];

fn set(parts: &[&[&'static str]]) -> Vec<&'static str> {
    let mut s: Vec<&str> = parts.iter().flat_map(|p| p.iter().copied()).collect();
    s.sort_unstable();
    s.dedup();
    s
}

fn check(module: &str, sets: &[Vec<&str>], entries: &[(&str, naga::ShaderStage)]) -> Vec<String> {
    let mut failures = Vec::new();
    for defs in sets {
        match compile(module, defs) {
            Ok(m) => {
                for (name, stage) in entries {
                    if !m.entry_points.iter().any(|e| e.name == *name && e.stage == *stage) {
                        failures.push(format!("{module} {defs:?}: no `{name}` entry point"));
                    }
                }
            }
            Err(e) => failures.push(e),
        }
    }
    failures
}

/// The main pass (vertex + tinting fragment) and the prepass vertex link and validate for every def set.
#[test]
fn groundcover_stages_compile_for_every_def_set() {
    let mut main = Vec::new();
    for v in VARIANTS {
        main.push(set(&[BASE, v]));
        main.push(set(&[BASE, HIGH, v]));
    }
    let mut prepass = Vec::new();
    for v in VARIANTS {
        for p in PASSES {
            prepass.push(set(&[PREPASS_BASE, v, p]));
        }
    }
    let mut failures = check(MAIN, &main, &[("vertex", naga::ShaderStage::Vertex), ("fragment", naga::ShaderStage::Fragment)]);
    failures.extend(check(PREPASS, &prepass, &[("vertex", naga::ShaderStage::Vertex)]));
    let count = main.len() + prepass.len();
    assert!(failures.is_empty(), "{} of {count} failed:\n{}", failures.len(), failures.join("\n\n"));
    println!("groundcover: {count} stage/def-set combinations link and validate");
}

/// The vendored `bevy_grass` passes our patches touch (GC-3b1c: the world offset in the gust and patch lookups)
/// link and validate: blade generation, the main pass, the prepass (motion vectors, deferred) and the far grass,
/// with every look feature on and off.
#[test]
fn vendored_grass_stages_compile() {
    use naga::ShaderStage::{Compute, Fragment, Vertex};
    const LOOK: &[&str] = &["GRASS_GROUND_TINT", "GRASS_SHEEN", "GRASS_SHAPE", "GRASS_VARIATION"];
    let main = [set(&[&["MESH_PIPELINE"]]), set(&[&["MESH_PIPELINE", "GRASS_TOON"], LOOK, HIGH])];
    let prepass = [
        set(&[&["PREPASS_PIPELINE", "DEPTH_PREPASS"]]),
        set(&[&["PREPASS_PIPELINE", "DEPTH_PREPASS", "MOTION_VECTOR_PREPASS", "PREPASS_FRAGMENT"], LOOK]),
        set(&[&["PREPASS_PIPELINE", "DEPTH_PREPASS", "DEFERRED_PREPASS", "PREPASS_FRAGMENT"], LOOK]),
    ];
    let mut failures = check("bevy_grass::shaders::generate", &[Vec::new()], &[("cull", Compute), ("generate", Compute)]);
    failures.extend(check("bevy_grass::shaders::grass", &main, &[("vertex", Vertex), ("fragment", Fragment)]));
    failures.extend(check("bevy_grass::shaders::grass_prepass", &prepass, &[("vertex", Vertex)]));
    failures.extend(check("bevy_grass::shaders::far_ground", &[set(&[BASE])], &[("fragment", Fragment)]));
    assert!(failures.is_empty(), "{} failed:
{}", failures.len(), failures.join("

"));
}

/// The WGSL `GroundcoverParams` / `GroundcoverSpecies` have the Rust structs' fields in their order, the species
/// array its length, and the bindings are the Rust ones.
#[test]
fn uniform_matches_rust() {
    let src = read(PathBuf::from(CRATE).join("src/shaders/groundcover_common.wesl"));
    let fields = |name: &str| -> Vec<String> {
        let start = src.find(&format!("struct {name} {{")).unwrap_or_else(|| panic!("{name} in groundcover_common.wesl"));
        let body = &src[start..start + src[start..].find('}').expect("end of struct")];
        body.lines()
            .skip(1)
            .map(str::trim)
            .filter(|l| !l.starts_with("//") && l.contains(':'))
            .filter_map(|l| l.split(':').next().map(str::to_string))
            .collect()
    };
    assert_eq!(fields("GroundcoverParams"), ["fade", "wind", "sway", "sway_cross", "species"]);
    assert_eq!(fields("GroundcoverSpecies"), ["tint_a", "tint_b"]);
    assert!(src.contains(&format!("species: array<GroundcoverSpecies, {MAX_SPECIES_PER_BATCH}>")));
    assert!(src.contains(&format!("min(t >> 24u, {}u)", MAX_SPECIES_PER_BATCH - 1)));
    assert!(src.contains(&format!("@binding({PARAMS_BINDING}) var<uniform> groundcover")));
    // The Rust layout: 4 vec4 + 16 × 2 vec4.
    assert_eq!(<crate::GroundcoverParams as bevy::render::render_resource::ShaderType>::min_size().get(), 16 * (4 + 2 * MAX_SPECIES_PER_BATCH as u64));
}

/// Every sway frequency the uniform carries is a whole number of cycles per `globals.time` wrap (3600 s), within
/// 1/7200 Hz of the request: for 0, the defaults, the range ends and the derived cross-sway frequency.
#[test]
fn sway_frequencies_are_whole_cycles_per_hour() {
    use crate::material::{CROSS_SWAY_RATIO, TIME_WRAP_S, quantise_hz};
    use crate::tunables as t;
    let mut requests = vec![0.0_f32];
    for def in [&t::SWAY_HZ, &t::FLUTTER_HZ] {
        let (lo, hi) = def.bounds().expect("Hz tunables have a range");
        requests.extend([lo, hi, def.default_value(), CROSS_SWAY_RATIO * def.default_value(), CROSS_SWAY_RATIO * hi]);
    }
    for hz in requests {
        let q = quantise_hz(hz);
        // In f32, as the shader multiplies (an f32 frequency is whole cycles to its own precision only).
        let cycles = q * TIME_WRAP_S;
        assert!((cycles - cycles.round()).abs() <= cycles * f32::EPSILON, "{hz} Hz -> {q} Hz: {cycles} cycles per wrap");
        assert!((f64::from(q) - f64::from(hz)).abs() <= 1.0 / 7200.0 + 1e-6, "{hz} Hz -> {q} Hz");
    }
    // `apply` puts the quantised values in the uniform.
    let wind = crate::WindSettings::default();
    let mut params = crate::GroundcoverParams::default();
    wind.apply(&mut params);
    let v4 = bevy_math::Vec4::new;
    assert_eq!(params.wind, v4(wind.scale, wind.bend_per_mps, wind.phase_offset_m, wind.gust_contrast));
    assert_eq!(params.sway, v4(wind.sway_amount, quantise_hz(wind.sway_hz), wind.flutter_amount, quantise_hz(wind.flutter_hz)));
    assert_eq!(params.sway_cross, v4(quantise_hz(CROSS_SWAY_RATIO * wind.sway_hz), 0.0, 0.0, 0.0));
}

/// `unpack_tag` reads the bytes `sf_groundcover::Tag::pack` writes.
#[test]
fn tag_layout_matches_core() {
    let t = sf_groundcover::Tag { wind_phase: 1, tint: 2, fade_seed: 3, batch_slot: 4 }.pack();
    assert_eq!(t, 1 | (2 << 8) | (3 << 16) | (4 << 24));
    let src = read(PathBuf::from(CRATE).join("src/shaders/groundcover_common.wesl"));
    for line in ["f32(t & 0xffu)", "f32((t >> 8u) & 0xffu)", "f32((t >> 16u) & 0xffu)", "t >> 24u"] {
        assert!(src.contains(line), "groundcover_common.wesl lost `{line}`");
    }
}
