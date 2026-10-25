//! `cloth.cook` on a synthetic robe (physics.md "Cloth lattice").

#![allow(clippy::cast_precision_loss, clippy::many_single_char_names)]

use glam::{Affine3A, Quat, Vec3};
use sf_anim::cloth::{ClothCollider, EdgeKind, GridSpec};
use sf_model::{MAX_INFLUENCES, SkinVertex};

use crate::cloth::column_dir;
use crate::{ClothCookError, ClothSpec, SkinSurface, cook_cloth};

const WAIST: f32 = 1.0;
const HEM: f32 = 0.12;

/// Robe radius at height `y`: 0.17 m at the waist flaring to 0.44 m at the hem, as p10a's solver test.
fn flare(y: f32) -> f32 {
    let t = ((WAIST - y) / (WAIST - HEM)).clamp(0.0, 1.0);
    0.17 + (0.44 - 0.17) * t
}

/// A robe tube from the chest (1.3 m) to the hem, `seg` around, `rings` down,
/// plus an inner lining 2 cm inside (rays must take the outer surface), all
/// on joint 0, centred off the origin at `axis`.
fn robe(axis: Vec3, seg: usize, rings: usize) -> SkinSurface {
    let mut s = SkinSurface::default();
    for layer in [0.0f32, -0.02] {
        let base = u32::try_from(s.positions.len()).unwrap();
        for k in 0..=rings {
            let y = 1.3 - (1.3 - HEM) * k as f32 / rings as f32;
            for i in 0..seg {
                let d = column_dir(i, seg);
                // Off-grid angles so lattice columns don't run along mesh edges.
                let d = Quat::from_rotation_y(0.013) * d;
                s.positions.push(axis + Vec3::new(0.0, y, 0.0) + d * (flare(y) + layer));
                s.normals.push(d);
                s.skin.push(SkinVertex { joints: [0; MAX_INFLUENCES], weights: [1.0, 0.0, 0.0, 0.0] });
            }
        }
        let seg32 = u32::try_from(seg).unwrap();
        for k in 0..u32::try_from(rings).unwrap() {
            for i in 0..seg32 {
                let (a, b) = (base + k * seg32 + i, base + k * seg32 + (i + 1) % seg32);
                let (c, d) = (a + seg32, b + seg32);
                s.triangles.extend([[a, b, c], [b, d, c]]);
            }
        }
    }
    s
}

/// Pelvis joint 1 at the waist, turned 30° and off the robe axis; joint 0 is the root.
fn rest() -> Vec<Affine3A> {
    vec![Affine3A::IDENTITY, Affine3A::from_rotation_translation(Quat::from_rotation_y(0.5), Vec3::new(0.01, WAIST, 0.02))]
}

fn spec() -> ClothSpec {
    ClothSpec { attach: "pelvis".into(), ..ClothSpec::default() }
}

#[test]
fn lattice_sits_on_the_outer_surface() {
    let axis = Vec3::new(0.03, 0.0, -0.02);
    let surface = robe(axis, 48, 24);
    let rest = rest();
    let collider = ClothCollider { joint: 1, center: Vec3::ZERO, axis: Vec3::Y, radius: 0.12, half_length: 0.05 };
    let cook = cook_cloth(&surface, &rest, 1, &spec(), vec![collider]).unwrap();
    let l = &cook.lattice;
    let GridSpec { rows, cols, .. } = GridSpec::default();
    assert_eq!((l.rows, l.cols, l.ring, l.particles.len()), (rows, cols, true, 50));
    assert_eq!(l.colliders, vec![collider]);
    assert_eq!(cook.filled, 0);
    assert!((cook.top - WAIST).abs() < 1e-6 && (cook.hem - HEM).abs() < 1e-6, "top {} hem {}", cook.top, cook.hem);
    assert!(cook.center.distance(Vec3::new(axis.x, WAIST, axis.z)) < 0.01, "centre {}", cook.center);
    let cols = usize::from(cols);
    for (i, p) in l.particles.iter().enumerate() {
        let (r, c) = (i / cols, i % cols);
        // On the outer surface (a 48-gon: within its chord sag), not the lining.
        let radial = Vec3::new(p.rest.x - axis.x, 0.0, p.rest.z - axis.z);
        assert!((radial.length() - flare(p.rest.y)).abs() < 0.006, "particle {i}: radius {} vs {}", radial.length(), flare(p.rest.y));
        assert!(radial.normalize().dot(column_dir(c, cols)) > 0.999, "column direction");
        assert_eq!(p.joint, 1);
        assert_eq!(p.inv_mass == 0.0, r == 0, "row 0 pinned");
        // Offsets in the joint frame give the rest position back.
        assert!(rest[1].transform_point3(cook.offsets[i]).distance(p.rest) < 1e-5);
    }
    let first = l.particles[0].rest.y;
    let last = l.particles[l.particles.len() - 1].rest.y;
    assert!(first < WAIST && first > WAIST - 0.02 && last > HEM && last < HEM + 0.02, "rows {first}..{last}");
    // The hem ring is the flare's circumference (10 chords of a 0.44 m circle).
    let hem: f32 = l.edges.iter().filter(|e| e.kind == EdgeKind::Structural && usize::from(e.a) >= 40 && usize::from(e.b) >= 40).map(|e| e.rest).sum();
    let chord = 10.0 * 2.0 * 0.44 * (std::f32::consts::PI / 10.0).sin();
    assert!((hem - chord).abs() < 0.03, "hem perimeter {hem} vs {chord}");
}

