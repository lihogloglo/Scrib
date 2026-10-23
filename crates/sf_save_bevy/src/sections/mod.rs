//! The framework's sections (save.md "What is saved"): keys, current schema versions, saved types
//! and [`SaveSection`] constructors.
//!
//! The state lives in resources of other crates or of the game, so each constructor takes the
//! game's accessors (built with [`read`](crate::read) / [`write`](crate::write) or by hand):
//!
//! ```ignore
//! app.add_save_section(sections::rpg(read(|r: &Rpg| r.0.clone()), write(|r: &mut Rpg, v| r.0 = v)));
//! ```
//!
//! Versions start at 1 with S2. A shape change bumps the version, freezes the old type in a
//! `legacy` module and adds a typed step (save.md "Versioning and migration"); the shape goldens
//! in this crate's tests fail until that is done.

pub mod character;
pub mod overlay;
pub mod records;
#[cfg(feature = "script")]
pub mod script;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use sf_rpg::WorldRpg;
use sf_save::{Family, SectionKey};
use sf_sky::SkyClock;
use sf_weather::WeatherState;
use sf_wind::WindState;
use sf_world::PlacementOverlay;

use crate::SaveSession;
use crate::section::SaveSection;
pub use character::{CharacterBody, CharacterSave, VersionedBlob, character_key, player_of};
pub use overlay::{BlobVersions, OverlaySave};
pub use records::{CreatedRecord, RecordOverlay, RecordPatch, SAVE_NAMESPACE, SavedOp, SavedValue, Skipped};

/// The framework's section owner.
pub const OWNER: &str = "sf";

pub const CLOCK: &str = "clock";
pub const CLOCK_VERSION: u32 = 1;
pub const WEATHER: &str = "weather";
pub const WEATHER_VERSION: u32 = 1;
pub const RECORDS: &str = "records";
pub const RECORDS_VERSION: u32 = 1;
pub const OVERLAY: &str = "overlay";
pub const OVERLAY_VERSION: u32 = 1;
pub const RPG: &str = "rpg";
pub const RPG_VERSION: u32 = 1;
/// Family of `sf/character/<player key>`.
pub const CHARACTER: &str = "character";
pub const CHARACTER_VERSION: u32 = 1;
/// Family of `<mod id>/script`.
pub const SCRIPT: &str = "script";
pub const SCRIPT_VERSION: u32 = 1;

/// Apply order on load (save.md "Loading" step 4): records, overlay, clock and weather, RPG,
/// party and characters, adapter, scripts (restored last so `on_load` sees everything).
pub mod order {
    pub const RECORDS: i32 = 0;
    pub const OVERLAY: i32 = 100;
    pub const CLOCK: i32 = 200;
    pub const WEATHER: i32 = 250;
    pub const RPG: i32 = 300;
    pub const PARTY: i32 = 400;
    pub const CHARACTERS: i32 = 450;
    /// Adapter sections (`mw/globals`): the default for game sections.
    pub const ADAPTER: i32 = 500;
    pub const SCRIPTS: i32 = 600;
}

/// `sf/<name>`.
pub fn key(name: &str) -> SectionKey {
    SectionKey::new(OWNER, name).expect("framework keys are valid")
}

/// `sf/clock`: the sky clock and the fixed simulation tick.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClockSave {
    pub clock: SkyClock,
    /// Fixed ticks since the world began ([`SaveSession::sim_tick`]).
    pub sim_tick: u64,
}

/// `sf/weather`: the weather's seed and explicit overrides, and the wind with its live sources.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WeatherSave {
    pub weather: WeatherState,
    pub wind: Option<WindState>,
}

/// `sf/clock` over the game's clock resource (`sf_sky_bevy::SkyTime`); the sim tick comes from
/// and returns to [`SaveSession`].
pub fn clock(
    get: impl Fn(&World) -> Option<SkyClock> + Send + Sync + 'static,
    set: impl Fn(&mut World, SkyClock) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::value(
        key(CLOCK),
        CLOCK_VERSION,
        move |w: &World| get(w).map(|clock| ClockSave { clock, sim_tick: w.get_resource::<SaveSession>().map_or(0, |s| s.sim_tick) }),
        move |w: &mut World, v: ClockSave| {
            set(w, v.clock);
            if let Some(mut s) = w.get_resource_mut::<SaveSession>() {
                s.sim_tick = v.sim_tick;
            }
        },
    )
    .order(order::CLOCK)
}

/// `sf/weather` over the game's weather (`sf_weather_bevy::WorldWeather`) and wind
/// (`sf_wind_bevy::WindField::state`).
pub fn weather(
    get: impl Fn(&World) -> Option<WeatherSave> + Send + Sync + 'static,
    set: impl Fn(&mut World, WeatherSave) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::value(key(WEATHER), WEATHER_VERSION, get, set).order(order::WEATHER)
}

/// `sf/records`: the runtime record overlay. `set` should also re-apply it to the record
/// database ([`RecordOverlay::apply`]).
pub fn records(
    get: impl Fn(&World) -> Option<RecordOverlay> + Send + Sync + 'static,
    set: impl Fn(&mut World, RecordOverlay) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::value(key(RECORDS), RECORDS_VERSION, get, set).order(order::RECORDS)
}

/// `sf/overlay` over the game's placement overlay (`sf_world_bevy::WorldOverlay`). `get` returns
/// a **copy** with the live state of resident placements flushed in (save.md "One path for unload
/// and save"), without removing anything. Blob versions come from [`BlobVersions`].
pub fn overlay(
    get: impl Fn(&World) -> Option<PlacementOverlay> + Send + Sync + 'static,
    set: impl Fn(&mut World, PlacementOverlay) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::value(
        key(OVERLAY),
        OVERLAY_VERSION,
        move |w: &World| {
            let blob_versions = w.get_resource::<BlobVersions>().map(|b| b.0.clone()).unwrap_or_default();
            get(w).map(|overlay| OverlaySave { overlay, blob_versions })
        },
        move |w: &mut World, v: OverlaySave| set(w, v.overlay),
    )
    .check(|w: &World, v: &OverlaySave| v.check(&w.get_resource::<BlobVersions>().cloned().unwrap_or_default()))
    .order(order::OVERLAY)
}

/// `sf/rpg`: quests, journal and known topics (with the heard dialogue lines).
pub fn rpg(
    get: impl Fn(&World) -> Option<WorldRpg> + Send + Sync + 'static,
    set: impl Fn(&mut World, WorldRpg) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::value(key(RPG), RPG_VERSION, get, set).order(order::RPG)
}

/// `sf/character/<player key>`, one per character `get` returns; `set` receives every character
/// the save holds.
pub fn characters(
    get: impl Fn(&World) -> Vec<CharacterSave> + Send + Sync + 'static,
    set: impl Fn(&mut World, Vec<CharacterSave>) + Send + Sync + 'static,
) -> SaveSection {
    SaveSection::family(
        Family { owner: Some(OWNER.to_owned()), family: CHARACTER.to_owned() },
        CHARACTER_VERSION,
        move |w: &World| Ok(get(w).into_iter().map(|c| (character_key(c.player), c)).collect()),
        move |w: &mut World, all: Vec<(SectionKey, CharacterSave)>| {
            let mut out = Vec::with_capacity(all.len());
            for (key, c) in all {
                if player_of(&key) != Some(c.player) {
                    return Err(format!("section {key} holds player {:032x}", c.player.0));
                }
                out.push(c);
            }
            set(w, out);
            Ok(())
        },
    )
    .order(order::CHARACTERS)
}
