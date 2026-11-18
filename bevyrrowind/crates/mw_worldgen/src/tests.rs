//! A synthetic 2×2-cell exterior: `LAND` in (0, 0), (1, 0), (0, 1), none in (1, 1). Heights are a ramp east,
//! `(10 + 64·gx + i) · 8` units at vertex `i` of cell `gx`, so every shared edge agrees and mosaic sample
//! `(col, row)` holds `meters((10 + col) · 8)`. Floats compare exactly on purpose: the dump copies, never blends.
#![allow(clippy::float_cmp, clippy::naive_bytecount)]

use std::io::Cursor;

use mw_records::keys;
use mw_world::terrain::fill_height;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{Cell, CellFlags, Landscape, LandscapeFlags, Region, VertexHeights};

use crate::{CELL_QUADS, dump, meters, read_meta, write};

fn ramp_land(grid: (i32, i32)) -> Landscape {
    let mut data = Box::new([[1i8; 65]; 65]);
    for row in data.iter_mut() {
        row[0] = 0;
    }
    #[allow(clippy::cast_precision_loss)]
    let offset = (10 + 64 * grid.0) as f32;
    Landscape {
        grid,
        landscape_flags: LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS,
        vertex_heights: VertexHeights { offset, data },
        ..Landscape::default()
    }
}

fn flat_land(grid: (i32, i32)) -> Landscape {
    Landscape {
        grid,
        landscape_flags: LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS,
        vertex_heights: VertexHeights { offset: 0.0, data: Box::new([[0; 65]; 65]) },
        ..Landscape::default()
    }
}

fn exterior(grid: (i32, i32), region: Option<&str>) -> Cell {
    let mut cell = Cell { region: region.map(str::to_owned), ..Cell::default() };
    cell.data.grid = grid;
    cell
}

fn region(id: &str, name: &str, color: [u8; 4]) -> Region {
    Region { id: id.into(), name: name.into(), map_color: color, ..Region::default() }
}

fn db_with(lands: Vec<Landscape>) -> RecordDb {
    let mut db = RecordDb::new();
    mw_records::register_kinds(&mut db).unwrap();
    let esm = db.add_source("Test.esm", &[]);
    for land in lands {
        db.define(esm, keys::landscape(&land), land).unwrap();
    }
    // Both regions ask for the same map colour: the second must get another.
    db.define(esm, RecordId::new("Ascadian Isles Region"), region("Ascadian Isles Region", "Ascadian Isles", [10, 20, 30, 0]))
        .unwrap();
    db.define(esm, RecordId::new("Red Mountain Region"), region("Red Mountain Region", "Red Mountain", [10, 20, 30, 0]))
        .unwrap();
    let mut cells = vec![
        exterior((0, 0), Some("Ascadian Isles Region")),
        exterior((1, 0), Some("ascadian isles region")),
        exterior((0, 1), Some("Red Mountain Region")),
        exterior((1, 1), None),
        // Outside the LAND box: ignored.
        exterior((5, 5), Some("Sheogorad Region")),
    ];
    let mut room = Cell { name: "Room".into(), region: Some("Ghost Region".into()), ..Cell::default() };
    room.data.flags = CellFlags::IS_INTERIOR;
    cells.push(room);
    for cell in cells {
        db.define(esm, keys::cell(&cell), cell).unwrap();
    }
    db
}

fn fixture() -> RecordDb {
    db_with(vec![ramp_land((0, 0)), ramp_land((1, 0)), ramp_land((0, 1))])
}

fn ramp(col: usize) -> f32 {
    #[allow(clippy::cast_precision_loss)]
    meters((10 + col) as f32 * 8.0)
}

#[test]
fn mosaic_dims_edges_and_mask() {
    let d = dump(&fixture(), 100.0).unwrap();
    let m = &d.mosaic;
    assert_eq!((m.cells_min, m.cells_max), ((0, 0), (1, 1)));
    assert_eq!((m.width, m.height), (2 * CELL_QUADS + 1, 2 * CELL_QUADS + 1));
    assert_eq!((m.land_cells, m.without_heights, m.edge_mismatches), (3, 0, 0));
    assert_eq!(m.cell_origin((0, 1)), (0, 0));
    assert_eq!(m.cell_origin((1, 0)), (64, 64));
    for row in 0..m.height {
        for col in 0..m.width {
            let k = row * m.width + col;
            // Cell (1, 1) (north-east) has no LAND: cols 65.., rows ..64. Its west and south edges are its
            // neighbours' LAND.
            let filled = col > 64 && row < 64;
            assert_eq!(m.mask[k], u8::from(!filled), "mask at ({col}, {row})");
            let want = if filled { fill_height() } else { ramp(col) };
            assert_eq!(m.heights[k], want, "height at ({col}, {row})");
        }
    }
    assert_eq!(m.mask.iter().filter(|&&v| v == 0).count(), 64 * 64);
    // North-up: the south-west cell's south-west vertex is the bottom-left sample.
    assert_eq!(m.mw_units(0, 128), (0.0, 0.0));
    assert_eq!(m.mw_units(0, 0), (0.0, 16384.0));
    assert_eq!(m.cell_of(128, 128), (1, 0));
    assert_eq!(m.cell_of(64, 63), (1, 1));
}

