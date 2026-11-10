# Sweep `au_ladder_stand`: `off` over balmora

- `bevyrrowind view Balmora --bench benchmarks/exterior_balmora.json` at 2560x1440; off: none
- medians over repeats of per-run medians (frame stats: per-run median / p95); 29 runs
- machine: quiet for every run

| off | runs | frame median ms | frame p95 ms | timed GPU ms | main opaque GPU ms | shadow views GPU ms | render busy ms | render thread ms | point lights |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| none | 2 | 9.83 | 11.68 | 8.77 | 2.47 | 0.00 | 7.60 | 8.56 | 210.00 |
| sky | 2 | 8.35 | 10.01 | 7.32 | 3.87 | 0.00 | 6.18 | 7.24 | 210.00 |
| envmap | 2 | 9.87 | 12.30 | 7.85 | 2.19 | 0.00 | 7.77 | 8.57 | 210.00 |
| clouds | 2 | 9.73 | 11.99 | 8.55 | 2.40 | 0.00 | 7.62 | 8.49 | 210.00 |
| fog | 2 | 9.56 | 11.28 | 8.58 | 2.41 | 0.00 | 7.54 | 8.30 | 210.00 |
| froxels | 2 | 9.59 | 11.21 | 8.49 | 2.42 | 0.00 | 7.50 | 8.34 | 210.00 |
| shadows | 2 | 8.71 | 10.33 | 8.31 | 2.20 | 0.00 | 6.60 | 7.60 | 210.00 |
| farshadows | 2 | 9.61 | 11.29 | 8.61 | 2.31 | 0.00 | 7.41 | 8.37 | 210.00 |
| lights | 1 | 9.30 | 11.04 | 8.07 | 1.95 | 0.00 | 7.38 | 8.06 | 0.00 |
| particles | 1 | 9.43 | 11.03 | 8.30 | 2.38 | 0.00 | 7.25 | 8.30 | 210.00 |
| ssao | 2 | 9.33 | 10.98 | 8.23 | 2.36 | 0.00 | 7.31 | 8.09 | 210.00 |
| bloom | 2 | 9.42 | 11.26 | 8.42 | 2.42 | 0.00 | 7.22 | 8.17 | 210.00 |
| surface | 1 | 9.38 | 10.90 | 8.14 | 2.24 | 0.00 | 7.22 | 8.12 | 210.00 |
| actors | 2 | 9.25 | 10.54 | 8.36 | 2.38 | 0.00 | 6.15 | 8.43 | 210.00 |
| terrain | 2 | 9.38 | 11.09 | 8.17 | 1.86 | 0.00 | 7.38 | 8.10 | 210.00 |
| statics | 2 | 7.99 | 9.33 | 7.69 | 1.65 | 0.00 | 4.87 | 7.25 | 0.00 |
