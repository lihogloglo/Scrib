//! The baked per-cell instance table (static-scene.md §2, "Data model").
//!
//! One [`StaticCellTable`] per streaming cell: its scene instances (sorted by placement id), the
//! [`LodChain`]s they draw, the cell's AABB, its land's minimum height (the horizon test, §4) and
//! the view layers every instance of the table is drawn in.
//!
//! **One instance per (placement, material part).** A model with several scene materials gives
//! several instances sharing a placement id, one per chain (a chain is one model part,
//! [`crate::chain`]). Instances are sorted by (placement, part); the part index is not stored, so
//! the reader checks only that placement ids never decrease, and [`StaticCellTableView::find`]
//! returns a placement's first instance (its other parts follow, [`StaticCellTableView::find_all`]).
//! Each instance carries its part's own bounding sphere.
//!
//! **Chains are cell-local** (format version 2). An instance names its chain by its index in this
//! table's chain section (`0..chain_count`, dense: every chain is used by some instance). A chain
//! record carries the 64-bit keys that identify it across cells: the **mesh key** (the cooked
//! geometry of one model part) and the **material key** (a content hash of its cooked material),
//! plus its [`MaterialClass`]. Chains are sorted by mesh key, unique. Stream-in maps each
//! cell-local index to a global dense slot keyed by mesh key ([`StaticCellTableView::chain_index`]
//! finds one, [`StaticCellTableView::write_remapped`] copies the instances with the global slots),
//! so no instance carries a hash and no two cells can disagree on an id.
//!
//! # Binary layout, format version 2
//!
//! All little-endian, no compression, no padding beyond what is listed; sections follow the
//! header in this order and the file is exactly their total size. Reserved bytes are zero (the
//! reader rejects anything else).
//!
//! | Section | Size | Content (byte offset) |
//! |---|---|---|
//! | header | 80 B | magic `SFSCENE\0` (0), version `u32` (8), instance count `u32` (12), chain count `u32` (16), level count `u32` (20), origin cell `i32 × 3` (24), land minimum `f32` (36), AABB min `f32 × 3` (40), AABB max `f32 × 3` (52), layer mask `u32` (64), reserved `u32 × 3` (68) |
//! | instances | 80 B each | transform rows `f32 × 12` (0), sphere centre + radius `f32 × 4` (48), placement `u32` (64), chain index `u32` (68), material override `u32` (72), flags `u32` (76) |
//! | chains | 32 B each | first level `u32` (0), level count `u32` (4), material class `u16` (8), reserved `u16` (10), reserved `u32` (12), mesh key `u64` (16), material key `u64` (24) |
//! | levels | 12 B each | first index `u32`, index count `u32`, error `f32` (metres, model space) |
//!
//! The header is a multiple of 16 bytes, so the instance section of a 16-aligned buffer is
//! 16-aligned. The instance record is the GPU layout (a WGSL struct of `array<vec4<f32>, 3>`,
//! `vec4<f32>` and four `u32`, 16-byte aligned): stream-in uploads
//! [`StaticCellTableView::write_remapped`]'s copy, which differs from
//! [`StaticCellTableView::instance_bytes`] only in the chain field (offset 68).
//!
//! The **material override** is 0 in every baked table: the instance draws its chain's material.
//! Other values are reserved for a per-placement material table (retextures, tint) that a later
//! format version adds, and for the runtime's own writes on the GPU copy; the reader rejects them.
//! Version 1 (chain ids folded from the mesh key, class and layers per instance) is not read.
//!
//! Like `sf_model`'s `.sfmesh` (`cooked.rs`, magic and a `u32` version), a reader takes only the
//! versions it knows and rejects the rest with [`TableError::Version`]; a layout change bumps
//! [`FORMAT_VERSION`]. Every read is bounds-checked: malformed input is an error, never a panic.

use glam::{Affine3A, IVec3, Mat3A, Vec3, Vec3A};

use crate::PlacementId;
use crate::material::MaterialClass;

pub const MAGIC: [u8; 8] = *b"SFSCENE\0";
/// The only format version read and written.
pub const FORMAT_VERSION: u32 = 2;
/// Most levels one chain may have (a level index fits a `u8` work item field).
pub const MAX_LOD_LEVELS: usize = 16;

