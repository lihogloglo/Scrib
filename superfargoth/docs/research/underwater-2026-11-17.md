# Underwater pass and waterline split: research (2026-11-17)

Scope: the render side of ROADMAP 3.6 (owner go 2026-11-17). Swimming physics is out of scope. Starting point is
`water.md` "Underwater" (468-496) and the water pass (287-369). Facts marked (verified) were read in a source or doc
this session; (recalled) comes from memory and must be checked before it goes into a spec. Ms figures are estimates
for 1707x960 internal (1440p + DLSS Quality) on the 4060 laptop, not measurements.

## Summary and recommendation

Do it the Crest way, in our own passes: a CPU camera-medium state, a per-pixel "ray starts underwater" mask from the
near-plane point against the displaced surface, one analytic underwater post (spectral Beer-Lambert plus in-scatter
by depth), the water's back face shaded with Snell's window and total internal reflection, caustics applied from depth
(in post, not in every material), and froxel shafts on High+ by reusing our atmosphere froxel volume as an additive
shaft term. Nothing in Bevy 0.20 or the crate ecosystem does this for us.

Three facts from our code shape the design:
- The water draws in the transmissive pass with depth write, and sky, aerial perspective, clouds and fog run after it
  by depth (`render-order.md` steps 4-8). So from below, the depth buffer already holds the surface seen from below.
  Those later passes would fog a below-water ray with air. They must be gated by the mask, or the look is double-fogged.
- `water_material.wesl:229` only flips the normal on a back face (`is_front`). Nothing shades from below yet.
- When the camera straddles the surface, the clipmap mesh is cut by the near plane. A pixel row can have no surface
  fragment at all and still start underwater. So the above/below decision cannot come from the mesh. It must be an
  analytic per-pixel test at the near-plane point (the spec already says this; Crest and Planet Coaster 2 do the same).

## Reuse check

| Item | What it is | Verdict |
|---|---|---|
| Bevy `FullscreenMaterial` (`bevy_core_pipeline-0.20.0/src/fullscreen_material.rs`, verified) | fullscreen triangle with a source texture and a uniform; default slot `Core3dSystems::PostProcess` before tonemapping | No. No depth, no extra textures, wrong slot (we need step 8b, before the transparent pass). Use our own system like `composite_pass`. |
| Bevy `VolumetricFog` + `FogVolume` (`bevy_pbr-0.20.0/src/volumetric_fog`, `bevy_light-0.20.0/src/volumetric.rs`, verified) | screen-space raymarch through shadow maps, 64 steps default, directional lights only, `VolumetricLight` marker, `FogVolume` is a transformed box with density, absorption, scattering | No. A box cannot be the infinite displaced ocean; per-pixel 64-step march costs more than our froxels; the repo does not use it (grep: no hits), our fog is `sf_atmos`. |
| Bevy `DistanceFog` (`bevy_pbr-0.20.0/src/fog.rs`, verified) | per-material exponential fog | No. Our fog is a composite by depth; one extinction scalar, no in-scatter by depth below the surface. |
| Bevy upstream | no underwater issue or PR found (search, not exhaustive); unified-volumetrics tracking issue #18151 mentioned in a design note (not read) | Nothing to adopt. Re-check #18151 before slice 6. |
| `bevy_water` and `bevy-aqua`-type crates | `bevy_water` roadmap lists volumetric water below the surface as undone (old v0.8 page, so possibly stale) | No. Neither does underwater. |
| Crest Ocean (Unity) | MIT for Crest 4 on GitHub (verified: repo page shows MIT; Crest 5 is paid, closed). Underwater Renderer: fullscreen effect between transparent and post, back-face water material, depth fog, optional meniscus, pixel-perfect partial submersion | Reference only, not ported (Unity/HLSL). We use the ideas; no code copied, so no notice needed. If a shader function is ever ported, MIT fits GPL-3.0-or-later and needs a `THIRD_PARTY_NOTICES.md` entry. |
| Abyssal Garden (MIT, one WebGPU file, verified licence) | shafts from a wavy surface, floor caustics, constants `WCOL`, `SUNC`, `RAYS`, `EXPO` in `shaders.js` | Mine for look constants at slice 5-6 (read the WGSL; the README gives no method). Port with a notice if code is used. |
| OpenMW underwater | fog colour and distance, dark-blue tint, camera-underwater flag (recalled) | Behaviour reference for a "classic" colour preset only. OpenMW is GPL-3.0; porting needs file + commit cited. Not needed for slices 1-5. |
| Our own `sf_atmos` froxel volume (`froxel.rs`, `froxel_inject.wesl`, `froxel_integrate.wesl`) | RGB in-scatter plus scalar transmittance, R3 jitter, clipped history, own CSM sampling | Reuse for shafts (slice 6). Its transmittance is scalar, so spectral water extinction cannot live in it (see Underwater medium). |

