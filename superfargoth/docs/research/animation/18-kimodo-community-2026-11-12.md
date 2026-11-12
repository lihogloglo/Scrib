# Kimodo: who else uses it, what they learned (2026-11-12)

Web research only. Finding: **little community material exists** (model is ~8 months old). No Reddit, X, YouTube,
ComfyUI or Godot threads surfaced; the 12 GitHub issues and the one HF discussion were not readable by the fetch tool
(unchecked, worth a manual look). Most of what follows is NVIDIA's own docs.

## Who uses it
- NVIDIA downstream: ProtoMotions, MuJoCo, GEAR-SONIC, GMR retargeter; related ARDY, MotionBricks, SOMA Retargeter ([repo](https://github.com/nv-tlabs/kimodo)).
- [kimodo-mcp](https://github.com/jon-jc/kimodo-mcp): local studio + MCP server, Blender/Unreal hand-off (BVH export; Unreal needs an existing matching skeleton; only one 24-bone character supported).
- [OzzyGT Diffusers conversion of RP-v1.2](https://huggingface.co/OzzyGT/Kimodo_SOMA_RP_v1_2).
- Third-party write-ups only: [NYU Shanghai](https://rits.shanghai.nyu.edu/ai/nvidia-releases-kimodo-controllable-text-to-motion-for-characters-and-humanoid-robots/), [Pebblous](https://blog.pebblous.ai/blog/kimodo-text-to-motion/en/).

## Tips (official docs, [best practices](https://research.nvidia.com/labs/sil/projects/kimodo/docs/key_concepts/limitations.html))
- Start prompts with "A person ..."; one or two behaviours; medium detail (match BONES-SEED prompt style).
- Dense 2D root paths are the one dense constraint the model is trained for; other constraints sparse, <20 keyframes each. Use root path for exact speed, not words.
- Keep constraints consistent with the prompt; to trade accuracy vs text fidelity change CFG (`--cfg_type separated` = separate text and constraint weights; no defaults published) ([repo](https://github.com/nv-tlabs/kimodo)).
- Keep post-processing on (foot skate and constraint cleanup); the raw model skates and misses constraints.
- Multi-prompt: each prompt self-contained; next clip is conditioned on previous last frames, so the transition eats the start of the second clip. Max 10 s per prompt.
- kimodo-mcp presets use 20/50/80 denoise steps; "more steps or candidates don't guarantee better output"; hands are not animated ([kimodo-mcp](https://github.com/jon-jc/kimodo-mcp)).
- Loops: no guidance anywhere; kimodo-mcp says seamless stitching is not guaranteed and a 120-sample 30 fps clip spans 3.97 s first-to-last key, so check the seam yourself ([kimodo-mcp](https://github.com/jon-jc/kimodo-mcp)).
- Posture (slouch, arms close to body), clavicle/shoulder issues, sample counts: nothing found. Try `--num_samples` and pick; "tired" style prompts are in-distribution, so avoid them for upright cycles.

## Limits
Foot skating, prompt mismatch, no cartoon or implausible motion, scene-unaware, one character at a time, stereotype bias; Llama 3 8B text encoder is gated and RAM-heavy ([HF card](https://huggingface.co/nvidia/Kimodo-SOMA-RP-v1.2), [kimodo-mcp](https://github.com/jon-jc/kimodo-mcp)).

## Licence
- Code Apache-2.0; most checkpoints (incl. RP-v1.2) NVIDIA Open Model License; card says "ready for commercial use" ([repo](https://github.com/nv-tlabs/kimodo), [HF card](https://huggingface.co/nvidia/Kimodo-SOMA-RP-v1.2)). SMPL-X variant is R&D-only.
- RP trained on proprietary Bones Rigplay (700 h); SEED on public BONES-SEED (288 h). Authors recommend RP for general use; SEED for benchmark comparison.
- Not verified: the Open Model License text on output ownership (read it before shipping) and BONES-SEED's own licence. A search found nothing.
- v1.1 leaked SEED test data into training; use v1.2 ([HF card](https://huggingface.co/nvidia/Kimodo-SOMA-RP-v1.2)).

## Alternatives
No head-to-head community verdicts found. MotionGPT reportedly sometimes yields near-irrelevant motion ([2025 paper](https://arxiv.org/html/2512.13840v2)); MoMask is HumanML3D-based. Kimodo's edge is constraints and 700 h of mocap ([paper](https://arxiv.org/html/2603.15546)). HY-Motion: nothing found.
