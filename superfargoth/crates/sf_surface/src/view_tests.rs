// Exact compares: both sides run the same operations.
#![allow(clippy::float_cmp, clippy::cast_possible_wrap, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::collections::HashSet;

use glam::{DVec2, DVec3, IVec2, Vec3};
use sf_records::RecordId;
use sf_weather::{ClimateIx, ClimateRaster};

use crate::view::*;
use crate::{CellGrid, Channel, SurfaceState};

fn cells(r: &[CellRect]) -> Vec<IVec2> {
    let mut out = Vec::new();
    for r in r {
        for z in 0..r.size.y {
            for x in 0..r.size.x {
                out.push(r.min + IVec2::new(x, z));
            }
        }
    }
    out
}

/// The entering strips are exactly new window minus old window, each cell once.
fn check_recenter(from: IVec2, to: IVec2) {
    let mut w = CellWindow::default();
    w.recenter(from);
    let old = w.rect().unwrap();
    let strips = w.recenter(to);
    let new = w.rect().unwrap();
    assert_eq!(new.min, to - IVec2::splat(WINDOW_CELLS / 2));
    let got = cells(&strips);
    let set: HashSet<IVec2> = got.iter().copied().collect();
    assert_eq!(set.len(), got.len(), "strips overlap ({from} -> {to})");
    let want: HashSet<IVec2> = cells(&[new])
        .into_iter()
        .filter(|c| !old.contains(*c))
        .collect();
    assert_eq!(set, want, "{from} -> {to}");
}

#[test]
fn recenter_writes_exactly_the_entering_strips() {
    let mut w = CellWindow::default();
    let first = w.recenter(IVec2::new(5, -7));
    assert_eq!(
        first,
        vec![CellRect {
            min: IVec2::new(5 - 64, -7 - 64),
            size: IVec2::splat(128)
        }]
    );
    assert!(
        w.recenter(IVec2::new(5, -7)).is_empty(),
        "same centre: nothing"
    );
    for d in [
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(0, 3),
        IVec2::new(0, -2),
        IVec2::new(2, -5),
        IVec2::new(-127, 127),
        IVec2::new(64, 64),
    ] {
        check_recenter(IVec2::new(-300, 40), IVec2::new(-300, 40) + d);
    }
    // One cell east: one column of 128 texels.
    let mut w = CellWindow::default();
    w.recenter(IVec2::ZERO);
    let s = w.recenter(IVec2::X);
    assert_eq!(
        s,
        vec![CellRect {
            min: IVec2::new(64, -64),
            size: IVec2::new(1, 128)
        }]
    );
    // A jump of a window or more rewrites all of it.
    let s = w.recenter(IVec2::new(500, 0));
    assert_eq!(s.iter().map(|r| r.area()).sum::<i32>(), 128 * 128);
    w.reset();
    assert_eq!(w.recenter(IVec2::new(500, 0)).len(), 1);
}

#[test]
fn texel_rects_split_at_the_wrap_and_cover_each_cell_once() {
    let r = CellRect {
        min: IVec2::new(-10, 120),
        size: IVec2::new(20, 30),
    };
    let t = CellWindow::texel_rects(r);
    assert_eq!(t.len(), 4);
    let mut seen = HashSet::new();
    for tr in &t {
        assert!(
            tr.texel.x + tr.size.x <= 128 && tr.texel.y + tr.size.y <= 128,
            "{tr:?}"
        );
        assert_eq!(
            tr.first_cell.rem_euclid(IVec2::splat(128)).as_uvec2(),
            tr.texel
        );
        for z in 0..tr.size.y as i32 {
            for x in 0..tr.size.x as i32 {
                assert!(seen.insert(tr.first_cell + IVec2::new(x, z)));
            }
        }
    }
    assert_eq!(seen, cells(&[r]).into_iter().collect());
    // A whole window maps onto the whole texture, in at most 4 pieces.
    let whole = CellWindow::texel_rects(CellRect {
        min: IVec2::new(-64, 1000),
        size: IVec2::splat(128),
    });
    assert_eq!(
        whole.iter().map(|t| t.size.x * t.size.y).sum::<u32>(),
        128 * 128
    );
}

