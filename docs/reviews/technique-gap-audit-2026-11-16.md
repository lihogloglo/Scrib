# Technique gap audit, 2026-11-16

Read-only audit by the `technique-gaps` research agent. No code changed, nothing built, launched or benched.
Independent of `frame-waste-audit-2026-11-16.md` (cost side).

**How to read this.** Part 1 checks every "match / partial / have" claim I could reach against the code path the
game runs by default. Part 2 lists the standard techniques per cost bucket with have / partial / missing. Part 3
ranks the ten most valuable missing ones. Part 4 lists doc lines to correct.

**Limits of this pass (stopped at the ~100k context hand-off):**
- No web research was done. Citations already in the wiki are reused; the others in Part 2 are from memory of
  well-known talks and engine docs and are marked `[mem]`. Verify a `[mem]` source before a spec quotes it.
- Expected impacts in Part 3 are estimates, not measurements. The lamp-shadow ceiling (ROADMAP 1.23 slice a:
  walk with lamp shadows off vs shipped) is still unmeasured, so every lamp item is "a share of that ceiling".
- Not audited (left for a follow-up): streaming spec table against `sf_world_bevy` code, water, weather,
  sky-lighting, gi and far-field spec tables, animation compression, impostor and light-sprite code paths,
  audit rows 2, 12, 16, 24, 29, 33-36. These are UNVERIFIED below or absent.

## Part 1: claims against the default code path

Verdicts: TRUE MATCH / PARTIAL / FALSE (not on by default, or another technique under the name) / UNVERIFIED.
"Stale" means the doc line no longer describes the code, in either direction.

