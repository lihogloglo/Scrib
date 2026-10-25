# Hard problems

Problems that several normal slices haven't cracked, or that cut across Bevy internals and many of our systems.
They are reserved for the owner's strongest agent (it unlocked the shadow performance, ROADMAP 1.4 resets A/B).
One problem per run. Started 2026-10-25 (scrib session); the orchestrator keeps it current.

Each entry: symptom, what's known and tried (so the agent doesn't redo it), why it's hard, where to start, done when.
Status: `open`, `running <session>`, `done <date>`.

## How to brief the agent

Paste the entry plus this:

> Workspace `D:\Gamedev\Bevyrrowind` (`superfargoth` framework + `bevyrrowind` game; read both CLAUDE.md files,
> "Parallel sessions" and "Hard rules"). Register in `.sessions/<name>.md`. Every cargo command through
> `bash bevyrrowind/tools/cargo_turn.sh <args> -j 4`. The owner's game window (`%TEMP%\bw-owner-*`, BRP 15730) is
> off-limits; your runs exit on their own. Bevy changes go in as vendored patches (`superfargoth/vendor/`, ideally
> upstreamable), never render-world injection. Diagnose with measurements before changing code; A/B runs vary
> ±0.5 ms, repeat 3x below ~1 ms. Don't commit; report cause, fix, numbers, files.

## H1. Town frame target: render-thread bound (ROADMAP 1.2, 1.9) — open

- **Symptom:** High, 1440p + DLSS Quality, RTX 4060 Laptop: Balmora ~12.3-12.5 ms median, the eye-height walk
  `balmora_walk` ~12-13 ms, Seyda Neen and the flight bench worse (last full table 2026-10-19: 14-19 ms, before the
  shadow resets). Target (owner decision 1): ≤ 11 ms median everywhere, stretch 7 ms. The render thread is the
  bottleneck (GPU 53-78 % busy).
- **Known:** `--off statics` (near ring) takes the walk from 25.8 to 7.5 ms (10-19 numbers); sun cascades re-cull
  as they follow the camera; `prepare_preprocess_bind_groups` 2.3 ms, `queue_submit` 2.4 ms (33 submits), shadow
  binning 1.4 + encoding 1.2 ms, skins 1.2 ms; 16 visibility passes on the main thread.
- **Tried, failed:** Bevy `NoCpuCulling` on statics (81 ms: every part queued in every view, incl. `NoIndirectDrawing`
  sun views that never GPU-cull); precombine (meshes -45 %, frame unchanged: the puzzle); caster size cull (pops);
  animation budget (froze poses). Done: big_space propagation patch, particles in view only, lamp cache + Bevy PRs.
- **Why hard:** halving mesh entities didn't move the frame, so the cost isn't per-mesh where we assumed; it's in
  Bevy's per-view prepare/queue/batching and submit structure.
- **Start:** `docs/perf.md` "Frame budget", "Bare baseline and precombine", "Lead (not yet tested)";
  `superfargoth/docs/specs/precombine.md`; `superfargoth/docs/research/perf/README.md`; `perf trace` + Nsight;
  `docs/research/mge-xe-code-2026-10-25.md` (MGE XE code audit, incl. the **Greatness7 fork** `9bb5e97`, whose
  distant-land generator is Rust: `distantland/crates/statics/src/{merge.rs,atlas/}`; GPLv2 text, no explicit
  "or later": confirm before porting code).
- **Best current hypothesis (unproven in Bevy):** cost scales with distinct material/mesh batches × views; precombine
  halved meshes but kept the union of materials, per-LOD entities and ~6-8 views. The Greatness7 fork is the proof
  at Morrowind scale that merge **plus atlas** pays (merge alone is what our precombine tried): BVH-bounded merge
  groups per cell (radius >= 32, no dynamic visibility), atlas pages (opaque BC1 / alpha BC3, UV-bound palette for
  tiled UVs), one cumulative index buffer per merged subset with per-tier prefix ranges (LOD inside the mesh; they
  rejected tier-split entities), horizon culling from a baked terrain max-height grid (~41 % fps in dense areas).
  First step: count batches, bind groups and work items per view on `balmora_walk`, precombine on/off, `--off
  statics` with lights kept. Then: one material per merged chunk (atlas or texture array), one entity per chunk per
  material class with LOD index ranges, horizon culling, MGE-style 2 cascades from the size-filtered set (owner
  trade-off: shorter far shadows). Watch memory: merged statics reach GiBs on heavy mod lists.
  Also Clockwork (`superfargoth/docs/research/clockwork-code-2026-10-25.md` items 2-3): a persistent GPU instance
  table culled per view by compute (cascades and cube faces too, Hi-Z one frame late, indirect + bindless; no
  published ms), and a reach-with-dissolve rule.
- **Done when:** every `benchmarks/exterior_*.json` median gate passes at the target setup, x3 runs.

