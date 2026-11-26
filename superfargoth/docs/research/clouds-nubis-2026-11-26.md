# Why our volumetric clouds do not look like Nubis / Hillaire (research, 2026-11-26)

Read-only research (research-1 sub-agent). Nothing built, measured or launched. "Calc" = derived from code constants; "(unverified)" = recalled, not
read in a source this session. Industry numbers marked "read" come from the PDFs/sources listed in section 9, which were downloaded and converted
with `pdftotext` (text only, nothing executed).

## 1. Summary

- Our lighting model is already the Frostbite 2016 / Nubis one (dual-lobe HG, Wrenninge 3-octave multiple scattering a=b=c=0.5, powder, atmosphere
  transmittance, 1/16 temporal tracing). The gap is **not a missing lighting technique**; it is shape authoring and detail handling, then ambient
  and sampling.
- Top cause (shape): our noise is *value-noise* lattices (not gradient Perlin) with only 3 octaves of 512/256/128 m cells (`noise.rs:23-24`,
  `settings.rs:267-269`), no curl distortion, the vertical axis tied to the layer thickness (anisotropic, 0.49x squash on Fair), and no profile
  variety beyond one smoothstep pair. Nubis/HZD/Frostbite use Perlin-Worley + Worley octaves + curl, km-scale base cells and per-type height profiles.
- Second cause (detail): shape and erosion are sampled at mip 0 at every distance (`cloud_density.wesl:132,143`) with the erosion at full strength to
  50 km. HDRP fades erosion over 3 km to 100 km and biases the noise mip with distance; Nubis steps and mips scale with distance. Ours aliases into
  sub-pixel noise at range: probable contributor to the "shimmer stays" ledger lines, and it makes distant clouds mush.
- Third (lighting contrast): ambient is flat and unoccluded (`cloud_trace.wesl:652`, one camera-position sky sample, x1.0), and the light march
  reads *uneroded* density (`cloud_trace.wesl:53-84`, `detailed=false`), so there is no self-shadow detail and little form on the shaded side.
  Nubis 2022/2023 scale ambient by local profile and the summed density above; HDRP by a baked AO.
- Fourth (sampling): 1/16 of the screen is traced per frame (`render.rs:821-823`) with history weight 0.06; the trace+resolve cost is only about
  0.22 ms (calc from the 2026-11-06 ledger numbers, ashstorm, 1707x960), so there is budget to buy back crispness.
- Ranked fixes (section 6): noise/profile re-authoring (+0.02-0.05 ms), mip + erosion distance fade (about -0.05 ms), occluded ambient (+0.05-0.1 ms),
  detailed cone light march (+0.1-0.2 ms), 1/8 sampling (+0.2-0.4 ms). Total under 1 ms. Six slices CL-1..CL-6. Owner questions: fly-through, cumulus height, dramatic ambient.

## 2. What we do today (path:line) and comparison

Root: `superfargoth/crates/sf_atmos_bevy/src/shaders/` (`cloud_*.wesl`), CPU: `superfargoth/crates/sf_atmos/src/`.

### 2.1 Shape
- Noise textures: shape 128^3 RG8 (R "Perlin-Worley", G Worley FBM), detail 32^3 R8 (Worley FBM), weather noise 512^2 (`noise.rs:14-20`). Same
  resolutions as HZD (128^3, 32^3, 128^2 curl; read, HZD slides). We have **no curl texture**.
