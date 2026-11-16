# Lamp shadow cache (CW-4)

Design, 2026-11-16. ROADMAP 1.25 CW-4 / 1.23c, visual check SH-21a. Not built. Parent spec: [shadows](shadows.md)
("Caching", "V1 as built", "Reset B1 as built"); wiki `docs/wiki/engine/shadows.md`; ledger `docs/ledger/shadows.md`.
Lamp shadow quality is hard problem H2: this spec covers cost and the SH-21a flicker only, not which lamps cast.

## Problem, with the numbers we have

A shadowed lamp is a cube map, six faces. On the live path (vendored `skip_unchanged_shadow_views`,
`vendor/bevy_pbr/src/render/light.rs:1468`) a face is redrawn whole when anything in it changed and skipped otherwise.
That is a skip rule, not a static cache. What it still costs:

- **Main thread.** Stock `bevy_light::check_point_light_mesh_visibility` (`bevy_light-0.20.0/src/lib.rs:517`) scans every
  CPU-culled mesh for every shadowed lamp each frame: 1.16 thread-ms, and +0.46 ms main/busy when 8 lamps are scanned
  instead of ~3 (ledger 2026-11-15, lamp-select), so about 0.09 ms main per lamp.
- **Extract and render.** `extract_shadow_caster_changes` (`light.rs:1220`) hashes every caster of every face each
  frame. Every face stays a view entity with a retained shadow phase: 42 views, 2148 bins, 168 pipelines on the day walk,
  48 views and 3603 items in the census interior (frame-waste audit §4). A skipped face runs no schedule and adds no
  encoder (`vendor/bevy_core_pipeline/src/schedule.rs:253`); a drawn face adds one (~0.03 ms of `queue.submit`,
  `docs/reviews/submit-cost-diagnosis-2026-11-16.md`).
