//! The rock solid's signed distance field (inside negative, unit frame), shared by every rock class (rockgen.md
//! "(a) Boulders and rocks", RG-1 and RG-4): a union of convex [`Piece`]s (each the smooth max of its planes and an
//! optional bounding ellipsoid), minus vesicle pits, plus weathering noise. Because every face is a known plane,
//! [`Solid::snap`] can put a surface-nets vertex back onto the analytic planes active in its cell (the QEF vertex
//! rule of dual contouring, Ju et al. 2002), which is what keeps fracture faces flat and edges crisp.

use glam::{Mat3, Vec3};

use crate::noise::{Stream, fbm, mix64};

/// Noise octaves finer than this many cells alias on the grid (and can pinch the net): they are dropped.
// tunables: const (sampling limit: shorter wavelengths alias on the grid)
const MIN_NOISE_WAVELENGTH_CELLS: f32 = 4.0;
/// Pit rims and the ridges between pits are rounded by at least this many cells (sharper folds the net).
// tunables: const (sampling limit of the mesher)
const PIT_ROUNDING_CELLS: f32 = 1.5;
/// Pits smaller than this many cells (radius) are not resolved by the grid: they are dropped.
// tunables: const (sampling limit: a smaller pit is noise on the grid)
const MIN_PIT_CELLS: f32 = 2.5;
/// Two pits that don't overlap keep at least this many cells of rock between them (a thinner wall folds the net):
/// a later pit closer than that is dropped.
// tunables: const (sampling limit of the mesher)
const MIN_PIT_WALL_CELLS: f32 = 2.0;
/// A pit is kept when its centre is within this fraction of its radius of the surface (shallower: barely a
/// dent; deeper: a sealed bubble nobody sees).
// tunables: const (definition of an open pit, not a look value: the look is pit_density / pit_size)
const PIT_OPEN_FRACTION: f32 = 0.6;
/// Smallest pit radius as a fraction of the largest (pit radii are uniform in between).
// tunables: const (definition of the pit size law; the look value is pit_size)
const PIT_MIN_FRACTION: f32 = 0.6;
/// QEF pseudo-inverse: eigenvalues of `AᵀA` below this (unit normals) are treated as zero, so near-parallel
/// planes don't throw the vertex along their intersection (Lindstrom 2000, Ju et al. 2002 use 0.1 of the top
/// singular value; this keeps planes ≥ ~18 deg apart).
// tunables: const (numerical method: QEF truncation)
const QEF_EIGEN_MIN: f32 = 0.05;
/// A snapped vertex may move at most this many cells (one cell diagonal: a dual vertex stays near its cell).
// tunables: const (numerical method: dual contouring keeps the vertex near its cell)
const SNAP_REACH_CELLS: f32 = 1.732_050_8;
/// A snapped vertex is accepted only where the field is this close to zero, in cells (float tolerance).
// tunables: const (numerical method: acceptance tolerance of the projection)
const SNAP_EPS_CELLS: f32 = 0.02;
/// Newton steps that put a snapped vertex back onto the weathered surface.
// tunables: const (numerical method: iteration count)
const NEWTON_STEPS: u32 = 2;
/// Central-difference step for gradients, in cells.
// tunables: const (numerical method: central-difference step)
pub(crate) const GRADIENT_STEP_CELLS: f32 = 0.25;
/// Gradient step for vertex normals on a hard-edged solid, in cells: small enough that a face vertex's
/// differences see only its own plane, not the edge next to it.
// tunables: const (numerical method: central-difference step)
const HARD_NORMAL_STEP_CELLS: f32 = 1.0e-3;
/// Noise stream purpose (any distinct word).
const NOISE_STREAM: u64 = 0x5a4e_0002;
const PIT_STREAM: u64 = 0x5a4e_0003;

/// A half-space `n·p ≤ d` (`n` unit).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub n: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn value(&self, p: Vec3) -> f32 {
        self.n.dot(p) - self.d
    }
}

