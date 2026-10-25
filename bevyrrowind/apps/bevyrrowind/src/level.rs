//! Synthetic test levels (TS1, `docs/specs/test-levels.md`): greybox spaces
//! from `levels/<name>.ron` (`sf_level`), opened as `view level:<name>` or a
//! `scenes.ron` entry with `level:`.
//!
//! A level is its own worldspace (`level/<name>`) in the streamed viewer
//! (`exterior.rs`), so the sky (fixed hour), clouds, fog, wind, physics,
//! BRP, the F1 A/B keys and `--screenshot` all work unchanged. Its
//! streaming index is empty ([`LevelIndex`], layered over the bake's), so
//! nothing of Vvardenfell streams in; the greybox is spawned directly: one
//! mesh per primitive under the worldspace root and the same triangles as a
//! `Fixed` mesh collider on the `STATIC` layer, as world statics get from
//! their `collision.cook`. Not a separate app: actors, walkers and items come
//! with the viewer (TS1 part 2 spawns actors through the level's index).

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::RenderLayers;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use mw_install::MorrowindInstall;
use sf_controller::{CollisionMesh, SurfaceId};
use sf_coords::{WorldPos, WorldTransform};
use sf_level::{Level, Material, Shape};
use sf_physics::{BodyDesc, BodyKind, Layer, MeshShape, ShapeRef};
use sf_physics_bevy::{Physics, PhysicsBody};
use sf_world::{IndexParams, IndexReader, MemoryIndex, Placement, WorldIndex, WorldspaceId, build_directory, build_pages};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{Streaming, WorldIndexSource, WorldspaceRegistry, to_grid};

mod actors;

/// `view level:<name>`.
pub const PREFIX: &str = "level:";

/// Levels built into the exe (the repo's `levels/`), used when no file is found.
const BUILT_IN: &[(&str, &str)] = &[
    ("flat-slopes", include_str!("../../../levels/flat-slopes.ron")),
    ("ledges", include_str!("../../../levels/ledges.ron")),
    ("robe-walk", include_str!("../../../levels/robe-walk.ron")),
    ("light-lab", include_str!("../../../levels/light-lab.ron")),
    ("anim-lab", include_str!("../../../levels/anim-lab.ron")),
];

/// When `view level:` started: the load time is measured from here.
static STARTED: OnceLock<Instant> = OnceLock::new();

/// What the viewer needs to show a level.
pub struct LevelSetup {
    pub level: Arc<Level>,
    pub worldspace: WorldspaceId,
    /// The level's actors and the walk-mode player, resolved against the records.
    pub actors: Arc<[actors::ActorSetup]>,
}

/// The level's worldspace id.
pub fn worldspace(name: &str) -> WorldspaceId {
    WorldspaceId::new(&format!("level/{name}"))
}

/// `view level:<name>`: content (actors and records stay available), then the
/// streamed viewer in the level's worldspace. `--camera <name>` picks one of
/// the level's cameras; raw "ex,ey,ez,tx,ty,tz" still works.
pub(crate) fn view(install: &MorrowindInstall, name: &str, mut flags: crate::ViewFlags) -> crate::Result {
    let started = *STARTED.get_or_init(Instant::now);
    let (source, mut level) = find(name)?;
    lab_overrides(&mut level, &flags)?;
    let camera = pick_camera(&level, &mut flags.camera);
    println!(
        "level {} ({source}): {} objects, cameras {}",
        level.name,
        level.objects.len(),
        level.cameras.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
    );
    let (vfs, content) = crate::load(install, false)?;
    println!("content loaded in {:.2} s", started.elapsed().as_secs_f32());
    let ws = worldspace(&level.name);
    let start = crate::StreamStart::Interior { worldspace: ws.clone(), eye: camera.eye.as_dvec3(), target: camera.target.as_dvec3() };
    let mut options = flags.exterior_options(install, &content.db, format!("Bevyrrowind - level {}", level.name), start)?;
    let actors = actors::resolve(&level, &content.db);
    options.actors = Some(Arc::new(content.db));
    options.level = Some(LevelSetup { level: Arc::new(level), worldspace: ws, actors: actors.into() });
    crate::exterior::run(Arc::new(vfs), options)
}

