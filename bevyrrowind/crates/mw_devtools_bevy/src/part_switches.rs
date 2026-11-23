//! Startup `--off` parts in the feature-switch list (slice FS-2c; spec
//! `superfargoth/docs/specs/feature-switches.md`).
//!
//! `--off <part>` stays a true startup removal (bench ceilings and ledger lines stay
//! reproducible). Every part shows up in `features` and the Features panel, honestly:
//!
//! - A part with a live feature switch ([`MAPPED`]): [`Kind::CommandLine`] parts write the switch's
//!   tunable off at the `CommandLine` layer where the part is applied (the feature is still built,
//!   so the switch brings it back live, e.g. `--off froxels`, FS-L2); [`Kind::Removed`] parts
//!   leave the feature out of the run, so the switch is marked not built (`--off <part>`);
//!   [`Kind::Live`] parts (the live A/B switches, FS-2d) get their `CommandLine` off written here.
//! - Any other part (or a mapped one whose switch this app does not register): a Dev,
//!   `Lifetime::Restart` row `part.<name>` ([`PART_ROWS`]), its value "built this run" (off at
//!   `CommandLine` under `--off`). The row is `startup_only`: a flip is refused with "startup only:
//!   relaunch with --off <name>" and changes nothing; an off part is also marked not built.
//!
//! Applied in [`PartSwitchesPlugin`]'s `finish`, after every plugin registered its switches and
//! before `PreStartup` seeds the running states.

use bevy_app::prelude::*;

use mw_game_core::perf::{Off, Part};
use sf_tunables_bevy::sf_tunables::{Category, Def, Effect, FeatureSwitch, Lifetime, OffValue, Scope, Source};
use sf_tunables_bevy::{FeatureSwitches, SetBy, Tunables, TunablesAppExt};

/// How a mapped part relates to its switch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `--off` writes the switch's tunable off at the `CommandLine` layer; the feature is built and
    /// the switch brings it back live (nothing to mark).
    CommandLine,
    /// `--off` leaves the feature out of the run: the switch is marked not built.
    Removed,
    /// A live A/B switch (FS-2d, [`crate::live_switches`]): the feature is still built, so `--off`
    /// writes the tunable off at the `CommandLine` layer here and the switch brings it back live;
    /// unless one of these parts is off too, which leaves it out of the run (marked not built).
    Live(&'static [Part]),
}

/// The parts with a live feature switch: `(part, switch, kind)`.
pub const MAPPED: [(Part, &str, Kind); 24] = [
    // GC-2b3: `--off groundcover` adds none of the renderer (mw_exterior_bevy groundcover.rs).
    (Part::Groundcover, "groundcover.enabled", Kind::Removed),
    // FS-L2: the froxel resources are still built (exterior.rs writes the tunable; live off skips
    // inject + integrate), so a CommandLine off, live.
    (Part::Froxels, "sky.froxels.enabled", Kind::CommandLine),
    (Part::SunVis, "sky.sun_vis.enabled", Kind::CommandLine),
    (Part::Shafts, "sky.shafts.enabled", Kind::CommandLine),
    (Part::AerialShadow, "sky.aerial.shadow", Kind::CommandLine),
    // mw_exterior_bevy quality_presets.rs `off_values`.
    (Part::Ssao, "render.ssao.quality", Kind::CommandLine),
    (Part::ContactShadows, "shadow.contact", Kind::CommandLine),
    (Part::SoftShadows, "shadow.sun.soft", Kind::CommandLine),
    (Part::Bloom, "render.bloom", Kind::CommandLine),
    (Part::Shelter, "surface.shelter", Kind::CommandLine),
    (Part::Puddles, "surface.puddles", Kind::CommandLine),
    // U3: `--off underwater` keeps the post off from the `Off` resource whatever `water.underwater`
    // says (mw_environment_bevy water.rs `underwater_switch`): the switch can't bring it back.
    (Part::Underwater, "water.underwater", Kind::Removed),
    // FS-2d: the live A/B switches (`crate::live_switches`). `--off clouds` keeps the cloud plugin
    // for the fog (`Clouds::enabled` false, exterior.rs), so the switch brings them back; both off
    // (or `--off sky`) adds none of it.
    (Part::Clouds, "sky.clouds.enabled", Kind::Live(&[Part::Fog, Part::Sky])),
    (Part::Fog, "sky.fog.enabled", Kind::Live(&[Part::Clouds, Part::Sky])),
    // exterior.rs: the start values of `SurfaceSettings`, live after.
    (Part::Surface, "surface.enabled", Kind::Live(&[])),
    (Part::Cover, "surface.cover.enabled", Kind::Live(&[])),
    (Part::Trails, "surface.trails.enabled", Kind::Live(&[])),
    (Part::Raise, "surface.raise.enabled", Kind::Live(&[])),
    // `mw_assets::MaterialMaps` start values; images stay loaded (map_ab.rs).
    (Part::NormalMaps, "texture.normal_maps.enabled", Kind::Live(&[])),
    (Part::Orm, "texture.orm.enabled", Kind::Live(&[])),
    (Part::Parallax, "texture.parallax.enabled", Kind::Live(&[])),
    // `--off farshadows` adds no far-map systems (exterior.rs): the live switch (which skips the bake,
    // FS-L3) has no map to bring back this run.
    (Part::FarShadows, "shadow.terrain_far.enabled", Kind::Removed),
    // `--off shadows` turns the sun's shadow maps off every frame (perf.rs `no_shadows`).
    (Part::Shadows, "shadow.sun.enabled", Kind::Removed),
    // `--off gi` adds no GI plugin: `render.gi` can't bring it back (console.rs).
    (Part::Gi, "render.gi", Kind::Removed),
];

