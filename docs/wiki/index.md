# Scrib wiki

The on-demand reference for how Scrib works **now**, how Bevy 0.20 works, and what the industry does. Read this
index, then only the page you need. Pages are current truth, never history: what was tried and how it went lives
in the ledger (`../ledger/`), old reasoning in the archives.

## Rules for pages

- **One page per topic, ≤ 6 KB.** Split a page that grows past that; link, don't repeat.
- **Current state only.** No "as built on date X", no abandoned designs, no measurements older than the last
  change (put dated results in the ledger). A page that is wrong is a bug: fix it in the same slice that changed
  the behaviour.
- **Evidence, not prose.** Every claim about code carries `path:line` (our code) or `bevy_<crate>/src/...:line`
  (Bevy 0.20 source in `~/.cargo/registry/src/*/bevy_*-0.20.0/`, or our vendored copy in `superfargoth/vendor/`).
  Industry claims cite the source (talk, paper, engine doc URL).
- **Fixed shape** for engine pages: *What it does* (3-5 lines) / *How it works* (data flow, the main types and
  systems, path:line) / *Bevy features used* (link to `bevy/`) / *Switches and counters* (`--off` parts, perf gauges)
  / *Known limits* (current, with ledger links) / *Specs* (links to the design spec, if any).
- Plain, direct English. No em dashes.

## Rules for the ledger (`../ledger/<system>.md`)

One line per attempt, newest first, never edited after the fact except to add a later verdict:
`YYYY-MM-DD | what was tried | measured result (scene, ms, counters) | verdict: kept / reverted / opt-in / open | commit or doc`.
Briefs paste the ledger lines of their system. Before proposing a change, grep the ledger for it.
Perf words: a **stutter** is a frame over 1.5x the moving median of 60 frames (report "stutters/min"); a **hitch**
is a frame over 25 ms (report "hitches"). A/B differences under ~0.5 ms are run-to-run noise (old path walk read
10.71, 10.95 and 11.72 ms across 2026-11-11/12); repeat 3x.

## Pages

### Engine (`engine/`)
<!-- one line each: link to engine/NAME.md, then what it covers -->
[Shadows](engine/shadows.md): sun cascades and PCSS, lamp shadow budget and cache, mover shadows, contact shadows, presets
[Lights](engine/lights.md): lamp lights from content, reach, clustering, night ambient and moonlight
[Sky, atmosphere, weather](engine/sky-atmosphere-weather.md): Bevy atmosphere and our patches, clouds, fog, weather state driving them
[Streaming](engine/streaming.md): exterior cell streaming, spawning, floating origin, what a placement becomes, origin-shift guard, `--off` parts and stream counters
[Statics and LOD](engine/statics-and-lod.md): precombine, far field, experimental static scene (`--scene`), opt-in `NoCpuCulling` path
[Frame and views](engine/frame-and-views.md): cameras and shadow views in an exterior frame, driving plugins, perf suite and report sections
[Characters](engine/characters.md): NPC and player assembly, merged parts, skin joint blocks (P5), first-person roles and view copies
[Animation](engine/animation.md): graph runtime, ops, humanoid graph, LOD tiers A0-A3, foot IK, clip sources and bake
[Locomotion, action, camera](engine/locomotion-and-action.md): fixed tick, mover, action chooser, player feel, camera, first-person eye
[Content pipeline](engine/content-pipeline.md): VFS, records, mods, Morrowind ESM/BSA/NIF adapters, the bake steps and store
[Terrain and water](engine/terrain-and-water.md): baked tiles, clipmaps, CDLOD, far shadow map; FFT ocean, wave twin, shore, pools
[Textures](engine/textures.md): BCn KTX2 cook, de-light, normal-map companions, tangents, runtime layer
[Physics](engine/physics.md): rapier world, collision from NIF, item bodies, carry, Rapier ragdolls, Delirium takeovers
[UI and menus](engine/ui-and-menus.md): UI kit, CSS theme, screen stack, main and pause menus, options, intro, dev console
[Saves](engine/saves.md): save file format, atomic write, sections, quickload, save/load screens
[RPG and scripting](engine/rpg-and-scripting.md): dialogue, journal, quests, stats, MWScript interpreter, parked Luau
[Navmesh and AI](engine/nav-and-ai.md): Recast bake, nav and AI cores, what the game wires (PGRD wander only)
[Audio](engine/audio.md): buses, mixer, cues, music, footsteps, Morrowind sound adapter
[App and dev tools](engine/app-and-dev-tools.md): subcommands, scenes, `--off`, perf suite, BRP, cargo_turn and owner_launch

