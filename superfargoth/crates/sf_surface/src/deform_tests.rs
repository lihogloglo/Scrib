//! The deformer capture's CPU twin (S1a): convert, ceiling composition, merge, window snap, the
//! view's texel mapping, sweep gaps, the ring pass's B channel and its packing.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::manual_midpoint)]

use glam::{DVec2, IVec2, Vec2, Vec3};

use crate::deform::*;
use crate::trail::{LIP_HEIGHT, PARAM_SLOTS, RING_SIZE, RING_TEXEL_M, TrailPass, cover_raise, window_min};

const RING: f32 = 0.4;
const GROUND: f32 = 2.0;
const NEAR: f32 = 1.75;
const RANGE: f32 = 1.0;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

/// The depth the view writes for a bottom at render Y `y` (reverse Z from the slab bottom).
fn depth_of(y: f32) -> f32 {
    (1.0 - (y - NEAR) / RANGE).clamp(0.0, 1.0)
}

/// The ceiling over the ground a bottom at `b` m over the ground leaves.
fn ceiling_for(b: f32) -> f32 {
    ceiling(convert(depth_of(GROUND + b), NEAR, RANGE, GROUND, RING), RING)
}

#[test]
fn convert_at_known_heights() {
    let h = ceiling_ref(RING);
    assert!(close(h, RING * (1.0 + LIP_HEIGHT)));
    // Bottom on the ground: the ceiling is the ground (a full carve).
    assert!(close(ceiling_for(0.0), 0.0));
    // Half the cover: half the cover is left.
    assert!(close(ceiling_for(RING / 2.0), RING / 2.0));
    // At the highest surface or above: nothing (B = 0).
    assert!(close(convert(depth_of(GROUND + h), NEAR, RANGE, GROUND, RING), 0.0));
    assert_eq!(convert(depth_of(GROUND + h + 0.1), NEAR, RANGE, GROUND, RING), 0.0);
    // Below the ground (the slab's bottom, and under it by depth clamp: d = 1): a full carve.
    assert!(close(ceiling_for(-0.1), 0.0));
    assert_eq!(convert(1.0, NEAR, RANGE, GROUND, RING), -h);
    // A cleared texel, and no ring: nothing.
    assert_eq!(convert(0.0, NEAR, RANGE, GROUND, RING), 0.0);
    assert_eq!(convert(0.5, NEAR, RANGE, GROUND, 0.0), 0.0);
}

/// S1a open point 2: the snow surface after carving never stands above the deformer's bottom and
/// never below what it would without it, whatever the bury and the channel depth. A leg 0.3 m
/// over the ground in 0.2 m of raised snow carves nothing; in 0.45 m it carves 0.15 m.
#[test]
fn ceiling_composes_after_a_partial_bury() {
    let ring = 0.6;
    let leg = 0.3;
    let c = (leg - ceiling_ref(ring)).clamp(-ceiling_ref(ring), 0.0);
    assert!(close(ceiling(c, ring), leg));
    assert!(close(capped_raise(0.2, c, ring), 0.2));
    assert!(close(capped_raise(0.45, c, ring), 0.3));
    // Over the bury's whole range (top 0: flat ground), on the snow channel (d = ring) and on a
    // shallower one (d = 0.2 in a 0.6 m ring).
    for d in [ring, 0.2] {
        for i in 0..=20 {
            let amount = i as f32 / 20.0;
            let raise = cover_raise(amount, 0.0, d, [0.0, 0.0], ring);
            let carved = capped_raise(raise, c, ring);
            assert!(carved <= leg + 1e-6, "above the leg: {carved} (amount {amount}, d {d})");
            assert!(close(carved, raise.min(leg)), "over-carved: {carved} vs {raise} (amount {amount}, d {d})");
        }
    }
    // Why not the spec's first sketch (R = bottom - ring depth, scaled by the bury): a partly
    // buried cover under a leg above it would still sink.
    let amount = 0.4;
    let raise = cover_raise(amount, 0.0, ring, [0.0, 0.0], ring);
    assert!(raise < leg, "a partial bury below the leg: {raise}");
    let sketch = cover_raise(amount, 0.0, ring, [leg - ring, 0.0], ring);
    assert!(sketch < raise - 0.01, "the sketch carves a cover the leg never touches: {sketch} < {raise}");
    // Lips stand under the ceiling too, and no ring means no cap.
    let lip = cover_raise(1.0, 0.0, ring, [0.0, LIP_HEIGHT * ring], ring);
    assert!(lip > ring);
    assert!(close(capped_raise(lip, c, ring), leg));
    assert!(close(capped_raise(lip, 0.0, ring), lip), "B = 0: above every raise");
    assert_eq!(capped_raise(0.7, -1.0, 0.0), 0.7);
}

