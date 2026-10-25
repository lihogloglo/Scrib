## Fog: aerial perspective, height fog, ground fog, volumetric fog, god rays

Four ranges, one look:

| Range | Technique | Where |
|---|---|---|
| 0 to ~384 m (High) | **Froxel volumetric fog**: 160×90×64 at 1440p (16 px tiles, exponential slices), RGBA16F scattering/extinction | `sf_atmos_bevy` compute |
| froxel end to ~8 km | **Far fog raymarch** at ¼ res, 16 steps, in the cloud node | same node as clouds |
| analytic, all distances | **Exponential height fog** (UE formula: closed-form integral of `ρ0·e^(−(h−h0)/H)` along the ray, with sun in-scattering lobe), used for the far fog density and as the Low preset | shared WESL |
| 1 km+ | **Aerial perspective** (Bevy `Atmosphere`), haze driven by weather | Bevy |

**Froxel pass** (Wronski 2014, Hillaire 2015):
1. Density per froxel = height fog (preset) + ground fog banks (Megalophobia bank function: warped
   value noise advected by `sf_wind`, thresholded, over an exponential profile, origin-phased) +
   precipitation mist (intensity-scaled extinction, e.g. rain haze, ash) + local `FogVolume`-like
   records (boxes/ellipsoids with density + noise: swamps, caves, the Ghostfence, spell clouds).
2. Lighting per froxel: sun/moon × atmosphere transmittance × **CSM shadow** (near) ×
   **shadow-height map** (terrain, to the horizon) × **cloud shadow map**, with HG phase. Plus the
   sky ambient from the sky-view LUT, plus clustered point/spot lights (Bevy's cluster buffers, with
   a cap per froxel). A sun shaft is just a froxel lit through a gap.
3. Temporal reprojection (jittered slice depth, 90-95% history), then front-to-back integration into
   an in-scatter/transmittance volume.
4. Apply: a full-screen pass on opaque pixels (main depth) after `render_sky` and before the
   transparent pass. The transparent world materials sample the volume in their fragment shader
   (a `sf_atmos::fog` WESL function the water and glass materials call).

**God rays** come from the froxels near and the far fog farther out: shafts between buildings at
sunrise (CSM), shafts from ridges (shadow-height map) and shafts under cloud gaps (cloud shadow map).
No screen-space radial blur.

**Ground fog** (Bitter Coast morning): a time-of-day `KeyCurve` on the preset (dense from 4 to 9 h)
× climate. Banks drift with the wind. The far fog continues them past the froxels, so a valley of fog
reads from a ridge.

**Bevy `DistanceFog`**: off on the exterior camera when `sf_atmos` runs, or both would fog surfaces.
Kept for interiors until the environment volumes of M8.

**Solari path:** froxel lighting uses the raster CSM. On the ray-traced path the sun's shadow maps are
off (sky-lighting.md), so the froxels use the shadow-height map + cloud shadows only (or a cheap
low-resolution CSM kept for the fog alone). Open question 6.

### Fog as built (M6 F2, m6-f2, 2026-10-13): height fog, ground-fog banks, far fog

