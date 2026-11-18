"""TD-2: landmark steering of terrain-diffusion-30m by decoder inversion + latent RePaint / SDEdit (research only).

Pipeline (xandergos/terrain-diffusion e8dcb4b, MIT; nothing in the clone is edited):
  latent px = 240 m, channels 0-3 = VAE latent of the 30 m sqrt-elevation residual (everything from 30 m to the
  ~6 km cut-off of the Laplacian split, so craters and ravines live here), channel 4 = blurred lowfreq.
  The weights ship no encoder for channels 0-3, so we *invert the decoder*: optimise z (4 x 240 m) so that the
  one-step consistency decoder, run exactly as the stock pipeline runs it (same per-position noise, 512/384 tiles),
  reproduces the target's residual. Then the base model's 2-step latent sampler is steered RePaint-style:
  before its second step the clean estimate's channels 0-3 are replaced by z inside a "known" mask (landmarks, or
  the whole crop), renoised to sigma s, and denoised; `known_final` replaces them again after the last step
  (hard keep), otherwise the model's re-imagined version is kept (SDEdit strength = s). Channel 4 = our lowfreq
  everywhere (TD-1 inject, td_inject.py).

Target (`build_target`): V3 (our 10x island) + inside feathered landmark masks the < 4 km band of the raw
Morrowind heightmap x10 (Caldera crater, Red Mountain crater, foyada trenches, Ghostgate), from
target/worldgen/lf/source/features.json. Everything runs on the whole island's guidance; only the crop is decoded.

usage (venv python, from the repo root):
  td_steer.py --arms B1,I0,I1,I2,I3 --out target/worldgen/td2
"""
from __future__ import annotations

import argparse
import json
import math
import os
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
REPO = r"C:\tools\terrain-diffusion\repo"
sys.path.insert(0, REPO)

import numpy as np
import torch
from scipy import ndimage

from guide_io import read_guide, write_guide, resample, guide_origin, lowpass
from td_inject import InjectedWorldPipeline, Guidance, CELL, LC

MODEL = "xandergos/terrain-diffusion-30m"
SP = 30.0
CLIMATE = {1: 16.0, 2: 600.0, 3: 700.0, 4: 60.0}  # same Guess as TD-1 run_arm.py
DOC_SNR = [0.2, 0.2, 1.0, 0.2, 1.0]
# Crop on V3's 30 m grid (rows/cols of the 30 m resample, multiples of 8): x -7969..16991, y 21420..-3540 (10x m).
CROP = (376, 408, 832)  # r0, c0, size (native px)
LANDMARK_IDS = {  # source census id -> (buffer m at 10x, label)
    "crater-1": (500.0, "Caldera crater"), "crater-3": (800.0, "Red Mountain crater"),
    "basin-15": (400.0, "Ghostgate"),
    "trench-1": (600.0, "Foyada (census trench-1)"), "trench-5": (600.0, "Foyada Mamaea (upper)"),
    "trench-9": (600.0, "Foyada Mamaea (lower)"), "trench-2": (600.0, "Foyada Bani-Dad"),
    "trench-8": (600.0, "Foyada Bani-Dad"), "trench-3": (600.0, "Foyada Bani-Dad"),
    "trench-7": (600.0, "Foyada Bani-Dad"),
}
FEATHER_M = 500.0
HP_CUT_M = 4000.0
ROOT = Path(__file__).resolve().parents[2]


def ssqrt(x):
    return np.sign(x) * np.sqrt(np.abs(x))


# ---------------------------------------------------------------------------------------------------------------
# target
# ---------------------------------------------------------------------------------------------------------------
def on_grid(src: np.ndarray, src_origin, src_sp, dst_shape, dst_origin, dst_sp) -> np.ndarray:
    """Bilinear lookup of `src` (guide frame, row 0 north) at the sample positions of a dst grid."""
    if dst_sp > src_sp:
        src = ndimage.gaussian_filter(src, sigma=0.5 * dst_sp / src_sp, mode="nearest")
    rr, cc = np.mgrid[0:dst_shape[0], 0:dst_shape[1]].astype(np.float64)
    x = dst_origin[0] + cc * dst_sp
    y = dst_origin[1] - rr * dst_sp
    return ndimage.map_coordinates(src, [(src_origin[1] - y) / src_sp, (x - src_origin[0]) / src_sp], order=1,
                                   mode="nearest").astype(np.float32)


