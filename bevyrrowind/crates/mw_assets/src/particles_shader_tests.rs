//! `shaders/particles.wesl` without a GPU or a picture (CONTRIBUTING "Tests and visual
//! checks"): linked by `wesl` with each preset's shader defs against the vendored `bevy_pbr`,
//! as Bevy's shader cache does, and validated by naga (the `sf_surface_bevy` shader-test
//! pattern). Checks that the volume lighting reuses Bevy's light path: the sun carries the
//! atmosphere's transmittance exactly when the atmosphere is on, the clustered lights are
//! summed, the environment map counts when present, and the cascades are sampled.

use std::borrow::Cow;
use std::path::PathBuf;

use wesl::syntax::{ModulePath, PathOrigin};

const CRATE: &str = env!("CARGO_MANIFEST_DIR");
const MODULE: &str = "mw_assets::shaders::particles";

/// Bevy's integer defs the shaders read through `constants::` (desktop values; only their
/// presence and type matter here).
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

/// Module paths to files: the vendored Bevy crates, the fog module and this crate.
struct Files;

impl Files {
    fn root(package: &str) -> Option<PathBuf> {
        let crate_dir = PathBuf::from(CRATE);
        let superfargoth = crate_dir.join("../../../superfargoth");
        match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => Some(superfargoth.join("vendor").join(package).join("src")),
            "sf_render_order_bevy" => Some(superfargoth.join("crates").join(package).join("src")),
            "mw_assets" => Some(crate_dir.join("src")),
            _ => None,
        }
    }
}

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
        let mut file = Self::root(package).ok_or_else(|| missing("unknown package"))?;
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

/// Links the particle shader with `defs` on and validates the result; the error names the defs.
fn compile(defs: &[&str]) -> Result<String, String> {
    let mut parts = MODULE.split("::");
    let path = ModulePath {
        origin: PathOrigin::Package(parts.next().unwrap().to_string()),
        components: parts.map(str::to_string).collect(),
    };
    let mut options = wesl::CompileOptions { imports: true, condcomp: true, visibility: false, strip: true, ..Default::default() };
    for d in defs {
        options.features.flags.insert((*d).to_string(), true.into());
    }
    let wgsl = wesl::compile(&path, &options, &Files)
        .map_err(|e| format!("{defs:?}: link: {}", e.diagnostic().render_plain()))?
        .to_string();
    let parsed = naga::front::wgsl::parse_str(&wgsl).map_err(|e| format!("{defs:?}: parse: {}", e.emit_to_string(&wgsl)))?;
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&parsed)
        .map_err(|e| format!("{defs:?}: validate: {}", e.emit_to_string(&wgsl)))?;
    Ok(wgsl)
}

/// The particle quads' vertex layout (`particles.rs` `quad_mesh`: position, uv, colour) on a
/// blended `StandardMaterial` with a texture.
const MESH: &[&str] = &[
    "VERTEX_POSITIONS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_COLORS",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
    "STANDARD_MATERIAL_BASE_COLOR_TEXTURE",
    "BLEND_ALPHA",
];
/// The High preset's view defs on top (atmosphere, sky environment map, fog, filtered cascades).
const HIGH: &[&str] = &[
    "ENVIRONMENT_MAP",
    "DISTANCE_FOG",
    "ATMOSPHERE",
    "SHADOW_FILTER_METHOD_TEMPORAL",
    "VIEW_PROJECTION_PERSPECTIVE",
    "AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_3",
];

fn with(sets: &[&[&'static str]]) -> Vec<&'static str> {
    sets.iter().flat_map(|s| s.iter().copied()).collect()
}

#[test]
fn particle_shader_compiles_per_preset() {
    let mut failures = Vec::new();
    let jobs = [
        ("low", with(&[MESH])),
        ("high", with(&[MESH, HIGH])),
        ("high, probes in an array", with(&[MESH, HIGH, &["MULTIPLE_LIGHT_PROBES_IN_ARRAY"]])),
        ("ultra", with(&[MESH, HIGH, &["GI_PROBES", "MULTIPLE_LIGHT_PROBES_IN_ARRAY"]])),
        ("irradiance volume", with(&[MESH, HIGH, &["IRRADIANCE_VOLUME", "IRRADIANCE_VOLUMES_ARE_USABLE"]])),
        ("high, no atmosphere", with(&[MESH, HIGH]).into_iter().filter(|d| *d != "ATMOSPHERE").collect()),
        ("clearcoat key", with(&[MESH, HIGH, &["STANDARD_MATERIAL_CLEARCOAT"]])),
    ];
    for (preset, defs) in jobs {
        match compile(&defs) {
            // The raster sun is the top-of-atmosphere one: dimmed by the transmittance exactly
            // when the atmosphere is on (night smoke must not glow).
            Ok(wgsl) if wgsl.contains("atmosphere_transmittance_texture") != defs.contains(&"ATMOSPHERE") => {
                failures.push(format!("[{preset}] sun transmittance linked = {}", !defs.contains(&"ATMOSPHERE")));
            }
            Ok(wgsl) if !wgsl.contains("clustered_lights") => failures.push(format!("[{preset}] no point lights")),
            Ok(wgsl) if wgsl.contains("diffuse_environment_map") != defs.contains(&"ENVIRONMENT_MAP") => {
                failures.push(format!("[{preset}] environment map linked = {}", !defs.contains(&"ENVIRONMENT_MAP")));
            }
            Ok(wgsl) if !wgsl.contains("textureSampleCompareLevel") => failures.push(format!("[{preset}] no cascade tap")),
            // No surface lighting: the volume path replaces `apply_pbr_lighting`.
            Ok(wgsl) if wgsl.contains("apply_pbr_lighting") => failures.push(format!("[{preset}] surface lighting linked")),
            Ok(wgsl) if !wgsl.contains("saturated_light") => failures.push(format!("[{preset}] no light saturation")),
            Ok(_) => {}
            Err(e) => failures.push(format!("[{preset}] {e}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn particle_shader_saturation_matches_the_cpu_twin() {
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/particles.wesl")).unwrap();
    // `particles::saturated_light`, token for token.
    assert!(src.contains("let cap = vec3<f32>(1.0) / max(tint, vec3<f32>(1e-4));"), "the clamp of tint × light at 1");
    assert!(src.contains("return max(scene, min(scene + lamps, cap));"), "the sum saturates, the scene stays physical");
    assert!(src.contains("saturated_light(scene / full, lamps / full, tint)"), "applied once, to the sum");
    assert!(src.contains("tint = in.color.rgb;"), "the vertex colour sets the saturation");
    assert!(!src.contains("smoothstep(0.5, 1.0, peak)"), "no per-lamp whitening ramp");
    assert!(src.contains("let full = 1.0 / view.exposure;"), "fully lit in exposed units: the plain texture at any exposure");
}
