# Ledger: big-world rendering (millions of trees, dense scatter)

Format: `date | what was tried | measured result (scene, ms, counters) | verdict | commit or doc`. Newest first. Spec:
`superfargoth/docs/specs/big-world-rendering.md`. Companions: [statics](statics.md), [groundcover](groundcover.md),
[worldgen](worldgen.md).

2026-11-26 | BW-0 research (no code): Bevy 0.20 source (meshlets, preprocess, vendored `GpuBatchedMesh3d`), crates (boimp, bevy_eidolon/feronia, feathrise_vegetation, bevy_flora, ze_tree, meshopt 0.7, SmartImposter), industry (Chlorophyll author statements via zye04/Meridian, UE 5.7 Nanite Foliage, Decima, Fortnite impostors) | meshlets opaque-only, no wind, per-instance upload every frame (`meshlet/asset.rs:30`, `instance_manager.rs:241`); slot path cost ~ capacity × views (`materialize.wesl`); generated worlds today: one entity per tree, 7.21 M flora = 26 fps (worldgen ledger 2026-11-19) | chosen: T0 mesh / T1 octahedral impostor / T2 per-tile canopy proxy, GPU producer into `GpuBatchedMesh3d`; model ~0.55 ms GPU avg, ~1.95 ms stand at High (Guess); BW-2 ceiling bench first | spec BW-0