#[test]
fn texels_blend_each_cells_climates_and_carry_cover() {
    // Climate 0 west, 1 east, blended over 128 m around x = 256 (cells 16x8 over 512x256 m).
    let raster = ClimateRaster::new(
        DVec2::ZERO,
        256.0,
        2,
        1,
        &[Some(ClimateIx(0)), Some(ClimateIx(1))],
        128.0,
    )
    .unwrap();
    let ws = RecordId::new("ws");
    let grid = CellGrid::new(ws.clone(), &raster);
    let mut s = SurfaceState::default();
    s.set_climate(ClimateIx(0), Channel::Wet, 1.0);
    s.set_climate(ClimateIx(1), Channel::Puddle, 0.5);
    s.set_cell(&ws, IVec2::new(3, 2), Channel::Snow, 0.25);
    s.set_cell(&ws, IVec2::new(3, 2), Channel::Ash, 1.0);
    let cover = s.cells.get(&ws);
    assert_eq!(
        cell_texel(&s, CellSites::Grid(&grid), cover, IVec2::new(0, 0)),
        [255, 0, 0, 0]
    );
    assert_eq!(
        cell_texel(&s, CellSites::Grid(&grid), cover, IVec2::new(15, 0)),
        [0, 128, 0, 0]
    );
    assert_eq!(
        cell_texel(&s, CellSites::Grid(&grid), cover, IVec2::new(3, 2)),
        [255, 0, 64, 255]
    );
    // At the border both climates weigh: the texel is the CPU sample at the cell centre.
    let c = IVec2::new(8, 3);
    let p = (c.as_dvec2() + 0.5) * crate::CELL_M;
    let cpu = crate::sample(&s, &raster, &ws, p);
    let t = cell_texel(&s, CellSites::Grid(&grid), cover, c);
    assert!(
        (f32::from(t[0]) / 255.0 - cpu.wet).abs() <= 0.5 / 255.0 + 1e-6,
        "{t:?} {cpu:?}"
    );
    assert!(
        (f32::from(t[1]) / 255.0 - cpu.puddle).abs() <= 0.5 / 255.0 + 1e-6,
        "{t:?} {cpu:?}"
    );
    // Outside the grid: dry; without a grid: the uniform site.
    assert_eq!(
        cell_texel(&s, CellSites::Grid(&grid), cover, IVec2::new(-1, 0)),
        [0, 0, 0, 0]
    );
    let site = [(ClimateIx(0), 0.5), (ClimateIx(1), 0.5)];
    assert_eq!(
        cell_texel(&s, CellSites::Uniform(&site), None, IVec2::new(-1, 0)),
        [128, 64, 0, 0]
    );
    // A filled strip: row-major from its first cell, channel maxima for the layer flags.
    let mut out = Vec::new();
    let max = fill_texels(
        &s,
        CellSites::Grid(&grid),
        Some(&ws),
        TexelRect {
            texel: glam::UVec2::new(2, 1),
            size: glam::UVec2::new(3, 2),
            first_cell: IVec2::new(2, 1),
        },
        &mut out,
    );
    assert_eq!(out.len(), 3 * 2 * 4);
    assert_eq!(
        &out[4 * 4..4 * 5],
        &[255, 0, 64, 255],
        "cell (3, 2) is row 1, column 1"
    );
    assert_eq!(max, [255, 0, 64, 255]);
}

