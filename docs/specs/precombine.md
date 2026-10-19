# Near-cell precombine (`sf_lod` `lod.near`, `sf_world_bevy`)

Status: **spec, slice 1 next** (2026-10-19, phase1; owner go 2026-10-19). Roadmap: bevyrrowind ROADMAP 1.2 (the
cut) and industry-audit.md J6/#10 ("Precombined near geometry", missing). Related: far-field.md (L1 proxies
merge the same cells, simplified and untextured), bake.md (`mesh.cook`), streaming.md (spawn stage).

## Why

Measured (bevyrrowind `docs/perf.md` "The gameplay view", 1440p DLSS Q, Balmora walk at eye height): 26 ms
median, render thread 24 ms. `--off statics` (no near-ring static entities): **7.5 ms**. Every other system
costs 0-8 ms. The render thread pays per mesh entity per view (extract, specialize, bin, upload, encode;
4 sun cascades + main + extra views), so the lever is the number of near static entities: today one entity
per placement per LOD level per material part (46-52k meshes in town).

## Industry practice

- **Fallout 4 / 76 precombines**: non-interactive statics of a cell merged at build time into combined meshes
  per material; references that scripts may move or disable are excluded; a precombine breaks (and must be
  rebuilt) when a mod edits a member.
- **OpenMW object paging, active grid** (behaviour, `bevyrrowind/docs/research/perf-morrowind-engines.md`):
  merges statics inside the active cells too; documented as a large win when CPU-limited.
- **MGE XE**: distant statics as one or two draws per cell (far field only; our L1 already does that).
- **UE HLOD / Unity static batching**: same idea, built offline, per material, with the source objects kept
  for collision and gameplay.
- Godotwind (anecdote): runtime merging without baking made frames worse (57 → 44 fps, 861 ms stalls): bake it.

## Design

**Unit: one precombine per streaming cell, split into size buckets.** For each bucket, one cooked model
(`sf_model::CookedModel`, the `.sfmesh` body the runtime already loads) in cell space (positions relative to
the cell center): its materials are the union of the members' materials (deduplicated by value), its LOD
chain is level `k` = every member's LOD `min(k, last)`, transformed into cell space and concatenated per
material. Level error = max over members of (member error at that level × member scale). Vertex format is
uniform already (position, normal, uv, color), so merging is a transform and an append; no simplification
(the far field's L1 simplifies; this must look identical to the separate objects).

**Members (eligibility, Bevy-free, plus an adapter veto).** Class `Static`; model cooked `Static` (not
`Skipped`, not `Skinned`); no flag in `NO_LOD | NEAR_ONLY | EDITOR_ONLY | DISABLED`; no blended material
(sorted translucents stay separate entities, whole placement); not a ground-blend model; not a light, door,
item, actor or marker (classes). The adapter hook `PrecombineSource::exclude(placement)` vetoes what only the
content pack knows: in Morrowind, statics a script can `Disable`/`Enable`/`SetPos` (MWScript object
references), so quests that hide a rock keep working. Placements the world overlay moved or removed are
excluded at runtime (below).

**Size buckets.** Size culling (industry-audit J1) gives each object a draw distance from its radius; a merged
mesh has one. Members go into buckets by radius (default edges 0.5 / 2 / 8 m); a bucket draws to the largest
draw distance of its members (`size_culled_end` of its biggest member), so nothing disappears early and small
clutter still stops at its bucket's short range. Ground clutter (radius < 0.5 m) is its own bucket.

**Ranges.** A merged level's switch distance is measured from the cell center (the entity origin), but its
members sit anywhere in the cell. To never coarsen a member earlier than it would alone, each switch distance
of the merged chain is increased by the bucket's bounding radius around the cell center (conservative: more
detail drawn near cell edges, never less). The last level ends where the bucket ends (ring unload or the
far-field handoff, as for its members), with the same end band.

**Bake step `lod.near`** (`sf_lod::bake`, feature `bake`): subject `<ws>/<x>,<y>,<z>` (a cell); inputs: the
cell's `world.page` placements; *dynamic*: `mesh.cook` of each candidate model, the adapter's fingerprint.
Output: `NearCombine { buckets: [{ max_radius, end_radius, members: [PlacementId], model: CookedModel }] }`
(postcard). Deterministic member order (placement id). Reruns only for cells whose placements or member
models change, like every other product.

**Runtime (`sf_world_bevy`).** When a cell's near level loads, the Fetch stage also fetches the cell's
`lod.near` product (if the bake has it and `StreamingConfig::precombine` is on). Its buckets join the cell's
work as synthetic placements (class `Static`, model = the bucket, transform = cell center) spawned through
`spawn_template` like any cooked static (so VisibilityRange chains, layers, AABB, `NoAutoAabb` all apply).
Member placements still go through their spawners for everything but visuals: `StaticSpawner` spawns no
mesh for a placement in `members` and its model is never loaded (`needs_model` false), while wrappers such
as `PhysicsHook` still activate (colliders come from placement data, not render entities). Overlay changes
(an item or static moved/removed by the save or a script at runtime) make that placement spawn on its own
and the bucket hide it: phase 1 simply drops the precombine of a cell with any overlaid member and spawns
the cell the old way (rare, correct); phase 2 would rebuild in the background.

**What else it touches.**
- Shadows: merged buckets cast as a few large casters (the cascade cost was the second biggest item).
- Picking / activation: statics aren't activatable; doors, containers, activators aren't members.
- Fog swap, terrain blend, RT: members never include blended or ground-blend parts; RT `ray_tracing_lod`
  works per mesh as before.
- Memory: unique geometry per cell instead of shared instances; measure (expected tens of MB in towns).
- Hot reload / mods: a mod editing a member changes the cell's inputs: rebake as for every product.

## Verification

1. Unit tests (Bevy-free): merge of two members keeps vertex positions in cell space, material dedup,
   LOD chain length and errors, bucket assignment, conservative ranges.
2. A/B switch `--off precombine` (runtime, no rebake needed); the walk and still benches before/after:
   entities, meshes, render/main/GPU ms. Target: most of the `--off statics` gap (26 → ~10 ms).
3. Visual check: same views with and without, nothing missing or popping (pots, crates, small clutter at
   their old distances), shadows unchanged; an MWScript-disabled static still disappears (adapter veto).

## Slices

1. Bevy-free builder `sf_lod::near` (members → buckets → `CookedModel`), tests.
2. Bake step `lod.near` + Morrowind adapter (`PrecombineSource`: script-referenced statics), bake report.
3. Runtime: fetch, synthetic placements, member skip, `--off precombine`, overlay fallback; measure; visual check.
