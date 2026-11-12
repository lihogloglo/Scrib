//! Shader checks without a GPU or a picture (CONTRIBUTING "Tests and visual checks"): the fog
//! and cloud passes are linked by `wesl` as Bevy's shader cache links them (`bevy_shader`
//! `shader_cache.rs`; the vendored `bevy_pbr` / `bevy_render` / `bevy_core_pipeline` modules)
//! and validated by naga. The resolver follows `sf_surface_bevy`'s `shader_tests.rs`.

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
            "sf_atmos_bevy" => Some(PathBuf::from(CRATE).join("src")),
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

/// The passes that light the fog (WE-9: the fill carries the view's ambient) compile, with and
/// without MSAA, and each reads the fog's `view_ambient` where the fill is formed.
#[test]
fn fog_passes_compile_and_read_the_view_ambient() {
    let mut failures = Vec::new();
    let jobs: [(&str, &[&str], bool); 6] = [
        ("sf_atmos_bevy::shaders::fog_light", &[], true),
        ("sf_atmos_bevy::shaders::cloud_composite", &[], false),
        ("sf_atmos_bevy::shaders::cloud_composite", &["MULTISAMPLED"], false),
        ("sf_atmos_bevy::shaders::cloud_trace", &[], true),
        ("sf_atmos_bevy::shaders::cloud_trace", &["MULTISAMPLED"], true),
        ("sf_atmos_bevy::shaders::froxel_inject", &[], false),
    ];
    for (module, defs, reads_ambient) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if reads_ambient && !wgsl.contains(".view_ambient") => failures.push(format!("{module} {defs:?}: no view_ambient in the fill")),
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The cloud history (WE-10 fix 1): the resolve compiles and reads the fresh weight and the
/// clip γ; the trace (with and without MSAA) compiles and reads the blue-noise jitter.
#[test]
fn cloud_history_passes_compile() {
    let mut failures = Vec::new();
    let jobs: [(&str, &[&str], &[&str]); 3] = [
        ("sf_atmos_bevy::shaders::cloud_resolve", &[], &[".history_weight", ".clip_gamma"]),
        ("sf_atmos_bevy::shaders::cloud_trace", &[], &["blue_noise"]),
        ("sf_atmos_bevy::shaders::cloud_trace", &["MULTISAMPLED"], &["blue_noise"]),
    ];
    for (module, defs, reads) in jobs {
        match compile(module, defs) {
            Ok(wgsl) => failures.extend(reads.iter().filter(|r| !wgsl.contains(*r)).map(|r| format!("{module} {defs:?}: never reads {r}"))),
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The fog split at the cloud's depth (WE12-FIX): the composite and the trace (with and without
/// MSAA) compile and read `cloud_fog` (the `Clouds` layout is checked below).
#[test]
fn cloud_fog_split_compiles() {
    let mut failures = Vec::new();
    for module in ["sf_atmos_bevy::shaders::cloud_composite", "sf_atmos_bevy::shaders::cloud_trace"] {
        for defs in [&[][..], &["MULTISAMPLED"][..]] {
            match compile(module, defs) {
                Ok(wgsl) if !wgsl.contains(".cloud_fog") => failures.push(format!("{module} {defs:?}: never reads cloud_fog")),
                Ok(_) => {}
                Err(e) => failures.push(e),
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The depth-aware fog bank (FOG-BANK): trace, resolve and composite compile with and without
/// MSAA and read `bank_edges`; the flag sits in `Fog`'s padding, so the WGSL `Fog` and `Clouds`
/// structs keep the size of their Rust mirrors (`FogUniform`, `CloudUniform`).
#[test]
fn fog_bank_edges_compile_and_keep_the_layout() {
    use bevy_render::render_resource::ShaderType;
    let mut failures = Vec::new();
    let jobs: [(&str, &[&str]); 5] = [
        ("sf_atmos_bevy::shaders::cloud_trace", &[]),
        ("sf_atmos_bevy::shaders::cloud_trace", &["MULTISAMPLED"]),
        ("sf_atmos_bevy::shaders::cloud_resolve", &[]),
        ("sf_atmos_bevy::shaders::cloud_composite", &[]),
        ("sf_atmos_bevy::shaders::cloud_composite", &["MULTISAMPLED"]),
    ];
    for (module, defs) in jobs {
        match compile(module, defs) {
            Ok(wgsl) if !wgsl.contains(".bank_edges") => failures.push(format!("{module} {defs:?}: never reads bank_edges")),
            Ok(_) => {}
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));

    let wgsl = compile("sf_atmos_bevy::shaders::cloud_composite", &[]).expect("compiles");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let span_with = |member: &str| {
        module.types.iter().find_map(|(_, ty)| match &ty.inner {
            naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some(member)) => Some(u64::from(*span)),
            _ => None,
        })
    };
    assert_eq!(span_with("bank_edges"), Some(crate::render::FogUniform::min_size().get()), "Fog layout");
    assert_eq!(span_with("glows"), Some(crate::render::CloudUniform::min_size().get()), "Clouds layout");
}

/// The froxel history clip (FOG-CLAMP): the inject compiles and reads `flags`, its workgroup is
/// `sf_atmos::froxel::INJECT_GROUP` square (the dispatch's tile math), and the WGSL `Froxels`
/// keeps the size of its Rust mirror (`FroxelUniform`).
#[test]
fn froxel_clamp_compiles_and_keeps_the_layout() {
    use bevy_render::render_resource::ShaderType;
    let wgsl = compile("sf_atmos_bevy::shaders::froxel_inject", &[]).expect("compiles");
    assert!(wgsl.contains(".flags"), "the inject never reads flags");
    let module = naga::front::wgsl::parse_str(&wgsl).expect("parses");
    let inject = module.entry_points.iter().find(|e| e.name == "inject").expect("inject entry point");
    let g = sf_atmos::froxel::INJECT_GROUP;
    assert_eq!(inject.workgroup_size, [g, g, 1], "workgroup vs INJECT_GROUP");
    let span = module.types.iter().find_map(|(_, ty)| match &ty.inner {
        naga::TypeInner::Struct { members, span } if members.iter().any(|m| m.name.as_deref() == Some("shadow_light")) => Some(u64::from(*span)),
        _ => None,
    });
    assert_eq!(span, Some(crate::render::FroxelUniform::min_size().get()), "Froxels layout");
}
