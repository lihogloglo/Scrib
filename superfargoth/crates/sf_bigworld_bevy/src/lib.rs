//! Bevy renderer for `sf_bigworld` (spec `superfargoth/docs/specs/big-world-rendering.md`): trees and dense scatter
//! drawn from retained GPU instance batches (`GpuBatchedMesh3d`), never as entities.
//!
//! BW-2 ships no plugin. The crate's one target is the example `forest_ceiling`, a bench that fills batch slots once
//! on the CPU and prints what the slot path costs (materialize, preprocess per view, prepass, shadows, main pass).
//! The GPU producer and the plugin arrive in BW-3.
