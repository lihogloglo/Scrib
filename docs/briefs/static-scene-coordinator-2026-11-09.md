# Session: coord-static-render (scrib-4f, started 2026-11-09 evening)

*Copied verbatim out of the untracked `.sessions/coord-static-render.md` on 2026-11-10, so it stops living only
on one laptop (owner: nothing may exist only on this machine). It is the record of the session that ran the audits,
wrote the two work orders, landed patch GS and the `sf_scene` core A1-A6, and left the S3b queue. Its successor
is `s3b-coordinator.md`.*

**Role:** sole live coordinator for the static / rendering / streaming system
(`superfargoth/docs/specs/static-scene.md`, hard problem H1). Appointed by the owner this
session with a wide mandate: "implement the whole system, or at least go as far as you can".

**Standing facts for this session (owner, 2026-11-09):**
- ThinkPad X13. Rust compiles; **the game is not on this machine and cannot be launched.**
  No in-game testing by anyone here. No game data files.
- Compiling is slow: do it **intently and seldom**. `repo/target` is warm (78 GB, 3953
  fingerprints) so incremental Bevy builds are feasible; Bevy-free crates are seconds.
- Priority is **a very fast game**. Other lanes' unfinished work may stay unfinished as long
  as it is committed and pushed and does not block this lane.
- Future agents (with the game on the workstation) verify that it all actually works.

**Sessions:** every other Scrib session is idle (checked via ListAgents). This session owns
the whole checkout `repo/` and the build slot. The multi-lane rules in `_meta-coordination.md`
are therefore relaxed to: one build at a time, still `-p <crate>`, never `--workspace`.

**Loose work preserved on arrival (nothing lost):**
- `origin/backup/mechanics-wip-2026-11-09` — the coordinator-mechanics lane's uncommitted
  working tree (mw_rpg, mw_script, mw_stats, sf_effects, specs/clock.md), committed verbatim
  via a temporary index; `repo/`'s working tree left untouched. UNVERIFIED, never compiled.
- `static-scene/s1-fixes` worktree held 8 uncommitted files of S1 review fixes; rescued,
  finished and pushed by an agent this session.
- `static-scene/docs-fixes` (3f748d0) is docs-only and still unmerged to `main`.

**Status:** see "Log" below.

## Log

- Audit wave dispatched: independent architecture review (is the design right?), `sf_scene`
  core rescue + audit, S3a vendored-port audit + S3b readiness.

### Audit result 1: the S3a port (agent report, full detail in
`superfargoth/docs/research/s3a-port-audit-2026-11-09.md`)

**The S3b brief as written is unbuildable.** `GpuBatchedMesh3d` carries ONE `Handle<Mesh>` and the
template fixes one index range for the whole batch, so "one batch entity per (space, material class)"
and per-instance LOD index ranges are structurally absent from the port, not merely unimplemented.
Spec §6 "What we add to #25978" is therefore a prerequisite, not a later nicety → new slice **S3a2**
(port extension) goes BEFORE S3b. Design agent dispatched; plan lands in `specs/gs-patch-plan.md`.

Also blocking: lamp/spot shadow views never run `unpack_ranges` yet the light shim bins batches on
every cube face over `max_capacity` UNINITIALISED work items (corrupt maps + atomicAdd corruption);
and the S0 batch-set/work-item/pipeline counters are blind to instance batches, so the measurement
that justifies the whole design would read ~0.

### Queued fixes (not yet dispatched — ordered)

1. **S3a2** port extension (vendored `bevy_pbr`/`bevy_render`): N ranges per batch sharing one batch
   set; `unpack_ranges` in `Core3dShadows` + the shadow-cache schedule; counters see instance
   batches. Then S3a3: batch-set merging, right-sizing by surviving instances, #26161 generation
   counter. One Bevy build.
2. **SERIOUS 10, foliage**: `mw_assets/src/cooked.rs:134-150` promotes `SfAlpha::Blend` + cutout to
   `AlphaMode::Mask`, but `mw_bake::scene::scene_material` (`scene.rs:96-108`) sees only `m.alpha`,
   bakes the part `Blended`, and `sf_scene::eligibility` then excludes it. Most alpha-masked foliage
   would silently vanish under `--scene` and the first A/B would be unreadable. Fix in `scene_material`
   (it already has `extras::BLEND`). Bevy-free, cheap. Hand to the `sf_scene` lane when it frees up.
