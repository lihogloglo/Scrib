# Lane roadmap: Big-world rendering (millions of trees, dense scatter)

Spec: [superfargoth/docs/specs/big-world-rendering.md](../../superfargoth/docs/specs/big-world-rendering.md) (`BW-`
slices). Ledger: [docs/ledger/bigworld.md](../ledger/bigworld.md). Visual checks: `BW-` entries in a new
`bevyrrowind/docs/visual-checks/bigworld.md` (created with the first owner look). Owner brief 2026-11-26: on the side,
with no build time or crates taken from other lanes. **Every build slice is `[p]` until the owner's go.**

## 1. Player-facing goal

**Done (Modern):**
- A generated world about 10x Vvardenfell carries forests of individual trees, on the order of 10 M of them.
- You can walk into a stand, see crowns to the horizon from a ridge, and fly over it.
- Nothing pops, there are no holes at tier changes, and rocks and props are dense between the trees.
- Frame cost depends on the view, not the world size: about 0.55 ms GPU typical and ≤ 2 ms in the densest stands at
  High (spec "Cost model").

**Classic option:** Morrowind's own exteriors keep today's paths (static scene and far field). See owner question 1.

## 2. Where we are (2026-11-26)

- **Not started.** No `sf_bigworld` crates exist.
- Generated worlds draw every scattered tree as an entity: 7.21 M flora ran at 26 fps (owner run, 2026-11-19). The
  static scene, precombine and far field are off there (scaffold `WorldgenPlacementStatics`).
- Placement exists and is deterministic per 256 m tile (`sf_worldgen::scatter`).
- The vendored `GpuBatchedMesh3d` draw path runs in game (static scene, groundcover).

## 3. Decisions (spec "Reuse check", "Industry comparison")

- **Three tiers by screen size:**
  - T0: mesh LOD chain plus wind, to ~125 m.
  - T1: octahedral impostor per tree, to ~2.2 km.
  - T2: per-tile canopy proxy, to the horizon.
  This is the UE/Fortnite impostor plus HLOD standard. It is also Chlorophyll's and SmartImposter's tile aggregate
  past ~1-3 km.
- **Instances are never entities.** Compact 16 B tile pools come from a deterministic `InstanceSource`, generated on
  tasks with nothing stored (Horizon). A GPU producer writes visible slots into the vendored `GpuBatchedMesh3d` (the
  eidolon pattern), so Bevy's stock preprocess does frustum and hi-Z culling in every view.
- **Bevy meshlets rejected** for T0/T1: opaque only, no wind, per-instance CPU upload every frame. Re-checked for the
  opaque T2 only.
- **Shadows:** T0 and T1 cast into cascades within ~500 m. T2 receives the terrain far shadow map. Wind stops below
  40 px, so cascades stay cacheable (UE 5.7, Chlorophyll).
- **New crates only:** `sf_bigworld`, `sf_bigworld_bevy`, `mw_bigworld_bevy`. Other lanes' crates are read or hooked,
  never edited ("Hooks" in the spec).

## 4. Stages

**Stage A: ceiling and core.** Decides whether the slot path holds before anything else is built.
- [p] **BW-1** `sf_bigworld` core: tiles, ring planner, tier bands, 16 B instance format, `InstanceSource` plus a
  synthetic source. NEW Bevy-free crate. Verify: exact tests, plus a measurement test printing ring counts.
- [p] **BW-2** ceiling bench `forest_ceiling` (NEW `sf_bigworld_bevy`, example only). Verify: the orchestrator runs
  one run per arm (10⁵ / 3·10⁵ / 10⁶ slots). Stop rule in the spec.

**Stage B: a forest in a framework example scene** (owner looks, synthetic content).
- [p] **BW-3** GPU producer. Depends on BW-1, BW-2. Verify: GPU twin test.
- [p] **BW-4** slot impostor material and atlas capture. Verify: naga compile tests; owner look `BW-impostor`.
- [p] **BW-5** T0 mesh tier with crossfade and wind. Verify: owner look `BW-near`.
- [p] **BW-6** tile streaming on tasks. Verify: headless flight with a hitch count.

**Stage C: to the horizon.**
- [p] **BW-7** T2 canopy bake. Verify: bake time and bytes measured; error-bound test.
- [p] **BW-8** T2 runtime with crossfade and far shadows. Verify: no-hole test; owner look `BW-far`.

**Stage D: in the game.**
- [p] **BW-9** `mw_bigworld_bevy`: `InstanceSource` over the scatter, trunk colliders, registration for generated
  worlds. Needs meta and world10x agreement for one plugin line and one adapter filter. Verify: owner look `BW-x10s`.
- [p] **BW-10** perf suite on x10s at High and Low. Verify: ledger lines with ms per preset.

**Later** (separate slices): horizon occlusion, GPU scatter, canopy into distant shadows, groundcover footprints, nav
obstacles, and vanilla trees on this path (owner question 1).

## 5. Spec changes

None pending: BW-0 is the spec. As-built detail goes into the spec as each slice lands.

## 6. Dependencies on other lanes

- **Builds:** BW-1/BW-2 add new crates. That is a root `Cargo.toml` member plus hakari, and meta schedules it with the
  CI and idle lanes. BW-2 uses the shared Bevy feature set (no new Bevy variant).
- **world10x:** BW-9 needs the flagged species list and the adapter filter.
- **Groundcover, nav, weather:** they consume our hooks when they choose to. Nothing is asked of them now.
- **Provides:** `InstanceSource`, `NearInstances` (colliders, nav) and `BigworldFootprints` (groundcover).

## 7. Owner questions (each with a recommended answer)

1. **Vanilla Morrowind trees:** should they move to this path too, or stay on today's static scene and far field?
   *Recommended: stay for now.* Only generated worlds use bigworld until BW-10. Vanilla trees can follow later as one
   slice if the look matches.
2. **Tree models for the 10x world:** Morrowind's own tree models only, or also generated trees (a clean re-port of
   the MIT ez-tree generator)? *Recommended: Morrowind models first.* The generator comes later, only if forests need
   more variety than about 16 species.
