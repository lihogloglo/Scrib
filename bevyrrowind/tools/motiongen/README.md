<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
# motiongen: offline clip factory (Kimodo batch worker)

Spec: `bevyrrowind/docs/specs/motion-generation.md` (MG1). Turns activity recipes (`recipes/*.toml`) into SOMA-77
BVH clips with NVIDIA Kimodo, in one persistent process. Offline tool only: never part of the game or the bake.

## Setup (once)

- Kimodo clone + venv outside the repo (Apache-2.0, imported as a library, never edited):
  `git clone https://github.com/nv-tlabs/kimodo C:\mbtest\kimodo`, then `uv venv` + `uv pip install -e .` in it
  (torch with CUDA, e.g. cu128). Python: `C:\mbtest\kimodo\.venv\Scripts\python.exe` (3.11).
- Hugging Face: `huggingface-cli login`, and accept the Meta Llama 3 licence for
  `meta-llama/Meta-Llama-3-8B-Instruct` (the LLM2Vec text encoder's base, ~15 GB). Weights land in the HF cache
  (`%USERPROFILE%\.cache\huggingface\hub`): `nvidia/Kimodo-SOMA-RP-v1.2` (1.1 GB) and the two McGill-NLP LLM2Vec repos.
- RAM: the encoder needs ~16 GB (bf16 on the CPU) while it encodes new prompts; it is freed before generation.

## Commands

Run from `bevyrrowind/tools` (or put that folder on `PYTHONPATH`) with the Kimodo venv python:

```sh
cd bevyrrowind/tools
PY=/c/mbtest/kimodo/.venv/Scripts/python.exe
$PY -m motiongen selftest                                    # parse recipes/, plan, format checks (no GPU)
$PY -m motiongen plan motiongen/recipes/seed.toml --out "../../animation packs/generated"   # dry run
$PY -m motiongen run  motiongen/recipes/seed.toml --out "../../animation packs/generated" [--only 'sit.*'] [--steps 50]
$PY -m motiongen bench motiongen/recipes/seed.toml --out <dir>   # speed matrix: steps, batch, precision, postprocess
$PY -m motiongen joints                                      # rewrite soma77_joints.txt
```

`run` options: `--bands <bands.json>` (default `<out>/bands.json` from the MG4 census if present, else
`default_bands.json`, placeholders), `--batch N` (max samples per GPU batch, default 8), `--steps N` (default 50),
`--precision fp32|fp16|bf16`, `--no-postprocess`, `--bvh-rest tpose|seed`, `--encoder-device cpu|cuda`,
`--cache <dir>`, `--force`. Resumable: a job whose `.bvh` + `.prov.json` all exist is skipped.

Output: `<out>/<activity>/<phase>/<activity>.<phase>.<band>.<nn>.bvh` + `.prov.json` (model, revision, prompt, seed,
constraints, steps, metrics, date, tool version) and `<out>/jobs.jsonl` (one line per GPU batch: stage timings,
VRAM, per-sample metrics). Prompt embeddings: `<out>/_cache/text-embeddings/<sha256>.npy`.

## Environment

| Variable | Use |
|---|---|
| `PYTHONIOENCODING=utf-8` | Windows console output |
| `TEXT_ENCODER_MODE=local` | Kimodo: no remote text encoder service |
| `TEXT_ENCODER_DEVICE` | set by motiongen from `--encoder-device` (default `cpu`) |
| `LOCAL_CACHE=true` | set by motiongen: weights from the HF cache, no network check |
| `HF_HUB_DISABLE_SYMLINKS_WARNING=1` | quiet HF cache warning on Windows |
| `MOTIONGEN_CACHE` | embedding cache dir (overrides `<out>/_cache/text-embeddings`) |

## Recipes

Format and constraint vocabulary: spec, "Recipes (MG1)". Implemented: `pelvis_height` (meters or
`band.<kind>[+offset]`, optional `{ height, at = "all"|"start"|"end"|[seconds] }`), `root = "fixed"` (XZ + facing),
`"fixed_xz"`, `"path:x,z;x,z"`, `hand_l`/`hand_r = { height, forward = 0.35, side = 0.2, at }` (wrist position; pins
the root XZ at its keyframes), `keys` (keyframe spacing, s). Not yet: `start_pose` (MG8). Unknown keys are errors.
Canonical space: meters, Y up, the actor starts at XZ (0, 0) facing +Z, left hand on +X.
