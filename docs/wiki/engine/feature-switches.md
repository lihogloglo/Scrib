# Feature switches and the Features panel

## What it does
Every costly feature has one on/off switch: a tunable plus metadata (label, category, skips or only hides, cost tags,
requirements). Console, command line, BRP, the dev Features panel (Ctrl+F3) and the Advanced options page all set the
same tunable. Flipping a switch with a panel open measures the frame change; the A/B button measures its cost.

## How it works
- **Core** `superfargoth/crates/sf_tunables/src/feature.rs`: `FeatureSwitch` (`new` for a `Def<bool>`, `choice` with an
  off value, `named`), `Effect::{Skips, Hides}`, `Cost::{Gpu, Cpu}`, `Category`, `startup_only`. `FeatureSwitches::state`
  gives `on`, the effective `SetBy` layer and availability (`Available`, `NotBuilt { reason }`, `Blocked { by }`).
- **Bevy** `superfargoth/crates/sf_tunables_bevy/src/feature.rs`: resource `FeatureSwitches`, `app.add_feature_switch`
  (after its tunable), `mark_feature_not_built`, run condition `feature_on(name)`, message `FeatureToggled { name, running }`
  sent in `TunableSystems::Apply` (also for switches whose requirement flipped).
- **Skipping is the owner's job**: run conditions, despawning, or a resource that makes the pass return. Live skips today:
  groundcover (`sf_groundcover_bevy/src/tunables.rs`, `groundcover.enabled`), froxels (`sf_atmos_bevy`,
  `sky.froxels.enabled`), terrain far map (`sf_terrain_bevy/src/far_map.rs`, `shadow.terrain_far.enabled`).
- **Registered**: 14 next to their tunables (sky, shadows, SSAO, upscaling, bloom, water, surface); 22 live A/B
  switches as interim game tunables synced both ways with `Switch` (`mw_devtools_bevy/src/live_switches.rs`); every
  startup `Part` (`mw_devtools_bevy/src/part_switches.rs`): mapped to its switch (`--off` writes it at `CommandLine`, or
  marks it not built when the run lacks the feature), else a `part.<name>` Dev row that answers "startup only: relaunch
  with --off x". A test fails if a new `Part` has neither.
- **Console** `superfargoth/crates/sf_console_bevy/src/features.rs`: `toggle <name>`, `features [filter]` (by category:
  state, layer, skips/hides, lifetime, availability), `set_feature` (refuses not-built and startup-only). Game
  `on/off/flip <name>` fall back to switches and part names (`mw_devtools_bevy/src/console.rs`).
- **Panel** `mw_devtools_bevy/src/features_panel.rs` (+ `_model`, `_ui`): Ctrl+F3 or `flip featurespanel`, every
  interactive run incl. release. An overlay: opens with `PlayerMode::paused` (controls stop, cursor free, clock runs).
  Tabs per category; row = toggle, label, name, layer, badges (`restart`, `hides`, `not built`, `blocked by`, `re-bake`),
  frame change, A/B. Writes the `Console` layer only (never saved); "Reset all" peels it.
- **Probe** `mw_devtools_bevy/src/switch_probe.rs` (+ `_model`): on `FeatureToggled`, mean frame/main/render/GPU over
  `diag.switch.window_s` before vs after `diag.switch.settle_s`; flags `noise` (< `diag.switch.noise_ms`), `moved`
  (camera past `still_m`/`still_deg`), `superseded`. Samples from the F3 panel's `PerfFrameFeed`; gauges run only while
  the panel is open or a flip waits. `QuickAb`: `diag.switch.ab_rounds` off/on pairs, median cost, restores the switch.
- **Player view**: Graphics > Advanced (`mw_exterior_bevy/src/advanced_page.rs`), rows generated from player-scope
  switches, `User` layer, `Machine` scope ([UI and menus](ui-and-menus.md)).

## Bevy features used
Run conditions, messages, `bevy_ui` + `bevy_ui_widgets::Button`, `ScrollArea`; see [schedules](../bevy/schedules-transforms-ecs.md).

## Switches and counters
`features` lists all of them; tunables `diag.switch.*`. No new gauges.

## Known limits
Hides-only switches still pay their cost (surface weather, night sky, ambient, terrain RO-16 set, puddles). No cost
column per GPU pass yet (cost tags declared, not matched to timers). Fog, froxels, sun visibility, groundcover, far
shadows and particles are in no `sg.*` preset group. `set part.x` writes the value past the refusal; `--off sky` leaves
cloud, fog and night switches "available". A flip with both panels closed records no delta.

## Specs
`superfargoth/docs/specs/feature-switches.md`; menus `superfargoth/docs/specs/menus.md` §7.
