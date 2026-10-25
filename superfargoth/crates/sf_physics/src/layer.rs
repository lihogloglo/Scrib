//! Collision layers and the pair matrix (physics.md "Data model": layers).

use bitflags::bitflags;
use rapier3d::geometry::{Group, InteractionGroups, InteractionTestMode};

bitflags! {
    /// What a collider is. Each body has exactly one layer; queries and the
    /// pair matrix take sets.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Layer: u32 {
        /// Placed static geometry (`collision.cook` meshes).
        const STATIC = 1 << 0;
        /// Terrain heightfield chunks.
        const TERRAIN = 1 << 1;
        /// Light, small dynamic items: pushed by characters, never block or carry them.
        const DEBRIS = 1 << 2;
        /// Every other dynamic item: blocks movers, pushed with a capped force.
        const PROP = 1 << 3;
        /// Characters' kinematic capsules.
        const CHARACTER = 1 << 4;
        /// Ragdoll bodies.
        const RAGDOLL = 1 << 5;
        /// The carried item.
        const HELD = 1 << 6;
        /// Triggers: report overlaps, never push.
        const SENSOR = 1 << 7;
        const PROJECTILE = 1 << 8;
        /// Powered ragdoll bodies (a crumpling death, a knockdown before it
        /// goes limp): contacts as `RAGDOLL`, outside the carry pick layers.
        const POWERED = 1 << 9;
    }
}

impl Layer {
    /// Fixed world geometry.
    pub const WORLD: Self = Self::STATIC.union(Self::TERRAIN);
    /// Everything the solver moves.
    pub const MOVING: Self =
        Self::DEBRIS.union(Self::PROP).union(Self::RAGDOLL).union(Self::HELD).union(Self::PROJECTILE).union(Self::POWERED);

    /// The layers this one makes contact with. Symmetric by construction
    /// (`a.collides_with().contains(b) == b.collides_with().contains(a)`).
    ///
    /// Fixed geometry touches everything that moves (characters run on the
    /// kinematic mover's own sweeps, not on contacts with the world).
    /// Characters touch moving bodies so falling and thrown things hit them;
    /// the mover's capped pushes (task P5) decide what they do to debris.
    /// A held item's contact with its own holder is excluded per pair
    /// (task P6), not by layer.
    #[must_use]
    pub fn collides_with(self) -> Self {
        let mut out = Self::empty();
        for layer in self.iter() {
            out |= match layer {
                Self::STATIC | Self::TERRAIN | Self::CHARACTER => Self::MOVING | Self::SENSOR,
                Self::SENSOR => Self::all().difference(Self::SENSOR),
                _ => Self::WORLD | Self::MOVING | Self::CHARACTER | Self::SENSOR,
            };
        }
        out
    }

    pub(crate) fn groups(self) -> InteractionGroups {
        InteractionGroups::new(
            Group::from_bits_truncate(self.bits()),
            Group::from_bits_truncate(self.collides_with().bits()),
            InteractionTestMode::And,
        )
    }

    /// Query filter groups: hits any collider whose layer is in `self`.
    pub(crate) fn query_groups(self) -> InteractionGroups {
        InteractionGroups::new(Group::ALL, Group::from_bits_truncate(self.bits()), InteractionTestMode::And)
    }
}
