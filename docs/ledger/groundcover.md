# Ledger: groundcover (species, rules, scatter, blade and mesh renderers)

Format: `date | what was tried | measured result (scene, ms, counters) | verdict | commit or doc`. Newest first. Same setup as
[perf](perf.md) for frame numbers. Spec: `superfargoth/docs/specs/groundcover.md`.

2026-11-22 | GC-1 `sf_groundcover` core: species + rule table + overrides, deterministic jittered-grid scatter per 16 m tile (SplitMix64, integer accept test), tile-local f32 positions, 32-bit tag | 18 tests (determinism across orders and origins, constraints, binomial 5σ, override priority, density twin = scatter); measurement (dev profile, synthetic ground): 772 instances/tile 121 µs, 103/28 µs, 26/7 µs at footprints 0.35/0.7/1.3 m; real terrain sampling not yet measured | kept | uncommitted (groundcover-research session)
2026-11-22 | GC-R mesh path choice (research, no code) | vendored `GpuBatchedMesh3d`: 64 B instance, one u32 tag, cost scales with capacity x views, no per-instance visibility range (`mod.rs:581`) | CPU tile scatter into batches (UE landscape grass pattern) | spec "Renderers"
