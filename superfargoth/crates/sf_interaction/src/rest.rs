//! The rest hook's verb (interaction.md "Beds, lean spots and rest (IN3)"): a def offers
//! [`VERB_SLEEP`] as a slotless verb at its top priority, so the press on a bed is Sleep and its pose
//! slots (`lie`, edge `sit`) are on the hold list. Its approval becomes the shell's
//! `RestRequested` (`sf_interaction_bevy::rest`); the rest UI, its refusals and the time skip are
//! the host's. Here so the Bevy-free adapter names the same tag.

use crate::def::TagId;

/// The slotless verb whose approved use asks the host for its rest UI at this placement.
pub const VERB_SLEEP: TagId = TagId::of("sleep");
