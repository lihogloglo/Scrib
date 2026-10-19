//! Render-order guard (render-order.md "Guard"): every translucent material
//! site in the framework's Bevy crates is declared in [`SITES`] with its fog
//! treatment and its placement, so a new blended or additive draw can't ship
//! unfogged or mis-sorted without a decision recorded in the spec.
//!
//! A source scan (like a tidy check), not a render test: it runs with plain
//! `std`, finds every `AlphaMode::{Blend, Add, Premultiplied, Multiply}` in
//! `crates/*_bevy/src` (comments, `matches!` tests and test modules skipped)
//! and checks:
//! 1. every file with a translucent site is declared, and every declaration
//!    still has one (no stale rows);
//! 2. a shader-placed or camera-local site tags its draws with a `DrawBand`
//!    (Bevy's sort key, the mesh AABB centre, means nothing for it), and a file
//!    spawning a translucent with `NoFrustumCulling` (no AABB, so the key is
//!    the entity origin) is not declared world-placed;
//! 3. fog: `Froxel` sites bind `TranslucentFog` and import the
//!    `translucent_fog` shader module, `Own` sites reference the froxel volume
//!    in their crate; `Host` and `Missing` sites name a violation id that
//!    render-order.md lists;
//! 4. shadows (rule 4, RO-1): a site's own `Material` and the host fog
//!    material (`sf_atmos_bevy` `FroxelFogExt`) disable shadow casting.

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // `Missing`: no framework site is unfogged today.
enum Fog {
    /// The material binds `TranslucentFog` and its shader imports
    /// `sf_render_order_bevy::translucent_fog` (render-order.md rule 1).
    Froxel,
    /// The crate fogs it its own way (documented in its spec).
    Own,
    /// The host must fog it (e.g. swap to `FroxelFogMaterial`): a tracked gap.
    Host,
    /// Not fogged: a tracked violation.
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Placement {
    /// Mesh AABB matches where it draws: Bevy's sort key is meaningful.
    World,
    /// Positions come from the shader (entity at the origin).
    ShaderPlaced,
    /// Follows the camera.
    CameraLocal,
}

struct Site {
    /// Relative to `superfargoth/crates`.
    file: &'static str,
    what: &'static str,
    fog: Fog,
    placement: Placement,
    /// render-order.md violation id, required unless fog and band are both fine.
    violation: Option<&'static str>,
}

/// The framework's translucent draws (render-order.md "Material inventory").
const SITES: &[Site] = &[
    Site { file: "sf_lod_bevy/src/materials.rs", what: "LightSpriteMaterial (additive distant light glows)", fog: Fog::Froxel, placement: Placement::World, violation: None },
    Site { file: "sf_precip_bevy/src/lib.rs", what: "PrecipMaterial (camera-wrapped rain/snow boxes)", fog: Fog::Froxel, placement: Placement::ShaderPlaced, violation: None },
    Site { file: "sf_water_bevy/src/lib.rs", what: "WaterMaterial (clipmap, refractive)", fog: Fog::Own, placement: Placement::ShaderPlaced, violation: None },
    Site { file: "sf_world_bevy/src/fade.rs", what: "FadeTransition overlay quad (camera child)", fog: Fog::Host, placement: Placement::CameraLocal, violation: Some("RO-9") },
    Site { file: "sf_world_bevy/src/spawn.rs", what: "generic model spawn, sf_model Blend -> StandardMaterial", fog: Fog::Host, placement: Placement::World, violation: Some("RO-4") },
];

const TRANSLUCENT: [&str; 4] = ["AlphaMode::Blend", "AlphaMode::Add", "AlphaMode::Premultiplied", "AlphaMode::Multiply"];

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("crates dir").to_path_buf()
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") && path.file_name().is_none_or(|n| n != "tests.rs") {
            out.push(path);
        }
    }
}

/// Non-test, non-comment source of a file.
fn code(text: &str) -> String {
    let body = text.split("mod tests {").next().unwrap_or(text);
    body.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n")
}

/// `superfargoth/crates`-relative paths of files with a translucent site.
fn discovered() -> Vec<(String, String)> {
    let root = crates_dir();
    let mut found = Vec::new();
    let Ok(crates) = fs::read_dir(&root) else { return found };
    for krate in crates.flatten() {
        let name = krate.file_name().to_string_lossy().into_owned();
        if !name.ends_with("_bevy") {
            continue;
        }
        let mut files = Vec::new();
        rust_files(&krate.path().join("src"), &mut files);
        for file in files {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let code = code(&text);
            let site = code.lines().any(|l| !l.contains("matches!") && TRANSLUCENT.iter().any(|t| l.contains(t)));
            if site {
                let rel = file.strip_prefix(&root).expect("under crates").to_string_lossy().replace('\\', "/");
                found.push((rel, code));
            }
        }
    }
    found
}

