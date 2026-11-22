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

/// Where the raised cover's distance fade starts, as a multiple of LOD 0's range
/// ([`CdlodSettings::raise_fade`]; `common.wesl` `RAISE_FADE_START`): 384 m at the default 48 m,
/// where the 0.45 m knee-deep cover subtends ~1.5 output pixels at 1440p (SW-5c).
pub const RAISE_FADE_START: f32 = 8.0;
/// Where it reaches 0 (`RAISE_FADE_END`, 576 m): sub-pixel, inside [`RAISE_MAX_LOD`]'s range, and
/// inside the terrain's splat range (from 700 m; the raise's high point, `layer_raise_top`, reads no distance).
pub const RAISE_FADE_END: f32 = 12.0;
/// The coarsest LOD the raise displaces (`common.wesl` `RAISE_MAX_LOD`): every coarser node lies
/// past this one's range, beyond the fade's end (SW-5c; was LOD 0 with a 43 m fade).
pub const RAISE_MAX_LOD: i8 = 4;

/// LOD ranges and morph zones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CdlodSettings {
    /// Range of LOD 0 in meters; LOD ℓ reaches `lod0_range · 2^ℓ`.
    pub lod0_range: f32,
    /// Fraction of each LOD's band `[range(ℓ−1), range(ℓ)]` over which it
    /// morphs into LOD ℓ+1 (the host's `morph_start` tunable sets it through
    /// [`Self::with_morph_start`]; the floor is [`Self::crack_free_morph_start`]).
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

    /// Where each LOD starts morphing, as a fraction of its range (`1 - morph_fraction / 2`: the
    /// band is the range's outer half).
    pub fn morph_start(&self) -> f32 {
        1.0 - 0.5 * self.morph_fraction
    }

    /// These settings morphing from `start` x each LOD's range (clamped to the band's inner edge).
    #[must_use]
    pub fn with_morph_start(self, start: f32) -> Self {
        Self { morph_fraction: (2.0 * (1.0 - start)).clamp(0.0, 1.0), ..self }
    }

    /// The smallest morph start (fraction of the range) that stays crack-free by construction over
    /// ground no steeper than `relief` (m/m) with a selection kept while the camera moves up to
    /// `slack_m` (the selection gate's quantum). Strugar's condition: where a LOD `l` patch meets an
    /// `l + 1` one, `l + 1` must not have started morphing. The `l` node's box reaches into `range(l)`,
    /// so the shared edge lies at most its 3D diagonal (`size·√2·√(1 + relief²)`) beyond, and the
    /// stale camera adds `slack_m` both to that distance and to the shader's zones (`morph_zone`
    /// moved closer): `start(l + 1) ≥ range(l) + diagonal + 2·slack`. The worst LOD is the finest
    /// (the slack weighs most against its range). A bound over worst-case camera placements: the
    /// measured floor (`sf_terrain_bevy` `measure_morph_start_floor`) sits below it.
    pub fn crack_free_morph_start(&self, relief: f32, slack_m: f32) -> f32 {
        (self.min_lod..self.max_lod)
            .map(|lod| {
                #[allow(clippy::cast_possible_truncation)]
                let size = (NodeKey::node_samples(lod) * 0.5) as f32;
                let diagonal = size * std::f32::consts::SQRT_2 * (1.0 + relief * relief).sqrt();
                (self.range(lod) + diagonal + 2.0 * slack_m) / self.range(lod + 1)
            })
            .fold(0.5, f32::max)
    }

    /// (morph start, morph end) distances of `lod`, in meters.
    pub fn morph_zone(&self, lod: i8) -> (f32, f32) {
        let end = self.range(lod);
        let start = end - self.morph_fraction * (end - end * 0.5);
        (start, end)
    }

    /// Weight of the raised cover (surface weather W6b, SW-5c; `common.wesl` `raise_fade`) at
    /// `d` meters from the camera (the vertex's 3D distance, as the morph uses), with no reach
    /// limit from outside: [`Self::raise_fade_within`] at an infinite reach.
    pub fn raise_fade(&self, d: f32) -> f32 {
        self.raise_fade_within(d, f32::INFINITY)
    }

    /// The raise's fade `(start, end)` in meters (SW-5e): it ends at [`RAISE_FADE_END`] × LOD 0's
    /// range (where the cover is sub-pixel) or at the host's `reach_m`, whichever is nearer, and
    /// starts at [`RAISE_FADE_START`] / [`RAISE_FADE_END`] of that end. `reach_m` is how far the
    /// cover's inputs are known whatever the camera's place (surface weather's shelter levels and
    /// the casters that fill them, `sf_surface::view::raise_reach_m`): past it a texel's cover
    /// would change as the camera neared, so the raise is already 0 there.
    pub fn raise_fade_range(&self, reach_m: f32) -> (f32, f32) {
        let end = (RAISE_FADE_END * self.lod0_range).min(reach_m.max(0.0));
        (end * (RAISE_FADE_START / RAISE_FADE_END), end)
    }

    /// Weight of the raised cover at `d` meters with the reach limit `reach_m`: 1 out to the
    /// fade's start ([`Self::raise_fade_range`]), a smoothstep down to exactly 0 at its end and
    /// beyond. It scales every ground point alike (open and sheltered): no relative pit near the
    /// camera. The raise is displaced on LOD ≤ [`RAISE_MAX_LOD`]; every coarser vertex lies past
    /// that LOD's range, beyond the fade's end (which a reach limit only brings nearer), so no
    /// seam or step remains.
    pub fn raise_fade_within(&self, d: f32, reach_m: f32) -> f32 {
        let (a, b) = self.raise_fade_range(reach_m);
        if d >= b {
            return 0.0;
        }
        let t = ((d - a) / (b - a)).clamp(0.0, 1.0);
        1.0 - t * t * (3.0 - 2.0 * t)
    }

    /// The raise weight of a vertex of `lod` at `d` meters (`common.wesl` `terrain_vertex`'s
    /// `raise_w`): [`Self::raise_fade`] on the displaced levels, 0 past [`RAISE_MAX_LOD`].
    pub fn raise_weight(&self, lod: i8, d: f32) -> f32 {
        if lod <= RAISE_MAX_LOD { self.raise_fade(d) } else { 0.0 }
    }

    /// The raised cover a vertex of `lod` at `d` meters displaces by (`common.wesl`
    /// `terrain_raise`): `field(filter_m)` is the cover's unweighted raise at the vertex's world
    /// point with its tops prefiltered to `filter_m` meters (`surface_raise_at`), and the vertex
    /// passes the view's one filter width (`filter_m`: SW-6's repose bound,
    /// `sf_surface::layer::raise_filter_m`, from the covers' depth and the repose tunables), never
    /// its own spacing or distance: the same world point raises alike on every LOD and at every
    /// distance inside the fade (SW-5c).
    pub fn vertex_raise(&self, lod: i8, d: f32, filter_m: f32, field: impl Fn(f32) -> f32) -> f32 {
        let w = self.raise_weight(lod, d);
        if w <= 0.0 {
            return 0.0;
        }
        w * field(filter_m)
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

    /// SW-5c: the raised cover's fade is 1 over every near LOD (past LOD 0's range), exactly 0
    /// at and beyond its end, which lies inside the coarsest displaced LOD's range (coarser nodes
    /// are never displaced, so no seam), monotone and continuous, with a slope far below W6b's
    /// (0.45 m over 25 m).
    #[test]
    #[allow(clippy::float_cmp)]
    fn raise_fade_reaches_zero_inside_the_displaced_levels() {
        let s = CdlodSettings::default();
        let (start, end) = (RAISE_FADE_START * s.lod0_range, RAISE_FADE_END * s.lod0_range);
        assert!(start > s.range(0), "full raise on every level to LOD 0 and past it");
        assert!(end < s.range(RAISE_MAX_LOD), "the fade ends inside the coarsest displaced LOD");
        assert!(end > s.range(RAISE_MAX_LOD - 1), "RAISE_MAX_LOD is the coarsest one needed");
        assert_eq!(s.raise_fade(0.0), 1.0);
        assert_eq!(s.raise_fade(start), 1.0);
        assert_eq!(s.raise_fade(end), 0.0);
        assert_eq!(s.raise_fade(s.range(RAISE_MAX_LOD)), 0.0);
        assert_eq!(s.raise_fade(1e5), 0.0);
        assert_eq!(s.raise_weight(RAISE_MAX_LOD + 1, 1.0), 0.0);
        let step = 0.1;
        let mut last = 1.0;
        let mut steepest: f32 = 0.0;
        for i in 0..8000u16 {
            let w = s.raise_fade(f32::from(i) * step);
            assert!(w <= last, "monotone at {}", f32::from(i) * step);
            steepest = steepest.max((last - w) / step);
            last = w;
        }
        // Max slope of a 0.45 m cover: 1.5 × depth / length (~0.35 %; W6b's was ~2.7 %).
        let knee = 0.45;
        assert!(steepest * knee < 0.004, "slope {}", steepest * knee);
    }

    /// SW-5e: a host reach limit only brings the fade nearer: it ends exactly at the reach (0 at
    /// and past it), starts at the same 2/3 of its end, stays inside the displaced levels, and an
    /// infinite reach is the unlimited fade.
    #[test]
    #[allow(clippy::float_cmp)]
    fn raise_fade_ends_at_the_reach_limit() {
        let s = CdlodSettings::default();
        assert_eq!(s.raise_fade_range(f32::INFINITY), (RAISE_FADE_START * s.lod0_range, RAISE_FADE_END * s.lod0_range));
        for reach in [120.0, 180.0, 270.0, 315.0, 360.0] {
            let (start, end) = s.raise_fade_range(reach);
            assert_eq!(end, reach);
            assert_eq!(start, reach * RAISE_FADE_START / RAISE_FADE_END);
            assert!(end < s.range(RAISE_MAX_LOD));
            assert_eq!(s.raise_fade_within(start, reach), 1.0);
            assert_eq!(s.raise_fade_within(end, reach), 0.0);
            assert_eq!(s.raise_fade_within(end + 1.0, reach), 0.0);
            assert!(s.raise_fade_within(end - 1.0, reach) > 0.0);
        }
        assert_eq!(s.raise_fade_within(0.0, 0.0), 0.0, "no reach: no raise");
        for d in [0.0, 200.0, 400.0, 500.0, 600.0] {
            assert_eq!(s.raise_fade(d), s.raise_fade_within(d, f32::INFINITY));
        }
    }

    /// SW-5c: a ground point's raise is a function of the world only. The same point, with a
    /// field whose tops rise as they are prefiltered coarser (the bury's concave clamp,
    /// `sw5-diag` cause 2), raises bit-equal at 5, 30 and 100 m from the camera and on LOD
    /// -2..4 (vertex spacing 0.125 to 8 m) and, SW-6, at the repose bound's filter width (4.28 m
    /// for 0.45 m of snow at 30 deg) as at SW-5c's 0.5 m; prefiltered to each LOD's own spacing
    /// (W6b), the same point did not.
    #[test]
    fn raise_is_the_same_on_every_lod_and_distance() {
        let s = CdlodSettings::default();
        // A sheltered snow point (amount 0.8 under a 0.45 m cover) whose top rises 0.04 per mip
        // of the 0.25 m drift atlas (`surface_raise_tops`), through SW-6's smoothstep bury.
        let field = |filter_m: f32| {
            let mip = (filter_m / 0.25).log2().max(0.0);
            let top = 0.04f32.mul_add(mip, 0.4);
            let x = ((0.8 - top) / (1.0 - top.min(0.6))).clamp(0.0, 1.0);
            x * x * (3.0 - 2.0 * x) * 0.45
        };
        for filter_m in [0.5, 4.28] {
            let reference = s.vertex_raise(0, 5.0, filter_m, field);
            assert!(reference > 0.0);
            let mut old = Vec::new();
            for lod in -2..=RAISE_MAX_LOD {
                let spacing = 0.5 * 2f32.powi(i32::from(lod));
                for d in [5.0, 30.0, 100.0, 300.0] {
                    let r = s.vertex_raise(lod, d, filter_m, field);
                    assert_eq!(r.to_bits(), reference.to_bits(), "LOD {lod} at {d} m, filter {filter_m} m");
                    old.push((s.raise_weight(lod, d) * field(spacing)).to_bits());
                }
            }
            old.dedup();
            assert!(old.len() > 1, "the per-LOD prefilter did change the raise");
        }
    }
}
