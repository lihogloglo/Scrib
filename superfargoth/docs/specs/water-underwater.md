# Underwater: camera medium, waterline, surface from below, lens droplets (M6 U-slices)

Status: U1 camera medium and U3 post (fully submerged) built (commit `86f4d9fb`); U4a-d, U2, W-LENS specced
2026-11-26 (this file); U5 caustics, U6 shafts planned. Parent spec: [water.md](water.md) (optics, water pass, budgets).
Research: `superfargoth/docs/research/water-line-2026-11-26.md` (waterline cause, Crest read, slices; owner answers §8).
Ledger: `docs/ledger/water.md` (U1, WA-10 diag, WATERLINE-RESEARCH lines). Wiki: `docs/wiki/engine/terrain-and-water.md`.
Crates: `sf_water` (Bevy-free: `medium`, `underwater`, new `waterline`, `lens`), `sf_water_bevy` (`medium.rs`,
`render.rs`, shaders `water_underwater`, `water_common`, `water_material`, new `water_lens`); game: tunable bindings only.

<!-- agent-summary -->
> **Agent summary (2026-11-26).** The camera medium is per view (U1); the post fogs a fully submerged view (U3). The
> waterline work makes it per pixel: a near-plane height grid from the drawn cascades (U4a), fog only below the line over
> min(scene, surface exit) from unfogged colour (U4b), a bright thin meniscus film with refraction above the line (U4c),
> the post's band driven by the sea's wave height (U4d), the surface seen from below with Snell's window and TIR (U2),
> lens droplets for ~4 s after the camera surfaces (W-LENS). Same rule in first and third person (the camera's near plane).
> Read next: "Canonical technique", then your slice's section, "Tunables", "Performance".
<!-- /agent-summary -->

## Goal

When the lens crosses the water, the frame splits along the drawn, displaced surface: fogged water below, air with its
sky above, a thin bright film on the line; no frame-wide flip, no dead zone, no line that disagrees with the waves. From
below, Snell's window and TIR; coming out, droplets that run down and clear. Owner answers (2026-11-26, research §8):
bright thin film + slight refraction just above the line; droplets yes, ~4 s on exit, downward streaks, default on,
switchable; same effect in first and third person (one near-plane rule).

## Design (moved from water.md "Underwater", 2026-11-26)

- **Camera medium** per frame: the query at the near plane's four corners and centre. All below →
  underwater; mixed → split; all above → above. Hysteresis 1 cm.
- **Medium**: the body's optics. Below the surface, in-scatter and transmittance follow the same
  formula as depth colour, with light attenuated by depth below the surface. **Froxel water medium**
  (High+): froxels below the water-level clipmap's surface get the water's extinction and albedo,
  lit through the surface (sun × transmittance to the surface × cascade-normal shafts), so light shafts
  under the waves come from the froxels like god rays above. **Low/Medium**: an analytic full-screen
  pass (homogeneous medium, distance from the depth buffer).
- **Surface from below**: the water material's back face: total internal reflection outside Snell's
  window (reflect the underwater colour/fog), refraction of the sky and scene inside it.
- **Waterline split**: in the underwater post, a per-pixel test of the near-plane point against the
  displaced surface (cascades sampled at that XZ) picks above/below, with a thin meniscus band (Crest's
  approach). No hard horizontal cut when waves cross the lens.
- **Caustics**: a tiling animated caustics atlas made by the `water.caustics` bake step (light refracted
  through a tiling wave normal field onto a plane, photon splatting; Guardado 2004), 32 frames, 256².
  The surface hook (terrain, `WorldMaterial`, characters) adds it on receivers below the water-level
  clipmap's surface, projected along the refracted sun, faded by depth (0.3-12 m), × sun visibility
  (CSM + cloud shadow). Because it runs in the opaque pass, the refracted view from above and the view
  from below both show it.
- **Exposure and cues**: no special exposure; the cue list gets `underwater` (the audio system applies a
  low-pass), `surface_break` on crossings.
- **Plan (2026-11-17, owner go; research `superfargoth/docs/research/underwater-2026-11-17.md`):** Crest's design in our
  own passes. *Reuse check:* Bevy 0.20 has no underwater; `FullscreenMaterial` has no depth input, `VolumetricFog`/`FogVolume`
  can't model a displaced infinite ocean, `DistanceFog` is per material; no crate does underwater (Crest 4 and Abyssal Garden,
  MIT, are references only). Changes to the design above: (a) sky, aerial perspective and the `sf_atmos` composite run after
  the water by depth, so they are **gated off** pixels whose ray starts underwater (a uniform when fully under, the waterline
  mask when split), or the view is fogged twice; (b) the back face samples the sky itself (the sky pass skips water depth);
  (c) **caustics are applied from depth** in the snapshot copy and the underwater post, not by a hook in every material;
  (d) froxel shafts are an additive in-scatter gain on the atmosphere froxel volume (spectral extinction stays analytic).
  Medium colour = the body's own region optics (`WaterLook`), so the swamp is murky and the open sea clear, no extra preset.
  The meniscus line is wanted (owner: "a midline"). Slices, one deliverable each: U1 camera medium (CPU, cues), U2 surface
  from below (Snell + TIR, `--off waterbelow`), U3 underwater post fully submerged (`--off underwater`), U4 waterline mask +
  meniscus (`water.waterline`), U5 caustics bake + apply (`--off caustics`), U6 froxel shafts High+ (`--off watershafts`).
  Deferred: lens droplets, bubbles/snow particles, fog on translucents underwater, swim physics.