### Bevy 0.20 (`bevy/`)
[Render and visibility](bevy/render-and-visibility.md): extraction, visibility and culling (CPU/GPU, `NoCpuCulling`, ranges), retained bins, GPU preprocessing, views, shadow views; path:line in Bevy 0.20
[Render assets and materials](bevy/render-assets-and-materials.md): RenderAsset flow, AssetEvent to render world, GpuImage reuse, material bind groups and bindless slabs, mesh slab allocator
[Skinning and animation](bevy/skinning-and-animation.md): SkinnedMesh extraction, joint buffers and motion vectors, morph targets, skinned batching, bevy_animation graph
[Schedules, transforms, ECS](bevy/schedules-transforms-ecs.md): main and render schedules, pipelined rendering, transform propagation and static optimisation, big_space grids and floating origin
[Shadows and lights](bevy/shadows-and-lights.md): cascades, point/spot shadow views, retained state, vendored shadow cache patch, clustering, probes
<!-- one line each -->

### Industry standards (`industry/`)
<!-- one line each -->
[Open-world streaming](industry/open-world-streaming.md): partitions, HLOD, async budgets, large-world coordinates, UE5/CDPR/RAGE/Decima
[GPU-driven rendering](industry/gpu-driven-rendering.md): retained scenes, per-view GPU culling, indirect draws, CPU cost per entity
[Shadows](industry/shadows.md): cascades, virtual and cached shadow maps, local-light budgets, contact and far shadows
[Character animation](industry/character-animation.md): motion matching vs blend graphs, warping, foot placement, anim budget, crowds, modular merge
[First person](industry/first-person.md): true first person vs arms rig, view model FOV, camera styles, first-person melee
[Bevy projects](industry/bevy-projects.md): what other Bevy open-world projects do, and that none reach the target scale

## Ledgers (`../ledger/`)
<!-- one line each: link to ../ledger/NAME.md, N entries, last date -->
[shadows](../ledger/shadows.md): 22 entries, last 2026-11-12 (sun cascades, lamp cache, resets A1-B2, PCSS, lamp reach)
[sky-weather](../ledger/sky-weather.md): 17 entries, last 2026-11-12 (atmosphere medium P1, clouds, fog, night lighting, rain)
[perf](../ledger/perf.md): 27 entries, last 2026-11-12 (frame time: culling, bind groups, skinning, ceilings, A/B)
[water](../ledger/water.md): 1 entry, last 2026-11-12 (water pass, SSPR reflections; older water lines in sky-weather)
[streaming](../ledger/streaming.md): 16 entries, last 2026-11-12 (cell streaming, origin shift, stutters)
[statics](../ledger/statics.md): 21 entries, last 2026-11-12 (precombine, static scene, far field, LOD, NoCpuCulling)
[animation](../ledger/animation.md): 15 entries, last 2026-11-12 (IK P-IK1..5, gaits, foot lock, anim LOD, 2026-11-12 audit, AN-/AL-/PL- verdicts)
[combat-feel](../ledger/combat-feel.md): 13 entries, last 2026-11-12 (combat C1-C4, guard, lock-on, first-person melee and view model, CB-/FP- verdicts)
[physics](../ledger/physics.md): 20 entries, last 2026-11-11 (fix plan slices 1-10, Delirium takeovers and hits, collision rules, PH- verdicts)
