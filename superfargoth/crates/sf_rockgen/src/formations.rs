//! Hero formations (rockgen.md "(d) Hero formations", RG-8 first try, owner ask 2026-11-23): the large, few, named
//! rocks no scatter makes. Each is a recipe over the same SDF as the boulders ([`crate::sdf`]: a union of convex
//! pieces, rounded, weathered) and the same mesher, so it inherits the LOD chain, the crease normals and the edge
//! snap. Three shapes:
//!
//! - [`Stack`]: a column of strata bands, each a polygonal frustum, hard bands wider than soft ones (differential
//!   erosion; Peytavie et al. 2009 layer hardness), with an optional caprock and a wave notch. Hoodoos (Bryce
//!   Canyon), pinnacle spires, sea stacks (Old Harry, the Twelve Apostles).
//! - [`Neck`]: a volcanic plug: a column cluster ([`crate::columns`], the columnar class's jointing) standing
//!   tall, tapered by a ring of shared planes (Devils Tower, Shiprock).
//! - [`Arch`]: a rib of convex segments along a half circle on two flared legs (Delicate Arch, Durdle Door; the
//!   spec's "slab minus a capsule" needs a subtraction the SDF doesn't have yet, so the rib is built directly).
//!
//! Presets ([`FORMATIONS`]) are `Guess` numbers for the owner's look loop, as [`crate::classes`]; an adapter maps
//! its formations onto these or its own (RG-A1). Built in a unit frame (+Z up, the longest side 1), then scaled
//! like a boulder. A pure function of (formation, mesh params, size, seed). Nothing here names a game.

use std::f32::consts::{PI, TAU};

use glam::{Vec2, Vec3};
use sf_tunables::Source;

use crate::boulder::{BoulderParams, scaled};
use crate::columns::{ColumnSpec, column_cluster};
use crate::mesher::{MeshSettings, mesh_solid};
use crate::noise::Stream;
use crate::sdf::{Piece, Plane, Solid, Weathering};
use crate::RockLods;

/// Stream purpose (any distinct word): formation shape draws.
const SHAPE_STREAM: u64 = 0x5a4e_0101;
/// Neighbouring pieces overlap by this fraction of a band or segment (at least [`MIN_OVERLAP_CELLS`]): two
/// rounded pieces meeting on one face would leave a slit along it, which folds the net.
// tunables: const (definition: the union's seam closure, not a look value)
const OVERLAP: f32 = 0.15;
// tunables: const (sampling limit of the mesher)
const MIN_OVERLAP_CELLS: f32 = 3.0;
/// The sampled field's rounding never goes below this many cells (as the boulder's).
// tunables: const (sampling limit of the mesher, not a look value)
const MIN_ROUNDING_CELLS: f32 = 2.0;

/// A banded tower: strata bands stacked from the ground (z = -0.5) to the top (z = 0.5). Radii are over the height.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stack {
    /// Radius at the foot and at the top of the profile.
    pub base_radius: f32,
    pub top_radius: f32,
    /// Sides of each band's polygon.
    pub sides: u32,
    /// Each vertex's radius varies by up to this fraction.
    pub side_jitter: f32,
    pub bands: u32,
    /// Band boundaries move by up to this fraction of a band.
    pub band_jitter: f32,
    /// A soft band (hardness 0) loses this fraction of its radius; hardness is uniform per band.
    pub recess: f32,
    /// Caprock: the top `cap_height` of the height is one band `cap` times the profile radius (0: none).
    pub cap: f32,
    pub cap_height: f32,
    /// Each band's polygon turns by up to this fraction of a side.
    pub twist: f32,
    /// The whole stack leans this far, toward a random azimuth.
    pub lean_deg: f32,
    /// Each band's centre wanders by up to this fraction of its radius.
    pub drift: f32,
    /// The foot band's bottom is this many times wider (a talus skirt).
    pub flare: f32,
    /// A wave notch: centred this fraction up the height (0: none), cutting this fraction of the radius.
    pub notch_at: f32,
    pub notch_depth: f32,
}

