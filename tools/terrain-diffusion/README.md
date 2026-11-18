# terrain-diffusion experiment (worldgen lane)

Tests whether [terrain-diffusion](https://github.com/xandergos/terrain-diffusion) (MIT code + MIT weights,
model `xandergos/terrain-diffusion-30m`) can add realistic 30 m detail to our 10x Vvardenfell guides while keeping
their large features. Research only: nothing here is part of the cargo workspace. Results and the arms table:
`target/worldgen/td/README.md`.

## Files

- `guide_io.py`: guide v1 read/write, resampling to 30 m, ~1 km low-pass, offshore bathymetry.
- `td_inject.py`: `InjectedWorldPipeline` (subclass, the clone is not edited) and `Guidance` (our heights in the
  model's coarse / latent spaces). Method in its module docs.
- `run_arm.py`: one arm -> one guide folder (`--mode stock|coarse|inject`).
- `retention.py`: input vs output correlation / RMS after a ~1 km low-pass over land, crater depth.

## Setup (once, on C:, D: is nearly full)

```sh
winget install --id Python.Python.3.12 --scope user
cd C:\tools\terrain-diffusion
git clone https://github.com/xandergos/terrain-diffusion repo      # tested at e8dcb4b
py -3.12 -m venv venv
venv\Scripts\python -m pip install torch torchvision --index-url https://download.pytorch.org/whl/cu128
venv\Scripts\python -m pip install -r repo\requirements.txt
# The pipeline needs WorldClim 10' bio 1/4/12/15 in repo\data\global (it asks interactively otherwise):
#   https://geodata.ucdavis.edu/climate/worldclim/2_1/base/wc2.1_10m_bio.zip -> extract those four .tif there
```

Weights download on first run to `HF_HOME` (set `HF_HOME=C:\tools\terrain-diffusion\hf`).

## Rerun

From the repo root (Git Bash), `PY=/c/tools/terrain-diffusion/venv/Scripts/python.exe`, `export HF_HOME='C:\tools\terrain-diffusion\hf'`:

```sh
$PY tools/terrain-diffusion/run_arm.py --mode stock  --size-km 60 --out target/worldgen/td/A1-stock
$PY tools/terrain-diffusion/run_arm.py --mode coarse --input target/worldgen/vo/V3 --out target/worldgen/td/A2-coarse-V3
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vo/V3 --sigmas 0.35 --out target/worldgen/td/A3a-V3
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vo/V3 --sigmas 1.5,0.7,0.35 --out target/worldgen/td/A3b-V3
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vo/V3 --sigmas 1.5,0.7,0.35 --no-final-replace --out target/worldgen/td/A3c-V3
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vo/V3 --sigmas 0.35 --decoder-sigma 0.3 --out target/worldgen/td/A3d-V3
#   A3e/f/g: --decoder-sigma 0.6 | 1.0,0.1 | 2.0,0.1
$PY tools/terrain-diffusion/freqsplit.py target/worldgen/td/A3a-V3 target/worldgen/td/A3h-V3 --cut-m 1000
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vvardenfell --scale 10 --sigmas 0.35 --out target/worldgen/td/A4a-raw10
$PY tools/terrain-diffusion/freqsplit.py target/worldgen/td/A4a-raw10 target/worldgen/td/A4b-raw10
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/vvardenfell --scale 10 --sigmas 0.35 --decoder-sigma 0.3 --out target/worldgen/td/A4c-raw10
$PY tools/terrain-diffusion/run_arm.py --mode inject --input target/worldgen/s4/C1 --sigmas 0.35 --out target/worldgen/td/A5a-C1
$PY tools/terrain-diffusion/freqsplit.py target/worldgen/td/A5a-C1 target/worldgen/td/A5b-C1
$PY tools/terrain-diffusion/retention.py target/worldgen/td/A2-coarse-V3 target/worldgen/td/A3?-V3 target/worldgen/td/A4?-raw10 target/worldgen/td/A5?-C1 --json target/worldgen/td/retention.json
./target/release/scrib-worldgen.exe measure --ref target/worldgen/real/etna-30m --ref target/worldgen/real/teide-30m <arms...> --mask-sea --summit auto --spacing 30 --out target/worldgen/td/measure.md
```

`freqsplit.py` = our heights above `--cut-m` + the model arm's detail below it (CPU only).

Default seed 1234 (`--seed`). Each output folder also holds `input30.npy` (the input on the same 30 m grid) and
`raw_unclamped.npy` (model output before the coastline clamp).

## TD-2: landmark steering (decoder inversion + latent RePaint/SDEdit)

- `td_steer.py`: builds the landmark target (V3 + raw x10 < 4 km band in census masks), inverts the decoder over
  the 25 km crop (latents cached in `target/worldgen/td2/_work/z_inv.pt`; delete it to re-invert), and runs arms
  (`ARMS` table in the script: B1, I0-I7, E1-E3). E arms invert an exemplar (`--stamp teide-30m`, cached
  `_work/z_teide-30m.pt`) and stamp it at `--stamp-at` (V3 summit).
- `td2_measure.py`: retention vs R0-target + crater / foyada / summit / coast numbers.

```sh
$PY tools/terrain-diffusion/td_steer.py --arms B1,I0,I1,I2,I3,I4,I5 --out target/worldgen/td2   # ~15 min inversion + ~1 min/arm
$PY tools/terrain-diffusion/td_steer.py --arms E3,E1,E2 --out target/worldgen/td2               # ~4 min Teide inversion + 3 arms
$PY tools/terrain-diffusion/td2_measure.py target/worldgen/td2 R-V3 R-raw10 B1 I0 I1 I2 I3 I4 I5 E3 E1 E2 --json target/worldgen/td2/measure.json
for a in R0-target B1 I0 I1 I2 I3 I4 I5 E3 E1 E2; do ./target/release/scrib-worldgen.exe landforms target/worldgen/td2/$a --scale 10 --names target/worldgen/lf/names-v3.json --out target/worldgen/td2/$a/lf; done
```