## H2. Shadow quality (ROADMAP 1.4, 1.12) — open

- **Symptom:** owner 2026-10-20: shadows "only showing when super close"; soft-shadow (PCSS) penumbra size is in
  cascade texels, so it jumps between cascades; distant flashes on camera move (SH-11); lamp shadows short-reaching,
  grainy, flickering, thin casters (chandelier cable) flicker (SH-13); choppy character shadows (SH-20, a parked
  fix exists in `superfargoth/vendor/parked/`); translucent casters (RO-1).
- **Known:** perf is solved (resets A1-B2: lamp cache + vendored Bevy PRs #26149/#26161, 2x2 PCF, contact 0.3 m).
  Quality was never finished. Constraint from the owner: stock Bevy shadows + vendored patches; own shadow atlas only
  if that falls short (decision 3).
- **Why hard:** cascade fitting, filtering and stability interact with TAA/DLSS and the cache; fixes must not
  re-break the perf numbers.
- **Start:** `superfargoth/docs/specs/shadows.md` (+ `shadows-history.md`), `research/shadows/README.md` and
  `2026-10-21-audit.md`, visual checks SH-11..SH-20 in `docs/visual-checks.md`. Lead: Clockwork's world-unit sun
  PCSS (`lib/light/sunsoft.glsl`: penumbra from sun disc x caster height, the same width in every cascade), its
  lamp cube filter (SH-13) and distant-shadow redraw triggers (SH-11): `clockwork-code-2026-10-25.md` items 1, 6, 7.
- **Done when:** owner confirms SH-11, SH-13, SH-20 and the "only close" complaint in Balmora day/night and the
  census office, with frame time no worse than today (perf.md "Shadow reset B2").

## H3. Transparency and draw order across sky, clouds, fog and precipitation (ROADMAP 1.8, 1.12) — open

- **Symptom:** the ash/blight storm draws over terrain and objects (WE-5; precip streaks ruled out, suspects ash
  height fog or the cloud composite); smoke not drawn in front of the sky (RO-B); translucents write no motion
  vectors (RO-6), so TAA/DLSS ghosts them.
- **Why hard:** several custom passes (clouds composite, froxel fog, precip, particles, water snapshot) each choose a
  place in Bevy's graph; the order and depth contracts are implicit.
- **Start:** `superfargoth/docs/specs/render-order.md`, `weather.md` (clouds/precip as built), `--off
  clouds|fog|froxels` A/B at the owner's view. Lead: Clockwork's K-nearest-layer OIT with a haze tail
  (`lib/core/layers.glsl`; needs wgpu 64-bit atomics): `clockwork-code-2026-10-25.md` item 4.
- **Done when:** one written order/depth contract per pass, RO-B / WE-5 / RO-6 confirmed by the owner.

## H4. Ragdolls and held objects (ROADMAP 3.2, 3.3) — open, Phase 3

- **Symptom:** PH-2 rejected (2026-10-19): ragdoll bodies sink into the ground, float, turn around, joints look
  unconstrained. Owner 2026-10-25: carrying an object pushes the capsule through the floor, jittery.
- **Known:** R0 write-back fixed headless 2026-10-22 (PH-2 awaiting owner again); avian3d has no spherical joint
  motor (PD torques instead); delirium-physics experiment parked (3.2b).
- **Why hard:** physics/animation coupling (write-back order, fixed tick vs render interpolation, collision layers
  between the held body and the player capsule).
- **Start:** `superfargoth/docs/specs/smooth-bodies.md`, `animation.md` ragdoll parts,
  `superfargoth/docs/research/animation/12-delirium-physics.md`, visual checks PH-*.
- **Done when:** owner confirms PH-2 and a held crate that neither sinks the capsule nor jitters.

## Second tier (hard-ish; a normal Opus slice may do)

- **Far-field LOD selection (FF-2, GF-1; ROADMAP 1.8):** L2/L3 nodes resident but never selected or tinted;
  Ghostfence field has no far copy, its light sprites float. `superfargoth/docs/specs/far-field.md`.
- **Hitches and pop-in (ROADMAP 1.1, 1.9):** interior objects pop in after the door fade; render-thread spikes;
  pipeline warm-up (J7). Pairs with the certified-screenshot slice (GPU readiness).
- **Image quality under TAA/DLSS:** smear on alpha-tested leaves at low internal resolution, distant water moiré
  (WA-1), grazing-light shimmer (SH-12).
- **Raster GI (Phase 4, parked):** IrradianceVolume / sky-visibility probes baked per cell. Design-heavy; only
  when Phase 4 opens.

## Not for the strong agent (fixes already exist, waiting for a slot)

- LT-4 lamp light cut along cluster block edges and SH-20 choppy character shadows: fixes parked in
  `superfargoth/vendor/parked/` (they edit vendored Bevy, so a full Bevy rebuild); apply when no lane is building.
