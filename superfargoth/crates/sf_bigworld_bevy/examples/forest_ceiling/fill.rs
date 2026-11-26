//! The bench's instances and slot bytes, filled once on the CPU: trees of a flat synthetic forest around a viewer at
//! the world origin, each packed as one 64-byte `GpuMeshInstance` slot (the layout of
//! `sf_groundcover_bevy::residency::slot_words`), and the arms (batch sets) the bench can run: sweeps (the `n`
//! nearest trees, one batch, all active) and `stand` (the renderer's cost model's worst case: a tree batch and a card
//! batch at the tiers' fixed capacities, filled the way the GPU producer will fill them).

use std::cmp::Ordering;
use std::f64::consts::PI;

use bevy::math::{Mat3, Quat, Vec3};
use sf_bigworld::{BandSettings, Bands, Instance, InstanceSource, SpeciesDims, SyntheticForest, TileKey, ViewScale};

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
/// Fixture: the bench species at scale 1, what the tier bands read.
const SPECIES: SpeciesDims = SpeciesDims { height_m: 12.0, crown_m: 8.0 };
/// Fixture: internal render height the bands are computed at, pixels (1440p with DLSS Quality renders 960 rows).
const INTERNAL_HEIGHT_PX: f32 = 960.0;
/// Fixture: the sun cascades' reach, metres (default of the planned tunable `bigworld.shadow.mesh_range_m`).
pub const SHADOW_RANGE_M: f32 = 500.0;
/// Fixture: the `stand` tree batch's capacity (default of the planned tunable `bigworld.mesh.max_instances`).
pub const STAND_TREE_CAPACITY: u32 = 4_096;
/// Fixture: the `stand` card batch's capacity (default of the planned tunable `bigworld.impostor.max_instances`).
pub const STAND_CARD_CAPACITY: u32 = 250_000;

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

/// Total order on trees: horizontal distance from the origin, ties broken by position bits (deterministic).
fn order(a: &Placed, b: &Placed) -> Ordering {
    a.horizontal_distance_sq()
        .total_cmp(&b.horizontal_distance_sq())
        .then_with(|| a.world[0].to_bits().cmp(&b.world[0].to_bits()))
        .then_with(|| a.world[2].to_bits().cmp(&b.world[2].to_bits()))
}

/// Every tree within `r` metres (horizontal) of the world origin, nearest first ([`order`]).
pub fn instances_within(r: f64) -> Vec<Placed> {
    let mut all = gather_tiles(r);
    let r_sq = r * r;
    all.retain(|p| p.horizontal_distance_sq() <= r_sq);
    all.sort_unstable_by(order);
    all
}

/// The `n` trees nearest the world origin, nearest first ([`order`]).
pub fn nearest_instances(n: u32) -> Vec<Placed> {
    if n == 0 {
        return Vec::new();
    }
    let per_m2 = f64::from(DENSITY_PER_HA) / M2_PER_HA;
    let diagonal = f64::from(TILE_M) * std::f64::consts::SQRT_2;
    let r = (f64::from(n) / (PI * per_m2)).sqrt() * 1.1 + diagonal;
    let mut all = gather_tiles(r);
    let n = n as usize;
    if all.len() > n {
        all.select_nth_unstable_by(n, order);
        all.truncate(n);
    }
    all.sort_unstable_by(order);
    all
}

/// Every tree of the tiles within `r` metres of the origin, unsorted (some lie farther than `r`).
fn gather_tiles(r: f64) -> Vec<Placed> {
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
    all
}

/// The bench camera, for the `stand` arm's frustum test: at (0, `eye_m`, 0), looking along -Z, pitched `pitch_rad`
/// below the horizon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandView {
    pub eye_m: f32,
    pub pitch_rad: f32,
    /// Vertical field of view.
    pub fov_y_rad: f32,
    /// Width over height.
    pub aspect: f32,
}

impl StandView {
    /// The camera's position.
    pub fn position(&self) -> Vec3 {
        Vec3::new(0.0, self.eye_m, 0.0)
    }

    /// The frustum's four side planes (left, right, bottom, top) in world space; near and far are ignored.
    pub fn planes(&self) -> SidePlanes {
        let ty = (0.5 * self.fov_y_rad).tan();
        let tx = ty * self.aspect;
        // The camera's rotation (main.rs): view space looks along -Z, pitched down about +X.
        let rot = Quat::from_rotation_x(-self.pitch_rad);
        let view_normals =
            [Vec3::new(1.0, 0.0, -tx), Vec3::new(-1.0, 0.0, -tx), Vec3::new(0.0, 1.0, -ty), Vec3::new(0.0, -1.0, -ty)];
        SidePlanes { origin: self.position(), normals: view_normals.map(|n| rot * n.normalize()) }
    }
}

