use sf_water::{WaterKind, WaterShape};

use crate::*;

#[test]
fn bodies_from_water_levels() {
    let mut ext = Worldspace::new("Vvardenfell");
    ext.has_terrain = true;
    ext.sea_level = Some(0.0);
    let mut room = Worldspace::new("Flooded Cave");
    room.sea_level = Some(-1.5);
    let dry = Worldspace::new("Dry Room");
    let mut out = Vec::new();
    MwWaterSource.bodies(&WorldspaceId::new("ext"), &ext, &mut out);
    MwWaterSource.bodies(&WorldspaceId::new("room"), &room, &mut out);
    MwWaterSource.bodies(&WorldspaceId::new("dry"), &dry, &mut out);
    assert_eq!(out.len(), 2);
    assert_eq!(
        (out[0].kind, &out[0].shape),
        (WaterKind::Ocean, &WaterShape::Infinite)
    );
    assert!(out[0].level.abs() < 1e-9);
    assert_eq!(out[0].optics.as_ref().unwrap().id.as_str(), optics::SEA);
    assert_eq!(
        (out[1].kind, &out[1].shape),
        (WaterKind::Pool, &WaterShape::Bounds)
    );
    assert!((out[1].level + 1.5).abs() < 1e-9);
    assert_eq!(
        out[1].optics.as_ref().unwrap().id.as_str(),
        optics::INTERIOR
    );
    assert_eq!(MwWaterSource.river_policy(), RiverPolicy::FlowOnly);
    assert!(
        (MwWaterSource
            .ocean_spectrum(&WorldspaceId::new("ext"))
            .fetch
            - SEA_FETCH_M.default_value())
            .abs()
            < 1e-3
    );
}

#[test]
fn optics_follow_regions() {
    // A swamp region's open water is coastal; its enclosed water swamp (`optics_for_cell`, WA-10).
    assert_eq!(optics_for_region("Bitter Coast Region"), optics::COASTAL);
    assert_eq!(optics_for_cell("Bitter Coast Region", Some(0.1), 0.5), optics::SWAMP);
    assert_eq!(optics_for_cell("Bitter Coast Region", Some(0.9), 0.5), optics::COASTAL);
    assert_eq!(optics_for_cell("Bitter Coast Region", None, 0.5), optics::COASTAL);
    assert_eq!(optics_for_cell("Azura's Coast Region", Some(0.1), 0.5), optics::SEA);
    assert_eq!(optics_for_region("Ascadian Isles Region"), optics::COASTAL);
    assert_eq!(optics_for_region("Azura's Coast Region"), optics::SEA);
    assert_eq!(optics_for_region("Sheogorad"), optics::SEA);
    assert_eq!(optics_for_region("Ashlands Region"), optics::ASH);
    assert_eq!(optics_for_region("Felsaad Coast Region"), optics::NORTH);
    assert_eq!(optics_for_region(""), optics::SEA);
    let source = MwWaterOptics {
        region_at: |_: WorldPos| Some("Bitter Coast Region".to_owned()),
    };
    let r = source
        .optics_at(&WorldspaceId::new("ext"), WorldPos::ORIGIN)
        .unwrap();
    assert_eq!(r.id.as_str(), optics::COASTAL);
    // Visibility: swamp murkiest, the open sea clearest, each at its target.
    let recs = optics_records(&WaterIni::default());
    let rec = |id: &str| &recs.iter().find(|(i, _)| *i == id).unwrap().1;
    let vis = |id: &str| rec(id).visibility();
    assert!((vis(optics::SWAMP) - 3.0).abs() < 0.01 && (vis(optics::SEA) - 25.0).abs() < 0.01);
    assert!(vis(optics::SWAMP) < vis(optics::COASTAL) && vis(optics::COASTAL) < vis(optics::SEA));
    assert!(
        recs.iter()
            .all(|(_, o)| o.absorption.iter().chain(&o.scattering).all(|v| *v > 0.0))
    );
    assert!(rec(optics::SWAMP).foam.whitecaps.abs() < f32::EPSILON);
}

#[test]
fn ini_water_section_with_fallbacks() {
    let text = b"[General]\nUnderwaterDayFog=9\n[Water]\nUnderwaterColor=010,020,030\nUnderwaterDayFog=2.0\nMaxNumberRipples=x\nNearWaterOutdoorID=Sea Waves\n[Weather]\nUnderwaterIndoorFog=7\n";
    let ini = WaterIni::parse(text);
    assert_eq!(ini.underwater_color, [10, 20, 30]);
    assert!((ini.day_fog - 2.0).abs() < 1e-6, "only [Water] keys count");
    assert_eq!(ini.max_ripples, 75, "unreadable values fall back");
    assert!((ini.indoor_fog - 3.0).abs() < 1e-6);
    assert_eq!(ini.near_water_outdoor_id, "Sea Waves");
    assert_eq!(WaterIni::parse(b""), WaterIni::default());
}

