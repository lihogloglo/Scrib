//! World ↔ character-space conversions for animation inputs.

use glam::{Quat, Vec3};
use sf_anim::RootDelta;
use sf_coords::WorldPos;

use crate::probe::{GroundHit, ProbeQuery};

/// Where a character's root is drawn and which way it faces: the frame of
/// the animation graph's character space (Y up, facing +Z).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CharacterFrame {
    pub position: WorldPos,
    pub yaw: f32,
}

impl CharacterFrame {
    pub fn new(position: WorldPos, yaw: f32) -> Self {
        Self { position, yaw }
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }

    /// A world position in character space.
    pub fn to_local(&self, p: WorldPos) -> Vec3 {
        self.rotation().inverse() * self.position.delta_f32(p)
    }

    /// A world direction in character space.
    pub fn dir_to_local(&self, v: Vec3) -> Vec3 {
        self.rotation().inverse() * v
    }

    /// A character-space point in the world.
    pub fn to_world(&self, p: Vec3) -> WorldPos {
        self.position.offset(self.rotation() * p)
    }

    /// How the character moved from `self` to `next`, in `self`'s frame:
    /// the graph's `UpdateInputs::moved`.
    pub fn delta_to(&self, next: &Self) -> RootDelta {
        RootDelta::new(self.to_local(next.position), crate::motion::wrap_angle(next.yaw - self.yaw))
    }

    /// Downward ray probes under character-space points (the graph's
    /// per-leg `probe_origins`): from `up` above each point, `up + down` long.
    pub fn foot_queries<'a>(&'a self, origins: &'a [Vec3], up: f32, down: f32) -> impl Iterator<Item = ProbeQuery> + 'a {
        origins.iter().map(move |&o| ProbeQuery::down(self.to_world(o + Vec3::Y * up), up + down))
    }

    /// A probe hit as a character-space ground sample: (point, normal).
    pub fn ground_sample(&self, hit: &GroundHit) -> (Vec3, Vec3) {
        (self.to_local(hit.point), self.dir_to_local(hit.normal))
    }
}
