//! Bevy side of `sf_rockgen` (spec `docs/specs/rockgen.md`, slice RG-3): rock meshes as Bevy [`Mesh`]es, one
//! entity per LOD with a Bevy [`VisibilityRange`] (engine LOD crossfade: dithered, distance to the camera), and
//! [`RockMaterial`], a world-axis triplanar PBR material so rocks need no UV unwrap.
//!
//! ```ignore
//! app.add_plugins(RockgenPlugin);
//! let lods = sf_rockgen::generate_boulder(&params_from_tunables(&store), seed);
//! let meshes: Vec<_> = lods.lods.iter().map(|m| mesh_assets.add(rock_mesh(m))).collect();
//! let ranges = lod_ranges(size_m, meshes.len(), &RockLodSettings::from_tunables(&store));
//! for bundle in lod_bundles(&meshes, &materials, &ranges, seed as u32) { commands.spawn((bundle, ChildOf(rock))); }
//! ```
//!
//! The triplanar code is a named temporary scaffold (`shaders/rock.wesl` header: "rock triplanar scaffold"),
//! replaced by a shared WESL module with the terrain in slice RG-3b. Nothing here names a game: texture sets
//! come from the adapter as [`RockTextures`].

mod material;
pub mod tunables;

#[cfg(test)]
mod shader_tests;
#[cfg(test)]
mod tests;

pub use material::{
    FLAG_ALBEDO, FLAG_FLIP_Y, FLAG_HEX, FLAG_LOD_TINT, FLAG_NORMAL, FLAG_OCCLUSION, FLAG_ORM, ROCK_SHADER, RockExt, RockMaterial,
    RockMaterialSettings, RockParams, RockTextures, rock_material,
};

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Assets, Handle, RenderAssetUsages, embedded_asset};
use bevy_camera::visibility::VisibilityRange;
use bevy_ecs::prelude::*;
use bevy_mesh::{Indices, Mesh, Mesh3d, MeshTag, PrimitiveTopology};
use bevy_pbr::{MaterialPlugin, MeshMaterial3d};
use bevy_transform::components::Transform;
use sf_rockgen::boulder::tunables as boulder;
pub use sf_rockgen::{CLASSES, Family, RockClass};
use sf_rockgen::{BoulderParams, RockMesh};
use sf_tunables::{Def, Type};
use sf_tunables_bevy::{Tunables, TunablesAppExt};

/// The far end of the last LOD's range: never culled by distance (a value, not a tunable: "no end").
// tunables: const (maths: an open range end, not a distance anyone tunes)
const OPEN_END_M: f32 = 1.0e7;

/// Rock meshes and materials: the material plugin, the shader, and the `rockgen.*` tunables (the boulder
/// shape set from `sf_rockgen` and this crate's material and LOD set) registered live in the store.
pub struct RockgenPlugin;

impl Plugin for RockgenPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/rock.wesl");
        app.add_plugins(MaterialPlugin::<RockMaterial>::default())
            .register_tunables(boulder::TUNABLES)
            .register_tunables(tunables::TUNABLES)
            .init_resource::<RockDebug>()
            .add_systems(Update, sync_materials);
    }
}

/// Viewer switches for every [`RockMaterial`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RockDebug {
    /// Tint each LOD by its level (LOD0 green, then yellow, orange, red, ...).
    pub lod_tint: bool,
}

/// Applies the live `rockgen.material.*` values and [`RockDebug`] to every rock material when either changes.
fn sync_materials(tunables: Option<Res<Tunables>>, debug: Res<RockDebug>, mut materials: ResMut<Assets<RockMaterial>>) {
    let tunables_changed = tunables.as_ref().is_some_and(Res::is_changed);
    if !tunables_changed && !debug.is_changed() {
        return;
    }
    let settings = tunables.as_ref().map_or_else(RockMaterialSettings::default, |t| RockMaterialSettings::from_tunables(&t.0));
    for (_, m) in materials.iter_mut() {
        let mut params = m.extension.params;
        settings.apply(&mut params, debug.lod_tint);
        if params != m.extension.params {
            m.extension.params = params;
        }
    }
}

/// A def's value in `store`, or its default when nobody registered it.
pub fn read<T: Type + Sync + Copy>(store: &sf_tunables::Tunables, def: &Def<T>) -> T
where
    for<'a> T::Read<'a>: Into<T>,
{
    store.id_of(def).map_or_else(|_| def.default_value(), |id| store.read(id).into())
}

