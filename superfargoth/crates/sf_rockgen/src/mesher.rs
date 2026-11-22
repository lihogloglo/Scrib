//! Solid → LOD chain (rockgen.md "(a) Boulders and rocks" steps 5-6, RG-4 "As built"): sample the [`Solid`] on a
//! padded grid, resolve ambiguous faces, surface nets (`fast-surface-nets`: closed, and edge-manifold once no
//! face is ambiguous), the `meshopt` LOD chain on that smooth net, then on a hard-edged solid the feature snap
//! (each vertex onto its cell's analytic planes, [`Solid::snap`]) gives every LOD's vertices their sharp
//! positions; SDF-gradient normals and crease-split normals per LOD.

use fast_surface_nets::ndshape::{RuntimeShape, Shape};
use fast_surface_nets::{SurfaceNetsBuffer, surface_nets};
use glam::Vec3;
use meshopt::{SimplifyOptions, VertexDataAdapter};

use crate::sdf::Solid;
use crate::{RockMesh, indices_closed};

/// Smallest closed triangle mesh (a tetrahedron): a LOD below this is not a solid.
// tunables: const (maths: a tetrahedron is the smallest closed mesh)
const MIN_CLOSED_TRIANGLES: usize = 4;
/// Cells of positive padding around the shape: the outer layer is outside, so the net is closed.
const PAD_CELLS: f32 = 2.0;
/// Edge of the sample blocks [`sample_field`] tries to skip (a loop layout, not a look value).
const SKIP_BLOCK: u32 = 4;
/// Simplification retries for a level that came out non-closed: each entry moves the target this fraction of
/// the way back toward the source (an algorithm schedule, not a look value).
const RETRY_SCHEDULE: [f32; 4] = [0.0, 0.25, 0.5, 0.75];
/// Position lattice of snapped vertices (unit frame, 2^-17: exact in f32, ~1/700 of the finest cell).
// tunables: const (numerical: float lattice)
const QUANTUM: f32 = 1.0 / 131_072.0;
/// Quanta a coincident copy moves off its twin.
// tunables: const (numerical: separation of coincident vertices)
const SEPARATE_QUANTA: f32 = 4.0;
/// A LOD keeps at least this fraction of the net's volume (below it the simplifier folded the solid).
// tunables: const (validity check of a LOD, not a look value)
const MIN_VOLUME_FRACTION: f64 = 0.5;
/// Passes of [`resolve_ambiguous_faces`] (a flip can make a neighbouring face ambiguous; a few passes settle it).
// tunables: const (algorithm schedule of the mesher)
const AMBIGUITY_PASSES: u32 = 8;
/// A flipped sample is set this many cells past zero (it moves the surface to that corner, no further).
// tunables: const (numerical: flipped sample value)
const FLIP_CELLS: f32 = 1.0e-3;
/// Error bound for LOD1+ (they are driven by the triangle target, not the error).
const UNBOUNDED_ERROR: f32 = 1.0;

/// How to mesh one solid (unit frame).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshSettings {
    /// Grid cell edge.
    pub voxel: f32,
    /// LOD0 simplification error (absolute, unit frame).
    pub lod0_error: f32,
    pub lod_levels: u32,
    pub lod_ratio: f32,
    /// Crease angle for split normals (≥ 180: smooth).
    pub crease_deg: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Vertex {
    p: [f32; 3],
    n: [f32; 3],
}

