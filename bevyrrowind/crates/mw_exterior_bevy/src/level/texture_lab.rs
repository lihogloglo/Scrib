//! The texture lab (`view level:texture-lab`, `docs/specs/test-levels.md` "Texture lab"): one
//! texture or one model at a time under a grazing sun and an orbiting lamp, so the owner can A/B
//! the packs' normal, ORM and depth maps (`flip normalmaps`, `flip orm`, `flip parallax`, keys N / M / L here) and tell "no
//! data" from "weak effect" by the info panel.
//!
//! - **Swatches** (`--lab-textures a,b,...`, default [`DEFAULT_TEXTURES`]): per texture a panel, a
//!   sphere and a cube (generated with UVs and `MikkTSpace` tangents) whose material is built by
//!   the statics' own path, `mw_assets::cooked::cooked_material_with` over the bake's
//!   [`TextureCompanions`], so a swatch gets exactly what a static with that texture gets.
//! - **Models** (`--lab-models a.nif,...`, default [`DEFAULT_MODELS`]): cooked statics spawned as
//!   [`CookedModelRoot`]s (the viewer's cooked path: the same `Surface` mapping and companions as
//!   streamed statics; a model without a cooked product falls back to its NIF), on a turntable.
//! - **Light**: the level's hour (a low morning sun; `[` / `]` move it) and one lamp per swatch
//!   orbiting it through the game's lamp path (`mw_assets::lights::spawn_light`).
//! - **Info** (key I): per swatch the texture's served file and source, size, format, mips, its
//!   normal and ORM companions (file, format, whether bound now), depth (tx-pom: the BC4 map, bound, the preset's parallax) and
//!   whether its meshes have tangents; per model its parts, materials with maps and tangents.
//!   The same lines go to the log once the images load (`texture-lab swatch N: ...`).
//!
//! Keys: Z turntable, X lamp orbit, I info panel, N normal maps, M ORM maps, L parallax (P is the
//! clock pause on every view).

use std::collections::HashSet;

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use mw_assets::cooked::{CookedSpawned, TextureCompanions, cooked_material_with};
use mw_assets::{CookedModelRoot, MaterialMaps, SharedVfs};
use sf_coords::WorldPos;
use sf_level::{CameraDef, Level};
use sf_vfs::{Vfs, VfsPath};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{WorldspaceRegistry, to_grid};

/// The level's name: its swatches and models spawn without any `--lab-*` flag.
pub const LEVEL: &str = "texture-lab";

/// Textures the packs cover (MVR PBR: `_n`, `_nh`, `_spec`; Tamrielic PBR West Gash: `_nh`,
/// `_spec`): a Hlaalu wall trim, a West Gash cobblestone, carved wood, Dwemer metal, a West
/// Gash rock and a dirt road (the last three are land textures: statics rarely use them, so the
/// bake may list no companion; the panel says so).
pub const DEFAULT_TEXTURES: &[&str] = &[
    "textures/tx_hlaalu_sideedge_01.dds",
    "textures/tx_wg_cobblestones_01.dds",
    "textures/tx_carvedwood_beam_01.dds",
    "textures/tx_dwrv_metal00.dds",
    "textures/tx_rock_wg_01.dds",
    "textures/tx_wg_dirtroad_01.dds",
];

/// Balmora and West Gash statics: a Hlaalu house, a Hlaalu wall, two West Gash rocks.
pub const DEFAULT_MODELS: &[&str] =
    &["meshes/x/ex_hlaalu_b_01.nif", "meshes/x/ex_hlaalu_wall_01.nif", "meshes/x/terrain_rock_wg_01.nif", "meshes/x/terrain_rock_wg_05.nif"];

/// Swatch row: centers this far apart along x, at z = 0.
pub const SWATCH_SPACING: f32 = 3.0;
const PANEL: f32 = 1.6;
const SPHERE: f32 = 0.4;
const CUBE: f32 = 0.7;
/// Model row: centers this far apart along x, at z = [`MODEL_Z`].
pub const MODEL_SPACING: f32 = 14.0;
pub const MODEL_Z: f32 = -14.0;
/// Turntable rate, degrees per second.
const TURNTABLE: f32 = 10.0;
/// Each swatch's lamp: orbit radius and height (m), rate (deg/s), light radius (m).
const LAMP_ORBIT: f32 = 1.4;
const LAMP_HEIGHT: f32 = 1.5;
const LAMP_RATE: f32 = 30.0;
const LAMP_RADIUS: f32 = 1.5;

