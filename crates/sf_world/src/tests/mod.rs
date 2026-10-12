#![allow(clippy::float_cmp)] // exact values are the point in these tests

use glam::{DVec3, Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_vfs::VfsPath;

use crate::*;

#[cfg(feature = "bake")]
mod bake;
mod index;
mod overlay;
mod reader;
mod records;
mod scheduler;

/// Deterministic pseudo-random numbers for tests (64-bit LCG, upper bits).
struct Lcg(u64);

impl Lcg {
    #[allow(clippy::cast_possible_truncation)]
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as u32
    }

    /// Uniform in `[lo, hi)`.
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * f64::from(self.next_u32()) / f64::from(u32::MAX)
    }
}

fn ws(name: &str) -> WorldspaceId {
    WorldspaceId::new(name)
}

fn path(p: &str) -> VfsPath {
    VfsPath::new(p).unwrap()
}

fn placement(key: &str, pos: DVec3, model: Option<&str>, class: PlacementClass) -> Placement {
    Placement {
        id: PlacementId::from_key("test", key.as_bytes()),
        base: RecordRef::new("thing", key),
        model: model.map(path),
        transform: WorldTransform::from_pos(WorldPos::from_f64(pos)),
        class,
        flags: PlacementFlags::empty(),
    }
}

/// Test models: `small` (1 m cube), `big` (20 × 8 × 20 m), `tower`
/// (10 × 300 × 10 m), `block` (a full 128 m cell footprint, 10 m tall).
fn models(p: &VfsPath) -> Option<ModelInfo> {
    let (half, bytes) = match p.as_str() {
        "small.mesh" => (Vec3::splat(0.5), 10_000),
        "big.mesh" => (Vec3::new(10.0, 4.0, 10.0), 500_000),
        "tower.mesh" => (Vec3::new(5.0, 150.0, 5.0), 2_000_000),
        "block.mesh" => (Vec3::new(64.0, 5.0, 64.0), 1_000_000),
        _ => return None,
    };
    Some(ModelInfo { min: -half, max: half, bytes, lod_distance: 500.0 })
}

fn rotated(mut p: Placement, angle: f32, scale: Vec3) -> Placement {
    p.transform.rotation = Quat::from_rotation_y(angle) * Quat::from_rotation_x(angle * 0.5);
    p.transform.scale = scale;
    p
}
