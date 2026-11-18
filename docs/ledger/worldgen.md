# Ledger: worldgen (world generation, `sf_worldgen`, `mw_worldgen`)

Format: `date | tried | result | verdict | commit or doc`. Newest first. Owner checks are WG- ids in `bevyrrowind/docs/visual-checks.md`.
Prior art (not ours): Procedural Tamriel's full-world stream power + droplets, 1,561 s on 90.7 M land vertices, rejected visually (`docs/research/procedural-tamriel-2026-11-18.md` §1).

2026-11-18 | WG-S2 macro look test: Vvardenfell ×10, 20 m grid (2408 × 2659 = 6.4 M samples, 48 × 53 km), implicit stream power (Braun-Willett, n 1, m 0.5, K·dt 0.03, 60 steps, restore 0.02/step) + priority-flood, arms A0 none / A1 uniform K / A2 per-region K + Red Mountain stratovolcano / A3 + thermal | 113 s for 4 arms (erosion 33-38 s, thermal 0.8 s); max 2707 / 2696 / 3269 / 3199 m; incision mean 26-30 m, p99 164-209 m; 0 pits after routing; wet/dry audit 0/0; hash equal across runs and 1 vs 4 threads | awaiting owner (WG-S2); K·dt picked from incision numbers only | uncommitted; `bevyrrowind/worldgen/vvardenfell-s2.toml`, `target/worldgen/s2/README.md`
