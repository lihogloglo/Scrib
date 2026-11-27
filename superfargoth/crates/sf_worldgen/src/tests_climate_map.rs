#![allow(clippy::float_cmp)] // exact comparisons are the point of these tests
//! Tests of `climate_map`: a 16 x 12 guide at 100 m, sea level 10 m, heights `10 + (col - 3) * 40 + row * 5` m
//! (columns 0-1 sea, column 2 sea to row 8, column 3 row 0 exactly at sea level), a 4 x 3 region grid of 400 m
//! cells: guide columns 0-7 are "Ash Plain" (legend 1), 8-15 "North Isles" (legend 2).

use super::*;
use crate::climate::NamedOverride;
use crate::guide::RegionMap;

const W: usize = 16;
const H: usize = 12;

fn guide() -> Guide {
    let heights = (0..W * H)
        .map(|i| {
            let (col, row) = ((i % W) as f32, (i / W) as f32);
            10.0 + (col - 3.0) * 40.0 + row * 5.0
        })
        .collect();
    let mut names = vec![String::new(); 256];
    names[1] = "Ash Plain".into();
    names[2] = "North Isles".into();
    let index = (0..12).map(|i| if i % 4 < 2 { 1 } else { 2 }).collect();
    Guide {
        w: W,
        h: H,
        spacing_m: 100.0,
        sea_level_m: 10.0,
        fill_m: None,
        origin_m: [500.0, -300.0],
        heights,
        regions: Some(RegionMap { w: 4, h: 3, cell_m: 400.0, index, names }),
    }
}

/// The guide's heights above sea level.
fn above_sea(g: &Guide) -> Grid {
    Grid {
        w: g.w,
        h: g.h,
        spacing: g.spacing_m,
        data: g.heights.iter().map(|&x| f64::from(x) - 10.0).collect(),
    }
}

/// Defaults with blend 0 and the given overrides.
fn file_with(overrides: Vec<NamedOverride>) -> ClimateFile {
    let mut file = ClimateFile::defaults();
    file.climate.override_blend_m = 0.0;
    file.overrides = overrides;
    file
}

fn pin_ash_plain() -> NamedOverride {
    NamedOverride {
        region: "Ash Plain".into(),
        temperature: Adjust::default(),
        rain: Adjust { pin: Some(900.0), ..Adjust::default() },
    }
}

#[test]
fn builds_the_chain_above_sea_level() {
    let gd = guide();
    let file = ClimateFile::defaults();
    let c = &file.climate;
    let table = ClassTable::whittaker();
    let map = build(&gd, &file, &table).unwrap();
    let g = above_sea(&gd);
    let t = temperature(&g, c);
    let rain = precipitation(&g, &t, c).unwrap();
    assert_eq!(map.temperature.len(), W * H);
    assert_eq!(map.precipitation.len(), W * H);
    assert_eq!(map.moisture.len(), W * H);
    assert_eq!(map.classes.len(), W * H);
    for i in 0..W * H {
        assert_eq!(map.temperature[i], t.data[i] as f32, "temperature at {i}");
        assert_eq!(map.precipitation[i], rain.p.data[i] as f32, "precipitation at {i}");
        assert!(map.moisture[i].is_finite() && (0.0..=1.0).contains(&map.moisture[i]), "moisture at {i}");
    }
    assert_eq!(map.budget, rain.budget);
    assert_eq!(map.classes, classes(&t, &rain.p, &table).unwrap());
    assert_eq!(map.class_names.len(), 12);
    assert_eq!(map.mfd_stuck, 0);
    assert!(map.overrides.is_empty());
}

