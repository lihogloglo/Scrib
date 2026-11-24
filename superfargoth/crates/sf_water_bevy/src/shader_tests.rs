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
            // The shared rain rings (R1).
            "sf_render_order_bevy" => Some(crates.join("sf_render_order_bevy").join("src")),
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
/// snapshot, velocity and SSPR passes, the water draw with and without an environment map
/// (its SSPR composition recomputes the environment specular only under `ENVIRONMENT_MAP`), and the
/// underwater post (U3) with and without MSAA and dual-source blending.
#[test]
fn water_shaders_compile() {
    let mut failures = Vec::new();
    let jobs: [(&str, &[&str]); 16] = [
        ("sf_water_bevy::shaders::water_gate", &[]),
        ("sf_water_bevy::shaders::water_fft", &[]),
        ("sf_water_bevy::shaders::water_mips", &[]),
        ("sf_water_bevy::shaders::water_snapshot", &[]),
        ("sf_water_bevy::shaders::water_snapshot", &["MULTISAMPLED"]),
        ("sf_water_bevy::shaders::water_velocity", &[]),
        ("sf_water_bevy::shaders::water_velocity", &["MULTISAMPLED"]),
        ("sf_water_bevy::shaders::water_sspr", &[]),
        ("sf_water_bevy::shaders::water_material", &[]),
        ("sf_water_bevy::shaders::water_material", &["ENVIRONMENT_MAP"]),
        ("sf_water_bevy::shaders::water_material", &["WATER_MESH"]),
        ("sf_water_bevy::shaders::water_material", &["WATER_MESH", "WATER_FLOW", "ENVIRONMENT_MAP"]),
        ("sf_water_bevy::shaders::water_underwater", &[]),
        ("sf_water_bevy::shaders::water_underwater", &["MULTISAMPLED"]),
        ("sf_water_bevy::shaders::water_underwater", &["DUAL_SOURCE_BLENDING"]),
        ("sf_water_bevy::shaders::water_underwater", &["MULTISAMPLED", "DUAL_SOURCE_BLENDING"]),
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

/// Mesh bodies (`WaterMeshSurface`): the water draw links on the `WATER_MESH` path (the def the
/// pipeline key pushes) with and without an environment map; that vertex stage places the host's
/// vertices through the mesh transform (`get_world_from_local` of the instance) and never the
/// clipmap, while the default path keeps the clipmap and no mesh transform; both share the uniform.
#[test]
fn mesh_body_path_links_and_skips_the_clipmap() {
    use bevy_render::render_resource::ShaderType;
    let def = crate::material::WATER_MESH_DEF;
    for defs in [&[def][..], &[def, "ENVIRONMENT_MAP"][..]] {
        compile("sf_water_bevy::shaders::water_material", defs).unwrap_or_else(|e| panic!("{e}"));
    }
    let mesh = compile("sf_water_bevy::shaders::water_material", &[def]).expect("compiles");
    let clipmap = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    assert!(mesh.contains("get_world_from_local") && mesh.contains("instance_index"), "the mesh path ignores the entity's transform");
    assert!(!mesh.contains("clipmap_xz") && !mesh.contains("fn displaced"), "the mesh path still places or displaces the clipmap");
    assert!(clipmap.contains("clipmap_xz") && !clipmap.contains("get_world_from_local"), "the default path changed");
    let module = naga::front::wgsl::parse_str(&mesh).expect("parses");
    assert!(module.entry_points.iter().any(|e| e.name == "vertex") && module.entry_points.iter().any(|e| e.name == "fragment"));
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("reflection")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::WaterParams::min_size().get()), "WaterParams layout on the mesh path");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_material.wesl")).expect("source");
    assert!(src.contains(&format!("@if({def})")) && src.contains(&format!("@if(!{def})")), "the shader's def is not the key's");
}