| # | Claim | Source | Verdict | Evidence |
|---|---|---|---|---|
| 1 | Static caster caching for lamps ("Match", "match since reset B1") | `docs/wiki/industry/shadows.md:38-39`; `superfargoth/docs/specs/shadows.md:327` | **FALSE** | The static cache plugin is compiled out: `POINT_SHADOW_CACHE = false`, `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37`, guard `:211`. What runs is skip-if-unchanged: a face is skipped only when its casters, view and projection are all unchanged (`superfargoth/vendor/bevy_pbr/src/render/light.rs:1454-1457`). A mover in a light's range dirties it and the dirty faces redraw whole, statics included. Every face stays a render view with its own schedule (`docs/wiki/bevy/shadows-and-lights.md:49-52`). RDR2 (baked cubes), UE, HDRP and Doom keep static depth and draw only movers. |
| 2 | "Lamps get a few budgeted cube maps whose static casters are cached; moving casters are redrawn over the cache", and the "Lamp cache" paragraph | `docs/wiki/engine/shadows.md:6-7`, `:44-48` | **FALSE** | Same switch. The page contradicts itself at `:71-73` ("Standard fix not built: cached static depth + movers drawn every frame") and `docs/wiki/engine/frame-and-views.md:23-25` says the plugin is not in the game. |
| 3 | "Dynamic casters over cached copy" (table row marked "stale, see resets") | `superfargoth/docs/specs/shadows.md:328` | **FALSE** | Nothing composites movers over a static copy. At most 2 mover lights plus 1 LRU light redraw per frame (`light.rs:1202`, `:1210`), which is the SH-21 flicker (`docs/ledger/shadows.md:6`). |
| 4 | Standing fact "The lamp cache is needed: moving casters defeat Bevy's skip rule", and "patch kept, cache kept" | `docs/ledger/shadows.md:35`, `:21` | **FALSE (stale)** | The measurement stands (B1: 11.4 vs 6.9 ms without the cache), but the cache has been off since 2026-10-21 (`docs/ledger/shadows.md:8`). The fact that it is needed and the fact that it is off were never put side by side. |
| 5 | Shadowed-light budget with importance and hysteresis ("Match") | `docs/wiki/industry/shadows.md:38`; `specs/shadows.md:325` | **PARTIAL** | On: `budget_shadows`, `superfargoth/crates/sf_sky_bevy/src/shadow_budget.rs:280`, High 8 maps at 512 (`render.rs:28`). Missing against HDRP, UE and Cyberpunk: importance is `priority x (range/d)^2 x min(1, range/d)^2` (`specs/shadows.md:234`), a function of the lamp and the viewer only. No sun or ambient term at the receiver, no screen coverage. So a day walk shadows as many lamps as a night walk (46 vs 51 views, ROADMAP 1.23). |
| 6 | "Selection by contribution including off-screen lights" ("Match") | `docs/wiki/industry/shadows.md:39`; `specs/shadows.md:326` | **PARTIAL** | The off-screen half is true (SH-21b, `docs/ledger/shadows.md:5`). "Contribution" is not: see row 5. The name promises more than range over distance. |
| 7 | "Our local lights: one shadowed point light on High" | `docs/wiki/industry/shadows.md:35` | **FALSE (stale)** | High is 8 lamps at 512 (`render.rs:28`; `docs/wiki/engine/shadows.md:41-42`). The stale line hides that we shadow more local lights than the page's own references suggest is normal. |
| 8 | "ours span 10 m to 504 m" (cascades) | `docs/wiki/industry/shadows.md:30-31` | **FALSE (stale)** | High is 3 cascades to 205 m (`docs/wiki/engine/shadows.md:14`). |
| 9 | Gap line "no cascade cache in Bevy" | `docs/wiki/industry/shadows.md:41` | **PARTIAL (stale)** | The far-cascade hold F3 is on by default (`render.rs:112`; `sf_sky_bevy/src/cascade_hold.rs:90`). It is an update-rate hold (max age 4 frames), not a static cache or a scroll. |
| 10 | Contact shadows "sun and moon only; lamps none since reset A1" | `specs/shadows.md:330`; `docs/wiki/industry/shadows.md:41` | **FALSE (stale)** | Lamps have them on High and Ultra (`docs/wiki/engine/shadows.md:28-29`; `docs/ledger/shadows.md:16`). |
| 11 | "We use the opt-in `NoCpuCulling` path on near statics" | `docs/wiki/industry/gpu-driven-rendering.md:35` | **FALSE as a "have"** | Off unless `BEVYRROWIND_GPU_CULL_STATICS=1` (`bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs:295-296`). The default path CPU-culls every entity per view. Re-measured at parity on 2026-11-16 (`docs/ledger/perf.md:12`), so it is a candidate, not a feature. |
| 12 | "are building a retained static scene" | `docs/wiki/industry/gpu-driven-rendering.md:35-36` | **FALSE (stale)** | Frozen by the owner, `--scene` opt-in (`bevyrrowind/docs/ROADMAP.md:77`; `docs/wiki/engine/statics-and-lod.md:34`). |
| 13 | "Bevy 0.20 already does GPU preprocessing, multi-draw indirect and bindless" | `docs/wiki/industry/gpu-driven-rendering.md:33-34` | **TRUE MATCH** (mechanism) | Stock `Culling` mode (`docs/wiki/bevy/render-and-visibility.md:94-99`). It does not save what a GPU-driven engine saves: CPU visibility runs first, per entity per view (`:14-17`, `:39-45`), and each shadow view runs its own schedule (`:108-113`). The page says so; keep it. |
| 14 | Audit row 3, GPU-only culling "partial" | `superfargoth/docs/specs/industry-audit.md:120` | **PARTIAL** (accurate) | As row 11. |
| 15 | Audit row 4, screen-size LOD selection "have" | `industry-audit.md:121` | **PARTIAL** | The 1 px rule is applied at bake for 1440p and 70 degrees and becomes fixed distances: handoffs 400 / 2000 / 8000 m x preset bias (`docs/wiki/engine/statics-and-lod.md:26-27`; `docs/wiki/industry/fableflight.md:20`). Nothing re-evaluates against the real FOV or the DLSS internal resolution. Engines test projected size at run time. |
| 16 | Audit row 5, dithered LOD crossfade "have" | `industry-audit.md:122` | **TRUE MATCH** | `VisibilityRange` margins (`statics-and-lod.md:30`). Cost note: the dither is why the exterior had no depth prepass by default (`bevyrrowind/apps/bevyrrowind/src/main.rs:355-358`). |
| 17 | Audit row 6, preset bias "partial: presets not wired" | `industry-audit.md:123` | **Stale** (now have, per wiki) | `statics-and-lod.md:27` ("times the preset bias"). Code not read. |
| 18 | Audit rows 7, 8, 9: HLOD, tree impostors, far light sprites "have" | `industry-audit.md:124-126` | **TRUE MATCH** (wiki level) | `FarFieldPlugin` is added (`docs/wiki/engine/frame-and-views.md:33-35`) and runs `build_impostor_arrays`, `spawn_upper_nodes`, `residency_pass` (`statics-and-lod.md:26-32`). Systems not read line by line. |
| 19 | Audit row 10, precombined near geometry "missing" | `industry-audit.md:127` | **Stale; now PARTIAL** | Default on (`superfargoth/crates/sf_world_bevy/src/stream.rs:135`). Short of Fallout 4 precombines: a bucket is still one entity per LOD level per part (`statics-and-lod.md:21-24`), and one touched member drops the whole cell to per-placement entities (`:18-20`). |
| 20 | Audit row 11, automatic instancing "have" | `industry-audit.md:128` | **TRUE MATCH** | Stock bins plus MDI. Note the precombine trades it away for near statics (merged meshes are unique). |
| 21 | Audit row 13, shadow caster culling by size "missing" | `industry-audit.md:130` | **Still missing; numbers stale** | Now 3 cascades to 205 m, not 4 to 504 m. The caster cull that existed was deleted in reset A2 (`docs/ledger/shadows.md:22`). No size-based `NotShadowCaster` in the spawn path (grep of `sf_world_bevy/src`: tests and warm-up only). |
| 22 | Audit row 14, static shadow caching "missing" | `industry-audit.md:131` | **Stale; now PARTIAL** | Rows 1 and 9: skip-if-unchanged for lamps, hold for far cascades. No static and dynamic split anywhere. |
| 23 | Audit row 15, occlusion culling "partial" | `industry-audit.md:132` | **PARTIAL; default state UNVERIFIED** | `OcclusionCulling` goes on the camera only with `start.prepass` (`exterior.rs:906-907`); `main.rs:355-360` says the default has none but "`--dlss` adds the same prepass", and ROADMAP 1.22 says "Kept: depth prepass (owner)". Whether the default High + DLSS path runs it was not traced. No `OcclusionCulling` on the sun light found, so cascades have none. |
| 24 | Audit row 20, upload pacing "partial" | `industry-audit.md:137` | **PARTIAL** (accurate) | Bevy uploads every new mesh in the frame it arrives (`docs/wiki/bevy/render-and-visibility.md:102-104`). |
| 25 | Audit row 22, PSO pre-warm "partial: not built" | `industry-audit.md:139` | **Stale** (built: W2-W4) | `docs/ledger/perf.md:10`, `:14-17`. |
| 26 | Audit row 27 and "animation budget tiers" ("Match") | `industry-audit.md:144`; `docs/wiki/industry/character-animation.md:43-44` | **PARTIAL** | Distance tiers run. The significance budget is `AnimBudget` default `None`: "distance only" (`docs/wiki/engine/animation.md:20`). UE's Animation Budget Allocator is a ms budget, not a distance table. |
| 27 | Audit row 28, light LOD "partial" | `industry-audit.md:145` | **PARTIAL** (accurate) | "No light budget or draw distance exists beyond the shadow budget" (`docs/wiki/engine/lights.md:24`). 286 lights stay resident on a day walk. |
| 28 | Audit row 30, incremental render tables "partial" | `industry-audit.md:147` | **Stale** (patched) | Own incremental `extract_visibility_ranges` patch (`superfargoth/vendor/README.md`, "own" row for `bevy_render` `view/visibility/range.rs`). |
| 29 | Layered graph, inertialization, stride and orientation warp, foot IK ("Match") | `docs/wiki/industry/character-animation.md:43-44` | **TRUE MATCH** (wiki level) | Ops are in the humanoid graph with gates default 1 (`docs/wiki/engine/animation.md:23-33`, `:95-96`). "ACL-style compression": UNVERIFIED. |
| 30 | Terrain far self-shadowing ("match") | `superfargoth/docs/specs/terrain.md` "Industry comparison" | **TRUE MATCH** (wiki level) | Far shadow map from the height clipmap (`docs/wiki/engine/shadows.md:30-32`). Terrain only: objects past the last cascade cast nothing (`:76`). |
| 31 | Partition, rings, hysteresis, time budget, fade-hidden bursts ("We match") | `docs/wiki/industry/open-world-streaming.md:42`; `industry-audit.md:135-136`, `:138`; `specs/streaming.md:743+` | **UNVERIFIED** | Not traced in `sf_world_bevy` this pass. |

