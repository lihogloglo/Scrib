# R10: what to borrow from Mudcrab

Slice R10 of the animation study (README). Whole-project audit of
[Mudcrab](https://github.com/Mudcrab-Team/mudcrab) at commit `3129b57` (2026-10-16), read only from a
shallow clone (`C:\cargo-target\refs\mudcrab`, nothing built or run). Mudcrab converts and renders
Skyrim SE data; this note describes techniques and project structure only (clean room, CLAUDE.md rule 3),
never formats or game internals.

Status: done 2026-10-18 (r10).

## Verdict in one paragraph

Mudcrab is a **converter-first** project: an offline pipeline (legacy archives, meshes, textures, plugins
and compiled scripts into GLB, KTX2, SQLite and Luau) plus a Bevy 0.19 viewer that streams the static
world, terrain, terrain LOD, water and an approximate sky. Everything we would most like to borrow for
gameplay (Luau runtime, UI, animation, combat, physics beyond a capsule, object LOD) is **roadmap text,
not code**, with no dates. We are ahead or level on every engine system and on scripting; they are ahead on
**measurement discipline** (settled reference shots, bounded frame traces, pacing metrics, proof that
Bevy's GPU-driven path is actually on), **converter robustness** (resume, repair, staged publication,
truncation/mutation sweeps), a **GPU texture encoder**, an **MO2 profile reader** and a **developer
console**. Nothing is worth taking as a dependency; four small things are worth porting; the
"piggyback on Mudcrab's Luau" premise in our ROADMAP is wrong and should be corrected.

## What Mudcrab is (built vs planned, quality)

- **Crates** (`Cargo.toml`): `converter` (offline pipeline, about two thirds of the code), `engine` (one monolithic Bevy
  crate: render, streaming, physics, console, profiling), `launcher` (Bevy UI setup wizard and conversion
  screen), `mo2` (Mod Organizer 2 instance reader), `shared` (versioned contracts), `dummy-content`
  (synthetic fixtures so tests and CI need no game data). About 163k lines of Rust, comparable to our
  ~157k across both repos.
- **Versions**: Bevy **0.19** (we are on 0.20), `bevy_rapier3d` 0.36, `mlua` **0.10** with `luau-jit` (we use
  0.12), `rusqlite` 0.32, `rkyv` 0.8, Rust 2024. Nothing is published to crates.io, so "adopt as a
  dependency" would mean a git dependency on a Bevy 0.19 crate: ruled out for every crate that touches
  Bevy, and the Bevy-free ones are Skyrim-specific.
- **Licence**: MIT OR Apache-2.0 (dual, not Apache only), **but** the converter vendors
  `project-wormhole-nif` 0.1.0, declared **GPL-3.0-or-later** (`vendor/project-wormhole-nif/Cargo.toml`;
  their ADR-0005 notes it "needs a maintainer license decision"). So: never depend on `converter`, and
  never port code from `converter/src/mesh.rs` or `material.rs` (written against that parser) into
  superfargoth (rule 2). Self-contained modules (`texture_gpu`, `engine/*`, `mo2`) are their own MIT/Apache
  work and are portable to either repo with attribution.
- **Built**: the converter (very thorough), static world streaming with an R-tree, terrain and native
  **terrain-only** LOD (4/8/16-cell tiers, baked albedo), planar-reflection water, a custom sky shader with a
  fixed palette, 64 nearest point lights, a kinematic capsule (Rapier KCC) with debug dynamic props, a
  launcher with MO2 import and repair, a developer console, profiling and acceptance scripts.
- **Planned only**: `crates/scripting` (Luau runtime, engine API, hot reload, `.d.lua` types), Flash-to-`bsn!`
  UI, character skinning at runtime, **all animation**, combat, saves, AI, object LOD chunks, tree
  billboards, dynamic LOD proxies, RT/multiplayer/VR (`docs/roadmap/03..05`, 150-250 words each, no dates).
  Phase 2 (core engine) itself is still open pending real-asset sign-off.
- **Quality bar**: high where built. 1,577 `#[test]`s (we have ~900), CI runs tests, an acceptance and a
  profiling workflow, independent human review required before merge, CodeRabbit. Specs carry numbered
  invariants and dated bug histories (`lod-compiler.md` "Regression invariants", "Bug history"), with
  negative controls in probes. Weak spots: the marketing layer (README feature list, `vercidium-optimizations.md`,
  `mods-and-ui.md` code samples, `pipeline.md` diagrams) describes things that don't exist; some roadmap
  files are in Portuguese; the engine is one crate (slow incremental builds, the opposite of our §8b).

## Area by area

| Area | Mudcrab | Ours | Status |
|---|---|---|---|
| Mesh conversion | NIF to GLB offline (`mesh-tools`), GPL NIF parser | runtime NIF via `tes3` (MIT) + `.sfmesh` bake (`bake.md`) | different; nothing to take (licence + game) |
| Records | plugins to SQLite + `rkyv` cell cache; schema-driven in-house reader with mod-supplied JSON schema add-ons (`schema-addons.md`) | `sf_records` typed overlays with field provenance, native RON/TOML patches (`mods.md` A2) | different; learn (below) |
| Textures | DDS to KTX2, native BC blocks copied, rest to BC7 or UASTC; **GPU UASTC encoder** in a wgpu compute shader, 13x faster on their data | textures load as shipped; terrain arrays cook BC7/BC5; texture cooking is M7 (`bake.md`) | they're ahead; **port later** |
| Bake pipeline | resume, failed-asset repair, staging dir vs published pack, runtime read lock vs conversion write lock, progress line, Ctrl+C | `sf_bake` content-hash cache, incremental, deterministic, atomic rename | level; learn repair + locks |
| LOD | terrain tiers only; objects/billboards/proxies are phases 2-4 of a plan | HLOD tree, octahedral impostors, distant lights, built and measured (`far-field.md`) | **we're ahead** |
| Terrain | per-cell meshes, splat shader, baked LOD albedo | CDLOD/clipmap GPU heightfield, one height source for render/physics/nav (`terrain.md`) | we're ahead |
| Streaming | cell grid, R-tree, 1 commit/frame, hysteresis, generation-guarded stale discard | rings, budgets, velocity lookahead, burst mode, `big_space` (`streaming.md`) | level; learn metrics |
| Colour pipeline | one HDR composition, single TonyMcMapface pass, EV100 pinned; GPU probe proving sky/fog/water/unlit agree within 2/255, with a negative control | scheduled EV100, Atmosphere, Solari option (`sky-lighting.md`) | level; learn the probe |
| Sky | custom sky shader, fixed clear-day palette, distance fog | Bevy `Atmosphere`, sun/moon/season, clouds (`weather.md`) | we're ahead |
| Water | planar reflection camera with near-plane clip | SSPR + env/probe fallback, FFT (`water.md`) | we're ahead |
| Point lights | 64 nearest enabled, re-chosen on 256-unit move or light-set change | Bevy clustered forward + Solari | we're ahead; one detail to learn |
| Reference shots | `--shots file.json`: many exact poses per run, waits for N quiet frames, fails on new errors, timeout reports what's pending | one `--screenshot` after `settled()` (`exterior.rs`), F12 manual | **they're ahead; port** |
| Profiling | per-run bundle (frames, CPU spans, GPU passes, streaming, memory slope), bounded `streaming-frames.json`, "unavailable" never zero-filled | `sf_diag` counters, gates, baselines, camera paths (`diag.md`) | level; learn 3 metrics |
| Physics | Rapier KCC capsule, fixed 60 Hz, static colliders from converted data | `sf_physics` (rapier 0.36, Bevy-free), `sf_controller`, `sf_ragdoll`, carry/throw (`physics.md`) | we're ahead |
| Luau | 105-line sandbox (`engine/src/papyrus_runtime.rs`) + 120-line Luau shim; `.pex` bytecode transpiler in the converter | `sf_script` ~5,000 lines: per-mod contexts, watchdog, escape tests, memory limits | **we're ahead** |
| UI | none (plan: transpile Flash to `bsn!`) | `sf_ui` spec + slice | we're ahead |
| Launcher / mods | Bevy launcher: detect install, convert, drag-and-drop, MO2 profile import | `sf_mods` repository/profiles/conflicts, no launcher yet | they're ahead on launcher + MO2 |
| Dev console | registry, plugins add commands, completion, "did you mean", pure-function parsing tested without UI | planned as a Lua console (L3/W3, parked) | they're ahead; learn/port |
| Animation | none; plan is glTF clips + a simple blend tree | `sf_anim`, `sf_anim_graph`, IK, ragdoll, this study | we're far ahead |
| First person | none | `character.md`, R2/R11 | we're ahead |
| Interiors | converter emits room/portal-marker tables for culling (data Morrowind lacks) | seamless portals spike (`portals.md`) | ignore |

## Ranked list

### Adopt (as is, small)

1. **DX12 indirect-count workaround** (`engine/src/renderer_init.rs`, `backend_features`, commit `68b5023`).
   Bevy's mesh preprocessing compacts indirect commands whenever the device has
   `MULTI_DRAW_INDIRECT_COUNT`, but the draw side ignores the count on DX12 (wgpu issue #7974), so stale
   commands can draw. I checked **Bevy 0.20.0** in our registry: the producer check
   (`bevy_pbr/src/render/gpu_preprocess.rs` ~1875) still has no backend test while the consumer
   (`bevy_render/src/render_phase/mod.rs` ~1204) still excludes DX12, so the mismatch is still there. Fix:
   remove that one feature from the device when the adapter is DX12. Cost: ~20 lines in our renderer init
   (bevyrrowind app), only if we ever run DX12 (DLSS pins Vulkan, but `sf_water_bevy` comments show DX12
   runs happen). Symptom to watch for: ghost or flickering instances on DX12 only.

### Port (license-clean, worth the cost)

2. **Reference-shot batches** (`engine/src/shots.rs`, spec `docs/specs/engine/reference-shots.md`, commit
   `18fa8c5`). A JSON list of named camera poses; each is captured only after the view is quiet for N
   consecutive frames (no loads, no pending assets, no new failures) and after a warm-up for pipeline
   compiles; a timeout still captures and logs what was pending; a per-shot log line is appended as it
   goes; any timeout or write failure exits non-zero. We already have the `settled()` predicate; this
   turns it into a batch tool that fits the owner's "visual sign-off in batches" rule
   (`visual-checks.md`) and gives agents a cheap regression image set. Port the idea into `sf_diag_bevy`
   (generic) with the readiness predicate supplied by the app. Cost: ~1 day.
3. **GPU UASTC/BC encoder** (`converter/src/texture_gpu/`, `uastc_encode.wgsl`, ~2,650 lines, own code plus
   attributed Apache-2.0 Basis Universal tables). Bounded byte queues, slot reuse, GPU watchdog-sized
   dispatches, CPU fallback on device loss. **Not now**: our texture cooking is M7 and vanilla Morrowind
   textures are tiny. When `bake.md`'s texture cook lands (HD texture mods, terrain arrays), port it into
   `sf_bake` behind a feature, on wgpu 29 (Bevy 0.20's). Cost: 2-3 days plus quality checks.
4. **MO2 instance reader** (`crates/mo2/src/lib.rs`, 641 lines): reads the instance INI, profiles,
   mod list order, overwrite folder, and resolves the virtual data folder without USVFS. Many Morrowind
   players manage mods in MO2. Port into bevyrrowind's `mw_install` as a second importer beside
   `openmw.cfg`, dropping the Skyrim plugin-selection parts. Cost: ~1 day, when M5's mod import is reopened.

### Learn (ideas for our specs, no code)

5. **Measurement semantics** (`docs/roadmap/02-profiling.md`, `docs/research/streaming-trace.md`,
   `engine/src/pacing.rs`): (a) **lag at speed**, the camera distance at which each model finished
   loading during a scripted fast flight (a model that lands close arrived late), a better pop-in metric
   than latency; (b) **world-ready time** and **jump time** (teleport, then time to ready again, the ready
   predicate held two frames in a row); (c) a **memory-growth slope** gate measured after a settle window
   (10% of the run, capped at 60 s); (d) "unavailable" and `null` are never written as zero; (e) a
   synthetic **streaming fixture** that teleports, rebases the origin repeatedly and returns, and fails on
   duplicate, orphaned or missing cell roots or stale work. Add (a)-(c) and (e) to `diag.md`/`streaming.md`.
6. **Renderer proof** (`engine/src/render.rs`, `RendererMetrics`): benchmarks record whether GPU
   preprocessing, GPU culling, indirect drawing and HZB occlusion were actually active, and refuse to
   compare runs otherwise. A silent fallback to CPU batching would wreck our far-field numbers; one
   boolean per run in `sf_diag` reports is cheap.
7. **Colour-pipeline probe** (`docs/specs/engine/color-pipeline.md` V3/V4, `examples/color_pipeline_probe`):
   headless GPU readback proves that sky, unlit, fully fogged, emissive and mirror-water surfaces fed the
   same composition-domain colour come out within 2/255, in gray and HDR, with a legacy-path negative
   control that must fail. Worth one test in `sky-lighting.md` now that we have Atmosphere, fog, water and
   Solari composing.
8. **Converter robustness** (`docs/adr/0005`, `pipeline.md` §5-7, `repair.rs`): truncation sweeps over
   every prefix and bounded mutation sweeps for every parser in the normal test suite under
   `catch_unwind`; performance budget tests `#[ignore]`d with 5-10x headroom, median of three; a
   **failed-asset repair** run that reconverts only failures into a staged copy and publishes on `--apply`;
   exclusive bake lock vs the game's shared read lock. `bake.md` has atomic rename but no repair mode and no
   lock against a running game; the sweeps fit `tes3`-facing code in bevyrrowind.
9. **Dev console design** (`engine/src/console.rs`): commands registered by each plugin through an `App`
   extension, case-insensitive long and short names, completion and "did you mean", parsing as pure
   functions tested without UI. When the Lua console unparks, keep this shape: a native command registry
   first, Lua as one command source.
10. **Frame-hitch fix pattern** (commit `d16b73c`): a cell crossing rescanned 65,000 LOD patches (12-40 ms);
    the fix keeps a per-quadrant index fed by `Added`/`RemovedComponents` and only rewrites changed
    quadrants. Standard ECS practice, but a good test idea: a cell-crossing timing test on a dense
    synthetic far ring. Check our `sf_lod_bevy` residency and `sf_terrain_bevy` selection for full scans.
11. **Schema add-ons** (`docs/specs/modding/schema-addons.md`): mods ship a data schema to decode record
    types the reader doesn't know, so tooling can show and remap them; unknown data is kept opaque in
    order. Our native record kinds already cover new types, but "unknown legacy subrecords survive
    round-trip, opaque and ordered" is a rule `mw_records` should state. Their record schema also records a
    public `source` per entry (clean-room provenance), a practice our adapter specs could copy.
12. **Point-light budget detail** (`point-lights.md` §6): re-choose on a change of the *set* of lights, not
    their count (a cell swap with equal counts left rooms dark). Relevant if we add a CPU light budget for
    raster distant lights.

### Ignore

- **Luau runtime and pex transpiler**: theirs is a stub; the transpiler targets a different bytecode and
  language. Nothing to take for MWScript translation (`scripting-adapter.md`) beyond the obvious (coroutine
  waits, state tables), which we already have.
- **NIF/material converters** (GPL-entangled, other game's formats), **SQLite + `rkyv` world database**
  (our records are typed overlays with field provenance; a SQL layer adds nothing we need),
  **`vercidium-optimizations.md`** (aspirational; in code they rely on Bevy's GPU-driven path, as we do),
  **room/portal tables** (Morrowind has no such data), **sky, water, terrain LOD, physics** (we're ahead),
  **launcher UI** (Bevy 0.19 UI, their flow; we'd build ours on `sf_ui`).

## What to wait for (rule 8)

Nothing on our path. Their Luau runtime (Phase 3), UI (Phase 3), animation, combat and saves (Phase 4) and
object LOD (LOD phases 2-4) all trail ours and have no dates. **ROADMAP correction:** the M5 note "Lua parked
... piggyback on Mudcrab's Luau later" should keep Lua parked for priority reasons only; there is no Mudcrab
Luau runtime to adopt, and ours is the more complete one. Re-check Mudcrab once their `crates/scripting`
exists, mainly for the `.d.lua` type-definition generator and LuaLS packaging, which we'd want for modders.

## Animation (for R11)

Mudcrab has **no animation code**: no runtime skinning of characters, no clip playback, no first person.
It does not play Skyrim's behavior graphs and does not plan to; `docs/roadmap/04-gameplay-and-physics.md`
§4.2 plans "glTF skinning clips" with "an animation state blending tree (idle, walk, run, sprint, attack)",
which is behind what `sf_anim_graph` already does. Their converter's skinning export is listed as owed
(`02-completion-plan.md`). Nothing here changes R1-R9 or the R11a trade-offs. One indirect point: when they
do reach Phase 4 they will need the same things we are designing (clip import into glTF, a move
chooser, first-person bodies), so our animation spec is a natural contribution (below).

## Where we're ahead and could contribute back

Both projects are MIT/Apache, so contribution is licence-free. Our Bevy-free cores avoid their Bevy 0.19
lock; Bevy plugins would need their 0.20 upgrade first.

- **Shared Bevy bug**: we found the same GPU-preprocessing upload-deferral bug independently
  (`far-field.md` "Bevy 0.20 pitfalls" 1: instances collected while a mesh upload is deferred keep a stale
  slab slice; their commit `268f988` and `engine/src/mesh_preparation_retry.rs`). Filing one upstream Bevy
  issue with both repros is the best contribution for both projects.
- **`sf_script`** (sandboxed per-mod Luau contexts, watchdog, escape tests): exactly their Phase 3.1.
- **Far field**: HLOD tree, octahedral impostors and distant lights are their LOD phases 2-4 and
  `billboard-generation.md`.
- **Terrain** (CDLOD/clipmap, one height source), **water** (SSPR, FFT), **sky** (Atmosphere, time of day,
  clouds), **physics** (`sf_physics`, `sf_ragdoll`, carry/throw), **character controller**, and the
  **animation stack** (`sf_anim`, `sf_anim_graph`, IK) once R11b lands.
- **Compile-time architecture** (§8b Bevy-free cores, thin plugins): their single `engine` crate is the
  pattern we left behind.

A short note to their maintainers listing these, plus the shared Bevy issue, costs an hour and opens the
door to them adopting `sf_*` crates, which in turn would give us a second user and testbed.

## Sources

Mudcrab `3129b57`: `README.md`, `Cargo.toml`, `crates/*/Cargo.toml`, `docs/roadmap/{README,02-core-engine,
02-profiling,02-completion-plan,03-luau-and-ui,04-gameplay-and-physics}.md`, `docs/specs/engine/{lod-architecture,
color-pipeline,point-lights,reference-shots,sky}.md`, `docs/specs/converters/{lod-compiler,pipeline,pex-to-lua,
dds-to-ktx2}.md`, `docs/specs/modding/{schema-addons,launcher}.md`, `docs/research/{streaming-trace,
interior-room-graph-converter}.md`, `docs/adr/0005,0011`, `crates/engine/src/{papyrus_runtime,physics,pacing,
console,render,renderer_init}.rs`, `crates/converter/src/texture_gpu/mod.rs`, `crates/mo2/src/lib.rs`, commits
`68b5023`, `d16b73c`, `268f988`, `18fa8c5`. Bevy 0.20.0 sources from the local cargo registry.
Ours: ARCHITECTURE §4-5, specs `far-field`, `streaming`, `diag`, `sky-lighting`, `water`, `terrain`, `mods`,
`bake`, `physics`, `portals`, `bevyrrowind/apps/bevyrrowind/src/exterior.rs`, ROADMAP M5 note.
