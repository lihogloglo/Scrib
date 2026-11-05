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