- "Perlin" = `Lattice`: tiling **value noise with smoothstep** (`noise.rs:86-134`), 3 octaves with periods 4, 8, 16 per tile (`SHAPE_OCTAVES`,
  `noise.rs:23`), halving amplitude. Perlin-Worley = `remap(pf, -worley[0].distance, 1, 0, 1)` using only the coarsest Worley (`noise.rs:222`). G is
  `worley_fbm` over the same 4/8/16 periods. Frostbite/Nubis use gradient Perlin (Hillaire's open `TileableVolumeNoise`, read: "Perlin-Worley and
  multiple octaves of Perlin"), HZD packs Perlin-Worley + 3 Worley octaves into RGBA (read).
- World scales (`settings.rs:267-269`): weather tile 8192 m, shape tile 2048 m (cells 512/256/128 m), detail tile 256 m (8 m/texel; octaves 4/8/16 ->
  64/32/16 m cells). The MIT Godot HZD port samples the 128^3 at `p*0.00008` (12.5 km tile) and the 32^3 at `p*0.001` (1 km tile)
  (`godot-volumetric-cloud-demo/clouds.gdshader:` `density()`), i.e. its base cells are ~6x larger than ours and its detail ~4x larger.
- Vertical mapping: `suv.y = h + rise` (`cloud_density.wesl:130-131`): the whole layer height (Fair 1000 m, Storm 8000 m) spans exactly one 128-texel
  tile *vertically* while horizontally one tile is 2048 m. So noise is vertically squashed by `thickness/shape_tile` (Fair 0.49, Cumulus 1.27, Storm
  3.9). Only storm cells (`cb`) switch to world altitude (`cloud_common.wesl:141-146`). A calm deck's puff aspect ratio therefore changes with the
  preset thickness, not with a design.
- Detail: `duv.y = h*b.y/detail_tile` (world altitude, OK), erosion `remap(d, erode*erosion, 1, 0, 1)` with `erode = mix(detail, 1-detail, sat(h*4))`
  (`cloud_common.wesl:122-125`), same as HZD's "invert the Worley at the base" (read, slide 250-251). No curl distortion of the detail sample
  (HZD distorts it with 2D curl "to fake swirly motion", read). Detail applies at full strength at any distance (`far = 50 km`, `settings.rs:276`).
- Density: `d = remap(R, G-1, 1, 0, 1) * height_gradient(h, kind)`, then `remap(d, 1-cov, 1, 0, 1) * cov` (`cloud_common.wesl:116-120`): the same
  Schneider coverage remap, correct. Missing vs the Godot HZD port: the final bottom-heavy shaping `pow(d, (1-h)*0.8 + 0.5)`.
- Vertical profile (`cloud_common.wesl:110-114`): `ramp(0, mix(0.07,0.22,kind), h) * ramp(1, mix(0.5,0.85,kind), h)`. HZD gradients (Godot port,
  4 knots in layer fraction): stratus (0.02, 0.05, 0.09, 0.11) = a layer only 11 % tall; stratocumulus (0.02, 0.2, 0.48, 0.625); cumulus
  (0.01, 0.0625, 0.78, 1.0). Ours at kind 0 (stratus) runs from 0.07 to a top ramp over h 0.5..1.0, i.e. half the layer: it is never stratus-thin
  (the preset thickness 650-900 m stands in for it).
- Weather/coverage: coverage field is 4-octave value noise (periods 4..32 per 8192 m, `noise.rs:25`), thresholded by `ramp(thr-0.2, thr+0.2, noise)`
  with `thr = 1.25 - 1.5*coverage` (`cloud_common.wesl:197-200`); a second octave drifts (`evolve_field`). Plus the world weather map (`weather_map.rs`).
  No cloud streets, cells or Worley layout; a coverage blob is a thresholded smooth value-noise lump.
- Layers: shell `base 1200 m, thickness 1000 m` (Fair), Cumulus 2600 m, Storm 8000 m (`settings.rs:103-167`). HZD: layer 1500-4000 m (Godot port,
  `sky_b_radius 6001500`, `sky_t_radius 6004000`), Nubis 2017 shows clouds 1.5 km to 8 km tall (read, slide 53-56).

### 2.2 Lighting
- Phase (`cloud_common.wesl:217-226`): `mix(HG(g), HG(-0.5 g), 0.35)`, g = `phase_g` 0.6 (`settings.rs:263`). Back lobe is therefore g = -0.3.
- Multiple scattering (`:230-242`): Wrenninge octaves, `ms_octaves 3`, falloff 0.5 for attenuation, contribution and eccentricity (a=b=c=0.5,
  Hillaire 2016 eq. 19-20, read). Each octave multiplies the *dual-lobe* phase by `eccentricity` (so the back lobe also narrows).
- Powder (`:245-248`): `1-exp(-4 d)` (local density), strength 0.7, weight `0.5-0.5 cos` (strong looking away from the sun). Same family as HDRP
  `PowderEffect` (`1-exp(-4d)`, `saturate(x*2)`, `smoothstep(0.5,-0.5,cos)`; HDRP `VolumetricCloudsUtilities.hlsl:183-187`).
- Light march (`cloud_trace.wesl:53-84`): `light_steps` 5 (High), first sample at half a step (100 m for a 1 km layer), step grows x1.5 (reach about
  2.6 km), **straight line** along `l`, **uneroded** density (`detailed=false`), weather re-sampled per step. HZD: 6 samples **in a cone** with
  random offsets plus one far sample 18 steps out, "sample the cheap version once alpha reaches 0.3" (read, slides 83-85; Godot port `march()`).
  HDRP: `_NumLightSteps` 6, step at most 1000 m, first at 0.25 step, full erosion in the light march (`EvaluateCloudProperties(..., true, true)`),
  `VolumetricCloudsUtilities.hlsl:391-433`. Hillaire 2016: 4 progressive shadow samples (read, 5.5.2).
- Extinction: `0.015 /m * density(<=1.15)` (`settings.rs:259`); HDRP `sigmaT` 0.04..0.12 (`:332-335`). Not an outlier.
- Ambient (`cloud_trace.wesl:652`): `mix(bounce, sky_up, saturate(h)) * ambient(1.0) * (1 - 0.6 wetness)`; `sky_up` is **one** sky-view LUT read per
  ray at the camera, straight up (`:572`); `bounce` is ground albedo x sun irradiance. **No occlusion** by cloud above and no desaturation.
  Hillaire 2016: ambient from SH probe, linear bottom-to-top gradient, "ambient lighting occlusion is not taken into account ... luminance can often
  be too bright", so artists get a scale and a desaturate control (read, 5.5.1). Nubis 2022: `ambient = pow(1 - dimensional_profile, 0.5)` (vertical
  profile) and for the envelope model `pow(1 - sat(coarse_density), 0.25) * height_fraction`; Nubis3: `... * exp(-summed_ambient_density)` (read,
  nubis2022/nubis2023 text). HDRP: SH probe top/bottom lerp by height times a baked/eroded **ambient occlusion** (`:626-631`, `:331-347`).
