//! `MikkTSpace` tangents for normal-mapped materials.
//!
//! `MikkTSpace` is the tangent basis normal maps are baked against (Blender,
//! Substance, xNormal, UE, Unity; the glTF spec requires it when a mesh ships
//! no tangents), so a renderer must rebuild exactly that basis or relief
//! shades wrong. `bevy_mikktspace` is Bevy's own byte-identical port of the
//! reference C code. Tangents are generated per triangle corner and the
//! cooks then weld vertex and tangent together: a vertex whose corners
//! disagree (a mirrored-UV seam, where the handedness flips) splits into one
//! vertex per side, which is what DCC exporters write.
//!
//! Handedness is Bevy's: `w` is the negated `MikkTSpace` sign, exactly what
//! `Mesh::generate_tangents` stores (`bevy_mesh` 0.20 `src/mikktspace.rs:126-129`,
//! "mikktspace seems to assume left-handedness so we can flip the sign"). Bevy's
//! PBR shader builds the bitangent as `w * cross(N, T)` (`bevy_pbr` 0.20
//! `src/render/pbr_functions.wesl:162`) and reads a normal map's +Y along it;
//! with Bevy's top-left UV origin that is `-dP/dv`, the image's up: the
//! canonical (OpenGL, +Y up) convention the cooks convert normal maps to. The
//! raw `MikkTSpace` sign points the bitangent along `+dP/dv` and inverts green.
//! Cooked tangents and the loader's `generate_tangents` fallback (meshes
//! without cooked tangents) must agree, so both use this sign.

use bevy_mikktspace::{Geometry, TangentSpace, generate_tangents};
use glam::Vec3;

use crate::cooked::{Tangent, Vertex};

/// The faces of one indexed triangle list, and the tangent of each corner.
struct Corners<'a> {
    vertices: &'a [Vertex],
    indices: &'a [u32],
    out: Vec<Tangent>,
}

impl Corners<'_> {
    fn vertex(&self, face: usize, vert: usize) -> &Vertex {
        &self.vertices[self.indices[face * 3 + vert] as usize]
    }
}

impl Geometry for Corners<'_> {
    fn num_faces(&self) -> usize {
        self.indices.len() / 3
    }
    fn num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }
    fn position(&self, face: usize, vert: usize) -> [f32; 3] {
        self.vertex(face, vert).position
    }
    fn normal(&self, face: usize, vert: usize) -> [f32; 3] {
        self.vertex(face, vert).normal
    }
    fn tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
        self.vertex(face, vert).uv
    }
    fn set_tangent(&mut self, tangent_space: Option<TangentSpace>, face: usize, vert: usize) {
        let normal = self.vertex(face, vert).normal;
        self.out[face * 3 + vert] = settle(tangent_space.map(|t| bevy_handed(t.tangent_encoded())), normal);
    }
}

/// One `MikkTSpace` tangent per index of `indices` (a triangle list over
/// `vertices`). Corners `MikkTSpace` leaves without a basis (a degenerate face
/// with no neighbour to borrow from) get an arbitrary unit tangent
/// perpendicular to their normal, so every tangent is finite, unit-length
/// and right-handed or left-handed (w = ±1).
pub(crate) fn corner_tangents(vertices: &[Vertex], indices: &[u32]) -> Vec<Tangent> {
    let out = indices.iter().map(|&i| settle(None, vertices[i as usize].normal)).collect();
    let mut corners = Corners { vertices, indices, out };
    // The error type has no variants: generation can't fail.
    let _ = generate_tangents(&mut corners);
    corners.out
}

/// A `MikkTSpace` encoded tangent in Bevy's handedness: `w` negated (module docs).
fn bevy_handed([x, y, z, w]: Tangent) -> Tangent {
    [x, y, z, -w]
}

/// `encoded` normalized, or a fallback perpendicular to `normal`.
fn settle(encoded: Option<Tangent>, normal: [f32; 3]) -> Tangent {
    if let Some([x, y, z, w]) = encoded {
        let t = Vec3::new(x, y, z);
        if t.is_finite() && t.length_squared() > 1e-12 {
            let t = t.normalize();
            return [t.x, t.y, t.z, if w < 0.0 { -1.0 } else { 1.0 }];
        }
    }
    let n = Vec3::from_array(normal);
    let t = if n.is_finite() && n.length_squared() > 1e-12 { n.normalize().any_orthonormal_vector() } else { Vec3::X };
    [t.x, t.y, t.z, 1.0]
}

/// `items` (one per vertex) un-indexed into one per index.
pub(crate) fn per_corner<T: Copy>(items: &[T], indices: &[u32]) -> Vec<T> {
    indices.iter().map(|&i| items[i as usize]).collect()
}
