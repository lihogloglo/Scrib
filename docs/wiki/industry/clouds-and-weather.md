# Clouds and localized weather

**Shipped games never draw a local storm as a column bounded by a region edge.** Clouds live in absolute altitude layers and
take their outline from a soft, noise-modulated coverage map; rain and low fog are *derived from cloud density and
precipitation under the cloud*, so the curtain has the cloud's ragged outline and fades toward its edge. Terrain
"anchors" clouds only by poking through them. Survey of 2026-11-12. Primary sources were read (Nubis decks); RDR2 and
Frostbite details are from memory or secondary sources and marked (unverified).

## Guerrilla Nubis lineage
| Step | Mechanism | Source |
|---|---|---|
| HZD 2015 | World-space 2-D weather map (R coverage, G precipitation, B type) x vertical profile per cloud type x tiling 3-D noise erosion; half-res raymarch with 16-frame temporal reprojection; ~2 ms budget. Precipitation also darkens the cloud and starts rain at the player (near and far read one map) | [Schneider 2015](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf) |
| Nubis 2017 | Authoring tools on the same model | [Schneider 2017](https://advances.realtimerendering.com/s2017/Nubis%20-%20Authoring%20Realtime%20Volumetric%20Cloudscapes%20with%20the%20Decima%20Engine%20-%20Final%20.pdf) |
| Nubis Evolved 2022 (HFW) | Authored/generated "Nubis data fields" (NDFs, 2-D maps) per cloud genus; **regional influence NDFs** (masks) blend per-region cloudscapes and weather states; vertical-profile model for ground views, **envelope model** (3-D bottom x top x edge gradient) for flying through; superstorm = world-space swirl blended by `pow(saturate(1 - d/R), 0.1)` (a wide, soft falloff, not a band); lightning = positioned flash + in-cloud glow + a stable mask so the upscaler keeps it | [Nubis Evolved](https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-NubisEvolved-NoVideos.pdf), [GDC superstorms](https://gdcvault.com/play/1027688/The-Real-Time-Volumetric-Superstorms) |
| Nubis3 2023 (Burning Shores) | Voxel clouds: fluid sim + Houdini "Frankenclouding" into a 500 m-scale voxel map, **up-rezzed at sample time** with noise as value erosion (0.5 m precision); 10 ms at 960x540 for the density sampler; cached summed-density lighting every 8 frames (0.1-0.2 ms amortized, -40% render). Free-form, flight-capable, interior ("not just a cave"), bespoke boss storms; reuses the superstorm local-weather system | [Nubis3](https://advances.realtimerendering.com/s2023/Nubis%20Cubed%20(Advances%202023).pdf) |

**How terrain "anchoring" works (inferred from the decks; they show it in images, not text).** Nothing special: cloud layers
are at fixed world altitudes (stratus low, cumulus ~1.5 km, cirrus 8 km), so a mountain simply intersects them and
the depth-aware composite hides what is behind rock. Mountain-hugging and low clouds are a low-altitude layer with a
flat base gradient plus noise erosion and the same coverage map; valley fog is the fog bank under it. The slides'
"Terrain-cast shadows" row: vertical profile method no, envelope and voxel yes.

## RDR2 (Bauer, SIGGRAPH 2019)
[Course page](https://advances.realtimerendering.com/s2019/index.htm): "Creating the Atmospheric World of RDR2: A Complete
and Integrated Solution". Slides not fetched (site timed out); no primary text confirmed. Secondary and inference only
(unverified): clouds and volumetric fog are one integrated system, quarter-res clouds with a 4x4 temporal grid, fog
density tied to the cloud map. We take the *principle* (rain fog under cloud density), not details.

## Others
- **UE5**: Volumetric Cloud (layer between two altitudes, world-placed pattern texture, per-channel cloud types) + Volumetric Fog + **Local Fog Volumes** (soft-edged ellipsoid/box fog with falloff; the standard answer for a localized fog, [docs](https://dev.epicgames.com/documentation/unreal-engine/volumetric-cloud-material-in-unreal-engine)). Weather volumes blend at edges.
- **Frostbite** (Hillaire 2016, unverified detail): the same weather-texture model, clouds at reduced res with temporal reprojection; physically based sky unified with fog.
- **MSFS**: gridded weather interpolated into volumetric layers (community sources).
- **Decima/Death Stranding, Ghost of Tsushima**: no cloud talk found; skip.

## Fit for us
- Our cost class matches HZD (half-res march + temporal); no engine here needs more than a soft map and a profile.
- **Our region-mask + far-shaft design is the wrong shape** (audit C3): a mask with a 256 m band and shafts at in-storm
  extinction is a Local-Fog-Volume-style hard box. Replace with: soft km-scale coverage falloff (superstorm-style
  `pow(1 - d/R, 0.1)` or noisy), shaft density = coverage x precipitation under the cloud base, ragged bottom.
- Shimmer: a high-contrast opaque edge sampled by few steps is aliasing; DLSS cannot fix an edge that flips per frame.
  Soft edges remove it at the source (history clamps already failed, ledger 2026-11-06).
- Low clouds and terrain: add a low stratus layer in the same march plus the existing fog banks; no terrain coupling needed.
- Interior/above-deck: envelope-style 3-D gradient, not vertical profile (WE-12, diagnosed elsewhere).
- Volcano plume and lightning: a world-space envelope/swirl volume at Red Mountain; HFW-style bolt + in-cloud glow + stable mask.
