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
//! - **Resolver** (`MA2d3`): `player` is the host's character ([`ActorKey::player`] of
//!   [`HOST_KEY`]; `OpenMW` `World::searchPtr`, `worldimp.cpp` @ 5f3aafb 634-637: the player is
//!   always there); other `AI_E`/`AI_F` targets are a spawned character placement with that base
//!   record, `AI_A` objects an R0 placement of it from [`PlacedObjects`] (lowest placement id
//!   first in both).
//! - **Late targets** (`MA2d3`): `OpenMW` looks a target up when the package runs, not when it is
//!   filled (`aipackage.cpp` 58-73 `getTarget`, `aiactivate.cpp` 26-32), so a target that isn't
//!   spawned yet is not dropped: the package waits in [`PendingPackages`] and [`retry_pending`]
//!   maps it again when a character or an indexed object spawns (change-driven), inserting it at
//!   its record position (before the first directive of a later record package). A target id
//!   nothing ever spawns waits for good; an escort without a destination is skipped. One `info!`
//!   per spawn batch counts both. A loaded save keeps only its stacks: what was pending is not
//!   saved (a respawn in the same run keeps it).
//! - **Extras** (wander idle table, follow end point) ride on the actor as [`AiPackageExtras`],
//!   by directive id, with the [`crate::directive_drive::DirectiveDrive`] that serves the top
//!   (`MA2d2`). A repeating directive comes back under a fresh id (`sf_ai` `StackEdit::renew`);
//!   the stack calls that edit [`Renewing`] note each one, and the extras follow it
//!   ([`AiPackageExtras::follow`], `MA2d4`), as `OpenMW`'s repeat keeps the package's own data
//!   (`aisequence.cpp` @ 5f3aafb, 349-354: `push_back(package->clone())`).

use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;
use std::sync::{Arc, Mutex, PoisonError};

use bevy_asset::{AssetServer, UntypedHandle};
use bevy_ecs::prelude::*;
use bevy_log::{debug, info};
use mw_ai::MorrowindPolicy;
use mw_ai::packages::{
    PackageCtx, PackageDirectives, PackageExtra, PackageResolver, SkipReason, directive, directives,
};
use mw_game_core::player::HOST_KEY;
use sf_ai::{
    Directive, DirectiveEnd, DirectiveId, DirectiveSource, DirectiveSpec, DirectiveStack, GameTime, StackEdit,
    StackPolicy,
};
use sf_ai_bevy::{AiActor, AiDirectives, AiLogical};
use sf_character_bevy::{Character, CharacterPlacement};
use sf_controller_bevy::Mover;
use sf_ids::ActorKey;
use sf_records::{RecordDb, RecordId};
use sf_sky_bevy::GameClock;
use sf_world::{IndexedPlacement, Location, PlacementClass, PlacementFlags, PlacementId};
use sf_world_bevy::{ActivateCx, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx};
use tes3::esp::{AiPackage, Creature, Npc};

use crate::directive_drive::{DirectiveDrive, follow_renewals};
use crate::walkers::{ActorSpace, WalkerContent};

/// The record id packages name the player by (`OpenMW` compares it case-blind).
pub const PLAYER_ID: &str = "player";

/// Data of the actor's record packages a directive has no room for, by the directive it rides
/// with (the wander idle table, follow's end point). Directives without extra data are absent.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct AiPackageExtras(pub Vec<(DirectiveId, PackageExtra)>);

impl AiPackageExtras {
    pub fn get(&self, id: DirectiveId) -> Option<&PackageExtra> {
        self.0.iter().find(|(d, _)| *d == id).map(|(_, e)| e)
    }

    /// Moves each extra to the new id of its directive's repeat (`renewed`: old, new, in the
    /// order they repeated, from [`Renewing::take`]).
    pub fn follow(&mut self, renewed: &[(DirectiveId, DirectiveId)]) {
        for &(old, new) in renewed {
            for (d, _) in self.0.iter_mut().filter(|(d, _)| *d == old) {
                *d = new;
            }
        }
    }
}

/// [`MorrowindPolicy`] that also notes each repeat's new id (module docs): pass it to the
/// stack's edits, then move what rides by id with [`Renewing::take`]'s pairs.
#[derive(Debug, Default)]
pub struct Renewing(Mutex<Vec<(DirectiveId, DirectiveId)>>);

