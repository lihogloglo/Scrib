# Ledger: groundcover (species, rules, scatter, blade and mesh renderers)

Format: `date | what was tried | measured result (scene, ms, counters) | verdict | commit or doc`. Newest first. Same setup as
[perf](perf.md) for frame numbers. Spec: `superfargoth/docs/specs/groundcover.md`.

2026-11-22 | GC-2b3 game wiring (`GroundcoverGamePlugin`, `--off groundcover`) + scenes gc-ascadian/grazelands/bittercoast | tests 71 + 22 + 2; dev build OK; not yet seen in the game | awaiting owner GC-2 | uncommitted
2026-11-22 | GC-2b2 Morrowind species data + resolver (`mw_world::groundcover`); `LayerChannels` made sparse (layer ids are hashes) | 4 + 1 tests; no runtime numbers yet | kept | uncommitted
2026-11-22 | GC-2b1 `TerrainGround` + `TerrainLayersMap` (terrain heights + land-texture weights for groundcover) | 21 + 21 headless tests; no runtime numbers yet | kept | uncommitted
2026-11-22 | GC-2a `sf_groundcover_bevy` mesh renderer: tile rings, slot pool in `GpuBatchedMesh3d`, origin-shift rewrite, wind `ExtendedMaterial` (forward), near-shadow split, gauges | headless only: 17 tests; synthetic 4-species tile ~303 instances, 19 KB GPU, ~42 us scatter; ground cache 124 us vs 42 us direct on a cheap sampler | kept; not run on a GPU yet | uncommitted
2026-11-22 | GC-1 `sf_groundcover` core: species + rule table + overrides, deterministic jittered-grid scatter per 16 m tile (SplitMix64, integer accept test), tile-local f32 positions, 32-bit tag | 18 tests (determinism across orders and origins, constraints, binomial 5σ, override priority, density twin = scatter); measurement (dev profile, synthetic ground): 772 instances/tile 121 µs, 103/28 µs, 26/7 µs at footprints 0.35/0.7/1.3 m; real terrain sampling not yet measured | kept | uncommitted (groundcover-research session)
2026-11-22 | GC-R mesh path choice (research, no code) | vendored `GpuBatchedMesh3d`: 64 B instance, one u32 tag, cost scales with capacity x views, no per-instance visibility range (`mod.rs:581`) | CPU tile scatter into batches (UE landscape grass pattern) | spec "Renderers"