/// What the lab shows: VFS texture and model paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabContent {
    pub textures: Vec<String>,
    pub models: Vec<String>,
}

impl LabContent {
    /// The flags' lists (bare names get `textures/` + `.dds`, `meshes/` + `.nif`), else the defaults.
    pub fn new(textures: Option<&[String]>, models: Option<&[String]>) -> Self {
        let textures = textures.map_or_else(|| DEFAULT_TEXTURES.iter().map(|t| (*t).to_owned()).collect(), |t| t.iter().filter_map(|t| texture_path(t)).collect());
        let models = models.map_or_else(|| DEFAULT_MODELS.iter().map(|m| (*m).to_owned()).collect(), |m| m.iter().filter_map(|m| model_path(m)).collect());
        Self { textures, models }
    }

    /// Swatch `i`'s center on the floor.
    #[allow(clippy::cast_precision_loss)]
    pub fn swatch_at(&self, i: usize) -> Vec3 {
        Vec3::new((i as f32 - (self.textures.len() as f32 - 1.0) * 0.5) * SWATCH_SPACING, 0.0, 0.0)
    }

    /// Model `i`'s origin.
    #[allow(clippy::cast_precision_loss)]
    pub fn model_at(&self, i: usize) -> Vec3 {
        Vec3::new((i as f32 - (self.models.len() as f32 - 1.0) * 0.5) * MODEL_SPACING, 0.0, MODEL_Z)
    }

    /// One close-up camera per swatch (`swatch-1`, ...) and per model (`model-1`, ...), after the
    /// level's own.
    pub fn add_cameras(&self, level: &mut Level) {
        for i in 0..self.textures.len() {
            let at = self.swatch_at(i);
            level.cameras.push(CameraDef { name: format!("swatch-{}", i + 1), eye: at + Vec3::new(0.0, 1.15, 2.4), target: at + Vec3::new(0.0, 0.9, -0.3) });
        }
        for i in 0..self.models.len() {
            let at = self.model_at(i);
            level.cameras.push(CameraDef { name: format!("model-{}", i + 1), eye: at + Vec3::new(0.0, 6.0, 15.0), target: at + Vec3::new(0.0, 3.0, 0.0) });
        }
    }
}

fn texture_path(name: &str) -> Option<String> {
    let name = name.trim().replace('\\', "/").to_ascii_lowercase();
    if name.is_empty() {
        return None;
    }
    let name = if name.starts_with("textures/") { name } else { format!("textures/{name}") };
    Some(if std::path::Path::new(&name).extension().is_some() { name } else { format!("{name}.dds") })
}

fn model_path(name: &str) -> Option<String> {
    let path = mw_world::model_path(name)?;
    Some(if std::path::Path::new(&path).extension().is_some() { path } else { format!("{path}.nif") })
}

/// The source material of a swatch: just the base texture, as a NIF surface with no extras.
pub fn swatch_source(texture: &str) -> sf_model::Material {
    sf_model::Material { base_color_texture: Some(texture.to_owned()), ..sf_model::Material::default() }
}

/// A swatch's panel (facing +z), sphere and cube, with UVs, normals and tangents.
pub fn swatch_meshes() -> [Mesh; 3] {
    [Mesh::from(Rectangle::new(PANEL, PANEL)), Sphere::new(SPHERE).mesh().uv(48, 24), Mesh::from(Cuboid::new(CUBE, CUBE, CUBE))]
        .map(|m| m.with_generated_tangents().expect("primitive meshes have UVs and normals"))
}

/// A swatch: its texture and the maps its material was built with (kept here, since the A/B
/// switches take them out of the material while off).
#[derive(Component, Clone, Debug)]
pub struct Swatch {
    pub index: usize,
    pub texture: String,
    pub material: Handle<StandardMaterial>,
    pub base: Option<Handle<Image>>,
    pub normal: Option<(String, Handle<Image>)>,
    pub orm: Option<(String, Handle<Image>, bool)>,
    /// The depth companion's served path and image (tx-pom).
    pub depth: Option<(String, Handle<Image>)>,
    pub meshes: Vec<Handle<Mesh>>,
}

