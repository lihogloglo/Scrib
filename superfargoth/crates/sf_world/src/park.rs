//! Parked worldspaces (CW-9, ROADMAP 1.25; frame-waste audit 2026-11-16 §2 item 10): the switch and
//! settle time of `sf_world_bevy`'s parking. A worldspace the player left stays resident (the door
//! back out stays fast: the residency cache, `SchedulerConfig::linger_secs`) but leaves every query
//! while it waits, as Unreal's loaded-but-hidden streaming levels stop ticking and `OpenMW` keeps
//! preloaded cells outside the scene graph.

use sf_tunables::{Def, Source, Unit, def};

/// `stream.interior.disable_exterior`: park the worldspaces the player left (the exterior while inside
/// an interior, and back): their entities stay resident, out of every query and render, until the
/// player returns or streaming touches them.
pub const PARK_LEFT: Def<bool> = def!(bool "stream.interior.disable_exterior" = false)
    .help("Worldspaces the player left (the exterior while indoors) stay resident but out of every query and render (Bevy Disabled)")
    .source(Source::Owner("2026-11-15: a change stays opt-in, off, until its interleaved A/B wins"));

/// `stream.interior.park_settle_s`: how long a left worldspace must stay quiet (no streaming request,
/// nothing spawned under it) before it parks, so late spawns (models finishing after the door) don't
/// make it flap between parked and live.
pub const PARK_SETTLE_S: Def<f32> = def!(float "stream.interior.park_settle_s" = 0.25)
    .range(0.0, 10.0)
    .unit(Unit::Seconds)
    .help("A worldspace the player left parks once it had no streaming request and no new entity for this long")
    .source(Source::Guess);
