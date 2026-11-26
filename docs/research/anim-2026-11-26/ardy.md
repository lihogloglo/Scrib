# ARDY (and Kimodo constraints, MotionBricks) for the AN44 hub-pose spike

Research 2026-11-26. [V] = read on the cited page, [I] = inference. The repo note `superfargoth/docs/research/animation/18-kimodo-community-2026-11-12.md:8` only names ARDY (no detail): nothing to add.

## 1. What ARDY is
- [V] Autoregressive diffusion model for interactive motion: streaming text prompts plus long-horizon kinematic constraints, "real-time". Paper: ACM TOG 45, art. 86 (SIGGRAPH 2026), DOI 10.1145/3811284; project page https://research.nvidia.com/labs/sil/projects/ardy/ ; code https://github.com/nv-tlabs/ardy ; weights https://huggingface.co/collections/nvidia/ardy (code/Core/G1 released 2026-07-10; SOMA checkpoints listed 2026-10-15).
- [V] Inputs: text (streaming), root paths/waypoints, full-body keyframes, sparse joint positions/rotations (hands, feet), history poses up to 8 s (https://huggingface.co/nvidia/ARDY-SOMA-RP-30FPS-Horizon60). Constraints may lie beyond the current window.
- [V] Exact hit: neither the project page nor the model card says hard or soft; only "strong constraint adherence ... performance may vary". [I] Diffusion conditioning is soft; expect residual error like Kimodo, so measure it (that is the spike).
- [V] Card limitations: foot skating, weaker outside locomotion/gestures/combat/dance/everyday activities, no scene awareness.

## 2. SOMA checkpoint
- [V] SOMA = NVIDIA's unified parametric human body model (https://github.com/NVlabs/SOMA-X). Kimodo's I/O uses the 77-joint `somaskel77` since 2026-03-19; the SOMA models run on 30 joints (`somaskel30`) and constraints are converted 77 to 30 (https://github.com/nv-tlabs/kimodo/blob/main/docs/source/user_guide/constraints.md).
- [V] ARDY ships `ARDY-SOMA-RP-30FPS-Horizon60` (192M params, 30 fps, 30-joint SOMA) and `ARDY-SOMA-RP-30FPS-KimodoRep-Horizon60`; also Core (27 joints, 20 fps) and G1 (34 joints). [I] The 30-joint SOMA skeleton is the same family as Kimodo's SOMA models; retarget by joint name. Our backend already works on `model.skeleton` = SOMA-30 (`scrib-motiongen/motiongen/kimodo_backend.py:167`). Verify joint names and rest pose match before trusting it (not confirmed on any page).
- [V] Output: `.npz` with `posed_joints [T,J,3]`, local and global rotations, root positions, foot contacts, fps, text. No BVH export mentioned. [I] Needs an npz-to-BVH step: reuse Kimodo's `kimodo.exports.bvh` (the worker uses it).
- [V] Kimodo also ships `Kimodo-SOMA-RP-ArdyRep-v1.2` (Kimodo on ARDY's motion representation, raw hips root) for direct comparison.

## 3. Licences
- [V] ARDY code Apache-2.0. Weights: NVIDIA Open Model Agreement; card: "ready for commercial or non-commercial use". No non-commercial blocker found. (Kimodo-SMPLX-RP-v1 is R&D-only: avoid it.)
- [V] Output ownership is not addressed on the card. [I] The Open Model Agreement says (from memory, unverified here) NVIDIA claims no ownership of outputs: read the agreement text before shipping and log it in THIRD_PARTY_NOTICES. Same terms as our current Kimodo SOMA weights.
- [V] ARDY's text encoder depends on gated Meta-Llama-3-8B-Instruct (HF token + Meta licence acceptance). [I] An extra tool-side licence to log; it does not touch the clips.
- [I] Code Apache-2.0 is GPL-3 compatible and nothing is linked into the game; generated clips are our data.

## 4. Install and run
- [V] Python 3.10+ (3.11 tested), PyTorch >= 2.4 with matching CUDA, CMake 3.15+ and a C++17 compiler for a bundled motion-correction extension, optional TensorRT (driver >= 525). Tested Ubuntu 22.04, RTX 4090.
- [V] VRAM: ARDY text encoder default (cuda/bfloat16) ~14 GB; CPU encoder option uses less. Kimodo: ~17 GB all on GPU, under 3 GB with `TEXT_ENCODER_DEVICE=cpu`. [I] The 192M denoiser should fit 8 GB with the encoder on CPU; our worker already does this for Kimodo (`kimodo_backend.py:256`, `encoder_device="cpu"`).
- [V] Windows: not mentioned by ARDY; Kimodo "should work" (Docker suggested). [I] The C++ extension needs MSVC; Kimodo already runs here, so only the ARDY extension build is new risk.
- [V] No latency numbers published. [I] Real-time design: seconds per clip offline; TensorRT not needed.

## 5. Kimodo constraints and MotionBricks, only as they bear on a hub pose
- **Kimodo** [V] (constraints doc above): types `fullbody` (local axis-angle rotations + root positions; the constraint acts on the FK joint positions), `left/right-hand/foot` end-effectors (affect only that effector and the hips), `root2d` (path + heading). A constraint need not start at frame 0; frames before it are free; it must be reachable from the prompt and duration. Units m, Y up, XZ relative to frame 0, Y absolute hip height. Hard vs soft is not documented. Our worker implements only `pelvis_height`, `root`, `hand_l/hand_r`; `start_pose` is "not yet (MG8)" (`scrib-motiongen/README.md:64-67`), and keyframes are capped below 20 per type (`recipe.py:22`). So arm 1 needs MG8: a `fullbody` constraint at first and last frame. [I] Soft; Kimodo post-process (foot skate, constraint cleanup; flag `--no-postprocess`, `kimodo_backend.py:339`) may snap constrained frames, so run with and without it.
- **MotionBricks** [V] (https://github.com/NVlabs/motionbricks, https://huggingface.co/nvidia/motionbricks, arXiv 2604.24833): fills motion from 4 recent context poses to up to 4 target poses, ~0.6-1.9 s per segment at 30 Hz; SOMA and G1 ONNX models; TensorRT runtime (NVIDIA GPU, first launch builds a plan in minutes; tested Windows and Linux on a 4090); `uv sync` + Git LFS; Inbetweener demo. Code Apache-2.0, models Open Model Agreement, bundled BONES sample motions are evaluation-only (do not ship). [V] No v0.2.0 notes found (Releases empty) and no Kimodo integration beyond a README remark that Kimodo or ARDY could generate the target poses. [I] A "link" is our own glue: it should start and end on the given context/target poses (the hub poses) by construction, but segments are short, so stand-to-sit may need chaining. Joint count (30 vs 77) is in `soma/README.md` in the repo: check.

## 6. AN44 spike recommendation
Feasible on the RTX 4060 8 GB (CPU text encoder for Kimodo and ARDY):
1. **Kimodo + fullbody keyframes** at first and last frame = hub pose (needs MG8). Cheapest; do first, with and without post-process.
2. **ARDY SOMA** (`--model soma`, same hub keyframes at start/end via its constraint API, one text prompt per piece; KimodoRep variant only as a sub-arm if time). Risks: gated Llama token, C++ extension on Windows, unknown VRAM. Cap setup at one day; drop if it does not build. Convert npz to BVH with Kimodo's exporter.
3. **Kimodo + MotionBricks**: Inbetweener headless with hub pose as context and target (Kimodo only supplies a mid pose or the content). Feasible (Windows tested) but integration is ours: time-box, exploratory.

Measure per clip (3 pieces x ~8 seeds, same prompt, duration, 30 fps):
- Hub pose error at first and last frame: per-joint rotation difference in degrees and per-joint position difference in cm after aligning root XZ and heading; report mean and max over joints plus root height error; on the raw SOMA skeleton and after our retarget.
- Foot slide: frames with foot height under a contact threshold (tunable, start 3 cm), horizontal foot speed in cm/s, mean and p95 per clip, before and after any post-process. Also log the existing `FootSkateFromHeight` number from `clip_metrics` for continuity.
- Constraint error already in `.prov.json`, seconds per clip, peak VRAM.
Numbers are printed, not pass/fail; look is for the owner.