/// Meshes `solid` into its LOD chain (unit frame, finest first; may be shorter than `lod_levels` when a level no
/// longer simplifies). Deterministic.
pub fn mesh_solid(solid: &Solid, s: &MeshSettings) -> Vec<RockMesh> {
    let voxel = s.voxel;
    let dims = solid.reach().to_array().map(|r| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // small positive cell counts
        let cells = (2.0 * (r + PAD_CELLS * voxel) / voxel).ceil() as u32;
        cells + 1
    });
    #[allow(clippy::cast_precision_loss)] // dims ≤ a few hundred
    let origin = Vec3::from_array(dims.map(|n| -0.5 * (n - 1) as f32 * voxel));
    let shape = RuntimeShape::<u32, 3>::new(dims);
    let mut field = sample_field(solid, &shape, dims, origin, voxel);
    resolve_ambiguous_faces(&mut field, &shape, dims, voxel);
    let mut net = SurfaceNetsBuffer::default();
    surface_nets(&field, &shape, [0; 3], dims.map(|n| n - 1), &mut net);

    let mut positions: Vec<Vec3> = net.positions.iter().map(|&g| origin + Vec3::from_array(g) * voxel).collect();
    let mut indices = net.indices;
    let raw = RockMesh { positions: positions.iter().map(Vec3::to_array).collect(), normals: Vec::new(), indices: indices.clone() };
    if raw.signed_volume() < 0.0 {
        for tri in indices.as_chunks_mut::<3>().0 {
            tri.swap(1, 2);
        }
    }
    let unsnapped = positions.clone();
    if solid.snaps() {
        for (q, g) in positions.iter_mut().zip(&net.positions) {
            let cell = Vec3::from_array(*g).floor();
            if let Some(snapped) = solid.snap(*q, origin + cell * voxel) {
                *q = snapped;
            }
        }
        resplit_quads(solid, &positions, &mut indices);
        separate_coincident(&mut positions, &unsnapped);
    }

    // The chain is simplified on the snapped positions (flat faces collapse to a few triangles). Where meshopt
    // can't keep a snapped LOD0 closed (it can pinch two sheets along collinear edge rows), the chain is chosen
    // on the smooth net instead and its vertices still get the snapped positions (every LOD vertex is a net vertex).
    let lod_indices = match lod_chain(&positions, &indices, s) {
        (chain, true) => chain,
        (chain, false) if !solid.snaps() => chain,
        (_, false) => lod_chain(&unsnapped, &indices, s).0,
    };
    let step = solid.normal_step();
    let vertices: Vec<Vertex> = positions
        .iter()
        .zip(&net.normals)
        .map(|(&q, &fallback)| {
            let n = solid.gradient(q, step).normalize_or(Vec3::from_array(fallback).normalize_or(Vec3::Z));
            Vertex { p: q.to_array(), n: n.to_array() }
        })
        .collect();

    lod_indices
        .into_iter()
        .map(|idx| {
            let mut idx = meshopt::optimize_vertex_cache(&idx, vertices.len());
            let compact = meshopt::optimize_vertex_fetch(&mut idx, &vertices);
            let mesh = RockMesh {
                positions: compact.iter().map(|v| v.p).collect(),
                normals: compact.iter().map(|v| v.n).collect(),
                indices: idx,
            }
            .split_creases(s.crease_deg);
            // Split copies are appended: one more fetch pass keeps the vertex order cache-friendly.
            let split: Vec<Vertex> = mesh.positions.iter().zip(&mesh.normals).map(|(&p, &n)| Vertex { p, n }).collect();
            let mut idx = mesh.indices;
            let compact = meshopt::optimize_vertex_fetch(&mut idx, &split);
            RockMesh { positions: compact.iter().map(|v| v.p).collect(), normals: compact.iter().map(|v| v.n).collect(), indices: idx }
        })
        .collect()
}

/// Samples the field on the grid. Blocks of [`SKIP_BLOCK`]³ samples whose centre is farther from the surface
/// than the Lipschitz bound allows any of them to be near it are filled with the centre's value: their signs are
/// right and none is a corner of a sign-changing cell, so the net is the same as with every sample evaluated
/// (interval/Lipschitz pruning, as SDF modellers do). The outer layer is forced outside: the net is always closed.
fn sample_field(solid: &Solid, shape: &RuntimeShape<u32, 3>, dims: [u32; 3], origin: Vec3, voxel: f32) -> Vec<f32> {
    let mut field = vec![0.0f32; shape.usize()];
    #[allow(clippy::cast_precision_loss)]
    let at = |x: u32, y: u32, z: u32| origin + Vec3::new(x as f32, y as f32, z as f32) * voxel;
    let corner_reach = 3f32.sqrt() * voxel;
    for bz in (0..dims[2]).step_by(SKIP_BLOCK as usize) {
        for by in (0..dims[1]).step_by(SKIP_BLOCK as usize) {
            for bx in (0..dims[0]).step_by(SKIP_BLOCK as usize) {
                let lo = [bx, by, bz];
                let hi = [0, 1, 2].map(|k| (lo[k] + SKIP_BLOCK).min(dims[k]) - 1);
                let a = at(lo[0], lo[1], lo[2]);
                let b = at(hi[0], hi[1], hi[2]);
                let centre_d = solid.eval((a + b) * 0.5);
                let skip = centre_d.abs() > solid.lipschitz() * ((b - a).length() * 0.5 + corner_reach);
                for z in lo[2]..=hi[2] {
                    for y in lo[1]..=hi[1] {
                        for x in lo[0]..=hi[0] {
                            let d = if skip { centre_d } else { solid.eval(at(x, y, z)) };
                            field[shape.linearize([x, y, z]) as usize] = d;
                        }
                    }
                }
            }
        }
    }
    for z in 0..dims[2] {
        for y in 0..dims[1] {
            for x in 0..dims[0] {
                if [x, y, z].iter().zip(&dims).any(|(&c, &n)| c == 0 || c + 1 == n) {
                    let i = shape.linearize([x, y, z]) as usize;
                    field[i] = field[i].max(0.5 * voxel);
                }
            }
        }
    }
    field
}