/// A volcanic plug: a tall column cluster, tapered.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Neck {
    /// Footprint semi-axes over the height.
    pub radius: [f32; 2],
    pub columns: u32,
    /// Column tops fall by up to this fraction of the height.
    pub step: f32,
    /// Top radius over the foot radius (the taper planes).
    pub taper: f32,
    /// Taper planes around the plug.
    pub facets: u32,
    pub groove: f32,
    pub jitter: f32,
    pub tilt_deg: f32,
    pub fuse: f32,
}

/// A natural arch: outer span 1 along x, the rib a half ring on two legs, `thickness` across (y).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arch {
    /// Top of the crown over the span.
    pub height: f32,
    /// The rib's radial thickness at the springing, over the span.
    pub rib: f32,
    /// The crown is this fraction thinner than the springing.
    pub thin_top: f32,
    /// Across the arch (y), over the span.
    pub thickness: f32,
    pub segments: u32,
    /// The legs widen by this fraction of their height toward the ground.
    pub leg_flare: f32,
    /// Each segment's size and position vary by up to this fraction.
    pub jitter: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Stack(Stack),
    Neck(Neck),
    Arch(Arch),
}

/// The surface: rounding and weathering, as fractions of the longest side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    pub rounding: f32,
    pub weather_amp: f32,
    pub weather_wavelength: f32,
    pub weather_octaves: u32,
}

/// One preset.
#[derive(Clone, Copy, Debug)]
pub struct Formation {
    pub name: &'static str,
    /// What it imitates.
    pub analogue: &'static str,
    pub source: Source,
    /// Albedo multiplier the viewer applies (as [`crate::RockClass::tint`]).
    pub tint: f32,
    /// Longest side the viewer shows it at, metres.
    pub size_m: f32,
    pub shape: Shape,
    pub look: Look,
}

const ERODED: Look = Look { rounding: 0.012, weather_amp: 0.006, weather_wavelength: 0.08, weather_octaves: 3 };
const FRESH: Look = Look { rounding: 0.0, weather_amp: 0.003, weather_wavelength: 0.05, weather_octaves: 2 };

/// Every preset.
pub const FORMATIONS: &[Formation] = &[
    Formation {
        name: "hoodoo",
        analogue: "hoodoo (Bryce Canyon): soft beds recessed between hard ones under a wide caprock",
        source: Source::Guess,
        tint: 1.0,
        size_m: 14.0,
        shape: Shape::Stack(Stack {
            base_radius: 0.14,
            top_radius: 0.08,
            sides: 9,
            side_jitter: 0.18,
            bands: 7,
            band_jitter: 0.6,
            recess: 0.45,
            cap: 1.8,
            cap_height: 0.08,
            twist: 0.5,
            lean_deg: 2.0,
            drift: 0.08,
            flare: 1.5,
            notch_at: 0.0,
            notch_depth: 0.0,
        }),
        look: ERODED,
    },
    Formation {
        name: "pinnacle",
        analogue: "volcanic pinnacle / aiguille: a jagged spire tapering to a point, leaning, on a scree skirt",
        source: Source::Guess,
        tint: 0.6,
        size_m: 30.0,
        shape: Shape::Stack(Stack {
            base_radius: 0.13,
            top_radius: 0.012,
            sides: 7,
            side_jitter: 0.3,
            bands: 10,
            band_jitter: 0.5,
            recess: 0.2,
            cap: 0.0,
            cap_height: 0.0,
            twist: 0.9,
            lean_deg: 5.0,
            drift: 0.15,
            flare: 1.8,
            notch_at: 0.0,
            notch_depth: 0.0,
        }),
        look: FRESH,
    },
    Formation {
        name: "sea_stack",
        analogue: "sea stack (Old Harry, the Twelve Apostles): jointed, near-vertical sides, a wave notch at its foot",
        source: Source::Guess,
        tint: 0.9,
        size_m: 24.0,
        shape: Shape::Stack(Stack {
            base_radius: 0.2,
            top_radius: 0.16,
            sides: 5,
            side_jitter: 0.12,
            bands: 6,
            band_jitter: 0.4,
            recess: 0.12,
            cap: 0.0,
            cap_height: 0.0,
            twist: 0.06,
            lean_deg: 1.0,
            drift: 0.04,
            flare: 1.25,
            notch_at: 0.07,
            notch_depth: 0.35,
        }),
        look: FRESH,
    },
    Formation {
        name: "volcanic_neck",
        analogue: "volcanic plug (Devils Tower, Shiprock): a tapered tower of columnar basalt with a stepped crown",
        source: Source::Guess,
        tint: 0.55,
        size_m: 36.0,
        shape: Shape::Neck(Neck {
            radius: [0.3, 0.26],
            columns: 40,
            step: 0.12,
            taper: 0.65,
            facets: 9,
            groove: 0.25,
            jitter: 0.25,
            tilt_deg: 10.0,
            fuse: 0.05,
        }),
        look: FRESH,
    },
    Formation {
        name: "arch",
        analogue: "natural arch (Delicate Arch, Durdle Door): a rib thinning toward the crown on two flared legs",
        source: Source::Guess,
        tint: 1.0,
        size_m: 30.0,
        shape: Shape::Arch(Arch {
            height: 0.62,
            rib: 0.17,
            thin_top: 0.45,
            thickness: 0.16,
            segments: 12,
            leg_flare: 0.25,
            jitter: 0.12,
        }),
        look: ERODED,
    },
];