#[test]
fn overrides_resolve_by_legend_name() {
    let gd = guide();
    let table = ClassTable::whittaker();
    let plain = build(&gd, &file_with(Vec::new()), &table).unwrap();
    let file = file_with(vec![pin_ash_plain()]);
    let map = build(&gd, &file, &table).unwrap();
    let t = temperature(&above_sea(&gd), &file.climate);
    for i in 0..W * H {
        if i % W < 8 {
            assert_eq!(map.precipitation[i], 900.0, "pinned rain at {i}");
            assert_eq!(map.classes[i], table.classify(t.data[i], 900.0), "class at {i}");
        } else {
            assert_eq!(map.precipitation[i].to_bits(), plain.precipitation[i].to_bits(), "rain at {i}");
            assert_eq!(map.temperature[i].to_bits(), plain.temperature[i].to_bits(), "temperature at {i}");
        }
    }
    assert_eq!(
        map.overrides,
        vec![AppliedOverride {
            name: "Ash Plain".into(),
            index: 1,
            temperature: Adjust::default(),
            rain: Adjust { pin: Some(900.0), ..Adjust::default() },
        }]
    );

    // A large negative add leaves no negative rain.
    let dry = file_with(vec![NamedOverride {
        region: "North Isles".into(),
        temperature: Adjust::default(),
        rain: Adjust { add: -1e6, ..Adjust::default() },
    }]);
    let map = build(&gd, &dry, &table).unwrap();
    for i in (0..W * H).filter(|i| i % W >= 8) {
        assert_eq!(map.precipitation[i], 0.0, "dried rain at {i}");
    }
}

#[test]
fn refuses_unresolved_overrides() {
    let table = ClassTable::whittaker();
    let file = file_with(vec![NamedOverride {
        region: "Nowhere".into(),
        temperature: Adjust::default(),
        rain: Adjust::default(),
    }]);
    let err = build(&guide(), &file, &table).unwrap_err().to_string();
    assert!(err.contains("Nowhere"), "{err}");
    let mut bare = guide();
    bare.regions = None;
    assert!(build(&bare, &file, &table).is_err());
}

#[test]
fn writes_and_reads_back() {
    let gd = guide();
    let table = ClassTable::whittaker();
    let file = file_with(vec![pin_ash_plain()]);
    let map = build(&gd, &file, &table).unwrap();
    let dir = std::env::temp_dir().join(format!("sf_worldgen_climate_map_{}", std::process::id()));
    let hash = map.write(&dir, &file.declared).unwrap();
    assert_eq!(hash, map.hash());
    let out = dir.join("climate");

    let read_json =
        |name: &str| serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(out.join(name)).unwrap()).unwrap();
    let meta = read_json("meta.json");
    assert_eq!(meta["blake3"], serde_json::json!(hash));
    assert_eq!(meta["format"], serde_json::json!(1));
    assert_eq!(meta["dims"], serde_json::json!([16, 12]));
    assert_eq!(meta["sample_spacing_m"], serde_json::json!(100.0));
    assert_eq!(meta["origin_m"], serde_json::json!([500.0, -300.0]));
    let files = meta["files"].as_object().unwrap();
    let names = ["temperature.f32", "precipitation.f32", "moisture.f32", "classes.u8"];
    assert_eq!(files.len(), names.len());
    for (name, size) in names.iter().zip([768u64, 768, 768, 192]) {
        let bytes = std::fs::read(out.join(name)).unwrap();
        assert_eq!(bytes.len() as u64, size, "{name}");
        assert_eq!(files[*name], serde_json::json!(blake3::hash(&bytes).to_hex().to_string()), "{name}");
    }
    let raw = std::fs::read(out.join("temperature.f32")).unwrap();
    let back: Vec<f32> = raw.as_chunks::<4>().0.iter().map(|b| f32::from_le_bytes(*b)).collect();
    assert_eq!(back, map.temperature);

    let climate = read_json("climate.json");
    assert_eq!(climate["classes"].as_array().unwrap().len(), 12);
    assert_eq!(climate["values"].as_object().unwrap().len(), 18);
    assert_eq!(climate["values"]["drain_flow_exponent"]["v"], serde_json::json!(1.1));
    assert_eq!(climate["overrides"][0]["region"], serde_json::json!("Ash Plain"));
    assert_eq!(climate["overrides"][0]["rain"]["pin"], serde_json::json!(900.0));
    assert!(climate["overrides"][0]["temperature"]["pin"].is_null());

    assert_eq!(build(&gd, &file, &table).unwrap().hash(), hash);
    std::fs::remove_dir_all(&dir).unwrap();
}