/// One convex piece: the smooth max of its planes (and of its ellipsoid's scaled-sphere distance, if any), then
/// the hard max of its `hard` planes: faces it shares with a neighbouring piece, which must never be rounded (two
/// rounded neighbours would leave a sub-cell notch along their shared face, which folds the net).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Piece {
    pub planes: Vec<Plane>,
    pub ellipsoid: Option<Vec3>,
    pub hard: Vec<Plane>,
}

fn ellipsoid_value(e: Vec3, p: Vec3) -> f32 {
    ((p / e).length() - 1.0) * e.min_element()
}

/// Polynomial smooth max (Quilez's smooth min mirrored): ≥ `max(a, b)`, so the blended solid lies inside both.
pub(crate) fn smax(a: f32, b: f32, k: f32) -> f32 {
    if k <= 0.0 {
        return a.max(b);
    }
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.max(b) + h * h * k * 0.25
}

impl Piece {
    fn value(&self, p: Vec3, rounding: f32) -> f32 {
        let mut d = self.ellipsoid.map_or(f32::NEG_INFINITY, |e| ellipsoid_value(e, p));
        for plane in &self.planes {
            d = smax(d, plane.value(p), rounding);
        }
        for plane in &self.hard {
            d = d.max(plane.value(p));
        }
        d
    }
}

/// Vesicle pits: at most one sphere per lattice cell (spacing = twice the largest radius, so the 27 cells around
/// a point hold every pit that can reach it), subtracted from the solid.
#[derive(Clone, Debug)]
struct Pits {
    origin: Vec3,
    spacing: f32,
    dims: [i32; 3],
    cells: Vec<Option<(Vec3, f32)>>,
    max_radius: f32,
}

impl Pits {
    /// `d` smooth-maxed (radius `k`) with `r - |p - c|` of every pit near `p`: the solid minus the pits.
    #[allow(clippy::many_single_char_names)] // maths notation (points, planes, cells)
    fn carve(&self, p: Vec3, mut d: f32, k: f32) -> f32 {
        let g = ((p - self.origin) / self.spacing).floor();
        #[allow(clippy::cast_possible_truncation)]
        let [gx, gy, gz] = [g.x as i32, g.y as i32, g.z as i32];
        for z in gz - 1..=gz + 1 {
            for y in gy - 1..=gy + 1 {
                for x in gx - 1..=gx + 1 {
                    if x < 0 || y < 0 || z < 0 || x >= self.dims[0] || y >= self.dims[1] || z >= self.dims[2] {
                        continue;
                    }
                    #[allow(clippy::cast_sign_loss)]
                    let i = (x + self.dims[0] * (y + self.dims[1] * z)) as usize;
                    if let Some((c, r)) = self.cells[i] {
                        d = smax(d, r - (p - c).length(), k);
                    }
                }
            }
        }
        d
    }
}

/// Weathering noise: `amp × fbm(p / wavelength)`, octaves finer than the grid can show dropped.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Weathering {
    pub amp: f32,
    pub wavelength: f32,
    pub octaves: u32,
}

/// The whole rock field. Build with [`Solid::new`], then optionally [`Solid::with_pits`].
#[derive(Clone, Debug)]
pub struct Solid {
    pieces: Vec<Piece>,
    /// Smooth-max radius of the sampled field (the mesh topology).
    rounding: f32,
    pits: Option<Pits>,
    pit_rounding: f32,
    amp: f32,
    inv_wavelength: f32,
    octaves: u32,
    noise_seed: u64,
    /// Outside `|base| > band` the noise can't flip the sign: the base value is returned as is.
    band: f32,
    /// Bound on the field's slope (used to skip blocks of samples far from the surface).
    lipschitz: f32,
    /// Half-extent of the region that holds the solid (noise included).
    reach: Vec3,
    voxel: f32,
    snap: bool,
}

