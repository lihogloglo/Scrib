//! The game's standalone viewers (app-split.md slice 12): [`view`] renders one cell or model
//! (cooked, [`skinned_view`] and [`anim_view`] scaffolds), [`character_view`] a lineup of dressed
//! actors, `clip_preview` (dev, `clip_preview` feature) clips on an NPC captured to MP4. Each
//! builds its own Bevy `App`; the binary's `view`/`character`/`clip-preview` subcommands call `run`.

pub mod anim_view;
pub mod character_view;
#[cfg(feature = "clip_preview")]
pub mod clip_preview;
pub mod skinned_view;
pub mod view;
