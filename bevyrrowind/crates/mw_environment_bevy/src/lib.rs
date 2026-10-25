//! The game's environment layer (app-split.md slice 6): the sea and interior water (`water`),
//! the weather records, CLI and live control (`weather`), Masser, Secunda and the sun disk
//! (`night_sky`), and the view's render settings (`render`: ray tracing, upscaling, shadows).

pub mod night_sky;
pub mod render;
pub mod water;
pub mod weather;
