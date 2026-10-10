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
| Physics | `avian3d` 0.7 | 0.19 | ECS-native colliders and bodies, transform interpolation. `bevy_rapier3d` 0.37 already targets 0.20 as a fallback. |
| Lua | `mlua` 0.12, **Luau** | n/a | Sandboxed by design, 5.1-compatible, optional native codegen, type checker for mod authors. |
| Char. controller | `bevy_tnua` 0.32 | 0.19 | Or our own kinematic controller. Decide during the physics milestone. |
| VFX | `bevy_hanabi` 0.19 | 0.19 | GPU particles: rain, ash, magic. |
| Clouds | own, or port `bevy_weather` / `bevy_fast_clouds` | 0.19 | No built-in clouds in Bevy. |
| IK | own | — | Existing crates are stale (Bevy 0.8–0.14). |
| Grass, ocean, terrain | own | — | No maintained crates for 0.20. These are core to the vision anyway. |

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
| `sf_ui` | Bevy UI and Feathers widgets; HUD, menus, dialogue, journal, all scriptable. |

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

### 7.3 Rigid-body physics

- **Avian**, on a fixed timestep with transform interpolation (the lesson
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

### Machine setup (Windows)

- **Exclude the `target/` folders (and `~/.cargo`) from Windows Defender
  real-time scanning.** Scanning every object file and rlib is often the
  single largest overhead on Windows. Needs admin; do it once.
- Use a Dev Drive (ReFS) for the repos and cargo home if available.

### Not adopted (yet)

- **Cranelift backend, `-Zshare-generics`, the parallel frontend
  (`-Zthreads`)**: real wins but nightly-only. Revisit if they reach stable.
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
| M2 | Exterior: `sf_terrain` from LAND, cell streaming with budgets, atmosphere + sun, Solari toggle with emissive light proxies | Fly over Vvardenfell without hitches |
| M3 | Characters: assembly, KF animation, animation graph, IK | NPCs stand, idle, and walk with feet on the ground |
| M4 | Physics + controller + interaction: rigid-body items, doors, carry/throw | Walk in, knock over a table's contents, pick things up |
| M5 | Lua runtime + mod pipeline end to end, MWScript → Luau for simple scripts | A Lua mod and a texture mod both work, hot reload works |
| M6 | Ocean, HD grass, clouds/weather, transient deformation | The Bitter Coast at sunset in a rainstorm |
| M7+ | Dialogue, journal, stats and rules, combat, magic, AI, saves, persistent deformation, HLOD/impostors | Main quest playable (long tail) |

M6 is listed late but can run in parallel with M3–M5: it depends only on
M2's terrain and world.

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

## 11. Open decisions

- Physics character controller: `bevy_tnua` vs our own. Decide in M4.
- Moved references (MVRF): counted on load but not yet re-homed into their
  destination cell. Do it with exterior streaming (M2).
- INFO response order (prev/next linked lists across plugins): resolved by
  the dialogue system (M7), not at load.
