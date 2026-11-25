//! Morrowind walkers and physics glue, split out of the app (app-split.md slice 4).
//! `walkers` (NPC movement on `sf_controller_bevy`) uses `physics` (`Dead`, `KnockedDown`); one-way.
//! `nav`: the navmesh and its baked tile ring around the player (nav.md `NV4c`).
//! `sight`: NPC sight on the walkers' ground (`MA14`), used by `walkers`; `light_inputs`: its light
//! exposure inputs from game state (`MA16`). `search`: walkers search at the last known position
//! when their awareness reaches Searching (`AI8f2`). `noise_emitters`: the player's footsteps and landings
//! as noise NPCs hear (`AI8c2`). `visibility`: the player's sneak stance and stealth stats as NPC
//! sight reads them (`MA18`).

pub mod light_inputs;
pub mod nav;
pub mod noise_emitters;
pub mod physics;
pub mod search;
pub mod sight;
pub mod visibility;
pub mod walkers;
