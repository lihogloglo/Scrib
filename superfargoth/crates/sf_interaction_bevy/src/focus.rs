//! Per-player focus (`docs/specs/interaction.md` "Point index and focus", activation step 1). Slice
//! `IN31b2`. The host's ray or cone query writes one [`FocusCandidates`] per player per fixed tick
//! (`.before(InteractionSystems::Focus)`); [`InteractionSystems::Focus`] (after `Apply`, so this
//! tick's claims count; before `Activate`) keeps each player's [`FocusState`] in [`PlayerFocus`]:
//!
//! - **Target**: a [`FocusStabilizer`] per player (live `interact.focus.switch_margin` and
//!   `switch_s`): the best candidate is focused at once; a challenger takes focus only after
//!   leading by the margin for the dwell. An empty candidate list clears the focus; a player
//!   with no message this tick keeps its state untouched (several messages: the last counts).
//! - **Verbs**: the target's offered verbs for this player, best first (the same check
//!   validation uses, so the prompt never offers what activation refuses). A target without a
//!   def offers none: the pack prompts its own class verb (talk, open) until IN32 gives it a def.
//! - **Hold**: while interact is held, the target **and its verb list** are frozen (no list
//!   reshuffling under the player's finger: another user taking the seat changes nothing until
//!   release); after `interact.focus.hold_s` of holding, `list_open` asks the prompt to list
//!   every verb.
//!
//! The host's input turns a state into the [`ActivateCommand`] it writes with
//! [`FocusState::command`] (best verb, or the hold-list choice), so it never re-derives the verb
//! or slot. This crate writes no commands: input is the host's. A host system ordered
//! `.after(InteractionSystems::Focus).before(InteractionSystems::Activate)` gets it validated in
//! the same tick.

use std::collections::BTreeMap;

use bevy_ecs::prelude::*;
use sf_interaction::{FocusStabilizer, TagId, UserKey, VerbChoice, VerbTarget};
use sf_script_bevy::sf_script::Handle;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::activate::offered_verbs;
use crate::{ActivateCommand, InteractionClaims, InteractionDefs, InteractionIndex, InteractionStateTags};

/// One player's focus query result for this fixed tick (the host's ray or cone).
#[derive(Message, Clone, Debug, PartialEq)]
pub struct FocusCandidates {
    pub player: UserKey,
    /// Targets in reach with their scores (higher is better; order free). Empty clears the focus.
    pub candidates: Vec<(PlacementId, f32)>,
    /// Whether interact is down this tick.
    pub held: bool,
    /// The player's claim priority (`InteractParams::priority_player`), for the offered verbs.
    pub priority: u8,
    /// The player's tags, for the slots' `users` filters.
    pub tags: SmallVec<[TagId; 4]>,
}

/// One player's focus, as the prompt and the host's input read it.
#[derive(Clone, Debug, PartialEq)]
pub struct FocusState {
    stabilizer: FocusStabilizer<PlacementId>,
    target: Option<PlacementId>,
    verbs: SmallVec<[VerbChoice; 4]>,
    list_open: bool,
    priority: u8,
    tags: SmallVec<[TagId; 4]>,
}

impl Default for FocusState {
    fn default() -> Self {
        Self {
            stabilizer: FocusStabilizer::new(),
            target: None,
            verbs: SmallVec::new(),
            list_open: false,
            // tunables: const (empty state; set every tick from FocusInput.priority = interact.claim.priority_player)
            priority: 0,
            tags: SmallVec::new(),
        }
    }
}

impl FocusState {
    /// The focused target.
    #[must_use]
    pub const fn target(&self) -> Option<PlacementId> {
        self.target
    }

    /// The target's offered verbs, best first (frozen while interact is held; empty for a
    /// def-less target).
    #[must_use]
    pub fn verbs(&self) -> &[VerbChoice] {
        &self.verbs
    }

    /// Interact held past `interact.focus.hold_s` on a target: the prompt lists every verb.
    #[must_use]
    pub const fn list_open(&self) -> bool {
        self.list_open
    }

    /// The command activating the focused target as `actor` (its script handle): the verb at
    /// `choice` in [`Self::verbs`] (the hold list), or the best one without a choice, with its
    /// slot; a def-less target's primary use (`verb: None`) without a choice. `None` without a
    /// target, with no verb offered, or with a choice out of range.
    #[must_use]
    pub fn command(&self, player: UserKey, actor: Handle, choice: Option<usize>) -> Option<ActivateCommand> {
        let target = self.target?;
        let (verb, slot) = if self.verbs.is_empty() && choice.is_none() {
            (None, None)
        } else {
            let c = self.verbs.get(choice.unwrap_or(0))?;
            let slot = match c.target {
                VerbTarget::Slot(r) => Some(r.slot),
                VerbTarget::Slotless { .. } => None,
            };
            (Some(c.verb), slot)
        };
        Some(ActivateCommand { user: player, actor, target, verb, slot, priority: self.priority, tags: self.tags.clone() })
    }
}

/// Every player's focus (co-op: one per player), by player.
#[derive(Resource, Clone, Debug, Default)]
pub struct PlayerFocus(BTreeMap<UserKey, FocusState>);

impl PlayerFocus {
    /// The player's focus, once a [`FocusCandidates`] named it.
    #[must_use]
    pub fn get(&self, player: UserKey) -> Option<&FocusState> {
        self.0.get(&player)
    }

    /// Every player's focus, by player.
    pub fn iter(&self) -> impl Iterator<Item = (UserKey, &FocusState)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }

    /// Forgets the player (it left the session).
    pub fn remove(&mut self, player: UserKey) {
        self.0.remove(&player);
    }
}

/// One fixed tick of every player's focus (module docs), by player.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn focus(
    mut focus: ResMut<PlayerFocus>,
    mut candidates_in: MessageReader<FocusCandidates>,
    defs: Res<InteractionDefs>,
    state: Res<InteractionStateTags>,
    claims: Res<InteractionClaims>,
    index: Res<InteractionIndex>,
) {
    // The last message per player counts; the map keeps them sorted by player.
    let mut latest: BTreeMap<UserKey, &FocusCandidates> = BTreeMap::new();
    for m in candidates_in.read() {
        latest.insert(m.player, m);
    }
    let params = *claims.0.params();
    for (player, m) in latest {
        let f = focus.0.entry(player).or_default();
        let before = f.target;
        if m.candidates.is_empty() {
            f.stabilizer.clear();
        }
        let target = f.stabilizer.update(&m.candidates, m.held, &params);
        let frozen = m.held && target.is_some() && target == before;
        if !frozen {
            f.verbs = target
                .and_then(|t| offered_verbs(player, &m.tags, m.priority, t, &defs, &state, &claims, &index))
                .unwrap_or_default();
            f.priority = m.priority;
            f.tags.clone_from(&m.tags);
        }
        f.target = target;
        f.list_open = target.is_some() && f.stabilizer.list_open(&params);
    }
}
