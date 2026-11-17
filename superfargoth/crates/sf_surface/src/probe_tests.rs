// Exact compares: the grids are text, and the snapped centres run the same operations.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use glam::{DVec2, DVec3, Vec3};

use crate::probe::*;
use crate::view::{NEAR_LEVEL, SHELTER_SIZE, ShelterLevel};

const SIZE: usize = SHELTER_SIZE as usize;

fn far_level() -> ShelterLevel {
    ShelterLevel::far(512.0, 64.0)
}

fn empty() -> Vec<f32> {
    vec![0.0; SIZE * SIZE]
}

/// Draws a caster into a map centred at `center`: every texel whose centre lies in world
/// `[min, max)` (X, Z) gets the depth of a top at world height `top_y`, as the capture would
/// (reverse Z, the highest caster wins).
fn stamp(depth: &mut [f32], level: &ShelterLevel, center: DVec3, min: DVec2, max: DVec2, top_y: f64) {
    let texel = f64::from(level.texel_m());
    let half = f64::from(level.half_m);
    let d = ((top_y - (center.y - f64::from(level.down_m))) / f64::from(level.depth_m())) as f32;
    for r in 0..SIZE {
        let z = center.z - half + (r as f64 + 0.5) * texel;
        if z < min.y || z >= max.y {
            continue;
        }
        for c in 0..SIZE {
            let x = center.x - half + (c as f64 + 0.5) * texel;
            if x >= min.x && x < max.x {
                depth[r * SIZE + c] = depth[r * SIZE + c].max(d);
            }
        }
    }
}

/// A grid line's cells (after the row's Z label).
fn cells(line: &str) -> &str {
    line.rsplit(' ').next().unwrap()
}

fn grid(lines: &[String]) -> Vec<&str> {
    lines[1..].iter().map(|l| cells(l)).collect()
}

const CENTER: DVec3 = DVec3::new(100.0, 20.0, -200.0);
const CAMERA: DVec3 = DVec3::new(101.0, 20.0, -199.0);

fn near_window(center: DVec2) -> ProbeWindow {
    ProbeWindow { center, half_m: 2.0, cell_m: PROBE_NEAR_CELL_M }
}

/// A 1 m block with its top 5 m above the camera prints `3` in exactly its four cells; a
/// one-texel trunk shows in its cell over a lower caster there; the header names the window.
#[test]
fn caster_block_prints_in_its_cells() {
    let mut depth = empty();
    stamp(&mut depth, &NEAR_LEVEL, CENTER, DVec2::new(100.5, -200.5), DVec2::new(101.5, -199.5), 25.0);
    // The trunk: one texel (0.125 m) with its top 2 m up, in a cell whose other texels hold a
    // caster at the camera's height.
    stamp(&mut depth, &NEAR_LEVEL, CENTER, DVec2::new(102.0, -201.5), DVec2::new(102.5, -201.0), 20.0);
    stamp(&mut depth, &NEAR_LEVEL, CENTER, DVec2::new(102.25, -201.125), DVec2::new(102.375, -201.0), 22.0);
    let map = ProbeMap { name: "near solid", level: NEAR_LEVEL, center: CENTER, depth: &depth };
    let lines = probe_grid(&map, CAMERA, &near_window(DVec2::new(101.0, -200.0)));
    assert_eq!(
        lines[0],
        "near solid: first cell x 99.25 z -201.75, cell 0.5 m, 8x8 cells, map centre x 100.000 z -200.000 y 20.000, \
         camera x 101.00 y 20.00 z -199.00, 80 caster texels of 1048576"
    );
    assert_eq!(
        grid(&lines),
        ["........", "......1.", "........", "...33...", "...33...", "........", "........", "........"]
    );
    // Each row starts with its cell centre's world Z, increasing downward.
    assert_eq!(lines[1], "   -201.75 ........");
    assert_eq!(lines[8], "   -198.25 ........");
}

