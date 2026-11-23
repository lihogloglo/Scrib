# Feature switches inventory (2026-11-23)

Slice R1 (`feature-toggles`): what switches off today, at startup and live, and whether "off"
skips the cost. HEAD `6f724424`. Paths are from the repo root; `perf.rs` = `bevyrrowind/crates/
mw_game_core/src/perf.rs` (enum `Part`, table `Part::ALL`), `ext` = `bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs`,
`main` = `bevyrrowind/apps/bevyrrowind/src/main.rs`. Live = dev console (backquote) in any interactive `view`/`scene`/game
run, release too; `set <tunable> <v>` and `<tunable> <v>` both work. "skips" = the pass or work stops; "hides" = shader
flag or zeroed term, the work still runs.

## 1. Inventory

| feature | owner (path:line) | startup | live today | off skips cost? | sg | notes |
|---|---|---|---|---|---|---|
| Sun cascades | `sf_sky_bevy/src/quality.rs:174` ShadowQualityPlugin | `--off shadows` (`mw_devtools_bevy/src/perf.rs:373`), `--sun-cascades` | F2 / `off sun_shadows`; `shadow.sun.cascades N`, `shadow.sun.distance_m` | skips (`shadow_maps_enabled=false`) | shadows | `--off shadows` is the sun only, lamps keep theirs. Hold: `--off cascadehold` (startup) |
| Sun shadow gate | `sf_sky/src/sun_gate.rs:27` | env `_SUN_SHADOW_GATE` | `shadow.sun.gate` | skips cascades at night | shadows | |
| PCSS soft sun | quality.rs:80 | `--off softshadows` | `shadow.sun.soft off` | skips (filter change) | shadows | High+ on |
| Contact shadows | quality.rs:96 | `--off contactshadows` | `off contactshadows` / `shadow.contact off` | skips (recompile hitch) | shadows | lamp one: `shadow.lamp.contact` |
| Lamp shadows | `sf_sky_bevy/src/shadow_budget.rs:451` | `--point-shadows N` | `shadow.lamp.count 0`, `.map_size_px` | skips | shadows | no `--off` part names lamp shadows |
| Lamp atlas / cache | `sf_sky/src/shadow_atlas/tunables.rs:14`, `lamp_cache/tunables.rs:9` | `--off shadowcache`, `movershadows`, `lampfilter` | `shadow.lamp.atlas.enabled`, `.cache.enabled` (off), `.day_gate`, `.face_cull`, `.batched_pass` | atlas off = stock cubes (+0.59 ms, ledger) | shadows | live atlas now 4096 px (347cf7a6) |
| Terrain far shadows | `sf_terrain_bevy/src/far_map.rs` | `--off farshadows` (systems not added, ext:653,675) | Shift+F3 / `off terrain_far_shadows` | live = hides (shader bit, `sf_terrain_bevy/src/lib.rs:1736`); startup skips | none | far map also feeds froxels and sun_vis |
| Lamp lights | `main:661`, `mw_viewers_bevy/src/view.rs:204` | `--off lights` | none | startup skips | none | |
| Probe GI | `sf_gi_bevy/src/lib.rs:120`, `mw_exterior_bevy/src/gi_volume.rs` | `--off gi`, `render.gi.probes` (restart) | `flip gi` / `render.gi off` | skips tracing | lighting | interiors only; can't turn on live if absent |
| SSAO (GTAO) | quality.rs:128 | `--off ssao` | `off ssao` / `render.ssao.quality off` | skips, normal prepass follows (recompile) | lighting | |
| Bloom | `mw_environment_bevy/src/quality_tunables.rs:24` | `--off bloom` | `render.bloom off` | skips | post | |
| Froxels / god rays | `sf_atmos_bevy/src/lib.rs:133` CloudsPlugin | `--off froxels` (`godrays`), `fogjitter/dither/history/clamp` | none alone; `flip fog` kills all fog | startup skips | none | froxel inject now samples far map + cloud shadows per froxel (FG-12 A1) |
| Fog (height, banks, far) | same | `--off fog`, `fogbank`, `cloudfog`, `stormshape`, `transfog` | `flip fog`, `flip fog_night_light` | `flip fog` skips (`CloudFrame.active`, lib.rs:533) | none | sub-parts startup only |
| Atmosphere LUTs, aerial persp. | `sf_sky_bevy/src/lib.rs:328` SkyPlugin + vendored `bevy_pbr/atmosphere` | `--off sky` (`atmosphere`), `envmap`, `envslice` | none | startup skips | none | |
| Aerial shadow (AS) | `sf_atmos_bevy/src/aerial_shadow.rs:26` | `--off aerialshadow` | `sky.aerial.shadow` (default off) | skips | none (Group::Sky, not in sg.sky) | |
| Light shafts (LSO) | `shafts.rs:70` | `--off shafts` | `sky.shafts.enabled` (default off since 9f4e7e87), `.bloom` | skips (key LUT only when on, vendored resources.rs:780) | none | |
| Sun visibility march | `sun_visibility.rs:66` | `--off sunvis` | `sky.sun_vis.enabled off`, `.samples` | skips (camera marker, :425) | none | NEW, on, half-res 16 samples 5 km, cost not measured |
| Clouds + cloud shadows | CloudsPlugin | `--off clouds`, `cloudedge`, `cbshape` | Shift+C / `flip clouds`, `flip cloud_history`; `sky.clouds.map_per_sample` | skips trace and shadow texture (lib.rs:537) | sky | no switch for cloud shadows alone |
| Rain / weather particles | `sf_precip_bevy/src/lib.rs:213` | `--off precipocc` | `C` key / BRP `set` weather only | n/a | none | no off switch for precipitation |
| Surface weather | `sf_surface_bevy/src/lib.rs:304` | `--off surface, wet, puddles, shelter, cover, trails, kickup, raise` | `flip surface, cover, trails, raise`; `surface.puddles`, `surface.shelter` | hides: hook compiled in, flags cleared (perf.rs Surface doc); shelter/puddles skip | effects | wet, kickup startup only |
| Water FFT ocean | `sf_water_bevy/src/lib.rs:359` | `--water on/scaffold/off` | `water.quality low..ultra` (no off) | no live off | water | runs every exterior frame unless `water.gate.visible on` (opt-in, unmeasured) |
| Water SSPR, shore, surf | same | `--off waterssr, shorenormals, shorewaves, seawind, shoredepth, surffoam, surfskew, seastate, waterrain` | none | startup | none | |
| Underwater post | `mw_environment_bevy/src/water.rs:182` | `--off underwater` | `water.underwater` | skips | water | |
| Groundcover | `sf_groundcover_bevy/src/lib.rs:141`, `mw_exterior_bevy/src/groundcover.rs:83` | `--off groundcover` (plugin not added) | **none** | startup skips | none | NEW. `groundcover.*` tunables are `Respawn` (new tiles only); casts shadows to 30 m; absent from tunables index and CATALOG |
| Rockgen | `sf_rockgen_bevy` | n/a | n/a | n/a | none | not in game: `level:rock-lab` only (`level/rock_lab.rs:210`) |
| Terrain clipmaps / CDLOD | `sf_terrain_bevy/src/lib.rs:668` | `--off terrain`, `gputerrain`, `selectgate` | `select.gate`; `terrain.cliff.enabled`, `terrain.hex_tiling`, `terrain.shade.decoupled`; `flip terrain_specular, terrain_env_specular, terrain_indirect, terrain_tint` | cliff/hex/decoupled: shader branches (likely skip taps, unverified); terrain_* = hides (RO-16 scaffolds) | none | shading changed 21078c18 (finest-level normals) |
| Far field / LOD | `sf_lod_bevy/src/lib.rs:209` | `--off farfield` (`lod`), `far`, `near`, `sizecull` | `lod.bias`, `lod.size_cull.min_px`; `far.handoff_scale` restart | startup skips; live scales | view_distance | |
| Static scene | `sf_scene_bevy/src/lib.rs:159` | `--off scene, precombine, matclass, flatnormal, cpuculltag, meshslabs, texvariety, skymotion, shiftrebase` | none | startup | none | |
| NIF particles | `mw_devtools_bevy/src/perf.rs:382` | `--off particles` | none | skips (emitter removed) | none | |
| NIF controllers | `mw_assets/src/animation.rs:50` | `--off nif` (uncooked spawner) | `mw.matctrl.gate` | gate skips unseen materials | effects | no off for controllers themselves |
| Foliage | `mw_assets` | `--off leafmask`, `foliagetransmission` | none | startup | none | |
| Actors, NPCs, anim LOD | `sf_character_bevy/src/lib.rs:487` | `--off actors` (`npcs`), `farnpcs`, `walkers` (`ai`) | `render.skin.skip_unchanged`, `character.finish.per_frame`; walk ladder `off stride_warp` etc. | startup skips | characters (not in presets.ron) | `farnpcs` hides at root, anim still runs |
| Physics | `ext:264` | `--off physics` | `tcl` (player collision only) | startup skips | none | |
| Exposure / tonemap | `mw_assets/src/lights.rs:85` | none | `exposure.min_ev100` | n/a (CPU, no pass) | post | no AutoExposure; no motion blur or DoF exist |
| TAA / DLSS | quality.rs:148 | `--dlss`, `--off denoiser` (RT) | F4 / `render.upscaling off/taa/quality...` | skips (recompile) | post | |
| Normal / ORM / POM maps | `mw_assets` MaterialMaps | `--off normalmaps, orm, parallax, delight` | `flip normalmaps, orm, parallax`; `texture.parallax.layers 0` | skips (materials rebuilt, hitch) | textures | delight = restart |
| Envmap / sky probe | SkyPlugin | `--off envmap`, `envslice` | none | startup | none | no reflection probes beyond the sky map |
| Occlusion culling | `ext:1030` (Bevy `OcclusionCulling`) | none | none | | none | always on for the game camera |
| Night sky, moon, ambient | `sf_sky` | `--off sky` | `flip night_sky_glow, moonlight, weather_ambient, ambient_specular` | hides | none | |

