//! The activation pipeline (`docs/specs/interaction.md` "Activation contract", steps 1-7). Slice
//! `IN31b1`. Every use, from player input, an NPC's `UseSlot` or a script, takes one path:
//!
//! 1. **Command** ([`ActivateCommand`], `FixedUpdate`, written `.before(InteractionSystems::Activate)`):
//!    the user, its script handle, the target placement, the verb (`None` = the target's primary
//!    use), optionally the slot, and the claim priority and user tags the claim checks use.
//! 2. **Validate** ([`InteractionSystems::Activate`], after `Apply`, so this tick's claims count):
//!    a target **with a def** ([`InteractionDefs`]: kept from `IndexChange::Insert`, dropped on
//!    `Remove`, so having one means resident) passes when the verb (and slot, if given) is among
//!    [`rank_verbs`] for this user: the target's [`InteractionStateTags`] meet the verb's
//!    `requires`, and its slot is either already this user's claim or one
//!    [`Claims::would_claim`] grants at the command's priority. `verb: None` takes the best offered
//!    verb, and the activation names it. A target **without a def** passes only with `verb: None`
//!    and no slot (the pack's classes: talk, containers, books, until IN32 gives them defs).
//!    **Reach is the host's check**, before it writes the command: the player's focus range, the
//!    NPC's plan (it stands at the slot's entrance); this crate knows no positions for it.
//!    A refused command writes nothing.
//! 3. **Event**: an accepted command writes `sf_script_bevy::Activated { target, actor, verb }`
//!    (`target` = the placement's id as a script handle), sorted by actor then target.
//! 4. **Scripts** (`ScriptSlot::Update`) answer every `Activated` with an `ActivationOutcome`; a
//!    handler returning `"handled"` suppresses everything below.
//! 5. **Gates** ([`ActivationSystems`], `Update`, after the script slot): [`ActivationSystems::Collect`]
//!    gathers every unhandled `ActivationOutcome` (whoever wrote the `Activated`, so the pack's
//!    direct writers pass the gates too) and every [`ScriptActivate`] into [`PendingUses`], sorted
//!    by actor, then target, then verb. Gate systems from other crates run
//!    `.in_set(ActivationSystems::Gates)` and call [`PendingUse::refuse`] (through [`PendingUses::uses_mut`]) with a reason tag (the
//!    first refusal wins; gates order among themselves with plain Bevy ordering).
//! 6. **Default** ([`ActivationSystems::Default`]): each pending use becomes one [`UseApproved`]
//!    (what the verb's default handler reads, `.after(ActivationSystems::Default)`) or one
//!    [`UseRefused`] (its reason, for the prompt), and the list empties. A use validated in step 2
//!    carries the slot its verb acts through (`IN2b2`: kept beside the `Activated` by actor, target
//!    and verb, since the scripts' answer has no slot); a script's `Activate` carries none.
//! 7. **Script `Activate`** ([`ScriptActivate`]) enters at step 5: gates, then default, never the
//!    scripts again, so a script cannot recurse into itself.
//!
//! **No escalation** (owner 2026-11-26, ai.md "Drives", "No escalation ladder"): a refused or
//! failed use never turns into another verb (a locked door never becomes "bash", a taken seat
//! never "take"). The refusal goes back to the actor and the prompt, and nothing else happens.

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use sf_interaction::{InteractableDef, SlotTerms, TagId, UserKey, VerbChoice, VerbTarget, rank_verbs};
use sf_script_bevy::sf_script::Handle;
use sf_script_bevy::{Activated, ActivationOutcome};
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::{InteractionClaims, InteractionIndex};

// --- Resources ----------------------------------------------------------------------------------

/// The def of each indexed placement (from `IndexChange::Insert`, dropped on `Remove`): what
/// [`rank_verbs`] ranks.
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionDefs(pub(crate) BTreeMap<PlacementId, Arc<InteractableDef>>);

impl InteractionDefs {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&Arc<InteractableDef>> {
        self.0.get(&placement)
    }
}

/// Per-placement state tags (`Lit`, `Locked`) the verbs' `requires` test; empty until a system
/// fills them (lights, crime's locks).
#[derive(Resource, Clone, Debug, Default)]
pub struct InteractionStateTags(BTreeMap<PlacementId, SmallVec<[TagId; 4]>>);

impl InteractionStateTags {
    /// The placement's tags (none when unset).
    #[must_use]
    pub fn tags(&self, placement: PlacementId) -> &[TagId] {
        self.0.get(&placement).map_or(&[], |t| t.as_slice())
    }

