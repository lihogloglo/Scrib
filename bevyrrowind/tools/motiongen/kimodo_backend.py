# SPDX-License-Identifier: GPL-3.0-or-later
"""Kimodo worker: one persistent model, a disk-cached text encoder, batched samples, SOMA-77 BVH out.

Kimodo (Apache-2.0, https://github.com/nv-tlabs/kimodo) is imported as a library and never edited.
Its pieces used here: `kimodo.load_model` (with a pre-built text encoder), the model's `__call__`,
`kimodo.constraints.Root2DConstraintSet`, `kimodo.exports.bvh.save_motion_bvh`,
`kimodo.metrics.foot_skate.FootSkateFromHeight`. Our own constraint classes only implement the
`update_constraints(data_dict, index_dict)` protocol that `kimodo.motion_rep.conditioning` consumes.
"""

from __future__ import annotations

import gc
import hashlib
import os
import time
from pathlib import Path

import numpy as np
import torch

MODEL_NAME = "Kimodo-SOMA-RP-v1.2"
ENCODER_PRESET = "llm2vec"


def _now() -> float:
    if torch.cuda.is_available():
        torch.cuda.synchronize()
    return time.perf_counter()


# ---------------------------------------------------------------- text embeddings


class CachedTextEncoder:
    """Drop-in for Kimodo's text encoder: `(texts) -> (feat [N, 1, 4096], lengths)`.

    Embeddings live on disk (`<cache>/<sha256(encoder_id + text)>.npy`); the 8B LLM2Vec encoder is
    loaded only on a cache miss, and `release()` frees it (16 GB of RAM in bf16).
    """

    def __init__(self, cache_dir: Path, device: str = "cpu", dtype: str = "bfloat16"):
        from kimodo.model.load_model import TEXT_ENCODER_PRESETS

        self.cache_dir = Path(cache_dir)
        self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.enc_device = device
        self.dtype = dtype
        kw = TEXT_ENCODER_PRESETS[ENCODER_PRESET]["kwargs"]
        self.llm_dim = int(kw["llm_dim"])
        # Device is part of the id: bf16 on CPU and on CUDA give slightly different embeddings.
        self.encoder_id = f"{kw['peft_model_name_or_path']}|{dtype}|{device.split(':')[0]}"
        self._encoder = None
        self.out_device = "cpu"
        self.load_s = 0.0
        self.encode_s = 0.0  # accumulated; read and reset by the worker per job
        self.hits = 0
        self.misses = 0

    def _path(self, text: str) -> Path:
        h = hashlib.sha256(f"{self.encoder_id}\n{text}".encode("utf-8")).hexdigest()
        return self.cache_dir / f"{h}.npy"

    def has(self, text: str) -> bool:
        return not text.strip() or self._path(text).exists()

    def _load(self) -> None:
        from kimodo.model import LLM2VecEncoder
        from kimodo.model.load_model import TEXT_ENCODER_PRESETS

        t0 = time.perf_counter()
        kw = dict(TEXT_ENCODER_PRESETS[ENCODER_PRESET]["kwargs"])
        kw["dtype"] = self.dtype
        kw["device"] = self.enc_device
        os.environ["TEXT_ENCODER_DEVICE"] = self.enc_device  # the Kimodo wrapper lets this env var win
        self._encoder = LLM2VecEncoder(**kw)
        self.load_s += time.perf_counter() - t0
        print(f"[motiongen] text encoder loaded on {self.enc_device} in {time.perf_counter() - t0:.1f} s", flush=True)

    def release(self) -> None:
        if self._encoder is not None:
            self._encoder = None
            gc.collect()
            if torch.cuda.is_available():
                torch.cuda.empty_cache()

    def embed(self, text: str) -> np.ndarray:
        if not text.strip():
            return np.zeros((1, self.llm_dim), dtype=np.float32)
        p = self._path(text)
        if p.exists():
            self.hits += 1
            return np.load(p)
        self.misses += 1
        if self._encoder is None:
            self._load()
        t0 = time.perf_counter()
        feat, _ = self._encoder([text])
        arr = feat.float().cpu().numpy()[0]  # [1, 4096]
        self.encode_s += time.perf_counter() - t0
        tmp = p.with_suffix(".tmp.npy")
        np.save(tmp, arr)
        os.replace(tmp, p)
        return arr

    def __call__(self, text):
        single = isinstance(text, str)
        texts = [text] if single else list(text)
        t0 = time.perf_counter()
        uniq = {t: self.embed(t) for t in dict.fromkeys(texts)}
        feat = torch.from_numpy(np.stack([uniq[t] for t in texts])).to(self.out_device)
        self._last_call_s = time.perf_counter() - t0
        lengths = [1] * len(texts)
        if single:
            return feat[0], lengths[0]
        return feat, lengths

    # Kimodo calls .to()/.eval() on its text encoder; embeddings are moved per call instead.
    def to(self, device):
        self.out_device = str(device)
        return self

    def eval(self):
        return self

    def get_device(self):
        return torch.device(self.out_device)


