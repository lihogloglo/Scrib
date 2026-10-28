# Motion generation: offline clip factory for ambient life (MG)

Status: spec 2026-10-28 (owner: "make a proper doc, then implement straight away"). Research and first tests:
[ambient-life-2026-10-28](../research/ambient-life-2026-10-28.md), activity list:
[ambient-catalog-2026-10-28](../research/ambient-catalog-2026-10-28.md). ROADMAP 3.8.

<!-- agent-summary -->
> **Agent summary.** An offline pipeline that turns activity recipes (prompt + constraints + furniture band + props)
> into many clips: NVIDIA Kimodo (text + constraints) for activities, MotionBricks (in-betweening) for links, SOMA BVH
> out, our existing BVH import + `sf_character` retarget onto `base_anim`, and a review renderer that plays each clip on
> a real Morrowind NPC with the real furniture and props, frame-stepped into MP4s. Slices MG1-MG8 below.
> Read: "Pipeline", then your slice. Tools: `bevyrrowind/tools/motiongen/` (Python), Rust in `mw_bake`, app `clip-preview`.
<!-- /agent-summary -->

## Goal

NPCs that sit, lean, sweep, farm, drink, sleep and talk, in the hundreds of variants an open world needs
(catalog: about 140 activities × phases × variants × furniture bands, so **thousands of clips**). The clips must fit
Morrowind's own furniture heights and race sizes, hold Morrowind props, and play on the Morrowind rig. The owner
judges every clip visually on a real NPC, so review must be fast and in context.

Non-goals here: the runtime activity system (interaction points, claims, schedules, `sf_interaction`, `sf_ai`:
separate specs), runtime generation (rejected, research note), shipping weights.

## Decisions

- **Generators are offline tools, never in the game or the bake.** Python, outside Cargo, run on the owner's GPU.
  Weights live in the user's HF cache. The bake only sees the BVH files they write (rule 3: a pack like any other).
- **Kimodo makes activities, MotionBricks makes links** (research note). Kimodo: `Kimodo-SOMA-RP-v1.2`, text +
  constraints, SOMA-77 BVH. MotionBricks: `MotionBricks-SOMA-RP-v1`, transitions between our own poses (enter/exit
  links, approach to a slot); its sample BVHs are evaluation only and never feed shipped output.
- **Measure, don't assume.** Furniture heights come from the user's install (collision census, MG4), clustered
  into bands; race heights from `RACE` records. Recipes are generated once per band. `FullBodyIk` absorbs the rest
  at runtime.
- **SOMA BVH is the interchange.** Kimodo writes it natively (`--bvh`); MotionBricks output is written with the same
  hierarchy. Our BVH importer + `auto_map` + retarget do the rest, like Quaternius UAL and LAFAN1.
- **Review on the real rig.** The review renderer is the game itself (dev command `clip-preview`): real NPC body,
  real furniture static, real props, real retarget, fixed-step capture to MP4. No dummy, no second render path.
- **Repo policy until the licence check (owner decision pending).** Generated BVH, provenance and MP4s stay local in
  `<repo>/animation packs/generated/` (untracked, the BYO folder). Every clip has a provenance sidecar (model,
  version, prompt, seed, constraints, date) so a later decision can ship them like the CC0 packs, or regenerate.
- **Throughput target:** ≤ 10 s of wall time per accepted 6 s clip on the RTX 4060 laptop, so 2,000 clips fit a
  night. Measured baseline 2026-10-28: about 40 s per clip (80 s per prompt for 2 samples, mostly reloading the 8B
  text encoder on the CPU per call).

## Pipeline

```
recipes/*.toml ──> motiongen plan ──> jobs (recipe × band × variant)
                       │                     │
furniture bands <── MG4 census              ├─ kimodo worker (persistent; encoder + model loaded once,
race heights                                │   prompt embedding cache, batched samples)  ──> SOMA BVH + .prov.json
                                            └─ motionbricks worker (links between clip end/start poses) ─> SOMA BVH
                                                     │
                     animation packs/generated/<activity>/<phase>/<id>.bvh (+ .prov.json, sources fragment)
                                                     │
            MG3: BVH import (source `generated`, unit/up), auto_map SOMA -> humanoid -> base_anim (mw_bake)
                                                     │
            MG5-7: clip-preview: NPC + furniture + props, fixed-step capture ──> <id>.mp4 + review.html
                                                     │
                     owner verdicts (keep / reject / note) ──> verdicts.toml ──> sources.ron candidates (later)
```

### Recipes (MG1)