impl Solid {
    /// A union of `pieces` (hard min), each the smooth max of its planes with radius `rounding`, weathered by
    /// `weathering`, meshed later at `voxel` cells. `reach` is the half-extent of the box holding the unweathered
    /// solid. With `snap` the sampled (rounded) field still decides the mesh topology, but [`Solid::snap`] puts
    /// the vertices onto the hard-edged solid (rounding 0): sharp edges on a net that stays manifold.
    pub fn new(pieces: Vec<Piece>, rounding: f32, weathering: Weathering, reach: Vec3, voxel: f32, snap: bool, seed: u64) -> Self {
        let mut octaves = 0;
        let mut wl = weathering.wavelength;
        while octaves < weathering.octaves && wl >= MIN_NOISE_WAVELENGTH_CELLS * voxel {
            octaves += 1;
            wl *= 0.5;
        }
        let amp = if octaves == 0 { 0.0 } else { weathering.amp };
        // Lipschitz bound: planes, the scaled-sphere ellipsoid, pits and their (smooth) max/min are 1-Lipschitz;
        // value noise with the quintic fade has a slope ≤ 1.875 per lattice unit per axis, range 2, and octave o
        // weighs 0.5^o at frequency 2^o, so the normalised fbm's slope is ≤ 1.875·2·√3·octaves/Σ0.5^o per lattice unit.
        let norm: f32 = (0..octaves).fold((0.0, 1.0), |(sum, a), _| (sum + a, a * 0.5)).0;
        #[allow(clippy::cast_precision_loss)] // octaves ≤ 6
        let noise_slope = if octaves == 0 { 0.0 } else { 1.875 * 2.0 * 3f32.sqrt() * octaves as f32 / norm };
        let wavelength = weathering.wavelength.max(f32::MIN_POSITIVE);
        let lipschitz = 1.0 + amp / wavelength * noise_slope;
        Self {
            pieces,
            rounding,
            pits: None,
            pit_rounding: rounding.max(PIT_ROUNDING_CELLS * voxel),
            amp,
            inv_wavelength: 1.0 / wavelength,
            octaves,
            noise_seed: mix64(seed ^ mix64(NOISE_STREAM)),
            // A corner of a sign-changing cell is within L·√3 cells of the surface; one more cell of margin.
            band: amp + (lipschitz * 3f32.sqrt() + 1.0) * voxel,
            lipschitz,
            reach: reach + Vec3::splat(1.5 * amp),
            voxel,
            snap,
        }
    }