def raster_feature(f, shape, origin, sp, scale=10.0):
    """Binary raster of a census feature outline (polygon) or polyline at 10x."""
    from skimage.draw import polygon, line
    m = np.zeros(shape, bool)
    pts = f.get("outline_m") or f.get("polyline_m") or []
    if not pts:
        return m
    p = np.asarray(pts) * scale
    rows = (origin[1] - p[:, 1]) / sp
    cols = (p[:, 0] - origin[0]) / sp
    if f.get("outline_m"):
        rr, cc = polygon(rows, cols, shape)
        m[rr, cc] = True
    for k in range(len(p) - 1):
        rr, cc = line(int(round(rows[k])), int(round(cols[k])), int(round(rows[k + 1])), int(round(cols[k + 1])))
        ok = (rr >= 0) & (rr < shape[0]) & (cc >= 0) & (cc < shape[1])
        m[rr[ok], cc[ok]] = True
    return m


def build_target():
    v3, vm = read_guide(ROOT / "target/worldgen/vo/V3")
    v3o = tuple(vm["origin_m"])
    v30 = resample(v3, float(vm["sample_spacing_m"]), SP)
    raw, rm = read_guide(ROOT / "target/worldgen/vvardenfell")
    ro = guide_origin(rm)
    raw10 = on_grid(raw * 10.0, (ro[0] * 10, ro[1] * 10), float(rm["sample_spacing_m"]) * 10, v30.shape, v3o, SP)
    hp = raw10 - lowpass(raw10, SP, HP_CUT_M)
    feats = {f["id"]: f for f in json.loads((ROOT / "target/worldgen/lf/source/features.json").read_text())["features"]}
    masks = {}
    paste = np.zeros(v30.shape, np.float32)
    for fid, (buf, label) in LANDMARK_IDS.items():
        b = raster_feature(feats[fid], v30.shape, v3o, SP)
        d = ndimage.distance_transform_edt(~b) * SP
        m = np.clip(1.0 - (d - buf) / FEATHER_M, 0.0, 1.0).astype(np.float32)
        masks[fid] = {"label": label, "core": b, "mask": m, "metrics_src": feats[fid].get("metrics", {}),
                      "centroid_10x": [c * 10 for c in feats[fid]["centroid_m"]],
                      "polyline_10x": (np.asarray(feats[fid]["polyline_m"]) * 10).tolist()
                      if feats[fid].get("polyline_m") else None}
        paste = np.maximum(paste, m)
    target = (v30 + paste * hp).astype(np.float32)
    return target, v30, raw10, paste, masks, v3o


def crop(a, c=CROP):
    r0, c0, n = c
    return a[..., r0:r0 + n, c0:c0 + n]


def crop_origin(v3o, c=CROP):
    return (v3o[0] + c[1] * SP, v3o[1] - c[0] * SP)