/// Four planes through the camera, inward unit normals.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SidePlanes {
    origin: Vec3,
    normals: [Vec3; 4],
}

impl SidePlanes {
    /// Whether a sphere is at least partly inside all four planes (a point when `radius` is 0).
    pub fn contains_sphere(&self, centre: Vec3, radius: f32) -> bool {
        let rel = centre - self.origin;
        self.normals.iter().all(|n| n.dot(rel) >= -radius)
    }
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

/// The look of one batch (BW-2b): the bench's leaf material (`StandardMaterial`, `AlphaMode::Mask(0.5)`, lit, casting
/// shadows) unless a flag says otherwise. `Default` is that material.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct BatchLook {
    /// `unlit: true` on the material (a cheap fragment shader).
    pub unlit: bool,
    /// `AlphaMode::Opaque` in place of the mask (same texture as colour, no discard: early Z works).
    pub opaque: bool,
    /// `NotShadowCaster` on the batch entity (it leaves every shadow pass).
    pub noshadow: bool,
}

impl BatchLook {
    /// [`Default`] as a `const`.
    pub const DEFAULT: Self = Self { unlit: false, opaque: false, noshadow: false };

    /// The modifiers of a `stand` part, in canonical order (`.unlit.opaque.noshadow`).
    pub const MODIFIERS: [&str; 3] = ["unlit", "opaque", "noshadow"];

    fn flags(self) -> [bool; 3] {
        [self.unlit, self.opaque, self.noshadow]
    }

    fn flag_mut(&mut self, modifier: &str) -> Option<&mut bool> {
        match modifier {
            "unlit" => Some(&mut self.unlit),
            "opaque" => Some(&mut self.opaque),
            "noshadow" => Some(&mut self.noshadow),
            _ => None,
        }
    }

    /// `.unlit.opaque.noshadow`, each only when set; empty for the default look.
    fn suffix(self) -> String {
        let mut suffix = String::new();
        for (m, on) in Self::MODIFIERS.iter().zip(self.flags()) {
            if on {
                suffix.push('.');
                suffix.push_str(m);
            }
        }
        suffix
    }
}

/// One batch of an arm: its mesh, its slot capacity, how many slots are active (the first ones), how many instances
/// did not fit, the bytes written at slot 0 (the whole capacity: inactive slots are zeroed, never garbage) and its
/// look.
#[derive(Clone, Debug)]
pub struct BatchSpec {
    pub kind: MeshKind,
    pub capacity: u32,
    pub active: u32,
    pub dropped: u32,
    pub bytes: Vec<u8>,
    pub look: BatchLook,
}

impl BatchSpec {
    /// The first `capacity` of `placed` (nearest first) as active slots, then zeroed slots up to `capacity`.
    fn fixed(kind: MeshKind, placed: &[Placed], capacity: u32, viewer: Vec3, look: BatchLook) -> Self {
        let keep = placed.len().min(capacity as usize);
        let mut bytes = slot_bytes(&placed[..keep], kind, viewer);
        bytes.resize(capacity as usize * SLOT_BYTES, 0);
        Self { kind, capacity, active: keep as u32, dropped: (placed.len() - keep) as u32, bytes, look }
    }
}

/// Which batches a `stand` arm spawns and the look of each (`None` = not spawned).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandParts {
    pub trees: Option<BatchLook>,
    pub cards: Option<BatchLook>,
}

impl StandParts {
    /// Plain `stand`: both batches, default look.
    pub const PLAIN: Self = Self { trees: Some(BatchLook::DEFAULT), cards: Some(BatchLook::DEFAULT) };

