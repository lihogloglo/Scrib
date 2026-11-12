# Shadows

## What it does

Sun shadows are stock Bevy 0.20 cascades with our split layout, world-unit PCSS and contact shadows (vendored
`bevy_pbr`). Past the cascades the terrain shadows itself (far shadow map). Lamps get a few budgeted cube maps whose
static casters are cached; moving casters are redrawn over the cache. Everything else is unshadowed.

## How it works

- **Sun cascades.** `SunLight` is a `DirectionalLight` with `shadow_maps_enabled`, spawned in
  `superfargoth/crates/sf_sky_bevy/src/lib.rs:458`. Its `CascadeShadowConfig` comes from `CascadeSplits` (PSSM
  practical split, `cascades.rs:17`), built by the game per preset (`bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs:1298`,
  table at `:1574`: Low 3 cascades/160 m, Medium 3/190, High 3/205, Ultra 4/280). Bevy still fits and texel-snaps each cascade.
  The moon has its own 2 shorter cascades, shadows off until the sky says so (`lib.rs:473`, `write_light` `:654`).
  The sky turns sun shadows off in interiors and at night (`write_light`); sun and moon use `RenderLayers::none()` indoors.
- **PCSS and filter.** `DirectionalLight::soft_shadow_size` = 10 cascade texels on High/Ultra (`exterior.rs:1588`,
  `SOFT_SHADOW_SIZE` `:1305`); the penumbra is world-metre scaled by our shader patch (vendor row "H2",
  `superfargoth/vendor/README.md:20`). Filter: Hardware2x2 on Low, Temporal with a jittered camera, else Gaussian
  (`exterior.rs:1602`; `sf_sky_bevy/src/upscale.rs` parks Temporal to Gaussian without `TemporalJitter`).
- **Contact shadows.** Bevy `ContactShadows` on the camera (Medium 8 steps, High/Ultra 16, length 0.3 m; Low off,
  `exterior.rs:1532`) plus `contact_shadows_enabled` on the sun (`SunShadow`, `lib.rs:201`). Lamps get them only
  on High/Ultra (`PointShadowBudget::contact_shadows`, `exterior.rs:1544`). The ray march runs in DLSS pixel space (vendor row "RO-11", `vendor/README.md:21`).
- **Far shadows.** `sf_terrain_bevy` computes a far shadow map from the height clipmap
  (`superfargoth/crates/sf_terrain_bevy/src/far_map.rs:1`); proxies, impostors and the fog's froxels read it
  (`exterior.rs:1240`, `:1254`).
- **Extra depth views** (precipitation occlusion, surface-weather shelter) ride Bevy's shadow pass as subviews
  (`sf_sky_bevy/src/sun_view.rs:108`, `add_sun_view`); they invent no private state.
- **Lamp shadow budget.** Lamps carry `BudgetedShadow`; `budget_shadows` (`sf_sky_bevy/src/shadow_budget.rs:241`)
  ranks view-visible lights by `importance` (`:107`), gives at most `PointShadowBudget::max` a cube map and fades
  them by distance and time through `PointLightShadowFade`. Game numbers: High 8 maps at 512, Medium 6 at 512, Low 0,
  Ultra 12 at 1024 (`exterior.rs:1517`, `mw_environment_bevy/src/render.rs:28`); fade 35-45 m (`render.rs:32`).
  `drop_stale_light_views` (`:201`) frees face views of lights that lost a map (a Bevy 0.20 leak).
- **Lamp cache.** `sf_shadow_cache_bevy` (`lib.rs:1`-`:70`, `ShadowCachePlugin` `:209`): per shadowed light a persistent
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
`ContactShadows`, `ShadowFilteringMethod`, PCSS (`experimental_pbr_pcss`, feature `pcss` in `sf_sky_bevy/Cargo.toml:11`).
Vendored: #26149/#26161 slim per-face schedule (`vendor/README.md:11`), plus the own patches named above.

## Switches and counters

`--off` parts (`mw_game_core/src/perf.rs:135`): `shadows`, `shadowcache`, `lampfilter`, `movershadows`, `farshadows`,
`softshadows`, `contactshadows`, `ssao`, `cpuculltag`. `--point-shadows N`, `--sun-cascades N:M`,
`BEVYRROWIND_LAMP_CONTACT`, `BEVYRROWIND_SHADOW_FILTER`, `BEVYRROWIND_GPU_CULL_STATICS=1`. Console `flip contactshadows`.
Gauges: `ShadowCacheStats` (`sf_shadow_cache_bevy/src/lib.rs:147`), `RUST_LOG=bevy_pbr::shadow_refresh=debug`.

## Known limits

- Lamp-shadow quality is open: player shadow flickers near some torches, which lamps cast looks arbitrary
  ([ledger](../../ledger/shadows.md), SH-13, SH-21); hard problem H2.
- Objects past the last cascade get no object shadows; terrain does (ledger 2026-10-19).
- A moving lamp is drawn whole by Bevy until it stops; torch-bearers cost about six faces per frame.
- Medium and Low have no or few lamp maps because the main pass samples every shadowed lamp per pixel.

## Specs

[shadows](../../../superfargoth/docs/specs/shadows.md), [distant-shadows](../../../superfargoth/docs/specs/weather/distant-shadows.md),
[lights](../../../superfargoth/docs/specs/lights.md); research `superfargoth/docs/research/shadows/`.