/// The light-lab flags (`--lab-lights`, `--lab-ring`, `--lab-radius`, `--lab-orbit`) edit the
/// level's [`sf_level::LightRig`]: a perf sweep steps one of them per run.
fn lab_overrides(level: &mut Level, flags: &crate::ViewFlags) -> crate::Result<()> {
    let lab = &flags.lab;
    if lab.lights.is_none() && lab.ring.is_none() && lab.radius.is_none() && lab.orbit.is_none() {
        return Ok(());
    }
    let Some(rig) = level.lights.as_mut() else {
        return Err(format!("level {}: --lab-* flags need a level with `lights`", level.name).into());
    };
    rig.count = lab.lights.unwrap_or(rig.count);
    if let Some(radius) = lab.ring {
        rig.layout = sf_level::LightLayout::Ring { radius };
    }
    rig.radius = lab.radius.unwrap_or(rig.radius);
    rig.orbit = lab.orbit.unwrap_or(rig.orbit);
    let problems = level.problems();
    if problems.is_empty() { Ok(()) } else { Err(format!("level {}: {problems:?}", level.name).into()) }
}

/// The start camera: the level camera `--camera` names (case-insensitive;
/// the flag is then consumed), else the first one, with a raw
/// "ex,ey,ez,tx,ty,tz" flag left for the viewer to apply.
fn pick_camera<'a>(level: &'a Level, flag: &mut Option<String>) -> &'a sf_level::CameraDef {
    let named = flag.as_deref().and_then(|p| level.cameras.iter().find(|c| c.name.eq_ignore_ascii_case(p)));
    if named.is_some() {
        *flag = None;
    }
    named.unwrap_or(&level.cameras[0])
}

/// A level by name (`levels/<name>.ron` in the current folder, then next to
/// the exe, then built in) or by `.ron` path; parsed and validated.
pub fn find(name: &str) -> crate::Result<(String, Level)> {
    let parse = |text: &str, source: &str| Level::parse(text).map_err(|e| format!("{source}: {e}"));
    if Path::new(name).extension().is_some_and(|e| e.eq_ignore_ascii_case("ron")) {
        let text = std::fs::read_to_string(name).map_err(|e| format!("{name}: {e}"))?;
        return Ok((name.to_owned(), parse(&text, name)?));
    }
    let file = format!("{name}.ron");
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf));
    for path in std::iter::once(PathBuf::from("levels").join(&file)).chain(exe_dir.map(|d| d.join("levels").join(&file))) {
        if let Ok(text) = std::fs::read_to_string(&path) {
            let source = path.display().to_string();
            return Ok((source.clone(), parse(&text, &source)?));
        }
    }
    let (_, text) = BUILT_IN
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("no level {name:?} (levels/<name>.ron; built in: {})", BUILT_IN.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(", ")))?;
    Ok(("built-in".to_owned(), parse(text, &format!("built-in {name}"))?))
}

/// The level's streaming index (its actors and coverage markers:
/// `actors::placements`) over the bake's: the level worldspace has a
/// directory, so streaming settles at once and nothing else streams in.
struct LevelIndex {
    level: MemoryIndex,
    base: Arc<dyn IndexReader>,
}

impl LevelIndex {
    fn new(ws: &WorldspaceId, base: Arc<dyn IndexReader>, placements: Vec<Placement>) -> Self {
        let params = IndexParams::default();
        let (pages, stats) = build_pages(&params, placements, &|_| None);
        let directory = build_directory(ws.clone(), &params, &pages);
        let mut level = MemoryIndex::new();
        level.insert(&WorldIndex { directory, pages, stats });
        Self { level, base }
    }
}

impl IndexReader for LevelIndex {
    fn directory(&self, ws: &WorldspaceId) -> Result<Option<Vec<u8>>, String> {
        match self.level.directory(ws)? {
            Some(d) => Ok(Some(d)),
            None => self.base.directory(ws),
        }
    }

    fn page(&self, ws: &WorldspaceId, page: IVec3) -> Result<Option<Vec<u8>>, String> {
        if self.level.directory(ws)?.is_some() {
            return self.level.page(ws, page);
        }
        self.base.page(ws, page)
    }
}

#[derive(Resource)]
struct ActiveLevel(LevelSetup);

