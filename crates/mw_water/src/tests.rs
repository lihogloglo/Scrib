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
            - SEA_FETCH_M)
            .abs()
            < 1e-3
    );
}

#[test]
fn optics_follow_regions() {
    assert_eq!(optics_for_region("Bitter Coast Region"), optics::SWAMP);
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
    assert_eq!(r.id.as_str(), optics::SWAMP);
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
