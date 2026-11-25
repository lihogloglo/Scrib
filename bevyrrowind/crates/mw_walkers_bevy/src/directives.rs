//! Record AI packages on the directive stack (`docs/specs/ai-adapter.md` "Packages ->
//! directives", slice `MA2d1`): when a walker gets its [`AiActor`], its base record's `AI_W`,
//! `AI_T`, `AI_E`, `AI_F` and `AI_A` packages ([`mw_ai::packages::directives`]) are pushed, in
//! record order, through [`MorrowindPolicy`] onto a fresh [`DirectiveStack`] in [`AiDirectives`]
//! (`OpenMW` `AiSequence::fill`: appended, so the top is the first record package).
//!
//! - **A stack already in the store wins** (a save's `sf/ai` section, or this actor's earlier
//!   spawn: tier 0 keeps every stack across cell unloads), so a respawn never re-seeds it.
//! - **Every actor gets a stack**, empty when its record has no packages: the key's presence
//!   means "seeded".
//! - **Resolver:** `AI_E`/`AI_F` targets resolve to a spawned character placement with that base
//!   record (lowest placement id first); `player` and `AI_A` objects resolve to nothing yet (no
//!   player actor key, no placement index in this crate). What doesn't resolve is skipped and
//!   counted in one `info!` per spawn batch.
//! - **Extras** (wander idle table, follow end point) ride on the actor as [`AiPackageExtras`],
//!   by directive id, with the [`crate::directive_drive::DirectiveDrive`] that serves the top
//!   (`MA2d2`).

use std::collections::BTreeMap;
use std::mem::discriminant;

use bevy_ecs::prelude::*;
use bevy_log::{debug, info};
use mw_ai::MorrowindPolicy;
use mw_ai::packages::{PackageCtx, PackageDirectives, PackageExtra, PackageResolver, directives};
use sf_ai::{DirectiveId, DirectiveSource, DirectiveStack, GameTime};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_character_bevy::{Character, CharacterPlacement};
use sf_controller_bevy::Mover;
use sf_ids::ActorKey;
use sf_records::RecordId;
use sf_sky_bevy::GameClock;
use sf_world::{Location, PlacementId};
use tes3::esp::{AiPackage, Creature, Npc};

use crate::walkers::{ActorSpace, WalkerContent};

/// Data of the actor's record packages a directive has no room for, by the directive it rides
/// with (the wander idle table, follow's end point). Directives without extra data are absent.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct AiPackageExtras(pub Vec<(DirectiveId, PackageExtra)>);

impl AiPackageExtras {
    pub fn get(&self, id: DirectiveId) -> Option<&PackageExtra> {
        self.0.iter().find(|(d, _)| *d == id).map(|(_, e)| e)
    }
}

/// Spawned character placements by base record (lowest placement id per record).
pub(crate) struct SpawnedActors(BTreeMap<RecordId, ActorKey>);

impl SpawnedActors {
    pub(crate) fn of(placed: &Query<(&Character, &CharacterPlacement)>) -> Self {
        let mut by_record = BTreeMap::new();
        for (character, placement) in placed {
            if let Character::Record { id, .. } = character {
                let key = ActorKey(placement.0.0);
                by_record.entry(id.clone()).and_modify(|k: &mut ActorKey| *k = (*k).min(key)).or_insert(key);
            }
        }
        Self(by_record)
    }
}

impl PackageResolver for SpawnedActors {
    fn actor(&self, id: &str) -> Option<ActorKey> {
        self.0.get(&RecordId::new(id)).copied()
    }

    fn placement(&self, _id: &str) -> Option<PlacementId> {
        None
    }
}

/// The record packages of `id` (a leveled list: the creature it shows, as `setup_of` picks it)
/// and the record id they belong to.
fn packages_of<'a>(content: &'a WalkerContent, id: &RecordId) -> (RecordId, &'a [AiPackage]) {
    let picked = mw_actors::leveled_pick(&content.db, id).map_or_else(|| id.clone(), |(_, p)| p);
    let packages = content
        .db
        .get::<Npc>(&picked)
        .map(|n| n.ai_packages.as_slice())
        .or_else(|| content.db.get::<Creature>(&picked).map(|c| c.ai_packages.as_slice()))
        .unwrap_or_default();
    (picked, packages)
}

/// Pushes `mapped` in record order onto a fresh stack of `key`; returns it and the extras.
pub fn seed_stack(key: ActorKey, now: GameTime, mapped: PackageDirectives) -> (DirectiveStack, AiPackageExtras) {
    let mut stack = DirectiveStack::new(key, now);
    let mut events = Vec::new();
    let mut extras = Vec::new();
    for d in mapped.directives {
        let id = stack.push(d.spec, now, &MorrowindPolicy, &mut events);
        if d.extra != PackageExtra::None {
            extras.push((id, d.extra));
        }
    }
    (stack, AiPackageExtras(extras))
}

/// The extras of a stack seeded earlier (a save, a respawn): each record directive still on it
/// takes the next unused record package of the same kind, in order.
pub fn extras_of(stack: &DirectiveStack, mapped: PackageDirectives) -> AiPackageExtras {
    let mut left: Vec<_> = mapped.directives.into_iter().map(Some).collect();
    let mut extras = Vec::new();
    for d in stack.iter().filter(|d| d.spec.source == DirectiveSource::Record) {
        let kind = discriminant(&d.spec.kind);
        let Some(slot) = left.iter_mut().find(|r| r.as_ref().is_some_and(|r| discriminant(&r.spec.kind) == kind)) else {
            continue;
        };
        let r = slot.take().expect("found above");
        if r.extra != PackageExtra::None {
            extras.push((d.id, r.extra));
        }
    }
    AiPackageExtras(extras)
}

/// `Update`, after `attach`: seeds the stack of each actor that just got its [`AiActor`].
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn seed_directives(
    mut commands: Commands,
    content: Res<WalkerContent>,
    store: Option<ResMut<AiDirectives>>,
    clock: Option<Res<GameClock>>,
    fresh: Query<(Entity, &AiActor, &Character, &ActorSpace, &Mover), Added<AiActor>>,
    placed: Query<(&Character, &CharacterPlacement)>,
) {
    let Some(mut store) = store else {
        return;
    };
    if fresh.is_empty() {
        return;
    }
    let resolver = SpawnedActors::of(&placed);
    let now = clock.map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let (mut seeded, mut pushed, mut skipped) = (0_usize, 0_usize, 0_usize);
    for (entity, actor, character, space, mover) in &fresh {
        let Character::Record { id, .. } = character else {
            continue;
        };
        let (record, packages) = packages_of(&content, id);
        let home = Location::new(space.0.clone(), mover.loco.position);
        let ctx = PackageCtx { actor: record.clone(), home, resolver: &resolver };
        let mapped = directives(packages, &ctx);
        let extras = if let Some(stack) = store.get(actor.0) {
            extras_of(stack, mapped)
        } else {
            for s in &mapped.skipped {
                debug!("ai packages of {record}: {} #{} skipped ({:?})", s.tag, s.index, s.reason);
            }
            skipped += mapped.skipped.len();
            pushed += mapped.directives.len();
            seeded += 1;
            let (stack, extras) = seed_stack(actor.0, now, mapped);
            store.insert(stack);
            extras
        };
        commands.entity(entity).try_insert((extras, crate::directive_drive::DirectiveDrive::default()));
    }
    if seeded > 0 {
        info!("ai packages: {seeded} actor stack(s) seeded, {pushed} directive(s), {skipped} skipped (unresolved target)");
    }
}

#[cfg(test)]
#[path = "directives_tests.rs"]
mod tests;
