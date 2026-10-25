//! The adapter side: source tiles at their own spacing, and the mosaic that
//! stitches them into one lattice for resampling.

use std::collections::HashMap;

use glam::{DVec2, UVec2};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_world::{Fingerprint, WorldspaceId};

use crate::TerrainError;
use crate::layers::LayerId;
use crate::tile::sample_meters;

/// An adapter's own terrain tile (for example one source region), opaque to
/// the framework.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceTileId(pub u64);

/// What the bake planner needs about a source tile without reading it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SourceTileInfo {
    pub id: SourceTileId,
    /// Changes whenever the tile's content does (bake.md: input fingerprints).
    pub fingerprint: Fingerprint,
    /// XZ bounds of the tile's samples (Y ignored), so the planner can map
    /// canonical tiles to the source tiles they read.
    pub min: WorldPos,
    pub max: WorldPos,
}

/// Adapter side (Bevy-free). Adapters hand it to the terrain bake directly
/// (`sf_world` doesn't depend on `sf_terrain`).
pub trait TerrainSource: Send + Sync {
    /// Every source tile of the worldspace, in a stable order.
    fn tiles(&self, ws: &WorldspaceId, out: &mut dyn FnMut(SourceTileInfo));
    /// One tile's data, or `None` if the id is unknown.
    fn read(&self, ws: &WorldspaceId, tile: SourceTileId) -> Option<SourceTile>;
}

/// A quad diagonal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Diagonal {
    /// From sample (i, j) to (i+1, j+1).
    Main,
    /// From sample (i+1, j) to (i, j+1).
    Anti,
}

impl Diagonal {
    #[must_use]
    pub fn flipped(self) -> Self {
        match self {
            Self::Main => Self::Anti,
            Self::Anti => Self::Main,
        }
    }
}

/// How a source interpolated between its samples: the surface its authors
/// placed objects on. The constrain step reproduces it under footprints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Triangulation {
    Bilinear,
    /// Every quad split along the same diagonal.
    Fixed(Diagonal),
    /// Quads with even `i + j` (tile-local quad indices) split along the
    /// given diagonal, odd ones along the other (a checkerboard).
    Alternating(Diagonal),
}

/// A source heightfield tile.
#[derive(Clone, Debug)]
pub struct SourceTile {
    /// Position of sample (0, 0); Y is ignored.
    pub origin: WorldPos,
    /// Sample spacing in meters.
    pub spacing: f32,
    /// Samples per axis.
    pub size: UVec2,
    /// Meters (absolute world Y); row-major, x fastest, rows step +Z.
    pub heights: Vec<f32>,
    pub surface: Triangulation,
    pub layers: Option<SourceLayers>,
    pub tint: Option<SourceTint>,
}

/// Terrain layers on their own grid: each texel covers a square and names
/// one `TerrainLayer` record. Blending between texels happens on resample.
#[derive(Clone, Debug)]
pub struct SourceLayers {
    /// Minimum corner of texel (0, 0); Y is ignored.
    pub origin: WorldPos,
    pub spacing: f32,
    pub size: UVec2,
    /// `TerrainLayer` record ids used by this tile.
    pub palette: Vec<RecordId>,
    /// Index into `palette` per texel, row-major; `u16::MAX` = no layer.
    pub texels: Vec<u16>,
}

/// Macro color samples (for example source vertex colors).
#[derive(Clone, Debug)]
pub struct SourceTint {
    /// Position of sample (0, 0); Y is ignored.
    pub origin: WorldPos,
    pub spacing: f32,
    pub size: UVec2,
    pub rgb: Vec<[u8; 3]>,
}

/// A regular lattice of points `phase + index · spacing` on each axis, in
/// absolute meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Lattice {
    pub spacing: f64,
    pub phase: DVec2,
}

/// Phases are rounded to 0.1 mm and must agree across tiles within 0.5 mm:
/// f32 offsets in `WorldPos` carry a few µm of noise.
const PHASE_QUANTUM: f64 = 1e-4;
const PHASE_TOLERANCE: f64 = 5e-4;

impl Lattice {
    /// The lattice through `origin` (absolute meters per axis) with `spacing`,
    /// and the index of `origin` on it.
    fn through(origin: DVec2, spacing: f32) -> (Self, [i64; 2]) {
        let s = f64::from(spacing);
        let mut phase = [0.0; 2];
        let mut index = [0; 2];
        for a in 0..2 {
            let m = origin[a];
            let mut p = ((m - (m / s).floor() * s) / PHASE_QUANTUM).round() * PHASE_QUANTUM;
            if p >= s - PHASE_TOLERANCE {
                p = 0.0;
            }
            phase[a] = p;
            index[a] = round_i64((m - p) / s);
        }
        (Self { spacing: s, phase: DVec2::from_array(phase) }, index)
    }

