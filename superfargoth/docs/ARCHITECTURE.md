# superfargoth & Bevyrrowind: Vision and Architecture

Live status and what's next: `../bevyrrowind/docs/ROADMAP.md`. Decisions: [DECISIONS.md](DECISIONS.md).
Engine baseline: **Bevy 0.20** (released 2026-10-08), Rust 2024 edition.

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

### 1b. Modern rules first, classic rules later (owner 2026-10-23)

The goal is the modern game ("Morrowind 2026": new combat, new AI, voiced dialogue with lip sync, ...).
A **classic rules** option (Morrowind's own mechanics on our engine, with our rendering, performance
and quality-of-life features) is a nice-to-have, kept cheap by design, built last, and only if it stays cheap.
It is not a reason to slow or bend the modern design.

Mechanics fall into two groups:

1. **Content mechanics: one implementation, shared by both.** What Morrowind's quests, NPCs and items need
   to work at all: MWScript (every function), dialogue filters and result scripts, magic effect semantics,
   levelled lists, factions, crime and bounty, journal. The modern game runs Morrowind's content, so it
   needs these exactly as Morrowind defines them. **Port them from OpenMW** (bevyrrowind only, GPL; cite the
   OpenMW file and commit in a comment) rather than re-deriving them from UESP: OpenMW already solved the
   edge cases. Where OpenMW deliberately differs from vanilla (its own fixes and opinions), follow vanilla.
2. **Feel systems: modern implementation first, classic one later behind the same seam.** Combat
   resolution (modern: hitboxes, stamina, poise; classic: swing then hit-chance roll), character
   progression (skill gain, attributes, level-up), AI behaviour (see below), persuasion / barter / spell
   chance / alchemy formulas, movement tuning, UI flow. Each one ends in a seam the game calls through,
   e.g. "an attack connected: how much damage?" or "this actor's turn: what does it do?". The modern
   version implements it; a classic version (`mw_rules`, ported from OpenMW) can be added later without
   touching callers.

**AI is a feel system, and the modern AI will be very different from Morrowind's packages.** The seam is
the *intent* content asks for, not Morrowind's package execution: MWScript `AiEscort`/`AiFollow`/
`AiTravel`/`AiWander`/`StartCombat` become goals ("escort the player to Pelagiad", "stay near here",
"fight this actor") with completion signals scripts can query (`GetAIPackageDone`). Modern AI fulfils
them its own way (schedules, perception, navmesh, group tactics); classic AI ports OpenMW's package
logic onto our navigation and movement. Quests only depend on the goal and its completion, never on how
it was carried out.

Rules for agents:
- Designing a feel system: name its seam in the spec, keep it to a trait or a data profile with a test
  double, and don't wire the modern implementation straight into the app. That is the whole ongoing cost
  of the classic option (~one trait and one test double per system); don't build the classic side early.
- Classic means **Morrowind's rules and numbers**, not bug-for-bug parity with OpenMW (an endless race
  against a mature project; players who want exact parity have OpenMW). Classic reuses our UI screens
  with the Morrowind theme; no MyGUI-style copies of the old menus.
- The graphics side is a separate axis: a Classic graphics preset on the `--off` feature ladder
  (perf.md), usable with either rule set.
- A new game picks the rule set (profile per system, stored in the save meta); switching mid-save is not
  a goal.

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

One repository since 2026-10-25 (the `scrib` repo), two parts with a one-way dependency:

```
┌──────────────────────────────────────┐
│ superfargoth/ (framework)            │  GPL-3.0-or-later (since 2026-10-25)
│ generic ARPG engine on Bevy          │  no Morrowind types
└──────────────────▲───────────────────┘
                   │ depends on (never the reverse)
┌──────────────────┴───────────────────┐
│ bevyrrowind/ (game)                  │  GPL-3.0-or-later
│ Morrowind adapter + rules + game exe │  may borrow OpenMW code freely
└──────────────────────────────────────┘
```

- **Both GPL-3.0-or-later (owner 2026-10-25).** The framework was MIT/Apache
  until then; it went GPL so GPL projects (Mudcrab, OpenMW, Skyrim Community
  Shaders, Rust rewrites) can be reused directly. Ported code keeps provenance
  (source file + commit, `THIRD_PARTY_NOTICES.md`). Morrowind-specific code,
  OpenMW ports included, still lives in bevyrrowind (§1b, §2 litmus test):
  the split is architectural, not a licence wall.
- **Additional permission (GPLv3 §7) for GPU vendor runtimes**
  (`LICENSE-EXCEPTION.md`). It covers only our own code: ported third-party
  GPL code doesn't carry it, so **the game binary never contains NVIDIA
  code**. DLSS goes through `vendor/dlss_wgpu` (a drop-in facade with no NVIDIA
  code) and a separately shipped plugin package (`plugins/sf_dlss` + NVIDIA's
  DLLs) that the player installs; without it the game runs, DLSS unsupported.
- Vendored Bevy patches stay MIT OR Apache-2.0 with no GPL-derived lines, so
  they can go upstream. MIT and Apache-2.0 dependencies are GPLv3-compatible.
- Using OpenMW only as a black-box oracle (run both, compare outcomes)
  involves no code and is fine anywhere.
- Morrowind's own data is never redistributed; users point the game at their
  install.

## 4. Technology baseline (checked 2026-10-10)

