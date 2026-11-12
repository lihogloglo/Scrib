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
- **Interiors.** Seamless engines put interiors inside the exterior partition, so there
  is no worldspace swap. Footnote: Bethesda engines and OpenMW use load doors plus preloading (OpenMW preload
  distance 1000 units, up to 20 cached cells,
  [OpenMW cells settings](https://openmw.readthedocs.io/en/stable/reference/modding/settings/cells.html)).

## Who ships it
| Engine | What it does | Source |
|---|---|---|
| UE5 | World Partition grids, data layers, HLOD, large world coordinates | UE docs above |
| Cyberpunk 2077 | A WorldStreaming system streams "sectors" (its job chain starts once the camera position is known), then a NodeStreaming system streams nodes and entities in and out. Resource decompression runs on the lowest-priority jobs, throttled, and no frame job waits on it | [Tremblay, CDPR, GDC 2024](https://media.gdcvault.com/gdc2024/Slides/GDC+slide+presentations/Tremblay_Charles_Anatomy_of_a.pdf); earlier talk [Tremblay, GDC 2023](https://gdcvault.com/play/1028981/Building-Night-City-The-Technology) |
| GTA V (RAGE; RDR2 not found) | Assets stream in and out in real time; far areas are low-poly stand-ins (Vinewood Hills is one draw call of about 2500 triangles); an archetype `lodDist` is the distance at which the object unloads | [Courrèges, GTA V graphics study part 2, 2015](https://www.adriancourreges.com/blog/2015/11/02/gta-v-graphics-study-part-2/); [Sollumz archetype docs](https://docs.sollumz.org/documentation/archetype-definition.ytyp) (modding docs, secondary) |
| Horizon (Decima) | Streaming groups define the tiles of the static render scene; tiles are built on the streaming thread and the main thread only activates ready ones. Separate talk on low and high level streaming, memory and scheduling (no numbers on its page) | [Guerrilla, Decima visibility, 2017](https://www.guerrilla-games.com/read/decima-engine-visibility-in-horizon-zero-dawn); [Guerrilla, Streaming the World of Horizon Zero Dawn, 2023](https://guerrilla-games.com/read/Streaming-the-World-of-Horizon-Zero-Dawn); placement: [van Muijden, GDC 2017](https://gdcvault.com/play/1024429/Between-Tech-and-Art-The) |
| Unity | Entities subscenes stream asynchronously, split into sections that load and unload individually | [Unity Entities streaming overview](https://docs.unity3d.com/Packages/com.unity.entities@1.3/manual/streaming-overview.html) |
| Bethesda, OpenMW | Load doors, preload, object paging (footnote) | OpenMW link above |

## Key numbers
- No default World Partition cell size is stated in the UE docs we found, so none is quoted here; read it from the runtime grid settings.
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
- UE, OpenMW, CDPR, Guerrilla and Courrèges links in the table above. Web research done 2026-11-12; Decima streaming numbers and RDR2 streaming details were not found.