/// A lab model (on the turntable).
#[derive(Component, Clone, Debug)]
pub struct LabModel {
    pub index: usize,
    pub path: String,
}

#[derive(Component)]
struct SwatchLamp {
    center: Vec3,
    phase: f32,
}

#[derive(Component)]
struct InfoText;

#[derive(Resource)]
#[allow(clippy::struct_excessive_bools)] // independent toggles
struct Lab {
    content: LabContent,
    /// The worldspace's content layer (cooked model parts get it, as level objects have it).
    layers: RenderLayers,
    turntable: bool,
    orbit: bool,
    orbit_time: f32,
    info: bool,
    logged: bool,
}

/// Adds the lab to a level run (after `level::add`'s worldspace setup).
pub fn add(app: &mut App, content: LabContent) {
    app.insert_resource(Lab { content, layers: RenderLayers::default(), turntable: true, orbit: true, orbit_time: 0.0, info: true, logged: false })
        .add_systems(OnEnter(sf_app_bevy::WorldAlive), spawn.after(crate::exterior::WorldSetup))
        .add_systems(Update, (keys, turn, orbit, part_layers, info).chain());
}

#[allow(clippy::too_many_arguments)] // a system's parameters
fn spawn(
    mut commands: Commands,
    mut registry: ResMut<WorldspaceRegistry>,
    level: Res<super::ActiveLevel>,
    mut lab: ResMut<Lab>,
    vfs: Res<SharedVfs>,
    server: Res<AssetServer>,
    calibration: Res<mw_assets::LightCalibration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let entry = registry.ensure(&mut commands, &level.0.worldspace);
    lab.layers = RenderLayers::layer(entry.layer);
    let companions = TextureCompanions::read(&vfs.0);
    let [panel, sphere, cube] = swatch_meshes().map(|m| meshes.add(m));
    let (mut normals, mut orms, mut depths) = (0, 0, 0);
    let content = lab.content.clone();
    for (i, texture) in content.textures.iter().enumerate() {
        let material = cooked_material_with(&swatch_source(texture), &vfs.0, &companions, &server);
        let normal = material.normal_map_texture.clone().map(|h| (companions.of(texture).unwrap_or_default().to_owned(), h));
        let orm = material.metallic_roughness_texture.clone().map(|h| {
            let c = companions.orm_of(texture);
            (c.map(|c| c.file.clone()).unwrap_or_default(), h, material.occlusion_texture.is_some())
        });
        normals += usize::from(normal.is_some());
        orms += usize::from(orm.is_some());
        let depth = material.depth_map.clone().map(|h| (companions.depth_of(texture).unwrap_or_default().to_owned(), h));
        depths += usize::from(depth.is_some());
        let base = material.base_color_texture.clone();
        let handle = materials.add(material);
        let swatch = Swatch { index: i, texture: texture.clone(), material: handle.clone(), base, normal, orm, depth, meshes: vec![panel.clone(), sphere.clone(), cube.clone()] };
        let at = content.swatch_at(i);
        let short = texture.rsplit('/').next().unwrap_or(texture);
        for (part, mesh, offset) in [
            ("panel", &panel, Vec3::new(0.0, 0.2 + PANEL * 0.5, -0.6)),
            ("sphere", &sphere, Vec3::new(-0.55, SPHERE, 0.5)),
            ("cube", &cube, Vec3::new(0.55, CUBE * 0.5, 0.5)),
        ] {
            let (cell, transform) = to_grid(WorldPos::from_f64((at + offset).as_dvec3()));
            commands.spawn((
                Name::new(format!("swatch {} {part} {short}", i + 1)),
                swatch.clone(),
                Mesh3d(mesh.clone()),
                MeshMaterial3d(handle.clone()),
                cell,
                transform,
                lab.layers.clone(),
                ChildOf(entry.root),
            ));
        }
        let info = mw_world::LightInfo { color: [1.0, 0.86, 0.68], radius: LAMP_RADIUS, animation: mw_world::LightAnimation::Steady, fire: false };
        #[allow(clippy::cast_precision_loss)]
        let phase = i as f32 * 0.9;
        let center = at + Vec3::new(0.0, LAMP_HEIGHT, 0.0);
        let (cell, transform) = to_grid(WorldPos::from_f64(lamp_at(center, phase, 0.0).as_dvec3()));
        #[allow(clippy::cast_possible_truncation)]
        let light = mw_assets::lights::spawn_light(&mut commands, &calibration, &info, None, transform, 1000 + i as u32);
        commands.entity(light).try_insert((Name::new(format!("swatch {} lamp", i + 1)), SwatchLamp { center, phase }, cell, transform, entry.layers().light(), ChildOf(entry.root)));
    }
    for (i, model) in content.models.iter().enumerate() {
        let (cell, transform) = to_grid(WorldPos::from_f64(content.model_at(i).as_dvec3()));
        let root = CookedModelRoot { model: mw_assets::cooked::load(&server, model, None), source: model.clone(), scale: 1.0 };
        commands.spawn((Name::new(format!("lab model {} {model}", i + 1)), LabModel { index: i, path: model.clone() }, root, cell, transform, Visibility::Inherited, ChildOf(entry.root)));
    }
    commands.spawn((
        InfoText,
        Text::new(""),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgb(0.95, 0.95, 0.85)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        Node { position_type: PositionType::Absolute, bottom: Val::Px(6.0), left: Val::Px(6.0), padding: UiRect::all(Val::Px(6.0)), ..Node::default() },
    ));
    let n = content.textures.len();
    info!("texture-lab: {n} swatches, {} models; normals {normals}/{n}, orm {orms}/{n}, depth {depths}/{n} (Z turntable, X lamp orbit, I info, N normal maps, M ORM maps, L parallax)", content.models.len());
}