**Tally: 9 FALSE, 11 PARTIAL.**
- FALSE (9): rows 1, 2, 3, 4, 11, 12 claim something we do not run by default; rows 7, 8, 10 are stale facts.
- PARTIAL (11): rows 5, 6, 9, 14, 15, 19, 22, 23, 24, 26, 27.
- Stale in our favour (the doc says missing or unbuilt, the code has it): rows 17, 25, 28.
- TRUE MATCH: rows 13, 16, 18, 20, 29, 30 (18, 29 and 30 at wiki level only). UNVERIFIED: row 31.

The pattern behind the FALSE rows: all six real ones are lamp shadows or GPU culling, the two buckets that cost
the most, and each names an opt-in or a switched-off component as if it ran.

## Part 2: technique catalogue

Status: **have** / **partial** / **missing** / **rejected** (with the ledger line). `[mem]` = cited from memory.

### 1. Local-light shadows (up to 8 lamps x 6 faces, most of 46 views)

| Technique | Who | Status |
|---|---|---|
| Static caster cache, movers drawn over a copy | UE cached shadow maps for movable lights `[mem]`; HDRP mixed cached; Doom 2016 (Sousa, Geffroy, SIGGRAPH 2016 `[mem]`); Cyberpunk static and dynamic slices per map (Dybisz, Witanowski 2021) | **missing in the game** (built, off: Part 1 row 1). Known: SH-21a, ROADMAP 1.23c |
| Baked static shadow maps for static lights | RDR2 baked cubes (imgeself 2020); UE Stationary lights, Unity Shadowmask `[mem]` | **missing**. Fits our content: Morrowind lamps and statics never move, so the cube can be made at `bevyrrowind bake` or at cell load, not captured during play |
| A cached face is not a view | Follows from the two above: a light with no mover in range costs no CPU | **missing**: every face is a render view and schedule (`bevy/shadows-and-lights.md:49-52`) |
| Per-face culling against the camera frustum and visible receivers | Doom 2016, UE `[mem]`; Cyberpunk culling shapes (2021) | **missing**: decisions are "per light, never per face" (`light.rs:1458-1459`); SH-21b skips only whole lights outside every frustum |
| Hemisphere or spot conversion for wall and ceiling lamps; drop faces with no casters | Common practice; UE artists place spots `[mem]` | **missing**: every lamp is a 6-face point light (`mw_assets/src/lights.rs:126`). A lamp on a wall has 1-3 faces that can never shadow anything visible |
| Dual-paraboloid (2 views) or tetrahedral (4 views) maps | Brabec et al. 2002; Liao, GPU Pro 1 `[mem]` | **missing**; dual-paraboloid is logged "later" (ROADMAP 1.22) |
| All faces in one pass into an atlas (one view, one encoder) | Doghramachi, "Tile-Based Omnidirectional Shadows", GPU Pro 6 `[mem]`; UE VSM one-pass projection | **missing**. Bevy 0.20 multiview support not checked; do not assume it |
| Atlas with per-light resolution by screen size | Doom 2016, HDRP dynamic rescale, RDR2 three tiers (imgeself) | **missing**: one cube size per preset (`specs/shadows.md:329`) |
| Update-rate tiers by distance or importance | Cyberpunk throttled refresh; Clockwork movers at 30 Hz (`specs/shadows.md:220-221`) | **partial**: 2 mover + 1 LRU light per frame, the same for every light (`light.rs:1202`, `:1210`); no distance tier |
| Selection by contribution against sun and ambient at the receiver | Cyberpunk budget by visibility and distance; HDRP shadow fade distance | **missing** (Part 1 row 5). Known: ROADMAP 1.23b |
| Shadow LOD for casters, skip small casters | UE `r.Shadow.RadiusThreshold` (wiki); shadow LOD bias `[mem]` | **missing**: shadow views take the camera's LOD (`ShadowLodOrigin`, `bevy/render-and-visibility.md:30`), no size cut (Part 1 row 21) |
| Budget, fade, hysteresis, off-screen holders | HDRP, UE, Clockwork | **have** |
| 1-tap PCF and fetch early-out for lamps | own patch B2 | **have** (`docs/ledger/shadows.md:20`) |
| Uncached lamp shadows | | **rejected** (SH-9, `docs/ledger/shadows.md:28`) |

