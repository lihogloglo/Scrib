"""Guide v1 folder I/O and the small raster helpers the terrain-diffusion experiment needs.

A guide v1 folder holds `meta.json` (dims [w, h], sample_spacing_m, sea_level_m, origin_m = guide-frame
position of sample (0, 0), x east / y north, row 0 = north) and `heights.f32` (LE f32, row-major, metres).
"""
from __future__ import annotations

import json
from pathlib import Path

import numpy as np
from scipy import ndimage


def read_guide(folder: str | Path) -> tuple[np.ndarray, dict]:
    folder = Path(folder)
    meta = json.loads((folder / "meta.json").read_text(encoding="utf-8"))
    w, h = meta["dims"]
    heights = np.fromfile(folder / "heights.f32", dtype="<f4").reshape(h, w).astype(np.float32)
    return heights, meta


def guide_origin(meta: dict) -> tuple[float, float]:
    """Guide-frame metres of sample (0, 0). The raw source dump has no origin_m: it is origin_mw_units x 1.8288/128."""
    if "origin_m" in meta and meta["origin_m"] is not None:
        return float(meta["origin_m"][0]), float(meta["origin_m"][1])
    if "origin_mw_units" in meta:
        u = 1.8288 / 128.0
        return meta["origin_mw_units"][0] * u, meta["origin_mw_units"][1] * u
    return 0.0, 0.0


def write_guide(folder: str | Path, heights: np.ndarray, spacing_m: float, origin_m, sea_level_m: float = 0.0,
                extra: dict | None = None) -> None:
    folder = Path(folder)
    folder.mkdir(parents=True, exist_ok=True)
    h, w = heights.shape
    meta = {
        "format": 1,
        "dims": [int(w), int(h)],
        "sample_spacing_m": float(spacing_m),
        "sea_level_m": float(sea_level_m),
        "origin_m": [float(origin_m[0]), float(origin_m[1])],
        "fill_m": None,
        "orientation": "row-major, row 0 = north edge, column 0 = west edge; sample (col, row) at "
                       "x = origin_m.x + spacing col (east), y = origin_m.y - spacing row (north)",
        "producer": "tools/terrain-diffusion (xandergos/terrain-diffusion-30m experiment)",
    }
    if extra:
        meta.update(extra)
    np.ascontiguousarray(heights, dtype="<f4").tofile(folder / "heights.f32")
    (folder / "meta.json").write_text(json.dumps(meta, indent=2), encoding="utf-8")


def resample(heights: np.ndarray, src_spacing: float, dst_spacing: float) -> np.ndarray:
    """Resample a grid to a new spacing over the same frame (sample (0, 0) stays put).

    Anti-aliased when coarsening (Gaussian prefilter, sigma = 0.5 x ratio in source samples), bilinear lookup.
    """
    ratio = dst_spacing / src_spacing
    src = heights
    if ratio > 1.0:
        src = ndimage.gaussian_filter(heights, sigma=0.5 * ratio, mode="nearest")
    h, w = heights.shape
    ho = int(np.floor((h - 1) / ratio)) + 1
    wo = int(np.floor((w - 1) / ratio)) + 1
    rr, cc = np.meshgrid(np.arange(ho) * ratio, np.arange(wo) * ratio, indexing="ij")
    return ndimage.map_coordinates(src, [rr, cc], order=1, mode="nearest").astype(np.float32)


def lowpass(heights: np.ndarray, spacing_m: float, cutoff_m: float) -> np.ndarray:
    """Gaussian low-pass with half power at wavelength `cutoff_m` (sigma = sqrt(ln 2 / 2) / pi x cutoff)."""
    sigma_m = np.sqrt(np.log(2.0) / 2.0) / np.pi * cutoff_m
    return ndimage.gaussian_filter(heights, sigma=sigma_m / spacing_m, mode="nearest").astype(np.float32)


def bathymetry(heights: np.ndarray, spacing_m: float, sea_level_m: float, slope: float, floor_m: float,
               shelf_m: float) -> np.ndarray:
    """Sea samples (h < sea level) get at least `shelf_m + slope x distance-to-land` of depth, capped at floor_m.

    Land is untouched, so the coastline (the sea-level contour) stays where it was.
    """
    sea = heights < sea_level_m
    dist = ndimage.distance_transform_edt(sea) * spacing_m
    depth = np.minimum(shelf_m + slope * dist, floor_m)
    out = heights.copy()
    out[sea] = np.minimum(heights[sea], sea_level_m - depth[sea])
    return out
