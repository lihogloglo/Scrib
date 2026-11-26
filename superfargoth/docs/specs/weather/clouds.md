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

### Nubis look and decks below the peaks (CL-1..CL-8, specced 2026-11-26)

Source: `superfargoth/docs/research/clouds-nubis-2026-11-26.md` (§2 path:line, §6 costs). Slice IDs CL-1..CL-8 are build
slices; the closed 2026-10 visual checks of the same names live in the archive. Owner answers (2026-11-26): towering
2-4 km cumulus; Nubis-contrast undersides with a floor; strong silver lining with a live A/B arm; **some decks sit
below the peaks**, so a player on a summit can stand inside a deck (fog-like view) or above it (sea of clouds). No
free-flight volumes, envelope SDFs or voxels (Nubis³ stays the hard-problem class).

**Why ours does not look like Nubis** (research §5, by impact): (1) shape: value-noise lattices, 128-512 m cells, no
curl, vertical axis = layer height fraction (squash `thickness / shape_tile`, Fair 0.49), one smoothstep profile;
(2) shape and erosion at mip 0 and full strength to 50 km (`cloud_density.wesl:132,143`): mush and shimmer far
away; (3) flat unoccluded ambient (`cloud_trace.wesl:652`) and an uneroded straight light march (`:53-84`): no form
on the shaded side; (4) weak forward lobe (g 0.6, `cloud_common.wesl:224`); (5) 1/16 px per frame, weight 0.06;
(6) decks not authored (no 11 % stratus, no 2-4 km cumulus, every Morrowind deck above Red Mountain's 271 m).

**Reuse check.** Bevy 0.20: no volumetric clouds (`bevy_pbr` atmosphere = LUTs; `VolumetricFog` = froxels).
Crates: `bevy-volumetric-clouds` (MIT, Bevy 0.18 HZD port, 12 steps) is below ours on every axis. Godot HZD demo
(MIT) gives constants; HDRP (Unity Companion Licence) technique only; Hillaire's `TileableVolumeNoise` is the
Perlin-Worley reference (licence unread: we write our own). Ours stays ours (composited with the fog split).

**Industry comparison** (research §2.4, §4).

| Component | Industry | Ours after CL | Match |
|---|---|---|---|
| Base noise | gradient Perlin-Worley + 3 Worley octaves RGBA 128³, curl 128² (HZD, Frostbite) | CL-1 same | match |
| Vertical axis, profile | world altitude; 4-knot profile per genus (HZD), UE5 height profile per type | CL-2 | match |
| Distance LOD | noise mip by distance (Nubis, HDRP), erosion fade 3→100 km (HDRP) | CL-3, by pixel footprint | match (footprint, not metres: our tiles scale with the world) |
| Ambient | profile x density above (Nubis 2022/3), SH x AO (HDRP), scale + desaturate (Hillaire 5.5.1) | CL-4 + floor | match |
| Light march | 6 cone samples + far sample, cheap LOD at T 0.3 (HZD); eroded (HDRP) | CL-4 | match |
| Phase | dual-lobe HG (Frostbite eq. 18), HDRP 0.7/-0.7; HG+Draine (Jendersie & d'Eon 2023) | CL-5 dual lobe | gap: Draine later (fits unread) |
| Sampling | 1/16 px + 16 frames (HZD); 1080p, 96-180 samples (Nubis PS5) | 1/16, CL-6 option 1/8 | match HZD |
| Camera in or above a layer | fixed-altitude layers, mountains poke through, low deck + fog under it (Nubis, ledger 2026-11-12); UE5 marches from the eye inside the layer (unverified detail) | CL-7 + CL-8 | match; envelope fly-through (HFW) not built |
| Fog with clouds | fog split at the cloud's weighted depth (UE5, Frostbite) | WE12-FIX kept | match |

**Design.**
1. *Noise v2 (CL-1).* `sf_atmos::noise`: gradient Perlin FBM, R = Perlin-Worley, GBA = Worley at 1x/2x/4x (RGBA8
   128³); detail 32³ Worley FBM; curl 128² RG8. Tiles divide 16 384 m (origin phase rule): shape 8 192 m, detail
   1 024 m (game x0.5). Generated off-thread as today; past 1 s it goes through the `atmos.noise` bake step.
2. *Shape sampling (CL-2).* Shape sampled in world altitude for every deck (`shape_height` with `cb` = 1);
   detail offset by curl x `sky.cloud.noise.curl_m`; 4-knot profile per genus (HZD: stratus 0.02/0.05/0.09/0.11,
   stratocumulus 0.02/0.2/0.48/0.625, cumulus 0.01/0.0625/0.78/1.0; preset data), bottom-heavy `pow(d, (1-h)·0.8+0.5)`.
   S3's storm terms stay on top. The CPU twin follows; the shadow map reads the same density.
3. *Distance LOD (CL-3).* `r` = pixel footprint / texel at the sample; mip = `log2(r) + noise_mip_bias`; erosion
   x `1 - smoothstep(erosion_fade_start, erosion_fade_end, r_detail)`. The shadow map keeps mip 0.
4. *Ambient and light (CL-4).* Fill = sky gradient (bottom bounce → top sky) x `max(floor, pow(1-profile, exp) ·
   exp(-od_up))`, `od_up` from two uneroded samples straight up (reused from the light march when the sun is within
   30° of zenith), desaturated toward luminance by `ambient.desaturate`. Light march: first `eroded_samples` eroded,
   STBN cone offsets, one far sample at `far_sample_m`; the T < 0.3 cheap LOD stays. Scene ambient under decks
   (exposure X3, `sky.cloud.ambient_nits`) is unchanged: only the cloud darkens.
5. *Phase (CL-5).* `mix(HG(g_fwd), HG(g_back), back_weight)`, octaves scale both g as today. `silver_lining`
   picks `strong` (default) or `subtle` (today's 0.6 / -0.3 / 0.35), live.
6. *Sampling (CL-6).* Trace pattern choice (1/16 today, 1/8 = 2x1 checker, 1/4 = Ultra) + history weight; keeps
   S1's depth-window semantics. Ships on High only if `perf ab` keeps clouds + far fog ≤ 0.9 ms.
7. *Decks below the peaks (CL-7).* Fixed-altitude layers, no terrain anchoring (CLOUD-RESEARCH): one volumetric deck
   per climate (WE4b local base/thickness from the map); a low deck is a low base and thickness in that weather's
   preset, so summits poke through or stand in it. Genus presets gain the profiles; cumulus 2-4 km in game metres (Cloudy ~2 km, cumulus ~3.5 km; meta-4 2026-11-26: a look value
   the owner scales live in WE-20 through the per-weather `mw.cloud.<weather>.thickness_m` tunables, never asked; the game's
   thickness cap becomes `mw.cloud.max_thickness_m`); per-weather Morrowind values in `weather-adapter.md` "Cloud
   decks". A second simultaneous layer (stratus under cumulus) is not planned.
8. *Camera inside or above a deck (CL-8).* As built already: `shell_segment` (`cloud_common.wesl:267-293`) starts
   the march at the eye inside the envelope shell and at the top hit above it, ending at the base; the local deck
   comes per sample from the map, so density is integrated from the eye, never from the layer bottom; the march ends
   at scene depth, so near terrain inside a deck gets only the cloud in front of it. Added: `eye_in = ramp` of the
   eye's height fraction in the local deck at the camera column (map at the eye, soft 10 % edges) x local coverage.
   With `eye_in > 0`: steps start at `inside.near_step_m` and grow to `ds0` (`cloud_trace.wesl:587`) by
   `inside.near_m`; the resolve's fresh weight rises to `inside.history_weight` (near wisps parallax under one
   mean reprojection distance). Above the deck: the eye is outside the local deck, empty space is coarse-skipped to
   the top, tops lit by the short light march and the near-zero `od_up` (CL-4); the valley fog and WE4d far weather
   below sit behind the cloud through WE12-FIX, seen through holes only. Lamps do not light the deck (froxels do the
   near lit fog): kept gap.

**Against the ledger** (`docs/ledger/sky-weather.md`).
- WE12-FIX (fog split at the clouds' depth, `cloud_composite.wesl:181-196`, trace far-weather order `:729`): **kept**.
  Its two distances cover both new views: inside, `tc` is small and the fog to it is the near column (the cloud is
  integrated from the eye); above, the front fog is the thin air to the deck top and the valley fog is behind. No
  third case: inside a deck fog and cloud interleave, but one split errs by the fog/cloud extinction ratio at deck
  altitude (calc ≤ 6 %: rain fog 3 km at sea level, falloff 250 m, deck at 250 m, against 0.0075 /m cloud); CL-8's
  test bounds it against an interleaved reference.
- CLOUD-RESEARCH (fixed altitude layers, mountains poke through, low deck + fog under it): **kept and extended**:
  CL-7 brings decks below the summits by data; the separate low stratus layer (+0.2-0.4 ms) stays unbuilt.
- WE12-DIAG (clouds missing below the horizon with the camera above or in the deck): **closed by WE12-FIX**, extended
  by CL-8 (near steps, inside history weight). Its cited `cloud_composite.wesl:112-135` and fog.md:73-76 now hold the
  tap loop and the split description.
- Other cloud lines respected: S1 silhouette history (CL-6, CL-8 keep its window and distance accumulation); the
  2026-11-06 history weight 0.06 stays the default outside a deck; CW-1/S3 storm terms unchanged.

**Tunables** (framework `sky.cloud.*`, `Net::Presentation`, `Lifetime::Live` unless noted; *look* rows are
`Scope::Player` on the options page Image, section Clouds; the rest Advanced).

| Name | Default | Range, unit | Source | Row |
|---|---|---|---|---|
| `sky.cloud.noise.shape_tile_m` | 8 192 | 2 048-16 384 m, divides 16 384 (Reload) | Industry (Godot HZD 12.5 km → power of 2) | Advanced |
| `sky.cloud.noise.detail_tile_m` | 1 024 | 256-2 048 m (Reload) | Industry (Godot HZD 1 km) | Advanced |
| `sky.cloud.noise.curl_m` | 60 | 0-200 m | Guess | Wispiness (look) |
| `sky.cloud.lod.noise_mip_bias` | 0 | -1..3 levels | Guess | Advanced |
| `sky.cloud.lod.erosion_fade_start` | 1 | 0.5-4 ratio (footprint/texel) | Industry (HDRP 3 km, converted) | Advanced |
| `sky.cloud.lod.erosion_fade_end` | 8 | 2-32 ratio | Industry (HDRP 100 km, converted) | Advanced |
| `sky.cloud.ambient.scale` | 1 | 0-1.5 x | Industry (Hillaire 2016 5.5.1) | Cloud brightness (look) |
| `sky.cloud.ambient.floor` | 0.15 | 0-0.3 x | Guess (owner lit look) | Underside darkness (look) |
| `sky.cloud.ambient.profile_exp` | 0.5 | 0.25-1 | Industry (Nubis 2022) | Advanced |
| `sky.cloud.ambient.desaturate` | 0.2 | 0-1 | Guess (Hillaire control) | Advanced |
| `sky.cloud.light.eroded_samples` | 2 | 0-3 count | Industry (HDRP eroded march) | Advanced |
| `sky.cloud.light.far_sample_m` | 3 000 | 1 000-6 000 m | Industry (HZD far sample) | Advanced |
| `sky.cloud.silver_lining` | `strong` | strong / subtle | Owner 2026-11-26 | Silver lining (look) |
| `sky.cloud.phase.g_fwd` | 0.8 | 0.5-0.9 | Guess (HDRP 0.7 read, HZD ~0.8 unverified) | Advanced |
| `sky.cloud.phase.g_back` | -0.5 | -0.8..-0.1 | Guess (HDRP -0.7) | Advanced |
| `sky.cloud.phase.back_weight` | 0.3 | 0-0.5 | Guess (Hillaire w) | Advanced |
| `sky.cloud.sample.pattern` | `checker4` | checker4 / checker2 / half (Reload) | Industry (HZD 1/16) | Advanced (preset) |
| `sky.cloud.sample.history_weight` | 0.06 | 0.04-0.15 | Measured (ledger 2026-11-06) | Advanced |
| `sky.cloud.inside.near_step_m` | 4 | 1-50 m | Industry (Nubis 2022 3 m near) | Advanced |
| `sky.cloud.inside.near_m` | 200 | 50-1 000 m | Guess | Advanced |
| `sky.cloud.inside.history_weight` | 0.15 | 0.06-0.3 | Guess | Advanced |

Genus profile knots are preset data (records), not tunables. Switches (`--off <name>` = `set <tunable> off`):
`cloudnoise2` (CL-1/2, Reload), `cloudlod` (CL-3), `cloudambient` (CL-4), `cloudcone` (CL-4), `clouddecks` (game,
CL-7: old heights), `cloudinside` (CL-8).

**Slices** (one deliverable + its test; crates `sf_atmos` = CPU twin, `sf_atmos_bevy` = WESL/render). GPU ms:
RTX 4060 laptop, High, 1440p + DLSS Quality (traced at 1707x960), calc from research §6, verified with `perf ab`.
Today ≈ 0.3-0.45 ms clouds (trace 0.146 + resolve 0.075 measured 2026-11-06, composite calc) against the High row
"Clouds + far fog" 0.9 ms (weather.md budget) and the 8.3 ms frame.

| Slice | Deliverable | Crates | Depends | Test | GPU ms |
|---|---|---|---|---|---|
| CL-1 | noise v2 textures + upload (RGBA shape, curl) | sf_atmos, sf_atmos_bevy | - | exact: deterministic by seed, tiling seams equal, channel mean/range, remap bounds; `#[ignore]` gen time | +0.01-0.02 (RGBA fetch) |
| CL-2 | world-altitude shape, curl detail, genus profiles | sf_atmos, sf_atmos_bevy | CL-1 | twin golden profiles per genus; storm gate unchanged; shader compiles | +0.02-0.05 |
| CL-3 | mip + erosion fade by footprint | sf_atmos, sf_atmos_bevy | CL-2 | fade monotone, 1 below start, 0 past end; mip ≥ 0 and finite | -0.03 to -0.1 |
| CL-4 | occluded ambient + eroded cone light + far sample | sf_atmos, sf_atmos_bevy | CL-2 | ambient monotone in `od_up`, never below floor; light OD ≥ uneroded at equal point | +0.15-0.3 |
| CL-5 | phase tunables + `silver_lining` | sf_atmos, sf_atmos_bevy | - | phase integrates to 1 (both arms), forward peak > back; `subtle` = today bit-exact | ≈ 0 |
| CL-7 | genus presets, cap, Morrowind deck table | sf_atmos, mw_weather | CL-2 | exact preset values; `mw_weather` table test (decks below 271 m where listed) | 0 calm; towers +0.05-0.15 |
| CL-8 | eye-in-deck steps + history weight | sf_atmos, sf_atmos_bevy | CL-7 | twin `shell_segment` cases (inside → 0, above → top..base, up from above → empty); `eye_in` bounds; split vs interleaved reference ≤ 0.06 | 0 outside; see below |
| CL-6 | trace pattern + history weight tunables | sf_atmos_bevy | CL-8 | accumulation twin unchanged; `perf ab` ms per pattern (no pass/fail) | +0.2-0.4 at 1/8 |

Sum CL-1..5, 7: ≈ +0.15-0.4 ms → clouds ≈ 0.5-0.85 ms, inside the 0.9 row with no far fog headroom; CL-6 only if
measured to fit. **Inside-deck worst case** (CL-8, summit in an overcast deck): every traced pixel starts in cloud
(no empty skip), small near steps, full light near the eye; rays end at T < 0.01 within ~300-600 m (mean free path
70-130 m) in ≤ 64 steps: trace ≈ 1.5-2x a fair sky, clouds ≈ 0.7-1.0 ms (calc), at the row; CL-6's 1/8 drops to 1/16
while `eye_in > 0`. Each slice: `test -p sf_atmos -p sf_atmos_bevy`, clippy on both + dependents, a `perf ab` on
`exterior_red_mountain`, a ledger line; look slices end with their visual check (WE-18..WE-21) awaiting owner.
H3: no slice changes pass order, depth or the fog split.

### As built (summary)

First build (2026-10-12) and S1 (2026-10-13) in full: [archive/clouds-history.md](../archive/clouds-history.md).
- Crates: `sf_atmos` (Bevy-free: `CloudSettings` = `CloudWeather` + `CloudLook` + `CloudQuality`, nine `CloudPreset`
  genera, noise generators, density/weather CPU twin, `CloudScroll`), `sf_atmos_bevy` (`CloudsPlugin`, `Clouds`,
  `CloudCamera`, `CloudShadowLight`, `CloudsOrigin`; passes `clouds_shadow`, `clouds_trace`, `clouds_resolve`,
  `clouds_composite`, each a GPU span).
- No weather-map pass: a tiling weather-noise texture thresholded by the `CloudWeather` scalars (since WE4b the
  world weather map adds local layers). Noise made at startup off-thread (`atmos.noise` bake step not built).
- Cloud shadows: 1024² R8 toroidal, time-sliced map (High 16 slices) on the sun's tiled `DirectionalLightTexture`
  (needs `pbr_clustered_decals` + `pbr_light_textures`); out of range reads as mean cover.
- Cirrus: 2-D sheet at `cirrus_height` from the weather map's A channel. Sun disk: remainder snapped to zero below
  T 0.01 (Nubis).
- Game: `--off clouds`, `--clouds <genus>`, F10/C in the viewer; Morrowind weather → genus in weather-adapter.md;
  heights x0.5 (`mw_weather::CLOUD_WORLD_SCALE`), thickness cap 1 600 m (`MAX_CLOUD_THICKNESS_M`, WE-5).
- Cost (S1, 2560x1440 native, shared machine): fair 0.95 / cumulus 0.83 / overcast 0.61 / storm 0.71 ms total.
- Test scenes: framework `cargo run --release -p sf_atmos_bevy --example clouds -- --preset <genus>`; app `clouds-*`.

#### Storm cloud shape (S3, `cbshape`, 2026-11-13)

Storm-shape problem, reuse check, industry comparison and design: [archive/clouds-history.md](../archive/clouds-history.md#storm-cloud-shape-s3-cbshape-2026-11-13).

**As built (S3, 2026-11-13).** CPU twin `sf_atmos::density` (`CB_*`, `weather_at_shaped`, `storm_gradient`,
`combine_density_shaped`, `erode_density_shaped`, `shape_height`); GPU `cloud_common.wesl` (same names) and
`cloud_density.wesl` (`weather_sample_a`, `cloud_density`; the coverage early-out lets the anvil through).
`WeatherSample` carries `storm` (the gate) and `anvil`. Switch `sf_atmos_bevy::CumulonimbusShape` (uniform
`Clouds.cb_shape`, last field), game `--off cbshape`. Constants: `T_MIN` 0.35, `TOP_EXP` 1.5, cell shift 0.8,
`ANVIL_WIDEN` 0.15 (0.2 hung the shelf over cells 0.55, too far out), anvil 0.72 + 0.08, lent cover 0.6, rag below
0.1 with up to 2x erosion. Measured (fraction of the shape layer with density > 0.1, 1 600 m deck): old storm
cells >= 0.65 are full columns from h 0.15 to 0.95; new tops ~0.95 / 0.95 / 0.8 / 0.5 / 0 for cell values 1.0 /
0.85 / 0.75 / 0.65 / 0.55, the anvil shelf at h 0.85-0.95 over cells 0.55-0.65; toward the edge (cell 0.8) the
column stays full at storm 1, a 0.65 dome + shelf at 0.85, a low dome at 0.7, nothing by 0.55. The CW-1 rain
shading (`cloud_trace.wesl` far weather) still calls the unshaped `weather_at`. Cost: inferred ±0.1 ms (taller
cell cores, fewer rim hits), not measured. Tests: `storm_shape_leaves_calm_decks_unchanged` (bit-exact gate),
`storm_top_follows_coverage_and_anvil_spreads`, `storm_shape_is_finite_and_bounded`, shader
`storm_cloud_shape_compiles`. Owner check WE-15.


### Temporal accumulation as built (WE-10 fix 1, cloud-history session, 2026-11-06)

The resolve kept a 50/50 history (`mix(history, current, 0.5)`, ≈ 58 % of one sample's noise left) under
a 3×3 min/max clamp, with interleaved-gradient jitter; far-weather steps are often opaque, so a sample is a
coin toss and the deck and curtain shimmered on a still camera. Now (`sf_atmos::CloudTemporal`, CPU twin
`sf_atmos::temporal`, on the `Clouds` resource): fresh weight **0.06** (≈ 16 fresh samples, ≈ 64 frames under
the checkerboard), **variance clip** in YCoCg (+ alpha) to mean ± **1.5σ** of the same-population 3×3,
intersected with min/max and clipped toward the box centre (Salvi 2016; Pedersen 2016), and the march /
bank / far-weather jitter from **spatio-temporal blue noise** (NVIDIA STBN 128² × 32, the texture Bevy 0.20
ships in `bevy_pbr` behind `bluenoise_texture`, read from the vendored file so that feature's other users keep
their noise): one layer per visit of a half-res pixel, a toroidal offset per checkerboard slot (else the four
pixels of a block shared one jitter), a golden-ratio shift per 32-layer cycle. `CloudTemporal::LEGACY` is the
old resolve (the `cloud_history` A/B). Tuning (`scene we3-ridge`, ashstorm forced in the Ashlands, 1280×720,
mean |Δ| RGB between screenshots of a still view, 4 pairs, a debugging aid): legacy deck 3.7 / curtain edge 2.2;
weight 0.15 + 1σ 2.8 / 2.2; 0.1 + 1.5σ 2.05 / 1.5; **shipped 0.06 + 1.5σ 1.55 / 1.54** (legacy in the same run 4.15 / 2.24); a frozen history 1.24 / 1.26
(the floor: TAA jitter and reprojection). The frame-to-frame change scales with the weight, the clip width
matters little. Cost (`--bench exterior_red_mountain` at 1707×960, the DLSS Quality input of 1440p, ashstorm forced, Bevy GPU spans): trace 0.146 / 0.146 ms, resolve 0.075 / 0.074 ms (new / legacy): ≈ +0.001 ms.

#### Stable silhouette history (S1, `cloudedge`, 2026-11-13)

STORM-DIAG (2026-11-12): under the checkerboard the trace's 6×6 furthest-depth window was centred on the
sub-pixel traced this frame, so it moved two full-res pixels with each phase (on top of DLSS-jittered depth) and
a block on a silhouette flipped open/cut (or "cut at the ridge"/"cut at far land") frame to frame; each flip
rejected its history and the block showed one raw STBN sample: a strip along every silhouette that never
accumulated (the horizon shimmer where no fog is behind; WE-10's "shimmer behind fog and clouds"). As built
(`CloudEdgeStable`, `clouds.stable_edges`; CPU twin `sf_atmos::temporal::{depth_window, mismatch_fallback,
accumulate_distance}`): (1) the window covers the whole block, its 4×4 full-res pixels plus the one-pixel
bilinear reach of the composite (`4b−1 … 4b+4`, still 36 loads), and the open/cut class is taken along the
block's centre ray, both phase-independent; (2) a texel whose reprojected history held only the other
population shows the mean of its same-population 3×3 neighbours (colour, far bank, distance), not the raw
sample; (3) the stored cloud distance accumulates like the colour (history clamped to the neighbourhood's
range), so the composite's occlusion test at the horizon (grazing deck ≈ far-land distance) reads a settled
distance instead of the last jittered march; with (1) the composite's set pick (`d.z`) is stable too.
`--off cloudedge`: the old window, raw sample, last-sample distance. Industry: Nubis/HZD (Schneider 2015-2017)
and UE5 Volumetric Cloud reconstruct the cloud layer at a fixed low-res footprint (min/max depth of the
covered pixels) and reject history by depth discontinuity with a neighbourhood fallback, which is what this
does. Expected cost ≈ 0 ms (same loads; one extra shell intersection per traced texel). Risk: a little sky
bleeding at silhouettes from the wider far side of the window (WE-14).