    /// The canonical spec string: `stand` when plain, else `stand/trees<mods>/cards<mods>` with the named parts only.
    pub fn name(self) -> String {
        if self == Self::PLAIN {
            return STAND_ARM.to_owned();
        }
        let mut name = STAND_ARM.to_owned();
        for (part, look) in [("trees", self.trees), ("cards", self.cards)] {
            if let Some(look) = look {
                name.push('/');
                name.push_str(part);
                name.push_str(&look.suffix());
            }
        }
        name
    }
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
        let batch = BatchSpec { kind, capacity: n, active: n, dropped: 0, bytes, look: BatchLook::DEFAULT };
        Self { name: format!("{}:{n}", kind.label()), batches: vec![batch] }
    }

    /// The cost model's worst case seen from `view`: a tree batch of every tree inside the mesh tier and a card batch
    /// of every tree inside the impostor tier that can cast into a sun cascade (within [`SHADOW_RANGE_M`]) or is in
    /// the frustum, at the tiers' capacities. A tree in a crossfade band is in both batches. Only the batches `parts`
    /// names are spawned, each with its look; the arm is named [`StandParts::name`].
    pub fn stand(view: &StandView, parts: StandParts) -> Self {
        // The High-preset defaults of `bigworld.mesh.switch_px`, `bigworld.aggregate.switch_px`, `bigworld.fade_band`.
        let bands = Bands::new(SPECIES, ViewScale::new(INTERNAL_HEIGHT_PX, view.fov_y_rad), BandSettings::default());
        let reach = bands.impostor_end_m.max(bands.mesh_end_m) * forest().scale[1];
        let planes = view.planes();
        let (mut trees, mut cards) = (Vec::new(), Vec::new());
        for p in instances_within(f64::from(reach)) {
            // Horizontal distance from the viewer's ground point (0, 0, 0): the ring planner's distance, so the counts
            // match the cost model's.
            let dist = p.horizontal_distance_sq().sqrt() as f32;
            let f = bands.fades(dist, p.scale);
            if f.mesh > 0.0 {
                trees.push(p);
            }
            if f.impostor > 0.0 {
                // Bounding sphere: centre at half the scaled height, radius half the scaled height.
                let half = 0.5 * SPECIES.height_m * p.scale;
                let centre = Vec3::new(p.world[0], p.world[1] + half, p.world[2]);
                // Nothing past the shadow range can cast into a cascade, so only there the frustum decides.
                if dist <= SHADOW_RANGE_M || planes.contains_sphere(centre, half) {
                    cards.push(p);
                }
            }
        }
        let viewer = view.position();
        let tree = parts.trees.map(|look| BatchSpec::fixed(MeshKind::Tree, &trees, STAND_TREE_CAPACITY, viewer, look));
        let card = parts.cards.map(|look| BatchSpec::fixed(MeshKind::Card, &cards, STAND_CARD_CAPACITY, viewer, look));
        Self { name: parts.name(), batches: tree.into_iter().chain(card).collect() }
    }

    /// Slots of every batch together.
    pub fn total_slots(&self) -> u64 {
        self.batches.iter().map(|b| u64::from(b.capacity)).sum()
    }

    /// Active slots of every batch together.
    pub fn total_active(&self) -> u64 {
        self.batches.iter().map(|b| u64::from(b.active)).sum()
    }

    /// Instances that did not fit their batch, every batch together.
    pub fn total_dropped(&self) -> u64 {
        self.batches.iter().map(|b| u64::from(b.dropped)).sum()
    }
}

/// The name of the arm with no batches.
pub const EMPTY_ARM: &str = "empty";
/// The name of the two-batch worst-case arm ([`Arm::stand`]).
pub const STAND_ARM: &str = "stand";

/// A parsed arm spec, before its bytes are built (cheap: validating a list of arms builds nothing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmSpec {
    /// No batches: the bare scene (camera, sun, ground), the baseline other arms are compared with.
    Empty,
    /// One batch of `n` slots of one mesh ([`Arm::sweep`]).
    Sweep(MeshKind, u32),
    /// A tree batch and a card batch, the cost model's worst case ([`Arm::stand`]), default look.
    Stand,
    /// `stand/<part>..`: only the named batches of the worst case, each with its look (BW-2b).
    StandParts(StandParts),
}

impl ArmSpec {
    /// Builds the arm's bytes (seconds at a million slots); only `stand` reads `view`.
    pub fn build(self, view: &StandView) -> Arm {
        match self {
            Self::Empty => Arm { name: EMPTY_ARM.to_owned(), batches: Vec::new() },
            Self::Sweep(kind, n) => Arm::sweep(kind, n),
            Self::Stand => Arm::stand(view, StandParts::PLAIN),
            Self::StandParts(parts) => Arm::stand(view, parts),
        }
    }
}