- **Reference for the look (owner 2026-11-15):** [Abyssal Garden](https://github.com/Noniv/abyssal-garden)
  (MIT, one WebGPU HTML file, all procedural): shafts through a moving surface, caustics on the floor, kelp swaying
  in a current with sparkle specks, drifting particles and bubbles, fish schools and mantas, a generated underwater
  soundscape, swim collision that glides along rock, and Low/Medium/High tiers. Mine it for ideas and constants when
  3.6 starts (MIT code can be ported with a notice); calibrate against our own measurements.


The "Waterline split" bullet and the Plan's U2/U4 items above are made precise by the sections below (U4 splits into
U4a-d); lens droplets leave the deferred list as W-LENS (owner 2026-11-26).

## Canonical technique (waterline)

Crest, HDRP and the custom-pass fixes for UE5 converge (research §4.7):
1. Per pixel, decide whether the view ray **starts in water**: displaced height `h(xz)` at the pixel's near-plane point
   `P0` against `P0.y` (Crest rasterises the displaced mesh into a mask; ours: a near-plane height grid from the cascades
   the water vertex shader uses).
2. Keep `s = P0.y - h` (m); `d_px = s / |∇s|` is the signed pixel distance to the line (anti-aliased edge, exact band
   width; Crest keeps the sign and probes 3 neighbour pixels). Jitter moves it sub-pixel, smoothly.
3. Fog only `s < 0` pixels, over `min(scene distance, surface exit)`; sky, aerial perspective and the composite must
   not also fog them.
4. Meniscus: a band `|d_px| < w` (Crest 3 px), shaded and refracted.
5. Cheap path: near plane deeper than the maximum displacement → no per-pixel test (Crest `_FULL_SCREEN_EFFECT`);
   higher → nothing. The horizon fill Crest needs (far-plane quad) is moot for a near-plane test.

## Reuse check (Bevy 0.20 first, then crates, then ours)

- **Bevy 0.20.** `FullscreenMaterial` (`bevy_core_pipeline/src/fullscreen_material.rs`) binds only its uniform and the
  source colour: no depth, no extra texture, so neither the fog nor the lens pass's waterline mask fit. `bevy_post_process`
  (bloom, DoF, motion blur, auto exposure, effect stack in Core3d `PostProcess`) has nothing for droplets or waterline;
  `effect_stack/lens_distortion.rs` is the lens pass's template. `SkyOccluded` (vendored `bevy_pbr/src/atmosphere/mod.rs:224`)
  is per view only; `VolumetricFog`/`FogVolume` can't model a displaced infinite ocean. `ViewTarget::post_process_write`
  gives the source/destination flip the Split post and the lens pass use.
