# Morrowind lighting rules in openmw-rtx: what to adopt (research, 2026-11-05)

Scope: research only, no code. Shadow quality is out of scope (owner's H2). Sources: openmw-rtx (GPLv3,
`github.com/xorza/openmw-rtx` @ `f2f8b2d5b2`, called "rtx" below; paths are in that tree), OpenMW upstream
(`gitlab.com/OpenMW/openmw` master @ `ca5e2576`, "upstream"), our repo. Frame target: High, 1440p + DLSS Q, 4060 laptop,
<= 11 ms. All rtx figures are from a 4090 laptop and are not ours.

## 1. Summary (ranked)

Headline: rtx's falloff is NOT different from ours. Its `falloff()` is the Frostbite/UE4 window
`(1-(d/R)^4)^2 / (d^2+s^2)` (`components/rtxvulkan/shaders/lib/lights.glsl:166-173`) and Bevy's is the same
(`superfargoth/vendor/bevy_pbr/src/render/pbr_lighting.wesl:133-137`). What differs is the reach policy, the
ambient, and a few Morrowind-content rules. rtx can afford a long reach because it ray-shadows every lamp
(one stochastic lamp per pixel, reservoir pick, `lights.glsl:269-282`); we shadow 8 and leave about 18 of 26 lamps
in the Census office lighting through walls (`.sessions/gi-weak.md`, cause 1b). The rtx rules that fix that problem
are the ones that do not need ray tracing.

| # | Rule | Verdict | High cost | Why |
|---|---|---|---|---|
| 1 | Reach tied to shadowing: unshadowed lamps get a short reach, shadowed ones a long one | **adopt** (as policy) | negative (fewer lights in clusters and GI list) | Addresses gi-weak cause 1b directly; same conclusion as lights.md:61-64 (Clockwork 1.0 x) |
| 2 | Cell ambient is an occludable fill with a ~2 m look-out, not a flat add | **adapt** | 0 (GTAO exists) | gi-weak cause 1a is already fixed by `ambient_scale` = 0 in the GI box; the rule matters for the no-GI path (Low, outside the box) |
| 3 | Lamp body never lights twice (model glow vs its own light) | **adopt** for GI hits | ~0 | Our probe hits shade emissive AND the lamp's point light; rtx removed it to kill fireflies |
| 4 | Lamp source size = radius/16, softened singularity | **adopt** (trivial) | 0 | We hard-code `radius: 0.05`; owner sees the look |
| 5 | Vanilla flicker band 0.25..1.0 at 15 Hz, pink-noise flame | **adapt** (owner taste) | 0 | Ours dips only to 0.65; vanilla lamps average ~0.62 |
| 6 | Distant lamps keep lighting regardless of the object size rule | **verify, mostly have** | 0 | `FarLightsNight` sprites exist; check coverage of lit-window geometry |
| 7 | `NightDaySwitch` children chosen by mode (Default / ExteriorNight / InteriorDay) | **adapt**, low priority | 0 near, bake cost far | Mod-only feature (upstream docs: "Some mods add models"); we show the authored child only |
| 8 | Negative lights as take-away lamps in a separate grid run | **skip for now** | n/a | Rare (Census office has one `dark_128`); no raster equivalent in Bevy |
| 9 | Sprite-light bake, additive-sheet glow lamps (spell bursts) | **skip** | n/a | Sprite-light is 2D directional shading of flame sprites, not emission; glow lamp is RT-specific; spell VFX not on the roadmap lane |
| 10 | Light list sorted by position each walk; light grid binned by reach | **skip** | n/a | Bevy 0.20 GPU clustering already does this (lights.md:19-26); rtx's own cost note: 17 us/frame |

## 2. Per rule

### 2.1 Reach follows shadowing (adopt)

- **rtx:** reach = `radius*2 + 128` units, intensity `colour * r^2 * 0.25*pi`
  (`components/rtx/mirror/lightbuilder.cpp:33,40-41,198-202`). The comment says why: Morrowind radii (64..256 u) were
  authored for a flat falloff plus an ambient term, "here the lamps have to be what lights the place"; the flat +128
  keeps candles from vanishing while lanterns grow only a little. Every lamp is shadow-traced, so the long reach does
  not light through walls. Cap per point: 256 lamps (`lights.glsl:119-123`). Note the pool is cut at the reach, so
  "reach is where the pool is cut, not where it fades" (`lightbuilder.cpp:59-65`).