impl Renewing {
    /// The (old, new) id of every repeat since the last call, in the order they repeated.
    pub fn take(&self) -> Vec<(DirectiveId, DirectiveId)> {
        std::mem::take(&mut *self.0.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

impl StackPolicy for Renewing {
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive) {
        MorrowindPolicy.push(stack, d);
    }

    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd) {
        let newest = stack.iter().map(|d| d.id).max();
        MorrowindPolicy.on_end(stack, ended, end);
        // A renewal's id is fresh: above every id already on the stack.
        if let Some(id) = stack.iter().map(|d| d.id).max().filter(|id| newest.is_none_or(|n| *id > n)) {
            self.0.lock().unwrap_or_else(PoisonError::into_inner).push((ended.id, id));
        }
    }
}

/// Spawned character placements by base record (lowest placement id per record), the player,
/// and the `AI_A` objects of [`PlacedObjects`] when given ([`Self::with_objects`]).
pub(crate) struct SpawnedActors<'a> {
    actors: BTreeMap<RecordId, ActorKey>,
    objects: Option<&'a PlacedObjects>,
}

impl SpawnedActors<'_> {
    pub(crate) fn of(placed: &Query<(&Character, &CharacterPlacement)>) -> Self {
        let mut by_record = BTreeMap::new();
        for (character, placement) in placed {
            if let Character::Record { id, .. } = character {
                let key = ActorKey(placement.0.0);
                by_record.entry(id.clone()).and_modify(|k: &mut ActorKey| *k = (*k).min(key)).or_insert(key);
            }
        }
        SpawnedActors { actors: by_record, objects: None }
    }
}

impl<'a> SpawnedActors<'a> {
    /// Also resolves `AI_A` objects through `objects`.
    pub(crate) fn with_objects(self, objects: Option<&'a PlacedObjects>) -> Self {
        SpawnedActors { objects, ..self }
    }
}

impl PackageResolver for SpawnedActors<'_> {
    fn actor(&self, id: &str) -> Option<ActorKey> {
        let id = RecordId::new(id);
        if id.as_str() == PLAYER_ID {
            return Some(ActorKey::player(HOST_KEY));
        }
        self.actors.get(&id).copied()
    }

    fn placement(&self, id: &str) -> Option<PlacementId> {
        self.objects?.first(&RecordId::new(id))
    }
}

/// R0 placements of the records some `AI_A` package names, by base record: the placement index
/// `AI_A` resolves through, kept by [`PlacedObjectsHook`] as cells enter and leave R0 (only the
/// named records, so the hook costs one set lookup per placement).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct PlacedObjects {
    targets: BTreeSet<RecordId>,
    placed: BTreeMap<RecordId, BTreeSet<PlacementId>>,
}

impl PlacedObjects {
    /// An empty index of `targets`.
    pub fn for_targets(targets: impl IntoIterator<Item = RecordId>) -> Self {
        Self { targets: targets.into_iter().collect(), placed: BTreeMap::new() }
    }

    /// An empty index of every `AI_A` target of `db`'s NPCs and creatures.
    pub fn of_content(db: &RecordDb) -> Self {
        let packages = db
            .iter::<Npc>()
            .flat_map(|(_, n)| n.ai_packages.iter())
            .chain(db.iter::<Creature>().flat_map(|(_, c)| c.ai_packages.iter()));
        Self::for_targets(packages.filter_map(|p| match p {
            AiPackage::Activate(a) => Some(RecordId::new(a.target.as_str())),
            _ => None,
        }))
    }

    pub fn wants(&self, record: &RecordId) -> bool {
        self.targets.contains(record)
    }

    /// Notes a placement of `record` (ignored unless a package names it).
    pub fn add(&mut self, record: &RecordId, id: PlacementId) {
        if self.wants(record) {
            self.placed.entry(record.clone()).or_default().insert(id);
        }
    }

    pub fn remove(&mut self, record: &RecordId, id: PlacementId) {
        if let Some(ids) = self.placed.get_mut(record) {
            ids.remove(&id);
            if ids.is_empty() {
                self.placed.remove(record);
            }
        }
    }

    /// The lowest placement id of `record` in R0.
    pub fn first(&self, record: &RecordId) -> Option<PlacementId> {
        self.placed.get(record)?.first().copied()
    }

    fn has(&self, record: &RecordId, id: PlacementId) -> bool {
        self.placed.get(record).is_some_and(|ids| ids.contains(&id))
    }
}

/// Wraps a class spawner: R0 activation of an enabled placement of a record in [`PlacedObjects`]
/// adds it to the index, deactivation removes it (`OpenMW` `aiactivate.cpp` 30-32: a disabled
/// target is no target).
pub struct PlacedObjectsHook(pub Arc<dyn PlacementSpawner>);

