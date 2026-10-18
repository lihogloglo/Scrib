# superfargoth & Bevyrrowind: Vision and Architecture

Status (2026-10-11): M0 done and verified on a real install (Morrowind +
Tribunal + Bloodmoon, ~62k records in ~0.3 s). M1 underway: static
interiors render (Census and Excise Office verified by screenshot); still
missing for M1: animated/particle NIF content, benchmark harness. Engine baseline: **Bevy 0.20** (released
2026-10-08), Rust 2024 edition.

**superfargoth** is the framework (this repo). **Bevyrrowind** is the
Morrowind content pack and game built on it.

---

## 1. Vision

An open-world action-RPG framework on Bevy, with Morrowind as its first and
flagship content pack. Not a faithful port: "Morrowind if Bethesda made it in
2026". Morrowind's 2002 feature set is a floor, not a ceiling.

Required from day one of the design (not bolted on later):

- Mods as a first-class citizen: asset overrides, record overrides/patches,
  Lua scripting, and legacy Morrowind ESP/ESM/BSA content.
- Raytraced lighting (Solari) with a raster fallback.
- High-definition grass and foliage.
- A proper ocean (FFT waves, shoreline, underwater, buoyancy).
- Rigid-body physics for the world (loose items, ragdolls, buoyancy).
- Terrain deformation (transient trails and persistent edits).
- A modern animation system (layered blending, IK, foot placement).

This continues Godotwind (`D:\Gamedev\Godotwind\godotwind`, Godot 4.6, MIT) and
keeps its philosophy. What changes is the engine and how strictly the
framework/Morrowind boundary is enforced.

## 2. Principles

1. **Framework first; Morrowind is an adapter.** Litmus test (inherited from
   Godotwind): could someone plug in another game's data with only a new
   adapter, without touching framework code?
2. **The boundary is a compile-time fact.** Godotwind enforced it with
   ratchet tests and still carried "known debt" bridges. Here the framework
   lives in its own Cargo workspace and repo and cannot name a Morrowind type.
3. **Everything is a mod.** Morrowind itself loads through the same mod
   pipeline as a user's texture pack. No privileged content path.
4. **Industry standard, never kludge** (Godotwind rule). Name the canonical
   pattern before building: Tessendorf FFT for the ocean, CDLOD/clipmaps for
   terrain, fixed timestep plus interpolation for physics, and so on.
5. **Durable vertical slices.** Small steps are fine if they are real slices
   of the final system. Temporary scaffolds are named, isolated, and come with
   a removal path.
6. **Measure, then optimize.** The benchmark/profiling harness exists before
   the streaming work, not after (Godotwind learned this the hard way).
7. **Verify visually.** A rendering change is not done until it has been seen
   running in the relevant scene or tool.

## 3. Licensing and repository split

Two repositories, one-way dependency:

```
┌──────────────────────────────────────┐
│ superfargoth repo                    │  MIT OR Apache-2.0 (Bevy's license)
│ generic ARPG engine on Bevy          │  no Morrowind types, no GPL code
└──────────────────▲───────────────────┘
                   │ depends on (never the reverse)
┌──────────────────┴───────────────────┐
│ bevyrrowind repo                     │  GPL-3.0-or-later
│ Morrowind adapter + rules + game exe │  may borrow OpenMW code freely
└──────────────────────────────────────┘
```

- **Framework repo, MIT/Apache.** Contributors never paste or translate
  code from OpenMW, MWSE, or `nif.xml` (all GPL). Behavior specs cite UESP,
  papers, or in-game observation. Godotwind code may be reused only for
  *generic* systems with clean provenance. Godotwind pieces derived from
  OpenMW sources (for example the dialogue "4-step filter" and the
  groundcover design notes that cite OpenMW file:line) stay out.
- **Bevyrrowind repo, GPL-3.0-or-later**, so OpenMW code can be reused
  without provenance audits. Making it GPL from day one is deliberate: going
  from MIT to GPL later is trivial, but GPL code can never go back to MIT.
- **The shipped game binary is GPL as a whole.** The framework stays MIT for
  everyone else. MIT and Apache-2.0 dependencies are GPLv3-compatible.
- Using OpenMW only as a black-box oracle (run both, compare outcomes)
  involves no code and is fine anywhere.
- Morrowind's own data is never redistributed; users point the game at their
  install.

## 4. Technology baseline (checked 2026-10-10)

| Need | Choice | Bevy version today | Note |
|---|---|---|---|
| Engine | `bevy` 0.20 | — | Solari: atmosphere, env maps, directional + emissive lights. Point/spot lights planned. |
| TES3 formats | `Greatness7/tes3` (git, MIT) | n/a | Reads and writes ESP/ESM/NIF/KF/BSA; active (Sep 2026). Pin a commit. |
| Physics | `rapier3d` 0.36 (direct, Bevy-free `sf_physics`) | n/a | Decided 2026-10-12 (§10, specs/physics.md): same parry3d as our probes, per-axis joint motors, own floating-origin rebase. `avian3d` 0.8 is the fallback. |
| Lua | `mlua` 0.12, **Luau** | n/a | Sandboxed by design, 5.1-compatible, optional native codegen, type checker for mod authors. |
| Char. controller | own (`sf_controller`) | — | Kinematic mover kept for M4 (§10 2026-10-12); `bevy_tnua` not adopted. |
| VFX | `bevy_hanabi` 0.19 | 0.19 | GPU particles: rain, ash, magic. |
| Clouds, fog, weather | own (`sf_atmos_bevy` et al., spec `weather.md`) | — | No built-in clouds or height fog in Bevy; we use its `Atmosphere` (aerial perspective) and `DirectionalLightTexture` (cloud shadows). Nubis math ported from the owner's Megalophobia (license gate in weather.md). |
| IK | own | — | Existing crates are stale (Bevy 0.8–0.14). |
| Grass, ocean, terrain | own | — | No maintained crates for 0.20. These are core to the vision anyway. |

**Ecosystem lag tracker** (re-checked 2026-10-18 on crates.io + git main; first checked 2026-10-11, three days after Bevy 0.20
shipped on 2026-10-08; re-check with the crates.io API before each
milestone). For 0.19, the main crates ported within about two weeks
(avian, tnua, hanabi, input managers: June 19–27, 2026).

| Crate | Need (milestone) | On 0.20? | Latest / Bevy |
|---|---|---|---|
| `big_space` | coordinates (M2) | rc | 0.14.0-rc.0 / 0.20-rc |
| `bevy_3d_tiles` | far-field evaluation (M2) | no | 0.5.1 / 0.19 |
| `avian3d` | physics fallback (M4; rapier3d chosen, §10 2026-10-12) | no | 0.7 / 0.19; main 0.8.0-dev still on 0.19, PR #1070 (0.20 port) still open (2026-10-18) |
| `bevy_tnua` | not adopted (own kinematic mover, §10 2026-10-12) | yes | 0.33.0 / 0.20 (2026-10-15) |
| `bevy_hanabi` | GPU VFX: splashes, magic (M5–M6); rain/snow/ash don't need it (weather.md) | git only | 0.19 / 0.19; main migrated to 0.20 + WESL shaders 2026-10-15 (#576), no release yet |
| `bevy-aqua` | water reference (M6) | no | 0.1.3 / 0.19 |
| `bevy_fast_clouds`, `bevy_weather` | clouds/weather **references only** (M6, weather.md) | no | 0.19.0 / 0.19, 0.2.0 / 0.19.1 |
| `bevy-volumetric-clouds` | rejected (skybox clouds, ignores depth) | no | 0.2.0 / 0.18 |
| `bevy_feronia` | grass/wind reference (M6) | no | 0.8.4 / 0.18 |
| `bevy_enoki` | rejected (2D particles only) | yes | 0.8.0 / 0.20 |
| `bevy_landmass` | navmesh candidate (M10) | no | 0.12.1 / 0.19 |
| `bevy_enhanced_input` | input candidate (UI actions, M5) | yes | 0.27.0 / 0.20 (2026-10-14); `leafwing-input-manager` 0.22.0 / 0.20 (2026-10-16) |
| `bevy_flair` | UI styling, modder CSS (M5) | yes | 0.9.0 / 0.20 (2026-10-14): no git pin or fork needed; the mod contract is still our documented CSS subset (ui.md R2b) |
| `bevy_ui_anchor` | world-anchored UI candidate (M5) | no | 0.12 / 0.19 |
| `bevy_fluent` | localization candidate (M5; or `fluent-bundle` directly) | no | 0.15 / 0.19 |
| `bevy-inspector-egui` | dev inspector (complements BRP, CLAUDE.md "Live inspection") | yes | 0.38.0 / 0.20 (2026-10-18) |
| `bevy_egui` | dev UI | yes | 0.43.1 |
| `bevy_kira_audio` | audio candidate | yes | 0.27 |
| `bevy_asset_loader` | loading states | yes | 0.28 |
| `bevy_mod_scripting` | rejected for Lua (§10 2026-10-13; Lua parked 2026-10-18) | yes | 0.23.0 / 0.20 (2026-10-15) |
| `bevy_rapier3d` | not used (we call `rapier3d` directly, Bevy-free) | yes | 0.37.0 / 0.20 (2026-10-10) |
| `bevy_brp_extras` + `bevy_brp_mcp` | agent live inspection (dev only) | yes | 0.23.0 / 0.20 |
| `mlua` (Luau) | Lua mods | n/a | Bevy-independent |

**Lag policy:** we track the latest Bevy. Crates that lag go behind
`[patch]` forks until upstream catches up. The first milestones (VFS,
records, NIF loading) need only Bevy itself.

**Bevy gives us what Godot didn't** (see Godotwind's
`docs/reference/godot_wishlist.md`): GPU-driven rendering, automatic bindless
materials, a render graph made of ECS systems, mesh shaders, Solari, an ECS
that handles hundreds of thousands of entities, and a real asset processor.
**Still missing in Bevy:** texture streaming and virtual texturing,
world-partition streaming, terrain, and clouds. We build those, so expect to
be an engine team early, as with Godot, but on better primitives.

## 5. Framework architecture

Framework crates use the `sf_` prefix; Bevyrrowind crates use `mw_`.
Built so far (M0): `sf_vfs`, `sf_records`, `sf_mods`.

### 5.1 Foundation

| Crate | Responsibility |
|---|---|
| `sf_vfs` | Layered virtual filesystem. Sources (loose dirs, zip, adapter-provided archives such as BSA) are registered in priority order; last layer wins. Paths normalized once. Read-only and lock-free after the index is built. Feeds Bevy's `AssetSource`. |
| `sf_records` | Typed record database. Records are Rust types (`Reflect` + serde) grouped by kind. Layered overlays with provenance (which mod set which field), field-level patches, and a conflict report. A runtime-change overlay feeds saves. |
| `sf_mods` | Mod manifest (id, version, `api_version`, dependencies, load-after hints, content roots), load-order resolution, profiles, enable/disable. |
| `sf_script` | Luau runtime: one isolated environment per mod, curated versioned API, event bus, timers, hot reload, serializable script state for saves. |
| `sf_save` | Save format: record overlay + entity state + script state; versioned with migrations. |
| `sf_diag` | Benchmark scenes, frame/stream/VRAM counters, automated fly-throughs, CSV/JSON reports. |

### 5.2 World

| Crate | Responsibility |
|---|---|
| `sf_world` | World partition: worldspaces (exterior grid plus interiors), a streaming scheduler with per-frame budgets and priority (frustum, velocity prediction), activation and eviction, interior/exterior transitions. Content arrives through a `WorldSource` trait implemented by adapters. |
| `sf_terrain` | GPU heightfield terrain: CDLOD/clipmap meshing, splat material on texture arrays (bindless), triplanar cliffs, macro variation, a deformation stack (§7.4), physics heightfield colliders per tile. |
| `sf_foliage` | HD grass (§7.1), scattered small props, trees with impostors. |
| `sf_water` | FFT ocean, shoreline, lakes, rivers, underwater, buoyancy queries (§7.2). |
| `sf_sky` | Bevy `Atmosphere`, volumetric clouds, weather state machine with blended presets, wetness, wind field shared by grass, cloth, particles, and clouds. |
| `sf_lod` | Offline HLOD merging, octahedral impostors, distant lights (proven Godotwind tiers, rebuilt on GPU-driven rendering). |

### 5.3 Actors and gameplay

| Crate | Responsibility |
|---|---|
| `sf_anim` | Bevy animation graphs with layers, masks, and additive blending. Solvers for two-bone IK, look-at, foot placement and pelvis adjustment, hand IK for interactions. Physical hit reactions; ragdoll handoff to physics. Animation events (Morrowind text keys map onto these). |
| `sf_character` | Character assembly from parts (body, head, hair, equipment slots), skeleton retargeting, first and third person. |
| `sf_controller` | Player and NPC locomotion on physics: kinematic controller, swimming, climbing hooks. |
| `sf_interaction` | Activation, grab/carry/throw (Oblivion-style), doors, containers. |
| `sf_rpg` | *Generic* RPG primitives: attributes, effects, inventory, dialogue graph, quests, journal, factions. The **rules** (formulas, leveling, combat math) are supplied by the content pack as a rules plugin, never hardcoded. |
| `sf_ai` | Navmesh generation per streamed tile, behavior layer (schedules and packages), perception. |
| `sf_ui` | Game UI (spec `ui.md`): bevy_ui + headless `bevy_ui_widgets` with our styled widget kit, CommonUI-style layers and input routing, MVVM view-models, two themes and two layout presets, CSS styling via `bevy_flair` layered by mod load order, extension slots, settings registry; HUD, menus, dialogue, journal, all scriptable. Feathers for dev tools only. Crates: `sf_ui` (Bevy-free), `sf_ui_bevy`, `sf_ui_css`, `sf_ui_screens`, `sf_ui_lua`, `sf_devui`. |

### 5.4 Tools

`sf_viewer` (asset and record browser), `sf_bake` (CLI around the asset
processor: HLOD, impostors, navmesh, terrain tiles), and benchmark scenes.

## 6. Mod system

### 6.1 Kinds of mod content (freely combined in one mod)

1. **Asset overrides** through the VFS. Modern formats always win their
   slot: if a higher layer has `meshes/x/foo.glb`, it replaces
   `meshes/x/foo.nif`, and the same goes for `.ktx2`/`.png` over `.dds`. HD
   replacers need no conversion tools.
2. **Record overrides and patches.** The native format is RON or TOML with
   explicit patch semantics (`set`, `merge`, `remove`, list `append`).
   Field-level patches avoid the classic ESP problem where two mods each
   override a whole record and the last one silently wins.
3. **Lua scripts** (Luau). Event handlers, new mechanics, UI.
4. **Legacy Morrowind content.** ESP/ESM, BSA, and loose files work out of
   the box through the Morrowind adapter. Each ESP record is diffed against
   its master, so only changed fields become patches. That gives automatic
   merging (what TES3Merge does today) for free.

No native code mods (dylibs): Rust has no stable ABI, and native mods break
sandboxing. Possibly WASM later, if Lua proves too slow for some mods.

### 6.2 Lua API rules

- **Curated and versioned**, not raw ECS reflection. Mods declare
  `api_version`; the API is semver'd so engine refactors don't break mods.
  A raw `Reflect` escape hatch exists only behind a dev flag.
- **Capability-sandboxed.** No `io`, `os`, network, or native loading. Each
  mod gets its own environment. Mods talk to each other through events and
  explicitly exported interfaces.
- **Generational entity handles**, safe across despawns and cell unloads.
- **Deterministic scheduling**: script callbacks run in defined ECS schedule
  slots with frame budgets, and script state is serialized into saves.
- **Hot reload** in dev builds.

### 6.3 Compatibility layers (bevyrrowind repo only)

- **MWScript → Luau transpiler.** Original scripts are compiled at bake time
  into Lua that calls the framework API plus an `mw` compatibility library.
  One VM runs everything, and translated scripts are debuggable as Lua.
- **OpenMW-Lua API compatibility** (later). Lets existing OpenMW Lua mods
  run. A large API surface, so it comes after the core game works.
- **MWSE-Lua mods won't run.** They patch `Morrowind.exe` memory and assume
  its object layout. Popular ones get reimplemented as native mods.

### 6.4 Baking and caching

Bevy's asset processor caches processed assets (NIF → mesh/scene, DDS → BCn
KTX2, terrain tiles, impostors), keyed by source hash and mod-set hash.
Adding a mod reprocesses only what it touches. A mod-manager UI comes later
and writes the same profile files the CLI uses.

## 7. Feature designs

### 7.1 HD grass

- **Placement:** GPU compute scatter per terrain tile from density masks.
  Masks are derived from terrain layers via data rules (the Morrowind adapter
  maps LTEX texture types to grass species; mods can repaint masks).
  Placement is deterministic per tile seed.
- **Near (≤ ~40 m):** procedural blades in the Ghost of Tsushima style:
  Bézier blades, per-blade wind, clumping, view-facing thickening.
- **Mid:** instanced cards or clumps. **Far:** terrain albedo and normal
  carry the color, with no geometry.
- **Interaction:** blades bend from the deformation trail map (actors,
  physics bodies) and are flattened, cut, or burned via a persistent mask
  layer.
- **Lighting:** grass is raster-only and stays out of the Solari
  acceleration structure (too many dynamic instances). It is lit from the
  same probes/irradiance as everything else, plus contact shadows and a
  terrain-shadow term.
- **Wind** comes from `sf_sky`'s shared wind field, so grass, clouds, rain,
  and cloth agree.

### 7.2 Ocean and water

- **Surface:** Tessendorf FFT with several cascades in WGSL compute
  (concepts from GodotOceanWaves, which is MIT), on a camera-centered clipmap
  mesh. Jacobian foam, subsurface scattering, refraction, screen-space
  reflections.
