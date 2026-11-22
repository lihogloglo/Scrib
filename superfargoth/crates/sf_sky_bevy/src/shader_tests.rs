//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks") for the
//! vendored `bevy_pbr` patch "N1" (`AmbientLightSpecular`): the flat ambient this crate drives
//! (the night floor) lights the diffuse lobe only when the view's ambient flag bit is set. The
//! real modules are linked by `wesl` as Bevy's shader cache does (`bevy_shader`
//! `shader_cache.rs`) and validated by naga; the resolver follows `sf_surface_bevy`'s
//! `shader_tests.rs`. The bit is a view uniform, not a shader def: one pipeline serves both
//! states, so each def set is compiled once and the gate checked in the linked source.
//! Patch "N2" (`AtmosphereSettings::airglow`, the night sky glow this crate drives): the
//! atmosphere's sky-view LUT, sky pass and environment capture link with it, and the uniform's
//! Rust and WGSL layouts agree.

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
        // This crate's own shaders (embedded assets at run time).
        let mut file = if package == "sf_sky_bevy" {
            PathBuf::from(CRATE).join("src/shaders")
        } else if matches!(package, "bevy_pbr" | "bevy_render" | "bevy_core_pipeline") {
            vendor(package)
        } else {
            return Err(missing("unknown package"));
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
fn compile(module: &str, defs: &[&str]) -> Result<String, String> {
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
    Ok(wgsl)
}

const BASE: &[&str] = &["VERTEX_POSITIONS", "VERTEX_NORMALS", "VERTEX_UVS", "VERTEX_UVS_A", "VERTEX_TANGENTS", "VERTEX_OUTPUT_INSTANCE_INDEX"];
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
    "STANDARD_MATERIAL_NORMAL_MAP",
    "STANDARD_MATERIAL_BASE_COLOR_TEXTURE",
    "MAY_DISCARD",
];

