# Shadows

## What it does

Sun shadows are stock Bevy 0.20 cascades with our split layout, world-unit PCSS and contact shadows (vendored
`bevy_pbr`). Past the cascades the terrain shadows itself (far shadow map). Lamps get a few budgeted cube maps; a face is
redrawn whole when anything in it changed and skipped otherwise. There is no static caster cache in the game (below).
Everything else is unshadowed.

## How it works

- **Sun cascades.** `SunLight` is a `DirectionalLight` with `shadow_maps_enabled`, spawned in
  `superfargoth/crates/sf_sky_bevy/src/lib.rs:458`. Its `CascadeShadowConfig` comes from `CascadeSplits` (PSSM
  practical split, `cascades.rs:17`), built by the game per preset (`bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs:1298`,
  table at `:1693`: Low 3 cascades/160 m, Medium 3/190, High 3/205, Ultra 4/280; 2 cascades (F2) measured no win with the
  hold, opt-in `--sun-cascades 2:205`). Bevy still fits and texel-snaps each cascade.
- **Far-cascade hold (F3, 2026-11-15).** Cascades from index 1 keep last frame's padded fit and map until the camera
  moves 4 m, turns 3°, the sun moves 0.1° or 4 frames pass: `hold_far_cascades` after Bevy's cascade build
  (`superfargoth/crates/sf_sky_bevy/src/cascade_hold.rs:90`), render-side skip `skip_held_cascades` (`:162`), decision
  logic Bevy-free in `sf_sky/src/cascade_hold.rs`. Walk -1.0 ms on 3 cascades (ledger perf 2026-11-16). Opt-in: one encoder
  per cascade (F1, `BEVYRROWIND_CASCADE_ENCODERS=1`, `per_view_shadow_encoder_slot`, vendored `bevy_pbr/src/render/light.rs:3752`).
  The moon has its own 2 shorter cascades, shadows off until the sky says so (`lib.rs:473`, `write_light` `:654`).
  The sky turns sun shadows off in interiors and at night (`write_light`); sun and moon use `RenderLayers::none()` indoors.