fn lamp_at(center: Vec3, phase: f32, t: f32) -> Vec3 {
    let a = phase + (LAMP_RATE * t).to_radians();
    center + Vec3::new(a.cos() * LAMP_ORBIT, 0.0, a.sin() * LAMP_ORBIT)
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut lab: ResMut<Lab>, mut maps: ResMut<MaterialMaps>) {
    let say = |what: &str, on: bool| println!("[A/B] {what} {}", if on { "on" } else { "off" });
    if input.just_pressed(KeyCode::KeyZ) {
        lab.turntable = !lab.turntable;
        say("texture-lab turntable", lab.turntable);
    }
    if input.just_pressed(KeyCode::KeyX) {
        lab.orbit = !lab.orbit;
        say("texture-lab lamp orbit", lab.orbit);
    }
    if input.just_pressed(KeyCode::KeyI) {
        lab.info = !lab.info;
    }
    if input.just_pressed(KeyCode::KeyN) {
        maps.normal_maps = !maps.normal_maps;
        say("normal maps", maps.normal_maps);
    }
    if input.just_pressed(KeyCode::KeyM) {
        maps.orm = !maps.orm;
        say("ORM maps", maps.orm);
    }
    // L, not P: P is the clock pause on every view (`mw_devtools_bevy::live_ab`).
    if input.just_pressed(KeyCode::KeyL) {
        maps.parallax = !maps.parallax;
        say("parallax", maps.parallax);
    }
}

fn turn(time: Res<Time>, lab: Res<Lab>, mut models: Query<&mut Transform, With<LabModel>>) {
    if !lab.turntable {
        return;
    }
    for mut t in &mut models {
        t.rotate_y((TURNTABLE * time.delta_secs()).to_radians());
    }
}

fn orbit(time: Res<Time>, mut lab: ResMut<Lab>, mut lamps: Query<(&SwatchLamp, &mut CellCoord, &mut Transform)>) {
    if !lab.orbit {
        return;
    }
    lab.orbit_time += time.delta_secs();
    for (lamp, mut cell, mut transform) in &mut lamps {
        let (c, t) = to_grid(WorldPos::from_f64(lamp_at(lamp.center, lamp.phase, lab.orbit_time).as_dvec3()));
        cell.set_if_neq(c);
        transform.translation = t.translation;
    }
}