/// Every lit path reaches the flat ambient through `pbr_ambient::ambient_light` (forward
/// `StandardMaterial`, deferred lighting, lightmapped meshes; the terrain and far proxies call
/// the same `apply_pbr_lighting`): each links and validates with the N1 gate in it.
#[test]
fn lit_shaders_compile_with_the_ambient_diffuse_only_gate() {
    let high: Vec<&str> = BASE.iter().chain(HIGH).copied().collect();
    let lightmap: Vec<&str> = BASE.iter().chain(&["VERTEX_UVS_B", "LIGHTMAP"]).copied().collect();
    let deferred: Vec<&str> = high.iter().chain(&["DEFERRED_PREPASS"]).copied().collect();
    let jobs: [(&str, &[&str]); 4] = [
        ("bevy_pbr::render::pbr", BASE),
        ("bevy_pbr::render::pbr", &high),
        ("bevy_pbr::render::pbr", &lightmap),
        ("bevy_pbr::deferred::deferred_lighting", &deferred),
    ];
    let mut failures = Vec::new();
    for (module, defs) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if !wgsl.contains("AMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT") => {
                failures.push(format!("{module} {defs:?}: the N1 ambient gate is not linked"));
            }
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The patch's Rust flag (`AmbientLightFlags::DIFFUSE_ONLY`, set by `prepare_lights` from
/// `AmbientLightSpecular(false)`) and the WGSL constant are the same bit.
#[test]
fn ambient_diffuse_only_bit_matches_between_rust_and_wgsl() {
    let read = |file: &str| std::fs::read_to_string(vendor("bevy_pbr").join(file)).unwrap_or_else(|e| panic!("{file}: {e}"));
    let rust = read("render/light.rs");
    let wgsl = read("render/mesh_view_types.wesl");
    let squash = |s: &str| s.split_whitespace().collect::<String>();
    assert!(squash(&rust).contains("constDIFFUSE_ONLY=1<<1;"), "light.rs: AmbientLightFlags::DIFFUSE_ONLY moved");
    assert!(squash(&wgsl).contains("constAMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT:u32=1u<<1u;"), "mesh_view_types.wesl: bit moved");
    assert!(squash(&rust).contains("ambient_light_flags|=AmbientLightFlags::DIFFUSE_ONLY;"), "prepare_lights no longer sets the bit");
}

/// CPU twin of `pbr_lighting::F_AB` (the polynomial path; Bevy uses its DFG LUT when bound,
/// which agrees to a few percent) and `EnvBRDFApprox`.
fn f_ab(perceptual_roughness: f32, n_dot_v: f32) -> (f32, f32) {
    let r = [perceptual_roughness.mul_add(-1.0, 1.0), perceptual_roughness.mul_add(-0.0275, 0.0425), perceptual_roughness.mul_add(-0.572, 1.04), perceptual_roughness.mul_add(0.022, -0.04)];
    let a004 = (r[0] * r[0]).min((-9.28 * n_dot_v).exp2()).mul_add(r[0], r[1]);
    ((-1.04f32).mul_add(a004, r[2]).max(0.000_05), 1.04f32.mul_add(a004, r[3]).max(0.000_05))
}

/// The flat ambient's specular share before (stock Bevy) and after N1 (0 %): the terrain at a
/// grazing view (N·V 0.15, roughness 0.85, F0 0.0144 = reflectance 0.3; the brief's case),
/// then a default `StandardMaterial` (reflectance 0.5, F0 0.04) at N·V 0.15 and 0.05.
#[test]
#[ignore = "measurement: flat ambient specular share before/after N1"]
fn flat_ambient_specular_share() {
    for (what, n_dot_v, roughness, f0) in [("terrain", 0.15_f32, 0.85_f32, 0.0144_f32), ("standard", 0.15, 0.85, 0.04), ("standard", 0.05, 0.85, 0.04)] {
        let (da, db) = f_ab(1.0, n_dot_v);
        let (sa, sb) = f_ab(roughness, n_dot_v);
        let specular = f0.mul_add(sa, sb) * (f0 * 50.0 * 0.33 * 3.0).clamp(0.0, 1.0);
        for albedo in [0.1_f32, 0.25, 0.5] {
            // `EnvBRDFApprox(diffuse_color, F_AB(1.0, NdotV))`, as `pbr_ambient.wesl`.
            let diffuse = albedo.mul_add(da, db);
            println!(
                "{what} N.V {n_dot_v:.2} F0 {f0:.4} albedo {albedo:.2}: diffuse {diffuse:.4}, specular {specular:.4} -> specular share {:.1}% before, 0% after",
                specular / (diffuse + specular) * 100.0,
            );
        }
    }
}

/// Patch N2 (sky glow, `AtmosphereSettings::airglow`): the sky-view LUT, the sky pass in every
/// variant and the environment-map capture link and validate, and the two that write sky
/// radiance carry the glow (the capture reads it through the LUT).
#[test]
fn atmosphere_shaders_compile_with_the_night_sky_glow() {
    let jobs: [(&str, &[&str], bool); 6] = [
        ("bevy_pbr::atmosphere::sky_view_lut", &[], true),
        ("bevy_pbr::atmosphere::render_sky", &[], true),
        ("bevy_pbr::atmosphere::render_sky", &["MULTISAMPLED"], true),
        ("bevy_pbr::atmosphere::render_sky", &["DUAL_SOURCE_BLENDING"], true),
        ("bevy_pbr::atmosphere::render_sky", &["DUAL_SOURCE_BLENDING", "MULTISAMPLED"], true),
        ("bevy_pbr::atmosphere::environment", &[], false),
    ];
    let mut failures = Vec::new();
    for (module, defs, glow) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if glow && !wgsl.contains("airglow_horizon") => failures.push(format!("{module} {defs:?}: no glow linked")),
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The Rust uniform (`GpuAtmosphereSettings`, encase layout) and the WGSL struct the shaders
/// read have the same size after the N2 fields (a field added on one side only would shift
/// every read after it).
#[test]
fn atmosphere_settings_layout_matches_between_rust_and_wgsl() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("bevy_pbr::atmosphere::sky_view_lut", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let (handle, _) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.contains("AtmosphereSettings")))
        .unwrap_or_else(|| panic!("AtmosphereSettings in the linked module: {:?}", module.types.iter().filter_map(|(_, t)| t.name.clone()).collect::<Vec<_>>()));
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    let rust = bevy_pbr::GpuAtmosphereSettings::min_size().get();
    assert_eq!(u64::from(layouter[handle].size), rust, "WGSL vs Rust size");
}

