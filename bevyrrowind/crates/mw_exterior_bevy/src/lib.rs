//! The streamed game view (app-split.md slice 13): `exterior::run` composes every game plugin
//! crate into the App; `level` holds the synthetic test levels (TS1) it can open, `pause_menu`
//! the Esc menu and `main_menu` the title menu of a release start. The binary keeps the CLI
//! (`view level:` parses its flags there).

pub mod exterior;
pub mod level;
pub mod main_menu;
pub mod pause_menu;
