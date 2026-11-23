# Groundcover: history

Superseded parts of `../groundcover.md`, moved here verbatim.

## Original GC-2 plan (2026-11-22; built as GC-2a to GC-2b3, see the spec's "Build plan")

- **GC-2** mesh renderer (`sf_groundcover_bevy`, ~600-900 lines, guess):
  - Tile ring manager around the camera, with hysteresis.
  - Task scatter from the GC-1 rules plus terrain `height_at`, written into a fixed tile-block slot pool, one
    `GpuBatchedMesh3d` per (model, material); several models share a batch through ranges when they share slab and
    bind group.
  - `ExtendedMaterial<StandardMaterial, GroundcoverWind>`: alpha mask, height-weighted bend from `sf_wind`,
    prepass previous-wind motion vectors.
  - Near batch casts shadows; the rest is `NotShadowCaster`.
  - Origin shift through the statics' rewrite, and live/capacity slot gauges.
  - Test: deterministic scatter and a valid slot layout (headless), plus an `#[ignore]` measurement printing
    instances and bytes per tile.
  - Then the adapter's first species set on one cell: a test scene registered (`visual-checks/groundcover.md`,
    "awaiting owner") and the cost measured (untraced A/B, `ledger/perf.md`).
  - Out of scope: GPU producer, visibility-range patch, LOD meshes.
