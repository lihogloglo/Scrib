//! Boulders (rockgen.md "(a) Boulders and rocks", slices RG-1 and RG-4): an SDF block cut by joint-set slabs and
//! chip planes (or a cluster of columns), smooth-max rounded or hard-edged, pitted and noise weathered, meshed by
//! surface nets with the feature snap ([`crate::mesher`]), simplified into a LOD chain by `meshopt`. A pure
//! function of ([`BoulderParams`], seed).
//!
//! Built in a unit frame (long axis 1, +Z the short axis), then scaled so the LOD0 bounds' longest side is
//! `size_m` and centred on the origin. Smooth max is Quilez's polynomial smooth min, mirrored
//! (iquilezles.org "smooth minimum"); the joint-set block follows the geology of research
//! `terrain-sharp-features-2026-11-19.md` §4 (rock breaks along 2-3 families of near-parallel planes).

pub mod tunables;

use glam::Vec3;
use sf_tunables::Def;

use crate::columns::{ColumnSpec, column_cluster};
use crate::mesher::{MeshSettings, mesh_solid};
use crate::noise::Stream;
use crate::sdf::{Piece, Plane, Solid, Weathering};
use crate::{Aabb, RockLods, RockMesh, content_hash};

/// Long axis of the unit frame the field is built in.
// tunables: const (units: the unit frame)
const UNIT_LONG_AXIS: f32 = 1.0;
/// The sampled field's rounding never goes below this many cells: a sharper edge than the grid can resolve folds
/// the net (with the edge snap, vertices still go onto the hard edge).
// tunables: const (sampling limit of the mesher, not a look value)
const MIN_ROUNDING_CELLS: f32 = 2.0;
/// A chip never removes more than this fraction of the block's support in its direction: past that it is a
/// different block, not a chip (definition, not a look value: the look is `chip_depth`).
// tunables: const (definition of a chip; the look value is chip_depth)
const MAX_CHIP_FRACTION: f32 = 0.5;
/// Stream purpose (any distinct word): shape draws and noise lattice never share numbers.
const SHAPE_STREAM: u64 = 0x5a4e_0001;

/// One rock class's parameters. Defaults are the `rockgen.boulder.*` tunables; an adapter fills per-type values
/// as data ([`crate::classes`] holds the viewer presets). Ratios are fractions of the rock's size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoulderParams {
    pub size_m: f32,
    pub aspect: [f32; 2],
    pub joint_sets: u32,
    pub joint_jitter_deg: f32,
    pub chips: u32,
    pub chip_depth: f32,
    pub chip_spread: f32,
    pub rounding: f32,
    pub weather_amp: f32,
    pub weather_wavelength: f32,
    pub weather_octaves: u32,
    pub resolution: u32,
    pub lod0_error_voxels: f32,
    pub lod_levels: u32,
    pub lod_ratio: f32,
    pub edge_snap: bool,
    pub crease_deg: f32,
    pub pit_density: f32,
    pub pit_size: f32,
    pub columns: u32,
    pub column_jitter: f32,
    pub column_groove: f32,
    pub column_step: f32,
    pub column_tilt_deg: f32,
    pub column_fuse: f32,
    pub resolution_ref_m: f32,
    pub resolution_size_exp: f32,
    pub resolution_max: u32,
    pub lod0_error_max_m: f32,
}