/// The shading's cut follows the ceiling on a fully covered surface and keeps the print's where
/// it is lower; a ceiling over a thin cover cuts nothing.
#[test]
fn shading_cut_takes_the_lower_of_print_and_ceiling() {
    let ring = 0.6;
    let c = |b: f32| (b - ceiling_ref(ring)).clamp(-ceiling_ref(ring), 0.0);
    assert!(close(shade_height([0.0, 0.0, c(0.0)], ring, ring, 1.0), -ring));
    assert!(close(shade_height([0.0, 0.0, c(0.45)], ring, ring, 1.0), -0.15));
    assert!(close(shade_height([0.0, 0.0, c(0.3)], ring, ring, 0.33), 0.0));
    assert!(close(shade_height([-ring, 0.0, c(0.45)], ring, ring, 1.0), -ring));
    assert!(close(shade_height([0.0, 0.0, 0.0], ring, ring, 1.0), 0.0));
}

#[test]
fn merge_is_order_free_and_leaves_the_prints_alone() {
    let vals = [0.0, -0.1, -0.35, -0.02];
    for &a in &vals {
        assert_eq!(merge(a, 0.0), a);
        for &b in &vals {
            assert_eq!(merge(a, b), merge(b, a));
            for &c in &vals {
                assert_eq!(merge(merge(a, b), c), merge(a, merge(b, c)));
            }
        }
    }
    // Captures in any order give one ceiling; the stamps' R and G never see it (the composed
    // surface is the same min whichever came first).
    let caps = [-0.2, -0.05, 0.0];
    let fwd = caps.iter().fold(0.0, |b, &c| ceiling_update(b, 0.0, c));
    let back = caps.iter().rev().fold(0.0, |b, &c| ceiling_update(b, 0.0, c));
    assert_eq!(fwd, back);
    let stamped = cover_raise(1.0, 0.0, RING, [-0.1, 0.0], RING);
    assert!(close(capped_raise(stamped, fwd, RING), stamped.min(ceiling(-0.2, RING))));
    // Refill raises the ceiling like a print, never past 0.
    assert!(close(ceiling_update(-0.2, 0.05, 0.0), -0.15));
    assert_eq!(ceiling_update(-0.01, 0.05, 0.0), 0.0);
}

#[test]
fn window_snaps_to_whole_world_texels() {
    let t = RING_TEXEL_M;
    let size = capture_size(16.0, t);
    assert_eq!(size, 256);
    assert_eq!(capture_size(64.0, t), MAX_CAPTURE_SIZE);
    // Two centres in one world texel: one window.
    let a = capture_min(DVec2::new(10.001, -3.06), t, size);
    let b = capture_min(DVec2::new(10.06, -3.001), t, size);
    assert_eq!(a, b);
    assert_eq!(a, IVec2::new(160, -49) - IVec2::splat(128));
    // At the ring's texel the capture's texels are the ring's: the same world texel index.
    assert_eq!(capture_min(DVec2::new(10.0, 3.0), t, 1024), window_min(DVec2::new(10.0, 3.0)));
    // A world point keeps its target texel while the window holds still.
    let cap = |min: IVec2| Capture { corner: (min.as_dvec2() * t).as_vec2(), texel_m: t as f32, size, ..Capture::default() };
    let q = Vec2::new(12.3, -1.7);
    assert_eq!(cap(a).texel_of(q), cap(b).texel_of(q));
    assert!(cap(a).texel_of(Vec2::new(100.0, 0.0)).is_none());
}

