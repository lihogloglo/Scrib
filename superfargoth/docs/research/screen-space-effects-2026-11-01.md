# Screen-space and image-space effects for Scrib: what 2024-2026 titles ship, what fits, what it costs (research, 2026-11-01)

Research only; no code, builds or game runs. The owner asked about SSR (including wet/glossy leaves and leaf
translucency) and "other screen-space shenanigans Crimson Desert and others do". GI is out of scope (see
`gi-2026-11-01.md`, `gi-dynamic-2026-11-01.md`). Grades as in the GI note: **[P]** primary (slides, engine docs,
papers I opened), **[S]** secondary (press, Digital Foundry as quoted, third-party frame captures), **[I]** inference.
Costs marked (est.) are mine: the 4060 laptop at DLSS Quality renders about 960p internal (0.44x the pixels of
1440p); effects Bevy runs after DLSS (bloom, motion blur, DoF) cost at 1440p output. "(meas.)" is measured in this repo.

## 1. Summary and recommendation

The honest finding: **nobody ships general SSR as the only reflection answer any more**, and no surveyed title is
known to do glossy-leaf reflections on purpose. The 2024-2026 pattern is a ladder (AC Shadows slides, Lumen docs, DF on
Crimson Desert): screen-space march first, then a fallback (probes/cubemaps, SDF, or RT), SSR being the cheap tier.
For a forward-rendered Bevy game with DLSS, the value per ms is elsewhere.

Ranked for this game (Morrowind: wet swamps, ash, lamp-lit interiors, alpha-tested trees, lots of water):

| # | Effect | Why | Cost (est.) |
|---|---|---|---|
| 1 | **Bloom** (Bevy `Bloom`, COD-AW dual filter) | Absent on the game camera today (`industry-audit.md` R-h). Biggest look per ms: lamps, sun disk, spells, ash glow. RDR2 uses the same filter [S]. | 0.3-0.5 ms at 1440p output |
| 2 | **Wet-material response + puddles** (not screen space), dynamic reflection probe as fallback | Already specced (`precipitation-surface.md`; hook `apply_surface_state` is still a no-op). Darken, smooth, flat puddle normals, ripples; reflect sky + `water.md`'s planned camera-centred probe (shore, mountains, far buildings at probe fidelity). Gives most of the wet look with no SSR. | <= 0.3 ms (borrow-audit est.) |
| 3 | **Thin-surface translucency for leaves/paper** in `WorldMaterial` | The one foliage item with real payoff: sun through canopy. Bevy has `diffuse_transmission` + `thickness` (second Lambert lobe, no extra pass). AC Shadows and Tsushima treat leaves/paper as thin translucent. | 0.05-0.15 ms |
| 4 | **Water SSPR** (planned) | `water.md` already chose Cichocki SSPR for the dominant plane; right for planar water, cheaper than SSR. | 0.3-0.5 ms |
| 5 | **Keep GTAO + contact shadows**; sun + nearest lamp only | GTAO ~0.9 ms (meas., 1440p native) so ~0.4 at 960p (est.). Contact shadows 0.80 ms per shadowed light (meas., native). Bevy already derives specular occlusion from SSAO. | already paid |
| 6 | **Motion blur** option, **DoF** for dialogue only | Runs after DLSS, needs the MV prepass DLSS already forces. | MB 0.2-0.4, DoF 0.3-0.6 |
| 7 | **Wet-only forward SSR** (custom pass) | Only if the owner wants buildings in puddles beyond probe fidelity; roughness comes analytically from surface state, no G-buffer. | 0.4-0.8 ms, Phase 4 |
| 8 | CA / vignette / lens distortion / film grain | Optional toggles, default off. Bevy has the first three; grain is ~20 lines as a `FullscreenMaterial`. | 0.05-0.1 ms |

**Skip for now:** general SSR via Bevy's deferred-only `ScreenSpaceReflections` on the raster path (4.1); reflections
on foliage (4.2); screen-space SSS for skin (4.3); SSGI (4.6; the only Bevy crate is dead); screen-space god rays (4.8,
froxels exist); Bevy OIT (4.12). Roadmap over polish applies: this note feeds later slices, it opens none.

## 2. Our constraints (verified in repo)

