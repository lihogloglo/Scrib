//! The host reports interaction noises to perception (`docs/specs/interaction-world-rules.md`
//! "Noise", decision "the host reports it", slice `IN18c`; UE `ReportNoiseEvent`): each
//! `sf_interaction_bevy` [`InteractionNoise`] (a door, a break, an impact or a thrown landing)
//! becomes one `sf_ai_bevy` [`NoiseEvent`], the vocabulary the player's footsteps already use
//! (`mw_walkers_bevy` `noise_emitters`, `AI8c2`). The shell never depends on the AI; this crate
//! is the one place that knows both.
//!
//! - **Fields** ([`to_noise_event`]): `at` (f32 world metres, the interaction frame) as a
//!   [`WorldPos`], `space`, `loudness_m` copied; `maker` → `source` ([`UserKey::Player`] →
//!   [`PerceptTarget::Player`], [`UserKey::Actor`] → [`PerceptTarget::Actor`], the same keys);
//!   `reveals_maker` → `reveals_source` (a thrown landing names its thrower without revealing
//!   it: a distraction, not blame); no `notify` (no interaction noise shares knowledge).
//! - **Tick**: the perception clock's ([`AiPerceptClock`]; 0 without one), the stamp every
//!   `NoiseEvent` carries, not the interaction tick (another clock).
//! - **Kinds** ([`InteractionNoiseKinds`]): `NoiseKind` records don't exist yet, so as the player
//!   table (`KIND_NAMES` 0..) the interaction kinds are a small table ([`KIND_NAMES`]) of
//!   [`NoiseKindId`]s from [`KIND_BASE`] on, disjoint from the player's. A material's noise tag
//!   (`MaterialDef::noise`, data) is [`InteractionNoiseKinds::register`]ed by whoever loads the
//!   materials; an unknown tag is `noise.object`. Removal: once `NoiseKind` records exist, the
//!   table resolves record ids instead.
//! - **Order**: `FixedUpdate`, after `NoiseSystems::Emit` and after
//!   [`AiPerceptionSystems::Hearing`]: heard on the next perception tick (one fixed tick later),
//!   which `hearing.rs` supports. Same tick would need the shell's door, destroy and impact steps
//!   before the hearing set, an order the physics step (before `ControllerSystems::Drive`, as
//!   hearing) does not promise; after both is deterministic and adds no edge between them.
//!   Runs only while both message types exist (an app without the AI or the shell skips it).

use std::collections::BTreeMap;

use bevy_ecs::message::Messages;
use bevy_ecs::prelude::*;
use sf_ai::{NoiseKindId, PerceptTarget};
use sf_ai_bevy::{AiPerceptClock, AiPerceptionSystems, NoiseEvent};
use sf_coords::WorldPos;
use sf_interaction::{TagId, UserKey};
use sf_interaction_bevy::{InteractionNoise, NOISE_BREAK, NOISE_DOOR, NoiseSystems};

/// The first [`NoiseKindId`] of the interaction kinds: after the player's table
/// (`mw_walkers_bevy` `KIND_NAMES`, from 0), until `NoiseKind` records exist.
// tunables: const (an id table, not a trade-off)
pub const KIND_BASE: u16 = 0x100;

/// The interaction kinds, in [`NoiseKindId`] order from [`KIND_BASE`]; the last is the kind of
/// an unknown tag.
// tunables: const (an id table, not a trade-off)
pub const KIND_NAMES: [&str; 3] = ["noise.door", "noise.break", "noise.object"];

/// The kind of an impact whose tag the table lacks.
pub const NOISE_OBJECT: TagId = TagId::of("noise.object");

/// Interaction noise tags → perception [`NoiseKindId`]s (module docs).
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct InteractionNoiseKinds {
    ids: BTreeMap<TagId, NoiseKindId>,
    next: u16,
}

impl Default for InteractionNoiseKinds {
    fn default() -> Self {
        let mut kinds = Self { ids: BTreeMap::new(), next: KIND_BASE };
        for name in KIND_NAMES {
            kinds.register(TagId::of(name));
        }
        debug_assert_eq!(kinds.ids.get(&NOISE_DOOR), Some(&NoiseKindId(KIND_BASE)));
        debug_assert_eq!(kinds.ids.get(&NOISE_BREAK), Some(&NoiseKindId(KIND_BASE + 1)));
        kinds
    }
}

impl InteractionNoiseKinds {
    /// The id of `tag`, given the next free one if it has none (a material's noise tag).
    pub fn register(&mut self, tag: TagId) -> NoiseKindId {
        if let Some(&id) = self.ids.get(&tag) {
            return id;
        }
        let id = NoiseKindId(self.next);
        self.next = self.next.saturating_add(1);
        self.ids.insert(tag, id);
        id
    }

    /// The id of `tag`; `noise.object`'s for a tag never registered.
    #[must_use]
    pub fn id(&self, tag: TagId) -> NoiseKindId {
        self.ids.get(&tag).or_else(|| self.ids.get(&NOISE_OBJECT)).copied().unwrap_or(NoiseKindId(KIND_BASE))
    }
}

/// Who made a noise, as perception names it (the same keys).
#[must_use]
pub fn percept_target(user: UserKey) -> PerceptTarget {
    match user {
        UserKey::Player(p) => PerceptTarget::Player(p),
        UserKey::Actor(a) => PerceptTarget::Actor(a),
    }
}

/// One interaction noise as a perception noise stamped `tick` (module docs).
#[must_use]
pub fn to_noise_event(noise: &InteractionNoise, kinds: &InteractionNoiseKinds, tick: u64) -> NoiseEvent {
    NoiseEvent {
        tick,
        at: WorldPos::from_f64(noise.at.as_dvec3()),
        space: noise.space.clone(),
        kind: kinds.id(noise.kind),
        loudness_m: noise.loudness_m,
        source: noise.maker.map(percept_target),
        reveals_source: noise.reveals_maker,
        notify: None,
    }
}

/// This tick's interaction noises, reported to perception.
fn report_noises(
    kinds: Res<InteractionNoiseKinds>,
    clock: Option<Res<AiPerceptClock>>,
    mut noises: MessageReader<InteractionNoise>,
    mut out: MessageWriter<NoiseEvent>,
) {
    let tick = clock.map_or(0, |c| c.tick);
    out.write_batch(noises.read().map(|n| to_noise_event(n, &kinds, tick)));
}

/// Adds the kinds table and the reporter (module docs).
pub fn add(app: &mut bevy_app::App) {
    app.init_resource::<InteractionNoiseKinds>().add_systems(
        bevy_app::FixedUpdate,
        report_noises
            .after(NoiseSystems::Emit)
            .after(AiPerceptionSystems::Hearing)
            .run_if(resource_exists::<Messages<InteractionNoise>>)
            .run_if(resource_exists::<Messages<NoiseEvent>>),
    );
}
