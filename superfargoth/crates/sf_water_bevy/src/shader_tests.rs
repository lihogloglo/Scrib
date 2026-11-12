//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks"): the water
//! passes are linked by `wesl` as Bevy's shader cache links them (`bevy_shader`
//! `shader_cache.rs`; the vendored `bevy_pbr` / `bevy_render` / `bevy_core_pipeline` modules)
//! and validated by naga. The resolver is `sf_atmos_bevy`'s `shader_tests.rs`.

use std::borrow::Cow;
use std::path::PathBuf;

use wesl::syntax::{ModulePath, PathOrigin};

const CRATE: &str = env!("CARGO_MANIFEST_DIR");

/// Bevy's integer defs the shaders read through `constants::` (values as the 0.20 pipelines
/// set them on desktop; only their presence and type matter here).
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

/// Module paths to files: the vendored Bevy crates and this crate.
struct Files;

impl Files {
    fn root(package: &str) -> Option<PathBuf> {
        let crates = PathBuf::from(CRATE).join("..");
        match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => Some(crates.join("../vendor").join(package).join("src")),
            "sf_water_bevy" => Some(PathBuf::from(CRATE).join("src")),
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

/// Links `module` with `defs` on and validates the result; the error names the defs.
fn compile(module: &str, defs: &[&str]) -> Result<String, String> {
    let mut parts = module.split("::");
    let path = ModulePath {
        origin: PathOrigin::Package(parts.next().unwrap().to_string()),
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
    Ok(wgsl)
}

/// Every water shader compiles (W-SSPR adds the first water shader test): the FFT, mips,
/// snapshot, velocity and SSPR passes, and the water draw with and without an environment map
/// (its SSPR composition recomputes the environment specular only under `ENVIRONMENT_MAP`).
#[test]
fn water_shaders_compile() {
    let mut failures = Vec::new();
    let jobs: [(&str, &[&str]); 9] = [
        ("sf_water_bevy::shaders::water_fft", &[]),
        ("sf_water_bevy::shaders::water_mips", &[]),
        ("sf_water_bevy::shaders::water_snapshot", &[]),
        ("sf_water_bevy::shaders::water_snapshot", &["MULTISAMPLED"]),
        ("sf_water_bevy::shaders::water_velocity", &[]),
        ("sf_water_bevy::shaders::water_velocity", &["MULTISAMPLED"]),
        ("sf_water_bevy::shaders::water_sspr", &[]),
        ("sf_water_bevy::shaders::water_material", &[]),
        ("sf_water_bevy::shaders::water_material", &["ENVIRONMENT_MAP"]),
    ];
    for (module, defs) in jobs {
        if let Err(e) = compile(module, defs) {
            failures.push(e);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The SSPR passes: both entry points on `SSPR_GROUP`-square workgroups (the dispatch math), the
/// `Sspr` struct the size of its Rust mirror, and the water draw reads the SSPR image and swaps
/// the environment specular for it only with an environment map.
#[test]
fn sspr_passes_match_their_rust_side() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("sf_water_bevy::shaders::water_sspr", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    for name in crate::render::SSPR_ENTRIES {
        let entry = module.entry_points.iter().find(|e| e.name == name).unwrap_or_else(|| panic!("entry point {name}"));
        let g = crate::render::SSPR_GROUP;
        assert_eq!(entry.workgroup_size, [g, g, 1], "{name} workgroup vs SSPR_GROUP");
    }
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("plane")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::render::SsprUniform::min_size().get()), "Sspr layout");

    let with_env = compile("sf_water_bevy::shaders::water_material", &["ENVIRONMENT_MAP"]).expect("compiles");
    let without = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    assert!(with_env.contains("reflection_sampler") && without.contains("reflection_sampler"), "the water draw never samples the SSPR image");
    assert!(with_env.contains("environment_map_light"), "no environment specular to replace");
    assert!(!without.contains("environment_map_light"));
}

/// The WGSL `WaterParams` (`water_common.wesl`) keeps the size of its Rust mirror.
#[test]
fn water_params_layout() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("reflection")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::WaterParams::min_size().get()), "WaterParams layout");
}

/// W-GERSTNER: the shore waves displace the water draw and its motion vectors alike (or DLSS/TAA
/// smear them), shade the fragment, and the WGSL constants are `sf_water::shore_waves`'s.
#[test]
// (Exact: the WGSL literal must parse to the very constant.)
#[allow(clippy::float_cmp)]
fn shore_waves_reach_every_pass_with_the_twins_constants() {
    use sf_water::shore_waves as sw;
    let material = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    let velocity = compile("sf_water_bevy::shaders::water_velocity", &[]).expect("compiles");
    assert!(material.contains("shore_wave_offset") && material.contains("sw_local_wave"), "no shore waves in the water draw");
    assert!(velocity.contains("shore_wave_offset"), "no shore waves in the motion vectors");
    // The previous frame's displacement uses the previous frame's time.
    assert!(velocity.contains("swash.w"), "motion vectors without the previous time");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_common.wesl")).expect("source");
    let constant = |name: &str| -> f32 {
        let line = src.lines().find(|l| l.starts_with(&format!("const {name} = "))).unwrap_or_else(|| panic!("const {name}"));
        line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap_or_else(|_| panic!("{line}"))
    };
    assert_eq!(constant("SW_BREAKER"), sw::BREAKER_INDEX);
    assert_eq!(constant("SW_STEEPNESS"), sw::STEEPNESS_LIMIT);
    assert_eq!(constant("SW_ORBIT"), sw::ORBIT_LIMIT);
    assert_eq!(constant("SW_MIN_DEPTH"), sw::MIN_DEPTH);
    assert_eq!(constant("SW_GRAVITY"), sf_water::GRAVITY);
    #[allow(clippy::cast_possible_truncation)]
    let repeat = sf_water::REPEAT_PERIOD as f32;
    assert_eq!(constant("SW_REPEAT"), repeat);
    let shares = format!("const SW_SHARES = vec3({:?}, {:?}, {:?});", sw::SHARES[0], sw::SHARES[1], sw::SHARES[2]);
    assert!(src.contains(&shares), "{shares}");
    assert!(src.contains(&format!("/ {:?});", sw::NOISE_PERIOD)), "noise period");
}

/// W-GERSTNER-DEPTH: the water draw (vertex and fragment) and its motion vectors read the baked
/// bed slope, at the bindings the Rust side declares (material 113, velocity 8).
#[test]
fn bed_slope_reaches_every_pass() {
    for (module, binding) in [("sf_water_bevy::shaders::water_material", 113), ("sf_water_bevy::shaders::water_velocity", 8)] {
        let wgsl = compile(module, &[]).expect("compiles");
        let parsed = naga::front::wgsl::parse_str(&wgsl).expect("parses");
        let bound = parsed.global_variables.iter().find(|(_, g)| g.name.as_deref() == Some("shore_bed")).and_then(|(_, g)| g.binding.as_ref().map(|b| b.binding));
        assert_eq!(bound, Some(binding), "{module}: shore_bed binding");
        assert!(wgsl.contains("shore_slope"), "{module} never samples the bed slope");
    }
}