/// Adds the level to the viewer; call after `WorldIndexSource` is inserted.
pub fn add(app: &mut App, setup: LevelSetup) {
    let base = app.world().resource::<WorldIndexSource>().0.clone();
    let placements = actors::placements(&setup.level, &setup.actors);
    actors::add(app, Arc::clone(&setup.actors));
    app.insert_resource(WorldIndexSource(Arc::new(LevelIndex::new(&setup.worldspace, base, placements))))
        .insert_resource(ActiveLevel(setup))
        // After Startup's commands: the viewer's `setup` creates the worldspace root.
        .add_systems(PreStartup, interior_environment)
        .add_systems(PostStartup, (spawn, spawn_lights).chain())
        .add_systems(Update, orbit_lights)
        .add_systems(Last, report_ready);
}

/// Labels a level primitive (BRP `world_find_entities_by_name` finds it by its name).
#[derive(Component)]
pub struct LevelObject;

/// A level prop (a dynamic body). `interact` is the level's interaction
/// hook, kept for P11 (interaction points); nothing reads it yet.
#[derive(Component)]
#[allow(dead_code)] // `interact`: the P11 hook, read by nothing yet
pub struct LevelProp {
    pub interact: Option<String>,
}

fn spawn(
    mut commands: Commands,
    mut registry: ResMut<WorldspaceRegistry>,
    level: Res<ActiveLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut physics: Option<ResMut<Physics>>,
) {
    let started = Instant::now();
    let ActiveLevel(LevelSetup { level, worldspace: ws, .. }) = &*level;
    let entry = registry.ensure(&mut commands, ws);
    let grid = images.add(grid_image());
    let mut palette: Vec<(Material, Handle<StandardMaterial>)> = Vec::new();
    let (mut bodies, mut props, mut triangles) = (0, 0, 0);
    for (i, object) in level.objects.iter().enumerate() {
        let mut data = object.mesh();
        triangles += data.indices.len() / 3;
        let material = if let Some((_, h)) = palette.iter().find(|(m, _)| *m == object.material) {
            h.clone()
        } else {
            let [r, g, b] = object.material.srgb();
            let h = materials.add(StandardMaterial {
                base_color: Color::srgb(r, g, b),
                base_color_texture: Some(grid.clone()),
                perceptual_roughness: 0.85,
                ..StandardMaterial::default()
            });
            palette.push((object.material, h.clone()));
            h
        };
        let at = WorldPos::from_f64(object.at.as_dvec3());
        let (cell, mut transform) = to_grid(at);
        transform.rotation = object.rotation();
        let name = object.name.clone().unwrap_or_else(|| format!("level object {i}"));
        // A prop: a dynamic box body about its center (the mesh shifts down
        // to match); `PhysicsBody` writes its pose back to the entity.
        if let (Some(mass), Shape::Box { size }, Some(physics)) = (object.dynamic, &object.shape, physics.as_deref_mut()) {
            let center = at.offset(Vec3::Y * 0.5 * size.y);
            for p in &mut data.positions {
                p[1] -= 0.5 * size.y;
            }
            let pose = WorldTransform { pos: center, rotation: object.rotation(), scale: Vec3::ONE };
            let desc = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Box(*size * 0.5), Layer::PROP);
            let id = physics.insert(ws, &BodyDesc { mass: Some(mass), ..desc });
            let (cell, mut transform) = to_grid(center);
            transform.rotation = object.rotation();
            commands.spawn((
                Name::new(name),
                LevelObject,
                LevelProp { interact: object.interact.clone() },
                Mesh3d(meshes.add(render_mesh(&data))),
                MeshMaterial3d(material),
                cell,
                transform,
                RenderLayers::layer(entry.layer),
                ChildOf(entry.root),
                PhysicsBody { worldspace: ws.clone(), id },
            ));
            props += 1;
            continue;
        }
        if object.collide
            && let Some(physics) = physics.as_deref_mut()
        {
            let mesh = CollisionMesh { positions: data.positions.clone(), triangles: data.triangles(), surfaces: vec![SurfaceId::DEFAULT; data.indices.len() / 3] };
            if let Some(shape) = MeshShape::new(&mesh) {
                let pose = WorldTransform { pos: at, rotation: object.rotation(), scale: Vec3::ONE };
                physics.insert(ws, &BodyDesc::fixed(pose, ShapeRef::Mesh(shape), Layer::STATIC));
                bodies += 1;
            }
        }
        commands.spawn((
            Name::new(name),
            LevelObject,
            Mesh3d(meshes.add(render_mesh(&data))),
            MeshMaterial3d(material),
            cell,
            transform,
            RenderLayers::layer(entry.layer),
            ChildOf(entry.root),
        ));
    }
    info!(
        "level {}: {} objects ({triangles} triangles), {bodies} static colliders, {props} props, {} actors, spawned in {:.2} ms (F11: player)",
        level.name,
        level.objects.len(),
        level.actors.len(),
        started.elapsed().as_secs_f64() * 1e3
    );
}

