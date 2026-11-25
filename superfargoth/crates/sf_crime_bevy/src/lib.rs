//! Bevy shell for [`sf_crime`] (`docs/specs/crime.md` "Crates and layers": host systems and the
//! wiring to AI). Slice `LW4b2`: the stealth tunables.
//!
//! - [`TUNABLES`]: the `crime.stealth.*` tunables, one per [`StandardDetectionConfig`] field, live
//!   (net `Gameplay`, scope `Dev`, source `Guess`). Their defaults are the struct's
//!   ([`StandardDetectionConfig::default`]; a test keeps the two equal).
//! - [`CrimeStealthPlugin`]: binds them into [`CrimeStealthSettings`] and installs
//!   `StandardDetection::new(settings)` as `sf_ai_bevy`'s [`AiDetectionRules`] on the first frame,
//!   then again (an `Arc` swap) only when a value changed, in `PreUpdate` after the tunable bind,
//!   so the fixed-tick sight of the same frame reads the new rules.
//! - **Ownership:** the plugin owns `AiDetectionRules` while the installed rules are the last ones
//!   it put there. Once something else replaces them (a game's own [`sf_crime::DetectionRules`],
//!   after the first frame), a tunable change no longer touches the resource; a game that brings
//!   its own rules from the start simply doesn't add this plugin.

use std::f32::consts::{FRAC_PI_4, PI};
use std::sync::Arc;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;
use sf_ai_bevy::AiDetectionRules;
use sf_crime::{DetectionRules, StandardDetection, StandardDetectionConfig};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, def};
use sf_tunables_bevy::{TunableSystems, TunablesAppExt};

#[cfg(test)]
mod tests;

// --- Tunables ---------------------------------------------------------------------------------

/// Gameplay metadata shared by every stealth tunable (net `Gameplay`, scope `Dev`, source
/// `Guess`, live).
const fn gp(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

pub const LIGHT_CONCEALMENT: Def<f32> = gp(def!(float "crime.stealth.light_concealment" = 100.0)
    .range(0.0, 200.0)
    .help("Concealment in full darkness (scaled by 1 - light), points; 100 keeps an upright still target's visibility = its light"));
pub const CROUCHED_CONCEALMENT: Def<f32> = gp(def!(float "crime.stealth.crouched_concealment" = 30.0)
    .range(0.0, 100.0)
    .help("Concealment from crouching, points (Skyrim/AC: crouch hides about a third of a lit target)"));
pub const PRONE_CONCEALMENT: Def<f32> = gp(def!(float "crime.stealth.prone_concealment" = 50.0)
    .range(0.0, 150.0)
    .help("Concealment from lying prone, points (more than crouched, MGS/AC)"));
pub const SPEED_FREE_MPS: Def<f32> = gp(def!(float "crime.stealth.speed_free_mps" = 1.5)
    .range(0.0, 10.0)
    .help("Speed up to which moving reveals nothing (a sneak walk), m/s"));
pub const SPEED_REVEAL_PER_MPS: Def<f32> = gp(def!(float "crime.stealth.speed_reveal_per_mps" = 10.0)
    .range(0.0, 50.0)
    .help("Concealment lost per m/s above speed_free_mps, points per m/s"));
pub const SPEED_REVEAL_MAX: Def<f32> = gp(def!(float "crime.stealth.speed_reveal_max" = 60.0)
    .range(0.0, 200.0)
    .help("Most concealment speed can take away, points"));
pub const FACING_FRONT_RAD: Def<f32> = gp(def!(float "crime.stealth.facing_front_rad" = FRAC_PI_4)
    .range(0.0, PI)
    .help("Half-angle of the observer's front, where facing hides nothing, radians"));
pub const BEHIND_CONCEALMENT: Def<f32> = gp(def!(float "crime.stealth.behind_concealment" = 40.0)
    .range(0.0, 100.0)
    .help("Concealment straight behind the observer (linear from facing_front_rad), points"));
pub const MAX_CONCEALMENT: Def<f32> = gp(def!(float "crime.stealth.max_concealment" = 200.0)
    .range(100.0, 400.0)
    .help("Upper clamp of the concealment, points (crime.md 0..~200; 100 = invisible to the Meter model)"));

/// The stealth tunables (`crime.stealth.*`), bound by [`CrimeStealthPlugin`].
pub const TUNABLES: &[&dyn AnyDef] = &[
    &LIGHT_CONCEALMENT,
    &CROUCHED_CONCEALMENT,
    &PRONE_CONCEALMENT,
    &SPEED_FREE_MPS,
    &SPEED_REVEAL_PER_MPS,
    &SPEED_REVEAL_MAX,
    &FACING_FRONT_RAD,
    &BEHIND_CONCEALMENT,
    &MAX_CONCEALMENT,
];

// --- Plugin -----------------------------------------------------------------------------------

/// The [`StandardDetection`] config, bound from [`TUNABLES`].
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CrimeStealthSettings(pub StandardDetectionConfig);

/// Binds [`TUNABLES`] and keeps [`AiDetectionRules`] = `StandardDetection` with their values
/// (module docs). Needs no other plugin; with `sf_tunables_bevy::TunablesPlugin` the values follow
/// the console, files and presets live.
#[derive(Default)]
pub struct CrimeStealthPlugin;

macro_rules! bind {
    ($app:ident, $def:expr, |$c:ident, $v:ident| $body:expr) => {
        let _ = $app.add_tunable(&$def).bind_resource(|s: &mut CrimeStealthSettings, $v| {
            let $c = &mut s.0;
            $body;
        });
    };
}

impl Plugin for CrimeStealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrimeStealthSettings>().add_systems(
            PreUpdate,
            install_rules.after(TunableSystems::Bind).run_if(resource_changed::<CrimeStealthSettings>),
        );
        bind!(app, LIGHT_CONCEALMENT, |c, v| c.light_concealment = v);
        bind!(app, CROUCHED_CONCEALMENT, |c, v| c.crouched_concealment = v);
        bind!(app, PRONE_CONCEALMENT, |c, v| c.prone_concealment = v);
        bind!(app, SPEED_FREE_MPS, |c, v| c.speed_free_mps = v);
        bind!(app, SPEED_REVEAL_PER_MPS, |c, v| c.speed_reveal_per_mps = v);
        bind!(app, SPEED_REVEAL_MAX, |c, v| c.speed_reveal_max = v);
        bind!(app, FACING_FRONT_RAD, |c, v| c.facing_front_rad = v);
        bind!(app, BEHIND_CONCEALMENT, |c, v| c.behind_concealment = v);
        bind!(app, MAX_CONCEALMENT, |c, v| c.max_concealment = v);
    }
}

/// The rules this plugin last installed and their config.
type Installed = Option<(StandardDetectionConfig, Arc<dyn DetectionRules>)>;

/// Installs `StandardDetection` with the settings: on the first run, then only when a value
/// changed and the current rules are still the ones installed here (module docs, "Ownership").
fn install_rules(
    mut commands: Commands,
    settings: Res<CrimeStealthSettings>,
    current: Option<Res<AiDetectionRules>>,
    mut installed: Local<Installed>,
) {
    if let Some((config, rules)) = installed.as_ref() {
        if *config == settings.0 {
            return;
        }
        if current.as_ref().is_some_and(|c| !Arc::ptr_eq(&c.0, rules)) {
            return;
        }
    }
    let rules: Arc<dyn DetectionRules> = Arc::new(StandardDetection::new(settings.0));
    commands.insert_resource(AiDetectionRules(rules.clone()));
    *installed = Some((settings.0, rules));
}