    /// Index of `origin` on this lattice, if it lies on it.
    fn index_of(&self, origin: DVec2, spacing: f32) -> Option<[i64; 2]> {
        // Exact: tiles of one source share the same f32 spacing.
        #[allow(clippy::float_cmp)]
        if f64::from(spacing) != self.spacing {
            return None;
        }
        let mut index = [0; 2];
        for a in 0..2 {
            let k = (origin[a] - self.phase[a]) / self.spacing;
            index[a] = round_i64(k);
            #[allow(clippy::cast_precision_loss)]
            let err = (k - index[a] as f64).abs() * self.spacing;
            if err > PHASE_TOLERANCE {
                return None;
            }
        }
        Some(index)
    }

    /// Continuous lattice coordinate of level-0 global sample `g` on `axis`
    /// (0 = x, 1 = z). One formula everywhere, so equal `g` give equal bits.
    pub fn coord(&self, g: i64, axis: usize) -> f64 {
        (sample_meters(g) - self.phase[axis]) / self.spacing
    }

    /// Continuous coordinate of absolute meters `m` on `axis`.
    pub fn coord_m(&self, m: f64, axis: usize) -> f64 {
        (m - self.phase[axis]) / self.spacing
    }
}

#[allow(clippy::cast_possible_truncation)]
fn round_i64(v: f64) -> i64 {
    v.round() as i64
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn floor_i64(v: f64) -> i64 {
    v.floor() as i64
}

fn abs_xz(p: WorldPos) -> DVec2 {
    let d = p.to_f64();
    DVec2::new(d.x, d.z)
}

/// A rectangular window of lattice values.
pub(crate) struct Window<T> {
    pub min: [i64; 2],
    pub size: [usize; 2],
    pub data: Vec<T>,
}

impl<T: Copy> Window<T> {
    /// Value at global lattice index (i, j), which must be inside.
    pub fn at(&self, i: i64, j: i64) -> T {
        let x = usize::try_from(i - self.min[0]).expect("index left of window");
        let z = usize::try_from(j - self.min[1]).expect("index below window");
        debug_assert!(x < self.size[0] && z < self.size[1]);
        self.data[z * self.size[0] + x]
    }
}

struct Placed<T> {
    start: [i64; 2],
    size: [usize; 2],
    data: Vec<T>,
}

/// Lattice cells per bucket edge in the spatial index.
const BUCKET: i64 = 64;

/// Grids on one lattice, stitched. Where tiles overlap (shared borders),
/// the later tile in a fixed order wins: sorted by start (z, then x), then
/// by id. The winner depends only on the tiles, never on the window asked.
pub(crate) struct GridMosaic<T> {
    pub lattice: Lattice,
    tiles: Vec<Placed<T>>,
    buckets: HashMap<(i64, i64), Vec<u32>>,
}

impl<T: Copy> GridMosaic<T> {
    fn new(lattice: Lattice, mut tiles: Vec<Placed<T>>) -> Self {
        // Stable: tiles arrive sorted by id.
        tiles.sort_by_key(|t| (t.start[1], t.start[0]));
        let mut buckets: HashMap<(i64, i64), Vec<u32>> = HashMap::new();
        for (n, t) in tiles.iter().enumerate() {
            let n = u32::try_from(n).expect("too many source tiles");
            let (lo, hi) = bucket_range(t.start, t.size);
            for bz in lo.1..=hi.1 {
                for bx in lo.0..=hi.0 {
                    buckets.entry((bx, bz)).or_default().push(n);
                }
            }
        }
        Self { lattice, tiles, buckets }
    }

    /// Values for lattice indices `min .. min + size`; `fill` where no tile
    /// has a value.
    pub fn gather(&self, min: [i64; 2], size: [usize; 2], fill: T) -> Window<T> {
        let mut data = vec![fill; size[0] * size[1]];
        let (lo, hi) = bucket_range(min, size);
        let mut hits: Vec<u32> = Vec::new();
        for bz in lo.1..=hi.1 {
            for bx in lo.0..=hi.0 {
                if let Some(list) = self.buckets.get(&(bx, bz)) {
                    hits.extend_from_slice(list);
                }
            }
        }
        hits.sort_unstable();
        hits.dedup();
        let max = [min[0] + to_i64(size[0]), min[1] + to_i64(size[1])];
        for n in hits {
            let t = &self.tiles[n as usize];
            let x0 = t.start[0].max(min[0]);
            let x1 = (t.start[0] + to_i64(t.size[0])).min(max[0]);
            let z0 = t.start[1].max(min[1]);
            let z1 = (t.start[1] + to_i64(t.size[1])).min(max[1]);
            if x0 >= x1 || z0 >= z1 {
                continue;
            }
            let w = to_usize(x1 - x0);
            for z in z0..z1 {
                let src = to_usize(z - t.start[1]) * t.size[0] + to_usize(x0 - t.start[0]);
                let dst = to_usize(z - min[1]) * size[0] + to_usize(x0 - min[0]);
                data[dst..dst + w].copy_from_slice(&t.data[src..src + w]);
            }
        }
        Window { min, size, data }
    }
}

fn bucket_range(start: [i64; 2], size: [usize; 2]) -> ((i64, i64), (i64, i64)) {
    let last = |a: usize| start[a] + to_i64(size[a].max(1)) - 1;
    (
        (start[0].div_euclid(BUCKET), start[1].div_euclid(BUCKET)),
        (last(0).div_euclid(BUCKET), last(1).div_euclid(BUCKET)),
    )
}

pub(crate) fn to_i64(n: usize) -> i64 {
    i64::try_from(n).expect("size fits i64")
}

pub(crate) fn to_usize(n: i64) -> usize {
    usize::try_from(n).expect("non-negative offset")
}

/// Source tiles stitched onto one lattice per data kind: the input of
/// [`upsample`](crate::upsample), [`constrain`](crate::constrain), and the
/// layer and tint resamplers.
///
/// All tiles must share spacing, lattice, and triangulation (one source
/// format per worldspace). The lattice is taken from the lowest-id tile;
/// its phase is canonicalized, so a mosaic of any subset of the tiles
/// resamples to identical bits.
pub struct SourceMosaic {
    pub(crate) heights: GridMosaic<f32>,
    pub(crate) surface: Triangulation,
    /// Parity of `i0 + j0` (lattice index of sample (0, 0)) shared by every
    /// tile, for `Triangulation::Alternating`.
    flip: bool,
    pub(crate) fill_height: f32,
    pub(crate) layers: Option<GridMosaic<Option<LayerId>>>,
    pub(crate) tint: Option<GridMosaic<Option<[u8; 3]>>>,
}

impl SourceMosaic {
    /// Stitches `tiles`. `fill_height` stands in where no tile has a sample.
    /// `resolve_layer` maps `TerrainLayer` record ids to global layer ids.
    pub fn new(
        mut tiles: Vec<(SourceTileId, SourceTile)>,
        fill_height: f32,
        resolve_layer: &mut dyn FnMut(&RecordId) -> LayerId,
    ) -> Result<Self, TerrainError> {
        tiles.sort_by_key(|(id, _)| *id);
        let mut height_lattice: Option<Lattice> = None;
        let mut layer_lattice: Option<Lattice> = None;
        let mut tint_lattice: Option<Lattice> = None;
        let mut surface = None;
        let mut flip = None;
        let (mut heights, mut layers, mut tints) = (Vec::new(), Vec::new(), Vec::new());

        for (id, tile) in tiles {
            let size = [tile.size.x as usize, tile.size.y as usize];
            check_len(id, "heights", tile.heights.len(), size)?;
            let start = place(id, "heights", &mut height_lattice, abs_xz(tile.origin), tile.spacing)?;
            if *surface.get_or_insert(tile.surface) != tile.surface {
                return Err(TerrainError::Mismatch { id, what: "triangulation" });
            }
            let parity = (start[0] + start[1]) & 1 == 1;
            if matches!(tile.surface, Triangulation::Alternating(_)) && *flip.get_or_insert(parity) != parity {
                return Err(TerrainError::Mismatch { id, what: "alternating diagonal parity" });
            }
            heights.push(Placed { start, size, data: tile.heights });

            if let Some(l) = tile.layers {
                let size = [l.size.x as usize, l.size.y as usize];
                check_len(id, "layer texels", l.texels.len(), size)?;
                let start = place(id, "layers", &mut layer_lattice, abs_xz(l.origin), l.spacing)?;
                let ids: Vec<LayerId> = l.palette.iter().map(&mut *resolve_layer).collect();
                let data = l.texels.iter().map(|&t| ids.get(usize::from(t)).copied()).collect();
                layers.push(Placed { start, size, data });
            }
            if let Some(t) = tile.tint {
                let size = [t.size.x as usize, t.size.y as usize];
                check_len(id, "tint", t.rgb.len(), size)?;
                let start = place(id, "tint", &mut tint_lattice, abs_xz(t.origin), t.spacing)?;
                tints.push(Placed { start, size, data: t.rgb.into_iter().map(Some).collect() });
            }
        }

        let unit = Lattice { spacing: 1.0, phase: DVec2::ZERO };
        Ok(Self {
            heights: GridMosaic::new(height_lattice.unwrap_or(unit), heights),
            surface: surface.unwrap_or(Triangulation::Bilinear),
            flip: flip.unwrap_or(false),
            fill_height,
            layers: layer_lattice.map(|l| GridMosaic::new(l, layers)),
            tint: tint_lattice.map(|l| GridMosaic::new(l, tints)),
        })
    }

    /// Source sample spacing in meters.
    #[allow(clippy::cast_possible_truncation)]
    pub fn spacing(&self) -> f32 {
        self.heights.lattice.spacing as f32
    }

    pub fn surface(&self) -> Triangulation {
        self.surface
    }

    pub fn fill_height(&self) -> f32 {
        self.fill_height
    }

    /// The source's own surface at absolute XZ meters, interpolated as the
    /// source did (its [`Triangulation`]). For tests and tools; the bake
    /// uses windows.
    pub fn reference_height(&self, x: f64, z: f64) -> f32 {
        let l = self.heights.lattice;
        let (u, v) = (l.coord_m(x, 0), l.coord_m(z, 1));
        let w = self.heights.gather([floor_i64(u), floor_i64(v)], [2, 2], self.fill_height);
        self.reference_in(&w, u, v)
    }

    /// The reference surface at continuous lattice coordinates (u, v); the
    /// quad's four corners must be inside `w`.
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn reference_in(&self, w: &Window<f32>, u: f64, v: f64) -> f32 {
        let (ci, cj) = (floor_i64(u), floor_i64(v));
        let (fx, fz) = ((u - u.floor()) as f32, (v - v.floor()) as f32);
        let h00 = w.at(ci, cj);
        let h10 = w.at(ci + 1, cj);
        let h01 = w.at(ci, cj + 1);
        let h11 = w.at(ci + 1, cj + 1);
        let diagonal = match self.surface {
            Triangulation::Bilinear => {
                let a = h00 + (h10 - h00) * fx;
                let b = h01 + (h11 - h01) * fx;
                return a + (b - a) * fz;
            }
            Triangulation::Fixed(d) => d,
            Triangulation::Alternating(d) => {
                let odd = (ci + cj) & 1 == 1;
                if odd ^ self.flip {
                    d.flipped()
                } else {
                    d
                }
            }
        };
        triangle_height(diagonal, [h00, h10, h01, h11], fx, fz)
    }
}

/// Height inside a quad split along `d`, corners `[h00, h10, h01, h11]`,
/// at fractions (fx, fz) in [0, 1].
pub(crate) fn triangle_height(d: Diagonal, [h00, h10, h01, h11]: [f32; 4], fx: f32, fz: f32) -> f32 {
    match d {
        Diagonal::Main => {
            if fx >= fz {
                h00 + (h10 - h00) * fx + (h11 - h10) * fz
            } else {
                h00 + (h01 - h00) * fz + (h11 - h01) * fx
            }
        }
        Diagonal::Anti => {
            if fx + fz <= 1.0 {
                h00 + (h10 - h00) * fx + (h01 - h00) * fz
            } else {
                h11 + (h01 - h11) * (1.0 - fx) + (h10 - h11) * (1.0 - fz)
            }
        }
    }
}

fn check_len(id: SourceTileId, what: &'static str, got: usize, size: [usize; 2]) -> Result<(), TerrainError> {
    let expected = size[0] * size[1];
    if got == expected { Ok(()) } else { Err(TerrainError::BadSize { id, what, got, expected }) }
}

/// Puts a grid on the shared lattice of its kind (creating it from the
/// first tile) and returns the grid's start index.
fn place(
    id: SourceTileId,
    what: &'static str,
    lattice: &mut Option<Lattice>,
    origin: DVec2,
    spacing: f32,
) -> Result<[i64; 2], TerrainError> {
    if !(spacing > 0.0 && spacing.is_finite()) {
        return Err(TerrainError::Mismatch { id, what: "spacing (must be positive)" });
    }
    if let Some(l) = lattice {
        return l.index_of(origin, spacing).ok_or(TerrainError::OffLattice { id, what });
    }
    let (l, start) = Lattice::through(origin, spacing);
    *lattice = Some(l);
    Ok(start)
}