    /// Adds `tag` (once).
    pub fn insert(&mut self, placement: PlacementId, tag: TagId) {
        let tags = self.0.entry(placement).or_default();
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }

    /// Removes `tag`; the placement's entry goes with its last tag.
    pub fn remove(&mut self, placement: PlacementId, tag: TagId) {
        if let Some(tags) = self.0.get_mut(&placement) {
            tags.retain(|t| *t != tag);
            if tags.is_empty() {
                self.0.remove(&placement);
            }
        }
    }

    /// Replaces the placement's tags.
    pub fn set(&mut self, placement: PlacementId, tags: impl IntoIterator<Item = TagId>) {
        let tags: SmallVec<[TagId; 4]> = tags.into_iter().collect();
        if tags.is_empty() {
            self.0.remove(&placement);
        } else {
            self.0.insert(placement, tags);
        }
    }
}

/// One use between the scripts and the default (step 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingUse {
    pub activation: Activated,
    pub verb: Option<TagId>,
    /// The slot the verb acts through, when step 2 chose one (`None` for a script's `Activate`).
    pub slot: Option<u8>,
    /// Entered from a script's `Activate` (step 7), not from the scripts' answer.
    pub from_script: bool,
    refused: Option<TagId>,
}

impl PendingUse {
    /// Refuses the use with `reason`; an earlier refusal stands.
    pub fn refuse(&mut self, reason: TagId) {
        self.refused.get_or_insert(reason);
    }

    /// The refusal, if a gate gave one.
    #[must_use]
    pub const fn refused(&self) -> Option<TagId> {
        self.refused
    }
}

/// This frame's uses awaiting the gates (filled in [`ActivationSystems::Collect`], emptied in
/// [`ActivationSystems::Default`]).
#[derive(Resource, Clone, Debug, Default)]
pub struct PendingUses(Vec<PendingUse>);

impl PendingUses {
    #[must_use]
    pub fn uses(&self) -> &[PendingUse] {
        &self.0
    }

    /// For gates: each pending use, to [`PendingUse::refuse`].
    pub fn uses_mut(&mut self) -> &mut [PendingUse] {
        &mut self.0
    }
}

/// The slot each of this fixed tick's accepted activations acts through, by (actor, target, verb):
/// written in step 2, taken in step 5 (the same frame: the scripts answer in `ScriptSlot::Update`).
#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct ActivationSlots(BTreeMap<(Handle, Handle, Option<u32>), u8>);

// --- Messages -----------------------------------------------------------------------------------

/// Step 1: a user asks to use `target` (player input on its focus, an NPC's `UseSlot` step).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ActivateCommand {
    pub user: UserKey,
    /// The user as scripts see it (`sf_script_bevy::PLAYER` for the player).
    pub actor: Handle,
    pub target: PlacementId,
    /// `None` = the target's primary use.
    pub verb: Option<TagId>,
    /// The slot the verb acts through, when the user picked one.
    pub slot: Option<u8>,
    /// Claim priority the offered-verb check uses (`InteractParams::priority_*`).
    pub priority: u8,
    /// The user's tags, for the slots' `users` filters.
    pub tags: SmallVec<[TagId; 4]>,
}

/// Step 7: a script's `Activate` on `target`: gates and default, never the scripts.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScriptActivate {
    pub target: Handle,
    pub actor: Handle,
    pub verb: Option<TagId>,
}

/// Step 6: the use passed scripts and gates; the verb's default handler runs it.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UseApproved {
    pub activation: Activated,
    pub verb: Option<TagId>,
    /// The slot the verb acts through, when validation chose one (a script's `Activate`: `None`,
    /// the verb's default picks).
    pub slot: Option<u8>,
}

/// Step 6: a gate refused the use; `reason` goes to the prompt. Nothing else happens.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UseRefused {
    pub activation: Activated,
    pub verb: Option<TagId>,
    pub reason: TagId,
}

/// The `Update` chain after `ScriptSlot::Update` (module docs, steps 5-6).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActivationSystems {
    Collect,
    Gates,
    Default,
}

// --- Systems ------------------------------------------------------------------------------------