- **Crates.** `bevy_water` (0.17/0.18), `bevy-aqua 0.1` (0.19), `bevy_vfx_bag` (0.9/0.10): no waterline (research §3).
- **Ours.** Extend the U3 pass and the water material; Crest (MIT, `ff0602f0`) read as reference only.

## Industry comparison

| Aspect | Crest (MIT) | Unity HDRP Water | UE5 Water plugin | Ours | Verdict |
|---|---|---|---|---|---|
| Per-pixel "starts in water" | mesh mask R16F + horizon quad | `IsUnderwater` buffer | none: per-view post volume | analytic near-plane grid (U4a) | match (finer) |
| Fog distance | min(scene, ocean depth) | analytic / volumetric | volume | min(scene, plane exit) (U4b) | match |
| Sky / composite gating | by the mask | internal | none ("half in, half out" pops) | overwrite from the unfogged snapshot (U4b) | match (MSAA: small residual) |
| Meniscus | 3 px dark rim (x0.9 per tap) | "a boundary" | none | bright film + refraction (U4c) | match, owner look |
| Cheap path | `_FULL_SCREEN_EFFECT` | - | - | band from Hs (U4d) | match |
| Surface from below | TIR in the surface shader | yes | back face | Snell + TIR (U2) | match |

Sea of Thieves: no public source on the waterline (SIGGRAPH 2018 text covers FFT and foam). Lens droplets: wetness +
procedural drops with trails is the common form (Fluid Frenzy wet lens, Tatarchuk 2006 rain on glass); W-LENS matches.
UE5 is the anti-example. Gaps left: interiors read Above (pools not in the CPU query), no SSR of the underwater scene
under TIR (Crest has none either).

## Data flow today

1. `mw_environment_bevy/src/water.rs:219` `medium` → `WaterMediums::update` (`sf_water_bevy/src/medium.rs:274`): near-plane
   corners from the projection (NDC z = 1, reverse Z; near 0.1 m, all five points within ~0.14 m of the eye).
2. `sf_water/src/medium.rs:135` `CameraMedium::update`: all below → `Under`, all above → `Above`, else `Split`
   (`water.medium.hysteresis_m`, `sf_water_bevy/src/medium.rs:51`); cues at `sf_water/src/medium.rs:84`.
3. CPU height = cascade 0 + shore waves (`sf_water/src/query.rs:323-345`); the GPU displaces every cascade
   (`water_common.wesl:577` `displaced`, from `water_material.wesl:73`).
4. `post_on` (`sf_water_bevy/src/medium.rs:213-214`) only for `Under`: `Split` draws nothing; `extract` (`medium.rs:320`)
   inserts `UnderwaterUniform` + `SkyOccluded`, so sky, aerial perspective (vendored `atmosphere/node.rs:236-254`) and the
   composite (`sf_atmos_bevy/src/render.rs:1207-1213`) skip the whole view.
5. The post (`render.rs:382-384`, `PassStage::Medium` `sf_render_order_bevy/src/passes.rs:46`: after the composite,
   before transparent) fogs every pixel over the lens-to-depth distance (`water_underwater.wesl` `view_distance`).
6. From below the water material only flips the normal (`water_material.wesl:318`; cull off, `material.rs:231`).

## Frame after U4a-d, U2, W-LENS

