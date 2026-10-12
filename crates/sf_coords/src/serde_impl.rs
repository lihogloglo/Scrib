//! Text and serde forms.
//!
//! Serializes as `{ cell = [x, y, z], local = [x, y, z] }`. Deserializes
//! that form or plain absolute meters `[x, y, z]` (f64), so mod authors
//! never do cell math by hand. Binary formats (postcard, used by saves and bake
//! products) carry only the grid form: untagged enums need a self-describing
//! format.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{WorldPos, WorldTransform};
use glam::{DVec3, IVec3, Quat, Vec3};

#[derive(Serialize, Deserialize)]
struct Grid {
    cell: [i32; 3],
    local: [f32; 3],
}

#[derive(Deserialize)]
#[serde(untagged)]
enum PosIn {
    Grid { cell: [i32; 3], local: [f32; 3] },
    Absolute([f64; 3]),
}

impl Serialize for WorldPos {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Grid { cell: self.cell.to_array(), local: self.local.to_array() }.serialize(s)
    }
}

impl<'de> Deserialize<'de> for WorldPos {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        if !d.is_human_readable() {
            let Grid { cell, local } = Grid::deserialize(d)?;
            return Ok(WorldPos::new(IVec3::from_array(cell), Vec3::from_array(local)));
        }
        Ok(match PosIn::deserialize(d)? {
            PosIn::Grid { cell, local } => WorldPos::new(IVec3::from_array(cell), Vec3::from_array(local)),
            PosIn::Absolute(p) => WorldPos::from_f64(DVec3::from_array(p)),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct TransformRepr {
    pos: WorldPos,
    #[serde(default = "identity_rotation")]
    rotation: [f32; 4],
    #[serde(default = "unit_scale")]
    scale: [f32; 3],
}

fn identity_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn unit_scale() -> [f32; 3] {
    [1.0; 3]
}

impl Serialize for WorldTransform {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        TransformRepr { pos: self.pos, rotation: self.rotation.to_array(), scale: self.scale.to_array() }.serialize(s)
    }
}

impl<'de> Deserialize<'de> for WorldTransform {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let r = TransformRepr::deserialize(d)?;
        Ok(WorldTransform {
            pos: r.pos,
            rotation: Quat::from_array(r.rotation).normalize(),
            scale: Vec3::from_array(r.scale),
        })
    }
}
