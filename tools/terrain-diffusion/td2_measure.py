"""TD-2 numbers for arm folders (guide v1, 30 m, same crop): retention vs the landmark target + landmark checks.

usage: td2_measure.py <td2 dir> <arm>... [--ref R0-target] [--json out.json] [--md out.md]

Per arm, against the reference (default R0-target, the composite the arms were steered towards):
  ret1k     corr / RMS of ~1 km low-passes over the reference's land
  band      corr of the 300 m - 2 km band (valley layout) and of the < 500 m band (fine detail), RMS out/ref
  craters   deepest closed depression (fill depth) within 1.5 km of the census centroid x10, its equivalent
            diameter (component of fill depth > 10 % of max around the deepest sample)
  foyadas   cross sections every 300 m along the census polyline x10: floor = min within +-210 m, banks = highest
            sample 150-1200 m each side, depth = lower bank - floor, floor width = width below floor + 25 % depth;
            medians over sections, and "present" = share of sections with depth >= 50 % of the reference's there
  summit    max height and its distance from the reference's summit
  coast     land IoU (h > 0) vs reference;  slope p50 over land (deg)
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import numpy as np
from scipy import ndimage
from skimage.morphology import reconstruction

from guide_io import read_guide, lowpass

SP = 30.0
CRATERS = {"crater-1": "Caldera crater", "crater-3": "Red Mountain crater"}
FOYADAS = {"trench-5": "Mamaea upper", "trench-1": "trench-1 (E of Ghostgate)", "trench-8": "Bani-Dad (S)",
           "trench-2": "Bani-Dad (N)"}


def fill_depth(h):
    seed = h.copy()
    seed[1:-1, 1:-1] = h.max()
    return reconstruction(seed, h, method="erosion") - h


def rc(meta, x, y):
    ox, oy = meta["origin_m"]
    return (oy - y) / SP, (x - ox) / SP


def crater(h, meta, xy, r_km=1.5, win_km=8.0):
    r, c = (int(round(v)) for v in rc(meta, *xy))
    hw = int(win_km * 500 / SP)
    r0, r1, c0, c1 = max(0, r - hw), min(h.shape[0], r + hw), max(0, c - hw), min(h.shape[1], c + hw)
    d = fill_depth(h[r0:r1, c0:c1].astype(np.float64))
    yy, xx = np.mgrid[r0:r1, c0:c1]
    near = (yy - r) ** 2 + (xx - c) ** 2 <= (r_km * 1000 / SP) ** 2
    dn = np.where(near, d, 0)
    k = np.unravel_index(np.argmax(dn), dn.shape)
    dmax = float(dn.max())
    if dmax < 1.0:
        return {"depth_m": round(dmax, 1), "diam_m": 0.0, "off_m": None}
    lab, _ = ndimage.label(d > 0.1 * dmax)
    comp = lab == lab[k]
    off = float(np.hypot(k[0] + r0 - r, k[1] + c0 - c) * SP)
    return {"depth_m": round(dmax, 1), "diam_m": round(float(2 * np.sqrt(comp.sum() / np.pi) * SP)),
            "off_m": round(off)}


def sections(meta, poly, step=300.0):
    p = np.asarray(poly, float)
    seg = np.diff(p, axis=0)
    L = np.hypot(seg[:, 0], seg[:, 1])
    s = np.concatenate([[0], np.cumsum(L)])
    out = []
    for d in np.arange(step / 2, s[-1], step):
        k = min(np.searchsorted(s, d) - 1, len(L) - 1)
        if L[k] == 0:
            continue
        t = (d - s[k]) / L[k]
        pt = p[k] + t * seg[k]
        tang = seg[k] / L[k]
        out.append((pt, np.array([-tang[1], tang[0]])))
    return out


def profiles(h, meta, poly):
    offs = np.arange(-1200, 1200 + 1, SP)
    res = []
    for pt, nrm in sections(meta, poly):
        xy = pt[None, :] + offs[:, None] * nrm[None, :]
        r, c = rc(meta, xy[:, 0], xy[:, 1])
        if r.min() < 0 or c.min() < 0 or r.max() > h.shape[0] - 1 or c.max() > h.shape[1] - 1:
            res.append(None)
            continue
        z = ndimage.map_coordinates(h, [r, c], order=1)
        mid = np.abs(offs) <= 210
        fi = np.where(mid)[0][np.argmin(z[mid])]
        floor = z[fi]
        left = z[(offs <= offs[fi] - 150)]
        right = z[(offs >= offs[fi] + 150)]
        if len(left) == 0 or len(right) == 0:
            res.append(None)
            continue
        depth = float(min(left.max(), right.max()) - floor)
        thr = floor + 0.25 * max(depth, 0)
        a = b = fi
        while a > 0 and z[a - 1] < thr:
            a -= 1
        while b < len(z) - 1 and z[b + 1] < thr:
            b += 1
        res.append((depth, float((b - a + 1) * SP)))
    return res


def foyada(h, meta, poly, ref_prof):
    pr = profiles(h, meta, poly)
    pairs = [(p, q) for p, q in zip(pr, ref_prof) if p is not None and q is not None]
    if not pairs:
        return {"n": 0}
    d = np.array([p[0] for p, _ in pairs])
    w = np.array([p[1] for p, _ in pairs])
    dr = np.array([q[0] for _, q in pairs])
    return {"n": len(pairs), "depth_med_m": round(float(np.median(d))), "floor_w_med_m": round(float(np.median(w))),
            "present": round(float(np.mean(d >= 0.5 * np.maximum(dr, 1.0))), 2)}


def slope_p50(h, land):
    gy, gx = np.gradient(h, SP)
    return float(np.degrees(np.arctan(np.median(np.hypot(gx, gy)[land]))))


def measure(h, meta, ref, lms, ref_prof):
    land = ref > 0
    a1, b1 = lowpass(ref, SP, 1000)[land], lowpass(h, SP, 1000)[land]
    band = lambda x: (lowpass(x, SP, 300) - lowpass(x, SP, 2000))[land]
    fine = lambda x: (x - lowpass(x, SP, 500))[land]
    r = {"ret1k_corr": round(float(np.corrcoef(a1, b1)[0, 1]), 3),
         "ret1k_rms_m": round(float(np.sqrt(np.mean((a1 - b1) ** 2))), 1),
         "band300_2k_corr": round(float(np.corrcoef(band(ref), band(h))[0, 1]), 3),
         "band300_2k_rms_m": round(float(np.sqrt(np.mean(band(h) ** 2))), 1),
         "fine500_corr": round(float(np.corrcoef(fine(ref), fine(h))[0, 1]), 3),
         "fine500_rms_m": round(float(np.sqrt(np.mean(fine(h) ** 2))), 1)}
    for fid, name in CRATERS.items():
        r[fid] = crater(h, meta, lms[fid]["centroid_10x"])
    for fid in FOYADAS:
        r[fid] = foyada(h, meta, lms[fid]["polyline_10x"], ref_prof[fid])
    k = np.unravel_index(np.argmax(h), h.shape)
    kr = np.unravel_index(np.argmax(ref), ref.shape)
    r["summit_m"] = round(float(h.max()))
    r["summit_off_m"] = round(float(np.hypot(k[0] - kr[0], k[1] - kr[1]) * SP))
    lo, lr_ = h > 0, ref > 0
    r["coast_iou"] = round(float((lo & lr_).sum() / max((lo | lr_).sum(), 1)), 3)
    r["slope_p50_deg"] = round(slope_p50(h, land), 1)
    return r


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("root")
    ap.add_argument("arms", nargs="+")
    ap.add_argument("--ref", default="R0-target")
    ap.add_argument("--json")
    a = ap.parse_args()
    root = Path(a.root)
    lms = json.loads((root / "_work/landmarks.json").read_text())
    ref, meta = read_guide(root / a.ref)
    ref_prof = {fid: profiles(ref, meta, lms[fid]["polyline_10x"]) for fid in FOYADAS}
    rows = {}
    for arm in [a.ref] + a.arms:
        h, m = read_guide(root / arm)
        assert m["origin_m"] == meta["origin_m"] and h.shape == ref.shape, arm
        rows[arm] = measure(h, m, ref, lms, ref_prof)
        print(arm, json.dumps(rows[arm]), flush=True)
    if a.json:
        Path(a.json).write_text(json.dumps(rows, indent=1), encoding="utf-8")


if __name__ == "__main__":
    main()