/// Patches P1 + P1b (live medium scales, `ScatteringMediumScales`): every atmosphere pass that
/// samples the medium links and validates. The term sum lives only in the combine pass
/// (`medium_lut`, run with the static LUTs); the per-sample passes do stock single fetches
/// (P1b: summing per sample doubled `atmosphere_luts`, 0.36 -> 0.84 ms).
#[test]
fn atmosphere_passes_compile_with_live_medium_scales() {
    let jobs: [(&str, &[&str]); 7] = [
        ("bevy_pbr::atmosphere::medium_lut", &[]),
        ("bevy_pbr::atmosphere::transmittance_lut", &[]),
        ("bevy_pbr::atmosphere::multiscattering_lut", &[]),
        ("bevy_pbr::atmosphere::sky_view_lut", &[]),
        ("bevy_pbr::atmosphere::aerial_view_lut", &[]),
        ("bevy_pbr::atmosphere::render_sky", &[]),
        ("bevy_pbr::atmosphere::environment", &[]),
    ];
    let mut failures = Vec::new();
    for (module, defs) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if module.ends_with("medium_lut") && !wgsl.contains("medium_term_count") => {
                failures.push(format!("{module}: the combine pass does not read the live terms"));
            }
            Ok(wgsl) if !module.ends_with("medium_lut") && wgsl.contains("sample_medium_falloff") => {
                failures.push(format!("{module}: per-sample term sum is back (P1b)"));
            }
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Patch P1: the Rust atmosphere uniform (`GpuAtmosphere` with the medium term arrays,
/// `MAX_MEDIUM_TERMS` long) and the WGSL `Atmosphere` struct have the same size.
#[test]
fn atmosphere_uniform_layout_matches_between_rust_and_wgsl() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("bevy_pbr::atmosphere::transmittance_lut", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let (handle, _) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.contains("Atmosphere") && !n.contains("Settings") && !n.contains("Transform")))
        .unwrap_or_else(|| panic!("Atmosphere in the linked module: {:?}", module.types.iter().filter_map(|(_, t)| t.name.clone()).collect::<Vec<_>>()));
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    let rust = bevy_pbr::resources::GpuAtmosphere::min_size().get();
    assert_eq!(u64::from(layouter[handle].size), rust, "WGSL vs Rust size");
    assert_eq!(bevy_pbr::MAX_MEDIUM_TERMS, 8, "types.wesl declares array<vec4<f32>, 8>");
}

/// NS-3: the night pass (stars, moons with their opacity and texture blend, glare) links and validates.
#[test]
fn night_sky_shader_compiles() {
    let wgsl = compile("sf_sky_bevy::night", &[]).unwrap_or_else(|e| panic!("{e}"));
    assert!(wgsl.contains("texture_blend") && wgsl.contains("alpha"));
}

/// NS-3c: the night pass with the moon glow (texture hue taps, glow under the disk) links and
/// validates, and the Rust night uniform (`GpuMoon` with the glow fields) and the WGSL `Night`
/// struct have the same size.
#[test]
fn night_sky_glow_compiles_and_the_uniform_layout_matches() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("sf_sky_bevy::night", &[]).unwrap_or_else(|e| panic!("{e}"));
    assert!(wgsl.contains("glow_falloff") && wgsl.contains("textureNumLevels"), "the glow and its hue are in the linked shader");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let (handle, _) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.contains("Night")))
        .unwrap_or_else(|| panic!("Night in the linked module: {:?}", module.types.iter().filter_map(|(_, t)| t.name.clone()).collect::<Vec<_>>()));
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    let rust = crate::night::NightUniform::min_size().get();
    assert_eq!(u64::from(layouter[handle].size), rust, "WGSL vs Rust size");
}

