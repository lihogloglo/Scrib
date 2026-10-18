//! The CPU water query (water.md "The water query"): bodies in a 2D BVH per worldspace, priority
//! for overlaps, the ocean's wave twin, depth from the bed. Shared by rendering, buoyancy, swimming,
//! wetness, sound and AI.

use std::collections::HashMap;
use std::sync::Arc;

use glam::{DVec2, Vec3};
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_terrain::TerrainHeights;
use sf_world::WorldspaceId;

use crate::records::{WaterBody, WaterKind, WaterShape};
use crate::twin::WaveTwin;

/// The query result.
#[derive(Clone, Debug, PartialEq)]
pub struct WaterSample {
    pub body: RecordId,
    pub kind: WaterKind,
    /// Displaced surface at this XZ, world Y.
    pub height: f64,
    pub normal: Vec3,
    /// Flow + wave orbital velocity, m/s.
    pub velocity: Vec3,
    /// Surface − bed, metres; infinite where the bed is unknown (no resident terrain, interiors).
    pub depth: f32,
}

/// A vertical-ish capsule (actors).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Capsule {
    /// Segment end points (centres of the end spheres).
    pub a: WorldPos,
    pub b: WorldPos,
    pub radius: f32,
}

/// How deep a capsule is in water.
#[derive(Clone, Debug, PartialEq)]
pub struct Immersion {
    /// Surface height above the capsule's lowest point, metres (≤ 0: dry).
    pub depth: f32,
    /// Submerged fraction of the capsule's height, 0..1.
    pub fraction: f32,
    /// The water at the capsule's axis, when there is any.
    pub sample: Option<WaterSample>,
}

/// Bed heights under the water (terrain), world Y.
pub trait BedHeights: Send + Sync {
    fn bed_height(&self, at: WorldPos) -> Option<f32>;
}

impl BedHeights for TerrainHeights {
    fn bed_height(&self, at: WorldPos) -> Option<f32> {
        self.height(at)
    }
}

/// The query contract.
pub trait WaterQuery {
    /// The water column at `at`'s XZ (its Y is ignored), in the state of the last step.
    fn sample(&self, ws: &WorldspaceId, at: WorldPos) -> Option<WaterSample>;
    /// `sample` for many points (buoyancy pontoons).
    fn sample_many(&self, ws: &WorldspaceId, at: &[WorldPos], out: &mut [Option<WaterSample>]) {
        for (p, o) in at.iter().zip(out.iter_mut()) {
            *o = self.sample(ws, *p);
        }
    }
    fn immersion(&self, ws: &WorldspaceId, capsule: Capsule) -> Immersion {
        let (lo, hi) = if capsule.a.delta(capsule.b).y >= 0.0 {
            (capsule.a, capsule.b)
        } else {
            (capsule.b, capsule.a)
        };
        let bottom = lo.to_f64().y - f64::from(capsule.radius);
        let top = hi.to_f64().y + f64::from(capsule.radius);
        let sample = self.sample(ws, lo);
        let Some(s) = &sample else {
            return Immersion {
                depth: 0.0,
                fraction: 0.0,
                sample,
            };
        };
        #[allow(clippy::cast_possible_truncation)]
        let depth = (s.height - bottom) as f32;
        #[allow(clippy::cast_possible_truncation)]
        let fraction = ((s.height - bottom) / (top - bottom).max(1e-6)).clamp(0.0, 1.0) as f32;
        Immersion {
            depth,
            fraction,
            sample,
        }
    }
}

/// Axis-aligned XZ box.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Aabb2 {
    min: DVec2,
    max: DVec2,
}

impl Aabb2 {
    fn contains(&self, p: DVec2) -> bool {
        p.cmpge(self.min).all() && p.cmple(self.max).all()
    }

    fn union(self, o: Self) -> Self {
        Self {
            min: self.min.min(o.min),
            max: self.max.max(o.max),
        }
    }
}

enum Node {
    Leaf {
        bounds: Aabb2,
        items: Vec<u32>,
    },
    Inner {
        bounds: Aabb2,
        left: u32,
        right: u32,
    },
}

/// A 2D bounding volume hierarchy over the finite bodies (median split on the longer axis).
#[derive(Default)]
struct Bvh {
    nodes: Vec<Node>,
}

impl Bvh {
    const LEAF: usize = 4;

    fn build(items: &[(u32, Aabb2)]) -> Self {
        let mut bvh = Self::default();
        if !items.is_empty() {
            let mut v = items.to_vec();
            bvh.split(&mut v);
        }
        bvh
    }