# ---------------------------------------------------------------- constraints


class PelvisHeightConstraint:
    """Root (pelvis) height keyframes only: Kimodo's `root_y_pos` feature, nothing else."""

    name = "mg-pelvis-height"

    def __init__(self, frames: list[int], y: float, device):
        self.frame_indices = torch.tensor(frames)
        self.y = torch.full((len(frames),), float(y), device=device)

    def update_constraints(self, data_dict, index_dict):
        data_dict["root_y_pos"].append(self.y)
        index_dict["root_y_pos"].append(self.frame_indices)


class HandPositionConstraint:
    """Wrist position keyframes (no rotation), relative to a smooth root pinned at those frames."""

    def __init__(self, skeleton, side: str, frames: list[int], pos: list[float], device):
        joint = "LeftHand" if side == "l" else "RightHand"
        self.name = f"mg-hand-{side}"
        self.frame_indices = torch.tensor(frames)
        j = skeleton.bone_index[joint]
        self.pairs = torch.stack([self.frame_indices, torch.full_like(self.frame_indices, j)], dim=1)
        self.pos = torch.tensor([pos] * len(frames), dtype=torch.float32, device=device)
        self.root2d = torch.zeros((len(frames), 2), device=device)

    def update_constraints(self, data_dict, index_dict):
        data_dict["global_joints_positions"].append(self.pos)
        index_dict["global_joints_positions"].append(self.pairs)
        data_dict["smooth_root_2d"].append(self.root2d)
        index_dict["smooth_root_2d"].append(self.frame_indices)


def build_constraints(resolved: dict, skeleton, device) -> list:
    """Map `recipe.resolve_constraints` output to Kimodo constraint objects (model skeleton = SOMA-30)."""
    from kimodo.constraints import Root2DConstraintSet

    out = []
    if "root" in resolved:
        r = resolved["root"]
        frames = torch.tensor(r["frames"])
        pts = torch.tensor(r["points"], dtype=torch.float32, device=device)
        heading = None
        if r["type"] == "fixed":  # also keep the initial facing (+Z, heading angle 0 -> [cos, sin] = [1, 0])
            heading = torch.tensor([[1.0, 0.0]] * len(r["frames"]), device=device)
        out.append(Root2DConstraintSet(skeleton, frames, pts, global_root_heading=heading))
    if "pelvis_height" in resolved:
        p = resolved["pelvis_height"]
        out.append(PelvisHeightConstraint(p["frames"], p["y"], device))
    for side in ("l", "r"):
        h = resolved.get(f"hand_{side}")
        if h:
            out.append(HandPositionConstraint(skeleton, side, h["frames"], h["pos"], device))
    return out


# ---------------------------------------------------------------- metrics


