"""TD-3 check: does xandergos/terrain-diffusion-autoencoder's encoder produce latents the 30 m decoder understands?

encode(residual) -> z (posterior mean) -> stock 30 m decoder (tiled, as the pipeline) -> compare with the residual.
Arms: residual normalised by 0.7 (30 m pipeline config) and 1.1678 (build_encoded_dataset default).
Baselines: the autoencoder's own decoder; TD-2's cached decoder inversion of Teide (z_teide-30m.pt, MSE 0.08).
"""
import math, sys, time, json
from pathlib import Path
import numpy as np
import torch

ROOT = Path(r"D:/Gamedev/Bevyrrowind")
sys.path.insert(0, str(ROOT / "tools/terrain-diffusion"))
sys.path.insert(0, r"C:/tools/terrain-diffusion/repo")
from guide_io import read_guide  # noqa: E402
from td_inject import Guidance, LC  # noqa: E402
from td_steer import make_pipe, decoder_tile  # noqa: E402
from terrain_diffusion.models.edm_autoencoder import EDMAutoencoder  # noqa: E402
from terrain_diffusion.inference.world_pipeline import linear_weight_window  # noqa: E402

torch.backends.cudnn.benchmark = False
pipe = make_pipe(seed=1)
TILE, STRIDE = pipe.decoder_tile_size, pipe.decoder_tile_stride
RSTD30 = pipe.kwargs["residual_std"]
ae = EDMAutoencoder.from_pretrained("xandergos/terrain-diffusion-autoencoder").to("cuda").eval()
print("residual_std (30m pipeline):", RSTD30, "| ae params M:", sum(p.numel() for p in ae.parameters()) / 1e6)


@torch.no_grad()
def decode30(z):
    """z (4, h, w) latent -> normalised residual (8h, 8w), tiled + blended like the pipeline."""
    H, W = z.shape[1] * LC, z.shape[2] * LC
    out = torch.zeros((H, W), device="cuda"); wsum = torch.zeros_like(out)
    win = linear_weight_window(TILE, "cuda", torch.float32)
    for a in range(0, H - TILE + 1, STRIDE):
        for b in range(0, W - TILE + 1, STRIDE):
            o = decoder_tile(pipe, z[:, a // LC:(a + TILE) // LC, b // LC:(b + TILE) // LC], a // STRIDE, b // STRIDE, TILE, STRIDE)
            out[a:a + TILE, b:b + TILE] += o.float() * win; wsum[a:a + TILE, b:b + TILE] += win
    return out / wsum.clamp_min(1e-6)


def stats(pred, tgt, m):
    p, t = pred[m:-m, m:-m], tgt[m:-m, m:-m]
    mse = float(((p - t) ** 2).mean())
    pc, tc = p - p.mean(), t - t.mean()
    corr = float((pc * tc).sum() / (pc.norm() * tc.norm()))
    return {"mse_norm07": round(mse, 4), "corr": round(corr, 4), "rmse_sqrtm": round(math.sqrt(mse) * RSTD30, 3)}


def case(name, guide, size):
    h, _ = read_guide(ROOT / guide)
    g = Guidance(h, 0.0)
    r = g.residual
    hr, wr = r.shape
    n = min(size, (min(hr, wr) - 64) // TILE * TILE)
    n = (n - TILE) // STRIDE * STRIDE + TILE
    i0, j0 = (hr - n) // 2 // LC * LC, (wr - n) // 2 // LC * LC
    res = r[i0:i0 + n, j0:j0 + n].float().to("cuda")  # sqrt-metres
    tgt = res / RSTD30
    m = 64  # ignore the outer 2 km (tile edges)
    rep = {"case": name, "px": n, "km": n * 30 / 1000, "target_std_norm07": round(float(tgt.std()), 3)}
    for label, rstd in (("enc/0.7", RSTD30), ("enc/1.1678", 1.1678)):
        t0 = time.perf_counter()
        with torch.no_grad():
            mu, lv = ae.preencode(((res - 0.0) / rstd)[None, None], conditional_inputs=[])
        torch.cuda.synchronize(); te = time.perf_counter() - t0
        z = mu[0, :4].float()
        t0 = time.perf_counter(); pred = decode30(z); torch.cuda.synchronize(); td = time.perf_counter() - t0
        # the decoder's output is in 0.7-normalised units whatever the encoder's input scale; rescale to compare
        s = stats(pred, tgt, m)
        with torch.no_grad():
            own = ae.decode(mu[:, :4])[0, 0].float() * rstd / RSTD30
        rep[label] = {**s, "z_mean": round(float(z.mean()), 3), "z_std": [round(float(x), 3) for x in z.std(dim=(1, 2))],
                      "post_std_mean": round(float((lv[0, :4] * 0.5).exp().mean()), 3),
                      "encode_s": round(te, 3), "decode30_s": round(td, 2), "ae_own_decoder": stats(own, tgt, m)}
    return rep, g, (i0, j0, n)


out = []
rep, g, (i0, j0, n) = case("Teide (real GLO-30)", "target/worldgen/real/teide-30m", 1536)
# TD-2's inverted Teide latents: square centred on the grid centre (invert_exemplar)
zt = torch.load(ROOT / "target/worldgen/td2/_work/z_teide-30m.pt", map_location="cuda")["z"].float()
hr, wr = g.residual.shape
cr, cc = hr // 2 // LC * LC, wr // 2 // LC * LC
k = zt.shape[1]; half = k * LC // 2
C = 1024 // 2  # context window (native px) around the centre so the 448 px region sits inside full decoder tiles
ctx = (g.residual[cr - C:cr + C, cc - C:cc + C].float().to("cuda") / RSTD30)
with torch.no_grad():
    mu, _ = ae.preencode(ctx[None, None], conditional_inputs=[])
ze = mu[0, :4].float()
zi = ze.clone(); o = (C - half) // LC
zi[:, o:o + k, o:o + k] = zt
pe, pin = decode30(ze), decode30(zi)
sl = slice(C - half + 16, C + half - 16)
def st2(p):
    return stats(p[sl, sl], ctx[sl, sl], 1) if False else stats(torch.nn.functional.pad(p[sl, sl], (1, 1, 1, 1)), torch.nn.functional.pad(ctx[sl, sl], (1, 1, 1, 1)), 1)
rep["inversion_region"] = {"km": round((2 * half - 32) * 30 / 1000, 2), "inverted_z_decoded": st2(pin), "encoded_z_decoded": st2(pe),
                           "latent_corr_enc_vs_inv": [round(float(np.corrcoef(ze[c, o:o + k, o:o + k].cpu().numpy().ravel(), zt[c].cpu().numpy().ravel())[0, 1]), 3) for c in range(4)],
                           "z_std_inv": [round(float(x), 3) for x in zt.std(dim=(1, 2))],
                           "z_std_enc_here": [round(float(x), 3) for x in ze[:, o:o + k, o:o + k].std(dim=(1, 2))]}
out.append(rep)
for name, guide in (("Etna (real GLO-30)", "target/worldgen/real/etna-30m"), ("TD-2 R0 target (ours, out of distribution)", "target/worldgen/td2/R0-target")):
    try:
        out.append(case(name, guide, 1536)[0])
    except Exception as e:  # noqa: BLE001
        out.append({"case": name, "error": repr(e)})
print(json.dumps(out, indent=1))