# ---------------------------------------------------------------------------------------------------------------
# steering pipeline
# ---------------------------------------------------------------------------------------------------------------
class SteerPipeline(InjectedWorldPipeline):
    """TD-1 injection (channel 4 everywhere) + RePaint of channels 0-3 inside a known mask (see module docs)."""

    known_z: torch.Tensor | None = None  # (4, lr, lcn) in the guidance region's latent frame
    known_m: torch.Tensor | None = None  # (lr, lcn)
    known_final: bool = False

    def _inject_known(self, ctx, tile):
        if self.known_z is None:
            return tile
        g = self.guidance
        size = tile.shape[-1]
        i0, j0 = ctx[1] * 32, ctx[2] * 32
        oi, oj = g.lat_origin
        lr, lcn = self.known_m.shape
        a0, a1 = max(i0, oi), min(i0 + size, oi + lr)
        b0, b1 = max(j0, oj), min(j0 + size, oj + lcn)
        if a0 >= a1 or b0 >= b1:
            return tile
        m = self.known_m[a0 - oi:a1 - oi, b0 - oj:b1 - oj]
        if not m.any():
            return tile
        tile = tile.clone()
        w = tile[-1, a0 - i0:a1 - i0, b0 - j0:b1 - j0]
        z = self.known_z[:, a0 - oi:a1 - oi, b0 - oj:b1 - oj]
        sub = tile[:4, a0 - i0:a1 - i0, b0 - j0:b1 - j0]
        tile[:4, a0 - i0:a1 - i0, b0 - j0:b1 - j0] = (1 - m) * sub + m * z * w
        return tile

    def _guided_latent(self, ctxs, samples, conds, t, scheduler, weight_window, hist, means, stds, seed_offset,
                       final):
        if self.guidance is not None and samples is not None:
            samples = [self._inject_known(c, self._inject(c, s)) for c, s in zip(ctxs, samples)]
        outputs = self._latent_inference(ctxs, samples, conds, t, scheduler, weight_window, hist, means, stds,
                                         seed_offset=seed_offset)
        if self.guidance is not None and final:
            if self.final_replace:
                outputs = [self._inject(c, o) for c, o in zip(ctxs, outputs)]
            if self.known_final:
                outputs = [self._inject_known(c, o) for c, o in zip(ctxs, outputs)]
        return outputs


def make_pipe(seed):
    pipe = SteerPipeline.from_pretrained(MODEL, seed=seed, cache_limit=6 * 1024 ** 3,
                                         latents_batch_size=[1, 2, 4, 8, 16], torch_compile=False)
    pipe.to("cuda")
    return pipe


def setup(pipe, g, sigmas, histogram=None):
    pipe.inject_sigmas = sigmas
    pipe.final_replace = True
    pipe.set_cond_snr(DOC_SNR)
    ci, cj = g.coarse_origin
    pipe.set_custom_conditioning_import(0, g.coarse_metres(), ci, cj, default_value=-1000.0, rebuild=False)
    for ch, v in CLIMATE.items():
        pipe.set_custom_conditioning_import(ch, np.full((1, 1), v, np.float32), 0, 0, default_value=v, rebuild=False)
    if histogram is not None:
        pipe.kwargs["histogram_raw"] = histogram
    pipe.guidance = g
    pipe.decoder_sigma = None
    if pipe.tile_store is None:
        pipe.bind()
    else:
        pipe.rebuild()