- **Shoreline:** a distance field baked from terrain against the water
  plane drives wave damping, shore waves, foam lines, and wetness (lessons
  from Godotwind's `shore_overhaul.md` and the ocean audits).
- **Underwater:** absorption (Beer-Lambert), caustics, and correct handling
  of a camera split across the waterline.
- **Buoyancy:** physics needs heights on the CPU. Evaluate the
  lowest-frequency cascade on the CPU (cheap, deterministic) rather than
  reading back from the GPU. Bodies sample a few points per hull.
- **Later:** rivers with flowmaps; lakes at arbitrary heights (Morrowind has
  only one water plane, the framework won't).
- **Specified 2026-10-13** in [specs/water.md](specs/water.md) (M6): bevy-aqua
  evaluated and ported rather than adopted; SSPR instead of SSR for water
  reflections (the exterior camera has no deferred prepass); rivers as baked
  flow; pontoon buoyancy (§10 2026-10-13 m6-spec).

### 7.3 Rigid-body physics

- **Rapier** (`rapier3d` behind `sf_physics`; was Avian, changed 2026-10-12, §10), on a fixed timestep with transform interpolation (the lesson
  of Godotwind's "carry vibration" bug).
- **Static world:** colliders baked from NIF collision nodes when present,
  otherwise from convex decomposition at bake time. Tile-level heightfields
  for terrain.
- **Dynamic world:** loose items (misc, weapons, armor, ingredients) are
  real rigid bodies once disturbed. They start asleep so placed scenes
  stay authored. Physics-driven carry and throw.
- **Ragdolls:** death and knockdown hand off from animation to a physics
  ragdoll and back via blended get-up.
- **Streaming:** bodies in unloading cells freeze and serialize their
  transforms. Nothing simulates outside the active radius.
- **Buoyancy** via §7.2 queries.

### 7.4 Terrain deformation

Two layers, because they have different lifetimes and costs:

1. **Transient surface deformation** (snow, mud, ash, sand trails): GPU
   stamp render target in a ring buffer around the camera, with time-based
   recovery. It affects terrain shading and displacement plus grass bending.
   Render-only, no collision changes. Nearby trails persist briefly in saves.
   This is the Godotwind RTT design, generalized.
2. **Persistent height edits** (craters, digging, landslides, construction):
   sparse per-tile delta heightfields stored in the save overlay. An edit
   rebuilds that tile's physics heightfield, navmesh tile, grass masks, and
   GPU heightmap region.

Both live in `sf_terrain` as a deformation stack that every consumer
(rendering, physics, navigation, foliage) reads, so they never disagree.

### 7.5 Lighting and rendering

- **Solari** is the high-end path; **raster** (clustered forward, cascaded
  shadows, contact shadows, SSR, volumetric fog) is the required fallback.
  It covers non-RT GPUs, and Solari is still experimental.
- **Solari gap:** no point or spot lights yet, and Morrowind interiors are
  lit almost entirely by point lights. Interim fix: each light gets an
  emissive proxy mesh, which Solari handles, while raster lights stay for the
  fallback path. Drop the proxies when Solari ships punctual lights.
- **Bindless** comes free from Bevy's material system. Material dedup
  (a big Godotwind win) is natural there.
- **Missing in Bevy: texture streaming.** Vanilla Morrowind fits in VRAM;
  HD texture packs won't. Plan a mip-residency system (load low mips, stream
  higher mips by screen-space demand) once HD packs are being tested.
- **Old assets:** 2002 diffuse-only textures look flat under PBR. The
  adapter generates conservative normal and roughness maps at bake time, and
  HD replacer mods slot in via §6.1.

### 7.6 Animation

Morrowind's `.kf` files and text keys convert into Bevy `AnimationClip`s
plus animation events. On top of that: a locomotion layer (blend spaces), an
upper-body action layer (masks), additive hit reactions, and an IK pass
(foot placement on terrain and stairs, look-at, hand IK for doors and
levers). The framework never assumes Bip01 naming; skeleton conventions
come from the adapter.

## 8. Bevyrrowind repo (Morrowind content pack)

| Crate | Responsibility |
|---|---|
| `mw_install` | **(built)** Install detection (env var, registry, Steam libraries) and `Morrowind.ini` parsing. |
| `mw_vfs` | **(built)** BSA archive source for `sf_vfs`; mounts Morrowind.bsa, ini archives, then loose `Data Files`. |
| `mw_records` | **(built)** ESM/ESP → `sf_records` with Morrowind merge rules (cell reference merging, deletions, topic-scoped INFO), load order from masters. Uses `tes3` directly; no wrapper crate. |
| `mw_assets` | NIF → Bevy scene loader: geometry, materials, skinning, controllers (keyframe/vis/UV/alpha), particles, billboards, switch nodes, collision. DDS/TGA. |
| `mw_world` | `WorldSource` impl over the loaded records; coordinate mapper (Z-up, ~1.42 cm per unit); LAND → terrain tiles; LTEX → terrain layers and grass rules; doors → transitions. |
| `mw_rules` | Morrowind formulas: combat, magic, skills, leveling, persuasion, as a rules plugin for `sf_rpg`. |
| `mw_dialogue` | INFO filtering semantics (OpenMW reuse allowed here). |
| `mw_script` | MWScript compiler → Luau + `mw` compatibility library. |
| `mw_openmw_lua` | OpenMW-Lua API compatibility (later). |
| `mw_weather` | Morrowind weather presets (ash storm, blight, snow) as `sf_sky` data. |
| `bevyrrowind` | The game binary: settings, install detection, launcher. |

## 8b. Build performance

Compile time is a design constraint, like frame time. The rules:

### Crate graph

1. **Bevy-free cores.** Logic that doesn't need the engine lives in crates
   that don't depend on Bevy: formats, records, mod resolution, rules,
   conversion (`sf_records`, `sf_mods`, `mw_install`, `mw_records`, `mw_nif`,
   `mw_world`). They build in seconds, their tests skip Bevy's link, and
   most logic changes only touch them.
2. **Thin Bevy shells.** A Bevy crate is a plugin: components, systems,
   asset loaders, gluing a core to the engine (`mw_assets` over `mw_nif`,
   `sf_vfs`'s `bevy` feature over `sf_vfs`). Keep heavy code out of them.
3. **Wide and shallow, not tall.** Many sibling crates build in parallel;
   a chain A → B → C → D builds serially. Avoid a "common" crate that
   everything depends on and that changes often: editing it rebuilds the
   world. Shared types that change rarely are fine there.
4. **Thin binary.** `apps/*` only wires plugins together. Gameplay never
   lives in the binary crate.
5. **Split by change frequency, not just by topic.** A crate that is
   edited daily should have few dependents.
6. **Minimal Bevy features per crate.** Libraries depend on
   `bevy = { default-features = false, features = [...] }` with only what
   they use; the app enables the full set. Disabled features are crates
   that never compile.
7. **Generics stay local.** Heavily generic code is monomorphized in every
   crate that uses it. Expose concrete or `dyn` APIs at crate boundaries
   where performance allows (e.g. `KindVisitor` instead of generic
   callbacks everywhere).

### Settings (in both repos)

| Setting | Where | Why |
|---|---|---|
| `rust-lld` linker | `.cargo/config.toml` | Linking is the slowest step of an incremental Bevy build; lld is several times faster than `link.exe`. Ships with Rust. |
| `bevy/dynamic_linking` | app `dev` feature, `cargo dev` alias | Bevy becomes a shared library, so a relink no longer re-links Bevy. The biggest incremental win. Never in shipped builds. |
| Deps at `opt-level = 3`, ours at `1` | `[profile.dev]` | Dependencies build once; Bevy at opt-level 0 is too slow to run. |
| `debug = "line-tables-only"`, deps `debug = false` | `[profile.dev]` | Readable backtraces with far less debug info to generate and link. `--profile debugging` for a full debugger build. |
| Optimized build scripts and proc macros | `[profile.dev.build-override]` | Derive macros run on every build of their users. |
| Pinned toolchain | `rust-toolchain.toml` | Same compiler everywhere; a compiler change invalidates every cache. |
| `profiling` / `dist` profiles | `Cargo.toml` | Release with symbols for Tracy; thin LTO + 1 codegen unit only for shipping. |

Checked against Bevy 0.20's `.cargo/config_fast_builds.toml` (2026-10-12): its Windows section is
exactly `rust-lld.exe` + `rustdocflags`, with every other flag nightly-only. rust-lld is not the
default linker on Windows MSVC (it is on x86_64 Linux), so the config line stays.

A superfargoth crate built from bevyrrowind is a path dependency, not a workspace member, so
bevyrrowind's `[profile.dev.package."*"]` builds it at opt-level 3 without line tables. Measured
(health, 2026-10-12): forcing opt-level 1 changed no touch or body-edit time beyond noise, so it
stays (framework hot paths run fast in `cargo dev`). Side effect: `--profile debugging` from
bevyrrowind has no debug info for framework crates; debug those from the superfargoth workspace.

### Gates (dev loop)

Measured 2026-10-12 on the reference laptop (Ryzen 7 7435HS, 16 threads), idle, bevyrrowind
`cargo dev` path; method and breakdown in `bevyrrowind/docs/perf.md` "Dev loop". Re-measure when a
crate is added to the app's graph or Bevy is upgraded; a gate miss is a bug.

| Gate | Limit | Measured |
|---|---|---|
| No-op build | ≤ 1 s | 0.83 s |
| Touch the app or a leaf plugin crate | ≤ 5 s | 3.3-4.7 s |
| Touch any hot crate (`sf_world_bevy`, `sf_model_bevy`, `mw_nif`, `sf_model`) | ≤ 8 s | 5.2-8.1 s (`sf_model`: 13 dependents, at the limit) |
| Edit a function body in a hot crate | ≤ 10 s | 6.2-10.3 s |
| Clean dev build | ≤ 12 min | 8 min 38 s (M1: 11 min) |
| Cold bake (Morrowind, fresh store) | ≤ 2 min | 47.7 s |
| No-op bake | ≤ 2.5 s | 1.75 s |

Incremental time is cargo's freshness check (~0.8 s) + ~0.5-1 s per crate between the edit and
the app + the app crate (~2 s rustc, 0.73 s link). It grows with chain depth, not crate size: keep
rule 3 (wide and shallow) and rule 5 (split by change frequency) in mind for `sf_model`.

### Machine setup (Windows)

- **Exclude the `target/` folders (and `~/.cargo`) from Windows Defender
  real-time scanning.** Scanning every object file and rlib is often the
  single largest overhead on Windows. Needs admin; do it once.
- Use a Dev Drive (ReFS) for the repos and cargo home if available.

### Not adopted (yet)

- **Cranelift backend, `-Zshare-generics`, the parallel frontend
  (`-Zthreads`)**: real wins but nightly-only. Revisit if they reach stable.
  Owner decision (health, 2026-10-12; not measured, no nightly is installed): a nightly toolchain
  for dev builds only. Bevy's fast-builds config lists `-Zthreads=0` and `-Zcache-proc-macros`
  for Windows and warns that `-Zshare-generics` must stay off (`=n`) with dynamic linking there
  (65k export limit). Cranelift ships for Windows but without panic unwinding (`panic=abort`).
  Where it would help: the app crate's ~2 s of rustc per edit and the clean build's Bevy chain.
- **Shared `CARGO_TARGET_DIR` across both workspaces**: different feature
  sets would make them evict each other's artifacts.
- **workspace-hack crate (`cargo hakari`)**: unifies dependency features
  so building one package doesn't rebuild deps another package built with
  different features. Adopt when that rebuild churn shows up.

## 9. Milestones

Every milestone ends with something visibly running and a benchmark entry.

| # | Slice | Done when |
|---|---|---|
| M0 | Both workspaces, VFS, record DB, mod manifest, MW install detection, ESM → records, record browser | Morrowind.esm + Tribunal + Bloodmoon load, with a conflict report and timing |
| M1 | NIF loader + DDS, interior cell rendering (raster), material dedup, benchmark harness | Seyda Neen Census Office renders correctly |
| M2 | Exteriors at scale: grid coordinates (`big_space`), worldspaces + door links, `sf_terrain` clipmap from LAND (upsampled), streaming rings with budgets, bake pipeline (`sf_bake`) with LODs, atmosphere + sun, Solari toggle | Fly over Vvardenfell within the §0 frame-time targets |
| M3 | Characters + animation: assembly, retarget to a standard rig, layered graph, blend spaces, root motion, IK (feet, look-at, hands) | NPCs walk the streets with feet on the ground |
| M4 | Physics: rigid-body items, controller, carry/throw, ragdolls (powered), cloth (capes, robes) | Knock a table's contents over; a dead guard ragdolls; robes move |
| M5 | Mods + Lua: runtime, curated API, mod pipeline end to end, MWScript → Luau; game UI slice (`ui.md`): HUD, virtualized inventory, settings + mod settings, message box, both themes and presets, `.bsn` layouts | A Lua mod and a texture mod both work, hot reload works; a theme mod, a CSS patch and a Lua inventory column work together, gamepad-only |
| M6 | Water + weather + surfaces: ocean (evaluate `bevy-aqua`), water bodies at any height, baked rivers to the sea; weather (`weather.md`): state machine with blended presets as records, Nubis clouds + cloud shadows, froxel/height/ground fog with god rays, one wind field (grass, trees via baked vertex data, impostors, cloth, particles), precipitation with occlusion, surface state (wetness, puddles, snow, ash), character wetness, HD grass, far terrain shadows (shadow-height map) | The Bitter Coast in a rainstorm: puddles, wet wading, a boat that floats; an ash storm at Red Mountain; sunrise god rays in Balmora; Bitter Coast morning fog; all within the weather.md budgets |
| M7 | RPG core (moved up from M10+, owner 2026-10-13): Dialogue, journal, stats and rules, magic, AI, saves, persistent deformation | Main quest playable (long tail) |
| M8 | Far-field performance: HLOD clusters, impostors, occlusion culling, terrain erosion bake, PBR generation, texture streaming | Vvardenfell skyline from Red Mountain within frame targets |
| M9 | Seamless interiors: portal rendering, crossing between worldspaces, environment-volume blending | Walk into the Census Office without a load or fade |
| M10 | Combat (Dark Souls 3–inspired): animation windows, stamina/poise, swept hitboxes, lock-on, rolls, parry/riposte, data-driven movesets | Duel a mudcrab and a Dark Brotherhood assassin, and it feels good |

Wishlist details and the early decisions they force:
[WISHLIST.md](WISHLIST.md). M6–M10 can overlap with each other once M2–M4
exist; order follows what's most fun to see next, gated by §0 targets.
Renumbered 2026-10-13 (owner): RPG core moved from M10+ to M7; far-field performance, seamless
interiors and combat moved to M8, M9, M10. Specs and logs written before that date use the old numbers
(old M7 = M8, old M8 = M9, old M9 = M10, old M10+ = M7). Priority now: M4, M5, M6.

## 10. Decisions log

- 2026-10-11: framework name **superfargoth**; Morrowind repo **GPL-3.0-or-later**.
- 2026-10-11: `tes3` is used only from the bevyrrowind repo.
- 2026-10-11: record conflicts are **dependency-aware**. A record touched by
  several sources is a conflict only when a later source does not
  (transitively) depend on an earlier one that also touched it. Known
  refinement: for merged kinds (cells), conflicts should eventually be
  reported per reference rather than per cell.
- 2026-10-11: loose `Data Files` always beat BSAs (the original engine used
  file dates). ESMs load before ESPs, then each file after its masters.

- 2026-10-11: load order uses a known-content hint table
  (`mw_records::KNOWN_ORDER`): GOTY Bloodmoon.esm doesn't list Tribunal as a
  master, Steam stamps all files with the install time, and Steam's
  `Morrowind.ini` lists files alphabetically.
- 2026-10-11: reference rotation convention (`mw_nif::coords::reference_rotation`)
  verified visually on the Census and Excise Office.
- 2026-10-11: editor markers (NorthMarker, DoorMarker, ...) are identified
  by base record ID, not mesh flags; kept as placements, never rendered.
- 2026-10-11: build performance setup (§8b). Measured: 11 min from
  scratch, then 2.2 s (app crate) and 3.9 s (`mw_assets`) to rebuild after
  a small edit with `cargo dev`.
- 2026-10-11: WISHLIST early decisions settled in the M2 specs
  (`docs/specs/`, drafts pending review):
  1. **Coordinates**: `big_space` 0.14 (pinned `=0.14.0-rc.0`, Bevy 0.20),
     `i32` cells, `GRID_CELL` = 256 m. Bevy-free `WorldPos` lives in
     `sf_coords`. Records and saves store (worldspace, cell, local), never
     `GlobalTransform`. Every partition (streaming cells, terrain tiles,
     HLOD clusters) is a power of two in the same grid.
  2. **Worldspaces**: `Worldspace` and `DoorLink` records are the streaming
     unit, and interiors are worldspaces. Each loaded worldspace gets its own
     `BigSpace` root and `RenderLayers` layer. M2 crosses links with the
     `FadeTransition` scaffold, which M8 removes.
  3. **Terrain stack**: base + persistent edits + transient deformation.
     CPU consumers read base + edits through `TerrainHeights`. Geometry is
     CDLOD over toroidal height clipmaps, on canonical 256·2^k m tiles
     with 0.5 m base spacing.
  4. **Water query**: one CPU-side, synchronous, deterministic API (height,
     normal, velocity, depth, body id) owned by `sf_water`, specified at the
     start of M6, before any water-dependent feature. GPU-readback queries
     (bevy-aqua's) may refine visuals only.
  5. **Surface state**: every framework material includes the shared
     surface-state WGSL hook from its first commit, starting with terrain.
     It is a no-op until M6.
  6. **Bake**: our own content-addressed step graph (`sf_bake`, blake3,
     constructive traces), not Bevy's `AssetProcessor`. Objects are shared
     across mod sets, with one manifest per profile. `meshopt` 0.6.2 (the
     version Bevy 0.20 uses) for LODs.
  7. **Performance**: the WISHLIST §0 targets are `sf_diag` gates. M2 adds
     the streaming fly-through scenarios.
- 2026-10-11: crate convention for engine glue. A system with substantial
  glue is a Bevy-free core `sf_x` plus a sibling plugin crate `sf_x_bevy`
  (world, terrain, model). Small glue stays a `bevy` feature (`sf_vfs`).
- 2026-10-11: Bevy 0.20's `OcclusionCulling` (no longer experimental)
  needs a depth prepass and doesn't work with deferred shading, so the
  raster path stays forward with a depth prepass.
- 2026-10-11: owner's priorities: **runtime speed and visuals first.**
  Baking is acceptable only as a one-time step, like a "compiling shaders"
  screen. Consequences, now in the specs (all four agreed):
  - The whole first bake, every product through M7, takes ≤ 2 min, with a
    5 min ceiling on the reference machine. This is an `sf_diag` gate.
  - Nothing bakes during play.
  - Static models are cooked whole (`mesh.cook`): no legacy format is
    parsed at runtime.
  - Stored objects use LZ4, for decode speed.
  - Render pipelines warm up on the bake screen.
  - Terrain stays at 0.5 m baked spacing and gets render-only detail
    displacement near the camera instead of a finer bake.
  - Ring radii are the largest that pass the §0 gates per quality preset.

- 2026-10-11 (m1): NIF runtime (spec `bevyrrowind/docs/specs/nif-runtime.md`).
  Controllers normalize to Hermite curves and slerp/Euler rotation tracks
  in Bevy-free `mw_nif`; Bevy only writes sampled values. Particles are
  our own CPU simulation (bevy_hanabi 0.19 needs Bevy 0.19; emitters hold
  tens of particles; exact NIF semantics). These are model data the bake
  (`mesh.cook` / `sf_model`) must carry when NIFs stop being parsed at
  runtime.
- 2026-10-11 (m1): a NIF root node's own transform is ignored: the engine
  replaces it with the placement's (fixed mis-rotated models such as
  `in_nord_fireplace_01`).
- 2026-10-11 (m1): fixed-function emissive maps to PBR emissive *through
  the base texture*, scaled to "fully lit" at EV100 7. Lights:
  8000 lm per m² of radius, range 4× radius, ambient 300 nits; chosen by
  screenshot, tunable with `view --lumens/--range/--ambient`.
- 2026-10-11: `sf_diag` (spec `diag.md`).
  - Frame time comes from `Time<Real>` at the frame boundary, so it lines
    up with that frame's counters. Bevy's `DiagnosticsStore` is sampled
    generically, which brings GPU per-pass timings with
    `RenderDiagnosticsPlugin`.
  - A regression is more than 10 % **and** more than 0.25 ms against the
    baseline. Tracy stays the profiler; `sf_diag` measures and gates.
- 2026-10-11: cooked meshes. Bevy can't load meshopt-compressed or
  quantized glTF, and `SerializedMesh` is not a stable on-disk format.
  So bake outputs use our `.sfmesh` (meshopt codecs), and the loader
  applies Bevy 0.20's own vertex compression
  (`MeshCompressionArgs::regular`). glTF imports through the `gltf`
  crate, the same version Bevy uses.
- 2026-10-11: `sf_world` core (streaming.md, updated from implementation).
  - `WorldSource` has no terrain hook. `sf_terrain` depends on `sf_world`
    (for `WorldspaceId`), and adapters hand their `TerrainSource` to
    `sf_terrain` directly.
  - The world index is a resident directory per worldspace plus 8×8×8-cell
    pages.
  - Velocity lookahead decides residency, while view direction only affects
    priority, so turning the camera never loads or unloads.
  - A budgeted stage starts an item only while usage is below the limit,
    so it overshoots by at most one item.
  - Binary formats use the grid form of `WorldPos`. Its untagged text form
    needs a self-describing format.
- 2026-10-11: M3 specs (draft: `animation.md`, `character.md`,
  `locomotion.md`) propose four changes:
  - Replace §7.6's "convert into Bevy `AnimationClip`s" with cooked clips
    and a Bevy-free pose runtime. Bevy 0.20's `AnimationGraph` has no root
    motion, no IK hook between blending and write-back, and no
    sync/inertialization; Bevy keeps skinning, morphs and culling.
  - Retargeting and clip cooking move into the one-time bake.
  - Crate plan: `sf_anim` (Bevy-free, with IK and retarget math),
    `sf_anim_graph`, `sf_anim_bevy`, `sf_character_bevy`, and
    `sf_controller(+_bevy)`, whose kinematic slice comes in M3.
  - `parry3d` for static ground probes before M4 physics.
  Known gap: skinned meshes aren't in Solari's ray-tracing scene (no joint
  attributes in `RaytracingMesh3d`). The likely fix is a compute skin
  cache: measure in M3, decide by M7.
- 2026-10-11: `sf_terrain` core (terrain.md, updated from implementation).
  - The pyramid stops at up to 2×2 roots, not one tile: the grid corner at
    −128 m is a tile border at every level.
  - Tile borders are stored as f32 whatever the quantization, so neighbors
    decode bit-identical seams.
  - `TerrainSource` takes `sf_world::WorldspaceId` and reports tile bounds
    and fingerprints. `TerrainHeights` is per worldspace and owns its edits.
  - Quads split on the (i,j)→(i+1,j+1) diagonal, and render patches and
    colliders must match it.
  - Bake inputs that aren't files use the record-input convention (kind
    `terrain.source_tile`, fingerprinted by the adapter's `Inputs`).
  - First real bake (all of Morrowind's 6,609 models): 21.8 s cold,
    1.4 s warm, inside the 2-minute budget. Border locking in `mesh.cook`
    is now off by default (most chains stalled).
- 2026-10-11: Bevy 0.20 bug worked around (named scaffold
  `sf_model_bevy::compression_args`).
  - `Mesh::take_gpu_data` recomputes the AABB from `Float32x3` positions
    only, so GPU-only meshes with compressed positions render collapsed.
  - Workaround: GPU-only meshes skip position compression. Normals, UVs
    and indices stay compressed.
  - Removal: when upstream keeps the AABB computed at compression time.
    The upstream issue is not filed yet (needs the owner's OK to post).
  - `mesh.cook` now escalates its error cap per level before giving up.
- 2026-10-11: Morrowind world adapter (`mw_world`, spec `world-adapter.md`).
  - `PlacementId` = (lowercased name of the file that first placed the
    reference, refnum), the original engine's identity. Plugin edits keep
    the master's id, and load order doesn't matter.
  - LTEX indices are per plugin, kept as `mw.ltex_slot` records.
  - Doors pair two-way within 8 m.
  - Vanilla Morrowind + Tribunal + Bloodmoon contain no MVRF; re-homing is
    implemented and tested on synthetic data.
- 2026-10-11: sky and render path (`sf_sky`, `sf_sky_bevy`, spec `sky-lighting.md`).
  - Solari in Bevy 0.20 supports directional lights, emissive meshes and
    the environment map only, so §7.5's point-light proxies stand.
  - Proxies are spheres registered with the ray tracer only, never
    rasterized, with luminance matched to the light's intensity.
  - Solari forces the deferred renderer, so the render-path switch owns
    `DefaultOpaqueRendererMethod`: forward + `OcclusionCulling` on raster,
    deferred on Solari. Switching re-prepares materials once and restores
    the camera's settings.
  - ReSTIR is on by default while there is no denoiser. DLSS (Ray
    Reconstruction and upscaling) needs the NVIDIA SDK at build time and
    isn't wired yet.
  - Exposure follows a sun-elevation schedule, not `AutoExposure`, which
    greys out nights.
  - Known gap: Solari ignores atmospheric transmittance on directional
    lights, so its sunsets come out too red and dark. A small upstream
    patch to `bevy_solari` is a candidate.
- 2026-10-11 (owner): **Solari is the "ultra" option, not the baseline.**
  The raster path is what most players see and must look great on its own
  (GI for it is open: baked interior probes vs dynamic probes). Solari is
  experimental, has no punctual lights, and its only denoiser is DLSS Ray
  Reconstruction (NVIDIA).
  - Ray-tracing meshes are built at load time, only on the ray-traced
    path, from one cooked LOD level (`SfMeshLoaderSettings::ray_tracing_lod`).
    Nothing extra is baked or stored. Solari takes only uncompressed
    position/normal/uv0/tangent meshes with u32 indices and skips anything
    else silently, while raster meshes stay compressed and keep vertex colors.
  - Solari ignores visibility (hidden entities and every LOD level would
    all be traced), so the mirror registers visible entities only, and
    loaders mark the levels the ray tracer doesn't use `NoRayTracing`.
  - The render path belongs to `RenderPathCamera`, which `SkyCamera`
    requires, so interiors without a sky can be ray traced.
  - First ray-traced Census Office (RTX 4060 Laptop): cooked meshes traced,
    bounce light and soft shadows work. Heavy noise without a denoiser;
    candle flames and glows (blended/additive) don't show.
- 2026-10-11 (owner): **far field moves from M7 into M2** (spec `far-field.md`, draft).
  - Shape: UE World Partition-style HLOD over our streaming grid, selected
    like a quadtree (CDLOD's refine rule). L0 = objects with their cooked
    LOD chains; L1 = a merged + simplified proxy per streaming cell, with a
    far texture array; L2/L3 = coarser proxies per 1 km / 4 km with
    vertex-color albedo, always resident when small. Octahedral impostors
    for foliage, light sprites for night skylines.
  - Swaps between representations use Bevy's `VisibilityRange` first; a
    custom complementary dither (per-node fade slot through `MeshTag`)
    only if measured pops justify it.
  - Evaluate `bevy_3d_tiles` / OGC 3D Tiles before writing selection and
    streaming of the proxy tree ourselves.
  - Bevy's `MeshletPlugin` is not used: Vulkan/Metal only, no vertex
    colors, no MSAA, no forward/deferred mix, and it doesn't remove the
    per-entity cost that dominates in Bevy.
  - Everything far is baked and simplified offline (UE's HLOD build),
    never merged at runtime.
- 2026-10-11 (owner): **reuse order is Bevy, then crates, then our own**,
  each skipped step justified in the spec (rule in both CLAUDE.md files).
  Prior projects (Godotwind on Godot) are anecdotes, never design
  references: their tiers and constants don't transfer to Bevy.
- 2026-10-11 (farfield-exterior): **first real exterior on the framework**
  (bevyrrowind `view <exterior>`; bake-adapter.md, world-adapter.md).
  - One `Baker` session bakes models, the world index, and terrain, so one
    manifest names every product; host-computed record kinds
    (`terrain.source_tile`, `terrain.layer`) enter as extra record
    fingerprints of the host's `Inputs`.
  - Wiring for any content pack: the terrain origin and the atmosphere's
    sea level are set from the `big_space` grid's local floating origin
    *after* propagation and *before* `TerrainSystems::Update`, so terrain,
    sky and objects move in the same frame; the sky's sun and moon need
    the worldspace's render layer (lights and shadow casters filter by
    `RenderLayers`). `sf_world_bevy`'s `WorldOrigin` is updated in the
    same window without an exported set, which is why hosts read the grid
    directly.
  - Measured: a town with the default rings holds ~350k entities and is
    CPU-bound at ~21 ms (GPU ~4 ms); frame time follows resident entity
    count, not streaming work (every stage stayed inside its budget). The
    far field, not stage budgets, is the lever; ring radii are calibrated
    from the table in the roadmap's session log.
- 2026-10-11: far field implemented (`sf_lod`, `sf_lod_bevy`; spec `far-field.md`).
  - Swaps between levels use Bevy's `VisibilityRange` (engine first) with
    "lead bands": a coarser node is fully drawn before any child fades out,
    so there are no holes by construction; a CPU pass forces a node fully
    visible while its children load (3D Tiles / UE rule). The custom
    time-based dither is a fallback, built only on visual evidence.
  - 3D Tiles semantics (geometric error per node, REPLACE refinement)
    adopted, not its format; `bevy_3d_tiles` rejected (Bevy 0.19, no
    instancing, entity churn).
  - Proxies: merged LOD0 welded by position + color, meshopt simplify with
    `Prune`, normals rebuilt at a 40° crease, albedo in vertex colors
    (texture averages through an adapter `TextureDecoder`). Measured on
    Morrowind: LOD0 sources beat cooked-LOD sources (4.1M vs 6.2M L1
    triangles); a sloppy pass never helped. Whole Morrowind bake 21.9 s.
  - Streaming: R2 loads a cell's L1 node through `sf_world_bevy::FarProxies`
    (a hook, so `sf_world_bevy` doesn't depend on the far field), measured
    to the cell's page; far-eligible objects end their LOD chain at `d1`.
  - `WorldBake::with_overrides`: index settings per worldspace (interiors
    as `Partition::Single`).
- 2026-10-11 (owner, farfield-ui): **game UI design** (spec `ui.md`,
  Morrowind side `bevyrrowind/docs/specs/ui-adapter.md`). Built from M5.
  - **Both themes from day one**: a content-pack theme (Bevyrrowind:
    parchment, bronze frames, Magic Cards-style face) and a modern minimal
    one. A theme is data, swappable live.
  - **Both layout presets** over the same MVVM view-models: PC-dense (column
    lists, optional draggable classic windows) and controller-first
    full-screen. Gamepad-complete in both; switching devices changes glyphs
    and focus visuals only, never the layout.
  - **Modder styling is CSS via `bevy_flair`**, contained in `sf_ui_css`.
    One composed stylesheet per layer root imports framework, preset, theme,
    then each mod's sheets as `@layer mods.<id>` in load order, then the
    player's `user` layer. Replaces the draft's own `ui.style` records.
    flair's 0.20 port is on git `main` only (§4 tracker, fallback plan in
    ui.md R2b). It pulls MPL-2.0 Servo crates (`cssparser`, `selectors`):
    the owner must allow MPL-2.0 for unmodified dependencies, or CSS needs
    another engine.
  - **Feathers is not used for game UI** (its own docs say it isn't meant
    for it); dev tools only. CommonUI-style layers and activatable screen
    stack, headless `bevy_ui_widgets`, `bevy_input_focus`.
  - **Timing**: spec done now, no UI code before M5; stock screens use
    `.bsn` files once Bevy 0.21 ships them (PR #26043, 0.21 milestone),
    `bsn!` templates if M5 starts earlier.
  - Bevy 0.20 facts the design relies on: UI picking resolves individual
    `TextSpan`s (dialogue links), text is TTF/OTF re-rasterized per size
    (no SDF needed), `Em`/`Rem` units, `InlineImage` in text runs.
- 2026-10-11 (anim-graph): animation graph built (`sf_anim_graph`, spec
  `animation.md` "As implemented in `sf_anim_graph`").
  - Bevy-free runtime. Authoring (RON via `ron` 0.12, the version Bevy
    already uses) and `animgraph.compile` (Delaunay via `spade` 2.15) sit
    behind a `bake` feature, so the runtime depends on neither.
  - Inertialization corrects Bollo's GDC 2018 form in two ways: the
    offset's initial velocity subtracts the target's velocity (one extra
    look-ahead evaluation on the switch frame), and offsets are 3-vectors
    with a quintic per component instead of a scalar along a fixed axis.
    Pose and velocity are then continuous (tested at shrinking dt). It is
    a post-process on the graph output, as in UE.
  - Sync membership is a property of players (UE), not a `Sync` node.
    Markers are used when the leader's current pair exists in the
    follower, otherwise phase.
  - `AimOffset` is an additive blend space in the graph, not a procedural
    op; `SpeedMatch` acts on playback rates before the advance.
  - Graph root motion is `Drive` players only, plus a `drive_weight` that
    the mover blends by.
- 2026-10-11 (controller): locomotion core built (`sf_controller`, spec
  `locomotion.md` "As implemented in `sf_controller`").
  - The kinematic mover's logic is Bevy-free in `sf_controller` (it was
    planned for `sf_controller_bevy`); the Bevy crate only schedules and
    batches. Probes are batched per stage across all characters.
  - Static queries use parry3d 0.31 directly (`Bvh`, `TriMesh`,
    `cast_shapes`; glam 0.33 types, no Bevy), not Rapier's character
    controller, which needs rapier's physics query pipeline and can't see
    terrain. Terrain enters through a `HeightSurface` trait implemented for
    `sf_terrain::TerrainHeights` behind a feature, so `sf_terrain` is
    untouched.
  - Walkability is the mover's decision (slope limit per character), not a
    probe result field. The slope limit blocks *climbing* motion on
    unwalkable ground; a rise threshold let springs creep up steep ramps.
  - Only steps are height-smoothed (drawn offset), not slope-following, which
    would lag below ramps; probes and the trajectory use the exact feet.
  - `collision.cook`: per model path, authored collision via adapter hook,
    else render LOD1 of `mesh.cook`, welded, surface per triangle;
    `collision/<model>.sfcol` overrides.
- 2026-10-11 (farfield-lod, verified on Morrowind): **the far field runs on
  Vvardenfell** (spec far-field.md, "Measured on Morrowind").
  - Towns drop from ~350k to ~48k entities; Balmora 7.8 ms at 720p and
    11.9 ms at 1440p (was 20.9 / 22.2 ms); far CPU 0.14–0.17 ms, far
    entities 2.3–3.5k, far GPU ~1.2 ms at 1440p (RTX 4060 Laptop). Fly-throughs
    stay under 33 ms; the 1.5× worst/median gate still fails on streaming
    frames (levers in far-field.md open question 2).
  - Lead bands (Bevy `VisibilityRange`) kept: the double draw costs 0.26 ms
    GPU at 1440p. Proxies sit behind what they replace by a depth offset in
    pixels (`StandardMaterial::depth_bias`; float reverse-Z makes constant
    bias a fraction of distance). `d1` calibrated: 400 m (300 m for a low
    preset).
  - **Bevy 0.20 workarounds:** `sf_world_bevy` leaves
    `RenderAssetBytesPerFrame` off (instances collected while their upload
    is deferred keep empty offsets: objects vanished at random), and the
    exterior camera has **no depth prepass** (Bevy's depth-only prepass
    ignores the `VisibilityRange` dither, punching holes into every
    crossfade). This defers sky-lighting.md's "forward + depth prepass for
    occlusion culling" until Bevy fixes the prepass dither. Both are
    upstream issues to file.
- 2026-10-11 (character): character core built (`sf_character`, spec
  `character.md` "As implemented (`sf_character`)").
  - **Auto-map is hierarchy-driven**: names only classify joints and anchor
    the search (hips, hands, feet, fingers); the bone comes from position in
    the hierarchy. Fuzzy name matching (UE chain auto-map) breaks across
    conventions (Mixamo `LeftLeg` = lower leg, `LeftArm` = upper arm;
    3ds Max thighs under the spine). Two real CC0 Quaternius rigs (UE-style
    and Rigify names) map with no fixes; Bip01-, Mixamo-, UE5- and VRM-named
    rigs are covered by tests.
  - **Auto-align targets fixed canonical directions** (N maps, not N²) and
    first corrects facing on the topmost joint, so **retargeted clips face
    +Z**; adapters whose native clips face elsewhere turn the skeleton at
    import. The VRM hierarchy is checked as ancestor relations, not direct
    parents (3ds Max rigs).
  - Spec changes: `Layer` is an open `u8` scale with named bands; an item
    clears its `hides` slots before placing its parts (it hides what lies
    under it); `Wearable` gained `first_person` parts and resolution a
    `View`; the outfit key hashes slot *names* (slot ids are
    session-local); `CharacterSource::recipe` takes (kind, id) until
    `RecordRef` moves from `sf_world` to `sf_records` (follow-up).
  - Small CC0 test rigs (node hierarchies only) live in the crate's
    `tests/data/`, with their sources in a LICENSE.md.
- 2026-10-11 (owner + anim-plan): **next-gen animation plan** (specs:
  `animation.md` "Next-gen roadmap", new `rig-profile.md`, `locomotion.md`
  "Context probes, traversal and creature modes",
  `bevyrrowind/docs/specs/animation-sources.md`). Design only.
  - **Hybrid, as UE5's Game Animation Sample**: authored state machines +
    root motion + event windows stay the backbone (souls-like timing,
    creatures); motion matching is a later graph node for humanoid
    locomotion once a library is baked. Kept: our ozz-model core and
    `sf_anim_graph` (`ozz-animation-rs` is MPL-2.0 and nightly-only).
    NVIDIA MotionBricks is a watch item: Apache-2.0 code and Open Model
    License weights do exist, but the checkpoints drive a Unitree G1 robot.
  - **Environment reaction, in order**: motion warping (Witkin & Popović
    1995, UE5 Motion Warping: warp windows as clip events, targets per
    update; M3), full-body IK (position-based like UE's PBIK, with XPBD
    compliance; M3), context probes (affordances from batched scene
    queries, GASP-style traversal; probes M3, actions M4), physical
    animation (powered ragdoll with stable-PD drives, hit reactions,
    stumble, knockdown + get-up through `PoseInput`; M4).
  - **Creatures are first-class**: every procedural op reads a per-skeleton
    `RigProfile` (girdles, N limbs with a solver kind, spine, neck/head,
    tails, wings; auto-detected at bake, Spore/UE IK Rig style). The
    humanoid rig becomes an optional layer that produces a profile. The
    audit found 17 biped/humanoid assumptions in `sf_anim`,
    `sf_anim_graph`, `sf_controller` and `sf_character` (table in
    animation.md); fixes are ROADMAP M3 tasks.
  - **Animation data is bring-your-own**, like the Morrowind install: the
    bake retargets libraries the user downloaded by hand (Quaternius UAL
    CC0, mesh2motion CC0, 100STYLE CC BY 4.0, CMU, Mixamo, LaFAN1 CC
    BY-NC-ND 4.0, Bandai Namco CC BY-NC 4.0); nothing third-party is in a
    repo or release, cooked clips carry license provenance and exports
    refuse non-redistributable ones. Every slot falls back to the
    content pack's own clips, so graphs support optional clips. UE GASP's
    clips are Unreal-only and excluded. Corrects animation.md, which had
    ruled LaFAN1 and Bandai Namco out entirely (they are fine for private
    local bakes).
  - Planned importers: BVH (own parser or `bvh_anim_parser`) and FBX via
    `ufbx` 0.11.5 (MIT OR Unlicense; needs a C compiler at bake only).
  - Input to the M4 physics decision (§11): avian3d 0.7 has no spherical
    joint motor, so powered-ragdoll drives are applied as PD torques
    (engine-agnostic); Rapier has per-axis generic joint motors.
- 2026-10-11 (weather-spec, spec only): **weather, atmosphere and distant
  shadows designed** (`specs/weather.md`, adapter `weather-adapter.md`).
  - One integrated atmosphere (RDR2/UE structure): a Bevy-free
    `WeatherFrame` per frame from blended `WeatherPreset`/`Climate`
    records drives sky haze, clouds, fog, wind, precipitation, surface
    state, cues and gameplay tags. The forecast is a pure function of
    (seed, climate, change index), so saves store only seed + overrides.
    Region borders crossfade over a 256 m band.
  - New crates: `sf_wind`, `sf_weather`, `sf_atmos`, `sf_surface`
    (Bevy-free) + `sf_wind_bevy`, `sf_weather_bevy`, `sf_atmos_bevy`,
    `sf_surface_bevy`, `sf_precip_bevy`. This supersedes §5.2's `sf_sky`
    row for clouds, weather, wetness and wind; `sf_sky` keeps the sky and
    `Weather`, and `Wind` moves to `sf_wind` (re-exported).
  - Bevy used first: `Atmosphere` already applies aerial perspective to
    all geometry (verified in `render_sky.wesl`); `DirectionalLightTexture`
    carries cloud shadows to every PBR material. Not used: `VolumetricFog`
    (per-pixel per-box raymarch, no temporal integration, large-volume
    artifacts measured in Megalophobia), `DistanceFog` on exteriors (no
    height falloff). Ours: Nubis clouds (¼ res, temporal), froxel fog,
    analytic height fog + far raymarch, closed-form precipitation volumes
    (no particle crate needed), wind field, surface-state clipmap.
  - **Distant shadows**: CSM `maximum_distance` must reach the far-field
    handoff (`d1·1.1` + margin, default 500 m, was 300 m) so no L0 object
    ever crosses a shadow edge. Past it, a terrain **shadow-height map**
    (horizon mapping reduced to the sun direction) + a far-shadow
    correction in our materials (Bevy returns "lit" past the last cascade
    with a hard cut, `shadows.wesl`). The correction is a named scaffold
    pending an upstream Bevy hook.
  - **Ghostfence pop** traced: animated-material models are skipped by
    `mesh.cook`, so they have no far proxy, and the NIF spawner ignores
    `SpawnCx::max_distance`/`end_band`. Fixes: ranges on NIF meshes, a
    `mesh.rest` product for far use, preload before reveal, later a
    split cook (weather.md "Near-term visual fixes").
  - Megalophobia (owner's Bevy 0.19 project): its Nubis deck math, noise
    generators, ground-fog density and curl-noise grass wind are ported,
    not its architecture. **Gate:** the owner licenses those files as
    MIT OR Apache-2.0 first (§11).
- 2026-10-11 (owner decisions, recorded by the orchestrator):
  - **MPL-2.0 allowed for unmodified third-party dependencies** in
    superfargoth (file-level copyleft only; we never patch those files in
    our tree, and a needed patch goes upstream or into a separate fork
    crate). Unblocks `bevy_flair`'s Servo CSS crates (`sf_ui_css`).
    Removed from §11.
  - **Animation data** (`bevyrrowind/docs/specs/animation-sources.md`):
    releases ship cooked clips derived from shareable sources (CC0:
    Quaternius, mesh2motion; CC BY: 100STYLE, with attribution).
    Mixamo is allowed as a manual, private bring-your-own source (never
    redistributed); LaFAN1 likewise private-only. Modders bring their own.
  - **M3 milestone check includes creatures**: a guar and a cliff racer
    animate, foot-place and move with the same gates as the Balmora NPCs.
- 2026-10-12: **DLSS wired** (spec `sky-lighting.md`; Bevy's
  `bevy_anti_alias::dlss`, `dlss_wgpu` 6.0.0, NVIDIA DLSS SDK 310.9.1).
  Supersedes the 2026-10-11 "DLSS isn't wired yet" note.
  - Super Resolution on the raster path and Ray Reconstruction (denoise +
    upscale) on the ray-traced path, one quality setting for both. Settings
    hook: `sf_sky_bevy::UpscalingSetting` (`sf_sky::Upscaling { Off,
    Dlss(UpscaleQuality) }`, switchable at runtime), with `ActiveUpscaling`,
    `UpscalingSupport` and `TemporalHistory { reset }` (games set it on a
    floating-origin shift; terrain MVs are wrong for that one frame).
    **Default off**; consider `auto` on NVIDIA once foliage smear at 1440p
    in motion is judged. Vendor-neutral fallback (Bevy TAA) not wired.
  - The `dlss` feature **no longer implies `solari`** (Bevyrrowind app and
    `sf_sky_bevy`): SR works on raster builds. The "ultra" build is
    `--features solari,dlss`.
  - **ReSTIR defaults off under Ray Reconstruction** (`SkyRenderOptions::
    ray_tracing_restir: Option<bool>`, `None` = on only without RR). Census
    Office 1440p, RR auto: 10.5 ms with ReSTIR, 8.4 ms without.
  - SR requires depth + motion-vector prepass. With motion vectors, Bevy
    0.20's default prepass fragment runs `visibility_range_dither`, so the
    LOD/far-field crossfade has no holes under DLSS (DLSS even smooths the
    dither into a blend). The depth-only prepass still has the holes
    (far-field.md pitfall 2; two-line upstream fix in `sky-lighting.md`).
  - Measured (RTX 4060 Laptop, release, Balmora 2560x1440 median): native
    11.9 ms, DLSS quality 8.9 (SR pass 2.1 ms; main pass 6.4 -> 1.3 ms),
    performance 9.2 (SR 2.8 ms, slower than quality), DLAA 11.7. Exterior
    `--prepass` (depth + MV + occlusion culling) 13.9 ms: occlusion culling
    doesn't pay in Balmora, so it stays an A/B flag.
  - Build: `DLSS_SDK`/`VULKAN_SDK`/`LIBCLANG_PATH` come from each repo's
    `.cargo/config.toml` `[env]` (relative = true, pointing at the
    workspace's `sdk/`, outside both repos; `sdk/fetch.sh` fetches the ~140 MB subset
    with exact-size checks and resume). Bevyrrowind's `build.rs` copies
    `nvngx_dlss.dll` + `nvngx_dlssd.dll` next to the executable.
  - Fixed a Startup race in the exterior view: `setup` and `setup_spawners`
    both call `registry.ensure`, so `ChildOf(root)` could apply before the
    root spawned (panic with `solari`); now `(setup, setup_spawners).chain()`.
- 2026-10-12 (owner decisions, recorded by the orchestrator):
  - **Megalophobia code may be ported into superfargoth** under MIT OR
    Apache-2.0: the owner wrote it ("my own work"). Applies to the cloud,
    ground-fog and grass-wind files weather.md lists; its third-party pieces
    (MPL erosion file, CC BY meshes, NASA imagery) stay excluded. Removed
    from §11; unblocks weather.md's sky work.
  - **DLSS on by default** on GPUs that support it (`--dlss` default
    `auto`, Quality); other GPUs keep native/TAA. `--dlss off` remains.
  - Upstream Bevy issue reports: not filed for now (owner). The DLSS
    project ID stays a placeholder until later.
- 2026-10-12 (anim-rig + anim-creatures; rig-profile.md, animation.md,
  locomotion.md "As implemented"):
  - **Creatures are first-class through a rig profile**, not the humanoid
    map: `sf_anim::rig::RigProfile` (girdles, N-joint limbs with a solver
    and pole, spine, neck/head, tails, wings, measures) is its own bake
    product (`rig.profile`, `SFRP` v1), auto-detected from topology and
    tags with adapter hints/overrides; the humanoid `RigMap` is an optional
    layer that also yields a profile (`RigRef { Humanoid, Profile }`).
    Digitigrade legs use the `TwoBoneAnkle` solver; every graph op reads
    limbs from the profile.
  - **Thresholds scale with the rig**: foot contacts are measured in leg
    lengths (height 0.06, speed 0.35 per second), not absolute cm and m/s.
  - **Root attitude** is optional root motion: `RootDelta::tilt` and a
    per-clip attitude track (clip format 2) for flyers and swimmers; ground
    clips are unchanged. The mover accumulates it as `Locomotion::attitude`.
  - **Motion warping** follows UE's Motion Warping: `warp` event windows,
    skew warp (rank-one skew of the remaining root path onto the target)
    as default, scale as option, recomputed every update.
  - **Foot placement reach is enforced by lowering the whole body** by the
    worst shortfall after the per-girdle fit (exact in one pass, keeps the
    fitted pitch); re-pitching only the short girdle converged slowly and
    over-pitched downhill.
  - **Probe origins lead the animated effectors by one update**, and the
    controller casts them after the mover step, so a landing foot samples
    the step it lands on.
  - **Mover modes are UE CharacterMovement-style**: `MoverMode { Ground,
    Fly { hover }, Swim { surface } }` with 3D intent, plus a
    `RootMotion::traversal` flag (from `ActiveWarp::vertical`) that applies
    vertical root motion without snap or gravity. Hover feeds the ground
    slope forward. Shapes come from the profile
    (`MoverParams::from_profile`; upright capsule half the body length
    across for non-bipeds until M4's horizontal capsule).

- **2026-10-12: distant shadows through Bevy's `directional_shadow_factor`, no override.** Bevy 0.20's
  `PbrInput::directional_shadow_factor` multiplies each shadowed directional light's cascade term and is
  the whole shadow past the last cascade, so terrain, far-field proxies (now
  `ExtendedMaterial<StandardMaterial, FarShadowExt>`) and impostors set it to a terrain far-shadow term
  (clipmap horizon march near, CPU shadow-height map far). The cascade end fades by a dithered transition
  (pixels past `0.85·s_max` move their lighting position past the cascades). This drops weather.md's
  planned `FAR_SHADOW_LIGHTING_INPUT` scaffold. Cascades: 504 m derived from `FarFieldSettings`, 2048²
  (4096² cost +1.4 ms). Specs: weather.md, terrain.md, far-field.md, sky-lighting.md.

- 2026-10-12 (fbik; animation.md "Full-body IK as built"): full-body IK is
  our own XPBD particle solver (`sf_anim::ik::FullBody`, built per skeleton
  from the rig profile, stored in the compiled `FullBodyIk` op), not a crate
  (none fits: `lux-ik`/`skelly` unmaintained, `k`/Rapier are Jacobian robot
  or physics IK). Hybrid of UE5 PBIK (particles, stiffness, pre-pull) and
  Final IK FBBIK (body first, then each limb exactly by its own solver):
  limb ends constrain their limb root to a reach sphere instead of pinning
  the end particle, which made 10 iterations enough. Stiffness is
  parent-relative (world-space directions sheared girdles); limits are
  deviations from the animated pose until rig profiles carry authored
  ranges. Gate: 7.3 µs biped (≤ 8 µs met); 66-joint humanoid 11.3 µs and
  quadruped with a head effector 13.3 µs still over (subtree refreshes).
- 2026-10-12 (fbik2; animation.md "Full-body IK as built"): the FBIK body
  solve drops XPBD multipliers for compliance-weighted PBD steps relaxed by
  0.5. Measured: the multipliers grew linearly (equilibrium ~150 iterations
  away) and full steps oscillated with period 2 against the hard reach and
  length constraints, so the result depended on the iteration count's
  parity. Accuracy now comes from post-passes that do not need convergence:
  path-child swing for pinned non-limb effectors, a root shift by their
  mean residual, and one CCD sweep for heads and reaching limb roots.

- **2026-10-12 (nifpop; weather.md bug B): animated models get a rest-pose far proxy; runtime-imported
  models get ranges and preload through framework hooks.** `mesh.rest` (`sf_model::cook_rest`) is a
  separate product baked lazily by the far steps only when `mesh.cook` is `Skipped` (not a flag on
  `mesh.cook`: static models keep one product, and no other consumer sees a frozen animated model as
  cookable). `SubtreeRange` + `apply_subtree_ranges` (sf_world_bevy) put a placement's range on meshes a
  scene spawns later, pushed out by node offset; `PlacementSpawner::prepare` lets the assets stage wait
  on assets a spawner instances itself. Both are game-agnostic (any runtime importer path), so they live
  in the framework rather than `mw_assets` as the spec first proposed. B4 (split cook) stays planned.
  Owner decision, same day: DLSS defaults to `auto` in builds with DLSS (`--dlss off` and
  `BEVYRROWIND_DLSS` override; falls back to native where unsupported).

- **2026-10-12 (optclips; animation.md "Optional clips as built")**: graph clip references may be
  `(source, optional: true, fallback: Clip(name) | Drop)`; a plain string stays required. Required
  clips load directly (a missing one fails the compile as before); only optional ones are checked for
  availability (`CompileSource::available`, in the bake step `GraphRules::clip_files` through
  `RunCx::file_exists`, so a missing file is a recorded dependency and adding the source later
  recompiles). Drops are structural, at compile time, never at runtime: a dropped sample leaves its
  blend space, a node with nothing left is removed with its subtree, one with one input left becomes a
  pass-through, a dropped state takes its transitions with it, a dropped initial state or base layer is
  an error. Provenance (`source`, `license`, `redistributable`) is adapter data per clip slot
  (`GraphRules::provenance`, default: own, shareable content) and `CompiledGraph::resolutions` records
  which references were missing per clip name; exports use `non_redistributable()`. Format `SFAG` v3,
  step version 3.
- **2026-10-12 (spikes; streaming.md "Spikes")**: streaming spikes are render-thread spikes. At 1440p
  the GPU limits and the render thread is the critical CPU path, so the time-budgeted streaming stages
  were not the problem; costs downstream of them were. Decisions:
  - Work nobody sees pauses: particle emitters simulate and rewrite their meshes only in view
    (`ViewVisibility` + frustum sphere); every mesh rewrite is a Bevy slab free/alloc/upload.
  - A floating-origin shift is a one-off full re-collection in Bevy (every `GlobalTransform` changes);
    nothing else structural may share its frames: `StreamingConfig::origin_shift_guard_s` (0.05 s of
    outward travel before, 2 frames after, at most 6 frames) holds the main-thread streaming stages, the
    terrain moves its patches in place and delays clipmap uploads 2 frames. Cell size stays 256 m: the
    shift count per distance doesn't depend on the switching threshold.
  - Gate status: flight frames over 1.5× median 32 → 9 (3 runs), worst 1.67–1.86×; still failing.
    Bevy's full `VisibilityRange` re-walk (~1.1 ms per change frame) is the third upstream candidate
    (private `RenderVisibilityRanges`, no isolated workaround). Next: byte-paced mesh uploads.

- **2026-10-12 (audit; `specs/industry-audit.md`): process: industry gap audit.** The owner caught a
  basic technique no spec had: per-object draw distance from screen size (we drew every cup to 400 m).
  A system-by-system comparison against UE, Unity, Bethesda, RAGE, Decima, Frostbite and OpenMW found:
  - **Missing, P0:** size-based draw distance (`d_draw = clamp(K·2r / s_min · bias, d_floor, d_end)`,
    `s_min` 4/3/2/1 px Low→Ultra, L1 contributors keep `d1`) and size-tiered spawning (R1 detail tiers;
    a cell spawns only the radius-sorted prefix its distance can show; R0 activation split from
    visuals), as RAGE `lodDist`, UE runtime grids and OpenMW object paging do.
  - **Bevy 0.20, read from source:** CPU visibility is O(entities × views) every frame (ranges, camera,
    and each of the 4 cascades), and camera rotation re-extracts frustum enter/leave.
    **`NoCpuCulling` drops all of that, and the GPU preprocess still culls frustum and
    `VisibilityRange` in every view, cascades included,** so LOD switching and lead bands survive (J2
    A/B). Batching and instancing are already GPU-driven and incremental. Bevy has no screen-size
    culling and no disk pipeline cache.
  - **P1:** shadow-caster size rules, the prepass dither fix then occlusion A/B, precombined near
    cells (L0.5), pipeline warm-up, light distance fade, incremental `extract_visibility_ranges`
    upstream.
  - **P2:** texture streaming, terrain virtual texture, static shadow caching, horizon occlusion.
  - **Rule:** every spec gets an "Industry comparison" section: what the major engines do for that
    system, and why we match or differ (specs README template). Jobs J1–J9: ROADMAP Next up 6–7.
- **2026-10-12 (health; §8b "Gates (dev loop)", `bevyrrowind/docs/perf.md` "Dev loop"): dev-loop
  gates.** Compile and bake times now have numeric gates (no-op ≤ 1 s, touch app/leaf ≤ 5 s, touch
  hot crate ≤ 8 s, body edit ≤ 10 s, clean ≤ 12 min, cold bake ≤ 2 min, no-op bake ≤ 2.5 s), all
  met. Incremental `cargo dev` grew from M1's 2-4 s to 3.3-8 s with the graph (more crates between
  `sf_model` and the app), not from a config regression: the settings match Bevy 0.20's Windows
  fast-builds config. Superfargoth crates stay at opt-level 3 in bevyrrowind's dev profile (O1
  measured no faster). Warm bake 2.41 → 1.75 s: the bake report decoded every cooked model
  serially; now in parallel. Not done, written down: cold-bake parallel efficiency (~1.4x CPU/wall
  on 16 threads), `bake gc` (the store grew to 2.5 GB vs 0.8 GB live), 8 `bevy_dylib` variants
  (~146 MB each) in `target/debug` from per-command feature sets (the workspace-hack trigger in
  §8b, costs disk on C:, not time once built). Nightly (`-Zthreads`, cranelift) is an owner
  decision.

- **2026-10-12 (interiors): every worldspace is indexed; worldspaces own their render layer end to
  end.** The bake indexes each interior as its own `Single` worldspace from its `Worldspace` record
  (`WorldBake::with_overrides`; `bake_worldspaces` resolves all layouts in one parallel call: 1,328
  interiors cost +8.3 s cold, ~0.16 s no-op). Door link ends without a baked directory fail the bake
  (vanilla: 0 of 1,828 links). Prefetched worldspaces must stay invisible to the active view, so
  everything a worldspace spawns carries its layer: cooked parts at spawn, runtime-imported scenes via
  `SubtreeLayers` (sf_world_bevy), lights via `SpawnCx::layers`, terrain patches via the host, and the
  far field only for worldspaces listed in `sf_lod_bevy::FarWorldspaces` (from the manifest). A
  worldspace's look (sky, terrain, ambient, fog, exposure) is the content pack's and follows
  `ActiveWorldspace`. Industry check (streaming.md "Industry comparison"): Bethesda/OpenMW data model
  (separate interior spaces, teleport markers, preload near doors) with UE-style loaded-hidden vs
  active states; seamless crossing stays M8.
- **2026-10-12 (orca; locomotion.md "Local avoidance (ORCA)", industry-audit.md G2/M3-b): local
  avoidance is our own ORCA in `sf_controller::avoid`, not `dodgy_2d`.** Written from van den Berg
  et al. 2011 (no OpenMW code); runs on the intents' desired velocities after intents are written
  and before the mover (UE: RVO inside the movement component); `separate` stays the overlap
  fallback. `dodgy_2d` 0.5.5 was rejected: glam 0.29 + rand 0.9 duplicates against our 0.33/0.10
  (§8b), `rand::random()` tie-breaks (non-deterministic), per-call allocations, responsibility
  applied to the slack. Our rules: player `NonReactive`, idle agents give way (`yield_weight` 9),
  responsibility on required change and the opposite split on slack, opposing pairs yield by
  coarse `priority` or a fixed heading rule on the route direction (`AvoidAgent::travel`;
  per-character priorities jam doorways), keep-right bias, deterministic (batched = single bit for
  bit). Neighbours from a per-frame spatial hash. 150 agents 0.054–0.087 ms single-thread.
  Wall segments in the solver and corridor optimization wait for navmesh paths (M10).
- **2026-10-12 (slotnode; animation.md "Slots and montages", industry-audit.md G4/M3-d): gameplay
  one-shots are UE-style montages in `Slot` nodes of `sf_anim_graph`, not authored states.** Slots
  and slot groups compile into the graph (`SFAG` v4); clips come from the binding (`slot_clips`) or
  are late-bound at runtime (`LateClip`: events, curves and warp windows mapped by name). Blends are
  inertialization only (one pose per slot); the montage clock runs every update whether or not
  its slot is evaluated, and its events bypass the per-layer highest-weight rule (script timing
  never depends on graph state). Interrupt-with-priority or queue rules are framework; script verbs
  (Morrowind PlayGroup/LoopGroup flags and counts) map onto `SlotRequest` in the adapter.
- **2026-10-12 (animbake; animation.md "Bake steps", bevyrrowind bake-adapter.md "Animation"): adapter
  animation bakes run in two phases, and `AnimBake` memoizes imports.** Bake keys are fixed at plan time, but
  an adapter's clip rules (ranges of sub-actions, loop tails, which layered file holds a group) come from the
  source files' contents. The adapter therefore bakes a small "group table" product per source file and the
  skeletons first, builds its `AnimRules`/`GraphRules` from those products, then bakes `anim.cook` and
  `animgraph.compile` (staged dynamic dependencies, as in build systems with dynamic deps; warm, phase 1 is
  all hits). `GraphRules::clip_files` returns every file of the layer set, so a group added to a
  higher-priority file recompiles the graphs. `AnimBake` keeps a per-session import memo by (path, content
  digest): one legacy file holds ~100 clips, and without it each `anim.cook` re-parsed it. Graph assets ship as
  a builtin VFS layer mounted lowest (mods override). First measurement (Morrowind, 96 skeletons, 3,987 clips,
  96 graphs): cold 13.5 s, warm 0.31 s; `anim.cook` misses its 5 s gate because 29% of the clips never
  reach the 0.1 mm bound and try all three rates (open: convergence-based rate stop in the cook).

- **2026-10-12 (animbevy; specs/animation.md "Bevy integration / As built"): `sf_anim_bevy` evaluates one pose per
  skeleton in a `par_iter` over characters, then writes bones in a `par_iter` over bone entities.** The write-back
  mirrors `bevy_animation::animate_targets` (each bone reads its character's output through a read-only query: the
  only parallel, `unsafe`-free way to write many entities), but a bone only copies a precomputed `Trs` and writes
  only when it changed, so frozen (A3) and still bones stay out of Bevy 0.20's static-subtree propagation. Rigs are
  spawned from the cooked `Skeleton` (`spawn_rig`, bones named by joint), skins bind by name; `spawn_rest_skeleton`
  remains only for bind-pose tools. LOD tiers use projected height normalised to the spec's 1440 px reference (so
  thresholds hold at any resolution); the budget allocator is UE's "least significant first, measured cost" loop
  with a per-tier EMA.
- **2026-10-12 (animcook; specs/animation.md "Rate search, as built"): `anim.cook` rate search skips rates
  that can't pass.** Each candidate rate first measures the resampling error of its unquantized frames; above
  1.5x the bound (quantization adds at most half) it is not quantized or measured, measurements stop at the
  first sample over the bound, and a clip that passes nowhere keeps the rate with the lowest resampling error.
  Industry check: ACL and UE compress against raw frames sampled once at an import rate and never measure
  between-sample fidelity, so our search is stricter than standard; adopting the ACL split (rate from key
  density, resampling error reported, not searched) is logged as a proposal in the spec. `anim.cook` v3.
- **2026-10-12 (bakelock; specs/bake.md "Execution", Concurrency rule): `sf_bake` never blocks inside the rayon
  pool.** Root cause of the intermittent bake hang: each product was a `OnceLock` whose initializer ran a nested
  `par_iter` over its inputs; the waiting worker stole an unrelated job that needed a product it was itself
  initializing further down its stack, so `get_or_init` waited on itself. Now `bake`/`product` plan the reachable
  graph first and run it with a dependency-counting scheduler (`rayon::in_place_scope`, a product spawns when
  its last planned dependency finishes: cargo's job queue; Bazel/Buck2 likewise never park a worker on a
  dependency). Inline resolution (dynamic reads, cycle checks) is sequential and never waits: a product
  claimed elsewhere but unfinished is built again (deterministic steps, first result wins, counted in
  `StepStats::duplicates`). Rule for step-side memos: block only if the initializer is sequential code with no
  rayon and no baker calls. Outputs unchanged (1 vs N threads identical); regression test
  `sf_bake/tests/concurrency.rs` hung on the old code. `mw_bake` bakes clips before graphs (graphs read clips
  dynamically). `TerrainBake::product_stages` was a workaround for this bug and is now optional.
- **2026-10-12 (nodecreatures; specs/character.md "Node-animated creatures"): character bodies animated per node are
  rigid skins, not parented mesh entities.** `sf_model::cook_rig` (bake step `mesh.rig`) binds every rigid mesh at
  full weight to its nearest transform node (mesh nodes are never joints), so segmented bodies and the rigid parts
  of mixed ones become an ordinary `Cooked::Skinned` and reuse the per-material merge, outfit cache, swap, rig and
  graph; `CharacterContent` is unchanged (the adapter hands over `mesh.rig` for bodies; `mesh.cook` stays the
  world product). Chosen over rigid `Mesh3d` children of joint entities: in Bevy 0.20 a moving rigid part costs a
  render instance (visibility, culling, mesh-uniform extraction) and cannot merge per material, while a joint
  costs one `Mat4` in the skin storage buffer; the industry DCC path is the same (UE imports rigid animation as a
  skeletal mesh). Sockets with separate meshes stay for runtime-swapped held items. Morrowind adapter:
  `mw_anim::rig::skeleton_source` also takes root children holding unskinned geometry (storm atronach rocks),
  `NifAnimImporter` v3. Census: 89 CREA models, all cook; 1 (heart_akulakhan) keeps billboard parts on the root.
- **2026-10-12 (dremora): animation-driven effect curves are rest-neutral.** The graph outputs 0 for a curve no
  playing clip carries (`sf_anim_graph` output `curves`), so any curve an adapter feeds it must mean "rest" at 0, like morph
  weights (Unity "write defaults", UE default curve values): a clip layered from another file (Morrowind biped
  creatures idle on the humanoid file) must not switch effects. The Morrowind adapter encodes visibility as
  `hide:`/`show:` by the node's rest state (a shown-is-1 curve hid every dremora's armor). Absolute curves
  (material values) stay allowed only where every playable clip carries them; a real-install census asserts it.

- **2026-10-13 (clouds2, sky pass S1; weather.md "Cloud genera, scale, motion, shadows, sun
  occlusion")**: cloud presets are cloud **genera** (nine `CloudPreset`s: one volumetric low/mid
  genus + a 2-D cirrus sheet in the weather map's A channel), not "clear/fair/overcast/storm".
  Compressed worlds scale cloud **sizes and heights together** (`CloudLook::scaled`, powers of two
  so tiles keep dividing 16 384 m); real-scale defaults follow cloud-atlas sizes (2 km weather
  blobs). Framework winds stay physical; adapters set game-readable speeds (Bevyrrowind: `Cloud
  Speed` x 10 m/s). Shape evolution is two drifting weather-map octaves + a rising shape phase (no
  map-writing pass). Light cookies in Bevy 0.20 need `pbr_clustered_decals` as well as
  `pbr_light_textures` (otherwise silently ignored). The trace snaps its last 1 % of transmittance
  to zero so the HDR sun disk is fully hidden behind thick cloud.
- **2026-10-13 (clouds3, sky pass S1 budget; weather.md "Cloud genera ...", "S1 cost")**: the
  cloud shadow map is **toroidal and time-sliced**: the light texture repeats, the light sits at a
  world anchor that moves in 2 km cells, each texel holds its repeat nearest the camera, and one
  band of rows refreshes per frame (`sf_atmos::ShadowSlicer`, `CloudQuality::shadow_slices`, High
  16); a full redraw only on an anchor move, a light turn of > 4 m at the camera, or a weather
  jump. Primary march steps are a continuous function of the span (no integer step count dividing
  it: that drew bands), with 2x empty-space steps and a lighting LOD (2 light samples where T < 0.3
  or past 15 % of the reach). Every cloud pass is a `RenderDiagnosticsPlugin` span: per-pass GPU ms
  are the cloud A/B measure, not frame deltas (frames are CPU-bound on the shared machine).
- **2026-10-13 (m5-spec; specs/scripting.md, specs/mods.md, bevyrrowind scripting-adapter.md; docs only, owner
  confirmation pending on the items marked "owner")**: M5 scripting and mod pipeline.
  - **Runtime: Luau through `mlua` 0.12.2 directly** (Luau 0.740 via `luau0-src` 0.22.0), in a Bevy-free `sf_script`
    plus thin `sf_script_bevy`; features `luau`, `serialize`, `send` (confirmed by the L1 spike), not `async`, native
    codegen off by default. Not adopted: `bevy_mod_scripting` 0.22 (on Bevy 0.20, but reflection-first against §6.2,
    "expect significant API changes", mlua 0.11), `bevy_scriptum` (Bevy 0.18), `piccolo` (unmaintained), `rhai` (no
    modding ecosystem), WASM (heavy; stays §6.1's "later"). Owner.
  - **Challenges §6.2 "each mod gets its own environment": each mod gets its own Luau state** (Factorio's model), for
    per-mod memory limits, CPU accounting, reload and later parallelism; mods talk through next-slot events and
    `interfaces` with copied plain data. Exception: an adapter may declare one shared context for translated legacy
    scripts (they address each other by ID across plugins). Owner.
  - **Execution**: exclusive script slots (`Input`, `Update`, `Ui`), synchronous API with immediate writes,
    spawn/despawn flushed at handler end, script-to-script events delivered next slot in load order. No saved
    coroutines: waits are named timers with plain-data payloads (OpenMW's registered timer callbacks; avoids Papyrus's
    suspended-stack save bloat). Script handles are `sf_world::PlacementId`s; entity-script state lives in the
    placement overlay. Mod-shipped bytecode is never loaded (Luau doesn't verify bytecode).
  - **API**: a registry of modules with Luau type declarations (generates `sf.d.luau` for luau-lsp); require-by-string
    aliases `@sf/*` (framework), `@<adapter>/*` (content pack), `@<dependency>/*` (library code loaded into the
    requiring state). Framework API is `0.x` until M7's RPG modules exist (owner); content-pack APIs version through a
    dependency on the content-pack mod. ui.md's placeholder `require("sf.ui")` becomes `require("@sf/ui")`.
  - **Limits**: per-call watchdog through Luau's interrupt (50 ms release), suspension after 3 aborts or 30 errors a
    minute, 64 MiB per mod state (owner: defaults); budget ≤ 1 ms of script time per frame for a typical load.
  - **§6.3 confirmed with evidence**: MWScript is translated to Luau at bake time (1,231 script texts, 981 KB, plus
    17,144 dialogue results; control flow maps 1:1; one runtime for sandbox/limits/saves/reload), and the same Bevy-free
    translator runs live for the dev console. Local legacy scripts tick only in MWScript's active cells, independent of
    render streaming.
  - **Mods**: manifest gains `[optional]`, `[conflicts]`, `[ui]`; `profiles/<name>.toml`; one mod = one record source,
    one VFS group, one script context; native record files in RON with `set`/`merge`/`remove`/`append`, and conflicts
    become **field-level** (disjoint patches merge). A profile fingerprint in the bake manifest decides "bake needed".
  - **Challenges §5.1 `sf_vfs` "read-only and lock-free after the index is built"**: still true per snapshot, but
    hot reload rebuilds the index and swaps the shared `Arc<Vfs>` atomically (readers never block). Dev builds may
    re-bake an affected product during play (exception to "nothing bakes during play", dev/mod-author mode only; owner).
  - Minimal `sf_rpg` data (inventory stacks, quest store) is pulled into M5 so scripts and the inventory slice use real
    data; M7 extends it (owner). Ecosystem check 2026-10-13: `bevy_enhanced_input` 0.26, `leafwing-input-manager` 0.21,
    `bevy_flair` 0.8.1, `bevy_fluent` 0.15, `bevy_ui_anchor` 0.12 are still on Bevy 0.19.

## 11. Open decisions

- ~~Physics character controller: `bevy_tnua` vs our own.~~ Resolved 2026-10-12 (§10): our kinematic mover.
- INFO response order (prev/next linked lists across plugins): resolved by
  the dialogue system (M7), not at load.

- **2026-10-12 (clouds; weather.md "Clouds as built"): volumetric clouds land as `sf_atmos` (Bevy-free)
  + `sf_atmos_bevy` (render plugin).** Provenance: the owner licensed Megalophobia's `src/clouds.rs`,
  `src/cloud_shadow.rs`, `src/volumetrics.rs` and `assets/shaders/cloud_*.wgsl` as MIT OR Apache-2.0
  (owner-approved scavenge, 2026-10-12); the density/lighting math and noise generators are ported with
  provenance notes in the module docs, its third-party files (`src/gen/erosion.rs` MPL, meshes,
  imagery) are untouched. Decisions: (1) the weather map is a fixed tiling weather-noise texture
  thresholded per sample by `CloudWeather` scalars (one CPU/GPU twin function), not a compute-written
  map, until anchors or painted climates need one; (2) the cloud passes reuse Bevy's atmosphere by
  rebuilding `render_sky`'s bind group from bevy_pbr's public resources and importing
  `bevy_pbr::atmosphere::functions`; (3) the composite is ordered after `render_sky` by a named scaffold
  (`SkyOrderScaffold`: orders after every system that precedes `main_transparent_pass_3d`), because
  `render_sky` is private; (4) cloud shadows use a **tiled** `DirectionalLightTexture` (untiled reads
  black outside the tile) whose edge fades to the weather's mean cover; (5) floating origin: an app
  resource `CloudsOrigin`, per-texture offsets with the origin phase, history shifted by the origin
  delta. Measured at 1440p native (RTX 4060 Laptop): +0.62 ms clear to +0.97 ms fair cumulus, at the
  High budget before far fog; amortizing the shadow map and empty-space skipping are next.
- **2026-10-12 (probes; locomotion.md "Context probes as built")**: context probes are procedural
  queries over any `GroundProbe` (terrain + `StaticWorld` now, M4 physics later), not authored ledge
  markup: GASP's traversal uses ledge splines on its level blocks, but our content is Morrowind meshes
  with no such data, so we follow ALS's mantle check (forward capsule sweep, a down-cast column behind
  the face, room checks) and GASP's chooser ratios (vault ≤ 0.7 h and ≤ 0.33 h deep, mantle ≤ 1.5 h)
  scaled by `ContextParams::from_mover`. Every request is a small phase machine answered in ≤ 4
  batched probe calls for the whole batch; a probe returns data only (`Affordance`, lip, face normal,
  height, depth, clearance, landing) plus `warp_target`/`hang_point` in world space, which glue
  converts with `CharacterFrame::to_local` into the graph's `WarpTarget` (sf_controller still doesn't
  depend on sf_anim_graph). A taller-than-climb wall is detected by the column's top falling below the
  sweep's contact height (no extra ray). Cost: 5.3 µs per blocked obstacle probe (2 µs capsule sweep,
  ~0.45 µs per ray in parry), so 150 all-blocked characters are 0.79 ms single-thread, over the 0.2 ms
  probe budget; the spec's gating (A0 only, blocked only, ≤ 4 per character) bounds it to ≈ 0.21 ms
  with 40 A0 characters blocked at once. The Bevy plugin should chunk requests over the compute pool
  (`ContextProbes` is per-chunk scratch) and cache results while the target holds. FBIK: body-only
  model refreshes during the solve and one refresh of non-body joints at the end; no measurable gain
  (8.65 µs humanoid), because the limb pass's `solve_limb` refreshes the hand subtree per joint turned.
- **2026-10-12 (animimport; animation.md "BVH and FBX importers")**: BVH and FBX enter as `sf_anim`
  importer features (`bvh`, `fbx`), bake only. BVH is our own ~250-line parser: no Bevy loader exists,
  `bvh_anim` 0.4 sits on 2019 dependencies and `bvh_anim_parser` 1.0 has `cargo`/`regex` as runtime
  dependencies. FBX uses `ufbx` 0.11.5 (MIT OR Unlicense; a C library built by `cc`, so FBX bakes need a C
  compiler; fine on MSVC) and lets ufbx resolve the FBX transform chain and convert to meters / Y up /
  right-handed (`AdjustTransforms`), sampling each stack at the scene frame rate. Unit and axis
  conversion for formats without metadata is one framework type, `SourceSpace` (signed up/forward axes,
  handedness, unit scale), applied by conjugating local transforms with the change of basis; it is part
  of the bake key through the new `AnimImporter::settings_key` (empty by default: glTF keys unchanged),
  and BVH takes it per path prefix because one directory holds one dataset. Importers report a detected
  root-motion joint (`AnimSource::root_motion`: moving `root`, else the shallowest moving joint), used
  only when `SkeletonRules::detect_root_motion` is set, so no existing bake changes. Verified by a
  BVH → auto-map → Biped retarget (0.000 cm / 0.034°) and a CC0 FBX we generated with Blender.
- **2026-10-12 (mwanim; bevyrrowind actors-adapter.md "KF import as built")**: Morrowind animation enters
  through a new Bevy-free adapter crate `mw_anim` (on `mw_nif` + `sf_anim`), not through `mw_nif`'s API, so
  the app's dependency graph is unchanged (`mw_nif` only gained `load_kf` and `Curve::keys`). No framework
  change was needed: `AnimImporter` is the adapter hook. Each text-key group becomes its own `SourceClip`
  (cut from the file's single timeline with exact cubic splits) rather than one long take plus
  `ClipRules::range` per group, so clip names are group names and the importer output stands alone;
  sub-actions inside a group (weapon attacks, equip, spell casts, block) are adapter `Section`s that the
  bake turns into ranged `ClipRules`. Event names (`footstep`, `attack.hit`, `attack.charge` windows, ...)
  are the adapter's vocabulary; `sf_anim` keeps treating them as opaque strings. Root motion is on
  `Bip01` (measured), reported through `AnimSource::root_motion`.
- **2026-10-12 (mwrig; bevyrrowind actors-adapter.md "Rig maps as built")**: Morrowind skeletons are
  turned to face +Z at import (`mw_anim::rig::FACING_TURN`, 180° about Y on the root joint's rest), the
  option character.md names for adapters whose native clips face elsewhere: KF tracks animate `Bip01` and
  below and root motion is extracted in model space, so clips need no change, and auto-align and
  `sf_anim::rig` detection see the canonical facing (before, detection flipped every limb's side).
  Actor spawning adds the same half turn to a reference's yaw. Humanoid maps run `auto_map` on the `Bip01*`
  joints only (body-part sockets are not bones), with root motion forced to `Bip01`: all four vanilla
  humanoid skeletons map cleanly with that two-line hint table, so no framework change. Creatures use
  `detect_profile` plus per-model `RigHints` in the adapter (nix-hound, cliff racer, centurion spider);
  known framework gap left for a framework session: the support-leg walk doesn't stop at a joint that
  carries several leg branches (a spider's body joint).
- **2026-10-12 (pathgrid2; bevyrrowind actors-adapter.md "NPC life in M3 / As built")**: NPC wander before
  `sf_ai` lives in a new Bevy-free adapter crate `mw_ai` (not `mw_world`, whose API the app uses). PGRD
  pathgrids become per-cell `petgraph` `DiGraph<WorldPos, f32>`s in engine space at load (exterior points
  get the cell corner added, as OpenMW does), searched with `petgraph::algo::astar`; no navmesh (whether a baked navmesh
  replaces pathgrids is an M10 question). The AI_W rules (idle roll, wander goals within the
  radius of home, game-hour duration) are adapter code; the framework only sees `PathFollower` nodes and
  `LocomotionIntent`s from the named scaffold `PathWalkScaffold`, removed when `sf_ai` owns the intent.
  No framework change was needed: `sf_controller::PathFollower` was the hook.
- **2026-10-12 (mwactors; bevyrrowind actors-adapter.md "`CharacterSource` as built", character.md
  "Part selection and socket offsets")**: the Morrowind `CharacterSource` is a new Bevy-free adapter crate
  `mw_actors`. It needed one small framework hook in `sf_character`'s `BodyPart`, kept engine- and game-neutral:
  `node_filter: Option<String>` (only geometry under a node whose name contains the text belongs to the part,
  because one model file can hold several parts: both sides of a limb, a head and its hair) and
  `PartKind::Rigid { joint, offset: Affine3A }` (a socket offset, as UE sockets carry; a negative determinant
  mirrors the part, the cook flips winding). Reuse check: Bevy's labelled sub-assets (`file#Label`) address
  sub-parts at load time, but `BodyPart` is a Bevy-free record and `VfsPath` has no label syntax; no crate
  models modular character parts. Adding a label to `VfsPath` would touch every content path, so a field on the
  part record was the smaller change. Part records are built lazily by the adapter (implementing
  `PartLookup`, each part needs its NIF read to tell skinned from rigid), not defined into the `RecordDb` at
  load: about 2,300 parts, most never used in a session. Original part priorities map onto `Layer`s plus a worn
  order (resolve's "later wins" tie becomes "earlier slot wins" by reversing the slot order).
- **2026-10-12 (skins; bake.md "Skinned and morphed models", character.md "Skin data in `sf_model`")**: `sf_model`
  carries skins and morph targets. Source side: `MeshData::skin` (palette of node + glTF-style inverse bind,
  per-vertex influence lists of any length) and `MeshData::morph_targets` (dense deltas); skinned positions are in
  bind space, the mesh node's transform ignored (glTF rule). `mesh.cook` cooks any model with a skin or morph to
  the new `Cooked::Skinned(CookedSkinned)`: the usual `CookedModel` (bind-space geometry, LOD chain) plus per-submesh
  `SkinVertex { joints: [u16; 4], weights: [f32; 4] }` and target-major `MorphDelta` streams, a model-wide palette
  (`CookedJoint { name, inverse_bind }`, deduplicated by node + inverse bind) and one submesh per source mesh with its
  node path (no flatten), so `BodyPart::node_filter` works on cooked parts. Rigid meshes in such models, and rigid
  body parts via `CookedSkinned::rigid(cooked, joint, offset)`, become single-joint skins; **the mirror of a
  negative-determinant offset is owned by that Bevy-free helper (winding flipped in data)**, never by a negative
  `Transform` scale, because Bevy 0.20 doesn't flip culling per instance. LODs keep the static chain (from LOD0,
  attribute-weighted) plus a joint-center attribute (continuous "enforce bone boundaries") and a morph-magnitude
  attribute; levels index LOD0 vertices, so weights stay normalized. No influence reduction per LOD (Bevy always
  skins four). **Format**: `.sfmesh` v2 adds the variant at the end of the enum, so v1 bodies decode unchanged;
  files are written with the lowest version that holds them (static stays v1, byte-identical) and the bake key's
  format part (`KEY_FORMAT`) stays 1: no full re-cook. Weights are `f32` (not `unorm8` as character.md first
  planned) so the Bevy loader maps straight to `ATTRIBUTE_JOINT_WEIGHT`.
- **2026-10-12 (spikes4; diag.md "Frame pacing", streaming.md "Spikes"): the streaming gate counts presented
  stutters against a moving median.** WISHLIST §0's "worst frame < 1.5× median, none > 33 ms" is now measured
  as at most 1 stutter a minute (a presented frame over 1.5× the median of the previous 60 frames,
  `sf_diag::pacing::stutters`, `Gate::MaxStutters`, preset `Gate::wishlist_streaming_stutters()`) plus no
  frame over 33 ms, uncapped, in a borderless window at the display's resolution. Why: a single worst/median
  ratio is decided by one frame (fragile both ways) and a global median penalizes scenes that get heavier;
  benchmarkers (CapFrameX, Digital Foundry) count stutters against the recent frame rate and Unreal's hitch
  detector compares with recent frames too. Gating on work time (`frame/work/ms`, spikes3's plan) was
  rejected on data: at 60 Hz fifo the presented frames had 0 stutters while work counted 116–158 a minute
  against its own median (spikes inside the slack are invisible). Work counters (PresentMon-style busy/wait
  split, `sf_diag_bevy::FramePhasePlugin`) stay for attribution. The setup matters more than the window path:
  1440p pixels on this GPU hide render-thread spikes in GPU-bound waits whatever the window; borderless
  1080p uncapped shows them (flight 21–27, run 3–9 stutters a minute today: the gate fails).
- **2026-10-12 (nifskin; bevyrrowind nif-runtime.md "Skins and geometry morphs"): NIF skins map to glTF-style
  inverse binds by folding the skin's placement into each joint.** Gamebryo skins as `world(G)·S·world(R)⁻¹·
  Σ w·world(B)·D·v`; `sf_model` (glTF) as `Σ w·J·IB·p`. The adapter keeps the shape's raw vertices as bind space
  and sets `IB = J⁻¹·M·J·D` with `M = world(G)·S·world(R)⁻¹`: exact in every pose when `M = I` (shape and
  skeleton root where they were bound, the normal case), exact at rest otherwise (flagged, counted). Rejected:
  rebinding to the rest pose (`IB = J⁻¹`, positions = baked rest) — wrong whenever a body part's bundled
  skeleton isn't in the bind pose. Morph targets are per mesh in a NIF, so the adapter names them
  `"<node>#<i>"` (sf_model matches targets across meshes by name). The runtime NIF path keeps drawing the baked
  rest pose; a skinned model's root and every morphing node are marked animated so the app's static set never
  takes a `Cooked::Skinned`.
- **2026-10-12 (sizecull; industry-audit.md "J1 as built and measured", streaming.md, far-field.md): size-based
  draw distance and quality presets.** Every placement's chain ends at `min(d_end, LodScreen::draw_distance(r))`
  (`max(K·2r / min_pixels · bias, min_draw_distance)`, `K = H / 2tan(fov/2)` at the 1440p/70° reference) with a
  dithered fade over its last 10 %, through `SpawnCx::max_distance`/`end_band` so every spawner follows. Exempt:
  actors, doors, `PlacementFlags::NO_SIZE_CULL`, unknown bounds, and placements the cell's L1 proxy represents
  (`CellProxies::kept_radius`: the far field evaluates the L1 bake's keep rule, diameter ≥ 2 px at
  `max(d1 − lead, d1/4)`, with the loaded node's lead; the audit's "r ≥ 0.78 m, the clamp is a guard" was a
  radius/diameter slip, the clamp is real). App presets (`--quality`, env `BEVYRROWIND_QUALITY`): one bias for LOD
  selection, size culling and the far handoff `d1` = 400 m × bias (rings follow), plus the cutoff:
  Low 0.75 / 4 px / 16 m floor (d1 300), Medium 0.875 / 3 px / 20 m (d1 350), High 1 / 2 px / 24 m (d1 400,
  default, = `LodScreen::default()`), Ultra 1 / 1 px / 32 m (d1 400); `--off sizecull` A/B. Measured: on vanilla
  exteriors it culls 3 % of placements at High (8 % Low), entities unchanged, CPU floor −0.3 ms in Balmora,
  GPU and stutters unchanged. Kept as the standard rule (UE/Unity/Bethesda/OpenMW all have it; it matters for
  mods with clutter); not a perf lever for vanilla exteriors, which lowers J3's expected value.
- **2026-10-12 (m3a; bake.md "Morph layout", character.md "Morph-bearing parts", industry-audit.md G1/M3-a):
  morph targets never ride on merged meshes.** Bevy 0.20 stores morphs densely (48 B per vertex of the whole mesh
  per target, every target looped per vertex per pass), so: `MorphTarget::kind` (`Shape` | `Expression`, set by
  the adapter; the cook and assembly obey it); `.sfmesh` v3 gives each submesh its own ascending target list
  (`CookedSkinStreams::targets`), strips targets that move nothing (`MORPH_EPSILON` 1e-6) and keeps expression
  targets only on LOD 0-1 (`RUNTIME_MORPH_LODS` 2; shapes on all LODs); v2 is retired (re-cook, `KEY_FORMAT` 2).
  Assembly is Bevy-free in `sf_character::assemble` (sf_character now depends on sf_model): shape targets baked
  into vertices at the recipe's `ShapeWeights` (UE Mutable / FaceGen), submeshes that keep expression targets get
  their own mesh on the shared palette, everything else merges per material with no morph data; the merged-mesh
  cache key gains `ShapeWeights::key()`. The loader gives each morphing mesh `MeshMorphWeights::Value` gathered
  from one model-wide weight vector through the submesh's target list.
- **2026-10-12 (skinloader; bake.md "Bevy mapping" / "As built (skinloader)", animation.md "Bevy integration"):
  skinned models load into the same `CookedModelAsset` with `skin: Some(CookedSkin)`; static consumers treat
  them as uncooked. Spawning copies `bevy_gltf` 0.20 (its default `Dynamic` bounds policy:
  `generate_skinned_mesh_bounds` before vertex compression + `DynamicSkinnedMeshBounds`), joints bind by name
  (`resolve_joints`), and morph weights live model-wide in `ModelMorphWeights` on the root, gathered per mesh into
  `MeshMorphWeights::Value` by `sync_morph_weights` (the seam `sf_anim_bevy` writes). Over-limit models (joints,
  targets, morph texture) fail the load instead of truncating. Bevy 0.20 batches skinned/morphed meshes (MDI) on
  storage-buffer platforms, so per-material merging still pays. `spawn_rest_skeleton` is a named scaffold until the
  rig spawn lands.
- **2026-10-12 (doorfix; streaming.md "Residency cache for the worldspace just left").** The scheduler keeps a
  keep-only ghost of an anchor in the worldspace it just left (`SchedulerConfig::linger_secs`, 5 s, extended while a
  gameplay anchor stands near a link back), so return trips find the cells resident (door-engine cell cache); burst
  mode waits only for the active worldspace. `FadeTransition` writes `TravelArrived` when it moves the traveler so
  apps resync controllers that cache a pose; the framework stays unaware of any camera controller.
- **2026-10-12 (mwgraphs; bevyrrowind actors-adapter.md "Graph assets as built"): adapter graph templates are
  built in code and shipped as RON data, one machine shape for every actor.** Bevyrrowind's `mw_graphs` generates
  `mw_humanoid` / `mw_first_person` / `mw_creature` `*.animgraph.ron` (a golden test keeps the files equal to the
  builder; mods edit the RON). Clip references are adapter strings resolved by its `CompileSource` (`Group`,
  `Group@section`, `Group@tail`) and almost all are optional with fallback chains, so one template compiles on
  every skeleton of its family and the compile's `resolutions` are the coverage report. Weapon stances are nested
  machines per movement state (UE Lyra's linked layers); upper-body actions and scripted groups are slots; hits,
  knockdown and death are base-machine states. Validation is headless (`mw_graphs::sim::Sim`: parameter scripts →
  states, clips, events); the viewer only confirms visually. No framework change was needed.
- **2026-10-12 (portal spike; specs/portals.md): M8 portals are render-to-texture portal views, built from Bevy
  0.20's own pieces.** One portal camera per open portal renders the far worldspace (its render layer only) from
  `DoorLink::cross` of the main pose into an HDR image, clipped by Bevy's oblique `near_clip_plane` at the far door;
  the opening is a quad on the door slab's front face (Morrowind doors lie on solid walls) sampling it in screen
  space; the crossing re-parents 0.15 m ahead of the quad, before the near plane reaches it. Stencil portals need a
  sub-view concept Bevy lacks. Lights, ambient, env maps, fog and exposure confine per view through render layers and
  camera components; light probes don't (upstream gap), and nothing may light layer 0. Measured +1.5 ms at 1440p
  per open interior view. `bevy_basic_portals` is stale (Bevy 0.13) and NVPL-licensed: not used.
- **2026-10-12 (portal2; specs/portals.md "Owner findings and fixes"): exposure belongs to the viewer, and portals
  share one grounded opening.** Sky and interiors already share one physical light scale (noon EV100 ~14, interiors
  7, night ~0.5; no calibration change); with two worldspaces on screen, every camera renders at one base EV100 (7)
  and the player camera's Bevy `AutoExposure` (compensation curve reproducing the schedule: night -4.5, interior -3.6,
  day -2.3 EV; 6/4 EV/s) adapts the whole frame, opening included. A fixed base, not the schedule, because Bevy
  meters the already-exposed HDR target and indexes the curve by it (no pre-exposure: upstream wish). Portal pairs
  use the per-dimension minimum opening (a bigger slab stays closed around it) with sills grounded on the floor (downward rays against the cell's
  statics), so the rigid crossing maps opening onto opening. Crossing is by position only, with the trigger mirrored
  after each crossing (no cooldown). Fixed on the way: the sun moved onto interior layers for a frame, and a new
  `SkyCamera` showed `Exposure::default()` for a frame (both white flashes).
- **2026-10-12 (charbevy): characters are a component plus an adapter hook.** `sf_character_bevy` spawns from a
  `Character` component (base record + `CharacterState`, or a recipe) instead of a `spawn_character` function, and
  reads everything game-specific through one `CharacterContent` trait object (recipes, part lookup, slots, decoded
  cooked products, animation asset, material mapping, placement facing turn). Resolve, part loading and
  `sf_character::assemble` run on `AsyncComputeTaskPool`; the main thread only uploads and spawns. Merged outfits are
  cached by (skeleton, outfit key, LOD, shape key) and shared by `Arc`; swaps wait for the new outfit to settle
  (2 frames) and replace meshes in one flush, keeping the rig. Actor placements spawn through `CharacterSpawner`
  (feature `world`, an `sf_world_bevy` `PlacementSpawner`), so no game names enter the framework. Details:
  character.md "As built (`sf_character_bevy`)".
- **2026-10-12 (walkers): the mover owns the transform; avoidance sits between intents and the mover.**
  `sf_controller_bevy` runs one Step system per frame, ORCA over every mover (plus host-updated
  non-reactive `AvoidObstacle`s: camera, player) then `KinematicMover::step_batch`, then writes root
  transforms; animation is fed from the mover in `AnimSystems::Probe` (UE CMC + RVO / Unity NavMeshAgent
  "agent drives, animation follows"; `Drive` root motion returns through `Mover::root`). The plugin is
  generic over a `GroundProbe` resource the host keeps current, positions are `WorldPos` with a per-entity
  parent-frame origin, and graph parameter names are adapter data (`AnimParamNames`), so the crate names no
  game and depends on neither streaming nor `big_space`. Static collision for the mover comes from R0
  activation hooks wrapping the placement spawners (no physics colliders before avian, M4). Details:
  locomotion.md "Bevy integration: `sf_controller_bevy` as built".
- **2026-10-12 (walkers2; locomotion.md "Processing range", streaming.md "Decided"): actor processing range =
  R0 coverage.** An actor simulates only in the active worldspace and only while every streaming cell within a
  small margin of it is R0-active (`sf_world::Scheduler::covers`), because R0 activation is what builds static
  colliders: the activation ring and the collision ring are one ring by construction, and no actor moves without
  ground and statics around it. Elsewhere it carries `sf_controller_bevy::Dormant` (parked, not stepped, no
  avoidance, zero motion fed, no foot probes; the host pauses its driver and lowers its animation significance)
  and resumes in place. Industry: OpenMW's actors processing range (no AI/animation/physics beyond it),
  Bethesda's AI process levels, UE significance + animation budget allocator. The host's ground is per
  worldspace (interior coordinates overlap the exterior's; door prefetch activates other worldspaces' R0).
  `Mover::body_scale` carries the race scale into foot probes and the motion feed. An off-screen schedule tier
  (Bethesda low process) waits for `sf_ai` (M10).
- **2026-10-12 (smoothbody; specs/smooth-bodies.md): smooth bodies ON by default, vanilla segmented as an option
  (owner decision).** Segmented characters (rigid limb, armor and clothing pieces on single joints) get their
  weights re-solved automatically, with no new art: `sf_character::smooth_body` (feature `smooth`) welds seams
  between parts (< 2 mm: one position, normal and weight set), links skeleton-adjacent parts within 4 cm, and
  inpaints weights harmonically in an 8 cm band around the seams (rigid mid-segment, blended at the joint);
  `transfer_weights` copies body weights onto close-fitting targets (closest point + normal test, inpainting
  the rest; Abdrashitov et al. 2023, the method behind UE's TransferSkinWeights). Measured finding: for bulky
  rigid armor, transfer from the naked body is worse than vanilla, so the default is `smooth_body` over the
  **visible outfit as one segmented set** (elbow 90° seam opening 56.5 → 1.6 mm, walk 37 → 7 mm, no new
  pokes); transfer is for hair and skinned cloth. Seam-free (already skinned) bodies such as Better Bodies
  pass through unchanged. Because player equipment combinations are unbounded, the step runs where the
  outfit is assembled (off-thread build task, cached by outfit key + mode), with an optional bake pre-warm
  for NPC outfits keyed the same way. No solver crate (Gauss-Seidel at ~1k unknowns, ~15 ms); parry3d for
  closest points. Live A/B (`--bodies vanilla|smooth`) is the next step.
- **2026-10-12 (actorfx): morph playback by name, two sources.** Character morph targets are driven by
  clip curves of the same name (graph-blended, as glTF/UE) and by an externally timed `MorphCurvePlayer`
  (faces), applied in `sf_character_bevy::drive_character_morphs` after `AnimSystems::Apply`, writing
  `MeshMorphWeights` only on change. `sf_anim::AnimImporter` gains `companion`/`import_with` (default off)
  so an importer can read a sibling file recorded as a bake input (Morrowind: morph curves live in
  `x<model>.nif`, bones in `x<model>.kf`). Morrowind blink/talk rules stay in bevyrrowind (`faces.rs`).
- **2026-10-12 (actorfx2): actor material/visibility effects are clip curves, applied by the adapter.**
  Effects keyed on an actor's animation timeline (Morrowind: visibility and material controllers of
  creature NIFs) import as named float curves in the clips, like morph weights, so the graph blends them
  with the pose; the content pack writes them after `AnimSystems::Apply` (hide = joint scale 0, as UE's
  `HideBoneByName`; materials on a per-character copy). The framework only exposes
  `OutfitMesh::material_name`. Camera-facing joints (billboards) are ordinary skeleton joints that the
  adapter rotates after the pose. A generic `sf_character_bevy` curve → material binding can replace the
  adapter code once a second content pack needs it.
- **2026-10-12 (smoothlive; specs/smooth-bodies.md "Live"): smooth bodies run at outfit assembly, guarded,
  as a cached look mode.** `sf_character_bevy`'s build task runs `sf_character::smooth_outfit` (one
  `smooth_body` over the visible parts, then `guard_seams` over the content's probe poses, so no seam opens
  wider than authored) before `assemble`, which now takes pre-decoded parts (`AssemblyPart::decoded`):
  rewritten parts are written back in rest space on a `rest⁻¹` palette. Heads with morph targets and hair
  stay as authored. The mode (`SmoothBodies`, default `Smooth`) lives in the `CharacterSettings` resource
  and in `OutfitCacheKey`, so both looks are separate cache entries; changing it rebuilds every outfit
  through the existing no-pop swap. Probe poses are adapter data (`CharacterContent::smooth_probes`; none:
  the outfit stays authored). An `outfit.smooth` bake pre-warm for NPC outfits remains optional.
- **2026-10-12 (helditems; specs/character.md "Held items"): held items are socket mounts per readiness, outside
  the outfit.** `HeldItem { item, model, hand, drawn: Option<Mount>, sheathed: Option<Mount>, light }` with
  `Mount { socket, offset }` (engine-socket model: joint + relative transform). Drawn/sheathed is runtime
  `Readiness`, not recipe data, so drawing a weapon re-mounts props without resolving or rebuilding; each
  mount may be empty (games without visible sheaths, an off-hand item hidden next to a drawn two-hander).
  Props never merge into the outfit mesh (they change often and later carry lights, trails, physics), so
  the outfit key ignores them. Carried lights are `HeldLight` (attach node, color, radius); the host
  calibrates intensity. Game rules (which weapon, when a torch comes out, which stance a weapon type
  maps to) stay in the adapter (`mw_actors::held`, `mw_graphs::Stance::of_weapon`). `CharacterState`
  dropped `Eq`.

- **2026-10-12 (animfix): locomotion graphs are fed measured motion; foot placement is guarded.**
  `sf_controller_bevy` feeds `Mover::measured` (the step's displacement over dt after avoidance and
  collision) and its yaw rate, never the intent, with hysteresis on moving/running thresholds the
  adapter sets (walk vs run by speed). `FootPlacement` carries `FootGuards` (step-height rejection,
  girdle drop/rise limits, unplant radius, reach clamp, critically damped weight that blends out
  without ground samples or above a speed, teleport and NaN resets), all lengths as fractions of leg
  reach; `GraphOutput::feet` / `AnimStats::feet_total` count what fired. Rationale: an unguarded foot
  lock drifting past leg reach pulled the pelvis to the ground (walkers). Graph format 5.

- **2026-10-12 (terrainblend2): object-terrain blending samples the terrain clipmaps from object
  shaders (no runtime virtual texture).** Same family as UE's RVT mesh blend and CryEngine 5's
  terrain layer blending into objects; our camera-centered height/layer/macro clipmaps already hold
  what an RVT would. `GroundBlendMaterial = ExtendedMaterial<StandardMaterial, GroundBlendExt>`
  (`sf_terrain_bevy`): the terrain's bindings moved to 100..112 so an extension can bind them beside
  `StandardMaterial`'s; the terrain's surface code is one shared WESL function (`surface.wesl`,
  `terrain_surface`) used by the terrain and the blend. Change from the RVT approach: on steep object
  faces the ground layers project triplanar (oriented by the object's normal) instead of top-down,
  because the top-down projection showed vertical streaks on Balmora wall bases. Opt-in per model by
  an adapter rule (`sf_world_bevy::GroundBlendModels`, evaluated once per template); the renderer
  swaps marked parts via `GroundBlendPlugin<M>` (generic marker, so `sf_world_bevy` and
  `sf_terrain_bevy` stay independent) only in the terrain's worldspace. Not adopted: pixel depth
  offset + dither (loses early-Z, noisy under DLSS). Excluded: far-field proxies (vertex-colour bake
  later) and Solari `--rt` (its RT scene reads plain `StandardMaterial`s).
- **2026-10-12 (heldrender): held items are separate prop entities on socket joints, never merged
  into the outfit.** `sf_character_bevy::held` parents one rigid prop per `HeldItem` to its mount's
  joint (transform propagation follows the animation), meshes shared per model so Bevy's GPU
  preprocessing batches identical props across characters. Merging would key outfits by loadout and
  re-assemble on every readiness/torch/loot change. Readiness (`CharacterReadiness`) re-mounts or
  hides; held-only recipe changes keep the outfit meshes. Lights are host-owned: the framework spawns
  a `HeldLightSlot` at `CharacterContent::node_position`, the adapter adds its calibrated, animated,
  budgeted light. Rig joints get visibility components only on chains holding props.
- **2026-10-12 (actorparticles): skinned held props get their own small skeleton; actor particles
  anchor at joints.** A `Cooked::Skinned` held model (bow) spawns one joint entity per palette name
  under the prop root at its rest transform (`CharacterContent::node_transforms`) and skins to them,
  so it follows the socket like a rigid prop and clips can later drive its joints by name (UE: a
  skeletal-mesh component on a socket). Particle emitters of actor models and held props (host side,
  bevyrrowind) are entities under the nearest skeleton joint or prop root; world-space systems carry
  their particles by the node's frame-to-frame motion instead of living in world entities (no
  floating-origin bookkeeping). Carried lights keep a horizontal clearance from the carrier and a
  reduced intensity because Bevy has no per-light near-field clamp.
- **2026-10-12 (physics-spec; specs/physics.md, bevyrrowind physics-adapter.md): M4 physics backend is `rapier3d`
  0.36.1 used directly, behind a Bevy-free `sf_physics`, not `avian3d` and not `bevy_rapier3d`** (supersedes §4's
  "Physics: avian3d" row and §7.3's "Avian"). Evidence (crates.io/GitHub, 2026-10-12): avian main (0.8.0-dev) is still
  on Bevy 0.19; open PR #1070 ports it to 0.20.0-rc.1 in ~100 changed `src/` lines but needs git forks of
  `bevy_heavy`, `bevy_transform_interpolation` and `glam_matrix_extras`; avian's joints have motors only on revolute
  and prismatic joints (no spherical/6-DOF motor) and its release uses parry 0.27. `bevy_rapier3d` 0.37 is on Bevy 0.20
  but syncs bodies on `Changed<GlobalTransform>`, which big_space rewrites for every entity on an origin cell change
  (a teleport of every body each shift). `rapier3d` has no Bevy dependency, uses **the same parry3d 0.31.1 as our M3
  probes** (`collision.cook` shapes and `StaticShape` trimeshes go in unchanged), per-axis motors on generic joints,
  reduced-coordinate multibodies, implicit acceleration-based motors (stable PD for powered ragdolls), and soft bodies
  since 0.36. Physics runs in an f32 frame around an anchor cell, rebased by whole 256 m cells when the player is
  512 m away (PhysX `shiftOrigin` style), independent of the render origin; only R0 simulates; bodies freeze into
  `PlacementOverlay` at R0's edge. Fixed 60 Hz with interpolation; held items interpolate in holder space (the
  carry-vibration fix). **Controller (resolves §11): keep `sf_controller`'s kinematic mover**, at frame rate, with a
  kinematic capsule body and capped push impulses (UE CMC, Jolt `CharacterVirtual`); `bevy_tnua` (Bevy 0.19, a dynamic
  floating capsule) not adopted. **Cloth for M4 is a bone lattice** (auto-placed cloth joints on robe/skirt hems,
  XPBD with tethers, max distance and leg capsules, in character space) running as an animation op and rendered by
  the existing skinning; mesh cloth for capes/banners (rapier soft bodies or GPU XPBD) is after M4. Avian 0.8 stays
  the fallback if the P1 spike misses the physics.md gates; the swap would touch only `sf_physics`.
- **2026-10-12 (physics-p1): owner confirmed the five M4 physics decisions** of the physics-spec entry above:
  (1) `rapier3d` directly, not Avian; (2) cloth is a bone lattice for robes and skirts in M4, mesh cloth later;
  (3) items are physical, asleep until touched, NPCs nudge small items, tap Activate picks up and hold grabs;
  (4) item mass factor and holder limits are calibrated by eye later; (5) unsupported items stay fixed until grabbed
  (`HOLD_UNTIL_GRABBED`). **Owner addition:** carry and throw limits follow the holder's strength stat (adapter
  `HolderLimits { carry_force, carry_mass, drag_mass, throw_impulse }`; Morrowind maps Strength), and objects too heavy
  to carry are **dragged** (ball joint at the grab point, holder slowed), including dead bodies dragged by the grabbed
  limb (physics.md "Carry and throw", task P6).
- **2026-10-12 (rootmotion): one root-motion rule for every actor.** Locomotion travel always lives on the
  clip's root-motion track (`anim.cook` extraction), never in the bones; the adapter names the joint (Bevyrrowind:
  the bones root, `Bip01`, else `Root Bone`). The world's mover drives and the track only sets the playback rate
  (`Match`); viewers store it on `AnimRootMotion` without applying it unless asked. A real-data census per adapter
  (every walk/run clip: distance extracted vs. travel left in the bones, with a no-extraction control) guards it.
  Cause of the owner's NC-1 note: 15 Morrowind creature skeletons travel on `Root Bone`, which had no root-motion
  joint, so they walked off and snapped back while the `Bip01` ones stepped in place.
- **2026-10-12 (physics-p1): the M4 P1 spike confirms `rapier3d`; ragdolls lean to impulse joints.** On an R0-sized
  world (2,400 statics, 64 terrain chunks, 600 sleeping items; release, shared machine) the idle step is 0.001 ms, a
  whole-cell rebase 1.4 ms including the next step, and a 50-item pile plus two 15-body ragdolls 0.44 ms with impulse
  joints, all inside the physics.md gates; multibodies miss (3–4 ms, one 450 ms hitch), so avian is not reconsidered
  and P7 starts from impulse joints. Rapier wakes the partners of every new contact and every moved collider, so
  `sf_physics` settles sleeping bodies (one locked step, re-slept by the engine) on insert and on rebase.
- 2026-10-12 (seams): seam shading (proximity normal blending, `blend_seam_normals`) is part of `SmoothBodies::Smooth`, not applied in Vanilla, so B keeps showing the authored look; colour seams between differently textured pieces wait for an owner decision (smooth-bodies.md "Seam shading").

- 2026-10-13 (owner): night exposure: auto-exposure gets a clamped range (a night EV floor) instead of exposure-scaled local lights; local lights stay in physical units calibrated once (LightCalibration). Option B (lights scaled by exposure) rejected as non-physical. Implementation: lighting job in ROADMAP "Next up" 6b.

- 2026-10-13 (owner): milestones renumbered: RPG core (dialogue, journal, stats/rules, magic, AI, saves) moves from M10+ to M7; far-field perf -> M8, seamless interiors -> M9, combat -> M10. Priority order: M4 physics, M5 mods+Lua+UI, M6 water+weather (§9).

- 2026-10-13 (lighting): exposure range + local-light shadow budget. `sf_sky::ExposureRange` (UE's Min/Max EV100) clamps the camera EV100 for both drivers (schedule; histogram auto exposure via a clamped compensation curve, since Bevy's `AutoExposure` has no output clamp); the framework default is wide, the game sets the floor from its light calibration (Bevyrrowind EV100 3.5: lights read at most 11x their EV-7 calibration) and buys night readability with a brighter night ambient, not a wide-open camera. Point-light shadows go through `sf_sky_bevy::PointShadowBudget` (opt-in `BudgetedShadow`; N lights by distance over priority with hysteresis; distance and time fade by splitting the light into a shadowed and an unshadowed part, Bevy having no per-light shadow strength). Measured cost is render-thread CPU per caster in each light's range (Balmora night, 1440p: +0.8 ms for one light, +3.9 ms for two), so presets default to 1 (High) / 4 (Ultra); caster culling for local-light shadow views is the follow-up that would raise them.

- 2026-10-13 (orchestrator, owner asleep): M5 spec decisions 1-6 (bevyrrowind ROADMAP "## M5", scripting.md) adopted provisionally with the spec's recommendations (Luau via mlua, one state per mod + shared legacy state; bsn! templates until .bsn files; minimal sf_rpg in M5; dev-only re-bake during play; fault defaults; API 0.x until M7). Decision 7 (ESP field merge timing) left open. All pending owner confirmation; L1 spike numbers inform decision 1.

- 2026-10-13 (nightsky): drawn sky bodies separate from lighting. `sf_sky::Moon` (N moons, each with its own phase cycle, optional discrete phase steps/textures, `MoonPath::Synodic` (physical) or `Circuit` (fantasy: own period, tilt, declination)), `StarField` (procedural, per-star illuminance spread over a pixel) and `SunDiskLook` (artistic disk size + glare; the light keeps the physical 0.53°). One moonlight sums all moons toward the brightest at the ground. Drawn by `sf_sky_bevy`'s night pass: fullscreen at the far plane, depth-tested to sky pixels, additive after `render_sky` and before the cloud composite (same `SkyOrderScaffold` discovery; `SkyPlugin` must be added before cloud plugins), so air and clouds dim and hide them. Bevy's `SunDisk` stays for the sun only (moons' are zero-intensity). Also: cloud lighting picks the key light by its light at the camera and applies the planet's shadow (Bevy's transmittance LUT clamps below-horizon rays to the horizon, which lit high clouds red at night).

- 2026-10-13 (m5-l1; scripting.md "Spike results (L1)"): `sf_script` core built; spike confirms M5 decision 1 (Luau via mlua 0.12.2, state per mod). Decided by measurement: mlua `send` stays on (an `sf_script` feature, calls within noise of `NonSend`); world access for API functions goes through an app-data guard, not `Lua::scope` (scope costs ~0.5 µs per scoped function per slot); `luau-jit` off (API-bound handlers gain nothing, start 2.4x slower). Corrected budgets: empty context <= 512 KiB (a bare mlua Luau state is 378 KiB), script state save/restore <= 40 ms/MiB each way (mlua per-value cost). Watchdog: one deadline per host entry into a context, clock sampled every 64th safepoint, latched once fired (a `pcall` loop could otherwise outlive it); its real cost is mlua's interrupt trampoline (~25 ns per safepoint). Host-side bulk conversion lifts the per-state memory limit temporarily (mlua protects every allocation while a limit is set: 2.6x slower). Script state uses our own `StateValue` walker (canonical key order, error paths) rather than mlua serde.

- **2026-10-13 (m6-spec; specs/water.md, bevyrrowind water-adapter.md, ROADMAP "## M6"): water design.**
  (1) **Own `sf_water` (Bevy-free) + `sf_water_bevy`, porting bevy-aqua's wave, foam and optics code** (MIT OR Apache-2.0,
  notices kept), not bevy-aqua as a dependency or fork: 0.1.3 targets Bevy 0.19 (no 0.20 branch, last commit 2026-09-23,
  one maintainer), samples world XZ in `f32` (no floating origin), takes one global bed image and one sea level, answers
  queries by GPU readback, and reflects through planar mirror cameras; most of its integration surface would be
  rewritten. Revisit if it ships on 0.20 with origin-relative coordinates and a CPU query. (2) **Ocean = Tessendorf FFT
  cascades with power-of-two tile sizes** (JONSWAP/TMA, Horvath 2015) on a geometry clipmap, sampled at `rel + phase`
  from integer origin cells (exact across shifts); weather transitions blend `h0` (the FFT is linear). (3) **The CPU
  water query (§10 2026-10-11 decision 4) is a CPU twin of the long cascade(s)** (`rustfft`, same `h0`, 60 Hz),
  plus bodies in a BVH and baked flow maps; short cascades are visual only (HDRP "evaluate ripples" off). (4) **Water
  draws in its own pass after the sky/cloud/fog composite from a colour + depth snapshot** (UE Single Layer Water), not
  Bevy transmission (colour only, unordered vs `render_sky`, no motion vectors); it depth-writes and redraws into the
  motion-vector texture for DLSS. (5) **Reflections: SSPR + a camera-centred dynamic reflection probe + environment map**; no planar cameras; Bevy SSR
  only if the deferred-prepass question resolves; RT reflections from the water pass later (Solari BLASes are
  opaque-only, so water stays raster on the RT path). (6) **Water bodies (ocean, lake, river, pool) are records**;
  `Worldspace::sea_level` stays the shorthand for an ocean/pool; a water-level clipmap (UE's water info texture) feeds
  the surface hook, precipitation, froxels and grass. (7) **Rivers: authored, or baked as flow** (`water.hydro`
  priority-flood + D8, `water.flow` geodesic direction + discharge routing); generating new river bodies is a content
  policy (`Generate`), off for Bevyrrowind (`FlowOnly`). (8) **Buoyancy = pontoon spheres from a `water.pontoons` bake
  over `body.cook`** (UE/Crest), per-triangle hull clipping (Kerner) as a per-body option; `sf_physics` only gains
  `add_force_at` and a `Floating` wake rule. (9) Budget: water ≈ 1.35 ms High at 1440p on top of weather's ≈ 3.3 ms.
  Owner decisions pending (ROADMAP "M6" list 1-7: approach, reflections, moored boats, rivers, budget, swamp look,
  skipping N3). HD grass gets its own spec (G0) before it is built.

- 2026-10-13 (orchestrator, owner asleep): M6 spec decisions 1-7 (bevyrrowind ROADMAP "## M6", water.md) adopted provisionally with the spec's recommendations (own sf_water porting bevy-aqua code; SS planar + cubemap reflections; moored small boats bob; no new rivers; water budget ~1.35 ms High; murky Bitter Coast; froxel fog without the Bevy god-ray prototype). Pending owner confirmation.

- 2026-10-13 (m5-a1): `sf_mods` pipeline core built (mods.md "As built (A1)"). Lasting choices: (1) manifest content
  paths (`records`, `scripts`, `ui.styles`) are VFS paths inside the mod's own asset roots, and record files are read
  from the mod's own layers; (2) `load_profile` drops a mod that can't load (and, by re-resolving, its dependents) with
  an error instead of failing the profile (Factorio semantics, incl. an enabled optional dependency with the wrong
  version); (3) `ContentAdapter::mount` returns layers and record formats route by extension, so the native RON loader
  (A2) is just another adapter; (4) script limits live in the profile only, never the manifest; (5) `sf_mods` hands
  scripts a plain `ScriptPlan` and does not depend on `sf_script`.
- 2026-10-13 (m6-f1): weather haze/dust on the atmosphere medium: `MediumTerm.weather` (`WeatherScale` None/Haze/Dust) replaces `weather_scaled`; `Weather::dust` + `dust_tint` scale an absorbing dust term in `AtmosphereModel::earth`; medium rebuilds are step- and time-limited (0.25 s). weather.md N1's "haze 2-3" is recalibrated: compressed worlds (1-5 km views) need haze ~20 (AOD ~0.18) for any aerial perspective; near fog at game distances belongs to analytic height fog (F2), not more haze.
- 2026-10-13 (m5-a2): native record files built (mods.md "As built (A2)"). Lasting choices: (1) generic patches run on
  our own serde value tree (`sf_records::Value`) whose RON leaves stay raw text until the record type deserializes
  them, because `ron::Value` drops enum variants and `serde_json::Value`/JSON Patch crates can't hold enums, tuples or
  non-string keys; (2) `register` stays serde-free and `register_serde` opts a kind into by-name writes and the default
  fingerprint (blake3 of postcard bytes); (3) a touch records the field paths it wrote, and a blind override is a
  conflict only when writes overlap (path, parent path, or whole record), else an automatic merge (TES3Merge/bashed
  patch behaviour built in); define/delete/closure patches count as whole-record writes; (4) a record file's syntax
  error rejects the file, but each failing operation is its own positioned error and the rest apply; (5) conflicts and
  merges become per-mod `ModReport` issues on the winning mod.
- 2026-10-13 (m6-f2): height fog + ground-fog banks + far fog live in the cloud node (`sf_atmos::fog` settings and
  CPU twin beside the cloud settings, not in `sf_sky::Weather`: the node reads `sf_atmos` only; WE3 drives both).
  Technique: UE-style analytic exponential layers applied per full-resolution pixel to its own depth, the bank
  layer scaled by a noise *modulation* raymarched at the clouds' reduced resolution (depth edges exact, no froxels
  needed for F2); per-frame lighting in a 66×1 texture. F3's froxels take over the near range with the same density
  terms. The sea scaffold is opaque so the pre-transparent composite fogs it; transparents wait for F3.
- **2026-10-13 (physics-p6): carry is jitter-free by sampling the view at step time; the movers keep analytic
  terrain.** A held item's hand target is the holder's view interpolated/extrapolated to each fixed step's time, and
  the item renders relative to the current view (holder-space interpolation); a dragged object's force cap is what
  slides the adapter's drag mass, not the carry grip; the holder is excluded per contact pair (rapier filter hook),
  not by layer. The controller's ground on physics composes `TerrainProbe` with the bodies' `STATIC | PROP`:
  heightfield casts cost ~13 µs each (physics.md "Queries", "Carry and throw" as built).
- 2026-10-13 (m6-f3b, M6 F3): **Froxel volumetric fog is ours; Bevy's `VolumetricFog` stays unused** (re-checked on
  0.20: per-pixel per-box raymarch, no temporal, transparents can't sample it, no height/bank density or terrain/cloud
  shadow terms). `sf_atmos_bevy` injects F2's density terms into a 160×90×64 (High) camera grid, lit by the key light ×
  CSM × terrain shadow-height map × cloud shadow + sky fill + the 32 nearest point lights, temporally reprojected,
  integrated front to back into a shared `FroxelVolume` image; the composite uses it to `far` (384 m) and F2's analytic
  layers beyond. Transparents sample the same volume (`froxel_apply.wesl`, `FroxelFogMaterial`); hosts opt in per
  material. The ground-fog bank mask is one shared WESL library (`fog_bank.wesl`).

- 2026-10-13 (m5-a3) **Content packs plug into the mod pipeline through one adapter crate; profile
  fingerprints read stats from directory listings.** Bevyrrowind's `mw_mods` synthesizes a mod per
  legacy plugin (archive paired by stem), one for the loose data folder, and a builtin mod the host
  pins first in every profile (lowest layer; no framework "pinned" rule). New framework hooks, all
  defaulted: `ContentAdapter::stamp` (what the pipeline can't stat: adapter version, archive stats,
  generated layers) and `::modern_alternates` (drives the `ShadowedModern` warning),
  `VfsSource::stat` (size + mtime as indexed; `DirSource` keeps the listing's, since per-file stats
  cost ~50 µs each on Windows). The profile fingerprint is stored beside the bake manifest, not in it
  (old manifests still load), and doesn't skip the incremental bake: step code isn't in it.
- 2026-10-13 (physics-p7, M4 P7): ragdolls use rapier **impulse joints** (not multibodies) with 8 extra solver
  iterations and a 4:1 neighbor mass-ratio cap: joint stretch 0.49 cm in a 0.5 m drop and 0.07 cm dragging 75 kg by
  the wrist (gate 1 cm); more iterations are not monotonic (12+ stretch 3 cm). New Bevy-free crate `sf_ragdoll`
  (`ragdoll.cook`, `RagdollInstance`). Limp ragdolls reach the skeleton through `sf_anim_bevy::PhysicsPose`, blended
  after the graph; P8 moves powered ragdolls into the graph's `PoseInput`.
- 2026-10-13 (m5-l2, M5 L2): script **world access = declared slot resources moved into one cell** shared by every
  context (`sf_script_bevy::access`): API modules register the `Resource`s they need, a slot moves them out of the
  `World` and back, API functions borrow them via `with_resource` (error outside a slot or on re-entry). Cost per
  slot is one move per declared resource, independent of the context count (per-context guard installation would
  be 0.23 µs × contexts × slots); no `unsafe`, no `scope`. Script handles resolve through `ScriptPlacements`
  (filled by the content pack's R0 hooks) until streaming owns a `PlacementId → Entity` map. Activation is a
  framework message (`Activated` → `ActivationOutcome { handled }`); default actions read the outcome.
- 2026-10-13 (m6-w1, M6 W1): **`sf_water` core built** (water.md "As built (W1)"). Choices that outlive W1: the wave
  `h0` noise is keyed by wave vector, not texel, so the 64² CPU twin and the 256² GPU grid hold identical
  coefficients; cascade bands split at `size(i+1)/2` (not `/4`), so cascade 0 keeps 8+ texels per wave on the twin;
  the twin samples with a Catmull-Rom cubic (bilinear at 4 m texels was 4-8 cm off at Hs 2 m, cubic 2.4 mm); the
  query reads the state of the last fixed `step(t)` (no `t` argument: one FFT per step, then O(1) samples);
  amplitudes are the physical JONSWAP spectrum (no fixed-RMS normalisation as in bevy-aqua). `WindInput` is a
  named scaffold until `sf_wind::WindState` (WD1). Ported code carries its notice in `THIRD_PARTY_NOTICES.md`.
- 2026-10-13 (physics-p8): **Powered ragdolls use composite-inertia gain scaling and the graph's built-in physics stage.** Rapier's acceleration-based motors see only a joint's two bodies, so drives scale stiffness by the subtree's inertia about the joint over the child body's (Zordan & Hodgins 2002) and the world pelvis drive by whole-body mass/inertia; full strength is ω 160 rad/s (stable PD). Ragdoll poses enter the graph through `GraphInstance::set_physics` (per-joint weights, after inertialization, before history and procedural ops) instead of a per-graph `PoseInput` node; get-ups hand back with `GraphInstance::inertialize_from`. Driven bodies use the new `Layer::POWERED` (not grabbable).
- 2026-10-13 (m5-a4, M5 A4): **VFS hot reload = one Bevy-free `notify-debouncer-full` watcher over the directory layers,
  an `arc-swap` snapshot (`SharedVfs`), and winning-change routing** (mods.md "As built (A4)"). Not Bevy's `FileWatcher`
  (one directory per asset source, no shadowing); the same crate and version Bevy uses, connected to the asset server
  only at its event channel (`AssetSourceBuilder::with_watcher`). A rebuild rescans only dirty `DirSource` layers and
  keeps layer positions; a change is "the winner's layer or indexed stat changed", so shadowed edits are silent.
  Records reload as a whole-DB re-run of step 4; placed objects don't respawn. Dev builds only; dev re-bake during play
  (owner decision 4) not built yet.
- 2026-10-13 (m6-w2, water.md "As built (W2)"): the water draw is an `ExtendedMaterial<StandardMaterial, WaterExt>` in
  the transparent phase, depth-writing, reading a colour + depth snapshot taken after the sky and the `sf_atmos`
  composite (UE Single Layer Water structure), fogging itself (aerial-view LUT copy + froxels), with a procedural
  velocity redraw for DLSS. Waves repeat every 256 s (ω rounded to whole cycles, twin and GPU alike) so the GPU runs
  on f32 time.
- 2026-10-13 (m5-l3, scripting.md "As built (L3)"): **script API v0.1.** A script's top level only registers
  (handlers, timers, entity scripts, exports) and has no world access; `core.on_load(info)` handlers are queued by
  start, hot reload and restore and run at the start of the next `Update` slot, where world reads work. Entity script
  state lives in the owning mod's context snapshot (one blob per mod for saves and reload), not the placement overlay;
  entity scripts are defined by name at start and attached by name (plain-data attachments, like named timers).
  Handle methods come from a per-context method table API modules fill at install. Spawn/despawn are queued as
  `WorldCommand` messages the content pack applies. `@sf/interfaces` calls are synchronous, copied `StateValue`s, run
  under the callee's watchdog, cycles rejected. The `.d.luau` is generated from the registry and golden-tested
  (`docs/api/sf.d.luau`). mlua `serialize` dropped.
- **2026-10-18 (m6-w3 + w3fin; specs/water.md "As built (W3)"): water FFT fused, swell amortised, shore distance as
  the depth proxy.** The ocean FFT runs as two fused dispatches (spectrum + rows, columns + assemble) with a runtime
  grid; High alternates the two swell cascades (3 of 4 per frame, 0.14 ms). The water material has no bed heights:
  the baked signed shore distance (`ShoreField`, EDT + openness by connected component) is its depth proxy for wave
  attenuation, shore foam and discarding sea under land, on GPU and CPU alike. Region optics are a cell map + a small
  look table, not per-body materials; a water body's body colour uses a backscatter-sized diffuse reflectance (2-5 %),
  not the total scattering (milky).
- **2026-10-18 (p10a; specs/physics.md "Cloth", as built): the cloth solver lives in `sf_anim::cloth`** (Bevy-free),
  not in `sf_anim_graph`: the cook (`sf_character`) builds `ClothLattice` and depends on `sf_anim` only; the op in
  `sf_anim_graph` calls the solver. Character-space inertia is **fictitious forces** (scaled frame acceleration,
  Euler, centrifugal and Coriolis terms; UE Chaos / `NvCloth`), not the position share `SpringBones` uses, so a steady
  run is free and only speed and heading changes swing the cloth. Tethers use the geodesic (down-the-column) length.
- **2026-10-18 (u1a, M5 U1; specs/ui.md OQ4 and "As built (U1a)"): input crate `bevy_enhanced_input` 0.27; `sf_ui`
  core is Bevy-free.** bevy_enhanced_input over `leafwing-input-manager` 0.22 (both on Bevy 0.20): its prioritised,
  input-consuming contexts are CommonUI's per-screen input configs, so the router is "activate the top screen's
  context", with no clash handling of ours; it lives only in `sf_ui_bevy`. `sf_ui` holds the UI records (serde kinds in
  `sf_records`), `UiRegistry` (owner = defining mod; slots and columns ordered by priority, then load order),
  `ScreenStack` (routing rule), `ViewModel` as named fields (`UiValue`, not `Reflect`), keyed `ListModel`,
  `SettingsStore` over profile tables, and a Fluent `Localizer` (later mods override messages; key as last fallback).
