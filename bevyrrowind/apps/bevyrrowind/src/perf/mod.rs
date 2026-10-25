//! `bevyrrowind perf ...` subcommands (suite runner, sweep, trace and Nsight summaries). The
//! viewer side (benchmark plugins, flags, `--off`, `render_size`) moved to `mw_devtools_bevy::perf`
//! (app-split.md slice 12a); the old `crate::perf::` paths keep working.

pub mod cli;
mod sweep;

pub use mw_devtools_bevy::perf::{Off, Part, Perf, PerfFlags, apply_off, render_size};