Reuse check: Bevy 0.20 has `DistanceFog` (per-material, distance-only, no height, no noise; it would also fog
the sky twice with the atmosphere) and `VolumetricFog`/`FogVolume` (box volumes; Megalophobia measured it grey
and dim at our scales, and it is the froxel tier's job, F3). Neither does height or bank fog over km views, and no
maintained crate targets 0.20, so the fog is ours, in the cloud node as planned.

- **Model** (`sf_atmos::fog`, CPU twin + tests): two exponential layers over sea level, `FogSettings { height,
  banks, albedo, phase_g, ambient, far, sky_distance, wind }`. Height fog = UE's exponential height fog
  (`FogLayer::optical_depth`, closed form written as a difference of end densities so steep rays never overflow;
  density capped at e² × below the base). Banks = a second, thin layer (12 m falloff in the app) × the Megalophobia
  bank mask: the weather map's G channel at two scales (1 km and 256 m tiles), domain-warped by a third sample
  (2 km tile, 220 m), stretched around its median (`NOISE_MID`/`NOISE_GAIN`, measured: the channel's 5-95 % spread
  is 0.29-0.43) and thresholded (coverage 0.42 + 0.18 ramp: ~half the ground in banks, a third in clear gaps,
  test `banks_cover_part_of_the_ground`). `FogDrift` advects banks with the ground wind (warp at 0.35×, wisps at
  1.35×), wrapped at 16 384 m with the origin phase like the clouds' scroll.
- **Render** (`sf_atmos_bevy`, same node as the clouds; runs with the clouds off too):
  1. *Far fog* in `clouds_trace`: per traced pixel, 16 quadratic steps over the ray's part inside the bank layer,
     out to `far` (6 km) or the depth buffer, store the bank **modulation** = the noise's share of the layer's
     optical depth, weighted by what the eye still sees (G channel of the distance target, now `Rg32Float`).
     `clouds_resolve` reprojects and clamps it with the clouds.
  2. `clouds_fog_light` (new, 66×1 `Rgba32Float`, once per frame): sky fill per view azimuth (sky-view LUT toward
     the horizon + zenith) and the key light (sun or moon, as seen at the camera) for the composite.
  3. `clouds_composite`, full resolution: both layers' closed-form optical depth to **this pixel's own depth**
     (sky pixels: `sky_distance`, 20 km), the bank layer × the bilaterally upsampled modulation, lit by the key
     light (HG g 0.6 over an isotropic floor) and the sky fill × albedo; composed as `T·(clouds) + L` in front of
     the clouds, so stars, moons and clouds behind dim with it. Depth edges are exact (the analytic part is per
     pixel); the reduced-resolution part only shapes the banks.
- Bevy `DistanceFog` was already off on the exterior camera; interiors keep their look. The sea scaffold became
  opaque (the composite runs before transparents and fogs by depth). **Transparents (particles, glass) are not
  fogged yet:** F3's `sf_atmos::fog` function for materials.
- Weather: `exterior.rs` `SkyLook::fog` per weather + `ground_fog(hour)` (weather-adapter.md "Fog"); `--fog <x>`
  scales, `--off fog` drops it. `sf_sky::Weather` stays the medium's input; the fog lives beside the clouds'
  settings because the node that draws it reads `sf_atmos` only (WE3's `WeatherPlugin` drives both).
- Also: Bevy's `sky_view_lut_samples` 16 → 32 on sky cameras (the F1 sea-horizon seam; a few µs).
- **Cost** (release, 2560×1440, RTX 4060 Laptop, `fg-balmora-foggy-noon`, `--off fog` A/B; shared machine: another
  `bevyrrowind.exe` used the GPU in every run): `clouds_fog_light` 0.004 ms; bank march +0.05-0.17 ms in
  `clouds_trace`; composite +0.36 ms when it walked the lights and sampled the LUTs per pixel, within noise once they
  moved to the fog-light texture. The 0.1 ms far-fog gate needs a quiet re-measure. Levers if over: fewer bank steps
  (8), skip the march where the bank layer's analytic depth is negligible, half-rate modulation.

### Froxel fog as built (M6 F3, m6-f3/m6-f3b, 2026-10-13): god rays, lit fog, transparents

**Bevy's `VolumetricFog` (decision, re-checked for 0.20).** Not used, on exteriors or as a stopgap (N3 stays
skipped, owner decision 7). `VolumetricFog` + `VolumetricLight` + `FogVolume` raymarch each pixel through each box
volume (`step_count` 64 by default) at full resolution: cost scales with pixels × steps × volumes (3.7 M pixels at
1440p vs 0.92 M froxels at High), there is no temporal reprojection (only per-frame jitter for TAA), transparent
materials can't sample it, it has no height-fog or bank density and no terrain or cloud shadow terms, and large
volumes show the artifacts of #22574. Our froxels reuse F2's density terms and the shadow maps we already have.
Revisit `FogVolume` only for small local volumes in interiors (M9 environment volumes), where its box shape fits.

- **Model** (`sf_atmos::froxel`, CPU twin + tests): `FroxelSettings { size, near, far, history, max_lights, jitter }`,
  presets Medium 128×72×48 to 256 m, **High 160×90×64 to 384 m (default)**, Ultra 160×90×128 to 512 m. Slices are
  exponential in distance along the view ray (not view z), `near` 0.5 m, so a pixel's slice depends only on its
  own distance (~10 % of the distance per slice at High: 38 m deep at the far end). `integrate_step` = Hillaire
  2015's energy-conserving slab integral (test against the homogeneous closed form).
- **Render** (`sf_atmos_bevy`, compute, in the cloud node after `clouds_fog_light`, before the trace):
  1. `atmos_froxel_inject` (8×8×1 groups, one thread per froxel): a Halton (2,3,5) jittered point inside the
     froxel; density = F2's height layer + bank layer × the bank mask (`fog_bank.wesl`, now a shared library with
     its own weather-map binding; the cloud trace calls the same function). Light = the key light seen at the
     camera (`clouds_fog_light`) × HG phase × **CSM** (Bevy's cascades and comparison sampler, one hard tap: a
     froxel is far coarser than a texel) × **terrain shadow-height map** (`FogShadowHeights`, the same rule as
     `sf_lod_bevy`'s far shadow: penumbra, distance bias, border fade) × **cloud shadow map**, + sky fill ×
     ambient, + **point lights**: the 32 nearest visible `PointLight`s whose range reaches the froxels (CPU pick
     per frame, unshadowed: Bevy's cube maps aren't sampled; a named limit), with Bevy's range falloff. Temporal:
     the point is reprojected into last frame's volume (origin shifts applied), history weight 0.9 inside the
     screen and the range, ping-pong `Rgba16Float` volumes.
     **Temporal sampling (shimmer, 2026-10-19):** the first build used ONE Halton(2,3,5) offset for the whole
     volume (16-frame cycle) and read the history at the jittered point. Every froxel erred in step, a
     history of 0.9 keeps ~26 % of a 16-frame cycle, and screen TAA cannot clamp a coherent change: the fog
     pulsed (owner report; measured ~1-2 % of frame luminance, static camera, clock paused, worst where shadow
     and bank edges sit in the volume). As built: `FroxelJitter::PerFroxel` (default): the frame offset is R3
     (Roberts 2018, never repeats) rotated per froxel by a pcg3d hash (Cranley-Patterson), so the residual is
     spatially uncorrelated noise the history and TAA/DLSS average; the history is read at the froxel centre
     (a still camera reads texel centres, no drift). A/B `--off fogdither` (old), `--off fogjitter` (centres).
     Cost: one integer hash + one centre ray per froxel (not benchmarked; negligible next to the CSM tap).
  2. `atmos_froxel_integrate` (one thread per tile column): front to back into (in-scatter, transmittance), into
     the shared `FroxelVolume` image.
  3. `clouds_composite`: the froxels from the camera to `min(depth, far)`, then F2's analytic layers only from the
     froxel end (`layer_depth` from the point at `far`), so the near fog isn't counted twice; the far bank
     modulation still shapes the analytic part. Sky pixels: froxels to `far`, analytic beyond.
  The froxels need the cascades bound (`ViewShadowBindings`); without them (or `Fog::froxels = None`, the Low
  preset) the composite fogs everything analytically as in F2.
- **Transparents** (`FroxelVolume`, `froxel_apply.wesl`, `FroxelFogMaterial`): the integrated volume is a shared
  3D `Image` that blended materials sample at their own screen position and distance (UE/Frostbite's
  translucency path). `froxel_fog_apply` handles blend, premultiplied (in-scatter × alpha), additive and
  multiplicative (transmittance only: the fog light isn't added twice). `FroxelFogMaterial` =
  `ExtendedMaterial<StandardMaterial, FroxelFogExt>`; the plugin keeps every such material's volume and params
  current. Past the froxel range the ray's medium carries on as the froxels saw it (`froxel_fog_lookup`: `T_far^(d/far)`, in-scatter × `(1 − T)/(1 − T_far)`; fogfix 2026-10-19), the stand-in for the composite's analytic far layers (exact for level rays through height fog, over looking up, under looking down; W2's water draws its own far fog). **As built in the app (fogfix):** `transparent_fog.rs` swaps every blended/additive/premultiplied/multiply `StandardMaterial` (NIF leaves without an alpha-test ref, the Ghostfence field, glows, glass, particles) for one fogged `FroxelFogMaterial` copy per source and copies NIF controller changes over; `--off transfog` is the A/B. Audit: the order (prepass → opaque + alpha-mask incl. impostors → composite → transparent → TAA/post) is the canonical split; the bug was the unwired translucent half. Open: Morrowind leaves (blend + test GREATER 0) are blended, so they skip depth, shadows and motion vectors; mapping them to alpha-test (UE "Masked" foliage) needs a re-bake and an owner look call. Light sprites (`sf_lod_bevy`, additive, night) are not fogged yet. Cost: one trilinear 3D fetch per translucent fragment (not benchmarked).
- Synthetic scene: `cargo run --release -p sf_atmos_bevy --example clouds -- --view glass --fog 400 [--time 23]`
  (glass panes beside stone panes at 20/60/150/300 m, a lantern by each).
- Weather (app): clear and fair keep a thin valley haze (40 km / 30 km visibility at sea level, ×1.5 at dawn by
  the ground-fog curve) so dawn shafts have a medium; `--off froxels` (alias `godrays`) is the Low-preset A/B.
- **Cost** (release, 2560×1440, RTX 4060 Laptop, `fg-balmora-foggy-noon`, A/B `--off froxels`, quiet machine for the
  froxel run): `atmos_froxel_inject` 0.122 ms, `atmos_froxel_integrate` 0.023 ms, composite +0.01 ms: **≈ 0.16 ms**
  against the High budget of 0.7 ms; timed GPU total 10.35 vs 10.21-10.23 ms, frame median 15.53 vs 15.31 ms (within
  noise). Bug found on the way: a render-world-only volume image was re-created every frame (+4 ms CPU); fixed.
  Not yet measured: the 3070, flight at 60 m/s for temporal smear, F2's quiet `--off fog` gate.
- Solari path: without sun cascades the CSM term is 1, so near shafts come only from the terrain map and clouds. Open
  question 6 stands (a fog-only low-res CSM vs SHM + clouds only).