/// The view's projection puts a world point in the texel `texel_of` names (column along +X, row
/// against Z), at depth 1 on the slab's bottom and 0 on its top.
#[test]
fn view_maps_texels_and_depth_as_the_convert_reads_them() {
    let ground = GroundGrid::flat(Vec2::new(-1.0, -1.0), 0.5, 20, GROUND);
    let cap = Capture { corner: Vec2::new(-0.5, 1.0), texel_m: 0.0625, size: 64, near_y: NEAR, range_m: RANGE, ground, press_m: 0.0, spread_m: 0.0 };
    let (_, clip) = cap.view();
    for q in [Vec2::new(-0.47, 1.03), Vec2::new(1.2, 3.9), Vec2::new(3.4, 1.53), Vec2::new(0.01, 2.01)] {
        // Drawn as the shadow pass rasterises it: over the whole target.
        let (px, d) = shadow_pass_raster(clip, Vec3::new(q.x, GROUND, q.y)).unwrap();
        let t = cap.texel_of(q).unwrap();
        assert_eq!(px, t, "{q}");
        assert!(close(d, depth_of(GROUND)));
    }
    let depth = |y: f32| {
        let p = clip * Vec3::new(0.5, y, 2.0).extend(1.0);
        p.z / p.w
    };
    assert!(close(depth(NEAR), 1.0));
    assert!(close(depth(NEAR + RANGE), 0.0));
    // A bottom read back from the target gives its height over the ground.
    let edge = MAX_CAPTURE_SIZE;
    let mut target = vec![0.0; (edge * edge) as usize];
    let q = Vec2::new(1.0, 2.0);
    let t = cap.texel_of(q).unwrap();
    target[(t.y * edge + t.x) as usize] = depth(GROUND + 0.1);
    assert!(close(ceiling(cap.value_at(q, &target, RING), RING), 0.1));
    assert_eq!(cap.value_at(q + Vec2::new(0.2, 0.0), &target, RING), 0.0, "cleared");
}

#[test]
fn ground_grid_is_bilinear_and_placed_on_world_steps() {
    let (first, edge) = GroundGrid::placement(DVec2::new(2.3, -7.9), 16.0, 0.5);
    assert_eq!((first, edge), (IVec2::new(3, -17), 35));
    let corner = Vec2::new(-4.0, 6.0);
    let plane = |p: Vec2| 0.3 * p.x - 0.1 * p.y + 5.0;
    let heights = (0..8u8).flat_map(|z| (0..8u8).map(move |x| (x, z))).map(|(x, z)| plane(corner + Vec2::new(f32::from(x), f32::from(z)) * 0.5)).collect();
    let g = GroundGrid { corner, step_m: 0.5, edge: 8, heights };
    for q in [Vec2::new(-3.9, 6.2), Vec2::new(-1.26, 8.4), Vec2::new(-0.5, 9.5)] {
        assert!(close(g.at(q), plane(q)), "{q}");
    }
    // Clamped at the edge.
    assert!(close(g.at(Vec2::new(-10.0, 6.0)), plane(corner)));
    assert_eq!(g.range(), (plane(corner + Vec2::new(0.0, 3.5)), plane(corner + Vec2::new(3.5, 0.0))));
    let (near, range) = slab(g.range(), 0.25, RING, 0.0);
    assert!(close(near, g.range().0 - 0.25));
    assert!(close(near + range, g.range().1 + ceiling_ref(RING)));
}

