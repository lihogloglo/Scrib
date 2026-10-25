# Sky and lighting: history

Moved out of [sky-lighting.md](../sky-lighting.md) on 2026-10-26 (doc diet S12a), word for word. Headings are unchanged. sky-lighting.md and the code win where they differ.

## Environment map refresh (lights1, 2026-10-20)

Bevy 0.20 re-filters the sky's `AtmosphereEnvironmentMapLight` (a `GeneratedEnvironmentMapLight`) every frame:
Nsight on the Balmora walk, 1440p: `lightprobe_irradiance_map` 0.54 ms + `lightprobe_radiance_map` 0.29 ms of GPU.
`sf_sky_bevy::EnvMapRefresh(n)` (default 8) re-filters every n-th frame and keeps the last result in between (UE
sky light capture time slicing, HDRP probe update modes): the render system drops Bevy's `RenderEnvironmentMap`
after extraction on the other frames; a new camera is filtered on its first frame. Balmora 1440p 13.3 → 12.4 ms,
960p 9.5 → 8.9 (GPU bound). A/B `--off envslice`. Left as is: `atmosphere_luts` 0.69 ms per frame (Bevy recomputes
transmittance and multiscattering though the medium rarely changes; an upstream change, not ours).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
Bevy features checked in the 0.20 source (`bevy_pbr`, `bevy_light`,
`bevy_post_process`). "Unverified" marks claims from memory not re-read
this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Physically based sky + aerial perspective | Hillaire 2020 LUTs: UE4.26+/UE5 SkyAtmosphere (transmittance, multi-scattering, sky-view and camera aerial-perspective froxel LUTs); Frostbite (Hillaire's origin, 2016 → 2020); Unity HDRP physically based sky (precomputed, Bruneton-style, unverified); RDR2 own atmosphere (Bauer, SIGGRAPH 2019 Advances, unverified) | Bevy `Atmosphere` (same LUT set incl. a 32 km aerial-view LUT), our medium | **match** |
| Sky as environment lighting | UE SkyLight real-time capture; Unity HDRP sky ambient probe | `AtmosphereEnvironmentMapLight` (filtered from the atmosphere) | **match** |
| Sun/moon from clock and latitude | UE SunPosition plugin (date, latitude); most open worlds use a hand-authored time-of-day curve instead | ephemeris + curves | **match** (more physical than most) |
| Exposure | UE: histogram auto exposure, physical camera, **exposure compensation curve** by average luminance, local exposure (5.1+); Unity HDRP: automatic histogram or fixed/physical, curve mapping; Bethesda/OpenMW: no physical exposure (LDR-era lighting, eye adaptation as a post effect) | scheduled EV100 by sun elevation; with portals, `AutoExposure` with a compensation curve (portals.md); `ExposureRange` min/max EV100 on both (2026-10-13) | **match** (the portal2 setup is the UE pattern). **Local exposure** (UE 5.1, bilateral-grid tone mapping) is not in Bevy 0.20: helps interior-looking-out shots; noted, not a job |
| Tone mapping | UE filmic (ACES-fitted), Unity ACES/Neutral, Blender/Godot AgX | Bevy default `TonyMcMapface` | **match** (a curve choice, not a technique gap) |
| Shadows | UE virtual shadow maps (+ CSM fallback); Unity CSM + cached shadows; RAGE CSM + far shadow (unverified); Decima CSM + cached far cascades (unverified) | 4 CSM + terrain far shadow (weather/distant-shadows.md) | **match** for CSM; VSM / cached far cascades = audit row 14 (P2) |
| Local-light shadows | Unity HDRP: shadow atlas, max shadows on screen, per-light shadow fade distance; UE: per-light shadow fade, small-caster culling by projected size; Bethesda: a small cap of shadowed lights; OpenMW: none (from docs as remembered, not re-read) | `PointShadowBudget`: N cube maps by distance over priority, hysteresis, distance + time fade (light split) | **match** for the budget and fade; **gap**: no caster culling per local-light shadow view (Bevy draws every caster in range on 6 faces: the measured render-thread cost) |
| Contact shadows | UE per-light contact shadows; Unity HDRP contact shadows; Bevy 0.20 `ContactShadows` (needs depth prepass) | **off** (prepass off in exteriors) | **GAP (P1)**: open question 6 |
| Ambient occlusion | UE SSAO/GTAO (+ Lumen short-range AO); Unity HDRP GTAO; Fallout 4 HBAO+, Skyrim SE SSAO (unverified); Bevy 0.20 SSAO = XeGTAO port (needs depth + normal prepass) | **none** on raster (Solari does it implicitly) | **GAP (P1)**: open question 6. Exteriors also lack large-scale sky occlusion: weather/distant-shadows.md's terrain sky-visibility bake covers that |
| Indirect diffuse (GI) | UE5 Lumen (software/hardware RT), volumetric lightmaps; Unity Adaptive Probe Volumes with lighting scenarios blended at runtime (the time-of-day pattern), Enlighten; RAGE/Decima baked probes per time of day (unverified detail); Bethesda Skyrim/FO4: none (ambient per cell/region), Starfield real-time GI (unverified); OpenMW: per-cell ambient only | raster: ambient floor + sky env map; ray-traced: Solari ReSTIR GI | **match** on the high end (Solari = Lumen class). **Differ** on raster: we match Bethesda/OpenMW, not modern raster engines. Interior baked probes: **GAP (P2)**, open question 7. Exterior time-of-day probes: skipped (sky env map + sky-visibility bake is the cheap equivalent) |
| Reflections | UE Lumen reflections / SSR + reflection captures; Unity SSR + reflection probes | env map; SSR available in Bevy, not wired | **differ**: source content is mostly diffuse; revisit with PBR generation (M7) |
| Many-light culling | clustered/tiled forward or deferred everywhere; UE MegaLights (5.5, stochastic) | Bevy clustered forward; Solari for RT | **match** |
| Bloom / glare | universal | not on game cameras | **GAP (P2, small)**: open question 8 |
| Upscaling | DLSS/FSR/XeSS/TSR | DLSS (SR + RR); Bevy TAA as the base and fallback | **match** on NVIDIA; vendor-neutral path open (above) |

**Sources** (checked 2026-10-12):
- Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique", EGSR 2020: https://sebh.github.io/publications/egsr2020.pdf
- UE, Sky Atmosphere component: https://dev.epicgames.com/documentation/en-us/unreal-engine/sky-atmosphere-component-in-unreal-engine
- UE, Auto exposure / local exposure (Eye Adaptation): https://dev.epicgames.com/documentation/en-us/unreal-engine/auto-exposure-in-unreal-engine (page not re-read; unverified detail)
- Unity, Adaptive Probe Volumes lighting scenarios and blending: https://docs.unity3d.com/Manual/urp/probevolumes-bakedifferentlightingsetups
- Unity, APV lighting panel: https://docs.unity3d.com/Manual/urp/probevolumes-lighting-panel-reference
- Intel XeGTAO (Bevy's SSAO source): https://github.com/GameTechDev/XeGTAO
- Bevy 0.20 source: `bevy_pbr/src/{ssao, contact_shadows.rs, light_probe/irradiance_volume.rs, atmosphere}` (irradiance volumes: "Bevy doesn't currently come with a way to bake"; points at Blender + `bevy-baked-gi`).
- Mitchell, "Shading in Valve's Source Engine", SIGGRAPH 2006 course (ambient cubes).
