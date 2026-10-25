## Distant shadows

Before 2026-10-12 (raster): 4 cascades to 600 m in the app (300 m in `SkyRenderOptions`' default). Bevy treats everything past the last cascade as fully lit with a
hard cut. Terrain past it has no shadows at all, and objects "acquire" shadows as they cross 300 m
(analysis in "Near-term visual fixes").

**Design: four tiers, each the cheapest representation for its range** (UE's CSM → far → distance
field structure, with a heightfield horizon map instead of distance fields):

| Tier | Range | Casters → receivers | Mechanism |
|---|---|---|---|
| 1 CSM | 0 → `s_max` (**250 m** since shadows F3; was ≥ `d1 + w` + margin = 504 m) | everything near → everything | Bevy cascades (4), retuned |
| 2 Far sun visibility | `0.8·s_max` → horizon | terrain (+ later far-field proxies) → terrain, proxies, impostors, grass, fog, clouds' ground bounce | **shadow-height map** (SHM) + the far-shadow correction in our materials |
| 3 Cloud shadows | 16 km around the camera | clouds → everything | cloud shadow map → `DirectionalLightTexture`, fog |
| 4 Terrain sky visibility (optional) | whole world, static | terrain → ambient on terrain/objects | baked horizon-based ambient occlusion per terrain tile (`terrain.skyvis` bake step, 8 directions) |

**Why `s_max` ≥ `d1 + w`.** Every L0 object (individual placements, out to the far-field handoff `d1`
plus its band) is then always inside the CSM. No object ever crosses a shadow boundary. Only terrain
and proxies do, and tier 2 covers them with a crossfade.

**Shadow-height map** (Max 1988, Sloan and Cohen 2000, reduced to one sun direction):
- 2 levels, toroidal and aligned like the height clipmap: 1024² at 2 m (2 km) and 1024² at 8 m (8 km);
  optional 1024² at 32 m. RG16F: `h_s` (height of the shadow boundary above that XZ) and `d_occ`
  (horizontal distance to the occluder that sets it).
- Compute: for each texel, march toward the sun in XZ over the height clipmap (finer levels near,
  coarser far, 48 exponentially spaced steps to 4 km horizontal),
  `h_s = max_i (h(x + s_i·L̂_xz) − s_i·tan(elevation))`.
- Receiver test: `vis = smoothstep(h_s − w, h_s + w, P.y)`, with penumbra half-width
  `w = d_occ·tan(sun angular radius + softening)`. One fetch per receiver, for terrain, objects,
  grass, fog and clouds alike.
- Update: strips when the clipmap scrolls; a full refresh spread over 8 frames whenever the sun moves
  > 0.05° (at Morrowind's time scale 30 the sun moves ~0.125°/s, so a refresh every ~0.4 s). Below 1°
  sun elevation, fade to unshadowed sun, since the sun itself is fading there.
- Phase 2: far-field proxies cast. A top-down orthographic pass rasterizes L1/L2 proxy heights
  (max blend) into a caster-height texture that the march reads with the terrain. Big structures
  (Vivec cantons, Ghostfence, strongholds) then cast far shadows.

**Far-shadow correction** (Bevy has no far-shadow hook, and past the last cascade it returns 1.0): in
**our** materials (terrain, proxy, impostor, `WorldMaterial`, grass), after `apply_pbr_lighting`:

```
f      = smoothstep(0.8·s_max, s_max, view_depth)      // 0 inside the CSM, 1 beyond
s_csm  = view_depth < s_max ? fetch_directional_shadow(sun, …) : 1.0   // only evaluated in the band
s_far  = shm_visibility(P)                              // tier 2
sun    = directional_light(sun, &lighting_input, true)  // Bevy's own function, unshadowed
color += (mix(s_csm, s_far, f) − s_csm) · sun          // = replace the sun's shadow term
```

This is exact for the shadowing light, because Bevy's sun contribution there is `s_csm · sun`
(cloud texture and atmosphere transmittance are inside `directional_light`). It costs a few ALU
outside the band and one extra light evaluation inside it. It needs the `LightingInput` that
`apply_pbr_lighting` builds internally: `sf_far_shadow.wesl` rebuilds it in ~25 lines mirroring Bevy
0.20 (named scaffold `FAR_SHADOW_LIGHTING_INPUT`, re-checked on every Bevy upgrade). The removal path is
an upstream Bevy hook (a directional-light "far shadow" texture or a cascade end fade; candidate
contribution, like the Solari transmittance patch in sky-lighting.md).

**As built (2026-10-12), replacing the correction above.** Bevy 0.20 already has the hook:
`PbrInput::directional_shadow_factor` multiplies every shadowed directional light's cascade term
(`pbr_functions.wesl`), and past the last cascade that term is 1.0, so the factor *is* the shadow there.
No `FAR_SHADOW_LIGHTING_INPUT` scaffold and no Bevy override. A multiplier can't fade the cascades *out*,
so the cascade end is a **dithered transition** (UE's dithered CSM transitions; Bevy's own
visibility-range crossfade): over `0.85·s_max → s_max` a growing fraction of pixels (ordered 4×4 dither)
takes the far term, and for those our materials move the lighting position just past `s_max` along the
view axis, so Bevy's cascade lookup returns 1.0 (directional lighting doesn't depend on the position;
the material restores it before fog). Object shadows fade out instead of ending on a line at 504 m.
Applies wherever a pixel past `0.85·s_max` can be: terrain, proxies, impostors (L0 objects end at 440 m).
- Terrain (`sf_terrain_bevy/src/shaders/far_shadow.wesl`): per-pixel horizon march over the height
  clipmap (14 steps to 5 km, stops above the highest resident height, skipped where `N·L ≤ 0`) out to
  700 m view distance, then the SHM (one fetch), dithered over 560-700 m.
- SHM (`sf_terrain::shadow::far_shadow_map`, `sf_terrain_bevy::FarShadowMap`): CPU, on the async pool,
  384² × 24 m (9.2 km) around the camera, `Rg16Float` (h_s − base, d_occ), from the resident heights
  resampled at 24 m; refreshed when the light turns 0.25°, the camera moves 1 km, or every 8 s. One
  level instead of the 2 planned: the near range is the march's. Unoccluded texels are floored to
  ground − 2 texels so bilinear filtering stays sane.
- Proxies become `ProxyMaterial = ExtendedMaterial<StandardMaterial, FarShadowExt>` (`proxy.wesl`,
  forward); impostors bind the map too (`far_shadow_map.wesl`). The app copies `FarShadowMap` into
  `sf_lod_bevy::FarShadowInput` (no crate dependency between the two).
- Cost (release, 1440p, AC, RTX 4060 Laptop, 2 × 12 s runs each, frame median; sunset 18:30):
  CSM 504 m / 2048² vs the old 600 m / 2048²: **−0.1 to −0.3 ms**; 4096² was +1.4 ms and is dropped.
  Far term (all materials): Red Mountain view ≈ **+0.0-0.4 ms**, north vista over the Foyada
  (`--camera "0,+250,0,300,0,-2000"`) ≈ **+0.5-0.6 ms** (main opaque pass +0.6-0.8 ms). Net vs the
  pre-session state: +0.0 / +0.4 ms. Over the High budget (0.15 + 0.08) in vistas; open: profile
  the split (terrain march vs map fetches vs proxies) and cut per preset (map-only terrain on Low/Medium).

**Not chosen:** more cascades (4 is Bevy's cap, and far cascades re-render casters with a cost that
grows with distance); distance-field shadows (no mesh SDFs or global SDF in Bevy; a project of its own,
revisit for M7 if proxies need sharper far shadows); virtual shadow maps (not in Bevy); static baked
shadow maps (the sun moves).

**Solari path:** ray-traced shadows have no range limit. Tiers 2 and 4 are off. Tier 3 applies if
Solari honors the light texture.

### Shadow bias and contact shadows (shadows F1 + F2, as built 2026-10-19)

Research: `docs/research/shadows/README.md` (fixes F1, F2). Rule 8: all Bevy 0.20, no new code paths.

- **Contact shadows (F1):** Bevy's `ContactShadows` on the exterior camera (screen-space march of the depth
  prepass toward the light, `bevy_pbr` `calculate_contact_shadow`) plus `contact_shadows_enabled` on the sun
  (`sf_sky_bevy::SunShadow::contact_shadows`) and on the budgeted shadowed point lights only
  (`PointShadowBudget::contact_shadows`; never the fade twins: one march per lit pixel per light). Bevy multiplies
  it into the shadow-map term, so it only darkens, and only where the map says lit. Presets: Low off; Medium
  8 steps; High/Ultra 16 steps; length 0.5 m, thickness 0.1 m. The moon gets none (it has no shadow map and
  would add a second march all day). A/B `--off contactshadows`; live: BRP `DirectionalLight.contact_shadows_enabled`.
