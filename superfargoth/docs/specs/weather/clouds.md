## Clouds

**Model** (Nubis 2017, Megalophobia math). The weather map (R coverage, G type, B wetness/darkness,
A anvil) is a 2D texture that weather writes (below). The shape volume is 128³ RG8 (Perlin-Worley,
Worley FBM) and the detail volume 32³ R8 (Worley FBM). Both tile and come from the `atmos.noise` bake
step, which is deterministic and cached. Type height profiles follow Nubis/Megalophobia
`combine_density`/`erode_density`.

**Geometry.** A spherical shell around the planet center that `Atmosphere`'s transform defines
(`inner_radius` + base/top from `CloudLayers`). Rays intersect it analytically. Morrowind's flat world
under a curved shell gives the correct horizon falloff, as in HZD. Up to two layers (low cumulus/stratus
deck, high cirrus as a 2D textured layer at 6-8 km: one fetch, as in Nubis/UE).

**Coordinates.** Shaders see origin-relative positions (big_space). Noise is sampled at
`rel + WorldOrigin.phase_16384`. Tile sizes divide 16 384 m (weather 16 384 m, shape 4 096 m, detail
256 m). Wind scroll offsets are wrapped **per texture in its own tile** on the CPU in f64. That is the
Megalophobia rule; one shared offset makes coverage jump.

**Driving from weather.** The weather map is not a static texture. A small compute pass synthesizes it
each time weather changes enough (or every N frames): base coverage noise (2 octaves, advected by
`WindState` at cloud altitude) thresholded by `clouds.coverage`, type from `clouds.type`, plus storm
cells (a tiling cellular pattern whose density follows precipitation intensity). Transitions are then
smooth, because only scalars change, and clouds visibly build up before rain. Storm cells can also be
anchored by the adapter (an ash plume over a volcano: a coverage boost around a world position).

**Rendering.** An `sf_atmos_bevy` render-graph node after Bevy's `render_sky`:
1. Raymarch at **quarter resolution** (half each axis), updating one pixel of each 2×2 block per frame
   (Ultra: every pixel at half res) with per-frame blue-noise jitter. 48-96 primary steps, adaptive:
   big steps in empty space, small inside cloud. Early out at transmittance < 0.01. 5-6 light samples
   with growing steps (cone-like), 3 multiple-scattering octaves.
2. Lighting: sun radiance × atmosphere **transmittance LUT** at the sample's altitude (sunset clouds
   turn red for free), ambient from the **sky-view LUT** (upper hemisphere) and a ground-bounce term
   (ground albedo × sun irradiance), Beer-powder, two-lobe HG. The moon works the same way through the
   second directional light. Lightning adds emission inside the strike's cloud.
3. Aerial perspective between camera and cloud: aerial-view LUT up to its range (32 km), transmittance
   LUT segment beyond. Clouds then fade into the same haze as the far field behind them.
4. Depth: the march stops at the scene depth from the **main depth texture** (bound like Bevy's
   `render_sky` does; no prepass). Mountains and far-field proxies cut into low cloud correctly.
5. Temporal reprojection (cloud motion + camera reprojection by the cloud's depth-weighted distance,
   neighborhood clamp), then a depth-aware upsample to full resolution. Composite: `color × T + L`.

The far fog raymarch (below) runs in the same node and resolution, so the two share reprojection.

**Cloud shadows.** A compute pass writes a **512² R8 cloud shadow map** covering 16 km around the
camera, projected along the sun direction (optical depth through the layer, 8 samples, low detail).
It updates 1/4 of the texels per frame. Three consumers:
- every PBR material, through `DirectionalLightTexture` on the sun (and moon). The light's
  `Transform` is re-centered on the camera each frame and scaled to the coverage, and `tiled: false`;
- froxel fog and far fog (shafts under cloud gaps: the RDR2 look);
- the surface model (rain under the clouds, not under a clear hole).

Past its coverage it reads as "average cover" (the weather map's mean), never black. That is
Megalophobia's out-of-range-is-lit rule, generalized.

**Interaction with the far field.** The far field is opaque geometry in the depth buffer, so clouds,
cloud shadows and aerial perspective need nothing special from it. Proxies receive cloud shadows
through `DirectionalLightTexture` (`StandardMaterial`) or the shared WESL (impostors). Clouds cast no
CSM shadows (`NotShadowCaster`). Cloud shadows come only from the shadow map.

**Solari path:** the same cloud pass composites after Solari's output. Cloud shadows reach ray-traced
lighting only through the sun's `DirectionalLightTexture`, if Solari honors it (unverified; open
question 5).

### Clouds as built (clouds session, 2026-10-12)