impl PlacementSpawner for PlacedObjectsHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        if p.flags.intersects(PlacementFlags::DISABLED | PlacementFlags::EDITOR_ONLY)
            || !world.get_resource::<PlacedObjects>().is_some_and(|o| o.wants(&p.base.id))
        {
            return;
        }
        if let Some(mut objects) = world.get_resource_mut::<PlacedObjects>() {
            objects.add(&p.base.id, p.id);
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        let p = &placement.placement;
        if world.get_resource::<PlacedObjects>().is_some_and(|o| o.has(&p.base.id, p.id))
            && let Some(mut objects) = world.get_resource_mut::<PlacedObjects>()
        {
            objects.remove(&p.base.id, p.id);
        }
    }
}

/// `PostStartup`: the [`PlacedObjects`] index of the content's `AI_A` targets, and its
/// [`PlacedObjectsHook`] on every class that can be one (after `exterior.rs` built the spawners
/// at `Startup`), the actor class's around [`crate::poses::ActorPoseHook`] (`AI10c3a`).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn hook_placed_objects(
    mut commands: Commands,
    content: Res<WalkerContent>,
    spawners: Option<ResMut<PlacementSpawners>>,
) {
    let objects = PlacedObjects::of_content(&content.db);
    info!("ai packages: {} AI_A target record(s) indexed", objects.targets.len());
    commands.insert_resource(objects);
    let Some(mut spawners) = spawners else { return };
    let classes = [
        PlacementClass::Static,
        PlacementClass::Dynamic,
        PlacementClass::Door,
        PlacementClass::Light,
        PlacementClass::Actor,
        mw_world::source::CONTAINER,
        mw_world::source::ACTIVATOR,
    ];
    for class in classes {
        if let Some(mut inner) = spawners.for_class(class).cloned() {
            if class == PlacementClass::Actor {
                // `AI10c3a`: an actor's body keeps its pose across its cell's unload.
                inner = Arc::new(crate::poses::ActorPoseHook(inner));
            }
            spawners.set(class, Arc::new(PlacedObjectsHook(inner)));
        }
    }
}

/// One actor's record packages whose target wasn't spawned at seed time (module docs).
#[derive(Clone, Debug, PartialEq)]
pub struct Pending {
    /// The actor's record (every directive's origin).
    record: RecordId,
    /// The actor's spawn point when seeded (the mapping's home).
    home: Location,
    /// Record index and package of each one still waiting, in record order.
    waiting: Vec<(usize, AiPackage)>,
    /// Record index and spec of each record package on the stack: a late one goes before the
    /// first of a later package (a repeat keeps its spec, so it matches too).
    stacked: Vec<(usize, DirectiveSpec)>,
}

/// [`Pending`] packages by actor; kept across the actor's respawns (its stack is too).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct PendingPackages(BTreeMap<ActorKey, Pending>);

impl PendingPackages {
    /// How many packages of `actor` wait.
    pub fn waiting(&self, actor: ActorKey) -> usize {
        self.0.get(&actor).map_or(0, |p| p.waiting.len())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Places one late record package at its record position, through [`Renewing`] for the rest.
struct Late<'a> {
    renewing: &'a Renewing,
    index: usize,
    stacked: &'a [(usize, DirectiveSpec)],
}

impl StackPolicy for Late<'_> {
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive) {
        let later = |e: &Directive| {
            e.spec.source == DirectiveSource::Record && self.stacked.iter().any(|(i, s)| *i > self.index && *s == e.spec)
        };
        let at = stack.iter().position(later).unwrap_or(stack.len());
        stack.insert(at, d);
    }

    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd) {
        self.renewing.on_end(stack, ended, end);
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

/// `Update`, after `attach`: seeds the stack of each actor that just got its [`AiActor`]; a
/// package whose target isn't spawned yet waits in [`PendingPackages`] (module docs).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn seed_directives(
    mut commands: Commands,
    content: Res<WalkerContent>,
    store: Option<ResMut<AiDirectives>>,
    mut pending: Option<ResMut<PendingPackages>>,
    objects: Option<Res<PlacedObjects>>,
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
    let resolver = SpawnedActors::of(&placed).with_objects(objects.as_deref());
    let now = clock.map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let (mut seeded, mut pushed, mut waiting, mut skipped) = (0_usize, 0_usize, 0_usize, 0_usize);
    for (entity, actor, character, space, mover) in &fresh {
        let Character::Record { id, .. } = character else {
            continue;
        };
        let (record, packages) = packages_of(&content, id);
        let home = Location::new(space.0.clone(), mover.loco.position);
        let ctx = PackageCtx { actor: record.clone(), home: home.clone(), resolver: &resolver };
        let mapped = directives(packages, &ctx);
        let extras = if let Some(stack) = store.get(actor.0) {
            extras_of(stack, mapped)
        } else {
            let mut late = Vec::new();
            for s in &mapped.skipped {
                let target_missing = matches!(s.reason, SkipReason::UnknownActor(_) | SkipReason::UnknownObject(_));
                if target_missing && pending.is_some() {
                    debug!("ai packages of {record}: {} #{} waits ({:?})", s.tag, s.index, s.reason);
                    late.push((s.index, packages[s.index].clone()));
                } else {
                    debug!("ai packages of {record}: {} #{} skipped ({:?})", s.tag, s.index, s.reason);
                    skipped += 1;
                }
            }
            if !late.is_empty()
                && let Some(pending) = pending.as_mut()
            {
                waiting += late.len();
                let stacked = mapped.directives.iter().map(|d| (d.index, d.spec.clone())).collect();
                pending.0.insert(actor.0, Pending { record: record.clone(), home, waiting: late, stacked });
            }
            pushed += mapped.directives.len();
            seeded += 1;
            let (stack, extras) = seed_stack(actor.0, now, mapped);
            store.insert(stack);
            extras
        };
        commands.entity(entity).try_insert((extras, DirectiveDrive::default()));
    }
    if seeded > 0 {
        info!(
            "ai packages: {seeded} actor stack(s) seeded, {pushed} directive(s), {waiting} pending (target not spawned), {skipped} skipped"
        );
    }
}