const HEADER: usize = 80;
const INSTANCE: usize = 80;
const CHAIN: usize = 32;
const LEVEL: usize = 12;
/// Offset of the chain index in an instance record (rewritten by [`StaticCellTableView::write_remapped`]).
const INSTANCE_CHAIN: usize = 68;

bitflags::bitflags! {
    /// Per-instance flags (§2). Bits not listed here are reserved (zero).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct InstanceFlags: u32 {
        const CASTS_SHADOW = 1;
        const ALPHA_MASKED = 1 << 1;
        /// Reserved for the far field on the same table (S7, §9): drawn only in sky / far views.
        /// Not set by the `scene.cells` bake, not read by the cull yet.
        const FAR_ONLY = 1 << 2;
        /// Reserved for Hi-Z occlusion (S6, §4 step 6): may be drawn into the occluder pass.
        /// Not set by the `scene.cells` bake, not read by the cull yet.
        const OCCLUDER = 1 << 3;
        /// The vertex shader applies wind sway.
        const WIND = 1 << 4;
        /// Runtime only, never baked: promotion's "one flag write" on the GPU copy (§1). The
        /// cull drops the instance; the table reader rejects it.
        const INACTIVE = 1 << 15;
    }
}

/// An axis-aligned box, cell-relative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// The same box moved by `offset`.
    #[must_use]
    pub fn translated(self, offset: Vec3) -> Self {
        Self { min: self.min + offset, max: self.max + offset }
    }
}

/// A bounding sphere, cell-relative (or model space where documented).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

/// One scene instance (§2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaticInstance {
    /// Cell-local: with the table's cell it names one placement (`crate::bake` "Placement ids").
    pub placement: PlacementId,
    /// Cell-relative 3×4 affine, as rows (the top three rows of the 4×4 matrix): the layout of
    /// Bevy's transposed `MeshUniform` affine. [`Self::affine`] converts.
    pub transform: [[f32; 4]; 3],
    /// Bounds of this instance's part (its chain's geometry), transformed, cell-relative.
    pub sphere: Sphere,
    /// Index of its [`LodChain`] in this table's chains (cell-local, module docs).
    pub chain: u32,
    /// 0: the chain's material. Other values are reserved (module docs).
    pub material_override: u32,
    pub flags: InstanceFlags,
}

impl StaticInstance {
    /// The transform as a glam affine.
    pub fn affine(&self) -> Affine3A {
        let [r0, r1, r2] = self.transform;
        Affine3A {
            matrix3: Mat3A::from_cols(
                Vec3A::new(r0[0], r1[0], r2[0]),
                Vec3A::new(r0[1], r1[1], r2[1]),
                Vec3A::new(r0[2], r1[2], r2[2]),
            ),
            translation: Vec3A::new(r0[3], r1[3], r2[3]),
        }
    }

    /// The rows of a glam affine.
    pub fn rows(affine: &Affine3A) -> [[f32; 4]; 3] {
        let m = affine.matrix3;
        let t = affine.translation;
        std::array::from_fn(|r| [m.x_axis[r], m.y_axis[r], m.z_axis[r], t[r]])
    }

    /// The largest axis scale of the transform (the longest column of its 3×3 part): how much
    /// a model-space length (a LOD error) can grow when placed.
    pub fn max_scale(&self) -> f32 {
        let [r0, r1, r2] = self.transform;
        (0..3).map(|c| Vec3::new(r0[c], r1[c], r2[c]).length_squared()).fold(0.0f32, f32::max).sqrt()
    }
}

/// One LOD level: an index range of the chain's mesh (all levels share its vertices).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodLevel {
    pub first_index: u32,
    pub index_count: u32,
    /// Worst geometric deviation from level 0, model-space metres (`sf_model` `CookedLod::error`).
    pub error: f32,
}

/// One mesh with its LOD levels, finest first (§2). Selecting a level is choosing an index
/// range: LOD never changes the instance or its batch.
#[derive(Clone, Debug, PartialEq)]
pub struct LodChain {
    /// The bake's key of the chain's cooked geometry (opaque here): equal keys, equal meshes,
    /// in every cell. Stream-in keys global chain slots on it.
    pub mesh: u64,
    /// Content hash of the chain's cooked material (opaque here): equal keys, equal materials,
    /// whichever model they come from (§5, one bindless material slot per key).
    pub material: u64,
    /// The batch set the chain draws in (§5); a pure function of its material.
    pub material_class: MaterialClass,
    /// 1..=[`MAX_LOD_LEVELS`] levels.
    pub levels: Vec<LodLevel>,
}