- **Redraw frames.** One walker dirties 1-3 faces per lamp in range, and each redraw draws every static in the face
  again: up to 2917 work items and 191 batch sets on a walk frame (the stutter census' 3.3-4 ms frames).
- **Flicker (SH-21a).** Because a redraw is expensive, only 2 mover + 1 LRU lights redraw per frame
  (`light.rs:1202`, `:1210`); each cube holds an older pose on a different frame.

Not measured: the lamp-shadow ceiling on the walk (1.23a). Slice S0 measures it before anything is built.

## Reuse check

- **Bevy 0.20 stock.** No shadow caching: point shadow textures come from `TextureCache` and every view with a phase is
  cleared and redrawn (`docs/wiki/bevy/shadows-and-lights.md` §3). One global cube size, no atlas, no static/dynamic
  split. Stock already gives the two hooks this design needs: the lamp caster scan skips a light whose
  `shadow_maps_enabled` is false (`bevy_light/src/lib.rs:584`), and `CubemapVisibleEntities` is a public component.
- **Upstream** (web search 2026-11-16; statuses as seen in search results, not re-read on GitHub):
  [#26149](https://github.com/bevyengine/bevy/pull/26149) slim schedule per point/spot shadow view and
  [#26161](https://github.com/bevyengine/bevy/pull/26161) skip unchanged views: both vendored already; #26161 has no
  static/mover split. Issues [#25576](https://github.com/bevyengine/bevy/issues/25576) (a full schedule per shadow
  view) and [#26059](https://github.com/bevyengine/bevy/issues/26059) (leaked face views; our `drop_stale_light_views`).
  [#24625](https://github.com/bevyengine/bevy/pull/24625) per-camera point/spot shadow maps: unrelated. A discussion
  "Pre-baked shadow maps" (#16510) exists; not read. No upstream issue or PR for static caster caching was found.
  A fork PR (megalike/bevy#26, "KeepShadowMaps") keeps a light's maps while nothing moves: the #26161 class again.
- **Crates.** None found for cached point shadows on Bevy 0.20.
- **Ours.** `sf_shadow_cache_bevy` (compiled out; see "Decision"), the per-light static lists of
  `sf_sky_bevy/src/gpu_culled_casters.rs` (H1), and in the vendored path: a persistent cube slot per light
  (`light.rs:1946`), a texture that never shrinks (`:2215`) and `CachedShadowViewState` (`:2869`).

## Industry comparison

| Engine | What it does | Source |
|---|---|---|
| UE4/5 | Cached shadow maps for movable point and spot lights that stay in place: depth of Static/Stationary primitives is cached, movable primitives are drawn each frame over a copy; materials with World Position Offset are not cached; a memory budget caps the cache (`r.Shadow.WholeSceneShadowCacheMb`), `r.Shadow.CachedShadowsCastFromMovablePrimitives=0` drops movers and the copy. Epic's example: three movable point lights 14 ms of shadow depths, about 1 ms cached | [Epic, movable lights](https://dev.epicgames.com/documentation/en-us/unreal-engine/movable-light-mobility-in-unreal-engine); cvars: [third-party cvar reference](https://indxzero.github.io/ue544cvarwiki/articles/r.shadow.cachewholesceneshadows) |
| Unity HDRP | Cached shadow atlas for punctual lights; update mode OnEnable or OnDemand; "Always draw dynamic" (mixed cached): static casters by update mode, dynamic casters every frame after a blit from the cached atlas to the dynamic atlas; a moved light must request an update; no dynamic rescale for cached maps | [HDRP shadow update mode](https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@16.0/manual/shadow-update-mode.html) |
| RDR2 | Most static point lights use baked shadow cube maps; runtime maps only near the player | [imgeself 2020](https://imgeself.github.io/posts/2020-06-19-graphics-study-rdr2) (frame capture, third party) |
| Cyberpunk 2077 | Fixed shadow-map budget per frame by visibility and distance, throttled refresh, a static and a dynamic slice per map | [Dybisz, Witanowski, SIGGRAPH 2021](https://history.siggraph.org/wp-content/uploads/2022/06/2021-Talks-Dybisz_Shadows-Optimizations-in-Cyberpunk-2077.pdf) |
| Doom 2016 | One atlas; a light's tile is redrawn only when something in its volume moved | Sousa, Geffroy, SIGGRAPH 2016 (from memory, not re-read) |
| Frostbite, Decima | No public source found for local-light shadow caching | `docs/wiki/industry/shadows.md` |

Verdict: the design below is UE's and HDRP's mixed cache (static depth kept, movers drawn over a copy each frame) plus
the consequence none of them needs to state because their shadow passes are not ECS views: a light with nothing moving
does no CPU work at all. The optional static-only tier (S6) is RDR2's split.

## Design

Terms. *Static*: a caster that has not moved relative to the lamp. *Mover*: skinned or morphed meshes, anything marked
movable, and rigid casters that moved recently. *Live cube*: the cube-array layers the PBR shader samples (Bevy's own
texture, unchanged). *Store*: our persistent copies of static faces. *Clean face*: its live layer holds static depth
and nothing has been drawn over it.

### 1. Where static depth comes from

**Rendered once, at run time, when a still lamp gains a shadow slot. Not baked to disk.**

- A capture is the same shadow pass Bevy runs today, with the face's static list only, cleared first. It is spread over
  frames (`capture_faces_per_frame`, 2); the lamp's shadow strength stays 0 until all its faces are captured, then the
  existing 0.3 s fade runs. No new render path, no bake format.
- Faces whose static list is empty are not drawn: their live layer is copied from one permanently cleared store layer.
- Captures use the lamp's position as LOD origin, not the camera's (`RenderShadowLodOrigin` is one global today,
  `vendor/bevy_render/src/view/mod.rs:717`), so a cached face does not depend on where the viewer stood. To verify in S2;
  fallback: one recapture when the viewer first comes within 20 m.
- A capture face is confirmed only once every pipeline its bins use is compiled (V1 lesson 7); until then it redraws.
- **Why no disk bake** (RDR2 does it): a lamp is 6 MiB at 512 px, so thousands of placed lamps are tens of GiB before
  compression; the bake would need a GPU and minutes; any mod that moves a static invalidates it. A capture costs what a
  first draw costs today and happens once per admission. If S4 shows captures as stutters, a content-hashed disk cache
  of captured cubes is the follow-up, with the same run-time path as fallback.
- **Memory.** A face is 1 MiB at 512 px, 4 MiB at 1024 px (Depth32Float). Live cubes exist today. The store is pooled
  per face, not per lamp: a face takes a store layer the first time a mover enters it (the live layer is clean at that
  moment, so it is copied to the store), and gives it back LRU. `store_faces_per_lamp` 3 (a walker dirties 1-3 faces).

  | Preset | Lamps x size | Live (today) | Store | Total |
  |---|---|---|---|---|
  | Medium | 6 x 512 | 36 MiB | 18 MiB | 54 MiB |
  | High | 8 x 512 | 48 MiB | 24 MiB | 72 MiB |
  | Ultra | 12 x 1024 | 288 MiB | 144 MiB | 432 MiB |

- **Eviction.** A slot is freed when its lamp loses the budget or despawns; store layers are LRU. A face that needs a
  store layer when none is free draws no movers that frame and counts in `lamp_cache/store_starved`.

### 2. Movers

- A face with at least one mover in its frustum and the lamp's range gets, each frame it is due: one copy store -> live,
  then a draw of the movers only, loading the depth instead of clearing it.
- A face no mover is in any more gets one last copy and is clean again. Clean faces are never copied or drawn.
- Copies are whole-layer `copy_texture_to_texture` calls (both textures get `COPY_SRC | COPY_DST`), all recorded in one
  encoder per frame with no bind groups, so none of the per-encoder material tracker cost of the submit diagnosis.
- Rate tiers replace the 2 + 1 lights-per-frame cap for cached lamps: movers within `mover_full_rate_m` (16, the SH-20
  rule) of a camera redraw every frame; out to `mover_half_rate_m` (30) every second frame, lamps alternating; a cap of
  `max_mover_faces_per_frame` (12), oldest first. Decisions stay per lamp, never per face (SH-20b: no cut at a cube seam).
- A lamp whose range sphere is in no camera frustum draws no movers (SH-21b), and a mover face outside the camera
  frustum is not drawn (CW-3b's test, §4).
- This is the SH-21a fix: a mover draw is a handful of items, so every near lamp can update every frame.

### 3. A clean face creates no view

Bevy creates six views for a light when its extracted `shadow_maps_enabled` is true, and the stock main-world scan runs
on the same flag. The design separates "this lamp samples a shadow cube" from "these faces are drawn this frame".

- **One new main-world component in vendored `bevy_pbr`: `CachedPointShadowMap { draw_faces, load_faces, restore }`**
  (masks over faces, plus copy orders). A lamp carrying it has `PointLight::shadow_maps_enabled = false`.
  - Main world: stock `check_point_light_mesh_visibility` skips it at `lib.rs:584`. No patch to `bevy_light`, which
    stays unvendored. The cache fills `CubemapVisibleEntities` itself, only for faces in `draw_faces`, after
    `SimulationLightSystems::CheckLightVisibility` (as `list_gpu_culled_casters` does today), and marks listed movers
    view-visible so off-screen walkers are extracted.
  - `extract_lights`: the lamp is extracted as shadowed for shading; lists are copied only for `draw_faces`.
    `extract_shadow_caster_changes` skips it (no hashing).
  - `prepare_lights`: slot, texture, GPU light record and `ViewShadowBindings` as for any shadowed lamp. View entities,
    `ShadowView`, the retained shadow phase and `live_shadow_mapping_lights` entries exist only for faces in `draw_faces`
    (`create_point_shadow_maps` `light.rs:2883`, the loop at `:2307`). A face in `load_faces` gets a depth attachment
    that loads. A view is despawned `mover_linger_s` (0.5) after its face left the mask; in between it carries
    `SkipRootViewRendering`.
  - `skip_unchanged_shadow_views` leaves cached lamps alone: the mask is the decision.
  - One new render system records the copies before `camera_driver`.
- A clean face therefore has: no main-world scan, no list, no hash, no view entity, no phase, no schedule run, no pass,
  no encoder. A lamp with six clean faces costs one iteration of the light loop in `prepare_lights`.
- Lamps without the component (moving lamps, carried torches, spot lights) take the stock path, byte for byte.
- Rule 4: every change is a marked block in the vendored `light.rs` (`superfargoth patch (own, CW-4)`), about 350-500
  lines, plus the texture usage flags. Nothing reaches into the render world from outside.
- **Policy lives in our crates.** Bevy-free state machine in `sf_sky::lamp_cache` (face states Capture, Clean, Movers,
  Linger, Dirty; capture queue; store pool; rate tiers), Bevy wiring in `sf_sky_bevy::lamp_cache` next to the budget:
  the caster tracker, per-lamp static lists (the H1 lists, extended from GPU-culled statics to every static), mover tests
  against face frusta, the component writes and the gauges.
- **Classification** (from the old crate, with its V1 lessons): compare light-relative affine, bounds and visibility at
  1 mm, never change ticks (lesson 2), so a floating-origin shift is invisible and `ShadowOriginShift` goes away; skinned,
  morphed and `ShadowMobility::Movable` casters always move; other rigid casters bake after `settle_time_s` (1) still;
  hosts mark casters that cast nothing `NotShadowCaster`; GPU-culled statics and GPU instance batches are statics at once.

### 4. With CW-3a, CW-3b, CW-13, CW-16

- **CW-3a (contrast gate)** decides who holds a slot; the cache acts on holders that are still. Nothing shared but the
  shadow fade. A lamp gated out at dawn frees its slot; dusk admission is a capture hidden by the fade.
- **CW-3b (per-face culling)** is being built in the same file for the stock path. Its three tests apply here as: no
  static casters -> no capture (§1); wall side -> the same; outside the camera frustum -> no mover draw (§2). Ask CW-3b to
  keep the face test a pure function both paths call. S2 starts after CW-3b lands, and CW-4's blocks stay separate.
- **CW-13 (one-pass atlas)** is kept open by four rules: the policy speaks of *tiles* (count and frusta come from a
  layout, 6 cube faces today, 2 or 4 later), never the literal 6; a request is "(tile, mover list)", so one view can draw
  all of them; the restore is "copy tile", a whole-layer copy today and a depth blit draw in an atlas, because WebGPU
  copies depth only by whole subresource (the old `copy_depth.wesl` is kept for that); and with explicit tile rects in
  the light record a clean tile can point at its static tile directly, which removes the live duplicate.
- **CW-16 (screen-space shadows past the budget)** covers lamps without a slot; the cache does not touch them. The
  static-only tier (S6) and CW-16 complement each other: static depth from the cube, movers from screen space.

### 5. Decision: extend the vendored path, port the old crate's policy, delete its render half

- **Not a revival of `sf_shadow_cache_bevy` as built.** Nothing was measured to break in B3: it was switched off the
  day B1 measured cache + patch best, with no A/B (perf-verdict audit §3 item 3). Its structural problems are real
  though: (a) its render half is what rule 4 forbids: it strips `RootNonCameraView` from Bevy's views after
  `prepare_lights` every frame, trusts `TextureCache` to hand back the same texture, derives layers from Bevy's sort
  order and injects capture subviews 6-11 (`render.rs:102-103`, `:240`, `:480`); (b) a detached face kept its view entity,
  lists and the stock main scan, and V1 missed its walk gate on exactly that; (c) it drew restores as one render pass
  per face and a capturing lamp twice; (d) the live path moved on (persistent slots, SH-20/20b/21b, H1), so it no
  longer matches (ledger 2026-11-12: "any retry must work on the live lamp path").
- **Not the vendored skip alone.** A skip rule cannot separate statics from movers; B1 measured that (11.4 vs 6.9 ms).
- **Extend the vendored path** with the mechanism of §3 and port the old crate's main-world half (`track_casters`,
  `classify_lights`, `ShadowMobility`, `ShadowCastersChanged`, the stats) into `sf_sky` / `sf_sky_bevy`. The old crate
  stays untouched until S4's A/B, then S5 deletes it, tagged, as replaced code.

### 6. Invalidation

| Event | Effect |
|---|---|
| Lamp despawns (cell unload) or loses its slot | slot and store layers freed; re-admission captures again |
| Static in range spawns, despawns, moves, hides or shows (streaming, a door, a picked-up item, script, console) | faces its bounds touch go Dirty and recapture, at most every `recapture_interval_s` (2, V1 lesson 6); a new static is drawn as a mover meanwhile, a removed one leaves its shadow until then |
| Door or physics object | rigid, automatic: baked after it has been still, a mover again when it moves |
| Lamp switched off or on, flicker, pulse, colour | nothing: depth only; a hidden lamp keeps its slot until evicted |
| Lamp range (the budget blends it every frame) | nothing: the cube projection has no far plane; static lists use the shadowed reach |
| Lamp moves more than 1 mm against its statics | component removed, stock path until it is still for `settle_time_s` |
| Time of day, weather, sun, floating-origin shift | nothing |
| Cube size, preset or texture change | every slot recaptured; the texture is sized for the budget up front |
| Caster animated in the vertex shader (wind) | frozen in the cached face unless the host marks it `Movable` (UE excludes such materials from its cache) |

### Tunables

All under `shadow.lamp.cache.*` (`tunables.md`), `Guess` unless a source is named: `enabled` (switch, default off),
`capture_faces_per_frame` 2 (V1), `recapture_interval_s` 2 (V1), `settle_time_s` 1 (V1), `store_faces_per_lamp` 3,
`mover_full_rate_m` 16 (SH-20), `mover_half_rate_m` 30, `max_mover_faces_per_frame` 12, `mover_linger_s` 0.5,
`static_tier_slots` 0 (S6). Game switch: `BEVYRROWIND_LAMP_CACHE=1`, off by default until S4.

## Slices

One deliverable and one test each. S2 and S3 need a strong model; S0b and S4 are the orchestrator's (bench sessions).

| # | Deliverable | Test |
|---|---|---|
| S0a | `POINT_SHADOW_CACHE` becomes an env switch (`BEVYRROWIND_LEGACY_LAMP_CACHE=1`), so the old cache can be an A/B arm; fix only what stops it from starting, at most half a day, else log that it no longer runs | plugin installed only with the switch |
| S0b | Ceiling: interleaved `base`, `--point-shadows 0`, `legacy` (if it runs) | ledger line with main, render, GPU, stutters |
| S1 | `sf_sky::lamp_cache`: Bevy-free face state machine, capture queue, store pool, rate tiers | exact assertions: still scene ends all Clean; mover in, out, linger; store starved; per-lamp atomic; cap order |
| S2 | Vendored mechanism: `CachedPointShadowMap`, views only for requested faces, load attachment, store texture and copy system, extraction skips; after CW-3b | render-world tests: mask 0 -> 0 face views and 0 phases, mask 0b101 -> 2, stock lamp -> 6; one headless run without wgpu validation errors |
| S3 | `sf_sky_bevy::lamp_cache`: tracker, static lists, list filling, component writes, gauges `lamp_cache/*`, game switch | headless app: still room -> 0 lamp views after capture; walker -> views only on its faces, 0 after linger; static spawned -> that face recaptured; lamp moved -> stock path |
| S4 | A/B below, visual-check entries, wiki and ledger lines | - |
| S5 | Delete `sf_shadow_cache_bevy` (tag first), move `ShadowMobility` users, docs | workspace builds, `--off shadowcache` removed or remapped |
| S6 | Optional, owner go: static-only tier (question 1) | lamp in the tier has 0 views ever |
| S7 | Optional: lazy capture of faces first seen by the camera; Depth16 store | - |

**S1 as built (2026-11-16).** `sf_sky::lamp_cache` (Bevy-free, `sf_tunables` added to `sf_sky`). `LampCache::new(TileLayout,
max_lamps, LampCacheConfig)`; per frame the host calls `admit(key, LampTier, statics_mask)`, `invalidate(key,
Invalidation)` (`Despawned`/`SlotLost`/`Moved` remove the lamp, `StaticsChanged{touched, statics}` dirties tiles,
`LightChanged` does nothing), `set_movers(key, MoverInput{tiles, distance_m})` after its SH-21b/CW-3b tests,
`confirm_capture(key, tile)` from render-world feedback (unconfirmed captures redraw every frame, head of a FIFO queue),
then `plan(now_s)` -> `FramePlan { lamps: [LampPlan{ready, capture, movers, views}], copies: [CopyOrder{op: Save|Restore|
Clear}], stats }`. S2's mapping: `draw_faces = capture | movers`, `load_faces = movers`, a view exists for `views` (drawn,
movers not due, lingering: the linger timer is here, not in `prepare_lights`), `restore` = that lamp's copies, recorded
in order before any shadow pass. Store: a free list then LRU of unpinned layers of other lamps; a layer is pinned while
movers are drawn over its face. Choices the spec left open: a lamp first in the cap order draws even when it alone
exceeds `max_mover_faces_per_frame`; past `mover_half_rate_m` a third tier draws every `mover_far_interval_frames` (4,
`Guess`, new); a tile being recaptured draws no movers until confirmed (copies precede passes, so the clean layer is saved
the next frame); a new pool size drops store copies only (tiles with movers over them recapture), `recapture_all` is the
cube-size path; lowering `max_lamps` evicts nobody. Owner questions stay open in data: `LampTier::StaticOnly`
(`static_tier_slots` 0) and `CasterMobility::AnimatedStatic` (frozen unless `animated_casters_move`); `classify` holds the
settle rule for S3's tracker. 12 tunables in `lamp_cache::TUNABLES`; 21 exact tests in `lamp_cache/tests.rs`.

## Measurements (S0b, S4)

- One exe, untraced, `perf suite --arm base --arm cache:--env BEVYRROWIND_LAMP_CACHE=1`, plus
  `gate` and `gate+cache` with `BEVYRROWIND_LAMP_DAY_GATE=1`; 3+ interleaved runs; scenes `balmora_walk_night`,
  `balmora_walk`, `census`. `balmora_night` says nothing (camera 60 m up, no lamp faces).
- Judge on main/busy, render/thread and GPU as well as the frame median and p99; stutters/min; encoders per frame and
  `queue.submit` (CW-5b gauges); `views/lamp_face` count and work items; `lamp_cache/*` (clean and mover faces, captures,
  copies, store starved); `bevy_pbr::shadow_refresh` debug counts for the stock arm.
- Visual checks, owner: SH-21 (walker among many lamps: no flicker), SH-25 (camera turn keeps shadows), SH-27 (budget),
  and a new entry for the cache: an interior door swung open and shut, an item picked up next to a lamp, a walker
  crossing a cube seam, an NPC entering a lamp's range, a lamp first seen from 40 m then walked up to.

## Expected ms

| Scene | Main | Render | Confidence |
|---|---|---|---|
| Night walk, 8 lamps | -0.4 to -0.7 | -0.2 to -0.5 | main medium (measured scan cost per lamp); render low (per-view upkeep was never timed alone, and mover faces stay views with an encoder each) |
| Day walk, CW-3a off | as night | as night | same |
| Day walk, CW-3a on | ~0 | ~0 | high: nothing left to cache |
| Census interior | -0.3 to -0.6 | -0.2 to -0.5 | low; B1's 9.0 -> 6.5 ms was against an older baseline, an upper bound |

Stutters: fewer and smaller, medium-high confidence (a redraw draws movers only). SH-21a flicker: fixed by construction
for cached lamps, to confirm by eye. Risk: with the cap lifted more faces draw per frame than today's 5-9, about 0.1 ms
render each; `max_mover_faces_per_frame` bounds it and CW-13 removes it. If S0b puts the ceiling under 0.5 ms on both
threads, the perf case is weak and the slice is justified by SH-21a alone.

## Unverified, to check in S2

- wgpu accepts whole-layer Depth32Float texture-to-texture copies between the two arrays (WebGPU allows it; not run here).
- A per-view LOD origin for capture views needs only the view-uniform path, not the GPU preprocess inputs.
- Spawning and despawning a mover face view does not re-specialise more than that face's movers.

## Open questions (owner)

1. **Static-only tier (S6).** Lamps past the budget could keep a captured cube and never draw: walls and furniture block
   their light, characters cast no shadow from them (RDR2's split). Costs one cube sample per lamp in the main pass
   (~0.1 ms GPU each at 1440p) and 6 MiB each. Do you want more lamps shadowed this way, and how many?
2. **Wind-animated casters near lamps** keep a frozen shadow in the cache. Acceptable, or mark trees and banners as
   movers (a permanent mover face per lamp near one)?

## Co-op

Presentation only, per client, nothing replicated; a remote player is one more mover.
