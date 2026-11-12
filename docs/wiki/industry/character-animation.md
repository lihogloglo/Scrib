# Character animation

## The standard technique
- **Locomotion base.** Current UE5 ships two answers. Lyra: a capsule-driven state machine (idle, start, cycle, stop,
  pivot) with starts advanced by distance and stops and pivots matched to a predicted stop point (distance
  matching), stride and orientation warping on the cycles, Foot Placement last. GASP (UE 5.4+): **motion matching**
  (Pose Search) over a large mocap library, with a Chooser picking the database
  ([UE distance matching](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine)).
- **Motion matching** (Clavet, *For Honor*, GDC 2016; Naughty Dog, TLOU2, GDC 2021; learned MM, Holden 2020) needs a
  big consistent clip library. Blend graphs (state machines, blend spaces, layers) remain the base for everything
  else, including combat and one-shot actions (montages in slots).
- **Warping.** Distance matching, stride warping (scale stride to speed), orientation warping (turn the lower body
  toward travel), motion warping (skew a clip to hit a target) ([UE motion warping](https://dev.epicgames.com/documentation/en-us/unreal-engine/motion-warping-in-unreal-engine)).
- **Foot placement.** Plant per foot (speed curve or contact curve), unplant radius, pelvis drop by the lowest foot,
  pelvis max offset ([UE FootPlacement](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/AnimationWarpingRuntime/FAnimNode_FootPlacement)).
- **Budget.** UE Animation Budget Allocator: `a.Budget.BudgetMs` 1.0 ms game thread by default, update rate down to a
  cap, interpolation between updates, significance per component
  ([UE docs](https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-budget-allocator-in-unreal-engine)).
- **Crowds.** Near: full rigs. Mid: Animation Sharing (a leader evaluates each pose bucket, followers copy). Far:
  instanced vertex or bone animation textures (UE AnimToTexture; Unity Animation Instancing). AC Unity: 40 real AIs,
  120 high-resolution models, about 10,000 crowd NPCs ([Cournoyer, GDC 2015](https://gdcvault.com/play/1022141/Massive-Crowd-on-Assassin-s)).
- **Modular characters.** UE merges skeletal meshes per material for crowds and keeps the player modular; merging
  drops morph targets ([UE modular characters](https://dev.epicgames.com/documentation/en-us/unreal-engine/working-with-modular-characters-in-unreal-engine)).

## Who ships it
| Engine | What it does | Source |
|---|---|---|
| UE5 | Lyra, GASP motion matching, Control Rig, Animation Sharing, URO | UE docs above |
| Cyberpunk 2077, RDR2 | Not verified at the technique level; RDR2 uses RAGE plus Euphoria physical animation (evolved from GTA IV) | [Wccftech](https://wccftech.com/rockstar-euphoria-evolved-rdr2/amp/); rest unverified |
| Decima | No public source found that Guerrilla uses motion matching | audit `animation` table |
| Naughty Dog | Motion matching in TLOU2 | GDC 2021 |
| Unity | Mecanim state machines; Kinematica motion matching was preview only (0.8.0-preview, 2020) | Unity changelog |
| Bethesda, OpenMW | Havok Behavior graphs, text-key groups, process levels (footnote); OpenMW stops AI and animation beyond its processing range (default 7168 units) | OpenMW game settings docs |

## Key numbers
- UE budget default 1.0 ms game thread, scaled up to 2.5 by quality, tick rate cap 10, `MaxInterpolatedComponents` 16,
  `MaxTickedOffscreen` 4 (UE docs above).
- Foot IK of FromSoftware: pelvis gains 0.2 up, 1.0 down (sekiro-rs, `superfargoth/docs/research/locomotion-rethink-2026-11-11.md`).
- Clip supply that matters for us: no walk or jog start, stop or pivot clips exist in the free packs we bundle.

## Where Scrib stands
Match: layered graph, inertialization, ozz-style evaluation, ACL-style compression, motion warping, stride and
orientation warp ops, foot IK, slots, animation budget tiers. Gaps against current UE5: no distance matching or
start/stop/pivot set (planned L3-L5), no leader-pose sharing or animation-texture crowd tier, no skin cache. Motion
matching is deferred (right call with our clip supply). See `superfargoth/docs/specs/animation.md`, `locomotion.md`,
`superfargoth/docs/research/animation/15-audit-and-plan-2026-11-12.md`, and [ledger/perf](../../ledger/perf.md) for skinning cost.

## Sources
- `superfargoth/docs/specs/archive/animation-history.md` "Industry comparison" (UE and Unity links, checked 2026-10-12),
  `superfargoth/docs/specs/locomotion.md`, `superfargoth/docs/research/locomotion-rethink-2026-11-11.md`.
- No web search was run. Unverified: City Sample use of AnimToTexture, GASP 5.8 Chooser details.
