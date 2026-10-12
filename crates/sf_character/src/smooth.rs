//! Smooth bodies, Bevy-free (spec `docs/specs/smooth-bodies.md`): turns a
//! segmented body (rigid limb pieces on single joints, as many legacy games
//! ship) into one smoothly skinned surface, and transfers its weights to
//! rigid clothing and armor so both bend together.
//!
//! Two steps, both "weight inpainting" (Abdrashitov et al. 2023):
//! - [`smooth_body`]: weld coincident boundary vertices between parts (they
//!   then share position, normal and weights, so seams can't open), then
//!   re-solve weights in a band around every seam as a harmonic function
//!   with the weights outside the band as fixed boundary values: rigid in
//!   mid-segment, blended across the joint. A body without seams (already
//!   skinned replacer bodies) is left unchanged. Overlapping pieces (one rim
//!   tucked inside the next piece) are linked by an inside test; seam
//!   vertices follow the other side's surface weights (child side follows
//!   parent, rigid follows authored); authored skins are fixed boundary
//!   values; vertices hidden inside a neighbor copy its weights. Output is
//!   bit-identical across runs and threads.
//! - [`transfer_weights`]: each target vertex copies the barycentric weights
//!   of the closest body point when it is close and its normal agrees;
//!   the rest are inpainted the same way over the target's own surface.
//!
//! Everything works in **rest space**: positions are the skeleton's rest
//! pose in model space and joints index the skeleton, so the inverse bind of
//! joint `j` is `rest_model[j]⁻¹` ([`bind_palette`]). Game specifics (which
//! parts, which joints, units) stay in the adapter; distances in the
//! settings are in model units.

use std::collections::HashMap;

use glam::{Affine3A, Mat4, Vec3};
use parry3d::shape::{CompositeShapeRef, TriMesh};
use sf_model::{CookedJoint, CookedSkinned, FormatError, MAX_INFLUENCES, SkinVertex};

use crate::records::BodyPart;

/// One part's surface in rest space.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinSurface {
    /// Rest-pose positions in skeleton model space.
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
    /// One per vertex; joints index the skeleton.
    pub skin: Vec<SkinVertex>,
}

impl SkinSurface {
    /// The part's surface at rest: every LOD-`level` submesh kept by
    /// `body_part`, transformed by its joints' rest × inverse bind, with
    /// joints remapped to skeleton indices through `joint_index`. A joint
    /// the skeleton lacks is an error (the adapter binds names first).
    pub fn from_cooked(
        cooked: &CookedSkinned,
        level: usize,
        body_part: Option<&BodyPart>,
        joint_index: impl Fn(&str) -> Option<u16>,
        rest_model: &[Affine3A],
    ) -> Result<Self, FormatError> {
        let mut out = Self::default();
        let Some(level) = cooked.model.lods.len().checked_sub(1).map(|last| level.min(last)) else { return Ok(out) };
        let mut binds = Vec::with_capacity(cooked.joints.len());
        for joint in &cooked.joints {
            let j = joint_index(&joint.name).ok_or_else(|| FormatError::Body(format!("joint {} not in skeleton", joint.name)))?;
            let ib = Affine3A::from_mat4(Mat4::from_cols_array(&joint.inverse_bind));
            binds.push((j, rest_model[usize::from(j)] * ib));
        }
        for i in 0..cooked.model.lods[level].submeshes.len() {
            let sub = cooked.decode_submesh(level, i)?;
            let path = cooked.parts.get(sub.part as usize).map_or(&[][..], |p| p.node_path.as_slice());
            if body_part.is_some_and(|b| !b.keeps_node(path.iter().map(String::as_str))) {
                continue;
            }
            let base = to_u32(out.positions.len());
            for (v, s) in sub.vertices.iter().zip(&sub.skin) {
                let (p, n) = (Vec3::from(v.position), Vec3::from(v.normal));
                let (mut rp, mut rn) = (Vec3::ZERO, Vec3::ZERO);
                let mut skin = SkinVertex { joints: [0; MAX_INFLUENCES], weights: [0.0; MAX_INFLUENCES] };
                for k in 0..MAX_INFLUENCES {
                    let (j, m) = binds[usize::from(s.joints[k])];
                    rp += m.transform_point3(p) * s.weights[k];
                    rn += m.transform_vector3(n) * s.weights[k];
                    skin.joints[k] = j;
                    skin.weights[k] = s.weights[k];
                }
                out.positions.push(rp);
                out.normals.push(rn.normalize_or_zero());
                out.skin.push(skin);
            }
            out.triangles.extend(sub.indices.as_chunks::<3>().0.iter().map(|t| t.map(|i| i + base)));
        }
        Ok(out)
    }

    /// Linear blend skinning with `skinning[j] = posed_model[j] · rest_model[j]⁻¹`
    /// ([`skinning_matrices`]).
    pub fn posed(&self, skinning: &[Affine3A]) -> Vec<Vec3> {
        self.positions
            .iter()
            .zip(&self.skin)
            .map(|(&p, s)| skin_point(p, s, skinning))
            .collect()
    }
}

/// The palette of a rest-space skin: joint names with `rest_model[j]⁻¹` as
/// inverse binds (the `sf_model` layout `assemble` and the loaders read).
pub fn bind_palette(names: &[String], rest_model: &[Affine3A]) -> Vec<CookedJoint> {
    names
        .iter()
        .zip(rest_model)
        .map(|(name, m)| CookedJoint { name: name.clone(), inverse_bind: Mat4::from(m.inverse()).to_cols_array() })
        .collect()
}

/// `posed[j] · rest[j]⁻¹` per joint.
pub fn skinning_matrices(rest_model: &[Affine3A], posed_model: &[Affine3A]) -> Vec<Affine3A> {
    rest_model.iter().zip(posed_model).map(|(r, p)| *p * r.inverse()).collect()
}

/// Smoothing parameters, in model units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmoothSettings {
    /// Boundary vertices of different parts closer than this are welded
    /// (one position, normal and weight set).
    pub weld: f32,
    /// A boundary vertex is linked to the nearest vertex of each other part
    /// within this distance (adjacent pieces of legacy bodies overlap or
    /// leave gaps of a few millimeters), so weights diffuse across the seam.
    pub link: f32,
    /// Overlapping pieces: a boundary vertex *inside* another part (behind
    /// its surface, along the inward normal) is linked to that surface up to
    /// this depth. Legacy pieces often tuck one rim deep inside the next
    /// piece (a sleeve over a limb), farther than `link` from any surface.
    pub overlap: f32,
    /// Links only join parts whose dominant joints are at most this many
    /// skeleton edges apart, one of them at most one below their common
    /// ancestor (a hand hanging next to a thigh, or thigh to thigh, is no seam).
    pub max_hops: usize,
    /// Geodesic half-width of the re-solved band around each seam.
    pub band: f32,
    /// Keep the weights of authored skins (parts that are not rigid on a
    /// single joint) as fixed boundary values: only rigid parts are
    /// re-solved. Authored weights are trustworthy; re-solving them can open
    /// seams the artist had closed.
    pub keep_authored: bool,
    /// Gauss-Seidel sweeps, at most.
    pub iterations: usize,
    /// Stop when no weight changes by more than this in a sweep.
    pub tolerance: f32,
}

impl Default for SmoothSettings {
    /// Meters, for a human-sized skeleton.
    fn default() -> Self {
        Self { weld: 0.002, link: 0.04, overlap: 0.08, max_hops: 3, band: 0.08, keep_authored: true, iterations: 4000, tolerance: 1e-5 }
    }
}

/// A pair of vertices of different parts that meet at a seam.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamLink {
    /// `(part, vertex)`: a boundary vertex.
    pub a: (u32, u32),
    /// `(part, vertex)`: the nearest corner of the closest triangle of the other part.
    pub b: (u32, u32),
    /// Overlapping pieces (`a` inside `b`'s part, or `a`'s rim a shell over
    /// it), not a gap seam.
    pub inside: bool,
}