CPU: medium + cues (U1) → waterline drive (U4d: off / waterline / deep) → lens wetness (W-LENS). Render, per water
camera: water draw (U2 back faces) → composite (unchanged) → `PassStage::Medium`: waterline grid compute (U4a, waterline
mode only) → underwater post (Split path: per-pixel; deep path: today's blend) → transparent → DLSS → `PostProcess`:
lens pass before bloom (W-LENS, only while wet) → tonemapping.

## U4a: waterline in the post (`Split` draws)

- **Grid.** A compute dispatch in the underwater node, before the post's draw, writes `WaterlineGrid`: an R16Float
  texture `grid_w × grid_h` (`grid_w` = `water.waterline.grid`, `grid_h` = ceil(grid_w / aspect)); texel `(i, j)` ↔ NDC
  `-1 + 2·i/(w-1)` (edges inclusive, so bilinear covers the screen). Per texel: `P0` = `world_from_clip · (ndc, 1)` with
  the frame's jittered view (it matches the depth buffer); invert the horizontal displacement by fixed-point steps
  `x0 ← P0.xz - D(x0).xz` (`water.waterline.iterations`, as `sf_water/src/twin.rs:386` on the CPU), then `h` = the
  displaced height at `x0` plus the shore-wave offset. New WESL `surface_at(params, x0, camera)` in `water_common.wesl`
  returns exactly what `water_material.wesl:66-73` computes (shore scales, bed slope, `shore_wave_offset`, `displaced`);
  the vertex shader calls it too, so the drawn surface and the grid cannot drift. The texel stores `s = P0.y - h`, not
  `h`: `P0.y` is linear across the near plane, so bilinear `s` is exact up to the surface's curvature (< 0.03 mm at 3 mm
  spacing, against ~0.12 mm per pixel on the 0.1 m near plane), and half floats are most precise near the line (s ≈ 0).
- **Body.** The post binds its own copy of the camera body's `WaterParams` (`params`, `sf_water_bevy/src/lib.rs:988`; the
  body `body_look` picks) and the shared cascades, sampler, shore field, bed. Flat bodies skip the dispatch (`h` = level).
- **Post.** Per pixel: `s` (grid, bilinear), `d_px = s / max(length(vec2(dpdx(s), dpdy(s))), ε)`
  (derivatives before any branch), `under = saturate(0.5 - d_px)` (1 px ramp). The fog of U3 is weighted by `under`
  (dual-source: in-scatter × under, transmittance `mix(1, T, under)`).
- **Drive.** `post_on` is true for `Split` and `Under`; `SkyOccluded` only for `Under`. U4d refines this with the band.
- **Switch.** `water.waterline` (FeatureSwitch "Waterline", `Effect::Skips`, `Cost::Gpu("water_underwater")`); off =
  today's per-view rule (`Split` draws nothing).
- **Scene.** `wa-waterline` in `bevyrrowind/scenes.ron` (Seyda Neen dock end, camera ~0.1 m above the water, noon, WA-11).
- **Test.** `sf_water::waterline::waterline_s(P0, height_fn)` and `grid_ndc(i, w)`, exact: flat surface (`s` = y − level);
  a camera pitched 30° and rolled 20° over flat water (rows above/below the analytic line); a sine wave across the lens
  (sign changes where `sin` crosses); all above, all below; grid edges at NDC ±1. Shader test: the post and
  `water_material.wesl` both call `surface_at`, the grid layout matches its Rust side; medium test: `post_on` in `Split`.

## U4b: no double fog, plane exit distance

- **Exit.** Fog distance `d = min(d_scene, t_exit)`, `t_exit = (h - P0.y) / dir.y` when `dir.y > ε` and `s < 0`, else ∞
  (`h - P0.y = -s` from the grid: the exit plane is the local surface). Covers rays with no surface depth (skirt, mesh-body edges,
  gate off). In `Split` and `Under`.
- **Source.** In `Split` on a non-MSAA target the post becomes a replace pass (`post_process_write`, source →
  destination): a below-line pixel whose snapshot depth (`PassStage::Snapshot`, opaque only) equals the live depth (no
  water in front) is fogged from the **snapshot colour** (before the composite: no air fog on a water path); where the
  water is in front (surface seen from below) it uses the live colour (the composite fogged only the eye-to-surface
  path, metres of air). Output `mix(live, fogged, under)`. Snapshot absent (gate off) → live colour.
