//! [`HeightSurface`] over `sf_terrain`'s CPU height queries (terrain.md:
//! base + persistent edits, finest resident level).

use glam::Vec3;
use sf_coords::WorldPos;
use sf_terrain::{TerrainHeights, WorldRay};

use crate::probe::HeightSurface;

impl HeightSurface for TerrainHeights {
    fn height(&self, p: WorldPos) -> Option<f32> {
        TerrainHeights::height(self, p)
    }
    fn normal(&self, p: WorldPos) -> Option<Vec3> {
        TerrainHeights::normal(self, p)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        TerrainHeights::raycast(self, WorldRay { origin, dir }, max).map(|h| (h.distance, h.normal))
    }
}
