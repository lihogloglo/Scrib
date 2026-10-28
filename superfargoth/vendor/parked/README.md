# Parked patches (all applied)

All three previously parked patches have been verified and applied to the vendored crates and game app in Slice 1:

1. `LT-4-cluster-main-pass-viewport.diff`: applied via upstream `#26236` (`superfargoth/vendor/bevy_pbr/src/render/clustered_forward.wesl`).
2. `SH-20-mover-shadow-refresh.diff`: applied to `superfargoth/vendor/bevy_pbr/src/render/light.rs` (`ShadowMoverRefresh`).
3. `SH-20-bevyrrowind-app.diff`: applied to `bevyrrowind/crates/mw_game_core/src/perf.rs`, `bevyrrowind/crates/mw_environment_bevy/src/render.rs`, and `bevyrrowind/apps/bevyrrowind/src/main.rs` (`--off movershadows`).
