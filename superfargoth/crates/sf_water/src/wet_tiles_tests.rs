#![allow(clippy::cast_precision_loss, clippy::float_cmp, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use glam::DVec2;

use crate::wet_tiles::{LEAF_TEXELS, Rect, WetTiles};

const TEXEL: f64 = 2.0;

fn tiles(width: usize, height: usize, wet: impl Fn(usize, usize) -> bool) -> (WetTiles, Vec<f32>) {
    let d: Vec<f32> = (0..width * height).map(|i| if wet(i % width, i / width) { 5.0 } else { -50.0 }).collect();
    (WetTiles::build(width, height, TEXEL, DVec2::new(100.0, -40.0), &d, -2.0), d)
}

fn all(t: &WetTiles) -> Vec<Rect> {
    let mut out = Vec::new();
    assert!(t.visit(0.0, usize::MAX, |_| true, &mut out));
    out
}

fn contains(r: &Rect, x: f32, z: f32) -> bool {
    x >= r[0] && x <= r[2] && z >= r[1] && z <= r[3]
}

/// An all-sea field is one rectangle, the field itself; an all-land field has none.
#[test]
fn open_sea_is_one_box_and_land_none() {
    let (sea, _) = tiles(37, 21, |_, _| true);
    assert_eq!(all(&sea), vec![[0.0, 0.0, 74.0, 42.0]]);
    let (land, _) = tiles(37, 21, |_, _| false);
    assert!(all(&land).is_empty());
    assert_eq!(land.rects(), 0);
}

/// Every wet texel lies in a box, and every leaf a box covers holds a wet texel or lies inside a
/// fully wet node (no box over a dry leaf).
#[test]
fn boxes_cover_every_wet_texel_and_no_dry_leaf() {
    let (w, h) = (70, 45);
    // A diagonal coast, a lake and a river.
    let wet = |x: usize, z: usize| x + z < 40 || (x as i32 - 55).pow(2) + (z as i32 - 30).pow(2) < 30 || x == 47;
    let (t, _) = tiles(w, h, wet);
    let boxes = all(&t);
    assert!(boxes.len() < (w / LEAF_TEXELS + 1) * (h / LEAF_TEXELS + 1), "merged: {} boxes", boxes.len());
    for z in 0..h {
        for x in 0..w {
            let (cx, cz) = ((x as f64 + 0.5) * TEXEL, (z as f64 + 0.5) * TEXEL);
            let inside = boxes.iter().any(|b| contains(b, cx as f32, cz as f32));
            if wet(x, z) {
                assert!(inside, "wet texel {x},{z} outside every box");
            }
        }
    }
    let leaf = (LEAF_TEXELS as f64 * TEXEL) as f32;
    for b in &boxes {
        // Each box is made of leaves; each of its leaves has a wet texel.
        let (lx0, lz0) = ((b[0] / leaf) as usize, (b[1] / leaf) as usize);
        let (lx1, lz1) = ((b[2] / leaf).ceil() as usize, (b[3] / leaf).ceil() as usize);
        for lz in lz0..lz1 {
            for lx in lx0..lx1 {
                let any = (lz * LEAF_TEXELS..((lz + 1) * LEAF_TEXELS).min(h)).any(|z| (lx * LEAF_TEXELS..((lx + 1) * LEAF_TEXELS).min(w)).any(|x| wet(x, z)));
                assert!(any, "box {b:?} covers dry leaf {lx},{lz}");
            }
        }
    }
}

/// The predicate culls whole subtrees; the margin grows every box; the cap stops the walk.
#[test]
fn visit_culls_grows_and_caps() {
    let (t, _) = tiles(64, 64, |x, z| x < 3 || z < 3);
    let mut out = Vec::new();
    assert!(t.visit(0.0, usize::MAX, |_| false, &mut out));
    assert!(out.is_empty());
    // Only the boxes touching x > 100 m: the strip along z = 0 reaches there, the x strip doesn't.
    assert!(t.visit(0.0, usize::MAX, |r| r[2] > 100.0, &mut out));
    assert!(!out.is_empty() && out.iter().all(|r| r[2] > 100.0 && r[1] < 1.0));
    let mut grown = Vec::new();
    assert!(t.visit(3.0, usize::MAX, |_| true, &mut grown));
    let plain = all(&t);
    assert_eq!(grown.len(), plain.len());
    assert!(grown.iter().all(|g| plain.iter().any(|p| [p[0] - 3.0, p[1] - 3.0, p[2] + 3.0, p[3] + 3.0] == *g)));
    let mut capped = Vec::new();
    assert!(!t.visit(0.0, 1, |_| true, &mut capped));
    assert_eq!(capped.len(), 1);
}

/// The threshold is the land margin: a texel 3 m inland counts at -4 m, not at -2 m. The exterior
/// strips tile the plane around the field with it.
#[test]
fn threshold_and_exterior() {
    let mut d = vec![-50.0_f32; 16 * 16];
    d[5 * 16 + 5] = -3.0;
    let at = |threshold| WetTiles::build(16, 16, TEXEL, DVec2::ZERO, &d, threshold).rects();
    assert_eq!(at(-2.0), 0);
    assert_eq!(at(-4.0), 1);
    let (t, _) = tiles(16, 16, |_, _| false);
    let ext = t.exterior(1000.0);
    for (x, z) in [(-5.0, 10.0), (40.0, 10.0), (10.0, -5.0), (10.0, 40.0), (-500.0, -500.0), (900.0, 900.0)] {
        assert!(ext.iter().any(|r| contains(r, x, z)), "{x},{z} outside the strips");
    }
    assert!(!ext.iter().any(|r| x_inside(r, 10.0, 10.0)), "a strip overlaps the field");
}

fn x_inside(r: &Rect, x: f32, z: f32) -> bool {
    x > r[0] && x < r[2] && z > r[1] && z < r[3]
}
