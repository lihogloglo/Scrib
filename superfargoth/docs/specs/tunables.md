# Tunables

Status: spec (2026-11-16); T1 core and T2 Bevy side built ("Migration", T1/T2 as built). Census: `tunables-census.md` (generated, grep only).
Crates (proposed): `sf_tunables` (Bevy-free core: definitions, layered values, provenance, presets, dump),
`sf_tunables_bevy` (thin plugin: resource, change messages, bindings, CLI, BRP). `sf_console` moves its variables
onto the core; nothing else in the console changes.

<!-- agent-summary -->
> **Agent summary (2026-11-16).** Core built (T1: `sf_tunables`); Bevy side built (T2: `sf_tunables_bevy`, the
> console on its shared store); app wiring is T3. One registry
> of named, typed, ranged values with a unit, a source ("provenance") and a lifetime, set through ordered layers (default < mod < preset < user < command line < code <
> console/BRP), live through Bevy change detection on the settings resources systems already read. Console variables
> become tunables; `ui.setting_preset` levels become bundles of tunables; perf arms and sweeps set them with `--set`.
> Read next: "Design" to add a tunable; "What stays a const" before migrating; "Migration" for the slice order.
<!-- /agent-summary -->

## Goal

Tuned values live as ~1,250 hardcoded candidates (census: 1,187 consts, 789 `Default` fields, 192 shader consts),
30 `BEVYRROWIND_*` env switches, 69 `--off` parts, 9 live console variables and preset values written as Rust
`match` arms (`mw_exterior_bevy/src/exterior.rs:1619`). Nobody knows where a value came from, two crates can hold
two truths for one value (census "Two truths": lamp shadow fade is 15/25 m in the framework, 35/45 m in the game),
and every change costs a multi-minute rebuild, so values get guessed once and never tuned.

Wanted (owner 2026-11-16): every value that trades quality against cost, or sets feel, is a **tunable**: named,
typed, ranged, documented with its unit and source, changeable at run time from the console, BRP, the command line,
the perf harness and mod/profile data, and grouped into player-facing presets. Plus a hygiene rule so new code
doesn't hardcode them.

Non-goals: a visual editor (the console and `sf_devui` are the front ends), auto-detecting a preset from hardware
(gap, "Industry comparison"), saving gameplay state (tunables are configuration, never state).

## Reuse check (CONTRIBUTING rule 0/2)

**1. Bevy 0.20.**

- **`bevy_settings`** (crate `bevy-settings` 0.20.0, `bevy_internal` feature `bevy_settings`): persists
  `Resource + Reflect + Default` types tagged `#[reflect(SettingsGroup, Default)]` to TOML in the OS preferences
  folder, atomic writes, deferred save (`bevy-settings-0.20.0/src/lib.rs:131` `SettingsPlugin`, `:195`
  `SettingsGroup`). It is a *user-preferences file*, not a registry: no per-value range, unit, help, provenance or
  priority layers, no change source, no console. It needs `Reflect` on the resource, and our settings types live
  in Bevy-free cores (`RingConfig` in `sf_world`, `FarLevels` in `sf_lod`), which can't derive it without taking
  `bevy_reflect` (rule: cores build cold without Bevy). We already persist player settings through
  `sf_ui::SettingsStore` (`load_machine`/`save_machine`, profile tables, `sf_ui/src/settings.rs:330-463`).
  **Not adopted**; revisit if `SettingsStore` ever moves its file layer onto it.
- **Reflection + BRP**: `world.mutate_resources` (`bevy_remote-0.20.0/src/builtin_methods.rs:87`, handler `:1229`)
  edits any *reflected, registered* resource by field path. Only 7 of our files derive `Reflect`, and BRP knows no
  range, unit or priority, so a BRP write would bypass validation and be overwritten by the next preset apply. We
  keep BRP as a transport: custom methods on `RemotePlugin::with_method_main` (`bevy_remote-0.20.0/src/lib.rs:591`),
  which `mw_devtools_bevy/src/remote.rs:48` already uses.
- **`bevy_dev_tools`**: FPS/diagnostics overlays, frame graph, states, picking debug, CI testing, `inspection`
  (`bevy_dev_tools-0.20.0/src/lib.rs:14-37`). No variables, no console (as `console.md` found 2026-10-22).
- **Shader values**: `ShaderDefVal` (`bevy_shader-0.20.0/src/shader_cache.rs:115`) for specialisation, uniforms for
  live values: both used below ("Shaders").

**2. Crates** (checked 2026-11-16).