/// River flow (`flow.rs`): the water draw links on the flowing mesh path (`WATER_MESH` +
/// `WATER_FLOW`, as `material::specialize_water` pushes them) with and without an environment map;
/// its vertex stage reads the flow at the Rust side's `FLOW_LOCATION` and passes it on; the
/// fragment samples the slopes at two advected phases (`flow_phases`, the twin's formula) and adds
/// speed foam. Without `WATER_FLOW` (lakes, the clipmap) none of it is linked; the uniform layout holds.
#[test]
fn flow_path_links_reads_the_attribute_and_leaves_still_water_alone() {
    use bevy_render::render_resource::ShaderType;
    let (mesh_def, flow_def) = (crate::material::WATER_MESH_DEF, crate::material::WATER_FLOW_DEF);
    for defs in [&[mesh_def, flow_def][..], &[mesh_def, flow_def, "ENVIRONMENT_MAP"][..]] {
        compile("sf_water_bevy::shaders::water_material", defs).unwrap_or_else(|e| panic!("{e}"));
    }
    let river = compile("sf_water_bevy::shaders::water_material", &[mesh_def, flow_def]).expect("compiles");
    let lake = compile("sf_water_bevy::shaders::water_material", &[mesh_def]).expect("compiles");
    let ocean = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    for name in ["flow_phases", "water.flow.", "water.flow2.", "water.flow3.", "sw_noise"] {
        assert!(river.contains(name), "no {name} on the river path");
    }
    for (path, wgsl) in [("lake", &lake), ("ocean", &ocean)] {
        assert!(!wgsl.contains("flow_phases") && !wgsl.contains("water.flow"), "the {path} path reads the flow");
    }
    let module = naga::front::wgsl::parse_str(&river).expect("parses");
    let location = |stage: naga::ShaderStage, member: &str, output: bool| -> Option<u32> {
        let entry = module.entry_points.iter().find(|e| e.stage == stage)?;
        let ty = if output { entry.function.result.as_ref()?.ty } else { entry.function.arguments.first()?.ty };
        match &module.types[ty].inner {
            naga::TypeInner::Struct { members, .. } => members.iter().find(|m| m.name.as_deref() == Some(member)).and_then(|m| match m.binding {
                Some(naga::Binding::Location { location, .. }) => Some(location),
                _ => None,
            }),
            _ => None,
        }
    };
    assert_eq!(location(naga::ShaderStage::Vertex, "flow", false), Some(crate::material::FLOW_LOCATION), "the flow attribute's location");
    assert!(location(naga::ShaderStage::Vertex, "flow", true).is_some(), "the vertex stage does not pass the flow on");
    assert!(location(naga::ShaderStage::Fragment, "flow", false).is_some(), "the fragment does not receive the flow");
    let lake_module = naga::front::wgsl::parse_str(&lake).expect("parses");
    let vertex = lake_module.entry_points.iter().find(|e| e.stage == naga::ShaderStage::Vertex).expect("vertex");
    let ty = vertex.function.arguments.first().expect("vertex input").ty;
    let naga::TypeInner::Struct { members, .. } = &lake_module.types[ty].inner else { panic!("vertex input is a struct") };
    assert_eq!(members.len(), 1, "a still mesh's vertex reads more than its position");
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("reflection")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::WaterParams::min_size().get()), "WaterParams layout on the river path");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_material.wesl")).expect("source");
    // The twin's formula (`flow::phases`), and the still path's single tap at the point.
    assert!(src.contains("let b = fract(t + 0.5);") && src.contains("return vec4(a, b, 1.0 - abs(1.0 - 2.0 * a), 1.0 - abs(1.0 - 2.0 * b));"), "flow_phases is not the twin's");
    assert!(src.contains("var taps = 1u;") && src.contains("var weights = vec2(1.0, 0.0);"), "still water no longer reads once at the point");
    assert!(src.contains(&format!("@location({}) flow: vec2<f32>", crate::material::FLOW_LOCATION)), "the attribute's location is not FLOW_LOCATION");
    let advect = src.find("for (var k = 0u; k < taps; k++)").expect("the advected taps");
    let normal = src.find("var N = normalize(").expect("normal");
    assert!(advect < normal, "the advected slopes must build the normal");
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

/// WA-6: the shore foam band is gated by the local wave energy (`shore_foam_weight`, the CPU twin's
/// `sf_water::shore_foam_weight`), no longer by a floored openness that left it on calm water.
#[test]
fn shore_foam_follows_the_wave_energy() {
    let material = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    assert!(material.contains("shore_foam_weight") && material.contains("breaking"), "the shore foam is not gated by the waves");
    assert!(material.contains("shore3"), "no foam reference in the uniform");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_material.wesl")).expect("source");
    assert!(!src.contains("0.3 + 0.7 * sh.y"), "the old openness floor is back");
}

