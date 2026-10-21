# Parked patches (not applied)

Fixes with a confirmed cause that were written but never built or verified. They're parked here as diffs so that
nobody's build triggers a long vendored-Bevy rebuild for unverified code. Owner, 2026-10-21: these bugs wait on the
rendering-bugs list (bevyrrowind ROADMAP 1.12, `docs/visual-checks.md`).

| File | Bug | Apply from | Status |
|---|---|---|---|
| `LT-4-cluster-main-pass-viewport.diff` | LT-4: lamp light cut in screen blocks under DLSS upscaling | `superfargoth/vendor/` (`patch -p1`) | Cause confirmed (source + owner A/B: DLSS quality only). Bevy 0.20's `view_fragment_cluster_index` multiplies a render-resolution `frag_coord` by `cluster_factors` sized for the output viewport; the hunk rescales by `viewport / main_pass_viewport`. Unbuilt. No upstream issue found yet: file one. |
| `SH-20-mover-shadow-refresh.diff` | SH-20: choppy lamp shadows on characters | repo root of superfargoth (`git apply`) | Cause confirmed in code: shadow reset B3's skip rule hashes skinned casters by root position on a ≥ 20 cm grid (joints ignored) and refreshes at most 2 faces per frame. The hunk redraws a face the frame its casters move or deform, outside the budget. Also documents B3's own hunks and the LT-4 row in `vendor/README.md`. Unbuilt, cost unmeasured. |
| `SH-20-bevyrrowind-app.diff` | SH-20 A/B switch `--off movershadows` | bevyrrowind repo root (`git apply`) | Goes with the one above. |

Next steps per bug: `.sessions/lightclusters.md`, `.sessions/charshadows.md` (not in git; summarised in the
visual-check entries).