/// The parts after `stand/`: `/`-separated, each `trees` or `cards` then `.unlit`, `.opaque`, `.noshadow` in any order,
/// each batch and each modifier of a part at most once.
fn parse_stand_parts(spec: &str, rest: &str) -> Result<StandParts, String> {
    let mut parts = StandParts { trees: None, cards: None };
    for part in rest.split('/') {
        let mut words = part.split('.');
        let batch = words.next().unwrap_or_default();
        let slot = match batch {
            "trees" => &mut parts.trees,
            "cards" => &mut parts.cards,
            "" => return Err(format!("arm `{spec}`: empty part (each `/` is followed by trees or cards)")),
            _ => return Err(format!("arm `{spec}`: unknown part `{batch}` (trees or cards)")),
        };
        if slot.is_some() {
            return Err(format!("arm `{spec}`: part `{batch}` named twice"));
        }
        let mut look = BatchLook::DEFAULT;
        for modifier in words {
            let flag = look.flag_mut(modifier).ok_or_else(|| {
                format!("arm `{spec}`: unknown modifier `.{modifier}` on `{batch}` (unlit, opaque or noshadow)")
            })?;
            if *flag {
                return Err(format!("arm `{spec}`: modifier `.{modifier}` repeated on `{batch}`"));
            }
            *flag = true;
        }
        *slot = Some(look);
    }
    Ok(parts)
}

