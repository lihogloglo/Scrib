# Feature switches: live on/off for every costly feature

Status: built 2026-11-23 (FS-1a..FS-L3, owner check FS-1 confirmed; session `feature-toggles`). Owner ask 2026-11-23: FPS dropped after the atmosphere and
groundcover work; switch each feature off live (god rays, groundcover, GI, SSAO, ...) from the console and from a
proper UI, to see what costs what. Research behind this spec: `../research/feature-switches-inventory-2026-11-23.md`
(what exists, path:line), `../research/feature-toggles-menus-industry-2026-11-23.md` (industry, Bevy 0.20, crates).
Menu flow and the options screen are in [menus.md](menus.md) §7; this spec is the switches and their dev panel.

## Goal

- **One switch per feature**, live wherever the feature allows it, and an honest label where it doesn't yet
  ("applies on restart", "hides only, cost still paid").
- **One variable, many views**: console (`toggle froxels`), command line (`--off froxels`), BRP, the dev
  Features panel, and, for player-scope switches, the Graphics options. All set the same tunable.
- **Cost beside the switch**: each switch says what it costs (GPU pass ms, CPU system ms) and what flipping it
  changed, so finding the FPS drop is a walk down one list, in normal play, release build included.
- **"Off" means the work stops.** A switch that only zeroes a shader term is marked `Hides` until a slice makes
  it skip.

Non-goals: new quality levels per feature (tunables T5 presets do that), automated benches (perf suite stays),
judging the look (owner).

## What exists (2026-11-23)

Three mechanisms grew side by side (inventory §3):

| Mechanism | Where | Count | Live | Skips cost |
|---|---|---|---|---|
| Startup `Part` (`--off x`, `BEVYRROWIND_OFF`) | `mw_game_core/src/perf.rs:11` | ~90 | no | yes (plugin/system not added) |
| Live `Switch` (F-keys, `on/off/flip x`, BRP `bevyrrowind/set`) | `mw_devtools_bevy/src/live_ab.rs:174` | 26 | yes | often only a shader bit |
| Tunables (`set x v`, `--set`, presets) | `sf_tunables`, `sf_tunables_bevy` | hundreds | per lifetime | per tunable |

Gaps that matter now: groundcover, froxels alone, terrain far map (live switch hides only), water, precipitation,
particles, lamp lights, carried-light redraws have no live off that skips work; fog, froxels, sun visibility,
shafts, groundcover, far shadows and particles are in no `sg.*` preset group; `--off shadows` stops only the sun.

## Reuse check (rule 0)

- **Bevy 0.20**: no feature registry. `bevy_dev_tools` has the FPS/diagnostics overlays and `render_debug`
  visualisers, no on/off registry. `bevy_feathers` has editor widgets (toggle switch, slider, list view), fine for
  a dev surface but BSN-scene based with an editor look. `bevy_ui_widgets` (checkbox, slider, tabs, modal) is
  what our options screen already uses. Run conditions and camera components (Bevy's own way to turn SSAO, bloom,
  TAA on and off per camera) are the skip mechanism.
- **Crates**: `bevy_egui` 0.43 / `bevy-inspector-egui` 0.38 support 0.20 (MIT/Apache) but bring a second UI stack;
  `bevy_console` duplicates `sf_console`. None fits the registry. egui stays an optional dev feature, not this.
- **Ours**: `sf_tunables` already has typed values, `SetBy` layers, lifetimes, scope (`Dev`/`Player`), groups,
  console and BRP. A switch is a tunable plus a little metadata. No new store.

## Industry comparison

| Engine | Has | Us |
|---|---|---|
| UE5 | `ShowFlag.*` / `show x` per-view feature flags, `r.*` cvars with `ECVF_SetBy*` priority, `sg.*` scalability, `stat gpu` per-pass ms as a separate tool | switches are tunables with `SetBy` layers (same order); cost shown *beside* the switch (UE keeps it apart) |
| Unity HDRP | Frame Settings (per-camera feature on/off) + Rendering Debugger runtime UI; debug overrides never persist; a debug toggle can't enable what the quality tier didn't build | dev panel writes the `Console` layer (never saved); a feature not built this run is a disabled row with the reason |
| Source / idTech | `FCVAR_*` flags (cheat, archive, dev-only), `r_`/`mat_` cvars | `Scope::Dev` vs `Player`, persistence only from the `User` layer |
| OpenMW | settings + F3/F4 profiler bars | F3 panel exists; Features panel adds the switch list |
| Cyberpunk 2.x, Nixxes ports | advanced graphics page, per-setting impact hints, presets + Custom | player-scope switches on Graphics > Advanced with a cost class from the same cost tags (menus.md §7) |

