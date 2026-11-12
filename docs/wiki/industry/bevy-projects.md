# Other Bevy open-world projects

**No Bevy project we found operates at the scale of the target (Cyberpunk 2077, RDR2, UE5 open worlds), and none
publishes frame times for a large world with many static meshes and many NPCs.** What follows is a technique source,
not a quality reference. Survey of 2026-11-12 from shallow clones; details with file paths in
`docs/research/bevy-open-world-projects-2026-11-12.md`.

## The standard technique
Bevy projects converge on: terrain as a custom GPU-refined phase or per-chunk LOD meshes; streaming by distance
rings with a per-frame job budget; many small things merged into per-chunk meshes; custom draws only for "one big
draw with GPU LOD" while keeping `StandardMaterial` so lighting, shadows and deferred still apply.

## Who ships it
| Project | Licence, Bevy | What it teaches |
|---|---|---|
| bevy_terrain (mate-h) | Apache/MIT, 0.20 | Own render phase, GPU tile refinement, one indirect draw per view, heightfield-raymarched terrain shadow, optional `big_space` |
| bevy_voxel_world | Apache/MIT, 0.20 | Streaming policy as a pure function over camera snapshots, async meshing with a pool budget, 16 LOD levels. Best reusable streaming reference |
| riverbed | MIT, 0.20.1 | Replaced Bevy CPU culling with its own parallel system on `NoFrustumCulling`; per-entity cost bites at thousands of entities (inference) |
| bevy_meadow | MIT, main | GPU grass: compute culling and LOD per view, a few `draw_indexed_indirect` calls, `ExtendedMaterial` |
| big_space | Apache/MIT, 0.19 | Floating origin with integer grids; no rendering. Bench: 100k entities |
| evroon/bevy-open-world | GPL-3.0, 0.19 | CPU quadtree terrain, OSM buildings merged per chunk |
| Ethertum | GPL-3.0, 0.19.1 | Voxel multiplayer, entity per chunk, own GI next to Solari |
| Mudcrab | MIT or Apache, 0.19 | Skyrim data viewer. Stock rendering with `OcclusionCulling`, 4 cascades at 2048, `maximum_distance` fitted. Its GPU-driven startup check is worth copying as a gauge (idea only) |
| rust-gta-3 | custom, 0.20 | **Closest Bevy architecture to ours (entity per instance)**, nothing more. It works because GTA III's visible set is a few hundred objects, unlit, with blob shadows. Not a target or quality reference |

Bevy's own `bevy_city`, `many_cubes` and `many_foxes` are stress scenes (flags `--no-cpu-culling`, GPU culling
switches; `many_foxes` has 1000 skinned foxes). Tiny Glade is built on Bevy ([80.lv interview](https://80.lv/articles/exclusive-tiny-glade-developers-discuss-bevy-proceduralism-publishers-cozy-games)); that it has its own renderer is only community-reported.


## Key numbers
- None published by these projects. Scrib's own numbers are in `ledger/perf.md`; our target is at most 11 ms median
  on High (1440p, DLSS Quality, 4060 laptop).
- Nobody in public Bevy code handles hundreds of skinned NPCs.
- pcwalton's retained-rendering PR series (#16385 to #25058) is the upstream programme to ride
  (`superfargoth/docs/research/perf/README.md` Q5).

## Where Scrib stands
Ahead of public Bevy work on measured frame time for a dense static scene, but far below the target engines'
techniques: see [gpu-driven-rendering](gpu-driven-rendering.md), [open-world-streaming](open-world-streaming.md) and
[shadows](shadows.md). Borrow ideas only (streaming policy as pure data from bevy_voxel_world, a startup GPU-path
gauge from Mudcrab); copy no code from the GPL projects without the licence check. See also
[engine/statics-and-lod](../engine/statics-and-lod.md) and [ledger/statics](../../ledger/statics.md).

## Sources
- `docs/research/bevy-open-world-projects-2026-11-12.md`; `superfargoth/docs/research/perf/README.md` "Q5. Big Bevy projects".
- Not searched: r/bevy, Discord showcase, This Week in Bevy.
