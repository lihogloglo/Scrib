"""Run one terrain-diffusion arm and write it as a guide v1 folder at 30 m.

Modes:
  stock   unconditioned model (synthetic Perlin conditioning), a square of --size-km at native (0, 0).
  coarse  documented path: our heights as the coarse elevation import (one value per 7.68 km cell,
          default -1000 m outside) + constant climate; stock latent/decoder.
  inject  coarse override (mean/p5 from our heights) + RePaint-style lowfreq injection (td_inject.py).

Run with the venv python from this folder (see README.md).
"""
from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
REPO = r"C:\tools\terrain-diffusion\repo"
sys.path.insert(0, REPO)

import numpy as np
import torch

from guide_io import read_guide, write_guide, resample, guide_origin
from td_inject import InjectedWorldPipeline, Guidance, CELL, LC

MODEL = "xandergos/terrain-diffusion-30m"
SPACING = 30.0
# Constant climate for conditioned arms (pipeline internal units): temp degC, temp std degC x100, precip mm,
# precip CV %. Mediterranean volcanic island (Etna/Teide-like); a Guess, kept equal across arms.
CLIMATE = {1: 16.0, 2: 600.0, 3: 700.0, 4: 60.0}
DOC_SNR = [0.2, 0.2, 1.0, 0.2, 1.0]  # tiff_export.py --snr default


def load_input(folder: str, scale: float):
    h, meta = read_guide(folder)
    spacing = float(meta["sample_spacing_m"]) * scale
    sea = float(meta.get("sea_level_m", 0.0)) * scale
    ox, oy = guide_origin(meta)
    h = h * scale
    h30 = resample(h, spacing, SPACING)
    return h30, sea, (ox * scale, oy * scale), meta


def sync():
    if torch.cuda.is_available():
        torch.cuda.synchronize()