#[test]
fn shelter_view_is_vertical_and_covers_its_square() {
    let center = shelter_center(Vec3::new(10.03, 5.0, -3.07));
    assert_eq!(
        center,
        Vec3::new(10.0, 5.0, -3.125),
        "snapped to 0.125 m texels"
    );
    let (view, clip) = shelter_view(center);
    // A roof straight above projects onto the same texel, higher (reverse Z).
    let ground = clip.project_point3(center);
    let roof = clip.project_point3(center + Vec3::Y * 4.0);
    assert!(
        ground.truncate().distance(roof.truncate()) < 1e-6 && roof.z > ground.z,
        "{ground} {roof}"
    );
    // Depth is linear in height: 4 m over the 350 m range.
    assert!((roof.z - ground.z - 4.0 / (SHELTER_UP_M + SHELTER_DOWN_M)).abs() < 1e-5);
    // View X is world X; the square's corners land on the clip edges.
    assert!((view.transform_point3(center + Vec3::X).x - 1.0).abs() < 1e-5);
    for (dx, dz) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
        let c =
            clip.project_point3(center + Vec3::new(dx * SHELTER_HALF_M, 0.0, dz * SHELTER_HALF_M));
        assert!(
            (c.x.abs() - 1.0).abs() < 1e-5 && (c.y.abs() - 1.0).abs() < 1e-5,
            "{c}"
        );
    }
    // Top and bottom planes.
    assert!((clip.project_point3(center + Vec3::Y * SHELTER_UP_M).z - 1.0).abs() < 1e-5);
    assert!(
        clip.project_point3(center - Vec3::Y * SHELTER_DOWN_M)
            .z
            .abs()
            < 1e-5
    );
    assert_eq!(SHELTER_TEXEL_M, 0.125);
}

#[test]
fn shelter_redraws_on_move_climb_and_streaming_only() {
    let mut t = ShelterTrigger::default();
    let p = Vec3::new(100.0, 20.0, 50.0);
    assert_eq!(t.frame(p, false), Some(Redraw::Fresh));
    assert_eq!(t.center(), Some(shelter_center(p)));
    // Steady and small moves: nothing.
    for i in 0..100 {
        assert_eq!(
            t.frame(p + Vec3::new(15.0 * (i as f32 / 100.0), 7.0, 0.0), false),
            None
        );
    }
    // Past 16 m horizontally.
    assert_eq!(
        t.frame(p + Vec3::new(12.0, 0.0, 11.0), false),
        Some(Redraw::Moved)
    );
    let c = t.center().unwrap();
    // Past 8 m vertically.
    assert_eq!(t.frame(c + Vec3::Y * 8.5, false), Some(Redraw::Moved));
    // Streamed casters: at most every 30 frames, and the centre stays.
    let c = t.center().unwrap();
    assert_eq!(t.frame(c, true), None);
    for _ in 0..27 {
        assert_eq!(t.frame(c, false), None);
    }
    assert_eq!(t.frame(c, false), None, "29 frames");
    assert_eq!(t.frame(c, false), Some(Redraw::Streamed), "30 frames");
    assert_eq!(t.center(), Some(c));
    assert_eq!(t.frame(c, false), None, "served");
    t.reset();
    assert_eq!(t.frame(c, false), Some(Redraw::Fresh));
}

/// SW-5: the centre snaps to the world texel grid, so a floating-origin shift with the camera at
/// the same world point gives the same world texel (and a whole-texel render centre), and two
/// redraws differ by whole texels.
#[test]
fn shelter_centre_snaps_to_the_world_grid() {
    let p = Vec3::new(10.03, 5.0, -3.07);
    assert_eq!(shelter_center_world(DVec3::ZERO, p).0, shelter_center(p), "origin 0: the render snap");
    // An origin off the texel grid (0.05 m) and one cell further: same world point, same texel.
    let a = DVec3::new(256.05, 0.0, -512.0);
    let b = a + DVec3::new(256.0, 0.0, 0.0);
    let (ca, ta) = shelter_center_world(a, p);
    let (cb, tb) = shelter_center_world(b, p - Vec3::new(256.0, 0.0, 0.0));
    assert_eq!(ta, tb);
    let world = |o: DVec3, c: Vec3| (o.x + f64::from(c.x), o.z + f64::from(c.z));
    let (wa, wb) = (world(a, ca), world(b, cb));
    assert!((wa.0 - wb.0).abs() < 1e-4 && (wa.1 - wb.1).abs() < 1e-4, "{wa:?} {wb:?}");
    assert!((wa.0 / 0.125 - f64::from(ta.x)).abs() < 1e-3, "on the world grid: {wa:?}");
    // 16 m east: 128 texels, whatever the origin.
    let (_, tc) = shelter_center_world(a, p + Vec3::new(16.0, 0.0, 0.0));
    assert_eq!(tc - ta, IVec2::new(128, 0));
    let mut t = ShelterTrigger::default();
    assert_eq!(t.frame_world(a, p, false), Some(Redraw::Fresh));
    assert_eq!((t.center(), t.center_texel()), (Some(ca), Some(ta)));
}