/// A cell's table, owned (the bake's output, and what [`StaticCellTableView::to_table`] gives).
#[derive(Clone, Debug, PartialEq)]
pub struct StaticCellTable {
    /// The `sf_coords` grid cell whose centre is the origin of every cell-relative value.
    pub origin_cell: IVec3,
    /// Bounds of every instance's sphere.
    pub aabb: Aabb,
    /// Lowest land height in the cell, cell-relative Y (`f32::NEG_INFINITY`: no land, or holes).
    pub land_min: f32,
    /// Which view layers draw the table's instances (replaces `RenderLayers`; one per table).
    pub layers: u32,
    /// Sorted by (placement id, part): a placement's parts are adjacent (module docs).
    pub instances: Vec<StaticInstance>,
    /// Sorted by mesh key, unique, each used by an instance; instances index this list.
    pub chains: Vec<LodChain>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TableError {
    #[error("not a static cell table")]
    BadMagic,
    #[error("static cell table format version {0}, expected {FORMAT_VERSION}")]
    Version(u32),
    #[error("static cell table is {got} bytes, its header says {expected}")]
    Size { expected: u64, got: usize },
    #[error("non-zero reserved bytes at offset {0}")]
    Reserved(usize),
    #[error("instances not sorted by placement id at {0}")]
    PlacementOrder(PlacementId),
    #[error("chains not sorted by unique mesh key at {0:#x}")]
    ChainOrder(u64),
    #[error("instance {placement} names chain {chain}, not in the table")]
    MissingChain { placement: PlacementId, chain: u32 },
    #[error("chain {0} is used by no instance")]
    UnusedChain(u32),
    #[error("chain {0} has no level, more than {MAX_LOD_LEVELS}, or a level range past the level section")]
    ChainLevels(u32),
    #[error("chain {0} has a negative or non-finite level error")]
    LevelError(u32),
    #[error("chain {chain} has unknown material class {class}")]
    MaterialClass { chain: u32, class: u16 },
    #[error("instance {0} has unknown or runtime-only flag bits")]
    Flags(PlacementId),
    #[error("instance {0} has a material override (none is defined in format version {FORMAT_VERSION})")]
    MaterialOverride(PlacementId),
    #[error("table too large for 32-bit counts")]
    TooLarge,
}

impl StaticCellTable {
    /// The binary form (module docs). Validated by the reader before it is returned, so a table
    /// that writes is a table that reads.
    pub fn write(&self) -> Result<Vec<u8>, TableError> {
        let count = |n: usize| u32::try_from(n).map_err(|_| TableError::TooLarge);
        let level_total: usize = self.chains.iter().map(|c| c.levels.len()).sum();
        let mut out = Vec::with_capacity(
            HEADER + INSTANCE * self.instances.len() + CHAIN * self.chains.len() + LEVEL * level_total,
        );
        out.extend_from_slice(&MAGIC);
        put_u32(&mut out, FORMAT_VERSION);
        put_u32(&mut out, count(self.instances.len())?);
        put_u32(&mut out, count(self.chains.len())?);
        put_u32(&mut out, count(level_total)?);
        for v in self.origin_cell.to_array() {
            out.extend_from_slice(&v.to_le_bytes());
        }
        put_f32(&mut out, self.land_min);
        put_vec3(&mut out, self.aabb.min);
        put_vec3(&mut out, self.aabb.max);
        put_u32(&mut out, self.layers);
        out.extend_from_slice(&[0; 12]);
        for i in &self.instances {
            for row in i.transform {
                for v in row {
                    put_f32(&mut out, v);
                }
            }
            put_vec3(&mut out, i.sphere.center);
            put_f32(&mut out, i.sphere.radius);
            put_u32(&mut out, i.placement);
            put_u32(&mut out, i.chain);
            put_u32(&mut out, i.material_override);
            put_u32(&mut out, i.flags.bits());
        }
        let mut first = 0usize;
        for c in &self.chains {
            put_u32(&mut out, count(first)?);
            put_u32(&mut out, count(c.levels.len())?);
            out.extend_from_slice(&(c.material_class as u16).to_le_bytes());
            out.extend_from_slice(&[0; 6]);
            out.extend_from_slice(&c.mesh.to_le_bytes());
            out.extend_from_slice(&c.material.to_le_bytes());
            first += c.levels.len();
        }
        for l in self.chains.iter().flat_map(|c| &c.levels) {
            put_u32(&mut out, l.first_index);
            put_u32(&mut out, l.index_count);
            put_f32(&mut out, l.error);
        }
        StaticCellTableView::parse(&out)?;
        Ok(out)
    }
}

/// A validated table borrowed from its bytes (an mmap or a read buffer). Records decode on
/// access; the instance section is exposed raw for the GPU upload.
#[derive(Clone, Copy, Debug)]
pub struct StaticCellTableView<'a> {
    instances: &'a [u8],
    chains: &'a [u8],
    levels: &'a [u8],
    origin_cell: IVec3,
    land_min: f32,
    aabb: Aabb,
    layers: u32,
}