    /// Subtracts vesicle pits: each cell of a lattice (spacing twice `max_radius`) holds one with chance
    /// `density`, radius uniform in [`PIT_MIN_FRACTION`, 1] × `max_radius`, kept only where it opens onto the
    /// surface. Pits under [`MIN_PIT_CELLS`] cells are dropped (the grid can't show them).
    #[must_use]
    #[allow(clippy::many_single_char_names)] // maths notation (points, planes, cells)
    pub fn with_pits(mut self, density: f32, max_radius: f32, seed: u64) -> Self {
        if density <= 0.0 || max_radius < MIN_PIT_CELLS * self.voxel {
            return self;
        }
        let spacing = 2.0 * max_radius;
        let origin = -self.reach - Vec3::splat(spacing);
        let n = ((2.0 * self.reach + Vec3::splat(2.0 * spacing)) / spacing).ceil();
        #[allow(clippy::cast_possible_truncation)]
        let dims = [n.x as i32, n.y as i32, n.z as i32];
        let mut rng = Stream::new(seed, PIT_STREAM);
        #[allow(clippy::cast_sign_loss)]
        let mut cells = vec![None; (dims[0] * dims[1] * dims[2]) as usize];
        let mut i = 0;
        let wall = MIN_PIT_WALL_CELLS * self.voxel;
        let min_radius = MIN_PIT_CELLS * self.voxel;
        for z in 0..dims[2] {
            for y in 0..dims[1] {
                for x in 0..dims[0] {
                    // Fixed draws per cell, so a density change never reshuffles the other pits.
                    let (keep, u, v, w, s) = (rng.unit(), rng.unit(), rng.unit(), rng.unit(), rng.unit());
                    #[allow(clippy::cast_precision_loss)]
                    let lo = origin + Vec3::new(x as f32, y as f32, z as f32) * spacing;
                    // Centre in the cell's middle half: the sphere (radius ≤ spacing/2) reaches only next cells.
                    let c = lo + Vec3::new(0.25 + 0.5 * u, 0.25 + 0.5 * v, 0.25 + 0.5 * w) * spacing;
                    let r = max_radius * (PIT_MIN_FRACTION + (1.0 - PIT_MIN_FRACTION) * s);
                    let depth = self.union(c, self.rounding);
                    if keep < density && r >= min_radius && depth.abs() < PIT_OPEN_FRACTION * r {
                        // Earlier neighbours (the 13 cells before this one in scan order are all that exist yet).
                        let thin = (-1..=1i32).flat_map(|dz| (-1..=1i32).flat_map(move |dy| (-1..=1i32).map(move |dx| (dx, dy, dz)))).any(
                            |(dx, dy, dz)| {
                                let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                                if nx < 0 || ny < 0 || nz < 0 || nx >= dims[0] || ny >= dims[1] || nz >= dims[2] {
                                    return false;
                                }
                                #[allow(clippy::cast_sign_loss)]
                                let j = (nx + dims[0] * (ny + dims[1] * nz)) as usize;
                                cells[j].is_some_and(|(c2, r2): (Vec3, f32)| {
                                    let gap = (c - c2).length() - r - r2;
                                    (0.0..wall).contains(&gap)
                                })
                            },
                        );
                        if !thin {
                            cells[i] = Some((c, r));
                        }
                    }
                    i += 1;
                }
            }
        }
        self.pits = Some(Pits { origin, spacing, dims, cells, max_radius });
        self
    }

    pub fn reach(&self) -> Vec3 {
        self.reach
    }

    pub fn lipschitz(&self) -> f32 {
        self.lipschitz
    }


    /// True when [`Solid::snap`] runs (vertices go onto the hard-edged solid).
    pub fn snaps(&self) -> bool {
        self.snap
    }

    /// The union of the pieces (no pits, no weathering), planes smooth-maxed with radius `k`.
    fn union(&self, p: Vec3, k: f32) -> f32 {
        self.pieces.iter().map(|piece| piece.value(p, k)).fold(f32::INFINITY, f32::min)
    }

    /// The field without weathering (pits carved), edges rounded by `k`.
    fn base(&self, p: Vec3, k: f32) -> f32 {
        let u = self.union(p, k);
        match &self.pits {
            Some(pits) if u > -2.0 * pits.max_radius => pits.carve(p, u, self.pit_rounding),
            _ => u,
        }
    }

    fn weathered(&self, p: Vec3, k: f32) -> f32 {
        let b = self.base(p, k);
        if self.amp <= 0.0 || b.abs() > self.band {
            return b;
        }
        b + self.amp * fbm(self.noise_seed, p * self.inv_wavelength, self.octaves)
    }

    /// Signed distance at `p` (unit frame): negative inside. The sampled field (edges rounded).
    pub fn eval(&self, p: Vec3) -> f32 {
        self.weathered(p, self.rounding)
    }

    /// The surface vertices are put on: hard edges when snapping, else [`Solid::eval`].
    pub fn surface(&self, p: Vec3) -> f32 {
        self.weathered(p, if self.snap { 0.0 } else { self.rounding })
    }

    /// Step for vertex-normal gradients: [`GRADIENT_STEP_CELLS`], or tiny on a hard-edged (snapped) solid.
    pub fn normal_step(&self) -> f32 {
        self.voxel * if self.snap { HARD_NORMAL_STEP_CELLS } else { GRADIENT_STEP_CELLS }
    }

    /// Central-difference gradient of [`Solid::surface`] (not normalised).
    pub fn gradient(&self, p: Vec3, h: f32) -> Vec3 {
        grad(|q| self.surface(q), p, h)
    }