/// Writes the fixture guide (no region grid) as a v1 guide folder.
fn write_guide_folder(dir: &Path) -> Guide {
    let mut gd = guide();
    gd.regions = None;
    std::fs::create_dir_all(dir).unwrap();
    let meta = serde_json::json!({
        "format": 1,
        "dims": [W, H],
        "sample_spacing_m": 100.0,
        "sea_level_m": 10.0,
        "origin_m": [500.0, -300.0],
    });
    std::fs::write(dir.join("meta.json"), meta.to_string()).unwrap();
    let bytes: Vec<u8> = gd.heights.iter().flat_map(|x| x.to_le_bytes()).collect();
    std::fs::write(dir.join("heights.f32"), bytes).unwrap();
    gd
}

#[test]
fn runs_from_a_guide_folder_with_set_values() {
    let dir = std::env::temp_dir().join(format!("sf_worldgen_climate_run_{}", std::process::id()));
    let gd = write_guide_folder(&dir.join("guide"));
    let request = |sets: &[(&str, f64)]| ClimateRequest {
        base: dir.join("guide"),
        params: None,
        classes: None,
        sets: sets.iter().map(|(k, v)| ((*k).to_string(), *v)).collect(),
        out: dir.join("out"),
    };
    let table = ClassTable::whittaker();

    // No --set: the declared defaults.
    let plain = run_climate(&request(&[])).unwrap();
    assert_eq!(plain.hash(), build(&gd, &ClimateFile::defaults(), &table).unwrap().hash());

    // --set capacity: the value reaches the build and the written declaration.
    let map = run_climate(&request(&[("capacity", 100_000.0)])).unwrap();
    let mut file = ClimateFile::defaults();
    file.declared.iter_mut().find(|d| d.name == "capacity").unwrap().v = 100_000.0;
    file.climate = Climate::from_declared(&file.declared).unwrap();
    assert_eq!(file.climate.capacity, 100_000.0);
    assert_eq!(map.hash(), build(&gd, &file, &table).unwrap().hash());
    assert_ne!(map.precipitation, plain.precipitation);
    let out = dir.join("out").join("climate");
    let read_json =
        |name: &str| serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(out.join(name)).unwrap()).unwrap();
    assert_eq!(read_json("meta.json")["blake3"], serde_json::json!(map.hash()));
    let values = &read_json("climate.json")["values"];
    assert_eq!(values["capacity"]["v"], serde_json::json!(100_000.0));
    assert_eq!(values["capacity"]["source"], serde_json::json!("Input(--set)"));
    assert_eq!(values["fallout_km"]["source"], serde_json::json!("Industry(Smith-Barstad fallout time 1000 s at 6-10 m/s)"));

    // An unknown name and a value outside its declared range (capacity 5e4..5e5) are refused.
    let err = run_climate(&request(&[("nope", 1.0)])).unwrap_err().to_string();
    assert!(err.contains("unknown value nope"), "{err}");
    assert!(run_climate(&request(&[("capacity", 1.0)])).is_err());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn stage_times_cover_the_build() {
    let (map, times) = build_timed(&guide(), &ClimateFile::defaults(), &ClassTable::whittaker()).unwrap();
    let names: Vec<&str> = times.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        ["heights", "temperature", "rain", "overrides", "flood", "mfd", "moisture", "classes", "to f32"]
    );
    assert!(times.iter().all(|(_, s)| s.is_finite() && *s >= 0.0));
    assert_eq!(map.hash(), build(&guide(), &ClimateFile::defaults(), &ClassTable::whittaker()).unwrap().hash());
}