TOML, one file per setting (`tools/motiongen/recipes/tavern.toml`), rows from the catalog:

```toml
[[activity]]
id = "sit.chair.drink"            # slot-style id: kind.family.variant
point = "seat"                     # interaction point kind (catalog column); selects the furniture band list
props = ["tankard"]                # prop classes (MG7 table)
phases = ["enter", "loop", "exit"]
variants = 4                       # samples per (phase, band)
duration = { enter = 3.0, loop = 8.0, exit = 3.0 }
prompt.enter = "A person standing in front of a chair turns and sits down on it."
prompt.loop  = "A person sitting on a chair drinks from a mug held in the right hand, then rests it on the knee."
prompt.exit  = "A person sitting on a chair stands up."
constraints.loop = { pelvis_height = "band.seat", root = "fixed" }
```

Constraint vocabulary (mapped to Kimodo constraints by the planner): `pelvis_height` (root height keyframes),
`root = fixed | path:<points>` (2D root path), `hand_l/hand_r = { height = "band.top", forward = 0.35 }`
(end-effector keyframes), `start_pose = "<clip id>:end"` (full-body keyframe, for chains), heights in meters or
`band.*` references. Planner scales SOMA-space targets by `soma_height / race_height` per band.

### Generation workers (MG1, MG8)

- `python -m motiongen run <recipes...> --out <dir>`: one process keeps Kimodo and the LLM2Vec encoder loaded (encoder
  on CPU by default, 32 GB RAM), caches prompt embeddings on disk (hash of prompt + encoder version), runs
  `num_samples` per job in one GPU batch, writes SOMA-77 BVH + `.prov.json` per sample and a `jobs.jsonl` log
  (time per stage). Resumable: a job with all outputs present is skipped.
- Speed levers, measured in MG1: embedding cache, batch size, `diffusion_steps` (100 default; try 50), FP16, and
  keeping the encoder resident. Print a measurement line per run; no pass/fail on speed (tests rule).
- MotionBricks worker (MG8): links `A.end -> B.start` (approach slot -> enter, loop end -> loop start for seamless
  loops, exit -> idle), using the TensorRT engine (cached plan), written as SOMA BVH with the same provenance.

### Import onto the Morrowind rig (MG3)

- New source in `mw_graphs/data/animation/sources.ron`: `generated` (folder `animation packs/generated`,
  BVH, unit/up from the Kimodo export; SPDX `LicenseRef-generated-nvidia-open-model`, `private_only` until the
  owner decides). `auto_map` must map SOMA-77 names (Hips, Spine1, Spine2, Chest, Neck1/2, Head, Left/RightShoulder,
  Arm, ForeArm, Hand, fingers, Left/RightLeg = thigh, ...) to the humanoid map; fingers map when the target has
  them, else drop. Root motion from the pelvis (`root_from_pelvis`, like LAFAN1).
- Verification: a generated BVH cooks and retargets onto `base_anim`, `base_anim_female`, `base_animkna` without
  NaN; feet within a few cm of the floor in a standing clip (measurement test, printed).

### Furniture and race census (MG4)

- `bevyrrowind furniture-census [--json <path>]`: every `STAT`/`ACTI`/`CONT`/`FURN`-like record whose id or mesh
  matches the seat/table/bed/bench/stool/counter/shelf/bar patterns (list in the tool), with the baked collision's
  top surfaces: seat height (highest horizontal surface ≤ 0.8 m with a ≥ 0.25 m square footprint), table/counter top,
  bed top, footprint, mesh scale in refs (`XSCL`) distribution, ref counts per cell type. Clustered into bands
  (k-means or fixed 5 cm bins merged under ref-count weight). Plus `RACE` heights (male/female).
- Output `animation packs/generated/bands.json`, read by the planner. The census prints its tables (measurement).

### Review renderer (MG5-MG7)

`bevyrrowind clip-preview --clip <bvh> [--npc <id>] [--furniture <record id> [--at x,y,z,yaw]] [--prop
<class>=<record id>] [--camera orbit|front|side] [--out <dir>] [--fps 30]` (dev feature):
- MG5 **capture**: a minimal level (flat floor, noon light, no streaming), one NPC (default a Dunmer commoner,
  `--npc` any `NPC_`), the clip cooked and retargeted **in process** (the same Bevy-free cook + retarget calls the
  bake uses, no bake round trip), played in the full-body slot with root motion; `Time<Virtual>` stepped by a fixed
  1/fps per frame, a `Screenshot` per frame to PNG, then `ffmpeg` to MP4 (H.264). Exit when the clip ends. A
  `--batch <list>` mode renders many clips in one process (one load, one window).
