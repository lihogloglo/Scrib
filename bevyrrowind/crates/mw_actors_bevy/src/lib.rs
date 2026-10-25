//! Morrowind actors: character content, actor effects, actor particles, faces and held items, split out of the app
//! (app-split.md slice 5). `actors` wires the other four; `held` reads `actors::MwCharacters`.

pub mod actor_fx;
pub mod actor_particles;
pub mod actors;
pub mod faces;
pub mod held;