/// One part's `part.<name>` row: its tunable and its startup-only switch.
pub struct PartRow {
    pub part: Part,
    pub def: Def<bool>,
    pub switch: FeatureSwitch,
}

macro_rules! rows {
    ($($part:ident $name:literal $cat:ident $help:literal;)*) => {
        /// Every part's `part.<name>` row, in `Part` order (registered for the parts with no
        /// switch in this app).
        pub static PART_ROWS: &[PartRow] = &[$(
            PartRow {
                part: Part::$part,
                def: Def::<bool>::new(concat!("part.", $name), true)
                    .help(concat!($help, " (startup part: true = built this run; relaunch with or without --off ", $name, ")"))
                    .source(Source::Owner("2026-11-23"))
                    .scope(Scope::Dev)
                    .lifetime(Lifetime::Restart)
                    .at(file!(), line!()),
                switch: FeatureSwitch::named(concat!("part.", $name), OffValue::Bool(false))
                    .label($help)
                    .category(Category::$cat)
                    .effect(Effect::Skips)
                    .startup_only(concat!("relaunch with --off ", $name))
                    .at(file!(), line!()),
            },
        )*];
    };
}

rows! {
    Terrain "terrain" World "Terrain";
    Near "near" World "Near cells' statics";
    Far "far" World "Far cells' statics";
    Sky "sky" Sky "Sky and atmosphere";
    Envmap "envmap" Lighting "Sky environment map";
    Shadows "shadows" Shadows "Sun shadows";
    Lights "lights" Lighting "Lamps (point and spot lights)";
    Particles "particles" Effects "NIF particle emitters";
    Nif "nif" World "NIF models on placements";
    FarField "farfield" ViewDistance "Far field (distant LOD)";
    Denoiser "denoiser" Post "Ray-traced lighting denoiser";
    FarShadows "farshadows" Shadows "Terrain far shadow past the cascades";
    SoftShadows "softshadows" Shadows "Soft contact-hardening sun shadows";
    ContactShadows "contactshadows" Shadows "Screen-space contact shadows";
    Ssao "ssao" Post "Screen-space ambient occlusion";
    Bloom "bloom" Post "Bloom";
    Gi "gi" Lighting "Dynamic probe GI";
    ShadowCache "shadowcache" Shadows "Cached point-light shadow maps";
    EnvSlice "envslice" Sky "Time-sliced sky environment map refresh";
    Clouds "clouds" Sky "Clouds";
    SizeCull "sizecull" ViewDistance "Size-based draw distance";
    Actors "actors" Characters "NPCs and creatures";
    FarNpcs "farnpcs" Characters "Far characters hidden (crowd LOD)";
    Walkers "walkers" Characters "NPCs walking";
    Physics "physics" World "Rigid-body physics";
    Fog "fog" Sky "Height fog, ground-fog banks and far fog";
    Froxels "froxels" Sky "Froxel volume (god rays, lit fog)";
    FogDither "fogdither" Sky "Per-froxel sample rotation";
    FogJitter "fogjitter" Sky "Froxel temporal jitter";
    FogHistory "foghistory" Sky "Froxel temporal history";
    FogBank "fogbank" Sky "Depth-aware ground-fog bank";
    FogClamp "fogclamp" Sky "Froxel history clip";
    CloudFog "cloudfog" Sky "Fog split at the clouds' depth";
    StormShape "stormshape" Sky "Far storm shape";
    CloudEdge "cloudedge" Sky "Stable cloud silhouettes";
    CbShape "cbshape" Sky "Storm cloud shape";
    AerialShadow "aerialshadow" Sky "Terrain shadows in the aerial perspective";
    Shafts "shafts" Sky "Screen-space light shaft occlusion";
    SunVis "sunvis" Sky "Per-pixel sun visibility";
    WaterSsr "waterssr" Water "Screen-space reflections on the water";
    ShoreNormals "shorenormals" Water "Wind wavelets up to the shoreline";
    ShoreWaves "shorewaves" Water "Gerstner shore waves and swash";
    SeaWind "seawind" Water "Sea follows the weather's wind";
    ShoreDepth "shoredepth" Water "Shore waves shoal over the bed slope";
    SurfFoam "surffoam" Water "Breaker foam";
    SurfSkew "surfskew" Water "Surf shape";
    Underwater "underwater" Water "Underwater post";
    SeaState "seastate" Water "Weather's sea state";
    WaterRain "water_rain" Water "Rain rings on the water";
    TransFog "transfog" Sky "Fog on blended and additive NIF materials";
    LeafMask "leafmask" Foliage "Foliage cutouts";
    FoliageTransmission "foliagetransmission" Foliage "Diffuse transmission on leaf cutouts";
    SkyMotion "skymotion" Post "Motion vectors for sky and clouds";
    Precombine "precombine" World "Near cells from precombines";
    MatClass "matclass" Debug "Material classes for statics";
    FlatNormal "flatnormal" Debug "Shared flat normal map in material classes";
    TexVariety "texvariety" Debug "Texture variety (off: one texture set, bench only)";
    MeshSlabs "meshslabs" Debug "2 GiB mesh slabs";
    CpuCullTag "cpuculltag" Debug "Near statics skip CPU culling";
    LampFilter "lampfilter" Shadows "2x2 hardware PCF on lamp shadows";
    MoverShadows "movershadows" Shadows "Lamp shadows redraw on moving casters";
    CascadeEncoders "cascadeencoders" Shadows "One command encoder per sun cascade";
    CascadeHold "cascadehold" Shadows "Far sun cascades held between redraws";
    PrecipOcc "precipocc" Effects "Precipitation occlusion map";
    Delight "delight" Textures "De-lit albedo textures";
    NormalMaps "normalmaps" Textures "Texture-pack normal maps";
    Orm "orm" Textures "ORM maps";
    Parallax "parallax" Textures "Parallax occlusion";
    Surface "surface" World "Surface weather";
    Wet "wet" World "Wetness";
    Puddles "puddles" World "Puddles";
    Shelter "shelter" World "Surface shelter map";
    Cover "cover" World "Snow and ash cover";
    Trails "trails" World "Trails in snow and ash";
    Kickup "kickup" Effects "Kick-up in snow and ash";
    Raise "raise" World "Raised cover";
    Scene "scene" World "Static scene from baked cell tables";
    ShiftRebase "shiftrebase" Post "Origin shifts rebase motion vectors";
    SelectGate "selectgate" Debug "Selection gates";
    GpuTerrain "gputerrain" World "GPU clipmap fill of a generated world";
    Groundcover "groundcover" Foliage "Groundcover";
}

