# Open-world streaming

Comparison targets: Cyberpunk 2077 (REDengine 4), RDR2 (RAGE), Horizon (Decima) and current UE5 open worlds.
Older engines (Bethesda, OpenMW) appear only as footnotes, never as the goal.

## The standard technique
- **Spatial partition, streamed by distance, per-source ranges.** The world is cut into cells; each streaming source
  (player, camera, fast-travel target) loads cells inside a radius and unloads with hysteresis. UE5 World Partition
  uses runtime grids, each with its own loading range (small actors on a short-range grid), and a cell state
  "loaded, hidden" before "activated"
  ([UE World Partition](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)).
- **HLOD beyond the load radius.** Far cells are replaced by merged, simplified proxies (UE HLOD layers, with an
  instancing layer and impostors for trees). The near/far handoff is dithered.
- **Time-budgeted integration.** Finished loads are applied under a ms budget per frame (UE
  `s.LevelStreamingActorsUpdateTimeLimit`); uploads are paced by bytes per frame (UE texture and mesh streaming pools).
- **Size-based load and draw distance.** Small objects load and draw only close (UE per-primitive cull distance and
  Cull Distance Volumes; RAGE per-entity `lodDist`).
- **Large-world coordinates.** UE5 keeps double-precision positions and makes them camera-relative on the GPU; older
  origin rebasing re-uploads everything.
- **Interiors.** Seamless engines (Witcher 3, Cyberpunk 2077) put interiors inside the exterior partition, so there
  is no worldspace swap. Footnote: Bethesda engines and OpenMW use load doors plus preloading (OpenMW preload
  distance 1000 units, up to 20 cached cells,
  [OpenMW cells settings](https://openmw.readthedocs.io/en/stable/reference/modding/settings/cells.html)).

## Who ships it
| Engine | What it does | Source |
|---|---|---|
| UE5 | World Partition grids, data layers, HLOD, large world coordinates | UE docs above |
| Cyberpunk 2077 | Sector streaming of the city, seamless interiors, per-category distances | unverified (memory; no CDPR talk read for this page) |
| RDR2 / GTA V (RAGE) | Cell streaming; `lodDist` is the unload distance; SLOD merged far models | unverified (audit row 2 states the `lodDist` rule) |
| Horizon (Decima) | Streamed terrain and objects; GPU-placed foliage | [van Muijden, GDC 2017](https://gdcvault.com/play/1024429/Between-Tech-and-Art-The) (placement only; streaming unverified) |
| Unity | Additive scene streaming, DOTS subscenes | unverified |
| Bethesda, OpenMW | Load doors, preload, object paging (footnote) | OpenMW link above |

## Key numbers
- UE World Partition default cell size 128 m: unverified (memory).
- Bevy re-collects every instance on an origin shift (3-4 ms), the spike class a camera-relative GPU path avoids
  (`superfargoth/docs/specs/industry-audit.md` row 29).
- Scrib's own rings and budgets: `engine/streaming.md`.

## Where Scrib stands
We match the partition, rings, hysteresis, time budget and fade-hidden bursts (audit rows 18-21). Gaps against current
engines: size-tiered loading (row 2), upload pacing by bytes (row 20), and rebasing without a full re-upload (row 29).
Interiors stay separate worldspaces because Morrowind interiors overlap exterior coordinates; we borrow UE's
"loaded, hidden" prefetch state. See [engine/streaming](../engine/streaming.md),
[engine/statics-and-lod](../engine/statics-and-lod.md) and [ledger/streaming](../../ledger/streaming.md).

## Sources
- `superfargoth/docs/specs/industry-audit.md` rows 1-3, 18-21, 29-30, 34; `superfargoth/docs/specs/streaming.md`
  "Industry comparison".
- UE and OpenMW links above. No web search was run; rows marked unverified come from memory.