/// SF-1: the breaker foam runs behind `shore3.y` (`--off surffoam` = 0), which keeps the WA-6 band
/// and the constant contact foam of before verbatim (the A/B arm); on, the foam reads the shore
/// waves' phase and swash and the cellular lace. The CPU twin's names are the WGSL's.
#[test]
fn surf_foam_switch_keeps_the_old_path() {
    let material = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    for name in ["surf_foam", "sw_cells", "crest_cos", "shore3.y", "surf5"] {
        assert!(material.contains(name), "no {name} in the water draw");
    }
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_material.wesl")).expect("source");
    assert!(src.contains("let surf_on = water.shore3.y > 0.5;"), "no switch");
    // The old terms, gated by the switch, unchanged.
    assert!(src.contains("if !(surf_on && trains) && sh.x < band && look.shore_foam > 0.0 {"), "old band not kept for the off arm");
    assert!(src.contains("if !surf_on && valid && path < 1e5 {"), "old contact foam not kept for the off arm");
    assert!(src.contains("foam = max(foam, 0.7 * (1.0 - smoothstep(0.0, look.contact, vertical)) * step(1e-3, look.contact));"), "old contact foam changed");
    assert!(src.contains("var foam_rgb = vec3(0.8);") && src.contains("mix(body, foam_rgb, foam)"), "the off arm's foam colour changed");
    let common = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_common.wesl")).expect("source");
    for name in ["fn surf_birth(", "fn bore_profile(", "fn behind_crest(", "fn foam_lace(", "fn swash_edge(", "fn surf_foam("] {
        assert!(common.contains(name), "no {name} twin of sf_water::{}", name.trim_start_matches("fn ").trim_end_matches('('));
    }
}

/// SF-1: the lace cell size becomes whole cells per noise period (the cellular lace tiles with the
/// floating origin's wrap), clamped for a zero size.
#[test]
#[allow(clippy::float_cmp)] // exact: whole cells, and the clamp
fn surf_foam_reaches_the_uniform() {
    let on = sf_water::SurfFoam::default();
    assert!(on.enabled);
    let cells = crate::lace_cells(on.lace_cell_m);
    assert_eq!(cells.fract(), 0.0);
    assert!(cells >= 1.0 && (1024.0 / cells - on.lace_cell_m).abs() < 0.01, "{cells}");
    assert_eq!(crate::lace_cells(0.0), crate::lace_cells(0.05), "clamped");
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

/// The visibility gate (CW-10a): the probe and argument entry points exist, and `gate_args` writes
/// the draw/dispatch layouts at the byte offsets the passes read (`GATE_ARGS_*`): instance count,
/// velocity instances and both SSPR group counts scale with the occlusion result.
#[test]
fn gate_shader_matches_its_rust_side() {
    let wgsl = compile("sf_water_bevy::shaders::water_gate", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    for name in ["probe", "gate_args"] {
        assert!(module.entry_points.iter().any(|e| e.name == name), "entry point {name}");
    }
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/shaders/water_gate.wesl")).expect("shader");
    let at = |offset: u64| offset / 4;
    let (s, v, r) = (at(crate::render::GATE_ARGS_SNAPSHOT), at(crate::render::GATE_ARGS_VELOCITY), at(crate::render::GATE_ARGS_SSPR));
    for line in [
        format!("args[{s}] = 3u;"),
        format!("args[{}] = k;", s + 1),
        format!("args[{v}] = gate.counts.x;"),
        format!("args[{}] = gate.counts.y * k;", v + 1),
        format!("args[{r}] = gate.counts.z * k;"),
        format!("args[{}] = gate.counts.w * k;", r + 1),
        format!("args[{}] = k;", r + 2),
    ] {
        assert!(src.contains(&line), "water_gate.wesl lacks `{line}`");
    }
}

/// U3: the underwater post's `Underwater` struct keeps the size of its Rust mirror, its fragment
/// entry point is the pipeline's, its extinction floor is `sf_water`'s, and it lights its
/// in-scatter at the view's exposure (the scene it blends onto is pre-exposed).
#[test]
#[allow(clippy::float_cmp)] // exact: the WGSL literal must parse to the very constant
fn underwater_post_matches_its_rust_side() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("sf_water_bevy::shaders::water_underwater", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    assert!(module.entry_points.iter().any(|e| e.name == "underwater"), "entry point underwater");
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("sun")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::UnderwaterUniform::min_size().get()), "Underwater layout");
    assert!(wgsl.contains("view.exposure"), "the in-scatter is not exposed like the scene");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_underwater.wesl")).expect("source");
    let line = src.lines().find(|l| l.starts_with("const MIN_EXTINCTION = ") || l.starts_with("const MIN_EXTINCTION: f32 = ")).expect("const MIN_EXTINCTION");
    let value: f32 = line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap_or_else(|_| panic!("{line}"));
    assert_eq!(value, sf_water::underwater::MIN_EXTINCTION);
    // An empty depth (reverse Z 0: nothing hit) is full extinction, never a distance.
    assert!(src.contains("var transmittance = vec3(0.0);") && src.contains("if depth > 0.0 {"), "infinite depth is not the medium colour");
    // Dual-source blending writes a per-channel transmittance; without it the mean goes in alpha.
    let dual = compile("sf_water_bevy::shaders::water_underwater", &["DUAL_SOURCE_BLENDING"]).expect("compiles");
    assert!(dual.contains("blend_src"), "no dual-source output");
    assert!(!wgsl.contains("blend_src"), "dual-source output without the feature");
}