/// World anchoring: the same world caster captured in two maps whose centres differ (16 m in X,
/// 8 m in Z, 3 m in Y) prints the identical grid for the same window; only the header's map
/// centre differs. Both levels.
#[test]
fn same_world_caster_prints_the_same_from_two_centres() {
    let far = far_level();
    let cases = [
        (NEAR_LEVEL, PROBE_NEAR_CELL_M, 2.0, DVec2::new(100.5, -200.5), DVec2::new(101.5, -199.5), 25.0, "...33..."),
        (far, PROBE_FAR_CELL_M, 4.0, DVec2::new(100.0, -200.0), DVec2::new(102.0, -198.0), 30.0, "...55..."),
    ];
    for (level, cell_m, half_m, min, max, top, row) in cases {
        let window = ProbeWindow { center: DVec2::new(101.0, -199.5), half_m, cell_m };
        let print = |center: DVec3| {
            let mut depth = empty();
            stamp(&mut depth, &level, center, min, max, top);
            probe_grid(&ProbeMap { name: "map", level, center, depth: &depth }, CAMERA, &window)
        };
        let a = print(CENTER);
        let b = print(CENTER + DVec3::new(16.0, 3.0, 8.0));
        assert_eq!(a[1..], b[1..], "the grid moved with the map centre");
        assert_eq!(a.len(), 9);
        assert_eq!(grid(&a).iter().filter(|r| **r == row).count(), 2, "{a:#?}");
        assert_eq!(grid(&a).iter().filter(|r| **r == "........").count(), 6, "{a:#?}");
        // The headers: every field but the map centre is the same.
        let fields = |h: &str| h.split(", ").map(str::to_owned).collect::<Vec<_>>();
        let (ha, hb) = (fields(&a[0]), fields(&b[0]));
        assert_eq!(ha.len(), hb.len());
        for (fa, fb) in ha.iter().zip(&hb) {
            if fa.starts_with("map centre") {
                assert_ne!(fa, fb);
            } else {
                assert_eq!(fa, fb);
            }
        }
    }
}

/// Cells with no texel of the map print `#`, empty cells inside it `.`.
#[test]
fn outside_the_map_is_hash_and_empty_is_dot() {
    let depth = empty();
    let map = ProbeMap { name: "near solid", level: NEAR_LEVEL, center: CENTER, depth: &depth };
    // The map ends at x = 164 and z = -136.
    let east = probe_grid(&map, CAMERA, &near_window(DVec2::new(164.0, -200.0)));
    assert!(grid(&east).iter().all(|r| *r == "....####"), "{east:#?}");
    let corner = probe_grid(&map, CAMERA, &near_window(DVec2::new(164.0, -136.0)));
    assert_eq!(
        grid(&corner),
        ["....####", "....####", "....####", "....####", "########", "########", "########", "########"]
    );
    let west = probe_grid(&map, CAMERA, &near_window(DVec2::new(36.0, -264.0)));
    assert_eq!(
        grid(&west),
        ["########", "########", "########", "########", "####....", "####....", "####....", "####....",]
    );
    assert!(west[0].ends_with("0 caster texels of 1048576"), "{}", west[0]);
    // A depth array of the wrong size is reported, not read.
    let short = probe_grid(&ProbeMap { depth: &depth[..16], ..map }, CAMERA, &near_window(DVec2::ZERO));
    assert_eq!(short, ["near solid: 16 depth texels, expected 1048576"]);
}

/// Heights print relative to the camera's Y: 2 m digits from 1 m below it, `-` under that.
#[test]
fn height_buckets_follow_the_camera() {
    for (h, c) in [
        (-40.0, '-'),
        (-1.01, '-'),
        (-1.0, '0'),
        (0.99, '0'),
        (1.0, '1'),
        (2.99, '1'),
        (3.0, '2'),
        (16.99, '8'),
        (17.0, '9'),
        (300.0, '9'),
    ] {
        assert_eq!(height_char(h), c, "h = {h}");
    }
    let mut depth = empty();
    stamp(&mut depth, &NEAR_LEVEL, CENTER, DVec2::new(100.5, -200.5), DVec2::new(101.5, -199.5), 25.0);
    let map = ProbeMap { name: "near solid", level: NEAR_LEVEL, center: CENTER, depth: &depth };
    let window = near_window(DVec2::new(101.0, -200.0));
    // The top is at world y = 25: 5 m over a camera at 20, 1 m over one at 24, 6 m under one at 31.
    for (camera_y, row) in [(20.0, "...33..."), (24.0, "...11..."), (10.0, "...88..."), (-40.0, "...99..."), (31.0, "...--...")] {
        let lines = probe_grid(&map, DVec3::new(101.0, camera_y, -199.0), &window);
        assert_eq!(grid(&lines)[3], row, "camera y = {camera_y}");
    }
}

/// The centre read back from a level's clip is the centre the level was drawn around: X and Z
/// on the world texel grid exactly, Y to well under a height bucket; a floating origin far from
/// zero changes nothing.
#[test]
fn map_center_comes_back_from_the_clip() {
    for level in [NEAR_LEVEL, far_level()] {
        for origin in [DVec3::ZERO, DVec3::new(12_345.5, 100.25, -54_321.75)] {
            let p = Vec3::new(-431.3, 17.2, 250.9);
            let (center, texel) = level.center_world(origin, p);
            let (_, clip) = level.view(center);
            let c = map_center(&level, clip, origin);
            let t = f64::from(level.texel_m());
            assert_eq!(c.x, f64::from(texel.x) * t);
            assert_eq!(c.z, f64::from(texel.y) * t);
            assert!((c.y - (origin.y + f64::from(center.y))).abs() < 1e-3, "{c:?} vs {center:?}");
        }
    }
}