#[test]
fn attachment_line_moves_with_the_offset_and_clamps_to_the_part() {
    let surface = robe(Vec3::ZERO, 32, 16);
    let rest = rest();
    let lower = cook_cloth(&surface, &rest, 1, &ClothSpec { attach_offset: -0.2, ..spec() }, Vec::new()).unwrap();
    assert!((lower.top - 0.8).abs() < 1e-6);
    // Above the part's top (1.3 m), the line stops at the top.
    let high = cook_cloth(&surface, &rest, 1, &ClothSpec { attach_offset: 0.5, ..spec() }, Vec::new()).unwrap();
    assert!((high.top - 1.3).abs() < 1e-6);
    let small = ClothSpec { grid: GridSpec { rows: 3, cols: 8, ..GridSpec::default() }, ..spec() };
    assert_eq!(cook_cloth(&surface, &rest, 1, &small, Vec::new()).unwrap().lattice.particles.len(), 24);
}

#[test]
fn rejects_what_it_cannot_place() {
    let surface = robe(Vec3::ZERO, 16, 8);
    let rest = rest();
    let err = |spec: &ClothSpec, attach| cook_cloth(&surface, &rest, attach, spec, Vec::new()).unwrap_err();
    assert_eq!(err(&ClothSpec { grid: GridSpec { cols: 2, ..GridSpec::default() }, ..spec() }, 1), ClothCookError::Size);
    assert_eq!(err(&ClothSpec { grid: GridSpec { ring: false, ..GridSpec::default() }, ..spec() }, 1), ClothCookError::Open);
    assert_eq!(err(&spec(), 7), ClothCookError::Joint(7));
    assert_eq!(err(&ClothSpec { attach_offset: -2.0, ..spec() }, 1), ClothCookError::Empty);
    assert!(matches!(err(&ClothSpec { attach_offset: -0.85, ..spec() }, 1), ClothCookError::Short(_)));
}

/// `SkinRebind`: the hem goes to the lattice, the waist keeps its weights, the band between blends;
/// moving the lattice moves the hem and leaves everything above the line.
#[test]
fn rebind_moves_the_hem_and_keeps_the_waist() {
    let mut surface = robe(Vec3::ZERO, 48, 40);
    // Above the waist on the pelvis (1), below on a "leg" joint (0), as a skinned robe is.
    for (p, s) in surface.positions.iter().zip(&mut surface.skin) {
        s.joints[0] = u16::from(p.y >= WAIST);
    }
    let rest = rest();
    let cook = cook_cloth(&surface, &rest, 1, &spec(), Vec::new()).unwrap();
    let rb = &cook.rebind;
    assert_eq!((rb.base, rb.skin.len()), (2, surface.positions.len()));
    println!("rebind: {} on the lattice, {} in the band, {} sweeps, residual {:e}", rb.on_lattice, rb.band, rb.sweeps, rb.residual);
    assert!(rb.on_lattice > 0 && rb.band > 0 && rb.residual < 1e-4);
    let l = &cook.lattice;
    let first = l.particles[0].rest.y;
    let spacing = first - l.particles[usize::from(l.cols)].rest.y;
    let mut mixed = 0;
    for ((p, s), orig) in surface.positions.iter().zip(&rb.skin).zip(&surface.skin) {
        let total: f32 = s.weights.iter().sum();
        assert!((total - 1.0).abs() < 1e-4, "weights sum {total}");
        let lattice: f32 = s.joints.iter().zip(s.weights).filter(|(j, _)| **j >= rb.base).map(|(_, w)| w).sum();
        if p.y >= first + 0.25 * spacing {
            assert_eq!(s, orig, "above the band: untouched");
        } else if p.y <= first - spacing {
            assert!((lattice - 1.0).abs() < 1e-4, "below row 1: lattice only, {lattice}");
            // The heaviest lattice joint is a particle next to the vertex.
            let k = (0..4).max_by(|&a, &b| s.weights[a].total_cmp(&s.weights[b])).unwrap();
            let near = l.particles[usize::from(s.joints[k] - rb.base)].rest;
            assert!(near.distance(*p) < 0.2, "vertex {p} on particle {near}");
        } else if lattice > 0.01 && lattice < 0.99 {
            mixed += 1;
        }
    }
    assert!(mixed > 0, "the band blends body and lattice");
    // Skinning: skeleton still, every lattice joint moved 10 cm sideways.
    let shift = Vec3::new(0.1, 0.0, 0.0);
    let skin_at = |s: &SkinVertex, p: Vec3| s.joints.iter().zip(s.weights).map(|(&j, w)| if j >= rb.base { (p + shift) * w } else { p * w }).sum::<Vec3>();
    for (p, s) in surface.positions.iter().zip(&rb.skin) {
        let moved = skin_at(s, *p).distance(*p);
        if p.y >= first + 0.25 * spacing {
            assert!(moved < 1e-6, "the waist stays");
        } else if p.y <= first - spacing {
            assert!((moved - 0.1).abs() < 1e-4, "the hem follows the lattice");
        }
    }
}

/// Per-particle target skins: the authored weights where each particle sits.
#[test]
fn targets_carry_the_authored_skin() {
    let mut surface = robe(Vec3::ZERO, 32, 24);
    for (p, s) in surface.positions.iter().zip(&mut surface.skin) {
        s.joints[0] = u16::from(p.y >= 0.6);
    }
    let cook = cook_cloth(&surface, &rest(), 1, &spec(), Vec::new()).unwrap();
    assert_eq!(cook.targets.len(), cook.lattice.particles.len());
    for (t, p) in cook.targets.iter().zip(&cook.lattice.particles) {
        assert!((t.weights.iter().sum::<f32>() - 1.0).abs() < 1e-5);
        if (p.rest.y - 0.6).abs() > 0.06 {
            assert_eq!(t.joints[0], u16::from(p.rest.y >= 0.6), "particle at {}", p.rest.y);
            assert!(t.weights[0] > 0.999);
        }
    }
}
