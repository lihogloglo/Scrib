"""Frequency-split blend: our heights above a cutoff wavelength + a model arm's detail below it (CPU only).

  out = lowpass(input, cut) + (model - lowpass(model, cut)), then the input coastline clamp.

usage: python freqsplit.py <model arm folder> <out folder> [--cut-m 1000]
The model arm must hold input30.npy and raw_unclamped.npy (run_arm.py writes them).
"""
from __future__ import annotations

import argparse
import json
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import numpy as np

from guide_io import read_guide, write_guide, lowpass

SP = 30.0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("arm")
    ap.add_argument("out")
    ap.add_argument("--cut-m", type=float, default=1000.0)
    ap.add_argument("--label", default="")
    a = ap.parse_args()
    src = Path(a.arm)
    inp = np.load(src / "input30.npy")
    model = np.load(src / "raw_unclamped.npy")
    _, meta = read_guide(src)
    sea = float(meta.get("sea_level_m", 0.0))
    out = lowpass(inp, SP, a.cut_m) + (model - lowpass(model, SP, a.cut_m))
    raw = out.astype(np.float32)
    land = inp > sea
    iou = float((land & (raw > sea)).sum() / max((land | (raw > sea)).sum(), 1))
    out = np.where(land, np.maximum(raw, sea + 0.5), np.minimum(raw, sea - 0.5)).astype(np.float32)
    prov = dict(meta.get("terrain_diffusion", {}))
    prov.update({"label": a.label, "freqsplit": {"detail_from": str(src), "cut_m": a.cut_m,
                                                 "formula": "lowpass(input) + (model - lowpass(model))"},
                 "coast_iou_before_clamp": iou})
    write_guide(a.out, out, SP, meta["origin_m"], sea_level_m=sea, extra={"terrain_diffusion": prov})
    np.save(Path(a.out) / "raw_unclamped.npy", raw)
    shutil.copyfile(src / "input30.npy", Path(a.out) / "input30.npy")
    print("wrote", a.out, json.dumps({"iou": iou}))


if __name__ == "__main__":
    main()