## 2. Added or changed since 2026-11-21

| commit | change | live off switch? |
|---|---|---|
| 347cf7a6 | lamp atlas live 2048 -> 4096 px; near movers redraw every frame | yes: `shadow.lamp.atlas.live_size_px`, `.atlas.enabled`, `shadow.lamp.count` |
| 6d314810 | carried lights redraw every frame; atlas admission by importance | indirect only (`shadow.lamp.count 0`); no CarriedLight switch |
| 25a77017 | far shadow map keeps its centre; gauges | **startup only** (`--off farshadows`); live switch only hides |
| b49325de | cutout classes from bake; model textures RENDER_WORLD; opt-in budgets | budgets yes (off); texture/cutout path has none (code path) |
| 21078c18 | terrain shading from finest level (WG-LOD), cliff fix; worldgen, rockgen lab | yes: `terrain.shade.decoupled`, `terrain.cliff.enabled`; rockgen not in game |
| b525bcb9, de475af7, 6f724424 | **groundcover** GC-1/2b, then denser grass and stronger bend; terrain ground hooks in `sf_terrain_bevy` (+221 lines) | **none live**; `--off groundcover` startup |
| 50e12cf0 | FG-12 aerial shadow (off), light shafts, shaft bloom (off); froxels read far map | yes for AS/LSO; **froxel far-map sampling none** |
| 9f4e7e87 | FG-12 sun visibility march (on, half-res, 16 samples, unmeasured); froxels import shared sun visibility (cascades x far map x cloud shadows) | `sky.sun_vis.enabled` yes; **froxel change none** (only `flip fog`) |