impl<'a> StaticCellTableView<'a> {
    /// Checks the header, the size and every cross-reference (order, chains, levels, classes,
    /// flags, reserved bytes).
    pub fn parse(bytes: &'a [u8]) -> Result<Self, TableError> {
        if bytes.len() < 12 || bytes[..8] != MAGIC {
            return Err(TableError::BadMagic);
        }
        let version = u32_at(bytes, 8);
        if version != FORMAT_VERSION {
            return Err(TableError::Version(version));
        }
        if bytes.len() < HEADER {
            return Err(TableError::Size { expected: HEADER as u64, got: bytes.len() });
        }
        let [n_inst, n_chain, n_level] = [12, 16, 20].map(|o| u64::from(u32_at(bytes, o)));
        let expected =
            HEADER as u64 + INSTANCE as u64 * n_inst + CHAIN as u64 * n_chain + LEVEL as u64 * n_level;
        if expected != bytes.len() as u64 {
            return Err(TableError::Size { expected, got: bytes.len() });
        }
        if let Some(k) = bytes[68..HEADER].iter().position(|&b| b != 0) {
            return Err(TableError::Reserved(68 + k));
        }
        // The sections now fit in the slice, so these products fit in usize.
        #[allow(clippy::cast_possible_truncation)]
        let inst_end = HEADER + INSTANCE * n_inst as usize;
        #[allow(clippy::cast_possible_truncation)]
        let chain_end = inst_end + CHAIN * n_chain as usize;
        #[allow(clippy::cast_possible_wrap)] // two's complement round trip of the written i32
        let origin_cell = IVec3::new(u32_at(bytes, 24) as i32, u32_at(bytes, 28) as i32, u32_at(bytes, 32) as i32);
        let view = Self {
            instances: &bytes[HEADER..inst_end],
            chains: &bytes[inst_end..chain_end],
            levels: &bytes[chain_end..],
            origin_cell,
            land_min: f32_at(bytes, 36),
            aabb: Aabb { min: vec3_at(bytes, 40), max: vec3_at(bytes, 52) },
            layers: u32_at(bytes, 64),
        };
        view.validate(inst_end)?;
        Ok(view)
    }

    /// `chains_at`: the chain section's offset in the file (for [`TableError::Reserved`]).
    fn validate(&self, chains_at: usize) -> Result<(), TableError> {
        let mut prev: Option<u64> = None;
        for (index, c) in (0u32..).zip(self.chain_records()) {
            if prev.is_some_and(|p| p >= c.mesh) {
                return Err(TableError::ChainOrder(c.mesh));
            }
            prev = Some(c.mesh);
            let at = index as usize * CHAIN;
            if let Some(k) = self.chains[at + 10..at + 16].iter().position(|&b| b != 0) {
                return Err(TableError::Reserved(chains_at + at + 10 + k));
            }
            let end = u64::from(c.first_level) + u64::from(c.level_count);
            if c.level_count == 0 || c.level_count as usize > MAX_LOD_LEVELS || end > self.level_count() as u64 {
                return Err(TableError::ChainLevels(index));
            }
            if !self.chain_levels(c).all(|l| l.error.is_finite() && l.error >= 0.0) {
                return Err(TableError::LevelError(index));
            }
            if MaterialClass::from_u16(c.class).is_none() {
                return Err(TableError::MaterialClass { chain: index, class: c.class });
            }
        }
        let mut used = vec![false; self.chain_count()];
        let mut prev: Option<u32> = None;
        for i in self.instances() {
            if prev.is_some_and(|p| p > i.placement) {
                return Err(TableError::PlacementOrder(i.placement));
            }
            prev = Some(i.placement);
            match used.get_mut(i.chain as usize) {
                Some(u) => *u = true,
                None => return Err(TableError::MissingChain { placement: i.placement, chain: i.chain }),
            }
            if i.material_override != 0 {
                return Err(TableError::MaterialOverride(i.placement));
            }
            match InstanceFlags::from_bits(i.flags.bits()) {
                Some(f) if !f.contains(InstanceFlags::INACTIVE) => {}
                _ => return Err(TableError::Flags(i.placement)),
            }
        }
        if let Some(unused) = used.iter().position(|&u| !u) {
            return Err(TableError::UnusedChain(u32::try_from(unused).unwrap_or(u32::MAX)));
        }
        Ok(())
    }

