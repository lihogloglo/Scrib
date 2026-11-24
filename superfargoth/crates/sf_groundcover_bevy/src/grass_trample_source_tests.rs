//! Exact tests of the vendored `bevy_grass` external trample texture (slice GC-3d1, own patch "trample source"):
//! the CPU twin of `generate.wesl` `source_push` (one texel at the blade root, forward differences one texel +X and
//! +Z, torus addressing, channel, edge fade), which built-in trample passes run, and the uniform the shader gets.
//! Here because cargo cannot test the vendored crate, which is not a workspace member.

// Exact by design: the presses compared are exactly representable.
#![allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_wrap)]

use bevy::math::{IVec4, UVec2, Vec2, Vec4};
use bevy_grass::superfargoth_test::{built_in_trample_passes, trample_source_uniform, trample_texture_push};
use bevy_grass::{GrassTrampleLookup, GrassTramplePlacement};

/// Texels per side of the test window.
const SIZE: u32 = 16;
/// 25 cm texels.
const TPM: f32 = 4.0;
const CORNER: Vec2 = Vec2::new(10.0, -20.0);

fn placement(first_texel: UVec2) -> GrassTramplePlacement {
    GrassTramplePlacement { corner: CORNER, texels_per_metre: TPM, size: SIZE, first_texel }
}

/// A hard-edged lookup reading `channel`.
fn lookup(channel: u32) -> GrassTrampleLookup {
    GrassTrampleLookup { channel, edge_margin: 0.0, edge_fade: 0.0, ..GrassTrampleLookup::default() }
}

/// The centre of window texel (x, z), in metres.
fn root(x: i32, z: i32) -> Vec2 {
    CORNER + (Vec2::new(x as f32, z as f32) + 0.5) / TPM
}

/// A `SIZE`² RGBA texture whose window texels `presses` (window x, z, press) are written to `channel`, stored as a
/// torus whose first corner is at `first`; the other channels hold noise.
fn texture(presses: &[(u32, u32, f32)], channel: usize, first: UVec2) -> Vec<[f32; 4]> {
    let mut t: Vec<[f32; 4]> = (0..SIZE * SIZE).map(|i| [0.9, 0.7, 0.3, 0.6].map(|v| if i % 3 == 0 { v } else { 0.0 })).collect();
    for v in &mut t {
        v[channel] = 0.0;
    }
    for &(x, z, p) in presses {
        let tx = (x + first.x) % SIZE;
        let tz = (z + first.y) % SIZE;
        t[(tz * SIZE + tx) as usize][channel] = p;
    }
    t
}

/// The edge of a footprint whose peak is at -X: press 0.5 at (10, 8), 0.25 one texel +X, 0.5 one texel +Z.
const SLOPE: &[(u32, u32, f32)] = &[(10, 8, 0.5), (11, 8, 0.25), (10, 9, 0.5), (9, 8, 0.75)];

/// The push the twin must give on `SLOPE` at (10, 8) for a blade facing 0 turns (own direction +Z).
fn slope_push() -> Vec2 {
    let away = Vec2::new(0.25, 0.0) + Vec2::new(0.0, 1.0) * (1.0 / 256.0);
    away / away.length() * 0.5
}

fn close(a: Vec2, b: Vec2) -> bool {
    (a - b).abs().max_element() < 1e-6
}

#[test]
fn a_blade_lies_down_the_press_slope_away_from_the_peak() {
    let t = texture(SLOPE, 0, UVec2::ZERO);
    let push = trample_texture_push(&t, SIZE, &placement(UVec2::ZERO), &lookup(0), root(10, 8), 0.0);
    assert!(close(push, slope_push()), "{push}");
    assert!(push.x > 0.0, "away from the peak at -X");
    assert!((push.length() - 0.5).abs() < 1e-6, "as long as the press");
}

#[test]
fn a_torus_window_reads_as_the_plain_one() {
    for first in [UVec2::new(5, 3), UVec2::new(15, 15), UVec2::new(8, 0)] {
        let t = texture(SLOPE, 0, first);
        let push = trample_texture_push(&t, SIZE, &placement(first), &lookup(0), root(10, 8), 0.0);
        assert!(close(push, slope_push()), "first {first}: {push}");
        // An unreduced first texel is the same torus.
        let unreduced = trample_texture_push(&t, SIZE, &placement(first + SIZE), &lookup(0), root(10, 8), 0.0);
        assert_eq!(unreduced, push);
    }
}

#[test]
fn the_press_is_read_from_the_lookup_channel() {
    let t = texture(SLOPE, 3, UVec2::ZERO);
    let push = trample_texture_push(&t, SIZE, &placement(UVec2::ZERO), &lookup(3), root(10, 8), 0.0);
    assert!(close(push, slope_push()), "{push}");
    // The press channel of the other lookup is empty there.
    let mut zeroed = t;
    for v in &mut zeroed {
        v[0] = 0.0;
    }
    assert_eq!(trample_texture_push(&zeroed, SIZE, &placement(UVec2::ZERO), &lookup(0), root(10, 8), 0.0), Vec2::ZERO);
}

