# Underwater water line / meniscus: research (2026-11-26)

Research only; nothing built or measured. All ms figures are estimates for an RTX 4060 laptop, High preset, 1440p + DLSS Quality (internal ~1706x960), and must be confirmed with `perf ab` once built.
Scope: the clean split line (half above, half below) when the lens crosses the surface, the meniscus band, lens droplets on exit. Storm waves and spray are out of scope.

## 1. Summary

- **Cause.** The medium is one CPU decision per view (`Above`/`Split`/`Under`), made from 5 near-plane points against a height that has only the swell cascade. The GPU draws the whole underwater post or nothing: `Split` draws nothing, `Under` fogs the whole frame at once. No pixel is ever tested against the displaced surface (`sf_water_bevy/src/medium.rs:211-214`). U4 (waterline mask), U2 (Snell/TIR from below), U5 and U6 are not built.
- **Canonical technique (Crest, Unity HDRP, UE water plugin all converge).** A per-pixel "ray starts underwater" mask from the displaced surface, rendered or computed before the underwater post. The post fogs only below-mask pixels, with fog distance = min(scene, surface exit). The meniscus is a thin band on the mask edge. Sky and aerial perspective are gated by the same mask. Horizon: a far-plane fill so pixels with no surface geometry still classify.
- **Best fit for us.** Compute the mask analytically in the post, from the near-plane point of each pixel ray against the displacement cascades (spec "Waterline split" bullet). Because we have the height, we get a signed distance `s`, so the meniscus is an analytic anti-aliased band (Crest has a binary mask and probes 3 neighbour pixels). This removes the Split dead zone and the all-or-nothing flip with no mask render target.
- **Top fixes (est.).** (1) Per-pixel waterline in the post, drawn in `Split` and `Under`: +0.10-0.25 ms only while the lens is near the surface, 0 otherwise. (2) Sky/composite gating by the same test: ~0 ms. (3) Ray-to-surface exit distance for fog: ~0 ms. (4) Meniscus band: ~0 ms (inside the post). (5) CPU query gets the short cascades, or the post turns on by a conservative band around the surface: CPU ~0. (6) Lens droplets after surfacing: 0.15-0.3 ms for ~4 s per exit, 0 otherwise.
- **Slices.** U4a (mask-in-post + Split draws), U4b (gating + exit distance), U4c (meniscus), U4d (CPU query / conservative band), U2 (back face), W-LENS (droplets).
- **Owner questions.** Meniscus look (bright film vs. dark rim vs. both), droplets on or off, whether the line follows waves above the lens in first and third person alike (recommended: yes).

## 2. What we do today

### Built / not built
`git log` shows U1 and U3 in commit `86f4d9fb` (water batch). No U2, U4, U5, U6 commit; `grep U[2-6]` in `docs/roadmaps/*.md` and `bevyrrowind/docs/ROADMAP.md` finds nothing for them. Wiki says it too: "the post, waterline and caustics are U3-U5" (`docs/wiki/engine/terrain-and-water.md:107`).

| Slice | State | Where |
|---|---|---|
| U1 camera medium (CPU, cues) | built | `sf_water/src/medium.rs`, `sf_water_bevy/src/medium.rs` |
| U2 surface from below (Snell, TIR) | **not built**; the water material only flips the normal on `!is_front` (`sf_water_bevy/src/shaders/water_material.wesl:318`), cull is off (`material.rs:231`) | - |
| U3 underwater post, fully submerged | built | `sf_water_bevy/src/shaders/water_underwater.wesl`, `render.rs:924-1000` |
| U4 waterline mask + meniscus | **not built** | - |
| U5 caustics, U6 shafts | not built | - |