- **MSAA views** (the Low preset) keep the blend path: the residual is the composite's air fog over the water segment.
- **Deep `Under`** keeps `SkyOccluded` and the blend path (exact there). The `sf_atmos` composite and Bevy's sky are not
  edited (another lane).
- **Test.** `sf_water::underwater::fog_distance(scene_d, s, h, P0, dir)` and `below_source(live_depth, snap_depth)`,
  exact: upward ray from 1 m below, plane exit at 1/dir.y m; downward ray → scene distance; sky (∞) looking up → exit;
  sky looking down → ∞ (medium colour); equal depths → snapshot, water nearer → live.

## U4c: meniscus band

- **Film** (owner: bright thin film). `f = (1 - smoothstep(0, meniscus_px, |d_px|)) × film_strength`; film radiance =
  (ambient + sun at the surface, the U3 uniform) × exposure × `mix(1, medium albedo, film_tint)`; colour
  `mix(c, film, f)`, after the fog.
- **Refraction above the line** (owner: slight). For `0 < d_px < refract_band_px` the source is sampled at
  `uv - n̂ · refract_px · (1 - d_px / refract_band_px)²` (`n̂` = the screen direction of `∇s`, toward the line): the film
  magnifies the strip just above it. Needs the source read, so replace path only (non-MSAA); `refract_px` 0 on
  Low/Medium (`sg.water`) to stay in budget.
- **Test.** `sf_water::waterline::meniscus_weight(d_px, w, strength)` and `refract_offset(d_px, band, px)`, exact at
  `d_px` = 0, ±w/2, ±w, beyond; offset 0 below the line and at `band`, `px` at 0+.

## U4d: CPU/GPU agreement, band from the wave height

- **Band.** `band = crest_hs × Hs + shore_amp + margin_m`: `Hs = 4·√(Σ_c variance_c) × WaveTerms.amplitude` from the
  generated cascades (`sf_water/src/spectrum.rs:391` `realized_variance`, recomputed when the sea wind regenerates the
  spectrum), `shore_amp` the shore waves' offshore amplitude. Flat bodies: Hs = 0, band = margin.
- **Drive** (pure `sf_water::waterline::drive(gap_lo, gap_hi, band) -> {Off, Waterline, Deep}`; gaps = lowest and
  highest near-plane point minus the CPU height): `gap_lo > band` → Off (no post); `gap_hi < -band` → Deep (U3 blend +
  `SkyOccluded`); else Waterline (grid + per-pixel, no `SkyOccluded`). So the post also runs where the CPU says
  `Above`/`Under` but chop it does not model crosses the lens. `MediumState` and its cues keep their duties (audio
  low-pass, optics, `surface_break`); the GPU test is the truth for the picture.
- **Test.** `drive` exact for the three cases and both band edges; `Hs` of a single-mode synthetic spectrum = 4·√m0;
  band of a flat body = margin.

## U2: surface seen from below (Snell window, TIR)

- In `water_material.wesl`'s `!is_front` branch (`:318`): `N_b` = the flipped normal (into the water), `I` = view ray
  (upward); `t = refract(I, N_b, 1.333)`; critical angle asin(1/1.333) = 48.6°. Exact unpolarised dielectric Fresnel `R`
  from the water side (cos_i, cos_t), `R = 1` under TIR.
- Transmitted `(1 - R)`: air-side radiance along `t`: the snapshot refracted in screen space (as the front face does)
  where its depth is behind the surface (docks, hulls above water), else the environment map / reflection probe along
  `t` (the sky pass skips water depth: Plan (b)).
- Reflected `R`: the medium's radiance at infinity (the post's in-scatter with `T = 0`, WESL twin of
  `sf_water::underwater_colour`, shared in `water_common.wesl`). Foam stays; SSPR and sun glitter are skipped.
