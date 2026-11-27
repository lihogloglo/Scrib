//! The rock lab (`view level:rock-lab`, `docs/specs/test-levels.md` "Rock lab"; rockgen.md slices RG-3, RG-4): a
//! field of procedural boulders (`sf_rockgen`) of every class (the eroded family block, platy, cobble, chipped at
//! -x; the sharp family talus, jagged, columnar, scoria, slab at +x) at power-law sizes from 0.5 m to 12 m on
//! flat ground, plus a few on and against a 30 deg slope, shaded by `sf_rockgen_bevy`'s triplanar material
//! with the mounted PBR packs' rock textures, so the owner can judge the shapes, the LODs and the material.
//!
//! - **Rocks** are generated on the async compute pool from the live `rockgen.boulder.*` tunables (console
//!   `set rockgen.boulder.chips 12`, then R), one column per class (`sf_rockgen_bevy::CLASSES`), sizes along
//!   -z from small (near the `wide` camera) to large. LODs are Bevy `VisibilityRange` children (dithered
//!   crossfade at `rockgen.lod.*` distances). No colliders yet (RG-2 adds the convex hulls).
//! - **Textures** ([`DEFAULT_TEXTURES`], `--rock-textures a,b,...`): each is built by the statics' material
//!   path (`crate::rock_materials`, shared with the library rocks: `cooked_material_with` over the bake's companions)
//!   and its maps are sampled triplanar, at the terrain's land-texture tiling. A texture the bake lists no
//!   companion for shows albedo only (the log line names the maps bound).
//!
//! Keys: X next texture, Z next class (all, then one at a time), R regenerate with a new seed, L LOD tint,
//! M wireframe, N crease normals on/off (off = RG-3's smooth SDF normals), Q edge snap on/off (off = RG-3's
//! bevelled surface-nets edges), I info panel. N and Q regenerate (an A/B on the same seed).

use bevy::camera::visibility::RenderLayers;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::tasks::futures::check_ready;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use mw_assets::SharedVfs;
use mw_assets::cooked::TextureCompanions;
use sf_coords::WorldPos;
use sf_level::{Level, Shape};
use sf_rockgen::{BoulderParams, FORMATIONS, RockLods, generate_boulder, generate_formation};
use sf_rockgen_bevy::{
    CLASSES, RockDebug, RockLodSettings, RockMaterial, RockMaterialSettings, RockgenPlugin, class_params, lod_bundles,
    lod_ranges, params_from_tunables, rock_mesh,
};

use sf_tunables_bevy::Tunables;
use sf_world_bevy::{WorldspaceRegistry, to_grid};

/// The level's name: the rock field spawns in it.
pub const LEVEL: &str = "rock-lab";

/// Rock and stone textures of the three mounted PBR packs (Wareya conventions, `textures-adapter.md` §7):
/// `tx_rock_wg_01` (mod `tamrielic-pbr-west-gash`), `tx_ashland_rock_03` and `tx_ac_rock_01` (`saintj-pbr`),
/// `tx_bm_rock_01` (`mvr-pbr`).
pub const DEFAULT_TEXTURES: &[&str] =
    &["textures/tx_rock_wg_01.dds", "textures/tx_ashland_rock_03.dds", "textures/tx_ac_rock_01.dds", "textures/tx_bm_rock_01.dds"];