**Crates.** `sf_atmos` (Bevy-free, 10 tests): `CloudSettings` = `CloudWeather` (coverage, type,
density, wetness, storm, erosion, base, thickness, wind: what weather drives, `lerp` for transitions)
+ `CloudLook` (extinction, fill, phase, tiles, reach, shadow strength) + `CloudQuality` (steps,
octaves, checkerboard, shadow map); presets `CloudPreset::{Clear, Fair, Overcast, Storm}`; the
Megalophobia noise generators (128³ RG8 shape, 32³ R8 detail) plus a 512² weather-noise map; the
density/weather functions as the CPU twin of the WESL; `CloudScroll` (wind travel wrapped at 16 384 m,
per-texture offsets that include the origin phase). `sf_atmos_bevy` (thin Rust, WESL): `CloudsPlugin`,
`Clouds` (resource the weather writes), `CloudCamera`, `CloudShadowLight`, `CloudsOrigin`.

**Deviations from the plan above (and why).**
- *Weather map:* no compute pass writes one. A fixed tiling weather-noise texture (R coverage field,
  G type field, B storm cells) is thresholded per sample by the `CloudWeather` scalars
  (`sf_atmos::weather_at`, same math in WESL). Same behavior (only scalars change, clouds grow where
  the field is high, storm cells add cover and wetness), one fetch, no pass. A written map comes back
  when adapter anchors (Red Mountain plume, open question 3) or painted climates need it.
- *Noise* is generated at startup on a background thread (226 ms, clouds appear when ready), not by
  an `atmos.noise` bake step yet (next step: cache it through `sf_bake`).
- *Composite order:* Bevy's `render_sky` is private, so the composite is ordered after every system
  that runs before `main_transparent_pass_3d` (found in the Core3d graph at `finish`): named scaffold
  `SkyOrderScaffold` in `render.rs`, removal path a public sky set upstream.
- *Atmosphere bindings:* group 0 replicates `render_sky`'s bind group from bevy_pbr's public resources
  (LUTs, uniforms, depth), so the cloud WESL imports `bevy_pbr::atmosphere::functions` unchanged.

**Passes (per `CloudCamera`, raster path).** Shadow map before the main pass; after the opaque pass:
trace (one pixel of each 2×2 half-res block per frame, 32-80 adaptive steps by segment length, IGN
jitter per frame, 5 growing light steps, 3 octaves, Beer-powder, two-lobe HG, sun or moon (brightest
light) through the transmittance LUT at the sample, sky fill from the sky-view LUT, ground bounce,
aerial perspective from the aerial-view LUT and beyond it the sky's in-scatter in proportion to the
transmittance reached), resolve (reprojection by transmittance-weighted cloud distance, minus the
wind step, plus the origin shift; 3×3 neighborhood clamp on this frame's samples; fresh pixels blend
50 %), composite (depth-aware bilinear: a tap whose cloud lies behind the pixel's geometry is dropped;
blend `One, SrcAlpha`). The march ends at the main depth buffer and at the ground sphere, so nothing
draws under the horizon or on the sea. MSAA layouts exist but only `Msaa::Off` was run.

**Cloud shadows.** 512² (1024² since S1, below) R8 render target, 16 km square in the sun's light plane centered on the
camera (snapped to the 32 m texel grid), 8 samples through the shell per texel, edge fading to the
mean cover; given to the sun as a **tiled** `DirectionalLightTexture` (Bevy reads 0 = black outside
an untiled texture, which breaks the out-of-range rule). The light's `GlobalTransform` gets the
scale/translation after propagation (Bevy builds cascades from its rotation only). Since S1
(clouds3) the map is toroidal and time-sliced, 6 samples per texel: see "S1 cost" below. Moon: none yet.

**Floating origin.** The app writes `CloudsOrigin` (absolute position of the origin cell) in its
origin sync; noise offsets carry `origin mod tile` (tiles divide 16 384 m), and the resolve shifts
the history by the origin delta, so neither clouds nor their history jump on shifts.

**App (Bevyrrowind).** On by default in exteriors with the sky; `--off clouds`; `--clouds
clear|fair|cumulus|overcast|storm`; viewer keys F10 (on/off) and C (cycle presets). Heights halved for
the compressed world (`CLOUD_HEIGHT_SCALE` 0.5: fair base 750 m, top 1,450 m), shape tile 2,048 m,
detail 128 m. Visual checks CL-1..CL-6 (awaiting owner).

