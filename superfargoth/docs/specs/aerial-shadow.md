# Shadowed aerial perspective (ROADMAP 1.27, FG-12)

## Problem

A low sun hidden behind a ridge still lights the haze in front of the shaded terrain: a round warm glow sits on the
rock, centred on the hidden sun (FG-12, owner 2026-11-22, x10s canyon at 18.09 h). Our own fog is ruled out (live
`flip fog` keeps the glow). The cause is Bevy's aerial perspective: `sample_local_inscattering`
(`vendor/bevy_pbr/src/atmosphere/functions.wesl:240`) shadows each directional light only by the atmosphere's
transmittance and the planet (`ray_intersects_ground`), never by terrain or shadow maps. Weather haze scales Mie up
(x20-x80, ledger SK-HB), so the forward lobe toward a low sun is strong.

## Goal

The sun's single scattering in the aerial view LUT is shadowed by the terrain, so haze in front of shaded terrain stops
glowing, and haze crossing a ridge's shadow gets coarse far light shafts as a side effect. Sky pixels (sky-view LUT) and
the multiple-scattering term are unchanged.

## Reuse check

- **Bevy 0.20:** no. Both `AtmosphereMode::LookupTexture` and `Raymarched` (bevy #20766) shadow only by the planet.
  Upstream tracking issue bevyengine/bevy #20374 (open) says it outright: "the shadows remain un-sampled in the
  volumetric passes in the atmosphere pipelines", and sketches the fix as specializing the atmosphere pipelines on the
  mesh view bindings to reuse the directional shadow sampling. No open PR (searched 2026-11-22: "atmosphere shadow",
  "volumetric shadow atmosphere", "aerial perspective"). #19037, the PR that first promised volumetric shadowing, was
  closed unmerged in 2025-08 and split; the shadow part never landed.
- **Crates:** none. `bevy_atmosphere` is a sky-dome shader with no aerial perspective.
- **Ours:** the terrain far shadow map (`sf_terrain_bevy/src/far_map.rs`, 384² texels over 9.2 km, a shadow height per
  texel) already reaches the froxel fog as `sf_atmos_bevy::FogShadowHeights` (`lib.rs:192`), read by `far_visibility`
  (`froxel_inject.wesl:74`). Sun cascades end at 205 m on High (wiki shadows.md), too close for this bug: the FG-12
  rock is past them. So the far map is the source, not the cascades.

## Industry comparison

- **UE5 SkyAtmosphere** (Hillaire 2020, the technique Bevy's atmosphere ports): the directional light's "Cast Shadows
  on Atmosphere" lets opaque shadows (cascades, plus Far Shadow cascades for large objects) shadow the atmosphere, and
  "Cast Cloud Shadows" lets clouds do it (light shafts). Off, UE has the same glow as ours.
- **Reference implementations:** AirGuanZ's DX11 port of Hillaire 2020 builds the aerial LUT with a volume shadow at
  200×150×32 in 0.2 ms on a GTX 1060.
- **Hillaire 2020** names volumetric shadows from hills and mountains as the hard case, where a per-pixel ray march
  beats the LUT. Our LUT is 32×32×32 with 1 km slices (`aerial_view_lut_max_distance` 32 km) and 6 samples a slice
  (`sf_sky_bevy/src/lib.rs:676`): good enough to kill a broad glow, coarse for crisp shafts. Crisp shafts are FG-10 and
  stay with the froxels near the camera.

## Design

1. **Vendored patch `superfargoth patch (own, AS: aerial shadow)` in `bevy_pbr/src/atmosphere/`**, game-agnostic: a
   camera component `AtmosphereHeightShadow { image: Handle<Image>, params: Vec4, light_dir: Vec3, ... }`, i.e. a
   top-down height field (texel = the height above which a point is lit, plus an occluder distance for the penumbra)
   for one directional light, in render space. It is extracted to the render world and bound to the aerial view LUT
   pass (and the raymarched path, same function) only when present; absent = stock shader (a shader def).
2. In the aerial view LUT loop, each sample's position goes back from atmosphere space to render space
   (`world_from_atmosphere`), reads the height field (same rule as `far_visibility`: smoothstep over a penumbra that
   grows with the occluder distance, bias with distance, fade to lit at the map edge), and multiplies the **single
   scattering** of the light whose direction matches `light_dir` (dot > 0.99, as the froxels do). The
   multiple-scattering term stays unshadowed (it is the sky's isotropic fill; shadowing it would grey the valleys at
   noon).
3. **Wiring (framework):** `sf_atmos_bevy` (or `sf_sky_bevy`, whichever already sees the `SkyCamera` and
   `FogShadowHeights`) copies `FogShadowHeights` onto the camera's `AtmosphereHeightShadow` each frame it changes. No
   Morrowind names; the game already fills `FogShadowHeights`.
4. **Tunables** (`tunables.md`): `sky.aerial.shadow` (bool, on; `--off aerialshadow` = stock, the A/B arm), and the
   bias, penumbra and edge-fade values as declared tunables shared with the froxels' `far_visibility` (today bare
   `const`s in `froxel_inject.wesl:33-37`; declare them once, both passes read them). `aerial_view_lut_samples` stays 6
   unless the owner sees banding in the shadowed haze (then raise it: cost scales linearly).

## Cost

One bilinear fetch per LUT sample: 32×32×32×6 ≈ 197k fetches a frame, expected well under 0.1 ms (AirGuanZ: 0.2 ms
for 30× the texels on a 1060). Measured in S2 with the F3 panel's `atmosphere_luts` span, untraced A/B
`--off aerialshadow` (ledger perf).

## Slices

- **S0** this spec (2026-11-22).
- **S1** the vendored patch + wiring + tunables. Tests: the patched passes link and validate (as `shader_tests.rs` NS-3),
  the extracted uniform's Rust/WGSL layout matches, the component is copied when `FogShadowHeights` changes, and absent
  = stock shader def. clippy -p on the touched crates. Patch file `vendor/patches/superfargoth-aerial-shadow.diff`,
  `vendor/README.md` row.
- **S2** cost A/B (above) + owner check FG-12 (x10s canyon at 18.09 h, low sun behind the ridge; A/B `--off
  aerialshadow`).
- **Later (not queued):** cloud shadows in the same term (UE "Cast Cloud Shadows": cloud shafts at distance); a shadowed
  sky-view LUT for crepuscular rays across the sky; offer it upstream on #20374 once it works.

## Owner verdict on S1 (2026-11-22)

AS on, x10s canyon at 18.09 h: "not very convinced". The shafts are blocky and move with the camera like big animated
blocks (32×32 LUT texels and 1 km slices, re-fit to the view every frame), and a halo is still there, also where it
shouldn't be. Owner asks for a classic god-ray shader. AS stays, off by default (`sky.aerial.shadow` false).

## S3: screen-space light shafts (the classic technique)

**Canonical technique:** volumetric light scattering as a post-process (Mitchell, GPU Gems 3 ch. 13): an occlusion
mask from the depth buffer (sky = 1, geometry = 0) is blurred radially toward the sun's screen position. UE ships it
as two directional-light options: **Light Shaft Occlusion** (the blurred mask masks fog and atmosphere in-scattering,
`OcclusionMaskDarkness`, `OcclusionDepthRange`) and **Light Shaft Bloom** (the colour around the light, blurred
radially and added back, with scale, threshold, max brightness and tint). Unity's legacy Sun Shafts and CryEngine's
sun shafts are the same idea. It is exact at silhouettes (read from depth at full detail), it doesn't swim (no world
volume re-fit), it reaches any distance and needs no shadow map; it fades out as the sun leaves the screen, which is the
accepted limit everywhere it ships.

