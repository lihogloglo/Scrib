//! Procedural rock meshes (spec `docs/specs/rockgen.md`): everything a heightfield can't show, made from a signed
//! distance field (SDF, inside negative) and meshed with surface nets, as a pure function of a seed and a declared
//! parameter set, so any peer regenerates the same rock (net-ready) and a bake can key products by [`RockLods::hash`].
//!
//! Slice RG-1: [`boulder`] (a convex block from joint-set slabs and chip planes, smooth-max rounded, noise
//! weathered) with a `meshopt` LOD chain. Slice RG-4: sharp families ([`classes`]: talus, jagged, columnar,
//! scoria, slab beside the eroded ones), vertices snapped onto the analytic planes ([`sdf`], [`mesher`]) and
//! crease-split normals ([`RockMesh::split_creases`]). Slice RG-2: [`library`] (per-class variant libraries, one
//! convex collision hull per variant with the `collision` feature). Output is plain arrays, no Bevy types:
//! `sf_rockgen_bevy` (RG-3) wraps them. Nothing here names a game.

#[cfg(feature = "bake")]
pub mod bake;
pub mod boulder;
pub mod classes;
#[cfg(feature = "collision")]
mod collision;
mod columns;
pub mod formations;
pub mod library;
pub mod mesher;
mod noise;
pub mod sdf;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use boulder::{BoulderParams, generate_boulder};
pub use classes::{CLASSES, Family, RockClass};
pub use formations::{FORMATIONS, Formation, generate_formation};
pub use library::{Hull, HullSettings, Library, LibrarySettings, Variant, build_library, library_key, variant_seed};

/// One triangle mesh: counter-clockwise front faces seen from outside, unit normals (the SDF gradient on smooth
/// vertices; on a crease, each side's own face-normal average, [`RockMesh::split_creases`]).
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct RockMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl RockMesh {
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// True when the surface is closed: with vertices welded by position (bit-equal, so a crease's split copies
    /// are one point), every edge is shared by exactly two triangles that use it in opposite directions (closed,
    /// edge-manifold, consistently oriented) and no triangle repeats a point.
    pub fn is_closed(&self) -> bool {
        indices_closed(&self.welded_indices())
    }

    /// The indices with every vertex replaced by the first vertex at the same position (bit-equal).
    pub fn welded_indices(&self) -> Vec<u32> {
        let mut first: HashMap<[u32; 3], u32> = HashMap::with_capacity(self.positions.len());
        let canon: Vec<u32> = (0..self.positions.len())
            .map(|i| {
                let key = self.positions[i].map(f32::to_bits);
                *first.entry(key).or_insert(u32::try_from(i).unwrap_or(u32::MAX))
            })
            .collect();
        self.indices.iter().map(|&i| canon[i as usize]).collect()
    }

    /// Hard edges: a vertex whose faces fall into several groups (neighbouring faces around it joined when they
    /// meet at less than `crease_deg`) gets one copy per group, each with its group's angle-weighted face normal;
    /// a vertex with one group keeps its normal (the SDF gradient) unless it leans more than half the crease off
    /// that group's face normal (next to a hard edge). Positions are copied bit-exact, so the welded
    /// surface is the same (only normals split; UE / Blender "smoothing groups by angle"). `crease_deg` ≥ 180
    /// returns the mesh unchanged.
    #[must_use]
    #[allow(clippy::too_many_lines, clippy::many_single_char_names)] // one pass per vertex fan; maths notation
    pub fn split_creases(&self, crease_deg: f32) -> Self {
        if crease_deg >= 180.0 || self.indices.is_empty() {
            return self.clone();
        }
        let cos_limit = crease_deg.to_radians().cos();
        let cos_half = (0.5 * crease_deg).to_radians().cos();
        let p = |i: u32| glam::Vec3::from_array(self.positions[i as usize]);
        let tris = self.indices.as_chunks::<3>().0;
        // Unit face normals; a sliver (near-zero area for its size: its normal is noise) has none. Slivers never
        // join two groups; each follows a neighbour.
        let unit: Vec<Option<glam::Vec3>> = tris
            .iter()
            .map(|t| {
                let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
                let n = (b - a).cross(c - a);
                let longest = (b - a).length_squared().max((c - b).length_squared()).max((a - c).length_squared());
                (n.length() > SLIVER_SIN * longest).then(|| n.normalize())
            })
            .collect();
        let mut incident: Vec<Vec<(usize, usize)>> = vec![Vec::new(); self.positions.len()];
        for (f, t) in tris.iter().enumerate() {
            for (corner, &v) in t.iter().enumerate() {
                incident[v as usize].push((f, corner));
            }
        }
        let mut out = self.clone();
        for (v, fan) in incident.iter().enumerate() {
            if fan.len() < 2 {
                continue;
            }
            // Union-find over the fan: faces sharing an edge through v and meeting at less than the crease.
            let mut parent: Vec<usize> = (0..fan.len()).collect();
            let find = |parent: &mut Vec<usize>, mut a: usize| {
                while parent[a] != a {
                    parent[a] = parent[parent[a]];
                    a = parent[a];
                }
                a
            };
            let others = |f: usize, c: usize| [tris[f][(c + 1) % 3], tris[f][(c + 2) % 3]];
            let adjacent = |a: usize, b: usize| {
                let (oa, ob) = (others(fan[a].0, fan[a].1), others(fan[b].0, fan[b].1));
                oa.iter().any(|w| ob.contains(w))
            };
            for a in 0..fan.len() {
                for b in a + 1..fan.len() {
                    if let (Some(na), Some(nb)) = (unit[fan[a].0], unit[fan[b].0])
                        && adjacent(a, b)
                        && na.dot(nb) >= cos_limit
                    {
                        let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                        parent[ra.max(rb)] = ra.min(rb);
                    }
                }
            }
            // Slivers follow a neighbour with a normal (repeated so a run of slivers follows too), else the first group.
            let solid_faces: Vec<usize> = (0..fan.len()).filter(|&a| unit[fan[a].0].is_some()).collect();
            let mut root_of: Vec<Option<usize>> = (0..fan.len()).map(|a| unit[fan[a].0].map(|_| find(&mut parent, a))).collect();
            for _ in 0..fan.len() {
                for a in 0..fan.len() {
                    if root_of[a].is_none() {
                        root_of[a] = (0..fan.len()).find(|&b| b != a && adjacent(a, b) && root_of[b].is_some()).and_then(|b| root_of[b]);
                    }
                }
            }
            let fallback = solid_faces.first().map_or(0, |&a| find(&mut parent, a));
            let roots: Vec<usize> = root_of.iter().map(|r| r.unwrap_or(fallback)).collect();
            let mut groups: Vec<usize> = roots.clone();
            groups.sort_unstable();
            groups.dedup();
            let group_normal = |root: usize| {
                let mut sum = glam::Vec3::ZERO;
                for (a, &(f, c)) in fan.iter().enumerate() {
                    if roots[a] == root
                        && let Some(n) = unit[f]
                    {
                        let t = tris[f];
                        let (e1, e2) = (p(t[(c + 1) % 3]) - p(t[c]), p(t[(c + 2) % 3]) - p(t[c]));
                        sum += n * e1.angle_between(e2);
                    }
                }
                sum.try_normalize()
            };
            if groups.len() < 2 {
                // One smooth group keeps its SDF normal, unless that leans off its own faces by more than half the
                // crease (a vertex next to a hard edge, where the gradient blends both sides).
                if let Some(avg) = group_normal(groups[0])
                    && avg.dot(glam::Vec3::from_array(self.normals[v])) < cos_half
                {
                    out.normals[v] = avg.to_array();
                }
                continue;
            }
            for (g, &root) in groups.iter().enumerate() {
                let normal = group_normal(root).unwrap_or_else(|| glam::Vec3::from_array(self.normals[v]));
                let id = if g == 0 {
                    v
                } else {
                    out.positions.push(self.positions[v]);
                    out.normals.push(normal.to_array());
                    out.positions.len() - 1
                };
                out.normals[id] = normal.to_array();
                let id = u32::try_from(id).unwrap_or(u32::MAX);
                for (a, &(f, c)) in fan.iter().enumerate() {
                    if roots[a] == root {
                        out.indices[3 * f + c] = id;
                    }
                }
            }
        }
        out
    }

    /// Signed volume (positive when faces point outward).
    pub fn signed_volume(&self) -> f64 {
        let p = |i: u32| self.positions[i as usize].map(f64::from);
        self.indices
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| {
                let [a, b, c] = [p(t[0]), p(t[1]), p(t[2])];
                (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                    + a[2] * (b[0] * c[1] - b[1] * c[0]))
                    / 6.0
            })
            .sum()
    }

    pub fn bounds(&self) -> Aabb {
        let mut b = Aabb::EMPTY;
        for p in &self.positions {
            b.grow(*p);
        }
        b
    }
}

