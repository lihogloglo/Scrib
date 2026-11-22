//! Variant libraries (rockgen.md "Variant libraries and instancing (RG-2)", "Collision (RG-2)"): per rock class, a
//! fixed set of `rockgen.library.variants` boulders, each with its LOD chain and (with the `collision` feature) one
//! convex collision hull. A pure function of (class parameters, settings, library seed), so a bake can key the
//! product by [`Library::key`] and any peer regenerates the same rocks.
//!
//! - **Variant seeds:** variant `i`'s seed is [`variant_seed`]`(seed, i)`, independent of the variant count: growing a
//!   library from 16 to 24 keeps the first 16 rocks.
//! - **Size:** every variant is generated at the class's `size_m` (the library's reference size; resolution and
//!   LOD error follow it). A placement scales an instance uniformly by [`Library::scale_for`]; a size class far
//!   from the reference (the spec's 0.3-1 / 1-3 / 3-10 / 10-40 m bands) is its own library with its own `size_m`.
//! - **Selection:** a placement's seed picks the variant ([`Library::pick`]); scale passes through.
//! - **Parallel build:** one scoped thread per slice of variants, results stored by index: the library is the same
//!   for any thread count.

use crate::boulder::{clamp_f, clamp_u, tunables as t};
use crate::noise::mix64;
use crate::{BoulderParams, RockLods, generate_boulder};

/// Purpose words (any distinct values): variant seeds and placement picks never share numbers with the shape
/// streams (`boulder.rs`, `sdf.rs`).
const VARIANT_PURPOSE: u64 = 0x5a4e_0010;
const PICK_PURPOSE: u64 = 0x5a4e_0011;
/// Bumped whenever the same key would mean different rocks: any change to the generator (`boulder.rs`, `sdf.rs`,
/// `mesher.rs`, `columns.rs`, `noise.rs`), the LOD chain, the hull algorithm (`collision.rs`) or the serialized
/// types. It is in [`library_key`] and is the `rockgen.library` bake step's version, so a bump re-bakes every library.
// tunables: const (format version of the bake key)
pub(crate) const LIBRARY_FORMAT: u32 = 1;

/// Convex hull settings (`rockgen.collision.*`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HullSettings {
    /// Largest distance a LOD0 vertex may lie outside the hull, as a fraction of the rock's size.
    pub tolerance: f32,
    /// Cap on that distance in metres.
    pub tolerance_max_m: f32,
    /// Most hull vertices (the reduction stops there even if the tolerance isn't met; [`Hull::error_m`] says).
    pub max_vertices: u32,
}

impl Default for HullSettings {
    fn default() -> Self {
        Self {
            tolerance: t::HULL_TOLERANCE.default_value(),
            tolerance_max_m: t::HULL_TOLERANCE_MAX.default_value(),
            max_vertices: t::HULL_MAX_VERTICES.default_value(),
        }
    }
}

impl HullSettings {
    /// Every value clamped to its tunable's range (NaN → the default).
    #[must_use]
    pub fn clamped(self) -> Self {
        Self {
            tolerance: clamp_f(&t::HULL_TOLERANCE, self.tolerance),
            tolerance_max_m: clamp_f(&t::HULL_TOLERANCE_MAX, self.tolerance_max_m),
            max_vertices: clamp_u(&t::HULL_MAX_VERTICES, self.max_vertices),
        }
    }

    /// The tolerance in metres for a rock of `size_m`.
    pub fn tolerance_m(&self, size_m: f32) -> f32 {
        (self.tolerance * size_m).min(self.tolerance_max_m)
    }
}

/// Library settings (`rockgen.library.*` plus the hull's).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LibrarySettings {
    pub variants: u32,
    pub hull: HullSettings,
}

impl Default for LibrarySettings {
    fn default() -> Self {
        Self { variants: t::LIBRARY_VARIANTS.default_value(), hull: HullSettings::default() }
    }
}

impl LibrarySettings {
    /// Every value clamped to its tunable's range.
    #[must_use]
    pub fn clamped(self) -> Self {
        Self { variants: clamp_u(&t::LIBRARY_VARIANTS, self.variants), hull: self.hull.clamped() }
    }
}

/// One convex collision hull in the rock's mesh frame: a subset of LOD0's vertices (every coarser LOD's vertices
/// are LOD0's too, so it bounds the whole chain within the same error). Plain arrays so a bake product can store it;
/// `Hull::to_parry` (feature `collision`) builds the `parry3d` shape.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct Hull {
    pub points: Vec<[f32; 3]>,
    /// Outward counter-clockwise triangles over `points` (parry3d's quickhull output).
    pub triangles: Vec<[u32; 3]>,
    /// Largest distance of any LOD0 vertex outside the hull, in metres (0 = the exact hull).
    pub error_m: f32,
}

/// One library rock.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    /// Its boulder seed ([`variant_seed`]).
    pub seed: u64,
    pub lods: RockLods,
    /// The collision hull; `None` without the `collision` feature (or for a degenerate mesh).
    pub hull: Option<Hull>,
}

/// A class's variant library.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct Library {
    /// [`library_key`] of the inputs: the bake-product key.
    pub key: u64,
    /// Reference size every variant was generated at (LOD0 bounds' longest side), metres.
    pub size_m: f32,
    pub variants: Vec<Variant>,
}

impl Library {
    /// The variant a placement with this seed uses (uniform over the variants, deterministic).
    pub fn pick(&self, placement_seed: u64) -> usize {
        let n = u64::try_from(self.variants.len()).unwrap_or(0);
        let h = mix64(placement_seed ^ mix64(PICK_PURPOSE)) >> 32;
        // Multiply-shift range reduction (Lemire): h < 2^32, n ≤ 32, so the product fits and the result < n.
        usize::try_from((h * n) >> 32).unwrap_or(0)
    }