    pub fn origin_cell(&self) -> IVec3 {
        self.origin_cell
    }

    pub fn aabb(&self) -> Aabb {
        self.aabb
    }

    pub fn land_min(&self) -> f32 {
        self.land_min
    }

    /// The view layers of every instance of the table.
    pub fn layers(&self) -> u32 {
        self.layers
    }

    pub fn len(&self) -> usize {
        self.instances.len() / INSTANCE
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// The instance section as stored, chain fields cell-local (module docs, "Binary layout").
    pub fn instance_bytes(&self) -> &'a [u8] {
        self.instances
    }

    /// Appends the instance section to `out` with each cell-local chain index `k` replaced by
    /// `slots[k]` (the chain's global slot, keyed by its mesh key): the stream-in copy.
    ///
    /// # Panics
    /// When `slots` does not hold one slot per chain of the table.
    pub fn write_remapped(&self, slots: &[u32], out: &mut Vec<u8>) {
        assert_eq!(slots.len(), self.chain_count(), "one global slot per chain of the table");
        out.reserve(self.instances.len());
        for record in self.instances.as_chunks::<INSTANCE>().0 {
            let start = out.len();
            out.extend_from_slice(record);
            // Validated: every chain index is in range.
            let slot = slots[u32_at(record, INSTANCE_CHAIN) as usize];
            out[start + INSTANCE_CHAIN..start + INSTANCE_CHAIN + 4].copy_from_slice(&slot.to_le_bytes());
        }
    }

    /// Instance `index` (table order), if in range.
    pub fn instance(&self, index: usize) -> Option<StaticInstance> {
        let b = self.instances.get(index * INSTANCE..(index + 1) * INSTANCE)?;
        Some(decode_instance(b))
    }