3. **Spec §5 is wrong** (`static-scene.md:186-188`): GI `1<<26` and surface weather `1<<27` are *view*
   pipeline-key bits (`vendor/bevy_pbr/src/render/mesh.rs:485-493`), not material state, so they
   cannot be material-class axes. The code's real third axis is two-sidedness. Coordinator owns this
   edit; do it before S3b keys batches off classes.
4. Handoff doc `s3a-gpu-batch.md` says "no motion vectors": **wrong**.
   `previous_input_index = u32::MAX` already makes `mesh_preprocess` use the current transform as
   previous, which IS spec §7's rule for rigid statics. Only wind/UV-scroll history is missing (S4).
5. Merge `static-scene/docs-fixes` (3f748d0, docs only) once the architecture reviewer stops reading
   `static-scene.md`.

### Audit result 2: the architecture (independent review, web-researched)
`superfargoth/docs/research/static-scene-architecture-review-2026-11-09.md`

Verdict: **right architecture, wrong about what the port can express.** Same BLOCKER as the
port audit, reached independently. New findings beyond it:
- **bevy#26149 and #26161 were closed unmerged upstream on 2026-10-30** (self-closed, branches
  deleted, zero reviews). Our shadow patch is an own fork and its "Drop when" can never fire.
  #26456 was closed as a duplicate and its slab-growth diagnosis retracted (real bug #25595,
  fixed by #26095, which we already vendor) — the spec cited it as live.
- **"LOD chains share vertices" is FALSE**: `sf_model::cook` runs `optimize_vertex_fetch` per
  level, so `chain_layout` concatenates (≈1.7-2× LOD0 vertex memory).
- **`is_active` is gated behind `@if(FRUSTUM_CULLING)`** → promotion is ignored in views without
  GPU frustum culling (our `NoIndirectDrawing` sun views): a promoted placement draws twice.
- **Per-view output memory is reserved capacity, not visible count** (≈140 MB at 100k × 8 views).
- **Chain mesh residency, refcounting, the global chain table and the per-chain
  concatenate-and-rebase are entirely undesigned** — all S3b work, none of it budgeted.
- Slot-allocator fragmentation has no end state (first-fit, never compacts, `high_water` only
  grows, every cell needs a contiguous range).
- #25978 is untested with shadows, occlusion culling, Vulkan or DX12.

### Coordinator decisions (2026-11-09, scrib-4f)

1. **Patch GS is our own fork.** We still offer it upstream; we do not plan around a merge.
2. **Accept LOD-chain vertex concatenation.** No `mesh.cook` change. Per-level vertex-fetch
   optimisation is a real GPU win, each level keeps its own `base_vertex`, and the property the
   design needs — a chain is ONE allocation, so LOD never changes the instance or its batch —
   holds anyway. Spec §2 and the memory budget corrected instead.
3. **S3a2 goes before S3b**, split in two: **Slice A** (~200 lines: range unpacking moved inside
   `early_gpu_preprocess` so it covers `Core3d` + `Core3dShadows` + the shadow-cache view at once;
   counters see instance batches; `is_active` gate un-nested) and **Slice B** (~400 vendored lines:
   N geometry ranges per batch, one batch set, per-range AABB, the DX12 MDI-count gate).
4. **Rejected: per-(cell, chain) LOD quantisation.** It would collapse S3a3 from ~250 to ~100
   lines by making every per-range count CPU-known, but it breaks §4 step 5's per-instance pixel
   rule. Owner may overrule — flagged, not taken.
5. **Batch key is (space, material class, vertex slab, index slab, material bind group)**, not
   (space, class): one indirect draw binds one vertex buffer, one index buffer, one material bind
   group. Slab count per class becomes a measured quantity in S3b.
6. **`GpuPreprocessingMode::Culling` is a hardware floor**; a `PreprocessingOnly` adapter falls
   back to the entity path automatically with one startup `warn!`. Steam Deck (RDNA2/Vulkan) has
   culling, so the owner's performance floor is safe.
7. **`size_cull_px` defaults to 0 (off) in shadow views**, per-view knob, implementation kept, so
   the owner can turn it on after a visual check (a main-view-style cutoff in shadow views was
   tried here and reverted the same day, perf.md:55-57).

### Build economy (hard constraint found this session)

