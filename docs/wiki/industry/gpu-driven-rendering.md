# GPU-driven rendering

## The standard technique
A **retained GPU scene**: per-instance data (transform, bounds, material ids) lives in GPU buffers and is uploaded
only when it changes. Culling (frustum, then occlusion against a hierarchical depth buffer) runs in compute **per
view** (camera, every shadow cascade, every cube face) and writes indirect-draw arguments; the CPU issues a handful
of multi-draw-indirect calls. CPU cost scales with changed instances and batch count, not entities times views.
- Two-phase occlusion culling: [Haar and Aaltonen, SIGGRAPH 2015](https://advances.realtimerendering.com/s2015/aaltonenhaar_siggraph2015_combined_final_footer_220dpi.pdf);
  Wihlidal, Frostbite, GDC 2016.
- Cluster geometry: UE5 Nanite ([Karis, SIGGRAPH 2021](https://advances.realtimerendering.com/s2021/Karis_Nanite_SIGGRAPH_Advances_2021_final.pdf)).
- Binned materials plus bindless textures remove per-draw binds.

## Who ships it
| Engine | What it does | Source |
|---|---|---|
| UE5 | GPUScene with dirty-primitive updates, GPU instance culling, Nanite, ISM/HISM | UE docs, Karis 2021 |
| Cyberpunk 2077 | GPU-driven culling and instancing for the city (no public detail read) | unverified |
| RDR2 (RAGE) | No public GPU-driven talk read | unverified |
| Decima | Not verified | unverified |
| Unity 6 | GPU Resident Drawer (Batch Renderer Group) with GPU occlusion culling | [Unity docs](https://docs.unity3d.com/Manual/gpu-resident-drawer.html) |
| Frostbite, AC Unity, id Tech 7 | GPU culling, compute triangle filtering, cluster culling | Wihlidal 2016, Haar and Aaltonen 2015 |
| Bethesda Creation, OpenMW | Scene-graph nodes plus offline merge (precombines, object paging): the CPU-bound baseline, not the target | audit row 10 |

## Key numbers
- Bevy 0.20 per-entity CPU work scales as N entities x V views (camera, 4 cascades, cube faces). Balmora has about
  48k resident mesh entities; the per-system breakdown is in `superfargoth/docs/specs/industry-audit.md`
  ("How Bevy 0.20 actually spends per entity").
- Bevy's own `bevy_city` stress scene uses `NoCpuCulling` plus static transform optimisations
  (`superfargoth/docs/research/perf/README.md` Q5).
- Our frame target: High, 1440p DLSS Quality, 4060 laptop, at most 11 ms median in exteriors (CONTRIBUTING.md).

## Where Scrib stands
Bevy 0.20 already does GPU preprocessing, multi-draw indirect and bindless, so identical mesh plus material becomes
one indirect draw. Missing against the standard: CPU visibility still runs per entity per view unless the entity has
`NoCpuCulling`, and phase items are per entity. We use the opt-in `NoCpuCulling` path on near statics and are
building a retained static scene (upstream bevy#25978) so phase items become O(batch sets). No meshlet path (Bevy
meshlets rejected, audit row 32). See [engine/statics-and-lod](../engine/statics-and-lod.md),
[engine/frame-and-views](../engine/frame-and-views.md), [bevy/render-and-visibility](../bevy/render-and-visibility.md),
[ledger/perf](../../ledger/perf.md) and [ledger/statics](../../ledger/statics.md).

## Sources
- `superfargoth/docs/specs/industry-audit.md` rows 3, 10-12, 15, 17, 30, 32; `superfargoth/docs/specs/static-scene.md`
  "Industry comparison"; `superfargoth/docs/research/perf/README.md` Q5.
- Talks linked above. No web search was run; CDPR, RAGE and Decima rows are unverified.