**Measured** (release, 2560×1440 native, RTX 4060 Laptop on AC, Red Mountain camera at 16:30,
2 × 12 s each, back to back; `shots/clouds/ab.sh`, results `shots/clouds/perf/`): frame median
`--off clouds` 8.97 / 8.98 ms; clear 9.59 / 9.60 (+0.62); fair 9.98 / 9.91 (+0.97); overcast
9.90 / 9.89 (+0.92); storm 9.86 / 9.84 (+0.88). Against the High budget (clouds + far fog 0.9 ms,
shadow and weather maps 0.05 ms) the clouds alone sit at the budget with no far fog yet. The fixed
part (+0.6 ms with almost no cloud) is the next target: amortize the shadow map, skip empty-space
samples by weather coverage before the shape fetch, and a cheaper composite. No `sf_diag` GPU
counter yet (`atmos/clouds/gpu_ms`).

**Not built yet.** High cirrus layer, far fog in the same node, lightning emission, quality presets
(Low 1/16 res), Solari path (open question 5), MSAA run, `atmos.noise` bake step, GPU timer.

### Cloud genera, scale, motion, shadows, sun occlusion (S1, clouds2 session, 2026-10-13)

Owner feedback (CL-1, CL-3 FAILED): two overcasts + one pattern + clear; one cloud covers the
island; nothing moves; no shadow edge on the ground; the sun seems to draw over the clouds.