/// Patch E1b (`DirectionalLightAtmosphereScatter`, `sky.moon.sky_scatter`): every atmosphere
/// pass that in-scatters the directional lights (sky-view LUT, aerial-view LUT, the raymarched
/// sky) links and validates with the per-light scale; the environment capture reads the
/// sky-view LUT, so it carries the scale through it.
#[test]
fn atmosphere_passes_scale_each_lights_in_scattering() {
    let jobs: [(&str, &[&str]); 4] = [
        ("bevy_pbr::atmosphere::sky_view_lut", &[]),
        ("bevy_pbr::atmosphere::aerial_view_lut", &[]),
        ("bevy_pbr::atmosphere::render_sky", &[]),
        ("bevy_pbr::atmosphere::render_sky", &["MULTISAMPLED"]),
    ];
    let mut failures = Vec::new();
    for (module, defs) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if !wgsl.contains(".atmosphere_scatter") => failures.push(format!("{module} {defs:?}: no per-light scatter scale")),
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Patches CW-8 + E1b: the Rust `GpuDirectionalLight` (`render/light.rs`, encase layout) and the
/// WGSL `DirectionalLight` (`mesh_view_types.wesl`) have the same fields in the same order and
/// the same size (both patched fields sit in the struct's tail padding, so the size alone would
/// not catch a field added on one side only).
#[test]
fn directional_light_layout_matches_between_rust_and_wgsl() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("bevy_pbr::atmosphere::sky_view_lut", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let (handle, ty) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.ends_with("DirectionalLight")))
        .unwrap_or_else(|| panic!("DirectionalLight in the linked module: {:?}", module.types.iter().filter_map(|(_, t)| t.name.clone()).collect::<Vec<_>>()));
    let naga::TypeInner::Struct { members, .. } = &ty.inner else { panic!("DirectionalLight is a struct") };
    let wgsl_fields: Vec<String> = members.iter().map(|m| m.name.clone().expect("named member")).collect();

    let source = std::fs::read_to_string(vendor("bevy_pbr").join("render/light.rs")).expect("vendored light.rs");
    let body = source
        .split("pub struct GpuDirectionalLight {")
        .nth(1)
        .and_then(|rest| rest.split("\n}").next())
        .expect("GpuDirectionalLight in light.rs");
    let rust_fields: Vec<String> = body
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//") && line.contains(':'))
        .map(|line| line.split(':').next().unwrap_or_default().trim().to_owned())
        // The one stock rename between the two sides.
        .map(|name| if name == "dir_to_light" { "direction_to_light".to_owned() } else { name })
        .collect();
    assert_eq!(rust_fields, wgsl_fields, "Rust vs WGSL fields");
    assert_eq!(wgsl_fields.last().map(String::as_str), Some("atmosphere_scatter"));

    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    let rust = bevy_pbr::GpuDirectionalLight::min_size().get();
    assert_eq!(u64::from(layouter[handle].size), rust, "WGSL vs Rust size");
}

