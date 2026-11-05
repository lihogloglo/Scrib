# Local storm seen from outside: cloud curtain and noisy border (WE-10), 2026-11-06

Diagnosis and research only (`cloud-edge` session), no code changed. Owner report: from outside a rain or
ash-storm region, the clouds look stretched from very high to almost the ground and plunge straight down at the
zone border, and that border is visually noisy. Line numbers are from the working tree at 01:30 (the night-fog
session has uncommitted edits in `cloud_trace.wesl`).

## Our pipeline in short

- `sf_weather::WeatherMap` (WE4a-c): 256² texels of 64 m; the resolve blends two climate rows over
  `BORDER_BAND_M` = **256 m** (`sf_weather/src/frame.rs:28`). Layers A coverage/type/wetness/storm, B
  base/thickness/density/erosion, C cirrus/fog density, D precipitation extinction.
- Clouds (`cloud_trace.wesl` `trace`): half-res march of the envelope shell, map per sample on High; coverage
  thresholds one weather noise (`cloud_common.wesl:118`); shape noise z = local height fraction.
- Far weather (WE4d, `far_weather`, `cloud_trace.wesl:153`): below the base, 8 quadratic steps to
  `min(scene, 8.2 km)`; shafts `D(x) − D(camera)` gated by deck coverage, plus far fog `C.z(x) − C.z(camera)`.
- Resolve (`cloud_resolve.wesl`): checkerboard (1 of 4 pixels per frame on High), reprojection, 3×3 min/max
  clamp, **`mix(history, current, 0.5)`** on a fresh pixel (`:112`).
- Morrowind (`mw_weather/src/visual.rs`, heights × 0.5, thickness ≤ 1.6 km): Clear base 600 m / 350 m thick;
  Rain (Overcast) 350 / 750 m, coverage 0.97; Thunderstorm 450 m / 1.6 km; Ashstorm (Stratocumulus) 450 / 325 m,
  coverage 0.74. Precipitation extinction rain 2e-3/m, ash 8e-3/m; fog visibility rain 3 km, storm 2 km, ash 450 m.

## What we saw (own exe copy, `scene we3-ridge`, BRP forces, 1280×720, High, 14:00)

- **Ashlands = ashstorm, looking north:** a brown wall fills the right two thirds of the frame from the deck at the
  top of the screen down to the horizon. Its left side is a straight vertical edge (the region border). With `fog`
  off the wall stays (it is the shaft term, not fog); with `clouds` off everything goes (deck, shafts, far fog
  share the clouds switch). Over the far sea the wall stops at a hard horizontal line: the sea is drawn
  unveiled, the same symptom as **WE-7**.
- **Ascadian Isles = rain, looking east:** an overcast slab with a soft vertical face, under it a grainy grey
  curtain to the horizon with a vertical edge. As a thunderstorm, a deck seen along a region edge is a vertical
  band from the top of the frame to the horizon.
- **Noise, measured, not judged:** mean per-pixel change between two consecutive frames of a static view (sum of
  RGB, 8-bit): clear sky 0.16-0.2, terrain 0.36, deck inside 0.39, **rain curtain 1.2, deck edge 0.8, ash curtain
  side edge 2.8, ash deck 4.8 (max 26)**. Enlarged, the deck shows a regular interleaved-gradient dither pattern.
  The noise is frame-to-frame shimmer that does not converge on a still camera.

## Causes

**(a) The curtain.** Three terms meet at the same vertical plane, the region border:
1. *The deck wall.* Coverage goes from Clear 0.08 to Overcast 0.97 (or Stratocumulus 0.74) across 256 m, about a
   fifth of the deck's height, and 0.74+ coverage puts the noise threshold below zero (`cloud_common.wesl:118`),
   so the region is a solid slab cut by a sheer vertical face, not clouds thinning out.