- Atmosphere: sun x transmittance LUT at the sample radius x planet shadow (`:643-644`), aerial perspective at the weighted mean cloud depth
  (`:681-686`): same as Hillaire 2020 / UE5 (read, Hillaire 2016 5.9 for the mean-depth aerial perspective).

### 2.3 Sampling
- Half-res + checkerboard: trace target = `ceil(half/2)` per axis, i.e. 1/4 of full-res per axis, **1/16 of the pixels per frame** (`render.rs:821-823`,
  `CloudQuality::HIGH checkerboard: true`, `settings.rs:334`). Same as HZD "1 out of 16 pixels of each 4x4 block" (read, slides 91-92). Nubis 2022 PS5:
  light/view ray samples at 1920x1080 max resolution with 96-180 view samples (read table).
- Steps: `min 32 / max 64` primary, step `0.05 * thickness + g*t`, coarse (2x) skipping with a 4-empty-sample return (`cloud_trace.wesl:589-624`).
  HZD 64-128 potential samples, step back on hit (read). Nubis 2022: `step = 3 + 60 * dist / 16384` m, i.e. 3 m near (read).
- Temporal: fresh weight 0.06 (about 16 samples, about 64 frames under the checkerboard), variance clip 1.5 sigma, STBN jitter (spec `clouds.md`
  "Temporal accumulation as built"). Cost measured 2026-11-06: trace 0.146 ms + resolve 0.075 ms at 1707x960 ashstorm.
- Upsample: depth-aware composite at full res, DLSS afterwards; Nubis 2022 blurs 1-2 px after the trace (read table "Blur Scale").

### 2.4 Table: ours vs Nubis vs Hillaire / UE5