#[test]
fn shared_edge_disagreement_keeps_the_first_cell() {
    let d = dump(&db_with(vec![ramp_land((0, 0)), flat_land((1, 0))]), 100.0).unwrap();
    let m = &d.mosaic;
    assert_eq!((m.width, m.height), (129, 65));
    assert_eq!(m.edge_mismatches, 65);
    assert_eq!(m.max_edge_mismatch_m, ramp(64));
    assert_eq!(m.at(64, 0), ramp(64));
    assert_eq!(m.at(65, 0), 0.0);
}

#[test]
fn region_indices_and_legend() {
    let d = dump(&fixture(), 100.0).unwrap();
    let r = &d.regions;
    assert_eq!((r.width, r.height), (2, 2));
    // Rows north first: (0, 1) (1, 1) / (0, 0) (1, 0).
    assert_eq!(r.index, vec![2, 0, 1, 1]);
    assert_eq!((r.of((0, 0)), r.of((1, 0)), r.of((0, 1)), r.of((1, 1))), (1, 1, 2, 0));
    let ids: Vec<_> = r.legend.iter().map(|e| (e.index, e.id.as_str(), e.name.as_str(), e.missing)).collect();
    assert_eq!(ids, vec![
        (0, "", "", false),
        (1, "Ascadian Isles Region", "Ascadian Isles", false),
        (2, "Red Mountain Region", "Red Mountain", false),
    ]);
    assert_eq!(r.legend[0].color, [0, 0, 0]);
    assert_eq!(r.legend[1].color, [10, 20, 30]);
    assert_ne!(r.legend[2].color, r.legend[1].color);
    assert_ne!(r.legend[2].color, [0, 0, 0]);
}

#[test]
fn stats_summit_regions_histogram() {
    let d = dump(&fixture(), 100.0).unwrap();
    let s = &d.stats;
    assert_eq!((s.land_cells, s.box_cells), (3, [2, 2]));
    assert_eq!(s.above_sea_samples, 129 * 129 - 64 * 64);
    assert_eq!(s.above_sea_box, Some([0, 0, 128, 128]));
    assert_eq!(s.histogram, vec![(0.0, 129 * 129 - 64 * 64)]);
    let summit = s.summit.as_ref().unwrap();
    assert_eq!((summit.height_m, summit.sample, summit.cell, summit.region), (ramp(128), [128, 64], [1, 0], 1));
    assert_eq!(summit.mw_units, [16384.0, 8192.0]);
    let rows: Vec<_> = s.regions.iter().map(|r| (r.index, r.cells, r.land_samples, r.heights_m)).collect();
    assert_eq!(rows, vec![
        // (1, 1) has no LAND; it owns its west edge (col 64, rows 0..64), which is (0, 1)'s LAND.
        (0, 1, 64, Some([ramp(64); 4])),
        // Rows 64..=128 (the box's south edge closes the bottom cells), cols 0..=128: 65 per column.
        (1, 2, 65 * 129, Some([ramp(0), ramp(4192 / 65), ramp(7965 / 65), ramp(128)])),
        (2, 1, 64 * 64, Some([ramp(0), ramp(31), ramp(60), ramp(63)])),
    ]);
    // Finer bins: every LAND sample lands in exactly one.
    let fine = dump(&fixture(), 5.0).unwrap().stats.histogram;
    assert_eq!(fine.iter().map(|b| b.1).sum::<usize>(), 129 * 129 - 64 * 64);
    assert_eq!(fine.first().unwrap().0, 0.0);
    assert_eq!(fine.last().unwrap().0, 15.0);
}