#[test]
fn sweep_gap_is_travel_past_the_width() {
    // A horse at 12 m/s, 30 fps, a 0.15 m leg: 0.4 m - 0.15 m = 0.25 m = 4 ring texels.
    assert!(close(sweep_gap(12.0, 1.0 / 30.0, 0.15, 0.0625), 4.0));
    // A walker at 1.5 m/s, 60 fps: within its width.
    assert_eq!(sweep_gap(1.5, 1.0 / 60.0, 0.15, 0.0625), 0.0);
    assert_eq!(sweep_gap(-12.0, 1.0 / 30.0, 0.15, 0.0625), sweep_gap(12.0, 1.0 / 30.0, 0.15, 0.0625));
    assert_eq!(sweep_gap(12.0, 1.0, 0.15, 0.0), 0.0);
}

/// The ring pass's B channel: the captured texel takes the ceiling, the rest keeps last frame's
/// refilled, entering rows clear; the packing carries the capture's three slots.
#[test]
fn pass_merges_the_capture_into_b() {
    let n = RING_SIZE;
    let idx = |a: IVec2| usize::try_from(a.y * n + a.x).unwrap();
    let min = window_min(DVec2::ZERO);
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let ground = GroundGrid::flat(Vec2::splat(-2.0), 0.5, 10, GROUND);
    let cap = Capture { corner: Vec2::splat(-1.0), texel_m: 0.0625, size: 32, near_y: NEAR, range_m: RANGE, ground, press_m: 0.0, spread_m: 0.0 };
    let edge = MAX_CAPTURE_SIZE;
    let mut target = vec![0.0; (edge * edge) as usize];
    let leg = Vec2::new(0.03, 0.03);
    let t = cap.texel_of(leg).unwrap();
    target[(t.y * edge + t.x) as usize] = depth_of(GROUND + 0.1);
    let pass = TrailPass { corner, depth_m: RING, new_min: min, old_min: Some(min), refill_m: 0.002, stamps: vec![], capture: Some(cap.clone()), press: crate::trail::Press::default() };
    let mut prev = vec![0.0; idx(IVec2::new(0, n))];
    let old = IVec2::new(500, 7);
    prev[idx(old)] = -0.2;
    let b = pass.run_ceiling(&prev, &target);
    let under = IVec2::ZERO.rem_euclid(IVec2::splat(n));
    assert!(close(ceiling(b[idx(under)], RING), 0.1));
    assert!(close(b[idx(old)], -0.198), "kept and refilled");
    let moved = TrailPass { new_min: min + IVec2::new(0, 1), corner: corner + Vec2::new(0.0, 0.0625), capture: None, ..pass.clone() };
    let entering = IVec2::new(500, (min.y + n).rem_euclid(n));
    prev[idx(entering)] = -0.3;
    assert_eq!(moved.run_ceiling(&prev, &target)[idx(entering)], 0.0);
    let s = pass.slots();
    assert_eq!(s.len(), PARAM_SLOTS);
    assert_eq!(s[2][3], 32);
    assert_eq!(f32::from_bits(s[4][3]), NEAR);
    assert!(close(f32::from_bits(s[5][1]), ceiling_ref(RING)));
    assert_eq!(f32::from_bits(s[6][1]), 10.0);
    assert_eq!(pass.ground_words().len(), 100);
    assert_eq!(TrailPass::default().ground_words(), vec![0.0]);
    assert_eq!(TrailPass::default().slots()[2][3], 0);
}

/// Where Bevy's shadow pass puts a render-space point `p` drawn with `clip`: the pass draws a
/// view over its **whole** depth attachment (vendored `bevy_pbr` `view_shadow_pass` begins the
/// render pass without a viewport; `ExtractedView::viewport` isn't read there), the capture's
/// [`MAX_CAPTURE_SIZE`]² target. Returns the pixel (column, row from the top) and the depth.
fn shadow_pass_raster(clip: glam::Mat4, p: Vec3) -> Option<(glam::UVec2, f32)> {
    let c = clip * p.extend(1.0);
    let ndc = c.truncate() / c.w;
    let e = MAX_CAPTURE_SIZE as f32;
    let px = Vec2::new((ndc.x + 1.0) * 0.5 * e, (1.0 - ndc.y) * 0.5 * e).floor();
    (px.min_element() >= 0.0 && px.max_element() < e).then(|| (px.as_uvec2(), ndc.z.clamp(0.0, 1.0)))
}