/// A light of the level's [`sf_level::LightRig`] (its index in [`sf_level::LightRig::positions`]).
#[derive(Component)]
pub struct LabLight(usize);

/// The rig's lights through the game's own path (`mw_assets::lights::spawn_light`: the
/// calibration's range and power for the radius, `BudgetedShadow`, so the point-shadow budget,
/// contact shadows and the shadow cache treat them as lamps), on the worldspace's layer.
fn spawn_lights(mut commands: Commands, mut registry: ResMut<WorldspaceRegistry>, level: Res<ActiveLevel>, calibration: Res<mw_assets::LightCalibration>) {
    let ActiveLevel(LevelSetup { level, worldspace: ws, .. }) = &*level;
    let Some(rig) = &level.lights else { return };
    let entry = registry.ensure(&mut commands, ws);
    let info = mw_world::LightInfo { color: rig.color.to_array(), radius: rig.radius, animation: mw_world::LightAnimation::Steady, fire: false };
    let positions = rig.positions(0.0);
    for (i, at) in positions.iter().enumerate() {
        let (cell, transform) = to_grid(WorldPos::from_f64(at.as_dvec3()));
        #[allow(clippy::cast_possible_truncation)]
        let light = mw_assets::lights::spawn_light(&mut commands, &calibration, &info, None, transform, i as u32);
        commands.entity(light).try_insert((Name::new(format!("lab light {i}")), LabLight(i), cell, transform, RenderLayers::layer(entry.layer), ChildOf(entry.root)));
    }
    info!("level {}: {} lights ({:?}, radius {} m, orbit {} deg/s)", level.name, positions.len(), rig.layout, rig.radius, rig.orbit);
}

/// Orbiting rig lights (`orbit` deg/s): carried torches, for the cost of moving shadowed lights.
fn orbit_lights(time: Res<Time>, level: Res<ActiveLevel>, mut lights: Query<(&LabLight, &mut CellCoord, &mut Transform)>) {
    let Some(rig) = level.0.level.lights.as_ref().filter(|r| r.orbit != 0.0) else { return };
    let positions = rig.positions(time.elapsed_secs());
    for (light, mut cell, mut transform) in &mut lights {
        if let Some(at) = positions.get(light.0) {
            let (c, t) = to_grid(WorldPos::from_f64(at.as_dvec3()));
            cell.set_if_neq(c);
            transform.translation = t.translation;
        }
    }
}

/// An enclosed level (`interior`) is an interior to the viewer: no sky, sun or moon in its
/// worldspace, the level's ambient (`doors::worldspace_look`, as for a Morrowind interior cell).
fn interior_environment(level: Res<ActiveLevel>, mut envs: ResMut<crate::doors::Environments>) {
    if let Some(interior) = &level.0.level.interior {
        let a = interior.ambient.to_array();
        let atmosphere = mw_world::worldspaces::Atmosphere { ambient: a, sunlight: [0.0; 3], fog_color: [0.0; 3], fog_density: 0.0 };
        envs.0.insert(level.0.worldspace.clone(), mw_world::CellEnvironment { atmosphere: Some(atmosphere), sky: false, water_level: None });
    }
}

/// Logs the time from `view level:` to the first frame with streaming settled.
fn report_ready(streaming: Res<Streaming>, level: Res<ActiveLevel>, mut done: Local<bool>, mut frames: Local<u32>) {
    if *done {
        return;
    }
    *frames += 1;
    if *frames > 2 && streaming.has_directory(&level.0.worldspace) && streaming.is_idle() {
        *done = true;
        if let Some(t) = STARTED.get() {
            info!("level {} ready {:.2} s after start ({} frames)", level.0.level.name, t.elapsed().as_secs_f32(), *frames);
        }
    }
}

fn render_mesh(data: &sf_level::MeshData) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone())
        .with_inserted_indices(Indices::U32(data.indices.clone()))
}