- **Raster is forward** (`sky-lighting.md`; `path.rs` sets `DefaultOpaqueRendererMethod::forward()`); Solari forces
  deferred. LOD materials are explicitly `OpaqueRendererMethod::Forward`. Bevy SSR needs `DeferredPrepass`
  (`ssr/mod.rs`: "only supported with deferred rendering"), so it fits the RT path only, where Solari reflects anyway.
- Open since 2026-10: does the deferred prepass honour the `VisibilityRange` dither (`weather.md` question 4)? Not re-tested.
- **Depth + normal + MV prepass exist** on the DLSS/GTAO camera. A depth+MV prepass cost +2.0 ms at native 1440p
  (13.9 vs 11.9 ms Balmora, `sky-lighting.md`) but DLSS needs it. DLSS SR pass 2.1-2.3 ms (meas.).
- Vendored `bevy_pbr` carries the **RO-11 patch**: contact shadows and SSR march depth in `main_pass_viewport`
  (DLSS internal resolution); #26234 puts GTAO at the main-pass resolution. Any new depth-reading pass must do the same.
- **Game camera today:** exposure/`AutoExposure`, TAA or DLSS, GTAO and contact shadows (console toggles). No bloom,
  motion blur, DoF or SSR. Wetness hook is a no-op (`surface_state.wesl`); `sf_atmos` has cloud wetness only;
  `sf_precip_bevy` draws rain; `sf_water_bevy` has FFT surface, refraction snapshot, motion vectors, but no SSPR (grep).
- Frame target <= 11 ms at High, 1440p DLSS Quality, 4060 laptop. Balmora: native 11.9 ms, DLSS Quality 8.9 (meas.,
  2026-10-12): about 2 ms headroom, shared with all of Phases 1-4.
- Rules: patch Bevy, do not inject; reuse order Bevy > crates > own; no Morrowind in `superfargoth/`; no automated
  screenshots to judge looks (owner, 2026-11-01).

## 3. Title survey

| Title / engine | Screen-space and image-space techniques found | Evidence |
|---|---|---|
| **Crimson Desert** (BlackSpace) | Per-pixel RT diffuse GI on all platforms. Reflections: RT blended with SSR plus SDF for distant geometry, "especially on water"; RT radius shrinks in Performance mode. Menus list reflection quality and volumetric fog quality; lower GI tiers use "screen-space alternatives". Denoiser weak (streaking, temporal noise); heavy displacement flicker; smoke/dust cast moving shadows. DLSS RR ~14% (5080, 4K Perf); guides report RR glitches (rain vanishing). No GDC/SIGGRAPH talk found (GDC 2025 was closed-door). Foliage, SSS, wetness, motion blur, DoF: nothing verifiable. | [S] DF via finalboss.io, pcgamesn, 80.lv, guides. Low confidence |
| **Kingdom Come: Deliverance 2** (CryEngine) | SVOGI (GI note). Options: object motion blur, near DoF, post quality, volumetrics detail. CryEngine has `r_SSReflections` (sharp-noisy to blurred modes), screen-space sun shafts, velocity-pass motion blur (CE2/CE3 docs). PC Gamer: its SSR and occlusion "don't look half as good" as RT. | [S] gamingbolt, PC Gamer; [P] older CE docs; KCD2 SSR detail [I] |
| **Assassin's Creed Shadows** (Anvil) | **Scalability ladder for specular: dynamic global cubemaps, relightable local cubemaps, SSR on weak hardware; RT specular on top.** RT starts with a screen-space march (linear + binary + secant: the same family as Bevy's), then world trace; specular at half width, GGX VNDF, ray length by roughness, rough pixels reuse diffuse SH (-20-30% cast cost), specular denoiser with firefly clamp. **Thin translucency** for paper and leaves (back-face probability t/(1+t), "more complex for foliage"). | [P] SIGGRAPH 2025 slides (opened) |
| **Avowed / UE5** | Lumen: screen traces first, then SDF or RT. Medium reflection quality = plain SSR ("save 1 ms on Series S"). `MaxRoughnessToTraceForFoliage` is its own knob, suggested 0. Fortnite C4: "extreme noise from screen traces on grass" fixed by skipping foliage hits. Contact shadows per light, length a screen fraction. | [P] Epic docs/blog; Avowed specifics [S] |
| **Ghost of Tsushima / Yotei** | Tsushima: probe-based indirect specular (one probe relit per frame, async compute), parallax correction, **horizon occlusion ~3% of deferred lighting**, Lambert with "optional energy-conserving translucency", froxel haze with AO. SSR not mentioned in the talk. Yotei: RT lighting + PSSR; wet/muddy/snowy character materials; no screen-space detail published (Sucker Punch promises 2026 talks). | [P] Patry, SIGGRAPH 2021 (opened); Yotei [S] |
| **Horizon Forbidden West** (Decima) | Not established. Talk titles found (Real-Time Live 2022, "Showcase of Decima", deferred texturing GDC, HZD vegetation 2018); I could not read slides for SSR, foliage translucency or wetness. | [S] titles only. **Gap** |
| **Red Dead Redemption 2** | Third-party RenderDoc study: SSAO + baked AO cubemap; **SSR for water combined with the frame-start env map**; planar only for mirrors; three separate depth downsamples (SSAO, SSR, volumetrics); velocity-target motion blur; **COD-AW bloom (non-thresholded, 13-tap down, 3x3 tent up, 7 mips)** with a fog-glow mask; light shafts; DoF. Ultra reflections ~40% fps. | [S] imgeself study; gamingbolt |
| **Elden Ring** | PCGH: RT patch swapped cubemaps for SSR (edge dropouts); no contact shadows, high-res shadow maps. DF: SSAO silhouette artefacts. | [S] |
| **Skyrim / Starfield** | Starfield: DF breakdown chapters name **screen-space shadows, volumetric lighting, bokeh DoF**. SSR/SSAO detail not found. Skyrim: [I] water SSR only. | [S] chapter list |
| **OpenMW** (baseline) | Moddable post-process framework (OMWFX): depth + view/world normals, HDR luminance helper. Community pack (Hrnchamd, vtastek et al.): **SSAO HQ, SSR (needs its "wetworld" puddle shader), godrays, motion blur, HDR eye adaptation, underwater**. Pack licence unstated: do not copy. | [P] OpenMW docs (via search); [S] pack READMEs |