- MG6 **furniture**: place a real `STAT` mesh by record id at the band's reference position (the seat under the
  clip's seated pelvis, from the provenance constraints), with its collision drawn on F-key debug only.
- MG7 **props**: a prop table (`tools/motiongen/props.toml` → `mw` record ids: tankard `misc_com_tankard_01`,
  broom `misc_com_broom_01`, book, quill, pick, basket, cloth...) with a grip socket per class (bone, offset,
  rotation, two-handed second-hand point). Attach = a child entity on the hand bone; the second hand is a
  `FullBodyIk` effector (only in preview until the runtime activity system lands). Spawn/despawn at phase events.
- `review.html` in the output folder: a grid of MP4s per activity/phase/band with id, prompt, seed; the owner writes
  verdicts in `verdicts.toml` (`id = "keep" | "reject" | "note: ..."`), which MG9 (later) turns into sources.ron
  candidates.

## Slices (one deliverable each)

| Slice | Deliverable | Needs | Status |
|---|---|---|---|
| MG1 | `motiongen` Kimodo batch worker + recipe/plan format + speed measurement | Kimodo env | done (uncommitted; see As built) |
| MG2 | 10 seed recipes from catalog Tier 1-2 (sit, lean, sweep, drink, farm, talk, sleep...) | MG1 format | todo |
| MG3 | SOMA BVH source + auto_map onto `base_anim` (+ tests) | a Kimodo BVH | done (uncommitted) |
| MG4 | furniture + race census CLI → `bands.json` | install | todo |
| MG5 | `clip-preview`: NPC + clip in process + fixed-step capture → MP4 (+ `--batch`) | MG3 (start with a UAL clip) | todo |
| MG6 | furniture static placement in `clip-preview` | MG5, MG4 | todo |
| MG7 | props table + hand attach + second-hand IK in preview | MG5 | todo |
| MG8 | MotionBricks link worker | MG1 | todo |
| MG9 | verdicts → sources.ron candidates + review.html | MG5 | later |

Order: MG1, MG3, MG5 in parallel (Python / bake / app), then MG4, MG2, MG6, MG7, MG8.

## As built (MG1, 2026-10-28)

