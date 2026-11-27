//! Light as state (`docs/specs/interaction-world-rules.md` "Light as state (IN19)"), slice
//! `IN19b`: the resident light-bearing objects, their restored rest state, the default of their
//! snuff / relight use, and [`GameplayLights`], the AI's light list. The state is the core's
//! [`LightState`]. This crate spawns no render light: the host's light hook follows
//! [`LightChanged`].
//!
//! - **Registry** ([`Lights`], `FixedUpdate`, in `InteractionSystems::Apply` after the door
//!   registry, so the same tick's validation sees [`LIT`]): every [`IndexChange::Insert`] whose def
//!   has a [`SlotKind::Light`] slot (the first) registers the placement with its def, world frame,
//!   the slot's world position and its [`LightDef`]. A first insert starts in
//!   [`LightState::initial`] (`lit_default`); a re-insert keeps the state. A re-insert without a
//!   light slot, or a removal, drops it. A move refreshes its frame and position. Changes apply in
//!   send order.
//! - **Rest** (same system, after all of the tick's index changes, in send order): a [`LightRest`]
//!   puts a registered light in its saved state (a restored save or a streamed-in cell). Not a use,
//!   so no [`LightChanged`]: the host that wrote it sets its own light to match. One for a
//!   placement that is not a registered light is dropped.
//! - **Default** (`Update`, [`LightSystems`], after `ActivationSystems::Default`): a [`UseApproved`]
//!   on a registered light with verb [`VERB_SNUFF`] or [`VERB_RELIGHT`] goes to
//!   [`LightState::use_verb`]; no verb (a script's `Activate`) toggles; any other verb (take, carry)
//!   is not this default's. The culprit is the [`InteractionUser`] whose actor made the activation
//!   (none: a script). A refusal (snuff on an unlit light that passed no `requires` gate, e.g. a
//!   script's, or the second of two same-frame uses) changes nothing and writes nothing.
//! - **Gate.** The adapter's verbs carry the state test in `requires` (snuff: `all: [LIT]`, relight:
//!   `none: [LIT]`), so validation already refuses a player's or an NPC's snuff on an unlit light.
//!
//! Every state change (registration, a use, a rest) keeps, in the same system: the [`LIT`] tag in
//! [`InteractionStateTags`] and the placement's [`GameplayLights`] entry. Each use that changes the
//! state writes one [`LightChanged`].
//!
//! **Space.** The index holds the resident objects of the active space (the host's ring around
//! the player, unloaded on a space change), world metres; so [`GameplayLight`] carries no space,
//! and the host copies every entry into its AI light inputs (spec "AI light, never from the GPU").

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_interaction::{
    Frame, InteractableDef, LIT, LightDef, LightState, SlotDef, SlotKind, TagId, VERB_RELIGHT, VERB_SNUFF, compose,
};
use sf_world::PlacementId;

use crate::{IndexChange, InteractionStateTags, InteractionTick, InteractionUser, UseApproved};

/// One resident light-bearing object.
#[derive(Clone, Debug)]
pub struct Light {
    pub def: Arc<InteractableDef>,
    /// The object's world frame, as last inserted or moved.
    pub object: Frame,
    /// The light slot's index in the def.
    pub slot: u8,
    pub light: LightDef,
    /// The light slot's world position (the flame).
    pub at: Vec3,
    pub state: LightState,
}

impl Light {
    /// The AI's view of this light.
    #[must_use]
    pub fn gameplay(&self, placement: PlacementId) -> GameplayLight {
        GameplayLight {
            placement,
            pos: self.at,
            radius_m: self.light.radius_m,
            intensity: self.light.intensity,
            lit: self.state.lit,
        }
    }
}

/// The resident light-bearing objects, by placement.
#[derive(Resource, Clone, Debug, Default)]
pub struct Lights(BTreeMap<PlacementId, Light>);

impl Lights {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&Light> {
        self.0.get(&placement)
    }

    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &Light)> {
        self.0.iter().map(|(p, l)| (*p, l))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One gameplay light for the AI's light exposure: the fields of `sf_ai::LightSource` (`pos`,
/// `radius_m`, `intensity`, `on`), in this crate's own type so it never names `sf_ai`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameplayLight {
    pub placement: PlacementId,
    /// World metres.
    pub pos: Vec3,
    /// Reach: the contribution falls to 0 here, meters.
    pub radius_m: f32,
    /// Contribution at the centre, 0..1 typical.
    pub intensity: f32,
    pub lit: bool,
}

/// The gameplay lights of the resident objects, by placement (module docs): the host copies them
/// into its AI light inputs in place of a renderer light list.
#[derive(Resource, Clone, Debug, Default)]
pub struct GameplayLights(BTreeMap<PlacementId, GameplayLight>);

impl GameplayLights {
    #[must_use]
    pub fn get(&self, placement: PlacementId) -> Option<&GameplayLight> {
        self.0.get(&placement)
    }