Reading across: (1) SSR survives as a fallback tier or a water/wet-surface effect; OpenMW's SSR-needs-puddles pairing
has the same shape. (2) Foliage is where reflection tracing gets switched off. (3) Leaf translucency is a material
term, not a pass. (4) Bloom, motion blur, DoF are universal and cheap; discipline is placement relative to the upscaler.
(5) The two titles with the loudest tech press (Crimson Desert, Yotei) have the weakest public screen-space detail.

## 4. Effect by effect

### 4.1 SSR (water, wet ground, metals)

- **What:** march lit colour through depth along the reflected ray. Variants: linear + binary (+secant) per pixel
  (Bevy; AC Shadows' pre-trace); **hierarchical (hi-Z)** traversal (McGuire and Mara 2014; AMD SSSR: 7-mip SPD depth
  pyramid, tile classification, ray list, denoiser, MIT); stochastic GGX-VNDF sampling with temporal/spatial reuse.
- **Roughness:** rough pixels skip the march (SSSR `roughnessThreshold` -> env map; UE max roughness ~0.8, Lumen
  dedicated rays 0.4; Bevy fades in at 0.08-0.12, out at 0.55-0.6).
- **Fallbacks:** probes/cubemaps (AC Shadows, Tsushima; Bevy blends the env map where the ray misses), SDF/RT.
- **Cost:** AMD publishes no SSSR timing on pages I could read. UE: SSR instead of Lumen reflections saves ~1 ms on
  Series S at 1080p [P]. AC Shadows RT specular 1.6-3.2 ms on a 4070 at 1440p [P] (includes world trace). Bevy SSR at
  960p (est.) 0.4-0.8 ms plus G-buffer bandwidth; unmeasured.
- **DLSS/TAA:** stochastic noise relies on temporal resolve; reflected content carries the surface's MVs, so it lags and
  smears. NVIDIA lists "motion vector for reflections" only as optional research input [P]; SR ignores it. Ray
  Reconstruction is the real answer on the RT path.
