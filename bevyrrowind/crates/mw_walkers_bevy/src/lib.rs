//! Morrowind walkers and physics glue, split out of the app (app-split.md slice 4).
//! `walkers` (NPC movement on `sf_controller_bevy`) uses `physics` (`Dead`, `KnockedDown`); one-way.
//! `nav`: the navmesh and its baked tile ring around the player (nav.md `NV4c`).

pub mod nav;
pub mod physics;
pub mod walkers;