/// trail-offset (owner 2026-11-17, Skaal Village: "the trail is not centered on my character,
/// patches of deformed snow next to me"), end to end through the real placement, far from the
/// world origin (a floating origin km out and 256 m up): a leg's world points drawn by the
/// capture view as the shadow pass rasterises it land in the texel `texel_of` (and `trail.wgsl`
/// `captured`) reads, and the ring pass carves exactly the ring texels holding those world
/// points, which `layer.wesl` `surface_ring_at` reads back at the same render XZ.
#[test]
#[allow(clippy::cast_sign_loss)]
fn capture_carves_the_ring_under_the_leg_far_from_the_origin() {
    let origin = glam::DVec3::new(-30_720.0, 256.0, 46_080.0);
    let o_xz = DVec2::new(origin.x, origin.z);
    // The player's root (render space) and the ground under it (render Y, absolute 297.6 m).
    let focus = Vec3::new(3.3, 41.6, -7.9);
    let texel = RING_TEXEL_M;
    let size = capture_size(16.0, texel);
    let world = origin + focus.as_dvec3();
    // `deform::update_deform`'s placement.
    let min = capture_min(DVec2::new(world.x, world.z), texel, size);
    let corner = (min.as_dvec2() * texel - o_xz).as_vec2();
    let ground = GroundGrid::flat(corner - Vec2::splat(0.5), 0.5, 36, focus.y);
    let (near_y, range_m) = slab(ground.range(), 0.25, RING, 0.0);
    let cap = Capture { corner, texel_m: texel as f32, size, near_y, range_m, ground, press_m: 0.0, spread_m: 0.0 };
    let (_, clip) = cap.view();
    // Leg and hand points around the root (both feet, ahead and behind, left and right), a few
    // cm to 0.3 m over the ground.
    let mut legs = Vec::new();
    for dx in [-0.31, -0.12, 0.0, 0.14, 0.27] {
        for dz in [-0.22, -0.05, 0.09, 0.2] {
            for dy in [0.03, 0.12, 0.3] {
                legs.push(focus + Vec3::new(dx, dy, dz));
            }
        }
    }
    let edge = MAX_CAPTURE_SIZE;
    let mut target = vec![0.0f32; (edge * edge) as usize];
    for &p in &legs {
        let q = Vec2::new(p.x, p.z);
        let (px, d) = shadow_pass_raster(clip, p).expect("inside the capture view");
        assert_eq!(Some(px), cap.texel_of(q), "drawn at {px}, read at {:?} (render {p})", cap.texel_of(q));
        // Reverse Z, greater wins: the lowest point per texel.
        let i = (px.y * edge + px.x) as usize;
        target[i] = target[i].max(d);
    }
    // The ring around the camera, 3 m behind the player (`trail::update_trails`).
    let cam = focus + Vec3::new(0.0, 1.6, 3.0);
    let cam_world = origin + cam.as_dvec3();
    let new_min = window_min(DVec2::new(cam_world.x, cam_world.z));
    let ring_corner = (new_min.as_dvec2() * RING_TEXEL_M - o_xz).as_vec2();
    let pass = TrailPass { corner: ring_corner, depth_m: RING, new_min, old_min: Some(new_min), refill_m: 0.0, stamps: vec![], capture: Some(cap.clone()), press: crate::trail::Press::default() };
    let n = RING_SIZE;
    let b = pass.run_ceiling(&vec![0.0; (n * n) as usize], &target);
    // The ring texel that holds a world point: its world texel index, wrapped (`trail.wgsl`).
    let addr_of_world = |w: glam::DVec3| (DVec2::new(w.x, w.z) / RING_TEXEL_M).floor().as_ivec2().rem_euclid(IVec2::splat(n));
    // The texel `surface_ring_at` reads at render XZ `xz` (the uniform's `trail` and `trail_cells`).
    let addr_read = |xz: Vec2| {
        let t = (xz - ring_corner) * (1.0 / RING_TEXEL_M as f32);
        (t.floor().as_ivec2() + new_min.rem_euclid(IVec2::splat(n))).rem_euclid(IVec2::splat(n))
    };
    let mut want: Vec<IVec2> = Vec::new();
    for &p in &legs {
        let a = addr_of_world(origin + p.as_dvec3());
        assert_eq!(addr_read(Vec2::new(p.x, p.z)), a, "the shading reads another texel than the pass writes for {p}");
        assert!(b[(a.y * n + a.x) as usize] < 0.0, "no carve under the leg at render {p} (ring texel {a})");
        want.push(a);
    }
    want.sort_by_key(|a| (a.y, a.x));
    want.dedup();
    let mut carved: Vec<IVec2> =
        (0..n * n).filter(|&i| b[i as usize] < 0.0).map(|i| IVec2::new(i % n, i / n)).collect();
    carved.sort_by_key(|a| (a.y, a.x));
    assert_eq!(carved, want, "carved texels away from the leg");
}