/// What [`smooth_body`] did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SmoothReport {
    /// Cross-part vertex pairs at seams (welded or linked), for metrics.
    pub seams: Vec<SeamLink>,
    /// Vertices snapped onto a seam partner.
    pub welded: usize,
    /// Linked (gap) pairs that were not welded.
    pub linked: usize,
    /// Vertices whose weights were re-solved.
    pub band: usize,
    pub iterations: usize,
    /// Largest weight change in the last sweep.
    pub residual: f32,
}

/// Welds the seams between `parts` and blends weights across them (module
/// docs). Positions of welded vertices move by at most `settings.weld`.
/// `parents` is the skeleton's parent table (joint indices as in the skins).
// One pipeline (links, weld, band, solve, overlap copy) over a dozen shared
// locals; the overlap pass is already split out.
#[allow(clippy::too_many_lines)]
pub fn smooth_body(parts: &mut [SkinSurface], parents: &[Option<usize>], settings: &SmoothSettings) -> SmoothReport {
    let mut report = SmoothReport::default();
    let mut flat = Flat::new(parts);
    let mut uf = UnionFind::new(flat.len());
    // Exact duplicates inside a part (UV and normal splits) share topology.
    let tiny = settings.weld * 1e-3;
    let grid = PointGrid::new(&flat.positions, settings.link.max(tiny));
    for i in 0..flat.len() {
        grid.near(&flat.positions, flat.positions[i], tiny, |k| {
            if flat.part[k] == flat.part[i] {
                uf.union(i, k);
            }
        });
    }
    let boundary = flat.boundary(parts, &mut uf);
    // Cross-part seams: each boundary vertex to the closest point of every
    // other skeleton-adjacent part (surface, not vertices: low-poly pieces
    // overlap by more than their vertex spacing), linked to that triangle's
    // nearest corner.
    let dominant: Vec<usize> = (0..flat.len())
        .map(|i| {
            let (p, v) = flat.local(i);
            dominant_joint(&parts[p as usize].skin[v as usize])
        })
        .collect();
    let meshes: Vec<Option<SurfaceMesh>> = parts.iter().map(|s| SurfaceMesh::from_raw(&s.positions, &s.triangles)).collect();
    let authored: Vec<bool> = parts.iter().map(|s| settings.keep_authored && !is_rigid(s)).collect();
    let depth: Vec<usize> = (0..parents.len().max(dominant.iter().max().map_or(0, |m| m + 1)))
        .map(|mut j| {
            let mut d = 0;
            while let Some(p) = parents.get(j).copied().flatten() {
                d += 1;
                j = p;
            }
            d
        })
        .collect();
    let mut links: Vec<Link> = Vec::new();
    let reach = settings.link.max(settings.overlap);
    for &i in &boundary {
        let p = flat.positions[i];
        // Only the nearest skeleton-adjacent part: a rim meets one
        // neighbor (an ankle's top meets the knee piece, not the thigh or a
        // skirt hem that happens to be in reach).
        let mut best: Option<Link> = None;
        let mut nearest = (f32::INFINITY, true);
        for (b, mesh) in meshes.iter().enumerate() {
            let Some(mesh) = mesh.as_ref().filter(|_| b != flat.part[i] as usize) else { continue };
            let Some((tri, q)) = mesh.closest(p, reach) else { continue };
            let d = q.distance(p);
            let open = on_open_edge(mesh, tri, q, |c| flat.is_boundary[flat.offsets[b] + c as usize]);
            let Some(contact) = overlap_kind(mesh, tri, q, p, flat.normal(parts, i), open, settings) else { continue };
            let inside = contact != Contact::Gap;
            let bary = mesh.barycentric(tri, q).map(|x| x.max(0.0));
            let corners = mesh.triangles[tri].map(|c| flat.offsets[b] + c as usize);
            let corner = (0..3).max_by(|&x, &y| bary[x].total_cmp(&bary[y])).unwrap_or(0);
            let k = corners[corner];
            let adjacent = hops(parents, dominant[i], dominant[k]) <= settings.max_hops;
            if d < nearest.0 {
                nearest = (d, adjacent);
            }
            // A rim hidden inside (or covering) a neighbor belongs with that
            // neighbor, even when a gap seam to another part is nearer (a
            // wrist rim inside a sleeve, a few centimeters from the hand).
            if !adjacent || best.as_ref().is_some_and(|l| (l.surface <= settings.weld, l.contact != Contact::Gap, -l.surface) >= (d <= settings.weld, inside, -d)) {
                continue;
            }
            let total = bary.iter().sum::<f32>().max(f32::EPSILON);
            best = Some(Link { a: i, b: k, distance: flat.positions[k].distance(p), surface: d, contact, corners, bary: bary.map(|x| x / total) });
        }
        // An authored rim (a tail root, a hand) whose nearest part is not
        // skeleton-adjacent passes through it (a tail through trousers, next
        // to a leg it is not attached to): no seam there.
        links.extend(best.filter(|l| !authored[flat.part[i] as usize] || nearest.1 || l.surface <= nearest.0 + settings.weld));
    }
    drop(grid);
    if links.is_empty() {
        return report;
    }
    for l in &links {
        report.seams.push(SeamLink { a: flat.local(l.a), b: flat.local(l.b), inside: l.contact != Contact::Gap });
        if l.distance <= settings.weld && flat.is_boundary[l.b] && l.contact == Contact::Gap {
            uf.union(l.a, l.b);
        }
    }
    let graph = Graph::new(&flat, parts, &mut uf, links.iter().map(|l| (l.a, l.b)));
    // Snap welded clusters (members from 2+ parts) to one position and normal.
    for members in &graph.members {
        if members.iter().any(|&m| flat.part[m] != flat.part[members[0]]) {
            let n = count(members.len());
            let p = members.iter().map(|&m| flat.positions[m]).sum::<Vec3>() / n;
            let nrm = members.iter().map(|&m| flat.normal(parts, m)).sum::<Vec3>().normalize_or_zero();
            for &m in members {
                let (part, v) = flat.local(m);
                let s = &mut parts[part as usize];
                report.welded += usize::from(s.positions[v as usize] != p);
                s.positions[v as usize] = p;
                if nrm != Vec3::ZERO {
                    s.normals[v as usize] = nrm;
                }
            }
        }
    }
    report.linked = links.iter().filter(|l| graph.node[l.a] != graph.node[l.b]).count();
    // Band: nodes within `band` (geodesic) of a seam node.
    let mut linked = vec![false; graph.len()];
    for l in &links {
        linked[graph.node[l.a]] = true;
        linked[graph.node[l.b]] = true;
    }
    let seeds: Vec<usize> = (0..graph.len())
        .filter(|&n| {
            let m = &graph.members[n];
            linked[n] || m.iter().any(|&x| flat.part[x] != flat.part[m[0]])
        })
        .collect();
    let dist = graph.geodesic(&seeds, settings.band);
    // Authored skins are boundary values (a node with any authored member
    // takes the authored members' weights).
    let pinned: Vec<bool> = graph.members.iter().map(|m| m.iter().any(|&x| authored[flat.part[x] as usize])).collect();
    let unknown: Vec<bool> = dist.iter().zip(&pinned).map(|(d, p)| *d < settings.band && !p).collect();
    let mut weights = graph.initial_weights(&flat, parts, |x| !pinned[graph.node[x]] || authored[flat.part[x] as usize]);
    // A seam's boundary vertex follows the other side's surface: its weights
    // are the barycentric blend at its closest point there, so both sides of
    // a seam (or an inner rim and the shell around it) carry the same
    // weights and move together. Next to an authored skin the rigid side
    // follows the authored vertex.
    let mut follow: Vec<Vec<(usize, f32)>> = vec![Vec::new(); graph.len()];
    for l in &links {
        let (na, nb) = (graph.node[l.a], graph.node[l.b]);
        // At a seam between two free pieces the child side follows the parent
        // side (deeper dominant joint). Between pieces on equally deep joints
        // (sibling pieces of one bone) the hidden side follows the visible
        // one: a rim inside a neighbor follows its surface, a shell rim leads
        // the surface it covers; gap ties go by part order. So two rims never
        // follow each other.
        let (da, db) = (depth[dominant[l.a]], depth[dominant[l.b]]);
        let child = match da.cmp(&db) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => match l.contact {
                Contact::Inside => true,
                Contact::Shell => false,
                Contact::Gap => flat.part[l.a] > flat.part[l.b],
            },
        };
        if unknown[na] && (child || !unknown[nb]) {
            for (c, b) in l.corners.iter().zip(l.bary) {
                if graph.node[*c] != na && b > 0.0 {
                    follow[na].push((graph.node[*c], b));
                }
            }
        } else if na != nb && unknown[nb] && (pinned[na] || !child) {
            // The parent side (or an authored skin) leads: the child-side
            // corner follows the boundary vertex.
            follow[nb].push((na, 1.0));
        }
    }
    // Followers follow fixed or free nodes, never other followers (two rims
    // following each other would be a cycle the solver crawls through): a
    // follower mostly on followers falls back to the plain harmonic mean.
    let is_follower: Vec<bool> = follow.iter().map(|f| !f.is_empty()).collect();
    for f in &mut follow {
        let on_followers: f32 = f.iter().filter(|x| is_follower[x.0]).map(|x| x.1).sum();
        let total: f32 = f.iter().map(|x| x.1).sum();
        if on_followers > 0.5 * total {
            f.clear();
        }
        f.retain(|x| !is_follower[x.0]);
        let total: f32 = f.iter().map(|x| x.1).sum();
        for x in f.iter_mut() {
            x.1 /= total;
        }
    }
    let (iterations, residual) = graph.inpaint(&mut weights, &unknown, &follow, settings.iterations, settings.tolerance);
    report.iterations = iterations;
    report.residual = residual;
    // Overlap zones: a rigid part's vertices inside a linked neighbor take
    // the neighbor's weights at their closest point there, so the hidden
    // inner surface moves with the shell around it and can't poke out.
    let copied = overlap_copies(&flat, &graph, &meshes, &links, &pinned, &follow, &weights, settings);
    let mut touched = vec![false; graph.len()];
    for (n, c) in copied.into_iter().enumerate() {
        if let Some(w) = c {
            weights[n] = w;
            touched[n] = true;
        }
    }
    // The copy moved some followed nodes: re-apply the seam constraints
    // (followers read only non-followers, so one pass settles them).
    for (n, f) in follow.iter().enumerate() {
        if !f.is_empty() && !touched[n] {
            let mut w = Vec::new();
            for &(m, x) in f {
                for &(j, y) in &weights[m] {
                    add(&mut w, j, x * y);
                }
            }
            weights[n] = w;
        }
    }
    for (n, members) in graph.members.iter().enumerate() {
        if unknown[n] || touched[n] {
            let skin = top_influences(&weights[n]);
            for &m in members {
                let (part, v) = flat.local(m);
                parts[part as usize].skin[v as usize] = skin;
                report.band += 1;
            }
        }
    }
    report
}