    /// In placement order.
    pub fn iter(&self) -> impl Iterator<Item = &GameplayLight> {
        self.0.values()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// One change of one light (the core's event) and, for a rule's change, its cause tag (`None` for a
/// use): the host's light hook, AI (`Darkened`, dusk lamps), sound and the replay log read it.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct LightChanged(pub sf_interaction::LightChanged, pub Option<TagId>);

/// Puts a registered light in this saved state: a restored save or a streamed-in cell. Not a use,
/// so no [`LightChanged`].
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LightRest {
    pub placement: PlacementId,
    pub lit: bool,
}

/// Where [`light_default`] runs: `Update`, after `ActivationSystems::Default`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightSystems;

/// The def's first light slot.
fn light_slot(def: &InteractableDef) -> Option<(u8, &SlotDef, LightDef)> {
    def.slots.iter().enumerate().find_map(|(i, s)| match &s.kind {
        SlotKind::Light(l) => Some((u8::try_from(i).ok()?, s, *l)),
        _ => None,
    })
}

/// The tag and the AI entry follow the light's state.
fn sync(placement: PlacementId, light: &Light, tags: &mut InteractionStateTags, gameplay: &mut GameplayLights) {
    if light.state.lit {
        tags.insert(placement, LIT);
    } else {
        tags.remove(placement, LIT);
    }
    gameplay.0.insert(placement, light.gameplay(placement));
}

/// A light left: its tag and AI entry go with it.
fn drop_light(
    placement: PlacementId,
    lights: &mut Lights,
    tags: &mut InteractionStateTags,
    gameplay: &mut GameplayLights,
) {
    if lights.0.remove(&placement).is_some() {
        tags.remove(placement, LIT);
        gameplay.0.remove(&placement);
    }
}

/// A rule sets a registered light (`IN39a2`, [`crate::rules`]): its state, tag and AI entry; the
/// change, if it was one (an unregistered placement or the same state: none).
pub(crate) fn set_lit(
    placement: PlacementId,
    lit: bool,
    culprit: Option<sf_interaction::UserKey>,
    tick: sf_interaction::Tick,
    lights: &mut Lights,
    tags: &mut InteractionStateTags,
    gameplay: &mut GameplayLights,
) -> Option<sf_interaction::LightChanged> {
    let light = lights.0.get_mut(&placement)?;
    let changed = light.state.set(lit, placement, light.at, culprit, tick)?;
    sync(placement, light, tags, gameplay);
    Some(changed)
}

/// The registry follows the tick's index changes, then its rest states (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn register(
    mut changes: MessageReader<IndexChange>,
    mut rests: MessageReader<LightRest>,
    mut lights: ResMut<Lights>,
    mut tags: ResMut<InteractionStateTags>,
    mut gameplay: ResMut<GameplayLights>,
) {
    for change in changes.read() {
        match change {
            IndexChange::Insert { placement, def, object, .. } => {
                let Some((slot, slot_def, light_def)) = light_slot(def) else {
                    drop_light(*placement, &mut lights, &mut tags, &mut gameplay);
                    continue;
                };
                let state = lights.0.get(placement).map_or_else(|| LightState::initial(&light_def), |l| l.state);
                let at = compose(object, &slot_def.local).translation;
                let light = Light { def: Arc::clone(def), object: *object, slot, light: light_def, at, state };
                sync(*placement, &light, &mut tags, &mut gameplay);
                lights.0.insert(*placement, light);
            }
            IndexChange::Move { placement, object } => {
                let Some(light) = lights.0.get_mut(placement) else { continue };
                let Some(slot_def) = light.def.slots.get(usize::from(light.slot)) else { continue };
                light.at = compose(object, &slot_def.local).translation;
                light.object = *object;
                sync(*placement, light, &mut tags, &mut gameplay);
            }
            IndexChange::Remove { placement, .. } => drop_light(*placement, &mut lights, &mut tags, &mut gameplay),
        }
    }
    for rest in rests.read() {
        let Some(light) = lights.0.get_mut(&rest.placement) else { continue };
        light.state = LightState { lit: rest.lit };
        sync(rest.placement, light, &mut tags, &mut gameplay);
    }
}

/// The light verb an approved `verb` names for a light that is `lit`: no verb toggles; `None` for a
/// verb that is not a light verb.
fn light_verb(verb: Option<TagId>, lit: bool) -> Option<TagId> {
    match verb {
        None => Some(if lit { VERB_SNUFF } else { VERB_RELIGHT }),
        Some(v) if v == VERB_SNUFF || v == VERB_RELIGHT => Some(v),
        Some(_) => None,
    }
}

/// Approved snuff / relight uses on registered lights into state changes (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn light_default(
    mut approved: MessageReader<UseApproved>,
    users: Query<&InteractionUser>,
    tick: Res<InteractionTick>,
    mut lights: ResMut<Lights>,
    mut tags: ResMut<InteractionStateTags>,
    mut gameplay: ResMut<GameplayLights>,
    mut out: MessageWriter<LightChanged>,
) {
    for a in approved.read() {
        let placement = PlacementId(a.activation.target.0);
        let Some(light) = lights.0.get_mut(&placement) else { continue };
        let Some(verb) = light_verb(a.verb, light.state.lit) else { continue };
        let culprit = users.iter().find(|u| u.actor == a.activation.actor).map(|u| u.user);
        let Ok(changed) = light.state.use_verb(verb, placement, light.at, culprit, tick.0) else { continue };
        sync(placement, light, &mut tags, &mut gameplay);
        out.write(LightChanged(changed, None));
    }
}