/// The plane through `point` with outward normal `n` (normalised here).
fn plane(n: Vec3, point: Vec3) -> Plane {
    let n = n.normalize();
    Plane { n, d: n.dot(point) }
}

/// The formation's solid (unit frame) and its meshing voxel, for a rock of `size_m` meshed with `mesh`'s
/// resolution and snap.
pub fn formation_solid(f: &Formation, mesh: &BoulderParams, size_m: f32, seed: u64) -> (Solid, f32) {
    let p = BoulderParams { size_m, ..*mesh }.clamped();
    let voxel = p.voxel();
    let mut rng = Stream::new(seed, SHAPE_STREAM);
    let (pieces, cap, reach) = match f.shape {
        Shape::Stack(s) => stack(&s, voxel, &mut rng),
        Shape::Neck(n) => neck(&n, voxel, &mut rng),
        Shape::Arch(a) => arch(&a, voxel, &mut rng),
    };
    let snap = p.edge_snap && f.look.rounding < voxel;
    let rounding = f.look.rounding.max(MIN_ROUNDING_CELLS * voxel);
    let weathering =
        Weathering { amp: f.look.weather_amp, wavelength: f.look.weather_wavelength, octaves: f.look.weather_octaves };
    (Solid::new(pieces, rounding, weathering, reach, voxel, snap, seed).with_cap(cap), voxel)
}

/// Generates one formation at `size_m` (its longest side): the LOD chain, bounds and content hash.
pub fn generate_formation(f: &Formation, mesh: &BoulderParams, size_m: f32, seed: u64) -> RockLods {
    let p = BoulderParams { size_m, ..*mesh }.clamped();
    let (solid, voxel) = formation_solid(f, mesh, size_m, seed);
    let settings = MeshSettings {
        voxel,
        lod0_error: p.lod0_error(),
        lod_levels: p.lod_levels,
        lod_ratio: p.lod_ratio,
        crease_deg: p.crease_deg,
    };
    scaled(mesh_solid(&solid, &settings), p.size_m)
}

type Parts = (Vec<Piece>, Vec<Plane>, Vec3);