/// Transfer parameters (model units, degrees).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransferSettings {
    /// A target vertex farther than this from the body is inpainted.
    pub max_distance: f32,
    /// A target vertex whose normal differs from the body's by more than
    /// this (either facing; double-sided cloth) is inpainted.
    pub max_angle: f32,
    /// Weld distance for the target's own seams between pieces.
    pub weld: f32,
    pub iterations: usize,
    pub tolerance: f32,
}

impl Default for TransferSettings {
    fn default() -> Self {
        Self { max_distance: 0.10, max_angle: 35.0, weld: 0.002, iterations: 4000, tolerance: 1e-5 }
    }
}

/// What [`transfer_weights`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TransferReport {
    /// Target vertices that copied body weights.
    pub matched: usize,
    /// Target vertices solved by inpainting.
    pub inpainted: usize,
    /// Target vertices in pieces with no match at all (weights kept).
    pub kept: usize,
    pub iterations: usize,
}

/// Copies `body` weights onto `targets` (clothing, armor), inpainting what
/// doesn't match (module docs). Pieces of `targets` that touch share weights
/// at their welded seams.
pub fn transfer_weights(body: &[SkinSurface], targets: &mut [SkinSurface], settings: &TransferSettings) -> TransferReport {
    let mut report = TransferReport::default();
    let Some(body_mesh) = SurfaceMesh::new(body) else { return report };
    let flat = Flat::new(targets);
    let mut uf = UnionFind::new(flat.len());
    let grid = PointGrid::new(&flat.positions, settings.weld.max(1e-6));
    for i in 0..flat.len() {
        grid.near(&flat.positions, flat.positions[i], settings.weld, |k| uf.union(i, k));
    }
    let graph = Graph::new(&flat, targets, &mut uf, std::iter::empty());
    let cos = settings.max_angle.to_radians().cos();
    let mut weights = graph.initial_weights(&flat, targets, |_| true);
    let mut unknown = vec![true; graph.len()];
    for (n, members) in graph.members.iter().enumerate() {
        let p = graph.positions[n];
        let normal = members.iter().map(|&m| flat.normal(targets, m)).sum::<Vec3>().normalize_or_zero();
        let Some((tri, q)) = body_mesh.closest(p, settings.max_distance) else { continue };
        if body_mesh.face_normal(tri).dot(normal).abs() < cos {
            continue;
        }
        weights[n] = body_mesh.weights_at(tri, q);
        unknown[n] = false;
    }
    // Pieces (graph components) without a single match keep their weights.
    let component = graph.components();
    let mut has_known = vec![false; graph.len()];
    for n in 0..graph.len() {
        has_known[component[n]] |= !unknown[n];
    }
    for n in 0..graph.len() {
        if !has_known[component[n]] {
            unknown[n] = false;
            report.kept += graph.members[n].len();
        } else if unknown[n] {
            report.inpainted += graph.members[n].len();
        } else {
            report.matched += graph.members[n].len();
        }
    }
    let (iterations, _) = graph.inpaint(&mut weights, &unknown, &[], settings.iterations, settings.tolerance);
    report.iterations = iterations;
    for (n, members) in graph.members.iter().enumerate() {
        if has_known[component[n]] {
            let skin = top_influences(&weights[n]);
            for &m in members {
                let (part, v) = flat.local(m);
                targets[part as usize].skin[v as usize] = skin;
            }
        }
    }
    report
}

/// Depth by which each point lies *outside* `surface` (along the closest
/// face's normal) where it was inside at rest, measured only where the
/// closest point is inside a face (not on its border, so points beyond the
/// surface's edge don't count). `rest_points`/`rest_surface` and
/// `points`/`posed_surface` are the same vertices at rest and posed; `0` for
/// points with nothing within `max_distance`.
pub fn penetration(
    rest_points: &[Vec3],
    points: &[Vec3],
    rest_surface: (&[Vec3], &[[u32; 3]]),
    posed_surface: (&[Vec3], &[[u32; 3]]),
    max_distance: f32,
) -> Vec<f32> {
    let (Some(rest), Some(posed)) = (SurfaceMesh::from_raw(rest_surface.0, rest_surface.1), SurfaceMesh::from_raw(posed_surface.0, posed_surface.1)) else {
        return vec![0.0; points.len()];
    };
    rest_points
        .iter()
        .zip(points)
        .map(|(&r, &p)| {
            let inside_at_rest = rest.closest(r, max_distance).is_some_and(|(t, q)| rest.face_normal(t).dot(r - q) <= 0.0);
            if !inside_at_rest {
                return 0.0;
            }
            match posed.closest(p, max_distance) {
                Some((t, q)) if posed.interior(t, q) => posed.face_normal(t).dot(p - q).max(0.0),
                _ => 0.0,
            }
        })
        .collect()
}

/// Cross-part seams of `parts` (boundary vertices of different parts within
/// `settings.link`, skeleton-adjacent), without changing anything: the seam
/// set a metric measures.
pub fn find_seams(parts: &[SkinSurface], parents: &[Option<usize>], settings: &SmoothSettings) -> Vec<SeamLink> {
    let mut copy = parts.to_vec();
    smooth_body(&mut copy, parents, &SmoothSettings { weld: 0.0, band: 0.0, iterations: 0, ..*settings }).seams
}

/// How far each seam opens in one pose: the distance from `a` to its closest
/// point on `b`'s part (found at rest, carried by that triangle's corners),
/// minus the rest distance, clamped at 0. One value per seam link.
pub fn seam_openings(parts: &[SkinSurface], seams: &[SeamLink], skinning: &[Affine3A]) -> Vec<f32> {
    let probes = SeamProbe::find(parts, seams);
    probes.iter().map(|p| p.opening(parts, skinning)).collect()
}