/// The prototype grid: one tile per meter (UVs are meters), a dark line on
/// the meter, a faint one on the half meter; white, so the material's colour
/// tints it. Full mip chain, so it doesn't shimmer at grazing angles.
fn grid_image() -> Image {
    const SIZE: u32 = 128;
    let shade = |x: u32, y: u32| -> u8 {
        let edge = |v: u32| !(2..SIZE - 2).contains(&v);
        let half = |v: u32| v.abs_diff(SIZE / 2) < 1;
        if edge(x) || edge(y) {
            150
        } else if half(x) || half(y) {
            215
        } else {
            245
        }
    };
    let mut level: Vec<u8> = (0..SIZE * SIZE).map(|i| shade(i % SIZE, i / SIZE)).collect();
    let mut data = Vec::new();
    let mut size = SIZE;
    let mut mips = 0;
    loop {
        data.extend(level.iter().flat_map(|&v| [v, v, v, 255]));
        mips += 1;
        if size == 1 {
            break;
        }
        let next = size / 2;
        level = (0..next * next)
            .map(|i| {
                let (x, y) = ((i % next) * 2, (i / next) * 2);
                let at = |dx: u32, dy: u32| u32::from(level[((y + dy) * size + x + dx) as usize]);
                u8::try_from((at(0, 0) + at(1, 0) + at(0, 1) + at(1, 1)) / 4).unwrap_or(u8::MAX)
            })
            .collect();
        size = next;
    }
    // `Image::new` asserts one level's worth of data: fill the chain after.
    let mut image = Image::new_uninit(
        Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: 1 },
        TextureDimension::D2,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.data = Some(data);
    image.texture_descriptor.mip_level_count = mips;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 16,
        ..ImageSamplerDescriptor::default()
    });
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_levels_parse() {
        for (name, _) in BUILT_IN {
            let (_, level) = find(name).expect("level parses");
            assert_eq!(level.name, *name, "file name and level name agree");
        }
    }

    /// The framework's headless course tests run on a copy of `ledges`.
    #[test]
    fn framework_copy_of_ledges_agrees() {
        let framework = include_str!("../../../../superfargoth/crates/sf_level/tests/levels/ledges.ron");
        let ours = BUILT_IN.iter().find(|(n, _)| *n == "ledges").map(|(_, t)| *t).unwrap_or_default();
        assert_eq!(framework.replace("\r\n", "\n"), ours.replace("\r\n", "\n"), "copy levels/ledges.ron to superfargoth's sf_level/tests/levels/");
    }

    /// Placements: every actor plus markers
    /// covering the floor's streaming cells.
    #[test]
    fn robe_walk_placements_cover_the_level() {
        let (_, level) = find("robe-walk").expect("level parses");
        let setups: Vec<actors::ActorSetup> = level
            .actors
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, def)| actors::ActorSetup { def, kind: "NPC_", placement: sf_world::PlacementId(i as u128) })
            .collect();
        let placements = actors::placements(&level, &setups);
        let actors = placements.iter().filter(|p| p.class == sf_world::PlacementClass::Actor).count();
        assert_eq!(actors, level.actors.len());
        let cells: std::collections::HashSet<IVec3> =
            placements.iter().map(|p| sf_world::Partition::STREAMING.cell_of(p.transform.pos)).collect();
        assert!(cells.len() >= 4, "the floor spans the four cells around the origin: {cells:?}");
    }

    /// `--camera <name>` (as `scene level-robe-walk --camera course` passes
    /// it) picks that camera and is consumed; a raw pose stays for the viewer.
    #[test]
    fn camera_flag_picks_a_named_camera() {
        let (_, level) = find("robe-walk").expect("level parses");
        let mut flag = Some("Course".to_owned());
        assert_eq!(pick_camera(&level, &mut flag).name, "course");
        assert_eq!(flag, None);
        let mut raw = Some("1,2,3,4,5,6".to_owned());
        assert_eq!(pick_camera(&level, &mut raw).name, level.cameras[0].name);
        assert!(raw.is_some(), "a raw pose reaches the viewer");
        assert_eq!(pick_camera(&level, &mut None).name, level.cameras[0].name);
    }

    #[test]
    fn grid_image_has_a_full_mip_chain() {
        let image = grid_image();
        assert_eq!(image.texture_descriptor.mip_level_count, 8);
        let bytes: u32 = (0..8).map(|m| (128u32 >> m).pow(2) * 4).sum();
        assert_eq!(image.data.as_ref().map(Vec::len), Some(bytes as usize));
    }
}