### Data flow now
1. `mw_environment_bevy/src/water.rs:216` `medium` system calls `WaterMediums::update` with `water.world.sample` (the CPU water query).
2. `sf_water_bevy/src/medium.rs:270-298`: near-plane corners from the projection (`NDC z = 1`, reverse Z); `NearPlane::from_view` gives 4 corners + centre. Near plane is 0.1 m (`mw_exterior_bevy/src/exterior.rs:1020`), so all five points lie within ~0.14 m of the eye.
3. `sf_water/src/medium.rs` `CameraMedium::update` (lines ~120-175): `all below by hysteresis -> Under`; `all above -> Above`; else `Split` (with the last-state rule for in-band points). Hysteresis 1 cm (`water.medium.hysteresis_m`, `sf_water_bevy/src/medium.rs:51`).
4. `extract` (`sf_water_bevy/src/medium.rs:313-348`): only if `post_on` (state == `Under`, line 213-214) does the camera get `UnderwaterUniform` and `SkyOccluded` (the sky, aerial perspective and `sf_atmos` composite skip the whole view: `sf_atmos_bevy/src/render.rs:1207-1213`, vendored `bevy_pbr/src/atmosphere/node.rs:236-254`).
5. `water_underwater.wesl`: full-screen pass at `PassStage::Medium` (after composite, before transparent: `sf_render_order_bevy/src/passes.rs:43`). Per pixel: `T = exp(-sigma_t * d)` with `d` = distance lens to depth-buffer hit; depth 0 (sky) -> T = 0 -> medium colour. Nothing in the shader reads a surface height.

### Why the switch is sudden (exact)
- **`Split` draws nothing.** `post_on` is false for state 1 (comment `medium.rs:211`: "Split (1) is the waterline's (U4)"). The frame stays fully "above" while the surface crosses the lens, with the water pass's own back-face shading (U2 missing) as the only hint. This is the "gap at the surface" noted in the WA-10 diag.
- **`Under` is a per-view switch.** The frame the 5th near-plane point drops 1 cm below the surface, `SkyOccluded` is inserted (sky, clouds, aerial perspective vanish for the **whole** view) and the post fogs the **whole** view. Pixels that, one frame earlier, showed the air above a wave crest are now fogged. With swamp optics (visibility ~0.5 m, WA-10) the screen goes to medium colour in one frame.
- **No per-pixel test exists.** Nothing compares a pixel's near-plane point with the displaced height. The decision is 5 points, CPU only.
- **The CPU height is not the drawn height.** `sf_water/src/query.rs:323-345` adds only cascade 0 (swell, `body.waves.first_cascade == 0`) plus shore waves; the short cascades (chop) that the GPU displaces are omitted (ledger WA-10: "CPU query omits the short cascades"). Chop of centimetres to decimetres is larger than the 1 cm hysteresis, so the CPU state and the visible crossing disagree by that much, and the state toggles against a surface the player does not see.
- **Fog distance ignores the surface.** The post uses the depth-buffer hit, not the point where the ray leaves the water. Pixels looking up/out through the surface are fogged as if the medium continued to the far hill (or infinity for sky), which is also why they flatten to medium colour.
- **Interiors read Above** (pools are not in the CPU query; ledger U1).

## 3. Bevy 0.20 native and crates

Checked in `~/.cargo/registry/src/*/bevy_*-0.20.0` and `superfargoth/vendor/`.

- `bevy_core_pipeline/src/fullscreen_material.rs`: `FullscreenMaterial` (fragment shader + schedule hook). The pass has `depth_stencil_attachment: None` (line ~327) and the trait offers no depth binding; it is a colour-in colour-out post. Fine for droplets/lens, not for the depth-aware fog (our own pass already has the depth binding, `render.rs:261-266`).
- `bevy_post_process`: `bloom`, `dof`, `motion_blur`, `auto_exposure`, `effect_stack/{chromatic_aberration, lens_distortion, vignette}`. `lens_distortion.rs` is a template for a source-texture-in, destination-out distortion pass (the droplet refraction); nothing for droplets, wetness, underwater or water line.
- `bevy_pbr`: `SkyOccluded` (vendored, `atmosphere/mod.rs:224`) is a per-view marker; no per-pixel variant. `VolumetricFog`/`FogVolume` cannot model an infinite displaced ocean (already in spec). Transmission/`ScreenSpaceTransmission` is a material feature, not a medium.
- **Crates.** `bevy_water` (Neopallium, 0.17/0.18): wave material and `get_wave_point`; its roadmap lists a "dynamic depth buffer for partially submerged objects", no underwater post. `bevy-aqua 0.1.0` (Bevy 0.19, depth-aware transmission, GPU surface queries; docs.rs build failed). `bevy_vfx_bag` underwater effect is Bevy 0.9/0.10. None give a waterline; confirmed nothing to adopt. (Search-level check only; no source read.)

