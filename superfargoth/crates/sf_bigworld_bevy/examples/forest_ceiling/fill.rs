//! The bench's instances and slot bytes, filled once on the CPU: the `n` trees of a flat synthetic forest nearest a
//! viewer at the world origin, each packed as one 64-byte `GpuMeshInstance` slot (the layout of
//! `sf_groundcover_bevy::residency::slot_words`), and the arms (batch sets) the bench can run.

use std::cmp::Ordering;
use std::f64::consts::PI;

use bevy::math::{Mat3, Vec3};
use sf_bigworld::{Instance, InstanceSource, SyntheticForest, TileKey};

/// Fixture: tile edge in metres.
pub const TILE_M: f32 = 256.0;
/// Fixture: the forest's density, trees per hectare (the High preset's).
pub const DENSITY_PER_HA: f32 = 400.0;
/// Fixture: bytes of one slot (`GpuMeshInstance`).
pub const SLOT_BYTES: usize = 64;
/// Fixture: the forest's seed.
const SEED: u64 = 0xf0_2e57;
/// Fixture: square metres per hectare (a unit).
const M2_PER_HA: f64 = 10_000.0;

/// The bench forest: flat, one species, scale 0.8 to 1.2.
pub fn forest() -> SyntheticForest {
    SyntheticForest {
        seed: SEED,
        density_per_ha: DENSITY_PER_HA,
        species_count: 1,
        scale: [0.8, 1.2],
        relief_m: 0.0,
        relief_wavelength_m: 1_000.0,
    }
}

/// One tree in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placed {
    /// World position in metres; y is the ground.
    pub world: [f32; 3],
    /// Radians about +Y.
    pub yaw: f32,
    /// Uniform scale.
    pub scale: f32,
}

impl Placed {
    fn horizontal_distance_sq(&self) -> f64 {
        let [x, _, z] = self.world.map(f64::from);
        x * x + z * z
    }
}

/// The `n` trees nearest the world origin, nearest first (ties broken by position bits, so the order is total and
/// deterministic).
pub fn nearest_instances(n: u32) -> Vec<Placed> {
    if n == 0 {
        return Vec::new();
    }
    let per_m2 = f64::from(DENSITY_PER_HA) / M2_PER_HA;
    let diagonal = f64::from(TILE_M) * std::f64::consts::SQRT_2;
    let r = (f64::from(n) / (PI * per_m2)).sqrt() * 1.1 + diagonal;
    let lo = TileKey::containing(-r, -r, TILE_M);
    let hi = TileKey::containing(r, r, TILE_M);
    let source = forest();
    let mut tile = Vec::<Instance>::new();
    let mut all = Vec::new();
    for tz in lo.z..=hi.z {
        for tx in lo.x..=hi.x {
            let key = TileKey::new(tx, tz);
            if key.distance_to(0.0, 0.0, TILE_M) > r {
                continue;
            }
            tile.clear();
            source.tile(key, TILE_M, &mut tile);
            let [cx, cz] = key.min_corner(TILE_M);
            all.extend(tile.iter().map(|i| Placed {
                world: [(cx + f64::from(i.pos[0])) as f32, i.pos[1], (cz + f64::from(i.pos[2])) as f32],
                yaw: i.yaw,
                scale: i.scale,
            }));
        }
    }
    let order = |a: &Placed, b: &Placed| -> Ordering {
        a.horizontal_distance_sq()
            .total_cmp(&b.horizontal_distance_sq())
            .then_with(|| a.world[0].to_bits().cmp(&b.world[0].to_bits()))
            .then_with(|| a.world[2].to_bits().cmp(&b.world[2].to_bits()))
    };
    let n = n as usize;
    if all.len() > n {
        all.select_nth_unstable_by(n, order);
        all.truncate(n);
    }
    all.sort_unstable_by(order);
    all
}

/// Which mesh a batch draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshKind {
    /// The full tree mesh, rotated by the instance's yaw.
    Tree,
    /// The upright card, turned to face the viewer.
    Card,
}

impl MeshKind {
    fn label(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::Card => "card",
        }
    }
}