## Industry comparison

| Engine / source | Above-below decision | Waterline look | Medium | Notes |
|---|---|---|---|---|
| Crest (Unity, docs verified) | mask of the water surface rendered per frame; pixel-perfect in Crest 5; older "curtain" mesh on the near plane was deprecated | optional meniscus line at the lens-water intersection | fullscreen effect between transparent and post; depth fog with a density factor; out-scattering darkens environment light with depth; underside needs double-sided/cull off | effect disabled when not underwater. Closest match to our plan. |
| Planet Coaster 2 (Game Developer deep dive, verified) | screen-space pass builds a low-res texture of vertical distance from each pixel's world position to the surface; thresholded | smooth split-shot waterline | n/a | no numbers published. Same idea as our per-pixel test, built from depth instead of the near-plane point. |
| UE5 Water / Single Layer Water | water-body volume and post-process material; stencil/near-plane tricks (recalled); known to be flaky half in/half out (forum threads) | thin line via the material | post-process fog, optional light shafts | we do not copy; shows the pitfalls. |
| Sea of Thieves (SIGGRAPH 2018 talk) | not found in the sources read | | | skip; do not cite. |
| Subnautica, Horizon FW, AC IV | not researched in the sources reachable here | | | gap: recalled only that games mostly fake shafts and caustics. Log in `industry-audit.md` as unverified. |

Industry-standard techniques named: Beer-Lambert extinction per channel; single-scatter in-scatter scaled by ambient
irradiance at depth; Snell's window with total internal reflection (critical angle asin(1/1.333) = 48.6 degrees);
caustics from a baked tiling animated atlas (Guardado, GPU Gems 1 ch. 2, 2004) projected along the refracted sun with
a depth fade (Crest also applies caustics in its underwater pass from depth, recalled); screen-space or froxel shafts.

## Technique detail

### Camera medium decision
CPU, Bevy-free, in `sf_water` next to `query.rs`. Per frame, `WaterQuery::sample` at the near-plane corners and centre
(5 points, world space, the camera's own near distance): all below is `Under`, mixed is `Split`, all above is `Above`.
Hysteresis 1 cm, as the spec says. Emit the `underwater` and `surface_break` cues on state change. The sample returns the
displaced height, body id and depth (`WaterSample`, `query.rs:19`), so the medium also gets the body (colour, extinction
come from `WaterLook` per body) and the camera's depth below the surface. In an interior pool or a sea cave the body is
the pool's. The GPU gets a small uniform: state, camera depth below the surface, body optics.
Under with no resident bed: depth is infinite, use a deep-water colour.

### Underwater medium (fog, in-scatter, shafts)
- Extinction per channel, not scalar: clear water absorbs red first (rough values: red 0.45, green 0.07, blue 0.03
  per metre for clean water; coastal and murky are higher; Jerlov types, recalled). Take them from the same body optics
  the water draw uses for depth colour, so the surface and the volume agree.
- Per pixel: `T = exp(-sigma * d)` with `d` = camera to depth-buffer hit (the opaque surface, or the water back face
  when looking up). Colour = `scene * T + inscatter * (1 - T)`, `inscatter = albedo * ambient(z) * sun_term`, where
  ambient and sun fall off as `exp(-Kd * z)` at the camera's depth `z` (Crest's out-scattering idea). Looking toward
  the horizon uses the same formula with `d` large, which gives the familiar blue-green wall.