- `bevyrrowind/tools/motiongen/` (README has setup/env): from `bevyrrowind/tools`, `C:/mbtest/kimodo/.venv/Scripts/python.exe -m motiongen selftest | plan | run | bench | joints`. Selftest all ok; `run` resumable, writes BVH + `.prov.json` + `jobs.jsonl`.
- Speed: model load 1.6 s; prompt embeddings cached on disk, encoder (CPU) loaded only on a miss (31 s load + ~7 s/prompt), then freed. 6 s clips, fp32: steps 100 = 3.7 s/clip, 50 = 2.1, 25 = 1.1; batch 1/2/4/8 at 50 steps = 2.3/2.0/2.1/2.1 s/clip (VRAM 1.2-1.5 GB); fp16 1.76 (jerk 41 -> 53 m/s^3), bf16 2.3 (jerk 284: avoid); post-process costs ~0.01 s/clip (skate 0.045 -> 0.038 m/s with it). Target (<= 10 s) met ~4x.
- Defaults: steps 50, batch 8, fp32, post-process on. Steps 25 loses the pelvis constraint (err 0.12 m vs 0.006 m at 50).
- Constraints: `pelvis_height` = Kimodo `root_y_pos` only (err ~6 mm), `root fixed|fixed_xz|path`, hand wrist position (custom class; err ~0.3 m, i.e. weakly followed: next lever is Kimodo's own end-effector set with a full reference pose).
- BVH (for MG3): Kimodo exporter, cm, +Y up, facing +Z, default rest = standard T-pose (`--bvh-rest seed` for BONES-SEED rest); wrapper `Root` joint + `Hips` with absolute translation channels (Hips OFFSET is a dummy 0,100,0); joints in `tools/motiongen/soma77_joints.txt`.
- Gaps: no seamless loops (MG8), `start_pose` rejected, hand constraint weak, encoder on GPU infeasible (16 GB bf16 > 8 GB; 4-bit needs bitsandbytes, not installed, not worth it with the cache). The LLM2Vec load prints LoRA MISSING/UNEXPECTED key warnings (inside Kimodo's loader, unchanged).
- Bench output: `animation packs/generated/mg1-test/_bench/bench.json`; next: `run motiongen/recipes/seed.toml --out "../../animation packs/generated"`.

## Verification

Sanity only (tests rule): generated BVH parses, cooks, retargets without NaN on the three skeletons; the preview
renders N frames for an N-frame clip and writes an MP4; the census finds chairs/tables/beds in Vanilla. Measurements
(printed, `#[ignore = "measurement: ..."]` or tool output): seconds per clip by stage, feet height error, census
tables. Quality is the owner's verdict in `review.html`.

## Industry comparison

Studios use text-to-motion and in-betweening to block out and fill large ambient sets, then clean the keepers
(Ubisoft La Forge's learned in-betweening; NVIDIA's MotionBricks UE5 demo; Kimodo/ARDY for authoring). Ambient sets
are scenario/workspot libraries placed on furniture markers (GTA scenarios, Cyberpunk workspots, Skyrim furniture
markers, UE Smart Objects). We generate per furniture band instead of hand-fitting each mesh, because Morrowind's
furniture is irregular and we have no animators.

## Open questions

- Licence: do Bones RigPlay terms reach model outputs? Copyright status of generated motion. Until answered,
  generated clips are local only.
- Fingers: SOMA-77 has them, `base_anim` hands are coarse; do we keep generated finger curves or per-prop hand poses?
- Beast races (`base_animkna`): digitigrade legs on a plantigrade source; check in MG3.

## As built (MG3, 2026-10-28)

- Source `generated` in `mw_graphs/data/animation/sources.ron`: cm (`unit_scale: 0.01`), +Y up, `root_from_pelvis`
  (Kimodo's `Root` is a static wrapper), `private_only`, `License::GeneratedNvidiaOpenModel`
  (`LicenseRef-generated-nvidia-open-model`), `files: []` (no slots yet).
- Export with `--bvh_standard_tpose`: the zero pose is a T-pose, auto-align needs no facing turn. Kimodo's default
  (BONES-SEED rest, every bone along +X) puts the retargeted ankles ~1.17 m up: unusable. Re-export from a run's `.npz`
  with `kimodo.exports.bvh.save_motion_bvh(..., standard_tpose=True)` (same motion, bit-for-bit hierarchy).
- `sf_character::auto_map`: SOMA-77 maps cleanly (`LeftLeg` thigh by hierarchy, `LeftShin` lower leg, Spine1/Spine2/Chest,
  Neck1); a non-thumb finger with four joints drops its base (metacarpal) unless the last is a childless end (Mixamo).
- In process (MG5): `mw_bake::anim::HumanoidTarget::new(skeleton)` and `cook_library_clip(source, path, clip, rules,
  target) -> Result<Clip, String>`, equal to the baked `anim.retarget` bytes (`generated_real_install.rs`).
- Measured (`measure_generated_feet`, 2.97-5.97 s clips): standing ankle error 3-7 mm (stand still), max 1-4 cm
  (sweep, lean); `base_animkna` ankle 0.16 m below its rest height: digitigrade legs, no beast-leg fix (as LAFAN1/UAL).

## As built (MG5, 2026-10-28; compiles, first run pending)

- Command (dev builds): `bevyrrowind clip-preview --clip <file|name in sources folder> [--name <clip>] [--source <id>]
  [--batch <file: path[|name[|source]] per line>] [--npc <NPC_>] [--camera front|side|orbit] [--out shots/clip-preview]
  [--fps 30] [--size 960x540] [--ffmpeg ffmpeg] [--keep-frames]`. Default NPC `draren thiralas` (Dunmer commoner).
- Code: `mw_viewers_bevy::clip_preview` (feature `clip_preview`, on in the app's `dev`); Bevy-free `clip_preview::cook`
  calls MG3's `mw_bake::anim::{HumanoidTarget, cook_library_clip}` on the dressed actor's skeleton (no bake round trip).
- Scene: the `character` viewer's actor path (mw_actors_bevy content: real body and graph, no AI, no physics), not the
  test-level machinery (that pulls streaming, player and physics); 1 m checker floor, fixed noon sun. Clip = `LateClip`
  in slot `full`, root motion applied, anim LOD forced A0, camera follows the actor.
- Capture: `TimeUpdateStrategy::ManualDuration(1/fps)`; frame 0 on a zero time scale (pose t=0), then one step per
  frame; `Screenshot` in `Last` -> `<out>/<id>/frames/%05d.png`; N = round(duration*fps)+1; run fails unless PNGs == N;
  ffmpeg libx264 yuv420p -> `<out>/<id>.mp4`. Timings per clip: printed (cook, capture, encode); not measured yet.
- Tests: `cook.rs` unit tests (frame count, batch lines, bundled UAL1 resolve). MG6/MG7 hook into `stage` / the body entity.
