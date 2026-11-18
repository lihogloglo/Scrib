# Steering terrain-diffusion for landmark-true Vvardenfell (TD-2, 2026-11-18)

Question: how to get the most out of terrain-diffusion (xandergos/terrain-diffusion, model
`xandergos/terrain-diffusion-30m`, MIT code + weights; paper arXiv 2512.08309 "InfiniteDiffusion") for a 10x
Vvardenfell whose hero landmarks keep their forms at their places, and whether a better model or method exists.
Follows TD-1 (`target/worldgen/td/README.md`). Numbers only; the owner judges looks in the viewer.
Prototype arms: `target/worldgen/td2/README.md`; scripts: `tools/terrain-diffusion/README.md` ("TD-2").

## 1. What the model is (facts, cited)

> **Correction (TD-3, 2026-11-18):** the encoder *is* published: `xandergos/terrain-diffusion-autoencoder` (MIT, HF,
> 2026-08-21). Its latents (residual / 0.7, posterior mean) decode through the stock 30 m decoder at corr 0.99 on Teide
> and Etna, in under a second. Methods 1 and 4 below should use it instead of decoder inversion (ledger TD-3).

| Stage | Grid | What it makes | Size | Steps | Source |
|---|---|---|---|---|---|
| coarse | 7.68 km cells | elev mean, p5 (signed sqrt), 4 climate channels | 128 ch, 11 MB | 1 | `coarse_model/config.json`, `world_pipeline.py` 1007-1095 |
| base (latent) | 240 m | ch 0-3 = VAE latent of the 30 m residual; ch 4 = lowfreq (blurred sqrt elevation) | EDM2 UNet 192 ch, attn at 8/16, 1.01 GB fp32 (~250 M params) | 2 (consistency) | `base_model/config.json`, `world_pipeline.py` 1150-1240 |
| decoder | 30 m | residual from ch 0-3 only (nearest-upsampled concat), 512 px tiles, stride 384 | UNet 64 ch, 112 MB (~28 M) | 1 (from sigma 80) | `decoder_model/config.json`, `world_pipeline.py` 1307-1373 |