| Crate | Version / Bevy | Licence | Verdict |
|---|---|---|---|
| `bevy-convars` | 0.2.1, tracks Bevy 0.16 (README table); no 0.20 release found | MIT OR Apache-2.0 | closest shape (cvars as resources, TOML loading, [docs.rs](https://docs.rs/crate/bevy-convars/0.2.1)); stuck three Bevy versions back, full `bevy` dependency, no provenance/layers/presets. No |
| `cvars` + `cvars-macros` | 0.4.2, engine-agnostic | **AGPL-3.0-or-later** ([lib.rs](https://lib.rs/crates/cvars)) | banned by licence (CONTRIBUTING "Licence") |
| `bevy_console` | 0.19 for Bevy 0.20 (console.md) | MIT | a UI over clap commands; no typed variables. No (console.md) |
| `bevy-inspector-egui` | no Bevy 0.20 release found ([lib.rs](https://lib.rs/crates/bevy-inspector-egui)) | MIT/Apache | an editor over reflected data, not a registry; a later front end for `sf_devui` |
| `inline_tweak` | 1.2.5 | CC0-1.0 ([lib.rs](https://lib.rs/crates/inline_tweak)) | re-reads literals from source in debug builds: fast iteration, but no names, ranges, presets or release path. Not a registry |

**3. Our own code.** `sf_console` (Bevy-free, `thiserror` only) already has typed variables with ranges, help,
change tracking and a "set by" priority (`sf_console/src/vars.rs:11-349`, `SetBy` Default < Config < CommandLine <
Code < Console), and `sf_ui` has data-driven setting records and presets (`sf_ui/src/records.rs:232` `SettingDef`,
`:357` `SettingPresetDef`; `sf_ui/src/presets.rs` level apply and "Custom" derivation). The registry **extends
these** instead of adding a third system: the variable store moves from `sf_console` into `sf_tunables` and gains
metadata and layers; the console becomes one front end over it, like UE's console over `IConsoleManager`.

## Industry comparison

| Technique | Who does it (source) | Us |
|---|---|---|
| Named typed variables with help, registered by each module | UE `IConsoleVariable`/`TAutoConsoleVariable` ([UE docs: console variables in C++](https://dev.epicgames.com/documentation/en-us/unreal-engine/console-variables-cplusplus-in-unreal-engine)); Source `ConVar`; Doom 3 `idCVar` (static declarations with min/max) | **match** (`Def`, registered by plugins) |
| Priority of who set a value | UE `ECVF_SetBy*`, weak to strong: Constructor < Scalability < GameSetting < ProjectSetting < ... < DeviceProfile < ... < Commandline < Code < Console ([EConsoleVariableFlags](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Core/EConsoleVariableFlags)) | **match**, with per-layer values kept (UE keeps one value) so `reset` unwinds one layer |
| Flags: cheat, read-only, render-thread-safe, scalability | UE `ECVF_Cheat`, `ECVF_ReadOnly`, `ECVF_RenderThreadSafe`, `ECVF_Scalability` (same page); Source `FCVAR_CHEAT`/`FCVAR_ARCHIVE`/`FCVAR_REPLICATED` | **match**: `Scope` (dev/player), `Net` (presentation/gameplay), `Lifetime`; render-thread safety is Bevy's extract, not ours |
| Latched values (apply on restart / map load) | Quake 3 `CVAR_LATCH` | **match**: `Lifetime::Restart` |
| Presets = bundles of variables per group | UE scalability groups `sg.ShadowQuality` etc., `BaseScalability.ini` `[ShadowQuality@2]` sections of cvars ([Scalability reference](https://dev.epicgames.com/documentation/unreal-engine/scalability-reference-for-unreal-engine)) | **match**: `sg.*` group drivers whose levels are tunable bundles, as `ui.setting_preset` records |
| Per-device defaults | UE device profiles, applied over scalability ([Scalability and device profiles](https://dev.epicgames.com/documentation/en-us/unreal-engine/scalability-and-device-profiles-in-lyra-sample-game-for-unreal-engine)); Unity Quality Settings per platform | **gap (P3)**: one PC target today; a profile is a mod-layer record set when needed |
| Auto-detect a level | UE `Scalability::BenchmarkQualityLevels` (synthetic benchmark) | **gap (P3)** |
| Budgeted per-object detail | UE Significance Manager `FOrderedBudget` ([API](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/SignificanceManager/FOrderedBudget)); Animation Budget Allocator `a.Budget.*` cvars ([docs](https://docs.unrealengine.com/4.27/AnimatingObjects/SkeletalMeshAnimation/AnimationBudgetAllocator)) | systems keep their own budget logic (`AnimBudget`, `PointShadowBudget`); their knobs become tunables, as UE exposes `a.Budget.*` |
| Quality as per-object asset config | Unity `QualitySettings` (`shadowDistance`, `lodBias`, `shadowCascades`) + `ScriptableObject` configs ([QualitySettings](https://docs.unity3d.com/ScriptReference/QualitySettings.html)) | presets match; per-asset configs are our records |
| Designer data, moddable, separate from engine cvars | Cyberpunk 2077 TweakDB: records of typed "flats", patched by mods (TweakXL YAML) ([redmodding wiki](https://wiki.redmodding.org/cyberpunk-2077-modding/for-mod-creators-theory/tweaks/tweaks)); REDengine `.ini` for engine settings | **match**: gameplay/content values are records (`sf_records`), engine tunables are this registry, and a mod overrides either through the same record pipeline |
| Player settings file | RDR2 `system.xml` per-option `kSettingLevel_*` values, hand-editable ([Rockstar tuning article](https://support.rockstargames.com/articles/4dhIZ7x25mF7gO4NEhlUO9/graphics-performance-tuning-in-red-dead-redemption-2-on-PC)) | **match** (`sf_ui` machine file) |
| Decima | no public documentation of its tunables found; its PC ports expose a preset plus per-option menu like the above | n/a |

What we take: UE's layering and scalability groups (the canonical shape for an engine with presets), id Tech's
latch, CDPR's split (designer records vs engine settings, both moddable). What we add: **provenance** per value,
which none of them stores in the variable (UE puts it in comments and `BaseScalability.ini` history), because our
problem is exactly "nobody knows where a value came from".

## Design

### Definition (core, `sf_tunables`)

A tunable is declared once, next to the code that reads it, as a `const` built with `const fn` builders (no proc
macro; a `macro_rules!` wrapper adds `file!()`/`line!()` for the dump):

```rust
pub const LAMP_FADE_START: Def<f32> = def!(float "shadow.lamp.fade_start_m" = 15.0)
    .range(0.0, 500.0).unit(Unit::Meters)
    .help("Lamp shadows at full strength within this distance of the viewer")
    .source(Source::Industry("UE r.Shadow.DistanceScale ~ local light fade"))
    .group(Group::Shadows).scope(Scope::Player).net(Net::Presentation).lifetime(Lifetime::Live);
```

Fields: **name** (lowercase dotted `<system>.<part>.<value>_<unit>`, same rules as console names); **kind**
(`Bool`, `Int`, `Float`, `Choice`, `Text`, and fixed tuples `Float2`/`Float3` for a pair like fade start/end);
**default**; **range** (required for numbers: validation clamps or refuses, as `VarSpec` does); **unit** (`m`, `s`,
`ms`, `px`, `deg`, `lux`, `count`, `ratio`, `texels`, none); **help**; **source** (provenance, below); **group**
(the preset group it belongs to, if any); **scope** (`Dev`: console/BRP/CLI/mods only, never in menus; `Player`: has
a menu row); **net** (`Presentation`: each peer its own; `Gameplay`: host-authoritative); **lifetime** (below).

**Provenance** (`Source`), one of: `BevyDefault(path)` (the Bevy field it mirrors), `Industry(source)` (engine doc,
talk, paper), `Reference(game)` (OpenMW, Morrowind data, another shipped game), `Measured(ledger line)`
(`docs/ledger/<system>.md` date: a sweep or A/B chose it), `Owner(date)` (an owner decision), `Guess`. `Guess` is
legal and honest: the dump and census count guesses per system, and a sweep turns one into `Measured`. A value a mod
or preset overrides carries its own source in the record (below), so the dump shows why the *effective* value is
what it is.

**Names and the split.** Framework tunables use system prefixes from `docs/SYSTEMS.md` (`shadow.`, `stream.`,
`lod.`, `far.`, `anim.`, `crowd.`, `sky.`, `weather.`, `surface.`, `terrain.`, `water.`, `texture.`, `render.`,
`ai.`, `physics.`, `camera.`, `move.`, `combat.`, `audio.`, `ui.`, `diag.`, `app.`). Morrowind-only values are
declared in `bevyrrowind/` under `mw.` (`mw.doors.reach_m`). A game that wants a different *default* for a
framework tunable doesn't redeclare it: its base mod ships a record (mod layer). That resolves the census "Two
truths": `PointShadowBudget` keeps the generic 15/25 m, Bevyrrowind's base mod sets 35/45 m with its source.

### Values: layers (core)

Each tunable stores one optional value per layer; the effective value is the highest layer set. Layers, weak to
strong (extends `sf_console::SetBy`, same order as UE's `SetBy`):

1. `Default` (the def), 2. `Mod` (records, load order; later mod wins, conflicts reported by `sf_mods`),
3. `Preset` (the level a preset driver wrote), 4. `User` (the options menu / machine file / profile; was `Config`),
5. `CommandLine` (`--set`, `--tunables <file>`, env aliases, perf arms), 6. `Code` (a system decides at run time,
e.g. a run without ray tracing forces `render.gi off`), 7. `Console` (typed lines and BRP).

`reset <name>` clears the strongest layer only (Console), `reset <name> all` clears all but Default. Change tracking
keeps `sf_console`'s design: per-tunable generation, a global revision, `take_changes()` in first-change order; a set
that doesn't change the effective value is not a change. Reads are by typed id (`Id<f32>`, an index): O(1), no
hashing, fine every frame.

**Gameplay tunables (net-ready rule).** `Net::Gameplay` values are host-authoritative: on a client, every layer
above `Mod` is ignored and the host's effective values arrive with the session config; a change at run time is a
gameplay command through the command path (net.md rule 2), applied on the fixed tick, never a direct write.
Single player is a host. Presentation tunables never feed gameplay (CONTRIBUTING "Net-ready").

### Lifetimes: what a change costs

| Lifetime | Applies | Examples | Console says |
|---|---|---|---|
| `Live` | next frame, through the bound resource/component | fade ranges, hysteresis, budgets in ms, bias, gains | nothing |
| `Respawn` | to things spawned after the change; existing ones keep theirs until they respawn (cell reload, `respawn <system>` command) | per-placement LOD tiers baked into spawned entities, mesh slab sizes | "applies to new spawns" |
| `Recompile` | live, but changes a shader def: pipelines re-specialise (a hitch; pipeline-warmup's catalog holds each preset's variants) | cascade count, PCF/PCSS choice, parallax steps | "recompiles pipelines" |
| `Restart` | stored, read at the next start (id Tech latch) | thread counts, pipelined rendering, texture cap, compute threads | "applies on restart" |
| `Bake` | part of the bake key; `bevyrrowind bake` picks it up | `FarLevels.handoff`, `CookSettings`, precombine rules | "needs a re-bake" |

`sf_ui::SettingApply` (`Live`, `Confirm`, `Restart`) maps onto these for menu rows; `Confirm` stays a menu
behaviour on top of `Live`/`Recompile`.

### Live in Bevy (`sf_tunables_bevy`)

- `Tunables` resource (the core store); `TunableSystems` set in `PreUpdate` after console input, so a value typed
  this frame applies to this frame's `Update` (as console.md). `TunableChanged { id, name }` messages per change.
- **Bindings keep consumers unchanged.** Systems keep reading the settings resources they read today; a binding
  copies the effective value into a field: `app.add_tunable(&LAMP_FADE_START).bind_resource(|b: &mut
  PointShadowBudget, v| b.fade_start = v)`, or into components (`bind_component::<ContactShadows, With<ViewCamera>>`).
  The binding system takes `ResMut` only when the value changed, so Bevy's change detection (`Res::is_changed`) fires
  exactly once per change and idle frames cost one revision compare. Bevy-free cores keep plain structs; no
  `Reflect` needed.
- Direct reads: `Res<Tunables>` + `tunables.get(ID)` for values with no settings struct; `tunable_changed(ID)`
  run condition (as `console_var_changed`).
- **Console**: every tunable is a console variable (one namespace). `vars` shows value, layer, unit and range;
  `help <name>` adds source and lifetime; `vars --guess <system>` lists guesses; `tunables diff` lists non-default
  values with their layer (UE `DumpCVars`-style).
- **BRP**: methods `sf.tunables/list` (filter), `sf.tunables/get`, `sf.tunables/set` (Console layer, validated),
  `sf.tunables/reset`, registered with `with_method_main`. The `bevyrrowind/set` agent tool's switches fold in.
- **Command line**: `--set name=value` (repeatable, `CommandLine` layer), `--tunables <file.toml>` (a flat
  `name = value` table), refused names fail the run (a typo must not silently measure the default). Env switches
  become aliases (census "Env switches": 21 of 30 fold in), kept one release with a deprecation line in the log.
- **User dev file** `<user data>/tunables.toml` (UE `ConsoleVariables.ini`) loads at `CommandLine` in interactive
  runs only; benchmark runs never read it unless passed with `--tunables`, so a stale local override can't skew a
  measurement (perf verdict audit, 2026-11-16).
- **Records** (everything is a mod): kind `sf.tunable`, id = the tunable name, value `(value: 35.0, source:
  "...")`, applied at the `Mod` layer through `sf_records`/`sf_mods` with their patch, conflict and hot-reload
  machinery (mods.md "Native record files", "Hot reload"). A `Dev` tunable is settable from mods too (Skyrim-style
  ini tweaks are the modding norm); `Scope` only hides it from menus.
- **Perf harness**: `--set` is a run setting like `--off` and `--env`, so `perf suite --arm "b:--set
  shadow.lamp.count=4"` and `perf sweep --var set:shadow.lamp.count --values 0,4,8` need only the parser
  (`bevyrrowind/apps/bevyrrowind/src/perf/sweep.rs` `Step`, `ab.rs` arms). Every perf report gets a "tunables"
  section listing non-default values with their layer, so a ledger line can be reproduced.

### Presets are bundles of tunables

`ui.setting_preset` records already map a driver's levels to setting keys and derive "Custom" (presets.rs). For
player tunables **the setting key is the tunable name**, and levels write the `Preset` layer, not the user store, so
a hand-changed option sits on `User` above the preset and the driver reads "custom" exactly as today.

Groups follow UE's scalability groups, one driver each with levels low/medium/high/ultra: `sg.view_distance`
(lod bias, size cull, rings, far handoff scale), `sg.shadows` (sun cascades, distance, PCSS, filter, lamp count/map
size/fade, contact), `sg.lighting` (GI, SSAO), `sg.post` (bloom, upscaler), `sg.textures` (cap, parallax),
`sg.effects` (weather/precip/particle budgets), `sg.characters` (anim LOD pixels, crowd tiers), `sg.water`,
`sg.sky` (clouds per-sample map, froxels), `sg.foliage` (later). The overall `sf.graphics_preset` driver's levels set
the group drivers. Group levels live in Bevyrrowind's base mod as records, replacing the `QualityRender` `match`
arms (census "Preset tables"); a mod adds keys to a level by defining its own record with that driver (existing
merge rule, records.rs:350). The vanilla option's Classic preset is one more level set.

### Shaders

A WESL `const` is right for maths and layout. A value someone tunes takes one of two routes: a field in the system's
existing params uniform (live, no recompile; dynamic loop bounds included) or a shader def through specialisation
(`ShaderDefVal`; lifetime `Recompile`) when a constant bound is worth the speed (unrolled loops, removed branches).
Default: uniform; a shader def only with a measured reason. The census marks 102 shader consts as tunable
candidates; they move only when their system's slice touches them, never in a sweep of their own.

### Capacities

A buffer or array bound (`MAX_CACHED_LIGHTS = 42`, `MAX_RAYS = 64`, `sf_wind::MAX_SOURCES = 32`) stays a `const`: it
sizes memory and GPU layouts. The *count used* is the tunable, with `range.max` equal to the capacity; a unit test
per system asserts `def.max <= CAPACITY`. Raising a capacity is a code change, by design.

### Self-documentation

- `bevyrrowind tunables --md` dumps every registered def (name, default, effective value under each preset level,
  unit, range, source, lifetime, net, scope, `file:line`) into `docs/tunables-index.md`, generated like
  `node tools/decisions-index.mjs`; `--json` for tools. The wiki pages' "Switches and counters" sections link their
  prefix in it.
- `tools/tunables-census.mjs` (slice T0) keeps finding what is *not* migrated. Its `--check` mode (slice T0)
  holds a per-crate baseline of T-class consts and fails when a crate's count rises: a ratchet that enforces the
  hygiene rule without a big-bang migration.

### Compile time

`sf_tunables` is Bevy-free, depends on `thiserror` and `serde` (already in every core's closure); no proc macro, no
`inventory`/`linkme` auto-registration (explicit `App` registration, as console.md chose; a test per crate checks
its `TUNABLES: &[&dyn AnyDef]` slice is all registered). Defs are `const` data: no startup cost beyond a `Vec` push.
`sf_tunables_bevy` needs `bevy_app`, `bevy_ecs`, optional `bevy_remote` behind the existing `remote` feature. It
goes in hakari's `final-excludes` list for the core (CONTRIBUTING "Adding a crate").

## What stays a `const`

Stays a const (the census marks these C): **physical and maths constants** (π, g = 9.81 when it means Earth's
gravity, unit conversions such as Morrowind units to metres); **file-format, layout and protocol values** (versions,
magic numbers, header and record sizes, strides, alignments, bit masks, GPU binding indices, enum and flag codes,
record ids); **capacities** that size memory (above); **sentinels and derived values** (`u32::MAX` markers, a const
computed from other consts); **test fixtures and synthetic test scenes**; **values fixed by an external contract**
(a Bevy API requirement, a vendor SDK constant).

Becomes a tunable (T): anything that **trades quality against cost** (distances, ranges, radii, fade ranges,
resolutions, sample and step counts, LOD bias and pixel thresholds, cascade layouts), **budgets** (ms, counts per
frame, in-flight limits, max lights/shadows/actors), **timings** (durations, intervals, half-lives, cooldowns,
settle frames), **thresholds and hysteresis**, **feel** (speeds, accelerations, damping, camera distances, impulses)
and **look gains** (intensities, biases, tints a designer tunes). Test: *would a sweep, a preset or a mod ever want
another value?* If yes, tunable. If a value is only ever changed together with code, const. Unsure: tunable with
`Scope::Dev`; it costs one line.

Inline literals in expressions (`masked_area > 0.5`) follow the same rule: name them first, then classify.

The census classifies by name, so it misreads some true constants as T or ?. Mark such a const (or `Default` field)
with `// tunables: const (<reason>)` on its line or the line above: the census then counts it C with that reason, and
`--check` stops counting it. The reason is required and says which rule above it falls under (format, capacity,
external contract ...); a value that a sweep, preset or mod could want changed is never opted out.

## Migration

One deliverable and its test per slice, in value order (perf- and look-critical first). Each system slice: declares
the defs with honest provenance (from the doc comment, the ledger, or `Guess`), binds them to the existing settings
resources, folds the system's env switches and `--off` parts in as aliases, removes the "two truths", lowers its
census baseline, and updates its wiki page's "Switches and counters". Test pattern: a golden test that every def's
default equals the value it replaces (no behaviour change), plus one live set per binding through `Tunables` that
reaches the consumer resource in a `MinimalPlugins` app.

| Slice | Deliverable | Test |
|---|---|---|
| **T0** | `tools/tunables-census.mjs` committed, census regenerated, `--check` baseline per crate | script runs; `--check` passes on main, fails on a planted const |
| **T1** | `sf_tunables` core: `Def`, kinds, units, `Source`, layers, change tracking, validation, dump; `sf_console` moves onto it (re-exports, `SetBy` gains `Mod`, `Preset`, `User`) | core unit tests; all existing `sf_console` tests pass unchanged |
| **T2** | `sf_tunables_bevy`: resource, messages, bindings, run condition, `--set`/file parsing helpers, BRP methods | MinimalPlugins: set per layer reaches a bound resource once; BRP set validated |
| **T3** | App wiring: `--set`, `--tunables`, user dev file (interactive only), env aliases, `tunables --md` dump, perf report section; perf `--arm`/`sweep set:` (after `bench-lock` lands, its files) | CLI parse tests; a bench run ignores the dev file; sweep step for `set:` |
| **T4** | Shadows and lights: `PointShadowBudget`, sun cascades, PCSS, filter, contact, lamp fade (two truths), `sf_shadow_cache_bevy` settings (or delete it with the dead cache); env `POINT_SHADOWS`, `SUN_CASCADES`, `SHADOW_FILTER`, `LAMP_CONTACT`, `SSAO` | golden defaults; live fade/count change applies |
| **T5** | Presets as data: `sf.tunable` record kind, `sg.*` group drivers and levels in the base mod replacing `QualityRender` and `Quality` arms | golden: each level yields the old `match` values; "custom" after a hand change |
| **T6** | Streaming and statics: `RingConfig`, `Budgets`, `SchedulerConfig`, `StreamingConfig`, fade/reveal, `LodScreen`, `FarLevels` (`Bake`), precombine rules | golden; ring change re-plans residency |
| **T7** | Characters: `AnimLodSettings`, `AnimBudget` costs, crowd tiers, walker AI wake/sleep (`Gameplay`), physics budgets; env `AI_RADIUS`, `STRENGTH` | golden; gameplay tunable refuses Console layer on a client stub |
| **T8a-c** | Sky/atmos/clouds; weather/precip/wind; surface (604 candidates, three slices) | golden per slice |
| **T9** | Terrain, water, textures (`texture.max_size` = `Restart`) | golden |
| **T10** | Feel: movement, camera, combat, physics (`Gameplay`, fixed tick) | golden; fixed-tick read |
| **T11** | The rest (audio, UI, AI/nav, effects, app/diag) and every `--off` part as a bool tunable | golden; `--off x` = `--set <x> off` |

Shader consts move inside their system's slice (uniform first). Env switches that stay env (paths, ports, tooling:
census "Env switches") are listed in `docs/wiki/engine/app-and-dev-tools.md` as process wiring, not tunables.

**T1 as built (2026-11-16).** `superfargoth/crates/sf_tunables` (Bevy-free; `thiserror`, `serde`; hakari
final-excludes). `Def<T>` is `const` data with `const fn` builders; `def!(float "name" = v)` adds `file!()`/`line!()`
(kind words `bool int count float float2 float3 text choice`; `Type` covers `bool`, `i32 i64 u32 usize`, `f32 f64`,
`[f32; 2|3]`, `&'static str`). A crate lists `TUNABLES: &[&dyn AnyDef]` for `Tunables::register_all`; a number
without a range is refused. `Id<T>` reads and writes typed (`read`, `write`); `id_of(&DEF)` checks the kind. Each
tunable holds one `Layer` (value + optional note: who and why) per `SetBy`; a set under the top layer is stored and
reports `Outranked`; `clear(id, layer)` peels one, `clear_layer(layer)` peels it everywhere, `reset(id)` clears all
but `Default`; setting `Default` is an error. `ArgKind` gained `Float2`/`Float3` (text `a,b`). `rows()` (serde) and
`dump_markdown()` sort by name; `guesses()` counts `Source::Guess`. `f32` values go through their shortest decimal
(`0.1` reads `0.1`). `sf_console` re-exports the store as `Vars`/`Var`/`VarId`/`VarSpec` (and `sf_tunables`);
`SetBy::Config` is an alias of `User`, which still prints "config", so console output is unchanged. Left for later:
the console's `reset <name>` still clears every layer (old behaviour, pinned by its tests; Console-only peel is T2/T3's
call), the `Net::Gameplay` client refusal (T7), per-preset-level dump columns (T5), JSON output in the app (T3).

**T2 as built (2026-11-16).** `superfargoth/crates/sf_tunables_bevy` (`bevy_app`, `bevy_ecs`, `bevy_log`, `serde_json`,
`toml`; optional `bevy_remote` behind `remote`). `Tunables` resource (newtype over the core store, `Deref`);
`TunablesPlugin` adds it with `SetTunable { name, value: text, layer, note }`, `PeelTunable { name, layer: Option }`
(`None`: every layer but `Default`), `TunableChanged { id, name }`, `TunableRefused { name, reason }` (also a
warning), in `PreUpdate` sets `TunableSystems::Apply` (requests, then `take_changes` as messages) then `::Bind`.
`TunablesAppExt`: `register_tunables(TUNABLES)` and `add_tunable(&DEF)` are get-or-register (the same name with
another kind or default panics); the builder has `.bind_resource(|r: &mut R, v| ..)`, `.bind_component::<C, F>(..)`
(all matches on a change, each newly added one) and `.id()`. A binding writes the first frame its target exists,
then only on a change (generation compare), so `R.is_changed()` fires once per change. `tunable_changed(id)` run
condition (`Id<T>` or `TunableId`; the core gained `From<Id<T>> for TunableId`). `overrides`: `parse_set`
(`name=value`), `parse_file` (TOML: bare dotted keys, quoted keys and `[sections]` flatten to dotted names; bools
on/off, arrays `a,b`), `apply_overrides(store, list, layer, note) -> Vec<Refusal>` (unknown names refused). `brp`:
pure JSON handlers `list` (`filter`, `changed`), `get`, `set` (Console layer, kind and range validated, outcome
`changed|unchanged|outranked`), `reset` (Console layer; `all`: every layer) over the dump `Row`; `remote` registers
them as `sf.tunables/*` into `RemoteMethods` in `Plugin::finish` (after the app's `RemotePlugin` built it), and
`mw_devtools_bevy/remote` turns it on (dev builds). **One store:** console variables live in `Tunables`;
`sf_console` resolves variable names through its store (no copy in its name map); `DevConsole::with_tunables`
swaps the shared store in while lines run, completion runs and things register; `ConsoleVarChanged` is forwarded
from `TunableChanged`; `console_flag` / `console_var_changed` read `Res<Tunables>`; `ConsolePlugin` adds
`TunablesPlugin` if missing. `reset <name>` peels the Console layer, `reset <name> all` clears every layer, a bare
`reset` peels the Console layer everywhere. `mw_devtools_bevy` console systems read `Tunables` (compile fix). Left
for T3: the app parsing `--set` / `--tunables` / the user dev file and failing the run on refusals; `vars` columns
(layer, unit, range), `help` source and lifetime, `tunables diff`; a headless test of the BRP glue (a `remote` test
build is its own Bevy: bevy_utils `debug`), today covered by the pure handlers' tests and the `dev` check build.

**T3 as built (2026-11-16).** `Def::env_alias("BEVYRROWIND_X")` (core `Meta.env_alias`, dump column `env`):
`register_tunables`/`add_tunable` read it for a freshly registered def (`TunableEnv`: process env, tests swap a map;
empty = unset), set `CommandLine` noted `env X (deprecated: --set name=value)` with one info line; a bad value goes to
`OverrideRefusals`. `CommandLineOverrides(Vec<OverrideSet>)` applies sets in order at `CommandLine` in `Plugin::finish`
(after every `build`), refusals to the same resource. Game side `mw_devtools_bevy/src/perf/tunables.rs`: `PerfFlags`
`--set`/`--tunables` (every `view`/`scene`/bench run), order user file < `--tunables` files < `--set`; the user dev file
`<user data>/tunables.toml` only in interactive runs (`main.rs` `is_interactive`: no `--bench`, no `--screenshot`;
`run_overrides` drops it for benches too); `FailOnRefusals` logs each refusal and exits 2 before the first frame;
`CATALOG` lists every crate's `TUNABLES` for `bevyrrowind tunables --md|--json` (no app; defaults), and a run warns about
a `def!` tunable missing from it. Bench reports carry meta `tunables` (non-default values, layer, note); the suite adds a
"Tunables" section per run; `RunSettings --set` (suite, `--arm`), sweep `--var set:<name>`. Console: `vars` adds unit,
kind/range; `help <var>` adds source, lifetime, env alias, layers; `tunables diff`. Left: `VarSpec` console vars are not
indexed; a sweep can't step pair values.

**T4 as built (2026-11-16).** `sf_sky_bevy::quality::ShadowQualityPlugin` (opt-in) binds every shadow and light
tunable: `shadow.lamp.{count, map_size_px, fade_m, fade_time_s, hysteresis, contact, day_gate(.receiver_m, .contrast),
face_cull}` into `PointShadowBudget` / `PointShadowFaceCull`; `shadow.sun.gate(.contrast, .hysteresis, .fade_s)` into
`SunShadowGating`; `shadow.sun.{cascades, distance_m, split_lambda, overlap, soft, soft_size_texels}` into
`SunShadowQuality` (inserted by the host) → `SkyRenderOptions`, which the sun/moon now follow live (`sync_sky_lights`);
`shadow.contact(.steps, .thickness_m, .length_m)`, `render.ssao.quality` and `shadow.filter` (auto = temporal under
TAA/DLSS) inserted/removed on cameras with `ViewQualityCamera` (off keeps the prepasses). Lifetimes: contact on/off,
GTAO and the filter `Recompile`, the rest `Live` (a map size or cascade count change reallocates). Game side:
`mw_environment_bevy::shadow_tunables` (alias copies registered first, lamp fade 35/45 m at `Mod`, base `Preset`,
`--point-shadows`/`--sun-cascades` sugar at `CommandLine`); `mw_exterior_bevy::quality_presets` (interim `Preset` table
replacing the `QualityRender` shadow arms, golden-tested; `--off contactshadows|ssao|softshadows` at `CommandLine`).
Left: the sun bias consts, `sf_shadow_cache_bevy` (CW-4), devtools' duplicate `render.ssao`/`render.contactshadows` vars.
**CW gates live (T4 part 1):** `shadow.lamp.day_gate`, `shadow.lamp.face_cull`, `shadow.sun.gate`, `select.gate`
(`--off selectgate` at `CommandLine`), `render.skin.skip_unchanged`, `mw.matctrl.gate`; old env names are aliases.


**T5a as built (2026-11-16).** Every value `Quality` decided is a tunable in `quality_presets::quality_preset_values` (golden
test per level). Live: `render.upscaling` (`sf_sky_bevy::quality`, into `UpscalingSetting`; `--dlss` at `CommandLine`, alias
`BEVYRROWIND_DLSS`, F4 writes it at `Console`), `render.bloom(.intensity)`, `water.quality`, `sky.clouds.map_per_sample`,
`surface.puddles|shelter` (game-declared in `mw_environment_bevy::quality_tunables` until their crates take `sf_tunables`),
`texture.parallax.layers|relief_steps` (`mw_assets::map_ab`, `Recompile`: retunes loaded materials; 0 stashes depth maps).
Prepasses and MSAA follow their consumers (`prepasses_follow_consumers`, after `SkySystems::RenderPath`, raster only): depth +
motion vectors for contact shadows, GTAO or TAA/DLSS, normals for GTAO, MSAA off under GTAO or a temporal resolve, else the
view's `ViewPrepassBase`. `Respawn`: `lod.bias`, `lod.size_cull.min_px|min_distance_m` (`Streaming::set_lod_screen`; ranges
are per spawned entity, no non-discarding restream exists). `Restart`, read before `finish` with their `--set`s
(`start_values`): `far.handoff_scale` (near rings sized from it), `texture.max_size_px` (`--texture-cap`), `render.gi.probes`.
Console `render.ssao|contactshadows|contactlength|contactthickness|bloom` deleted; `on|off|flip ssao|contactshadows` set the
tunables. Left: a `respawn statics` restream, the texture cap live (reload), T5b records and `sg.*` drivers.
## Proposed CONTRIBUTING rule (for the orchestrator to apply)

> **No hardcoded tunables (owner 2026-11-16).** A value that trades quality against cost, sets feel, or is a
> distance, budget, count, timing, threshold or hysteresis is a tunable (`superfargoth/docs/specs/tunables.md`):
> declared with its unit, range and source (`Guess` is allowed and counted), never a bare `const` or literal.
> Maths, units, formats, layouts, capacities and fixtures stay `const`. `node tools/tunables-census.mjs --check` must
> pass (a crate's count of candidate consts may only go down).

## Open questions (owner)

1. **Player-facing depth.** Beyond the overall preset and the `sg.*` group drivers, which tunables get their own
   options row (UE-style "advanced" page: e.g. view distance slider, lamp shadow count, crowd density)? Default
   until answered: groups only, everything else `Dev`.
2. **Dev tunables in release builds.** Mods and `--set` can always set them (everything is a mod); should the
   in-game console of a release build also set them, or only `Player` ones? Default until answered: the console
   sets everything in builds with the `dev` feature and only `Player` tunables otherwise.

Decided here from the industry standard (no question): layer order (UE `SetBy`), group structure (UE scalability),
gameplay values host-authoritative (net.md), benchmark runs ignore user overrides, device profiles and auto-detect
deferred (P3).
