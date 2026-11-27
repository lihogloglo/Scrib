//! FS-2c: every startup `--off` part is in the feature-switch list, honestly.

use super::*;
use bevy_ecs::prelude::Mut;
use sf_tunables_bevy::sf_tunables::{Availability, def};

const GROUNDCOVER: Def<bool> = def!(bool "groundcover.enabled" = true);
const FROXELS: Def<bool> = def!(bool "sky.froxels.enabled" = true);
const SUN_VIS: Def<bool> = def!(bool "sky.sun_vis.enabled" = true);
const SHAFTS: Def<bool> = def!(bool "sky.shafts.enabled" = true);
const AERIAL: Def<bool> = def!(bool "sky.aerial.shadow" = true);
const SSAO: Def<&'static str> = def!(choice "render.ssao.quality" = "low").choices(&["off", "low"]);
const CONTACT: Def<bool> = def!(bool "shadow.contact" = true);
const SOFT: Def<bool> = def!(bool "shadow.sun.soft" = true);
const BLOOM: Def<bool> = def!(bool "render.bloom" = true);
const SHELTER: Def<bool> = def!(bool "surface.shelter" = true);
const PUDDLES: Def<bool> = def!(bool "surface.puddles" = true);
const UNDERWATER: Def<bool> = def!(bool "water.underwater" = true);
const GI: Def<bool> = def!(bool "render.gi" = false);

static GROUNDCOVER_SW: FeatureSwitch = FeatureSwitch::new(&GROUNDCOVER);
static STAND_INS: [FeatureSwitch; 11] = [
    FeatureSwitch::new(&FROXELS),
    FeatureSwitch::new(&SUN_VIS),
    FeatureSwitch::new(&SHAFTS),
    FeatureSwitch::new(&AERIAL),
    FeatureSwitch::choice(&SSAO, "off"),
    FeatureSwitch::new(&CONTACT),
    FeatureSwitch::new(&SOFT),
    FeatureSwitch::new(&BLOOM),
    FeatureSwitch::new(&SHELTER),
    FeatureSwitch::new(&PUDDLES),
    FeatureSwitch::new(&UNDERWATER),
];

/// An app with the mapped switches registered as the game's plugins would (stand-in defs under
/// the same names); `groundcover` false leaves its switch out (as `--off groundcover` does).
fn app(groundcover: bool) -> App {
    let mut app = App::new();
    app.register_tunables(&[&FROXELS, &SUN_VIS, &SHAFTS, &AERIAL, &SSAO, &CONTACT, &SOFT, &BLOOM, &SHELTER, &PUDDLES, &UNDERWATER]);
    for switch in &STAND_INS {
        app.add_feature_switch(switch);
    }
    // FS-2d: the live A/B switches and GI, as `live_ab::add` registers them.
    crate::live_switches::register(&mut app);
    app.register_tunables(&[&GI]).add_feature_switch(&crate::console::GI_SWITCH);
    if groundcover {
        app.register_tunables(&[&GROUNDCOVER]).add_feature_switch(&GROUNDCOVER_SW);
    }
    app
}

fn off(parts: &[&str]) -> Off {
    Off::from_flags(&parts.iter().map(|p| (*p).to_owned()).collect::<Vec<_>>()).expect("known parts")
}

fn availability(app: &App, name: &str) -> Option<String> {
    let switches = app.world().resource::<FeatureSwitches>();
    let tunables = app.world().resource::<Tunables>();
    let state = switches.state(tunables, name)?;
    Some(match state.availability {
        Availability::Available => "available".to_owned(),
        Availability::NotBuilt { reason } => format!("not built: {reason}"),
        Availability::Blocked { by } => format!("blocked: {by}"),
    })
}

/// Every part is either mapped to a registered switch or has a `part.*` row: an exact count, so a
/// new `Part` without a row (and a decision) fails here.
#[test]
fn every_part_is_mapped_or_has_a_row() {
    let parts: Vec<Part> = Part::all().collect();
    assert_eq!(parts.len(), 83, "a new Part needs a row in PART_ROWS (and maybe MAPPED)");
    assert_eq!(PART_ROWS.len(), parts.len());
    for (row, part) in PART_ROWS.iter().zip(&parts) {
        assert_eq!(row.part, *part, "PART_ROWS in Part order");
        assert_eq!(row.def.name(), format!("part.{}", part.name()));
        assert_eq!(row.switch.name(), row.def.name());
        assert_eq!(row.switch.startup_hint(), Some(format!("relaunch with --off {}", part.name()).as_str()));
    }
    let mut mapped: Vec<Part> = MAPPED.iter().map(|(p, _, _)| *p).collect();
    mapped.dedup();
    assert_eq!(mapped.len(), MAPPED.len(), "one switch per mapped part");

    let mut app = app(true);
    apply(&mut app, &Off::default());
    let switches = app.world().resource::<FeatureSwitches>();
    let rows = switches.iter().filter(|s| s.name().starts_with("part.")).count();
    assert_eq!(rows, parts.len() - MAPPED.len(), "a row for each unmapped part, none for the mapped ones");
    let unmapped_live = crate::live_switches::LIVE.iter().filter(|l| !MAPPED.iter().any(|(_, s, _)| *s == l.def.name())).count();
    assert_eq!(unmapped_live, 10, "live switches with no part of their own");
    assert_eq!(switches.len(), parts.len() + unmapped_live);
    for part in &parts {
        let name = switch_for(switches, part.name()).unwrap_or_else(|| panic!("{} has no switch", part.name()));
        // Built (`render.gi` is off by default: available, not running).
        assert_eq!(availability(&app, name).as_deref(), Some("available"), "{name}: built, nothing off");
    }
    assert_eq!(switch_for(switches, "godrays"), Some("sky.froxels.enabled"), "aliases too");
    assert_eq!(switch_for(switches, "statics"), Some("part.near"));
    assert_eq!(switch_for(switches, "nope"), None);
}