/// GC-T2b: the capture's press (A) is 1 where a body touches the ground, 0.5 at half the press
/// height, 0 from it on; a texel the capture didn't hit leaves A as it was; B doesn't change.
#[test]
fn capture_presses_a_from_the_bodys_height_over_the_ground() {
    const PRESS: f32 = 0.5;
    let at = |b: f32| press_of(depth_of(GROUND + b), NEAR, RANGE, GROUND, PRESS);
    assert!(close(at(0.0), 1.0));
    assert!(close(at(-0.1), 1.0), "under the ground: full");
    assert!(close(at(PRESS * 0.5), 0.5));
    assert_eq!(at(PRESS), 0.0);
    assert_eq!(at(PRESS + 0.2), 0.0);
    assert_eq!(press_of(0.0, NEAR, RANGE, GROUND, PRESS), 0.0, "cleared texel");
    assert_eq!(press_of(depth_of(GROUND), NEAR, RANGE, GROUND, 0.0), 0.0, "no press demand");
    // Through the ring pass: a leg on the ground over one texel, the rest of the window cleared.
    let n = RING_SIZE;
    let idx = |a: IVec2| usize::try_from(a.y * n + a.x).unwrap();
    let min = window_min(DVec2::ZERO);
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let ground = GroundGrid::flat(Vec2::splat(-2.0), 0.5, 10, GROUND);
    let cap = Capture { corner: Vec2::splat(-1.0), texel_m: 0.0625, size: 32, near_y: NEAR, range_m: RANGE, ground, press_m: PRESS, spread_m: 0.0 };
    let edge = MAX_CAPTURE_SIZE;
    let mut target = vec![0.0; (edge * edge) as usize];
    let leg = Vec2::new(0.03, 0.03);
    let t = cap.texel_of(leg).unwrap();
    target[(t.y * edge + t.x) as usize] = depth_of(GROUND + 0.1);
    assert!(close(cap.press_at(leg, &target), 1.0 - crate::smoothstep(0.0, PRESS, 0.1)));
    let pass = TrailPass { corner, depth_m: RING, new_min: min, old_min: Some(min), refill_m: 0.0, stamps: vec![], capture: Some(cap.clone()), press: crate::trail::Press::default() };
    let mut prev = vec![0.0; idx(IVec2::new(0, n))];
    let beside = IVec2::new(1, 0);
    prev[idx(beside)] = 0.3;
    let a = pass.run_press(&prev, &target);
    assert!(close(a[idx(IVec2::ZERO)], cap.press_at(leg, &target)));
    assert_eq!(a[idx(beside)], 0.3, "no capture hit: A unchanged");
    // B: the same with or without the press (a cover look, ring depth 0.4).
    let no_press = TrailPass { capture: Some(Capture { press_m: 0.0, ..cap.clone() }), ..pass.clone() };
    let zeros = vec![0.0; prev.len()];
    assert_eq!(pass.run_ceiling(&zeros, &target), no_press.run_ceiling(&zeros, &target));
    assert!(close(ceiling(pass.run_ceiling(&zeros, &target)[idx(IVec2::ZERO)], RING), 0.1));
    assert_eq!(f32::from_bits(pass.slots()[6][2]), PRESS, "the press height rides in the ground slot");
    // Ring depth 0 (press only): B stays 0, A still presses.
    let bare = TrailPass { depth_m: 0.0, ..pass.clone() };
    assert!(bare.run_ceiling(&zeros, &target).iter().all(|b| *b == 0.0));
    assert!(close(bare.run_press(&zeros, &target)[idx(IVec2::ZERO)], a[idx(IVec2::ZERO)]));
}