    /// Uniform instance scale that makes a variant `size_m` long.
    pub fn scale_for(&self, size_m: f32) -> f32 {
        if self.size_m > 0.0 { size_m / self.size_m } else { 1.0 }
    }

    /// FNV-1a 64 over every variant's seed, mesh hash and hull: the determinism check of the whole product.
    pub fn content_hash(&self) -> u64 {
        let mut h = Fnv::new();
        h.u64(self.key);
        for v in &self.variants {
            h.u64(v.seed);
            h.u64(v.lods.hash);
            match &v.hull {
                None => h.u32(0),
                Some(hull) => {
                    h.u32(u32::try_from(hull.points.len()).unwrap_or(u32::MAX));
                    for x in hull.points.iter().flatten() {
                        h.f32(*x);
                    }
                    for i in hull.triangles.iter().flatten() {
                        h.u32(*i);
                    }
                    h.f32(hull.error_m);
                }
            }
        }
        h.finish()
    }
}

/// Seed of variant `index` in the library seeded `library_seed` (independent of the variant count).
pub fn variant_seed(library_seed: u64, index: u32) -> u64 {
    mix64(library_seed ^ mix64(VARIANT_PURPOSE.wrapping_add(u64::from(index))))
}

/// Stable hash of everything that shapes the library: the clamped class parameters, the clamped settings, the seed
/// and the format version. The bake product key (RG-2b adds the crate's code version beside it).
pub fn library_key(params: &BoulderParams, settings: &LibrarySettings, seed: u64) -> u64 {
    // Exhaustive destructuring: a new parameter is a compile error here until it joins the key.
    let BoulderParams {
        size_m,
        aspect,
        joint_sets,
        joint_jitter_deg,
        chips,
        chip_depth,
        chip_spread,
        rounding,
        weather_amp,
        weather_wavelength,
        weather_octaves,
        resolution,
        lod0_error_voxels,
        lod_levels,
        lod_ratio,
        edge_snap,
        crease_deg,
        pit_density,
        pit_size,
        columns,
        column_jitter,
        column_groove,
        column_step,
        column_tilt_deg,
        column_fuse,
        resolution_ref_m,
        resolution_size_exp,
        resolution_max,
        lod0_error_max_m,
    } = params.clamped();
    let LibrarySettings { variants, hull: HullSettings { tolerance, tolerance_max_m, max_vertices } } =
        settings.clamped();
    let mut h = Fnv::new();
    h.u32(LIBRARY_FORMAT);
    h.u64(seed);
    for x in [size_m, aspect[0], aspect[1], joint_jitter_deg, chip_depth, chip_spread, rounding, weather_amp] {
        h.f32(x);
    }
    for x in [weather_wavelength, lod0_error_voxels, lod_ratio, crease_deg, pit_density, pit_size, column_jitter] {
        h.f32(x);
    }
    for x in [column_groove, column_step, column_tilt_deg, column_fuse, resolution_ref_m, resolution_size_exp] {
        h.f32(x);
    }
    for x in [lod0_error_max_m, tolerance, tolerance_max_m] {
        h.f32(x);
    }
    for x in [joint_sets, chips, weather_octaves, resolution, lod_levels, columns, resolution_max, u32::from(edge_snap)]
    {
        h.u32(x);
    }
    h.u32(variants);
    h.u32(max_vertices);
    h.finish()
}

/// Builds a class's library: `settings.variants` boulders from [`variant_seed`]s, each with its hull when the
/// `collision` feature is on. Deterministic in (`params`, `settings`, `seed`) and independent of thread count.
pub fn build_library(params: &BoulderParams, settings: &LibrarySettings, seed: u64) -> Library {
    let params = params.clamped();
    let settings = settings.clamped();
    let n = settings.variants;
    let threads = std::thread::available_parallelism()
        .map_or(1, std::num::NonZero::get)
        .clamp(1, usize::try_from(n).unwrap_or(1).max(1));
    let mut slots: Vec<Option<Variant>> = (0..n).map(|_| None).collect();
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..threads)
            .map(|first| {
                scope.spawn(move || {
                    (0..n)
                        .skip(first)
                        .step_by(threads)
                        .map(|i| (i, build_variant(&params, &settings.hull, variant_seed(seed, i))))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for job in jobs {
            for (i, v) in job.join().expect("library worker panicked") {
                slots[usize::try_from(i).unwrap_or(usize::MAX)] = Some(v);
            }
        }
    });
    Library {
        key: library_key(&params, &settings, seed),
        size_m: params.size_m,
        variants: slots.into_iter().map(|v| v.expect("every variant index is built once")).collect(),
    }
}

fn build_variant(params: &BoulderParams, hull: &HullSettings, seed: u64) -> Variant {
    let lods = generate_boulder(params, seed);
    let hull = variant_hull(&lods, hull, params.size_m);
    Variant { seed, lods, hull }
}

#[cfg(feature = "collision")]
fn variant_hull(lods: &RockLods, settings: &HullSettings, size_m: f32) -> Option<Hull> {
    lods.lods.first().and_then(|lod0| crate::collision::build_hull(lod0, settings, size_m))
}

#[cfg(not(feature = "collision"))]
fn variant_hull(_: &RockLods, _: &HullSettings, _: f32) -> Option<Hull> {
    None
}

/// FNV-1a 64 over little-endian words (same constants as [`crate::content_hash`]).
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn byte(&mut self, b: u8) {
        self.0 ^= u64::from(b);
        self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
    }

    fn u32(&mut self, x: u32) {
        for b in x.to_le_bytes() {
            self.byte(b);
        }
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.byte(b);
        }
    }

    fn f32(&mut self, x: f32) {
        self.u32(x.to_bits());
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests;