/// Horizon bands (2026-11-19, bevy #26208 transmittance part + own upward read): the
/// transmittance LUT holds optical depth, every reader turns it back with `exp(-x)`, and the
/// aerial perspective's segment subtracts optical depths, reading a descending segment backwards
/// (rays climbing out of both ends). The passes that read it link and validate.
#[test]
fn transmittance_lut_holds_optical_depth_and_segments_read_upward() {
    let read = |rel: &str| std::fs::read_to_string(vendor("bevy_pbr").join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let writer = read("atmosphere/transmittance_lut.wesl");
    assert!(writer.contains("vec4(finite_depth, 1.0)") && !writer.contains("exp(-optical_depth)"), "the LUT stores optical depth");
    let functions = read("atmosphere/functions.wesl");
    assert!(functions.contains("return exp(-sample_optical_depth_lut(r, mu));"), "atmosphere sampler converts");
    for rel in ["render/pbr_lighting.wesl", "volumetric_fog/volumetric_fog.wesl"] {
        assert!(read(rel).contains("return exp(-textureSampleLevel("), "{rel}: own LUT sampler converts");
    }
    let segment = read("atmosphere/bruneton_functions.wesl");
    assert!(segment.contains("ray_intersects_ground(r, mu) || mu_t <= 0.0"), "descending segments read upward");
    assert!(!segment.contains("sample_transmittance_lut("), "no transmittance ratio left");
    let high: Vec<&str> = BASE.iter().chain(HIGH).copied().collect();
    let jobs: [(&str, &[&str]); 4] = [
        ("bevy_pbr::atmosphere::transmittance_lut", &[]),
        ("bevy_pbr::atmosphere::render_sky", &[]),
        ("bevy_pbr::atmosphere::render_sky", &["DUAL_SOURCE_BLENDING"]),
        ("bevy_pbr::render::pbr", &high),
    ];
    let failures: Vec<String> = jobs.into_iter().filter_map(|(m, d)| compile(m, d).err()).collect();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// CW-13b: 2D Point Shadow Atlas shaders link & validate under Naga with `POINT_SHADOW_ATLAS` both
/// on and off, `point_shadow_atlas_blit.wesl` validates cleanly, and GPU struct layouts match Rust.
#[test]
fn point_shadow_atlas_shaders_compile_and_validate() {
    use bevy_render::render_resource::ShaderType;

    let high: Vec<&str> = BASE.iter().chain(HIGH).copied().collect();
    let high_atlas: Vec<&str> = high.iter().copied().chain(["POINT_SHADOW_ATLAS", "AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_3"]).collect();
    let high_atlas_samplers: Vec<&str> = high_atlas.iter().copied().chain(["USE_DEPTH_SAMPLERS"]).collect();
    let high_atlas_gaussian: Vec<&str> = BASE
        .iter()
        .chain(HIGH.iter().filter(|&&d| d != "SHADOW_FILTER_METHOD_TEMPORAL"))
        .copied()
        .chain(["SHADOW_FILTER_METHOD_GAUSSIAN", "POINT_SHADOW_ATLAS", "AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_3", "USE_DEPTH_SAMPLERS"])
        .collect();
    let deferred_atlas: Vec<&str> = high_atlas.iter().copied().chain(["DEFERRED_PREPASS"]).collect();
    let fog_off: &[&str] = &["SHADOW_FILTER_METHOD_HARDWARE_2X2", "ATMOSPHERE", "AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_3"];
    let fog_on: &[&str] = &["SHADOW_FILTER_METHOD_HARDWARE_2X2", "ATMOSPHERE", "AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_3", "POINT_SHADOW_ATLAS"];

    let jobs: [(&str, &[&str]); 8] = [
        ("bevy_pbr::render::pbr", &high),
        ("bevy_pbr::render::pbr", &high_atlas),
        ("bevy_pbr::render::pbr", &high_atlas_samplers),
        ("bevy_pbr::render::pbr", &high_atlas_gaussian),
        ("bevy_pbr::deferred::deferred_lighting", &deferred_atlas),
        ("bevy_pbr::volumetric_fog::volumetric_fog", fog_off),
        ("bevy_pbr::volumetric_fog::volumetric_fog", fog_on),
        ("bevy_pbr::render::point_shadow_atlas_blit", &[]),
    ];

    let mut failures = Vec::new();
    let mut pbr_atlas_wgsl = String::new();
    let mut blit_wgsl = String::new();
    for (module, defs) in jobs {
        match compile(module, defs) {
            Ok(wgsl) => {
                if module == "bevy_pbr::render::pbr" && defs.contains(&"POINT_SHADOW_ATLAS") {
                    pbr_atlas_wgsl = wgsl;
                } else if module == "bevy_pbr::render::point_shadow_atlas_blit" {
                    blit_wgsl = wgsl;
                }
            }
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));

    // Verify `ShadowAtlasTile` WGSL layout matches Rust `GpuShadowAtlasTile` (16 bytes).
    let module = naga::front::wgsl::parse_str(&pbr_atlas_wgsl).expect("pbr atlas wgsl");
    let (handle, _) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.ends_with("ShadowAtlasTile")))
        .expect("ShadowAtlasTile struct in linked pbr wgsl");
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    assert_eq!(u64::from(layouter[handle].size), bevy_pbr::GpuShadowAtlasTile::min_size().get(), "ShadowAtlasTile WGSL vs Rust size");

    // Verify `AtlasBlitUniform` WGSL layout matches Rust `AtlasBlitUniform` (32 bytes).
    let blit_mod = naga::front::wgsl::parse_str(&blit_wgsl).expect("blit wgsl");
    let (blit_handle, _) = blit_mod
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.ends_with("AtlasBlitUniform")))
        .expect("AtlasBlitUniform struct in blit wgsl");
    let mut blit_layouter = naga::proc::Layouter::default();
    blit_layouter.update(blit_mod.to_ctx()).expect("layout");
    assert_eq!(u64::from(blit_layouter[blit_handle].size), bevy_pbr::AtlasBlitUniform::min_size().get(), "AtlasBlitUniform WGSL vs Rust size");
}