/// Two cells can snap to the same point (a corner). Positions are rounded onto a lattice of [`QUANTUM`] (exact
/// in f32, so later scaling can't fold two distinct ones together), then each later copy of a lattice point is
/// moved a few quanta toward its own surface-nets position: no two vertices share a position, so a mesh welded by
/// position ([`RockMesh::is_closed`]) has the index topology. `meshopt` collapses the near-zero edges where the
/// topology allows.
fn separate_coincident(positions: &mut [Vec3], unsnapped: &[Vec3]) {
    let quantize = |q: Vec3| (q / QUANTUM).round() * QUANTUM;
    let mut seen: std::collections::HashSet<[u32; 3]> = std::collections::HashSet::with_capacity(positions.len());
    for (q, &from) in positions.iter_mut().zip(unsnapped) {
        *q = quantize(*q);
        let away = (from - *q).normalize_or(Vec3::X);
        let mut step = 1.0;
        #[allow(clippy::cast_precision_loss)]
        while !seen.insert(q.to_array().map(f32::to_bits)) {
            *q = quantize(*q + away * (SEPARATE_QUANTA * step * QUANTUM));
            step += 1.0;
        }
    }
}

/// Naive surface nets is non-manifold exactly where a grid face is ambiguous: its four corners alternate in sign
/// (four crossings, so the mesh edge between the two cells on that face gets four quads). Each such face gets its
/// corner nearest zero flipped to the other sign (the smallest change that removes the saddle, as marching cubes'
/// ambiguity resolution does), repeated until no face is ambiguous: every mesh edge then has exactly two faces.
/// The outer layer is never flipped (it stays outside, so the net stays closed), and no sample flips twice, so the
/// passes end.
fn resolve_ambiguous_faces(field: &mut [f32], shape: &RuntimeShape<u32, 3>, dims: [u32; 3], voxel: f32) {
    let flip = FLIP_CELLS * voxel;
    let inner = |p: [u32; 3]| (0..3).all(|k| p[k] > 0 && p[k] + 1 < dims[k]);
    // A sample flips at most once (else two neighbouring faces can flip one corner back and forth).
    let mut flipped = std::collections::HashSet::new();
    for _ in 0..AMBIGUITY_PASSES {
        let mut flips = 0;
        for axis in 0..3 {
            let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
            for z in 0..dims[2] {
                for y in 0..dims[1] {
                    for x in 0..dims[0] {
                        let p = [x, y, z];
                        if p[u] + 1 >= dims[u] || p[v] + 1 >= dims[v] {
                            continue;
                        }
                        let mut corners = [p; 4];
                        corners[1][u] += 1;
                        corners[2][u] += 1;
                        corners[2][v] += 1;
                        corners[3][v] += 1;
                        // Cyclic order 0, 1, 2, 3: ambiguous when the signs alternate around the face.
                        let at = corners.map(|c| shape.linearize(c) as usize);
                        let neg = at.map(|i| field[i] < 0.0);
                        if !(neg[0] == neg[2] && neg[1] == neg[3] && neg[0] != neg[1]) {
                            continue;
                        }
                        let pick = (0..4)
                            .filter(|&k| inner(corners[k]) && !flipped.contains(&at[k]))
                            .min_by(|&a, &b| field[at[a]].abs().total_cmp(&field[at[b]].abs()));
                        if let Some(k) = pick {
                            field[at[k]] = if neg[k] { flip } else { -flip };
                            flipped.insert(at[k]);
                            flips += 1;
                        }
                    }
                }
            }
        }
        if flips == 0 {
            break;
        }
    }
}