/// A triangle whose area is below this times its longest edge squared is a sliver (angle under ~0.006 deg): its
/// normal is float noise.
// tunables: const (numerical: degenerate-triangle test)
const SLIVER_SIN: f32 = 1.0e-4;

/// [`RockMesh::is_closed`] on an index list.
pub fn indices_closed(indices: &[u32]) -> bool {
    let mut directed: HashMap<(u32, u32), u32> = HashMap::with_capacity(indices.len());
    for tri in indices.as_chunks::<3>().0 {
        let [a, b, c] = [tri[0], tri[1], tri[2]];
        if a == b || b == c || a == c {
            return false;
        }
        for edge in [(a, b), (b, c), (c, a)] {
            *directed.entry(edge).or_insert(0) += 1;
        }
    }
    directed.iter().all(|(&(a, b), &n)| n == 1 && directed.get(&(b, a)) == Some(&1))
}

/// Axis-aligned bounds in the rock's local frame (metres).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    pub const EMPTY: Self = Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] };

    pub fn grow(&mut self, p: [f32; 3]) {
        for ((lo, hi), v) in self.min.iter_mut().zip(&mut self.max).zip(p) {
            *lo = lo.min(v);
            *hi = hi.max(v);
        }
    }

    pub fn extent(&self) -> [f32; 3] {
        [self.max[0] - self.min[0], self.max[1] - self.min[1], self.max[2] - self.min[2]]
    }
}

/// A rock's LOD chain, finest first. Triangle counts strictly decrease along the chain.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bake", derive(serde::Serialize, serde::Deserialize))]
pub struct RockLods {
    pub lods: Vec<RockMesh>,
    /// Bounds of LOD0 (every LOD's vertices are a subset of LOD0's).
    pub bounds: Aabb,
    /// FNV-1a 64 over every LOD's positions, normals (bit patterns) and indices: the bake-product key and the
    /// determinism check.
    pub hash: u64,
}

/// FNV-1a 64 over the meshes (stable across runs and platforms with the same float results).
pub fn content_hash(lods: &[RockMesh]) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0100_0000_01b3;
    let mut h = OFFSET;
    let mut eat = |word: u32| {
        for byte in word.to_le_bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(PRIME);
        }
    };
    for mesh in lods {
        eat(u32::try_from(mesh.positions.len()).unwrap_or(u32::MAX));
        for x in mesh.positions.iter().chain(&mesh.normals).flatten() {
            eat(x.to_bits());
        }
        for &i in &mesh.indices {
            eat(i);
        }
    }
    h
}