- **The prepass it needs:** `ContactShadows` requires `DepthPrepass`; ours adds `MotionVectorPrepass` with it,
  because a depth-only prepass ignores the `VisibilityRange` dither (holes at LOD crossfades, `exterior.rs`
  note). So Medium and up now run the depth + MV prepass (measured 0.4 ms earlier); Low keeps none.
- **Bias (F2):** sun normal bias 1.8 → **0.8 texels** (depth bias 0.02 m kept), `exterior.rs` `SUN_NORMAL_BIAS`;
  per cascade the lift falls from 2.5 / 9 / 34 / 130 cm to 1.1 / 4 / 15 / 56 cm. No acne seen at 15:00 in Seyda
  Neen or Balmora; grazing sun (07:00, the 6a outline) is the owner check SH-7. Point lights keep Bevy's 0.08 m / 0.6 texel (already in range).
- **Filtering (F2):** `ShadowFilteringMethod` per preset on the camera: Low `Hardware2x2`, Medium and up
  `Gaussian`. `Temporal` needs TAA (raster runs none); revisit with F5 (PCSS).
- Seen 2026-10-19 (1080p, 15:00): differences are small at a high sun; cascade 0 already grounds feet and pots.
  Contact shadows mainly darken creases (column bases, wall feet) and the 30-40 m NPC contact. Expected larger
  at low sun and in cascades 2-3; owner sign-off SH-7.