2. *The shafts fill the whole region.* `sigma_p = dp × ramp(0.05, 0.5, w.coverage) × ...`
   (`cloud_trace.wesl:197`): with a solid deck the coverage gate is 1 everywhere, so the "cells" gate never cuts and the
   column below the base is uniform from the base down to the ground. `dp` is the in-storm extinction
   (`PrecipKind.extinction`, i.e. what you see standing inside, `visibility_m`): ash at 8e-3/m is opaque about
   300 m inside the border, so the border plane reads as a solid brown wall. The far fog difference adds another
   2e-3 (thunderstorm) to 8.7e-3/m (ash) on the same plane.
3. *Vertical noise stretch and shear.* Shape noise z = height fraction (`cloud_density.wesl:128`): a 1.6 km
   thunderstorm deck stretches one noise period over 1.6 km vertically (1 km horizontally), and inside the band the
   base and thickness change, so at one altitude the noise z slides fast with horizontal position. Billows become
   vertical streaks that seem to dive at the border.
Together: deck face (to 1.1-2 km) + opaque curtain (base to ground) = one wall from "very high" to the ground. A
storm 1-3 km from the camera spans 10-30° of the view.

**(b) The noise.**
1. *Far-weather undersampling.* 8 steps over up to 8.2 km: step lengths 128 m … 1.9 km (`ds = span(2i+1)/64`).
   With 2e-3 to 1.7e-2/m one step is already opaque (τ 2-30), so whether a jittered sample lands inside the 256 m band,
   the base fade (150 m) or a short in-region chord is a coin toss per pixel and frame. That makes the dithered
   side edges and the curtain grain.
2. *The deck wall at distance.* `ds = 0.05·thickness + g·t` grows to hundreds of metres; the coarse march (2·ds)
   steps back on a hit, so the entry into a sheer face jitters by a step.
3. *History too short to average it.* Fresh pixels blend 50/50 with history (`cloud_resolve.wesl:112`); with
   checkerboard a pixel is fresh 1 frame in 4. An exponential average with weight 0.5 leaves √(0.5/1.5) ≈ 58 % of
   one sample's noise; the 3×3 min/max clamp of noisy neighbours doesn't help. TAA/DLSS run after the composite but
   clamp the shimmer back in. HZD integrates over 16 frames.
4. *Not the cause:* map resolution or filtering (64 m bilinear texels, 4 per band: smooth, just narrow).

## How the references do it

- **Horizon Zero Dawn** (Schneider & Vos, SIGGRAPH 2015, [pdf](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf),
  slides 39-45, 81, 92-93): weather map R coverage / G precipitation / B type over the visible area;
  precipitation turns the map into cumulonimbus at 70 % coverage, darkens it, and drives the rain effects;
  64-128 march steps; quarter-res buffer, **1 of 16 pixels per 4×4 block per frame**, the rest reprojected.