- **Bevy:** `ScreenSpaceReflections` (vendored with RO-11) needs `DepthPrepass` + `DeferredPrepass`, runs after
  `deferred_lighting`; STBN noise needs the `bluenoise_texture` feature; exposes thickness, steps, bisection, secant,
  edge fade. It reads G-buffer roughness/base colour, so forward-opaque materials never reflect.
- **Our options:** (a) probes plus the planned dynamic probe (recommended); (b) deferred raster + Bevy SSR: blocked by
  the dither question, G-buffer writes in terrain/grass/water materials, unmeasured bandwidth; (c) **wet-only forward
  pass** later: depth + normal prepass exist, roughness/F0 come from the surface-state clipmap, pass only where
  `wetness > 0`. Precedent bevy_atmos (`borrow-audit` item 3: ~1.2 ms for a river at 720p, so budget conservatively).

### 4.2 Foliage: wet and glossy leaves

- No title found ships traced glossy leaves deliberately. Evidence against: Fortnite C4 (screen-trace noise on grass,
  fixed by skipping foliage hits) [P]; UE's separate foliage roughness cutoff [P]; depth buffers are heightfields, so
  thin masked cards get wrong thickness, flicker and holes (Future of SSR article) [S]. Our DLSS/TAA path has MSAA off,
  so Bevy's `AlphaToCoverage` degrades to `Mask` (`material.rs:697`): leaf edges rely on TAA/DLSS and mip coverage.
- **Wet-leaf look, cheaply:** lower roughness, darker albedo, sky/probe specular and the analytic sun lobe, driven by the
  same surface state as the ground (ranking item 2). No trace noise.
- DLSS ghosts thin objects, animated textures and missing MVs [P]: wind-animated leaves need correct MVs
  (`render-order.md` rule 3).

### 4.3 Subsurface scattering and thin-surface transmission

- **Skin SSSS:** Jimenez separable SSS, 0.489 ms at 1080p on a Radeon HD 7970 (2015 paper) [P], stencil-culled to skin
  pixels. **Skip:** few NPCs, low-res Morrowind textures, tight budget; revisit with new head meshes.
- **Foliage/paper:** industry standard is a material term: Frostbite "Approximating Translucency" (GDC 2011), Crysis 2
  vegetation (GPU Gems 3 ch.16), Tsushima, AC Shadows. No screen-space pass.
- **Bevy:** `diffuse_transmission`, `thickness`, `specular_transmission`, `ior`, `attenuation_*`.
  `diffuse_transmission` adds a Lambert lobe at a thickness-shifted position (`pbr_functions.wesl` ~374-380).
  `ScreenSpaceTransmission` is the refraction path (one scene copy per step, no depth/MV); `water.md` rejected it.
- **Do:** expose transmission/thickness through `WorldMaterial` and test with the sun behind a canopy. ALU only
  (est. 0.05-0.15 ms); no DLSS issue.

### 4.4 Screen-space (contact) shadows

- UE: per light, length a **screen fraction**, fixed samples so long rays are noisy [P]. Bend: compute-based sun shadows
  from Days Gone, SIGGRAPH 2023, code released [S]; reported in UE5 and Tsushima [S, unverified].
- **Bevy:** `ContactShadows { linear_steps 16, thickness 0.1, length 0.3 }`, forward or deferred, `DepthPrepass`.
  **0.80 ms per shadowed light at 1440p native (meas.)**, ~0.35 at 960p (est.). A 0.3 m ray is sub-pixel at distance.
- Keep sun + moon + nearest lamp; do not extend to every lamp. Bend's tiled compute is the known faster implementation
  if we vendor-patch later (unmeasured).

### 4.5 AO, bent normals, specular occlusion

- GTAO ~0.9 ms (meas.). XeGTAO reference: 0.56 ms at 1080p High on an RTX 2060, 1.4 ms at 4K on a 3070; Medium 2/3 of
  High; **bent normals +~25%**; repo archived 2024-04, MIT [P].
- Bevy has **no bent normals**; it derives `specular_occlusion = saturate(pow(NdotV + ssao, exp2(-16 r - 1)) - 1 +
  ssao)` (`pbr_fragment.wesl:545`). Tsushima applies horizon occlusion to specular [P]. Bent normals only pay with
  probes/irradiance volumes; skip until the GI decision.

