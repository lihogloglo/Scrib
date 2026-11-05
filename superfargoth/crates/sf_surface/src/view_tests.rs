// Exact compares: both sides run the same operations.
#![allow(clippy::float_cmp, clippy::cast_possible_wrap, clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::collections::HashSet;

use glam::{DVec2, IVec2, Vec3};
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
