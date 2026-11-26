# MotionBricks v0.2.0 for MG8 link pieces (AN13)

Research 2026-11-26, from the local clone `C:\mbtest\motionbricks` (commit 54fc563 "v0.2.0 commit", 2026-10-23) and one
spike run on the RTX 4060 laptop. [V] = read in the source or measured, [I] = inference. Spike scripts and outputs:
`C:\mbtest\out\an13\` (`scripts/a_hubs_to_seed.py`, `b_generate.py`, `c_to_bvh.py`, `hubcheck.json`, `bvh/`).
This note replaces `ardy.md` §5 for MotionBricks.

## 1. What v0.2.0 changed
- [V] `CHANGELOG.md`: MotionBricks moved out of GR00T-WholeBodyControl, where it was a Unitree G1 preview, into its own
  repo. It adds a new model retrained on the SOMA human skeleton (`MotionBricks-SOMA-RP-v1`, about 200 M params,
  trained on Bones RigPlay 1), the Smart Primitive setup rebuilt in plain Python (Newton + Warp + TensorRT), and the
  Inbetweener and Scene Preview demos. There are no other notes.
- [V] The "Injured" presets are not a model feature. `app/runtime/soma/presets.py:89-101` defines six style modes
  (Skipping, Vogue, Injured Torso, Injured Leg, Zombie, Combat). Each one is a 1D blend from an Idle target to a
  style target in `assets/smart_primitives/soma/locomotion_style_variations_bs1_linear.smart_primitive.json`. A
  style target is a `target_pose` that samples 4 poses at one frame of a style BVH clip (e.g.
  `injured_torso_grab_walk_ff_180_001__A549.bvh` frame 293).
- [V] So style comes entirely from the key pose: the model only reaches the poses it is given. Those clips are BONES
  samples under the evaluation-only licence (§5). To get our own injured or old gait we supply our own key poses,
  e.g. frames of a Kimodo take. [I] This fits AN12/AN15 posture profiles: a style class is a set of key poses, not
  a retrained model.

## 2. Skeleton, formats, hub poses as inputs
- [V] The model has 30 joints: Hips, Spine1, Spine2, Chest, Neck1, Neck2, Head, Jaw, LeftEye, RightEye,
  LeftShoulder, LeftArm, LeftForeArm, LeftHand, LeftHandThumbEnd, LeftHandMiddleEnd (and the same 6 on the right),
  LeftLeg, LeftShin, LeftFoot, LeftToeBase (same on the right). Source: `motionbricks/assets/onnx/soma/soma_n1.json`.
  This is exactly Kimodo's SOMA-30 (the `hubcheck.py` `BODY_JOINTS` plus the 4 hand End joints). The engine maps
  joints by name (`model/soma/engine.py:90-127`). The runtime skeleton is SOMA-77 without Root, from
  `motionbricks/assets/human/soma_base_skel_minimal_no_root.usd` (77 joints, Hips at index 0). The 47 finger joints
  are not predicted: they decode to the reference pose, then the endpoint correction blends them from the context
  pose to the target pose.
- [V] Space and units: SOMA model space is Y up, metres, 30 Hz. A pose is a local `wp.transform` per joint (p xyz,
  q xyzw), root first. The model inputs world 6D rotations, so the rest convention matters. MotionBricks clips use
  the BONES-SEED rest (bone offsets along local X). Our hub takes are standard T-pose rest. Kimodo bridges the two:
  `read_hub_take` (scrib-motiongen `kimodo_backend.py:189`) converts a hub to T-pose locals, then
  `save_motion_bvh(..., standard_tpose=False)` writes a seed-rest BVH. In that BVH, Spine1's offset
  `5.003913 -0.037094 -0.013727` matches the MotionBricks sample BVHs exactly.
- [V] Input path: `motionbricks.io.bvh.load_bvh(path, skeleton=usd_skeleton)` (cm to m, drops Root, which is
  identity in Kimodo exports). Then sample frames `k-3..k` as context and `k..k+3` as target, and pass them to
  `PoseToken.update_from_poses`. The same hub frame works as both context and target (spike: stand.idle frame 45,
  sit.chair frame 120).
- [V] Output: an `AnimationBuffer` with `num_frames` local transforms plus `foot_contacts [n,4]` (heel/toe L/R, 0-1).
  MotionBricks has no BVH writer. The shortest path to SOMA-77 BVH is quats to rotation matrices in Kimodo order,
  then `SOMASkeleton77.to_standard_tpose`, then `kimodo.exports.bvh.save_motion_bvh(standard_tpose=True)`, so takes
  share the hubs' rest pose and `hubcheck` measures rotation (`c_to_bvh.py`, 20 lines).
- [V] Placement is our job: the engine only moves roots (`docs/runtime_api.md` "Place poses in the scene"). The
  spike turned the target to the context heading (feet-to-toes vector) and put its mean ankle XZ on the context's,
  so the feet stay planted for stand<->sit.

## 3. Exactness, length, chaining, loops
- [V] Endpoints are hard by construction. With `MotionBricksInferenceConfig.correct_generated_diff=True` (the default,
  `model/config.py:16`), `_post_process_generated_poses` (`model/soma/engine.py:198`) takes per-joint deltas
  (`kernels.py:245` `compute_endpoint_deltas_kernel`) and spreads them linearly across the motion
  (`apply_endpoint_deltas_kernel`; root position linear, rotation slerp, `utils/pose_utils.py:265-299`). Frame 0 then
  equals the last context pose and the last frame equals the first target pose, on every joint.
- [V] Measured with `motiongen hubcheck` after the full round trip (T-pose hub, seed BVH, MotionBricks, T-pose BVH):
  start and end error was 0.0 deg / 0.0 cm on all 26 body joints, for stand->sit, sit->stand and the wide
  token-range variant. The raw model (correction off) misses by 3.6 / 6.2 cm worst effector and 47 / 53 deg max
  (RightHand), mean 5.5 / 6.6 deg. That is the same size as Kimodo's soft keyframes (AN44b: 1-8 cm, 10-50 deg), so
  the exactness comes from the correction, not the network. [I] The same delta-spread correction could also be
  applied to Kimodo takes to make them land exactly on their hubs.
- [V] Length: the model picks 6 to 16 tokens of 4 frames, which is 18 to 58 valid frames (0.6-1.93 s). The default
  `allowed_token_counts` is 8-14; a single count forces the length (`docs/runtime_api.md` "Generation length").
  Measured: stand->sit took 50 frames (1.67 s) at the default and 58 (1.93 s) at 6-16; sit->stand 50 frames.
  Longer pieces chain: feed the last 4 output frames back as the next context and add an intermediate target pose
  (`docs/runtime_api.md` "Chaining generations"). Frame 0 of each segment duplicates the previous end, so drop it
  when concatenating.
- [V] Loop seam: context = the last 4 frames of a loop, target = its first 4 frames, placed at the end root, with
  `allowed_token_counts=(6,)`. That gave an 18-frame (0.6 s) link with an exact seam. `AnimationBuffer.make_loopable`
  (`animation/animation_buffer.py:200`) is the cheaper alternative for clips that almost loop already.
- [V] Slide (hubcheck, 3 cm contact band): p95 was 14.8-18.6 cm/s, against ~100-150 cm/s on the Kimodo stand<->sit
  arm (AN44b). The look is still the owner's call.

## 4. Runtime on the laptop (Windows, RTX 4060 8 GB)
- [V] The existing `.venv` works: Python 3.12.15, TensorRT 10.8.0.43, Warp 1.17.0 (CUDA 12.9, driver 13.4).
  `MotionBricksSOMAEngine` imports, and the ONNX files are real (LFS pulled: fp32 758 MB, fp16 380 MB). This is a
  second venv: Kimodo's is Python 3.11.
- [V] TensorRT plan build: about 6.9 min wall time (14:00:14 to 14:07:09, including Python and USD start-up) for the
  FP16-kernel plan (`TensorRTBuildOptions(enable_fp16=True)`, 386 MB plan). Peak host RAM was 7.0 GB working set:
  build only when RAM is free. Each option set gets its own cache key (`inference/builder.py:56-65`). The older
  763 MB plan `.trt-cache/mmm_v6_soma_fp16-aeaff7….plan` came from the fp16 ONNX built without `enable_fp16`; it is
  stale and unused by `--model-precision fp16`. The FP32 plan was not built.
- [V] With the plan cached: engine load 1.53 s; VRAM +384 MiB for the engine and buffers (Warp `free_memory` delta,
  CUDA context not counted), +388 MiB after 5 generations. Per segment: 261 ms on the first call (CUDA graph
  capture), then 4.8-9.1 ms median including decode and correction. The vendor's 4090 figure is 1.28 ms GPU-only
  (`docs/onnx_inference.md:79`). Generation cost is negligible; process start (~1.5 s engine, plus USD and Warp
  kernel load) dominates.

## 5. Licences
- [V] Code: Apache-2.0 (`LICENSE`). Docs and media: CC-BY-4.0. Models (ONNX and the HF SafeTensors): NVIDIA Open
  Model Agreement (release 2026-04-02, v. March 9 2026; `motionbricks/assets/onnx/LICENSE.md` links to it). It
  allows commercial use and redistribution of modified models, and says "NVIDIA does not claim ownership to any
  outputs generated using the Works or Derivative Works". Licences terminate if we start patent or copyright
  litigation. Redistribution must ship a copy of the agreement, and "Licensed by NVIDIA Corporation under the NVIDIA
  Open Model Agreement" if a NOTICE file exists. It carries no Trustworthy-AI use restriction.
- [V] Sample motions (`assets/motions/LICENSE.txt`, NVIDIA Sample Data Evaluation License) are internal evaluation
  only, and derivative works may not be distributed. [I] So any clip generated with a BONES sample pose as context
  or target, which includes every built-in style preset, must never ship. MG8 must only take poses from our own
  takes (hubs.toml sources, Kimodo output) and should reject paths under `motionbricks/assets/motions`.
- [V] Dependencies: `tensorrt-cu12*` 10.8 (NVIDIA TensorRT SLA, proprietary runtime), Newton 1.6, Warp, usd-core
  (`pyproject.toml`). None is gated: there is no HF token, and the weights come via Git LFS. [I] All of them stay
  inside the offline tool repo (scrib-motiongen). Scrib links none of them and ships only the clips.
- [I] THIRD_PARTY_NOTICES: Scrib needs no code entry. Add a provenance line for generated assets: "Link clips
  generated offline with NVIDIA MotionBricks v0.2.0 (code Apache-2.0; SOMA model under the NVIDIA Open Model
  Agreement, which claims no ownership of outputs)". scrib-motiongen's README lists the dependency and its
  licences. It must not vendor the weights; if it ever does, it ships the agreement text with them.

## 6. Recommendation for MG8
- Worker shape [I]: `python -m motiongen link <recipe>` runs in the MotionBricks venv (Python 3.12) with one resident
  engine, FP16-kernel plan, `cache_dir` pinned to `C:\mbtest\motionbricks\.trt-cache`. Kimodo-dependent steps (hub
  rest conversion, BVH write) run as a small step in the Kimodo venv, as in the spike. Alternatively port
  `change_tpose` plus Kimodo's `global_rot_offsets` (`kimodo/skeleton/transforms.py:76-110`) to numpy inside
  motiongen, so one venv suffices.
- Link spec per job: `from = "hub:<id>" | "clip:<bvh>@end"`, `to = "hub:<id>" | "clip:<bvh>@start"`,
  `poses_to_use` 1-4 (4 = keep velocity), `tokens` (range or single), `placement = feet | root | explicit
  (dx, dz, dyaw)`, and `variants` (token counts or placements to sweep). [I] Whether one input gives the same output
  on every call is unmeasured: the spike saved only the last of its 6 calls.
- Calls: `load_bvh`, `PoseToken.update_from_poses` for context and target, `generate_animation(ctx, tgt, out,
  config=MotionBricksInferenceConfig(...))` (config is keyword-only), `out.poses.numpy()[:out.num_frames]`, and
  `foot_contacts`. Chain segments with an intermediate target when a piece runs past 1.9 s.
- Outputs: SOMA-77 T-pose BVH plus `.prov.json` with `worker = "motionbricks"`, version 0.2.0 / commit 54fc563,
  `model = MotionBricks-SOMA-RP-v1 fp16`, `from_hub`/`to_hub` (or `from_clip`/`to_clip` plus frame), token count,
  placement and foot contacts (sidecar).
- Measure (hubcheck table per job group): start and end deg/cm (expect 0.0; non-zero means a rest or placement bug),
  slide mean/p95, raw-vs-corrected delta (correction size: the largest per-joint delta spread over the piece), and
  ms per segment.
- Risks: wrist drift where the correction is large (53 deg on RightHand over 50 frames); placement heuristics for
  pieces that move the feet (approach -> enter); hubs outside the training distribution; FP16 vs FP32 picking
  different tokens; 7 GB host RAM for the one-time plan build; two venvs.
- Size [I]: one slice, about 350-450 lines of Python (worker 250, recipe and placement 100, tests 80), plus README
  and spec lines. The spike scripts are its skeleton.
