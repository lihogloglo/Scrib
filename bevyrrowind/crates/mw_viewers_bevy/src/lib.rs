//! The game's standalone viewers (app-split.md slice 12): [`view`] renders one cell or model
//! (cooked, [`skinned_view`] and [`anim_view`] scaffolds), [`character_view`] a lineup of dressed
//! actors. Each builds its own Bevy `App`; the binary's `view`/`character` subcommands call `run`.

pub mod anim_view;
pub mod character_view;
pub mod skinned_view;
pub mod view;