def clip_metrics(posed_joints: torch.Tensor, skel77, fps: float, resolved: dict) -> list[dict]:
    """Per-sample sanity numbers (printed / logged, never pass/fail): foot skate, jitter, constraint error."""
    from kimodo.metrics.foot_skate import FootSkateFromHeight

    pj = posed_joints.float()
    B, T = pj.shape[:2]
    lengths = torch.full((B,), T, device=pj.device)
    skate = FootSkateFromHeight(skeleton=skel77, fps=fps)._compute(posed_joints=pj, lengths=lengths)["foot_skate_from_height"]
    jerk = (pj[:, 3:] - 3 * pj[:, 2:-1] + 3 * pj[:, 1:-2] - pj[:, :-3]) * fps**3
    jerk = jerk.norm(dim=-1).mean(dim=(1, 2))
    root = skel77.root_idx
    out = []
    for b in range(B):
        m = {"foot_skate_mps": round(float(skate[b]), 4), "jerk_mps3": round(float(jerk[b]), 1)}
        if "pelvis_height" in resolved:
            p = resolved["pelvis_height"]
            y = pj[b, p["frames"], root, 1]
            m["pelvis_height_err_m"] = round(float((y - p["y"]).abs().mean()), 4)
        if "root" in resolved and resolved["root"]["type"] != "path":
            xz = pj[b, :, root][:, [0, 2]]
            m["root_drift_max_m"] = round(float(xz.norm(dim=-1).max()), 4)
        for side, joint in (("l", "LeftHand"), ("r", "RightHand")):
            h = resolved.get(f"hand_{side}")
            if h:
                j = skel77.bone_index[joint]
                tgt = torch.tensor(h["pos"], device=pj.device)
                m[f"hand_{side}_err_m"] = round(float((pj[b, h["frames"], j] - tgt).norm(dim=-1).mean()), 4)
        out.append(m)
    return out


# ---------------------------------------------------------------- worker


def _kimodo_provenance(model_name: str) -> dict:
    from importlib import metadata

    from huggingface_hub import snapshot_download

    from kimodo.model.loading import MODEL_NAMES

    repo = MODEL_NAMES.get(model_name, model_name)
    rev = None
    try:
        rev = Path(snapshot_download(repo_id=repo, local_files_only=True)).name
    except Exception:  # noqa: BLE001 - provenance is best effort
        pass
    git = None
    try:
        import kimodo

        head = Path(kimodo.__file__).resolve().parents[1] / ".git"
        ref = (head / "HEAD").read_text().strip()
        git = (head / ref[5:]).read_text().strip() if ref.startswith("ref: ") else ref
    except Exception:  # noqa: BLE001
        pass
    try:
        ver = metadata.version("kimodo")
    except metadata.PackageNotFoundError:
        ver = None
    return {"model": model_name, "hf_repo": repo, "hf_revision": rev, "kimodo_version": ver, "kimodo_git": git}