### 4.6 SSGI as a near-field supplement

- Lumen screen traces, Unity HDRP SSGI; AC Shadows and Crimson Desert use a screen march as stage one of RT GI. GI is
  decided elsewhere. `bevy_ridiculous_ssgi` (deferred-only, last push 2024-02, no licence) is unusable. Cost (est.)
  1-2 ms at 960p. **Skip.**

### 4.7 Wetness and puddles (rain)

- Standard is a **material term**: Lagarde "Water drop 3a/3b" (porous albedo darkens, roughness toward 0.1, puddles as a
  water layer, mask from a world map or height). `precipitation-surface.md` is this, with a persisted clipmap.
- OpenMW's "wetworld" does puddles in post and its SSR depends on it [S]; a post puddle cannot be right with
  normal-mapped statics. Do not copy. Rain-on-lens: in no surveyed title; optional later.

### 4.8 Volumetric light shafts

- Industry: froxel volumetric fog (Wronski 2014, Hillaire 2015; Tsushima builds on both) [P]. RDR2 has shafts [S];
  CE2 and OpenMW packs use radial-blur sun shafts. Ours: `sf_atmos` froxel fog with cloud shadows is built; Bevy also
  has `VolumetricFog`/`VolumetricLight`. A screen-space radial blur would double-count and ignore cloud shadows.
  **Skip**; shafts are a froxel injection/anisotropy tuning question (`weather/fog.md`).

### 4.9 Motion blur

- Per-object velocity blur (Jimenez COD-AW class); RDR2 reads a velocity target [S]. Bevy `MotionBlur` (shutter angle,
  samples) needs `MotionVectorPrepass`, runs in `PostProcess` before bloom, i.e. after DLSS at output resolution, as
  NVIDIA's "DLSS early in post" requires [P]. Bevy docs: transparents write no MVs; fast-object edges over empty space
  stay sharp. Our water writes MVs. Cost (est.) 0.2-0.4 ms. Ship as a setting, default low/off.

### 4.10 Depth of field

- Bevy `DepthOfField` (Gaussian default, Bokeh option), after bloom, before tonemapping. Dialogue and inventory only.
  Cost (est.) Gaussian 0.3-0.6 ms, Bokeh ~1 ms. Starfield ships bokeh DoF [S].

### 4.11 Bloom and lens effects

- Bevy `Bloom` is the COD-AW design (the RDR2 capture matches) with presets (`Bloom::NATURAL`; used in the
  `sf_atmos_bevy`/`sf_sky_bevy` examples, not in the game). Bevy 0.20 also has `ChromaticAberration`, `LensDistortion`,
  `Vignette`; no film grain. Placement after DLSS needs no upscaler care. Risk: bloom plus froxel glow double-counts the
  sun; tune against scheduled exposure so night lamps do not bloom by day.

### 4.12 OIT and alpha handling

- Bevy `OrderIndependentTransparency` (fragment budget, memory heavy; est. 0.5-2 ms); it writes no MVs or reactive
  mask, so DLSS ghosts. We use sorted `Transparent3d` bands and masked cutouts. **Skip.** Foliage answer: alpha-tested or
  hashed alpha (Wyman and McGuire 2017) + TAA, coverage-preserving mips.

### 4.13 Screen-space decals

- Bevy `ForwardDecal` (needs `DepthPrepass`, steep-angle distortion) and clustered decals; unused in the repo. Fit for
  blood, scorch, footprints (Phase 3-4). Deferred-decal G-buffer blending does not apply to our forward path.

### 4.14 DLSS/TAA interaction (NVIDIA guide, 2026-03-31) [P]

- DLSS runs **before tone mapping, as early in post as possible**; later effects must handle output resolution. Bloom,
  motion blur, DoF, CA, vignette, grain go after. Depth-reading passes before it (SSR, GTAO, contact shadows) run at
  internal resolution and need the main-pass viewport.
- Ghosting: small particles, animated textures, thin objects, missing MVs; fix with the reactive mask (`industry-audit.md`
  R-c). Rain, foam, reflected content and translucents are exactly these cases. Optional hints (roughness, material ID,
  reflection MVs) are research inputs only, and our `dlss_wgpu` facade passes none.