Conclusion: build in our own pass (as already planned).

## 4. Industry

### 4.1 Crest (MIT; cloned, commit `ff0602f07a687d47e5c803e10e3383d41c42e709`, `crest/Assets/Crest/Crest/`)
Data flow, in order:
1. **Mask pass** (`Shaders/Underwater/UnderwaterMaskShared.hlsl`): the ocean mesh is drawn with the same vertex displacement as the surface (`Vert` samples `_LD_TexArray_AnimatedWaves` for two LOD slices, same snapping/blending), into an R16F target (`UnderwaterRenderer.Mask.cs:182`; RFloat on Intel) with a depth buffer. `Frag` writes by `SV_IsFrontFace`: back face (the camera sees the surface from below) = `CREST_MASK_BELOW_SURFACE` -1, front face = `ABOVE` +1 (`OceanConstants.hlsl:34-41`). A `discard` below `near+0.001` is the MSAA outline fix ("@MSAAOutlineFix").
2. **Horizon pass** (`UnderwaterMaskHorizonShared.hlsl`): a full-screen triangle at a configurable far-plane offset writes above/below by world Y of the far point vs the ocean centre height; fills pixels where no tile exists (horizon, culled tiles).
3. **Post** (`UnderwaterEffect.hlsl`): per pixel reads mask and ocean depth (`_CrestOceanMaskDepthTexture`); `GetOceanSurfaceAndUnderwaterData` (`UnderwaterEffectShared.hlsl:98-160`): `isUnderwater = mask <= BELOW`; if ocean depth is nearer than scene depth, the ocean depth becomes the fog distance ("Merge ocean depth with scene depth"). So fog distance = min(scene, surface exit): exactly the piece we lack.
4. **Meniscus** (`ComputeMeniscusWeight`, `UnderwaterEffectShared.hlsl:57-92`): take the horizon normal projected on the camera plane (`UnderwaterRenderer.Effect.cs:420-425`: dot(up, cam.right), dot(up, cam.up)), sample the mask 1, 2, 3 px along it (direction flipped by sign of mask); each sample that differs from the centre multiplies the colour by 0.9. Band = ~3 px, darkening ~27%. Scaled down with distance for water volumes (`MENISCUS_MAXIMUM_DISTANCE 15`). The older `UnderwaterMeniscus.shader` is a camera-attached quad (deprecated, kept for the old path). Alpha output marks where the effect wrote.
5. **Cheap path**: `_FULL_SCREEN_EFFECT` keyword: if the near plane is below sea level by more than the maximum displacement (`cameraY + nearPlaneFrustumWorldHeight + maxOceanVerticalDisplacement <= seaLevel`, `Effect.cs:~410`) the mask is skipped and the meniscus off. Same idea as our conservative CPU band.
6. **Temporal**: the mask is MSAA-resolved or not; the discard near-plane trick avoids a shimmering rasterised edge. No special TAA handling.

### 4.2 Unity HDRP Water (docs, 17.2 "Underwater view")
Full-screen post (analytic absorption, or volumetric buffer when volumetric fog is on, giving light shafts). "When the camera is at the water surface, the view adds a boundary between the surface above and the underwater view below"; customise it through a Custom Pass with the HD Sample Buffer node, source `IsUnderwater`. So HDRP also has a per-pixel underwater buffer, no documented mask resolution or line thickness. Limits worth noting: non-infinite surfaces use a collider volume; custom meshes must be flat at Y = 0.