    fn split(&mut self, items: &mut [(u32, Aabb2)]) -> u32 {
        let bounds = items
            .iter()
            .map(|i| i.1)
            .reduce(Aabb2::union)
            .expect("non-empty");
        let at = u32::try_from(self.nodes.len()).expect("node count");
        if items.len() <= Self::LEAF {
            self.nodes.push(Node::Leaf {
                bounds,
                items: items.iter().map(|i| i.0).collect(),
            });
            return at;
        }
        let ext = bounds.max - bounds.min;
        let axis = usize::from(ext.y > ext.x);
        items.sort_by(|a, b| (a.1.min + a.1.max)[axis].total_cmp(&(b.1.min + b.1.max)[axis]));
        self.nodes.push(Node::Leaf {
            bounds,
            items: Vec::new(),
        }); // placeholder
        let mid = items.len() / 2;
        let (l, r) = items.split_at_mut(mid);
        let left = self.split(l);
        let right = self.split(r);
        self.nodes[at as usize] = Node::Inner {
            bounds,
            left,
            right,
        };
        at
    }

    fn query(&self, p: DVec2, out: &mut impl FnMut(u32)) {
        if self.nodes.is_empty() {
            return;
        }
        let mut stack = [0u32; 64];
        let mut top = 1;
        while top > 0 {
            top -= 1;
            match &self.nodes[stack[top] as usize] {
                Node::Leaf { bounds, items } => {
                    if bounds.contains(p) {
                        items.iter().copied().for_each(&mut *out);
                    }
                }
                Node::Inner {
                    bounds,
                    left,
                    right,
                } => {
                    if bounds.contains(p) {
                        stack[top] = *left;
                        stack[top + 1] = *right;
                        top += 2;
                    }
                }
            }
        }
    }
}

/// Water of one worldspace.
#[derive(Default)]
pub struct WorldspaceWater {
    bodies: Vec<WaterBody>,
    /// Bodies everywhere (Infinite, Bounds), by priority, highest first.
    unbounded: Vec<u32>,
    bvh: Bvh,
    bed: Option<Arc<dyn BedHeights>>,
    waves: Option<WaveTwin>,
    shore: Option<Arc<crate::ShoreField>>,
}

impl WorldspaceWater {
    pub fn new(bodies: Vec<WaterBody>) -> Self {
        let mut unbounded = Vec::new();
        let mut boxed = Vec::new();
        for (i, b) in bodies.iter().enumerate() {
            let i = u32::try_from(i).expect("body count");
            match shape_bounds(&b.shape) {
                Some(bounds) => boxed.push((i, bounds)),
                None => unbounded.push(i),
            }
        }
        unbounded.sort_by_key(|i| std::cmp::Reverse(bodies[*i as usize].priority));
        Self {
            bvh: Bvh::build(&boxed),
            bodies,
            unbounded,
            bed: None,
            waves: None,
            shore: None,
        }
    }

    pub fn bodies(&self) -> &[WaterBody] {
        &self.bodies
    }

    pub fn set_bed(&mut self, bed: Option<Arc<dyn BedHeights>>) {
        self.bed = bed;
    }

    /// The ocean wave twin (sampled by bodies whose `waves.first_cascade` is 0).
    pub fn set_waves(&mut self, waves: Option<WaveTwin>) {
        self.waves = waves;
    }

    /// The ocean's shore product (`water.shore`): attenuates the twin's swell as the GPU does.
    pub fn set_shore(&mut self, shore: Option<Arc<crate::ShoreField>>) {
        self.shore = shore;
    }

    pub fn shore(&self) -> Option<&crate::ShoreField> {
        self.shore.as_deref()
    }

    pub fn waves(&self) -> Option<&WaveTwin> {
        self.waves.as_ref()
    }

    /// The body that owns `p` (highest priority containing it), with its rest height and flow there.
    fn body_at(&self, p: DVec2) -> Option<(u32, f64, Vec3)> {
        let mut best: Option<(u32, f64, Vec3)> = None;
        let mut consider = |i: u32| {
            let b = &self.bodies[i as usize];
            if best.is_some_and(|(j, ..)| self.bodies[j as usize].priority >= b.priority) {
                return;
            }
            if let Some((level, flow)) = contains(b, p) {
                best = Some((i, level, flow));
            }
        };
        self.bvh.query(p, &mut consider);
        // Unbounded bodies, highest priority first: the first one beats any lower-priority box.
        if let Some(&i) = self.unbounded.first() {
            consider(i);
        }
        best
    }

