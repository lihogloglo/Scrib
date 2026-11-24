//! Exact tests of the trail ring -> blade trample source glue (slice GC-3d2, `blade_press`).

// Exact by design: the values compared are copied, not computed.
#![allow(clippy::float_cmp)]

use std::path::PathBuf;

use bevy_ecs::prelude::*;
use bevy_grass::{GrassGenerateSystems, GrassTramplePlacement};
use bevy_math::{IVec4, UVec2, Vec2, Vec4};
use sf_surface_bevy::{TrailPassSystems, TrailPlacement};

use crate::blade_press::{order_generate_after_trail, press_source, ring_lookup, ring_placement};

/// A ring as `sf_surface_bevy::trail::update_trails` places it: 1/16 m texels, 1024 per side.
const PLACE: Vec4 = Vec4::new(-31.75, 12.5, 16.0, 0.0);
const CELLS: IVec4 = IVec4::new(1000, 3, 1024, 0);

fn ring(place: Vec4, cells: IVec4) -> TrailPlacement {
    TrailPlacement { place, cells }
}

#[test]
fn ring_placement_maps_the_uniform_layout() {
    let p = ring_placement(PLACE, CELLS);
    assert_eq!(
        p,
        GrassTramplePlacement {
            corner: Vec2::new(-31.75, 12.5),
            texels_per_metre: 16.0,
            size: 1024,
            first_texel: UVec2::new(1000, 3),
        }
    );
    assert!(p.is_placed());
    // The print depth (w) plays no part: a press-only ring (depth 0) and a snow ring place alike.
    assert_eq!(ring_placement(PLACE.with_w(0.3), CELLS), p);
}

#[test]
fn an_unplaced_ring_maps_to_none() {
    for (place, cells) in [
        (Vec4::ZERO, IVec4::ZERO),     // trails off (`update_trails` zeroes `trail`)
        (PLACE.with_z(0.0), CELLS),    // no texels per metre
        (PLACE.with_z(-16.0), CELLS),  // negative scale
        (PLACE.with_z(f32::NAN), CELLS),
        (PLACE, CELLS.with_z(0)),      // no size
        (PLACE, CELLS.with_x(-1)),     // a first texel off the texture
    ] {
        assert_eq!(ring_placement(place, cells), GrassTramplePlacement::NONE, "{place} {cells}");
    }
}

#[test]
fn the_lookup_reads_the_press_as_the_terrain_and_mesh_plants() {
    let l = ring_lookup(1.0);
    assert_eq!(l.channel, 3);
    assert_eq!(l.strength, 1.0);
    assert_eq!(l.edge_margin, 0.5);
    assert_eq!(l.edge_fade, 2.0);
    assert_eq!(l.direction_floor, 1.0 / 256.0);
    assert_eq!(ring_lookup(-1.0).strength, 0.0);
}

#[test]
fn source_follows_the_ring_and_the_trample_amount() {
    let on = ring(PLACE, CELLS);
    let moved = ring(PLACE + Vec4::new(0.0625, 0.0, 0.0, 0.0), CELLS.with_x(1001));
    // No ring views (no trail plugin render part): no source.
    assert_eq!(press_source(None, 1.0), None);
    // Ring on: this frame's and last frame's placements, strength = amount.
    let (now, then, lookup) = press_source(Some((moved, on)), 0.5).expect("source");
    assert_eq!(now, ring_placement(moved.place, moved.cells));
    assert_eq!(then, ring_placement(on.place, on.cells));
    assert!(now.is_placed() && then.is_placed());
    assert_eq!(lookup.strength, 0.5);
    // Ring off (no placement): a source that presses nothing, so the crate's own passes stay skipped.
    let off = ring(Vec4::ZERO, IVec4::ZERO);
    let (now, then, _) = press_source(Some((off, off)), 1.0).expect("source while off");
    assert_eq!((now, then), (GrassTramplePlacement::NONE, GrassTramplePlacement::NONE));
    // Just turned on: last frame had no ring.
    let (now, then, _) = press_source(Some((on, off)), 1.0).expect("source");
    assert!(now.is_placed());
    assert_eq!(then, GrassTramplePlacement::NONE);
    // `trample_amount` 0 is the live off switch: nothing placed, strength 0.
    let (now, then, lookup) = press_source(Some((on, on)), 0.0).expect("source at amount 0");
    assert_eq!((now, then), (GrassTramplePlacement::NONE, GrassTramplePlacement::NONE));
    assert_eq!(lookup.strength, 0.0);
}

#[derive(Resource, Default)]
struct Order(Vec<&'static str>);

/// The blade generate runs after the ring pass: stand-ins in both sets, the generate's added first.
#[test]
fn generate_runs_after_the_ring_pass() {
    let mut world = World::new();
    world.init_resource::<Order>();
    let mut schedule = Schedule::default();
    schedule.add_systems((|mut o: ResMut<Order>| o.0.push("generate")).in_set(GrassGenerateSystems));
    schedule.add_systems((|mut o: ResMut<Order>| o.0.push("trail")).in_set(TrailPassSystems));
    order_generate_after_trail(&mut schedule);
    schedule.run(&mut world);
    assert_eq!(world.resource::<Order>().0, ["trail", "generate"]);
}

/// The vendored generate is in [`GrassGenerateSystems`] (source check: cargo cannot test the vendored crate,
/// and the render graph does not run without a GPU).
#[test]
fn vendored_generate_is_in_its_set() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vendor/bevy_grass/src/render/mod.rs");
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let squash: String = src.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<String>().split_whitespace().collect();
    assert!(squash.contains("RenderGraph,compute::run_generate.in_set(crate::GrassGenerateSystems).before(camera_driver)"));
}
