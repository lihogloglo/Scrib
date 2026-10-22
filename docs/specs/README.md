# Framework specs

Specs for `sf_*` systems. Same template and rules as `../../../bevyrrowind/docs/specs/README.md`,
plus: no game-specific content, no GPL-derived material.

**Every spec has an "Industry comparison" section** (rule since 2026-10-12,
after the owner caught a missing basic technique; see
[industry-audit.md](industry-audit.md)). It lists what major engines (UE,
Unity, Bethesda Creation, RAGE, Decima, Frostbite, OpenMW, as relevant) do
for that system, with citations, and says for each technique whether we
match, differ (and why), or lack it (gap + priority). A spec without it is
not "agreed". Existing specs get theirs through roadmap job J9;
industry-audit.md is the cross-system starting point.

**Every spec follows the eight net-ready rules** in [net.md](net.md) (rule since 2026-10-19,
co-op N0): fixed-tick gameplay, commands as the only input, state apart from presentation, pure
steps, one owner per piece of state, no singleton player, stable ids and absolute positions,
events as data. It says how in a "Co-op" section or line, or lists its exceptions; known
violations are in net.md's audit table.

Index:

| Spec | Status | Crates | Milestone |
|---|---|---|---|
| [world-coordinates.md](world-coordinates.md) | agreed (2026-10-11) | `sf_coords`, `sf_world_bevy` | M2 |
| [streaming.md](streaming.md): worldspaces, door links, rings, budgets | agreed (2026-10-11) | `sf_world`, `sf_world_bevy` | M2 |
| [portals.md](portals.md): seamless door portals (render-to-texture views, crossing) | spike done (2026-10-12) | `sf_world_bevy` (M8: `sf_portal_bevy`) | M8 |
| [terrain.md](terrain.md): height stack, CDLOD + clipmaps, splatting, upsampling | agreed (2026-10-11) | `sf_terrain`, `sf_terrain_bevy` | M2 |
| [bake.md](bake.md): `sf_bake`, content-hash cache, mesh LODs | agreed (2026-10-11) | `sf_bake`, `sf_model`, `sf_model_bevy` | M2 |
| [audio.md](audio.md): buses, faded loops, one-shots, bevy_audio decision | draft (2026-10-19) | `sf_audio`, `sf_audio_bevy` | M6 (WE3) |
| [far-field.md](far-field.md): HLOD hierarchy over the streaming grid, octahedral impostors, distant lights, crossfade | draft (2026-10-11) | `sf_lod`, `sf_lod_bevy` | M2 (moved from M7) |
| [diag.md](diag.md): counters, frame stats, gates, baselines, fly-throughs | implemented (2026-10-11) | `sf_diag`, `sf_diag_bevy` | M1 |
| [sky-lighting.md](sky-lighting.md): atmosphere, sun/moon from clock and latitude, exposure, raster/Solari switch, point-light proxies, weather/wind hooks | implemented (2026-10-11) | `sf_sky`, `sf_sky_bevy` | M2 |
| [shadows.md](shadows.md): local-light shadows at scale (importance budget, cached static cube maps, F6) | draft (2026-10-19) | `sf_sky_bevy` (+ `sf_shadow_cache_bevy`) | 6a |
| [animation.md](animation.md): cooked clips, pose runtime, layered graph, blend spaces, root motion, events, IK, LOD/budget | draft (2026-10-11) | `sf_anim`, `sf_anim_graph`, `sf_anim_bevy` | M3 |
| [character.md](character.md): assembly + merging, equipment slots, standard humanoid rig, bake-time retargeting, first/third person | draft (2026-10-11) | `sf_character`, `sf_character_bevy`, `sf_model` | M3 |
| [ui.md](ui.md): game UI stack (CommonUI-style layers, MVVM), two themes × two layout presets, CSS styling via bevy_flair layered by load order, slots, Luau UI API, gamepad-complete input, scaling, a11y | draft, design complete (2026-10-11) | `sf_ui`, `sf_ui_bevy`, `sf_ui_css`, `sf_ui_screens`, `sf_ui_lua`, `sf_devui` | M5 |
| [journal-ui.md](journal-ui.md): journal screen logic (book pagination behind a `Paginator` trait, active/finished quests, topic index with letter tabs, search, per-player new markers) | implemented (2026-10-22) | `sf_rpg_ui` | M5 |
| [scripting.md](scripting.md): Luau runtime via mlua (state per mod, sandbox, memory/CPU limits), schedule slots, curated versioned API + `.d.luau`, named timers and plain-data state, hot reload, error isolation | draft (2026-10-13) | `sf_script`, `sf_script_bevy` | M5 |
| [mods.md](mods.md): mod pipeline end to end (repository, profiles, load order, VFS/records/scripts/UI per mod, RON record patches with field-level conflicts, reports, hot reload, bake fingerprints) | draft (2026-10-13) | `sf_mods`, `sf_vfs`, `sf_records`, `sf_bake` | M5 |
| [locomotion.md](locomotion.md): intent, trajectory springs, ground probes, kinematic mover | draft (2026-10-11) | `sf_controller`, `sf_controller_bevy` | M3 |
| [action.md](action.md): action layer (commands, move rules, chooser on the fixed tick, motion owners, windows, mode profiles, stat bands, interaction points and edges, co-op readiness), phases A1-F | draft, awaiting owner review (R11b, 2026-10-19) | `sf_action`, `sf_action_bevy`, `sf_interaction` | M4 P11 (A1-B5), M10 (E) |
| [camera.md](camera.md): third person (orbit, collision, optional lock-on), first person from a baked camera track, per-move look limits, framed moves, comfort | draft, awaiting owner review (R11b, 2026-10-19) | `sf_camera`, `sf_camera_bevy` | M4 P11 (D1) |
| [net.md](net.md): 2-4 player co-op, host-authoritative listen server, own-character prediction, replication and ownership per system, context player, multi-region host, saves, quest model, crate criteria, the 8 net-ready rules and the spec audit | spec (N0, 2026-10-19) | `sf_net` (N2) | co-op N0-N8 |
| [physics.md](physics.md): rapier3d behind Bevy-free `sf_physics`, floating-origin rebase, sleeping items + `body.cook`, controller pushes, carry/throw, `ragdoll.cook` + powered ragdolls, bone-lattice cloth | draft (2026-10-12) | `sf_physics`, `sf_physics_bevy`, `sf_ragdoll`, `sf_anim`, `sf_character`, `sf_controller` | M4 |
| [rig-profile.md](rig-profile.md): limb-chain rig profile for any skeleton (girdles, N limbs, spine, neck/head, tails, wings), auto-detection, creature support | draft (2026-10-11) | `sf_anim`, `sf_anim_graph`, `sf_controller`, `sf_character` | M3 |
| [weather.md](weather.md): weather state machine + presets as records, Nubis clouds + cloud shadows, froxel/height/ground fog + god rays, wind field, precipitation with occlusion, surface state (wetness, puddles, snow, ash), distant shadows (CSM + shadow-height map); near-term shadow-pop and Ghostfence-pop fixes | draft (2026-10-11) | `sf_wind`, `sf_weather`, `sf_atmos`, `sf_surface` + `_bevy` plugins, `sf_precip_bevy` | M6 (near-term fixes: M2) |
| [industry-audit.md](industry-audit.md): gap audit of streaming, rendering and LOD vs shipped engines; Bevy 0.20 per-entity costs; size-based draw distance; jobs J1–J9 | audit (2026-10-12) | `sf_model`, `sf_world`, `sf_world_bevy`, `sf_lod_bevy` | M2 |
| [water.md](water.md): water bodies at any height (ocean, lake, river, pool) + CPU water query, FFT ocean (bevy-aqua evaluated: ported, not adopted), shoreline, single-layer water pass with SSPR, rivers as baked flow (hydrology), underwater (froxel medium, waterline), buoyancy (pontoons via `sf_physics`), wading hooks | draft (2026-10-13) | `sf_water`, `sf_water_bevy` | M6 |
| [render-order.md](render-order.md): the frame graph in order (passes, phases, bands), per-material fog/sort/depth/prepass/shadow inventory, six render-order rules, UE/Frostbite comparison, violations RO-1..RO-12, the `translucent_guard` test | audit + rules (2026-10-19) | `sf_render_order_bevy` (guard), every `*_bevy` crate that draws | M6 |
