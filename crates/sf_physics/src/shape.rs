//! Shapes bodies are made of. Cooked shapes are built once per model and
//! shared by every placement through an `Arc` (one parry `TriMesh` per model,
//! like `sf_controller::StaticShape`).

use std::sync::{Arc, Mutex};

use glam::Vec3;
use parry3d::utils::Array2;
use rapier3d::geometry::SharedShape;
use parry3d::query::{Ray, RayCast, RayCullingMode};
use parry3d::shape::{TriMesh, TriMeshFlags};
use sf_controller::{CollisionMesh, StaticShape, SurfaceId};

/// Static triangle meshes are one-sided (outward normals) with internal-edge
/// fixes: an item that starts sunk into a shelf or table deeper than half its
/// thickness is pushed out the top, not through (double-sided contacts push
/// it toward the nearer side), and bodies slide over the seams between
/// triangles without bumps (Bullet's internal-edge adjustment, Havok's
/// welding).
fn mesh_flags() -> TriMeshFlags {
    TriMeshFlags::ORIENTED | TriMeshFlags::FIX_INTERNAL_EDGES
}

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
        let tri = TriMesh::with_flags(vertices, mesh.triangles.clone(), mesh_flags());
        // Topology errors: two-sided, still a valid collider.
        let tri = tri.or_else(|_| TriMesh::new(mesh.positions.iter().map(|p| Vec3::from_array(*p)).collect(), mesh.triangles.clone())).ok()?;
        let shape = SharedShape::new(tri);
        Some(Arc::new(Self { shape, surfaces: mesh.surfaces.clone(), scaled: Mutex::new(Vec::new()) }))
    }

    /// The engine shape over a controller [`StaticShape`]'s mesh: a copy with
    /// [`mesh_flags`] (the controller's queries keep their two-sided mesh); one
    /// per model, shared by its placements. Build it off the main thread.
    pub fn from_static(shape: &StaticShape) -> Arc<Self> {
        let mut mesh: TriMesh = (**shape.trimesh()).clone();
        // Topology errors leave the mesh two-sided: still a valid collider.
        let _ = mesh.set_flags(mesh_flags());
        Arc::new(Self { shape: SharedShape::new(mesh), surfaces: shape.surfaces().to_vec(), scaled: Mutex::new(Vec::new()) })
    }

    /// Whether a model-space `point` lies inside the mesh as a closed volume:
    /// the nearest triangle straight below it (along `down`, model space) is a
    /// back face, i.e. faces away from the point. An authored collision proxy
    /// that is one box around a whole bookshelf or cabinet encloses the items
    /// placed on its shelves; an open or render-derived mesh puts a shelf's top
    /// face under them instead.
    pub fn encloses(&self, point: Vec3, down: Vec3) -> bool {
        let Some(mesh) = self.shape.as_trimesh() else { return false };
        let ray = Ray::new(point, down);
        let Some(any) = mesh.cast_local_ray(&ray, f32::MAX, false) else { return false };
        mesh.cast_local_ray_with_culling(&ray, f32::MAX, RayCullingMode::IgnoreFrontfaces)
            .is_some_and(|back| back.time_of_impact <= any + 1e-5)
    }

    pub fn triangle_count(&self) -> usize {
        self.shape.as_trimesh().map_or(0, TriMesh::num_triangles)
    }

    pub(crate) fn surface(&self, triangle: u32) -> SurfaceId {
        self.surfaces.get(triangle as usize).copied().unwrap_or_default()
    }

    /// Whether `scale` needs no build: unit scale, or a cached scaled copy.
    pub fn has_scale(&self, scale: Vec3) -> bool {
        let key = scale.to_array().map(f32::to_bits);
        (scale - Vec3::ONE).abs().max_element() <= 1e-6 || self.scaled.lock().expect("mesh shape cache poisoned").iter().any(|(k, _)| *k == key)
    }

    /// Builds and caches the scaled copy for `scale` (a BVH rebuild: run it off
    /// the main thread before inserting the placement).
    pub fn prepare_scale(&self, scale: Vec3) {
        drop(self.shared(scale));
    }

    /// The engine shape at `scale`: the shared one at unit scale, else a cached scaled copy.
    pub(crate) fn shared(&self, scale: Vec3) -> SharedShape {
        if (scale - Vec3::ONE).abs().max_element() <= 1e-6 {
            return self.shape.clone();
        }
        let key = scale.to_array().map(f32::to_bits);
        let cached = |cache: &[([u32; 3], SharedShape)]| cache.iter().find(|(k, _)| *k == key).map(|(_, s)| s.clone());
        if let Some(s) = cached(&self.scaled.lock().expect("mesh shape cache poisoned")) {
            return s;
        }
        // Built outside the lock: other threads keep reading the cache meanwhile.
        let mesh = self.shape.as_trimesh().expect("MeshShape holds a trimesh").clone().scaled(scale);
        let mut cache = self.scaled.lock().expect("mesh shape cache poisoned");
        if let Some(s) = cached(&cache) {
            return s;
        }
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
    /// Upright (Y axis) cylinder: radius, half height. Round in plan, so yaw does not matter:
    /// the shape of a wide, low character a capsule cannot cover.
    Cylinder { r: f32, half: f32 },
    Box(Vec3),
    Sphere(f32),
    /// A `body.cook` shape (items): hulls and primitives with their mass properties.
    Body(Arc<crate::cook::BodyShape>),
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
            Self::Cylinder { r, half } => {
                let s = scale.x.max(scale.z);
                SharedShape::cylinder(half * scale.y, r * s)
            }
            Self::Box(h) => {
                let h = *h * scale;
                SharedShape::cuboid(h.x, h.y, h.z)
            }
            Self::Sphere(r) => SharedShape::ball(r * scale.max_element()),
            // `body.cook` guarantees a valid part; a 1 cm ball stands in otherwise.
            Self::Body(b) => b.shared(scale).unwrap_or_else(|| SharedShape::ball(0.005)),
        }
    }
}