/// SW-5: a redraw's pass reads last pack's texel at `texel + shift`: a world-anchored texel keeps
/// its value bit for bit; the texels the old map did not cover take the fresh value as target and
/// start from open sky (what they showed).
#[test]
fn exposure_shift_keeps_world_texels_and_seeds_new_ones() {
    let n = SHELTER_SIZE as i32;
    // The map moved 3 texels east and 2 north: new texel t holds world texel t + new_min, last
    // pack held it at t + shift.
    let shift = IVec2::new(3, -2);
    assert_eq!(exposure_source(IVec2::new(0, 5), shift), Some(IVec2::new(3, 3)));
    assert_eq!(exposure_source(IVec2::new(n - 3, 5), shift), None, "entered from the east");
    assert_eq!(exposure_source(IVec2::new(4, 1), shift), None, "entered from the south");
    assert_eq!(exposure_source(IVec2::new(n - 4, n - 1), shift), Some(IVec2::new(n - 1, n - 3)));
    assert_eq!(exposure_source(IVec2::new(7, 7), IVec2::splat(n)), None, "a jump of a map: all new");
    // A whole pack over a world pattern, moved by `shift` (factor 1: the take-in frame). Steady
    // texels (lag 0) whose world point keeps its fresh value come out bit for bit; entering
    // texels take the fresh value as target and show open sky (lag = 1 - fresh).
    let carry = ExposurePass { mode: ExposureMode::Carry(shift), decay: 1.0 };
    let world = |w: IVec2| ((w.x * 7 + w.y * 13).rem_euclid(97) as f32) / 96.0;
    let (old_min, new_min) = (IVec2::new(-500, 40), IVec2::new(-500, 40) + shift);
    let old: Vec<(f32, f32)> = (0..n * n).map(|i| (0.0, world(IVec2::new(i % n, i / n) + old_min))).collect();
    for y in (0..n).step_by(7) {
        for x in (0..n).step_by(5) {
            let t = IVec2::new(x, y);
            let held = exposure_source(t, shift).map(|s| old[(s.y * n + s.x) as usize]);
            let fresh = world(t + new_min);
            let (lag, target) = exposure_texel(carry, held, fresh);
            assert_eq!(target.to_bits(), fresh.to_bits());
            match held {
                Some(h) => {
                    assert_eq!(h.1.to_bits(), fresh.to_bits(), "the same world texel at {t}");
                    assert_eq!((lag.to_bits(), target.to_bits()), (0.0_f32.to_bits(), h.1.to_bits()), "bit for bit at {t}");
                }
                None => assert_eq!(lag, 1.0 - fresh, "entering texel {t} shows open sky"),
            }
        }
    }
    // In mid-ease, what a texel shows is carried (lag + target), to rounding.
    let held = (0.137_f32, 0.402_f32);
    let (lag, fresh) = exposure_texel(carry, Some(held), 0.9);
    assert_eq!(fresh, 0.9);
    assert!((lag + fresh - (held.0 + held.1)).abs() < 1e-6, "shown kept");
    // Same map (streamed casters): nothing changes where nothing changed.
    let same = ExposurePass { mode: ExposureMode::Carry(IVec2::ZERO), decay: 1.0 };
    assert_eq!(exposure_texel(same, Some((0.0, 0.25)), 0.25), (0.0, 0.25));
    // A new texel: target fresh, shown open sky (1), eased from there.
    assert_eq!(exposure_texel(carry, None, 0.3), (0.7, 0.3));
    // Fresh: no lag. Ease: the lag decays, the target stays.
    let fresh_pass = ExposurePass { mode: ExposureMode::Fresh, decay: 1.0 };
    assert_eq!(exposure_texel(fresh_pass, Some(held), 0.6), (0.0, 0.6));
    let ease = ExposurePass { mode: ExposureMode::Ease, decay: 0.5 };
    assert_eq!(exposure_texel(ease, Some((0.4, 0.2)), 0.9), (0.2, 0.2));
    assert_eq!((ExposureMode::Fresh.code(), carry.mode.code(), ease.mode.code()), (0, 1, 2));
    assert_eq!((carry.mode.shift(), ease.mode.shift()), (shift, IVec2::ZERO));
}