**Only ~40 GB free on C:.** `repo/target` is 78 GB warm. A second vendored-Bevy target dir does
not fit, so **every Bevy build must happen in `repo/` with the warm target** and `repo/` must be
owned exclusively for the duration. Bevy-free crates can use small private target dirs in
worktrees. Four merged worktrees (`s3a`, `docs-fixes`, `s1-fixes`, `static-scene`) and two stale
non-git dirs (`ai1`, `todo`) are reclaimable; removal needs the owner's go.

### Decision 8: the static-scene spec's size budget

The spec came back **38.2 KB** against a 25 KB budget: it was already at budget before the audits,
and the 19 corrections are all normative. The docs agent correctly refused to hollow it out and
escalated. Decision:

- **§6 keeps the decisions and a link; the mechanism lives in `gs-patch-plan.md`.** That is the
  right separation anyway — the spec says what and why, the patch plan says how. (~4.2 KB)
- **§2's byte layout points at the `sf_scene::table` / `::chain` module docs**, which are
  authoritative and are where an implementer actually looks. (~1.5 KB)
- **§4's cull and LOD math STAYS in the spec.** It is the one place the CPU twin and the GPU pass
  must be held to the same contract; a pointer there would cost us correctness.
- The result (~32 KB) is **over budget by design, recorded as an explicit exception**: this is the
  contract for a nine-slice system with a vendored renderer fork. Revisit at S9, when six of the
  nine slices become history and move to the archive.

Not yet applied — scheduled for the next docs pass, after Slice A clears `repo/`.

### Also still open from the docs pass

- `sf_scene/src/material.rs:10-11` still documents the material class as per-instance `u16`;
  format v2 moved it to the chain record. Code doc, one line.
- Two evidence-backed notes dropped for space: the lamp cache's `CubemapVisibleEntities` clear and
  the port's `add_gpu_batched_meshes_to_light_visibility` both run `PostUpdate` after
  `CheckLightVisibility` with **no `.after()` between them** (an ordering gap, review S6); and
  `size_cull_px` is a projected *radius*, so High = 1 px radius = 2 px diameter, which matches J1's
  `s_min = 2` — worth stating so nobody "fixes" it to 2.
- The Industry comparison table was corrected against the review's web research (UE5 occlusion is
  opt-in for non-Nanite + VSM per-mip cull; Unity's per-cascade shadow view, occlusion off by
  default; "Frostbite" relabelled Ubisoft Montreal + RedLynx, baked cluster-visibility mask, not
  "cone"; id Tech 7 only, with LOD **CPU-side** and shadows not on the same indirect buffer; the
  **Decima row was deleted** as unsourceable).

### Landed: four Bevy-free defect fixes (`static-scene/scene-fixes`, 5 commits)

`cdef0de` cutout candidates baked alpha-tested as the runtime draws them; `a8d8c00` sub-pixel cull
per view, off by default in shadow views (`ViewKind`, `ScenePreset::size_cull_px(kind)`,
`SceneView::with_preset`); `31ed02b` `SceneSettings { layers: 0 }` rejected where it enters;
`67783fa` debug-assert `horizon` is never set on an orthographic view. sf_scene 53 / 66 with bake,
mw_bake 51; clippy `-D warnings` clean. NOT yet merged to `main` — Slice A holds `repo/`.

**My brief overstated the foliage defect and the agent corrected it, correctly:** `Cutout::Promoted`
(blend + `GREATER 0`, the *tested* foliage idiom) was already classed `AlphaMasked`. The mis-classed
case is `Cutout::Candidate` — textured, untested, full-alpha blend. So the loss was narrower than
"most alpha-masked foliage", and the audit's "silently vanish" is really "bake and runtime
disagreed", since a wrongly-`Blended` part is *meant* to spawn as a split entity (a later slice).

**Residual, now dispatched as a follow-up:** the bake reads no textures, so it promotes EVERY
candidate — which pulls glass, gauze and smoke cards into the table as alpha-tested. The missing
fact is already computed and discarded at `mw_bake/src/delight.rs:375`
(`cutoff.filter(|_| !job.candidate || alpha_is_cutout(&image))`); the follow-up records it as a
bake fact so `scene.cells` can match the loader exactly.

Known-unfixable without a cache-key input: `--off leafmask` and `--off foliagetransmission` change
the runtime material class after the table is baked. The candidate promotion now joins them.

### Design result: the scene runtime (`superfargoth/docs/specs/scene-runtime-plan.md`, 599 lines)