**What was wrong.**
1. *Shadows:* Bevy 0.20 defines `LIGHT_TEXTURES` only when clustered decals are usable
   (`bevy_pbr` mesh.rs), which needs the `pbr_clustered_decals` cargo feature. We enabled only
   `pbr_light_textures`, so the sun's `DirectionalLightTexture` was extracted but **never
   sampled**: no cloud shadows at all, anywhere. Fix: `sf_atmos_bevy` enables both features.
   Also: the map now uses the detailed (eroded) density, full extinction (the cookie scales
   only the sun's direct light; sky fill stays, so `shadow_strength` 0.9 is physical), a
   bilinear repeating sampler, and 1024² over 16 km (16 m texels).
2. *Scale:* the weather tile was 16 384 m with its coarsest coverage octave at a quarter
   tile: cloud-field blobs of ~4 km over an island ~5 km across, and the app scaled only the
   heights. Real cumulus are 0.5-2 km wide (humilis/mediocris), congestus towers 1-3 km,
   cumulonimbus cells 5-10 km, cirrus at 6-12 km (WMO International Cloud Atlas). Now the real
   scale defaults are weather 8 192 m (2 km blobs), shape 2 048 m, detail 256 m, cirrus
   16 384 m, storm cells a third of the weather tile; and a compressed world scales **sizes
   and heights together** (`CloudLook::scaled` + `CloudWeather::scaled_heights`, powers of
   two so tiles still divide 16 384 m). Bevyrrowind uses 0.5: fair cumulus a few hundred
   meters wide, base 600 m.
3. *Motion:* 9 m/s is a real-world speed; beside a 30x game clock it reads frozen. The
   framework keeps physical preset winds; the adapter sets the speed (Bevyrrowind:
   Morrowind's per-weather `Cloud Speed` x 10 m/s, cloudy 20 m/s). Shapes evolve
   (Schneider 2017's advected weather map, done without a pass): the cloud field is two
   octaves of the weather map (the second transposed) drifting `evolve` (0.35) of the wind
   apart, contrast-restored (`sf_atmos::evolve_field`), so clouds form and dissolve as they
   drift; the billows rise through the shape volume (its vertical axis tiles: a `rise`
   phase) and the detail octave boils 3x faster.
4. *Variety:* nine genus presets (`CloudPreset`): clear, fair (cumulus humilis/mediocris),
   cumulus (congestus), stratocumulus, altocumulus, altostratus, cirrus, overcast
   (stratus/nimbostratus), storm (cumulonimbus cells + high veil). The volumetric layer holds
   one low or mid genus at its own base/thickness/type/erosion; the **high layer** is the
   planned 2-D cirrus sheet (Nubis/UE practice): a sphere at `cirrus_height` (8 km real),
   one fetch of the weather map's new A channel (domain-warped fibres along X, tiling),
   lit as thin ice (two-lobe HG g 0.8/0.2 + sky fill), alpha from the slant optical depth,
   composited behind the low layer, moving at 2x the wind (parallax), faded past the reach
   (no mips), and in the shadow map. `CloudWeather` gained `cirrus` and `cirrus_height`.
5. *Sun through clouds:* the composite order was right (no "render_sky not found" warning;
   the composite runs after `render_sky`), but the march stops at T < 0.01 and kept that
   1 %: the HDR sun disk is ~10^5 times the sky, so 1 % still burned through and bloomed.
   Now the remainder is snapped to zero (`T' = (T − 0.01) / 0.99`, Nubis): behind thick
   cloud the disk is gone; thin edges still let it through (correct, silver lining).

**Weather mapping** (Bevyrrowind, weather-adapter.md): Clear → clear, Cloudy → fair,
Foggy → altostratus, Overcast / Rain / Snow / Blizzard → overcast, Thunderstorm → storm,
Ashstorm / Blight → stratocumulus (fast). Cumulus, altocumulus and cirrus are reachable by
genus name and for mods.

**Test scenes.** Framework: `cargo run --release -p sf_atmos_bevy --example clouds -- --preset
<genus> --scale 0.5 --view sky|sun|shadow|up [--screenshot f.png]` (24 km plane, pillars every
500 m for scale; C cycles genera). App: `clouds-*` scenes, visual checks CL-1..CL-6.

**Cost.** Per density sample one more 2-D fetch (second weather octave); per pixel two more
(cirrus); the shadow map is 4x the texels with the detail fetch. Measured: see the S1
session log line in ROADMAP (shared machine).

**S1 cost (clouds3, 2026-10-13).** GPU timers: every cloud pass is a `RenderDiagnosticsPlugin`
span (`clouds_shadow`, `clouds_trace`, `clouds_resolve`, `clouds_composite`; the perf report's
"GPU passes" table). Per-pass GPU ms are the A/B measure: on the shared machine the frame was
CPU-bound (render thread ~20 ms), so frame-median deltas against `--off clouds` were noise.
Release, 2560×1440, RTX 4060 Laptop, Balmora camera toward Red Mountain at 16:30
(`shots/clouds3/ab.sh`, results `shots/clouds3/perf/`; shared machine: other agents compiled):

| preset | before: trace / shadow / composite / resolve = total | after (High) |
|---|---|---|
| fair | 1.02 / 0.44 / 0.19 / 0.05 = 1.70 | 0.68 / 0.04 / 0.19 / 0.05 = 0.95 |
| cumulus | 0.72 / 0.54 / 0.19 / 0.05 = 1.49 | 0.55 / 0.04 / 0.18 / 0.05 = 0.83 |
| overcast | 0.53 / 0.59 / 0.19 / 0.05 = 1.35 | 0.29 / 0.09 / 0.19 / 0.05 = 0.61 (8 slices) |
| storm | 0.53 / 0.69 / 0.19 / 0.05 = 1.47 | 0.38 / 0.09 / 0.19 / 0.05 = 0.71 (8 slices) |

Against High (clouds + far fog 0.9, shadow + weather maps 0.05): the shadow map is in budget
(0.04 at 16 slices); clouds alone are 0.91 on fair (at the line, no far fog yet), 0.79 cumulus,
≤ 0.62 overcast/storm. What changed:
1. *Shadow map:* **toroidal** (the light texture repeats; the light sits at a world anchor that
   moves in 2 km cells; each texel holds the repeat of its sky column nearest the camera, edge
   fade by distance from the camera), so texels stay fixed in the world and the map can be
   **time-sliced** (as reflection probes and far cascades are): one band of `size / 16` rows per
   frame (`CloudQuality::shadow_slices`, High 16, Ultra 4; `sf_atmos::ShadowSlicer`). Stale
   bands are ≤ 16 frames old: ≤ 5 m of drift at cloudy speed against 16 m texels. A full redraw
   on an anchor move, a light turn of more than 4 m at the camera since the oldest band (a
   time-of-day jump), or a weather jump (a preset switch, not a blend). Each texel now marches
   its column from sea level (before: from camera height). 6 samples per texel (Ultra 8).
2. *Trace:* the step is a continuous function of the span (`thickness / 20`, stretched only to
   keep the count in 32..64): an integer count dividing the span evenly jumps at every count
   change, which drew the faint **horizontal bands** in thick decks. Empty-space skipping
   (Schneider 2015): twice the step with the cheap density outside cloud, step back on a hit,
   back to coarse after 4 empty samples. Lighting LOD: two light samples (stretched to the same
   reach) where T < 0.3 or past 15 % of the reach; the first light sample reuses the primary
   sample's weather (a tenth of the layer away). High `max_steps` 80 → 64.
3. *Cumulus* coverage 0.40 → 0.34 (more sky between towers from the ground).

**Next.** Crisper cauliflower billows (curl-noise distortion of the detail lookup, height-
dependent erosion remap; tops read as soft vertical columns today), anvils (weather map A is
now cirrus: anvils need a written map), a cheaper composite (0.19 ms, a constant full-resolution
cost), far fog in the cloud node (no budget left on fair: trim the trace first), Low/Medium
presets measured.

