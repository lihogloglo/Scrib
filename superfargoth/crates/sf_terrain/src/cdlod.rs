//! CDLOD node selection (Strugar, *Continuous Distance-Dependent Level of
//! Detail for Rendering Heightmaps*, JGT 2009) over the pyramid's min/max.
//!
//! A node at LOD `ℓ` is one render patch of [`PATCH_QUADS`]² quads at
//! `0.5·2^ℓ` m spacing (16·2^ℓ m), aligned to the level-0 global sample
//! grid, so LOD `ℓ ≥ 0` nodes nest inside level-`ℓ` canonical tiles: a
//! level-`k` tile is 16×16 nodes of LOD `k`, and each node is 2×2 of the
//! tile's min/max blocks. Negative LODs are the detail-displacement levels
//! (terrain.md): finer than the bake, heights come from the shader.
//!
//! Selection walks the quadtree from the roots: a node is drawn whole if
//! its box is outside the next finer LOD's range, otherwise its children are
//! visited, and children outside their own range are drawn as a quarter of
//! the parent (`quadrant`). Ranges double per LOD and the vertex shader
//! morphs over the last part of each range, so neighbors differ by at most
//! one LOD and shared edges meet exactly (no cracks, no popping).
//!
//! Selection is by distance only, never by frustum: the renderer culls each
//! view (main camera, shadow cascades) itself, so shadows of terrain behind
//! the camera stay correct and every view sees the same geometry.

use glam::{DVec2, DVec3};

/// Quads along a patch edge.
pub const PATCH_QUADS: u32 = 32;
/// The finest LOD: two detail-displacement levels below the 0.5 m bake.
pub const MIN_LOD: i8 = -2;
/// The coarsest LOD (256 m spacing, 8 km nodes): matches the 10-level height clipmap.
pub const MAX_LOD: i8 = 9;

/// A selected patch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeKey {
    pub lod: i8,
    /// Node index in the LOD's node grid (node `x` starts at level-0 global
    /// sample `x · node_samples(lod)`).
    pub x: i64,
    pub z: i64,
    /// `None`: the whole node. `Some(q)`: one quarter of it, `q` = bit 0 for
    /// +x, bit 1 for +z (the order of [`crate::TileCoord::children`]).
    pub quadrant: Option<u8>,
}

impl NodeKey {
    /// Edge of a whole node in level-0 global sample units (8 at LOD −2).
    pub fn node_samples(lod: i8) -> f64 {
        f64::from(PATCH_QUADS) * 2f64.powi(i32::from(lod))
    }

    /// Vertex spacing in meters.
    pub fn spacing(lod: i8) -> f32 {
        0.5 * 2f32.powi(i32::from(lod))
    }

    /// Continuous level-0 global sample coordinates of the patch's (0, 0)
    /// vertex (exact: powers of two).
    pub fn first_sample(self) -> DVec2 {
        let size = Self::node_samples(self.lod);
        #[allow(clippy::cast_precision_loss)]
        let mut g = DVec2::new(self.x as f64, self.z as f64) * size;
        if let Some(q) = self.quadrant {
            g += DVec2::new(f64::from(q & 1), f64::from(q >> 1)) * size * 0.5;
        }
        g
    }

    /// Quads along the drawn patch's edge (half for a quadrant).
    pub fn quads(self) -> u32 {
        if self.quadrant.is_some() { PATCH_QUADS / 2 } else { PATCH_QUADS }
    }
}

/// LOD ranges and morph zones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CdlodSettings {
    /// Range of LOD 0 in meters; LOD ℓ reaches `lod0_range · 2^ℓ`.
    pub lod0_range: f32,
    /// Fraction of each LOD's band `[range(ℓ−1), range(ℓ)]` over which it
    /// morphs into LOD ℓ+1.
    pub morph_fraction: f32,
    pub min_lod: i8,
    pub max_lod: i8,
}

impl Default for CdlodSettings {
    fn default() -> Self {
        Self { lod0_range: 48.0, morph_fraction: 0.3, min_lod: MIN_LOD, max_lod: MAX_LOD }
    }
}

impl CdlodSettings {
    /// Range of `lod` in meters.
    pub fn range(&self, lod: i8) -> f32 {
        self.lod0_range * 2f32.powi(i32::from(lod))
    }

    /// (morph start, morph end) distances of `lod`, in meters.
    pub fn morph_zone(&self, lod: i8) -> (f32, f32) {
        let end = self.range(lod);
        let start = end - self.morph_fraction * (end - end * 0.5);
        (start, end)
    }

    /// The view distance: the coarsest LOD's range.
    pub fn view_distance(&self) -> f32 {
        self.range(self.max_lod)
    }
}

/// A node chosen for drawing, with the height bounds it was tested with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectedNode {
    pub key: NodeKey,
    pub min_height: f32,
    pub max_height: f32,
}

/// Height bounds of a whole node: `(lod, x, z) -> (min, max)` in meters.
/// Must be conservative (cover every finer level's samples in the node).
pub type NodeBounds<'a> = dyn Fn(i8, i64, i64) -> (f32, f32) + 'a;

/// Selects the nodes to draw for a camera at `camera` (x, z: level-0
/// global sample coordinates; y: meters). Appends to `out` in a stable
/// order (depth-first, children in `TileCoord::children` order).
pub fn select(camera: DVec3, settings: &CdlodSettings, bounds: &NodeBounds<'_>, out: &mut Vec<SelectedNode>) {
    let top = settings.max_lod;
    let size = NodeKey::node_samples(top);
    let reach = f64::from(settings.range(top)) * 2.0; // meters to samples
    #[allow(clippy::cast_possible_truncation)]
    let span = |c: f64| ((c - reach) / size).floor() as i64..=((c + reach) / size).floor() as i64;
    let cx = DVec2::new(camera.x, camera.z);
    for z in span(cx.y) {
        for x in span(cx.x) {
            visit(camera, settings, bounds, top, x, z, out);
        }
    }
}