/// One slot: the 3 rows of the affine world-from-local matrix (rotation x scale, then the translation), `is_active`
/// = 1, `tag` = 0, 2 pad words.
pub fn slot_words(p: &Placed, kind: MeshKind, viewer: Vec3) -> [u32; 16] {
    let yaw = match kind {
        MeshKind::Tree => p.yaw,
        MeshKind::Card => {
            // Mat3::from_rotation_y(a) maps +Z to (sin a, 0, cos a): point it at the viewer on the ground plane
            // (atan2(0, 0) = 0 for a card under the viewer).
            (viewer.x - p.world[0]).atan2(viewer.z - p.world[2])
        }
    };
    let m = Mat3::from_rotation_y(yaw) * p.scale;
    let mut w = [0u32; 16];
    for r in 0..3 {
        let row = m.row(r);
        w[r * 4] = row.x.to_bits();
        w[r * 4 + 1] = row.y.to_bits();
        w[r * 4 + 2] = row.z.to_bits();
        w[r * 4 + 3] = p.world[r].to_bits();
    }
    w[12] = 1;
    w
}

/// Little-endian bytes of the slots of `placed`, 64 B each.
pub fn slot_bytes(placed: &[Placed], kind: MeshKind, viewer: Vec3) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(placed.len() * SLOT_BYTES);
    for p in placed {
        for w in slot_words(p, kind, viewer) {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
    }
    bytes
}

/// One batch of an arm: its mesh, its slot capacity and the bytes written at slot 0.
#[derive(Clone, Debug)]
pub struct BatchSpec {
    pub kind: MeshKind,
    pub capacity: u32,
    pub bytes: Vec<u8>,
}

/// One bench configuration: a set of batches spawned together.
#[derive(Clone, Debug)]
pub struct Arm {
    pub name: String,
    pub batches: Vec<BatchSpec>,
}

impl Arm {
    /// One batch of `kind`, capacity `n`, all `n` slots active (the `n` trees nearest the origin).
    pub fn sweep(kind: MeshKind, n: u32) -> Self {
        let placed = nearest_instances(n);
        let bytes = slot_bytes(&placed, kind, Vec3::ZERO);
        Self { name: format!("{}:{n}", kind.label()), batches: vec![BatchSpec { kind, capacity: n, bytes }] }
    }

    /// Slots of every batch together.
    pub fn total_slots(&self) -> u64 {
        self.batches.iter().map(|b| u64::from(b.capacity)).sum()
    }
}

/// The name of the arm with no batches.
pub const EMPTY_ARM: &str = "empty";

/// A parsed arm spec, before its bytes are built (cheap: validating a list of arms builds nothing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmSpec {
    /// No batches: the bare scene (camera, sun, ground), the baseline other arms are compared with.
    Empty,
    /// One batch of `n` slots of one mesh ([`Arm::sweep`]).
    Sweep(MeshKind, u32),
}

impl ArmSpec {
    /// Builds the arm's bytes (seconds at a million slots).
    pub fn build(self) -> Arm {
        match self {
            Self::Empty => Arm { name: EMPTY_ARM.to_owned(), batches: Vec::new() },
            Self::Sweep(kind, n) => Arm::sweep(kind, n),
        }
    }
}

/// `empty`, `tree:<n>` or `card:<n>` with `n > 0`, parsed without building anything.
pub fn parse_spec(spec: &str) -> Result<ArmSpec, String> {
    if spec == EMPTY_ARM {
        return Ok(ArmSpec::Empty);
    }
    let (kind, n) =
        spec.split_once(':').ok_or_else(|| format!("arm `{spec}`: expected empty, tree:<n> or card:<n>"))?;
    let kind = match kind {
        "tree" => MeshKind::Tree,
        "card" => MeshKind::Card,
        _ => return Err(format!("arm `{spec}`: unknown mesh `{kind}` (tree or card)")),
    };
    let n: u32 = n.parse().map_err(|e| format!("arm `{spec}`: bad count `{n}`: {e}"))?;
    if n == 0 {
        return Err(format!("arm `{spec}`: the count must be positive"));
    }
    Ok(ArmSpec::Sweep(kind, n))
}