**Reuse check:** Bevy 0.20's `VolumetricFog` + `VolumetricLight` raymarch the shadow map per pixel: our cascades end at
205 m, and the froxels already cover the near field, so it adds nothing far away. No Bevy crate for screen-space
shafts (searched 2026-11-22). AS (S1) reused the far map but is too coarse; it stays as an opt-in.

**Design (framework, `sf_atmos_bevy`, a game-agnostic render pass; one vendored hook in `render_sky`):**

1. **Mask + radial blur** after the depth prepass, before `render_sky`, at half the internal (DLSS) resolution: m = 1
   where depth is sky (or farther than `sky.shafts.occlusion_depth_m`), else 0; then a few passes of a few taps each,
   each pass stepping further toward the sun's screen position (the multi-pass form of Mitchell's march: few taps,
   long reach, no noise). Off when the sun is behind the camera; strength fades as the sun's screen position leaves the
   screen (`sky.shafts.offscreen_fade`). Output: o in [0, 1] per pixel, "how much of the way to the sun is open sky".
2. **Occlusion (S3a, the halo fix):** geometry pixels' aerial perspective in-scatter (vendored `render_sky`, geometry
   branch) and the analytic fog's sun term (`cloud_composite.wesl:152`) are multiplied by
   `1 − darkness · (1 − o) · lobe`, where `lobe` is the haze's forward lobe toward the sun normalised to 1 at the sun
   (HG with `g` = the medium's Mie asymmetry), so only the light that comes from toward the sun is masked and side-lit
   haze is untouched. The froxels keep their own shadow-mapped sun (not masked twice).
3. **Bloom shafts (S3b, the visible rays):** the scene colour of open-sky pixels near the sun (after the clouds
   composite, so cloud gaps make shafts too), thresholded and clamped, blurred radially with the same passes and added
   back before the transparents and bloom: scale, threshold, max brightness, tint as tunables.

**Tunables** (`sky.shafts.*`, `Guess` until the owner tunes them live): `enabled`, `occlusion` (darkness), `occlusion_depth_m`,
`passes`, `taps`, `length` (fraction of the way to the sun), `offscreen_fade`, `bloom_scale`, `bloom_threshold`,
`bloom_max`, `bloom_tint`. Switch `--off shafts`.

**Cost:** half-res fullscreen passes of 8-ish taps each; expected ~0.1-0.2 ms on the 4060 at 1440p + DLSS Q
(UE's docs time the two options separately: enable only what is used). Measured in the slice's A/B.

**Slices:** S3a mask + blur + occlusion (halo), S3b bloom shafts; owner check FG-12 after each, tuned live.

**S3a as built (2026-11-22):** `sf_atmos_bevy/src/shafts.rs` + `shaders/shafts_{common,mask,blur}.wesl`, CPU twin
`sf_atmos::shafts`. Sun = the `CloudShadowLight` (`GlobalTransform::back`, the light the far map and cloud shadows are
made for; the composite masks its sun term only when its key light is that light, `dot > 0.99`). Per view with Bevy's
`AtmosphereSettings`: mask + blurs into two cached `R16Float` targets at `(internal + 1) / 2` (the
`MainPassResolutionOverride` size, as the clouds), one GPU span `shafts`; `Core3d` after the transmissive pass, in the
vendored set `RenderSkyInputs` that `render_sky` runs after, before the transparents (the composite also `.after` it).
Hook: vendored patch "own, LSO" (`AtmosphereLightShafts` render-world component, `ATMOSPHERE_LIGHT_SHAFTS` def,
`patches/superfargoth-light-shaft-occlusion.diff`). Blur step of pass k = `length · taps^k / (taps^passes − 1)`
(`(taps − 1) · Σ step = length`). Defaults: occlusion 0.95 (UE `OcclusionMaskDarkness` 0.05), passes 3 (UE
`r.LightShaftBlurPasses`), lobe g 0.8 (our haze Mie), the rest `Guess` (taps 8, length 1, fade 0.3 UV, depth 100 km:
UE's 1 km `OcclusionDepthRange` would leave the km-away ridges of FG-12 open).

**S3b as built (2026-11-22):** `bloom_pass` in `sf_atmos_bevy/src/shafts.rs` + `shaders/shafts_{bloom,apply}.wesl`, the
same `shafts_blur` shader (now summing all four channels) and pass steps as the mask; CPU twins `sf_atmos::shafts::
{bloom_color, bloom_falloff, sun_seen}`. Graph: `render::finish` adds it with `add_pass(PassStage::Clouds)` `.after`
the composite, so after `render_sky`, the night sky and the clouds, before the medium (underwater) pass, the
transparents and Bevy's post. Per view with `ShaftsBloomCamera` (enabled + `sky.shafts.bloom`): source at half the
internal resolution (2x2 texels: the mask's depth test, then the resolved pre-exposed scene colour, luminance rescaled
to `bloom_max` and its excess over `bloom_threshold`, hue kept), times `(1 − d/bloom_radius)²` around the sun (d in
screen heights); blurs into two cached `Rgba16Float` targets; additive apply into the view target at the internal
viewport × `bloom_tint` × `bloom_scale` × off-screen fade × night gate. Night gate: the planet's shadow on the sun as
seen from the camera, the twin of `key_light.wesl` `planet_lit_at` (CPU, from the view's `ExtractedAtmosphere`); the
scene colour itself fades the twilight. GPU span `shafts_bloom`. No vendored patch. Defaults (UE's light shaft bloom
values as recalled; UE's docs state none, so `Guess`): scale 0.2, threshold 8, max 100 (pre-exposed units = UE's
scene colour after pre-exposure: 1 ≈ white before tonemapping, so 8 keeps the sun's disc and inner aureole), tint
white, radius 0.5 screen heights.

## Architecture review (2026-11-22)

Owner: everything FG-12 built on the atmosphere except the god rays is "maybe"; audit it and settle the architecture.

**What lights the haze on a terrain pixel today (low sun):**

1. **Bevy aerial perspective** (`render_sky`, aerial view LUT 32×32×32, 1 km slices, 6 samples each): the medium
   (Rayleigh, Mie × weather haze x20-x80, ozone). Weather haze lives here. Every directional light's single and
   multiple scattering, shadowed only by the planet (`functions.wesl` `sample_local_inscattering`).
2. **Our analytic height fog and banks** (`cloud_composite.wesl` `fog_to`), from the froxels' far (256-512 m) on:
   sun = `key_seen × HG(fog.phase_g = 0.6)` plus the ambient fill. Unshadowed; now × the LSO mask.
3. **Froxels** (to 256-512 m): our fog only; the sun shadowed per froxel by the cascades, the terrain far map and the
   cloud shadow map (`froxel_inject.wesl` `csm_visibility`, `far_visibility`, `cloud_visibility`).

Sky pixels: the sky-view LUT (unshadowed, which is right: a sky ray's light comes from 100+ km) plus our fog to
`fog.sky_distance`.

**Verdict on the three media:** keep them. The split is the industry one (UE: Sky Atmosphere + Exponential Height Fog +
Volumetric Fog, composed in the same order). The fog can't move into Bevy's medium: its density LUT spreads 256
texels over the whole atmosphere height (hundreds of metres a texel), and our fog's scale height is tens of metres.
The fault is elsewhere: **the sun's visibility is computed in one medium (froxels) and missing or faked in the other
two.**

**FG-12's work, audited:**

- **AS (off):** the right idea (shadow only the key light's single scattering, with the same far map), at the wrong
  resolution: 32×32 screen texels × 1 km slices, re-fit to the view every frame, no jitter or history, so blocky and
  swimming. Keep it off. Its shader variant becomes dead code once A3 lands. Its wiring (`aerial_shadow.rs`, far map →
  render world) is reused.
- **S3a LSO (on by default): a screen-space guess, wrong for structural reasons, not tuning ones.**
  (a) `render_sky` multiplies the whole aerial in-scatter (Rayleigh, multiple scattering, moon, every light) by the
  mask, so the haze vanishes where the sun hides: the **dark disc**. (b) The mask's lobe is `HG(0.8)^1.5` normalised to
  1. That is narrower than the medium's real Mie halo and is not the fog's phase (g 0.6), so a **ring** of glow
  survives in both the aerial perspective and the fog. (c) It fades as the sun's screen position leaves the screen
  (`offscreen_fade` 0.3), so the **FG-12 glow comes back whenever the hidden sun is just off frame**, and it says
  nothing about shadowed haze away from the sun. UE ships Light Shaft Occlusion as a look option on its height fog,
  not as atmosphere shadowing; on the atmosphere, UE's answer is the light's "Cast Shadows on Atmosphere".
  The rays the owner liked come from the composite masking the fog's sun term only (ambient kept), which is the
  correct split. The disc comes from the `render_sky` mask.
- **S3b bloom (off):** the look was rejected; leave it off.
- **Tunables:** `sky.far_shadow.*` drive the froxels and AS, but the terrain's own far shadow
  (`sf_lod_bevy/src/shaders/far_shadow_map.wesl:15-22`) keeps bare consts "kept in step". A live change makes the
  surface and volume shadows disagree.
- **Default game now:** a dark disc ringed by glow wherever the sun sits behind terrain on screen; the glow returns off
  screen.

**Target: one sun visibility for all haze.** The canonical technique is to shadow the key light's *single scattering*
per pixel along the view ray, leaving multiple scattering and the other lights alone. Sources: Hillaire 2020 (a
per-pixel march for mountain shadows, where the LUT is too coarse); UE "Cast Shadows on Atmosphere"; Yusov, GPU Pro 5
(epipolar sampling, the full-resolution form).

1. **Split the key light's single scattering out of the aerial LUT** (vendored; replaces AS's shader change). The
   aerial view LUT pass also writes a second 32³ texture holding only the shadow light's single scattering
   (unshadowed, same log encoding). `render_sky` then computes `L = L_total − (1 − V) · L_key_single`. That can't go
   negative and can't leave a disc (Rayleigh fill, multiple scattering and the moon are untouched), and it needs no
   phase guess (the real phase is in the LUT).
2. **One sun-visibility pass** (framework, `sf_atmos_bevy`). Half the internal resolution, in `RenderSkyInputs`
   (after depth, before `render_sky`, as the LSO mask runs now). Each pixel marches from the camera to the scene
   depth, or to the far map's reach for sky pixels and far terrain: 16 samples (Guess), blue-noise jitter, history
   reprojected and clipped (the froxels' rule). Every sample reads **one** `sun_visibility(p)`: cascades inside their
   range, far map beyond, × cloud shadow. It moves out of `froxel_inject.wesl` into a shared `sun_visibility.wesl`
   that the froxels import, together with `far_shadow_map.wesl`'s values (one rule, no "kept in step"). One march
   yields two weighted means: `V_atmo`, weighted by the Bevy medium's scattering density × transmittance to the camera
   (density LUT), and `V_fog`, weighted by our fog density from `froxels.far` on.
3. **Consumers:** `render_sky` uses `V_atmo` through the vendored hook (the LSO binding slot, re-purposed); the
   composite's fog sun term uses `V_fog` in place of `shaft_occlusion`. The froxels already use the same rule and
   don't change.
4. LSO S3a stays as an opt-in arm, off once the new pass is signed off (rule "a change that doesn't win stays").

**What it fixes:** no disc and no ring; it works with the sun off screen and for haze away from the sun; it is
world-space, so it doesn't swim; it is as sharp as the far map (24 m texels) and the cascades. Shafts become real
crepuscular rays in the fog and haze wherever terrain or clouds shadow them, on screen or not (also FG-10).

**Limits:** past the far map's reach (±4.6 km) the shadow fades to lit, so a ridge farther away still lets the glow
through. The later fix is a coarser second far-map level (UE Far Shadow cascades). The sky-view LUT stays stock.

**Cost (expected):** about 854×480 × 16 samples × (shadow + density + cloud fetch), roughly 20 M fetches, ~0.15-0.3 ms
on the 4060 at 1440p + DLSS Q. It replaces the LSO mask and its 3 blurs (similar size). The LUT split costs ~0. It
gets measured in A4.

**Slices:**

- **A0** (small, look): LSO's `render_sky` mask off by default (no dark disc), and the fog mask's lobe set to the
  fog's own phase (no fog ring). The rays stay; Bevy's aerial glow (the original FG-12) returns until A3.
- **A1** shared `sun_visibility.wesl` + `sky.far_shadow.*` read by `far_shadow_map.wesl` too. A refactor: froxel
  output unchanged.
- **A2** sun-visibility pass + `V_fog` in the composite.
- **A3** vendored aerial LUT split + `V_atmo` in `render_sky`; AS's shader variant deleted.
- **A4** owner check FG-12 (x10s canyon 18.09 h, sun on and just off screen) + cost A/B against LSO; then the default.

**A3 as built (2026-11-22):** vendored patch "own, LSO" extended (`patches/superfargoth-light-shaft-occlusion.diff`). A
view with `AtmosphereLightShafts` gets `AtmosphereTextures::aerial_view_key_lut` (32³ `Rgba16Float`, same log encoding)
and the aerial view LUT pass runs with `ATMOSPHERE_KEY_SINGLE` (bindings 17 shafts uniform, 18 key LUT): per sample the
single scattering of the light within `dot > 0.99` of `light_dir` (planet shadow, E1b scale, same analytic integration
and throughput as the total). With AS on, the `ATMOSPHERE_HEIGHT_SHADOW` + key variant gives the key term the same
height shadow, so `total − key` stays the unmasked rest (they combine; AS is still off). `render_sky` (LUT path, geometry
pixels) computes `total − (1 − v) · min(key, total)`, `v` = the unchanged `light_shaft_occlusion` (exactly 1 at
`lobe_g` 0 on an open texel, `1 − darkness` on a closed one); the raymarched path stays unmasked. The light-shafts
variant runs only on frames whose aerial pass wrote the key LUT; absent component = stock. AS's variant stays (cleanup
slice). Tests: `sf_sky_bevy` `shader_tests` (four aerial variants, `render_sky` bindings, CPU twins of the mask).

**A1/A2 as built (2026-11-22):** A1: `sf_atmos_bevy/src/shaders/sun_visibility.wesl` holds the rule (`csm_visibility`,
`far_visibility`, `cloud_visibility`, `sun_visibility = min(csm, far) · cloud`, textures passed in); `froxel_inject.wesl`
imports it (output unchanged). `far_shadow_map.wesl` (sf_lod_bevy) still keeps its own consts (logged bug, not A1's).
A2: `sf_atmos_bevy/src/sun_visibility.rs` + `shaders/sun_vis_{common,march,resolve}.wesl`, CPU twin
`sf_atmos::sun_visibility`. Half internal resolution, `RenderSkyInputs`, span `sun_visibility`. March: nearest of the 2×2
depths, `t = reach · x²` (denser near the camera), R2 pixel + R1 frame jitter; segment weights in closed form
(`T · σs/σt · (1 − e^(−σt Δ))` from Bevy's density LUT, mean of RGB; fog `e^(−τ_before)(1 − e^(−τ_seg))` from the froxels'
far, height layer only, banks left out). Beyond `max_distance_m`: one lit closed-form segment to the scene (sky:
`fog.sky_distance`). Resolve: world-point reprojection (sky: direction), 3×3 Karis clip, `Rg16Float` R = V_atmo, G =
V_fog. Hook: `AtmosphereLightShafts` (light_dir sun, darkness 1, lobe_g 0); composite fog sun term × V_fog.
Tunables `sky.sun_vis.{enabled true, samples 16, max_distance_m 5000, history 0.9}` (Guess but `enabled`), `--off sunvis`;
`sky.shafts.enabled` now false (LSO = A/B arm). With A3, V_atmo masks only the key light's single scattering.

## Open questions

None for the owner: the look is judged in FG-12.
