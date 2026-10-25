# Shadows: why they only show up close, and what to fix first

Research session `shadowr`, 2026-10-19. Read-only diagnosis: no builds, no code changes. It answers the owner's
complaint in ROADMAP 6a ("Shadows: most shadows only appear very close") and the 2026-10-18 "Shadows pass" request.
OpenMW's `clockwork-alpha` branch is GPL. This note describes **only its behaviour and techniques**, taken from its
settings documentation and commit titles. Nothing here is code from it (superfargoth CLAUDE.md rules 2 and 3).

## 1. Our current setup (verified in source)

| Item | Value | Where |
|---|---|---|
| Sun cascades | 4; first far bound **10 m**; max **504 m** (`(handoff(1)+band)·1.1+20`, High preset); overlap 0.2 | `bevyrrowind/apps/bevyrrowind/src/exterior.rs` `sun_cascades` |
| Cascade splits | Bevy: first bound fixed, the rest geometric → **10 / 36.9 / 136 / 504 m** | `bevy_light-0.20.0/src/cascade.rs` `calculate_cascade_bounds` |
| Shadow map | **2048²** per cascade (Bevy default; `DirectionalLightShadowMap` never set; 4096² measured +1.4 ms and dropped) | weather/distant-shadows.md "As built" |
| Fitting | Bevy fits each cascade to a sphere around the frustum slice and snaps it to texels (stable, no shimmer, but square coverage is wasted) | `cascade.rs` `calculate_cascade` |
| Bias | Bevy defaults: depth 0.02 m along the light, **normal 1.8 × cascade texel size** | `bevy_light` `DirectionalLight`, `shadows.wesl` |
| Filtering | `ShadowFilteringMethod::Gaussian` (default; never set). No TAA on raster, so `Temporal` and PCSS aren't usable as is | `bevy_light/src/lib.rs` |
| Contact shadows | **Off.** Bevy 0.20 has them (`ContactShadows` camera component + `contact_shadows_enabled` on lights); the exterior camera already has a `DepthPrepass` | `bevy_pbr-0.20.0/src/contact_shadows.rs` |
| AO | **None** (no SSAO/GTAO anywhere) | grep |
| Far shadows | Tier 2: terrain horizon march to 700 m + CPU shadow-height map (9.2 km); proxies/impostors read it; dithered cascade end 428-504 m via `directional_shadow_factor` | weather/distant-shadows.md "Distant shadows" |
| Point lights | `PointShadowBudget`: Low/Medium **0**, High **1** (1024² cube faces), Ultra 4; faded out over **15-25 m**. No caching: each shadowed light re-renders 6 faces every frame (+0.8 ms for 1, +3.9 ms for 2) | `sf_sky_bevy/src/shadow_budget.rs`, `exterior.rs` `point_shadows` |
| Excluded casters | far-field proxies/impostors (`NotShadowCaster` unless `l1_shadows`), sea, precipitation, held item; small objects culled by `LodScreen.min_pixels` also leave the shadow pass (Bevy culls casters by the main view's `VisibilityRange`) | `sf_lod_bevy/src/runtime.rs` |
| Blended foliage | Bevy 0.20 *does* draw blended materials into the shadow pass, discarding below alpha **0.05**, so leaves cast fat blobs, not dapples | `bevy_pbr` `pbr_prepass_functions.wesl` |

**Texel size per cascade** (70° vertical FOV, 16:9, 2048²; Bevy sizes a cascade by its slice's larger diagonal):

| Cascade | Covers | Texel | Normal offset (1.8 texels) | Gaussian footprint (~±2 texels) |
|---|---|---|---|---|
| 0 | 0.1-10 m | 1.4 cm | 2.5 cm | ±3 cm |
| 1 | 8-37 m | 5 cm | 9 cm | ±10 cm |
| 2 | 30-136 m | 19 cm | 34 cm | ±40 cm |
| 3 | 109-504 m | **70 cm** | **1.3 m** | **±1.4 m** |

### Why shadows only show close

1. **No contact-scale shadowing at all.** No contact shadows, no AO. Everything that grounds an object (a crate on a
   table, feet on the floor, a pot against a wall, the base of a rock) depends on the shadow map alone, and the
   normal offset lifts every lookup off the surface by 1.8 texels. Close to the camera that's 2.5 cm. At 30 m it is
   34 cm, at 110 m 1.3 m. Any caster shorter than the offset loses its shadow where it touches the ground, which is
   exactly the "objects float, shadows only right next to you" look. Industry engines never ship CSM alone for this:
   contact shadows (UE, Days Gone) and AO fill the gap.
2. **The far half of the screen sits in a 70 cm-texel cascade.** The 504 m maximum is tied to the far-field handoff
   (weather/distant-shadows.md tier 1, so no L0 object crosses the cascade end). That's correct for popping, but cascade 3's
   texel size depends only on that maximum. With a ±1.4 m filter and a 1.3 m normal offset, anything under ~2-3 m
   across (NPCs, fences, carts, crates, most trees' trunks) casts nothing visible past ~110 m, and only a smudge
   past ~35 m. The first bound (10 m) is fine. The problem is that one camera-centred CSM is asked to cover 504 m.
3. **Blur, not softness.** The Gaussian kernel is fixed in texels, so penumbras widen with distance from the camera
   instead of with distance from the caster: crisp up close, washed out at range, and never "contact-hardening".
4. **Interiors are mostly lit by unshadowed lights.** High gives one shadowed point light, faded out from 15 m. An
   interior holds 20-30 lights (Clockwork's doc counts them the same way). Every other light passes through walls,
   furniture and actors, which is the Census-office report.
5. **Leaves cast blobs** (alpha cutoff 0.05 in the shadow pass) until leafmask lands. After it lands, check the
   alpha-coverage mips in the shadow pass: far cascades sample high mips, where leaves fade unless coverage is kept.
6. Ruled out: max distance too short (504 m is long), the cascade end (dithered from 428 m), the RT path (raster is
   the default), point-light shadows being off (one on High). Size culling only removes casters that aren't drawn
   either. Possibly related: the "grazing-light blocky half-transparent outline" (6a) is the classic symptom of a
   texel-scaled normal offset plus a wide kernel on surfaces almost parallel to the sun. Bias tuning (fix 3) should
   be checked against it.

## 2. OpenMW `clockwork-alpha` (Thomas Portal / OursCodeur): behaviour only

Source: [commits](https://gitlab.com/OursCodeur/openmw/-/commits/clockwork-alpha) (fetched through the GitLab API,
2026-10-19), the branch's `docs/source/reference/modding/settings/shadows.rst` and `files/settings-default.cfg`.
"Clockwork" is a new Vulkan renderer inside OpenMW ("Vulkan entries"; "Classic matches OpenGL"). The commit titles say
little ("Visual Upgrades and frame pacing", "Sun bounce, heat haze and bloom", "More perf work", "Fifth round of
feedback", "Rebase Clockwork on upstream master"). The shadow work is spread through them:
- `1dba3a92` (rebase, 2026-10-18): brings the renderer in, with tests for far shadows, local shadows, world shadows
  and clustered lighting.
- `559da599`: sun shadow softness, local shadows and AO shaders.
- `ebc29570`: far shadow, shadow-casting pass and shadow parameters reworked.
- `610729d2`: one `upgrades` master switch (classic vs modern shadows) and the world-shadow shader.
- `dbe9de7b`: "sun blockers" (blocker marks).
- `60c7bfb6` / `75746e6c`: local and world shadows, groundcover in the shadow pass, actor bounce light.

What the settings doc says it does (units: 1 unit ≈ 1.4 cm):

- **Sun CSM:** 3 cascades, **2048²**, max **16384 units ≈ 230 m**, fade from 90 %. Splits mix log and uniform (practical
  split, λ = 0.5). Cascades are **fitted to the bounds of the casters and receivers actually in view** rather than the
  frustum slice. Biases: slope-scaled polygon offset in the caster pass plus a small normal offset (1 unit ≈ 1.4 cm),
  not a texel-scaled one. So the CSM is about half our range, with tighter fitting.
- **Sun softness (PCSS-like):** the penumbra is the sun disc's physical width × 2 (≈2 cm under a 2 m caster, 26 cm
  under a 30 m one), capped above 30 m casters, and **the same width in world units in every cascade**, so a
  shadow doesn't sharpen or blur as the camera moves. This is contact-hardening: crisp at the base, soft at the tip.
  This is the "interesting shadows" look.
- **Weather on shadows:** haze and cloud widen the penumbra up to 4×. Past ~85 % cloud cover the sun shadow fades to
  60 %. A cloud-texture shadow follows the sky's clouds.
- **Distant terrain shadows:** a per-cell horizon map (16 directions, a sample every 128 units, built on background
  threads). Land past the cascades loses the sun below the horizon; objects read their ground's horizon, lowered by
  their height. Under 30 µs on an RTX 4090.
- **Distant object shadows:** **one cached 4096² sun depth map over the whole view distance, static objects only**
  (buildings, towers, trees). It is redrawn when the sun moves 0.02°, the camera moves 1/16 of its reach, or new
  objects arrive, spread over 9 frames (≈0.05 ms per frame on average). This is the far tier our objects lack.
- **Groundcover casts** sun shadows that sway with the wind. Grass doesn't receive its own shadow within a tuft's
  height and darkens toward its roots instead. Cost ≈0.08 ms of cascade drawing on a dense field.
- **Local (point-light) shadows, two systems:**
  - Per-actor shadows from the 2 nearest lights, for 6 actors, at 512². Each shadow is clipped at the nearest
    wall or floor behind the actor (an "occluder layer"), so it can't leak into the room below. Floor-margin logic
    keeps shadows attached on stairs.
  - **World shadow cubes for many lights:** 8-61 lights (2 per GiB of VRAM; 47 on a 4090), 512² faces, **static
    casters cached** and redrawn only when something near them changes. Moving objects redraw at 30 Hz over a cached
    static copy. Lights are chosen by their contribution at the camera, including lights out of view (turning the
    camera redraws nothing), with crossfades when a cube moves to another light. 0.06-0.24 ms for 8 lights.
  - Area-light softening (source radius 8 units fixed, 2 carried), and **light-fixture self-shadowing** (a brazier's
    bowl darkens the floor under it).
  - "Local shadow strength": a shadow from the dominant light isn't filled in by the other lights.
- Blended surfaces cast where alpha > 0.5. Additive glows cast nothing.

**Takeaway:** Clockwork's look comes from (a) physically sized, contact-hardening soft shadows, (b) a shorter, tightly
fitted CSM plus a **cached static far map** instead of one long CSM, and (c) **cached point-light shadows for dozens of
lights**. None of these is exotic. All are standard practice (below).

## 3. Industry canon and what Bevy 0.20 has

| Technique | Canon | Bevy 0.20 | Us |
|---|---|---|---|
| CSM, practical split (log/linear λ) | [PSSM, GPU Gems 3 ch. 10](https://developer.nvidia.com/gpugems/gpugems3/part-ii-light-and-shadows/chapter-10-parallel-split-shadow-maps-programmable-gpus); [MS CSM](https://learn.microsoft.com/en-us/windows/win32/dxtecharts/cascaded-shadow-maps) | fixed first bound + geometric rest; 4 cascades max (`MAX_CASCADES_PER_LIGHT`) | 10 m / 504 m |
| Stable cascades (sphere fit + texel snap) | Valient, *Stable CSM* (ShaderX6); [MJP, A Sampling of Shadow Techniques](https://therealmjp.github.io/posts/shadow-maps/) | **yes** | yes |
| Tight fit to scene bounds | [MS, Common Techniques to Improve Shadow Depth Maps](https://learn.microsoft.com/en-us/windows/win32/dxtecharts/common-techniques-to-improve-shadow-depth-maps) | no (it conflicts with stability) | no |
| Cascade blending | overlap / dithered transition | overlap blend | 0.2 + dithered end |
| Bias | slope-scaled depth bias + normal offset in texels (MJP) | depth + normal (texels); no per-cascade value | defaults (1.8) |
| PCF / PCSS (contact-hardening) | [Fernando, PCSS 2005](https://developer.download.nvidia.com/shaderlibrary/docs/shadow_PCSS.pdf) | Hardware2x2 / Gaussian / Temporal; **PCSS behind `experimental_pbr_pcss`** (`soft_shadow_size`, noisy without TAA) | Gaussian |
| Screen-space contact shadows | [UE Contact Shadows](https://dev.epicgames.com/documentation/en-us/unreal-engine/contact-shadows-in-unreal-engine); Bend Studio, *Days Gone* screen-space shadows (SIGGRAPH 2023) | **yes**: [`ContactShadows`](https://docs.rs/bevy/0.20.0/bevy/pbr/struct.ContactShadows.html) + `contact_shadows_enabled` on directional/point/spot lights (16 steps, 0.3 m ray by default) | **off** |
| Far shadows (distance field / heightfield) | [UE distance-field shadows](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-field-soft-shadows-in-unreal-engine), far cascades; horizon maps | none (past the last cascade = lit) | terrain SHM + march (tier 2) |
| Cached static shadows | UE [VSM page caching](https://dev.epicgames.com/documentation/en-us/unreal-engine/virtual-shadow-maps-in-unreal-engine) / cached whole-scene shadows for movable lights; HDRP on-demand shadow updates | **none** (every shadow view re-renders every frame) | none |
| Caster culling by projected size per cascade | UE `r.Shadow.RadiusThreshold` | **none** per cascade; casters only culled by the main view's `VisibilityRange` | via LodScreen only |
| Alpha-tested foliage casting | every engine: mask in the shadow pass, coverage-preserving mips | Mask/Blend discard in the shadow prepass | blend (0.05) → mask (leafmask) |
| Many local-light shadows | shadow atlas + caching + budget (UE, Frostbite, Doom 2016) | per-light cube maps, no atlas, no caching | 1 light on High |

## 4. Recommendation: fixes in order of impact

Each slice is 1-2 steps with an A/B switch. GPU costs are 1440p on the RTX 4060 Laptop. "Est." means not measured yet.
New visual checks SH-7..SH-11 go into `bevyrrowind/docs/visual-checks.md`:
- **SH-7** Seyda Neen 15:00 (long, low shadows, docks, crates and NPCs at 5/30/120 m).
- **SH-8** Balmora streets 11:00 and 17:30 (`balmora_a`; awnings, steps, guards).
- **SH-9** a forest (Bitter Coast or Ascadian Isles) at 10:00 (leaf dapples, trunk shadows, grass).
- **SH-10** Census office interior (lamps, NPCs, furniture).
- **SH-11** a mid-range vista (a Balmora rooftop toward Odai, 50-300 m).

**F1. Contact shadows, sun first, then budgeted point lights** (Bevy-native, config only; 1 slice). Add `ContactShadows`
to the exterior and interior cameras (the depth prepass is already there) and turn on `contact_shadows_enabled` on the
sun, the moon and the budgeted point light(s). Tune `length` to 0.5-1.0 m outdoors and 0.3 m indoors, with 16 steps.
This restores grounding at every distance, independent of cascade texels. Per-preset settings: Low off, Medium 8
steps, High 16. A/B: `--off contactshadows`. Cost est. 0.2-0.5 ms (one raymarch per shaded pixel per enabled light).
Verify: SH-7/8/10 side by side on/off; crates, feet and table clutter grounded; no halos at depth edges.

**F2. Bias and filtering retune** (config + measurement; 1 slice). Normal bias 1.8 → ~0.6-1.0 texels, depth bias swept
for acne, against the grazing-light artefact. A/B Gaussian vs Hardware2x2 (sharper, keeps small casters at range).
Try `DirectionalLightShadowMap` 3072² on Ultra only. A/B: the existing `--off shadowtune` gains a `--shadow-bias` /
`--shadow-filter` override. Cost ≈ 0 (bias), +0.5-0.8 ms est. at 3072². Verify: SH-2 and SH-8 at grazing sun
(07:00): no acne, the 6a outline gone or explained, NPC shadows readable at 60 m.

**F3. Shorter CSM plus a cached far static-object map** (2 slices; the structural fix, Clockwork's and UE's scheme).
- (a) Cascades to ~250 m (bounds ≈ 8 / 25 / 79 / 250 m) give texels 1.1 / 3.5 / 11 / **35 cm**: 2× sharper at mid
  range, and fewer casters in the last cascade (est. −0.2 ms).
- (b) To keep weather/distant-shadows.md's no-pop rule for L0 objects out to 504 m, add one **cached 4096² orthographic sun depth map**
  of static L0 and L1 casters, from ~200 m to ~2-4 km. It is redrawn in slices over several frames when the sun turns
  past a threshold or the camera moves 1/16 of its reach. It is read through the existing `directional_shadow_factor`
  hook with a dithered handoff, as SHM is today, and it also gives far-field proxies real shadows (T2's open item).
  Bevy has no caching shadow API, so this is ours: a custom render-graph pass (rule 8: Bevy has nothing; no crate
  found).
- Cost est. +0.05-0.2 ms amortized, 64 MB VRAM.
- Verify: SH-1/SH-6 re-run (no pop), SH-11, and fly from 700 m to 50 m at a Balmora building.
- Supersedes A1's "s_max ≥ d1 + w" (weather/distant-shadows.md and ARCHITECTURE §10 update when built).

**F4. Foliage and grass shadows after leafmask** (1 slice). Check that masked leaves cast at the 0.5 cutoff with
coverage-preserving mips in every cascade, and that wind sway runs in the shadow pass (sf_wind's shared function).
Let grass cast into cascades 0-1 only (per-cascade caster limits are a Bevy gap, so try a grass `VisibilityRange` or a
separate light layer). Grass shouldn't self-shadow within its own height. A/B `--leaves blend|mask` (leafmask's
switch). Cost est. +0.1-0.3 ms for foliage in cascades 0-2. Verify: SH-9 dapples on the ground that sway, with no
flicker at 50 m.

**F5. Soft, contact-hardening sun shadows** (1-2 slices). Enable Bevy PCSS (`experimental_pbr_pcss`, `soft_shadow_size`
≈ 2× the sun disc, like Clockwork's default). It needs temporal filtering: under `--dlss` it works now. On plain raster
it needs TAA or a spatial-only blocker search. Cost est. +0.3-1.0 ms, so High/Ultra only. Weather widening comes later
from `sf_weather` haze and cloud cover. Verify: SH-7 penumbra sharp at the post base and soft at the roof edge.

**F6. Interior local-light shadows at scale** (larger project; needs the owner's go, per the ROADMAP "Shadows pass").
A point-light **shadow atlas with static caching**: casters re-rendered only when something near the light moves, and
dynamic actors drawn over a cached static copy. Budget 8-16 lights chosen by contribution at the camera (including
lights out of view), with crossfades. Bevy has none of this, so either upstream it or write our own render-graph
pass. Contact shadows (F1) on all nearby lights are the cheap stopgap until then. Clockwork reports 0.06-0.24 ms for 8
cached lights on a 4090. Verify: SH-10, plus the 2-4 light cost table re-measured.

Also consider SSAO/GTAO (Bevy `ScreenSpaceAmbientOcclusion`, ~0.3-0.6 ms est.) as a separate lighting item. AO isn't
a shadow, but it's half of the "grounded" look. Log it, don't bundle it.

**Expected total for High:** F1 + F2 + F3(a) ≈ +0.0-0.3 ms net (the shorter CSM pays for the contact shadows). F3(b)
+0.1 ms, F4 +0.2 ms, F5 +0.5 ms. Ultra adds 3072² and PCSS.

## Owner questions (max 3)

1. **Soft vs crisp:** do you want Clockwork-style contact-hardening soft sun shadows (F5, +0.3-1 ms, best with
   DLSS/TAA), or crisp shadows plus contact shadows only (F1-F3)?
2. **Far object shadows:** OK to drop the "one long 504 m CSM" rule (weather/distant-shadows.md A1) in favour of a 250 m CSM plus a
   cached far map (F3)? It's sharper near and mid, but it's a new bespoke pass.
3. **Interiors:** go for the cached point-light shadow atlas (F6, multi-slice, our own pass), or live with contact
   shadows plus 1-4 shadowed lights for now?

## Owner decisions (2026-10-19)

- **Soft, contact-hardening shadows** (F5, PCSS-style): crisp at contact, softer with distance; cheaper filter on low presets.
- **Drop the 504 m cascade rule** (F3): sun cascades to ~250 m plus a cached static far shadow map beyond.
- **Interior shadow atlas soon** (F6): right after F1-F3, before F5.
- Order: F1+F2 (contact shadows, bias) -> F3 -> F6 -> F5; F4 leaf shadows with leafmask; render-order.md RO-1
  (translucents casting solid shadows) after leafmask.

## Status

- **F3 step a built (shf3, 2026-10-19):** practical-split cascades to 250 m (Low 3 / 200 m), texels 1.4 / 3.8 / 10.5 /
  35 cm; GPU ≈ 0 at Seyda Neen. Step b (cached far static map) built off by default, then deleted in shadow reset A2
  (2026-10-21); weather/distant-shadows.md "Sun cascades". Owner check SH-8.
