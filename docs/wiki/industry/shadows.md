# Shadows

## The standard technique
- **Sun: cascaded shadow maps** with a practical log/linear split, stabilised by sphere fit and texel snapping,
  blended at borders, softened by PCF or PCSS
  ([GPU Gems 3 ch. 10](https://developer.nvidia.com/gpugems/gpugems3/part-ii-light-and-shadows/chapter-10-parallel-split-shadow-maps-programmable-gpus);
  [MJP](https://therealmjp.github.io/posts/shadow-maps/)).
- **Current UE5: Virtual Shadow Maps.** A huge virtual map per light, split into pages rendered only where visible
  pixels need them, with **page caching** so static geometry is not redrawn; Nanite geometry renders into it
  ([UE VSM](https://dev.epicgames.com/documentation/en-us/unreal-engine/virtual-shadow-maps-in-unreal-engine)).
- **Cached static shadows** for far cascades and local lights; only movers redraw (Doom 2016, HDRP, UE).
- **Local lights: a budget.** A few shadowed lights chosen by screen contribution, distance fade, an atlas with
  per-light resolution; the rest unshadowed (UE, HDRP, Frostbite).
- **Contact shadows**, a short screen-space ray for small detail
  ([UE](https://dev.epicgames.com/documentation/en-us/unreal-engine/contact-shadows-in-unreal-engine); Days Gone, SIGGRAPH 2023).
- **Far shadows** past the last cascade: distance-field or heightfield shadows, or ray-traced shadows on top tier.
- **Caster culling by projected size** per cascade (UE `r.Shadow.RadiusThreshold`).

## Who ships it
| Engine | What it does | Source |
|---|---|---|
| UE5 | VSM with page cache, distance-field far shadows, contact shadows, optional hardware RT shadows | UE docs above |
| Cyberpunk 2077 | Sun and moon: up to 4 per-frame cascades (interval-based selection, culling shape of up to 8 planes, main-view and shadow-view occlusion) plus up to 4 "distant" cascades refreshed over several frames, the last covering the whole 256 km2 map. Offline time-of-day visibility (12 steps, 2 h) drops casters wholly inside others' shadows. Local lights: a fixed shadow-map budget per frame picked by visibility and distance, throttled refresh, static and dynamic slices per map. RT shadows are optional (NVIDIA) | [Dybisz and Witanowski, CDPR, SIGGRAPH 2021](https://history.siggraph.org/wp-content/uploads/2022/06/2021-Talks-Dybisz_Shadows-Optimizations-in-Cyberpunk-2077.pdf); [NVIDIA](https://www.nvidia.com/en-us/geforce/news/cyberpunk-2077-ray-tracing-dlss-geforce-now-screenshots-trailer.md) |
| RDR2 | Sun: 4 cascades in one atlas (1024 px tiles at medium). Local lights: spot and point shadow arrays in three resolution tiers (up to 2048x3072 spot at high); most static point lights use baked shadow cubemaps, runtime maps only near the player | [imgeself, RDR2 graphics study, 2020](https://imgeself.github.io/posts/2020-06-19-graphics-study-rdr2) (frame capture, third party) |
| Decima | Shadow technique not found in public slides we could read; talks cover area lights and AA ([Guerrilla/Kojima, SIGGRAPH 2017](https://www.advances.realtimerendering.com/s2017/DecimaSiggraph2017.pdf)). Visibility queries for shadow maps are separate from the camera query and skip occlusion ([Guerrilla, GCAP 2017](https://www.guerrilla-games.com/read/decima-engine-visibility-in-horizon-zero-dawn)) | see links |
| Unity HDRP | Cached shadow maps with an On Demand update mode; point and area lights have their own cached atlas, directional cached shadows share the directional atlas | [HDRP shadow update mode](https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@16.0/manual/shadow-update-mode.html) |
| bevy_terrain | Heightfield-raymarched terrain shadow, called "RDR2-style" in its source | `docs/research/bevy-open-world-projects-2026-11-12.md` |

## Key numbers
- Bevy 0.20: at most 4 cascades; ours span 10 m (first bound) to 504 m
  (`superfargoth/docs/research/shadows/README.md` s3).
- Bevy `ContactShadows` default: 16 steps, 0.3 m ray.
- Mudcrab (Bevy, Skyrim data): 4 cascades at 2048 squared, `maximum_distance` fitted to the streamed radius; its doc
  names distance, then cascade count, then map size as the levers (`superfargoth/docs/research/perf/README.md` Q5).
- Our local lights: one shadowed point light on High.

## Where Scrib stands
Match: shadowed-light budget with importance and hysteresis, static caster caching (vendored bevy PRs #26149 and
#26161 skip unchanged views), selection by contribution including off-screen lights. Gaps against UE5: no virtual map
or atlas with per-light resolution (one cube size), no per-cascade size-based caster culling, no distance-field far
shadows beyond terrain, no cascade cache in Bevy. Contact shadows: sun and moon only. See
[engine/shadows](../engine/shadows.md), [engine/lights](../engine/lights.md) and [ledger/shadows](../../ledger/shadows.md).

## Sources
- `superfargoth/docs/specs/shadows.md` "Industry comparison"; `superfargoth/docs/research/shadows/README.md` s3;
  `superfargoth/docs/specs/industry-audit.md` rows 13-14.
- Web research done 2026-11-12. Not found: RDR2 cached far-cascade detail, Decima shadow cascade and local-light detail, HDRP max shadowed lights.
