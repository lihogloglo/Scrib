"""Feature retention of conditioned arms: input vs output, both low-passed at ~1 km, over land; crater depth.

usage: python retention.py <arm folder>... [--json out.json]
Each arm folder holds input30.npy (input resampled to 30 m, same grid) and heights.f32 (+ raw_unclamped.npy).
Crater depth = deepest closed depression (priority-flood style fill by morphological reconstruction) within
--crater-r-km of the input's highest sample, on a --window-km crop (crop border = outlet).
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import numpy as np
from skimage.morphology import reconstruction

from guide_io import read_guide, lowpass

SP = 30.0


def fill_depth(h: np.ndarray) -> np.ndarray:
    seed = h.copy()
    seed[1:-1, 1:-1] = h.max()
    filled = reconstruction(seed, h, method="erosion")
    return filled - h


def crater(h: np.ndarray, rc, r_km: float, win_km: float):
    r, c = rc
    hw = int(win_km * 500 / SP)
    r0, r1 = max(0, r - hw), min(h.shape[0], r + hw)
    c0, c1 = max(0, c - hw), min(h.shape[1], c + hw)
    crop = h[r0:r1, c0:c1].astype(np.float64)
    d = fill_depth(crop)
    yy, xx = np.mgrid[r0:r1, c0:c1]
    near = (yy - r) ** 2 + (xx - c) ** 2 <= (r_km * 1000 / SP) ** 2
    dn = np.where(near, d, 0)
    k = np.unravel_index(np.argmax(dn), dn.shape)
    return {"depth_m": float(dn.max()), "summit_m": float(crop[near].max()),
            "floor_at_px": [int(k[0] + r0), int(k[1] + c0)]}


def stats(inp, out, land, cutoff_m):
    a = lowpass(inp, SP, cutoff_m)[land].astype(np.float64)
    b = lowpass(out, SP, cutoff_m)[land].astype(np.float64)
    return {"corr": float(np.corrcoef(a, b)[0, 1]), "rms_m": float(np.sqrt(np.mean((a - b) ** 2))),
            "bias_m": float(np.mean(b - a))}


def detail(inp, out, land, cut_m=500.0):
    """Band below ~500 m (h - low-pass): RMS in / out over land and their correlation (1 = model kept our detail)."""
    a = (inp - lowpass(inp, SP, cut_m))[land].astype(np.float64)
    b = (out - lowpass(out, SP, cut_m))[land].astype(np.float64)
    return {"rms_in_m": float(np.sqrt(np.mean(a * a))), "rms_out_m": float(np.sqrt(np.mean(b * b))),
            "corr": float(np.corrcoef(a, b)[0, 1])}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("arms", nargs="+")
    ap.add_argument("--crater-r-km", type=float, default=3.0)
    ap.add_argument("--window-km", type=float, default=16.0)
    ap.add_argument("--json")
    a = ap.parse_args()
    res = {}
    for arm in a.arms:
        p = Path(arm)
        inp = np.load(p / "input30.npy")
        out, meta = read_guide(p)
        raw = np.load(p / "raw_unclamped.npy")
        sea = float(meta.get("sea_level_m", 0.0))
        land = inp > sea
        rc = np.unravel_index(np.argmax(inp), inp.shape)
        r = {
            "lp1km": stats(inp, out, land, 1000.0),
            "lp1km_unclamped": stats(inp, raw, land, 1000.0),
            "lp3km": stats(inp, out, land, 3000.0),
            "detail_lt500m": detail(inp, out, land),
            "full_rms_diff_m": float(np.sqrt(np.mean((inp[land].astype(np.float64) - out[land]) ** 2))),
            "crater_in": crater(inp, rc, a.crater_r_km, a.window_km),
            "crater_out": crater(out, rc, a.crater_r_km, a.window_km),
            "coast_iou_before_clamp": meta.get("terrain_diffusion", {}).get("coast_iou_before_clamp"),
        }
        res[p.name] = r
        print(p.name, json.dumps(r))
    if a.json:
        Path(a.json).write_text(json.dumps(res, indent=1), encoding="utf-8")


if __name__ == "__main__":
    main()
