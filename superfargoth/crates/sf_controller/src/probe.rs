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

    /// A contact query ([`GroundProbe::contacts`]): the upright capsule at `origin` and
    /// whatever moving body comes within `margin` of it. It goes nowhere (`dir` zero).
    pub fn contact(origin: WorldPos, margin: f32, radius: f32, half_height: f32) -> Self {
        Self { origin, dir: Vec3::ZERO, max: margin, radius, half_height }
    }

    /// Straight down within 0.1°.
    pub fn is_down(&self) -> bool {
        self.dir.y < -0.999_998
    }
}

/// The body a cast hit, as the probe's physics layer names it (opaque here:
/// `sf_physics` maps its `BodyId` to and from these bits). The mover keeps it
/// to stand on a moving base (UE `MovementBase`, Jolt `CharacterVirtual`
/// ground body).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GroundBody(pub u64);

/// The first contact of a cast.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroundHit {
    /// Contact point on the surface.
    pub point: WorldPos,
    /// Surface normal at the contact, unit, facing the caster.
    pub normal: Vec3,
    /// Distance the cast travelled before touching (≤ `max`).
    pub distance: f32,
    pub surface: SurfaceId,
    /// The body hit; `None` for ground without one (height fields, the
    /// static collision world). Runtime only, like the two velocities: a body
    /// id means nothing in another session, so saves skip all three and a
    /// loaded hit is static until the next probe.
    #[serde(skip)]
    pub body: Option<GroundBody>,
    /// Velocity of the ground at `point`, m/s (`v + ω × (point − com)`):
    /// zero for static ground. A kinematic body's is its last tick's pose
    /// delta over the tick, rotation included.
    #[serde(skip)]
    pub velocity: Vec3,
    /// Angular velocity of the ground body, rad/s, world axes (zero for static).
    #[serde(skip)]
    pub angular_velocity: Vec3,
}

impl GroundHit {
    /// A hit on static ground: no body, no velocity.
    pub fn new(point: WorldPos, normal: Vec3, distance: f32, surface: SurfaceId) -> Self {
        Self { point, normal, distance, surface, body: None, velocity: Vec3::ZERO, angular_velocity: Vec3::ZERO }
    }

    /// The same hit on `body`, moving with point velocity `velocity` (m/s) and
    /// angular velocity `angular_velocity` (rad/s).
    #[must_use]
    pub fn on_body(self, body: GroundBody, velocity: Vec3, angular_velocity: Vec3) -> Self {
        Self { body: Some(body), velocity, angular_velocity, ..self }
    }

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

    /// Moving bodies against characters (slice `IN11c1`; Jolt `CharacterVirtual` collects its
    /// contacts each update, moving ones with their point velocity, and keeps the character
    /// out of them, so a kinematic wall pushes it). `out[i]` receives the nearest contact
    /// of a moving body (one with a velocity: static geometry never pushes, the movers'
    /// sweeps handle it) within `queries[i].max` of the upright capsule `queries[i]`
    /// ([`ProbeQuery::contact`]), at the body's pose after its motion this tick, as a
    /// [`GroundHit`] whose `normal` is that of the face that moved toward the capsule this
    /// tick (from the body's motion, not the shallowest way out: a fast thin wall that ended
    /// past the capsule's centre still reports the face it came in by, so it pushes the
    /// character ahead of it instead of tunnelling) and whose `distance` is the signed
    /// separation along it, negative when overlapping. The default reports nothing (no
    /// moving bodies: the character is never pushed).
    fn contacts(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        let _ = queries;
        out.fill(None);
    }
}

impl<T: GroundProbe + ?Sized> GroundProbe for &T {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        (**self).probe(queries, out);
    }
    fn contacts(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        (**self).contacts(queries, out);
    }
}

/// Several probes composed by taking the nearest hit (terrain + statics).
pub struct Nearest<'a>(pub &'a [&'a dyn GroundProbe]);

impl Nearest<'_> {
    fn nearest(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>], cast: impl Fn(&dyn GroundProbe, &mut [Option<GroundHit>])) {
        out.fill(None);
        let mut scratch = vec![None; queries.len()];
        for p in self.0 {
            cast(*p, &mut scratch);
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

impl GroundProbe for Nearest<'_> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.nearest(queries, out, |p, scratch| p.probe(queries, scratch));
    }
    fn contacts(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        self.nearest(queries, out, |p, scratch| p.contacts(queries, scratch));
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
            return Some(GroundHit::new(point, n, distance, s.surface(point)));
        }
        let (distance, normal) = s.raycast(q.origin, q.dir, q.max)?;
        let point = q.origin.offset(q.dir * distance);
        let normal = if normal.dot(q.dir) > 0.0 { -normal } else { normal };
        Some(GroundHit::new(point, normal, distance, s.surface(point)))
    }
}

impl<T: HeightSurface> GroundProbe for TerrainProbe<T> {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = self.probe_query(q);
        }
    }
}
