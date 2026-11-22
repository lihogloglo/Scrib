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
// tunables: const (algorithm: the outer layer must be outside for the net to close)
const PAD_CELLS: f32 = 2.0;
/// Edge of the sample blocks [`sample_field`] tries to skip (a loop layout, not a look value).
// tunables: const (loop layout of the sampler)
const SKIP_BLOCK: u32 = 4;
/// Simplification retries for a level that came out non-closed: each entry moves the target this fraction of
/// the way back toward the source (an algorithm schedule, not a look value).
// tunables: const (algorithm schedule of the mesher)
const RETRY_SCHEDULE: [f32; 4] = [0.0, 0.25, 0.5, 0.75];
/// Position lattice of snapped vertices (unit frame, 2^-17: exact in f32, ~1/700 of the finest cell).
// tunables: const (numerical: float lattice)
const QUANTUM: f32 = 1.0 / 131_072.0;
/// Quanta a coincident copy moves off its twin.
// tunables: const (numerical: separation of coincident vertices)
const SEPARATE_QUANTA: f32 = 4.0;
/// Pairings [`unpinch`] tries before giving a level up (the radial one, then the other pairing of the edges still
/// pinched; a pinch has two sheets, so two or three settle it).
// tunables: const (algorithm schedule of the mesher)
const UNPINCH_ATTEMPTS: u32 = 4;
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
// tunables: const (algorithm: LOD1+ are target-driven, the error bound is off)
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
    if solid.has_shared_faces() {
        fill_cavities(&mut field, &shape, dims, voxel, solid.noise_amp());
    }
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
    } else {
        separate_duplicates(&mut positions, &net.positions, origin, voxel);
    }

    // The chain is simplified on the snapped positions (flat faces collapse to a few triangles). Where meshopt
    // can't keep a snapped LOD0 closed (it can pinch two sheets along collinear edge rows), the chain is chosen
    // on the smooth net instead and its vertices still get the snapped positions (every LOD vertex is a net vertex).
    let (lod_indices, copies) = match lod_chain(&positions, &indices, s) {
        (chain, true, copies) => (chain, copies),
        (chain, false, copies) if !solid.snaps() => (chain, copies),
        (_, false, _) => {
            let (chain, _, copies) = lod_chain(&unsnapped, &indices, s);
            (chain, copies)
        }
    };
    // A vertex the chain copied (a pinch it split) keeps its parent's fallback normal.
    let mut normal_of: Vec<usize> = (0..positions.len()).collect();
    for c in &copies {
        // A later level can copy a copy: follow it back to the net vertex.
        normal_of.push(normal_of[c.parent as usize]);
    }
    place_copies(&mut positions, &copies);
    let step = solid.normal_step();
    let vertices: Vec<Vertex> = positions
        .iter()
        .zip(&normal_of)
        .map(|(&q, &from)| {
            let fallback = Vec3::from_array(net.normals[from]).normalize_or(Vec3::Z);
            let n = solid.gradient(q, step).normalize_or(fallback);
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

/// Two surface-nets vertices can land on the same point (both on the face between their cells, seen ~1 in 400
/// columnar rocks): welded by position ([`RockMesh::is_closed`]) they would join two sheets. Each later copy moves
/// a few [`QUANTUM`]s toward its own cell's centre; every other vertex is left bit-exact.
fn separate_duplicates(positions: &mut [Vec3], grid: &[[f32; 3]], origin: Vec3, voxel: f32) {
    let mut seen: std::collections::HashSet<[u32; 3]> = std::collections::HashSet::with_capacity(positions.len());
    for (q, g) in positions.iter_mut().zip(grid) {
        let centre = origin + (Vec3::from_array(*g).floor() + Vec3::splat(0.5)) * voxel;
        let away = (centre - *q).normalize_or(Vec3::X);
        let mut step = 1.0;
        while !seen.insert(q.to_array().map(f32::to_bits)) {
            *q += away * (SEPARATE_QUANTA * step * QUANTUM);
            step += 1.0;
        }
    }
}

/// Naive surface nets is non-manifold exactly where a grid face is ambiguous: its four corners alternate in sign
/// (four crossings, so the mesh edge between the two cells on that face gets four quads). Each such face gets its
/// corner nearest zero flipped to the other sign (the smallest change that removes the saddle, as marching cubes'
/// ambiguity resolution does), repeated until no face is ambiguous: every mesh edge then has exactly two faces.
/// The outer layer is never flipped (it stays outside, so the net stays closed), and no sample flips twice, so the
/// passes end. The first pass scans every face; later ones visit only the faces touching a flipped sample, in the
/// same order (a face whose corners didn't change can't change), so the result is that of full rescans.
fn resolve_ambiguous_faces(field: &mut [f32], shape: &RuntimeShape<u32, 3>, dims: [u32; 3], voxel: f32) {
    // A face: (axis, z, y, x) of its low corner, the scan order of a full pass.
    type Face = (usize, u32, u32, u32);
    let flip = FLIP_CELLS * voxel;
    let inner = |p: [u32; 3]| (0..3).all(|k| p[k] > 0 && p[k] + 1 < dims[k]);
    // A sample flips at most once (else two neighbouring faces can flip one corner back and forth).
    let mut flipped = std::collections::HashSet::new();
    // Checks one face, flipping a corner if it is ambiguous; returns the flipped sample.
    let mut check = |field: &mut [f32], (axis, z, y, x): Face| -> Option<[u32; 3]> {
        let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
        let p = [x, y, z];
        if p[u] + 1 >= dims[u] || p[v] + 1 >= dims[v] {
            return None;
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
            return None;
        }
        let k = (0..4)
            .filter(|&k| inner(corners[k]) && !flipped.contains(&at[k]))
            .min_by(|&a, &b| field[at[a]].abs().total_cmp(&field[at[b]].abs()))?;
        field[at[k]] = if neg[k] { flip } else { -flip };
        flipped.insert(at[k]);
        Some(corners[k])
    };
    // The faces that have sample `s` as a corner.
    let touching = |s: [u32; 3]| {
        let mut out = Vec::with_capacity(12);
        for axis in 0..3 {
            let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
            for du in 0..2u32 {
                for dv in 0..2u32 {
                    if s[u] < du || s[v] < dv {
                        continue;
                    }
                    let mut p = s;
                    p[u] -= du;
                    p[v] -= dv;
                    out.push((axis, p[2], p[1], p[0]));
                }
            }
        }
        out
    };

    let mut next: std::collections::BTreeSet<Face> = std::collections::BTreeSet::new();
    for axis in 0..3 {
        for z in 0..dims[2] {
            for y in 0..dims[1] {
                for x in 0..dims[0] {
                    let face = (axis, z, y, x);
                    if let Some(s) = check(field, face) {
                        // Faces after this one are checked later in this pass anyway; this one and earlier ones next pass.
                        next.extend(touching(s).into_iter().filter(|&f| f <= face));
                    }
                }
            }
        }
    }
    for _ in 1..AMBIGUITY_PASSES {
        if next.is_empty() {
            break;
        }
        let mut current = std::mem::take(&mut next);
        while let Some(face) = current.pop_first() {
            if let Some(s) = check(field, face) {
                for f in touching(s) {
                    if f > face {
                        current.insert(f);
                    } else {
                        next.insert(f);
                    }
                }
            }
        }
    }
}

/// Outside samples not connected to the outside are sealed cavities: a union of pieces that share a face has a
/// field near zero along that face deep inside the solid (`min` of two pieces each just inside), and the
/// weathering noise lifts some of those samples above zero, so the net would hold tiny inward-facing shells nobody
/// can see (that `meshopt` then pinches). Each sealed region is set inside ([`FLIP_CELLS`] past zero), the
/// standard cavity fill of voxel meshing. A sample above `noise_amp` is outside the unweathered union (a union of
/// convex pieces, no voids), so it is outside for certain: only the band of samples in `[0, noise_amp]` is
/// flooded, from those certain ones and the border. Connectivity is 26-neighbour, so only what is sealed under
/// any reading of the grid is filled.
fn fill_cavities(field: &mut [f32], shape: &RuntimeShape<u32, 3>, dims: [u32; 3], voxel: f32, noise_amp: f32) {
    let band = |d: f32| (0.0..=noise_amp).contains(&d);
    let border = |p: [u32; 3]| p.iter().zip(&dims).any(|(&c, &n)| c == 0 || c + 1 == n);
    let neighbours = |p: [u32; 3]| {
        let mut out = [[0u32; 3]; 26];
        let mut count = 0;
        for dz in -1i64..=1 {
            for dy in -1i64..=1 {
                for dx in -1i64..=1 {
                    let q = [i64::from(p[0]) + dx, i64::from(p[1]) + dy, i64::from(p[2]) + dz];
                    if (dx, dy, dz) == (0, 0, 0) || (0..3).any(|k| q[k] < 0 || q[k] >= i64::from(dims[k])) {
                        continue;
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // in 0..dims
                    let q = q.map(|c| c as u32);
                    out[count] = q;
                    count += 1;
                }
            }
        }
        (out, count)
    };
    let mut reached: Vec<bool> = field.iter().map(|&d| d > noise_amp).collect();
    let mut stack: Vec<[u32; 3]> = Vec::new();
    for z in 0..dims[2] {
        for y in 0..dims[1] {
            for x in 0..dims[0] {
                let p = [x, y, z];
                let i = shape.linearize(p) as usize;
                if !band(field[i]) {
                    continue;
                }
                let (near, count) = neighbours(p);
                if border(p) || near[..count].iter().any(|&q| reached[shape.linearize(q) as usize] && !band(field[shape.linearize(q) as usize])) {
                    reached[i] = true;
                    stack.push(p);
                }
            }
        }
    }
    while let Some(p) = stack.pop() {
        let (near, count) = neighbours(p);
        for &q in &near[..count] {
            let i = shape.linearize(q) as usize;
            if !reached[i] && band(field[i]) {
                reached[i] = true;
                stack.push(q);
            }
        }
    }
    let inside = -FLIP_CELLS * voxel;
    for (d, &r) in field.iter_mut().zip(&reached) {
        if band(*d) && !r {
            *d = inside;
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

/// A vertex the chain added: a copy of `parent` (same normal), moved a few [`QUANTUM`]s along `toward` (into its
/// own sheet) so no two vertices share a position.
#[derive(Clone, Copy, Debug)]
struct VertexCopy {
    parent: u32,
    toward: Vec3,
}

/// Appends `copies` to `positions` (each at its parent, moved off it until its position is unique).
fn place_copies(positions: &mut Vec<Vec3>, copies: &[VertexCopy]) {
    if copies.is_empty() {
        return;
    }
    let mut seen: std::collections::HashSet<[u32; 3]> = positions.iter().map(|q| q.to_array().map(f32::to_bits)).collect();
    for c in copies {
        let from = positions[c.parent as usize];
        let mut step = 1.0;
        let mut q = from + c.toward * (SEPARATE_QUANTA * QUANTUM);
        while !seen.insert(q.to_array().map(f32::to_bits)) {
            step += 1.0;
            q = from + c.toward * (SEPARATE_QUANTA * step * QUANTUM);
        }
        positions.push(q);
    }
}

/// LOD0 (error-bound simplification of the net) then levels at `lod_ratio` of the previous triangle count,
/// stopping once a level no longer reduces. `meshopt::simplify` doesn't test the link condition, so a collapse can
/// pinch two sheets into one edge (a handful of edges where two parts of the surface come within the error of
/// each other: a column's step, a joint's floor). A pinched level is repaired topologically ([`unpinch`]: each
/// pinched edge split into its sheets, each vertex copied per fan), which keeps its full reduction. A level that
/// still isn't closed is retried gentler ([`RETRY_SCHEDULE`]), and LOD0 falls back to the raw net, which is closed
/// by construction (the flag says whether LOD0 was simplified). Returns the chain and the vertices it added
/// (appended to `positions` in order, [`place_copies`]).
fn lod_chain(positions: &[Vec3], indices: &[u32], s: &MeshSettings) -> (Vec<Vec<u32>>, bool, Vec<VertexCopy>) {
    let mut work: Vec<Vec3> = positions.to_vec();
    let mut copies: Vec<VertexCopy> = Vec::new();
    let volume = |idx: &[u32], work: &[Vec3]| {
        RockMesh { positions: work.iter().map(Vec3::to_array).collect(), normals: Vec::new(), indices: idx.to_vec() }.signed_volume()
    };
    let source_volume = volume(indices, &work);
    // Closed, and still the solid: a level that lost half its volume (or turned inside out) is a collapse.
    let solid = |idx: &[u32], work: &[Vec3]| {
        idx.len() >= 3 * MIN_CLOSED_TRIANGLES && indices_closed(idx) && volume(idx, work) > MIN_VOLUME_FRACTION * source_volume
    };
    // One simplification, unpinched when needed; the copies it made join `work` and `copies` only if the level
    // is kept.
    let simplify = |source: &[u32], target_triangles: usize, error: f32, work: &[Vec3]| -> Option<(Vec<u32>, Vec<VertexCopy>)> {
        let flat: Vec<[f32; 3]> = work.iter().map(Vec3::to_array).collect();
        let adapter = VertexDataAdapter::new(meshopt::typed_to_bytes(&flat), size_of::<[f32; 3]>(), 0).ok()?;
        let mut idx = meshopt::simplify(source, &adapter, target_triangles * 3, error, SimplifyOptions::ErrorAbsolute, None);
        let mut added = Vec::new();
        if !indices_closed(&idx) {
            added = unpinch(&mut idx, work)?;
        }
        let mut grown = work.to_vec();
        place_copies(&mut grown, &added);
        solid(&idx, &grown).then_some((idx, added))
    };

    let simplified = RETRY_SCHEDULE.iter().find_map(|gentler| simplify(indices, 0, s.lod0_error * (1.0 - gentler), &work));
    let lod0_simplified = simplified.is_some();
    let lod0 = match simplified {
        Some((idx, added)) => {
            place_copies(&mut work, &added);
            copies.extend(added);
            idx
        }
        None => indices.to_vec(),
    };
    let mut chain = vec![lod0];
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
            simplify(source, target, UNBOUNDED_ERROR, &work).filter(|(idx, _)| idx.len() / 3 < prev)
        });
        let Some((next, added)) = next else { break };
        place_copies(&mut work, &added);
        copies.extend(added);
        chain.push(next);
    }
    (chain, lod0_simplified, copies)
}

fn find_root(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Pairs the triangles around edge a→b (`fwd`) with those around b→a (`back`): each with its radial neighbour on
/// its inner side (the sheets that really join there). Coincident sheets (no clean radial order) get any
/// one-to-one pairing, which is valid topology too. Pairs are (forward, backward) triangles.
#[allow(clippy::many_single_char_names)] // maths notation (points, edges, triangles)
fn pair_radially(indices: &[u32], positions: &[Vec3], (a, b): (u32, u32), fwd: &[usize], back: &[usize]) -> Vec<(usize, usize)> {
    let p = |i: u32| positions[i as usize];
    let opposite = |t: usize| (0..3).map(|k| indices[3 * t + k]).find(|&v| v != a && v != b).unwrap_or(a);
    let axis = (p(b) - p(a)).normalize_or(Vec3::X);
    let u = axis.any_orthonormal_vector();
    let v = axis.cross(u);
    let mut around: Vec<(f32, usize, bool)> = fwd
        .iter()
        .map(|&t| (t, true))
        .chain(back.iter().map(|&t| (t, false)))
        .map(|(t, forward)| {
            let w = p(opposite(t)) - p(a);
            (w.dot(v).atan2(w.dot(u)), t, forward)
        })
        .collect();
    around.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
    let n = around.len();
    // Turning toward increasing angle moves along axis × w: that side is inner when it is against the normal.
    let inner = |i: usize| {
        let t = around[i].1;
        let [x, y, z] = [indices[3 * t], indices[3 * t + 1], indices[3 * t + 2]].map(p);
        let w = p(opposite(t)) - p(a);
        if (y - x).cross(z - x).dot(axis.cross(w)) < 0.0 { (i + 1) % n } else { (i + n - 1) % n }
    };
    let mut pairs = Vec::new();
    for i in 0..n {
        let j = inner(i);
        if around[i].2 && !around[j].2 && inner(j) == i {
            pairs.push((around[i].1, around[j].1));
        }
    }
    if pairs.len() == fwd.len() { pairs } else { fwd.iter().copied().zip(back.iter().copied()).collect() }
}

/// Repairs a pinched (edge-non-manifold) index list in place, the standard non-manifold split (CGAL PMP
/// `duplicate_non_manifold_vertices`, after pairing the faces of each non-manifold edge, [`pair_radially`]): each
/// vertex whose faces then fall into several fans gets one copy per extra fan. Where the pinched sheets coincide
/// the radial order can't tell them apart and the crossed pairing joins both sheets into one fan: the edges still
/// pinched take their other pairing, up to [`UNPINCH_ATTEMPTS`] times. Returns the copies (new indices from
/// `positions.len()` on), or `None` when an edge is open, a triangle degenerate, or no pairing closes it.
fn unpinch(indices: &mut [u32], positions: &[Vec3]) -> Option<Vec<VertexCopy>> {
    let original = indices.to_vec();
    let first_new = u32::try_from(positions.len()).ok()?;
    let mut flip: std::collections::HashSet<(u32, u32)> = std::collections::HashSet::new();
    for _ in 0..UNPINCH_ATTEMPTS {
        let copies = split_fans(indices, positions, &flip)?;
        let mut directed: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::with_capacity(indices.len());
        for tri in indices.as_chunks::<3>().0 {
            for edge in [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                *directed.entry(edge).or_insert(0) += 1;
            }
        }
        let source = |v: u32| if v >= first_new { copies[(v - first_new) as usize].parent } else { v };
        let mut still: Vec<(u32, u32)> = directed
            .iter()
            .filter(|&(&(a, b), &n)| n != 1 || directed.get(&(b, a)) != Some(&1))
            .map(|(&(a, b), _)| {
                let (x, y) = (source(a), source(b));
                (x.min(y), x.max(y))
            })
            .collect();
        if still.is_empty() {
            return Some(copies);
        }
        still.sort_unstable();
        still.dedup();
        for edge in still {
            if !flip.remove(&edge) {
                flip.insert(edge);
            }
        }
        indices.copy_from_slice(&original);
    }
    None
}

/// One pairing pass of [`unpinch`]: the edges in `flip` (vertex pair, low first) take their next pairing.
#[allow(clippy::many_single_char_names)] // maths notation (edges, triangles)
fn split_fans(indices: &mut [u32], positions: &[Vec3], flip: &std::collections::HashSet<(u32, u32)>) -> Option<Vec<VertexCopy>> {
    let tris = indices.len() / 3;
    let mut by_edge: std::collections::HashMap<(u32, u32), Vec<usize>> = std::collections::HashMap::with_capacity(indices.len());
    for t in 0..tris {
        let tri = [indices[3 * t], indices[3 * t + 1], indices[3 * t + 2]];
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
            return None;
        }
        for k in 0..3 {
            by_edge.entry((tri[k], tri[(k + 1) % 3])).or_default().push(t);
        }
    }
    // partner[(a, b, t)]: the triangle across t's edge a→b (it uses b→a).
    let mut partner: std::collections::HashMap<(u32, u32, usize), usize> = std::collections::HashMap::with_capacity(indices.len());
    for (&(a, b), fwd) in &by_edge {
        let back = by_edge.get(&(b, a))?;
        if back.len() != fwd.len() {
            return None;
        }
        if a > b {
            continue;
        }
        let mut pairs = if fwd.len() == 1 { vec![(fwd[0], back[0])] } else { pair_radially(indices, positions, (a, b), fwd, back) };
        if flip.contains(&(a, b)) && pairs.len() > 1 {
            let backs: Vec<usize> = pairs.iter().map(|p| p.1).collect();
            for (k, pair) in pairs.iter_mut().enumerate() {
                pair.1 = backs[(k + 1) % backs.len()];
            }
        }
        for (f, g) in pairs {
            partner.insert((a, b, f), g);
            partner.insert((b, a, g), f);
        }
    }

    // Fans: the corners at one vertex joined across paired edges (union-find over corners).
    let mut parent: Vec<usize> = (0..indices.len()).collect();
    let corner_of = |t: usize, vertex: u32| (0..3).find(|&k| indices[3 * t + k] == vertex).map(|k| 3 * t + k);
    for t in 0..tris {
        for k in 0..3 {
            let (a, b) = (indices[3 * t + k], indices[3 * t + (k + 1) % 3]);
            let g = *partner.get(&(a, b, t))?;
            for (vertex, mine) in [(a, 3 * t + k), (b, 3 * t + (k + 1) % 3)] {
                let theirs = corner_of(g, vertex)?;
                let (x, y) = (find_root(&mut parent, mine), find_root(&mut parent, theirs));
                parent[x.max(y)] = x.min(y);
            }
        }
    }
    let p = |i: u32| positions[i as usize];
    let next_index = u32::try_from(positions.len()).ok()?;
    let mut first_fan: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    let mut fan_copy: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut copies: Vec<VertexCopy> = Vec::new();
    let original = indices.to_vec();
    for (c, slot) in indices.iter_mut().enumerate() {
        let vertex = *slot;
        let root = find_root(&mut parent, c);
        if *first_fan.entry(vertex).or_insert(root) == root {
            continue;
        }
        let k = *fan_copy.entry(root).or_insert_with(|| {
            copies.push(VertexCopy { parent: vertex, toward: Vec3::ZERO });
            copies.len() - 1
        });
        // Toward the fan's own triangles (summed centroid offsets; normalised below).
        let t = c / 3;
        let centroid = (p(original[3 * t]) + p(original[3 * t + 1]) + p(original[3 * t + 2])) / 3.0;
        copies[k].toward += centroid - p(vertex);
        *slot = next_index + u32::try_from(k).ok()?;
    }
    for copy in &mut copies {
        copy.toward = copy.toward.normalize_or(Vec3::X);
    }
    Some(copies)
}

#[cfg(test)]
mod unpinch_tests {
    use super::*;

    /// Two unit cubes touching along one edge, sharing its two vertices: the edge has four triangles.
    fn cubes_sharing_an_edge() -> (Vec<Vec3>, Vec<u32>) {
        // Cube corners by bit (x, y, z); quads counter-clockwise seen from outside.
        let quads = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
        let bit = |k: u32, b: u32| if (k >> b) & 1 == 1 { 1.0 } else { 0.0 };
        let corner = |offset: Vec3, k: u32| offset + Vec3::new(bit(k, 0), bit(k, 1), bit(k, 2));
        let mut positions: Vec<Vec3> = (0..8).map(|k| corner(Vec3::ZERO, k)).collect();
        // Cube B at (1, 1, 0): its corners 0 and 4 (x = y = 1 of A's corners 3 and 7) are shared.
        let b: Vec<u32> = (0..8)
            .map(|k| match k {
                0 => 3,
                4 => 7,
                _ => {
                    positions.push(corner(Vec3::new(1.0, 1.0, 0.0), k));
                    u32::try_from(positions.len() - 1).unwrap()
                }
            })
            .collect();
        let mut indices = Vec::new();
        for map in [&(0..8).collect::<Vec<u32>>(), &b] {
            for q in quads {
                let [a, b2, c, d] = q.map(|k| map[k]);
                indices.extend([a, b2, c, a, c, d]);
            }
        }
        (positions, indices)
    }

    #[test]
    fn unpinch_splits_an_edge_shared_by_two_solids() {
        let (mut positions, mut indices) = cubes_sharing_an_edge();
        assert!(!indices_closed(&indices), "the fixture must be pinched");
        let copies = unpinch(&mut indices, &positions).expect("a pinch is repairable");
        assert_eq!(copies.len(), 2, "each end of the shared edge gets one copy");
        assert!(indices_closed(&indices));
        place_copies(&mut positions, &copies);
        let mesh = RockMesh { positions: positions.iter().map(Vec3::to_array).collect(), normals: Vec::new(), indices };
        assert!(mesh.is_closed(), "the copies don't weld back onto their parents");
        assert!((mesh.signed_volume() - 2.0).abs() < 1e-3);
    }

    #[test]
    fn unpinch_refuses_an_open_mesh() {
        let (positions, mut indices) = cubes_sharing_an_edge();
        indices.truncate(indices.len() - 3);
        assert!(unpinch(&mut indices, &positions).is_none());
    }
}