// Viewer layout (fixture: the test scene's arrangement, not a gameplay or look value).
// tunables: const (fixture: rock-lab layout, x between class columns, centred on x = 0)
const COLUMN_SPACING_M: f32 = 22.0;
// tunables: const (fixture: rock-lab layout, z of the nearest rock's near edge)
const FIELD_START_Z: f32 = 24.0;
// tunables: const (fixture: rock-lab layout, gap between neighbours)
const GAP_M: f32 = 2.0;
// tunables: const (fixture: rock-lab layout, rocks per class column)
const FIELD_COUNT: usize = 10;
// tunables: const (fixture: rock-lab size range shown)
const SIZE_MIN_M: f32 = 0.5;
// tunables: const (fixture: rock-lab size range shown)
const SIZE_MAX_M: f32 = 12.0;
/// Truncated power-law exponent of the field's sizes (many small, few large; RG-A1 fits real ones).
// tunables: const (fixture: rock-lab size spread)
const SIZE_POWER: f32 = 1.2;
/// Fraction of a rock's height sunk below the ground (rockgen.md: 20-40 % at scatter time, RG-2).
// tunables: const (fixture: rock-lab placement)
const SINK: f32 = 0.2;
/// Rocks on the slope face: (size m, fraction of the run up the face).
// tunables: const (fixture: rock-lab layout)
const ON_SLOPE: [(f32, f32); 2] = [(1.5, 0.3), (4.0, 0.65)];
/// Rocks at the slope's toe: (size m, m out from the toe), the big one half against the face.
// tunables: const (fixture: rock-lab layout)
const AT_TOE: [(f32, f32); 2] = [(8.0, 1.0), (3.0, 7.0)];
/// The hero formations' row (`sf_rockgen::formations`, RG-8 first try): z of the row, beyond the plateau.
// tunables: const (fixture: rock-lab layout)
const FORMATION_ROW_Z: f32 = -135.0;
// tunables: const (fixture: rock-lab layout, x between formations)
const FORMATION_SPACING_M: f32 = 42.0;
/// Fraction of a formation's height sunk below the ground (a tower stands on its foot, a rock rests sunk).
// tunables: const (fixture: rock-lab placement)
const FORMATION_SINK: f32 = 0.02;
/// Mixes the lab seed with a rock's index.
const MIX_SEED: u64 = 0x9e37_79b9_7f4a_7c15;

/// What the lab shows: VFS texture paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RockLabContent {
    pub textures: Vec<String>,
}

impl RockLabContent {
    /// The flag's list (bare names get `textures/` + `.dds`), else the defaults.
    pub fn new(textures: Option<&[String]>) -> Self {
        let textures = textures.map_or_else(
            || DEFAULT_TEXTURES.iter().map(|t| (*t).to_owned()).collect(),
            |t| t.iter().filter_map(|t| super::texture_lab::texture_path(t)).collect(),
        );
        Self { textures }
    }
}

/// One rock of the layout: class, size, where it rests and on what.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    pub class: usize,
    pub size_m: f32,
    /// The ground point under the rock's centre.
    pub ground: Vec3,
    /// The ground's normal there.
    pub up: Vec3,
    /// Turn about `up`, radians.
    pub yaw: f32,
}

/// The `i`th of `n` sizes on a truncated power law `N(>d) ~ d^-b` between the bounds (quantile `i / (n - 1)`,
/// so both ends show).
#[allow(clippy::cast_precision_loss)]
pub fn power_law_size(i: usize, n: usize) -> f32 {
    let u = if n > 1 { i as f32 / (n - 1) as f32 } else { 0.0 };
    let (lo, hi) = (SIZE_MIN_M.powf(-SIZE_POWER), SIZE_MAX_M.powf(-SIZE_POWER));
    (lo - u * (lo - hi)).powf(-1.0 / SIZE_POWER)
}

/// The level's slope: (toe z, run, rise, angle in radians, width), from its `slope` ramp object.
fn slope(level: &Level) -> Option<(f32, f32, f32, f32, f32, f32)> {
    level.objects.iter().filter(|o| o.name.as_deref() == Some("slope")).find_map(|o| match o.shape {
        Shape::Ramp { width, rise, angle } => {
            let run = o.shape.length();
            Some((o.at.z + 0.5 * run, run, rise, angle.to_radians(), width, o.at.x))
        }
        _ => None,
    })
}

/// Boulder classes then formations: what a [`Placement::class`] indexes.
fn kinds() -> usize {
    CLASSES.len() + FORMATIONS.len()
}

/// A kind's name (a boulder class or a formation).
fn kind_name(kind: usize) -> &'static str {
    CLASSES.get(kind).map_or_else(|| FORMATIONS.get(kind - CLASSES.len()).map_or("?", |f| f.name), |c| c.name)
}