/// Patch AS (aerial shadow, `bevy_pbr::AtmosphereHeightShadow`, `docs/specs/aerial-shadow.md`):
/// the aerial view LUT links and validates with and without `ATMOSPHERE_HEIGHT_SHADOW`. Without
/// it the pass is stock (no height shadow linked, the stock in-scattering); with it, it reads the
/// map through the shadowed in-scattering. The Rust uniform (`GpuAtmosphereHeightShadow`, encase
/// layout) and the WGSL struct have the same fields in the same order and the same size.
#[test]
fn aerial_view_lut_compiles_with_and_without_the_height_shadow() {
    use bevy_render::render_resource::ShaderType;
    let stock = compile("bevy_pbr::atmosphere::aerial_view_lut", &[]).unwrap_or_else(|e| panic!("{e}"));
    assert!(!stock.contains("height_shadow") && !stock.contains("sample_local_inscattering_shadowed"), "absent = the stock pass");
    assert!(stock.contains("sample_local_inscattering"), "the stock in-scattering");
    let def = bevy_pbr::ATMOSPHERE_HEIGHT_SHADOW_DEF;
    let shadowed = compile("bevy_pbr::atmosphere::aerial_view_lut", &[def]).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        shadowed.contains("height_shadow_map") && shadowed.contains("sample_local_inscattering_shadowed") && shadowed.contains(".atmosphere_scatter"),
        "the height shadow, the shadowed in-scattering and E1b's per-light scale are linked"
    );
    let module = naga::front::wgsl::parse_str(&shadowed).expect("parses");
    let (handle, ty) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.contains("AtmosphereHeightShadow")))
        .unwrap_or_else(|| panic!("AtmosphereHeightShadow in the linked module: {:?}", module.types.iter().filter_map(|(_, t)| t.name.clone()).collect::<Vec<_>>()));
    let naga::TypeInner::Struct { members, .. } = &ty.inner else { panic!("a struct") };
    let names: Vec<&str> = members.iter().filter_map(|m| m.name.as_deref()).collect();
    assert_eq!(names, ["params", "light_dir", "penumbra_per_m", "penumbra_min", "bias", "bias_per_m", "fade_start"], "field order = GpuAtmosphereHeightShadow");
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    assert_eq!(u64::from(layouter[handle].size), bevy_pbr::GpuAtmosphereHeightShadow::min_size().get(), "WGSL vs Rust size");
}

/// The binding indices of every resource the linked module declares (all in group 0), sorted.
fn bindings(wgsl: &str) -> Vec<u32> {
    let module = naga::front::wgsl::parse_str(wgsl).expect("parses");
    let mut out: Vec<u32> = module.global_variables.iter().filter_map(|(_, v)| v.binding.as_ref().map(|b| b.binding)).collect();
    out.sort_unstable();
    out
}

/// The field names and WGSL size of struct `name` (substring, as the linker may mangle it).
fn struct_layout(wgsl: &str, name: &str) -> (Vec<String>, u64) {
    let module = naga::front::wgsl::parse_str(wgsl).expect("parses");
    let (handle, ty) = module
        .types
        .iter()
        .find(|(_, t)| t.name.as_deref().is_some_and(|n| n.contains(name)))
        .unwrap_or_else(|| panic!("{name} in the linked module"));
    let naga::TypeInner::Struct { members, .. } = &ty.inner else { panic!("{name}: a struct") };
    let names = members.iter().filter_map(|m| m.name.clone()).collect();
    let mut layouter = naga::proc::Layouter::default();
    layouter.update(module.to_ctx()).expect("layout");
    (names, u64::from(layouter[handle].size))
}

