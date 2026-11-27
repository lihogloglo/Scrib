//! The rest hook (`IN3a`, interaction.md "Beds, lean spots and rest (IN3)"): the default of an
//! approved [`VERB_SLEEP`] use is one [`RestRequested`], "open the rest UI for this user at this
//! placement". Nothing else happens here: the host gates the use beforehand
//! ([`crate::ActivationSystems::Gates`], a refusal writes no approval), and owns the rest UI, its
//! own refusals (underwater, in the air) and the time skip (ai.md `fast_forward`).
//!
//! - **Match.** `verb == Some(VERB_SLEEP)` only: a validated use always names its verb; a script's
//!   `Activate` with no verb is the primary use of no default here. A handled (script) use or a
//!   gate's refusal never reaches [`UseApproved`], so writes none.
//! - **User.** The [`InteractionUser`] whose `actor` is the approval's (as the seat default finds
//!   it); none found: nothing written. The handler does not filter users (it knows no players):
//!   the host's consumer acts on its players only (NPCs sleep through AI activities on `lie`).
//! - **Slot.** The approval's; `None` for the slotless verb.
//! - **No ownership** in the request: an owned bed is the game's crime rule, a gate.
//! - **Co-op** (rule): the request carries its user; the host decides the skip.
//!
//! The seat default ignores the verb: no pose slot's verb is `sleep`.

use bevy_ecs::prelude::*;
use sf_interaction::{Tick, UserKey, VERB_SLEEP};
use sf_script_bevy::sf_script::Handle;
use sf_world::PlacementId;

use crate::{InteractionTick, InteractionUser, UseApproved};

/// One approved `sleep` use: the host opens its rest UI for `user` at `placement` (module docs).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RestRequested {
    /// [`InteractionTick`] when the default wrote it.
    pub tick: Tick,
    pub user: UserKey,
    /// What `Activated.actor` named for this user.
    pub actor: Handle,
    pub placement: PlacementId,
    /// The approval's slot; `None` for the slotless verb.
    pub slot: Option<u8>,
}

/// Where [`rest_default`] runs: `Update`, after `ActivationSystems::Default`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RestSystems;

/// Approved `sleep` uses into [`RestRequested`] (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn rest_default(
    mut approved: MessageReader<UseApproved>,
    users: Query<&InteractionUser>,
    tick: Res<InteractionTick>,
    mut out: MessageWriter<RestRequested>,
) {
    for a in approved.read().filter(|a| a.verb == Some(VERB_SLEEP)) {
        let Some(user) = users.iter().find(|u| u.actor == a.activation.actor) else { continue };
        out.write(RestRequested {
            tick: tick.0,
            user: user.user,
            actor: user.actor,
            placement: PlacementId(a.activation.target.0),
            slot: a.slot,
        });
    }
}