### 4.3 UE5 Water plugin
Underwater is a **post-process volume** per water body, enabled when the camera is detected inside via collision (docs `UnderwaterPostProcessSettings`, "only available when bGenerateCollisions is true"). It has no waterline: forum threads (e.g. "Underwater fog, half in and half out", "Ocean post process pops above the water", "Under water / above water split view") report exactly our symptom (flat volume, wave crests break it) and workarounds by custom post-process materials that test height per pixel or a mesh split. Epic ships no partial-submersion solution. UE counts as the anti-example, not the reference.

### 4.4 Sea of Thieves
SIGGRAPH 2018 "The Technical Art of Sea of Thieves" (Ang, Catling, Ciardi, Kozin) covers FFT water stylisation, foam from depth-buffer comparison, shallow-water sim; the public text has nothing on the waterline. Not usable as a reference for this problem (the PDF's underwater part was not read: talk PDF at `history.siggraph.org/.../2018-Talks-Ang_The-Technical-Art-of-Sea-of-Thieves.pdf`, only the abstract was checked).

### 4.5 Others (no public technical source found)
Subnautica, Uncharted 4, AC Black Flag, Battlefield: no public talk on waterline or lens split found in this pass. Not claimed.

### 4.6 Lens droplets after surfacing
- Fluid Frenzy 2.0 wet-lens (Frenzy Byte, via 80.lv): screen-space render textures remember where the lens is wet and how deep it went; water runs down and leaves smaller droplets; fades over a few seconds. Same data flow we would use (wetness state + droplet normal layer).
- Tatarchuk, "Artist-Directable Real-Time Rain Rendering in City Environments" (SIGGRAPH 2006 Advances, `advances.realtimerendering.com/s2006/Tatarchuk-Rain.pdf`): droplet simulation on glass (wetting, merging, separation).
- Godot "Rain on Glass" (godotshaders.com): droplet normals offset the background, blur grows where trails are dense. Procedural grid-cell droplet shaders with trails are the common cheap form.
Canonical: a scalar wetness decaying over seconds; a screen-space droplet normal field (procedural cells, no texture needed); refract the scene colour by the normal; blur slightly; trails drip downward.

### 4.7 The canonical technique, stated precisely
1. For each pixel, decide whether its view ray **starts in water**: displaced-surface height `h(xz)` at the near-plane point vs the point's height (analytic in our case; Crest rasterises the mesh and reads the face orientation).
2. Store or compute `s = P.y - h` (signed, metres). Crest stores only the sign.
3. Underwater post fogs only `s < 0` pixels, over `min(scene distance, distance to surface exit)`. Sky / aerial perspective / cloud composite are gated off those pixels.
4. Meniscus: a band `|s| < w` where `w` = N pixels of footprint (Crest 3 px; ours 2-4 px, tunable), darkened/brightened and refracted.
5. Horizon: pixels where no surface exists (beyond the clipmap) classify by the far-plane point vs the sea level (Crest's horizon quad); irrelevant to an analytic near-plane test, which needs no surface geometry.
6. Temporal: with jittered projection the edge moves sub-pixel; use a smooth band edge (analytic `s` gives it for free) instead of a binary mask.

## 5. Why ours differs (point by point)

| # | Crest / canonical | Ours today |
|---|---|---|
| 1 | Per-pixel mask from the displaced surface | One CPU state per view, 5 points |
| 2 | Mask uses the full displaced mesh (all cascades) | CPU height = cascade 0 + shore waves only (`query.rs:323-352`); GPU draws all cascades |
| 3 | Post runs for Split and Under | `post_on` only for Under (`medium.rs:213`) |
| 4 | Fog distance = min(scene, surface exit) | Scene depth only (`water_underwater.wesl` `view_distance`) |
| 5 | Sky/fog composite gated per pixel by the mask | `SkyOccluded` is per view, on at Under |
| 6 | Meniscus band at the mask edge | none |
| 7 | Surface seen from below handled (back face) | Back face only flips the normal; no Snell/TIR (U2 unbuilt) |
| 8 | `_FULL_SCREEN_EFFECT` skips mask when far below | all-or-nothing, with no between state |
| 9 | Wetness/droplets are add-ons in other engines too | none (deferred in spec) |

Agreements: camera medium on the near plane, not the eye (U1); depth-fog analytic post; the plan's "Crest's design in our own passes". The gap is U4 and its coupling to gating.

## 6. Ranked fix list

Frame target <= 8.3 ms median, <= 11 ms p99. All GPU estimates are for the waterline-active case only (lens within the wave band of the surface, a few seconds per crossing); steady-state Above and deep Under cost what they cost today.

| # | Fix | Est. cost | Spec / slice | Risk |
|---|---|---|---|---|
| 1 | Per-pixel waterline inside the underwater post: sample displacement cascades at the near-plane point (same cascades as the water vertex shader, one fixed-point iteration for choppy XZ offset), output `s`, fog weight `smoothstep` of `-s`; run the post in `Split` as well as `Under` | +0.10-0.25 ms GPU (3-4 cascade taps on a ~1.6 MP target; post already ~0.1-0.2 ms when Under) | water.md "Waterline split" bullet; **U4a** | Medium: GPU height must equal the vertex shader's (parity test vs water vertex path); DLSS edge behaviour |
| 2 | Gate sky/composite/aerial per pixel by the same test; replace per-view `SkyOccluded` by a mask read in the composite, or keep the composite and have the post fully replace below-mask pixels (cheaper: no new gating, below pixels get overwritten by `T*scene + inscatter` after a composite that shouldn't have touched them; double fog is small for short air paths but wrong for sky pixels, which the post already sets to medium colour via T = 0) | ~0 ms (recommended first step: post-overwrite, no gating change); mask-gated composite +0.05 ms | Plan point (a); **U4b** | Low for post-overwrite; Medium for editing the atmos composite (shared lane) |
| 3 | Fog distance = min(scene distance, distance to the surface along the ray) using the flat plane at the local surface height `h_c` (camera's surface), not a mesh: `t = (h_c - eye.y) / dir.y` for upward rays | ~0 ms | **U4b** | Low; waves tilt the true exit, hidden by the surface being drawn over it. If wrong look: use `h` at the exit's XZ (one more tap) |
| 4 | Meniscus: analytic band `|s| < w_px * footprint`; bright thin film (colour/intensity tunables) + 1-2 px refraction offset of the colour above the line; footprint = near-plane pixel size (`0.2 near*tan(fov/2)/height_px`, ~0.2 mm/px at 0.1 m, 90 deg, 960 px, so the band is under a millimetre in world terms: it is a screen-space look, not a physical size) | ~0 ms (in the post) | **U4c** | Low; look = owner |
| 5 | CPU truth: either add the short cascades to `WaterQuery::sample` (cost: more cascade reads per sample; spec says CPU = cascade 0 on purpose) or, simpler and standard (Crest `_FULL_SCREEN_EFFECT`), run the post whenever `|camera near-plane Y - local level| <= max displacement amplitude + margin` (`water.waterline.band_m`); the GPU test is then the truth and the CPU state is only for audio/cues/optics | CPU ~0 | **U4d** (new) | Low |
| 6 | Drive `post_on` by the band, not by `state == Under` (`medium.rs:213`): Above (outside band) none; inside band and Under both draw the post with the per-pixel test; `SkyOccluded` only when the test says the whole screen is below (`s < -margin` for all, i.e. the existing Under, where it is exact) | 0 | **U4a** | Low |
| 7 | U2 surface from below (back face: Snell window, TIR to the underwater colour) | +0.05-0.15 ms when seen | water.md plan **U2** | Medium; needed for the line to read well (the surface above the line is seen from below near it) |
| 8 | Lens droplets on exit: CPU wetness scalar (`surface_break` up: 1.0; decay 3-6 s, scaled by time submerged); procedural cell droplets + downward trails, refract + 2 px blur, drawn only while wetness > 0 | 0.15-0.3 ms for ~4 s per exit, else 0 | new **W-LENS** | Low; owner taste; ordering vs DLSS |
| 9 | Interior pools into the CPU query (so interiors leave Above) | CPU small | new / U1 gap | Medium (CPU query data model) |

## 7. Proposed build slices

One deliverable plus its test each. Framework code stays in `sf_*`; the game adds only tunable bindings in `mw_environment_bevy`. Tunables declared in `sf_water_bevy` `medium.rs` style with unit/range/source (a `Guess` is allowed and counted); census must pass.

1. **U4a, waterline in the post.** `water_underwater.wesl` gets `s` per pixel from the cascades; `post_on` true in the band (`Split` and `Under`); uniform carries band + meniscus params. Test: CPU reference function `sf_water::underwater::waterline_s` (exact assertions: flat surface, tilted camera, wave crossing, all-above/all-below) and a headless parity check that the shader's cascade sampling matches the water vertex path's height at a grid of points (log-line check, no pictures). Tunables: `water.waterline` (bool, live switch `--off waterline`), `water.waterline.band_m` (m, 0.05-2, Guess 0.4).
2. **U4b, no double fog and exit distance.** Fog distance = min(scene, plane exit); post overwrite for below-mask pixels in `Split`; `SkyOccluded` stays per-view at deep `Under`. Test: exact `underwater_colour` cases for the new distance in `sf_water::underwater` tests.
3. **U4c, meniscus band.** Band width in px, strength, film colour as tunables (`water.waterline.meniscus_px` 0-8 Guess 3; `.meniscus_strength` 0-1; `.refract_px` 0-4). Test: CPU weight function exact values at `s` = 0, +-w, beyond. Visual-check entry (awaiting owner): scene = Seyda Neen shore plus open sea, camera dipped in and out, handheld pitch up/down, roll; what to look for: line continuous, no pop, follows waves.
4. **U4d, CPU/GPU agreement.** `water.waterline.band_m` derived from the body's wave amplitude (`WaterParams`) instead of a bare constant; the CPU `Under` state keeps its cue/audio duty. Test: the medium tests extended with an amplitude-derived band.
5. **U2, surface from below** (already in the plan). Test: Snell/TIR math exact (critical angle 48.6 deg).
6. **W-LENS, lens droplets.** `LensWetness` state per `WaterCamera` (CPU, decays; exact tests: set on exit, linear/exponential decay, scaled by submersion time), droplet pass at the end of the main chain; skip entirely at wetness 0. Tunables: decay s, droplet density, refraction px. Visual-check entry.
7. **Later:** interior pools into the query (U1 gap), U5, U6.

Order: U4a -> U4b -> U4c, then U4d, then U2, then W-LENS. U4a/b/c are the owner's "midline".

DLSS note (all of them): the band is written into the pre-upscale colour, analytic and smooth so jitter only moves it sub-pixel; DLSS may ghost a moving wave-crossing line slightly. If it does, options are a 3 px band, or applying the meniscus after upscale (render-order stage check needed). Not measured; owner judges the look.

## 8. Owner questions (taste and scope only)

1. **Meniscus look.** Bright thin film (water clinging to the lens), dark rim (Crest default), or both? Recommended: bright film with a slight refraction of the image just above the line (lit, dramatic).
2. **Lens droplets after surfacing.** Yes or no, and how long (3-6 s)? Recommended: yes, ~4 s with downward streaks; default on, switchable.
3. **First person vs third person.** The line at the camera lens makes sense in first person; in third person the camera swings through the surface often. Same effect in both? Recommended: yes, same effect (the near-plane test is the camera's, one rule).
Not asked (industry default): mask format, resolution, per-pixel analytic vs mesh mask, DLSS handling, band defaults.

**Answers (owner via meta-3, 2026-11-26):** 1 yes: bright thin film + slight refraction above the line; 2 yes: lens droplets ~4 s on exit; 3 same effect in 1st and 3rd person (meta-decided).

## 9. Sources

- Crest (wave-harmonic/crest), cloned `depth 1` to `C:\Users\leogloglo\AppData\Local\Temp\research-1\water-line\crest`, commit `ff0602f07a687d47e5c803e10e3383d41c42e709`. Files read: `Shaders/Underwater/{UnderwaterMaskShared,UnderwaterMaskHorizonShared,UnderwaterEffect,UnderwaterEffectShared,UnderwaterMeniscus}.{hlsl,shader}`, `Shaders/OceanConstants.hlsl`, `Scripts/Underwater/UnderwaterRenderer.{Effect,Mask}.cs`. MIT. Read as data only, nothing run.
- Unity HDRP docs, "Underwater view": https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@17.2/manual/water-underwater-view.html
- UE Water plugin: https://dev.epicgames.com/documentation/unreal-engine/water-body-actors-in-unreal-engine ; API `FUnderwaterPostProcessSettings`; forum threads: https://forums.unrealengine.com/t/underwater-fog-half-in-and-half-out/299106 , https://forums.unrealengine.com/t/under-water-above-water-split-view-is-this-possible/88081 , https://forums.unrealengine.com/t/ocean-post-process-pops-above-the-water/839266
- Ang et al., "The Technical Art of Sea of Thieves", SIGGRAPH 2018: https://history.siggraph.org/?p=84316 (abstract only checked).
- Fluid Frenzy 2.0 wet lens: https://80.lv/articles/this-unity-fluid-simulation-tool-lets-you-add-realistic-wet-lens-effects
- Tatarchuk 2006, rain: https://advances.realtimerendering.com/s2006/Tatarchuk-Rain.pdf ; Godot "Rain on Glass": https://godotshaders.com/?p=13124
- Crates: https://github.com/Neopallium/bevy_water , https://docs.rs/bevy-aqua , https://docs.rs/bevy-vfx-bag (search results only).
- Bevy 0.20 source: `bevy_core_pipeline-0.20.0/src/fullscreen_material.rs`, `bevy_post_process-0.20.0/src/effect_stack/`, vendored `superfargoth/vendor/bevy_pbr/src/atmosphere/{mod,node}.rs`.
- Our code: paths above; spec `superfargoth/docs/specs/water.md` "Underwater" (lines 489-530); ledger `docs/ledger/water.md`.

### Comparison with the older doc (`underwater-2026-11-17.md`, read after the findings above)
- **Agree:** Crest design in our passes; near-plane-point per-pixel test with the displacement cascades and a fixed-point XZ iteration (§"Waterline split and meniscus"), `s = P.y - h` as the meniscus signed distance, gating of sky/composite by the mask, DLSS-stable because world-space, lens film as a separate optional slice, no crate/Bevy feature to adopt.
- **Differ / add:** (a) the old doc proposes an R8 mask render target (full or half res); this report computes `s` inside the post, so no target is needed unless the composite is to be gated (then a half-res R8 or a recompute in the composite). (b) It does not point out that `Split` draws nothing and that `post_on` is the single line that makes the switch a pop; this report names `medium.rs:213`. (c) It misses that the CPU height has only cascade 0, so the CPU `Under` disagrees with the drawn surface by the chop height. (d) It leaves out the min(scene, surface-exit) fog distance that Crest has, and the conservative-band skip (Crest `_FULL_SCREEN_EFFECT`). (e) Its meniscus width "1-3 mm" is in world units; a world-metric band is sub-pixel at a 0.1 m near plane, so the band must be defined in pixels.
