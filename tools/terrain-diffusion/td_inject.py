"""Lowfreq injection into terrain-diffusion's WorldPipeline (xandergos/terrain-diffusion, MIT, master e8dcb4b).

The stock pipeline (terrain_diffusion/inference/world_pipeline.py):
  coarse stage  1 cell = 256 native px (7.68 km at 30 m); channels [elev, p5, temp, temp_std, precip, precip_cv],
                elev and p5 in signed-sqrt metres.
  latent stage  1 px = 8 native px (240 m); 5 channels: 0-3 = VAE latent of the 30 m residual, 4 = "lowfreq"
                (signed-sqrt elevation, Gaussian sigma 5 lowres px, normalised by LOWFREQ_MEAN/STD -31.4/38.6).
                A 2-step consistency sampler (t_init, then sigma 0.35), conditioned on a 4x4 coarse window.
  decoder       30 m residual from latent channels 0-3 only (nearest-upsampled); elevation =
                laplacian_decode(residual, lowfreq) squared back to metres.

What this subclass changes (nothing in the clone is edited):
  * `_coarse_inference`: after the stock tile, coarse channels 0 (mean) and 1 (p5) are replaced inside the guided
    region by values computed from our heights exactly as the training data computed them (per-cell mean and
    5 % quantile of the 240 m median-reduced sqrt elevation, h5_latents_dataset.py ~196-199).
  * `_build_latent_stage`: same T=2 chain as stock (copied, MIT) but the intermediate sigmas are configurable
    (`inject_sigmas`; stock = [0.35]).
  * Injection (RePaint-style): before every intermediate step the clean estimate's channel 4 is replaced by our
    lowfreq (blended by a region mask), so the model renoises it at that step's level and re-generates channels
    0-3 to agree with it. With `final_replace` the last step's channel 4 is replaced as well, so the decoded
    elevation's >~1 km part is ours and the model adds only the residual (channels 0-3 -> decoder).
"""
from __future__ import annotations

import numpy as np
import torch
import skimage.measure
from infinite_tensor import InfiniteTensor, TensorWindow

from terrain_diffusion.inference.world_pipeline import WorldPipeline, linear_weight_window
from terrain_diffusion.data.laplacian_encoder import laplacian_encode
from terrain_diffusion.scheduler.dpmsolver import EDMDPMSolverMultistepScheduler

from guide_io import bathymetry

LOWFREQ_MEAN = -31.4  # world_pipeline.py _compute_elev
LOWFREQ_STD = 38.6
LOWRES_SIGMA = 5.0  # build_base_dataset.py --lowres-sigma default
CELL = 256  # native px per coarse cell
LC = 8  # native px per latent px


def ssqrt(x):
    return np.sign(x) * np.sqrt(np.abs(x))