### 2. Local lights without shadows (286 point lights)

| Technique | Who | Status |
|---|---|---|
| Clustered culling | Doom 2016, UE, HDRP | **have** (stock GPU clustering, `engine/lights.md:22`) |
| Draw distance with fade per light | UE `MaxDrawDistance`, HDRP fade distance, OpenMW 8192 units (`specs/lights.md:57-60`) | **missing** (designed P2, `specs/lights.md` "Design" 2) |
| Cut by screen radius | UE `r.MinScreenRadiusForLights` `[mem]` | **missing** |
| Day gating or dimming | Time-of-day light states in RDR2 and Cyberpunk `[mem]`; in an HDR pipeline a lamp below about 1 % of the sun's illuminance is invisible | **missing**: no code gates lamps or their shadows by daylight (ROADMAP 1.23) |
| Far lights as sprites | Bethesda, RAGE | **have** |
| Reach from perception (shorter range) | OpenMW 1.75x, Clockwork 1.0x | **rejected** by the owner (LT-reach, `docs/ledger/shadows.md:13`) |
| Flicker as intensity only | standard | **have** (`engine/lights.md:14`) |

Measured value is small: about 0.5 ms GPU standing, nothing on the walk (`specs/lights.md` "Measured").

### 3. Sun shadows

| Technique | Who | Status |
|---|---|---|
| Far-cascade update rate | Cyberpunk distant cascades over several frames | **have** (F3 hold, -1.0 ms walk) |
| Staggered cascades (one far cascade per frame) | common `[mem]` | **UNVERIFIED**: F3 holds cascades 1+ together as far as the wiki says |
| Cascade scrolling or static cache | Acton, CSM scrolling, GDC 2012 `[mem]`; UE cached far cascades | **missing**; a cached far static map was built, never adopted, deleted (`docs/ledger/shadows.md:29`, `:22`) |
| Caster culling by projected size per cascade | UE `r.Shadow.RadiusThreshold` | **missing** (deleted in A2) |
| Per-cascade caster sets (stand-ins far, detail near) | FableFlight (`industry/fableflight.md:23`); UE far shadow cascade from HLOD `[mem]` | **missing** |
| Occlusion culling of shadow views | Cyberpunk main-view and shadow-view occlusion (2021); Bevy `OcclusionCulling` on a directional light (`bevy/render-and-visibility.md:100-101`) | **missing**, no ledger line found |
| Far object shadows | UE distance fields; Cyberpunk distant cascades to the map edge | **partial**: terrain only |
| Cascade count and distance per preset | all | **have** (3 on High; 2 is opt-in, no win with the hold) |
| One encoder per cascade | UE parallel command lists | **partial**: opt-in F1, no win with the hold |