/// Every rock of the lab, or only `class`'s: the flat field, then the slope rocks, then the formations' row.
#[allow(clippy::cast_precision_loss)]
pub fn placements(level: &Level, class: Option<usize>) -> Vec<Placement> {
    let mut out = Vec::new();
    let yaw = |k: usize| (k as f32 * 2.399_963).rem_euclid(std::f32::consts::TAU); // golden angle steps
    for c in 0..CLASSES.len() {
        if class.is_some_and(|only| only != c) {
            continue;
        }
        let x = (c as f32 - 0.5 * (CLASSES.len() - 1) as f32) * COLUMN_SPACING_M;
        let mut z = FIELD_START_Z;
        for i in 0..FIELD_COUNT {
            let size_m = power_law_size(i, FIELD_COUNT);
            z -= 0.5 * size_m;
            out.push(Placement { class: c, size_m, ground: Vec3::new(x, 0.0, z), up: Vec3::Y, yaw: yaw(out.len()) });
            z -= 0.5 * size_m + GAP_M;
        }
        if let Some((toe, run, rise, angle, _, _)) = slope(level) {
            let up = Vec3::new(0.0, angle.cos(), angle.sin());
            for (size_m, along) in ON_SLOPE {
                let z = toe - along * run;
                out.push(Placement { class: c, size_m, ground: Vec3::new(x, rise * along, z), up, yaw: yaw(out.len()) });
            }
            for (size_m, out_m) in AT_TOE {
                out.push(Placement { class: c, size_m, ground: Vec3::new(x, 0.0, toe + out_m), up: Vec3::Y, yaw: yaw(out.len()) });
            }
        }
    }
    for (k, f) in FORMATIONS.iter().enumerate() {
        let c = CLASSES.len() + k;
        if class.is_some_and(|only| only != c) {
            continue;
        }
        let x = (k as f32 - 0.5 * (FORMATIONS.len() - 1) as f32) * FORMATION_SPACING_M;
        // Formations face the cameras (yaw 0: the arch's span runs along x).
        out.push(Placement { class: c, size_m: f.size_m, ground: Vec3::new(x, 0.0, FORMATION_ROW_Z), up: Vec3::Y, yaw: 0.0 });
    }
    out
}

/// A rock being generated.
#[derive(Component)]
struct RockTask {
    placement: Placement,
    seed: u64,
    task: Task<RockLods>,
}

/// A spawned rock's root.
#[derive(Component)]
struct LabRock;

/// A rock's LOD entity: its class and level.
#[derive(Component, Clone, Copy)]
struct LabLod {
    class: usize,
    lod: usize,
}

#[derive(Component)]
struct InfoText;

/// One texture's materials (per class, for its tint; one per LOD level) and what got bound.
struct TextureSet {
    path: String,
    materials: Vec<Vec<Handle<RockMaterial>>>,
    maps: String,
}

#[derive(Resource)]
#[allow(clippy::struct_excessive_bools)] // independent toggles
struct RockLab {
    content: RockLabContent,
    layers: RenderLayers,
    root: Option<Entity>,
    textures: Vec<TextureSet>,
    texture: usize,
    class: Option<usize>,
    seed: u64,
    wireframe: bool,
    info: bool,
    /// Crease-split normals (off: smooth SDF normals everywhere, RG-3's look).
    crease: bool,
    /// Edge snap (off: surface-nets bevelled edges, RG-3's look).
    snap: bool,
    /// Rocks spawned since the last (re)generation, their triangles per LOD summed, and when it started.
    spawned: usize,
    triangles: Vec<usize>,
    started: Option<std::time::Instant>,
    took_s: Option<f32>,
}

/// Adds the lab to a level run (after `level::add`'s worldspace setup).
pub fn add(app: &mut App, content: RockLabContent) {
    if !app.is_plugin_added::<RockgenPlugin>() {
        app.add_plugins(RockgenPlugin);
    }
    app.insert_resource(RockLab {
        content,
        layers: RenderLayers::default(),
        root: None,
        textures: Vec::new(),
        texture: 0,
        class: None,
        seed: 1,
        wireframe: false,
        info: true,
        crease: true,
        snap: sf_rockgen::boulder::tunables::EDGE_SNAP.default_value(),
        spawned: 0,
        triangles: Vec::new(),
        started: None,
        took_s: None,
    })
    .add_systems(OnEnter(sf_app_bevy::WorldAlive), spawn.after(crate::exterior::WorldSetup))
    .add_systems(Update, (keys, finish_rocks, info).chain());
}