def generate(pipe, ho, wo, i0=0, j0=0):
    """Touch each stage over the needed window in order, so each stage's time is measured on its own."""
    times = {}
    t = time.perf_counter()
    c0, c1 = (i0 // CELL) - 3, -(-(i0 + ho) // CELL) + 3
    d0, d1 = (j0 // CELL) - 3, -(-(j0 + wo) // CELL) + 3
    _ = pipe.coarse[:, c0:c1, d0:d1]
    sync(); times["coarse_s"] = time.perf_counter() - t
    t = time.perf_counter()
    _ = pipe.latents[:, i0 // LC - 8:(i0 + ho) // LC + 8, j0 // LC - 8:(j0 + wo) // LC + 8]
    sync(); times["latent_s"] = time.perf_counter() - t
    t = time.perf_counter()
    _ = pipe.residual[:, i0 - 64:i0 + ho + 64, j0 - 64:j0 + wo + 64]
    sync(); times["decoder_s"] = time.perf_counter() - t
    t = time.perf_counter()
    elev = pipe.get(i0, j0, i0 + ho, j0 + wo, with_climate=False)["elev"]
    times["compose_s"] = time.perf_counter() - t
    return elev.cpu().numpy().astype(np.float32), times


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", choices=["stock", "coarse", "inject"], required=True)
    ap.add_argument("--input", help="guide v1 folder (coarse/inject)")
    ap.add_argument("--scale", type=float, default=1.0, help="multiply input spacing and heights (raw source: 10)")
    ap.add_argument("--out", required=True)
    ap.add_argument("--seed", type=int, default=1234)
    ap.add_argument("--size-km", type=float, default=60.0, help="stock square")
    ap.add_argument("--sigmas", default="0.35", help="intermediate latent sigmas, comma separated (stock 0.35)")
    ap.add_argument("--no-final-replace", action="store_true")
    ap.add_argument("--decoder-sigma", default=None, help="SDEdit our 30 m residual: comma-separated step sigmas")
    ap.add_argument("--no-coast", action="store_true", help="do not clamp the output to the input coastline")
    ap.add_argument("--label", default="")
    a = ap.parse_args()
    import os
    a.out = os.path.abspath(a.out)
    a.input = os.path.abspath(a.input) if a.input else None
    os.chdir(REPO)  # the pipeline reads data/global/* relative to the clone

    sigmas = [float(s) for s in a.sigmas.split(",") if s.strip()]
    torch.backends.cudnn.benchmark = False
    torch.use_deterministic_algorithms(False)
    t_load = time.perf_counter()
    pipe = InjectedWorldPipeline.from_pretrained(MODEL, seed=a.seed, cache_limit=6 * 1024 ** 3,
                                                 latents_batch_size=[1, 2, 4, 8, 16], torch_compile=False)
    pipe.to("cuda")
    load_s = time.perf_counter() - t_load
    pipe.inject_sigmas = sigmas if a.mode == "inject" else [0.35]
    pipe.final_replace = not a.no_final_replace

    h30 = None
    i0 = j0 = 0
    prov = {"model": MODEL, "repo_commit": "e8dcb4b1a834ab2f6b1a6f5256ed7c9f2f3e8230", "seed": a.seed,
            "mode": a.mode, "label": a.label, "native_m": SPACING}
    if a.mode == "stock":
        n = int(round(a.size_km * 1000 / SPACING))
        ho = wo = n
        origin = (-n * SPACING / 2, n * SPACING / 2)
        pipe.bind()
        # Pick a land-rich, mountainous window: scan the coarse map (cheap) and take the k x k cell window with
        # the highest mean positive sqrt-elevation among those that are >= 60 % land.
        k = -(-n // CELL)
        cm = pipe.coarse[:, -64:64, -64:64]
        e = (cm[0] / cm[-1]).numpy()
        best, bi, bj = -1.0, 0, 0
        for i in range(0, 128 - k):
            for j in range(0, 128 - k):
                w = e[i:i + k, j:j + k]
                if (w > 0).mean() >= 0.6 and np.clip(w, 0, None).mean() > best:
                    best, bi, bj = float(np.clip(w, 0, None).mean()), i, j
        i0, j0 = (bi - 64) * CELL, (bj - 64) * CELL
        prov["stock_window_native_px"] = [i0, j0]
        pipe.empty_cache()
    else:
        h30, sea, origin, meta = load_input(a.input, a.scale)
        ho, wo = h30.shape
        g = Guidance(h30, sea)
        prov.update({"input": a.input, "input_scale": a.scale, "input_spacing_m": meta["sample_spacing_m"],
                     "sea_level_m": sea, "climate_const": CLIMATE, "cond_snr": DOC_SNR,
                     "bathymetry": "sea >= 30 m + 0.1 x distance-to-land deep, floor 1000 m"})
        pipe.set_cond_snr(DOC_SNR)
        # Documented import: elevation in metres per coarse cell (anchored at the guidance region), deep sea outside.
        ci, cj = g.coarse_origin
        pipe.set_custom_conditioning_import(0, g.coarse_metres(), ci, cj, default_value=-1000.0, rebuild=False)
        for ch, v in CLIMATE.items():
            pipe.set_custom_conditioning_import(ch, np.full((1, 1), v, np.float32), 0, 0, default_value=v,
                                                rebuild=False)
        if a.mode == "inject":
            pipe.guidance = g
            pipe.decoder_sigma = [float(x) for x in a.decoder_sigma.split(",")] if a.decoder_sigma else None
            prov["decoder_sigma"] = pipe.decoder_sigma
            prov.update({"inject_sigmas": sigmas, "final_replace": pipe.final_replace,
                         "coarse_override": True})
        pipe.bind()

    torch.cuda.reset_peak_memory_stats()
    elev, times = generate(pipe, ho, wo, i0, j0)
    times["load_s"] = load_s
    times["peak_vram_alloc_mb"] = torch.cuda.max_memory_allocated() / 2 ** 20
    times["peak_vram_reserved_mb"] = torch.cuda.max_memory_reserved() / 2 ** 20
    area = ho * wo * (SPACING / 1000) ** 2
    gen = times["coarse_s"] + times["latent_s"] + times["decoder_s"] + times["compose_s"]
    times.update({"area_km2": area, "total_gen_s": gen, "s_per_km2": gen / area})
    prov["timings"] = times
    print(json.dumps(times, indent=1))

    raw = elev.copy()
    if h30 is not None:
        land_in = h30 > sea
        land_out = elev > sea
        inter = np.logical_and(land_in, land_out).sum()
        union = np.logical_or(land_in, land_out).sum()
        prov["coast_iou_before_clamp"] = float(inter / max(union, 1))
        if not a.no_coast:
            clamped = (land_in & (elev <= sea + 0.5)) | (~land_in & (elev >= sea - 0.5))
            elev = np.where(land_in, np.maximum(elev, sea + 0.5), np.minimum(elev, sea - 0.5)).astype(np.float32)
            prov["coast_clamp"] = "land >= sea+0.5 m, sea <= sea-0.5 m where the input says so"
            prov["coast_clamped_share"] = float(clamped.mean())
    write_guide(a.out, elev, SPACING, origin, sea_level_m=(0.0 if h30 is None else sea),
                extra={"terrain_diffusion": prov})
    np.save(Path(a.out) / "raw_unclamped.npy", raw)
    if h30 is not None:
        np.save(Path(a.out) / "input30.npy", h30)
    print("wrote", a.out, elev.shape, float(elev.min()), float(elev.max()))


if __name__ == "__main__":
    main()
