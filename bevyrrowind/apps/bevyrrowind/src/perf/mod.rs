//! `bevyrrowind perf ...` subcommands (suite runner, sweep, trace and Nsight summaries). The
//! viewer side (benchmark plugins, flags, `--off`, `render_size`) is `mw_devtools_bevy::perf`
//! (app-split.md slice 12a).

mod ab;
mod chain;
pub mod cli;
mod power;
mod sweep;
