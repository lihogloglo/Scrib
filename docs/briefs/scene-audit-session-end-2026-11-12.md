# Session end 2026-11-12 (scene-audit): static scene perf audit, fixes, open questions, next session

Orchestrator session `scene-audit`, owner present. Commits on `main`, pushed: `a3e3d4d` (audit, fixes 1-3 + 5, fps
overlay, capture tool, docs), `2063d89` (untraced A/B, per-class capacity re-tune). Full write-up:
`superfargoth/docs/research/static-scene-perf-audit-2026-11-11.md`.

## Done

- **Audit** of why `--scene` was slower (traced A/B incl. a 32k-slot run, reviewed by a second model): per-frame
  O(ranges) CPU in the vendored GS port, per-frame `extract_chain_slabs`, per-class capacity skew; overflow secondary.
- **Fixes (headless-tested, Opus agents):** 1 slab lookups only for pending ids; 2 GS port caches extracted batches and
  resolved ranges (`MeshAllocator::changes()` generation, debug-build recheck); 3 capacity per material class +
  `scene/batch/<class>/*` gauges + warn once per class; 5 lamp shadow cache treats batches as static casters
  (`GpuBatchedLightVisibility` set, `ShadowCastersChanged` on stream-in/out/promotion).
- **Measured** (untraced, 3 runs, 1440p DLSS Quality, Balmora): `--scene` 11.23 -> 9.44 ms standing, 14.50 -> 11.10 ms
  walking; old path 9.31 / 10.71. Parity, fewer stutters on `--scene` (12.8 vs 15.5/min), ~10k fewer entities.
- Dev overlay shows `NN fps NN.N ms` (top-left, `mw_devtools_bevy/src/live_ab.rs`, Bevy `FrameTimeDiagnosticsPlugin`).
- RO-18 (floating foliage) **not reproduced** (scripted climb to 700 m + owner flight to 595 m, BRP capture of every new
  mesh): tool `bevyrrowind/tools/brp_capture_meshes.mjs`. Clouds not drawn over the sea added to WE-7 and H3.
- New check SC-lamps (scene `sc-scene-night`), awaiting owner.

## Left running / uncommitted

- **Fix 4** (one geometry range per chain level, `sf_scene/src/space.rs`) was running at log-out (`.sessions/scene-fix4.md`).
  Next session: check it finished at a compiling state (`cargo_turn.sh test -p sf_scene --features bake`, floor 112),
  then A/B (`perf suite fix4_on balmora balmora_walk --repeat 3 --resolution 2560x1440 --env BEVYRROWIND_DLSS=quality
  --arg=--scene`) and commit. Target `views/all/instance_draws` ~8-10k (was 32.1k).
- Traces `target/perf/au_tr_{off,on,on32k}` (~10 GB on C:): delete.
- 145 other sessions' changed files (build-speed, menus/UI, nav, textures, IK) are not committed; some don't build alone.

## Questions left hanging (owner)

1. **Is the static scene worth more work?** Owner 2026-11-12: a large feature for parity. Before S3a3 or anything else on
   it: the ceiling run `--off statics` (old path, standing + walk, untraced 3x). If statics cost only ~1-2 ms in total,
   freeze `--scene` as is and spend sessions on the real costs (walk: CPU-bound, render thread ~10 ms, PostUpdate ~4.7 ms;
   standing: GPU-bound, DLSS 2.3, main opaque 2.4, light probes 0.7, prepass 0.6, bloom 0.4 ms).
2. S3a3 (GPU per-view LOD, size cull, horizon cull, handoff end, compaction): only if the ceiling justifies it; LOD
   thresholds need the owner's visual sign-off.
3. SC-lamps night check (`scene sc-scene-night`) needs a dev build with fix 5, then the owner's eyes.
4. Carried from earlier handoffs: interiors in the table, `NO_LOD` placements, slot budget past ~16 batches.

## Next session (owner 2026-11-12)

1. **Player walk animation is "super wonky"** (ROADMAP 3.4 character checks; `.sessions/loco-r1.md`, `ik-p5.md` and
   `superfargoth/docs/research/locomotion-rethink-2026-11-11.md` are the latest context; foot IK P-IK1..5 landed 2026-11-11).
2. **Shadow on the player flickers hard near some torches and lamps** (SH-21, `docs/visual-checks/shadows.md`; SH-20b).
3. **Clouds don't render over the sea** (WE-7 / WE-10; hard problem H3 draw order sky/clouds/fog/precip/water, which
   `bevyrrowind/docs/hard-problems.md` reserves for the owner's strong agent: the owner asked for it next session).
Rule from tonight (memory `measure-ceiling-first`): every perf slice states a measured ceiling and expected ms first.
