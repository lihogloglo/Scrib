# Reinvented wheels: audit against Bevy 0.20 and crates (2026-11-15)

Read-only audit. Bevy paths are the 0.20.0 sources in `~/.cargo/registry/src/*/bevy_<crate>-0.20.0/` unless they say
`vendor/`. Line counts are `wc -l` of `src/` (Rust + WGSL, tests included). Upstream PR states checked with `gh` on
2026-11-15; crate versions from the crates.io API the same day.

## Verdict

The static scene is the only large, clear duplicate. Everything else that is big and ours (animation runtime, UI
screens, saves, bake, physics wrapper, terrain, water, clouds, fog, texture residency) fills a gap Bevy 0.20 really
has, and its spec's reuse check still holds when re-read against the 0.20 source. The second real finding is
**two lamp shadow caches**: `sf_shadow_cache_bevy` (1.7k lines) is not installed in the game, while the vendored
`bevy_pbr` fork (#26149/#26161 + SH-20) does the job. Together, deleting both dead paths removes ~16k lines plus
~3.5k lines of vendored Bevy diff, and makes the vendor tree much smaller to re-port on the next Bevy. Five vendored
upstream PRs are now merged on Bevy main (not in 0.20.1), so they drop on 0.21. Three crate reasons went stale
since the specs were written (`bevy_hanabi` and `avian3d` now ship for 0.20; `bevy_seedling` still does not), but
none of them flips a decision on its own.

## Ranked findings

| # | Our system | What already exists | Overlap | Why ours; does it still hold on 0.20 | Lines | Recommendation | Risk/size |
|---|---|---|---|---|---|---|---|
| 1 | Static scene: `sf_scene` (`superfargoth/crates/sf_scene`), `sf_scene_bevy` (`lib.rs:64`, `residency.rs:280`, `shift.rs`), vendored `vendor/bevy_pbr/src/gpu_instance_batch/` (#25978 port + own "GS" tags), `--scene` wiring `mw_exterior_bevy/src/exterior.rs:299-304`, `sf_world_bevy/src/scene.rs` | Retained binned phases `bevy_render/src/render_phase/mod.rs:109` (`BinnedRenderPhase`), multidraw batch sets `:872`, `BinnedRenderPhaseType::MultidrawableMesh` `:982`; GPU frustum/occlusion culling `bevy_pbr/src/render/gpu_preprocess.rs:91,350,364`; `NoCpuCulling` `bevy_camera/src/visibility/mod.rs:69`; `VisibilityRange` `bevy_camera/src/visibility/range.rs:80` | Full | Decima/SPUD-style table to cut CPU per instance. Does not hold: measured parity only (`--scene` walk 11.07 vs 10.71 old path, `docs/ledger/statics.md:10-11`), frozen by the owner 2026-11-12 (ledger `:7`, ROADMAP 1.16). Upstream #25978 still OPEN, merge conflicts | ~9.1k + ~4.0k + ~1.8k vendored module (+~1.6k other #25978 hunks, diff 3,415 lines) | **Delete** (crates, vendored #25978 + GS, `--scene`, `scene_*` StreamStats, `GpuBatchedLightVisibility`) | M: touches `stream.rs` branching, vendor re-merge of `gpu_preprocess.rs`/`mesh.rs`; no game path depends on it |
| 2 | `sf_shadow_cache_bevy` (`lib.rs:209` `ShadowCachePlugin`, `render.rs:480-557` capture views) + `sf_scene_bevy/src/shadows.rs` + vendored `CpuListedPointShadowCasters` half of the "CPU-listed shadow views" patch (`vendor/bevy_pbr/src/render/mesh.rs`, ~70 lines) | Our own vendored lamp cache that the game uses instead: `skip_unchanged_shadow_views` `vendor/bevy_pbr/src/render/light.rs:1435` (#26161 fork) + SH-20b light-relative per-face keys (`light.rs:1205-1370`) + `ShadowMoverRefresh` | Full (parallel path inside our code) | Built before the vendored per-face cache. Off in the game since 2026-10-21: `POINT_SHADOW_CACHE = false`, `bevyrrowind/crates/mw_environment_bevy/src/render.rs:37`, guard `:185`. The architecture review found its transient capture views are what blocked `NoCpuCulling` | ~1.7k + 150 + ~70 vendored | **Delete** (keep the `sun_view` half of the CPU-listed patch, see #6) | S |
| 3 | Three near-static paths in `sf_world_bevy/src/stream.rs:807-908` (per placement, precombine, scene) | Bevy has one: entity per mesh with retained bins (wiki `bevy/render-and-visibility.md`) | Parallel paths inside our code | Per placement is the required fallback (`valid_members` `stream.rs:1773`); precombine fills a real gap (Bevy has no HLOD/merge); the scene path is #1 | branching in a 1.8k-line file | **Keep but thin**: goes to two paths once #1 is deleted | S, follows #1 |
| 4 | Vendored `#26149/#26161` own fork (`Core3dShadows`, `SkipRootViewRendering`) + SH-20/SH-20b on top | Nothing in 0.20 or main. Issue bevy#25576 still OPEN; both PRs CLOSED unmerged 2026-10-30 | n/a (reverse check) | Stock 0.20 redraws 6 faces per lamp per frame through a full `Core3d` run | diff 472 lines (309 added) + SH-20 hunks | **Keep** (it is the game's lamp cache); it is the biggest re-port cost on Bevy 0.21 | L at each Bevy bump |
| 5 | Particle sim for NIF emitters, `bevyrrowind/crates/mw_assets/src/particles.rs` (CPU sim + vertex pulling), `mw_nif/src/particles.rs`, `mw_actors_bevy/src/actor_particles.rs` | `bevy_hanabi` 0.20.0 (2026-11-01, Bevy ^0.20, MIT/Apache) | Partial | Module docs give three reasons (`particles.rs:5-8`): exact NIF semantics in a tested Bevy-free core, tens of particles per emitter, and "hanabi isn't on 0.20". The third no longer holds; the first two do (CPU sim of small emitters is what UE Niagara's CPU path does) | ~1.8k | **Keep**; fix the stale sentence. Revisit only when magic VFX need GPU sim (ARCHITECTURE.md:158 already plans hanabi for that) | n/a |
| 6 | `sf_sky_bevy/src/sun_view.rs:108` `add_sun_view` (precipitation occlusion, surface-weather shelter depth views as shadow subviews) + vendored CPU-listed shadow views patch | No public API for extra caster depth views in 0.20; light views are created in `bevy_pbr/src/render/light.rs` (stock `PointAndSpotLightViewEntities` `:995`) | Partial | Needs a top-down depth map; riding the shadow pass is the cheapest legal way | 301 + part of ~70 | **Keep but thin**: the vendored patch only exists because injected subviews get every GPU-culled mesh | S |
| 7 | Load-side mesh throttle `max_loading_meshes`, `sf_world_bevy/src/stream.rs:82-91` | `RenderAssetBytesPerFrame` `bevy_render/src/render_asset.rs:509`; `RenderMesh::byte_len` `bevy_render/src/mesh/mod.rs:223` means it can meter mesh preparation | Partial (inference) | Comment says the mesh allocator has no per-frame cap. Whether the stock limiter delays slab allocation as well as preparation was not checked; textures.md §4 keeps it off because it is whole-asset | ~40 | **Keep** until someone checks; a one-line test, not a system | S |
| 8 | Audio buses, tweens, attenuation curves in `sf_audio` (`lib.rs:34,85`, `attenuation.rs`) on `bevy_audio` | `bevy_kira_audio` 0.27.1 (Bevy ^0.20, MIT/Apache): tracks, tweens, distance curves; `bevy_seedling` 0.8 still Bevy ^0.19 | Partial | audio.md "Backend" (`superfargoth/docs/specs/audio.md:141-159`): each gap is small and pure; kira is a second stack beside Bevy's. Still holds | ~2.0k + 1.3k | **Keep**; the spec already names the DSP upgrade path | n/a |

## Keep, justified (looks duplicated, is not)

- **Animation runtime** (`sf_anim` 14.2k, `sf_anim_graph` 11.9k): `bevy_animation` 0.20 has only bit masks
  (`bevy_animation/src/graph.rs:82-97`), no root motion, no hook between blend and write-back, per-bone evaluation
  (`animate_targets`, wiki `bevy/skinning-and-animation.md` §5). Reasons in `superfargoth/docs/specs/animation.md:90-110` still
  hold. `bevy_animation_graph` is still 0.11 on Bevy ^0.19; `bevy_mod_inverse_kinematics` 0.12 still Bevy ^0.19.
- **Skinning, joint blocks** (vendored P5, `vendor/bevy_pbr/src/render/skin.rs`): stock skinning is used; the patch only
  shares joint buffers. Upstream #23283 (merged, in 0.20) only stops table scans, still keys per mesh.
- **Character merge** (`sf_character` 6.8k): no crate does modular characters (`specs/character.md:68`).
- **UI kit** (`sf_ui`, `sf_ui_bevy`, `sf_ui_screens`): built on `bevy_ui_widgets` (used in 10 screen files),
  `bevy_input_focus` + `AutoDirectionalNavigation` (`sf_ui_bevy/src/nav.rs`), `bevy_flair` CSS. Keyed row
  reconciliation (`sf_ui_screens/src/keyed.rs`) has no 0.20 equivalent (BSN in `bevy_scene` 0.20 spawns, it does not
  reconcile lists). Feathers is dev-only by its own docs (`specs/ui.md:93`).
- **Keymap and rebinding** (`sf_ui/src/keymap.rs`, 968 lines): `bevy_enhanced_input` 0.27 gives contexts and
  bindings, not a persisted per-device keymap with scopes. Input itself is the crate, not ours.
- **Dev console** (`sf_console`): reuse check `specs/console.md:26-45` holds; `bevy_console` 0.19 is on Bevy 0.20 now
  but still has no typed variables and pulls egui + clap.
- **Diagnostics** (`sf_diag`, `sf_diag_bevy`): reads `DiagnosticsStore`, `RenderDiagnosticsPlugin` GPU timings and
  `Time<Real>` (`specs/diag.md:51-60`); what is ours is the benchmark/gate harness, which Bevy does not have.
- **Saves** (`sf_save`, `sf_save_bevy`): `bevy_world_serialization` 0.20 dumps reflected components, unversioned; our state
  lives in Bevy-free cores (`specs/save.md:43-47`). Still holds.
- **Bake** (`sf_bake`, `mw_bake`): Bevy `AssetProcessor` is one source to one product (`specs/bake.md:51`); our steps
  are cross-asset (precombine, nav, far field). Bevy reads products through our `AssetReader`
  (`sf_model_bevy/src/source.rs:82`), which is the right seam.
- **Physics wrapper** (`sf_physics`, `sf_physics_bevy` on `rapier3d` 0.36 directly): `specs/physics.md:60-70`.
  New fact: `avian3d` 0.8.0 (2026-10-20) now targets Bevy ^0.20 and `bevy_rapier3d` 0.37 too. The decision rested on
  joint motors and our own floating-origin rebase, not on versions, so it holds; avian 0.8's joint motors were not
  re-checked (inference: unchanged).
- **Ragdolls** (`sf_ragdoll` + optional `mw_delirium`): two active-ragdoll paths, but Delirium is an opt-in GPL-3-only
  feature and an owner decision (wiki `engine/physics.md:35-41`).
- **Navmesh** (`sf_nav_recast` over vendored `navmesh-rs` 0.2.2): a Recast/Detour port is the industry standard;
  `vleue_navigator` 0.17 (Bevy ^0.20) is polyanya-based, not tiled Detour with crowds. `oxidized_navigation` last
  released 2024.
- **Floating origin** (vendored `big_space` 0.14.0-rc.0): Bevy has none. Our patch is low-precision dirty-tree pruning;
  big_space PR #86 (open) covers only the high-precision side.
- **Atmosphere**: stock `bevy_pbr::atmosphere` with patches (below). `EnvMapRefresh` (`sf_sky_bevy/src/env_refresh.rs`,
  62 lines) is needed because `AtmosphereEnvironmentMapLight` has no update-rate field (`bevy_light/src/probe.rs:306-317`).
- **Clouds, height fog, froxels** (`sf_atmos`, `sf_atmos_bevy`): Bevy 0.20 `VolumetricFog` is screen-space raymarch with
  directional lights only (`bevy_pbr/src/volumetric_fog/mod.rs:1-30`), `DistanceFog` has no height
  (`bevy_pbr/src/fog.rs:56`), no clouds. Reasons in `specs/weather/fog.md:44-45,94` hold.
- **Cascade splits** (`sf_sky_bevy/src/cascades.rs`, 85 lines): Bevy uses pure geometric splits
  (`bevy_light/src/cascade.rs:41-53`); ours is the PSSM practical blend fed into Bevy's own `CascadeShadowConfig`.
- **Lamp shadow budget** (`sf_sky_bevy/src/shadow_budget.rs`): Bevy has no shadow budget. `drop_stale_light_views` (`:201`)
  works around a 0.20 view leak; remove when Bevy frees them.
- **Texture residency and rebind** (`sf_texture`, `sf_texture_bevy`): `specs/textures.md` §4 is thorough; image changes do
  not re-prepare materials in 0.20 (wiki `bevy/render-assets-and-materials.md:44-48`).
- **Far field / HLOD** (`sf_lod`, `sf_lod_bevy`): Bevy has no HLOD; it uses stock `VisibilityRange` for fades.
- **Terrain, water, GI, video, precipitation**: Bevy 0.20 has no terrain, ocean, raster DDGI or video. Precipitation boxes
  (`sf_precip_bevy`) are closed-form, not particles, so hanabi would not replace them.
- **Free camera**: the game already uses Bevy's `bevy_camera_controller::free_camera::FreeCamera`
  (`mw_controls_bevy/src/lib.rs:40`); the `fly` helpers in examples are a few lines each.

## Vendored patches vs upstream

| Patch | Upstream state (2026-11-15) | Action |
|---|---|---|
| #26235, #26236, #26237 (DLSS cluster/transmission/OIT) | MERGED on main (10-20 to 10-27), **not** in 0.20.1 | drop on 0.21 |
| #26234 (SSAO main-pass res), #26170 (occlusion edge clamp) | MERGED on main, not in 0.20.1 | drop on 0.21 |
| #26095 (stale unpack jobs), #26418 (preprocess bind-group cache) | OPEN | keep; #26095 is entangled with the GS port (#1) |
| #26321 (`ExtendedMaterial` bindless fallback) | OPEN | keep |
| #25978 (`GpuBatchedMesh3d`) + own GS tags | OPEN, merge conflicts | **delete with #1** |
| #26149 + #26161 (own fork) | CLOSED unmerged; issue #25576 OPEN | keep (finding #4) |
| Own: shadow early-out, lamp filter, SH-20 mover refresh, H2 PCSS, RO-11 SSR/contact | no upstream equivalent found | keep |
| Own: CPU-listed shadow views | none | keep the sun-view half, delete the `CpuListedPointShadowCasters` half (#2) |
| Own: incremental `extract_visibility_ranges` | none (only #24381, merged 2026-05, in 0.20: stable buffer indices, not incremental) | keep, offer upstream |
| Own: `SharedImageSamplers` | none found | keep (used by `matclass`, not only the scene) |
| Own: phase counts, preprocess bind-group count, pipeline compile timing | counters only | keep; drop the scene-only `instance_batch_*` counters with #1 |
| Own: atmosphere LUT cache, P1 medium scales, N2 airglow, N1 ambient specular, sky-after-water, FLICKER-1 origin rebase, GI probes, surface weather, P5 joint blocks, P4 memo | none upstream | keep |
| `bevy_solari` scene bind group without lights | none | keep |
| `navmesh-rs` detached query | own, offerable | keep |
| `dlss_wgpu` facade | licence wall, not a feature gap | keep |
| Bevy 0.20.1 (2026-10-28) | we are on 0.20.0 because of the vendor tree; 0.20.1 has 10 fixes incl. #26090 `UninitBufferVec` size and #25336 fog volumes near plane | inference: worth a look in the "update all" pass |

Not duplication but stale docs found on the way: `docs/wiki/engine/shadows.md` "Lamp cache" bullet describes
`sf_shadow_cache_bevy` as the active cache; the game runs the vendored per-face cache. `ROADMAP.md:135` waits for
`bevy_hanabi` on 0.20, which shipped 2026-11-01.

## What I did not get to

- `sf_controller` / `sf_action` / `sf_camera` vs character-controller crates (`bevy_tnua`, avian's kinematic controller):
  wiki says Bevy has none; crates not re-checked.
- `sf_stats` / `sf_effects` vs `bevy_gauge` / `bevy_diesel` (`specs/stats.md:35` mentions them; not re-checked).
- `sf_render_order_bevy` (626 lines) vs Bevy's transparent sort and `render_sky` ordering.
- `sf_records` / `sf_mods` / `sf_vfs` internals beyond the `AssetReader` seam.
- `sf_ai`, `sf_script`, `mw_script`, `sf_rpg*`: content/gameplay, no Bevy counterpart expected; not opened.
- Whether `RenderAssetBytesPerFrame` limits mesh slab allocation (finding #7).
- Whether avian 0.8 added spherical joint motors (would reopen the physics fallback question, not the decision).