#[test]
fn wet_mask_from_land_heights() {
    // One cell (0, 0): land except its south-west corner vertex.
    let mut heights = vec![100.0f32; 65 * 65];
    heights[0] = -10.0;
    let bounds = cell_bounds([(0, 0)]).unwrap();
    assert_eq!(bounds, (-1, -1, 1, 1));
    let m = wet_mask(bounds, [((0, 0), heights.as_slice())]);
    assert_eq!((m.width, m.height), (97, 97));
    assert!((m.texel - 3.6576).abs() < 1e-9);
    // Cell (0, 0)'s south-west vertex is texel (32, 64) (rows run south from the north edge).
    assert!(m.wet[64 * 97 + 32]);
    assert!(!m.wet[64 * 97 + 34] && !m.wet[32 * 97 + 48]);
    // Margin cells are water.
    assert!(m.wet[0] && m.wet[96 * 97 + 96]);
    // Texel (32, 64) sits at world x = 0, z = 0 (engine z = −north).
    let x = m.origin.x + 32.0 * m.texel;
    let z = m.origin.y + 64.0 * m.texel;
    assert!(x.abs() < 1e-6 && z.abs() < 1e-6, "{x} {z}");
    // W-GERSTNER-DEPTH: the bed depth below sea level, m (10 units under, 100 units above), unknown
    // (NaN) in the margin cells without `LAND`.
    let depth = m.depth.as_ref().expect("bed depths");
    assert!((depth[64 * 97 + 32] - 10.0 * 0.9144 / 64.0).abs() < 1e-6, "{}", depth[64 * 97 + 32]);
    assert!((depth[64 * 97 + 34] + 100.0 * 0.9144 / 64.0).abs() < 1e-6);
    assert!(depth[0].is_nan() && depth[96 * 97 + 96].is_nan());
}

#[test]
fn optics_map_by_region() {
    let bounds = (-1, -1, 1, 1);
    let map = optics_map(
        bounds,
        |x, y| (x == 0 && y == 1).then(|| "Bitter Coast Region".to_owned()),
        |_, _| Some(0.1),
        SWAMP_ENCLOSED_BELOW.default_value(),
    );
    assert_eq!((map.width, map.height), (3, 3));
    // Row 0 is the northernmost (y = 1).
    assert_eq!(map.index[1], optics_index(optics::SWAMP));
    assert_eq!(map.index[4], optics_index(optics::SEA));
    assert_eq!(optics_index(optics::INTERIOR), 5);
}

/// WA-10 item 4: in the Bitter Coast the open sea is coastal and only enclosed water swamp, from the
/// baked shore openness; other regions keep their record. Same lookup as the underwater post's
/// (`sf_water::optics_index_at`).
#[test]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
fn swamp_optics_only_on_enclosed_water() {
    let cell = CELL_UNITS * UNIT_M;
    let texel = 4.0;
    // Cells -4..=4 both ways; rows run +Z = south, so z spans -(4 + 1) cells .. 4 cells.
    let origin = glam::DVec2::new(-4.0 * cell, -5.0 * cell);
    let n = (9.0 * cell / texel) as usize + 1;
    let mut mask = WetMask::new(n, n, texel, origin);
    // A land block over cells x -1..=1, y -1..=1 holding a 30 m pool at cell (0, 0)'s centre.
    let pool = glam::DVec2::new(0.5 * cell, -0.5 * cell);
    for row in 0..n {
        for col in 0..n {
            let p = origin + glam::DVec2::new(col as f64, row as f64) * texel;
            let land = (-cell..2.0 * cell).contains(&p.x) && (-2.0 * cell..cell).contains(&p.y);
            mask.wet[row * n + col] = !land || p.distance(pool) < 30.0;
        }
    }
    let field = ShoreField::bake(&mask, ShoreSettings::default());
    let open = cell_openness(&field);
    let regions: HashMap<(i32, i32), &str> = [
        ((0, 0), "Bitter Coast Region"),   // the enclosed pool
        ((3, 0), "Bitter Coast Region"),   // open sea
        ((1, 1), "Bitter Coast Region"),   // dry land
        ((-3, 0), "Azura's Coast Region"), // open sea, another region
        ((0, 3), "Ascadian Isles Region"),
    ]
    .into_iter()
    .collect();
    assert!(open[&(0, 0)] < 0.1 && open[&(3, 0)] > 0.8 && !open.contains_key(&(1, 1)), "openness {:?}", (open.get(&(0, 0)), open.get(&(3, 0))));
    let map = optics_map(
        (-4, -4, 4, 4),
        |x, y| regions.get(&(x, y)).map(|r| (*r).to_owned()),
        |x, y| open.get(&(x, y)).copied(),
        SWAMP_ENCLOSED_BELOW.default_value(),
    );
    let at = |cx: i32, cy: i32| sf_water::optics_index_at(&map, glam::DVec2::new((f64::from(cx) + 0.5) * cell, -(f64::from(cy) + 0.5) * cell));
    assert_eq!(at(0, 0), optics_index(optics::SWAMP));
    assert_eq!(at(3, 0), optics_index(optics::COASTAL));
    assert_eq!(at(1, 1), optics_index(optics::COASTAL));
    assert_eq!(at(-3, 0), optics_index(optics::SEA));
    assert_eq!(at(0, 3), optics_index(optics::COASTAL));
    assert_eq!(at(2, 2), optics_index(optics::SEA)); // no region: the open sea
}