/// Patch LSO, A3 key split (`docs/specs/aerial-shadow.md` "Architecture review" item 1): the aerial
/// view LUT links and validates in all four variants (stock, AS, key split, AS + key split). The
/// stock and AS passes keep their bindings and link nothing of the split; the split variants add
/// the light shafts' uniform (17, `GpuAtmosphereLightShafts` layout) and the key LUT (18), and with
/// AS the key light's single scattering carries the same height shadow as the total.
#[test]
fn aerial_view_lut_compiles_with_and_without_the_key_split() {
    use bevy_render::render_resource::ShaderType;
    let hs = bevy_pbr::ATMOSPHERE_HEIGHT_SHADOW_DEF;
    let key = bevy_pbr::ATMOSPHERE_KEY_SINGLE_DEF;
    let mut failures = Vec::new();
    for (defs, has_hs, has_key) in [(&[][..], false, false), (&[hs][..], true, false), (&[key][..], false, true), (&[hs, key][..], true, true)] {
        let wgsl = match compile("bevy_pbr::atmosphere::aerial_view_lut", defs) {
            Ok(wgsl) => wgsl,
            Err(e) => {
                failures.push(e);
                continue;
            }
        };
        let mut expected: Vec<u32> = vec![0, 1, 3, 4, 5, 6, 7, 8, 9, 12, 13];
        if has_hs {
            expected.extend([14, 15, 16]);
        }
        if has_key {
            expected.extend([17, 18]);
        }
        expected.sort_unstable();
        if bindings(&wgsl) != expected {
            failures.push(format!("{defs:?}: bindings {:?}, expected {expected:?}", bindings(&wgsl)));
        }
        let split = wgsl.contains("aerial_view_key_lut_out") || wgsl.contains("sample_key_single_scattering") || wgsl.contains("light_shafts");
        if split != has_key {
            failures.push(format!("{defs:?}: key split linked = {split}"));
        }
        if has_key {
            let (names, size) = struct_layout(&wgsl, "AtmosphereLightShafts");
            if names != ["light_dir", "darkness", "lobe_g"] || size != bevy_pbr::GpuAtmosphereLightShafts::min_size().get() {
                failures.push(format!("{defs:?}: AtmosphereLightShafts {names:?} {size}"));
            }
            // The key single scattering keeps E1b's per-light scale, like the total.
            if !wgsl.contains(".atmosphere_scatter") {
                failures.push(format!("{defs:?}: no per-light scale"));
            }
        }
        // With AS the key light's term is shadowed like the total's: both calls read the light.
        if has_hs && has_key && wgsl.matches("height_shadow.light_dir").count() < 2 {
            failures.push(format!("{defs:?}: the key LUT does not carry the height shadow"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Patch LSO, A3 key split: `render_sky` without the def is the stock pass (bindings 0-13, nothing
/// of the shafts linked) in every MSAA / dual-source variant; with it, it adds the occlusion (14,
/// 15) and the key LUT (16) and masks only the key light's single scattering (no whole-in-scatter
/// multiply left in the source).
#[test]
fn render_sky_masks_only_the_key_single_scattering() {
    let def = bevy_pbr::ATMOSPHERE_LIGHT_SHAFTS_DEF;
    let mut failures = Vec::new();
    for base in [&[][..], &["MULTISAMPLED"][..], &["DUAL_SOURCE_BLENDING"][..], &["DUAL_SOURCE_BLENDING", "MULTISAMPLED"][..]] {
        match compile("bevy_pbr::atmosphere::render_sky", base) {
            Ok(wgsl) => {
                if wgsl.contains("light_shaft") || wgsl.contains("aerial_view_key_lut") || wgsl.contains("mask_key_single") {
                    failures.push(format!("{base:?}: stock pass links the shafts"));
                }
                if bindings(&wgsl) != (0..=13).collect::<Vec<u32>>() {
                    failures.push(format!("{base:?}: stock bindings {:?}", bindings(&wgsl)));
                }
            }
            Err(e) => failures.push(e),
        }
        let defs: Vec<&str> = base.iter().copied().chain([def]).collect();
        match compile("bevy_pbr::atmosphere::render_sky", &defs) {
            Ok(wgsl) => {
                if !(wgsl.contains("mask_key_single(") && wgsl.contains("sample_aerial_view_key_lut(") && wgsl.contains("light_shaft_occlusion(")) {
                    failures.push(format!("{defs:?}: no key split mask"));
                }
                if bindings(&wgsl) != (0..=16).collect::<Vec<u32>>() {
                    failures.push(format!("{defs:?}: bindings {:?}", bindings(&wgsl)));
                }
            }
            Err(e) => failures.push(e),
        }
    }
    let source = std::fs::read_to_string(vendor("bevy_pbr").join("atmosphere/render_sky.wesl")).expect("render_sky.wesl");
    assert!(!source.contains("*= light_shaft_occlusion"), "the whole in-scatter is no longer multiplied");
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// CPU twin of `render_sky.wesl` `mask_key_single` (the WGSL line is checked verbatim below).
fn mask_key_single(total: f32, key_single: f32, occlusion: f32) -> f32 {
    total - (1.0 - occlusion.clamp(0.0, 1.0)) * key_single.min(total)
}

/// CPU twin of `render_sky.wesl` `light_shaft_occlusion` for a texel `open` (lines checked verbatim).
fn light_shaft_occlusion(open: f32, cos_t: f32, darkness: f32, lobe_g: f32) -> f32 {
    if darkness <= 0.0 {
        return 1.0;
    }
    let g = lobe_g.clamp(-0.999, 0.999);
    let cos_t = cos_t.clamp(-1.0, 1.0);
    let lobe = ((1.0 - g) * (1.0 - g) / ((1.0 - g) * (1.0 - g) + 2.0 * g * (1.0 - cos_t)).max(1e-6)).powf(1.5).min(1.0);
    (1.0 - darkness.clamp(0.0, 1.0) * (1.0 - open.clamp(0.0, 1.0)) * lobe).clamp(0.0, 1.0)
}

/// Patch LSO, A3 key split: the mask never takes the in-scattering below the unmasked rest
/// `max(total - key, 0)` nor above `total`; it is `total` when open and the rest when closed. The
/// occlusion is exactly 1 at `lobe_g` 0 on an open texel and `1 - darkness` on a closed one (so
/// A2's per-pixel visibility passes through). The twins match the WGSL lines verbatim.
#[test]
fn key_split_mask_keeps_the_unmasked_rest() {
    let source = std::fs::read_to_string(vendor("bevy_pbr").join("atmosphere/render_sky.wesl")).expect("render_sky.wesl");
    for line in [
        "return total - (1.0 - clamp(occlusion, 0.0, 1.0)) * min(key_single, total);",
        "let g = clamp(light_shafts.lobe_g, -0.999, 0.999);",
        "let lobe = min(pow((1.0 - g) * (1.0 - g) / max((1.0 - g) * (1.0 - g) + 2.0 * g * (1.0 - cos_t), 1e-6), 1.5), 1.0);",
        "return clamp(1.0 - clamp(light_shafts.darkness, 0.0, 1.0) * (1.0 - clamp(open, 0.0, 1.0)) * lobe, 0.0, 1.0);",
    ] {
        assert!(source.contains(line), "render_sky.wesl no longer has the twin's line `{line}`");
    }
    let values = [0.0_f32, 1e-6, 0.01, 0.3, 1.0, 7.5, 1e4];
    for total in values {
        for key in values {
            let eps = 1e-6 * total.max(1.0);
            for occ in [-0.5_f32, 0.0, 0.25, 0.5, 0.99, 1.0, 1.5] {
                let masked = mask_key_single(total, key, occ);
                let rest = (total - key).max(0.0);
                assert!(masked >= rest - eps && masked <= total + eps, "total {total} key {key} occ {occ}: {masked} outside [{rest}, {total}]");
            }
            assert!((mask_key_single(total, key, 1.0) - total).abs() <= eps, "open = total");
            assert!((mask_key_single(total, key, 0.0) - (total - key.min(total))).abs() <= eps, "closed = the rest");
        }
    }
    for cos_t in [-1.0_f32, -0.3, 0.0, 0.7, 1.0] {
        for darkness in [0.0_f32, 0.4, 1.0] {
            assert!((light_shaft_occlusion(1.0, cos_t, darkness, 0.0) - 1.0).abs() < 1e-6, "open texel at g 0 = 1");
            assert!((light_shaft_occlusion(0.0, cos_t, darkness, 0.0) - (1.0 - darkness)).abs() < 1e-6, "closed texel at g 0 = 1 - darkness");
        }
    }
}