/// GC-T2b: with a press demand the slab reaches the press height over the highest ground even
/// at ring depth 0; without one it is the cover's alone.
#[test]
fn slab_reaches_the_press_height_on_demand() {
    let g = (1.0, 1.6);
    let (near, h) = slab(g, 0.25, 0.0, 0.5);
    assert!(close(near, 0.75));
    assert!(near + h >= g.1 + 0.5 - 1e-6 && h >= 0.5, "{h}");
    assert!(close(near + h, g.1 + 0.5));
    // A deep cover wins over the press.
    assert!(close(near + slab(g, 0.25, RING, 0.1).1, g.1 + ceiling_ref(RING)));
    // No demand: as before.
    assert_eq!(slab(g, 0.25, RING, 0.0), (near, g.1 + ceiling_ref(RING) - near));
    assert!(close(slab(g, 0.25, 0.0, 0.0).1, g.1 - near));
}

/// GC-T3: a capture with a leg texel at ground height under each of `legs` (the rest cleared)
/// and the spread `spread`, plus its target.
fn spread_capture(legs: &[Vec2], spread: f32) -> (Capture, Vec<f32>) {
    let ground = GroundGrid::flat(Vec2::splat(-2.0), 0.5, 10, GROUND);
    let cap = Capture { corner: Vec2::splat(-1.0), texel_m: 0.0625, size: 32, near_y: NEAR, range_m: RANGE, ground, press_m: 1.0, spread_m: spread };
    let edge = MAX_CAPTURE_SIZE;
    let mut target = vec![0.0; (edge * edge) as usize];
    for leg in legs {
        let t = cap.texel_of(*leg).unwrap();
        target[(t.y * edge + t.x) as usize] = depth_of(GROUND);
    }
    (cap, target)
}

/// GC-T3: the tap layout: 24 taps, tap 0 at the centre, the rest inside the reach.
#[test]
fn spread_taps_fill_a_disk() {
    assert_eq!(spread_tap(0), Vec2::ZERO);
    assert!((spread_tap(SPREAD_TAPS - 1).length() - SPREAD_REACH).abs() < 1e-6);
    for i in 1..SPREAD_TAPS {
        let r = spread_tap(i).length();
        assert!(r > 0.0 && r <= SPREAD_REACH + 1e-6, "tap {i} at {r}");
    }
    assert_eq!(spread_weight(0.0, 0.0), 1.0);
    assert_eq!(spread_weight(0.1, 0.0), 0.0);
    assert!(close(spread_weight(0.25, 0.5), 0.5));
    assert_eq!(spread_weight(0.5, 0.5), 0.0);
}