    /// Feature snap of a surface-nets vertex `x` in the grid cell whose low corner is `cell_lo`: the planes (and
    /// linearised ellipsoids) of the pieces that cross the cell are the vertex's Hermite data; the least-squares
    /// point on them nearest `x` (QEF, truncated pseudo-inverse) is taken when it lies on the unweathered surface
    /// within one cell of `x`, trying all of them, then the three, two and one closest to `x`. Then Newton steps
    /// put it back on the weathered surface. `None` keeps the surface-nets position (pits, blends, no plane near).
    #[allow(clippy::many_single_char_names)] // maths notation (points, planes, cells)
    pub fn snap(&self, x: Vec3, cell_lo: Vec3) -> Option<Vec3> {
        if !self.snap {
            return None;
        }
        let v = self.voxel;
        let centre = cell_lo + Vec3::splat(0.5 * v);
        let mut terms: Vec<(f32, Plane)> = Vec::new();
        for piece in &self.pieces {
            // The sampled surface is rounded (radius `rounding`) inside the hard planes: a plane counts for a cell
            // it passes within that much of.
            let mut add = |plane: Plane| {
                if plane.value(centre).abs() <= 0.5 * v * plane.n.abs().element_sum() + self.rounding {
                    terms.push((plane.value(x).abs(), plane));
                }
            };
            for plane in piece.planes.iter().chain(&piece.hard) {
                add(*plane);
            }
            if let Some(e) = piece.ellipsoid {
                let g = (x / (e * e)) * (e.min_element() / (x / e).length().max(f32::MIN_POSITIVE));
                if let Some(n) = g.try_normalize() {
                    let value = ellipsoid_value(e, x) / g.length();
                    add(Plane { n, d: n.dot(x) - value });
                }
            }
        }
        if terms.is_empty() {
            return None;
        }
        terms.sort_by(|a, b| a.0.total_cmp(&b.0));
        let eps = SNAP_EPS_CELLS * v;
        let reach = SNAP_REACH_CELLS * v + self.rounding;
        let counts = [terms.len(), 3, 2, 1];
        let mut snapped = None;
        for (k, &count) in counts.iter().enumerate() {
            if count > terms.len() || (k > 0 && count >= terms.len()) {
                continue;
            }
            let planes: Vec<Plane> = terms[..count].iter().map(|t| t.1).collect();
            let mut q = qef(&planes, x);
            // One Newton step on the unweathered field (a linearised ellipsoid is only nearly right).
            let b = self.base(q, 0.0);
            if b.abs() > eps && b.abs() < 0.5 * v {
                let g = grad(|p| self.base(p, 0.0), q, GRADIENT_STEP_CELLS * v);
                if g.length_squared() > 0.0 {
                    q -= g * (b / g.length_squared());
                }
            }
            if (q - x).length() <= reach && self.base(q, 0.0).abs() <= eps {
                snapped = Some(q);
                break;
            }
        }
        let mut q = snapped?;
        if self.amp > 0.0 {
            for _ in 0..NEWTON_STEPS {
                let f = self.surface(q);
                let g = self.gradient(q, GRADIENT_STEP_CELLS * v);
                if g.length_squared() <= 0.0 {
                    break;
                }
                let next = q - g * (f / g.length_squared());
                if self.surface(next).abs() >= f.abs() || (next - x).length() > reach + self.amp {
                    break;
                }
                q = next;
            }
        }
        Some(q)
    }
}

fn grad(f: impl Fn(Vec3) -> f32, p: Vec3, h: f32) -> Vec3 {
    Vec3::new(f(p + Vec3::X * h) - f(p - Vec3::X * h), f(p + Vec3::Y * h) - f(p - Vec3::Y * h), f(p + Vec3::Z * h) - f(p - Vec3::Z * h))
}