/// SW-5: the ease is frame-rate independent and shows ~95 % of a change after 2.4 s; the clock
/// takes a redraw in the frame after its request, eases for `EXPOSURE_RUN_S`, ends on factor 0
/// and then runs no pass.
#[test]
fn exposure_ease_is_frame_rate_independent() {
    // N small steps == one big step.
    for (n, dt) in [(60, 1.0 / 60.0), (144, 1.0 / 144.0), (7, 0.31)] {
        let product: f32 = (0..n).map(|_| ease_decay(dt)).product();
        let big = ease_decay(dt * n as f32);
        assert!((product - big).abs() < 1e-5, "{n} x {dt}: {product} vs {big}");
    }
    assert!(1.0 - ease_decay(3.0 * EXPOSURE_TAU_S) > 0.949, "95 % after 3 tau");
    assert!((3.0 * EXPOSURE_TAU_S - 2.4).abs() < 1e-6);
    // The clock at two frame rates: the remaining lag after 2.4 s matches.
    let run = |dt: f32| {
        let mut c = ExposureClock::default();
        assert_eq!(c.frame(dt), None, "nothing held");
        c.redraw(IVec2::new(10, 10));
        assert!(c.pending());
        assert_eq!(c.frame(dt), Some(ExposurePass { mode: ExposureMode::Fresh, decay: 1.0 }));
        assert_eq!(c.frame(dt), None, "a fresh map has no lag to ease");
        c.redraw(IVec2::new(140, 10));
        assert_eq!(c.frame(dt), Some(ExposurePass { mode: ExposureMode::Carry(IVec2::new(130, 0)), decay: 1.0 }));
        assert_eq!(c.held(), Some(IVec2::new(140, 10)));
        let mut lag = 1.0_f32;
        let mut t = 0.0;
        while t + dt <= 2.4 + 1e-4 {
            let p = c.frame(dt).expect("easing");
            assert_eq!(p.mode, ExposureMode::Ease);
            lag *= p.decay;
            t += dt;
        }
        lag
    };
    let (slow, fast) = (run(1.0 / 30.0), run(1.0 / 120.0));
    assert!((slow - fast).abs() < 1e-3 && slow < 0.051, "{slow} {fast}");
    // The run ends with a snap to 0, then steady frames run nothing.
    let mut c = ExposureClock::default();
    c.redraw(IVec2::ZERO);
    c.frame(0.1);
    c.redraw(IVec2::ONE);
    c.frame(0.1);
    let mut last = None;
    for _ in 0..1000 {
        match c.frame(0.1) {
            Some(p) => last = Some(p),
            None => break,
        }
    }
    assert_eq!(last.map(|p| p.decay), Some(0.0), "the last pass snaps the lag");
    assert_eq!(c.frame(0.1), None);
    c.reset();
    assert_eq!(c.held(), None);
}

#[test]
fn noise_atlas_tiles_and_spans_its_range() {
    let a = noise_atlas();
    let n = NOISE_SIZE as usize;
    assert_eq!(a.len(), n * n);
    assert_eq!(a, noise_atlas(), "deterministic");
    // Tiling: the jump across the wrap is no bigger than the largest step inside.
    for ch in 0..2 {
        let step = |x0: usize, x1: usize, y: usize| {
            (i32::from(a[y * n + x0][ch]) - i32::from(a[y * n + x1][ch])).abs()
        };
        let inner = (0..n)
            .map(|y| (0..n - 1).map(|x| step(x, x + 1, y)).max().unwrap())
            .max()
            .unwrap();
        let wrap = (0..n).map(|y| step(n - 1, 0, y)).max().unwrap();
        assert!(wrap <= inner, "channel {ch}: wrap {wrap} > inner {inner}");
        let (lo, hi) = a
            .iter()
            .fold((255, 0), |(lo, hi), t| (lo.min(t[ch]), hi.max(t[ch])));
        assert!(lo < 96 && hi > 160, "channel {ch} spans {lo}..{hi}");
    }
    assert_eq!(DebugView::parse("Exposure"), Some(DebugView::Exposure));
    assert_eq!(DebugView::parse("nope"), None);
}

