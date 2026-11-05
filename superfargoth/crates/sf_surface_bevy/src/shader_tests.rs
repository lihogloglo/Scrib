//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks"): the real
//! modules are linked by `wesl` with each preset's shader defs, as Bevy's shader cache does
//! (`bevy_shader` `shader_cache.rs`), and validated by naga. Covered: the `StandardMaterial`
//! fragment of the vendored `bevy_pbr` (patch "SW" calling `sf_surface_bevy::view`), the
//! terrain main pass and the far proxies and impostors, with and without `SURFACE_WEATHER`.
//! Plus the binding numbers against the patch and the wet and puddle constants against the CPU
//! twin.

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

/// Module paths to files: the vendored Bevy crates and the workspace crates.
struct Files {
    constants: String,
}

impl Files {
    fn root(package: &str) -> Option<PathBuf> {
        let crates = PathBuf::from(CRATE).join("..");
        let vendor = crates.join("../vendor");
        match package {
            "bevy_pbr" | "bevy_render" | "bevy_core_pipeline" => Some(vendor.join(package).join("src")),
            "sf_surface_bevy" | "sf_terrain_bevy" | "sf_lod_bevy" | "sf_render_order_bevy" => Some(crates.join(package).join("src")),
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
            return Ok(Cow::Borrowed(&self.constants));
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
pub(crate) fn compile(module: &str, defs: &[&str]) -> Result<String, String> {
    compile_with(module, defs, true)
}

/// [`compile`], optionally without dead-code stripping (every declaration of the root module
/// is kept and validated, bindings no entry point reads yet included).
fn compile_with(module: &str, defs: &[&str], strip: bool) -> Result<String, String> {
    let mut parts = module.split("::");
    let path = ModulePath {
        origin: PathOrigin::Package(parts.next().unwrap().to_string()),
        components: parts.map(str::to_string).collect(),
    };
    let mut options = wesl::CompileOptions {
        imports: true,
        condcomp: true,
        visibility: false,
        strip,
        ..Default::default()
    };
    for d in defs {
        options.features.flags.insert((*d).to_string(), true.into());
    }
    let files = Files { constants: CONSTANTS.to_string() };
    let wgsl = wesl::compile(&path, &options, &files)
        .map_err(|e| format!("{module} {defs:?}: link: {}", e.diagnostic().render_plain()))?
        .to_string();
    let parsed = naga::front::wgsl::parse_str(&wgsl)
        .map_err(|e| format!("{module} {defs:?}: parse: {}", e.emit_to_string(&wgsl)))?;
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&parsed)
        .map_err(|e| format!("{module} {defs:?}: validate: {}", e.emit_to_string(&wgsl)))?;
    Ok(wgsl)
}

/// The forward main pass defs per preset (`bevyrrowind` Low / High / Ultra shapes: Low has no
/// SSAO, contact shadows or prepasses; High adds them, atmosphere and fog; Ultra the GI probes).
pub(crate) const LOW: &[&str] = &[
    "VERTEX_POSITIONS",
    "VERTEX_NORMALS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_TANGENTS",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
];
pub(crate) const HIGH: &[&str] = &[
    "VERTEX_POSITIONS",
    "VERTEX_NORMALS",
    "VERTEX_UVS",
    "VERTEX_UVS_A",
    "VERTEX_TANGENTS",
    "VERTEX_OUTPUT_INSTANCE_INDEX",
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

pub(crate) fn with(base: &[&'static str], extra: &[&'static str]) -> Vec<&'static str> {
    base.iter().chain(extra).copied().collect()
}

#[test]
fn shaders_compile_per_preset_with_and_without_surface_weather() {
    let mut failures = Vec::new();
    for (preset, base) in [("low", with(LOW, &[])), ("high", with(HIGH, &[])), ("ultra", with(HIGH, &["GI_PROBES"]))] {
        for sw in [false, true] {
            let defs = |extra: &[&'static str]| {
                let mut d = with(&base, extra);
                if sw {
                    d.push("SURFACE_WEATHER");
                }
                d
            };
            let jobs: [(&str, Vec<&str>); 5] = [
                ("bevy_pbr::render::pbr", defs(&[])),
                ("bevy_pbr::render::pbr", defs(&["SKINNED"])),
                ("sf_terrain_bevy::shaders::terrain", defs(&[])),
                ("sf_lod_bevy::shaders::proxy", defs(&[])),
                ("sf_lod_bevy::shaders::impostor", defs(&[])),
            ];
            for (module, d) in jobs {
                match compile(module, &d) {
                    // The layer is linked in exactly when the def is on.
                    Ok(wgsl) if wgsl.contains("apply_surface_layer") != sw => {
                        failures.push(format!("[{preset}] {module} {d:?}: layer linked = {}", !sw));
                    }
                    // Puddles and rings (W4) come with the layer; their level gate is the true
                    // surface: the terrain's height field, a mesh's face normal.
                    Ok(wgsl)
                        if sw
                            && (!wgsl.contains("surface_ripples")
                                || (module.contains("terrain") && !wgsl.contains("terrain_ground"))
                                || (module.contains("pbr") && !wgsl.contains("surface_face_up"))) =>
                    {
                        failures.push(format!("[{preset}] {module} {d:?}: puddles not linked"));
                    }
                    // Cover (W5b) samples the layer array with the caller's derivatives.
                    Ok(wgsl) if sw && !wgsl.contains("surface_apply_cover") => {
                        failures.push(format!("[{preset}] {module} {d:?}: cover not linked"));
                    }
                    Ok(_) => {}
                    Err(e) => failures.push(format!("[{preset}] {e}")),
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The view module on its own, nothing stripped: every binding (the W5a cover array and its
/// sampler too, read by the cover since W5b) is declared with a type naga accepts, and
/// the module links with and without the def set of a lit pass.
#[test]
fn view_module_declares_every_binding() {
    for defs in [with(HIGH, &[]), with(HIGH, &["SURFACE_WEATHER"])] {
        let wgsl = compile_with("sf_surface_bevy::view", &defs, false).unwrap_or_else(|e| panic!("{e}"));
        for decl in ["var surface_cover: texture_2d_array<f32>", "var surface_cover_sampler: sampler", "var surface_trail: texture_2d<f32>", "var surface_trail_previous: texture_2d<f32>"] {
            assert!(wgsl.contains(decl), "{decl} missing:\n{wgsl}");
        }
    }
}

/// `view.wesl`'s bindings are the ones the `bevy_pbr` patch puts in the mesh view layout.
#[test]
fn view_bindings_match_the_bevy_pbr_patch() {
    let wgsl = include_str!("view.wesl");
    let binding = |name: &str| -> u32 {
        let line = wgsl
            .lines()
            .find(|l| l.starts_with("@group(0) @binding(") && l.contains(&format!(" {name}:")))
            .unwrap_or_else(|| panic!("{name}"));
        line["@group(0) @binding(".len()..].split(')').next().unwrap().parse().unwrap()
    };
    assert_eq!(
        [
            binding("surface_uniform"),
            binding("surface_cells"),
            binding("surface_shelter"),
            binding("surface_noise"),
            binding("surface_sampler"),
            binding("surface_shelter_sampler"),
            binding("surface_canopy"),
            binding("surface_cover"),
            binding("surface_cover_sampler"),
            binding("surface_trail"),
            binding("surface_trail_previous"),
        ],
        [
            bevy_pbr::SURFACE_WEATHER_UNIFORM_BINDING,
            bevy_pbr::SURFACE_WEATHER_CELLS_BINDING,
            bevy_pbr::SURFACE_WEATHER_SHELTER_BINDING,
            bevy_pbr::SURFACE_WEATHER_NOISE_BINDING,
            bevy_pbr::SURFACE_WEATHER_SAMPLER_BINDING,
            bevy_pbr::SURFACE_WEATHER_SHELTER_SAMPLER_BINDING,
            bevy_pbr::SURFACE_WEATHER_CANOPY_BINDING,
            bevy_pbr::SURFACE_WEATHER_COVER_BINDING,
            bevy_pbr::SURFACE_WEATHER_COVER_SAMPLER_BINDING,
            bevy_pbr::SURFACE_WEATHER_TRAIL_BINDING,
            bevy_pbr::SURFACE_WEATHER_TRAIL_PREVIOUS_BINDING,
        ]
    );
}

/// The wet constants in `layer.wesl` are the CPU twin's (`sf_surface::layer`).
#[test]
fn wet_constants_match_the_cpu_twin() {
    let wgsl = include_str!("layer.wesl");
    for (name, value) in sf_surface::layer::SHADER_CONSTANTS {
        let line = wgsl
            .lines()
            .find(|l| l.starts_with(&format!("const {name}: f32 = ")))
            .unwrap_or_else(|| panic!("{name} in layer.wesl"));
        let v: f32 = line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap();
        assert!((v - value).abs() < 1e-6, "{name}: wgsl {v}, rust {value}");
    }
    assert!(
        wgsl.contains("return (1.0 - metallic) * saturate((perceptual_roughness - POROSITY_ROUGH_LOW) / POROSITY_ROUGH_SPAN);"),
        "the porosity formula is sf_surface::layer::porosity's"
    );
}

/// The trail ring pass (W6a) parses and validates as naga reads it, and its uniform is the
/// size `sf_surface::trail::TrailPass::slots` packs (the stamp array's length is `MAX_STAMPS`).
#[test]
fn trail_pass_compiles_and_matches_the_cpu_twin() {
    let src = include_str!("trail.wgsl");
    let module = naga::front::wgsl::parse_str(src).unwrap_or_else(|e| panic!("{}", e.emit_to_string(src)));
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&module)
        .unwrap_or_else(|e| panic!("{e:?}"));
    let params = module.types.iter().find(|(_, t)| t.name.as_deref() == Some("TrailParams")).expect("TrailParams");
    let naga::TypeInner::Struct { span, .. } = params.1.inner else { panic!("TrailParams is a struct") };
    assert_eq!(span as usize, sf_surface::trail::PARAM_SLOTS * 16);
    assert!(src.contains(&format!("stamps: array<Stamp, {}>", sf_surface::trail::MAX_STAMPS)));
    // The layer's lip height is the twin's (the debug view scales by it).
    let layer = include_str!("layer.wesl");
    assert!(layer.contains(&format!("const TRAIL_LIP_HEIGHT: f32 = {:?};", sf_surface::trail::LIP_HEIGHT)));
}

/// W6b: the terrain's main pass and prepass displace by one function (`common::terrain_raise`
/// over `view::surface_raise_at`), linked exactly when their surface weather def is on, so depth
/// and motion vectors match the drawn surface; shadow views (no def) stay undisplaced.
#[test]
fn terrain_passes_share_one_displacement() {
    fn body(wgsl: &str) -> Option<&str> {
        // The linker may mangle the name: the first `fn` line naming it.
        let start = wgsl.match_indices("fn ").map(|(i, _)| i).find(|&i| wgsl[i..].lines().next().is_some_and(|l| l.contains("terrain_raise(")))?;
        let end = start + wgsl[start..].find("\n}\n")?;
        Some(&wgsl[start..end])
    }
    let prepass: &[&'static str] = &["DEPTH_PREPASS", "MOTION_VECTOR_PREPASS", "NORMAL_PREPASS", "PREPASS_FRAGMENT"];
    let main = compile("sf_terrain_bevy::shaders::terrain", &with(HIGH, &["SURFACE_WEATHER"])).unwrap_or_else(|e| panic!("{e}"));
    let pre = compile("sf_terrain_bevy::shaders::terrain_prepass", &with(prepass, &["SURFACE_WEATHER_PREPASS"]))
        .unwrap_or_else(|e| panic!("{e}"));
    let (a, b) = (body(&main).expect("main pass displaces"), body(&pre).expect("prepass displaces"));
    assert_eq!(a, b, "one displacement function");
    assert!(pre.contains("previous_world_position"), "motion vectors from last frame's surface");
    for (module, defs) in [
        ("sf_terrain_bevy::shaders::terrain", with(HIGH, &[])),
        ("sf_terrain_bevy::shaders::terrain_prepass", with(prepass, &[])),
        ("sf_terrain_bevy::shaders::terrain_prepass", vec!["DEPTH_PREPASS", "UNCLIPPED_DEPTH_ORTHO_EMULATION"]),
    ] {
        let wgsl = compile(module, &defs).unwrap_or_else(|e| panic!("{e}"));
        assert!(body(&wgsl).is_none(), "{module} {defs:?}: displaced without surface weather");
    }
}