### 4. Per-view CPU cost in Bevy

| Technique | Who | Status |
|---|---|---|
| Retained bins, MDI, bindless | Bevy 0.20 stock | **have** |
| GPU-driven culling per view, no CPU list | UE GPUScene, Unity GRD, Decima clusters (GCAP 2017) | **partial**: opt-in `NoCpuCulling`, parity (Part 1 row 11). Known: F8 |
| Fewer views | every budgeted engine | **missing** as a policy: the day walk carries about 40 lamp faces |
| Many views in one pass | bucket 1, one-pass atlas | **missing** |
| Hierarchical culling (cell or cluster bounds first) | Decima static tiles; Frostbite grid (Collin 2011 `[mem]`) | **missing**: Bevy scans every entity per camera and per light (`bevy/render-and-visibility.md:14-17`, `:39-45`). Known in part: F6 |
| Fewer command buffers per submit | | **missing**. Known: F5 |
| Pre-recorded draws for unchanged views (wgpu `RenderBundle`) | | **UNVERIFIED** against Bevy 0.20; low value once batch sets are few |

### 5. Statics, streaming, LOD

| Technique | Who | Status |
|---|---|---|
| HLOD, impostors, far light sprites | UE, Bethesda, GTA | **have** |
| Precombine | Fallout 4, OpenMW object paging | **partial** (Part 1 row 19) |
| Instancing of repeated meshes | UE, Unity | **have** (stock), lost inside precombines |
| LOD by screen size at run time | UE, 3D Tiles SSE | **partial** (baked distances, Part 1 row 15) |
| Distance cull by object size | UE cull distance volumes, OpenMW | **have** (J1, `docs/ledger/statics.md:29`: 3 % of placements at High) |
| Size-tiered streaming (do not load what cannot draw) | RAGE `lodDist`, UE runtime grids | **missing** per audit row 2; UNVERIFIED |
| GPU HZB occlusion | UE, Unity, Bevy | **partial**: needs the prepass; measured slower in Balmora (audit row 15); default state unverified |
| Software occluders | Frostbite, Cyberpunk CPU occlusion buffer | **missing** |
| Portals or PVS for towns and interiors | Bethesda room bounds | **missing**; `specs/portals.md` exists, not read |
| Terrain horizon occlusion | terrain engines | **missing** (audit row 16) |
| Merged depth-only shadow proxy per cell | follows from precombine plus our depth-only key patch (`vendor/README.md`, "depth-only material key") | **missing** |

