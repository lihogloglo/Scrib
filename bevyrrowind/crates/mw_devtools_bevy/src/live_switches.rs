//! The live A/B [`Switch`]es as feature switches (slice FS-2d; spec
//! `superfargoth/docs/specs/feature-switches.md`), so `features`, `toggle` and the Features panel
//! cover every F-key / `on|off|flip` switch that turns a feature on or off.
//!
//! Each feature [`Switch`] gets a `Bool`, `Lifetime::Live`, Dev tunable named after the feature in
//! its system's namespace ([`LIVE`]) and a [`FeatureSwitch`] on it. The two stay in step both
//! ways ([`sync`], `Update` after the keys):
//!
//! - The tunable changed (`toggle`, the Features panel, `set`): the switch is applied through
//!   [`Switch::set`], the same path the keys use.
//! - The switch changed (F-key, BRP `set`, console `on`/`off`/`flip <switch>`): its state is
//!   written to the tunable at the `Console` layer.
//! - The switch's resource appears (start, door travel): a tunable some layer chose applies to
//!   it; one still at its default takes the game's start state at the `Code` layer.
//!
//! Not feature switches (pure dev visualisers): `wireframe`, `terrain_lod_colors`, `rings`,
//! `far_tint` and the two panels. The RO-16 `terrain_*` scaffolds are in category `Debug`.
//! GI (`render.gi`, a console variable) has its switch in [`crate::console`].
//!
//! Interim shape, removal path: the tunables live here in the game devtools for now; a later slice
//! moves each one into its owning crate (`sf_atmos_bevy`, `sf_surface_bevy`, `sf_terrain_bevy`,
//! `mw_assets`, ...), which then reads the tunable itself, and its [`Switch`] arm and row here go.
//! Moved so far (an `owned` row, still synced with its key): `shadow.terrain_far.enabled`
//! (`sf_terrain_bevy`, FS-L3: off skips the far shadow map's bake).

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

use sf_tunables_bevy::sf_tunables::{Category, Def, Effect, FeatureSwitch, Lifetime, OffValue, Scope, Source};
use sf_tunables_bevy::{SetBy, TunableId, Tunables, TunablesAppExt};

use crate::live_ab::{LiveAb, Switch, Switches};

/// One live switch's tunable and feature switch.
pub struct LiveSwitch {
    pub switch: Switch,
    pub def: &'static Def<bool>,
    pub feature: &'static FeatureSwitch,
}

/// One [`LIVE`] row: declared here (Dev scope), or `owned` by the crate that runs the feature (it
/// registers the pair itself; registering it here again is a no-op).
macro_rules! row {
    ($switch:ident owned $def:path, $feature:path) => {
        LiveSwitch { switch: Switch::$switch, def: &$def, feature: &$feature }
    };
    ($switch:ident $name:literal $default:literal $cat:ident $effect:ident [$($req:literal),*] $label:literal) => {
        LiveSwitch {
            switch: Switch::$switch,
            def: &Def::<bool>::new($name, $default)
                .help(concat!($label, " (live A/B switch: F-keys, BRP and on/off/flip write it at the console layer)"))
                .source(Source::Owner("2026-11-23"))
                .scope(Scope::Dev)
                .lifetime(Lifetime::Live)
                .at(file!(), line!()),
            feature: &FeatureSwitch::named($name, OffValue::Bool(false))
                .label($label)
                .category(Category::$cat)
                .effect(Effect::$effect)
                .requires(&[$($req),*])
                .at(file!(), line!()),
        }
    };
}

