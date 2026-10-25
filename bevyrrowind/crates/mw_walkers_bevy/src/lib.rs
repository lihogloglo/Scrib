//! Morrowind walkers and physics glue, split out of the app (app-split.md slice 4).
//! `walkers` (NPC movement on `sf_controller_bevy`) uses `physics` (`Dead`, `KnockedDown`); one-way.

pub mod physics;
pub mod walkers;