/// The point minimising Σ (nᵢ·q − dᵢ)² nearest `mass` (dual contouring's QEF): `q = mass + (AᵀA)⁺ Aᵀ(d − A·mass)`
/// with the pseudo-inverse truncated at [`QEF_EIGEN_MIN`].
fn qef(planes: &[Plane], mass: Vec3) -> Vec3 {
    let mut ata = Mat3::ZERO;
    let mut atb = Vec3::ZERO;
    for p in planes {
        ata += Mat3::from_cols(p.n * p.n.x, p.n * p.n.y, p.n * p.n.z);
        atb += p.n * (p.d - p.n.dot(mass));
    }
    let (values, vectors) = jacobi_eigen(ata);
    let mut delta = Vec3::ZERO;
    for (k, &value) in values.iter().enumerate() {
        if value > QEF_EIGEN_MIN {
            let e = vectors.col(k);
            delta += e * (e.dot(atb) / value);
        }
    }
    mass + delta
}

/// Eigenvalues and eigenvectors (columns) of a symmetric 3×3 matrix by cyclic Jacobi rotations.
#[allow(clippy::many_single_char_names, clippy::needless_range_loop)] // the textbook's notation and row updates
fn jacobi_eigen(m: Mat3) -> ([f32; 3], Mat3) {
    // tunables: const (numerical method: Jacobi sweeps; 3×3 converges in a handful)
    const SWEEPS: usize = 12;
    let mut a = m.to_cols_array_2d();
    let mut v = [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    for _ in 0..SWEEPS {
        let off = a[0][1].abs() + a[0][2].abs() + a[1][2].abs();
        if off < 1e-9 {
            break;
        }
        for (p, q) in [(0, 1), (0, 2), (1, 2)] {
            if a[p][q].abs() < 1e-12 {
                continue;
            }
            let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
            let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
            let t = if theta == 0.0 { 1.0 } else { t };
            let c = 1.0 / (t * t + 1.0).sqrt();
            let s = t * c;
            for row in &mut a {
                let (x, y) = (row[p], row[q]);
                row[p] = c * x - s * y;
                row[q] = s * x + c * y;
            }
            for k in 0..3 {
                let (x, y) = (a[p][k], a[q][k]);
                a[p][k] = c * x - s * y;
                a[q][k] = s * x + c * y;
            }
            for row in &mut v {
                let (x, y) = (row[p], row[q]);
                row[p] = c * x - s * y;
                row[q] = s * x + c * y;
            }
        }
    }
    // v is stored row-major here (v[row][col]); column k is the k-th eigenvector.
    let vectors = Mat3::from_cols(
        Vec3::new(v[0][0], v[1][0], v[2][0]),
        Vec3::new(v[0][1], v[1][1], v[2][1]),
        Vec3::new(v[0][2], v[1][2], v[2][2]),
    );
    ([a[0][0], a[1][1], a[2][2]], vectors)
}

#[cfg(test)]
mod qef_tests {
    use super::*;

    #[test]
    fn qef_finds_a_corner_an_edge_and_a_face() {
        let planes = [Plane { n: Vec3::X, d: 1.0 }, Plane { n: Vec3::Y, d: 2.0 }, Plane { n: Vec3::Z, d: 3.0 }];
        let mass = Vec3::new(0.9, 1.8, 2.7);
        assert!((qef(&planes, mass) - Vec3::new(1.0, 2.0, 3.0)).length() < 1e-5);
        // Two planes: the line x = 1, y = 2; z stays at the mass point.
        assert!((qef(&planes[..2], mass) - Vec3::new(1.0, 2.0, 2.7)).length() < 1e-5);
        assert!((qef(&planes[..1], mass) - Vec3::new(1.0, 1.8, 2.7)).length() < 1e-5);
        // A tilted pair (a 45 deg edge).
        let n = Vec3::new(1.0, 1.0, 0.0).normalize();
        let tilted = [Plane { n, d: 0.0 }, Plane { n: Vec3::Z, d: 0.5 }];
        let q = qef(&tilted, Vec3::new(0.3, 0.1, 0.4));
        assert!(tilted.iter().all(|p| p.value(q).abs() < 1e-5), "{q}");
    }
}