/// U3: the post draws only for a camera fully under the water (uniform state 2) with the post on:
/// never above (0) or split (1, the waterline's, U4), never with `--off underwater`. The extract
/// gives the post its uniform and the sky gate (`SkyOccluded`) through that one test, the pass runs
/// only on a view with the bindings the prepare makes from that uniform, and the vendored sky and
/// the `sf_atmos` composite skip a view with the gate.
#[test]
fn underwater_post_is_gated_on_state_two() {
    use crate::{post_on, WaterMedium, WaterMediumSettings, WaterMediumUniform};
    use sf_water::MediumState;
    let on = WaterMediumSettings::default();
    assert!(on.post, "the post is on by default");
    let off = WaterMediumSettings { post: false, ..on };
    for (state, code) in [(MediumState::Above, 0), (MediumState::Split, 1), (MediumState::Under, 2)] {
        let mut m = WaterMedium::default();
        m.medium.state = state;
        let u = WaterMediumUniform::from(&m);
        assert_eq!(u.state, code);
        assert_eq!(post_on(&on, &u), code == 2, "{state:?}");
        assert!(!post_on(&off, &u), "{state:?} with --off underwater");
    }
    let medium = std::fs::read_to_string(PathBuf::from(CRATE).join("src/medium.rs")).expect("source");
    assert!(medium.contains("settings.post && medium.state == MediumState::Under.as_u32()"), "post_on changed");
    assert!(medium.contains("if post_on(&settings, &uniform) {"), "the extract does not gate on post_on");
    assert!(medium.contains("e.insert((UnderwaterUniform::new(m, &look, sun, ambient, &settings), SkyOccluded));"), "uniform and sky gate not inserted together");
    assert!(medium.contains("e.remove::<(UnderwaterUniform, SkyOccluded, crate::render::UnderwaterBindings)>();"), "the post is not removed when the camera surfaces");
    let render = std::fs::read_to_string(PathBuf::from(CRATE).join("src/render.rs")).expect("source");
    assert!(render.contains("view: ViewQuery<(&UnderwaterBindings,"), "the pass runs without the bindings");
    assert!(render.contains("views: Query<(Entity, &UnderwaterUniform,"), "the bindings are made without the uniform");
    assert!(render.contains("PassStage::Medium, underwater_pass"), "the pass is not at step 8b");
    assert!(render.contains("pass_span(&mut pass, \"water_underwater\")"), "no GPU timer");
    let vendor = PathBuf::from(CRATE).join("../../vendor/bevy_pbr/src/atmosphere");
    let sky = std::fs::read_to_string(vendor.join("node.rs")).expect("vendored render_sky");
    assert!(sky.contains("bevy_ecs::query::Without<super::SkyOccluded>"), "the vendored render_sky ignores the gate");
    let atmos = std::fs::read_to_string(PathBuf::from(CRATE).join("../sf_atmos_bevy/src/render.rs")).expect("sf_atmos render");
    assert!(atmos.contains("AtmosphereOffsets), Without<bevy_pbr::SkyOccluded>>"), "the composite ignores the gate");
}