impl Default for BoulderParams {
    fn default() -> Self {
        use tunables as t;
        Self {
            size_m: t::SIZE.default_value(),
            aspect: t::ASPECT.default_value(),
            joint_sets: t::JOINT_SETS.default_value(),
            joint_jitter_deg: t::JOINT_JITTER.default_value(),
            chips: t::CHIPS.default_value(),
            chip_depth: t::CHIP_DEPTH.default_value(),
            chip_spread: t::CHIP_SPREAD.default_value(),
            rounding: t::ROUNDING.default_value(),
            weather_amp: t::WEATHER_AMP.default_value(),
            weather_wavelength: t::WEATHER_WAVELENGTH.default_value(),
            weather_octaves: t::WEATHER_OCTAVES.default_value(),
            resolution: t::RESOLUTION.default_value(),
            lod0_error_voxels: t::LOD0_ERROR.default_value(),
            lod_levels: t::LOD_LEVELS.default_value(),
            lod_ratio: t::LOD_RATIO.default_value(),
            edge_snap: t::EDGE_SNAP.default_value(),
            crease_deg: t::CREASE.default_value(),
            pit_density: t::PIT_DENSITY.default_value(),
            pit_size: t::PIT_SIZE.default_value(),
            columns: t::COLUMNS.default_value(),
            column_jitter: t::COLUMN_JITTER.default_value(),
            column_groove: t::COLUMN_GROOVE.default_value(),
            column_step: t::COLUMN_STEP.default_value(),
            column_tilt_deg: t::COLUMN_TILT.default_value(),
            column_fuse: t::COLUMN_FUSE.default_value(),
            resolution_ref_m: t::RESOLUTION_REF.default_value(),
            resolution_size_exp: t::RESOLUTION_SIZE_EXP.default_value(),
            resolution_max: t::RESOLUTION_MAX.default_value(),
            lod0_error_max_m: t::LOD0_ERROR_MAX.default_value(),
        }
    }
}

pub(crate) fn clamp_f(def: &Def<f32>, v: f32) -> f32 {
    let (lo, hi) = def.bounds().unwrap_or((v, v));
    if v.is_nan() { def.default_value() } else { v.clamp(lo, hi) }
}

pub(crate) fn clamp_u(def: &Def<u32>, v: u32) -> u32 {
    let (lo, hi) = def.bounds().unwrap_or((v, v));
    v.clamp(lo, hi)
}

impl BoulderParams {
    /// Every value clamped to its tunable's declared range (NaN → the default).
    #[must_use]
    pub fn clamped(self) -> Self {
        use tunables as t;
        let (alo, ahi) = t::ASPECT.bounds().unwrap_or(([0.0; 2], [1.0; 2]));
        let aspect = [0, 1].map(|k| {
            let v = self.aspect[k];
            if v.is_nan() { t::ASPECT.default_value()[k] } else { v.clamp(alo[k], ahi[k]) }
        });
        Self {
            size_m: clamp_f(&t::SIZE, self.size_m),
            aspect,
            joint_sets: clamp_u(&t::JOINT_SETS, self.joint_sets),
            joint_jitter_deg: clamp_f(&t::JOINT_JITTER, self.joint_jitter_deg),
            chips: clamp_u(&t::CHIPS, self.chips),
            chip_depth: clamp_f(&t::CHIP_DEPTH, self.chip_depth),
            chip_spread: clamp_f(&t::CHIP_SPREAD, self.chip_spread),
            rounding: clamp_f(&t::ROUNDING, self.rounding),
            weather_amp: clamp_f(&t::WEATHER_AMP, self.weather_amp),
            weather_wavelength: clamp_f(&t::WEATHER_WAVELENGTH, self.weather_wavelength),
            weather_octaves: clamp_u(&t::WEATHER_OCTAVES, self.weather_octaves),
            resolution: clamp_u(&t::RESOLUTION, self.resolution),
            lod0_error_voxels: clamp_f(&t::LOD0_ERROR, self.lod0_error_voxels),
            lod_levels: clamp_u(&t::LOD_LEVELS, self.lod_levels),
            lod_ratio: clamp_f(&t::LOD_RATIO, self.lod_ratio),
            edge_snap: self.edge_snap,
            crease_deg: clamp_f(&t::CREASE, self.crease_deg),
            pit_density: clamp_f(&t::PIT_DENSITY, self.pit_density),
            pit_size: clamp_f(&t::PIT_SIZE, self.pit_size),
            columns: clamp_u(&t::COLUMNS, self.columns),
            column_jitter: clamp_f(&t::COLUMN_JITTER, self.column_jitter),
            column_groove: clamp_f(&t::COLUMN_GROOVE, self.column_groove),
            column_step: clamp_f(&t::COLUMN_STEP, self.column_step),
            column_tilt_deg: clamp_f(&t::COLUMN_TILT, self.column_tilt_deg),
            column_fuse: clamp_f(&t::COLUMN_FUSE, self.column_fuse),
            resolution_ref_m: clamp_f(&t::RESOLUTION_REF, self.resolution_ref_m),
            resolution_size_exp: clamp_f(&t::RESOLUTION_SIZE_EXP, self.resolution_size_exp),
            resolution_max: clamp_u(&t::RESOLUTION_MAX, self.resolution_max),
            lod0_error_max_m: clamp_f(&t::LOD0_ERROR_MAX, self.lod0_error_max_m),
        }
    }