/// The registered switch part `name` (or an alias) reports on: its mapped switch, else its
/// `part.<name>` row; `None` for no part or none registered.
pub fn switch_for(switches: &FeatureSwitches, name: &str) -> Option<&'static str> {
    let part = Part::from_name(name)?;
    MAPPED
        .iter()
        .find(|(p, switch, _)| *p == part && switches.get(switch).is_some())
        .map(|(_, switch, _)| *switch)
        .or_else(|| {
            PART_ROWS
                .iter()
                .find(|row| row.part == part)
                .map(|row| row.switch.name())
                .filter(|name| switches.get(name).is_some())
        })
}

/// Adds the part rows and not-built marks for `off` in `finish` ([module docs](self)).
pub struct PartSwitchesPlugin(pub Off);

impl Plugin for PartSwitchesPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        apply(app, &self.0);
    }
}

/// Registers a `part.*` row for every part without a registered switch and marks what `off`
/// left out of the run. After every plugin's switches (a `finish`).
pub fn apply(app: &mut App, off: &Off) {
    app.world_mut().init_resource::<FeatureSwitches>();
    for row in PART_ROWS {
        let reason = format!("--off {}", row.part.name());
        let built = !off.has(row.part);
        let mapped = MAPPED.iter().find(|(p, switch, _)| {
            *p == row.part && app.world().resource::<FeatureSwitches>().get(switch).is_some()
        });
        if let Some((_, switch, kind)) = mapped {
            if !built {
                match kind {
                    Kind::CommandLine => {}
                    Kind::Live(with) if !with.iter().any(|p| off.has(*p)) => {
                        let mut tunables = app.world_mut().resource_mut::<Tunables>();
                        let id = tunables.id(switch).expect("a registered switch has its tunable");
                        if let Err(e) = tunables.set_noted(id, false.into(), SetBy::CommandLine, Some(reason.clone())) {
                            bevy_log::warn!("{switch}: {e}");
                        }
                    }
                    Kind::Removed | Kind::Live(_) => {
                        app.mark_feature_not_built(switch, &reason);
                    }
                }
            }
            continue;
        }
        let _ = app.add_tunable(&row.def);
        if !built {
            let mut tunables = app.world_mut().resource_mut::<Tunables>();
            let id = tunables.id(row.def.name()).expect("registered above");
            if let Err(e) = tunables.set_noted(id, false.into(), SetBy::CommandLine, Some(reason.clone())) {
                bevy_log::warn!("{}: {e}", row.def.name());
            }
        }
        app.add_feature_switch(&row.switch);
        if !built {
            app.mark_feature_not_built(row.switch.name(), &reason);
        }
    }
}

#[cfg(test)]
#[path = "part_switches_tests.rs"]
mod tests;