    #[allow(clippy::many_single_char_names)]
    pub fn sample(&self, at: WorldPos) -> Option<WaterSample> {
        let p = at.to_f64();
        let (i, level, flow) = self.body_at(DVec2::new(p.x, p.z))?;
        let body = &self.bodies[i as usize];
        let (mut height, mut normal, mut velocity) = (level, Vec3::Y, flow);
        if body.waves.first_cascade == 0
            && body.waves.amplitude > 0.0
            && let Some(w) = &self.waves
        {
            let s = w.sample(at);
            // The twin is cascade 0 (swell): the shore's swell scale, as on the GPU.
            let shore = self
                .shore
                .as_ref()
                .filter(|_| matches!(body.shape, WaterShape::Infinite))
                .map_or(1.0, |f| {
                    let (d, o) = f.sample(p.x, p.z);
                    f.swell_scale(d, o)
                });
            let scale = body.waves.amplitude * shore;
            height += f64::from(s.height * scale);
            normal = Vec3::Y.lerp(s.normal, shore).normalize_or(Vec3::Y);
            velocity += s.velocity * scale;
        }
        let depth = match &self.bed {
            Some(bed) => match bed.bed_height(at) {
                #[allow(clippy::cast_possible_truncation)]
                Some(b) => (height - f64::from(b)) as f32,
                None => f32::INFINITY,
            },
            None => f32::INFINITY,
        };
        // The ocean is "below the level and not cut by land": dry where the bed is above the surface.
        if depth <= 0.0 && matches!(body.shape, WaterShape::Infinite) {
            return None;
        }
        Some(WaterSample {
            body: body.id.clone(),
            kind: body.kind,
            height,
            normal,
            velocity,
            depth,
        })
    }
}

fn shape_bounds(shape: &WaterShape) -> Option<Aabb2> {
    let pts: Box<dyn Iterator<Item = (DVec2, f64)>> = match shape {
        WaterShape::Infinite | WaterShape::Bounds => return None,
        WaterShape::Polygon { outline, .. } => Box::new(outline.iter().map(|p| (*p, 0.0))),
        WaterShape::Ribbon { spline } => {
            Box::new(spline.iter().map(|p| (p.center, f64::from(p.width) * 0.5)))
        }
    };
    pts.map(|(p, r)| Aabb2 {
        min: p - r,
        max: p + r,
    })
    .reduce(Aabb2::union)
}

/// Even-odd point in polygon.
fn in_ring(ring: &[DVec2], p: DVec2) -> bool {
    let mut inside = false;
    let mut j = ring.len().wrapping_sub(1);
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[j]);
        if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Rest height and flow velocity of body `b` at `p`, if `p` is in it.
#[allow(clippy::cast_possible_truncation, clippy::many_single_char_names)]
fn contains(b: &WaterBody, p: DVec2) -> Option<(f64, Vec3)> {
    match &b.shape {
        WaterShape::Infinite | WaterShape::Bounds => Some((b.level, Vec3::ZERO)),
        WaterShape::Polygon { outline, holes } => (in_ring(outline, p)
            && !holes.iter().any(|h| in_ring(h, p)))
        .then_some((b.level, Vec3::ZERO)),
        WaterShape::Ribbon { spline } => {
            let mut best: Option<(f64, f64, Vec3)> = None; // distance², height, flow
            for w in spline.windows(2) {
                let (a, c) = (w[0], w[1]);
                let d = c.center - a.center;
                let t = ((p - a.center).dot(d) / d.length_squared().max(1e-12)).clamp(0.0, 1.0);
                let q = a.center + d * t;
                let dist2 = (p - q).length_squared();
                let half = f64::from(a.width + (c.width - a.width) * t as f32) * 0.5;
                if dist2 <= half * half && best.is_none_or(|bb| dist2 < bb.0) {
                    let dir = d.normalize_or_zero().as_vec2();
                    let speed = a.flow_speed + (c.flow_speed - a.flow_speed) * t as f32;
                    let h = a.height + (c.height - a.height) * t;
                    best = Some((dist2, h, Vec3::new(dir.x, 0.0, dir.y) * speed));
                }
            }
            best.map(|(_, h, f)| (h, f))
        }
    }
}

/// Every worldspace's water: the query the game shares.
#[derive(Default)]
pub struct WaterWorld {
    worldspaces: HashMap<WorldspaceId, WorldspaceWater>,
}

impl WaterWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, ws: WorldspaceId, water: WorldspaceWater) {
        self.worldspaces.insert(ws, water);
    }

    pub fn remove(&mut self, ws: &WorldspaceId) -> Option<WorldspaceWater> {
        self.worldspaces.remove(ws)
    }

    pub fn get(&self, ws: &WorldspaceId) -> Option<&WorldspaceWater> {
        self.worldspaces.get(ws)
    }

    pub fn get_mut(&mut self, ws: &WorldspaceId) -> Option<&mut WorldspaceWater> {
        self.worldspaces.get_mut(ws)
    }

    /// Advances every wave twin to time `t` seconds (the physics fixed step).
    pub fn step(&mut self, t: f64) {
        for w in self.worldspaces.values_mut() {
            if let Some(waves) = &mut w.waves {
                waves.evaluate(t);
            }
        }
    }
}

impl WaterQuery for WaterWorld {
    fn sample(&self, ws: &WorldspaceId, at: WorldPos) -> Option<WaterSample> {
        self.worldspaces.get(ws)?.sample(at)
    }
}