Snapshot of 2026-10-10; versions may have moved since (see Cargo.toml).

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
| `bevy_hanabi` | GPU VFX: splashes, magic (M5–M6); rain/snow/ash don't need it (weather/precipitation-surface.md) | git only | 0.19 / 0.19; main migrated to 0.20 + WESL shaders 2026-10-15 (#576), no release yet |
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
| `sf_rpg` | *Generic* RPG primitives: attributes, effects, inventory, dialogue (condition-filtered response database + session state, not a graph; `specs/rpg.md`), quests, journal, factions. The **rules** (formulas, leveling, combat math) are supplied by the content pack as a rules plugin, never hardcoded. |
| `sf_ai` | Navmesh generation per streamed tile, behavior layer (schedules, goals, perception). Takes content goals (escort, follow, travel, wander, fight) through a seam so a classic Morrowind package executor can replace the modern behaviour (§1b). |
| `sf_ui` | Game UI (spec `ui.md`): bevy_ui + headless `bevy_ui_widgets` with our styled widget kit, CommonUI-style layers and input routing, MVVM view-models, two themes and two layout presets, CSS styling via `bevy_flair` layered by mod load order, extension slots, settings registry; HUD, menus, dialogue, journal, all scriptable. Feathers for dev tools only. Crates: `sf_ui` (Bevy-free), `sf_rpg_ui` (Bevy-free RPG screen view-models over `sf_rpg`: dialogue window, journal), `sf_ui_inventory` (Bevy-free item-list logic: inventory, container, barter), `sf_ui_bevy`, `sf_ui_css`, `sf_ui_screens`, `sf_ui_lua`, `sf_devui`. |

### 5.4 Tools

`sf_viewer` (asset and record browser), `sf_bake` (CLI around the asset
processor: HLOD, impostors, navmesh, terrain tiles), benchmark scenes, and the dev
console (`sf_console` + `sf_console_bevy`, spec `specs/console.md`: typed variables and
commands every plugin registers, live switches).

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

Original sketch moved to [archive](archive/architecture-feature-designs.md#72-ocean-and-water); current design: [water](specs/water.md).

### 7.3 Rigid-body physics

Original sketch moved to [archive](archive/architecture-feature-designs.md#73-rigid-body-physics); current design: [physics](specs/physics.md).

### 7.4 Terrain deformation

Original sketch moved to [archive](archive/architecture-feature-designs.md#74-terrain-deformation); current design: [terrain](specs/terrain.md).

### 7.5 Lighting and rendering

Original sketch moved to [archive](archive/architecture-feature-designs.md#75-lighting-and-rendering); current design: [sky-lighting](specs/sky-lighting.md).

### 7.6 Animation

Original sketch moved to [archive](archive/architecture-feature-designs.md#76-animation); current design: [animation](specs/animation.md).

## 8. Bevyrrowind repo (Morrowind content pack)

| Crate | Responsibility |
|---|---|
| `mw_install` | **(built)** Install detection (env var, registry, Steam libraries) and `Morrowind.ini` parsing. |
| `mw_vfs` | **(built)** BSA archive source for `sf_vfs`; mounts Morrowind.bsa, ini archives, then loose `Data Files`. |
| `mw_records` | **(built)** ESM/ESP → `sf_records` with Morrowind merge rules (cell reference merging, deletions, topic-scoped INFO), load order from masters. Uses `tes3` directly; no wrapper crate. |
| `mw_assets` | NIF → Bevy scene loader: geometry, materials, skinning, controllers (keyframe/vis/UV/alpha), particles, billboards, switch nodes, collision. DDS/TGA. |
| `mw_world` | `WorldSource` impl over the loaded records; coordinate mapper (Z-up, ~1.42 cm per unit); LAND → terrain tiles; LTEX → terrain layers and grass rules; doors → transitions. |
| `mw_rules` | Classic rule set (§1b), ported from OpenMW: hit chance and damage, skill gain, leveling, persuasion, barter, spell chance, alchemy; classic AI package execution. Plugs into the feel-system seams; built after the modern versions exist. Content mechanics shared by both rule sets live in `mw_script` / `mw_rpg` / magic effects, not here. |
| `mw_rpg` | DIAL/INFO filtering semantics, topic links, journal, equipment rules (`rpg-adapter.md`; OpenMW reuse allowed here). |
| `mw_script` | MWScript compiler → Luau + `mw` compatibility library. |
| `mw_openmw_lua` | OpenMW-Lua API compatibility (later). |
| `mw_weather` | Morrowind weather presets (ash storm, blight, snow) as `sf_sky` data. |
| `bevyrrowind` | The game binary: settings, install detection, launcher. |

## 8b. Build performance

Moved to [build-perf.md](build-perf.md).

## 9. Milestones

Historical scope labels (priority order is now the ROADMAP phases, see the note under the table). Every milestone ends with something visibly running and a benchmark entry.

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
(old M7 = M8, old M8 = M9, old M9 = M10, old M10+ = M7).
**Order since 2026-10-19 (owner audit): phases in the ROADMAP** (0 cleanup, 1 performance + stability + image
quality, 2 playable vertical slice, 3 animation/combat/physics, 4 eye candy); milestone numbers stay as scope labels.

## 10. Decisions log

Moved to [DECISIONS.md](DECISIONS.md) (2026-10-19). Append-only; grep it for the system you touch rather than
reading it whole. Old references to §10 and §11 ("Open decisions") point there.
