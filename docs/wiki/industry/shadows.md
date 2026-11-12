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
| Cyberpunk 2077 | Sun cascades, budgeted local-light shadows, optional RT shadows | unverified (memory) |
| RDR2 | Cascaded sun shadows, cached far content, local-light budget | unverified (memory) |
| Decima | Not verified | unverified |
| Unity HDRP | Shadow atlas, cached shadows, max shadowed lights and fade distance | our spec table, unverified detail |
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
- No web search was run; CDPR, RDR2 and Decima rows are unverified.