/// `empty`, `tree:<n>` or `card:<n>` with `n > 0`, built ([`parse_spec`] then [`ArmSpec::build`]).
pub fn parse_arm(spec: &str) -> Result<Arm, String> {
    parse_spec(spec).map(ArmSpec::build)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(bytes: &[u8], slot: usize, w: usize) -> u32 {
        let at = slot * SLOT_BYTES + w * 4;
        u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
    }

    #[test]
    fn sweep_bytes_are_whole_active_slots() {
        let n = 5_000;
        let arm = Arm::sweep(MeshKind::Tree, n);
        assert_eq!(arm.name, "tree:5000");
        assert_eq!(arm.batches.len(), 1);
        let b = &arm.batches[0];
        assert_eq!(b.capacity, n);
        assert_eq!(b.bytes.len(), n as usize * SLOT_BYTES);
        assert!((0..n as usize).all(|s| word(&b.bytes, s, 12) == 1 && word(&b.bytes, s, 13) == 0));
    }

    #[test]
    fn nearest_first_and_dense_enough() {
        let n = 1_000;
        let placed = nearest_instances(n);
        assert_eq!(placed.len(), n as usize);
        assert!(placed.windows(2).all(|w| w[0].horizontal_distance_sq() <= w[1].horizontal_distance_sq()));
        let farthest = placed[placed.len() - 1].horizontal_distance_sq().sqrt();
        let expected = (f64::from(n) / (PI * 0.04)).sqrt();
        assert!((farthest / expected - 1.0).abs() < 0.1, "farthest {farthest} m, expected about {expected} m");
    }

    #[test]
    fn fill_is_deterministic() {
        let a = Arm::sweep(MeshKind::Card, 20_000);
        let b = Arm::sweep(MeshKind::Card, 20_000);
        assert_eq!(a.batches[0].bytes, b.batches[0].bytes);
    }

    #[test]
    fn card_faces_the_viewer() {
        let viewer = Vec3::new(3.0, 60.0, -7.0);
        for p in nearest_instances(2_000) {
            let w = slot_words(&p, MeshKind::Card, viewer);
            let f = |i: usize| f32::from_bits(w[i]);
            // Local +Z through the matrix rows = the third column.
            let z = Vec3::new(f(2), f(6), f(10));
            let to_viewer = Vec3::new(viewer.x - p.world[0], 0.0, viewer.z - p.world[2]);
            if to_viewer.length() < 1e-3 {
                continue;
            }
            let dot = Vec3::new(z.x, 0.0, z.z).normalize().dot(to_viewer.normalize());
            assert!(dot > 0.999, "card at {:?} faces {z:?}, dot {dot}", p.world);
            assert!((z.length() - p.scale).abs() < 1e-4);
        }
    }

    #[test]
    fn tree_slot_is_yaw_and_translation() {
        let p = Placed { world: [10.0, 0.0, -20.0], yaw: 0.7, scale: 1.1 };
        let w = slot_words(&p, MeshKind::Tree, Vec3::ZERO);
        let m = Mat3::from_rotation_y(0.7) * 1.1;
        for r in 0..3 {
            assert_eq!(w[r * 4], m.row(r).x.to_bits());
            assert_eq!(w[r * 4 + 3], p.world[r].to_bits());
        }
        assert_eq!(&w[12..], &[1, 0, 0, 0]);
    }

    #[test]
    fn parse_arm_accepts_kinds_and_rejects_junk() {
        let tree = parse_arm("tree:10").map(|a| (a.name, a.batches[0].kind, a.batches[0].capacity));
        assert_eq!(tree, Ok(("tree:10".to_owned(), MeshKind::Tree, 10)));
        let card = parse_arm("card:7").map(|a| (a.total_slots(), a.batches[0].kind, a.name));
        assert_eq!(card, Ok((7, MeshKind::Card, "card:7".to_owned())));
        for junk in ["", "tree", "tree:", "tree:0", "tree:-3", "tree:x", "bush:10", "tree:10:2", ":10", "Empty", "empty:3"] {
            assert!(parse_arm(junk).is_err(), "`{junk}` was accepted");
        }
    }

    #[test]
    fn empty_arm_has_no_batches() {
        let empty = parse_arm("empty").map(|a| (a.batches.len(), a.total_slots(), a.name));
        assert_eq!(empty, Ok((0, 0, "empty".to_owned())));
        assert_eq!(parse_spec("card:3"), Ok(ArmSpec::Sweep(MeshKind::Card, 3)));
    }
}
