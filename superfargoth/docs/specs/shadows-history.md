# Shadows: as-built history

Moved out of [shadows.md](shadows.md) on 2026-10-20 (doc budget). Append-only; grep it.

## Slice 2 as built (shf6b, 2026-10-19): cache works, Bevy's per-view cost remains

Crate `sf_shadow_cache_bevy` (`ShadowCachePlugin`, `ShadowCacheSettings`, `ShadowMobility`, `ShadowCacheStats`;
Bevyrrowind adds it in `render.rs`, A/B `--off shadowcache`). Built as specified above with these choices:

- **Mobility is automatic** (no game markers needed): skinned meshes and `ShadowMobility::Movable` always move; any
  other mesh is baked once it has been still for `settle_time` (1 s) after appearing; a baked caster that moves
  (transform, `InheritedVisibility` or `Aabb` change) unbakes the lights it touches and **stays a mover** (at most one
  bake + one unbake per caster; without this rule actors' rigid parts and live emitters kept the lights recapturing,
  p99 300 ms). A light that moves is left to Bevy until it stops; flicker (intensity) never recaptures.
- **Copies are draws:** Bevy's cube texture has no `COPY_*` usage, so a fullscreen depth-copy pipeline
  (`copy_depth.wesl`, compare Always, `frag_depth`) restores our layer into Bevy's face view before
  `shared_shadow_pass` (taking the attachment's first use, so Bevy loads instead of clearing) and captures Bevy's
  face after the late shadow pass. GPU: 0.027 ms restore, 0.024 ms capture (8 lights, census).
- Captures are confirmed (six faces, current generation) only while no pipeline is compiling; slots are LRU, the
  cache texture grows in steps of 4 cubes (24 max at 1024² = 576 MiB worst case; usually 8 → 192 MiB).
- Known gaps: movers are missing from a light's map for the 2-3 frames of a capture; a static mesh whose asset
  loads more than `settle_time` after its entity spawns is baked without it.

**Measured (census bench, 720p, `--off farshadows`, machine busy with other builds):** median frame / render
thread, cache on vs off: 1 light 6.3 / 6.5 ms; 4: 12.0 / 13.0; 8: 20.7 / 21.0 (render thread 20.3 / 20.7);
16: 28.2 / 29.8. Without actors (`--off actors`, 8 lights): 19.6 / 20.2 ms, render thread 19.3 / 20.5. **So the
cost is not the casters drawn: it is Bevy's fixed per-view work** (~0.35 ms render thread per face view: view
setup, specialize/queue/batch/preprocess per view, even with an empty `CubemapVisibleEntities`). Emptying the
lists, as planned, saves only ~1 ms at 8 lights. Presets stay 0/0/1/4.

**Next (slice 2b): skip Bevy's views for cached lights.** In the render world after `prepare_lights`, take the
six face-view entities of a cached light out of Bevy's per-view systems (remove `ExtractedView` and
`RootNonCameraView`, keep `ShadowView` for its texture layer) and run our restore from one system (e.g. the
camera's Core3d before `shared_shadow_pass`, looping over cached lights), with movers drawn by our own small
shadow phase or by letting Bevy keep views only on faces whose frustum holds a mover (spec "per-face"). Check
that `prepare_lights` doesn't re-insert the removed components each frame (it does only for new/changed lights;
a flickering light is "changed" every frame: then filter `changed_point_lights` or re-remove after it). Then
re-bench 1/4/8/16 and raise High to 8.

## F5 as built (shf5, 2026-10-19): Bevy PCSS sun shadows

Bevy 0.20 has PCSS behind `experimental_pbr_pcss` (`DirectionalLight::soft_shadow_size`; point and spot lights have
`soft_shadows_enabled`, using `radius`). `sf_sky_bevy` gets a `pcss` feature forwarding it to `bevy_light` and
`bevy_pbr`; `SunShadow::soft_size` sets the sun's size. The game sets it on High and Ultra (`Quality::soft_shadow_size`,
10 texels); Medium keeps Gaussian PCF, Low Hardware2x2. `--off softshadows` is the A/B.
- The directional size is in **cascade texels**, not an angle: blur = (blocker gap in normalized depth) x size / depth,
  so the ~0.53 deg sun can't be set literally; 10 was tuned by eye (6 was barely visible, 14 fades tips strongly).
  A per-cascade angular size would need a Bevy shader change.
- Bevy's PCSS always uses the Jimenez-14 filter, noisy without `ShadowFilteringMethod::Temporal` + TAA; raster runs no
  TAA, so a faint dither shows (fine under `--dlss`). Not enabled on point lights (uncached maps already cost ~1.9 ms each).
- GPU at Seyda Neen 15:00 (dev build, 15 s bench): median 22.6 ms on vs 23.2 ms off, within noise (frame is not shadow bound).
- Owner check SH-10.

## Sun caster rules (ROADMAP 1.2, 2026-10-19)

Measured first (`bevyrrowind/docs/perf.md` "Frame budget"): at the frame target the render thread is the
bottleneck, and `--off shadows` takes ~4.8 ms off it in Balmora; the point lights and the far shadow take nothing
measurable, so the cost is the four sun cascades queueing, batching and encoding every caster they touch.
Rule 1, **caster size per cascade** (`sf_sky_bevy::CasterCulling`, UE `r.Shadow.RadiusThreshold`): a caster with a
bounding radius under `min_texels` (1.5) texels of a cascade leaves that cascade's list (High texels 1.4 / 3.8 /
11 / 35 cm: a 10 cm cup shadows in the first two only). It filters Bevy's `CascadesVisibleEntities` right after
`check_dir_light_mesh_visibility`, one task per cascade. A/B `--off castercull`.

## Measurements

Census office bench (`benchmarks/census_office.json`, streamed view, release, 1280×720, High, 1024² faces, RTX 4060
laptop + Ryzen 7 7435HS; shf6 2026-10-19), uncached Bevy maps, `--point-shadows N` (156 lights shown):

| Shadowed lights | Median frame | Render thread | Shadow views GPU | Shadow views CPU (encode) |
|---|---|---|---|---|
| 0 | 4.8 ms | 4.3 ms | 0 | 0 |
| 1 | 6.6 ms | 6.1 ms | 0.54 ms | 0.85 ms |
| 4 | 12.2 ms | 11.9 ms | 2.35 ms | 2.69 ms |
| 8 | 21.7 ms | 21.8 ms | (timer dropped) | 6.37 ms |

≈ +1.9 ms frame per shadowed light, **render-thread bound** (six views queued, specialized and encoded per light);
the GPU part is ≈ 0.55 ms per light at 1024². So raising the count on Bevy's uncached maps is not affordable: the
presets stay (Low/Medium 0, High 1, Ultra 4) and slice 2's cache must remove the per-frame views, not only the GPU
draws (empty `CubemapVisibleEntities` → nothing to queue). Visually 8 lights already change the room a lot
(table, chair and chandelier shadows on floor and ceiling; SH-9).