/// What [`guard_seams`] did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuardReport {
    /// Seams (part pairs) checked.
    pub seams: usize,
    /// Seams that opened wider than with the original weights in some pose,
    /// as part pairs; their vertices got the original weights back.
    pub reverted: Vec<(u32, u32)>,
    /// Vertices whose original weights were restored.
    pub vertices: usize,
}

/// The "never worse than the original" guard: per seam (part pair), if the
/// smoothed seam opens wider than the original (`original`, as authored) in
/// any of `poses` (skinning matrices, [`skinning_matrices`]) by more than
/// `tolerance` (model units), the smoothed vertices nearest that seam get
/// their original weights back; if that is not enough, the vertices of the
/// links that open too wide, then every changed vertex
/// of both parts within `radius` of the seam does, and at last both whole
/// parts (positions too). Repeats until no seam is worse. `seams` are the
/// [`SmoothReport::seams`] of the run that turned `original` into `smoothed`.
pub fn guard_seams(original: &[SkinSurface], smoothed: &mut [SkinSurface], seams: &[SeamLink], poses: &[Vec<Affine3A>], tolerance: f32, radius: f32) -> GuardReport {
    let pair_of = |l: &SeamLink| (l.a.0.min(l.b.0), l.a.0.max(l.b.0));
    let mut pairs: Vec<(u32, u32)> = seams.iter().map(pair_of).collect();
    pairs.sort_unstable();
    pairs.dedup();
    let link_pair: Vec<usize> = seams.iter().map(|l| pairs.binary_search(&pair_of(l)).unwrap_or(0)).collect();
    let mut report = GuardReport { seams: pairs.len(), ..GuardReport::default() };
    // Widest opening over the poses per seam, and per link.
    let widest = |set: &[SkinSurface], probes: &[SeamProbe]| {
        let mut links = vec![0.0_f32; probes.len()];
        for skinning in poses {
            for (x, p) in links.iter_mut().zip(probes) {
                *x = x.max(p.opening(set, skinning));
            }
        }
        let mut out = vec![0.0_f32; pairs.len()];
        for (x, &k) in links.iter().zip(&link_pair) {
            out[k] = out[k].max(*x);
        }
        (out, links)
    };
    let vanilla = widest(original, &SeamProbe::find(original, seams)).0;
    let probes = SeamProbe::find(smoothed, seams);
    // Each changed vertex belongs to its nearest seam of its own part.
    let ends: Vec<(Vec3, usize)> = seams
        .iter()
        .zip(&link_pair)
        .flat_map(|(l, &k)| [(smoothed[l.a.0 as usize].positions[l.a.1 as usize], k), (smoothed[l.b.0 as usize].positions[l.b.1 as usize], k)])
        .collect();
    let mut owned: Vec<Vec<(u32, u32)>> = vec![Vec::new(); pairs.len()];
    for (pi, (s, o)) in smoothed.iter().zip(original).enumerate() {
        let part = to_u32(pi);
        for v in (0..s.skin.len()).filter(|&v| s.skin[v] != o.skin[v]) {
            let p = s.positions[v];
            let nearest = ends
                .iter()
                .filter(|(_, k)| pairs[*k].0 == part || pairs[*k].1 == part)
                .min_by(|x, y| x.0.distance_squared(p).total_cmp(&y.0.distance_squared(p)).then(x.1.cmp(&y.1)));
            if let Some(&(_, k)) = nearest {
                owned[k].push((part, to_u32(v)));
            }
        }
    }
    let mut level = vec![0_u8; pairs.len()];
    loop {
        let (now, links) = widest(smoothed, &probes);
        let worse: Vec<usize> = (0..pairs.len()).filter(|&k| now[k] > vanilla[k] + tolerance && level[k] < 4).collect();
        if worse.is_empty() {
            break;
        }
        for k in worse {
            level[k] += 1;
            let near = |p: Vec3| ends.iter().any(|(e, x)| *x == k && e.distance(p) <= radius);
            let whole = |p: u32| (0..to_u32(smoothed[p as usize].skin.len())).map(move |v| (p, v));
            // Widening steps: the seam's own vertices; the vertices of the
            // links that open too wide (and the triangles they are measured
            // against); everything within `radius`; both whole parts.
            let restore: Vec<(u32, u32)> = match level[k] {
                1 => owned[k].clone(),
                2 => probes
                    .iter()
                    .zip(&links)
                    .zip(&link_pair)
                    .filter(|((_, x), kk)| **kk == k && **x > vanilla[k] + tolerance)
                    .flat_map(|((p, _), _)| std::iter::once(p.a).chain(p.corners.iter().map(|&c| (p.part, c))))
                    .collect(),
                3 => [pairs[k].0, pairs[k].1].into_iter().flat_map(whole).filter(|&(p, v)| near(smoothed[p as usize].positions[v as usize])).collect(),
                _ => [pairs[k].0, pairs[k].1].into_iter().flat_map(whole).collect(),
            };
            for (p, v) in restore {
                let (s, o) = (&mut smoothed[p as usize], &original[p as usize]);
                let v = v as usize;
                if s.skin[v] != o.skin[v] {
                    s.skin[v] = o.skin[v];
                    report.vertices += 1;
                }
                // Last resort: the original surface too (welds undone).
                if level[k] == 4 {
                    s.positions[v] = o.positions[v];
                    s.normals[v] = o.normals[v];
                }
            }
        }
    }
    report.reverted = pairs.iter().zip(&level).filter(|(_, l)| **l > 0).map(|(p, _)| *p).collect();
    report
}

/// A seam link's `a` vertex and its closest point on `b`'s part at rest.
struct SeamProbe {
    a: (u32, u32),
    part: u32,
    corners: [u32; 3],
    bary: [f32; 3],
    rest: f32,
}

impl SeamProbe {
    fn find(parts: &[SkinSurface], seams: &[SeamLink]) -> Vec<Self> {
        let meshes: Vec<Option<SurfaceMesh>> = parts.iter().map(|s| SurfaceMesh::from_raw(&s.positions, &s.triangles)).collect();
        seams
            .iter()
            .map(|l| {
                let p = parts[l.a.0 as usize].positions[l.a.1 as usize];
                let hit = meshes[l.b.0 as usize].as_ref().and_then(|m| m.closest(p, 1.0e6).map(|(t, q)| (m.triangles[t], m.barycentric(t, q).map(|x| x.max(0.0)))));
                let (corners, bary) = hit.unwrap_or(([l.b.1; 3], [1.0, 0.0, 0.0]));
                let total = bary.iter().sum::<f32>().max(f32::EPSILON);
                let bary = bary.map(|x| x / total);
                // The carried point at rest (not the projection: degenerate
                // triangles snap to a corner), so the rest pose opens nothing.
                let b = &parts[l.b.0 as usize].positions;
                let q: Vec3 = corners.iter().zip(bary).map(|(&c, x)| b[c as usize] * x).sum();
                Self { a: l.a, part: l.b.0, corners, bary, rest: p.distance(q) }
            })
            .collect()
    }

    fn opening(&self, parts: &[SkinSurface], skinning: &[Affine3A]) -> f32 {
        let a = &parts[self.a.0 as usize];
        let b = &parts[self.part as usize];
        let p = skin_point(a.positions[self.a.1 as usize], &a.skin[self.a.1 as usize], skinning);
        let q: Vec3 = self.corners.iter().zip(self.bary).map(|(&c, x)| skin_point(b.positions[c as usize], &b.skin[c as usize], skinning) * x).sum();
        (p.distance(q) - self.rest).max(0.0)
    }
}

/// Linear blend skinning of one point.
fn skin_point(p: Vec3, s: &SkinVertex, skinning: &[Affine3A]) -> Vec3 {
    (0..MAX_INFLUENCES).filter(|&k| s.weights[k] > 0.0).map(|k| skinning[usize::from(s.joints[k])].transform_point3(p) * s.weights[k]).sum()
}

