# Shadows: as-built history

Moved out of [shadows.md](../shadows.md) on 2026-10-20 (doc budget). Append-only; grep it.

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

## Shadows v2: one design (2026-10-20)

Moved out of shadows.md on 2026-10-25 (doc cleanup S6). Superseded by the shadow reset, 2026-10-21: the far static sun map and the 3-cascade/8-cached-lamp plan below are not the live design (see "Reset A2/B1/B2 as built" in shadows.md).


**Why.** Bevy 0.20 redraws every shadow view from scratch every frame, and each lamp cube face is a root view that
runs the whole `Core3d` schedule (~0.35 ms render thread per face, below). Our answers so far were layers on top of
that (budget of 1 lamp, far map off, cache without the view skip), and the look still fails (shadows "only when
super close", SH-11/13/14). Industry engines and Clockwork cache what doesn't move and redraw only what changed.

**Budget (High, 1440p, the walk):** all shadows ≤ 2 ms render thread and ≤ 1.5 ms GPU; 8 shadowed lamps; sun
shadows visible to the far-field handoff (~500 m).

| Tier | What | Redrawn | Bevy pieces | Ours |
|---|---|---|---|---|
| Sun near | CSM, **3 cascades** to ~150 m (Clockwork: 3 to 230 m), practical split | every frame | cascades, PCF/PCSS | splits (`CascadeSplits`); caster size cull deleted in shadow reset A2 (2026-10-21) |
| Sun far | **one cached static map** over the view distance (Clockwork, UE cached shadows), statics only; terrain horizon march past it | on sun move > 0.25°, camera move > 1/16 reach, new cells; spread over frames | an extra shadow view on redraw frames only | none: the far static map (F3b-d, never on) was deleted in shadow reset A2 (2026-10-21); terrain far shadow only |
| Lamps | **cached static cube faces** (Doom 2016, HDRP mixed cached, Clockwork 8-61 lamps); movers drawn over the copy, per face | statics on change; movers per frame (30 Hz Low/Medium), only faces holding one | cube array + sampling | `sf_shadow_cache_bevy` + **2b: no Bevy views for cached faces** |
| Contact | screen-space, sun + shadowed lamps, **short** (SH-14) | every frame | `ContactShadows` (0.19) | lengths per preset |
| Grounding | GTAO | every frame | SSAO (XeGTAO) | ROADMAP 1.5 |

**What goes away:** High's one-uncached-lamp budget (replaced by 8 cached), the 504 m four-cascade CSM (3 short
cascades), cascade-texel PCSS sizes (world-size penumbra, V3). Kept: importance selection, crossfade,
terrain far shadow, rain occlusion (vertical, already redrawn only on demand: Nsight 0.04/frame).

**Slices (each measured with the perf suite + Nsight; owner checks per slice):**
- **V0** Measure today's shadow costs: `--off shadows`, 3 vs 4 cascades, Nsight per cascade, census 8 lamps.
- **V1 Lamps 2b** (biggest CPU win): cached lights keep Bevy's cube-array layer but their six face views lose
  `RootNonCameraView`/`ExtractedView` after `prepare_lights` (Bevy runs nothing for them); one node per frame
  restores cached faces into Bevy's layers (slice 2's depth copy); faces whose frustum holds a mover keep a Bevy
  view that loads (not clears) the restored depth. Raise High to 8 lamps at 512², fade 40 m. Gate: 8 lamps ≤ 1 ms
  render thread (census, walk); no flicker when the set changes. **Built (shadows2, 2026-10-20), gate not met on
  the walk**: see "V1 as built" below.
- **V2 Sun**: 3 cascades on High/Ultra to ~150 m (the far static map was deleted in shadow reset A2, 2026-10-21).
  Gate: render -1 ms, shadows present 0-500 m (owner check).
- **V3 Soft shadows by world size**: Bevy's directional PCSS size is in cascade texels (penumbra jumps between
  cascades); scale it per cascade so the penumbra is the sun's angular size × blocker distance (Clockwork). Needs a
  small `bevy_pbr` shader patch (vendored like `big_space`, upstream PR). Plus SH-14 contact length.
- **V4 Cascade update scheduling** (only if V2 leaves cascades > 0.5 ms): far cascade at half rate with its matrix
  held between redraws (CryEngine `r_ShadowsCache`, Unity HDRP cached cascades).
- **V5 Correctness**: LM-1 leaf shadows, RO-1 translucent casters, SH-11 distant flashes, SH-13 lamp grain.

Goal: interiors lit by 20-30 point lights where every light that matters on screen casts a shadow (8-16 on High,
more on Ultra), at well under 1 ms, with no visible popping when the shadowed set changes. Today High shadows
**one** light, faded out over 15-25 m, because every shadowed light re-renders six cube faces every frame.


## Lamp slices (F6, before v2)

Moved out of shadows.md on 2026-10-25 (doc cleanup S6). Superseded by the shadow reset, 2026-10-21.


1. **(done)** Spec; importance selection. 2. **(done, parked 2026-10-19, unparked by v2)** cache, static only
(shadows-history.md). 3. Movers over the static copy, 8-16 lamps: now v2's V1. 4. Upstream to Bevy if small;
Clockwork extras (fixture self-shadowing, per-actor shadows) later.

## Old status header of shadows.md (2026-10-20)

Moved out of shadows.md on 2026-10-25 (doc cleanup S6). Superseded by the shadow reset, 2026-10-21.

Status: **v2 design (lights1, 2026-10-20, owner: "we need a smarter way to do shadows")**: one design for sun and
lamps, below; supersedes the F-slices. Lamp slices 1-2 built (`sf_sky_bevy::shadow_budget`, `sf_shadow_cache_bevy`,
the cache parked since 2026-10-19 and unparked by v2). Sun details (cascades, bias, far tiers) live in
[weather/distant-shadows.md](../weather/distant-shadows.md) ("Distant shadows", "Shadow bias and contact shadows"); diagnosis and Clockwork notes in
[../research/shadows/README.md](../../research/shadows/README.md); as-built history in
[shadows-history.md](shadows-history.md).
