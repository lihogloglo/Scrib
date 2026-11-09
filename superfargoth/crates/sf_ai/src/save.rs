//! The `sf/ai` save section (ai.md "Saves", save.md "What is saved", slice AI5): the shape, its
//! key and schema version, and the checks a loaded section must pass. Bevy-free and
//! save-agnostic like every owner crate: `sf_save_bevy::sections::ai` registers it and the shell
//! (`sf_ai_bevy`) supplies the accessors.
//!
//! **One world section, not per-placement overlay blobs.** The host keeps every actor's AI state
//! in one store keyed by [`ActorKey`], whether or not the actor's body is streamed in: tier 0
//! simulates every actor in every worldspace (ai.md "Tiers and budget"), so a stack never unloads
//! and streaming never holds a second copy (the thing save.md's "One path for unload and save"
//! guards against). The section snapshots that store at a tick boundary. Actors whose content is
//! missing (a removed mod) stay in the store and in the section untouched (save.md "Mod-list
//! changes"). Why not overlay blobs (`ai` owner in `PlacementOverlay`): a frozen blob is not
//! advanced by tier 0, and the overlay's one-version-per-owner table refuses older blobs until
//! overlay v2 can migrate them (save.md "As built", C7), while this section changes version often.
//!
//! **What is saved** (what can't be re-derived; Bethesda saves runtime package overrides and
//! package data as change-form deltas, `OpenMW` saves each package's parameters and remaining
//! duration): per actor, the directive stack as a [`SavedStack`] (entries with served minutes,
//! the never-reused id sequence, the stack's clock, the serving mode). Later slices add the
//! schedule cursor, tier-0 `(route, progress, until)`, activity and claim, running drive to
//! [`ActorSave`] by version steps (frozen types in [`legacy`]). **Not saved**, rebuilt on load:
//! plans, traces, awareness, path requests, [`DirectiveDone`](crate::DirectiveDone) events (a
//! consumer polls [`DirectiveStack::status`], which answers from the saved id sequence; `OpenMW`
//! does not save its `mDone` flag either), and anything a record can rebuild.
//!
//! **Clock:** a stack ignores clock readings older than its own (`DirectiveStack::advance`), so
//! its timers would freeze after a load that put the host clock behind it. Stacks are only ever
//! advanced with the host's logical clock, and `sf/clock` is snapshotted at the same tick
//! boundary, so a load restores both consistently; whoever converts the host clock to
//! [`GameTime`](crate::GameTime) must round down (ai-as-built.md "AI5").

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use sf_ids::ActorKey;

use crate::directive::{DirectiveStack, SavedStack, StackError};

/// Section key (owner `sf`, ai.md "Saves").
pub const AI_SECTION: &str = "sf/ai";
/// Schema version of [`AI_SECTION`]. A shape or meaning change bumps it, freezes the old types
/// in [`legacy`] and adds a typed step (save.md "Versioning and migration").
pub const AI_VERSION: u32 = 1;

/// The stored form of `sf/ai`: every actor the host holds AI state for, by stable key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AiSave {
    pub actors: BTreeMap<ActorKey, ActorSave>,
}

/// One actor's saved AI state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorSave {
    pub directives: SavedStack,
}

/// A saved stack that breaks the stack's invariants (corrupt or forged data).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiSaveError {
    pub actor: ActorKey,
    pub error: StackError,
}

impl fmt::Display for AiSaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "actor {:032x}: {}", self.actor.0, self.error)
    }
}

impl std::error::Error for AiSaveError {}

impl AiSave {
    /// The save form of `stacks` (keyed by their actors).
    pub fn from_stacks<'a>(stacks: impl IntoIterator<Item = &'a DirectiveStack>) -> Self {
        let actors = stacks.into_iter().map(|s| (s.actor(), ActorSave { directives: s.to_saved() })).collect();
        Self { actors }
    }

    /// Refuses a section whose stacks break their invariants, naming the first such actor. Run
    /// before anything is applied (`SaveSection::check`).
    pub fn check(&self) -> Result<(), AiSaveError> {
        for (actor, a) in &self.actors {
            a.directives.check().map_err(|error| AiSaveError { actor: *actor, error })?;
        }
        Ok(())
    }

    /// The live stacks, in key order. Fails like [`check`](Self::check).
    pub fn into_stacks(self) -> Result<Vec<DirectiveStack>, AiSaveError> {
        self.actors
            .into_iter()
            .map(|(actor, a)| DirectiveStack::from_saved(actor, a.directives).map_err(|error| AiSaveError { actor, error }))
            .collect()
    }
}

/// Frozen older `sf/ai` shapes, one module per version, each with its step to the next. None
/// yet: version 1 is the first.
pub mod legacy {}