// Effects from `superfargoth/docs/research/feature-switches-inventory-2026-11-23.md`: a shader bit,
// cleared flags or a look variant hides; skipped passes or rebuilt materials skip.
/// Every feature [`Switch`], in [`Switch::ALL`] order.
pub static LIVE: &[LiveSwitch] = &[
    row!(SunShadows "shadow.sun.enabled" true Shadows Skips [] "Sun shadows"),
    // FS-L3: declared and registered by the terrain plugin, whose far map reads it.
    row!(TerrainFarShadows owned sf_terrain_bevy::FAR_SHADOWS_ENABLED, sf_terrain_bevy::FAR_SHADOWS),
    row!(Clouds "sky.clouds.enabled" true Sky Skips [] "Clouds and cloud shadows"),
    row!(TerrainSpecular "terrain.specular.enabled" true Debug Hides [] "Terrain specular lobes (RO-16)"),
    row!(TerrainEnvSpecular "terrain.env_specular.enabled" true Debug Hides [] "Terrain env-map specular (RO-16)"),
    row!(TerrainIndirect "terrain.indirect.enabled" true Debug Hides [] "Terrain indirect light (RO-16)"),
    row!(TerrainTint "terrain.tint.enabled" true Debug Hides [] "Terrain land tint and macro variation (RO-16)"),
    row!(Surface "surface.enabled" true Effects Hides [] "Surface weather"),
    row!(Fog "sky.fog.enabled" true Sky Skips [] "Fog: height fog, banks and froxels"),
    row!(FogNightLight "sky.fog.night_light.enabled" true Sky Hides ["sky.fog.enabled"] "Fog lit by the ambient floor"),
    row!(WeatherAmbient "sky.weather_ambient.enabled" true Lighting Hides [] "Weather ambient tint"),
    row!(AmbientSpecular "render.ambient_specular.enabled" false Lighting Hides [] "Flat ambient specular"),
    row!(CloudHistory "sky.clouds.history.enabled" true Sky Hides ["sky.clouds.enabled"] "Cloud temporal accumulation"),
    row!(NightSkyGlow "sky.night.glow.enabled" true Sky Hides [] "Night sky glow"),
    row!(Moonlight "sky.moonlight.enabled" true Lighting Hides [] "Artistic moonlight"),
    row!(Cover "surface.cover.enabled" true Effects Hides ["surface.enabled"] "Snow and ash cover"),
    row!(Trails "surface.trails.enabled" true Effects Hides ["surface.enabled"] "Trails in the cover"),
    row!(Raise "surface.raise.enabled" true Effects Hides ["surface.enabled"] "Raised snow (displacement)"),
    row!(NormalMaps "texture.normal_maps.enabled" true Textures Skips [] "Texture-pack normal maps"),
    row!(Orm "texture.orm.enabled" true Textures Skips [] "ORM maps"),
    row!(Parallax "texture.parallax.enabled" true Textures Skips [] "Parallax (depth maps)"),
];

/// The live entry of `switch`, `None` for a dev visualiser.
pub fn live_of(switch: Switch) -> Option<&'static LiveSwitch> {
    LIVE.iter().find(|l| l.switch == switch)
}

/// Registers every [`LIVE`] tunable and its feature switch (in `build`, before the parts'
/// `finish`, so `--off <part>` reaches them).
pub fn register(app: &mut App) {
    for live in LIVE {
        let _ = app.add_tunable(live.def);
        app.add_feature_switch(live.feature);
    }
}

/// Registers the live switches and keeps them in step with their tunables ([module docs](self)).
pub struct LiveSwitchesPlugin;

impl Plugin for LiveSwitchesPlugin {
    fn build(&self, app: &mut App) {
        register(app);
        let ids = {
            let tunables = app.world().resource::<Tunables>();
            LIVE.iter().map(|l| tunables.id(l.def.name()).expect("registered above")).collect()
        };
        app.insert_resource(LiveSync { ids, applied: vec![None; LIVE.len()] })
            .add_systems(Update, sync.after(crate::live_ab::keys));
    }
}

/// Per [`LIVE`] row: its tunable and the generation last applied (`None`: the switch has no
/// resource to act on, or not seen yet).
#[derive(Resource)]
pub struct LiveSync {
    ids: Vec<TunableId>,
    applied: Vec<Option<u64>>,
}

/// Keeps each live switch and its tunable in step ([module docs](self)).
#[allow(clippy::needless_pass_by_value)]
pub fn sync(mut tunables: ResMut<Tunables>, mut state: ResMut<LiveAb>, mut s: Switches, mut sync: ResMut<LiveSync>, real: Res<Time<Real>>) {
    let now = real.elapsed_secs();
    let LiveSync { ids, applied } = &mut *sync;
    for ((live, &id), applied) in LIVE.iter().zip(ids.iter()).zip(applied.iter_mut()) {
        let Some(current) = live.switch.get(&state, &s) else {
            *applied = None;
            continue;
        };
        let (want, layer, generation) = {
            let var = tunables.var(id);
            (tunables.flag(id), var.set_by(), var.generation())
        };
        match *applied {
            // Appeared: a chosen value applies, an untouched default takes the game's start state.
            None if layer == SetBy::Default => {
                if current != want {
                    let _ = tunables.set_noted(id, current.into(), SetBy::Code, Some("the game's start state".to_owned()));
                }
            }
            Some(seen) if seen == generation => {
                // Flipped by a key, BRP or `on`/`off`/`flip`: the console layer.
                if current != want {
                    let note = format!("flip {}", live.switch.name());
                    let _ = tunables.set_noted(id, current.into(), SetBy::Console, Some(note));
                }
            }
            // The tunable changed (or a chosen value meets a new resource).
            _ => {
                if current != want {
                    let text = live.switch.set(&mut state, &mut s, Some(want));
                    state.note(text, now);
                }
            }
        }
        *applied = Some(tunables.var(id).generation());
    }
}

#[cfg(test)]
#[path = "live_switches_tests.rs"]
mod tests;