## 5. Bevy 0.20 inventory

| Feature | Where | Constraint |
|---|---|---|
| `ScreenSpaceReflections` | `bevy_pbr/src/ssr/`, `ssr.wesl` | Deferred only; RO-11 patched; STBN needs `bluenoise_texture` |
| `ScreenSpaceAmbientOcclusion` | `bevy_pbr/src/ssao/` | Depth+normal+MV, `Msaa::Off`; #26234; no bent normals |
| `ContactShadows` | `bevy_pbr/src/contact_shadows.rs` | `DepthPrepass`; per-light cost; RO-11 |
| `ScreenSpaceTransmission`, `*_transmission` | `bevy_pbr/src/transmission/`, `pbr_material.rs` | Colour copy per step, no depth/MV |
| `VolumetricFog`, `VolumetricLight` | `bevy_pbr/src/volumetric_fog/` | Raymarched; we have froxels |
| `MotionBlur`, `DepthOfField`, `Bloom`, `AutoExposure` | `bevy_post_process/src/` | After DLSS; MB needs MV prepass |
| `ChromaticAberration`, `LensDistortion`, `Vignette` | `effect_stack/` | no film grain |
| `OrderIndependentTransparency` | `bevy_core_pipeline/src/oit/` | memory heavy, no MVs |
| `ForwardDecal`, clustered decals | `bevy_pbr/src/decal/` | `DepthPrepass` |
| `FullscreenMaterial` | `bevy_core_pipeline/src/fullscreen_material.rs` | cheapest route for grain, wet-only SSR |
| `TemporalAntiAliasing`, `Dlss` SR/RR | `bevy_anti_alias` | `EarlyPostProcess` |

The 0.20 release notes I could read cover only Solari for rendering (RT reflections less shimmery, DLSS-RR 4.5); raster
facts above are from source.

## 6. What we would write or patch

Bloom preset on the game camera (config). `WorldMaterial` transmission fields (cook + shader). Surface-state wetness and
puddles (specced). Water SSPR (specced). Later: wet-only SSR and film grain as `FullscreenMaterial`s. No new Bevy patch
unless a depth-reading pass is added, which follows RO-11. Not recommended: our own hi-Z SSR or a WGSL port of SSSR
before the owner says reflections of buildings matter.

## 7. Crates for Bevy 0.20 (crates.io, 2026-11-01)

| Crate | State | Verdict |
|---|---|---|
| `bevy_post_process` 0.20.1 | Bevy's own | in use via `bevy` |
| `sgl-post-fx` 0.5.0 | Apache-2.0, 155 downloads; SSR + TAA from DiligentFX for the `sgl` wgpu engine, not Bevy | reference only |
| `bevy_ridiculous_ssgi` (GitHub) | last push 2024-02-22, no licence, deferred-only | dead |
| `bevy_weather` 0.2.0 | MIT OR Apache-2.0, 67 downloads | not evaluated; low maturity |
| `bevy_grass` 0.20.6 | MIT, active | Phase 4 groundcover, unrelated |
| `bevy_hanabi` 0.19.0 | MIT OR Apache-2.0 | waits for 0.20 (update-batch gate) |
| `bevy_water` 0.18.1, `bevy_aqua` 0.1.3 | MIT OR Apache-2.0 | evaluated in `water.md` |
| FidelityFX SSSR 1.1.4, XeGTAO | MIT, HLSL | algorithm references, not crates |

No maintained Bevy 0.20 crate for SSR, SSGI or wet-surface post effects turned up (searched ssr, ssgi, screen space,
god rays, post process, subsurface, wet). `bevy_atmos` (borrow-audit) is a GitHub project, not on crates.io.

## 8. Risks and open questions

- The deferred dither question gates any Bevy SSR on raster; deferred cost unmeasured.
- Every (est.) is unmeasured; `CONTRIBUTING.md` wants ms per preset from any perf slice.
- Wet-only SSR with analytic roughness has one precedent and no public 1440p DLSS number; 0.4-0.8 ms is a guess.
- Crimson Desert, Yotei, HFW, KCD2 screen-space detail is secondary or absent: do not cite them as precedent for foliage
  SSR or SSS. A real Crimson Desert answer needs the DF video watched (transcripts unreachable) or a Pearl Abyss talk I
  could not find.