/// Sparse weights → the strongest [`MAX_INFLUENCES`], normalized.
fn top_influences(w: &[(u16, f32)]) -> SkinVertex {
    let mut sorted: Vec<(u16, f32)> = w.iter().copied().filter(|(_, x)| *x > 1e-4).collect();
    sorted.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    sorted.truncate(MAX_INFLUENCES);
    let total: f32 = sorted.iter().map(|(_, x)| x).sum();
    let mut out = SkinVertex { joints: [0; MAX_INFLUENCES], weights: [0.0; MAX_INFLUENCES] };
    for (k, (j, x)) in sorted.into_iter().enumerate() {
        out.joints[k] = j;
        out.weights[k] = if total > 0.0 { x / total } else { 0.0 };
    }
    out
}

/// A seam link: boundary vertex `a`, the other part's nearest corner `b`,
/// and the closest point there as `bary` over `corners` (flat indices).
struct Link {
    a: usize,
    b: usize,
    distance: f32,
    /// To the closest point (not the corner).
    surface: f32,
    contact: Contact,
    corners: [usize; 3],
    bary: [f32; 3],
}

/// How a rim vertex meets the other part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Contact {
    /// Across a gap (or touching).
    Gap,
    /// Hidden inside the other part.
    Inside,
    /// A shell rim over the other part.
    Shell,
}

/// Whether boundary point `p` (normal `normal`) may link to the closest
/// point `q` on triangle `tri` of `mesh`: `Some(false)` across a gap within
/// `link`, `Some(true)` for overlapping pieces within `overlap`: `p` inside
/// the other part (the way out runs against its outward normal), or `p`'s
/// rim a shell over it (outside it, `p`'s own surface facing away).
/// The closest point `q` lies on an open edge (or open corner) of the
/// surface: a point beyond an open tube's end projects onto its rim, where
/// the normal says nothing about inside or outside.
fn on_open_edge(mesh: &SurfaceMesh, tri: usize, q: Vec3, open: impl Fn(u32) -> bool) -> bool {
    let bary = mesh.barycentric(tri, q);
    let c = mesh.triangles[tri];
    (0..3).any(|k| bary[k] < 1e-3 && open(c[(k + 1) % 3]) && open(c[(k + 2) % 3]))
        || (0..3).any(|k| bary[k] > 1.0 - 1e-3 && open(c[k]))
}

fn overlap_kind(mesh: &SurfaceMesh, tri: usize, q: Vec3, p: Vec3, normal: Vec3, open_edge: bool, settings: &SmoothSettings) -> Option<Contact> {
    let d = q.distance(p);
    let along = mesh.face_normal(tri).dot(p - q);
    let overlap = d > settings.weld && !open_edge && d <= settings.overlap.max(settings.link);
    if overlap && along < -0.5 * d {
        Some(Contact::Inside)
    } else if overlap && along > 0.5 * d && normal.dot(p - q) > 0.5 * d {
        Some(Contact::Shell)
    } else {
        (d <= settings.link).then_some(Contact::Gap)
    }
}

/// Overlap zones (see `smooth_body`): for each free node of a rigid part inside a
/// linked neighbor, the neighbor's solved weights at the closest point there.
#[allow(clippy::too_many_arguments)]
fn overlap_copies(
    flat: &Flat,
    graph: &Graph,
    meshes: &[Option<SurfaceMesh>],
    links: &[Link],
    pinned: &[bool],
    follow: &[Vec<(usize, f32)>],
    weights: &[Vec<(u16, f32)>],
    settings: &SmoothSettings,
) -> Vec<Option<Vec<(u16, f32)>>> {
    let mut partners: Vec<(u32, u32)> = links.iter().flat_map(|l| [(flat.part[l.a], flat.part[l.b]), (flat.part[l.b], flat.part[l.a])]).collect();
    partners.sort_unstable();
    partners.dedup();
    let mut copied: Vec<Option<Vec<(u16, f32)>>> = vec![None; graph.len()];
    for v in 0..flat.len() {
        let (n, part) = (graph.node[v], flat.part[v]);
        // Seam followers keep their constraint (the seam wins over the overlap).
        if pinned[n] || copied[n].is_some() || !follow[n].is_empty() {
            continue;
        }
        let p = flat.positions[v];
        let mut best: Option<(f32, usize, usize, Vec3)> = None;
        for &(_, b) in partners.iter().filter(|x| x.0 == part) {
            let Some(mesh) = meshes[b as usize].as_ref() else { continue };
            let Some((tri, q)) = mesh.closest(p, settings.overlap) else { continue };
            let d = q.distance(p);
            if d > settings.weld
                && mesh.face_normal(tri).dot(p - q) < -0.5 * d
                && !on_open_edge(mesh, tri, q, |c| flat.is_boundary[flat.offsets[b as usize] + c as usize])
                && best.is_none_or(|x| d < x.0)
            {
                best = Some((d, b as usize, tri, q));
            }
        }
        if let Some((_, b, tri, q)) = best {
            let mesh = meshes[b].as_ref().expect("partner mesh");
            let bary = mesh.barycentric(tri, q).map(|x| x.max(0.0));
            let total = bary.iter().sum::<f32>().max(f32::EPSILON);
            let mut w = Vec::new();
            for (c, x) in mesh.triangles[tri].iter().zip(bary) {
                for &(j, y) in &weights[graph.node[flat.offsets[b] + *c as usize]] {
                    add(&mut w, j, y * x / total);
                }
            }
            copied[n] = Some(w);
        }
    }
    copied
}

/// Rigid on one joint: every vertex fully weighted to the same joint (a
/// segmented piece), as opposed to an authored skin.
fn is_rigid(s: &SkinSurface) -> bool {
    let joint = |v: &SkinVertex| (0..MAX_INFLUENCES).find(|&k| v.weights[k] >= 0.999).map(|k| v.joints[k]);
    let first = s.skin.first().and_then(joint);
    first.is_some() && s.skin.iter().all(|v| joint(v) == first)
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).expect("mesh over 4G vertices")
}

/// All parts' vertices in one index space.
struct Flat {
    positions: Vec<Vec3>,
    part: Vec<u32>,
    offsets: Vec<usize>,
    is_boundary: Vec<bool>,
}

impl Flat {
    fn new(parts: &[SkinSurface]) -> Self {
        let mut out = Self { positions: Vec::new(), part: Vec::new(), offsets: Vec::new(), is_boundary: Vec::new() };
        for (i, s) in parts.iter().enumerate() {
            out.offsets.push(out.positions.len());
            out.positions.extend(&s.positions);
            out.part.extend(std::iter::repeat_n(to_u32(i), s.positions.len()));
        }
        out.is_boundary = vec![false; out.positions.len()];
        out
    }

    fn len(&self) -> usize {
        self.positions.len()
    }

    fn local(&self, i: usize) -> (u32, u32) {
        let p = self.part[i];
        (p, to_u32(i - self.offsets[p as usize]))
    }

    fn normal(&self, parts: &[SkinSurface], i: usize) -> Vec3 {
        let (p, v) = self.local(i);
        parts[p as usize].normals.get(v as usize).copied().unwrap_or(Vec3::ZERO)
    }

    /// Marks and returns vertices on open edges (edges of one triangle,
    /// counted on duplicate-merged vertices).
    fn boundary(&mut self, parts: &[SkinSurface], uf: &mut UnionFind) -> Vec<usize> {
        let mut edges: HashMap<(usize, usize), u32> = HashMap::new();
        for (p, s) in parts.iter().enumerate() {
            for t in &s.triangles {
                for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                    let (a, b) = (uf.find(self.offsets[p] + a as usize), uf.find(self.offsets[p] + b as usize));
                    *edges.entry((a.min(b), a.max(b))).or_default() += 1;
                }
            }
        }
        let mut open = vec![false; self.len()];
        for ((a, b), n) in edges {
            if n == 1 {
                open[a] = true;
                open[b] = true;
            }
        }
        let mut out = Vec::new();
        for i in 0..self.len() {
            if open[uf.find(i)] {
                self.is_boundary[i] = true;
                out.push(i);
            }
        }
        out
    }
}

struct UnionFind(Vec<usize>);

impl UnionFind {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }

    fn find(&mut self, mut i: usize) -> usize {
        while self.0[i] != i {
            self.0[i] = self.0[self.0[i]];
            i = self.0[i];
        }
        i
    }

    fn union(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b {
            self.0[a.max(b)] = a.min(b);
        }
    }
}