/// Squared distance in meters from the camera to a node's box.
fn box_distance_sq(camera: DVec3, lod: i8, x: i64, z: i64, (lo, hi): (f32, f32)) -> f64 {
    let size = NodeKey::node_samples(lod);
    #[allow(clippy::cast_precision_loss)]
    let (x0, z0) = (x as f64 * size, z as f64 * size);
    // Per-axis distance outside [a, b], with x/z converted from samples to meters.
    let axis = |c: f64, a: f64, b: f64| {
        if c < a {
            a - c
        } else if c > b {
            c - b
        } else {
            0.0
        }
    };
    let dx = axis(camera.x, x0, x0 + size) * 0.5;
    let dz = axis(camera.z, z0, z0 + size) * 0.5;
    let dy = axis(camera.y, f64::from(lo), f64::from(hi));
    dx * dx + dy * dy + dz * dz
}

/// Strugar's `LODSelect`: returns false when the node is outside its own
/// range (the parent then covers its area).
fn visit(
    camera: DVec3,
    settings: &CdlodSettings,
    bounds: &NodeBounds<'_>,
    lod: i8,
    x: i64,
    z: i64,
    out: &mut Vec<SelectedNode>,
) -> bool {
    let b = bounds(lod, x, z);
    let d2 = box_distance_sq(camera, lod, x, z, b);
    let range = f64::from(settings.range(lod));
    if d2 > range * range {
        return false;
    }
    let whole = |out: &mut Vec<SelectedNode>| {
        out.push(SelectedNode { key: NodeKey { lod, x, z, quadrant: None }, min_height: b.0, max_height: b.1 });
    };
    if lod <= settings.min_lod {
        whole(out);
        return true;
    }
    let finer = f64::from(settings.range(lod - 1));
    if d2 > finer * finer {
        whole(out);
        return true;
    }
    for q in 0..4u8 {
        let (cx, cz) = (x * 2 + i64::from(q & 1), z * 2 + i64::from(q >> 1));
        if !visit(camera, settings, bounds, lod - 1, cx, cz, out) {
            out.push(SelectedNode { key: NodeKey { lod, x, z, quadrant: Some(q) }, min_height: b.0, max_height: b.1 });
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(_: i8, _: i64, _: i64) -> (f32, f32) {
        (0.0, 0.0)
    }

    /// Area of the selection in level-0 samples², per drawn patch.
    fn area(nodes: &[SelectedNode]) -> f64 {
        nodes
            .iter()
            .map(|n| {
                let s = NodeKey::node_samples(n.key.lod) * if n.key.quadrant.is_some() { 0.5 } else { 1.0 };
                s * s
            })
            .sum()
    }

    #[test]
    fn selection_covers_the_roots_without_overlap() {
        let settings = CdlodSettings::default();
        let mut out = Vec::new();
        select(DVec3::new(1000.0, 3.0, -400.0), &settings, &flat, &mut out);
        // Every root that is in range is covered exactly once.
        let mut roots = Vec::new();
        select(DVec3::new(1000.0, 3.0, -400.0), &CdlodSettings { min_lod: MAX_LOD, ..settings }, &flat, &mut roots);
        assert!((area(&out) - area(&roots)).abs() < 1e-6, "{} vs {}", area(&out), area(&roots));
        assert!(out.iter().any(|n| n.key.lod == MIN_LOD));
        assert!(out.len() < 3000, "{} nodes", out.len());
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn neighbors_differ_by_at_most_one_lod() {
        let settings = CdlodSettings::default();
        let mut out = Vec::new();
        let hills = |lod: i8, x: i64, _z: i64| {
            let h = (x as f32 * 0.37 * f32::from(lod + 3)).sin() * 40.0;
            (h - 50.0, h + 50.0)
        };
        select(DVec3::new(10.0, 120.0, 10.0), &settings, &hills, &mut out);
        // Rasterize the LOD of each drawn area on a coarse grid around the camera and compare neighbors.
        let cell = NodeKey::node_samples(MIN_LOD) * 0.5;
        let lod_at = |p: DVec2| {
            out.iter()
                .find(|n| {
                    let o = n.key.first_sample();
                    let s = NodeKey::node_samples(n.key.lod) * if n.key.quadrant.is_some() { 0.5 } else { 1.0 };
                    p.x >= o.x && p.x < o.x + s && p.y >= o.y && p.y < o.y + s
                })
                .map(|n| n.key.lod)
        };
        for j in -200..200 {
            for i in -200..200 {
                let p = DVec2::new(f64::from(i) * cell * 3.0 + 0.1, f64::from(j) * cell * 3.0 + 0.1);
                let (Some(a), Some(b)) = (lod_at(p), lod_at(p + DVec2::new(cell * 3.0, 0.0))) else { continue };
                assert!((a - b).abs() <= 1, "LOD {a} next to {b} at {p}");
            }
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn morph_zones_end_at_the_range() {
        let s = CdlodSettings::default();
        let (a, b) = s.morph_zone(0);
        assert_eq!(b, 48.0);
        assert!((a - (48.0 - 0.3 * 24.0)).abs() < 1e-4);
    }
}
