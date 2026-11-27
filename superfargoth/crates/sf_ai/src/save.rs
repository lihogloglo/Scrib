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
//! the never-reused id sequence, the stack's clock, the serving mode) and, for an actor at tier 0,
//! its [`LogicalState`] with its due instant as a [`SavedLogical`] (v2, `MA22b`: where it walks,
//! how far, until when; tier 0 is the only record of where a bodiless actor is).
//! Either may be absent (a tier-0 state whose stack is not built yet; a body with no tier-0
//! state). Later slices add the schedule cursor, activity claim and running drive to
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

use sf_ids::GameInstant;

use crate::directive::{DirectiveStack, SavedStack, StackError};
use crate::logical::LogicalState;

/// Section key (owner `sf`, ai.md "Saves").
pub const AI_SECTION: &str = "sf/ai";
/// Schema version of [`AI_SECTION`]. A shape or meaning change bumps it, freezes the old types
/// in [`legacy`] and adds a typed step (save.md "Versioning and migration").
///
/// v2 (`MA22b`): [`ActorSave`] carries the tier-0 state; v1 is frozen in [`legacy::v1`].
pub const AI_VERSION: u32 = 2;

/// The stored form of `sf/ai`: every actor the host holds AI state for, by stable key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AiSave {
    pub actors: BTreeMap<ActorKey, ActorSave>,
}

/// A loaded section's live stacks and tier-0 states, each in key order ([`AiSave::into_parts`]).
pub type AiParts = (Vec<DirectiveStack>, Vec<(ActorKey, SavedLogical)>);

/// One actor's saved AI state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorSave {
    /// Its directive stack (`None`: a tier-0 state whose stack the host had not built yet).
    pub directives: Option<SavedStack>,
    /// Its tier-0 state (`None`: not at tier 0; its body is streamed in, or it has none yet).
    pub logical: Option<SavedLogical>,
}

/// A tier-0 state as the host's store holds it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavedLogical {
    pub state: LogicalState,
    /// When the store steps it next: its `until`, or earlier when the host woke it (a push or a
    /// cancel since its last step) or it serves nothing yet; `None` when nothing is due. Saved as
    /// is so a load steps the same actors at the same instants, in the same order.
    pub due: Option<GameInstant>,
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
    /// The save form of `stacks` (keyed by their actors), no tier-0 states.
    pub fn from_stacks<'a>(stacks: impl IntoIterator<Item = &'a DirectiveStack>) -> Self {
        Self::from_parts(stacks, std::iter::empty())
    }

    /// The save form of `stacks` and the tier-0 `states` (by actor; a key may be in either or
    /// both).
    pub fn from_parts<'a>(
        stacks: impl IntoIterator<Item = &'a DirectiveStack>,
        states: impl IntoIterator<Item = (ActorKey, SavedLogical)>,
    ) -> Self {
        let mut actors: BTreeMap<ActorKey, ActorSave> = stacks
            .into_iter()
            .map(|s| (s.actor(), ActorSave { directives: Some(s.to_saved()), logical: None }))
            .collect();
        for (actor, state) in states {
            actors.entry(actor).or_insert(ActorSave { directives: None, logical: None }).logical = Some(state);
        }
        Self { actors }
    }

    /// Refuses a section whose stacks break their invariants, naming the first such actor. Run
    /// before anything is applied (`SaveSection::check`). A tier-0 state needs no check: any
    /// value is one the host can step (a missing route end or placement holds the actor).
    pub fn check(&self) -> Result<(), AiSaveError> {
        for (actor, a) in &self.actors {
            if let Some(d) = &a.directives {
                d.check().map_err(|error| AiSaveError { actor: *actor, error })?;
            }
        }
        Ok(())
    }

    /// The live stacks, in key order (tier-0 states dropped). Fails like [`check`](Self::check).
    pub fn into_stacks(self) -> Result<Vec<DirectiveStack>, AiSaveError> {
        self.into_parts().map(|(stacks, _)| stacks)
    }

    /// The live stacks and the tier-0 states, each in key order. Fails like [`check`](Self::check).
    pub fn into_parts(self) -> Result<AiParts, AiSaveError> {
        let mut stacks = Vec::new();
        let mut states = Vec::new();
        for (actor, a) in self.actors {
            if let Some(d) = a.directives {
                stacks.push(DirectiveStack::from_saved(actor, d).map_err(|error| AiSaveError { actor, error })?);
            }
            if let Some(l) = a.logical {
                states.push((actor, l));
            }
        }
        Ok((stacks, states))
    }
}

/// Frozen older `sf/ai` shapes, one module per version, each with its step to the next
/// (`sf_save_bevy::sections::ai` registers the steps).
pub mod legacy {
    /// `sf.ai.v1` (AI5 until `MA22b`): stacks only, no tier-0 states.
    pub mod v1 {
        use std::collections::BTreeMap;

        use serde::{Deserialize, Serialize};
        use sf_ids::ActorKey;

        use crate::directive::SavedStack;

        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        pub struct AiSave {
            pub actors: BTreeMap<ActorKey, ActorSave>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ActorSave {
            pub directives: SavedStack,
        }

        impl AiSave {
            /// v1 -> v2: every stack kept, no tier-0 states (the host rebuilds them as any load
            /// did before v2).
            pub fn migrate(self) -> super::super::AiSave {
                let actors = self
                    .actors
                    .into_iter()
                    .map(|(k, a)| (k, super::super::ActorSave { directives: Some(a.directives), logical: None }))
                    .collect();
                super::super::AiSave { actors }
            }
        }
    }
}
