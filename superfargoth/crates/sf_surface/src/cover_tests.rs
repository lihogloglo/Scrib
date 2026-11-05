//! `SurfaceLayer` records (W5a): parsing from a native record file, defaults, mod patches, and
//! the active pick.

#![allow(clippy::float_cmp)]

use sf_records::{RecordDb, RecordFormat, RecordId, apply_record_file};

use crate::cover::{ActiveLayers, CoverChannel, LAYER_KIND, SurfaceLayer, register_kinds};

const BASE: &str = r#"
[
  define(kind: "surface.layer", id: "snow_fresh", value: (
    channel: snow, albedo_height: "textures/snow_ah.png", normal: "textures/snow_n.png",
    arm: "textures/snow_arm.png", tiling_m: 3.0, max_depth_m: 0.2, roughness: 0.6,
    sparkle: 1.0, contrast: 4.0,
  )),
  define(kind: "surface.layer", id: "ash_grey", value: (channel: ash, albedo_height: "textures/ash.dds")),
]
"#;

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let out = apply_record_file(&mut db, base, BASE, RecordFormat::Ron).unwrap();
    assert!(out.errors.is_empty(), "{:?}", out.errors);
    assert_eq!(out.defined, 2);
    db
}

fn get(db: &RecordDb, id: &str) -> SurfaceLayer {
    db.get::<SurfaceLayer>(&RecordId::new(id)).unwrap().clone()
}

#[test]
fn layer_records_parse_with_defaults() {
    let db = db();
    assert_eq!(
        get(&db, "snow_fresh"),
        SurfaceLayer {
            channel: CoverChannel::Snow,
            albedo_height: "textures/snow_ah.png".into(),
            normal: Some("textures/snow_n.png".into()),
            arm: Some("textures/snow_arm.png".into()),
            tiling_m: 3.0,
            max_depth_m: Some(0.2),
            roughness: 0.6,
            sparkle: 1.0,
            contrast: 4.0,
        }
    );
    let ash = get(&db, "ash_grey");
    assert_eq!(ash, SurfaceLayer::new(CoverChannel::Ash, "textures/ash.dds"));
    assert_eq!((ash.tiling_m, ash.roughness, ash.sparkle, ash.contrast, ash.normal.clone(), ash.arm.clone()), (4.0, 0.8, 0.0, 1.0, None, None));
    // Depth: the record's, else the channel's default.
    assert_eq!((get(&db, "snow_fresh").depth_m(), ash.depth_m()), (0.2, 0.03));
    assert_eq!(SurfaceLayer::new(CoverChannel::Snow, "x").depth_m(), 0.15);
    // Round trip through the generic value form (what patches edit).
    let v = db.value_of(LAYER_KIND, &RecordId::new("snow_fresh")).unwrap().unwrap();
    assert_eq!(v.deserialize_into::<SurfaceLayer>().unwrap(), get(&db, "snow_fresh"));
}

#[test]
fn bad_channel_is_rejected_and_mods_patch_fields() {
    let mut db = db();
    let m = db.add_source("mod", &[]);
    let text = r#"[
      define(kind: "surface.layer", id: "mud", value: (channel: mud, albedo_height: "a.dds")),
      patch(kind: "surface.layer", id: "ash_grey", set: { "albedo_height": "textures/better_ash.png", "sparkle": 0.25 }),
    ]"#;
    let out = apply_record_file(&mut db, m, text, RecordFormat::Ron).unwrap();
    assert_eq!((out.defined, out.patched, out.errors.len()), (0, 1, 1));
    let ash = get(&db, "ash_grey");
    assert_eq!((ash.albedo_height.as_str(), ash.sparkle), ("textures/better_ash.png", 0.25));
}

#[test]
fn active_pick_per_worldspace_and_channel() {
    let db = db();
    let (snow, ash) = (RecordId::new("snow_fresh"), RecordId::new("ash_grey"));
    let (main, north) = (RecordId::new("main"), RecordId::new("north"));
    let mut active = ActiveLayers { default: [None, Some(ash.clone())], ..ActiveLayers::default() };
    // North: snow only; a pick of the wrong channel resolves to none.
    active.worldspaces.insert(north.clone(), [Some(snow.clone()), Some(snow.clone())]);
    assert_eq!(active.pick(&main, CoverChannel::Ash), Some(&ash));
    assert_eq!(active.pick(&main, CoverChannel::Snow), None);
    let [s, a] = active.resolve(&db, &main);
    assert_eq!((s, a.map(|l| l.albedo_height.as_str())), (None, Some("textures/ash.dds")));
    let [s, a] = active.resolve(&db, &north);
    assert_eq!((s.map(|l| l.channel), a), (Some(CoverChannel::Snow), None));
    // A pick of a missing record: none.
    active.default[0] = Some(RecordId::new("gone"));
    assert_eq!(active.resolve(&db, &main)[0], None);
}