| Component | Ours (path:line) | Nubis / HZD (read unless noted) | Hillaire Frostbite / UE5 |
|---|---|---|---|
| Base noise | value-noise 3 oct (4/8/16 per 2048 m) + 1 Worley, `noise.rs:211-230` | gradient Perlin-Worley + 3 Worley octaves (RGBA 128^3), 128^2 curl | Perlin-Worley + Perlin octaves; single-channel baked (Hillaire 2016 5.4) |
| Detail noise | Worley 3 oct, 32^3, 256 m tile, full strength at all distances | 32^3 Worley, curl-distorted; erosion fades with distance (HDRP 3-100 km) | Worley octaves, Frostbite single channel; UE5 material Noise1/2/3 (docs) |
| World scale | shape 2048 m, detail 256 m, weather 8192 m | Godot port: 12.5 km / 1 km; HZD weather map 32 km (Nubis 2017: "square kilometers") | UE5 material-defined |
| Vertical axis | h (0..1) = 1 tile, squash = thickness/2048 (`cloud_density.wesl:130`) | world-space; Nubis Evolved model per genus | UE5: world altitude texture/height-profile per type |
| Profile | 2 smoothsteps, kind lerp (`cloud_common.wesl:110`) | 4-knot per-type (stratus top at 0.11) | UE5 `Layout_CloudHeightProfile` per channel (docs); Frostbite cloud-type texture |
| Remap | Schneider coverage remap (`:116`) + erosion remap (`:122`) | same | same "similar to [Sch15]" |
| Phase | HG 0.6 + 0.35 HG -0.3 | HG forward + silver lining (g about 0.8, unverified numbers) | dual lobe `lerp(HG g0, HG g1, w)` (eq. 18); HDRP 0.7/-0.7 sum |
| Multi-scatter | Wrenninge 3 oct a=b=c=0.5 | Nubis 2022: `MS * secondary phase`, MS from profile x depth x height powers | Wrenninge octaves, N=2-3 (Hillaire 2016 5.8); UE5 material MS contribution/occlusion/eccentricity (docs) |
| Powder | `1-exp(-4d)`, 0.7, view-dependent | Beer-powder, view-dependent (slide 66) | HDRP same |
| Ambient | flat, one sky sample x1.0, bounce lerp | profile and density-above scaled (`pow(1-profile,0.5)`, `exp(-sum_amb)`) | SH probe x height gradient, no occlusion + artist scale/desaturate; UE5 cloud AO output |
| Light march | 5 straight steps x1.5, uneroded | 6 cone + far sample, cheap LOD at alpha 0.3 | 4 progressive steps; HDRP 6 steps, eroded |
| View samples | 32-64 | 64-128 (HZD), 96-180 (Nubis PS5), 3 m near step | UE5 view sample count scale (docs, default not stated) |
| Resolution | 1/16 px per frame, half-res stored | 1/16 per frame (HZD); 1080p max (PS5) | Frostbite EMA reprojection; UE5 1/2 or 1/4 res + TAA (unverified) |
| Temporal | 0.06 weight, variance clip | reproject + neighbourhood clamp | EMA, constant blend |

## 3. Bevy 0.20 native and crates / open source read

- `bevy_pbr-0.20.0` (`C:\Users\leogloglo\.cargo\registry\src\...\bevy_pbr-0.20.0\src\atmosphere\mod.rs`): atmosphere (LUTs) only. The only
  "cloud" mentions are the shadow-cookie notes at `mod.rs:428,433`. No volumetric clouds in Bevy 0.20 (verified by grep over `bevy_pbr`).
  Bevy's volumetric fog is a froxel fog, not a cloud tracer.
- `evroon/bevy-volumetric-clouds` (MIT), commit `8245ed2b6786783154193eb7902e1158b85d5b23`: Bevy 0.18 HZD port, compute shader
  (`src/shaders/clouds_compute.wgsl`): 12 raymarch steps default, 6 shadow steps, forward/backward HG lerp, bottom 1250 m top 2400 m. Cheap and
  simple; below our implementation in every axis; nothing to adopt (reuse check: our own passes are further along).
- `clayjohn/godot-volumetric-cloud-demo` (MIT), commit `6f5ecab924fa1493d5e9eeea6dc7ae5b684eb184`, `clouds.gdshader` (read): the most concrete HZD constants
  (gradients, noise scales, cone vectors, `max(beers, beers2*0.7)` light term, `pow(d, (1-h)*0.8+0.5)`, phase = max of three HG 0.6 / -0.2 /
  `0.4-1.4*sun.y`, 96..54 steps, 6 cone light samples + a far sample, mip bias per light sample).