class KimodoWorker:
    def __init__(self, cache_dir: Path, encoder_device: str = "cpu", precision: str = "fp32", postprocess: bool = True,
                 bvh_rest: str = "tpose"):
        os.environ.setdefault("LOCAL_CACHE", "true")  # resolve weights from the HF cache without a network check
        from kimodo import load_model
        from kimodo.model.registry import resolve_model_name

        self.device = "cuda:0" if torch.cuda.is_available() else "cpu"
        torch.backends.cuda.matmul.allow_tf32 = True
        torch.backends.cudnn.allow_tf32 = True
        self.encoder = CachedTextEncoder(cache_dir, device=encoder_device)
        self.precision = precision
        self.postprocess = postprocess
        self.bvh_rest = bvh_rest  # "tpose": standard T-pose rest (world-axis offsets); "seed": BONES-SEED rest
        t0 = time.perf_counter()
        name = MODEL_NAME
        self.model, self.resolved = load_model(name, device=self.device, text_encoder=self.encoder, return_resolved_name=True)
        self.encoder.to(self.device)
        self.load_s = time.perf_counter() - t0
        from kimodo.skeleton import SOMASkeleton30

        sk = self.model.skeleton
        self.skel77 = sk.somaskel77.to(self.device) if isinstance(sk, SOMASkeleton30) else sk
        self.fps = float(self.model.fps)
        self.provenance = _kimodo_provenance(self.resolved if "-" in self.resolved else name)
        self.provenance["model"] = name
        self._install_hooks()
        print(f"[motiongen] {name} loaded on {self.device} in {self.load_s:.1f} s (fps {self.fps:g})", flush=True)

    def _install_hooks(self) -> None:
        """Time post-processing and wrap the denoiser in autocast, without touching Kimodo's files."""
        import kimodo.model.kimodo_model as km

        orig_pp = km.post_process_motion
        self.pp_s = 0.0

        def timed_pp(*a, **kw):
            t0 = _now()
            r = orig_pp(*a, **kw)
            self.pp_s += _now() - t0
            return r

        km.post_process_motion = timed_pp
        self._install_precision(self.precision)

    def _install_precision(self, precision: str) -> None:
        """fp32: Kimodo's own denoising_step; fp16/bf16: the same step under torch.autocast."""
        self.model.__dict__.pop("denoising_step", None)
        self.precision = precision
        if precision not in ("fp16", "bf16"):
            return
        dtype = torch.float16 if precision == "fp16" else torch.bfloat16
        orig_step = self.model.denoising_step

        def step(*a, **kw):
            with torch.autocast("cuda", dtype=dtype):
                return orig_step(*a, **kw).float()

        self.model.denoising_step = step

    def warm_embeddings(self, prompts: list[str]) -> dict:
        """Encode every missing prompt up front, then free the 8B encoder (cache-only from then on)."""
        missing = [p for p in dict.fromkeys(prompts) if not self.encoder.has(p)]
        t0 = time.perf_counter()
        for p in missing:
            self.encoder.embed(p)
        self.encoder.release()
        return {"prompts": len(set(prompts)), "encoded": len(missing), "s": round(time.perf_counter() - t0, 2),
                "encoder_load_s": round(self.encoder.load_s, 2)}

    def generate(self, prompt: str, num_frames: int, num_samples: int, steps: int, seed: int, resolved: dict) -> tuple[dict, dict]:
        from kimodo.tools import seed_everything

        seed_everything(seed)
        cons = build_constraints(resolved, self.model.skeleton, self.device)
        self.pp_s = 0.0
        self.encoder._last_call_s = 0.0
        t0 = _now()
        out = self.model(
            prompt,
            num_frames,
            num_denoising_steps=steps,
            constraint_lst=cons,
            num_samples=num_samples,
            post_processing=self.postprocess,
            return_numpy=False,
            progress_bar=lambda x: x,
        )
        total = _now() - t0
        enc = self.encoder._last_call_s  # whole encoder call: cache reads (+ encoding on a miss)
        timings = {"encode_s": enc, "postprocess_s": self.pp_s, "diffusion_s": max(0.0, total - enc - self.pp_s)}
        return out, timings

    def export(self, out: dict, paths: list[Path]) -> float:
        from kimodo.exports.bvh import save_motion_bvh
        from kimodo.skeleton import global_rots_to_local_rots

        t0 = time.perf_counter()
        sk = self.skel77
        for i, path in enumerate(paths):
            path.parent.mkdir(parents=True, exist_ok=True)
            joints_pos = out["posed_joints"][i].float()
            local = global_rots_to_local_rots(out["global_rot_mats"][i].float(), sk)
            tmp = path.with_suffix(".tmp")
            save_motion_bvh(str(tmp), local, joints_pos[:, sk.root_idx, :], skeleton=sk, fps=self.fps,
                            standard_tpose=self.bvh_rest == "tpose")
            os.replace(tmp, path)
        return time.perf_counter() - t0

    def vram_peak_mb(self) -> float:
        if not torch.cuda.is_available():
            return 0.0
        return torch.cuda.max_memory_allocated() / 2**20
