# Decisions superseded

Entries moved out of [DECISIONS.md](DECISIONS.md) because a later entry explicitly replaced them (doc cleanup S9b,
2026-10-26). Entries are verbatim, grouped by topic; each is followed by `superseded-by:` naming the later entry in
DECISIONS.md. New decisions are still appended to DECISIONS.md only.

## Shadows (pre-reset)

- 2026-10-19 (shf6): local-light shadows (spec `specs/shadows.md`): the shadowed set is chosen by
  `sf_sky_bevy::shadow_importance` (strength at the viewer from `range`, not the flickering intensity, × screen
  coverage × priority, off-screen lights included, hysteresis), replacing nearest-first. Bevy 0.20 redraws every
  point shadow every frame and is render-thread bound (~1.9 ms per light in the census office), so more shadowed
  lights need our own cache: static casters cached per light in a persistent cube array, movable casters drawn over
  a copy, the result copied into Bevy's cube array while Bevy's own views for those lights are emptied (Bevy keeps
  sampling and filtering; upstream if it stays small).
  superseded-by: 2026-10-21 "(render-audit, owner): shadows reset. Stock Bevy shadow path first" (DECISIONS.md); the importance-ranked set and own lamp cache were replaced by stock Bevy plus vendored PRs #26149/#26161.

- 2026-10-21 (render-audit, after lampcost): the lamp cube cache stays through slice A and is deleted only when slice B's
  patch (#26149/#26161) measures equal or better: stock Bevy costs ~+12 ms render thread for 8 lamps (perf.md
  "Per-lamp GPU cost"). The ~0.8 ms GPU per lamp was Bevy contact shadows (0.68/lamp), not the cube maps.
  superseded-by: 2026-10-21 "(shadowB1) Bevy 0.20.0 `bevy_core_pipeline`/`bevy_pbr`/`bevy_render` vendored" (DECISIONS.md); B1 measured the patch and decided the cache stays.