#[test]
#[ignore = "measurement: land drainage-area percentiles of the guide folder in SF_WORLDGEN_GUIDE (tuning of drain_area_min_m2 / drain_area_max_m2)"]
fn measure_drainage_area_on_a_guide() {
    let dir = std::env::var("SF_WORLDGEN_GUIDE").expect("set SF_WORLDGEN_GUIDE to a guide folder");
    let gd = Guide::read(Path::new(&dir)).unwrap();
    let heights = Grid {
        w: gd.w,
        h: gd.h,
        spacing: gd.spacing_m,
        data: gd.heights.iter().map(|&x| f64::from(x) - gd.sea_level_m).collect(),
    };
    let (wet, z) = routing_surface(&heights);
    let exponent = ClimateFile::defaults().climate.drain_flow_exponent;
    let (area, stuck) = mfd_area(gd.w, gd.h, gd.spacing_m, &z, &wet, exponent);
    let mut land: Vec<f64> = area.iter().zip(&wet).filter(|(_, w)| !**w).map(|(a, _)| *a).collect();
    land.sort_unstable_by(f64::total_cmp);
    let q = |f: f64| land[((land.len() - 1) as f64 * f).round() as usize];
    println!(
        "drainage area, {} land samples of {} (cell {:.1} m2, stuck {stuck}), m2:",
        land.len(),
        area.len(),
        gd.spacing_m * gd.spacing_m
    );
    for f in [0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 0.999, 0.9999, 1.0] {
        println!("  p{:<6} {:.3e}", 100.0 * f, q(f));
    }
    for a in [1e3, 3e3, 1e4, 3e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
        let above = land.len() - land.partition_point(|x| *x < a);
        println!("  >= {a:.0e} m2: {:.3} % of land", 100.0 * above as f64 / land.len() as f64);
    }
}

#[test]
fn reads_the_product_back_exactly_and_refuses_a_tampered_file() {
    let gd = guide();
    let table = ClassTable::whittaker();
    let file = file_with(vec![pin_ash_plain()]);
    let map = build(&gd, &file, &table).unwrap();
    let dir = std::env::temp_dir().join(format!("sf_worldgen_climate_read_{}", std::process::id()));
    map.write(&dir, &file.declared).unwrap();

    let back = ClimateMap::read(&dir).unwrap();
    assert_eq!((back.w, back.h, back.spacing_m, back.origin_m), (map.w, map.h, map.spacing_m, map.origin_m));
    assert_eq!(back.temperature, map.temperature);
    assert_eq!(back.precipitation, map.precipitation);
    assert_eq!(back.moisture, map.moisture);
    assert_eq!(back.classes, map.classes);
    assert_eq!(back.class_names, map.class_names);
    assert_eq!(back.overrides, map.overrides);
    assert_eq!(back.mfd_stuck, map.mfd_stuck);
    // JSON floats: serde_json without `float_roundtrip` may land one ulp off.
    for (a, b) in [
        (back.budget.inflow, map.budget.inflow),
        (back.budget.source, map.budget.source),
        (back.budget.rain, map.budget.rain),
        (back.budget.outflow, map.budget.outflow),
    ] {
        assert!((a - b).abs() <= 1e-12 * b.abs().max(1.0), "budget {a} vs {b}");
    }
    assert_eq!(back.hash(), map.hash());

    // One flipped byte in a raster: refused, naming the file.
    let path = dir.join("climate").join("moisture.f32");
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[5] ^= 0x01;
    std::fs::write(&path, &bytes).unwrap();
    let err = ClimateMap::read(&dir).unwrap_err().to_string();
    assert!(err.contains("moisture.f32 blake3"), "{err}");
    // A truncated file: refused on its size.
    bytes.truncate(bytes.len() - 4);
    std::fs::write(&path, &bytes).unwrap();
    let err = ClimateMap::read(&dir).unwrap_err().to_string();
    assert!(err.contains("moisture.f32 is 764 bytes"), "{err}");
    std::fs::remove_dir_all(&dir).unwrap();
}
