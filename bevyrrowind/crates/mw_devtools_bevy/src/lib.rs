//! The game's dev tools (app-split.md slice 11): live A/B switches and the F3 perf panel ([`live_ab`],
//! [`perf_panel`], linked in every
//! interactive run), the dev console's game commands ([`console`]), BRP agent tools ([`remote`],
//! `remote` feature), the GPU pipeline watch behind the screenshot gate ([`pipeline_watch`]) and
//! readiness checks shared with the viewers ([`ready`]), and the viewers' benchmark and `--off` settings
//! ([`perf`], slice 12a). One crate because `live_ab`, `console` and `remote`
//! import each other (app-split.md cycle 4).

pub mod console;
pub mod live_ab;
pub mod perf;
pub mod perf_panel;
pub mod perf_panel_model;
pub mod pipeline_watch;
pub mod ready;
#[cfg(feature = "remote")]
pub mod remote;