/// Cooked parts spawn without render layers: give the lab models' meshes the worldspace's, so
/// the sun and the lamps light them (lights pair with meshes by layer).
fn part_layers(mut commands: Commands, lab: Res<Lab>, parts: Query<Entity, (Added<Mesh3d>, Without<RenderLayers>)>, parents: Query<&ChildOf>, models: Query<(), With<LabModel>>) {
    for part in &parts {
        if parents.iter_ancestors(part).any(|a| models.contains(a)) {
            commands.entity(part).try_insert(lab.layers.clone());
        }
    }
}

/// Whether a mesh has tangents: from its main-world attributes, else the render world's layout
/// (`sf_diag_bevy` texture gauge probe; GPU-only cooked meshes). `None`: not known yet.
fn has_tangents(id: AssetId<Mesh>, meshes: &Assets<Mesh>, probe: Option<&sf_diag_bevy::textures::MeshAttributeProbe>) -> Option<bool> {
    if let Some(found) = meshes.get(id).and_then(|m| m.try_contains_attribute(Mesh::ATTRIBUTE_TANGENT).ok()) {
        return Some(found);
    }
    let probe = probe?.lock();
    probe.prepared.contains(&id).then(|| !probe.no_tangent.contains(&id))
}

/// `path` as served: the file the VFS resolves (with the texture alternates) and its source,
/// plus the original's source when the served file is a bake product.
pub fn provenance(vfs: &Vfs, path: &str) -> String {
    let Ok(vpath) = VfsPath::new(path) else { return format!("{path} (bad path)") };
    let Some((served, source)) = vfs.resolve_preferring(&vpath, mw_assets::TEXTURE_ALTERNATES) else { return format!("{path} MISSING") };
    let label = vfs.source(source).label().to_owned();
    let original = vfs.resolve_preferring(&vpath, &["dds", "tga", "png"]).filter(|(p, _)| *p != served).map(|(p, s)| format!(" from {} ({})", p.as_str(), vfs.source(s).label()));
    format!("{} ({label}){}", served.as_str(), original.unwrap_or_default())
}

/// The pack companion files the VFS has for `texture` (`_n`, `_nh`, `_spec` next to it), for
/// "the pack ships one but the bake lists none" (land textures, a missed role).
pub fn pack_companions(vfs: &Vfs, texture: &str) -> Vec<String> {
    let stem = texture.rsplit_once('.').map_or(texture, |(s, _)| s);
    let mut found = Vec::new();
    for suffix in ["_n", "_nh", "_spec"] {
        for ext in ["dds", "tga", "png", "ktx2"] {
            let candidate = format!("{stem}{suffix}.{ext}");
            if VfsPath::new(&candidate).is_ok_and(|p| vfs.contains(&p)) {
                found.push(candidate);
                break;
            }
        }
    }
    found
}

/// The panel's depth companion (tx-pom): the served BC4 map, whether bound, and the parallax the
/// material runs (the preset's: method, layers, scale); else why there is none.
fn depth_line(s: &Swatch, material: Option<&StandardMaterial>, images: &Assets<Image>) -> String {
    let bound = material.is_some_and(|m| m.depth_map.is_some());
    s.depth.as_ref().map_or_else(
        || match mw_assets::parallax() {
            None => "none (parallax off on this preset)".to_owned(),
            Some(_) if s.normal.as_ref().is_some_and(|(file, _)| file.contains("_nh")) => "none in the bake's index".to_owned(),
            Some(_) => "none (no _nh companion)".to_owned(),
        },
        |(file, h)| {
            let how = material.filter(|m| m.depth_map.is_some()).map_or_else(String::new, |m| {
                format!(" {:?} {} layers, scale {}", m.parallax_mapping_method, m.max_parallax_layer_count, m.parallax_depth_scale)
            });
            let state = if bound { "bound" } else { "UNBOUND (switch off)" };
            format!("{file} {} {state}{how}", image_line(images.get(h)))
        },
    )
}

