//! Primitive meshes: flat-shaded, outward counter-clockwise triangles, UVs in
//! meters along each face's own edges (a 1 m texture tile is a 1 m square).

use std::f32::consts::TAU;

use glam::Vec3;

use crate::Shape;

/// Triangles in an object's frame. The same data serves the render mesh and
/// the static collider.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshData {
    /// Triangles as index triples (collision meshes).
    pub fn triangles(&self) -> Vec<[u32; 3]> {
        self.indices.as_chunks::<3>().0.to_vec()
    }

    /// Local bounds (min, max); zero for an empty mesh.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let mut it = self.positions.iter().map(|p| Vec3::from_array(*p));
        let Some(first) = it.next() else { return (Vec3::ZERO, Vec3::ZERO) };
        it.fold((first, first), |(lo, hi), p| (lo.min(p), hi.max(p)))
    }

    fn base(&self) -> u32 {
        u32::try_from(self.positions.len()).expect("mesh fits u32 indices")
    }

    fn vertex(&mut self, p: Vec3, n: Vec3, u: Vec3, v: Vec3) {
        self.positions.push(p.to_array());
        self.normals.push(n.to_array());
        self.uvs.push([p.dot(u), -p.dot(v)]);
    }

    /// A planar polygon (3 or 4 points), wound to face `outward`.
    fn face(&mut self, points: &[Vec3], outward: Vec3) {
        let mut pts = points.to_vec();
        let mut n = (pts[1] - pts[0]).cross(pts[2] - pts[0]);
        if n.dot(outward) < 0.0 {
            pts.reverse();
            n = -n;
        }
        let n = n.normalize_or(outward);
        // UV axes: along the face, `v` as close to up as the face allows.
        let up = if n.y.abs() > 0.99 { Vec3::NEG_Z } else { Vec3::Y };
        let u = up.cross(n).normalize_or(Vec3::X);
        let v = n.cross(u);
        let base = self.base();
        for p in &pts {
            self.vertex(*p, n, u, v);
        }
        for i in 1..u32::try_from(pts.len()).expect("small polygon") - 1 {
            self.indices.extend([base, base + i, base + i + 1]);
        }
    }

    /// An axis-aligned box `lo..hi`; `skip` drops faces by outward normal.
    fn cuboid(&mut self, lo: Vec3, hi: Vec3, skip: &[Vec3]) {
        let c = |x: bool, y: bool, z: bool| Vec3::new(if x { hi.x } else { lo.x }, if y { hi.y } else { lo.y }, if z { hi.z } else { lo.z });
        let faces = [
            (Vec3::X, [c(true, false, false), c(true, true, false), c(true, true, true), c(true, false, true)]),
            (Vec3::NEG_X, [c(false, false, false), c(false, false, true), c(false, true, true), c(false, true, false)]),
            (Vec3::Y, [c(false, true, false), c(false, true, true), c(true, true, true), c(true, true, false)]),
            (Vec3::NEG_Y, [c(false, false, false), c(true, false, false), c(true, false, true), c(false, false, true)]),
            (Vec3::Z, [c(false, false, true), c(true, false, true), c(true, true, true), c(false, true, true)]),
            (Vec3::NEG_Z, [c(false, false, false), c(false, true, false), c(true, true, false), c(true, false, false)]),
        ];
        for (n, quad) in faces {
            if !skip.contains(&n) {
                self.face(&quad, n);
            }
        }
    }
}

pub(crate) fn build(shape: &Shape) -> MeshData {
    let mut m = MeshData::default();
    match *shape {
        Shape::Ground { size } => {
            let h = Vec3::new(size.0 / 2.0, 0.0, size.1 / 2.0);
            m.cuboid(Vec3::new(-h.x, -1.0, -h.z), h, &[]);
        }
        Shape::Box { size } => {
            let h = Vec3::new(size.x / 2.0, 0.0, size.z / 2.0);
            m.cuboid(-h, h + Vec3::Y * size.y, &[]);
        }
        Shape::Ramp { width, rise, .. } => {
            let (w, l) = (width / 2.0, shape.length() / 2.0);
            // Low edge at +z on the ground, high edge at -z, `rise` up.
            let (fl, fr) = (Vec3::new(-w, 0.0, l), Vec3::new(w, 0.0, l));
            let (bl, br) = (Vec3::new(-w, 0.0, -l), Vec3::new(w, 0.0, -l));
            let (tl, tr) = (bl + Vec3::Y * rise, br + Vec3::Y * rise);
            let slope_n = Vec3::new(0.0, 2.0 * l, rise).normalize();
            m.face(&[fl, fr, tr, tl], slope_n);
            m.face(&[fl, fr, br, bl], Vec3::NEG_Y);
            m.face(&[bl, br, tr, tl], Vec3::NEG_Z);
            m.face(&[fl, bl, tl], Vec3::NEG_X);
            m.face(&[fr, br, tr], Vec3::X);
        }
        Shape::Stairs { width, steps, rise, run } => {
            let (w, l) = (width / 2.0, shape.length() / 2.0);
            for i in 0..steps {
                #[allow(clippy::cast_precision_loss)]
                let (k, top) = (i as f32, (i + 1) as f32 * rise);
                let (z_front, z_back) = (l - k * run, l - (k + 1.0) * run);
                let lo = Vec3::new(-w, 0.0, z_back);
                let hi = Vec3::new(w, top, z_front);
                // Only the visible surfaces: the riser above the previous
                // tread, the tread, the sides, the bottom, the last back.
                let mut skip = vec![Vec3::Z];
                if i + 1 < steps {
                    skip.push(Vec3::NEG_Z);
                }
                m.cuboid(lo, hi, &skip);
                let below = k * rise;
                m.face(
                    &[Vec3::new(-w, below, z_front), Vec3::new(w, below, z_front), Vec3::new(w, top, z_front), Vec3::new(-w, top, z_front)],
                    Vec3::Z,
                );
            }
        }
        Shape::Cylinder { radius, height } => {
            const SEGMENTS: u32 = 24;
            #[allow(clippy::cast_precision_loss)]
            let at = |i: u32| {
                let a = TAU * i as f32 / SEGMENTS as f32;
                Vec3::new(a.cos() * radius, 0.0, -a.sin() * radius)
            };
            let up = Vec3::Y * height;
            for i in 0..SEGMENTS {
                let (a, b) = (at(i), at(i + 1));
                let mid = ((a + b) / 2.0).normalize_or(Vec3::X);
                m.face(&[a, b, b + up, a + up], mid);
                m.face(&[Vec3::ZERO, a, b], Vec3::NEG_Y);
                m.face(&[up, a + up, b + up], Vec3::Y);
            }
        }
    }
    m
}