/// The verbs `target` offers `user` now, best first ([`rank_verbs`]: the target's state tags meet
/// the verb's `requires`, and its slot is the user's claim already or one [`Claims::would_claim`]
/// grants at `priority` for `tags`); `None` when the target has no def. Shared by validation
/// (step 2) and the player's focus ([`crate::focus`]), so the prompt offers what validation accepts.
///
/// [`Claims::would_claim`]: sf_interaction::Claims::would_claim
#[allow(clippy::too_many_arguments)]
pub(crate) fn offered_verbs(
    user: UserKey,
    tags: &[TagId],
    priority: u8,
    target: PlacementId,
    defs: &InteractionDefs,
    state: &InteractionStateTags,
    claims: &InteractionClaims,
    index: &InteractionIndex,
) -> Option<SmallVec<[VerbChoice; 4]>> {
    let def = defs.get(target)?;
    let claims = &claims.0;
    let offered = |r: sf_interaction::SlotRef| {
        if claims.claim_on(r).is_some_and(|c| c.user == user) {
            return true;
        }
        let Some(slot) = def.slots.get(usize::from(r.slot)) else { return false };
        let mut terms = SlotTerms::of(slot, claims.params());
        terms.enabled = index.0.get(r).is_some();
        claims.would_claim(r, &terms, user, tags, priority).is_ok()
    };
    Some(rank_verbs(def, target, state.tags(target), offered))
}

/// The activation `cmd` gives and the slot its verb acts through, `None` when refused (step 2).
fn validate(
    cmd: &ActivateCommand,
    defs: &InteractionDefs,
    state: &InteractionStateTags,
    claims: &InteractionClaims,
    index: &InteractionIndex,
) -> Option<(Activated, Option<u8>)> {
    let activation = |verb: Option<TagId>| Activated { target: Handle(cmd.target.0), actor: cmd.actor, verb: verb.map(|v| v.0) };
    let Some(verbs) = offered_verbs(cmd.user, &cmd.tags, cmd.priority, cmd.target, defs, state, claims, index) else {
        return (cmd.verb.is_none() && cmd.slot.is_none()).then(|| (activation(None), None));
    };
    verbs
        .into_iter()
        .find(|c| {
            cmd.verb.is_none_or(|v| c.verb == v)
                && cmd.slot.is_none_or(|s| matches!(c.target, VerbTarget::Slot(r) if r.slot == s))
        })
        .map(|c| {
            let slot = match c.target {
                VerbTarget::Slot(r) => Some(r.slot),
                VerbTarget::Slotless { .. } => None,
            };
            (activation(Some(c.verb)), slot)
        })
}

/// Step 2-3: validates this tick's commands and writes `Activated` for the accepted ones.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn activate(
    mut commands_in: MessageReader<ActivateCommand>,
    defs: Res<InteractionDefs>,
    state: Res<InteractionStateTags>,
    claims: Res<InteractionClaims>,
    index: Res<InteractionIndex>,
    mut slots: ResMut<ActivationSlots>,
    mut out: MessageWriter<Activated>,
) {
    let mut accepted: Vec<(Activated, Option<u8>)> =
        commands_in.read().filter_map(|cmd| validate(cmd, &defs, &state, &claims, &index)).collect();
    accepted.sort_by_key(|(a, _)| (a.actor, a.target, a.verb));
    for (a, slot) in &accepted {
        if let Some(slot) = slot {
            slots.0.insert((a.actor, a.target, a.verb), *slot);
        }
    }
    out.write_batch(accepted.into_iter().map(|(a, _)| a));
}

/// Step 5: unhandled uses and script `Activate`s, in order.
pub(crate) fn collect(
    mut pending: ResMut<PendingUses>,
    mut slots: ResMut<ActivationSlots>,
    mut outcomes: MessageReader<ActivationOutcome>,
    mut script: MessageReader<ScriptActivate>,
) {
    pending.0.clear();
    let slots = std::mem::take(&mut slots.0);
    let unhandled = outcomes.read().filter(|o| !o.handled).map(|o| PendingUse {
        activation: o.activation,
        verb: o.activation.verb.map(TagId),
        slot: slots.get(&(o.activation.actor, o.activation.target, o.activation.verb)).copied(),
        from_script: false,
        refused: None,
    });
    let scripted = script.read().map(|s| PendingUse {
        activation: Activated { target: s.target, actor: s.actor, verb: s.verb.map(|v| v.0) },
        verb: s.verb,
        slot: None,
        from_script: true,
        refused: None,
    });
    pending.0.extend(unhandled.chain(scripted));
    pending.0.sort_by_key(|u| (u.activation.actor, u.activation.target, u.verb));
}

/// Step 6: approvals and refusals out; the list empties.
pub(crate) fn finish(
    mut pending: ResMut<PendingUses>,
    mut approved: MessageWriter<UseApproved>,
    mut refused: MessageWriter<UseRefused>,
) {
    for u in pending.0.drain(..) {
        match u.refused {
            None => {
                approved.write(UseApproved { activation: u.activation, verb: u.verb, slot: u.slot });
            }
            Some(reason) => {
                refused.write(UseRefused { activation: u.activation, verb: u.verb, reason });
            }
        }
    }
}