/// The stack's bands: frusta between jittered boundaries, each the convex hull of a bottom polygon and its scaled,
/// shifted copy on top (so every side is a planar trapezoid).
#[allow(clippy::cast_precision_loss)] // band and side counts are small
fn stack(s: &Stack, voxel: f32, rng: &mut Stream) -> Parts {
    let bands = s.bands.max(1);
    let sides = s.sides.max(3);
    let top_of_body = 0.5 - if s.cap > 0.0 { s.cap_height } else { 0.0 };
    // Boundaries from the ground up; the notch adds its own three.
    let body = top_of_body + 0.5;
    let mut zs: Vec<f32> = (0..=bands)
        .map(|i| {
            let jitter = if i == 0 || i == bands { 0.0 } else { 0.5 * s.band_jitter * rng.signed() };
            -0.5 + (i as f32 + jitter) / bands as f32 * body
        })
        .collect();
    let notch_w = 0.5 * body / bands as f32;
    if s.notch_at > 0.0 {
        let c = -0.5 + s.notch_at;
        zs.retain(|z| (z - c).abs() > 0.6 * notch_w || *z <= -0.5 || *z >= top_of_body);
        zs.extend([c - notch_w, c, c + notch_w].into_iter().filter(|z| *z > -0.5 && *z < top_of_body));
        zs.sort_by(f32::total_cmp);
    }
    if s.cap > 0.0 {
        zs.push(0.5);
    }
    let lean = s.lean_deg.to_radians().tan();
    let lean_dir = Vec2::from_angle(rng.unit() * TAU);
    let profile = |z: f32| {
        let t = z + 0.5;
        let notch = if s.notch_at > 0.0 { 1.0 - s.notch_depth * (1.0 - ((z + 0.5 - s.notch_at).abs() / notch_w).min(1.0)) } else { 1.0 };
        (s.base_radius + (s.top_radius - s.base_radius) * t) * notch
    };
    let overlap = |dz: f32| (OVERLAP * dz).max(MIN_OVERLAP_CELLS * voxel);
    let mut pieces = Vec::new();
    let mut reach = Vec3::new(0.0, 0.0, 0.5);
    let n_bands = zs.len() - 1;
    for b in 0..n_bands {
        let (z0, z1) = (zs[b], zs[b + 1]);
        let is_cap = s.cap > 0.0 && b + 1 == n_bands;
        let hardness = rng.unit();
        let factor = if is_cap { s.cap } else { 1.0 - s.recess * (1.0 - hardness) };
        let (mut rb, rt) = if is_cap {
            let r = profile(z0) * factor;
            (r, r * 0.9)
        } else {
            (profile(z0) * factor, profile(z1) * factor)
        };
        if b == 0 {
            rb *= s.flare;
        }
        let drift = Vec2::from_angle(rng.unit() * TAU) * s.drift * rng.unit() * rb.max(rt);
        let centre = |z: f32| lean_dir * lean * (z + 0.5) + drift;
        let turn = s.twist * rng.signed() / sides as f32 * TAU;
        let outline: Vec<Vec2> = (0..sides)
            .map(|i| {
                let a = (i as f32 + 0.3 * rng.signed()) / sides as f32 * TAU + turn;
                Vec2::from_angle(a) * (1.0 + s.side_jitter * rng.signed())
            })
            .collect();
        let (lo, hi) = (if b == 0 { z0 } else { z0 - overlap(z1 - z0) }, if b + 1 == n_bands { z1 } else { z1 + overlap(z1 - z0) });
        // The frustum's rings at the band's own boundaries; the overlap extends the same side planes.
        let (cb, ct) = (centre(z0), centre(z1));
        let bottom: Vec<Vec3> = outline.iter().map(|u| (cb + *u * rb).extend(z0)).collect();
        let top: Vec<Vec3> = outline.iter().map(|u| (ct + *u * rt).extend(z1)).collect();
        let mid = ((cb + ct) * 0.5).extend(f32::midpoint(z0, z1));
        let mut piece = Piece::default();
        for i in 0..outline.len() {
            let j = (i + 1) % outline.len();
            let n = (bottom[j] - bottom[i]).cross(top[i] - bottom[i]);
            let n = if n.dot(bottom[i] - mid) < 0.0 { -n } else { n };
            if n.length_squared() > 0.0 {
                piece.planes.push(plane(n, bottom[i]));
            }
        }
        piece.planes.push(Plane { n: Vec3::Z, d: hi });
        piece.planes.push(Plane { n: Vec3::NEG_Z, d: -lo });
        // The overlap can carry the slanted sides past the rings: reach covers it.
        let grow = (hi - lo) / (z1 - z0).max(f32::MIN_POSITIVE);
        for q in bottom.iter().chain(&top) {
            reach.x = reach.x.max(q.x.abs() * grow.max(1.0));
            reach.y = reach.y.max(q.y.abs() * grow.max(1.0));
        }
        pieces.push(piece);
    }
    (pieces, Vec::new(), reach)
}