### Sun cascades (shadows F3, step a as built 2026-10-19)

Research: `docs/research/shadows/README.md` (F3, owner decision 2026-10-19: drop the 504 m rule). **Supersedes
"Why `s_max` ≥ `d1 + w`"** and A1's range: the CSM no longer has to contain every L0 object.

- **Splits:** `sf_sky_bevy::CascadeSplits`, the practical split (PSSM, GPU Gems 3 ch. 10): each bound is
  `λ·n·(f/n)^(i/N) + (1−λ)·(n + (f−n)·i/N)` with a 1 m split near (the 0.1 m camera near would give a 0.7 m first
  cascade). It builds a plain Bevy `CascadeShadowConfig` (`SkyRenderOptions::cascades` is now the built component),
  so Bevy 0.20 still does the rest: bounding-sphere fit per slice + snapping to whole texels (`bevy_light`
  `cascade.rs` `calculate_cascade`: stable cascades, no shimmer as the camera moves or turns) and the 20 % overlap
  blend between cascades. Rule 8: Bevy's builder can only do "first bound + geometric", so the split is ours (12 lines).
- **Presets** (`exterior.rs` `Quality::sun_cascades`): Low 3 cascades to 200 m (λ 0.85); Medium/High/Ultra 4 to
  250 m (λ 0.9). Maps stay 2048² (4096² was +1.4 ms).
- **Texels** (70° vertical FOV, 16:9, 2048²; `sf_sky_bevy` test `practical_split_cascades_and_texel_sizes`, Bevy's
  sizing): High bounds 9.9 / 26.8 / 75.4 / 250 m, texels **1.4 / 3.8 / 10.5 / 35 cm**; A1 (10 / 36.9 / 136 / 504 m):
  1.4 / 5.2 / 19 / 70 cm. Cascade 3 is 2× sharper, cascades 1-2 1.4-1.8×; with the 0.8-texel normal bias the
  lift in the last cascade falls from 56 to 28 cm.
- **Cost** (dev build, 1440p, `exterior_seyda_neen` bench 2 × 12 s each, RTX 4060 Laptop): Bevy-timed GPU
  17.4 / 16.2 ms vs A1 16.8 / 16.4 ms (noise, ≈ 0); frame median 30.0 / 29.4 ms vs 35.0 / 33.3 ms (the dev build is
  CPU-bound: fewer casters queued in the last cascade). VRAM unchanged (4 × 2048² × 32 bit = 64 MB; Low 48 MB).