- Unity Graphics HDRP volumetric clouds (Unity Companion Licence: technique only, nothing copied), commit `09e812d2cbd497aa507fe67c07319cd9e384388c`,
  `Packages/com.unity.render-pipelines.high-definition/Runtime/Lighting/VolumetricClouds/`: `VolumetricCloudsUtilities.hlsl` (phase 0.7/-0.7 per octave,
  multi-scatter octaves, PowderEffect, SH ambient x AO, erosion distance 3 km to 100 km, light step max 1000 m, 6 light / 64 primary defaults in
  `VolumetricClouds.cs:182,188`), `VolumetricCloudsTrace.compute`, denoise/accumulation files.
- Hillaire's open-source noise generator `sebh/TileableVolumeNoise` (cited by Nubis 2017 and Hillaire 2016 appendix D; not cloned, URL only): the
  reference for the Perlin-Worley construction; our `noise.rs` should be compared to it (licence not checked: read before porting, we can write our own).
- No crate adoptable for the look. Ours stays our own (wrong fit: all are single-pass sky domes; ours is composited in the frame graph with fog split).

## 4. Industry: canonical technique per component

| Component | Canonical technique | Published constants |
|---|---|---|
| Shape | Weather map (coverage, type) x vertical gradient x low-frequency Perlin-Worley remapped by Worley FBM (`remap(pw, wfbm-1, 1, 0, 1)`), erosion by Worley (inverted at the base) with curl distortion; Nubis Evolved: 2D "NDF" maps + vertical profile (ground view), envelope SDF model (fly-through), Nubis3 voxels + SDF + noise up-rez | 128^3 RGBA + 32^3 + 128^2 curl (HZD); 6 cone/long samples; weather map 2D (HZD) |
| Density | coverage remap `remap(d*grad, 1-cov, 1, 0, 1)*cov`; "noise as base probability" and erosion as subtraction (Nubis 2017) | extinction typically 0.04-0.12 (HDRP) |
| Lighting | Beer-Lambert primary + Wrenninge multi-octave multiple scattering (a,b,c <= 1, a <= b) + dual-lobe HG; powder view-dependent; ambient from sky probe x profile gradient (and occlusion in Nubis 2022/3, HDRP AO); Jendersie & d'Eon 2023 HG+Draine as the better single phase | Wrenninge a=b=c=0.5, N=2-3; HDRP eccentricities 0.7 / -0.7; Nubis 2022 MS from `pow(att,Pdepth) * pow(h,Pheight)` |
| Sampling | quarter-res spatial/16-frame temporal, adaptive steps (3 m near to +60 m per 16 km, Nubis 2022), cheap-density skip + step back, cone light with cheap LOD | HZD 64-128 view samples, 6 light; Nubis PS4/PS5 60-90 / 96-180 view samples, 2-4 ms (PS4 <= 4 ms, PS5 2-3 ms) |
| Atmosphere coupling | cloud mean-depth aerial perspective; fog column split at cloud depth | Hillaire 2016 5.9 (we already do this, `FogCloudSplit`) |
| Jendersie & d'Eon 2023 | `phi_fog = (1-w_D) HG(g_HG) + w_D Draine(alpha, g_D)`, parameters fitted per droplet diameter 5-50 um | example w_D 0.25, alpha inf, g 0 (supplemental); exact fits: read the PDF before implementing (not read here) |

## 5. Why ours looks different, ranked by visual impact

1. **Noise character and scale (shape).** Smooth value noise at 128-512 m cells gives rounded featureless lumps; the missing Perlin gradient/Worley
   cauliflower structure, curl wisps, per-type profiles and km-scale cells make clouds read as small "goofy" blobs (owner WE-15 "still look goofy").
   The vertical squash (2.1) changes puff aspect with thickness. Highest impact, cheap to fix, owner-visible.
2. **Detail at full strength at all distances with mip 0.** Aliased sub-pixel erosion far away = mush/shimmer, and sharp-vs-soft inconsistency near
   vs far. Calc: detail texel = 8 m; at 20 km a 1707x960 pixel footprint is about 20000*tan(fov/1707) ~ 20 m, so 2-3 detail texels per pixel
   (minification without mips) beyond about 10 km. Probable link to the standing shimmer lines in the ledger (2026-11-06, 2026-11-12). Also check
   the shape volume: 16 m texels at 2048 m tile, similar.
