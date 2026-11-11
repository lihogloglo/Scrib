//! Opt-in corrections for defects that upstream leaves in place.
//!
//! The port matches recastnavigation by default, including its wrong but defined
//! behaviour, so that every record stays comparable with the reference driver.
//! [`UpstreamFixes`] switches individual corrections on. Each flag is off in
//! [`UpstreamFixes::NONE`], which is the default everywhere, and results with no
//! flag set are bit-identical to the ones recorded before the flag existed.

use core::ops::{BitAnd, BitOr, BitOrAssign};

/// A set of upstream defect corrections, as a bit set.
///
/// Carried by `BakeConfig` (`fixes`), `NavMeshQuery::set_fixes`,
/// `Crowd::set_fixes` and `ObstacleAvoidanceQuery::set_fixes`; a component
/// ignores flags that concern another component. Unknown bits cannot be built
/// through the public constructors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UpstreamFixes(u32);

impl UpstreamFixes {
    /// No correction: upstream behaviour.
    pub const NONE: Self = Self(0);

    /// Sliced A* keys its nodes by (polygon reference, cross side), as the
    /// unsliced search does, instead of by reference alone. With the flag the
    /// budgeted, sliced and plain searches return the same corridor on tiled
    /// meshes.
    pub const SLICED_CROSS_SIDE: Self = Self(1 << 0);

    /// The crowd keeps the nearest neighbours of an agent. Upstream stops
    /// collecting at the first 32 candidates in proximity-grid order and keeps the
    /// nearest of those. With the flag every candidate in range is collected (the
    /// grid walk is charged to the workspace) and the nearest are kept, ordered by
    /// distance then agent index.
    pub const NEAREST_NEIGHBOURS: Self = Self(1 << 1);

    /// Adaptive velocity sampling writes the last sample of every ring in its own
    /// slot for even division counts. Upstream writes it one slot too far and
    /// samples an unwritten slot (zero in the port).
    pub const ADAPTIVE_PATTERN: Self = Self(1 << 2);

    /// A detail sample distance of zero builds the detail mesh from the polygon
    /// hull vertices only. Upstream's `triangulateHull` never terminates on the
    /// empty hull this case produces; the port, without the flag, emits a polygon
    /// with no detail triangles.
    pub const ZERO_SAMPLE_DIST: Self = Self(1 << 3);

    /// Every correction this version defines.
    pub const ALL: Self = Self(
        Self::SLICED_CROSS_SIDE.0
            | Self::NEAREST_NEIGHBOURS.0
            | Self::ADAPTIVE_PATTERN.0
            | Self::ZERO_SAMPLE_DIST.0,
    );

    /// Whether every flag of `other` is set.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Whether no flag is set.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// The raw bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Builds a set from raw bits, or `None` if a bit is not a defined flag.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if bits & !Self::ALL.0 == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
}

impl BitOr for UpstreamFixes {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for UpstreamFixes {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for UpstreamFixes {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