/// The boulder parameters the store holds (`rockgen.boulder.*`, live in the console; a regenerate reads them).
pub fn params_from_tunables(store: &sf_tunables::Tunables) -> BoulderParams {
    BoulderParams {
        size_m: read(store, &boulder::SIZE),
        aspect: read(store, &boulder::ASPECT),
        joint_sets: read(store, &boulder::JOINT_SETS),
        joint_jitter_deg: read(store, &boulder::JOINT_JITTER),
        chips: read(store, &boulder::CHIPS),
        chip_depth: read(store, &boulder::CHIP_DEPTH),
        chip_spread: read(store, &boulder::CHIP_SPREAD),
        rounding: read(store, &boulder::ROUNDING),
        weather_amp: read(store, &boulder::WEATHER_AMP),
        weather_wavelength: read(store, &boulder::WEATHER_WAVELENGTH),
        weather_octaves: read(store, &boulder::WEATHER_OCTAVES),
        resolution: read(store, &boulder::RESOLUTION),
        lod0_error_voxels: read(store, &boulder::LOD0_ERROR),
        lod_levels: read(store, &boulder::LOD_LEVELS),
        lod_ratio: read(store, &boulder::LOD_RATIO),
        edge_snap: read(store, &boulder::EDGE_SNAP),
        crease_deg: read(store, &boulder::CREASE),
        pit_density: read(store, &boulder::PIT_DENSITY),
        pit_size: read(store, &boulder::PIT_SIZE),
        columns: read(store, &boulder::COLUMNS),
        column_jitter: read(store, &boulder::COLUMN_JITTER),
        column_groove: read(store, &boulder::COLUMN_GROOVE),
        column_step: read(store, &boulder::COLUMN_STEP),
        column_tilt_deg: read(store, &boulder::COLUMN_TILT),
        column_fuse: read(store, &boulder::COLUMN_FUSE),
        resolution_ref_m: read(store, &boulder::RESOLUTION_REF),
        resolution_size_exp: read(store, &boulder::RESOLUTION_SIZE_EXP),
        resolution_max: read(store, &boulder::RESOLUTION_MAX),
        lod0_error_max_m: read(store, &boulder::LOD0_ERROR_MAX),
    }
    .clamped()
}

/// [`CLASSES`]`[class]` (`sf_rockgen::classes`: the eroded family block, platy, cobble, chipped, then the sharp
/// family talus, jagged, columnar, scoria, slab) over `base`; an index past the end gives `base`. Per-rock-type
/// values are adapter data at RG-A1; these show the range.
pub fn class_params(class: usize, base: BoulderParams) -> BoulderParams {
    CLASSES.get(class).map_or(base, |c| c.params(base))
}

/// A rock mesh as a Bevy mesh: positions, normals, `u32` indices. `sf_rockgen` builds rocks with +Z the short
/// axis; Bevy is +Y up, so (x, y, z) maps to (x, z, -y) (a rotation: winding and handedness kept), and a rock
/// lies on its flat side.
pub fn rock_mesh(m: &RockMesh) -> Mesh {
    let up = |[x, y, z]: [f32; 3]| [x, z, -y];
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, m.positions.iter().copied().map(up).collect::<Vec<_>>())
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, m.normals.iter().copied().map(up).collect::<Vec<_>>())
        .with_inserted_indices(Indices::U32(m.indices.clone()))
}

/// LOD switch distances (`rockgen.lod.*`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RockLodSettings {
    /// LOD1 starts at this many rock sizes from the camera.
    pub lod1_distance: f32,
    /// Each further switch is this many times farther.
    pub growth: f32,
    /// Crossfade half-width as a fraction of the switch distance.
    pub fade: f32,
}

impl Default for RockLodSettings {
    fn default() -> Self {
        Self {
            lod1_distance: tunables::LOD1_DISTANCE.default_value(),
            growth: tunables::LOD_GROWTH.default_value(),
            fade: tunables::LOD_FADE.default_value(),
        }
    }
}

impl RockLodSettings {
    pub fn from_tunables(store: &sf_tunables::Tunables) -> Self {
        Self {
            lod1_distance: read(store, &tunables::LOD1_DISTANCE),
            growth: read(store, &tunables::LOD_GROWTH),
            fade: read(store, &tunables::LOD_FADE),
        }
    }

    /// The distance where LOD `k` (≥ 1) takes over from LOD `k - 1`.
    pub fn switch_m(&self, size_m: f32, k: usize) -> f32 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // k ≤ 8
        let steps = k.saturating_sub(1) as i32;
        size_m * self.lod1_distance * self.growth.powi(steps)
    }
}

/// One [`VisibilityRange`] per LOD of a rock `size_m` long: LOD0 from 0, LOD k from switch k to switch k + 1,
/// the last one open-ended; neighbours overlap by the dithered crossfade (all LODs share one origin, as
/// Bevy's crossfade needs).
pub fn lod_ranges(size_m: f32, levels: usize, s: &RockLodSettings) -> Vec<VisibilityRange> {
    let margin = |d: f32| {
        let f = d * s.fade;
        (d - f)..(d + f)
    };
    (0..levels)
        .map(|k| VisibilityRange {
            start_margin: if k == 0 { 0.0..0.0 } else { margin(s.switch_m(size_m, k)) },
            end_margin: if k + 1 == levels { OPEN_END_M..OPEN_END_M } else { margin(s.switch_m(size_m, k + 1)) },
            use_aabb: false,
        })
        .collect()
}

/// The components of each LOD entity of one rock (spawn them as children of the rock's root, identity local
/// transform): mesh, material (`materials[k]`, or the last one), range, and `tag` (the per-rock texture offset).
pub fn lod_bundles(
    meshes: &[Handle<Mesh>],
    materials: &[Handle<RockMaterial>],
    ranges: &[VisibilityRange],
    tag: u32,
) -> Vec<(Mesh3d, MeshMaterial3d<RockMaterial>, VisibilityRange, MeshTag, Transform)> {
    meshes
        .iter()
        .zip(ranges)
        .enumerate()
        .filter_map(|(k, (mesh, range))| {
            let material = materials.get(k).or_else(|| materials.last())?;
            Some((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), range.clone(), MeshTag::new(tag), Transform::default()))
        })
        .collect()
}
