//! Ground probes: batched ray, sphere and capsule casts that the mover and
//! foot placement share (locomotion.md "Ground probes").

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_coords::WorldPos;

/// Surface material of a hit (stone, wood, sand…), for footsteps and foot
/// placement. Values are adapter-defined; 0 is the default surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SurfaceId(pub u16);

impl SurfaceId {
    pub const DEFAULT: Self = Self(0);
}

/// One cast. `radius` 0 is a ray, `half_height` 0 a sphere, otherwise an
/// upright (Y-axis) capsule centered on `origin`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbeQuery {
    pub origin: WorldPos,
    /// Normalized.
    pub dir: Vec3,
    /// Meters along `dir`.
    pub max: f32,
    pub radius: f32,
    pub half_height: f32,
}

impl ProbeQuery {
    pub fn ray(origin: WorldPos, dir: Vec3, max: f32) -> Self {
        Self { origin, dir, max, radius: 0.0, half_height: 0.0 }
    }

    /// A ray straight down.
    pub fn down(origin: WorldPos, max: f32) -> Self {
        Self::ray(origin, Vec3::NEG_Y, max)
    }

    pub fn sphere(origin: WorldPos, dir: Vec3, max: f32, radius: f32) -> Self {
        Self { origin, dir, max, radius, half_height: 0.0 }
    }

    pub fn capsule(origin: WorldPos, dir: Vec3, max: f32, radius: f32, half_height: f32) -> Self {
        Self { origin, dir, max, radius, half_height }
    }

    /// Straight down within 0.1°.
    pub fn is_down(&self) -> bool {
        self.dir.y < -0.999_998
    }
}

/// The first contact of a cast.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroundHit {
    /// Contact point on the surface.
    pub point: WorldPos,
    /// Surface normal at the contact, unit, facing the caster.
    pub normal: Vec3,
    /// Distance the cast travelled before touching (≤ `max`).
    pub distance: f32,
    pub surface: SurfaceId,
}

impl GroundHit {
    /// Walkable for a mover whose slope limit has this cosine. Walkability
    /// belongs to the mover, not the probe (it depends on the character).
    pub fn walkable(&self, min_normal_y: f32) -> bool {
        self.normal.y >= min_normal_y
    }
}

/// Answers casts. Batched: one call per stage per frame for every
/// character, so implementations can amortize setup and run in parallel.
pub trait GroundProbe: Send + Sync {
    /// `out[i]` receives the nearest hit of `queries[i]`, if any.
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]);

    fn probe_one(&self, query: ProbeQuery) -> Option<GroundHit> {
        let mut out = [None];
        self.probe(&[query], &mut out);
        out[0]
    }
}

impl<T: GroundProbe + ?Sized> GroundProbe for &T {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        (**self).probe(queries, out);
    }
}

/// Several probes composed by taking the nearest hit (terrain + statics).
pub struct Nearest<'a>(pub &'a [&'a dyn GroundProbe]);

impl GroundProbe for Nearest<'_> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        out.fill(None);
        let mut scratch = vec![None; queries.len()];
        for p in self.0 {
            p.probe(queries, &mut scratch);
            for (o, s) in out.iter_mut().zip(&scratch) {
                if let Some(s) = s
                    && o.is_none_or(|o| s.distance < o.distance)
                {
                    *o = Some(*s);
                }
            }
        }
    }
}

/// A height field: terrain or anything shaped like it. Implemented for
/// `sf_terrain::TerrainHeights` with the `terrain` feature.
pub trait HeightSurface: Send + Sync {
    /// Surface height (absolute Y, meters) under `p`, if known there.
    fn height(&self, p: WorldPos) -> Option<f32>;
    /// Unit surface normal under `p`.
    fn normal(&self, p: WorldPos) -> Option<Vec3>;
    /// First crossing along a normalized `dir` within `max`: (distance, normal).
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)>;
    fn surface(&self, _p: WorldPos) -> SurfaceId {
        SurfaceId::DEFAULT
    }
}

impl<T: HeightSurface + ?Sized> HeightSurface for &T {
    fn height(&self, p: WorldPos) -> Option<f32> {
        (**self).height(p)
    }
    fn normal(&self, p: WorldPos) -> Option<Vec3> {
        (**self).normal(p)
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        (**self).raycast(origin, dir, max)
    }
    fn surface(&self, p: WorldPos) -> SurfaceId {
        (**self).surface(p)
    }
}

/// Probes a [`HeightSurface`].
///
/// Downward casts read the height and normal directly (no marching). A
/// downward sphere or capsule rests on the local tangent plane: its center
/// stops `radius / normal.y` above the surface, which is exact on planar
/// ground and close on terrain-scale curvature. Other directions are rays
/// ([`HeightSurface::raycast`]); a swept shape's radius is ignored there,
/// which only matters against near-vertical terrain the slope limit blocks
/// anyway.
pub struct TerrainProbe<T>(pub T);

impl<T: HeightSurface> TerrainProbe<T> {
    fn probe_query(&self, q: &ProbeQuery) -> Option<GroundHit> {
        let s = &self.0;
        if q.is_down() {
            let h = s.height(q.origin)?;
            let n = s.normal(q.origin).unwrap_or(Vec3::Y);
            #[allow(clippy::cast_possible_truncation)]
            let oy = q.origin.to_f64().y as f32;
            // The shape's lowest point is `half_height + radius` below its center
            // on flat ground; tilted, the sphere touches at `radius / n.y`.
            let lift = q.half_height + if q.radius > 0.0 { q.radius / n.y.max(0.05) } else { 0.0 };
            let distance = oy - h - lift;
            if !(0.0..=q.max).contains(&distance) {
                return None;
            }
            let center = q.origin.offset(Vec3::NEG_Y * (distance + q.half_height));
            let point = center.offset(-n * q.radius);
            return Some(GroundHit { point, normal: n, distance, surface: s.surface(point) });
        }
        let (distance, normal) = s.raycast(q.origin, q.dir, q.max)?;
        let point = q.origin.offset(q.dir * distance);
        let normal = if normal.dot(q.dir) > 0.0 { -normal } else { normal };
        Some(GroundHit { point, normal, distance, surface: s.surface(point) })
    }
}

impl<T: HeightSurface> GroundProbe for TerrainProbe<T> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.probe_query(q);
        }
    }
}