fn spec() -> String {
    let path = crates_dir().join("../docs/specs/render-order.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn every_translucent_site_is_declared_and_no_declaration_is_stale() {
    let found = discovered();
    let mut errors = Vec::new();
    for (file, _) in &found {
        if !SITES.iter().any(|s| s.file == file) {
            errors.push(format!(
                "{file}: new translucent material site. Declare its fog (froxel_fog_apply / own / host) and placement \
                 (shader-placed and camera-local draws need a DrawBand) in render-order.md and in SITES here."
            ));
        }
    }
    for site in SITES {
        if !found.iter().any(|(f, _)| f == site.file) {
            errors.push(format!("{}: declared ({}) but has no translucent site any more: remove the row here and in render-order.md", site.file, site.what));
        }
    }
    assert!(errors.is_empty(), "\n{}", errors.join("\n"));
}

#[test]
fn shader_placed_and_camera_local_translucents_take_a_band() {
    let found = discovered();
    let mut errors = Vec::new();
    for site in SITES {
        let Some((_, code)) = found.iter().find(|(f, _)| f == site.file) else { continue };
        let banded = code.contains("DrawBand::");
        if site.placement != Placement::World && !banded && site.violation.is_none() {
            errors.push(format!("{}: {:?} translucent without a DrawBand (Bevy sorts it by its entity origin)", site.file, site.placement));
        }
        if site.placement == Placement::World && code.contains("NoFrustumCulling") {
            errors.push(format!("{}: spawns NoFrustumCulling (no AABB: sorted by the entity origin) but is declared World", site.file));
        }
    }
    assert!(errors.is_empty(), "\n{}", errors.join("\n"));
}

#[test]
fn every_translucent_is_fogged_or_a_tracked_violation() {
    let spec = spec();
    let root = crates_dir();
    let mut errors = Vec::new();
    for site in SITES {
        if let Some(id) = site.violation
            && !spec.contains(&format!("**{id}**"))
        {
            errors.push(format!("{}: violation {id} is not listed in render-order.md", site.file));
        }
        match site.fog {
            Fog::Froxel => {
                let krate = site.file.split('/').next().unwrap_or_default();
                let mut files = Vec::new();
                rust_files(&root.join(krate).join("src"), &mut files);
                let binds = files.iter().any(|f| fs::read_to_string(f).is_ok_and(|t| t.contains("TranslucentFog")));
                let shaders = fs::read_dir(root.join(krate).join("src/shaders")).map(|d| d.flatten().map(|e| e.path()).collect::<Vec<_>>()).unwrap_or_default();
                let imports = shaders.iter().any(|f| fs::read_to_string(f).is_ok_and(|t| t.contains("import sf_render_order_bevy::translucent_fog::") && t.contains("froxel_fog_lookup(")));
                if !binds || !imports {
                    errors.push(format!("{}: declared Froxel but its crate doesn't bind TranslucentFog ({binds}) and call the translucent_fog module's froxel_fog_lookup ({imports})", site.file));
                }
            }
            Fog::Own => {
                let krate = site.file.split('/').next().unwrap_or_default();
                let mut files = Vec::new();
                rust_files(&root.join(krate).join("src"), &mut files);
                let fogged = files.iter().any(|f| fs::read_to_string(f).is_ok_and(|t| t.to_lowercase().contains("froxel")));
                if !fogged {
                    errors.push(format!("{}: declared fogged ({:?}) but its crate never references the froxel volume", site.file, site.fog));
                }
            }
            Fog::Host | Fog::Missing => {
                if site.violation.is_none() {
                    errors.push(format!("{}: unfogged ({:?}) without a render-order.md violation id", site.file, site.fog));
                }
            }
        }
    }
    assert!(errors.is_empty(), "\n{}", errors.join("\n"));
}

/// Rule 4 (RO-1): translucents cast no shadows unless their class opts in.
/// Bevy's shadow pass draws a blended or additive surface as solid wherever
/// alpha >= 0.05, so the policy lives in the materials: every framework
/// translucent `Material` and the host fog material (every NIF blended/additive
/// surface and particle is swapped to it) return `false` from `enable_shadows`.
#[test]
fn translucents_cast_no_shadows() {
    let root = crates_dir();
    let no_shadows = |text: &str| code(text).replace(char::is_whitespace, "").contains("fnenable_shadows()->bool{false}");
    let mut errors = Vec::new();
    let mut files: Vec<&str> = SITES.iter().map(|s| s.file).collect();
    files.push("sf_atmos_bevy/src/transparent.rs");
    for file in files {
        let text = fs::read_to_string(root.join(file)).unwrap_or_default();
        let own = text.contains("impl Material for") || text.contains("impl MaterialExtension for FroxelFogExt");
        if own && !no_shadows(&text) {
            errors.push(format!("{file}: translucent material without `fn enable_shadows() -> bool {{ false }}` (render-order.md rule 4)"));
        }
    }
    assert!(errors.is_empty(), "\n{}", errors.join("\n"));
}