/// Material settings and LOD distances as the store holds them now.
fn live(tunables: Option<&Tunables>) -> (BoulderParams, RockMaterialSettings, RockLodSettings) {
    tunables.map_or_else(
        || (BoulderParams::default(), RockMaterialSettings::default(), RockLodSettings::default()),
        |t| (params_from_tunables(&t.0), RockMaterialSettings::from_tunables(&t.0), RockLodSettings::from_tunables(&t.0)),
    )
}

#[allow(clippy::too_many_arguments)] // a system's parameters
fn spawn(
    mut commands: Commands,
    mut registry: ResMut<WorldspaceRegistry>,
    level: Res<super::ActiveLevel>,
    mut lab: ResMut<RockLab>,
    vfs: Res<SharedVfs>,
    server: Res<AssetServer>,
    tunables: Option<Res<Tunables>>,
    debug: Res<RockDebug>,
    mut materials: ResMut<Assets<RockMaterial>>,
) {
    let entry = registry.ensure(&mut commands, &level.0.worldspace);
    lab.layers = RenderLayers::layer(entry.layer);
    lab.root = Some(entry.root);
    let companions = TextureCompanions::read(&vfs.0);
    let (params, settings, _) = live(tunables.as_deref());
    let lods = params.lod_levels.max(1);
    let mut sets = Vec::new();
    for texture in &lab.content.textures {
        let textures = crate::rock_materials::rock_textures(texture, &vfs.0, &companions, &server);
        let maps = format!(
            "albedo {}, normal {}, orm {}{}",
            yes(textures.albedo.is_some()),
            yes(textures.normal.is_some()),
            yes(textures.orm.is_some()),
            if textures.orm_occlusion { " (+occlusion)" } else { "" }
        );
        info!("rock-lab texture {texture}: {maps} ({})", super::texture_lab::provenance(&vfs.0, texture));
        let materials = CLASSES
            .iter()
            .map(|class| class.tint)
            .chain(FORMATIONS.iter().map(|f| f.tint))
            // The class tint darkens the texture (scoria); adapter data at RG-A1.
            .map(|tint| crate::rock_materials::lod_materials(&textures, tint, &settings, lods, debug.lod_tint, &mut materials))
            .collect();
        sets.push(TextureSet { path: texture.clone(), materials, maps });
    }
    lab.textures = sets;
    commands.spawn((
        InfoText,
        Text::new(""),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgb(0.95, 0.95, 0.85)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        Node { position_type: PositionType::Absolute, bottom: Val::Px(6.0), left: Val::Px(6.0), padding: UiRect::all(Val::Px(6.0)), ..Node::default() },
    ));
    let level = level.0.level.clone();
    generate(&mut commands, &mut lab, &level, params);
    info!(
        "rock-lab: {} textures, {} classes + {} formations, {} rocks (X texture, Z class, R regenerate, L LOD tint, M wireframe, N crease normals, Q edge snap, I info)",
        lab.textures.len(),
        CLASSES.len(),
        FORMATIONS.len(),
        placements(&level, None).len()
    );
}

