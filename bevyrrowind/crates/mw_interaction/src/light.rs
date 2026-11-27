//! `LIGH` records to light defs (interaction-adapter.md "Lights", slice `IN19c`): which lights get
//! a `Light` slot, its [`LightDef`] and the def's verbs. Pure: the game shell reads the record into
//! [`LightFacts`] and the placement's flame point, and indexes the def.
//!
//! - **Which lights.** Only fire lights (`FIRE`) can be snuffed: candles, lanterns, braziers, torches
//!   on walls; glow and magic lights get no def. A `NEGATIVE` light or one of radius 0 gives no
//!   gameplay light (as `mw_walkers_bevy::light_record`). An `OFF_BY_DEFAULT` light spawns with no
//!   render light at all (`mw_world::light_info`), so relighting it would light nothing: it gets no
//!   def until the host can spawn its render light (a later slice); every def'd light starts lit.
//! - **Def.** One [`SlotKind::Light`] slot ([`LIGHT_SLOT`]) at the flame point whose verb is
//!   [`VERB_SNUFF`] (requires [`LIT`]), plus the slotless [`VERB_RELIGHT`] (requires no [`LIT`]), both
//!   at [`CLASS_VERB_PRIORITY`]; the slot takes one user at a time (two NPCs never relight one
//!   lamp). A carryable light (`CAN_CARRY`) keeps "Take" as its press: a slotless `take` at
//!   [`TAKE_PRIORITY`], above the light verbs, so snuff / relight are on its hold list.
//! - **Switch off** (`mw.rules.interaction.snuff`, Classic): the same def without the light verbs
//!   (a carryable keeps `take`); the shell disables the slot, so the light is still registered (the
//!   AI's `GameplayLights`) but offers nothing, as in vanilla.
//! - **Intensity** is [`LIGHT_INTENSITY`], a relative weight of 1: the AI's light inputs scale it by
//!   their own `mw.ai.light.lamp_intensity` when they copy `GameplayLights`.

use glam::{Quat, Vec3};
use sf_interaction::{
    ClipFamily, Frame, IkTargets, InteractableDef, LIT, LightDef, Occupancy, SlotDef, SlotKind, TagFilter, TagId,
    VERB_RELIGHT, VERB_SNUFF, VerbDef,
};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

use crate::activation::verb;
use crate::defs::CLASS_VERB_PRIORITY;

/// The index of a light def's `Light` slot.
pub const LIGHT_SLOT: u8 = 0;

/// The relative intensity of a def'd light (module docs, "Intensity").
// tunables: const (a unit weight: the AI's `mw.ai.light.lamp_intensity` is the tunable that scales it)
pub const LIGHT_INTENSITY: f32 = 1.0;

/// A carryable light's slotless `take`: above the light verbs, so its press stays "Take".
pub const TAKE_PRIORITY: i16 = CLASS_VERB_PRIORITY + 1;

/// What the adapter reads from a `LIGH` record (its flags, one bool each).
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightFacts {
    /// The record's radius, metres.
    pub radius_m: f32,
    /// `FIRE`: a flame (candle, lantern, brazier, torch).
    pub fire: bool,
    /// `CAN_CARRY`.
    pub carryable: bool,
    /// `NEGATIVE`: darkens.
    pub negative: bool,
    /// `OFF_BY_DEFAULT`.
    pub off_by_default: bool,
}

/// The [`LightDef`] of a light the player can snuff and relight (module docs, "Which lights"),
/// `None` for every other light.
#[must_use]
pub fn light_def(f: &LightFacts) -> Option<LightDef> {
    (f.fire && !f.negative && !f.off_by_default && f.radius_m > 0.0).then_some(LightDef {
        radius_m: f.radius_m,
        intensity: LIGHT_INTENSITY,
        lit_default: true,
    })
}

/// The def of light record `base` (module docs, "Def"): its `Light` slot at `flame` (the object's
/// frame, metres), its verbs when `snuff` is on (`mw.rules.interaction.snuff`); `None` when
/// [`light_def`] gives none.
#[must_use]
pub fn light_interactable(base: &RecordId, f: &LightFacts, flame: Vec3, snuff: bool) -> Option<InteractableDef> {
    let light = light_def(f)?;
    let slot = SlotDef {
        kind: SlotKind::Light(light),
        local: Frame { translation: flame, rotation: Quat::IDENTITY },
        band: flame.y.max(0.0),
        verb: VerbDef {
            verb: VERB_SNUFF,
            priority: CLASS_VERB_PRIORITY,
            requires: TagFilter { all: smallvec![LIT], ..TagFilter::default() },
        },
        entrances: SmallVec::new(),
        clips: ClipFamily::default(),
        ik: IkTargets::default(),
        reach: 0.0,
        occupancy: Occupancy::Pose,
        control: None,
        users: TagFilter::default(),
        group: None,
        queue: None,
    };
    let mut verbs = SmallVec::new();
    if f.carryable {
        verbs.push(VerbDef { verb: TagId::of(verb::TAKE), priority: TAKE_PRIORITY, requires: TagFilter::default() });
    }
    if snuff {
        verbs.push(VerbDef {
            verb: VERB_RELIGHT,
            priority: CLASS_VERB_PRIORITY,
            requires: TagFilter { none: smallvec![LIT], ..TagFilter::default() },
        });
    }
    Some(InteractableDef {
        id: base.clone(),
        slots: vec![slot],
        verbs,
        material: None,
        destructible: None,
        carry: None,
        adverts: Vec::new(),
    })
}