## Design

### The switch (core, `sf_tunables`)

A feature switch is metadata attached to an existing `Bool` (or `Choice` with an off value) tunable, declared next
to it with the same `const fn` builder style:

```rust
pub const FROXELS: FeatureSwitch = FeatureSwitch::new(&FROXELS_ENABLED)      // the tunable it drives
    .label("Volumetric fog (froxels)").category(Group::Sky)
    .effect(Effect::Skips)                       // Skips | Hides
    .cost(&[Cost::Gpu("froxel"), Cost::Cpu("sf_atmos::froxel_inject")])
    .requires(&["sky.fog.enabled"])              // other switches that must be on
    .at(file!(), line!());
```

- **Fields**: tunable name (the switch's console name is the tunable's: one namespace), off value (`false`, or a
  choice such as `off` for `render.bloom`), label, category (`Category`: the ten `Group`s plus `World` and `Debug`, a separate
  enum because each `Group` has exactly one `sg.*` driver; default from the tunable's group, else `World`), `effect`, `cost` tags, `requires`, scope and lifetime come from the
  tunable (`Lifetime::Restart` = still startup-only; the panel shows it).
- **Names**: `<system>.<feature>.enabled` for new ones (as `sky.sun_vis.enabled`, `shadow.lamp.atlas.enabled`);
  existing tunables keep their names.
- **Registry**: `FeatureSwitches` lists every registered switch with its effective state, layer, availability
  (`Available`, `NotBuilt { reason }` when the run was started without it, `Blocked { by }` when a `requires` is
  off). Pure, Bevy-free, exact tests.
- **Framework/game split**: framework features declare their switches in their `sf_*` crate; Morrowind-only
  ones in `mw_*`. The startup `Part` names (game) map to switches in a game table, so `--off froxels` sets
  `FROXELS_ENABLED` at `CommandLine` where a switch exists (one namespace, the old part name kept as an alias).

### Live in Bevy (`sf_tunables_bevy`)

- `app.add_feature_switch(&FROXELS)` registers it (the tunable must be registered too).
- **Skipping is the owning plugin's job**: a run condition `feature_on(&FROXELS)` on its systems and render graph
  node, or removing the camera component (Bevy's own pattern for SSAO/bloom/TAA), or despawning/hiding its
  entities. Hiding by a shader bit is legal only with `Effect::Hides`.
- **Startup-only features** stay `Lifetime::Restart`: their row exists from day one with "applies on restart";
  each conversion to `Live` is its own slice (Build plan FS-L*), ordered by the suspect list.
- `FeatureToggled { name, on }` message; the cost probe (below) listens to it.

### Console and command line

- `toggle <name>` flips a switch (Console layer); `features [filter]` lists switches: state, layer, effect, lifetime,
  availability, last measured delta. `reset <name>` as today. `on/off/flip <name>` keep working: the live `Switch`
  names become aliases of switch tunables as each is converted (FS-2).
- `--off <part>` and `BEVYRROWIND_OFF` unchanged for benches; where a switch exists they write it at `CommandLine`.
  New part `lampshadows`; `shadows` keeps its meaning (sun only, ledger lines stay reproducible) with its help text
  saying so.

### Cost beside the switch

- **Cost tags**: `Cost::Gpu(label)` matches GPU pass timer labels (`sf_diag_bevy`, the F3 panel's top passes);
  `Cost::Cpu(name)` matches a system or set span. The panel sums the matching timers, shown when GPU timers are on
  (today `BEVYRROWIND_PANEL_GPU_TIMERS=1` at start; FS-C makes it a live tunable `diag.gpu_timers`).
- **Flip delta** (our addition; nothing in UE or Unity, cheap and what the owner needs): on `FeatureToggled`, the
  probe compares the mean frame, main, render and GPU ms of `diag.switch.window_s` (2 s, Guess) before the flip
  with the same window after `diag.switch.settle_s` (0.5 s, Guess). The row shows "frame -1.3 ms (render -0.9)",
  greyed under `diag.switch.noise_ms` (0.5 ms, Measured: ledger noise floor). A **quick A/B** action alternates
  off/on `diag.switch.ab_rounds` (3) times and reports the median delta: an interleaved mini A/B in ~15 s, not a
  bench. A move of the camera during the window marks the result "moved".

### Dev Features panel (game devtools, `mw_devtools_bevy`)

- An overlay, not a menu screen: the game keeps running, so the frame and F3 graph react while you click. Opened
  by a key (not F10: it opens key bindings; F1-F4, F6-F8, F12 are taken by dev tools; pick a free one in FS-UI1) or `features` with no argument; frees the cursor while
  open; in every interactive run, release included (as F3).
- Layout: category tabs (Shadows, Lighting, Sky, Water, World, Foliage, Characters, Post, Debug); rows = toggle,
  label, badges (`restart`, `hides`, `not built`, `blocked by x`), cost ms, last delta, quick A/B button; a header
  with frame/main/render/GPU ms and "reset all" (clears the Console layer of every switch).
- Built on `bevy_ui_widgets` (checkbox, tabs, scroll) styled by our CSS; framework widgets from `sf_ui_screens`
  so another game gets the same panel by registering switches. `bevy_feathers` is the fallback if our widgets slow
  the slice down (dev surface only).
- Writes the `Console` layer only: nothing it does is saved; `reset all` restores the preset look.

### Player view

Player-scope switches (`Scope::Player`) also get a row on Graphics > Advanced (menus.md §7, MF-6), writing the
`User` layer (saved, `Machine` scope), with a cost class (low/medium/high) derived from the measured cost tags.
Owner 2026-11-23: yes, players get the Advanced page (presets and groups stay the main path).

### Presets

The features with no preset group today join one (`sg.sky`: froxels, sun visibility, shafts; `sg.shadows`:
terrain far shadows, lamp shadows; `sg.foliage`: groundcover density and shadows; `sg.effects`: particles,
precipitation), values per level from the industry defaults with sources. A look change: owner check per group.

## Build plan

One deliverable + its test per slice. FS = framework switches, FS-L = live conversions (one feature each, in suspect
order, coordinated with the session owning that crate), FS-C = cost.

- **FS-1** `FeatureSwitch` + `Effect` + `Cost` + registry with availability and `requires` in `sf_tunables`
  (Bevy-free); `add_feature_switch`, `feature_on` run condition and `FeatureToggled` in `sf_tunables_bevy`.
  Test: registry states (on/off/layer/blocked/not built), exact.
- **FS-2** console `toggle` + `features` listing; register switches for every feature that already has a live
  tunable (inventory §1: sun cascades, PCSS, contact, lamp count, lamp atlas, GI, SSAO, bloom, sun visibility,
  shafts, aerial shadow, clouds, underwater, TAA/upscaler, normal/ORM/parallax) and `Restart` switches for the
  startup-only ones, so the list is complete; `Part` alias table. Test: every `Part` with a switch maps to it;
  `toggle` round trip.
- **FS-L1** groundcover live off (plugin systems gated, batches hidden, scatter + per-frame ground refresh skipped).
- **FS-L2** froxels live off alone (inject + integrate skipped; fog falls back to the analytic height fog).
- **FS-L3** terrain far shadow map live skip (today: shader bit); consumers (froxels, sun visibility) read "unlit".
- **FS-L4** lamp shadows: one switch `shadow.lamp.enabled` + carried/near-mover redraw switch.
- **FS-L5** water live off (FFT, SSPR, shore passes), **FS-L6** precipitation, **FS-L7** NIF particles,
  **FS-L8** lamp lights; then the rest of the inventory's startup-only rows, each its own slice.
- **FS-C1** cost probe: flip delta + quick A/B (pure stats half Bevy-free with exact tests on synthetic frames).
- **FS-C2** cost tags matched to GPU timers; `diag.gpu_timers` live.
- **FS-UI1** dev Features panel (rows, tabs, badges, toggles); **FS-UI2** cost and delta columns + quick A/B button.
- **FS-P** preset coverage for the groups above (look check per group).
- Docs (orchestrator): tunables index regenerated (stale since 21078c18: no `sf_atmos_bevy`/`sf_groundcover*`),
  perf `CATALOG` entries for groundcover, wiki app-and-dev-tools "Switches", ledger perf lines for each measured
  delta.

Order: FS-1, FS-2, FS-L1, FS-L2 (the top suspects), FS-UI1, FS-C1, then the rest. Menu slices (menus.md §7) run
beside them in other crates.

## As built (2026-11-23)

- **Core** `sf_tunables/src/feature.rs`: `FeatureSwitch` (`new`, `choice`, `named`; `.label .category .effect .cost
  .requires .startup_only .at`), `Category` (the ten `Group`s + `World`, `Debug`: separate enum, one `sg.*` driver per
  `Group`), `FeatureSwitches` registry (`register`, `mark_not_built`, `state` -> `SwitchState { on, layer, availability }`;
  `Blocked` transitive, read-time; `NotBuilt` wins). **Bevy** `sf_tunables_bevy/src/feature.rs`: resource, `add_feature_switch`,
  `mark_feature_not_built`, `feature_on(name)` run condition, `FeatureToggled` (in `TunableSystems::Apply`, dependents too).
- **Console** `sf_console_bevy/src/features.rs`: `toggle` (replaces sf_console's bool `toggle`, which still flips plain
  bools), `features [filter]`, `set_feature` (one choke point: refuses not-built and `startup_only`); game `on/off/flip`
  fall back to switches and part names (`mw_devtools_bevy/src/console.rs`).
- **Registered**: 14 next to their tunables (FS-2b: froxels, sun visibility, shafts, aerial shadow, groundcover, lamp
  atlas, soft sun, contact, SSAO, upscaling, underwater, bloom, shelter, puddles = `Hides`); 22 live A/B switches as
  interim game-side tunables (`mw_devtools_bevy/src/live_switches.rs`, synced both ways with `Switch`; removal path: move
  each into its crate, as far shadows did); GI via `render.gi`. **Parts** (`part_switches.rs`): 81 parts, 24 mapped to a
  switch (`--off` writes it at `CommandLine`, or marks it not built when the feature is absent), 57 `part.*` Dev rows
  (`Restart`, `startup_only`: "relaunch with --off x"). `--off` stays a true startup removal for benches.
- **Live skips**: groundcover (`groundcover.enabled`: batches despawned, jobs/residency/uploads dropped, drive and ground
  refresh gated), froxels (`sky.froxels.enabled`: inject + integrate skipped, textures freed, volume stand-in), terrain far
  map (`shadow.terrain_far.enabled` now in `sf_terrain_bevy`: no bake or upload, params zero = consumers unshadowed).
- **Panel** `mw_devtools_bevy/src/features_panel*.rs`: Ctrl+F3 / `flip featurespanel`; overlay, opens with
  `PlayerMode::paused` (controls stop, cursor free, clock runs); category tabs, badges, Console layer only, Reset all;
  delta column + live ms header (FS-UI2); A/B button (FS-C1b). **Probe** `switch_probe*.rs`: `SwitchProbe`/`SwitchDelta`,
  samples via `perf_panel` `PerfFrameFeed` (gauges kept alive by `PerfGaugeDemand` only while the panel is open or a flip
  waits: ~0 cost closed); `QuickAb` runner (`AbRun`), tunables `diag.switch.{window_s,settle_s,noise_ms,still_m,still_deg,ab_rounds}`.
- **Player view** (MF-6): Graphics > Advanced, rows generated from player-scope switches (`advanced_page.rs`).
- Open: Hides switches to make skip (surface, night sky, terrain RO-16 set), FS-L4..L8, FS-C2 cost tags to GPU timers,
  FS-P presets, `set part.x` bypasses the refusal, `--off sky` leaves cloud/fog/night switches "available", groundcover
  and other pre-existing tunables missing from the perf CATALOG (needs a devtools dependency).

## Owner answers (2026-11-23)

1. Player view: **an Advanced page for players** (Cyberpunk/Nixxes style) with player-safe feature switches and a
   cost hint, saved per machine; presets and groups stay the main path (MF-6).