3. **Ambient flat and unoccluded + uneroded light march.** No dark pockets inside, no light/dark crevices on the shaded side, no bright rims:
   the "lit but shapeless" look. Industry dims ambient by local profile/density above (Nubis 2022: `pow(1-profile,0.5)`).
4. **Forward scattering weak (g 0.6, back lobe -0.3 with 0.35 weight).** Silver lining/sun-side glow is low next to HDRP 0.7/-0.7 sum, Nubis. A
   cheap taste control; owner prefers dramatic.
5. **Sampling: 1/16 pixels, 0.06 history.** Soft, slow to converge after motion and under DLSS, which also reprojects. Not the cause of the
   gap in character, but it limits perceived detail (HZD accepted this; it cost them 2 ms, ours 0.22).
6. **Profile/authoring**: thin decks (stratus 11 % tall in HZD) and tall cumulus (2.5 km) are not authored; Fair is 1 km thick.

## 6. Ranked fix list

GPU ms: estimates (calc/industry-based), RTX 4060 laptop, High, 1440p + DLSS Quality (clouds traced at 1707x960 input). Current trace+resolve about
0.22 ms (measured 2026-11-06 in one scene). Frame target <= 8.3 ms median / 11 ms p99. Verify each with `perf ab`.

| # | Fix | Expected cost | Bake | Spec / slice | Risk |
|---|---|---|---|---|---|
| 1 | Re-author noise: gradient Perlin (Hillaire's TileableVolumeNoise construction) + Perlin-Worley + 3 Worley octaves in RGBA 128^3 (R PW, GBA Worley 1x/2x/4x), 128^2 curl texture, base cells km-scale, world-altitude vertical axis for all decks (generalise `shape_height`), per-genus profiles (HZD 4-knot) | +0.02-0.05 ms (curl fetch, 4 channels) | `atmos.noise` step: seconds (shape was about 1 s; Worley 3 oct maybe 3-5 s), deterministic, cached | `clouds.md` "Model", `noise.rs`, `cloud_common.wesl`, `settings.rs`; slices CL-1, CL-2 | Look changes everywhere: owner sign-off scene; CPU twins (`sf_atmos::density`) must follow |
| 2 | Mip LOD for shape/detail by distance (HDRP noise mip offset; Nubis "noise texture mip level") + erosion fade 3 km -> 100 km | -0.03 to -0.1 ms (fewer cache misses) | none | `cloud_density.wesl:132,143` (`textureSampleLevel` mip arg), `clouds.md`; CL-3 | Distant clouds smoother (intended). Must not change the cloud shadow map (`cloud_shadow.wesl`) |
| 3 | Occluded ambient: `ambient = sky(gradient top/bottom) * pow(1-profile, 0.5) * exp(-od_up)` with `od_up` from 1-2 un-eroded vertical samples (reuse the first light samples when sun is high); scale + desaturate tunables (Hillaire 5.5.1) | +0.05-0.1 ms | none | `cloud_trace.wesl:652`, `clouds.md`; CL-4 | Darker underside: owner prefers lit look (see 8); keep a floor tunable |
| 4 | Light march: erosion on the first 2 samples, STBN cone offsets (HZD), one far sample at 3-6 km for cloud-on-cloud shadow; keep cheap LOD at T < 0.3 (we have `full_light`) | +0.1-0.2 ms | none | `cloud_trace.wesl:53-84`; CL-4 | More noise before the resolve; jitter must stay STBN to keep H-lines (WE-10) |
| 5 | Phase: g 0.8 forward (silver lining) + back lobe, weight tunables; optional HG+Draine (Jendersie & d'Eon) | about 0 ms | none | `cloud_common.wesl:217-226`, `settings.rs`; CL-5 | Pure taste; A/B in the owner scene |
| 6 | Sampling: 1/8 (2x1 checker) or reuse 1/16 with weight 0.1 + sharper reprojection; step cost per Nubis (3 m near is unaffordable; keep adaptive) | +0.2-0.4 ms for 1/8 | none | `render.rs:821`, `clouds.md` "Temporal"; CL-6 | Touches the shimmer-sensitive resolve (S1 silhouette history); do after 1-5 |
| 7 | Authoring: stratus 11 %-tall profile, cumulus layer 1.5-4 km (HZD), cloud streets / cell layout in the weather noise; Morrowind data mapping in `mw_weather` (adapter, not framework) | 0 ms | none | `settings.rs` presets, `noise.rs` weather; CL-7 | Taste + H-world scale (Vvardenfell x10) |
| later | Nubis Evolved envelope/voxel (Nubis3 cost 2.2-4 ms, 10 ms density sampler at 960x540 for the authored-voxel path) | too costly now | Houdini-grade authoring | fly-through only | do not start without owner go (hard problem class) |

## 7. Proposed build slices (one deliverable + its test each)

All shape/lighting constants become tunables (`tunables.md`: unit, range, source; `Guess` counted). Framework code in `sf_atmos*` stays game-agnostic;
Morrowind mapping (type/coverage curves) goes through the existing `mw_weather` adapter. Rendering slices end with a registered scene in
`bevyrrowind/docs/visual-checks/` (awaiting owner); no screenshot judging by agents.

| Slice | Deliverable | Test | Tunables (unit, range, source) |
|---|---|---|---|
| CL-1 noise v2 (CPU) | `noise.rs`: gradient-Perlin FBM, PW + 3 Worley RGBA 128^3, curl 128^2; world-scale tiles | exact: deterministic by seed; tiling seam (opposite edges equal); channel stats (mean, range); remap bounds | `shape_tile` (m, 4096-16384, Godot HZD port 12 500), `detail_tile` (m, 512-2048, 1000), curl strength (m, 0-200, Guess) |
| CL-2 shape sampling | shader/CPU twin: world-altitude vertical axis for all decks, curl-distorted detail, per-genus 4-knot profile | CPU-twin parity test (extend `sf_atmos::density` tests: calm deck bit-identical where `cb`=0 is no longer promised, new golden profile tests) | profile knots per genus (fraction, 0-1, HZD) |
| CL-3 mip + erosion fade | `textureSampleLevel` mip by footprint; erosion strength fade | CPU twin `erosion_fade(d)` monotone, 1 at <= near, 0 at >= far; mip function bounds | `erosion_fade_near` (m, 1000-10000, HDRP 3000), `erosion_fade_far` (m, 20000-150000, HDRP 100000), `noise_mip_bias` (levels, -1..3, Guess) |
| CL-4 ambient + light march | occluded ambient, eroded first samples, far sample, STBN cone | CPU twin: ambient monotone in `od_up` (0 -> floor, large -> floor); light OD >= uneroded OD at equal point | `ambient_scale` (x, 0-1.5, Hillaire 5.5.1), `ambient_desaturate` (0-1, Hillaire), `ambient_floor` (x, 0-0.3, Guess), `light_far_sample_m` (m, 1000-6000, HZD 18 steps) |
| CL-5 phase | forward/back g, weight, optional Draine mix | CPU twin: phase integrates to 1 (energy), forward peak > back | `phase_g_fwd` (0.5-0.9, HDRP 0.7), `phase_g_back` (-0.8..-0.1, HDRP -0.7), `phase_back_weight` (0-0.5, Hillaire w) |
| CL-6 sampling density | 1/8 checker option + resolve weight tunable | `perf ab` ms per preset (no pass/fail), CPU twin of accumulation unchanged | `trace_div` (px, 2-4, HZD 4), `history_weight` (0.04-0.15, S1 0.06) |
| CL-7 authoring + Morrowind map | presets thickness/base/types, weather noise with cells; `mw_weather` mapping | exact: preset values; `mw_weather` mapping tests | preset fields (existing) |

H3 (draw order of sky/clouds/fog/precip): none of CL-1..7 changes pass order, depth, or the fog split; CL-6 touches the resolve inputs the composite
reads for occlusion (`dist` channels), so it must keep S1's depth-window semantics. Link only; belongs to the owner's own agent.

## 8. Owner questions (taste/scope only), recommended answers

1. **Fly through or inside clouds?** (e.g. levitation, tall peaks, ships in storms). Determines whether the envelope/voxel path is ever needed.
   Recommended: no (clouds are a sky and a storm wall; mountains poke through the deck as in the 2026-11-12 ledger).
2. **Cumulus height and drama:** keep Morrowind's low flat decks (1 km) or allow 2-4 km towering cumulus in fair weather as HZD? Recommended:
   towering for Fair/Cumulus presets, flat for Overcast/Ash (more dramatic, costs 0 ms).
3. **Underside and interior contrast:** lit/airy (current) or darker, higher-contrast bases and crevices (Nubis look)? Recommended: Nubis contrast
   with `ambient_floor` so towns stay lit (owner "lit look over correctness" memory).
4. **Silver lining strength** (sun-side rim): subtle (g 0.6, now) or strong (HDRP 0.7/-0.7)? Recommended: strong, with the owner scene A/B.

## 9. Sources

Read (text extracted this session):
- Schneider & Vos, SIGGRAPH 2015, "The Real-time Volumetric Cloudscapes of Horizon Zero Dawn" (advances.realtimerendering.com/s2015): noise 128^3/32^3/128^2 curl, 64-128 steps, 6 cone light samples + far, powder, 1/16 update. Link: https://www.advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf
- Schneider, 2017, "Nubis: Authoring Realtime Volumetric Cloudscapes with the Decima Engine": https://advances.realtimerendering.com/s2017/Nubis%20-%20Authoring%20Realtime%20Volumetric%20Cloudscapes%20with%20the%20Decima%20Engine%20-%20Final%20.pdf
- Schneider, 2022, "Nubis Evolved": https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf (step formula, ambient and MS formulas, PS4/PS5 table)
- Schneider, 2023, "Nubis Cubed": https://advances.realtimerendering.com/s2023/Nubis%20Cubed%20(Advances%202023).pdf (ambient `exp(-summed_ambient_density)`, 2.2-4 ms ray-march, 10 ms density sampler)
- Hillaire, 2016, "Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite": https://media.contentapi.ea.com/content/dam/eacom/frostbite/files/s2016-pbs-frostbite-sky-clouds-new.pdf (5.4 noise, 5.5.1 ambient, 5.5.2-3 shadows and temporal, 5.7 dual lobe, 5.8 Wrenninge)
- Toft, Bowles, Zimmermann, "Optimisations for Real-Time Volumetric Cloudscapes", arXiv 1609.05344 (downloaded, not mined).
- Source read: `godot-volumetric-cloud-demo` (MIT) @ `6f5ecab9...`; `bevy-volumetric-clouds` (MIT) @ `8245ed2b...`; Unity Graphics HDRP volumetric clouds @ `09e812d2...` (sparse checkout of that folder).
- Local clones: `C:\Users\leogloglo\AppData\Local\Temp\research-1\clouds\{godot-volumetric-cloud-demo,bevy-volumetric-clouds,Graphics}`.

Not read, or docs gave no numbers (unverified):
- Hillaire 2020 EGSR (atmosphere only; abstract seen via search): https://diglib.eg.org/handle/10.1111/cgf14050 ; UE5 Volumetric Cloud component and material docs (fetched; no defaults listed): https://dev.epicgames.com/documentation/en-us/unreal-engine/volumetric-cloud-component-in-unreal-engine . UE5 specifics in the table (height profile channels, MS outputs) come from those pages; defaults and "1/2 or 1/4 res" are recalled (unverified).
- Jendersie & d'Eon 2023: https://research.nvidia.com/labs/rtr/approximate-mie (structure seen in search snippets; fitted parameter formulas not read).
- Wrenninge et al., "Oz: The Great and Volumetric" (via Hillaire 2016 eq. 19-20 and HDRP code only); RDR2 Bauer 2019 (not re-read; wiki `docs/wiki/industry/clouds-and-weather.md` marks it unverified); HZD "g about 0.8 forward" (recalled, unverified).
- `sebh/TileableVolumeNoise` (referenced, not cloned).

Agreement with older docs (skimmed after the findings, not read in full): none read; `superfargoth/docs/research/clouds-upsampling.md` is about sampling only.