def decode_crop(pipe, c=CROP):
    r0, c0, n = c
    t = time.perf_counter()
    _ = pipe.latents[:, r0 // LC - 8:(r0 + n) // LC + 8, c0 // LC - 8:(c0 + n) // LC + 8]
    torch.cuda.synchronize()
    lat_s = time.perf_counter() - t
    t = time.perf_counter()
    elev = pipe.get(r0, c0, r0 + n, c0 + n, with_climate=False)["elev"]
    torch.cuda.synchronize()
    return elev.cpu().numpy().astype(np.float32), {"latent_s": lat_s, "decoder_compose_s": time.perf_counter() - t}


# ---------------------------------------------------------------------------------------------------------------
# decoder inversion
# ---------------------------------------------------------------------------------------------------------------
def decoder_tile(pipe, z, ti, tj, tile=512, stride=384):
    """Stock decoder for tile (ti, tj) (world_pipeline.py _decoder_inference, T=1 from sigma_max), differentiable in z."""
    from terrain_diffusion.inference.world_pipeline import gaussian_noise_patch
    sd = 0.5
    t = math.atan(80.0 / sd)
    noise = torch.as_tensor(gaussian_noise_patch(pipe.seed + 5819, ti * stride, tj * stride, tile, tile, channels=1,
                                                 tile_h=tile, tile_w=tile))[None].to("cuda")
    x_t = math.sin(t) * noise * sd  # cos(t) * 0 + sin(t) * z
    up = torch.nn.functional.interpolate(z[None], size=(tile, tile), mode="nearest")
    model_in = torch.cat([x_t / sd, up], dim=1)
    pred = -pipe.decoder_model(model_in, noise_labels=torch.tensor([t], device="cuda"), conditional_inputs=[])
    return (math.cos(t) * x_t - math.sin(t) * sd * pred)[0, 0] / sd


def invert(pipe, g, region, iters, lr, prior, log):
    """Optimise latent channels 0-3 over `region` (native r0, c0, r1, c1) so the decoder reproduces g.residual there.

    Returns (z over the latent window, window latent origin, region mask at latent res, stats)."""
    tile, stride = pipe.decoder_tile_size, pipe.decoder_tile_stride
    R0, C0, R1, C1 = region
    ti0, ti1 = -(-(R0 - tile + 1) // stride), (R1 - 1) // stride
    tj0, tj1 = -(-(C0 - tile + 1) // stride), (C1 - 1) // stride
    lr0, lc0 = ti0 * stride // LC, tj0 * stride // LC
    lr1, lc1 = (ti1 * stride + tile) // LC, (tj1 * stride + tile) // LC
    lat = pipe.latents[:, lr0:lr1, lc0:lc1]
    z0 = (lat[:5] / lat[5:])[:4].float().to("cuda")
    # target residual (normalised) over the native window
    nr0, nc0 = lr0 * LC, lc0 * LC
    nh, nw = (lr1 - lr0) * LC, (lc1 - lc0) * LC
    vals, _ = g.nat_patch(nr0, nc0, max(nh, nw))
    tgt = ((vals[:nh, :nw] - pipe.kwargs["residual_mean"]) / pipe.kwargs["residual_std"]).to("cuda")
    regm = torch.zeros((nh, nw), device="cuda")
    regm[R0 - nr0:R1 - nr0, C0 - nc0:C1 - nc0] = 1
    zm = torch.zeros((1, lr1 - lr0, lc1 - lc0), device="cuda")
    zm[:, (R0 - nr0) // LC:(R1 - nr0) // LC, (C0 - nc0) // LC:(C1 - nc0) // LC] = 1
    from terrain_diffusion.inference.world_pipeline import linear_weight_window
    win = linear_weight_window(tile, "cuda", torch.float32)
    var = z0.clone().requires_grad_(True)
    opt = torch.optim.Adam([var], lr=lr)
    tiles = [(ti, tj) for ti in range(ti0, ti1 + 1) for tj in range(tj0, tj1 + 1)]
    for p in pipe.decoder_model.parameters():
        p.requires_grad_(False)
    # parity: our differentiable tile == the stock decoder tile (fp32, no grad)
    with torch.no_grad():
        ti, tj = ti0, tj0
        zt = z0[:, :tile // LC, :tile // LC]
        mine = decoder_tile(pipe, zt, ti, tj, tile, stride).float().cpu()
        lat6 = torch.cat([zt.cpu(), torch.zeros(1, *zt.shape[1:]), torch.ones(1, *zt.shape[1:])])
        from terrain_diffusion.scheduler.dpmsolver import EDMDPMSolverMultistepScheduler
        sch = EDMDPMSolverMultistepScheduler(sigma_min=0.002, sigma_max=80, sigma_data=0.5)
        ww = torch.ones(tile, tile)
        stock = SteerPipeline.__mro__[2]._decoder_inference(
            pipe, (0, ti, tj), lat6, sch, ww, [torch.atan(sch.sigmas[0] / 0.5)], tile, stride)[0]
        log(f"decoder parity max|d| = {float((mine - stock).abs().max()):.2e} (std {float(stock.std()):.3f})")
    log(f"target residual std (normalised) in region {float(tgt[regm > 0].std()):.3f}")
    hist = []
    torch.backends.cudnn.benchmark = True  # 9x faster bf16 backward on this GPU; arms decode with it off
    t0 = time.perf_counter()
    for it in range(iters + 1):
        opt.zero_grad(set_to_none=True)
        z = z0 * (1 - zm) + var * zm
        tot, wsum = 0.0, 0.0
        for ti, tj in tiles:
            a, b = ti * stride - nr0, tj * stride - nc0
            zt = (z0 * (1 - zm) + var * zm)[:, a // LC:(a + tile) // LC, b // LC:(b + tile) // LC]
            with torch.autocast("cuda", dtype=torch.bfloat16):
                out = decoder_tile(pipe, zt, ti, tj, tile, stride)
            w = win * regm[a:a + tile, b:b + tile]
            err = ((out.float() - tgt[a:a + tile, b:b + tile]) ** 2 * w).sum()
            ws = float(w.sum())
            if it < iters:
                (err / max(ws, 1.0) / len(tiles)).backward()
            tot += float(err.detach())
            wsum += ws
        mse = tot / max(wsum, 1.0)
        if it < iters:
            pr = prior * (var * zm).pow(2).sum() / zm.sum() / 4
            pr.backward()
            opt.step()
        if it % 10 == 0 or it == iters:
            zz = (z.detach() * zm)[:, zm[0] > 0]
            hist.append({"it": it, "mse_norm": mse, "rmse_sqrt_m": math.sqrt(mse) * pipe.kwargs["residual_std"],
                         "z_std": float(zz.std()), "s": time.perf_counter() - t0})
            log(json.dumps(hist[-1]))
    torch.backends.cudnn.benchmark = False
    z = (z0 * (1 - zm) + var * zm).detach()
    stats = {"tiles": len(tiles), "iters": iters, "lr": lr, "prior": prior, "z0_std_in_region":
             float(z0[:, zm[0] > 0].std()), "history": hist, "invert_s": time.perf_counter() - t0}
    return z.cpu(), (lr0, lc0), zm[0].cpu(), stats


def known_arrays(g, z, zorigin, zmask, weight_lat):
    """Place z (latent window) into the guidance latent frame; known mask = zmask x weight_lat (both 0..1)."""
    lr, lcn = g.lat_norm.shape
    oi, oj = g.lat_origin
    K = torch.zeros((4, lr, lcn))
    M = torch.zeros((lr, lcn))
    a, b = zorigin[0] - oi, zorigin[1] - oj
    h, w = zmask.shape
    K[:, a:a + h, b:b + w] = z
    M[a:a + h, b:b + w] = zmask
    return K, M * weight_lat


def lat_from_native(m_nat, g, ho, wo, ramp_px=2):
    """30 m mask over the input grid -> latent res in the guidance frame (block mean, then ramp-dilated)."""
    lr, lcn = g.lat_norm.shape
    oi, oj = g.lat_origin
    full = np.zeros((lr * LC, lcn * LC), np.float32)
    full[-oi * LC:-oi * LC + ho, -oj * LC:-oj * LC + wo] = m_nat
    lm = full.reshape(lr, LC, lcn, LC).mean(axis=(1, 3))
    core = lm > 0.25
    d = ndimage.distance_transform_edt(~core)
    return torch.from_numpy(np.clip(1.0 - (d - 1.0) / ramp_px, 0, 1).astype(np.float32))


# ---------------------------------------------------------------------------------------------------------------
ARMS = {
    # name: (known region, sigma, known_final, help)
    "B1": (None, 0.35, False, "TD-1 inject (lowfreq ch4 only) on the landmark target"),
    "I0": ("all", 0.35, True, "decoder(z_inverted): the inversion's fidelity (base model only outside the crop)"),
    "I1": ("lm", 0.35, True, "landmarks keep inverted latents (hard), the model fills the rest"),
    "I2": ("lm", 0.35, False, "landmarks: inverted latents re-imagined at sigma 0.35 (SDEdit), rest free"),
    "I3": ("all", 0.35, False, "whole crop: inverted latents re-imagined at sigma 0.35"),
    "I4": ("lm", 0.7, False, "landmarks: SDEdit at sigma 0.7, rest free"),
    "I5": ("all", 0.7, False, "whole crop: SDEdit at sigma 0.7"),
    "I6": ("all", 1.2, False, "whole crop: SDEdit at sigma 1.2"),
    "I7": ("lm", 1.2, False, "landmarks: SDEdit at sigma 1.2, rest free"),
    "E1": ("all", 0.35, False, "I3 + exemplar stamp: inverted Teide summit latents (ch 0-3) at Red Mountain"),
    "E2": ("all", 0.7, False, "I5 + exemplar stamp: inverted Teide summit latents at Red Mountain"),
    "E3": ("all", 0.35, True, "I0 + exemplar stamp (hard): what the stamp alone decodes to"),
}


def invert_exemplar(pipe, name, half_px, args, log):
    """Invert a real 30 m DEM guide (summit at the grid centre) -> ch 0-3 latents of a square around the centre."""
    h, m = read_guide(ROOT / f"target/worldgen/real/{name}")
    gx = Guidance(h, 0.0)
    setup(pipe, gx, [0.35])
    cr, cc = h.shape[0] // 2, h.shape[1] // 2
    cr, cc = cr // LC * LC, cc // LC * LC
    z, zo, zm, st = invert(pipe, gx, (cr - half_px, cc - half_px, cr + half_px, cc + half_px), args.iters, args.lr,
                           args.prior, log)
    a, b = (cr - half_px) // LC - zo[0], (cc - half_px) // LC - zo[1]
    n = 2 * half_px // LC
    return z[:, a:a + n, b:b + n].clone(), st


def stamp(K, M, g, zpatch, at_xy, v3o, r_km):
    """Write zpatch (centred) into K at 10x-frame position at_xy with a feathered disc of radius r_km."""
    oi, oj = g.lat_origin
    r = (v3o[1] - at_xy[1]) / SP / LC - oi
    c = (at_xy[0] - v3o[0]) / SP / LC - oj
    n = zpatch.shape[-1]
    r0, c0 = int(round(r - n / 2)), int(round(c - n / 2))
    yy, xx = np.mgrid[0:n, 0:n] - (n - 1) / 2
    d = np.hypot(yy, xx) * LC * SP / 1000
    w = torch.from_numpy(np.clip((r_km - d) / 1.0, 0, 1).astype(np.float32))
    K = K.clone()
    K[:, r0:r0 + n, c0:c0 + n] = (1 - w) * K[:, r0:r0 + n, c0:c0 + n] + w * zpatch
    return K


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--arms", default="B1,I0,I1,I2,I3")
    ap.add_argument("--out", default=str(ROOT / "target/worldgen/td2"))
    ap.add_argument("--seed", type=int, default=1234)
    ap.add_argument("--iters", type=int, default=100)
    ap.add_argument("--lr", type=float, default=0.05)
    ap.add_argument("--prior", type=float, default=1e-3)
    ap.add_argument("--margin", type=int, default=64, help="inversion region = crop + margin native px")
    ap.add_argument("--histogram", default=None, help="histogram_raw (beauty) JSON list, e.g. [0,0,0,1,1.5]")
    ap.add_argument("--suffix", default="")
    ap.add_argument("--stamp", default="teide-30m", help="exemplar guide under target/worldgen/real (E arms)")
    ap.add_argument("--stamp-at", default="3371,9760", help="10x-frame x,y of the stamp centre (V3 summit)")
    ap.add_argument("--stamp-r-km", type=float, default=5.0)
    a = ap.parse_args()
    out = Path(os.path.abspath(a.out))
    out.mkdir(parents=True, exist_ok=True)
    os.chdir(REPO)
    logf = open(out / f"log{a.suffix}.txt", "a", encoding="utf-8")

    def log(s):
        print(s, flush=True)
        logf.write(s + "\n")
        logf.flush()

    t = time.perf_counter()
    target, v30, raw10, paste, masks, v3o = build_target()
    co = crop_origin(v3o)
    log(f"target built {time.perf_counter() - t:.1f}s shape {target.shape} crop origin {co}")
    work = out / "_work"
    work.mkdir(exist_ok=True)
    np.save(work / "paste_crop.npy", crop(paste))
    meta_lm = {k: {kk: vv for kk, vv in v.items() if kk not in ("core", "mask")} for k, v in masks.items()}
    (work / "landmarks.json").write_text(json.dumps(meta_lm, indent=1), encoding="utf-8")
    if not (out / "R0-target").exists():
        write_guide(out / "R0-target", crop(target), SP, co, extra={"td2": "landmark target: V3 + raw x10 <4 km band in masks"})
        write_guide(out / "R-V3", crop(v30), SP, co, extra={"td2": "V3 resampled to 30 m, crop"})
        write_guide(out / "R-raw10", crop(raw10), SP, co, extra={"td2": "raw source x10 on V3's 30 m grid, crop"})
    hist = json.loads(a.histogram) if a.histogram else None

    g = Guidance(target, 0.0)
    pipe = make_pipe(a.seed)
    arms = [s.strip() for s in a.arms.split(",") if s.strip()]
    # B1-style latents are both an arm and the inversion's init
    setup(pipe, g, [0.35], hist)
    zfile = work / f"z_inv{a.suffix}.pt"
    inv_stats = None
    need_z = any(ARMS[x][0] for x in arms)
    if need_z and zfile.exists():
        d = torch.load(zfile)
        z, zorigin, zmask, inv_stats = d["z"], tuple(d["origin"]), d["mask"], d["stats"]
        log(f"loaded {zfile}")
    elif need_z:
        r0, c0, n = CROP
        m = a.margin
        torch.cuda.reset_peak_memory_stats()
        z, zorigin, zmask, inv_stats = invert(pipe, g, (r0 - m, c0 - m, r0 + n + m, c0 + n + m), a.iters, a.lr,
                                             a.prior, log)
        inv_stats["peak_vram_mb"] = torch.cuda.max_memory_allocated() / 2 ** 20
        torch.save({"z": z, "origin": list(zorigin), "mask": zmask, "stats": inv_stats}, zfile)
    lm_lat = lat_from_native(paste, g, *target.shape)
    zstamp = None
    if any(x.startswith("E") for x in arms):
        sfile = work / f"z_{a.stamp}.pt"
        if sfile.exists():
            zstamp = torch.load(sfile)["z"]
        else:
            half = int(a.stamp_r_km * 1000 / SP) // LC * LC + 64
            zstamp, st = invert_exemplar(pipe, a.stamp, half, a, log)
            torch.save({"z": zstamp, "stats": st}, sfile)
    for arm in arms:
        region, sigma, kfinal, helptext = ARMS[arm]
        name = f"{arm}{a.suffix}"
        pipe.known_z = pipe.known_m = None
        pipe.known_final = kfinal
        if region:
            weight = lm_lat if region == "lm" else torch.ones_like(lm_lat)
            pipe.known_z, pipe.known_m = known_arrays(g, z, zorigin, zmask, weight)
            if arm.startswith("E"):
                xy = [float(v) for v in a.stamp_at.split(",")]
                pipe.known_z = stamp(pipe.known_z, pipe.known_m, g, zstamp, xy, v3o, a.stamp_r_km)
        setup(pipe, g, [sigma], hist)
        torch.cuda.reset_peak_memory_stats()
        elev, times = decode_crop(pipe)
        times["peak_vram_mb"] = torch.cuda.max_memory_allocated() / 2 ** 20
        prov = {"model": MODEL, "repo_commit": "e8dcb4b", "seed": a.seed, "arm": arm, "help": helptext,
                "known": region, "sigma": sigma, "known_final": kfinal, "histogram_raw": hist,
                "timings": times, "crop_native_px": list(CROP), "climate_const": CLIMATE, "cond_snr": DOC_SNR}
        if region:
            prov["inversion"] = {k: v for k, v in inv_stats.items() if k != "history"}
            prov["inversion"]["final"] = inv_stats["history"][-1]
        write_guide(out / name, elev, SP, co, extra={"td2": prov})
        log(f"{name}: {json.dumps(times)} min {elev.min():.0f} max {elev.max():.0f}")
    logf.close()


if __name__ == "__main__":
    main()