    pub fn instances(&self) -> impl ExactSizeIterator<Item = StaticInstance> + use<'a> {
        let bytes: &'a [u8] = self.instances;
        bytes.as_chunks::<INSTANCE>().0.iter().map(|b| decode_instance(b))
    }

    /// The table index of `placement`'s first instance (binary search: instances are sorted by
    /// placement). Its other parts' instances follow it ([`Self::find_all`]).
    pub fn find(&self, placement: PlacementId) -> Option<usize> {
        let (mut lo, mut hi) = (0, self.len());
        while lo < hi {
            let mid = usize::midpoint(lo, hi);
            if self.placement(mid) < placement {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        (lo < self.len() && self.placement(lo) == placement).then_some(lo)
    }

    /// The table indices of every instance of `placement` (empty when it has none).
    pub fn find_all(&self, placement: PlacementId) -> std::ops::Range<usize> {
        let Some(first) = self.find(placement) else { return 0..0 };
        let end = (first..self.len()).find(|&k| self.placement(k) != placement).unwrap_or(self.len());
        first..end
    }

    pub fn chain_count(&self) -> usize {
        self.chains.len() / CHAIN
    }

    /// The cell-local index of the chain with mesh key `mesh` (binary search: chains are sorted
    /// by mesh key).
    pub fn chain_index(&self, mesh: u64) -> Option<u32> {
        let (mut lo, mut hi) = (0, self.chain_count());
        while lo < hi {
            let mid = usize::midpoint(lo, hi);
            match u64_at(self.chains, mid * CHAIN + 16).cmp(&mesh) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
                std::cmp::Ordering::Equal => return u32::try_from(mid).ok(),
            }
        }
        None
    }

    /// The levels of chain `index`, finest first; `None` past the chain section.
    pub fn levels(&self, index: u32) -> Option<impl ExactSizeIterator<Item = LodLevel> + use<'a>> {
        let c = self.chain_record(index)?;
        Some(self.chain_levels(c))
    }

    /// The material class of chain `index`.
    pub fn material_class(&self, index: u32) -> Option<MaterialClass> {
        self.chain_record(index).and_then(|c| MaterialClass::from_u16(c.class))
    }

    /// Chain `index`, decoded.
    pub fn chain(&self, index: u32) -> Option<LodChain> {
        self.chain_record(index).map(|c| self.decode_chain(c))
    }

    /// Every chain, in index order.
    pub fn chains(&self) -> impl Iterator<Item = LodChain> + '_ {
        self.chain_records().map(move |c| self.decode_chain(c))
    }

    /// The owned table.
    pub fn to_table(&self) -> StaticCellTable {
        StaticCellTable {
            origin_cell: self.origin_cell,
            aabb: self.aabb,
            land_min: self.land_min,
            layers: self.layers,
            instances: self.instances().collect(),
            chains: self.chains().collect(),
        }
    }

    fn placement(&self, index: usize) -> PlacementId {
        u32_at(self.instances, index * INSTANCE + 64)
    }

    fn level_count(&self) -> usize {
        self.levels.len() / LEVEL
    }

    fn decode_chain(&self, c: ChainRecord) -> LodChain {
        LodChain {
            mesh: c.mesh,
            material: c.material,
            // Validated by `parse`.
            material_class: MaterialClass::from_u16(c.class).unwrap_or(MaterialClass::Opaque),
            levels: self.chain_levels(c).collect(),
        }
    }

    fn chain_records(&self) -> impl Iterator<Item = ChainRecord> + use<'a> {
        let bytes: &'a [u8] = self.chains;
        bytes.as_chunks::<CHAIN>().0.iter().map(|b| decode_chain_record(b))
    }

    fn chain_record(&self, index: u32) -> Option<ChainRecord> {
        let at = index as usize * CHAIN;
        self.chains.get(at..at + CHAIN).map(decode_chain_record)
    }

    /// Callers validated (or `validate` is checking) the range.
    fn chain_levels(&self, c: ChainRecord) -> impl ExactSizeIterator<Item = LodLevel> + use<'a> {
        let start = (c.first_level as usize * LEVEL).min(self.levels.len());
        let end = ((c.first_level as usize + c.level_count as usize) * LEVEL).min(self.levels.len());
        let bytes: &'a [u8] = self.levels;
        bytes[start..end].as_chunks::<LEVEL>().0.iter().map(|b| LodLevel {
            first_index: u32_at(b, 0),
            index_count: u32_at(b, 4),
            error: f32_at(b, 8),
        })
    }
}

#[derive(Clone, Copy)]
struct ChainRecord {
    first_level: u32,
    level_count: u32,
    class: u16,
    mesh: u64,
    material: u64,
}

fn decode_chain_record(b: &[u8]) -> ChainRecord {
    ChainRecord {
        first_level: u32_at(b, 0),
        level_count: u32_at(b, 4),
        class: u16_at(b, 8),
        mesh: u64_at(b, 16),
        material: u64_at(b, 24),
    }
}

fn decode_instance(b: &[u8]) -> StaticInstance {
    StaticInstance {
        transform: std::array::from_fn(|r| std::array::from_fn(|c| f32_at(b, (r * 4 + c) * 4))),
        sphere: Sphere { center: vec3_at(b, 48), radius: f32_at(b, 60) },
        placement: u32_at(b, 64),
        chain: u32_at(b, INSTANCE_CHAIN),
        material_override: u32_at(b, 72),
        flags: InstanceFlags::from_bits_retain(u32_at(b, 76)),
    }
}

fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_f32(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_vec3(out: &mut Vec<u8>, v: Vec3) {
    v.to_array().iter().for_each(|&x| put_f32(out, x));
}

// Readers below take offsets inside slices whose length was checked first.
fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn u64_at(b: &[u8], o: usize) -> u64 {
    u64::from(u32_at(b, o)) | (u64::from(u32_at(b, o + 4)) << 32)
}

fn f32_at(b: &[u8], o: usize) -> f32 {
    f32::from_bits(u32_at(b, o))
}

fn vec3_at(b: &[u8], o: usize) -> Vec3 {
    Vec3::new(f32_at(b, o), f32_at(b, o + 4), f32_at(b, o + 8))
}

#[cfg(test)]
mod tests;