/// `Update`, after [`seed_directives`]: when a character spawns or [`PlacedObjects`] changes,
/// maps each [`PendingPackages`] entry again and puts what resolves on its actor's stack at its
/// record position ([`Late`], through [`Renewing`]), with its extras.
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn retry_pending(
    pending: Option<ResMut<PendingPackages>>,
    store: Option<ResMut<AiDirectives>>,
    objects: Option<Res<PlacedObjects>>,
    clock: Option<Res<GameClock>>,
    mut logical: Option<ResMut<AiLogical>>,
    spawned: Query<(), Added<CharacterPlacement>>,
    placed: Query<(&Character, &CharacterPlacement)>,
    mut actors: Query<(Entity, &AiActor, Option<&mut AiPackageExtras>, Option<&mut DirectiveDrive>)>,
) {
    let (Some(mut pending), Some(mut store)) = (pending, store) else {
        return;
    };
    let objects_changed = objects.as_ref().is_some_and(Res::is_changed);
    if pending.is_empty() || (spawned.is_empty() && !objects_changed) {
        return;
    }
    let resolver = SpawnedActors::of(&placed).with_objects(objects.as_deref());
    let now = clock.map_or_else(GameTime::default, |c| GameTime::from(c.now));
    let by_key: BTreeMap<ActorKey, Entity> =
        actors.iter().filter(|(_, a, ..)| pending.0.contains_key(&a.0)).map(|(e, a, ..)| (a.0, e)).collect();
    let policy = Renewing::default();
    let (mut landed, mut events) = (0_usize, Vec::new());
    for (key, entry) in &mut pending.0 {
        let Some(stack) = store.get_mut(*key) else {
            continue;
        };
        let ctx = PackageCtx { actor: entry.record.clone(), home: entry.home.clone(), resolver: &resolver };
        let mut actor = by_key.get(key).and_then(|e| actors.get_mut(*e).ok());
        let waiting = std::mem::take(&mut entry.waiting);
        for (index, package) in waiting {
            let Ok((spec, extra)) = directive(&package, &ctx) else {
                entry.waiting.push((index, package));
                continue;
            };
            let late = Late { renewing: &policy, index, stacked: &entry.stacked };
            let id = stack.push(spec.clone(), now, &late, &mut events);
            if let Some(store) = logical.as_deref_mut() {
                // AI10c2: a tier-0 actor takes the late package up on the next tier-0 step.
                store.wake(*key);
            }
            entry.stacked.push((index, spec));
            landed += 1;
            if let Some((_, _, extras, drive)) = actor.as_mut() {
                follow_renewals(&policy, drive.as_deref_mut(), extras.as_mut(), stack);
                if let Some(extras) = extras {
                    extras.0.retain(|(d, _)| stack.get(*d).is_some());
                    if extra != PackageExtra::None {
                        extras.0.push((id, extra));
                    }
                }
            } else {
                policy.take();
            }
            debug!("ai packages of {}: #{index} resolved late as {id}", entry.record);
        }
    }
    pending.0.retain(|_, p| !p.waiting.is_empty());
    for e in &events {
        debug!("directive {:?} of actor {:?} ended ({:?}) by a late package", e.id, e.actor, e.end);
    }
    if landed > 0 {
        let left: usize = pending.0.values().map(|p| p.waiting.len()).sum();
        info!("ai packages: {landed} pending package(s) resolved, {left} still pending");
    }
}

#[cfg(test)]
#[path = "directives_tests.rs"]
mod tests;