/// `--off groundcover` leaves the renderer out: its switch reads not built. `--off froxels` only
/// writes the tunable (FS-L2), so its switch stays available (a live `CommandLine` off).
#[test]
fn off_groundcover_marks_its_switch_not_built() {
    let mut app = app(true);
    apply(&mut app, &off(&["groundcover", "froxels", "underwater"]));
    assert_eq!(availability(&app, "groundcover.enabled").as_deref(), Some("not built: --off groundcover"));
    assert_eq!(availability(&app, "water.underwater").as_deref(), Some("not built: --off underwater"));
    assert_eq!(availability(&app, "sky.froxels.enabled").as_deref(), Some("available"));
    assert_eq!(availability(&app, "part.groundcover"), None, "no row beside a registered switch");
}

/// FS-2d: `--off clouds` reaches the live switch (no `part.clouds` row): the cloud plugin stays
/// for the fog, so the tunable is off at the command line and the switch brings the clouds back;
/// with the fog off too none of it is built. Far shadows, sun shadows and GI are left out.
#[test]
fn off_parts_with_a_live_switch_write_it_off_or_mark_it_not_built() {
    let mut app = app(true);
    apply(&mut app, &off(&["clouds", "normalmaps", "farshadows", "shadows", "gi"]));
    assert_eq!(availability(&app, "sky.clouds.enabled").as_deref(), Some("available"));
    assert_eq!(value(&app, "sky.clouds.enabled"), (false, SetBy::CommandLine));
    assert_eq!(availability(&app, "part.clouds"), None, "no row beside the live switch");
    assert_eq!(value(&app, "texture.normal_maps.enabled"), (false, SetBy::CommandLine));
    assert_eq!(value(&app, "sky.fog.enabled"), (true, SetBy::Default));
    for (name, part) in [("shadow.terrain_far.enabled", "farshadows"), ("shadow.sun.enabled", "shadows"), ("render.gi", "gi")] {
        assert_eq!(availability(&app, name), Some(format!("not built: --off {part}")), "{name}");
        assert_eq!(availability(&app, &format!("part.{part}")), None);
    }

    let mut both = self::app(true);
    apply(&mut both, &off(&["clouds", "fog"]));
    assert_eq!(availability(&both, "sky.clouds.enabled").as_deref(), Some("not built: --off clouds"));
    assert_eq!(availability(&both, "sky.fog.enabled").as_deref(), Some("not built: --off fog"));
}

/// A mapped part whose switch the app never registered gets its row instead.
#[test]
fn a_mapped_part_without_its_switch_gets_a_row() {
    let mut app = app(false);
    apply(&mut app, &off(&["groundcover"]));
    assert_eq!(availability(&app, "groundcover.enabled"), None);
    assert_eq!(availability(&app, "part.groundcover").as_deref(), Some("not built: --off groundcover"));
    let switches = app.world().resource::<FeatureSwitches>();
    assert_eq!(switch_for(switches, "groundcover"), Some("part.groundcover"));
}

fn set(app: &mut App, name: &str, on: Option<bool>) -> Result<String, String> {
    app.world_mut().resource_scope(|world, tunables: Mut<Tunables>| {
        sf_console_bevy::features::set_feature(world, &tunables.0, name, on)
    })
    .expect("a registered switch")
}

fn value(app: &App, name: &str) -> (bool, SetBy) {
    let t = app.world().resource::<Tunables>();
    let id = t.id(name).expect("registered");
    (t.flag(id), t.var(id).set_by())
}

/// Toggling a `part.*` row changes nothing: a built part answers the relaunch line, an off one
/// that it was not built.
#[test]
fn toggling_a_part_row_changes_nothing() {
    let mut app = app(true);
    apply(&mut app, &off(&["terrain"]));
    let before = value(&app, "part.near");
    for on in [Some(false), Some(true), None] {
        assert_eq!(set(&mut app, "part.near", on), Err("part.near: startup only: relaunch with --off near; unchanged".to_owned()));
    }
    app.update();
    assert_eq!(value(&app, "part.near"), before);
    assert!(before.0, "built this run");

    assert_eq!(value(&app, "part.terrain"), (false, SetBy::CommandLine));
    let answer = set(&mut app, "part.terrain", Some(true)).expect_err("refused");
    assert!(answer.starts_with("part.terrain: not built this run (--off terrain); unchanged"), "{answer}");
    app.update();
    assert_eq!(value(&app, "part.terrain"), (false, SetBy::CommandLine));
}