- **Upstream:** fixed-function style attenuation from the `Fallback` constants, linear and quadratic radius
  multipliers (`components/sceneutil/lightutil.cpp:46-90` in rtx's copy; upstream `lightutil.cpp`), minimum radius
  16 (`:122-128`).
- **Us:** `range = radius * 4.0` (`bevyrrowind/crates/mw_assets/src/lights.rs:63,93-95`), Phi = 8000 r^2 lm; only 8
  shadowed lamps at 512^2 (`mw_environment_bevy/src/render.rs:24-28`), faded 35..45 m (`:32`).
- **Gap:** reach and shadow budget are decoupled. A 128 u (1.83 m) lamp reaches 7.3 m through walls, where rtx
  reaches 5.5 m but with a ray. gi-weak measured the through-wall light (about 49 E/pi average) as ~5-10x the
  bounce it is meant to show.
- **Proposal:** `LightCalibration::point_light` takes `shadowed: bool` (generic part in `sf_sky_bevy`
  `BudgetedShadow`/`PointShadowBudget`; the numbers stay in `mw_assets`): unshadowed lamps `range_scale` about 1.5
  (OpenMW GL 1.75, Clockwork 1.0, rtx pool-edge window), shadowed ones keep 2-4. The 3/4-radius rule in lights.md:63
  means the interior pool inside 0.75 r does not change. Because the window gives exact zero at range, there is no
  ring if the change fades with the shadow fade (35..45 m). Cost: fewer lights per cluster; also fewer lights pass
  `select_nearest` for the GI list (`gi.md:163`, cap 64).

### 2.2 Ambient as an occludable fill (adapt)

- **rtx:** keeps the cell `AMBI` as a stand-in for every bounce past the first traced one (`shading.glsl:401-418`
  `pathEnd`) and multiplies it by one cosine-weighted occlusion ray of length `ROOM_FILL_REACH` = 140 u = 2 m in
  rooms (`shading.glsl:541-620`, `components/rtx/shaders/look.h:520-526`). Rationale (their words): walls are not
  occluders of it, they are what makes it; only furniture within 2 m is. Settings text: "lights a room by the ambient
  its record states" (`docs/source/reference/modding/settings/rtx.rst:117`). It does not delete the ambient; it AO-gates it.
- **Upstream:** ambient is a uniform added to every fragment (`SceneUtil::createLightSource` hands lamps no ambient;
  cell ambient is the `LightManager` sun-ambient, `lightmanager.cpp`).
- **Us:** the flat `GlobalAmbientLight` at `ambient_nits` 300 x cell colour (`lights.rs:64,82`); inside a GI volume it is
  scaled by `GiVolume::ambient_scale`, game value 0 (`gi.md:130-133`, `mw_exterior_bevy/src/gi_volume.rs:107`).
  gi-weak cause 1a is therefore handled; the owner confirmed GI-3.
- **Gap/adapt:** rtx's reasoning shows what ambient 0 loses: the multi-bounce tail. Our probes carry multi-bounce via
  hysteresis (`gi.md:97`), so 0 is right where probes exist (DDGI/Lumen practice). Remaining uses: (a) no-GI path
  (Low preset, outside the box, no RT hardware): keep ambient but AO-gate it with GTAO radius about 2 m (our GTAO
  already acts on indirect light only, `gi.md:14`; check its radius in the interior camera); (b) a small floor
  (rtx has none; Clockwork `minimum interior brightness` 0.08, research `clockwork-code-2026-10-25.md` row 12) so
  probe-less nooks are not black. Cost: 0.

### 2.3 Lamp body does not light twice (adopt for GI)

- **rtx:** `SceneUtil::LampBody` marks every drawable under a light's group as that light's model; the renderer
  must not bounce the model's glow because "the light already delivers what the model glows with"
  (`components/sceneutil/lampbody.hpp:15-21`, `lightbuilder.hpp` `givesLight`). Look.h:396-401: "A lamp's own model
  does not light even so: its LIGH lamp already lights for it". The architecture doc credits this for letting
  ReBLUR's anti-firefly stay off (`docs/rtx/architecture.md:420-422`).
- **Upstream:** nothing; a rasteriser never bounces.
- **Us:** probe hits take "albedo and emissive from the hit" plus direct from our light list (`gi.md:92-93`). A lantern
  mesh with `emissive_nits` therefore adds its glow to probes AND its `PointLight` does too. Visibility rays stop
  `shadow_map_near_z` short of the lamp (`gi.md:168`), so the cage can also block its own light (gi-weak cause 4).
- **Proposal:** the game marks meshes under an `AttachLight` node (`AttachLight` exists, `lights.rs:8`) with a
  `LampBody`-style marker; `sf_gi_bevy` hit shading skips emissive for marked meshes. Generic flag in `sf_gi`
  ("emitter already represented by a light"), set by `mw_assets` at spawn. Cost: one material/instance bit.
  Visual effect on the raster pass: none (the body still glows on screen).

### 2.4 Source size and singularity (adopt, trivial)

- **rtx:** source radius = record radius / 16 (4..16 u for 64..256 u), used for penumbra and as the falloff
  softening `max(source, 1 u)` so no hot bead appears beside a flame (`lightbuilder.cpp:43-48,205`;
  `lights.glsl:147-163`).
- **Us:** `PointLight.radius = 0.05` constant (`lights.rs:100-105`). Scaling it as `info.radius/16` gives 0.06..0.23 m.
  Affects specular hotspot and Bevy's sphere-light diffuse softening; zero cost. Taste: owner.

### 2.5 Flicker and pulse statistics (adapt, owner taste)

- **Upstream:** `LightController` walks brightness inside 0.25..1.0 at 15 ticks/s, speed 0.1 (flicker/pulse) or 0.05
  (slow), flicker re-rolls a target in 0.25..1.0, pulse alternates 0.25/1.0 (`components/sceneutil/lightcontroller.cpp:16,49-66`).
- **rtx:** same band and mean 0.625, but shaped as four golden-ratio-spaced sines from 9 Hz down (pink spectrum),
  pulse a triangle; deterministic per id (`lightbuilder.cpp:69-136,262-284`).
- **Us:** `1 - depth*dip` with depth 0.35 (flicker), 0.5 (pulse), rates 9/3/1/0.3 Hz (`mw_world/src/lib.rs:73-90`): range
  0.65..1, mean about 0.8 for flicker. So our flames are about 25% brighter on average and less dramatic than vanilla.
  Note the calibration (`lumens_per_radius_sq` 8000) was set against steady lights; changing depth shifts the average
  lamp. Option: keep our value-noise but set depth 0.75 and divide by the mean so average power stays; owner decides.
  Per-frame `intensity` writes already re-extract each frame (lights.md:33,93), no new cost.

### 2.6 Distant lamps vs the size rule (verify)

- **rtx:** the cell ring stands every `LIGH` of every shown cell regardless of distance; only reflecting statics are
  thinned by `mRadius >= threshold` (`components/rtx/mirror/cells/cellplacer.cpp:723-735` vs `:749-767`);
  `docs/rtx/architecture.md:246-248`: "the size rule thins what reflects light and never what gives its own: a lit
  window is a point of light long after it is a pixel".
- **Us:** far-field L1-L3 carry "light sprites" fed by `LIGH` records: color, intensity `(radius/4).clamp(0.3,2)`,
  radius clamp 1..12 m, negative/off-by-default dropped (`mw_bake/src/far.rs:99-115`, `far-field.md:229`, runtime
  `sf_lod_bevy/src/runtime.rs:665-700`, `FarLightsNight`). That already is the rule for lamps. Gap: emissive geometry that
  is not a `LIGH` (lit windows, glowing mushrooms, Dwemer glass) is merged into proxies with vertex-colour albedo and
  loses its glow when the object drops under 2 px (far.rs:12-13). Needs a census of which models carry emissive +
  no LIGH before deciding. Candidate: bake an "emissive point" per such placement into the light batch (same
  `FarSource::light` hook, game-side policy in `mw_bake`). Cost: sprites only.
- Open: do flickering lamps in the near ring and the far sprites agree (same hour switch)? Not checked.

### 2.7 NightDaySwitch (adapt, low)

- **Upstream:** NiSwitchNodes named `NightDaySwitch` get a callback that shows child `mode` (0 Default, 1 ExteriorNight,
  2 InteriorDay), or child 0 if fewer children (`apps/openmw/mwrender/animation.cpp:109-123,2126` rtx copy;
  `components/misc/constants.hpp:40`; mode from `WeatherManager`: exterior and not day -> ExteriorNight; interior at
  day with weather glare view >= 0.5 -> InteriorDay, `apps/openmw/mwworld/weather.cpp:838-850`). Setting `day night
  switches` default true; doc: "Some mods add models ..." so this is mod content, not vanilla.
- **rtx:** reads every branch for far cells and records which modes show each part (`components/rtx/mirror/cells/nightday.hpp`,
  `templatewalk.cpp:41-57`) so a ring-stood building shows the right child.
- **Us:** `mw_nif` stores `NodeKind::Switch { active }` and shows only `i == active` (`mw_nif/src/lib.rs:410-422`); the
  bake flags it `Dynamic::Switch` (`mw_bake/src/importer.rs:146`). No day/night mode anywhere (grep: none).
- **Proposal:** generic "named switch variants" in `sf_model` (keep all children, tag by variant index), the adapter
  (`mw_assets`) maps the node name `NightDaySwitch` and chooses a child from the environment (sun below horizon and
  exterior; interior flag). Hysteresis at dusk to avoid popping while looking at a window. Far field: bake the night
  variant as an extra light-sprite batch. Do the census first (how many Morrowind.esm/Tribunal/Bloodmoon models and
  popular mods carry the name) because vanilla may have zero.

### 2.8 Negative lights (skip)

- rtx builds them as lamps of negative intensity, stored in a separate "takes light" run of the same grid
  (`lightbuilder.cpp:190-196`, `lightgrid.hpp:70-77`, `lights.glsl:85,113`); upstream sets `diffuse *= -1`
  (`lightutil.cpp:134-139`). We drop them (`mw_world/src/lib.rs:179-185`; far.rs:102). Bevy's clustered forward has no
  subtract path and negative candela is untested. If dark spots matter later, a cheaper stand-in is a local
  ambient/AO volume or a decal; log it, do not build now.

### 2.9 Other rtx items, skipped

- `spritelight.comp` bakes four directional light-propagation passes of a sprite's own texture (`shaders/shared/spritelight.h:1-24`):
  it lights particle sprites; it does not make flames emit. Flame emission is the `LIGH` lamp, plus for spell
  bursts a derived "glow" fill lamp (`lightbuilder.hpp:96-160`, gain 4, reach 16 radii). Not relevant until spell VFX.
- Material glow `EMISSIVE_INTENSITY` = 8 chosen "against rendered frames", "a glow has no lamp of its own"
  (`look.h:404`): same conclusion as ours (emissive is shown and bounced, not turned into a lamp per mesh).
- Vanilla textures are pre-lit; rtx's `ShadingMap` estimates painted light and divides it out
  (`architecture.md:22-24,267`). Out of scope here, but it is the same issue as brown albedo 0.12-0.2 in gi-weak cause 2;
  worth a separate research slice (does our albedo include painted light?).

## 3. Industry comparison

- **Falloff window:** Karis, "Real Shading in UE4" (2013), and Lagarde and de Rousiers, "Moving Frostbite to PBR"
  (2014), both use `saturate(1-(d/r)^4)^2/(d^2+1)`. rtx, Bevy and Godot-style engines match.
  https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf ,
  https://seblagarde.files.wordpress.com/2015/07/course_notes_moving_frostbite_to_pbr_v32.pdf
- **Many lights:** clustered shading (Olsson 2012, http://www.cse.chalmers.se/~uffe/clustered_shading_preprint.pdf) is what Bevy does;
  stochastic one-light-per-pixel selection with reservoirs is ReSTIR DI (Bitterli 2020,
  https://research.nvidia.com/labs/rtr/publication/bitterli2020spatiotemporal/): the technique behind rtx's lamp pick; it
  is the Ultra (Solari) answer, not a High one.
- **Flat ambient replaced by indirect:** DDGI integrations and Lumen drop the flat sky/ambient term where probes cover
  (DDGI: https://jcgt.org/published/0008/02/01/ ; Lumen: https://dev.epicgames.com/documentation/en-us/unreal-engine/lumen-technical-details-in-unreal-engine).
  rtx's variant (AO-gated ambient as the path-end stand-in) is the classic "ambient with occlusion" for engines
  without GI. Our own lights.md:55-72 already lists per-light draw distance and shadow fade (UE `MaxDrawDistance`,
  Godot `distance_fade_*`, HDRP fade distance) and far-lights-as-sprites.
- **Emissive as light:** Lumen treats emissive meshes as lights only through the GI path, not the direct pass, and
  excludes nothing explicitly; the "don't double count a lamp's own glow" step is a content convention each team
  sets (rtx's `LampBody`). Not verified for Bethesda, RAGE or Decima; no public source found, so no claim.
- **Distant lit windows / lamps as point sprites:** we cite only our own spec (far-field.md:41-43) and rtx
  (`architecture.md:246-248`); I did not find a verifiable public URL for UE/RAGE/Decima practice in this pass.
- **Day/night swapped meshes:** common in games (lit window material swap by time-of-day parameter, UE material
  parameter collections), but I have no cited source; the Morrowind mechanism is upstream's `NightDaySwitch` above.

## 4. Slices (one deliverable each, in order)

1. **L-A: reach by shadow state.** `LightCalibration::point_light(info, shadowed)`; `PointShadowBudget` assigns before
   spawn (or a system re-ranges when the shadowed set changes). Test: unit (range for both states, exact). Measure:
   `lights` ms on Balmora, Census office, before/after (printed, not asserted). Owner visual: LT-new.
2. **L-B: lamp body flag for GI hits.** `sf_gi` flag + `mw_assets` marker under `AttachLight`; test: hit shading skips
   emissive for a flagged mesh (exact). Re-check cause 4 (opaque cage) with the same slice only if it is one line.
3. **L-C: source size radius/16** (one constant change + test). Fold into L-A if the owner wants one look-check.
4. **L-D: flicker band option** `LightAnimation::brightness` with a vanilla preset (0.25..1.0 mean-preserving); exact
   test of range/mean; default stays until the owner signs off.
5. **L-E: ambient fill for the no-GI path:** confirm GTAO radius for interiors (research first: where it is set),
   optional `ambient floor` inside the GI box. Cost 0.
6. **L-F: emissive census** (`mw_nif` census tool): count models with emissive and no LIGH, and models with a switch
   node named `NightDaySwitch`, in the vanilla ESMs and installed mods. Output feeds 2.6 and 2.7 decisions. No code behind the game.
7. Later, only if L-F says so: **named switch variants** (2.7) and **far emissive points** (2.6).

## 5. Visual-check scenes (for `docs/visual-checks/lights.md` after each slice; status "awaiting owner")

- **LT-reach (L-A):** `view "Seyda Neen, Census and Excise Office"`, camera in the upstairs room, 14:00 interior, GI on and off
  (`flip gi`); look for: light no longer on the far side of walls, the pool edge on floors not ringing, GI visible next to a lamp.
  Second: Balmora square at 22:00 walking up to a house: nearby unshadowed lamps should not paint the facade across the
  street.
- **LT-body (L-B):** same office, `flip gi` with a chandelier in view; look for the warm bounce no longer bright on
  the ceiling right above a lamp (double count) and no new dark patch under it.
- **LT-flicker (L-D):** Seyda Neen tavern fireplace, 22:00, 20 s still camera, vanilla preset vs ours: depth of
  dips and average brightness.
- **LT-far (2.6):** from Ghostgate road toward Balmora at 23:00, 1-3 km: lamp sprites, then windows if any.

## 6. Questions for the owner

1. Unshadowed lamp reach: accept about 1.5 x radius (shadowed lamps keep 4 x) as the High default, or keep 4 x and
   rely on GI + ambient 0 (cause 1b stays)?
2. Flicker: do you want the vanilla band (0.25..1.0, avg 0.62, darker average) or ours (0.65..1)? Same lumens
   calibration or re-calibrated?
3. Negative lights (Census office `dark_128` and caves): ignore for good, or later via a darkening volume?
4. Is mod `NightDaySwitch` support wanted for first release (mod-compat promise), or after the playable slice?
5. Ambient floor inside the GI box: strictly 0, or a small floor (0.1) so that unlit corners are not pitch black?

## 7. Caveats

- rtx is a 4090-laptop ray tracer; its numbers (bounce rays, 1 ray = 1 ms) do not transfer to our raster cost.
- Several rtx constants are "set by eye, provisional" (`lightbuilder.cpp:30-33`); we should take the structure, not the values.
- The rtx `makeLight` doc in `lightbuilder.hpp` says negative lights error out; the `.cpp` (`:190-196`) builds them. Trust the code.
- Licence: all adopted items are ideas or small constants. If any rtx code is ported, cite file + commit
  (`f2f8b2d5b2`) in the module doc and add a `superfargoth/THIRD_PARTY_NOTICES.md` entry (GPLv3 is compatible with our GPL-3.0-or-later).