/// Uniform hash grid over points for radius queries.
struct PointGrid {
    cell: f32,
    cells: HashMap<[i32; 3], Vec<usize>>,
}

impl PointGrid {
    fn new(points: &[Vec3], cell: f32) -> Self {
        let mut cells: HashMap<[i32; 3], Vec<usize>> = HashMap::new();
        for (i, p) in points.iter().enumerate() {
            cells.entry(Self::key(*p, cell)).or_default().push(i);
        }
        Self { cell, cells }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn key(p: Vec3, cell: f32) -> [i32; 3] {
        (p / cell).floor().to_array().map(|x| x as i32)
    }

    fn near(&self, points: &[Vec3], p: Vec3, radius: f32, mut f: impl FnMut(usize)) {
        let (lo, hi) = (Self::key(p - radius, self.cell), Self::key(p + radius, self.cell));
        for x in lo[0]..=hi[0] {
            for y in lo[1]..=hi[1] {
                for z in lo[2]..=hi[2] {
                    for &i in self.cells.get(&[x, y, z]).map_or(&[][..], Vec::as_slice) {
                        if points[i].distance(p) <= radius {
                            f(i);
                        }
                    }
                }
            }
        }
    }
}

/// Weld clusters as nodes, triangle edges plus seam links as edges.
struct Graph {
    node: Vec<usize>,
    members: Vec<Vec<usize>>,
    positions: Vec<Vec3>,
    /// `(neighbor, weight)` per node; weight = 1 / edge length.
    adjacency: Vec<Vec<(usize, f32)>>,
}

impl Graph {
    fn new(flat: &Flat, parts: &[SkinSurface], uf: &mut UnionFind, links: impl Iterator<Item = (usize, usize)>) -> Self {
        let mut ids: HashMap<usize, usize> = HashMap::new();
        let mut node = Vec::with_capacity(flat.len());
        let mut members: Vec<Vec<usize>> = Vec::new();
        for i in 0..flat.len() {
            let root = uf.find(i);
            let n = *ids.entry(root).or_insert_with(|| {
                members.push(Vec::new());
                members.len() - 1
            });
            members[n].push(i);
            node.push(n);
        }
        let positions: Vec<Vec3> = members.iter().map(|m| m.iter().map(|&i| flat.positions[i]).sum::<Vec3>() / count(m.len())).collect();
        // Sorted, not hashed: adjacency order fixes the solver's summation
        // order, so the result is bit-identical across runs and threads.
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (p, s) in parts.iter().enumerate() {
            for t in &s.triangles {
                for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                    let (a, b) = (node[flat.offsets[p] + a as usize], node[flat.offsets[p] + b as usize]);
                    if a != b {
                        edges.push((a.min(b), a.max(b)));
                    }
                }
            }
        }
        for (a, b) in links {
            let (a, b) = (node[a], node[b]);
            if a != b {
                edges.push((a.min(b), a.max(b)));
            }
        }
        edges.sort_unstable();
        edges.dedup();
        let mut adjacency = vec![Vec::new(); members.len()];
        let scale = positions.iter().fold(0.0_f32, |m, p| m.max(p.abs().max_element())).max(1.0) * 1e-6;
        for (a, b) in edges {
            let w = 1.0 / positions[a].distance(positions[b]).max(scale);
            adjacency[a].push((b, w));
            adjacency[b].push((a, w));
        }
        Self { node, members, positions, adjacency }
    }

    fn len(&self) -> usize {
        self.members.len()
    }

    /// Mean of the weights of the members `keep` accepts, sparse.
    fn initial_weights(&self, flat: &Flat, parts: &[SkinSurface], keep: impl Fn(usize) -> bool) -> Vec<Vec<(u16, f32)>> {
        self.members
            .iter()
            .map(|m| {
                let kept: Vec<usize> = m.iter().copied().filter(|&i| keep(i)).collect();
                let mut w: Vec<(u16, f32)> = Vec::new();
                for &i in &kept {
                    let (p, v) = flat.local(i);
                    let s = parts[p as usize].skin[v as usize];
                    for k in 0..MAX_INFLUENCES {
                        add(&mut w, s.joints[k], s.weights[k] / count(kept.len()));
                    }
                }
                w
            })
            .collect()
    }

    /// Dijkstra from `seeds`, stopping at `limit`.
    fn geodesic(&self, seeds: &[usize], limit: f32) -> Vec<f32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut dist = vec![f32::INFINITY; self.len()];
        let mut heap = BinaryHeap::new();
        for &s in seeds {
            dist[s] = 0.0;
            heap.push(Reverse((0_u32, s)));
        }
        while let Some(Reverse((d, n))) = heap.pop() {
            let d = f32::from_bits(d);
            if d > dist[n] {
                continue;
            }
            for &(m, w) in &self.adjacency[n] {
                let nd = d + 1.0 / w;
                if nd < dist[m] && nd < limit {
                    dist[m] = nd;
                    // Non-negative f32 bits order like the values.
                    heap.push(Reverse((nd.to_bits(), m)));
                }
            }
        }
        dist
    }

    fn components(&self) -> Vec<usize> {
        let mut comp = vec![usize::MAX; self.len()];
        for start in 0..self.len() {
            if comp[start] != usize::MAX {
                continue;
            }
            let mut stack = vec![start];
            comp[start] = start;
            while let Some(n) = stack.pop() {
                for &(m, _) in &self.adjacency[n] {
                    if comp[m] == usize::MAX {
                        comp[m] = start;
                        stack.push(m);
                    }
                }
            }
        }
        comp
    }

    /// Harmonic inpainting: unknown nodes become the weighted mean of their
    /// neighbors (Gauss-Seidel), known nodes are fixed. Harmonic weights stay
    /// in [0, 1] and sum to 1 (maximum principle), with no solver crate.
    /// A node with a non-empty `follow` list is instead that convex blend of
    /// other nodes (a seam constraint).
    fn inpaint(&self, weights: &mut [Vec<(u16, f32)>], unknown: &[bool], follow: &[Vec<(usize, f32)>], iterations: usize, tolerance: f32) -> (usize, f32) {
        let order: Vec<usize> = (0..self.len()).filter(|&n| unknown[n] && !self.adjacency[n].is_empty()).collect();
        let mut residual = 0.0;
        for sweep in 0..iterations {
            residual = 0.0_f32;
            for &n in &order {
                let mut acc: Vec<(u16, f32)> = Vec::new();
                let mut total = 0.0;
                let terms = follow.get(n).filter(|f| !f.is_empty()).map_or(&self.adjacency[n][..], Vec::as_slice);
                for &(m, w) in terms {
                    total += w;
                    for &(j, x) in &weights[m] {
                        add(&mut acc, j, x * w);
                    }
                }
                for e in &mut acc {
                    e.1 /= total;
                }
                acc.retain(|e| e.1 > 1e-6);
                for &(j, x) in &acc {
                    residual = residual.max((x - get(&weights[n], j)).abs());
                }
                for &(j, x) in &weights[n] {
                    if get(&acc, j) == 0.0 {
                        residual = residual.max(x);
                    }
                }
                weights[n] = acc;
            }
            if residual <= tolerance {
                return (sweep + 1, residual);
            }
        }
        (iterations, residual)
    }
}

fn add(w: &mut Vec<(u16, f32)>, j: u16, x: f32) {
    if x == 0.0 {
        return;
    }
    match w.iter_mut().find(|e| e.0 == j) {
        Some(e) => e.1 += x,
        None => w.push((j, x)),
    }
}

fn get(w: &[(u16, f32)], j: u16) -> f32 {
    w.iter().find(|e| e.0 == j).map_or(0.0, |e| e.1)
}

/// Closest-point queries on a triangle soup (parry's BVH).
struct SurfaceMesh {
    mesh: TriMesh,
    positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    skin: Vec<SkinVertex>,
    /// `-1` when the winding faces inward.
    outward: f32,
}