/// R1: the rain rings are the puddles' (`sf_render_order_bevy::rain_rings`, linked into the water
/// draw), added to the slopes before the normal is built, only on the front face and only while
/// the rain slot is on (`rain.x`: the host's intensity, 0 dry, indoors and with `--off water_rain`),
/// faded by the pixel footprint; no rain is no slots; the clock wraps on whole ring cycles.
#[test]
fn rain_rings_are_gated_by_the_switch_and_intensity() {
    let material = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    assert!(material.contains("rain_ripples"), "the rain rings are not linked into the water draw");
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_material.wesl")).expect("source");
    assert!(src.contains("import sf_render_order_bevy::rain_rings::rain_ripples;"), "not the shared rings");
    assert!(src.contains("if is_front && water.rain.x > 0.0 {"), "rings not gated by the rain and the front face");
    assert!(src.contains("let rings = water.rain.z * (1.0 - smoothstep(water.rain2.x, water.rain2.y, footprint));"), "rings not faded by the footprint");
    assert!(src.contains("rain_ripples(in.world_position.xz, water.rain.y, water.rain.x) * rings;"), "rings not scaled by the rain");
    let term = src.find("rain_ripples(in.world_position").expect("ring term");
    let normal = src.find("var N = normalize(").expect("normal");
    assert!(term < normal, "the rings must join the slopes before the normal is built");
    // Off (no rain, or `--off water_rain` from the host): the slots are zero, the branch is skipped.
    let settings = crate::WaterRainSettings::default();
    assert_eq!(crate::rain::slots(5.0, crate::WaterRain { intensity: 0.0 }, settings), [bevy_math::Vec4::ZERO; 2]);
    assert!(crate::rain::slots(5.0, crate::WaterRain { intensity: 0.5 }, settings)[0].x > 0.0);
    // The ring clock wraps on whole cycles of the shared rings' drop rate (no pop at the wrap).
    let rings = std::fs::read_to_string(PathBuf::from(CRATE).join("../sf_render_order_bevy/src/rain_rings.wesl")).expect("shared rings");
    let rate: f64 = rings
        .lines()
        .find_map(|l| l.strip_prefix("const RIPPLE_RATE: f32 = "))
        .expect("RIPPLE_RATE in rain_rings.wesl")
        .trim_end_matches(';')
        .parse()
        .expect("a number");
    let turns = rate * crate::RAIN_CLOCK_WRAP_S;
    assert!((turns - turns.round()).abs() < 1e-3, "{turns} ring cycles per clock wrap");
}

/// SF-2: the surf shape reaches every pass through `shore_trains` with the twin's constants and gap
/// seeds, and `--off surfskew` zeroes slot 0's x (the shader's switch, γ back to `SW_BREAKER`).
#[test]
// (Exact: the WGSL literal must parse to the very constant; the slots copy the settings.)
#[allow(clippy::float_cmp)]
fn surf_shape_reaches_the_shader_with_the_twins_constants() {
    use sf_water::shore_waves as sw;
    let material = compile("sf_water_bevy::shaders::water_material", &[]).expect("compiles");
    let velocity = compile("sf_water_bevy::shaders::water_velocity", &[]).expect("compiles");
    for name in ["surf_life", "surf_envelope", "shape3"] {
        assert!(material.contains(name) && velocity.contains(name), "no {name} in the water draw or its motion vectors");
    }
    let src = std::fs::read_to_string(PathBuf::from(CRATE).join("src/shaders/water_common.wesl")).expect("source");
    let constant = |name: &str| -> f32 {
        let line = src.lines().find(|l| l.starts_with(&format!("const {name} = "))).unwrap_or_else(|| panic!("const {name}"));
        line.split('=').nth(1).unwrap().trim().trim_end_matches(';').parse().unwrap_or_else(|_| panic!("{line}"))
    };
    assert_eq!(constant("SW_SKEW_RATE_LIMIT"), sw::SKEW_RATE_LIMIT);
    for seed in sw::GAP_SEEDS {
        assert!(src.contains(&format!("sw_noise(foot, cells, {seed}u)")), "gap seed {seed}");
    }
    // The twin's switch, γ clamp, shoreline foot and skew clamps.
    assert!(src.contains("let on = p.shape.x > 0.5;"), "no switch");
    assert!(src.contains("let gamma = select(SW_BREAKER, clamp(p.shape.y, 0.1, 2.0), on);"), "γ not the twin's");
    assert!(src.contains("let foot = fract(noise + d * dir / 1024.0);"), "gaps not at the shoreline foot");
    assert!(src.contains("let skew_bore = clamp(p.shape2.z, 0.0, 0.9);") && src.contains("let e = clamp(p.shape.x, 1.0, 32.0);"), "clamps");
    let on = crate::surf_shape_slots(&sf_water::SurfShape::default());
    let off = crate::surf_shape_slots(&sf_water::SurfShape::off());
    let d = sf_water::SurfShape::default();
    assert_eq!(on[0].x, d.break_sharpness);
    assert_eq!(off[0].x, 0.0);
    assert_eq!(off[0].y, sw::BREAKER_INDEX);
    assert_eq!(on[0].y, d.breaker_gamma);
    assert_eq!(on[2].y.fract(), 0.0, "gap cells not whole per noise period");
    assert_eq!(on[2].y, d.gap_cells());
}