#[test]
fn where_the_press_is_flat_the_blade_lies_its_own_way() {
    let flat: Vec<(u32, u32, f32)> = (4..8).flat_map(|x| (4..8).map(move |z| (x, z, 0.75))).collect();
    let t = texture(&flat, 0, UVec2::ZERO);
    for facing in [0.0, 0.25, 0.6] {
        let push = trample_texture_push(&t, SIZE, &placement(UVec2::ZERO), &lookup(0), root(5, 5), facing);
        let turn = facing * std::f32::consts::TAU;
        assert!(close(push, Vec2::new(turn.sin(), turn.cos()) * 0.75), "facing {facing}: {push}");
    }
    // The push is the press times the strength, at most 1.
    let strong = GrassTrampleLookup { strength: 2.0, ..lookup(0) };
    let push = trample_texture_push(&t, SIZE, &placement(UVec2::ZERO), &strong, root(5, 5), 0.0);
    assert!(close(push, Vec2::new(0.0, 1.0)), "{push}");
}

#[test]
fn nothing_is_pressed_outside_the_window_or_unplaced() {
    // Every texel pressed fully.
    let all = vec![[1.0; 4]; (SIZE * SIZE) as usize];
    let at = |p: GrassTramplePlacement, point: Vec2| trample_texture_push(&all, SIZE, &p, &lookup(0), point, 0.0);
    let p = placement(UVec2::ZERO);
    assert!(at(p, root(3, 3)).length() > 0.99);
    for outside in [root(-1, 3), root(3, -1), root(16, 3), root(3, 16), CORNER - 0.01] {
        assert_eq!(at(p, outside), Vec2::ZERO, "{outside}");
    }
    assert_eq!(at(GrassTramplePlacement::NONE, root(3, 3)), Vec2::ZERO);
    assert_eq!(at(GrassTramplePlacement { size: 0, ..p }, root(3, 3)), Vec2::ZERO);
    assert_eq!(at(GrassTramplePlacement { texels_per_metre: 0.0, ..p }, root(3, 3)), Vec2::ZERO);
    // No press: no push.
    let none = vec![[0.0; 4]; (SIZE * SIZE) as usize];
    assert_eq!(trample_texture_push(&none, SIZE, &p, &lookup(0), root(3, 3), 0.0), Vec2::ZERO);
}

#[test]
fn the_press_fades_in_from_the_window_edge() {
    let all = vec![[1.0; 4]; (SIZE * SIZE) as usize];
    let faded = GrassTrampleLookup { edge_margin: 0.25, edge_fade: 1.0, ..lookup(0) };
    let p = placement(UVec2::ZERO);
    // 0.25 m in (texel 0 at x is 0.125 m in): inside the margin.
    assert_eq!(trample_texture_push(&all, SIZE, &p, &faded, CORNER + Vec2::new(0.125, 2.0), 0.0), Vec2::ZERO);
    // 0.75 m from the nearest edge: half faded in.
    let push = trample_texture_push(&all, SIZE, &p, &faded, CORNER + Vec2::new(0.75, 2.0), 0.0);
    assert!((push.length() - 0.5).abs() < 1e-6, "{push}");
    // 2 m in (the window's middle): fully.
    let push = trample_texture_push(&all, SIZE, &p, &faded, CORNER + 2.0, 0.0);
    assert!((push.length() - 1.0).abs() < 1e-6, "{push}");
}

#[test]
fn an_external_source_skips_the_built_in_trample_passes() {
    assert_eq!(built_in_trample_passes(false, 0), (true, false));
    assert_eq!(built_in_trample_passes(false, 3), (true, true));
    assert_eq!(built_in_trample_passes(true, 0), (false, false));
    assert_eq!(built_in_trample_passes(true, 3), (false, false));
}

#[test]
fn the_uniform_holds_the_reduced_torus_corner_and_the_channel() {
    let now = placement(UVec2::new(21, 3));
    let (n, nt, t, tt, l) = trample_source_uniform(&now, &GrassTramplePlacement::NONE, &GrassTrampleLookup { channel: 7, edge_fade: -1.0, ..GrassTrampleLookup::default() });
    assert_eq!(n, Vec4::new(CORNER.x, CORNER.y, TPM, 0.0));
    assert_eq!(nt, IVec4::new(5, 3, SIZE as i32, 3), "first texel below the size, channel clamped");
    assert_eq!((t, tt), (Vec4::ZERO, IVec4::new(0, 0, 0, 3)), "not placed: size 0");
    assert_eq!(l, Vec4::new(1.0, 0.0, 0.0, 1.0 / 256.0));
}