Two findings that reshape S3b:

1. **The spec's per-space instance table and the port's per-batch output buffer are TWO buffers**
   (80 B table records vs 64 B output slots). A verbatim upload was never possible — which is why
   the "chain residency / fragmentation" questions looked unsolvable.
2. **`mesh_key` is a blake3 hash and not invertible** (`sf_scene/src/bake.rs:106-117`), so a v2
   table cannot tell the runtime which model a chain came from. There is no path from a baked
   table to a mesh without a format change.

**Decision: take format v3 and a new `scene.chains` bake step.** v3 adds a per-chain model index +
material index into a new model-string section, inside already-reserved bytes, same record sizes.
`scene.chains` emits each chain as a one-level-per-LOD `.sfmesh` read by the **existing**
`SfMeshLoader` — no runtime concatenate-and-rebase, no new asset type, no new loader, no new
dependency. **Nothing has ever been baked** (`bake_scene` is still not called), so the format is
free now and never again. Taking it.

**Ownership:** one `SceneSpace` per worldspace owns a refcounted `ChainRegistry` (mesh key → global
dense chain slot), a `SlotAllocator` over the 80 B scene buffer (one contiguous range per resident
cell, `write_remapped` is the upload), one `ClassAllocator` per batch over its 64 B output slots,
the resident-cell map, and **the frame counter** — `sf_scene_bevy` increments it in `First` and
feeds every `advance`, because streaming despawns immediately and has no frames-in-flight deferral
(`stream.rs:1311-1355`). Chain meshes are asset handles in a side map indexed by chain slot,
dropped when the registry reports the last cell released the chain.