    /// Meshing cells along the long axis for this rock's size: `resolution` up to `resolution_ref_m`, then
    /// growing with `(size / ref)^resolution_size_exp`, at most `resolution_max` (never below `resolution`).
    pub fn effective_resolution(&self) -> u32 {
        let grow = (self.size_m / self.resolution_ref_m).max(1.0).powf(self.resolution_size_exp);
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // ≤ 192
        let cells = (self.resolution as f32 * grow).round() as u32;
        cells.min(self.resolution_max).max(self.resolution)
    }

    /// Mesh cell edge in the unit frame.
    fn voxel(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)] // resolution ≤ 192
        let cells = self.effective_resolution() as f32;
        UNIT_LONG_AXIS / cells
    }

    /// LOD0 error in the unit frame: `lod0_error_voxels` cells, at most `lod0_error_max_m` metres.
    fn lod0_error(&self) -> f32 {
        (self.lod0_error_voxels * self.voxel()).min(self.lod0_error_max_m / self.size_m * UNIT_LONG_AXIS)
    }
}

fn sign(x: f32) -> f32 {
    if x < 0.0 { -1.0 } else { 1.0 }
}

/// The boulder's solid (unit frame): a joint-set block cut by chips, or a column cluster (`columns` > 0) whose
/// union is cut by the chips and its floor once ([`Solid::with_cap`]); then pits and weathering.
pub fn boulder_solid(params: &BoulderParams, seed: u64) -> Solid {
    let p = params.clamped();
    let voxel = p.voxel();
    let mut rng = Stream::new(seed, SHAPE_STREAM);
    // Half extents, long to short: x long, z short.
    let mut aspect = p.aspect;
    aspect.sort_by(|a, b| b.total_cmp(a));
    let half = Vec3::new(0.5, 0.5 * aspect[0], 0.5 * aspect[1]) * UNIT_LONG_AXIS;

    let (mut pieces, mut cap, reach, ellipsoid) = if p.columns > 0 {
        let spec = ColumnSpec {
            count: p.columns,
            jitter: p.column_jitter,
            groove: p.column_groove,
            step: p.column_step,
            tilt_deg: p.column_tilt_deg,
            fuse: p.column_fuse,
        };
        let (pieces, cap, reach) = column_cluster(&spec, half, voxel, &mut rng);
        (pieces, cap, reach, reach)
    } else {
        let (piece, reach) = joint_block(&p, half, &mut rng);
        let ellipsoid = piece.ellipsoid.unwrap_or(reach);
        (vec![piece], Vec::new(), reach, ellipsoid)
    };
    // A block's chips are its own planes; a cluster's cut the union once (rounded across the columns).
    let shared_chips = p.columns > 0;
    for _ in 0..p.chips {
        // Toward a random corner, weighted so the cut is symmetric relative to the extents.
        let corner = Vec3::new(sign(rng.signed()), sign(rng.signed()), sign(rng.signed()));
        let n = (corner / half * half.min_element() + rng.direction() * p.chip_spread).normalize_or(corner);
        let support_box = (half * n.abs()).element_sum();
        let support_ell = (ellipsoid * n).length();
        let support = support_box.min(support_ell);
        // √u: deep cuts as likely as shallow ones per unit of removed area.
        let depth = p.chip_depth * UNIT_LONG_AXIS * rng.unit().sqrt();
        let chip = Plane { n, d: (support - depth).max((1.0 - MAX_CHIP_FRACTION) * support) };
        if shared_chips {
            cap.push(chip);
        } else {
            for piece in &mut pieces {
                piece.planes.push(chip);
            }
        }
    }

    // A rounding under one cell is a hard edge: the snap puts the vertices on the hard planes, while the sampled
    // field keeps at least MIN_ROUNDING_CELLS (the net's topology stays manifold at acute wedges).
    let snap_wanted = p.edge_snap && p.rounding * UNIT_LONG_AXIS < voxel;
    let rounding = (p.rounding * UNIT_LONG_AXIS).max(MIN_ROUNDING_CELLS * voxel);
    let weathering = Weathering {
        amp: p.weather_amp * UNIT_LONG_AXIS,
        wavelength: p.weather_wavelength * UNIT_LONG_AXIS,
        octaves: p.weather_octaves,
    };
    Solid::new(pieces, rounding, weathering, reach, voxel, snap_wanted, seed).with_cap(cap).with_pits(p.pit_density, p.pit_size * UNIT_LONG_AXIS, seed)
}