/// The plug: a column cluster standing on z = -0.5, tapered by `facets` planes shared by every column.
fn neck(n: &Neck, voxel: f32, rng: &mut Stream) -> Parts {
    let half = Vec3::new(n.radius[0].max(n.radius[1]), n.radius[0].min(n.radius[1]), 0.5);
    let spec = ColumnSpec {
        count: n.columns,
        jitter: n.jitter,
        groove: n.groove,
        step: n.step,
        tilt_deg: n.tilt_deg,
        fuse: n.fuse,
    };
    let (pieces, mut cap, reach) = column_cluster(&spec, half, voxel, rng);
    let facets = n.facets.max(3);
    let turn = rng.unit() * TAU;
    for k in 0..facets {
        #[allow(clippy::cast_precision_loss)]
        let u = Vec2::from_angle(turn + k as f32 / facets as f32 * TAU);
        // The footprint ellipse's radius toward u; the foot plane stands just outside it.
        let r = 1.0 / ((u.x / half.x).powi(2) + (u.y / half.y).powi(2)).sqrt();
        let (rb, rt) = (r * 1.05, r * n.taper);
        let normal = u.extend(rb - rt);
        cap.push(plane(normal, (u * rb).extend(-0.5)));
    }
    (pieces, cap, reach)
}