- Output is radiance at the surface; the post fogs it by the eye-to-surface distance (U3/U4b).
- Switch `water.below` (`--off waterbelow`; off = today's flipped-normal front shading).
- **Test.** `sf_water::underwater::snell(cos_i)` exact: normal incidence `R` = ((1.333-1)/(2.333))² = 0.0204, transmitted
  straight up; 48.5° transmits, 48.7° TIR (`R` = 1); `R` continuous and rising to 1 at the critical angle.

## W-LENS: lens droplets after surfacing

- **State** (`sf_water::lens::LensWetness`, Bevy-free; `WaterLens` component on each `WaterCamera`, updated after the
  medium from its cues): under water `wet = 0`, `under_s += dt`; on `underwater: Some(false)` `wet = clamp(under_s /
  full_after_s, min_wet, 1)`, `under_s = 0`; above, `wet -= dt / duration_s` to 0 (owner: ~4 s). First and third person
  alike (the camera's cue).
- **Pass** (own node in `sf_water_bevy`, `water_lens.wesl`): Core3d `PostProcess` before `bloom` (after DLSS: output
  resolution, unjittered, so DLSS never sees the drops; HDR, so refracted highlights bloom); `post_process_write`.
  Extracted only while `wet > 0` and the switch is on. Procedural hashed cells (`density` across the screen height,
  aspect-correct): static beads plus running drops that slide down at `trail_speed` with a bead trail above them
  (owner: downward streaks); a cell's drop lives while its hash > 1 − wet, so drops clear one by one; hemisphere height
  → normal; colour = source at `uv + normal.xy × refract` with a 4-tap blur of `blur` inside drops; time from globals.
  While the view is in Waterline mode the drops are masked to `s > 0` (the grid holds `s` per NDC, readable at
  any resolution). `FullscreenMaterial` doesn't fit (no extra bindings).
- **Switch** `water.lens` (FeatureSwitch "Lens droplets", `Cost::Gpu("water_lens")`, `Scope::Player`, default on).
  Options row "Lens droplets" (on/off, graphics page) once tunables.md open question 1 allows per-feature rows;
  until then the feature switch and console.
- **Test.** `LensWetness` exact: 2 s under then out → 1.0; 0.1 s dip → `min_wet`; linear to 0 at `duration_s`; going
  under clears; switch off → never wet.

## Tunables

Declared like `sf_water_bevy/src/medium.rs:51` (`def!` with unit, range, source); host-bound live; census must pass.
The three switches are also live feature-switch rows (ROADMAP 1.28 registry) and game `--off` parts
(`mw_game_core/src/perf.rs:312`, `mw_devtools_bevy/src/part_switches.rs:161` style): `waterline`, `waterbelow`, `lensdrops`.

| Name | Unit | Range | Default | Source | Slice |
|---|---|---|---|---|---|
| `water.waterline` | bool | - | on | Owner 2026-11-17 ("a midline") | U4a |
| `water.waterline.grid` | count | 16-256 | 64 | Guess (~3 mm on the near plane) | U4a |
| `water.waterline.iterations` | count | 1-4 | 3 | Industry (HDRP, Crest; CPU twin uses 3) | U4a |
| `water.waterline.meniscus_px` | px | 0-8 | 3 | Industry (Crest 3 px) | U4c |
| `water.waterline.film_strength` | ratio | 0-1 | 0.5 | Guess (owner: bright thin film) | U4c |
| `water.waterline.film_tint` | ratio | 0-1 | 0.3 | Guess | U4c |
| `water.waterline.refract_px` | px | 0-6 | 2 | Guess (owner: slight) | U4c |
| `water.waterline.refract_band_px` | px | 0-24 | 8 | Guess | U4c |
| `water.waterline.crest_hs` | ratio | 0.5-2 | 1.0 | Industry (Rayleigh: max crest ≈ 0.93 Hs per 1000 waves) | U4d |
| `water.waterline.margin_m` | m | 0-0.5 | 0.1 | Guess | U4d |
| `water.below` | bool | - | on | Owner 2026-11-17 (Plan U2) | U2 |
| `water.lens` | bool | - | on | Owner 2026-11-26 | W-LENS |
| `water.lens.duration_s` | s | 1-10 | 4 | Owner 2026-11-26 ("~4 s") | W-LENS |
| `water.lens.full_after_s` | s | 0.1-5 | 1.0 | Guess | W-LENS |
| `water.lens.min_wet` | ratio | 0-1 | 0.3 | Guess | W-LENS |
| `water.lens.density` | count | 4-40 | 14 | Guess (cells across the screen height) | W-LENS |
| `water.lens.refract` | ratio | 0-0.02 | 0.005 | Guess (screen heights) | W-LENS |
| `water.lens.blur` | ratio | 0-0.005 | 0.0015 | Guess (screen heights) | W-LENS |
| `water.lens.trail_speed` | ratio | 0-2 | 0.35 | Guess (screen heights per s) | W-LENS |

Look values stay `Dev` (no own options row: tunables.md open question 1, groups only); `refract_px` is an `sg.water`
member (0 on Extremely Low to Medium); `water.lens` is `Player` and gets a graphics-page row "Lens droplets" once per-feature rows
are allowed. Constants (maths): n = 1.333, ε guards, the 1 px edge ramp.

## Performance

Estimates (research §6, RTX 4060 laptop, High, 1440p + DLSS Q, internal ~1706×960); each lands with an `sf_diag` GPU
counter (`water/underwater/gpu_ms`, `water/lens/gpu_ms`) and a `perf ab` arm on its switch. Against water.md
"Performance budgets": the "Underwater (when below)" row is 0.1 ms (Low/Medium) and 0.2 (High: analytic 0.1 + the
unbuilt U6's +0.1).

| Slice | When | Est. GPU ms | Budget |
|---|---|---|---|
| U4a grid + per-pixel test | Waterline mode only (seconds per crossing) | U3's ~0.1 now also runs here; grid < 0.01, test +0.01 | see total |
| U4b exit + snapshot source | Waterline mode, non-MSAA | +0.01 (two fetches; the replace pass reads what blending read) | see total |
| U4c film + refraction | Waterline mode | ~0 film; refraction +0.01 (High/Ultra) | see total |
| U4d drive | CPU | ~0 | - |
| U2 back face | surface seen from below | ≤ +0.03 net (replaces SSPR, probe and glitter work) | inside the water draw row |
| W-LENS | ≤ 4 s after an exit, output 3.7 MP | 0.15-0.25 | transient; 0 dry; no steady row |

Waterline-mode total: Low ≈ 0.11 (MSAA: blend path), Medium ≈ 0.12, High/Ultra ≈ 0.13 ms, against 0.1 / 0.1 / 0.2 /
0.25: Low and Medium run 0.01-0.02 ms over their row only for the seconds the lens straddles the surface (accepted:
under the 0.5 ms noise floor, nothing steady). Steady states are unchanged: Above (Off) pays nothing, deep Under pays U3.

## Tests

One Bevy-free exact test per slice (in its section) plus the naga shader tests per pass (layouts, the shared
`surface_at`). No picture is judged by a test.

## Verification (owner visual checks)

- **WA-11** waterline (after U4a-c, U4d, U2): `scene wa-waterline`, calm and `--weather thunderstorm`, first and third
  person, A/B `--off waterline`, `--off waterbelow`.
- **WA-12** lens droplets (after W-LENS): same scene, surfacing after 2 s and after a short dip, A/B `--off lensdrops`.
- Entries: `bevyrrowind/docs/visual-checks/terrain-water.md`. DLSS may smear a fast-moving line (research §7 note);
  if WA-11 shows it, a follow-up slice moves the meniscus after the upscale.

## Later

Interior pools in the CPU query (interiors leave `Above`), U5 caustics, U6 froxel shafts, bubbles and particles, fog on
translucents underwater, swim physics (3.6).
