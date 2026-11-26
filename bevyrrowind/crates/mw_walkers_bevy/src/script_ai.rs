//! Script AI packages on the directive stack (`docs/specs/ai-adapter.md` "Packages ->
//! directives", slice `MA2c2`): [`apply_script_pushes`] stacks each [`ScriptAiPush`] (an
//! `MWScript` `AIWander` / `AITravel` / `AIEscort` / `AIFollow`, which `mw_rpg_bevy` sends) on
//! its actor's [`DirectiveStack`], and the directive drive keeps [`AiRuns`] (what `mw_rpg_bevy`
//! answers `GetAIPackageDone` and `GetCurrentAIPackage` from) as it serves the stacks.
//!
//! - **The push** is mapped as a record package is ([`mw_ai::packages::directive`]: same home,
//!   resolver and extras as [`crate::directives::seed_directives`]), from where the actor stands
//!   now, with a [`DirectiveSource::Script`] source, and pushed through [`mw_ai::MorrowindPolicy`]
//!   (`OpenMW` `AiSequence::stack`: an actual package cancels the cancellable ones, so a script
//!   travel ends the record wander). A push for an actor with no entity or no stack yet (not
//!   spawned, not seeded) is dropped with a `debug!`; its target not resolving is skipped as a
//!   record package's is. A repeat the push's clock step renews keeps its extras and home
//!   ([`crate::directives::Renewing`], `MA2d4`); a wander's home is where the actor stands when
//!   it first serves ([`crate::directive_drive`]).
//! - **The reads** port `OpenMW`'s `AiSequence::execute` (`aisequence.cpp` @ 5f3aafb, lines
//!   258-372): each AI tick an actor runs (not [`sf_controller_bevy::Dormant`]), an empty stack
//!   sets the last package to none (:264-268) and leaves done as it was; otherwise the serving
//!   directive, if an actual package (`isActualAiPackage`, :243-246), becomes the last package
//!   (:274-277), and done becomes true when it finished or expired on this tick as an actual
//!   package (:366), false when it ran on (:370). A cancelled directive never sets done (`erase`,
//!   :131-138). A suspended stack (another layer drives) changes nothing.
//!
//! Ours: `OpenMW` saves `mDone` / `mLastAiPackage`; [`AiRuns`] is not saved yet (the next tick
//! rebuilds the last package; a done flag on an empty stack is lost on load).

use std::collections::BTreeMap;

use bevy_ecs::prelude::*;
use bevy_log::{debug, info};
use mw_ai::packages::{PackageCtx, PackageExtra, directive};
use sf_ai::{DirectiveDone, DirectiveEnd, DirectiveId, DirectiveKind, DirectiveSource, DirectiveStack, GameTime};
use sf_ai_bevy::{AiActor, AiDirectives};
use sf_character_bevy::{Character, CharacterPlacement};
use sf_controller_bevy::Mover;
use sf_ids::ActorKey;
use sf_records::RecordId;
use sf_sky_bevy::GameClock;
use sf_world::Location;
use tes3::esp::AiPackage;

use crate::directive_drive::{DirectiveDrive, follow_renewals};
use crate::directives::{AiPackageExtras, Renewing, SpawnedActors};
use crate::walkers::ActorSpace;

/// A script's AI package for `actor`'s stack, as the record struct with the same meaning
/// (`mw_rpg::AiPush::package`), so it maps as an `AI_*` record package does.
#[derive(Message, Clone, Debug, PartialEq)]
pub struct ScriptAiPush {
    pub actor: ActorKey,
    pub package: AiPackage,
}

/// The packages Morrowind knows (`OpenMW` `isActualAiPackage`): what `GetCurrentAIPackage`
/// reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiPackageKind {
    Wander,
    Travel,
    Escort,
    Follow,
    Activate,
}

/// One actor's AI as the script reads see it (`OpenMW` `AiSequence::mDone`, `mLastAiPackage`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AiRun {
    /// `GetAIPackageDone`: the package served on the last AI tick finished on it.
    pub done: bool,
    /// `GetCurrentAIPackage`: the package served on the last AI tick (`None`: an empty stack).
    pub last: Option<AiPackageKind>,
}

/// Every actor's [`AiRun`] by key; an actor without an entry reads not done, no package.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct AiRuns(pub BTreeMap<ActorKey, AiRun>);

impl AiRuns {
    pub fn get(&self, actor: ActorKey) -> AiRun {
        self.0.get(&actor).copied().unwrap_or_default()
    }
}

