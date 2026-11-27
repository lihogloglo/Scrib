//! Library rocks as streamed placements (spec `rockgen.md` "(f) Placement in the game", point 6, slice RG-P3).
//!
//! [`RockSpawner`] is the [`PlacementSpawner`] of the content pack's library-rock class: a placement whose model id
//! is `rockgen/<class>/<index>` ([`sf_rockgen::placed::model_id`]) draws variant `index` of `class` with one entity
//! per LOD level. Every variant's LOD meshes are built once, when the spawner is made, and shared by every rock of
//! that variant (one `Mesh` and one material handle per level batch into one instanced draw); per class one
//! [`RockMaterial`] per level. The ids are never mounted, so [`PlacementSpawner::needs_model`] is false.
//!
//! Ranges: the rock LOD law ([`lod_ranges`] over the rock's size, `rockgen.lod.*`), the last kept level ended at
//! the placement's (size-culled) end with its fade band, as [`sf_world_bevy::chain_ranges`] ends a static chain,
//! and every bound snapped by [`shared_range`] (Bevy's 65535 range slots). The parts carry no `LodPart`: a live
//! `LodScreen` change does not re-range them (rocks follow their own law). Parts sit directly under the cell's
//! level entity with the placement's transform, as the framework's statics do (no extra root entity per rock).

use std::collections::HashMap;

use bevy_asset::{Assets, Handle};
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::{NoAutoAabb, NoCpuCulling, VisibilityRange};
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_mesh::{Mesh, Mesh3d, MeshTag};
use bevy_pbr::MeshMaterial3d;
use sf_rockgen::placed::{Libraries, parse_model_id};
use sf_world::IndexedPlacement;
use sf_world_bevy::{PlacementSpawner, SpawnCx, StaticGpuCulling, shared_range};

use crate::{RockLodSettings, RockMaterial, lod_ranges, rock_mesh};

/// Rock materials per class, one handle per LOD level (a level past the end takes the last one). A class without
/// its own row takes `default`. Built by the adapter (texture sets are content).
#[derive(Clone, Debug, Default)]
pub struct RockMaterialSet {
    pub default: Vec<Handle<RockMaterial>>,
    pub by_class: HashMap<String, Vec<Handle<RockMaterial>>>,
}

impl RockMaterialSet {
    /// The levels' materials of `class`.
    #[must_use]
    pub fn for_class(&self, class: &str) -> &[Handle<RockMaterial>] {
        self.by_class.get(class).unwrap_or(&self.default)
    }
}

/// One variant's shared meshes.
#[derive(Clone, Debug)]
struct VariantParts {
    /// LOD0 first.
    lods: Vec<Handle<Mesh>>,
    /// LOD0 bounds in the engine frame (every part's `Aabb`: no CPU mesh scan).
    aabb: Aabb,
}

/// One class's variants and materials.
#[derive(Clone, Debug)]
struct ClassParts {
    /// The library's reference size (longest LOD0 side), m: a rock's size is this times its scale.
    size_m: f32,
    variants: Vec<VariantParts>,
    materials: Vec<Handle<RockMaterial>>,
}

/// The spawner of library-rock placements (see the module docs).
#[derive(Clone, Debug)]
pub struct RockSpawner {
    classes: HashMap<String, ClassParts>,
    settings: RockLodSettings,
}

/// `sf_rockgen` bounds (+Z the short axis) in Bevy's frame: (x, y, z) -> (x, z, -y), as [`rock_mesh`].
fn engine_aabb(b: &sf_rockgen::Aabb) -> Aabb {
    Aabb::from_min_max(Vec3::new(b.min[0], b.min[2], -b.max[1]), Vec3::new(b.max[0], b.max[2], -b.min[1]))
}

impl RockSpawner {
    /// Builds every variant's LOD meshes into `meshes` once and keeps the handles; `materials` per class.
    pub fn new(libraries: &Libraries, meshes: &mut Assets<Mesh>, materials: &RockMaterialSet, settings: RockLodSettings) -> Self {
        let classes = libraries
            .classes()
            .filter_map(|name| {
                let library = libraries.get(name)?;
                let variants = library
                    .variants
                    .iter()
                    .map(|v| VariantParts {
                        lods: v.lods.lods.iter().map(|m| meshes.add(rock_mesh(m))).collect(),
                        aabb: engine_aabb(&v.lods.bounds),
                    })
                    .collect();
                let parts = ClassParts { size_m: library.size_m, variants, materials: materials.for_class(name).to_vec() };
                Some((name.to_owned(), parts))
            })
            .collect();
        Self { classes, settings }
    }

    /// Mesh handles held (every LOD of every variant), for the boot summary.
    #[must_use]
    pub fn mesh_count(&self) -> usize {
        self.classes.values().flat_map(|c| &c.variants).map(|v| v.lods.len()).sum()
    }

    /// The unsnapped range of each drawn level of a rock `size_m` long (`levels` LODs) ending at `max_distance`
    /// with a fade band `end_band` (`[max, max + band]`; 0: abrupt): the rock LOD law, levels that would start past
    /// the end dropped, the last kept one ended there. A non-positive or infinite `max_distance` keeps the open end.
    #[must_use]
    pub fn part_ranges(&self, size_m: f32, levels: usize, max_distance: f32, end_band: f32) -> Vec<VisibilityRange> {
        let mut ranges = lod_ranges(size_m, levels, &self.settings);
        if max_distance > 0.0 && max_distance.is_finite() {
            ranges.truncate(ranges.iter().take_while(|r| r.start_margin.start < max_distance).count());
            if let Some(last) = ranges.last_mut() {
                last.end_margin = max_distance..max_distance + end_band.max(0.0);
            }
        }
        ranges
    }
}

impl PlacementSpawner for RockSpawner {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        let Some(model) = &placement.placement.model else { return 0 };
        let Some((class, index)) = parse_model_id(model.as_str()) else { return 0 };
        let Some(parts) = self.classes.get(class) else { return 0 };
        let Some(variant) = usize::try_from(index).ok().and_then(|i| parts.variants.get(i)) else { return 0 };
        let size_m = parts.size_m * cx.local.scale.max_element();
        let ranges = self.part_ranges(size_m, variant.lods.len(), cx.max_distance, cx.end_band);
        // The texture offset: the placement id's low 32 bits.
        let tag = MeshTag::new(u32::try_from(placement.placement.id.0 & u128::from(u32::MAX)).unwrap_or(0));
        let bundles: Vec<_> = variant
            .lods
            .iter()
            .zip(ranges)
            .enumerate()
            .filter_map(|(k, (mesh, range))| {
                let material = parts.materials.get(k).or_else(|| parts.materials.last())?;
                Some((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    shared_range(range),
                    tag.clone(),
                    cx.local,
                    cx.layers.clone(),
                    variant.aabb,
                    NoAutoAabb,
                    ChildOf(cx.parent),
                ))
            })
            .collect();
        let n = u32::try_from(bundles.len()).unwrap_or(u32::MAX);
        // Library rocks never move: GPU-culled like statics when the app allows it.
        if StaticGpuCulling::on(world) {
            world.spawn_batch(bundles.into_iter().map(|b| (b, NoCpuCulling)));
        } else {
            world.spawn_batch(bundles);
        }
        n
    }

    fn needs_model(&self, _placement: &IndexedPlacement) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