class Guidance:
    """Our heights in the pipeline's three spaces, over a padded region around the input footprint.

    Input sample (r, c) is native pixel (r, c). The region is the footprint rounded up to whole coarse cells plus
    `ring_cells` of open sea on every side (bathymetry continued outward), so tile blends and the decoder's
    padding never see the edge of our data.
    """

    def __init__(self, h30: np.ndarray, sea_level_m: float, *, ring_cells: int = 2, ramp_lat: int = 16,
                 shelf_m: float = 30.0, slope: float = 0.1, floor_m: float = 1000.0):
        ho, wo = h30.shape
        ring = ring_cells * CELL
        hp = -(-ho // CELL) * CELL
        wp = -(-wo // CELL) * CELL
        region = np.full((hp + 2 * ring, wp + 2 * ring), -1.0, dtype=np.float32)  # metres relative to sea level
        region[ring:ring + ho, ring:ring + wo] = h30 - sea_level_m
        region = bathymetry(region, 30.0, 0.0, slope=slope, floor_m=floor_m, shelf_m=shelf_m)
        self.region_m = region
        s = ssqrt(region).astype(np.float32)

        lr, lcn = s.shape[0] // LC, s.shape[1] // LC
        lowres_exact = skimage.measure.block_reduce(s, (LC, LC), np.median).astype(np.float32)
        residual, lowfreq = laplacian_encode(torch.from_numpy(s), (lr, lcn), LOWRES_SIGMA)
        self.lowfreq = lowfreq.float()  # sqrt-metres, (lr, lcn)
        self.residual = residual.float()  # sqrt-metres, native px, same region
        self.lat_norm = ((self.lowfreq - LOWFREQ_MEAN) / LOWFREQ_STD).float()
        ii = torch.arange(lr, dtype=torch.float32)
        jj = torch.arange(lcn, dtype=torch.float32)
        ri = torch.clamp(torch.minimum(ii + 1, lr - ii) / ramp_lat, 0, 1)
        rj = torch.clamp(torch.minimum(jj + 1, lcn - jj) / ramp_lat, 0, 1)
        self.lat_mask = (ri[:, None] * rj[None, :]).float()
        self.lat_origin = (-ring // LC, -ring // LC)
        self.nat_mask = self.lat_mask.repeat_interleave(LC, 0).repeat_interleave(LC, 1)
        self.nat_origin = (-ring, -ring)

        cr, cc = lr // 32, lcn // 32
        blocks = lowres_exact.reshape(cr, 32, cc, 32)
        self.coarse_mean = torch.from_numpy(blocks.mean(axis=(1, 3)).astype(np.float32))
        self.coarse_p5 = torch.from_numpy(np.quantile(blocks, 0.05, axis=(1, 3)).astype(np.float32))
        self.coarse_origin = (-ring_cells, -ring_cells)

    def coarse_metres(self) -> np.ndarray:
        """Per coarse cell elevation in metres (squared mean sqrt), for the documented conditioning import."""
        m = self.coarse_mean.numpy()
        return (np.sign(m) * m * m).astype(np.float32)

    def nat_patch(self, i0: int, j0: int, size: int):
        """(residual sqrt-metres, mask) for native rows i0..i0+size (mask 0 outside the region)."""
        vals = torch.zeros((size, size))
        mask = torch.zeros((size, size))
        oi, oj = self.nat_origin
        hr, wr = self.residual.shape
        a0, a1 = max(i0, oi), min(i0 + size, oi + hr)
        b0, b1 = max(j0, oj), min(j0 + size, oj + wr)
        if a0 < a1 and b0 < b1:
            vals[a0 - i0:a1 - i0, b0 - j0:b1 - j0] = self.residual[a0 - oi:a1 - oi, b0 - oj:b1 - oj]
            mask[a0 - i0:a1 - i0, b0 - j0:b1 - j0] = self.nat_mask[a0 - oi:a1 - oi, b0 - oj:b1 - oj]
        return vals, mask

    def lat_patch(self, i0: int, j0: int, size: int):
        """(values, mask) for latent rows i0..i0+size, cols j0..j0+size (mask 0 outside the region)."""
        vals = torch.zeros((size, size))
        mask = torch.zeros((size, size))
        oi, oj = self.lat_origin
        lr, lcn = self.lat_norm.shape
        a0, a1 = max(i0, oi), min(i0 + size, oi + lr)
        b0, b1 = max(j0, oj), min(j0 + size, oj + lcn)
        if a0 < a1 and b0 < b1:
            vals[a0 - i0:a1 - i0, b0 - j0:b1 - j0] = self.lat_norm[a0 - oi:a1 - oi, b0 - oj:b1 - oj]
            mask[a0 - i0:a1 - i0, b0 - j0:b1 - j0] = self.lat_mask[a0 - oi:a1 - oi, b0 - oj:b1 - oj]
        return vals, mask


class InjectedWorldPipeline(WorldPipeline):
    """WorldPipeline with optional coarse override and latent lowfreq injection (see module docs)."""

    guidance: Guidance | None = None
    inject_sigmas: list[float] = [0.35]
    decoder_sigma: list[float] | None = None  # SDEdit our 30 m residual: sigmas of successive steps (None = stock)
    final_replace: bool = True
    coarse_override: bool = True

    # ---- coarse --------------------------------------------------------------------------------------------
    def _coarse_inference(self, ctx, scheduler, weight_window, t_cond, cond_inputs, pool_size=1):
        out = super()._coarse_inference(ctx, scheduler, weight_window, t_cond, cond_inputs, pool_size)
        g = self.guidance
        if g is None or not self.coarse_override:
            return out
        assert pool_size == 1
        _, i, j = ctx
        i1, j1 = i * 48, j * 48  # TILE_STRIDE = 64 - 16
        size = out.shape[-1]
        oi, oj = g.coarse_origin
        cr, cc = g.coarse_mean.shape
        a0, a1 = max(i1, oi), min(i1 + size, oi + cr)
        b0, b1 = max(j1, oj), min(j1 + size, oj + cc)
        if a0 < a1 and b0 < b1:
            out = out.clone()
            w = out[-1, a0 - i1:a1 - i1, b0 - j1:b1 - j1]
            out[0, a0 - i1:a1 - i1, b0 - j1:b1 - j1] = g.coarse_mean[a0 - oi:a1 - oi, b0 - oj:b1 - oj] * w
            out[1, a0 - i1:a1 - i1, b0 - j1:b1 - j1] = g.coarse_p5[a0 - oi:a1 - oi, b0 - oj:b1 - oj] * w
        return out

    # ---- latent --------------------------------------------------------------------------------------------
    def _inject(self, ctx, tile: torch.Tensor) -> torch.Tensor:
        """tile = (6, 64, 64): 5 weighted channels + weight. Replace channel 4 by ours where the mask says."""
        vals, mask = self.guidance.lat_patch(ctx[1] * 32, ctx[2] * 32, tile.shape[-1])
        if not mask.any():
            return tile
        tile = tile.clone()
        w = tile[-1]
        tile[4] = (1 - mask) * tile[4] + mask * vals * w
        return tile

    def _guided_latent(self, ctxs, samples, conds, t, scheduler, weight_window, hist, means, stds, seed_offset,
                       final):
        if self.guidance is not None and samples is not None:
            samples = [self._inject(c, s) for c, s in zip(ctxs, samples)]
        outputs = self._latent_inference(ctxs, samples, conds, t, scheduler, weight_window, hist, means, stds,
                                         seed_offset=seed_offset)
        if self.guidance is not None and final and self.final_replace:
            outputs = [self._inject(c, o) for c, o in zip(ctxs, outputs)]
        return outputs

    def _build_latent_stage(self):
        # Copy of WorldPipeline._build_latent_stage's T=2 branch (MIT), with configurable intermediate sigmas.
        TILE_SIZE = 64
        TILE_STRIDE = TILE_SIZE // 2
        COND_INPUT_MEAN = torch.tensor([14.99, 11.65, 15.87, 619.26, 833.12, 69.40, 0.66])
        COND_INPUT_STD = torch.tensor([21.72, 21.78, 10.40, 452.29, 738.09, 34.59, 0.47])
        HISTOGRAM_RAW = torch.tensor([self.kwargs['histogram_raw']])
        scheduler = EDMDPMSolverMultistepScheduler(sigma_min=0.002, sigma_max=80, sigma_data=0.5)
        weight_window = linear_weight_window(TILE_SIZE, 'cpu', torch.float32)
        t_inter = [torch.arctan(torch.tensor(float(s)) / 0.5) for s in self.inject_sigmas]
        t_init = torch.atan(scheduler.sigmas[0] / scheduler.config.sigma_data)
        output_window = TensorWindow(size=(6, TILE_SIZE, TILE_SIZE), stride=(6, TILE_STRIDE, TILE_STRIDE))
        coarse_window = TensorWindow(size=(7, 4, 4), stride=(7, 1, 1), offset=(0, -1, -1))
        n = len(t_inter)
        tensor = InfiniteTensor(
            shape=(6, None, None),
            f=lambda ctxs, conds: self._guided_latent(
                ctxs, None, conds, t_init, scheduler, weight_window, HISTOGRAM_RAW, COND_INPUT_MEAN,
                COND_INPUT_STD, 5819, final=(n == 0)),
            output_window=output_window,
            args=(self.coarse,),
            args_windows=(coarse_window,),
            batch_size=self.latents_batch_size,
            tile_store=self.tile_store,
            tensor_id="init_latent_map",
        )
        for i, t in enumerate(t_inter):
            tensor = InfiniteTensor(
                shape=(6, None, None),
                f=lambda ctxs, samples, conds, t=t, i=i: self._guided_latent(
                    ctxs, samples, conds, t, scheduler, weight_window, HISTOGRAM_RAW, COND_INPUT_MEAN,
                    COND_INPUT_STD, 5820 + i, final=(i == n - 1)),
                output_window=output_window,
                args=(tensor, self.coarse),
                args_windows=(output_window, coarse_window),
                batch_size=self.latents_batch_size,
                tile_store=self.tile_store,
                tensor_id=f"step_latent_map_{i}",
            )
        return tensor

    # ---- decoder -------------------------------------------------------------------------------------------
    def _decoder_inference(self, ctx, latents, scheduler, weight_window, t_list, tile_size, tile_stride):
        """Stock decoder, or SDEdit of our residual: noise it to decoder_sigma[0], denoise; later sigmas renoise the estimate.

        Body follows WorldPipeline._decoder_inference (MIT); only the initial sample and t_list differ.
        """
        g = self.guidance
        if g is None or self.decoder_sigma is None:
            return super()._decoder_inference(ctx, latents, scheduler, weight_window, t_list, tile_size, tile_stride)
        from terrain_diffusion.inference.world_pipeline import gaussian_noise_patch
        model_dtype = self._dtype or torch.float32
        sd = scheduler.config.sigma_data
        vals, mask = g.nat_patch(ctx[1] * tile_stride, ctx[2] * tile_stride, tile_size)
        norm = (vals - self.kwargs['residual_mean']) / self.kwargs['residual_std'] * sd
        sample = (mask * norm)[None, None].to(self.device, dtype=model_dtype)
        lc = self.latent_compression
        lat = (latents[:-1] / latents[-1:])[:4].view(1, 4, tile_size // lc, tile_size // lc)
        up = torch.nn.functional.interpolate(lat, size=(tile_size, tile_size), mode='nearest').to(
            self.device, dtype=model_dtype)
        t_dec = [torch.atan(torch.tensor(float(x)) / sd) for x in self.decoder_sigma]
        for i, t in enumerate(t_dec):
            noise = torch.as_tensor(gaussian_noise_patch(
                self.seed + 5819 + i, ctx[1] * tile_stride, ctx[2] * tile_stride,
                tile_size, tile_size, channels=1, tile_h=tile_size, tile_w=tile_size))[None]
            t = t.view(1, 1, 1, 1).to(self.device, dtype=model_dtype)
            z = noise.to(self.device, dtype=model_dtype) * sd
            x_t = torch.cos(t) * sample + torch.sin(t) * z
            model_in = torch.cat([(x_t / sd), up], dim=1)
            pred = -self.decoder_model(model_in, noise_labels=torch.tensor([t.item()], device=self.device,
                                                                           dtype=model_dtype), conditional_inputs=[])
            sample = torch.cos(t) * x_t - torch.sin(t) * sd * pred
        sample = sample.cpu().float() / sd
        return torch.cat([sample[0] * weight_window[None], weight_window[None]], dim=0)