- **Nubis 2017** (Schneider, [pdf](https://advances.realtimerendering.com/s2017/Nubis%20-%20Authoring%20Realtime%20Volumetric%20Cloudscapes%20with%20the%20Decima%20Engine%20-%20Final%20.pdf),
  slides 46-50, 54-59, 71-72): the ~10 × 10 km cloud map's coverage is **low-frequency noise for air masses plus
  higher-frequency noise for cloud footprints**; anvils inflate coverage near the top; weather states blend over
  10-30 s. HZD's 10 regions and 70 zones **switch the weather state at the camera in time**; a neighbouring
  zone's weather is never drawn as a spatial region.
- **Nubis Evolved, Horizon Forbidden West** (Schneider, SIGGRAPH 2022, [pdf](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf),
  "Vertical Profile Model", "Influence NDFs", "Superstorm"): 2-D data fields over 16 km tiles (min/max height, coverage, bottom/top type);
  `dimensional_profile = vertical_profile × coverage`, `density = saturate(noise − (1 − dimensional_profile))`,
  so a falling profile *erodes* the noise into cloud edges rather than cutting a face; noise sampled at the world
  position (wind-scrolled); step `3 m + 60 m × distance / 16 km`. Regional weather = **"influence NDFs" with
  influence masks** blended over the procedural fields; the superstorm has its own mask and fields.
- **Red Dead Redemption 2** (Bauer, SIGGRAPH 2019, [pptx](https://advances.realtimerendering.com/s2019/slides_public_release.pptx),
  slides "Cloud Map", "Rain Fog", "Fog Map", "Raymarch Reconstruction"): a 512² cloud map over the ~32 × 32 km world,
  "modulated by texture and weather data", plus a height LUT; **"rain fog beneath clouds, placement derived off the
  density in the cloud map", with a height falloff connecting it to the cloud**; a 512² fog map (~8 × 8 km) composited
  from **noise textures**, weather and time of day. Half-res raymarch, 1 of 4 rays per frame, blue-noise ray offsets,
  3×3 AABB clamp, blue-noise dithered upscale + TAA; they note "some flickering can still occur from undersampling + clamp".
- **UE5** (see `distant-weather.md`): layout textures; local weather = override volumes blended at their edges.

**Common thread:** what is seen from afar is shaped by noise-modulated maps (air masses, cloud footprints), not by
region masks; rain hangs under *cloud density*; regional weather is a soft influence over a procedural field, with
transitions as wide as the clouds are tall or wider; clouds converge over 4-16 frames.

## Gaps (also industry-audit.md C1-C4)

C1 short cloud history (50/50, min/max clamp, IGN); C2 far media undersampled (8 steps, opaque per step);
C3 region masks shape the far look (256 m band, uniform curtain); C4 shape noise in height-fraction space.

## Fix plan (3 slices; RTX 4060 laptop, 1440p + DLSS Quality, High; clouds now ≈ 0.54-0.60 ms)

1. **Cloud temporal accumulation (C1).** `cloud_resolve.wesl`: fresh weight ~0.1-0.2 (≈ 8-16 frames) with
   variance clipping (mean ± γσ, Salvi 2016) instead of min/max; spatio-temporal blue-noise jitter (STBN, 64-frame
   cycle, one small texture) for the trace and `far_weather` instead of IGN. Size S (≈ 60 lines + one 128²×64
   texture). Cost ≤ +0.01 ms. Risk: ghosting on fast turns and wind; keep the population rule, test FG-1's turn.
   Check: re-run the frame-to-frame measurement (printed, `#[ignore = "measurement"]`) and owner WE-10.
2. **Far media shaped by clouds, not regions (C2, C3).** `far_weather`: shafts only where the deck *rains*
   (deck density at the base, Nubis' 70 %-coverage rule as a precipitation gate, RDR2 rain fog) eroded by the
   weather noise at the edges, with virga; far fog and shaft terms read a **visual map blurred to ~1-2 km**
   (separable blur of C/D in the resolve, on change only) with differences still taken against the blurred
   value at the camera so near = far holds; 12-16 steps with per-step τ clamped. Size M (`cloud_trace.wesl`,
   `weather_map.rs`, `sf_weather/src/map.wgsl`, twin test). Cost +0.03-0.06 ms with a storm in view, +0.02 ms
   per resolve. Owner question: should ash read as a wall from outside (haboob) or as haze?
3. **Deck edges (C3, C4).** Shape noise z in world metres (`altitude / shape_tile`); coverage edge erodes the
   noise (`saturate(noise − (1 − profile·coverage))`) and A/B read the blurred visual map, so a storm deck thins
   out over kilometres. Size S-M (`cloud_density.wesl`, `cloud_common.wesl`, `sf_atmos` CPU twin). Cost ≈ 0.
   Changes every weather's clouds: owner pass over all genera plus `scene we3-ridge` (WE-10).

Order: 1 first (cheap, fixes the noise everywhere and makes 2-3 judgeable), then 2, then 3.