- Bloom vs auto-exposure, motion-blur feel in combat, DoF under DLSS jitter: owner look checks, not tests.
- Owner questions: (a) buildings in puddles worth a Phase-4 pass beyond probe fidelity? (b) leaf translucency in Phase 3
  or 4? (c) bloom default on at High?

## 9. Sources

- Bevy 0.20.0 source under `~/.cargo/registry/src/*/` (`bevy_pbr`, `bevy_post_process`, `bevy_core_pipeline`,
  `bevy_anti_alias`); https://bevy.org/news/bevy-0-20/
- Repo: `superfargoth/vendor/README.md`; `docs/specs/{sky-lighting,shadows,water,render-order,weather,industry-audit}.md`;
  `docs/specs/weather/precipitation-surface.md`; `docs/research/borrow-audit-2026-10.md`
- NVIDIA DLSS Programming Guide (2026-03-31): https://github.com/NVIDIA/DLSS/blob/main/doc/DLSS_Programming_Guide_Release.pdf
- AC Shadows, SIGGRAPH 2025: https://advances.realtimerendering.com/s2025/content/Advances%202025%20-%20Raytracing%20the%20world%20of%20Assassin's%20Creed%20Shadows.pdf
- Patry, Ghost of Tsushima, SIGGRAPH 2021: https://www.advances.realtimerendering.com/s2021/jpatry_advances2021.pdf
- Epic: https://dev.epicgames.com/documentation/en-us/unreal-engine/lumen-technical-details-in-unreal-engine ,
  .../lumen-performance-guide-for-unreal-engine , .../contact-shadows-in-unreal-engine ; Fortnite C4 blog
  https://www.unrealengine.com/tech-blog/lumen-brings-real-time-global-illumination-to-fortnite-battle-royale-chapter-4
- AMD SSSR: https://gpuopen.com/fidelityfx-sssr/ , https://gpuopen.com/manuals/fidelityfx_sdk/techniques/stochastic-screen-space-reflections/
- XeGTAO https://github.com/GameTechDev/XeGTAO ; Separable SSS https://iryoku.com/separable-sss ; Bend
  https://www.bendstudio.com/blog/inside-bend-screen-space-shadows/
- Crimson Desert: https://www.pcgamesn.com/crimson-desert/pc-ray-tracing-upgrade ,
  https://finalboss.io/crimson-desert-on-ps5-pro-keeps-its-wild , https://80.lv/articles/pearl-abyss-demonstrated-crimson-desert-s-blackspace-engine-tech-advancements
- RDR2 study https://imgeself.github.io/posts/2020-06-19-graphics-study-rdr2 ; Future of SSR
  https://www.gamedeveloper.com/programming/the-future-of-screenspace-reflections
- Elden Ring https://www.pcgameshardware.de/Elden-Ring-Spiel-67687/Specials/PC-Performance-Anforderung-Release-Test-Benchmark-1390102/ ,
  https://80.lv/articles/rt-in-elden-ring-better-for-screenshots-than-gameplay-says-digital-foundry
- OpenMW https://modding-openmw.com/mods/omwfx-shaders , https://gitlab.com/vtastek/omwfx-shaders ,
  https://openmw-vr.readthedocs.io/en/latest/reference/postprocessing/overview.html (direct fetch 404; via search summary)
- KCD2 https://gamingbolt.com/kingdom-come-deliverance-2-tech-analysis-ps5-vs-pc-comparison-graphics-overview-and-more ;
  CryEngine SSR https://cryengine.com/docs/static/engines/cryengine-3/categories/9895942/pages/9219181
- Cited from memory, not re-read: McGuire and Mara, Efficient GPU Screen-Space Ray Tracing (JCGT 2014); Stachowiak,
  Stochastic Screen-Space Reflections (2015); Wronski, Volumetric Fog (2014); Hillaire, Frostbite volumetric fog (2015);
  Barre-Brisebois and Bouchard, Approximating Translucency (GDC 2011); Lagarde, Water drop 3a/3b; Sousa, Crysis vegetation
  (GPU Gems 3 ch.16); Wyman and McGuire, Hashed Alpha Testing (2017).