/// The package kind `GetCurrentAIPackage` reports for a directive kind; `None` for the ones
/// Morrowind has no package for (`isActualAiPackage`; [`mw_ai::is_actual_package`]).
pub fn package_kind(kind: &DirectiveKind) -> Option<AiPackageKind> {
    match kind {
        DirectiveKind::StayNear { .. } => Some(AiPackageKind::Wander),
        DirectiveKind::GoTo(_) => Some(AiPackageKind::Travel),
        DirectiveKind::Escort { .. } => Some(AiPackageKind::Escort),
        DirectiveKind::Follow(_) => Some(AiPackageKind::Follow),
        DirectiveKind::Use(_) => Some(AiPackageKind::Activate),
        DirectiveKind::Fight(_) | DirectiveKind::Custom(_) => None,
    }
}

/// What a stack served at the start of an AI tick (before it advanced).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Served {
    Empty,
    /// Not empty, but nothing is served (another layer drives).
    Suspended,
    Directive(DirectiveId, Option<AiPackageKind>),
}

impl Served {
    pub(crate) fn of(stack: &DirectiveStack) -> Self {
        if stack.is_empty() {
            return Served::Empty;
        }
        stack.serving().map_or(Served::Suspended, |d| Served::Directive(d.id, package_kind(&d.spec.kind)))
    }
}

/// One actor's [`AiRun`] after a tick that started serving `served`, with the tick's `events`
/// (module docs).
pub(crate) fn next_run(run: AiRun, actor: ActorKey, served: Served, events: &[DirectiveDone]) -> AiRun {
    match served {
        Served::Empty => AiRun { last: None, ..run },
        Served::Suspended => run,
        Served::Directive(id, kind) => {
            let ended =
                events.iter().any(|e| e.actor == actor && e.id == id && e.end != DirectiveEnd::Cancelled);
            let done = match (ended, kind) {
                (true, Some(_)) => true,
                (true, None) => run.done,
                (false, _) => false,
            };
            AiRun { done, last: kind.or(run.last) }
        }
    }
}

/// Writes the tick's runs into `runs`, touching it only where a run changed.
pub(crate) fn record_runs(runs: &mut ResMut<AiRuns>, ran: &[(ActorKey, Served)], events: &[DirectiveDone]) {
    for &(actor, served) in ran {
        let run = runs.get(actor);
        let next = next_run(run, actor, served, events);
        if next != run {
            runs.0.insert(actor, next);
        }
    }
}

/// `FixedUpdate`, before the directive drive: stacks every [`ScriptAiPush`] (module docs).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub(crate) fn apply_script_pushes(
    mut pushes: MessageReader<ScriptAiPush>,
    store: Option<ResMut<AiDirectives>>,
    clock: Option<Res<GameClock>>,
    mut actors: Query<(
        Entity,
        &AiActor,
        &ActorSpace,
        &Mover,
        Option<&Character>,
        Option<&mut AiPackageExtras>,
        Option<&mut DirectiveDrive>,
    )>,
    placed: Query<(&Character, &CharacterPlacement)>,
) {
    if pushes.is_empty() {
        return;
    }
    let Some(mut store) = store else {
        pushes.clear();
        return;
    };
    let by_key: BTreeMap<ActorKey, Entity> = actors.iter().map(|(e, a, ..)| (a.0, e)).collect();
    let resolver = SpawnedActors::of(&placed);
    let now = clock.map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let (mut stacked, mut dropped) = (0_usize, 0_usize);
    let mut events = Vec::new();
    let policy = Renewing::default();
    for push in pushes.read() {
        let Some((_, _, space, mover, character, mut extras, mut drive)) =
            by_key.get(&push.actor).and_then(|e| actors.get_mut(*e).ok())
        else {
            debug!("script ai: {:?} has no walker; the push is dropped", push.actor);
            dropped += 1;
            continue;
        };
        let Some(stack) = store.get_mut(push.actor) else {
            debug!("script ai: {:?} has no directive stack yet; the push is dropped", push.actor);
            dropped += 1;
            continue;
        };
        let origin = match character {
            Some(Character::Record { id, .. }) => id.clone(),
            _ => RecordId::new(""),
        };
        let ctx = PackageCtx { actor: origin, home: Location::new(space.0.clone(), mover.loco.position), resolver: &resolver };
        let (mut spec, extra) = match directive(&push.package, &ctx) {
            Ok(mapped) => mapped,
            Err(reason) => {
                debug!("script ai: {} on {:?} skipped ({reason:?})", mw_ai::packages::tag(&push.package), push.actor);
                dropped += 1;
                continue;
            }
        };
        spec.source = DirectiveSource::Script;
        let id = stack.push(spec, now, &policy, &mut events);
        follow_renewals(&policy, drive.as_deref_mut(), extras.as_mut(), stack);
        if let Some(mut extras) = extras {
            extras.0.retain(|(d, _)| stack.get(*d).is_some());
            if extra != PackageExtra::None {
                extras.0.push((id, extra));
            }
        }
        stacked += 1;
    }
    for e in &events {
        debug!("directive {:?} of actor {:?} ended ({:?}) by a script push", e.id, e.actor, e.end);
    }
    info!("script ai: {stacked} push(es) stacked, {dropped} dropped");
}