fn yes(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

/// Starts one generation task per placement of the current class filter and seed.
fn generate(commands: &mut Commands, lab: &mut RockLab, level: &Level, params: BoulderParams) {
    let pool = AsyncComputeTaskPool::get();
    for (i, placement) in placements(level, lab.class).into_iter().enumerate() {
        let seed = lab.seed.wrapping_mul(MIX_SEED) ^ (i as u64);
        let mut p = BoulderParams { size_m: placement.size_m, ..class_params(placement.class, params) };
        if !lab.crease {
            p.crease_deg = 180.0;
        }
        p.edge_snap = lab.snap;
        let task = match placement.class.checked_sub(CLASSES.len()).and_then(|k| FORMATIONS.get(k)) {
            Some(f) => pool.spawn(async move { generate_formation(f, &p, p.size_m, seed) }),
            None => pool.spawn(async move { generate_boulder(&p, seed) }),
        };
        commands.spawn((Name::new(format!("rock task {i}")), RockTask { placement, seed, task }));
    }
    lab.spawned = 0;
    lab.triangles.clear();
    lab.started = Some(std::time::Instant::now());
    lab.took_s = None;
}

/// Spawns each finished rock: a root on the ground, one child per LOD.
#[allow(clippy::too_many_arguments)] // a system's parameters
fn finish_rocks(
    mut commands: Commands,
    mut lab: ResMut<RockLab>,
    mut tasks: Query<(Entity, &mut RockTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    tunables: Option<Res<Tunables>>,
) {
    let Some(root) = lab.root else { return };
    let (_, _, lod_settings) = live(tunables.as_deref());
    let mut pending = 0;
    for (entity, mut job) in &mut tasks {
        let Some(lods) = check_ready(&mut job.task) else {
            pending += 1;
            continue;
        };
        commands.entity(entity).despawn();
        let p = job.placement;
        // sf_rockgen's +Z (short axis) is the mesh's +Y (`rock_mesh`): rest the rock on its flat side.
        let height = lods.bounds.extent()[2];
        let rotation = Quat::from_rotation_arc(Vec3::Y, p.up) * Quat::from_rotation_y(p.yaw);
        let sink = if p.class < CLASSES.len() { SINK } else { FORMATION_SINK };
        let centre = p.ground + p.up * (0.5 - sink) * height;
        let (cell, transform) = to_grid(WorldPos::from_f64(centre.as_dvec3()));
        let handles: Vec<Handle<Mesh>> = lods.lods.iter().map(|m| meshes.add(rock_mesh(m))).collect();
        for (k, m) in lods.lods.iter().enumerate() {
            if lab.triangles.len() <= k {
                lab.triangles.push(0);
            }
            lab.triangles[k] += m.triangle_count();
        }
        let ranges = lod_ranges(p.size_m, handles.len(), &lod_settings);
        let set = &lab.textures[lab.texture.min(lab.textures.len().saturating_sub(1))];
        #[allow(clippy::cast_possible_truncation)] // the tag only salts the texture offset
        let tag = job.seed as u32;
        let rock = commands
            .spawn((
                Name::new(format!("rock {} {:.1} m seed {}", kind_name(p.class), p.size_m, job.seed)),
                LabRock,
                cell,
                transform.with_rotation(rotation),
                Visibility::Inherited,
                ChildOf(root),
            ))
            .id();
        let class_materials = &set.materials[p.class.min(set.materials.len().saturating_sub(1))];
        for (k, bundle) in lod_bundles(&handles, class_materials, &ranges, tag).into_iter().enumerate() {
            let mut child = commands.spawn((bundle, LabLod { class: p.class, lod: k }, lab.layers.clone(), ChildOf(rock)));
            if lab.wireframe {
                child.insert(Wireframe);
            }
        }
        lab.spawned += 1;
    }
    if pending == 0
        && lab.took_s.is_none()
        && let Some(started) = lab.started
    {
        let s = started.elapsed().as_secs_f32();
        lab.took_s = Some(s);
        info!("rock-lab: {} rocks generated in {s:.2} s (seed {}), triangles per LOD {:?}", lab.spawned, lab.seed, lab.triangles);
    }
}

#[allow(clippy::too_many_arguments)] // a system's parameters
fn keys(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut lab: ResMut<RockLab>,
    mut debug: ResMut<RockDebug>,
    level: Res<super::ActiveLevel>,
    tunables: Option<Res<Tunables>>,
    rocks: Query<Entity, With<LabRock>>,
    pending: Query<Entity, With<RockTask>>,
    mut lods: Query<(Entity, &LabLod, &mut MeshMaterial3d<RockMaterial>)>,
) {
    let say = |what: &str, value: &str| println!("[A/B] rock-lab {what} {value}");
    if input.just_pressed(KeyCode::KeyX) && !lab.textures.is_empty() {
        lab.texture = (lab.texture + 1) % lab.textures.len();
        let set = &lab.textures[lab.texture];
        for (_, lod, mut material) in &mut lods {
            let Some(class) = set.materials.get(lod.class) else { continue };
            if let Some(m) = class.get(lod.lod).or_else(|| class.last()) {
                material.0 = m.clone();
            }
        }
        say("texture", &set.path);
    }
    if input.just_pressed(KeyCode::KeyL) {
        debug.lod_tint = !debug.lod_tint;
        say("LOD tint", if debug.lod_tint { "on" } else { "off" });
    }
    if input.just_pressed(KeyCode::KeyM) {
        lab.wireframe = !lab.wireframe;
        for (entity, _, _) in &lods {
            if lab.wireframe {
                commands.entity(entity).insert(Wireframe);
            } else {
                commands.entity(entity).remove::<Wireframe>();
            }
        }
        say("wireframe", if lab.wireframe { "on" } else { "off" });
    }
    if input.just_pressed(KeyCode::KeyI) {
        lab.info = !lab.info;
    }
    let next_class = input.just_pressed(KeyCode::KeyZ);
    let reseed = input.just_pressed(KeyCode::KeyR);
    let crease = input.just_pressed(KeyCode::KeyN);
    let snap = input.just_pressed(KeyCode::KeyQ);
    if crease {
        lab.crease = !lab.crease;
        say("crease normals", if lab.crease { "on" } else { "off (smooth SDF normals)" });
    }
    if snap {
        lab.snap = !lab.snap;
        say("edge snap", if lab.snap { "on" } else { "off (bevelled surface nets)" });
    }
    if !(next_class || reseed || crease || snap) {
        return;
    }
    if next_class {
        lab.class = match lab.class {
            None => Some(0),
            Some(c) if c + 1 < kinds() => Some(c + 1),
            Some(_) => None,
        };
        say("class", lab.class.map_or("all", kind_name));
    }
    if reseed {
        lab.seed = lab.seed.wrapping_add(1);
        say("seed", &lab.seed.to_string());
    }
    for entity in rocks.iter().chain(pending.iter()) {
        commands.entity(entity).despawn();
    }
    let (params, _, _) = live(tunables.as_deref());
    let level = level.0.level.clone();
    generate(&mut commands, &mut lab, &level, params);
}

fn info(lab: Res<RockLab>, tunables: Option<Res<Tunables>>, debug: Res<RockDebug>, mut text: Query<(&mut Text, &mut Visibility), With<InfoText>>) {
    let Ok((mut text, mut visibility)) = text.single_mut() else { return };
    *visibility = if lab.info { Visibility::Inherited } else { Visibility::Hidden };
    if !lab.info {
        return;
    }
    let (p, m, l) = live(tunables.as_deref());
    let set = lab.textures.get(lab.texture);
    let s = format!(
        "rock lab  seed {}  class {}  rocks {}{}\n\
         texture {} ({})  [X next]\n\
         triangles per LOD {:?}  LOD1 at {:.0} x size, x{:.1} per level  LOD tint {} [L]  wireframe {} [M]\n\
         crease normals {} [N] ({:.0} deg)  edge snap {} [Q]  (eroded: block platy cobble chipped | sharp: talus jagged columnar scoria slab | formations: hoodoo pinnacle sea_stack volcanic_neck arch, camera `forms`)\n\
         boulder: resolution {} cells (up to {} by size), joints {}, chips {}, rounding {:.2}, weather {:.2}, LOD0 error {:.2} cell\n\
         material: blend {:.0}, hex tiling {} (rotation {:.2}, contrast {:.0})\n\
         Z class, R new seed (console `set rockgen.boulder.* ...` then R), I hide",
        lab.seed,
        lab.class.map_or("all", kind_name),
        lab.spawned,
        lab.took_s.map_or_else(|| "  generating...".to_owned(), |s| format!("  ({s:.2} s)")),
        set.map_or("none", |s| s.path.as_str()),
        set.map_or("", |s| s.maps.as_str()),
        lab.triangles,
        l.lod1_distance,
        l.growth,
        if debug.lod_tint { "on" } else { "off" },
        if lab.wireframe { "on" } else { "off" },
        if lab.crease { "on" } else { "off" },
        p.crease_deg,
        if lab.snap { "on" } else { "off" },
        p.resolution,
        p.resolution_max,
        p.joint_sets,
        p.chips,
        p.rounding,
        p.weather_amp,
        p.lod0_error_voxels,
        m.blend_sharpness,
        if m.hex_tiling { "on" } else { "off" },
        m.hex_rotation,
        m.hex_contrast,
    );
    if text.0 != s {
        text.0 = s;
    }
}