## Part 3: the ten most valuable missing techniques

Walk = CPU bound (11.42 ms, render thread about 9.9, target 8.3). Standing = GPU bound (10.27 ms).
"Known" = already in the ROADMAP; listed because it still ranks. Impact is an estimate until 1.23a is measured.

| # | Technique | Helps | Expected impact | Cost in Bevy | Source |
|---|---|---|---|---|---|
| 1 | Gate lamp shadows by contribution against sun and ambient (known, 1.23b) | walk, day | Up to the whole lamp-shadow ceiling by day (about 40 of 46 views) | S: one term in `shadow_importance` | Cyberpunk 2021; HDRP |
| 2 | Real static cache: static depth per lamp made at bake or cell load, movers over a copy, a clean face is not a view (known, 1.23c) | walk, night; fixes SH-21 flicker | Most of the night ceiling; B1 measured 11.4 vs 6.9 ms for 8 lamps | M-L: revive `sf_shadow_cache_bevy` on the vendored path, or bake cubes | RDR2 (imgeself); UE; HDRP; Doom 2016 |
| 3 | Per-face culling: skip faces outside the camera frustum, faces with no casters, and the wall-side faces of mounted lamps | walk, any hour | Roughly half the faces of a shadowed lamp never reach a visible receiver; estimate, count it first with `shadow_refresh` debug | S-M: `skip_unchanged_shadow_views` decides per light today | Doom 2016; UE `[mem]` |
| 4 | One-pass lamp atlas: every lamp face in one view and one encoder (tetrahedral or paraboloid layout) | walk, night | Attacks the 38 encoders and serial submit directly: about 40 views become 1 | L: own vendored render path | GPU Pro 6 `[mem]`; UE VSM |
| 5 | Merged depth-only shadow proxy per cell, with caster culling by projected size per cascade and face | walk and standing | Fewer caster entities in the cascade and lamp scans (1.27 + 1.16 thread-ms today, `docs/ledger/perf.md:18`) and fewer shadow batch sets | M: bake product next to the precombine | UE radius threshold; FableFlight per-cascade sets |
| 6 | GPU-only culling that wins (known, F8) | walk | Removes the per-view CPU scan for statics; parity today | M | UE GPUScene; Decima |
| 7 | Shadow update-rate and resolution tiers by distance (near lamp every frame at full size, far lamp every Nth at half) | walk, night; standing GPU | Smooths the 2 + 1 refresh cap that causes SH-21 | M (rate), L (atlas sizes) | Cyberpunk; RDR2 tiers; Doom atlas |
| 8 | Occlusion culling on shadow views (`OcclusionCulling` on the sun first: a one-line experiment) | standing | GPU shadow-pass draws in towns; no CPU win | S to try | Cyberpunk 2021; Bevy `bistro` |
| 9 | Run-time screen-size LOD and size-tiered streaming | walk | Fewer resident entities for every scan; unmeasured | M | UE, RAGE, 3D Tiles |
| 10 | Light draw distance with fade, screen-radius cut, day gate for unshadowed lamps | standing | At most about 0.5 ms GPU | S | UE, HDRP, OpenMW |