- Laplacian split: lowfreq = bilinear downsample x8 then Gaussian blur sigma 5 latent px (kernel 11, truncated at
  +-1.2 km); residual = sqrt-elevation minus the upsampled lowfreq (`laplacian_encoder.py` `laplacian_encode`,
  paper §5.2). So **everything between 30 m and ~6 km — craters, foyadas, gorges, summit cones — lives in latent
  channels 0-3**, which the weights give no way to write: the autoencoder was trained (TRAINING.md "Train
  AutoEncoder", 18 GB GPU) and its *encoder* used offline to build the latent dataset
  (`build_encoded_dataset.py` 69-92); paper §5.3 says the VAE decoder was discarded; the HF snapshot holds only
  base, coarse and decoder.
- The decoder is conditioned on raw sampled VAE latents (`h5_decoder_terrain_dataset.py` 216-230) and the base model
  was trained on the same latents with mean 0 / std 1 (`configs/diffusion_base/30m/diffusion_192-3.cfg` 81-82), so
  base ch 0-3 feed the decoder unchanged.
- Base conditioning vector: 4x4 coarse means + p5 + climate + mask + `histogram_raw` ("beauty", 5 logits) + noise
  level (`world_pipeline.py` 1116-1148). The pipeline default is all-zero beauty (`world_pipeline.py` 357); the CLI
  default is `[0,0,0,1,1.5]` (API_README.md 21). TD-1 used zeros.
- Training data of the 30 m model: Copernicus GLO-30 2048 px tiles (`util_scripts/download_dem_30m.sh`); paper:
  MERIT 90 m + ETOPO + WorldClim (§4). Full pipeline training ~2 weeks on a 3090 Ti (§10).
- Paper's user control = coarse maps (hand drawn or procedural, noised per channel by a user SNR, §5.4) and climate.
  No inpainting, mask or sketch conditioning (§5.3-5.4, §8 lists more conditioning as future work).

## 2. Other models (verified on their repos 2026-11-18, research sub-agent)

| Model | Licence | Weights | Control | Verdict |
|---|---|---|---|---|
| Earthbender (MIG'25), github.com/danial-barazandeh/Earthbender | Apache-2.0 | none (400-pair dataset only) | 3-colour sketch (mountain/river/lake) via SD1.5 ControlNet, 512 px | recipe only; retrain needed |
| Lochner et al. PG'23, github.com/Arches-Team/Interactive-Authoring-of-Terrain-using-Diffusion-Models | MIT code; weights on Drive, licence unstated | yes, 256 px | sketch, infill | tiles too small (7.7 km at 30 m), weight licence unclear |
| TDN (AAAI'24), github.com/TDNResearch/TDN | no licence file | not released | river/ridge/basin/peak sketch | unusable |
| TerraFusion (2025) | no code | no | sketch adapter | unusable |
| MESA (CVPRW'25) | Adobe Research, non-commercial | yes | text only | unusable |

No open model beats terrain-diffusion for our scale (30 m, unbounded, MIT weights). Generic steering code:
RePaint is CC BY-NC-SA and DPS/FreeDoM/Universal Guidance/PnP-CM have no licence file, so we reimplement from the
papers (ideas are free); DiffPIR (MIT), ControlNet (Apache-2.0) and openai/consistency_models (MIT) are usable.
Consistency-model-specific steering in the literature: CoSIGN (arXiv 2407.12676: ControlNet soft constraint + an
optimisation step for hard ones), PnP-CM (arXiv 2509.22736: alternate CM denoise and a data-fidelity step).

## 3. Methods surveyed and ranked (written before prototyping)

Inference: with a 2-step consistency sampler there is one place to steer the base model (between its two steps)
and the decoder runs once from pure noise. Every steering method reduces to "choose the clean latent the second
step starts from, and how much noise to put back". The missing piece for everything below 6 km is an encoder.

| Rank | Method | Expected quality / control | Cost to build | Speed | Notes |
|---|---|---|---|---|---|
| 1 | **Decoder inversion + latent RePaint/SDEdit** (optimise ch 0-3 so the shipped decoder reproduces a target residual; keep them in landmark masks, re-imagine at sigma s, free elsewhere) | landmarks at place, form strength by s; whole-crop SDEdit = detail on our drainage | 1 day (done here) | inversion ~1 s per km2 at 100 iters (see §4); arms ~1 min per 800 km2 | training-free, uses all three shipped stages |
| 2 | **Exemplar latent stamps** (invert real GLO-30 craters / summits, paste their latents at landmark positions, harmonise with method 1) | real-world forms for the landmarks the owner calls "cones" | +1 day + an exemplar library | same | scale mismatch for a 5.5 km summit; best for craters/calderas |
| 3 | Reconstruction guidance (DPS-style: gradient of a constraint loss — low-pass height in masks, crater depth/diameter, channel depth along a polyline — through decoder (and base) between the two steps) | lets the model choose the form, only constraints fixed | +1-2 days on top of 1 | slower (gradients through the 250 M base model) | = method 1 with a constraint loss instead of a full target; PnP-CM/CoSIGN are the CM-specific versions |
| 4 | Amortised encoder (train a small CNN on decoder(z) -> z pairs, or retrain the VAE encoder on GLO-30 crops) | same as 1, instant | 1-3 days + 2-6 h GPU | one pass | the scaling path for 1-3 |
| 5 | ControlNet/LoRA on the base model (landmark masks/sketch + lowfreq as control), volcanic fine-tune | strongest control, learnt harmonisation | weeks: data (GLO-30 volcanic tiles -> latents needs 4), 10s of GPU hours, 24 GB class GPU for base fine-tune | one pass | Earthbender/CoSIGN recipe; not on a 4060 laptop |
| - | Coarse-map SNR tuning (documented path) | none below 7.68 km (TD-1 A2: crater 568 -> 8 m) | 0 | fast | rejected by TD-1 |
| - | Frequency split (TD-1 A3h) | shape kept, detail ignores drainage | done | fast | baseline |

## 4. Prototype (crop, 25 x 25 km, 10x frame)

Built: methods 1 and 2 (`tools/terrain-diffusion/td_steer.py`, `td2_measure.py`). Crop x -7969..16991,
y -3540..21420 m (V3 frame), with the whole island as guidance (the lowfreq channel and coarse cells), so only the crop
is decoded. Target R0 = V3 + the raw x10 < 4 km band inside feathered masks (Caldera crater, Red Mountain crater,
Ghostgate, census trenches). Full table and what to look at: `target/worldgen/td2/README.md`.

Facts measured:
- Our differentiable decoder tile matches the stock one (max |d| 1.9e-6). bf16 backward needs
  `cudnn.benchmark` (4.3 s -> 0.49 s per 512 px tile); fp32 backward is 15x the forward.
- Inversion of the crop + 1.9 km margin (830 km2, 16 tiles, 100 Adam steps): 15.2 min, normalised residual MSE
  43.7 -> 2.7 (still falling), latent std 1.15 -> 1.91. The target's residual std is 5.6 in the decoder's
  normalised units, so it sits far outside what the decoder was trained on. Teide (real GLO-30) inverts to MSE 0.08
  in 4 min at residual std 0.88.
- B1 (the TD-1 method) loses the landmarks (Caldera crater 15 m, Red Mountain crater 0 m, foyadas present 7-36 %).
  I0 (decode the inverted latents) keeps them: 268 m / 1.64 km crater (target 320 m / 1.64 km), Red Mountain crater
  453 m, upper Mamaea 253 m deep, present 94 %; 1 km RMS 22 m.
- Re-imagining strength (sigma) is the knob. Landmark-only at 0.35 (I2): crater 299 m, Red Mountain crater 230 m,
  foyadas present 55-100 %. At 0.7 (I4) the Caldera crater grows to 438 m / 1.83 km, the Red Mountain crater is
  lost, and narrow ravines fade (trench-1 35 %). Whole-crop 0.35 (I3) keeps the valley layout (300 m-2 km band corr
  0.82) and 36 % of our < 500 m detail correlation.
- Re-imagining steepens: slope p50 19.1 (I0) -> 20.4 (I3) -> 24.1 (I5); Etna/Teide refs 10-12.8 (TD-1 measure).
  Closed basins rise from 14 to 35 (the model adds pits). The summit loses height (5669 target -> 5123 I3, 4836 I5).
- Exemplar stamp (Teide summit latents in a 5 km disc on Red Mountain): summit-zone slope p50 37.6 (target) ->
  30.5 (E3) -> 30.0 (E1); summit 5205 / 4860 m.
- One landmark (Bani-Dad south, census trench-8) is missing in every arm including I0: unexplained.
- Repeatability: arms run from cached latents, with a fixed seed and cudnn benchmark off, which should be
  deterministic (not re-run to check). The inversion itself uses cudnn benchmark, whose algorithm choice may vary
  between runs.

Inferences (not tested):
- The target is the main limit on quality. V3 plus the x10 raw bands is steeper and rougher than real terrain
  (summit-zone slope p50 38 deg), so inversion needs out-of-distribution latents. Re-imagining them then
  roughens the result further. A smoother target with real exemplars should invert better and come out calmer.
- Per-landmark sigma fits the owner's list. Small or narrow forms (summit crater, foyadas) need sigma <= 0.35 or a
  hard keep. Broad forms (the Caldera crater, lakes, the bay) survive 0.7.

## 5. Recommendation

1. **Adopt decoder inversion + masked RePaint/SDEdit (method 1) as the TD path.** It is the only tested way to
   get sub-6 km landmarks through the model. Per landmark, choose hard keep or a sigma. Use the exemplar stamp for
   Red Mountain's summit. Use whole-island sigma 0.35 for the in-between terrain, so its detail follows our valleys.
2. **Build the target from forms, not from V3 + raw x10.** Use a smooth, low-relief landmark target (positions,
   depth and diameter from the census) plus real GLO-30 exemplars (inverted once, cached) for craters and
   summits. Let the model add the detail. This needs an exemplar library: more GLO-30 calderas and summit craters
   (download), each inverted in ~4 min.
3. **Replace per-run inversion with an amortised encoder (method 4)** once the look is approved. Train a small
   CNN on (decoder(z) -> z) pairs drawn from the base model's own latents, plus inverted GLO-30 tiles. Cost: ~2-3
   days to build, ~2-4 h of GPU once. Until then, invert once per target change and cache it. Seed and sigma
   iterations reuse the cache.
4. Next, if the owner wants the model to choose the forms: constraint guidance (method 3), with losses on crater
   depth and diameter and on channel depth along the polylines instead of a full target. ControlNet/LoRA
   (method 5) only if 1-3 fall short; it needs an encoder and a 24 GB-class GPU.

Cost to scale to the island (~3000 km2 decoded): inversion at today's rate ~55 min per target at 100 steps
(not converged), or ~2-5 min with an encoder. Arms: TD-1 measured ~23 s for the island. This crop decode took
~1 min for 830 km2 (slower per km2 than TD-1, not investigated), so ~1-4 min per island arm. VRAM stays at
2.2 GB.