**Fragmentation: bounded.** Segregated size-class allocator per batch (64-slot multiples to 512
then geometric ×1.25, never split or merged, per-class LIFO free lists, release deferred by frames
in flight) → zero external fragmentation within a class, high-water monotone and convergent once
the worst neighbourhood has been walked. First fit over mixed sizes (today's allocator) is bounded
only by Robson's ≈11× the live set, so it stays only for the 1-slot chain table where every request
is the same size. **Batch capacity is fixed at config (4096), never grown at stream-in** — growing
reallocates the output buffer and rebuilds the materialize bind group (review S7). On exhaustion the
cell publishes without that batch and those placements spawn as entities, counted and warned once.

**Accepted defaults:** batch granularity per (class, slab pair, bind group); capacity 4096;
`frames_in_flight = 3`; one mesh per level; LOD0 only in S3b; both `--scene` and `--off scene`
(`Part::ALL` → 55); **interiors off** (owner question).

**S3b's honest ceiling, which must be in the brief:** `is_active` is per *slot*, not per view, and
S3b has no per-view work items. So S3b delivers the CPU-cost win (O(batches) per view instead of
46-55k entities), residency and promotion — but **not** our cull, horizon, sub-pixel or per-view
LOD. Those need the GS plan's S3a3 count/allocate/scatter pass. Without this stated, S3b's numbers
will read as a failure.

**Size: ≈4 000 lines, 4 agent slices, 2 Bevy build slots + 1 cold rebuild** for the new crate.

### Correction to my own brief (decision 9)

The architecture review's "a promoted placement draws twice in non-frustum-culled views" is
**probably wrong**: every prepared indirect key already carries `FRUSTUM_CULLING`
(`gpu_preprocess.rs:2095-2123`) and non-indirect views **drop instance batches entirely**
(the port audit's own SERIOUS 7, `gpu_preprocessing.rs:2574-2576`). So statics go **missing** in
such views — the documented hardware floor — they do not double-draw. The review conflated "the
gate is unreachable" with "the gate is bypassed". Slice A has been told to settle it from the code,
keep the un-nesting as defence-in-depth, and not claim a bug fix. Spec §1 needs the same correction.

## State at 2026-11-10 (end of the scrib-4f coordinator session)

`main` = `8047c44`, verified on the laptop at that commit: **`sf_world_bevy` 29 tests,
`sf_scene --features bake` 85 tests, clippy `--all-targets -D warnings` clean.**
Everything is UNTESTED in game — no game and no data files on this machine.

### Landed on main this session

| | What |
|---|---|
| audits | `research/s3a-port-audit-2026-11-09.md`, `research/static-scene-architecture-review-2026-11-09.md` |
| plans | `specs/gs-patch-plan.md`, `specs/scene-runtime-plan.md` |
| spec | 19 normative corrections + 4 late ones; §6 mechanism moved to the patch plan; size exception recorded |
| docs | 9 decisions in `superfargoth/docs/DECISIONS.md`; test-pass ledger reordered (S0 baseline FIRST); ROADMAP 1.16; SYSTEMS row |
| S1c | rescued (the handoff state did not compile), finished, tested |
| defects | 4 Bevy-free fixes + the baked per-texture alpha class (`texture.alpha_class`) |
| **patch GS** | **COMPLETE**: slice A (range unpacking inside `early_gpu_preprocess`, counters see instance batches, guards), slice B-i (N geometry ranges per batch), slice B-ii (N indirect slots sharing one batch set, DX12 gate) |
| sf_scene core | A1 product names out of `feature = "bake"`, A2 **format v3**, A3 `ChainRegistry`, A4 `ClassAllocator` |

### Next, in order

1. **A5/A6** — in flight on `static-scene/scene-core-a56` (`SceneSpace`, `BatchKey`,
   `batch_ranges`, `ChainsStep`, `chain_keys`, v3 writer, `CookedSubmesh::encode` pub).
   Brief: `docs/briefs/scene-core-a5-a6.md`. Also fixes `scene-runtime-plan.md` §4.4 and
   records the allocator reversal in DECISIONS.
2. **B1-B6 + C1/C2** — the `sf_scene_bevy` crate itself (~4 000 lines, 4 slices,
   2 Bevy builds + 1 cold rebuild for the new crate). Plan: `specs/scene-runtime-plan.md`.
   **B1 is the only step that touches `Cargo.toml`/`Cargo.lock`/hakari** — keep its feature
   set inside what hakari already unifies or it forces a cold rebuild of vendored Bevy.
3. **S3a3 is PARKED and its value is in doubt** — see the open owner question below.

### Open owner questions (none of these are mine to settle)

1. **Is S3a3 worth building at all?** SPUD — the shipped, measured reference for this exact
   data model — has **no GPU culling whatsoever** (`research/spud-code-2026-11-08.md`: no
   culling compute shader in their `shaders/`; flat per-cell arrays, a tight CPU cull loop,
   instancing per (submesh, LOD)). S3a3 (~450 vendored lines: count → bump-allocate →
   scatter) exists only to serve our choice of the GPU variant. The data-layout half — which
   is where the CPU win lives — is already built.
2. **The fps expectation was corrected mid-session.** `perf.md:41`: `balmora_walk` was
   **10.50 ms** at 2026-10-28, ×3, 1440p + DLSS Q — the ≤ 11 ms gate was already met two
   weeks before any static-scene code. The 25.8 ms figure quoted in `static-scene.md` "Why"
   is an A/B diagnostic from 2026-10-19, **not a bench**, and should not be read as the
   current frame. So the immediate fps headroom in Balmora is small; the real cases are the
   **Steam Deck floor (no number exists at all)**, the 10× instance target, and every future
   feature that adds a view.
3. Interiors in or out of the static scene (default taken: out).
4. `NO_LOD` placements are barred from the table entirely; the scene could honour them by
   pinning level 0 instead of paying per-entity cost for ever. Decide before S9.
5. **Cheapest independent win, not started:** the `prepare_preprocess_bind_groups` cache
   (2.3 ms in an old trace, ~9 % of a 10.5 ms frame), upstream bevy #26418. Unrelated to
   this whole system.

### Process lessons from this session

- **No coordinator git operations in a checkout an agent holds.** A merge run while an
  agent's branch was checked out there swept 142 lines of its unverified mid-slice code onto
  `main` (backed out in `ac82a88`). Use `git push origin <sha>:refs/heads/main` and
  `git branch -f`, which need no checkout, or a worktree of your own.
- **`remote.origin.fetch` only tracks `main`**, so `origin/<branch>` refs do not exist until
  fetched explicitly with a refspec.
- **Always `--all-targets` on clippy** — without it the `cfg(test)` module is never linted;
  it found real lints in two separate slices.
- **Agent sizing:** ~180-230k tokens per slice is healthy; 350-455k means the brief was too
  big. Point agents at section ranges and handoff briefs, never at whole 600-line plans, and
  hand off rather than resuming an agent for a second large leg.
- **Ask for the measurement, not the assertion.** Telling the A4 agent that a disproof was
  the more valuable result is what surfaced the allocator reversal.