Order of work the numbers suggest: measure 1.23a, then 1 and 3 (cheap, cut views at any hour), then 2, and
decide 4 only from what is left at night.

## Part 4: doc lines to correct (not edited)

1. `docs/wiki/industry/shadows.md:38-39`: replace "static caster caching (vendored bevy PRs #26149 and #26161 skip
   unchanged views), selection by contribution including off-screen lights" with: "selection all around the
   viewer, on screen or not. Partial: lamp faces are skipped only while nothing in them changed (vendored fork of
   bevy #26149 and #26161); this is not static caster caching: a mover redraws the face whole, every face stays a
   render view, and the static cache `sf_shadow_cache_bevy` is off (`POINT_SHADOW_CACHE`,
   `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37`). The importance is range over distance, with no sun
   or ambient term."
2. `docs/wiki/industry/shadows.md:41`: replace "no cascade cache in Bevy. Contact shadows: sun and moon only." with
   "no static cascade cache (we hold far cascades for up to 4 frames). Contact shadows: sun and moon, lamps on
   High and Ultra."
3. `docs/wiki/industry/shadows.md:30-31`: replace "ours span 10 m (first bound) to 504 m" with "ours: 3 cascades
   to 205 m on High, 4 to 280 m on Ultra (`docs/wiki/engine/shadows.md`)".
4. `docs/wiki/industry/shadows.md:35`: replace "one shadowed point light on High" with "8 shadowed lamps at 512 px
   on High, 12 at 1024 on Ultra".
5. `docs/wiki/industry/gpu-driven-rendering.md:35-36`: replace "We use the opt-in `NoCpuCulling` path on near
   statics and are building a retained static scene (upstream bevy#25978) so phase items become O(batch sets)."
   with "Neither answer is on by default: `NoCpuCulling` on near statics is opt-in
   (`BEVYRROWIND_GPU_CULL_STATICS=1`, parity on 2026-11-16) and the retained static scene is frozen (`--scene`,
   ROADMAP 1.16)."
6. `docs/wiki/engine/shadows.md:6-7`: replace "Lamps get a few budgeted cube maps whose static casters are
   cached; moving casters are redrawn over the cache." with "Lamps get a few budgeted cube maps; a face is redrawn
   whole when anything in it changed and skipped otherwise. There is no static caster cache in the game."
7. `docs/wiki/engine/shadows.md:44`: start the "Lamp cache" bullet with "**Not in the game** (`POINT_SHADOW_CACHE
   = false`, `mw_environment_bevy/src/render.rs:37`, since 2026-10-21). As built:". At `:62`, note that
   `shadowcache` is a no-op (as `frame-and-views.md:65-66` says).
8. `docs/ledger/shadows.md:35` (standing facts, not a ledger line): replace with "A static lamp cache is needed
   (moving casters defeat Bevy's skip rule, B1) but none runs: `sf_shadow_cache_bevy` is off since 2026-10-21."
9. `superfargoth/docs/specs/shadows.md:327`: verdict "**match** since reset B1" becomes "**GAP**: cache off in the
   game since 2026-10-21; skip-if-unchanged only". `:328`: "**GAP**: no mover compositing; 2 + 1 lights redraw
   per frame". `:330`: lamps have contact shadows on High and Ultra.
10. `superfargoth/docs/specs/industry-audit.md`: rows 6 (`:123`), 10 (`:127`), 14 (`:131`), 22 (`:139`), 30
    (`:147`) are built or partly built; row 13 (`:130`) still missing but now "3 cascades to 205 m".