- Gate: when the mask says "starts underwater", `sf_atmos` composite (clouds, height fog, banks, froxel fog, far rain)
  and Bevy's aerial perspective in `render_sky` must not apply to that pixel. Fully underwater: a uniform flag
  skips them (the vendored `render_sky` can be patched, `superfargoth/vendor/`; the composite is ours). Split: they read
  the mask texture.
- Post slot: new `underwater_pass` at step 8b (after `composite_pass`, before the transparent pass), so particles drawn
  later (bubbles, silt) can use the same medium through `FroxelFogMaterial`-style fog. Transparent underwater objects
  are the Crest caveat; out of scope for the first cut.
- Shafts (High+): reuse the froxel inject, with a water variant. Do not put spectral extinction in the froxel
  (scalar transmittance); keep that in the analytic post. The froxel adds only an in-scatter gain: sun visibility
  (our CSM sampling already in `froxel_inject`) times a shaft modulation, where the modulation is the caustic atlas
  sampled at `xz - refracted_sun.xz * depth_below_surface` with a coarse mip. That makes shafts that follow the wave
  focusing without a ray march. Low/Medium: no shafts, or a screen-space radial blur from the sun point (cheap, only
  when the sun is inside Snell's window). Gap to check: how the froxel volume's `near/far` and slice distribution
  (`FroxelSettings::slice_distance`) behave with 20-40 m water visibility; tune, do not rebuild.

### Surface from below
Water material, `is_front == false` branch (`water_material.wesl:229` and below). Steps: shading normal as now (flipped),
view ray `v`, refract with eta = 1.333 about the flipped wave normal. If the refracted ray does not exist
(sin theta_t > 1) the pixel is total internal reflection: reflect `v`, and colour it with the medium's far colour
(deep in-scatter) plus, optionally, the SSPR-style bed reflection later (not planned). Inside the window: sample the
environment map and sky LUT along the refracted ray (inside the cone the sky is compressed into a circle about 97 degrees
wide). The edge is soft: blend by Fresnel (Schlick with eta swapped; F rises to 1 at the critical angle) and by
roughness so wave normals wobble the rim. Add sun disc glitter through the window. Because the sky pass skips pixels where
water wrote depth, the sky must be sampled inside this shader. The cloud and sky composite does not apply here (the mask
gates it). Needs: back faces drawn in the main water draw and the velocity redraw (cull mode check in `render.rs`), and
the snapshot refraction branch disabled from below (it would sample the opaque scene as if seen from above).
Keep a CPU reference of the Fresnel/critical-angle maths in `sf_water` so the shader has an exact test.

### Waterline split and meniscus
Per pixel, in a small pass before the sky: world position `P` of the near-plane point along the pixel ray (view origin +
ray * near / cos). Test `P.y < h(P.xz)`. `h` comes from the displacement cascades at that XZ, with one fixed-point
iteration for the horizontal choppy offset (the CPU twin `WaterQuery` does the same inversion; the shader must match its
height; test by an equal-input parity check). Output an `R8` mask (full resolution; or half resolution with a depth-aware
upsample if the cost shows). The signed distance `s = P.y - h` becomes the meniscus: a band `|s| < w`, with `w` about 1-3
mm scaled by pixel footprint so it stays about 1.5 pixels wide, drawn as a bright thin line using the wave normal for
a specular glint on the rising film. Waves crossing the lens work for free because `h` is the real displaced surface.
Never use the mesh for this decision (near-plane clipping, above). With DLSS: the mask edge is stable enough in jitter space
because it derives from the world-space test, not from a rasterised edge.
Transition polish (separate slice, optional): a lens-water film for a few seconds after emerging (a screen droplet normal
map, decay 3-6 s, distort + slight blur), and `surface_break` cue for splash. Both belong to the owner's taste
questions below.

### Caustics
Keep the spec's baked tiling atlas (32 frames, 256 squared, `water.caustics` bake step, Guardado photon splat). It is
the industry standard for games and fits "bake once under a few minutes". Change from the spec: apply it from depth in
two fullscreen spots instead of a hook in every material: (a) on the opaque snapshot copy (step 3), so a seabed seen
from above through the surface has it; (b) in the underwater post, for pixels that start underwater. Reconstruct world
position from depth, project the atlas along the refracted sun, fade by depth 0.3-12 m, multiply by sun visibility
(CSM plus cloud shadow; the froxel inject already has its own CSM sampling to copy) and by `max(0, n.l)` with `n` from
depth derivatives. No per-material cost, and terrain, `WorldMaterial` and characters all get it. Caveat: it lights what is
in the depth buffer only, so translucent objects and the water itself get none. If the owner finds a tiling atlas too
repetitive, the upgrade is a real-time Jacobian of the refracted wave field (caustic intensity from the cascade slope
derivatives, one fetch, matches the visible waves); log as a follow-up, not now.

## Build order (each slice: one deliverable + test; switch for A/B)

Every slice ends with a registered test scene in `bevyrrowind/docs/visual-checks/water.md` (status "awaiting owner")
and the question to the owner; the agents do not judge the look. Tunables (extinction, albedo, meniscus width, fade
depths, hysteresis) are declared with unit, range and source per `tunables.md`.

1. **Camera medium (CPU).** `sf_water::UnderwaterState` plus `WaterMedium` uniform; wired to `WaterCamera`; cues
   `underwater`, `surface_break`. Test: exact assertions (all-below, mixed, all-above, hysteresis at 1 cm, a camera
   riding a wave through the surface, pool vs sea body). 0 ms. Switch `water.underwater` (default on once slice 3 lands).
   Hooks: `sf_water/src/query.rs`, `sf_water_bevy/src/lib.rs`.
2. **Surface from below (Snell and TIR).** Back-face branch in `water_material.wesl`, the CPU reference
   (`sf_water::snell`: critical angle, Fresnel from below), back faces in the velocity redraw, snapshot refraction off
   from below. Test: CPU reference exact (48.6 degrees, F = 1 past it), shader-vs-reference parity on a sample grid, no
   NaN. About 0 ms (replaces shading, does not add a pass; ceiling to confirm: it runs only on back-face pixels). Switch
   `--off waterbelow` (old flipped-normal look). Scene: camera 2 m under a calm sea looking up at 0, 30, 60 degrees.
3. **Underwater post, fully submerged.** `underwater_pass` at 8b: spectral Beer-Lambert + in-scatter by depth, gate flag
   in the `sf_atmos` composite and the vendored `render_sky` aerial perspective. Test: a CPU reference of the colour
   formula (`sf_water::medium`, exact), a headless check that the pass is skipped above water (zero draws counted), log
   timer `render/water_underwater/elapsed_gpu`. Expected 0.1-0.25 ms when under, 0 above. Switch `--off underwater`.
   Hooks: `render.rs` pass setup, `sf_atmos_bevy/src/render.rs` composite (read the flag), `vendor/bevy_pbr` sky.
4. **Waterline split + meniscus (owner's priority).** The mask pass and the per-pixel gating (composite, sky, and the
   post read the mask), the meniscus band. Test: the mask against the CPU `WaterQuery` height on a grid of near-plane
   points (parity within a millimetre), mask empty when the state is `Above`, mask full when `Under`. Expected +0.1-0.2 ms
   only in `Split`. Switch `water.waterline` (off falls back to the camera-state binary switch at the centre height, a
   hard horizontal cut: the A/B arm). Scene: camera bobbing in 0.5 m waves, half in, at dusk and noon.
5. **Caustics.** `water.caustics` bake step, the depth-based apply in the snapshot copy and the underwater post. Test:
   bake determinism and tiling seam (edge texels match, exact), atlas energy conserved within a tolerance (sanity),
   shader compile log. Expected 0.1-0.2 ms with water in view (snapshot) plus 0.1 ms under. Switch `--off caustics`.
   Re-bake note: `bevyrrowind bake` before owner checks.
6. **Froxel shafts and in-scatter (High+).** Water variant of `froxel_inject` as an additive gain. Test: CPU reference of
   the shaft term (`sf_atmos::froxel`-style, exact), froxel count unchanged, timer. Expected 0.2-0.4 ms under, 0 above
   (a guess: the volume already runs for fog, so this reuses it rather than adding one). Switch `--off watershafts`.
   Low/Medium use the cheap screen-space radial option or none.

Deferred (not in this build): lens droplets, bubbles and marine snow particles, the audio low-pass (audio system owns it),
swim physics, and underwater Solari (water is not in the TLAS, `water.md` Solari note; beds lit as if dry, accepted).

Perf rule: each slice's A/B is an untraced interleaved `perf suite --arm` run (3+ runs), logged in `docs/ledger/water.md`.
A slice that wins becomes default; one that does not stays behind its switch (CONTRIBUTING).

## Risks
- The gating of sky and composite touches vendored Bevy and `sf_atmos`; keep it to one uniform plus one mask read.
- `render_sky` is a private set (`SkyOrderScaffold`); confirm the patch point before slice 3.
- DLSS motion vectors for the water's back face need the velocity redraw to include it (slice 2), else underwater ghosting.
- Near-plane distance and the clipmap: if the camera sits very close to a steep wave, the single fixed-point iteration for
  the choppy offset may miss by a few mm; the meniscus width absorbs it. Check against `WaterQuery`.

## Open questions for the owner (taste and scope only)
1. Lens droplets and a short wet-lens film after surfacing: yes now, or parked until swimming exists?
2. Underwater colour direction: physical clear tropical water (red gone at 5 m, visibility 30 m), or Morrowind's classic
   murky green-brown as default, with the physical look as a preset? (OpenMW's look is a reference for the classic one.)
3. Is a visible meniscus line wanted at all, or only the clean split with waves crossing the lens?
4. Transparent objects underwater (alpha particles, glass): fogged by the medium from the first cut, or a later slice?

## Sources
- Crest underwater docs: https://crest.readthedocs.io/en/4.17/user/underwater.html and https://crest.readthedocs.io/en/4.11/user/underwater.html
- Crest repo (MIT): https://github.com/crest-ocean/crest
- Abyssal Garden (MIT): https://github.com/Noniv/abyssal-garden
- Planet Coaster 2 water deep dive (split-shot waterline): https://gamedeveloper.com/programming/deep-dive-crafting-detailed-and-dynamic-water-in-planet-coaster-2
- Sea of Thieves SIGGRAPH 2018 talk (no waterline detail found): https://history.siggraph.org/?p=84316
- bevy_water (old release page, underwater unchecked): https://crates-io.ganymede-static.progval.net/crates/bevy_water/0.9.0
- Bevy volumetric fog docs: https://bevy.org/examples/3d-rendering/volumetric-fog/ ; unified volumetrics design note: https://hackmd.io/@bevy/SyaDBESi1l
- Bevy 0.20 sources read: `bevy_core_pipeline-0.20.0/src/fullscreen_material.rs`, `bevy_pbr-0.20.0/src/volumetric_fog/mod.rs`, `bevy_light-0.20.0/src/volumetric.rs`
- Guardado, "Rendering Water Caustics", GPU Gems ch. 2 (2004), https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-2-rendering-water-caustics (cited from memory, not fetched)