/// The joint-set block (RG-1): three near-orthogonal families of near-parallel faces (fewer: the ellipsoid
/// bounds the remaining axes), and the half-extent its solid can reach.
fn joint_block(p: &BoulderParams, half: Vec3, rng: &mut Stream) -> (Piece, Vec3) {
    let axes = [Vec3::X, Vec3::Y, Vec3::Z];
    let jointed = |k: usize| k + (p.joint_sets as usize) >= 3; // short axis (z) first
    let ellipsoid = Vec3::from_array([0, 1, 2].map(|k| if jointed(k) { half[k] * 3f32.sqrt() } else { half[k] }));
    let mut planes = Vec::new();
    let max_tilt = p.joint_jitter_deg.to_radians();
    // Where the solid can reach along each axis: the ellipsoid, or tighter, a tilted joint face's bound.
    let mut solid_reach = ellipsoid;
    for k in (0..3).rev().filter(|&k| jointed(k)) {
        let across = (0..3).filter(|&j| j != k).map(|j| ellipsoid[j]).fold(0.0, f32::max);
        let mut face_reach = 0.0f32;
        // The two faces of one family are near-parallel, each tilted on its own.
        for side in [1.0f32, -1.0] {
            let r = rng.direction();
            let perp = (r - axes[k] * r.dot(axes[k])).normalize_or(axes[(k + 1) % 3]);
            let tilt = max_tilt * rng.unit();
            let n = (axes[k] * tilt.cos() + perp * tilt.sin()) * side;
            planes.push(Plane { n, d: half[k] });
            // n·q ≤ d with |q⊥| ≤ across bounds q_k·cos t ≤ d + sin t·across.
            face_reach = face_reach.max((half[k] + tilt.sin() * across) / tilt.cos());
        }
        solid_reach[k] = solid_reach[k].min(face_reach);
    }
    (Piece { planes, ellipsoid: Some(ellipsoid), hard: Vec::new() }, solid_reach)
}

/// Generates one boulder: the LOD chain, bounds and content hash. Deterministic in (`params`, `seed`).
pub fn generate_boulder(params: &BoulderParams, seed: u64) -> RockLods {
    let p = params.clamped();
    let solid = boulder_solid(&p, seed);
    let settings = MeshSettings {
        voxel: p.voxel(),
        lod0_error: p.lod0_error(),
        lod_levels: p.lod_levels,
        lod_ratio: p.lod_ratio,
        crease_deg: p.crease_deg,
    };
    scaled(mesh_solid(&solid, &settings), p.size_m)
}

/// Scales the chain so LOD0's longest bounds side is `size_m`, centred on the origin, and hashes it.
pub fn scaled(mut lods: Vec<RockMesh>, size_m: f32) -> RockLods {
    let unit_bounds = lods.first().map_or(Aabb::EMPTY, RockMesh::bounds);
    let extent = unit_bounds.extent();
    let longest = extent[0].max(extent[1]).max(extent[2]);
    let scale = if longest > 0.0 { size_m / longest } else { 1.0 };
    let centre = (Vec3::from_array(unit_bounds.min) + Vec3::from_array(unit_bounds.max)) * 0.5;
    for mesh in &mut lods {
        for q in &mut mesh.positions {
            *q = ((Vec3::from_array(*q) - centre) * scale).to_array();
        }
    }
    let bounds = lods.first().map_or(Aabb::EMPTY, RockMesh::bounds);
    let hash = content_hash(&lods);
    RockLods { lods, bounds, hash }
}