/// The arch: a half ring of convex segments (two radial cuts, an outer tangent, an inner chord, two faces across)
/// on two legs whose outer and inner faces flare toward the ground. Outer span 1, ground at z = -height/2.
#[allow(clippy::cast_precision_loss)] // segment counts are small
fn arch(a: &Arch, voxel: f32, rng: &mut Stream) -> Parts {
    let segments = a.segments.max(3);
    let height = a.height.max(0.1);
    let ground = -0.5 * height;
    let ri = (0.5 - a.rib).max(0.05);
    let crown = ri + a.rib * (1.0 - a.thin_top);
    let spring = (height - crown).max(0.0);
    let c = Vec3::new(0.0, 0.0, ground + spring);
    let half_t = 0.5 * a.thickness;
    let step = PI / segments as f32;
    let ext = (OVERLAP * step).max(MIN_OVERLAP_CELLS * voxel / ri);
    let mut pieces = Vec::new();
    let mut reach = Vec3::new(0.5, half_t, 0.5 * height);
    for k in 0..segments {
        let (a0, a1) = (k as f32 * step - ext, (k + 1) as f32 * step + ext);
        let am = f32::midpoint(a0, a1);
        let m = Vec3::new(am.cos(), 0.0, am.sin());
        let ro = (ri + a.rib * (1.0 - a.thin_top * am.sin())) * (1.0 + 0.3 * a.jitter * rng.signed());
        let wobble = a.jitter * half_t * rng.signed();
        let t = half_t * (1.0 + 0.5 * a.jitter * rng.signed());
        let piece = Piece {
            planes: vec![
                plane(Vec3::new(a0.sin(), 0.0, -a0.cos()), c),
                plane(Vec3::new(-a1.sin(), 0.0, a1.cos()), c),
                plane(m, c + m * ro),
                plane(-m, c + m * ri),
                plane(Vec3::new(0.0, 1.0, 0.1 * rng.signed()), Vec3::new(0.0, wobble + t, c.z)),
                plane(Vec3::new(0.0, -1.0, 0.1 * rng.signed()), Vec3::new(0.0, wobble - t, c.z)),
            ],
            ellipsoid: None,
            hard: Vec::new(),
        };
        reach.y = reach.y.max(wobble.abs() + t * 1.2);
        reach.x = reach.x.max(ro / (0.5 * (a1 - a0)).cos());
        reach.z = reach.z.max(c.z + ro / (0.5 * (a1 - a0)).cos());
        pieces.push(piece);
    }
    if spring > MIN_OVERLAP_CELLS * voxel {
        let top = c.z + (OVERLAP * spring).max(MIN_OVERLAP_CELLS * voxel);
        let flare = a.leg_flare * spring;
        for side in [1.0f32, -1.0] {
            let outer_top = Vec3::new(side * 0.5, 0.0, c.z);
            let outer_foot = Vec3::new(side * (0.5 + flare), 0.0, ground);
            let inner_top = Vec3::new(side * ri, 0.0, c.z);
            let inner_foot = Vec3::new(side * (ri - 0.5 * flare), 0.0, ground);
            // In-plane normal of a leg face: perpendicular to its run, toward `out` (±x).
            let face = |hi: Vec3, lo: Vec3, out: f32| {
                let v = hi - lo;
                let n = Vec3::new(v.z, 0.0, -v.x);
                if n.x * out < 0.0 { -n } else { n }
            };
            let t = half_t * (1.0 + a.leg_flare);
            pieces.push(Piece {
                planes: vec![
                    plane(face(outer_top, outer_foot, side), outer_foot),
                    plane(face(inner_top, inner_foot, -side), inner_foot),
                    Plane { n: Vec3::Y, d: t },
                    Plane { n: Vec3::NEG_Y, d: t },
                    Plane { n: Vec3::Z, d: top },
                    Plane { n: Vec3::NEG_Z, d: -ground },
                ],
                ellipsoid: None,
                hard: Vec::new(),
            });
            reach.x = reach.x.max(0.5 + flare);
            reach.y = reach.y.max(t);
        }
    }
    (pieces, Vec::new(), reach)
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // determinism: bit-equal by design
mod tests {
    use super::{FORMATIONS, generate_formation};
    use crate::BoulderParams;

    /// Coarser than the viewer (test time); the shapes are the same recipes.
    fn mesh() -> BoulderParams {
        BoulderParams { resolution_max: 64, ..BoulderParams::default() }
    }

    #[test]
    fn every_formation_is_closed_finite_and_sized() {
        for f in FORMATIONS {
            for seed in [1u64, 7] {
                let rock = generate_formation(f, &mesh(), f.size_m, seed);
                assert!(!rock.lods.is_empty(), "{} seed {seed}: no LODs", f.name);
                for (level, lod) in rock.lods.iter().enumerate() {
                    assert!(lod.triangle_count() > 0, "{} seed {seed} LOD{level}: empty", f.name);
                    assert!(lod.is_closed(), "{} seed {seed} LOD{level}: not closed", f.name);
                    assert!(lod.signed_volume() > 0.0, "{} seed {seed} LOD{level}: inside out", f.name);
                    assert!(lod.positions.iter().flatten().all(|x| x.is_finite()), "{} LOD{level}: NaN", f.name);
                }
                let e = rock.bounds.extent();
                let longest = e[0].max(e[1]).max(e[2]);
                assert!((longest - f.size_m).abs() < 1e-3 * f.size_m, "{}: longest side {longest} != {}", f.name, f.size_m);
            }
        }
    }

    #[test]
    fn towers_stand_up_and_the_arch_is_wide() {
        for f in FORMATIONS {
            let e = generate_formation(f, &mesh(), f.size_m, 3).bounds.extent();
            if f.name == "arch" {
                assert!(e[0] > e[2] && e[0] > 2.0 * e[1], "arch extents {e:?}");
            } else {
                assert!(e[2] > e[0] && e[2] > e[1], "{} is not tallest along +Z: {e:?}", f.name);
            }
        }
    }

    #[test]
    fn same_seed_same_formation() {
        for f in FORMATIONS {
            let (a, b) = (generate_formation(f, &mesh(), f.size_m, 5), generate_formation(f, &mesh(), f.size_m, 5));
            assert_eq!(a.hash, b.hash, "{}", f.name);
            assert_ne!(a.hash, generate_formation(f, &mesh(), f.size_m, 6).hash, "{}: seed ignored", f.name);
        }
    }

    #[test]
    fn aspect_spread_varies_per_seed_within_range() {
        use crate::boulder::spread_aspect;
        assert_eq!(spread_aspect([0.75, 0.55], 0.0, 9), [0.75, 0.55]);
        let all: Vec<[f32; 2]> = (0..64).map(|s| spread_aspect([0.75, 0.55], 1.0, s)).collect();
        assert!(all.iter().flatten().all(|v| (0.05..=1.0).contains(v)));
        let (lo, hi) = all.iter().fold((f32::MAX, 0.0f32), |(lo, hi), a| (lo.min(a[1]), hi.max(a[1])));
        assert!(lo < 0.35 && hi > 0.9, "short axis spread {lo}..{hi}");
        assert_eq!(spread_aspect([0.75, 0.55], 1.0, 4), spread_aspect([0.75, 0.55], 1.0, 4));
    }
}