#[test]
#[ignore = "measurement: CPU cost of a full 128² cell window fill (a step or console change)"]
fn measure_full_window_fill() {
    // A Vvardenfell-sized grid (~30k cells over 6 x 5.5 km) with every cell covered.
    let raster = ClimateRaster::new(
        DVec2::ZERO,
        256.0,
        24,
        22,
        &[Some(ClimateIx(0)); 24 * 22],
        256.0,
    )
    .unwrap();
    let ws = RecordId::new("ws");
    let grid = CellGrid::new(ws.clone(), &raster);
    let mut s = SurfaceState::default();
    s.set_climate(ClimateIx(0), Channel::Wet, 0.7);
    for ix in 0..grid.len() {
        s.set_cell(&ws, grid.cell(ix), Channel::Ash, 0.5);
    }
    let mut w = CellWindow::default();
    let rect = w.recenter(IVec2::new(90, 90))[0];
    let runs = 20;
    let t0 = std::time::Instant::now();
    let mut out = Vec::new();
    for _ in 0..runs {
        out.clear();
        for r in CellWindow::texel_rects(rect) {
            fill_texels(&s, CellSites::Grid(&grid), Some(&ws), r, &mut out);
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0 / f64::from(runs);
    println!(
        "full window fill: {ms:.3} ms ({} active cells, {} bytes)",
        s.active_cells(),
        out.len()
    );
}

/// SW-5e: the far level is the near capture scaled: 1024² over ±512 m (1 m texels), its depth
/// range its half extent each way, its vertical recentre and streamed-redraw interval in the near
/// level's ratios; shelter is known (unfaded for any camera inside one recentre step) out to
/// 0.875 x 512 - 64 = 384 m, the near level's 40 m. The near level is the old shelter map.
#[test]
fn far_level_scales_the_near_capture() {
    let far = ShelterLevel::far(512.0, 64.0);
    assert_eq!((far.texel_m(), far.up_m, far.down_m, far.recenter_y_m, far.refresh_frames), (1.0, 512.0, 512.0, 32.0, 120));
    assert_eq!(far.known_m(), 384.0);
    assert_eq!((NEAR_LEVEL.texel_m(), NEAR_LEVEL.known_m()), (SHELTER_TEXEL_M, 40.0));
    let c = shelter_center(Vec3::new(10.03, 5.0, -3.07));
    assert_eq!(NEAR_LEVEL.view(c), shelter_view(c));
    // The far centre snaps to whole metres of the world grid.
    let (center, texel) = far.center_world(DVec3::new(256.0, 0.0, -512.0), Vec3::new(10.4, 3.2, -3.6));
    assert_eq!((center, texel), (Vec3::new(10.0, 3.0, -4.0), IVec2::new(266, -516)));
    let (_, clip) = far.view(center);
    for (dx, dz) in [(1.0, 1.0), (-1.0, -1.0)] {
        let p = clip.project_point3(center + Vec3::new(dx * 512.0, 0.0, dz * 512.0));
        assert!((p.x.abs() - 1.0).abs() < 1e-5 && (p.y.abs() - 1.0).abs() < 1e-5, "{p}");
    }
    assert!((clip.project_point3(center + Vec3::Y * 512.0).z - 1.0).abs() < 1e-5);
    // Moves inside a step redraw nothing; past it the level recentres.
    let mut t = ShelterTrigger::default();
    assert_eq!(t.frame_level(&far, DVec3::ZERO, Vec3::ZERO, false), Some(Redraw::Fresh));
    assert_eq!(t.frame_level(&far, DVec3::ZERO, Vec3::new(63.0, 30.0, 0.0), false), None);
    assert_eq!(t.frame_level(&far, DVec3::ZERO, Vec3::new(65.0, 0.0, 0.0), false), Some(Redraw::Moved));
    for _ in 0..119 {
        assert_eq!(t.frame_level(&far, DVec3::ZERO, Vec3::new(65.0, 0.0, 0.0), true), None);
    }
    assert_eq!(t.frame_level(&far, DVec3::ZERO, Vec3::new(65.0, 0.0, 0.0), false), Some(Redraw::Streamed));
}

/// A test world for the levels' twin: a tree crown (a disc of 4 m radius over the ground) at
/// `TREE`. A level's ground exposure at `p` is bilinear between its texel centres, each 0 under
/// the crown and 1 outside (the layer's far test; the near tent is finer still, and both read a
/// fixed world grid: the centres snap to whole texels).
const TREE: Vec3 = Vec3::new(300.0, 0.0, -20.0);

fn level_ground(level: &ShelterLevel, center: Vec3, p: Vec3) -> f32 {
    let t = level.texel_m();
    let covered = |x: f32, z: f32| if Vec3::new(x, 0.0, z).distance(TREE) < 4.0 { 0.0 } else { 1.0 };
    // Texel centres sit at (k + 0.5) texels from the snapped centre (an even texel count).
    let g = (p - center) / t - 0.5;
    let (fx, fz) = (g.x.floor(), g.z.floor());
    let (bx, bz) = (g.x - fx, g.z - fz);
    let at = |i: f32, j: f32| covered(center.x + (fx + i + 0.5) * t, center.z + (fz + j + 0.5) * t);
    let lo = at(0.0, 0.0) + (at(1.0, 0.0) - at(0.0, 0.0)) * bx;
    let hi = at(0.0, 1.0) + (at(1.0, 1.0) - at(0.0, 1.0)) * bx;
    lo + (hi - lo) * bz
}

/// What the layer shows at `p` for a camera that walked from the far side to `camera` (both
/// levels recentred by their triggers on the way, as in the game).
fn shown_from(camera: Vec3, p: Vec3) -> f32 {
    let far = ShelterLevel::far(512.0, 64.0);
    let (mut near_t, mut far_t) = (ShelterTrigger::default(), ShelterTrigger::default());
    let start = camera + Vec3::X * 300.0;
    for i in 0..=300 {
        let c = start.lerp(camera, i as f32 / 300.0);
        near_t.frame_level(&NEAR_LEVEL, DVec3::ZERO, c, false);
        far_t.frame_level(&far, DVec3::ZERO, c, false);
    }
    let (nc, fc) = (near_t.center().unwrap(), far_t.center().unwrap());
    let near = (NEAR_LEVEL.edge(nc, p), level_ground(&NEAR_LEVEL, nc, p));
    let far_v = (far.edge(fc, p), level_ground(&far, fc, p));
    blended_exposure(Some(near), Some(far_v))
}

/// SW-5e (owner 2026-11-16: snow under trees still sank as the camera neared): a ground point
/// under a far tree reads the same shelter from cameras 5, 60 and 200 m away (near level, the
/// band, far level): the levels hold the same world texels whatever their centres; open sky only
/// beyond the far level. Points under the crown's rim, where the near level's finer texels differ
/// from the far ones, move by at most the band weight times that difference.
#[test]
fn exposure_under_a_far_tree_holds_as_the_camera_nears() {
    let under = TREE + Vec3::new(1.2, 0.0, -0.7);
    for d in [5.0, 60.0, 200.0, 380.0] {
        let camera = under - Vec3::X * d + Vec3::Y * 1.7;
        assert_eq!(shown_from(camera, under), 0.0, "camera {d} m away");
    }
    assert_eq!(shown_from(under - Vec3::X * 700.0, under), 1.0, "beyond the far level: open");
    // The far level alone (no near map): the same.
    let far = ShelterLevel::far(512.0, 64.0);
    let fc = far.center_world(DVec3::ZERO, under - Vec3::X * 200.0).0;
    assert_eq!(blended_exposure(None, Some((far.edge(fc, under), level_ground(&far, fc, under)))), 0.0);
    // The rim: the near and far values differ; the shown one lies between them, by the band.
    let rim = TREE + Vec3::new(3.8, 0.0, 0.3);
    let camera = rim - Vec3::X * 60.0;
    let nc = NEAR_LEVEL.center_world(DVec3::ZERO, camera).0;
    let fc = far.center_world(DVec3::ZERO, camera).0;
    let (n, f) = (level_ground(&NEAR_LEVEL, nc, rim), level_ground(&far, fc, rim));
    let w = level_fade(NEAR_LEVEL.edge(nc, rim));
    let shown = blended_exposure(Some((NEAR_LEVEL.edge(nc, rim), n)), Some((far.edge(fc, rim), f)));
    assert!((shown - (n + (f - n) * w)).abs() < 1e-6, "{n} {f} {w} {shown}");
    // The levels' texel grids coincide in the world (whole far texels are whole near texels).
    assert_eq!((far.texel_m() / NEAR_LEVEL.texel_m()).fract(), 0.0);
    // No map at all: open sky.
    assert_eq!(blended_exposure(None, None), 1.0);
}

/// SW-5e: a carried texel shows exactly what it showed, whatever the band moved under it
/// (`exposure_texel_in` against `shown_exposure`, factor 1); a texel the old map did not reach
/// showed the next level out; a single level with open sky outside is SW-5's `exposure_texel`.
#[test]
fn carried_texels_keep_what_they_showed_across_the_band() {
    let carry = ExposurePass { mode: ExposureMode::Carry(IVec2::new(128, 0)), decay: 1.0 };
    let outer = 0.3;
    for (held, fade) in [((0.0, 0.1, 0.0), 0.6), ((0.12, 0.4, 0.9), 0.0), ((-0.05, 0.9, 0.5), 1.0), ((0.0, 0.0, 1.0), 0.25)] {
        let fresh = 0.55;
        let (lag, target) = exposure_texel_in(carry, Some(held), fresh, fade, outer);
        assert_eq!(target, fresh);
        let before = shown_exposure(held.0, held.1, held.2, outer);
        let after = shown_exposure(lag, target, fade, outer);
        assert!((before - after).abs() < 1e-6, "{held:?} {fade}: {before} {after}");
    }
    let (lag, fresh) = exposure_texel_in(carry, None, 0.8, 0.0, outer);
    assert!((shown_exposure(lag, fresh, 0.0, outer) - outer).abs() < 1e-6, "entered: what the far level showed");
    let held = (0.137_f32, 0.402_f32);
    assert_eq!(exposure_texel(carry, Some(held), 0.9), exposure_texel_in(carry, Some((held.0, held.1, 0.0)), 0.9, 0.0, 1.0));
    assert_eq!((level_fade(0.5), level_fade(SHELTER_FADE_START), level_fade(1.0), level_fade(3.0)), (0.0, 0.0, 1.0, 1.0));
}

/// SW-5e: the near pass's uv into the far pack (`outer_uv_map`) maps every near texel onto the far
/// pack's uv of the same world point, for any pair of centres; the raise's reach is the known
/// shelter or the casters, whichever ends first, and unlimited with no shelter map.
#[test]
fn near_texels_map_onto_the_far_pack_and_the_reach_is_the_nearer_bound() {
    let far = ShelterLevel::far(512.0, 64.0);
    let (nc, fc) = (Vec3::new(100.25, 3.0, -40.5), Vec3::new(64.0, 0.0, 0.0));
    let (near_clip, far_clip) = (NEAR_LEVEL.view(nc).1, far.view(fc).1);
    let (scale, offset) = outer_uv_map(near_clip, NEAR_LEVEL.half_m, far_clip, far.half_m);
    assert_eq!(scale, 0.125);
    let uv = |clip: glam::Mat4, p: Vec3| {
        let c = clip.project_point3(p);
        glam::Vec2::new(c.x * 0.5 + 0.5, -c.y * 0.5 + 0.5)
    };
    for p in [Vec3::new(100.0, 0.0, -40.0), Vec3::new(160.0, 7.0, -100.0), Vec3::new(40.5, -3.0, 20.0)] {
        let mapped = uv(near_clip, p) * scale + offset;
        assert!((mapped - uv(far_clip, p)).length() < 1e-5, "{p}: {mapped} {}", uv(far_clip, p));
    }
    assert_eq!(raise_reach_m(None, 360.0), f32::INFINITY);
    assert_eq!(raise_reach_m(Some(far.known_m()), 360.0), 360.0);
    assert_eq!(raise_reach_m(Some(NEAR_LEVEL.known_m()), 360.0), 40.0);
}