/// `empty`, `stand`, `stand/<part>..` (see [`parse_stand_parts`]), `tree:<n>` or `card:<n>` with `n > 0`, parsed
/// without building anything.
pub fn parse_spec(spec: &str) -> Result<ArmSpec, String> {
    if spec == EMPTY_ARM {
        return Ok(ArmSpec::Empty);
    }
    if spec == STAND_ARM {
        return Ok(ArmSpec::Stand);
    }
    if let Some(rest) = spec.strip_prefix("stand/") {
        return parse_stand_parts(spec, rest).map(ArmSpec::StandParts);
    }
    if spec.contains('.') {
        return Err(format!(
            "arm `{spec}`: look modifiers (.unlit, .opaque, .noshadow) apply only to stand parts \
             (stand/trees.<modifier>, stand/cards.<modifier>)"
        ));
    }
    let (kind, n) = spec
        .split_once(':')
        .ok_or_else(|| format!("arm `{spec}`: expected empty, stand, stand/<part>.., tree:<n> or card:<n>"))?;
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

/// `empty`, `stand`, `stand/<part>..`, `tree:<n>` or `card:<n>` with `n > 0`, built ([`parse_spec`] then
/// [`ArmSpec::build`]).
pub fn parse_arm(spec: &str, view: &StandView) -> Result<Arm, String> {
    parse_spec(spec).map(|s| s.build(view))
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
        let view = bench_view();
        let tree = parse_arm("tree:10", &view).map(|a| (a.name, a.batches[0].kind, a.batches[0].capacity));
        assert_eq!(tree, Ok(("tree:10".to_owned(), MeshKind::Tree, 10)));
        let card = parse_arm("card:7", &view).map(|a| (a.total_slots(), a.total_active(), a.batches[0].kind, a.name));
        assert_eq!(card, Ok((7, 7, MeshKind::Card, "card:7".to_owned())));
        for junk in [
            "", "tree", "tree:", "tree:0", "tree:-3", "tree:x", "bush:10", "tree:10:2", ":10", "Empty", "empty:3", "Stand",
            "stand:3",
        ] {
            assert!(parse_arm(junk, &view).is_err(), "`{junk}` was accepted");
        }
    }

    #[test]
    fn empty_arm_has_no_batches() {
        let empty = parse_arm("empty", &bench_view()).map(|a| (a.batches.len(), a.total_slots(), a.name));
        assert_eq!(empty, Ok((0, 0, "empty".to_owned())));
        assert_eq!(parse_spec("card:3"), Ok(ArmSpec::Sweep(MeshKind::Card, 3)));
    }

    /// The view `main.rs` builds at the default eye height.
    fn bench_view() -> StandView {
        crate::stand_view(&crate::Eye(crate::DEFAULT_EYE_M))
    }

    /// The slot's translation (the last word of each matrix row).
    fn slot_pos(bytes: &[u8], slot: usize) -> Vec3 {
        let f = |w: usize| f32::from_bits(word(bytes, slot, w));
        Vec3::new(f(3), f(7), f(11))
    }

    #[test]
    fn stand_parses_without_building() {
        assert_eq!(parse_spec("stand"), Ok(ArmSpec::Stand));
    }

    #[test]
    fn stand_frustum_sees_ahead_not_behind_or_aside() {
        let planes = bench_view().planes();
        assert!(planes.contains_sphere(Vec3::new(0.0, 0.0, -1_000.0), 0.0));
        assert!(!planes.contains_sphere(Vec3::new(0.0, 0.0, 1_000.0), 0.0));
        assert!(!planes.contains_sphere(Vec3::new(5_000.0, 0.0, -1_000.0), 0.0));
    }

    #[test]
    fn stand_arm_is_the_cost_model_worst_case() {
        let view = bench_view();
        let bands = Bands::new(SPECIES, ViewScale::new(INTERNAL_HEIGHT_PX, view.fov_y_rad), BandSettings::default());
        let arm = ArmSpec::Stand.build(&view);
        assert_eq!(arm.name, "stand");
        assert_eq!(arm.batches.len(), 2);
        let (tree, card) = (&arm.batches[0], &arm.batches[1]);
        println!(
            "stand: tree active={} card active={} mesh_end_m={:.1} impostor_end_m={:.1}",
            tree.active, card.active, bands.mesh_end_m, bands.impostor_end_m
        );
        assert_eq!((tree.kind, tree.capacity), (MeshKind::Tree, STAND_TREE_CAPACITY));
        assert_eq!((card.kind, card.capacity), (MeshKind::Card, STAND_CARD_CAPACITY));
        assert_eq!((STAND_TREE_CAPACITY, STAND_CARD_CAPACITY), (4_096, 250_000));
        for b in &arm.batches {
            assert_eq!(b.bytes.len(), b.capacity as usize * SLOT_BYTES);
            assert!(b.active <= b.capacity);
            assert_eq!(b.dropped, 0);
            let active = b.active as usize;
            assert!((0..active).all(|s| word(&b.bytes, s, 12) == 1));
            let dist = |s: usize| {
                let p = slot_pos(&b.bytes, s);
                Placed { world: p.to_array(), yaw: 0.0, scale: 1.0 }.horizontal_distance_sq()
            };
            assert!((1..active).all(|s| dist(s - 1) <= dist(s)), "{:?} slots not sorted by distance", b.kind);
            assert!(b.bytes[active * SLOT_BYTES..].iter().all(|&x| x == 0), "{:?} inactive slots not zeroed", b.kind);
        }
        assert!((1_700..=2_400).contains(&tree.active), "tree active {}", tree.active);
        assert!((150_000..=230_000).contains(&card.active), "card active {}", card.active);
        assert_eq!(arm.total_active(), u64::from(tree.active) + u64::from(card.active));
        assert_eq!((arm.total_slots(), arm.total_dropped()), (254_096, 0));

        // Shadow range: behind the camera (z > 0) only within it.
        let behind: Vec<f32> = (0..card.active as usize)
            .map(|s| slot_pos(&card.bytes, s))
            .filter(|p| p.z > 0.0)
            .map(|p| p.x.hypot(p.z))
            .collect();
        assert!(behind.iter().any(|&d| d <= SHADOW_RANGE_M), "no card behind the camera within the shadow range");
        assert!(behind.iter().all(|&d| d <= 600.0), "a card behind the camera beyond 600 m");

        let again = ArmSpec::Stand.build(&view);
        assert!(arm.batches.iter().zip(&again.batches).all(|(a, b)| a.bytes == b.bytes && a.active == b.active));
    }

    /// The BW-2b arm list (the `--arms` doc line in `main.rs`), every `stand` variant in it canonical.
    const BW2B_STAND_ARMS: [&str; 10] = [
        "stand",
        "stand/trees",
        "stand/cards",
        "stand/cards.unlit",
        "stand/cards.noshadow",
        "stand/cards.opaque",
        "stand/cards.unlit.noshadow",
        "stand/trees.opaque",
        "stand/trees.noshadow",
        "stand/trees.opaque/cards.unlit.noshadow",
    ];

    fn parts(spec: &str) -> StandParts {
        match parse_spec(spec) {
            Ok(ArmSpec::StandParts(p)) => p,
            other => panic!("`{spec}` parsed as {other:?}"),
        }
    }

    #[test]
    fn stand_variants_parse_and_name_themselves() {
        assert_eq!(parse_spec("stand"), Ok(ArmSpec::Stand));
        for spec in &BW2B_STAND_ARMS[1..] {
            assert_eq!(parts(spec).name(), *spec);
        }
        let look = |unlit, opaque, noshadow| Some(BatchLook { unlit, opaque, noshadow });
        assert_eq!(parts("stand/trees"), StandParts { trees: look(false, false, false), cards: None });
        assert_eq!(parts("stand/cards.opaque"), StandParts { trees: None, cards: look(false, true, false) });
        assert_eq!(
            parts("stand/trees.opaque/cards.unlit.noshadow"),
            StandParts { trees: look(false, true, false), cards: look(true, false, true) }
        );
        // Any order of parts and modifiers; the name is canonical.
        assert_eq!(parts("stand/cards.noshadow.unlit/trees.opaque").name(), "stand/trees.opaque/cards.unlit.noshadow");
        assert_eq!(parts("stand/cards.opaque.unlit").name(), "stand/cards.unlit.opaque");
        // Both parts at the default look are plain `stand`.
        assert_eq!(parts("stand/trees/cards"), StandParts::PLAIN);
        assert_eq!(StandParts::PLAIN.name(), "stand");
        let list = format!("empty,{}", BW2B_STAND_ARMS.join(","));
        assert_eq!(crate::bench::parse_arms(&list).map(|a| a.len()), Ok(11));
    }

    #[test]
    fn stand_variants_reject_junk() {
        for junk in [
            "stand/",
            "stand//cards",
            "stand/cards/",
            "stand/bush",
            "stand/Trees",
            "stand/tree",
            "stand/trees.shiny",
            "stand/trees.",
            "stand/trees..unlit",
            "stand/trees/trees",
            "stand/cards.unlit/cards",
            "stand/cards.unlit.unlit",
            "stand/trees.opaque.noshadow.opaque",
            "stand.unlit",
            "stand.trees",
            "tree:10.unlit",
            "card:5.noshadow",
            "card.opaque:5",
            "empty.unlit",
            "empty/trees",
        ] {
            let parsed = parse_spec(junk);
            assert!(parsed.is_err(), "`{junk}` was accepted as {parsed:?}");
        }
        let msg = |s: &str| parse_spec(s).unwrap_err();
        assert!(msg("stand/cards.unlit.unlit").contains("repeated"));
        assert!(msg("stand/trees/trees").contains("twice"));
        assert!(msg("stand/").contains("empty part"));
        assert!(msg("stand/bush").contains("unknown part"));
        assert!(msg("stand/trees.shiny").contains("unknown modifier"));
        assert!(msg("tree:10.unlit").contains("only to stand parts"));
        assert!(msg("empty.unlit").contains("only to stand parts"));
    }

    #[test]
    fn stand_parts_match_the_plain_batches() {
        let view = bench_view();
        let plain = ArmSpec::Stand.build(&view);
        assert!(plain.batches.iter().all(|b| b.look == BatchLook::default()));
        let (plain_tree, plain_card) = (&plain.batches[0], &plain.batches[1]);
        let same = |a: &BatchSpec, b: &BatchSpec| {
            (a.kind, a.capacity, a.active, a.dropped) == (b.kind, b.capacity, b.active, b.dropped) && a.bytes == b.bytes
        };

        let trees = parse_arm("stand/trees", &view).expect("parses");
        assert_eq!((trees.name.as_str(), trees.batches.len()), ("stand/trees", 1));
        assert!(same(&trees.batches[0], plain_tree));
        assert_eq!(trees.batches[0].look, BatchLook::DEFAULT);

        let cards = parse_arm("stand/cards", &view).expect("parses");
        assert_eq!((cards.name.as_str(), cards.batches.len()), ("stand/cards", 1));
        assert!(same(&cards.batches[0], plain_card));
        assert_eq!(cards.batches[0].look, BatchLook::DEFAULT);

        let spec = "stand/trees.opaque/cards.unlit.noshadow";
        let both = parse_arm(spec, &view).expect("parses");
        assert_eq!((both.name.as_str(), both.batches.len()), (spec, 2));
        let (tree, card) = (&both.batches[0], &both.batches[1]);
        assert!(same(tree, plain_tree) && same(card, plain_card));
        assert_eq!(tree.look, BatchLook { unlit: false, opaque: true, noshadow: false });
        assert_eq!(card.look, BatchLook { unlit: true, opaque: false, noshadow: true });
        assert_eq!((both.total_slots(), both.total_active()), (plain.total_slots(), plain.total_active()));
    }
}
