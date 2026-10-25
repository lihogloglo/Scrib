//! The game's shared markers, enums and constants: what the app's plugins read from each other
//! (`docs/specs/app-split.md`, layer L0). Types only, no systems; it must rarely change, since
//! every game plugin crate depends on it. The app re-exports each item at its old path
//! (`crate::exterior::Settled`, ...).

pub mod doors;
pub mod errors;
pub mod exterior;
pub mod perf;
pub mod player;
pub mod scenes;

pub use errors::shipped_errors;