- **Sun/moon gate (on by default (owner 2026-11-17); ROADMAP 1.25 CW-8).** The light the elevation rule lets cast keeps its
  cascades only while its shadow shows: contrast `c = E_direct / (E_direct + E_fill)` (`sf_sky/src/sun_gate.rs:176`),
  `E_direct` = ground illuminance on a receiver facing the light x the cloud shadow map's mean
  (`SunLightOcclusion`, filled by the game, `mw_environment_bevy/src/render.rs:333`, from `sf_atmos_bevy::mean_cloud_shadow`);
  `E_fill` = both lights' single-scattering sky fill (`SkySettings::sky_fill`, `sky.rs:282`; cloud cover dims it with
  the light, as Bevy's env map is lit by the light) + the other light on level ground + π x ambient floor. Weight 0 at
  c <= 0.05, 1 at >= 0.15 (`shadow.sun.gate.contrast`, `Guess`, the lamp gate's), holders x 1/0.8; the strength moves
  toward it at most 1/3 s (`shadow.sun.gate.fade_s`), a fresh caster starts at its weight; strength 0 = maps off (far
  shadows follow: they key on the flag). The strength goes to the vendored `DirectionalLightShadowFade` (own patch
  "CW-8", `vendor/README.md`): cascades, the material's far-shadow factor and contact shadows fade together. Glue
  `apply_lights` (`sf_sky_bevy/src/lib.rs:723`) + `gate_light` (`sun_gate.rs:64`); stats `SunShadowGateStats` (in the
  screenshot lighting log). By day every weather stays at full strength except the last ~30 min of low sun in fog,
  rain, snow, thunder, ash, blight and blizzard; at night a dim moon (< ~1.5 lux against the 4-nit floor) drops its 2
  cascades (ledger 2026-11-16). Switch: `BEVYRROWIND_SUN_SHADOW_GATE=1` (`render.rs:34`). Not faded in the band:
  froxel fog shadows (own CSM sampling).
- **PCSS and filter.** `DirectionalLight::soft_shadow_size` = 10 cascade texels on High/Ultra (`exterior.rs:1588`,
  `SOFT_SHADOW_SIZE` `:1305`); the penumbra is world-metre scaled by our shader patch (vendor row "H2",
  `superfargoth/vendor/README.md:20`). Filter: Hardware2x2 on Low, Temporal with a jittered camera, else Gaussian
  (`exterior.rs:1602`; `sf_sky_bevy/src/upscale.rs` parks Temporal to Gaussian without `TemporalJitter`).
- **Contact shadows.** Bevy `ContactShadows` on the camera (Medium 8 steps, High/Ultra 16, length 0.3 m; Low off,
  `exterior.rs:1532`) plus `contact_shadows_enabled` on the sun (`SunShadow`, `lib.rs:201`). Lamps get them only
  on High/Ultra, capped at `shadow.lamp.contact_max = 8` top-ranked lamps (`PointShadowBudget::contact_shadows`,
  `exterior.rs:1544`). Vendored `pbr_functions.wesl` (`SH-14` / Bend SSS upgrade, 2026-11-20) runs a single-fetch
  (`depth_sample_nearest` in DLSS `main_pass_viewport` pixel space, vendor row "RO-11") screen-space-clamped march
  (`min(length, max(0.03, view_dist * 0.05))`) with distance-aware thickness, normal start bias, sub-pixel & far
  distance early-outs (`28 m` local / `60 m` sun), and smooth ray-end / thickness / screen-edge feathering.
- **Far shadows.** `sf_terrain_bevy` computes a texel-snapped far shadow map from the height clipmap
  (`superfargoth/crates/sf_terrain_bevy/src/far_map.rs:1`); proxies, impostors and the fog's froxels read it
  (`exterior.rs:1240`, `:1254`). Terrain itself marches 18 camera-independent geometric heightfield steps (`FAR_FIRST_M = 4.0`,
  `far_shadow.wesl`, RO-20 swimming fix 2026-11-20) and blends smoothly into the far shadow map over `560–700 m`.
- **Extra depth views** (precipitation occlusion, surface-weather shelter) ride Bevy's shadow pass as subviews
  (`sf_sky_bevy/src/sun_view.rs:108`, `add_sun_view`); they invent no private state.
- **Lamp shadow budget.** Lamps carry `BudgetedShadow`; `budget_shadows` (`sf_sky_bevy/src/shadow_budget.rs:432`)
  ranks the shown lights of the viewer's `RenderLayers` all around the viewer, on screen or not, by `importance`
  with hysteresis (`:242`, `pick_shadowed` `:268`), and gives at most `PointShadowBudget::max` a cube map. Turning the camera
  changes nothing; a timed fade (0.3 s) happens only when a light gains or loses its slot, else the fade follows
  distance (`PointLightShadowFade`). Holders carry stock `NoFrustumCulling` (+ `ShadowHeldOffscreen`), so Bevy
  keeps their caster lists, face views and cached cube while they are off screen; the vendored refresh rule draws
  nothing for a holder whose range sphere is in no camera frustum (`vendor/README.md` "SH-21b"). Game numbers: High 8 maps at 512, Medium 6 at 512, Low 0,
  Ultra 12 at 1024 (`exterior.rs:1517`, `mw_environment_bevy/src/render.rs:33`); fade 35-45 m (`render.rs:37`).
  `drop_stale_light_views` (`:364`) frees face views of lights that lost a map (a Bevy 0.20 leak).
- **Contrast gate (on by default (owner 2026-11-17); ROADMAP 1.23b).** `ShadowContrastGate` (`shadow_budget.rs:141`, tunables
  `shadow.lamp.day_gate*` `:96`): outdoors (viewer has a `SkyCamera`; interiors unchanged) each candidate's importance
  and shadow strength are multiplied by a weight of its shadow contrast `c = E_lamp / (E_lamp + E_sky)`: `E_lamp` =
  steady lumens (`SteadyIntensity`, not the flicker) / 4πd² at a 2 m receiver (at most half the range) times Bevy's
  range window; `E_sky` = direct sun + moon on level ground + π x ambient floor (`sky_reference_lux` `:204`; the env
  map's sky diffuse is not counted, erring toward keeping shadows; the direct sun is counted even on sun-shadowed receivers, erring the other way under awnings). Weight 0 at c <= 0.05, 1 at c >= 0.15
  (`Guess`, Weber ~2 % raised for textured frames), holders' c counts 1/hysteresis times; weight 0 = no map. Noon: none;
  night: identical pick. Never touches a lamp's intensity (`ShadowReach` still blends range by shadow strength; the
  game's two reaches are equal). Switch: `BEVYRROWIND_LAMP_DAY_GATE=1` (`render.rs:24`).
- **Face culling (on by default (owner 2026-11-17), night walk -0.46 ms; ROADMAP 1.25 CW-3b).** Vendored own patch "point shadow face culling":
  `point_shadow_faces_seen` (`superfargoth/vendor/bevy_pbr/src/render/point_face_cull.rs:83`) keeps a cube face only if
  its 90° pyramid (out to the range, widened by `shadow.lamp.face_cull.margin_deg` = 2°, `sf_sky_bevy/src/shadow_budget.rs:124`,
  for filter taps across seams) meets an active `Camera3d` frustum clipped to the light's sphere (face-plane separating
  test, conservative). Mask per light in `extract_shadow_caster_changes` (`light.rs:1327`): culled faces are not hashed
  and the mask joins the light's entity hash (re-extraction on change). `skip_unchanged_shadow_views` (`light.rs:1582`)
  never draws a culled face, keeps its map and keys; a dirty face back in view draws its light that frame, outside the
  budgets (`:1693`). A culled face still exists as a view: stock `check_point_light_mesh_visibility` (bevy_light, not
  vendored) still lists its casters, `prepare_lights` still makes its view; no hash, uniforms, batching, preprocessing,
  schedule, encoder or draw. Faces without casters already draw once (a clear) and stay clean. Switch:
  `BEVYRROWIND_POINT_FACE_CULL=1` (`mw_environment_bevy/src/render.rs:29`). Gauges `views/lamp_face/culled|revealed|drawn`
  (`sf_diag_bevy/src/views.rs`), debug line `bevy_pbr::shadow_refresh`.
- **Lamp shadow cache (opt-in, off; ROADMAP 1.25 CW-4, confirmed on GPU SH-31) + Single-encoder batched pass (CW-13a, SH-32).**
  `shadow.lamp.cache.enabled` (live; alias `BEVYRROWIND_LAMP_CACHE=1`) + `shadow.lamp.batched_pass` (live; alias
  `BEVYRROWIND_LAMP_BATCHED_PASS=1`). `sf_sky_bevy/src/lamp_cache.rs` gives each still slot holder
  `bevy_pbr::CachedPointShadowMap` + `ShadowMapCached` and turns its stock flag off (no stock scan, no hash); the budget
  holds its fade at 0 until every face is captured. One caster scan at admission, then only changed casters (spatial
  pre-reject, light-relative 1 mm, origin shifts invisible); skinned, morphed and `LampShadowMobility::Movable` always
  move, rigid ones settle after 1 s. Clean faces keep no view; mover faces restore static depth and draw movers only.
  CW-13a (`record_batched_point_shadows`, vendored `bevy_pbr`) collapses all active point/spot shadow views into one
  command encoder instead of 5 encoders/face in `Core3dShadows` (reversing CW-4's render-thread regression: −0.70 ms on
  `balmora_walk_night`, −0.37 ms `faster` on `census`; SH-32 confirmed).
- **2D Point Shadow Atlas (on by default since 2026-11-20; ROADMAP 1.25 CW-13b, SH-33).** `shadow.lamp.atlas.enabled`
  (live; alias `BEVYRROWIND_LAMP_ATLAS=1|0`; Low/Extremely Low keep point shadows off via `budget.max == 0`). Replaces the
  fixed 8-lamp cube array (and Bevy's 42-cube `max_texture_array_layers / 6` cap) with two persistent 2D `Depth32Float`
  atlases managed by a buddy quadtree (`sf_sky::shadow_atlas::{BuddyAtlas, ShadowAtlasManager}`): Static Atlas (default
  4096², persistent cached static depth) + Live Atlas (default 2048², dynamic mover overlays and carried torches), with
  variable 512 / 256 / 128 px distance/luminance tiers (1.5× hysteresis, 2 s cooldown), empty-face omission, 2-texel guard
  band (`guard_band_fov_y` + `inv_guard_tan` + inner half-texel clamp), 4-tap IGN-rotated RGSS PCF (`shadow.lamp.atlas.soft_pcf_radius`,
  default `1.0` texel, live with 0 pipeline recompiles, `0` = crisp 1-tap 2×2 HW PCF; volumetric fog stays 1-tap), and
  handover spares for simultaneous crossfades. On tier resize, static casters stay baked while `retiring_static` keeps
  serving the shader (`static_rects`) and `Invalidation::RecaptureNow` captures immediately into `draw_static_rects`,
  releasing the retiring tile only on GPU confirmation (`AtlasPointShadowReport::confirmed`). Contact shadows stay capped
  at `shadow.lamp.contact_max = 8` top-ranked lamps so scaling to 48+ shadowed lamps never multiplies raymarch cost.
  Render side (`bevy_pbr/src/render/point_shadow_atlas.rs` + `point_shadow_atlas_blit.wesl`, `@group(0)` bindings `54..=56`,
  `MeshPipelineKey::POINT_SHADOW_ATLAS`): `record_point_shadow_atlas` runs in ONE command encoder with a single `RenderPass`
  over the Live Atlas for all `NoIndirectDrawing` mover-only faces (using `fs_blit` with exact `guard_ratio` remapping when
  Static/Live tile sizes differ, or 1:1 `textureLoad` when equal, before drawing movers; faces with `GpuBatchedMesh3d`
  movers stay on the indirect path). Phase wipes also invalidate `LightKeyCache` so `queue_shadows` re-queues all casters,
  and non-draw frames preserve their view's previous direct/indirect mode. Interleaved A/B `shadow_a3_atlas`
  (2560x1440 DLSS Quality): `atlas8` −0.84 ms walk / −0.53 ms census; `atlas48` (19–24 shadowed lamps simultaneously)
  −0.76 ms walk / −0.32 ms census (all 4 comparisons `faster`, U=0.0; p99 −1.9 to −3.5 ms).
- **Legacy lamp cache: not in the game** (`POINT_SHADOW_CACHE = false`, `mw_environment_bevy/src/render.rs:37`, since
  2026-10-21; `--off shadowcache` is a no-op). As built: `sf_shadow_cache_bevy` (`lib.rs:1`-`:70`, `ShadowCachePlugin` `:209`): per shadowed light a persistent
  cube array holds the depth of its static casters; Bevy's views draw movers over a restored copy. Captures:
  `captures_per_frame` 2, `recapture_interval` 2 s, `settle_time` 1 s, 24 slots (`:85`-`:125`). Skinned meshes and
  `ShadowMobility::Movable` always count as movers (`:401`-`:416`). GPU-culled statics (`NoCpuCulling`) are listed by
  the cache itself (`add_gpu_culled_casters`). Meshes that cast nothing must carry `NotShadowCaster`.
- **Mover shadows.** Vendored `ShadowMoverRefresh` redraws faces of close movers per frame within 16 m
  (`vendor/README.md:19`; resource inserted at `mw_environment_bevy/src/render.rs:179`).
- **Lamp filter.** Point and spot maps use 1-tap 2x2 PCF (vendor "lamp shadow filter", `vendor/README.md:13`); the
  shader skips the shadow fetch where a light contributes nothing (same row, sun too).

## Bevy features used

`CascadeShadowConfig`, `DirectionalLight` shadows, `PointLightShadowMap`, `PointLightShadowFade` (vendored),
`ContactShadows`, `ShadowFilteringMethod`, PCSS (`experimental_pbr_pcss`, feature `pcss` in `sf_sky_bevy/Cargo.toml:11`), `DirectionalLightShadowFade` (vendored, CW-8).
Vendored: #26149/#26161 slim per-face schedule (`vendor/README.md:11`), plus the own patches named above.

## Switches and counters

Live tunables (`set <name> <value>` in the console, BRP `sf.tunables/set`, `--set name=value`; bound by
`sf_sky_bevy/src/quality.rs` `ShadowQualityPlugin`, quality levels at the `Preset` layer from
`mw_exterior_bevy/src/quality_presets.rs`): lamps `shadow.lamp.count`, `.map_size_px`, `.fade_m` (Bevyrrowind 35,45 at
`Mod`), `.fade_time_s`, `.hysteresis`, `.contact`, `.contact_max` (8), `.day_gate` (+ `.receiver_m`, `.contrast`),
`.face_cull` (+ `.margin_deg`), `.batched_pass`, `.cache.enabled` (+ 11 `shadow.lamp.cache.*` policy values),
`.atlas.enabled` (+ 10 `shadow.lamp.atlas.*` sizes/tiers/guard/max_lamps/handover values); sun `shadow.sun.cascades`,
`.distance_m`, `.split_lambda`, `.overlap`, `.soft`, `.soft_size_texels`,
`.gate` (+ `.contrast`, `.hysteresis`, `.fade_s`); views `shadow.contact` (+ `.steps`, `.thickness_m`, `.length_m`),
`shadow.filter` (auto|hardware|gaussian|temporal), `render.ssao.quality`. Deprecated env aliases: `BEVYRROWIND_POINT_SHADOWS`,
`_LAMP_CONTACT`, `_SHADOW_FILTER`, `_SSAO`, `_LAMP_DAY_GATE`, `_POINT_FACE_CULL`, `_LAMP_CACHE`, `_LAMP_BATCHED_PASS`,
`_LAMP_ATLAS`, `_SUN_SHADOW_GATE`; `--point-shadows N`,
`--sun-cascades N:M` (and `BEVYRROWIND_SUN_CASCADES`) set the `CommandLine` layer. `--off` parts (`mw_game_core/src/perf.rs:135`):
`shadows`, `shadowcache`, `lampfilter`, `movershadows`, `farshadows`, `softshadows`, `contactshadows`, `ssao`, `cpuculltag`,
`cascadehold`, `cascadeencoders` (`softshadows`, `contactshadows`, `ssao` force their tunables at `CommandLine`).
`BEVYRROWIND_GPU_CULL_STATICS=1` stays env (startup). Console `flip contactshadows` (devtools, older duplicate).
Gauges: `ShadowCacheStats` (`sf_shadow_cache_bevy/src/lib.rs:147`), `RUST_LOG=bevy_pbr::shadow_refresh=debug`; lamp
cache `lamp_cache/*` + `lamp_cache/render/*` (`PointShadowCacheStats`); 2D shadow atlas `shadow_atlas/lamps|moving_lamps|
views|captures|mover_draws|movers|static_tiles|live_tiles|empty_faces|static_occ_pct|live_occ_pct|alloc_failures|resizes`
(main world, `sf_sky_bevy/src/shadow_atlas.rs`) and `shadow_atlas/render/lamps|views|static_draws|live_indirect|live_direct|
blits|clears|reallocated` (render world, `PointShadowAtlasStats`).

## Known limits

- Lamp-shadow quality is open: player shadow flickers near some torches, which lamps cast looks arbitrary
  ([ledger](../../ledger/shadows.md), SH-13, SH-21); hard problem H2.
- Shadows on a moving character flicker among many lamps: only 2 (+1) lamps redraw per frame, so each lamp's cube
  holds an older pose on a different frame. Standard fix (cached static depth + movers drawn every frame, UE, HDRP
  mixed cached shadows) built opt-in as the CW-4 lamp cache, not yet checked on a GPU; ledger 2026-11-15, SH-25.
- The lamp pick uses the camera position: a third-person orbit can still hand a slot over when more lamps than slots
  are near; with all slots held, a far lamp ahead may have no shadow (SH-25).
- Objects past the last cascade get no object shadows; terrain does (ledger 2026-10-19).
- A moving lamp is drawn whole by Bevy until it stops; torch-bearers cost about six faces per frame.
- Medium and Low have no or few lamp maps because the main pass samples every shadowed lamp per pixel.

## Specs

[shadows](../../../superfargoth/docs/specs/shadows.md), [distant-shadows](../../../superfargoth/docs/specs/weather/distant-shadows.md),
[lights](../../../superfargoth/docs/specs/lights.md), [lamp-shadow-cache](../../../superfargoth/docs/specs/lamp-shadow-cache.md) (static lamp cache design, CW-4, not built); research `superfargoth/docs/research/shadows/`.
