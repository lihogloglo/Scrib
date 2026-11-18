//! The streamed game view (app-split.md slice 13): `exterior::run` composes every game plugin
//! crate into the App; `level` holds the synthetic test levels (TS1) it can open, `pause_menu`
//! the Esc menu, `main_menu` the title menu of a release start and `phase` the app phases (menus.md
//! "App phases": the menu shows with no world). The binary keeps the CLI
//! (`view level:` parses its flags there). `worldgen` opens a generated world as the exterior (`view world:`).

mod catalog_warm;
pub mod exterior;
pub mod graphics_page;
mod gi_volume;
mod intro;
mod teardown;
pub mod level;
mod loading_screen;
mod origin_rebase;
pub mod quality_presets;
pub mod restream;
pub mod main_menu;
pub mod pause_menu;
pub mod phase;
mod shelter_casters;
mod trails;
pub mod worldgen;