#[test]
fn files_round_trip() {
    let d = dump(&fixture(), 100.0).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let meta = write(&d, dir.path()).unwrap();
    assert_eq!(read_meta(dir.path()).unwrap(), meta);
    assert_eq!((meta.dims, meta.region_dims, meta.origin_cell, meta.origin_mw_units), ([129, 129], [2, 2], [0, 1], [0.0, 16384.0]));
    assert_eq!((meta.min_m, meta.max_m, meta.fill_m), (fill_height(), ramp(128), fill_height()));
    assert_eq!(meta.plugins, vec!["Test.esm".to_owned()]);
    assert_eq!(meta.stats, d.stats);

    let raw = std::fs::read(dir.path().join("heights.f32")).unwrap();
    assert_eq!(blake3::hash(&raw).to_hex().as_str(), meta.heights_blake3);
    let heights: Vec<f32> = raw.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect();
    assert_eq!(heights, d.mosaic.heights);

    let read_png = |name: &str| {
        let bytes = std::fs::read(dir.path().join(name)).unwrap();
        let mut decoder = png::Decoder::new(Cursor::new(bytes));
        decoder.set_transformations(png::Transformations::IDENTITY);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        let palette = reader.info().palette.as_ref().map(|p| p.to_vec());
        ((info.width, info.height, info.color_type, info.bit_depth), buf, palette)
    };
    let (info, gray, _) = read_png("heights.png");
    assert_eq!(info, (129, 129, png::ColorType::Grayscale, png::BitDepth::Sixteen));
    let gray: Vec<u16> = gray.as_chunks::<2>().0.iter().map(|b| u16::from_be_bytes(*b)).collect();
    // Min = the fill of cell (1, 1) (north-east), max = the ramp top on the east edge of (1, 0).
    assert_eq!((gray[128], gray[64 * 129 + 128]), (0, 65535));
    let (info, mask, _) = read_png("mask.png");
    assert_eq!(info, (129, 129, png::ColorType::Grayscale, png::BitDepth::Eight));
    assert_eq!(mask, d.mosaic.mask.iter().map(|&v| v * 255).collect::<Vec<_>>());
    let (info, index, palette) = read_png("regions.png");
    assert_eq!(info, (2, 2, png::ColorType::Indexed, png::BitDepth::Eight));
    assert_eq!(index, vec![2, 0, 1, 1]);
    assert_eq!(palette.unwrap(), d.regions.legend.iter().flat_map(|e| e.color).collect::<Vec<_>>());
    let legend: crate::RegionsFile = serde_json::from_slice(&std::fs::read(dir.path().join("regions.json")).unwrap()).unwrap();
    assert_eq!(legend.regions, d.regions.legend);
}

/// A named exterior cell.
fn named(grid: (i32, i32), name: &str) -> Cell {
    let mut cell = Cell { name: name.into(), ..Cell::default() };
    cell.data.grid = grid;
    cell
}

fn near(a: [f64; 2], b: [f64; 2]) -> bool {
    (a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9
}

#[test]
fn markers_group_named_cells_and_keep_the_summit() {
    let mut db = db_with(vec![ramp_land((0, 0)), ramp_land((1, 0)), ramp_land((0, 1)), ramp_land((3, 0))]);
    let esp = db.add_source("Names.esp", &[]);
    // Box (0, 0)..(3, 1). "Seyda Neen" is (0, 0) + (1, 1): diagonal neighbours, one group, spelt as the lowest
    // cell. "Ghostgate" is (3, 0) + (3, 1); (1, 0) and (2, *) stay unnamed; (9, 9) is outside the box.
    for cell in [
        named((0, 0), "Seyda Neen"),
        named((1, 1), "seyda neen "),
        named((0, 1), "Pelagiad"),
        named((3, 0), "Ghostgate"),
        named((3, 1), "Ghostgate"),
        named((9, 9), "Outside"),
    ] {
        db.define(esp, keys::cell(&cell), cell).unwrap();
    }
    let d = dump(&db, 100.0).unwrap();
    let m = &d.markers;
    let s = crate::markers::cell_size_m();
    assert_eq!((m.format, m.cell_size_m), (crate::markers::MARKERS_FORMAT, s));
    let rows: Vec<_> = m.markers.iter().map(|k| (k.name.as_str(), k.cells.clone())).collect();
    assert_eq!(rows, vec![
        ("Ghostgate", vec![[3, 0], [3, 1]]),
        ("Seyda Neen", vec![[0, 0], [1, 1]]),
        ("Pelagiad", vec![[0, 1]]),
    ]);
    let gg = &m.markers[0];
    assert!(near(gg.centroid_m, [3.5 * s, 1.0 * s]), "{:?}", gg.centroid_m);
    assert_eq!((gg.min_m, gg.max_m), ([3.0 * s, 0.0], [4.0 * s, 2.0 * s]));
    let sn = &m.markers[1];
    assert!(near(sn.centroid_m, [1.0 * s, 1.0 * s]), "{:?}", sn.centroid_m);
    // The summit: the ramp's top (east edge of (3, 0)), first in row-major order: the north edge of (3, 0).
    let top = m.summit.as_ref().unwrap();
    assert_eq!(top.height_m, ramp(4 * 64));
    assert_eq!(top.cell, [3, 0]);
    assert_eq!((top.x_m, top.y_m), (4.0 * s, 1.0 * s));
}

#[test]
fn markers_file_round_trips() {
    let d = dump(&fixture(), 100.0).unwrap();
    let dir = tempfile::tempdir().unwrap();
    write(&d, dir.path()).unwrap();
    let back = crate::read_markers(&dir.path().join("markers.json")).unwrap();
    assert_eq!(back, d.markers);
    // The fixture names no cell; the summit is the stats' summit.
    assert!(back.markers.is_empty());
    let stats = d.stats.summit.as_ref().unwrap();
    let top = back.summit.unwrap();
    assert_eq!(top.height_m, stats.height_m);
    assert_eq!([top.x_m, top.y_m], stats.mw_units.map(|u| u * mw_world::convert::UNIT_METERS_F64));
}