impl SurfaceMesh {
    fn new(parts: &[SkinSurface]) -> Option<Self> {
        let (mut positions, mut triangles, mut skin) = (Vec::new(), Vec::new(), Vec::new());
        for s in parts {
            let base = to_u32(positions.len());
            positions.extend(&s.positions);
            skin.extend(&s.skin);
            triangles.extend(s.triangles.iter().map(|t| t.map(|i| i + base)));
        }
        let mut out = Self::from_raw(&positions, &triangles)?;
        out.skin = skin;
        Some(out)
    }

    fn from_raw(positions: &[Vec3], triangles: &[[u32; 3]]) -> Option<Self> {
        if triangles.is_empty() {
            return None;
        }
        let mesh = TriMesh::new(positions.to_vec(), triangles.to_vec()).ok()?;
        // Winding may face inward (mirrored pieces): orient by the sign of
        // the volume enclosed about the centroid, which holds for closed
        // meshes, open tubes and shells alike.
        let centroid = positions.iter().sum::<Vec3>() / count(positions.len());
        let volume: f32 = triangles
            .iter()
            .map(|t| {
                let [a, b, c] = t.map(|i| positions[i as usize] - centroid);
                a.dot(b.cross(c))
            })
            .sum();
        let outward = if volume < 0.0 { -1.0 } else { 1.0 };
        Some(Self { mesh, positions: positions.to_vec(), triangles: triangles.to_vec(), skin: Vec::new(), outward })
    }

    fn closest(&self, p: Vec3, max_distance: f32) -> Option<(usize, Vec3)> {
        let (tri, proj) = CompositeShapeRef(&self.mesh).project_local_point(p, max_distance, false)?;
        (proj.point.distance(p) <= max_distance).then_some((tri as usize, proj.point))
    }

    fn corners(&self, tri: usize) -> [Vec3; 3] {
        self.triangles[tri].map(|i| self.positions[i as usize])
    }

    fn face_normal(&self, tri: usize) -> Vec3 {
        let [a, b, c] = self.corners(tri);
        (b - a).cross(c - a).normalize_or_zero() * self.outward
    }

    #[allow(clippy::many_single_char_names)]
    fn barycentric(&self, tri: usize, q: Vec3) -> [f32; 3] {
        let [a, b, c] = self.corners(tri);
        let (v0, v1, v2) = (b - a, c - a, q - a);
        let (d00, d01, d11, d20, d21) = (v0.dot(v0), v0.dot(v1), v1.dot(v1), v2.dot(v0), v2.dot(v1));
        let den = d00 * d11 - d01 * d01;
        if den.abs() < f32::EPSILON {
            return [1.0, 0.0, 0.0];
        }
        let v = (d11 * d20 - d01 * d21) / den;
        let w = (d00 * d21 - d01 * d20) / den;
        [1.0 - v - w, v, w]
    }

    fn interior(&self, tri: usize, q: Vec3) -> bool {
        self.barycentric(tri, q).iter().all(|&x| x > 1e-3)
    }

    fn weights_at(&self, tri: usize, q: Vec3) -> Vec<(u16, f32)> {
        let bary = self.barycentric(tri, q).map(|x| x.clamp(0.0, 1.0));
        let total: f32 = bary.iter().sum();
        let mut w = Vec::new();
        for (corner, b) in self.triangles[tri].iter().zip(bary) {
            let s = self.skin[*corner as usize];
            for k in 0..MAX_INFLUENCES {
                add(&mut w, s.joints[k], s.weights[k] * b / total.max(f32::EPSILON));
            }
        }
        w
    }
}

#[allow(clippy::cast_precision_loss)]
fn count(n: usize) -> f32 {
    n as f32
}

/// The joint with the largest weight.
fn dominant_joint(s: &SkinVertex) -> usize {
    let k = (0..MAX_INFLUENCES).max_by(|&a, &b| s.weights[a].total_cmp(&s.weights[b])).unwrap_or(0);
    usize::from(s.joints[k])
}
/// Skeleton edges between joints `a` and `b` through their common ancestor
/// (`usize::MAX` if unrelated, or if neither is the ancestor itself or a
/// leaf joint right below it).
/// (`usize::MAX` if unrelated or if both sit two or more below it).
fn hops(parents: &[Option<usize>], a: usize, b: usize) -> usize {
    let chain = |mut j: usize| {
        let mut out = vec![j];
        while let Some(p) = parents.get(j).copied().flatten() {
            out.push(p);
            j = p;
        }
        out
    };
    let (ca, cb) = (chain(a), chain(b));
    // One side must be the common ancestor or a leaf right below it (a piece
    // hung on the parent bone): seams join a segment to its parent chain,
    // never two sibling limbs (thigh to thigh, tail to thigh).
    let leaf = |j: usize| !parents.contains(&Some(j));
    ca.iter()
        .enumerate()
        .find_map(|(i, j)| cb.iter().position(|k| k == j).map(|k| (i, k)))
        .filter(|&(i, k)| i == 0 || k == 0 || (i == 1 && leaf(a)) || (k == 1 && leaf(b)))
        .map_or(usize::MAX, |(i, k)| i + k)
}

#[cfg(test)]
mod tests {
    use glam::{Affine3A, Quat, Vec3};
    use sf_model::SkinVertex;

    use super::{
        SkinSurface, SmoothSettings, TransferSettings, find_seams, guard_seams, penetration, seam_openings, skinning_matrices, smooth_body, transfer_weights,
    };

    /// An open tube along +Y from `y0` to `y1`, rigid on `joint`.
    #[allow(clippy::cast_precision_loss)]
    fn tube(y0: f32, y1: f32, radius: f32, joint: u16) -> SkinSurface {
        let (rings, around) = (5_u32, 8_u32);
        let mut s = SkinSurface::default();
        for r in 0..rings {
            let y = y0 + (y1 - y0) * r as f32 / (rings - 1) as f32;
            for k in 0..around {
                let a = std::f32::consts::TAU * k as f32 / around as f32;
                s.positions.push(Vec3::new(radius * a.cos(), y, radius * a.sin()));
                s.normals.push(Vec3::new(a.cos(), 0.0, a.sin()));
                s.skin.push(SkinVertex { joints: [joint, 0, 0, 0], weights: [1.0, 0.0, 0.0, 0.0] });
            }
        }
        for r in 0..rings - 1 {
            for k in 0..around {
                let (a, b) = (r * around + k, r * around + (k + 1) % around);
                s.triangles.push([a, b + around, b]);
                s.triangles.push([a, a + around, b + around]);
            }
        }
        s
    }