/// Surface nets emits each quad as two triangles split along the shorter diagonal. After the snap, a quad
/// straddling a crease has two vertices on the crease line and one on each face: the right diagonal is the one
/// that lies on the surface (along the crease), the other cuts a notch across it. Each quad keeps the diagonal
/// whose midpoint is nearer the surface (|field| smaller). Topology is unchanged: the two cells on a quad's
/// diagonal share only that quad, so no edge gains a third face.
#[allow(clippy::many_single_char_names)] // maths notation (points, planes, cells)
fn resplit_quads(solid: &Solid, positions: &[Vec3], indices: &mut [u32]) {
    for quad in indices.as_chunks_mut::<6>().0 {
        // Only true quads (two triangles sharing one diagonal, opposite directions) are touched.
        let (t1, t2) = ([quad[0], quad[1], quad[2]], [quad[3], quad[4], quad[5]]);
        // T1 rotated to (w, x, y) with w not in T2; then T2 = (y, x, u) and the quad cycle is w, x, u, y.
        let Some(r) = (0..3).find(|&k| !t2.contains(&t1[k])) else { continue };
        let (w, x, y) = (t1[r], t1[(r + 1) % 3], t1[(r + 2) % 3]);
        let Some(&u) = t2.iter().find(|v| !t1.contains(v)) else { continue };
        let Some(iy) = t2.iter().position(|&v| v == y) else { continue };
        if t2[(iy + 1) % 3] != x || t2[(iy + 2) % 3] != u {
            continue;
        }
        let p = |i: u32| positions[i as usize];
        let current = solid.surface((p(x) + p(y)) * 0.5).abs();
        let other = solid.surface((p(w) + p(u)) * 0.5).abs();
        if other < current {
            quad.copy_from_slice(&[w, x, u, w, u, y]);
        }
    }
}

/// LOD0 (error-bound simplification of the net) then levels at `lod_ratio` of the previous triangle count,
/// stopping once a level no longer reduces. `meshopt::simplify` keeps topology but can pinch two sheets into one
/// edge (seen on ~1 in 100 levels); a level that isn't closed is retried gentler ([`RETRY_SCHEDULE`]), and LOD0
/// falls back to the raw net, which is closed by construction (the flag says whether LOD0 was simplified).
fn lod_chain(positions: &[Vec3], indices: &[u32], s: &MeshSettings) -> (Vec<Vec<u32>>, bool) {
    let flat: Vec<[f32; 3]> = positions.iter().map(Vec3::to_array).collect();
    let Ok(adapter) = VertexDataAdapter::new(meshopt::typed_to_bytes(&flat), size_of::<[f32; 3]>(), 0) else {
        return (vec![indices.to_vec()], false);
    };
    let simplify = |source: &[u32], target_triangles: usize, error: f32| {
        meshopt::simplify(source, &adapter, target_triangles * 3, error, SimplifyOptions::ErrorAbsolute, None)
    };
    let volume = |idx: &[u32]| RockMesh { positions: flat.clone(), normals: Vec::new(), indices: idx.to_vec() }.signed_volume();
    let source_volume = volume(indices);
    // Closed, and still the solid: a level that lost half its volume (or turned inside out) is a collapse.
    let solid = |idx: &[u32]| {
        idx.len() >= 3 * MIN_CLOSED_TRIANGLES && indices_closed(idx) && volume(idx) > MIN_VOLUME_FRACTION * source_volume
    };

    let simplified = RETRY_SCHEDULE.iter().map(|gentler| simplify(indices, 0, s.lod0_error * (1.0 - gentler))).find(|idx| solid(idx));
    let lod0_simplified = simplified.is_some();
    let mut chain = vec![simplified.unwrap_or_else(|| indices.to_vec())];
    while chain.len() < s.lod_levels as usize {
        let source = chain.last().map_or(&[][..], Vec::as_slice);
        let prev = source.len() / 3;
        let next = RETRY_SCHEDULE.iter().find_map(|gentler| {
            // Gentler = closer to the previous count.
            let keep = s.lod_ratio + (1.0 - s.lod_ratio) * gentler;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
            let target = (prev as f32 * keep) as usize;
            if target < MIN_CLOSED_TRIANGLES || target >= prev {
                return None;
            }
            let idx = simplify(source, target, UNBOUNDED_ERROR);
            (idx.len() / 3 < prev && solid(&idx)).then_some(idx)
        });
        let Some(next) = next else { break };
        chain.push(next);
    }
    (chain, lod0_simplified)
}