- **A/B:** the legacy layouts (`--off shadowsplits`, `--off shadowtune`) were deleted in shadow reset A2 (2026-10-21).
- **Gap:** L0 objects at 250-440 m get no object shadows: their shadows on terrain fade with the
  dithered cascade end (212-250 m, unchanged code: it reads `s_max` from the light), and object *receivers* (NIF
  statics' materials have no far term) end on Bevy's hard cut at 250 m. The terrain march (to 700 m) and the SHM
  are unchanged.

**Step b (cached far static shadow map, shf3b-d, never on by default): deleted in shadow reset A2 (2026-10-21).**

**Shared sun-view mechanism (intpanic 2026-10-19):** extra sun shadow views (now only the precipitation occlusion map) go through `sf_sky_bevy::sun_view` (`SunViewRequest`, `SunViewSource`, `add_sun_view::<S>`): extract the request, inject its casters as a light subview, spawn the view after `prepare_lights`. Invariant from `bevy_pbr` (`get_shadow_map_visible_entities` panics otherwise): a `LightEntity::Directional` view in `ViewLightEntities` needs the light's `RenderShadowMapVisibleEntities` with its subview key, which Bevy extracts only for a visible light with shadow maps on. The main-world request (PreUpdate) can be stale (the sky turns sun shadows off in Update, e.g. interiors), which panicked the render thread in interiors; `view_allowed` now gates the spawn and the request is dropped (unit test `view_needs_shadows_and_its_entry`).

## Near-term visual fixes (before M6)

### Bug A: distant objects have no shadows, then acquire them up close

**Cause** (verified in source):
1. `SkyRenderOptions::cascades` = 4 cascades, `first_cascade_far_bound` 12 m, `maximum_distance`
   **300 m** (`sf_sky_bevy/src/lib.rs`).
2. Bevy 0.20 `fetch_directional_shadow`: when the fragment's view depth is past the last cascade's
   far bound it returns **1.0**, which is lit, with no fade (`bevy_pbr/src/render/shadows.wesl`).
   So everything past 300 m view depth is unshadowed, and the boundary is a hard line that moves with
   the camera.
3. The far-field handoff is `d1` = 400 m (far-field.md). L0 objects live from 0 to 440 m, so between
   300 and 440 m they draw **without** shadows and gain them as they cross 300 m. L1 proxies are
   configured to cast "into far cascades" (`FarFieldSettings::l1_shadows`), but no cascade reaches
   400 m+, so proxies never cast or receive.
4. Terrain past 300 m has no sun shadows at all, so the hills look flat (`shots/farfield/*`).

**Fix, in order** (each step verified by screenshots before the next one starts):

- **A1. Retune the CSM** (config only). This is in `sf_sky_bevy`, owned by the dlss session, so
  coordinate or apply the change after it finishes. `maximum_distance = d1·1.1 + margin` (default
  500 m), derived in the app from `FarFieldSettings` so the two can't drift, `first_cascade_far_bound`
  10 m, `overlap_proportion` 0.2, `DirectionalLightShadowMap` 4096 on high and 2048 on low. Keep
  `l1_shadows` on: proxies now cast in the 400-500 m band. They duplicate the L0 shadow there, and
  shadows combine by `min`, so nothing is visible.
  Verify: `perf suite` balmora 1440p, A/B on the shadow pass GPU time (expect +0.3-0.8 ms). A
  screenshot sequence flying from 700 m to 100 m at a Balmora building: shadow present at every
  distance below 440 m. `balmora_a` re-shot.
- **A2. Shadow-height map** in `sf_atmos_bevy` (or first inside `sf_terrain_bevy`, moved later),
  computed from the height clipmap (above). It needs only the terrain clipmap and the sun direction.
  Verify: CPU test against `sf_atmos`'s reference march on a synthetic ridge, and a GPU readback test.
- **A3. Far-shadow correction** in the terrain material first (it already calls
  `apply_pbr_lighting`), then impostors, then proxies (proxies become an `ExtendedMaterial`, which
  far-field.md's phase-2 far textures need anyway).
  Verify: `redmtn_view` and `summit_vivec` re-shot at 7:00 and 17:30 (long ridge shadows), and a
  fly-through with no shadow line crossing the terrain near 400-500 m. GPU cost of the band: < 0.2 ms.
- **A4.** Cloud shadows wait for the cloud pass (M6).
- **Status (2026-10-12):** A1 done with 2048² (4096² measured +1.4 ms, over budget), range derived in
  the app (`exterior.rs` `sun_cascades`: `(handoff(1) + band(1))·1.1 + 20` = 504 m, first bound 10 m,
  overlap 0.2). A2 + A3 done for terrain, proxies and impostors, through Bevy's
  `directional_shadow_factor` and a dithered cascade end (see "As built" in "Distant shadows").
  The app ran the CSM at 600 m before this (not the 300 m this spec assumed). A/B:
  `--off farshadows` (far term off everywhere); `--off shadowtune` (old cascades) was deleted in shadow reset A2.

### Bug B: some objects pop in close (the Ghostfence)

**Cause** (verified in the bake store and code):
1. The Ghostfence wall segments `meshes/x/ex_gg_fence_s_01..04.nif` and `ex_gg_fence_s_h_01.nif` are
   **not cooked**: their `mesh.cook` product is `Skipped { reason: animated node "Tri Object03" }`
   (the energy field's material is animated). The pylons and plain fence pieces (`ex_gg_pylon_*`,
   `ex_gg_fence_0*`) are cooked. 736 of 6,619 models are skipped this way (bake-adapter.md), covering
   ~6% of placements; the viewer's "112 uncooked" counts placements spawned through that path so far.
2. `lod.policy`/`lod.l1` read `mesh.cook` and drop `Skipped` models (`sf_lod/src/bake.rs`:
   `Cooked::Skipped { .. } => Ok(None)`). So uncooked models have **no far-field representation**.
3. With a far field, every placement spawns at R1 (520-648 m) and none in R2. So an uncooked object
   exists only while its cell is in R1.
4. The runtime NIF spawner (`apps/bevyrrowind/src/exterior.rs`, `NifSpawner`, also `LightSpawner` for
   every light placement) **ignores `SpawnCx::max_distance` and `end_band`**: no `VisibilityRange`, so
   no dither-in. And the NIF loads through the `AssetServer` only when spawned, so it appears some
   frames **after** its cell's group is revealed.
   Net effect: big animated statics (the Ghostfence) are invisible past ~520-650 m, then pop in at full
   opacity, late. Lamp models (light placements) also never fade and double-draw over their merged
   L1 copy until the cell leaves R1.

**Fix, in order:**
- **B1. Ranges on NIF-spawned meshes** (`mw_assets` + the app): `NifSpawner`/`LightSpawner` put
  `NifRange { end: cx.max_distance, band: cx.end_band }` on the `WorldAssetRoot`. A system on scene
  spawn inserts `VisibilityRange { start: 0..0, end: e..e + band }` on every `Mesh3d` descendant, with
  `e = end + |node origin − placement origin|`. The dither is measured from each node's origin, and
  the extra offset guarantees no node fades before the proxy is fully drawn (the lead-band invariant,
  far-field.md). Verify: `--far-tint` at 380-460 m from a lamp shows a clean handoff, and the dither
  appears on NIF meshes.
- **B2. Rest-pose cook for far use** (`sf_model` + `mw_bake`): a new product `mesh.rest` for models
  `mesh.cook` skips **only** for animated nodes, particles, billboards or switches (never skinned). It
  is the model at t = 0: material/UV animation is irrelevant to geometry, transform keyframes at
  frame 0, switch → first child, billboards as static quads, particle systems dropped. `lod.policy`
  and `lod.l1` use `mesh.rest` when `mesh.cook` is `Skipped`. Bump the far rules version. Blended
  parts still leave L1 (phase-1 rule), so the energy field itself stays out of the far field until
  proxies support emissive/additive parts (open question 7). Its pylons and frame appear.
  Verify: the bake summary prints the rest-cooked count, `--far-tint` at Red Mountain shows the
  Ghostfence green (L1), and flying at it from 1.5 km shows no pop.
- **B3. Preload uncooked models before reveal** (`sf_world_bevy`): a `PlacementSpawner::prepare(model)
  -> Option<UntypedHandle>` hook that the assets stage calls and waits on, like cooked models. The
  group then reveals with its NIFs loaded. Verify: a fly-through toward Ghostgate shows no late
  appearance after the cell group reveals.
- **B4 (M6/M7). Split cook**: cook the static nodes of animated models with a LOD chain, and keep only
  the dynamic nodes on the NIF path. This is bake-adapter.md's open question "gameplay keyframes block
  cooking" generalized, and it makes the Ghostfence a normal cooked static near the camera. Material UV
  animation becomes a material parameter (`nif-runtime.md` material controllers) instead of blocking
  the cook.

The Ghostfence's white energy field (the farfield-exterior session's "look bug") is a separate
material issue in the NIF material mapping (additive/emissive blend), not part of the pop.

**Status (nifpop session, 2026-10-12): B1-B3 built, B4 planned.**
- **B1 built** as a framework component, not an `mw_assets` one: `sf_world_bevy::SubtreeRange { end, band }`
  (`SubtreeRange::of(cx)`) on the scene root, and `apply_subtree_ranges` (PostUpdate, before
  `CalculateBounds`) gives every new `Mesh3d` below it `VisibilityRange` with the fade `[e, e + band]`,
  `e = end + |node origin − root origin|` (scaled). The app's `NifSpawner` and `LightSpawner` insert it.
  Unit test `subtree_ranges_reach_late_meshes`.
- **B2 built**: `sf_model::cook_rest` + step `mesh.rest` (`MeshRestStep`, registered next to
  `mesh.cook` in `mw_bake::model_baker`). It cooks with the `animated` flags ignored (rest transforms as
  imported; particle nodes carry no mesh, inactive switch children are hidden by the importer); skinned
  stays `Skipped`. `sf_lod`'s `cooked()` reads `mesh.rest` only when `mesh.cook` is `Skipped`, so it is
  baked lazily for exterior far-eligible models only. Versions: `lod.policy` 3, `lod.impostor` 3,
  `lod.l1-3` 4. Bake: **21 models rest-cooked** (lamps and fires: `light_tikitorch00` 1,457 placements,
  `light_torch10`, `light_dae_brazier00`, candles, lanterns, `furn_mist256`, lava sparks; 4,037
  placements over all cells), 0 skinned among them; the far rebuild took 20.5 s, the whole bake 71 s
  with models cached.
- **B3 built**: `PlacementSpawner::prepare(&AssetServer, placement, model) -> Option<UntypedHandle>`.
  Once a load's models resolve, the assets stage asks each placement's spawner
  (`PlacementSpawners::spawner_for`) and waits until the handles' recursive dependencies have loaded or
  failed. `NifSpawner`/`LightSpawner` return the NIF's `WorldAsset`. `StreamStats::prepared` counts them
  (Ghostgate start: 59 NIFs preloaded, 148 uncooked placements, settled in 7.6 s).
- **Ghostfence finding:** the wall segments `ex_gg_fence_s_*` are **activators** (`ACTI`, class
  `Custom(2)`), so they are not far-eligible at all, and every part of them is the alpha-blended field
  (three 0.5-opacity planes, UV-scrolled, emissive 0.3). B2 therefore gives them no proxy (blended
  parts leave L1); B1 keeps their ring range (no handoff, no fade); B3 removes the late appearance.
  Seeing the field from 2 km needs **B5**: blended/emissive L1 submeshes (open question 7) *and* far
  eligibility for scriptless activators (adapter: map an `ACTI` without a script to a far-eligible
  class, or a `FarSource` override). Not built: making them eligible now would fade them out at the
  handoff with nothing replacing them.
- **Look (GF-2), investigated, not changed:** the material maps faithfully (texture
  `tx_gg_fence_01/02`, `Blend` non-additive, alpha 0.5, emissive 0.3 through the base texture). Three
  stacked 0.5 layers give 87.5 % coverage of a pale texture lit by an unclamped sun, which reads as an
  opaque white wall. Candidate differences from the fixed-function original, to check with data before
  changing the shared mapping: (a) with `AmbientDiffuse` vertex colors the fixed-function diffuse alpha
  comes from the vertex color, while we multiply material alpha × vertex alpha; (b) NIF lighting is
  clamped to 1 (texture × min(1, emissive + ambient + diffuse)), Bevy's PBR lighting isn't; (c) the
  environment-map specular on large blended planes. Visual check GF-2 carries the A/B.
- **B4 plan (split cook, M6/M7):** the importer already marks animated nodes per node. (1) `sf_model`
  cooks the static subset (nodes not animated and with no animated ancestor) into the normal LOD chain
  plus a *dynamic remainder* model (the animated subtrees with their parent chain) written as a second
  product, `mesh.dynamic`; `Cooked::Static` gains an optional `dynamic` reference. (2) Material UV/color
  controllers stop blocking: the cooked material carries the controller curve as a material parameter
  (`nif-runtime.md` material controllers), so UV scrolling parts like the Ghostfence field cook
  statically. (3) The runtime spawns the cooked chain through `StaticSpawner` and the remainder through
  the content pack's hook, both under the placement root. (4) Gameplay keyframes (doors, 307 models)
  stay on the NIF path until animation graphs drive cooked nodes. Gate: the bake summary's skipped
  count drops from 736 by the UV/material/particle-only models (~300), bake time unchanged.

### Aerial perspective and haze (now)

Bevy's atmosphere already fogs every opaque pixel with the aerial-view LUT. The shots look clear
because the medium is a clean Earth sky and Vvardenfell views are ≤ 10 km.
- **N1.** Drive `sf_sky::Weather::haze` (already wired: Mie density multiplier, medium rebuilt at
  ≥ 0.02 change) from a temporary per-time-of-day curve in the app (`--haze`) until `sf_weather`
  exists: 2-3 on clear days, 4-8 on Bitter Coast mornings. Verify: `summit_vivec` and `redmtn_view` at
  several values. The owner picks the default.
- **N2.** Add an absorbing dust term to `AtmosphereModel` (a brown-grey Mie-like term with absorption,
  low scale height) so ash regions and storms tint the haze. The weather scales it.
  sky-lighting.md's `AtmosphereModel.terms` already supports extra terms.
- **As built (M6 F1, m6-f1, 2026-10-13).** `Weather` gained `dust` and `dust_tint`; `MediumTerm.weather_scaled`
  became `weather: WeatherScale { None, Haze, Dust }`; `Weather::medium()` → `MediumScales` feeds both the GPU medium
  and the CPU transmittance. `AtmosphereModel::earth()` has a 4th term, dust (per unit: extinction ~9.5e-5 /m at the
  ground, scale height 2 km, single-scattering albedo 0.87/0.81/0.71, Mie g 0.7); `build_medium` drops empty terms.
  Rebuilds: a step (haze 0.05, dust 0.02, tint 0.02) and at most every `SkyRenderOptions::medium_min_interval`
  (0.25 s real time); `MediumStats` counts them. Cost: GPU unchanged (same LUT sizes and passes; terms are baked into
  the LUTs); a rebuild is a ~2.8 ms CPU LUT bake at 256² on the render thread (release, `medium_rebuild_cost`,
  shared machine) plus a 1 MB upload, so ≤ 4 short hitches per second while the weather moves, none when it is still.
  F2/WE3: lower the phase LUT resolution or bake off-thread if the hitch shows in time-lapses.
  **Calibration changed:** "2-3 on clear days" assumed Earth-scale views. Morrowind's far field is 1-5 km away, where
  haze 2-3 is invisible (shots/f1/redmtn_old vs clear); the app now uses haze 20 for clear (aerosol optical depth
  ~0.18, a typical clear continental day), 60 foggy, dust 4 for ash storms (weather-adapter.md "Haze and dust"). Near
  fog at game distances stays F2's analytic height fog, not more haze. Known issue: a purple/white seam on the sea
  horizon at noon with dust (sky-view LUT vs aerial LUT; visual check FG-1).
- **N3 (optional prototype, owner's eyes only).** God rays now with Bevy's own volumetrics:
  `VolumetricLight` on the sun, `VolumetricFog { jitter, step_count: 32 }` on the exterior camera, and
  one camera-following `FogVolume` (≈ 300×60×300 m, low density). Megalophobia found large boxes go
  grey and dim (above), so keep it small. Measure. Throw it away when the froxels land.