Prime suspects (no live switch, or the switch hides only): groundcover (new geometry, shadows, scatter jobs, per-frame
terrain-ground refresh); the froxel inject's new far-map + cloud-shadow taps; sun visibility (live switch exists, but on
and unmeasured); terrain far map (live switch hides); carried-light and near-mover lamp redraws.

## 3. Mechanisms and gaps

- **Startup `Part`** (`perf.rs:11`, 90 names; `--off a,b` or `BEVYRROWIND_OFF`): decided once, usually by not adding a
  plugin or system, so it really skips. The console answers "startup only, relaunch with --off X" for these
  (`mw_devtools_bevy/src/console.rs:629`).
- **Live `Switch`** (`mw_devtools_bevy/src/live_ab.rs:174`, 26 names; F-keys, `on/off/flip <name>`, BRP `bevyrrowind/set`):
  writes a settings resource read every frame. Several only clear shader bits (terrain_*, terrain_far_shadows, surface).
- **Console specials** (`console.rs:592`): `ssao`, `contactshadows` (write tunables at `SetBy::Console`), `gi`, `ambient`,
  `viewmodel`, walk ladder.
- **Tunables** (`sf_tunables`, `docs/tunables-index.md`): layers default < `Preset` < `Mod` < `CommandLine` (`--set`,
  `--off`, env aliases) < `Console`; `reset <name>` drops a layer. Lifetimes `live`, `recompile` (pipeline re-specialise
  hitch), `respawn` (new spawns only), `restart`, `bake`.
- **Preset groups**: `sf.graphics_preset` drives `sg.view_distance, shadows, lighting, post, textures, effects, water, sky`
  (`graphics_presets.ron`), live from Esc > Options > Graphics or `preset shadows low`. Only
  tunables are in groups: fog, froxels, sun_vis, shafts, groundcover, far shadows, particles have no preset effect.
- Gap: the tunables index is stale (no `sf_atmos_bevy` or `sf_groundcover*` rows; 21078c18 says not regenerated);
  `sf_groundcover*` TUNABLES are not in the perf `CATALOG` (`mw_devtools_bevy/src/perf/tunables.rs`).
- Gap: no live off for groundcover, froxels alone, lamp lights, precipitation, water (only quality tiers), particles,
  atmosphere/sky, far field, actors, physics.
- Gap: `--off shadows` names all shadows but only stops the sun's.
- Live controls exist in every interactive run (release too); BRP needs the `remote` (dev) feature; benches and
  `--screenshot` runs have neither, so benches can only A/B startup parts and `--set`.

Not covered: per-pass GPU cost; whether `terrain.cliff/hex/decoupled` off skips
taps; groundcover in the lamp atlas; RT path past `--off denoiser`.
