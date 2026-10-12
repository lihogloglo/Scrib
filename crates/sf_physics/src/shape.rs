//! Shapes bodies are made of. Cooked shapes are built once per model and
//! shared by every placement through an `Arc` (one parry `TriMesh` per model,
//! like `sf_controller::StaticShape`).

use std::sync::{Arc, Mutex};

use glam::Vec3;
use parry3d::utils::Array2;
use rapier3d::geometry::SharedShape;
use sf_controller::{CollisionMesh, SurfaceId};

/// A `collision.cook` mesh ready for the engine: a parry `TriMesh` with a
/// surface per triangle. Scaled placements get scaled copies, built once per
/// distinct scale and cached here.
pub struct MeshShape {
    shape: SharedShape,
    surfaces: Vec<SurfaceId>,
    scaled: Mutex<Vec<([u32; 3], SharedShape)>>,
}

impl MeshShape {
    /// `None` for an empty mesh or one parry rejects.
    pub fn new(mesh: &CollisionMesh) -> Option<Arc<Self>> {
        if mesh.is_empty() {
            return None;
        }
        let vertices = mesh.positions.iter().map(|p| Vec3::from_array(*p)).collect();
        let shape = SharedShape::trimesh(vertices, mesh.triangles.clone()).ok()?;
        Some(Arc::new(Self { shape, surfaces: mesh.surfaces.clone(), scaled: Mutex::new(Vec::new()) }))
    }

    pub fn triangle_count(&self) -> usize {
        self.shape.as_trimesh().map_or(0, parry3d::shape::TriMesh::num_triangles)
    }

    pub(crate) fn surface(&self, triangle: u32) -> SurfaceId {
        self.surfaces.get(triangle as usize).copied().unwrap_or_default()
    }

    /// The engine shape at `scale`: the shared one at unit scale, else a cached scaled copy.
    pub(crate) fn shared(&self, scale: Vec3) -> SharedShape {
        if (scale - Vec3::ONE).abs().max_element() <= 1e-6 {
            return self.shape.clone();
        }
        let key = scale.to_array().map(f32::to_bits);
        let mut cache = self.scaled.lock().expect("mesh shape cache poisoned");
        if let Some((_, s)) = cache.iter().find(|(k, _)| *k == key) {
            return s.clone();
        }
        let mesh = self.shape.as_trimesh().expect("MeshShape holds a trimesh").clone().scaled(scale);
        let s = SharedShape::new(mesh);
        cache.push((key, s.clone()));
        s
    }
}

/// A square terrain chunk as a heightfield: `n × n` samples `spacing` meters
/// apart, `heights[z * n + x]` (absolute Y in meters), x and z ascending from
/// the chunk's minimum corner. The body's pose is the chunk's center at Y 0.
pub struct HeightShape {
    shape: SharedShape,
    pub n: usize,
    pub spacing: f32,
}

impl HeightShape {
    /// `None` when `heights.len() != n * n` or `n < 2`.
    pub fn new(n: usize, spacing: f32, heights: &[f32]) -> Option<Arc<Self>> {
        if n < 2 || heights.len() != n * n {
            return None;
        }
        // parry: rows advance along Z, columns along X, column-major storage.
        let mut zx = Array2::zeros(n, n);
        for z in 0..n {
            for x in 0..n {
                zx[(z, x)] = heights[z * n + x];
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let extent = spacing * (n - 1) as f32;
        let shape = SharedShape::heightfield(zx, Vec3::new(extent, 1.0, extent));
        Some(Arc::new(Self { shape, n, spacing }))
    }

    /// Edge length in meters.
    #[allow(clippy::cast_precision_loss)]
    pub fn extent(&self) -> f32 {
        self.spacing * (self.n - 1) as f32
    }
}

/// The shape of a body. Sizes in meters; `Box` takes half extents.
#[derive(Clone)]
pub enum ShapeRef {
    Mesh(Arc<MeshShape>),
    Heightfield(Arc<HeightShape>),
    /// Upright (Y axis) capsule: radius, half height of the cylinder part.
    Capsule { r: f32, half: f32 },
    Box(Vec3),
    Sphere(f32),
}

impl ShapeRef {
    pub(crate) fn shared(&self, scale: Vec3) -> SharedShape {
        match self {
            Self::Mesh(m) => m.shared(scale),
            Self::Heightfield(h) => h.shape.clone(),
            // Primitives take the placement's scale by size (uniform part for round shapes).
            Self::Capsule { r, half } => {
                let s = scale.x.max(scale.z);
                SharedShape::capsule_y(half * scale.y, r * s)
            }
            Self::Box(h) => {
                let h = *h * scale;
                SharedShape::cuboid(h.x, h.y, h.z)
            }
            Self::Sphere(r) => SharedShape::ball(r * scale.max_element()),
        }
    }
}
