# Sweep `au_ladder_walk`: `off` over balmora_walk

- `bevyrrowind view Balmora --bench benchmarks/exterior_balmora_walk.json` at 2560x1440; off: none
- medians over repeats of per-run medians (frame stats: per-run median / p95); 13 runs
- machine: quiet for every run

| off | runs | frame median ms | frame p95 ms | timed GPU ms | main opaque GPU ms | shadow views GPU ms | render busy ms | render thread ms | point lights |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| none | 1 | 10.53 | 12.97 | 7.18 | 1.47 | 0.00 | 8.47 | 8.99 | 286.00 |
| sky | 1 | 8.70 | 11.50 | 4.90 | 1.99 | 0.00 | 6.71 | 7.21 | 286.00 |
| clouds | 1 | 10.63 | 13.45 | 6.99 | 1.46 | 0.00 | 8.61 | 9.11 | 286.00 |
| fog | 1 | 10.82 | 13.97 | 7.02 | 1.47 | 0.00 | 8.81 | 9.33 | 286.00 |
| shadows | 1 | 9.01 | 10.90 | 6.94 | 1.36 | 0.00 | 6.96 | 7.53 | 286.00 |
| lights | 1 | 10.00 | 11.15 | 6.86 | 1.23 | 0.00 | 8.01 | 8.52 | 0.00 |
| particles | 1 | 10.41 | 12.76 | 7.18 | 1.46 | 0.01 | 8.55 | 9.11 | 286.00 |
| surface | 1 | 10.57 | 12.84 | 7.02 | 1.37 | 0.00 | 8.56 | 9.07 | 286.00 |
| actors | 1 | 8.32 | 10.04 | 6.93 | 1.44 | 0.00 | 6.63 | 7.44 | 286.00 |
| walkers | 1 | 10.58 | 12.74 | 7.07 | 1.46 | 0.00 | 8.61 | 9.12 | 286.00 |
| physics | 1 | 11.04 | 13.31 | 7.12 | 1.47 | 0.00 | 8.97 | 9.48 | 286.00 |
| terrain | 1 | 10.38 | 12.44 | 6.37 | 0.91 | 0.00 | 8.33 | 8.83 | 286.00 |
| statics | 1 | 7.04 | 8.05 | 6.58 | 1.34 | 0.00 | 4.51 | 6.35 | 0.00 |