    /// Two rigid segments meeting at a hinge: the vanilla seam opens when
    /// the child bends 90°; after smoothing it stays closed and weights blend.
    #[test]
    fn segmented_limb_seam_closes_and_weights_blend() {
        let parents = [None, Some(0)];
        let rest = [Affine3A::IDENTITY, Affine3A::from_translation(Vec3::Y)];
        let bent = [rest[0], Affine3A::from_translation(Vec3::Y) * Affine3A::from_quat(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))];
        let skinning = skinning_matrices(&rest, &bent);
        let vanilla = vec![tube(0.0, 1.0, 0.1, 0), tube(1.0, 2.0, 0.1, 1)];
        let settings = SmoothSettings { band: 0.3, ..SmoothSettings::default() };
        let seams = find_seams(&vanilla, &parents, &settings);
        assert!(!seams.is_empty());
        let opening = |set: &[SkinSurface]| {
            let posed: Vec<Vec<Vec3>> = set.iter().map(|s| s.posed(&skinning)).collect();
            seams.iter().map(|l| posed[l.a.0 as usize][l.a.1 as usize].distance(posed[l.b.0 as usize][l.b.1 as usize])).fold(0.0, f32::max)
        };
        assert!(opening(&vanilla) > 0.05, "vanilla seam should open");
        let mut smooth = vanilla.clone();
        let report = smooth_body(&mut smooth, &parents, &settings);
        assert!(report.band > 0 && !report.seams.is_empty());
        assert!(opening(&smooth) < 1e-4, "smoothed seam stays closed: {}", opening(&smooth));
        for s in &smooth {
            for (p, w) in s.positions.iter().zip(&s.skin) {
                assert!((w.weights.iter().sum::<f32>() - 1.0).abs() < 1e-4);
                let child = w.joints.iter().zip(w.weights).filter(|(j, _)| **j == 1).map(|(_, x)| x).sum::<f32>();
                if (p.y - 1.0).abs() > 0.3 {
                    assert!((child - f32::from(u8::from(p.y > 1.0))).abs() < 1e-4, "rigid outside the band at y {}", p.y);
                } else if (p.y - 1.0).abs() < 1e-4 {
                    assert!((child - 0.5).abs() < 0.05, "seam blends half and half: {child}");
                }
            }
        }
        // A sleeve around the limb takes the limb's weights.
        let mut sleeve = vec![tube(0.6, 1.4, 0.12, 0)];
        let t = transfer_weights(&smooth, &mut sleeve, &TransferSettings::default());
        assert_eq!(t.matched, sleeve[0].positions.len());
        let tip = sleeve[0].skin.last().unwrap();
        assert_eq!(tip.joints[0], 1, "sleeve end follows the child");
        // A body without seams is left alone.
        let mut whole = vec![tube(0.0, 2.0, 0.1, 0)];
        assert_eq!(smooth_body(&mut whole, &parents, &settings).band, 0);
    }

    /// Weights blending linearly from joint 0 to joint 1 between `y0` and `y1`.
    fn skinned(mut s: SkinSurface, y0: f32, y1: f32) -> SkinSurface {
        for (p, w) in s.positions.iter().zip(&mut s.skin) {
            let t = ((p.y - y0) / (y1 - y0)).clamp(0.0, 1.0);
            *w = SkinVertex { joints: [0, 1, 0, 0], weights: [1.0 - t, t, 0.0, 0.0] };
        }
        s
    }

    /// Overlapping pieces: the child's rim sits 6 cm deep inside a wider
    /// parent sleeve (farther than `link` from any surface). The volume test
    /// links it, and after smoothing the rim stays inside when bent.
    #[test]
    fn overlapping_rim_is_linked_and_stays_inside() {
        let parents = [None, Some(0)];
        let rest = [Affine3A::IDENTITY, Affine3A::from_translation(Vec3::Y)];
        let bent = [rest[0], Affine3A::from_translation(Vec3::Y) * Affine3A::from_quat(Quat::from_rotation_z(1.2))];
        let skinning = skinning_matrices(&rest, &bent);
        let parts = vec![tube(0.0, 1.1, 0.16, 0), tube(0.9, 2.0, 0.09, 1)];
        let gap_only = SmoothSettings { overlap: 0.0, band: 0.3, ..SmoothSettings::default() };
        assert!(find_seams(&parts, &parents, &gap_only).is_empty(), "rim is 7 cm from the sleeve: no gap link");
        let settings = SmoothSettings { band: 0.3, ..SmoothSettings::default() };
        let seams = find_seams(&parts, &parents, &settings);
        assert!(seams.iter().any(|l| l.inside && l.a.0 == 1), "inner rim linked as inside");
        assert!(seams.iter().any(|l| l.inside && l.a.0 == 0), "sleeve rim linked as covering the child");
        let mut smooth = parts.clone();
        smooth_body(&mut smooth, &parents, &settings);
        // How far the child pokes out through the sleeve when bent.
        let poke = |set: &[SkinSurface]| {
            let identity = skinning_matrices(&rest, &rest);
            let (rest_sleeve, posed_sleeve) = (set[0].posed(&identity), set[0].posed(&skinning));
            let d = penetration(&set[1].posed(&identity), &set[1].posed(&skinning), (&rest_sleeve, &set[0].triangles), (&posed_sleeve, &set[0].triangles), 0.2);
            d.into_iter().fold(0.0, f32::max)
        };
        assert!(poke(&parts) > 0.02, "vanilla child pokes out {}", poke(&parts));
        assert!(poke(&smooth) < 0.005, "smoothed child stays inside: {}", poke(&smooth));
    }

    /// Already-skinned content passes through unchanged: a seamless skinned
    /// tube, and two authored pieces welded at a seam.
    #[test]
    fn authored_skins_pass_through() {
        let parents = [None, Some(0)];
        let settings = SmoothSettings::default();
        let whole = vec![skinned(tube(0.0, 2.0, 0.1, 0), 0.8, 1.2)];
        let mut out = whole.clone();
        smooth_body(&mut out, &parents, &settings);
        assert_eq!(out, whole);
        let split = vec![skinned(tube(0.0, 1.0, 0.1, 0), 0.8, 1.2), skinned(tube(1.0, 2.0, 0.1, 0), 0.8, 1.2)];
        let mut out = split.clone();
        let report = smooth_body(&mut out, &parents, &settings);
        assert!(!report.seams.is_empty());
        assert_eq!(report.band, 0);
        for (o, s) in out.iter().zip(&split) {
            // Welding renormalizes the shared normals (last-bit rounding only).
            assert_eq!((&o.positions, &o.skin), (&s.positions, &s.skin));
            assert!(o.normals.iter().zip(&s.normals).all(|(a, b)| a.distance(*b) < 1e-6));
        }
        // A rigid piece next to an authored one blends into it; the authored side is kept.
        let mixed = vec![skinned(tube(0.0, 1.0, 0.1, 0), 0.8, 1.2), tube(1.004, 2.0, 0.1, 1)];
        let mut out = mixed.clone();
        smooth_body(&mut out, &parents, &SmoothSettings { band: 0.3, ..settings });
        assert_eq!(out[0].skin, mixed[0].skin);
        assert_ne!(out[1].skin, mixed[1].skin);
    }

    /// Bit-identical output across runs and threads.
    #[test]
    fn deterministic() {
        let parents = [None, Some(0)];
        let parts = vec![tube(0.0, 1.0, 0.1, 0), tube(1.004, 2.0, 0.1, 1), tube(0.9, 1.3, 0.13, 1)];
        let settings = SmoothSettings { band: 0.3, ..SmoothSettings::default() };
        let run = || {
            let mut p = parts.clone();
            let r = smooth_body(&mut p, &parents, &settings);
            (p, r)
        };
        let first = run();
        let threads: Vec<_> = std::thread::scope(|s| (0..4).map(|_| s.spawn(run)).collect::<Vec<_>>().into_iter().map(|h| h.join().unwrap()).collect());
        for t in threads {
            assert_eq!(t, first);
        }
    }

    /// The guard: a smoothed seam that opens wider than the original in a
    /// probe pose gets the original weights back; a better one is kept.
    #[test]
    fn guard_reverts_seams_worse_than_original() {
        let parents = [None, Some(0), Some(0)];
        let rest = [Affine3A::IDENTITY, Affine3A::from_translation(Vec3::Y), Affine3A::from_translation(Vec3::Y)];
        let bent = [rest[0], Affine3A::from_translation(Vec3::Y) * Affine3A::from_quat(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)), Affine3A::from_translation(Vec3::new(0.5, 1.0, 0.0))];
        let poses = vec![skinning_matrices(&rest, &bent)];
        let original = vec![tube(0.0, 1.0, 0.1, 0), tube(1.004, 2.0, 0.1, 1)];
        let settings = SmoothSettings { band: 0.3, ..SmoothSettings::default() };
        let mut smooth = original.clone();
        let report = smooth_body(&mut smooth, &parents, &settings);
        let kept = smooth.clone();
        let g = guard_seams(&original, &mut smooth, &report.seams, &poses, 5e-4, settings.band);
        assert!(g.reverted.is_empty() && smooth == kept, "a closed seam is kept");
        // Corrupt the child's rim onto an unrelated joint: worse than the original, reverted.
        let mut bad = kept.clone();
        let child = &mut bad[1];
        for (p, w) in child.positions.iter().zip(&mut child.skin) {
            if p.y < 1.1 {
                *w = SkinVertex { joints: [2, 0, 0, 0], weights: [1.0, 0.0, 0.0, 0.0] };
            }
        }
        let wide = |set: &[SkinSurface]| seam_openings(set, &report.seams, &poses[0]).into_iter().fold(0.0, f32::max);
        let g = guard_seams(&original, &mut bad, &report.seams, &poses, 5e-4, settings.band);
        assert_eq!(g.reverted, vec![(0, 1)]);
        assert!(wide(&bad) <= wide(&original) + 5e-4, "{} vs {}", wide(&bad), wide(&original));
    }
}