fn image_line(image: Option<&Image>) -> String {
    image.map_or_else(
        || "loading".to_owned(),
        |i| {
            let d = &i.texture_descriptor;
            format!("{}x{} {:?} {} mips", d.size.width, d.size.height, d.format, d.mip_level_count)
        },
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)] // a system's parameters
fn info(
    real: Res<Time<Real>>,
    mut lab: ResMut<Lab>,
    vfs: Res<SharedVfs>,
    server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    maps: Res<MaterialMaps>,
    probe: Option<Res<sf_diag_bevy::textures::MeshAttributeProbe>>,
    swatches: Query<&Swatch>,
    models: Query<(Entity, &LabModel, Option<&CookedSpawned>)>,
    children: Query<&Children>,
    parts: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
    mut text: Query<(&mut Text, &mut Visibility), With<InfoText>>,
    mut next: Local<f32>,
) {
    let now = real.elapsed_secs();
    if now < *next {
        return;
    }
    *next = now + 0.5;
    let Ok((mut text, mut shown)) = text.single_mut() else { return };
    let probe = probe.as_deref();
    let yes = |b: Option<bool>| b.map_or("?", |b| if b { "yes" } else { "NO" });
    let mut lines = vec![format!(
        "texture lab  |  normal maps {} (N)  ORM {} (M)  parallax {} (L)  |  Z turntable {}  X lamp orbit {}  I info",
        if maps.normal_maps { "on" } else { "off" },
        if maps.orm { "on" } else { "off" },
        if maps.parallax { "on" } else { "off" },
        if lab.turntable { "on" } else { "off" },
        if lab.orbit { "on" } else { "off" }
    )];
    let mut seen = HashSet::new();
    let mut loaded = true;
    let mut sorted: Vec<&Swatch> = swatches.iter().filter(|s| seen.insert(s.index)).collect();
    sorted.sort_by_key(|s| s.index);
    for s in sorted {
        let base = s.base.as_ref().map(Handle::id);
        loaded &= [base, s.normal.as_ref().map(|n| n.1.id()), s.orm.as_ref().map(|o| o.1.id()), s.depth.as_ref().map(|d| d.1.id())].into_iter().flatten().all(|id| images.contains(id) || server.load_state(id).is_failed());
        let material = materials.get(&s.material);
        let bound = |b: bool| if b { "bound" } else { "UNBOUND (switch off)" };
        let normal = s.normal.as_ref().map_or_else(
            || {
                let pack = pack_companions(&vfs.0, &s.texture);
                if pack.is_empty() { "none".to_owned() } else { format!("none in the bake's index (pack has {})", pack.join(", ")) }
            },
            |(file, h)| format!("{} {} {}", provenance(&vfs.0, file), image_line(images.get(h)), bound(material.is_some_and(|m| m.normal_map_texture.is_some()))),
        );
        let orm = s.orm.as_ref().map_or_else(
            || "none".to_owned(),
            |(file, h, occlusion)| {
                format!(
                    "{} {}{} {}",
                    provenance(&vfs.0, file),
                    image_line(images.get(h)),
                    if *occlusion { " +occlusion" } else { "" },
                    bound(material.is_some_and(|m| m.metallic_roughness_texture.is_some()))
                )
            },
        );
        let depth = depth_line(s, material, &images);
        let tangents = s.meshes.iter().map(|m| has_tangents(m.id(), &meshes, probe)).try_fold(true, |all, t| t.map(|t| all && t));
        lines.push(format!(
            "swatch {}: {} {} | normal: {normal} | orm: {orm} | depth: {depth} | tangents {}",
            s.index + 1,
            provenance(&vfs.0, &s.texture),
            s.base.as_ref().map_or_else(|| "no base texture".to_owned(), |h| image_line(images.get(h))),
            yes(tangents)
        ));
    }
    let mut sorted: Vec<_> = models.iter().collect();
    sorted.sort_by_key(|(_, m, _)| m.index);
    for (root, model, spawned) in sorted {
        let Some(spawned) = spawned else {
            loaded = false;
            lines.push(format!("model {}: {} loading", model.index + 1, model.path));
            continue;
        };
        let (mut count, mut tangents, mut unknown) = (0, 0, 0);
        let mut used = HashSet::new();
        for part in children.iter_descendants(root) {
            let Ok((mesh, material)) = parts.get(part) else { continue };
            count += 1;
            used.insert(material.0.id());
            match has_tangents(mesh.0.id(), &meshes, probe) {
                Some(true) => tangents += 1,
                Some(false) => {}
                None => unknown += 1,
            }
        }
        let with = |f: fn(&StandardMaterial) -> bool| used.iter().filter(|id| materials.get(**id).is_some_and(f)).count();
        lines.push(format!(
            "model {}: {} {} | {count} parts, {} materials: normal map bound {}, ORM bound {}, depth bound {} | tangents {tangents}/{count}{}",
            model.index + 1,
            model.path,
            if spawned.fallback { "NIF fallback (no cooked product)" } else { "cooked" },
            used.len(),
            with(|m| m.normal_map_texture.is_some()),
            with(|m| m.metallic_roughness_texture.is_some()),
            with(|m| m.depth_map.is_some()),
            if unknown > 0 { format!(" ({unknown} not prepared yet)") } else { String::new() }
        ));
    }
    if loaded && !lab.logged {
        lab.logged = true;
        for line in &lines[1..] {
            info!("texture-lab {line}");
        }
    }
    *shown = if lab.info { Visibility::Inherited } else { Visibility::Hidden };
    let out = lines.join("\n");
    if text.0 != out {
        text.0 = out;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    /// Bare names get their folder and extension; no flag means the defaults.
    #[test]
    fn content_normalizes_names_and_defaults() {
        let content = LabContent::new(Some(&["TX_Wall".to_owned(), "textures\\tx_b.tga".to_owned(), " ".to_owned()]), Some(&["x\\Ex_Rock".to_owned()]));
        assert_eq!(content.textures, ["textures/tx_wall.dds", "textures/tx_b.tga"]);
        assert_eq!(content.models, ["meshes/x/ex_rock.nif"]);
        let defaults = LabContent::new(None, None);
        assert_eq!(defaults.textures.len(), DEFAULT_TEXTURES.len());
        assert_eq!(defaults.models.len(), DEFAULT_MODELS.len());
        assert!((defaults.swatch_at(0).x + defaults.swatch_at(defaults.textures.len() - 1).x).abs() < 1e-4, "centered row");
    }

    /// `swatch-N` / `model-N` cameras follow the content and the level stays valid.
    #[test]
    fn cameras_per_swatch_and_model() {
        let (_, mut level) = super::super::find(LEVEL).expect("level parses");
        let content = LabContent::new(None, None);
        let before = level.cameras.len();
        content.add_cameras(&mut level);
        assert_eq!(level.cameras.len(), before + content.textures.len() + content.models.len());
        assert!(level.cameras.iter().any(|c| c.name == "swatch-6") && level.cameras.iter().any(|c| c.name == "model-4"));
        assert!(level.problems().is_empty(), "{:?}", level.problems());
    }

    /// A swatch built for a texture with a (fake) companion gets the normal map through the
    /// statics' material path, and every swatch mesh has tangents and UVs (Bevy ignores a normal
    /// map on a mesh without tangents).
    #[test]
    fn swatch_gets_the_companion_normal_map_and_tangents() {
        let vfs = Vfs::builder()
            .mount(sf_vfs::MemSource::new("data").with("textures/tx_lab.dds", vec![0u8; 4]).with("textures/tx_lab_n.dds", vec![0u8; 4]).with("textures/tx_bare.dds", vec![0u8; 4]))
            .build();
        let companions = TextureCompanions::from(BTreeMap::from([("textures/tx_lab".to_owned(), "textures/tx_lab_n.dds".to_owned())]));
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Image>();
        let server = app.world().resource::<AssetServer>().clone();

        let lab = cooked_material_with(&swatch_source("textures/tx_lab.dds"), &vfs, &companions, &server);
        assert!(lab.base_color_texture.is_some());
        assert!(lab.normal_map_texture.is_some(), "the companion is bound");
        let bare = cooked_material_with(&swatch_source("textures/tx_bare.dds"), &vfs, &companions, &server);
        assert!(bare.base_color_texture.is_some() && bare.normal_map_texture.is_none());
        assert_eq!(pack_companions(&vfs, "textures/tx_lab.dds"), ["textures/tx_lab_n.dds"]);
        assert!(pack_companions(&vfs, "textures/tx_bare.dds").is_empty());

        for mesh in swatch_meshes() {
            assert!(mesh.contains_attribute(Mesh::ATTRIBUTE_TANGENT) && mesh.contains_attribute(Mesh::ATTRIBUTE_UV_0));
        }
    }
}