/// GC-T3: spread 0 is the GC-T2b press exactly, everywhere (window and around it).
#[test]
fn spread_zero_is_the_bare_press() {
    let legs = [Vec2::new(0.03, 0.03), Vec2::new(0.4, -0.2)];
    let (cap, target) = spread_capture(&legs, 0.0);
    for z in -24..24 {
        for x in -24..24 {
            #[allow(clippy::cast_precision_loss)]
            let q = Vec2::new(x as f32, z as f32) * 0.05 + Vec2::splat(0.01);
            assert_eq!(cap.spread_press_at(q, &target), cap.press_at(q, &target), "at {q}");
        }
    }
    // The pass: the same ring as with the bare press, and the spread in the ground slot.
    let n = RING_SIZE;
    let min = window_min(DVec2::ZERO);
    #[allow(clippy::cast_possible_truncation)]
    let corner = min.as_vec2() * RING_TEXEL_M as f32;
    let pass = TrailPass { corner, depth_m: 0.0, new_min: min, old_min: Some(min), refill_m: 0.0, stamps: vec![], capture: Some(cap.clone()), press: crate::trail::Press::default() };
    let prev = vec![0.0; usize::try_from(n * n).unwrap()];
    let a = pass.run_press(&prev, &target);
    for (i, v) in a.iter().enumerate() {
        let i = i32::try_from(i).unwrap();
        let q = pass.texel_center(IVec2::new(i % n, i / n));
        assert_eq!(*v, crate::trail::press_update(0.0, q, &[], &pass.press, cap.press_at(q, &target)));
    }
    let spread = TrailPass { capture: Some(Capture { spread_m: 0.45, ..cap }), ..pass };
    assert_eq!(f32::from_bits(spread.slots()[6][3]), 0.45, "the spread rides in the ground slot");
}

/// GC-T3: one leg on the ground presses 1 at itself, ~0.5 at half the spread (where a tap lands
/// on it), 0 beyond the spread; two legs combine with Max; nothing outside the window.
#[test]
fn spread_presses_around_a_leg() {
    const SPREAD: f32 = 0.45;
    let leg = Vec2::new(0.03, 0.03);
    let (cap, target) = spread_capture(&[leg], SPREAD);
    assert!(close(cap.spread_press_at(leg, &target), 1.0), "at the leg");
    // Half the spread: the tap nearest 0.5 (tap 9 at 0.8 √(9/23) ≈ 0.5004) lands on the leg.
    let o = spread_tap(9) * SPREAD;
    assert!((o.length() / SPREAD - 0.5).abs() < 1e-3);
    let half = cap.spread_press_at(leg - o, &target);
    assert!((half - 0.5).abs() < 0.01, "half the spread: {half}");
    assert!(close(half, spread_weight(o.length(), SPREAD)));
    // Beyond the spread (past the leg's texel by the spread): nothing, in any direction.
    for k in 0..16 {
        #[allow(clippy::cast_precision_loss)]
        let a = k as f32 * std::f32::consts::TAU / 16.0;
        let q = leg + Vec2::new(a.cos(), a.sin()) * (SPREAD + 0.1);
        assert_eq!(cap.spread_press_at(q, &target), 0.0, "beyond at {q}");
    }
    // Two legs: Max of each alone.
    let other = Vec2::new(0.4, 0.03);
    let (both, t2) = spread_capture(&[leg, other], SPREAD);
    let (alone, t1) = spread_capture(&[other], SPREAD);
    for x in 0..12 {
        #[allow(clippy::cast_precision_loss)]
        let q = Vec2::new(-0.2 + x as f32 * 0.07, 0.1);
        let m = cap.spread_press_at(q, &target).max(alone.spread_press_at(q, &t1));
        assert_eq!(both.spread_press_at(q, &t2), m, "Max at {q}");
    }
    // Outside the window (it spans -1..1): nothing, though a leg inside is within the spread.
    let (edge_cap, te) = spread_capture(&[Vec2::new(-0.97, 0.03)], SPREAD);
    assert!(edge_cap.spread_press_at(Vec2::new(-0.97, 0.03), &te) > 0.99);
    assert_eq!(edge_cap.spread_press_at(Vec2::new(-1.05, 0.03), &te), 0.0, "outside the window");
}
