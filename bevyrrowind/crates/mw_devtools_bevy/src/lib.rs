//! The game's dev tools (app-split.md slice 11): live A/B switches and the F3 perf panel ([`live_ab`],
//! [`perf_panel`], the Ctrl+F3 Features panel ([`features_panel`]) and its flip delta probe
//! ([`switch_probe`]), linked in every interactive run), the dev console's game commands ([`console`]), BRP agent tools ([`remote`],
//! `remote` feature), the GPU pipeline watch behind the screenshot gate ([`pipeline_watch`]) and
//! readiness checks shared with the viewers ([`ready`]), and the viewers' benchmark and `--off` settings
//! ([`perf`], slice 12a). One crate because `live_ab`, `console` and `remote`
//! import each other (app-split.md cycle 4).

pub mod console;
pub mod features_panel;
pub mod features_panel_model;
pub mod live_ab;
pub mod live_switches;
pub mod part_switches;
pub mod perf;
pub mod perf_panel;
pub mod perf_panel_model;
pub mod pipeline_watch;
pub mod ready;
pub mod switch_probe;
pub mod switch_probe_model;
#[cfg(feature = "remote")]
pub mod remote;
