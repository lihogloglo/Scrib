//! A body that walks from one loaded cell into another becomes the new cell's (`AI10c3b2`;
//! streaming.md "Re-homing a placement", `docs/roadmaps/ai.md`).
//!
//! `attach` gives every streamed actor body its [`BodyCell`] (the streaming cell its entity hangs
//! under). [`rehome_crossers`] (`Update`, after the streaming drive and the processing range)
//! compares, per body, the cell its feet stand in (`Partition::cell_of(mover.loco.position)`) with
//! that cell. When they differ it asks `Streaming::rehome` (`AI10c3b1`) to move the placement's
//! bookkeeping; on success, in the same system run:
//!
//! - the overlay gets the body's pose (`crate::poses::write_crossing`), so the cell it now stands
//!   in lists it at its next fetch and its home cell's reload does not spawn a twin;
//! - one entity command sets, together, `ChildOf` (the new cell's near group), the local
//!   translation (the drawn world position kept: no jump), `Mover::frame_origin` (the new cell's
//!   origin: `interpolate` draws the root from it) and the [`BodyCell`].
//!
//! A refusal changes nothing: the comparison asks again next frame. It is a state comparison, so
//! no hysteresis and no tunable: a re-home is a `ChildOf` swap and one overlay write. A tier-0
//! promotion that placed a body in another cell (`AI10c2b`) is caught the same way. When the new
//! cell is only at `Level::Near` (`Rehomed::listed` false) the placement leaves
//! [`ResidentActors`]: no Active list holds it until that cell's activation lists it again from
//! the overlay.
//!
//! - **`Held` bodies are skipped** (dead, knocked down, a physics takeover): their
//!   `Transform` is their owner's and their mover's position stays where the hold began (only a
//!   get-up moves it, when the hold ends), so the first frame after the hand-back re-homes them.
//! - **Ordering:** the entity side is one command, so any reader sees either the old parent, local
//!   translation and frame origin or the new ones, never a mix; the system runs after
//!   `StreamingSystems::Transition` (after the drive), so no stage starts between the bookkeeping
//!   and the command. `head_track` reads only the player's frame origin; `interpolate` (next frame)
//!   is the only reader of an NPC's.
//! - **Gap (counted):** a body whose placement no Active list and no overlay entry holds (spawned
//!   by a cell that was never Active, at its indexed pose, then moved by a promotion) is not
//!   re-homed: nothing can be written for it. The proof line counts those waiting.
//! - **Proof line:** `actors: N re-homed` (`, M without a held placement` when some wait), at most
//!   once a second.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_log::prelude::*;
use bevy_math::IVec3;
use bevy_time::prelude::*;
use bevy_transform::components::Transform;
use sf_character_bevy::{CharacterPlacement, Characters};
use sf_controller_bevy::{Held, Mover};
use sf_world::{Partition, PlacementId, WorldspaceId};
use sf_world_bevy::{Rehomed, StreamCell, Streaming, WorldOverlay};

use crate::poses::{ResidentActors, content_turn, held_placement, write_crossing};
use crate::walkers::ActorSpace;

/// The streaming cell a body's entity belongs to (`AI10c3b2`): set by `attach` from its ancestor
/// [`StreamCell`], changed by a re-home.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct BodyCell {
    /// The cell's worldspace.
    pub worldspace: WorldspaceId,
    /// The cell.
    pub cell: IVec3,
}

/// Re-homes since the last proof line, and the bodies the last run left in their home cell for
/// want of a held placement (module docs, "Gap").
#[derive(Resource, Debug, Default)]
pub struct CrossingStats {
    rehomed: u32,
    unplaced: u32,
    logged_unplaced: u32,
}

/// The [`BodyCell`] of an entity: its nearest ancestor (or itself) carrying a [`StreamCell`].
pub(crate) fn body_cell_of(entity: Entity, links: &Query<(Option<&StreamCell>, Option<&ChildOf>)>) -> Option<BodyCell> {
    let mut current = entity;
    // Bounded walk: body -> batch -> near group -> cell.
    for _ in 0..8 {
        let (cell, parent) = links.get(current).ok()?;
        if let Some(c) = cell {
            return Some(BodyCell { worldspace: c.worldspace.clone(), cell: c.cell });
        }
        current = parent?.parent();
    }
    None
}

/// What [`rehome_crossers`] reads and writes apart from the streaming runtime, so a test drives
/// [`crossings`] and [`rehome_found`] with its own `rehome`.
#[derive(SystemParam)]
pub(crate) struct Crossers<'w, 's> {
    commands: Commands<'w, 's>,
    bodies: Query<
        'w,
        's,
        (Entity, &'static Mover, &'static ActorSpace, &'static CharacterPlacement, &'static BodyCell),
        Without<Held>,
    >,
    resident: ResMut<'w, ResidentActors>,
    overlay: Option<Res<'w, WorldOverlay>>,
    characters: Option<Res<'w, Characters>>,
    stats: ResMut<'w, CrossingStats>,
}

/// A body whose feet stand outside the cell it belongs to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Crossing {
    pub entity: Entity,
    pub worldspace: WorldspaceId,
    pub id: PlacementId,
    pub from: IVec3,
    pub to: IVec3,
}

/// Every body standing outside its [`BodyCell`] in the same worldspace (`partition_of`: each
/// worldspace's streaming partition). A body that changed worldspace is not this slice's.
pub(crate) fn crossings(cx: &Crossers<'_, '_>, partition_of: impl Fn(&WorldspaceId) -> Option<Partition>) -> Vec<Crossing> {
    let mut found = Vec::new();
    let mut last: Option<(WorldspaceId, Option<Partition>)> = None;
    for (entity, mover, space, placement, home) in &cx.bodies {
        if space.0 != home.worldspace {
            continue;
        }
        let partition = match &last {
            Some((ws, p)) if *ws == space.0 => *p,
            _ => {
                let p = partition_of(&space.0);
                last = Some((space.0.clone(), p));
                p
            }
        };
        let Some(partition) = partition else { continue };
        let to = partition.cell_of(mover.loco.position);
        if to != home.cell {
            found.push(Crossing { entity, worldspace: space.0.clone(), id: placement.0, from: home.cell, to });
        }
    }
    found
}

/// Re-homes each of `found` that `rehome` accepts (module docs): the overlay write now, the
/// entity side as one command. A refusal or a body without a held placement changes nothing.
pub(crate) fn rehome_found(
    cx: &mut Crossers<'_, '_>,
    found: Vec<Crossing>,
    mut rehome: impl FnMut(&WorldspaceId, PlacementId, IVec3, IVec3) -> Option<Rehomed>,
) {
    cx.stats.unplaced = 0;
    let Some(overlay) = cx.overlay.as_deref() else { return };
    let turn = content_turn(cx.characters.as_deref());
    for c in found {
        let Ok((_, mover, space, ..)) = cx.bodies.get(c.entity) else { continue };
        let Some(placement) = held_placement(&cx.resident, overlay, c.id) else {
            cx.stats.unplaced += 1;
            continue;
        };
        let Some(r) = rehome(&c.worldspace, c.id, c.from, c.to) else { continue };
        write_crossing(overlay, &placement, mover, space, turn);
        if !r.listed {
            cx.resident.forget(c.id);
        }
        cx.stats.rehomed += 1;
        let cell = BodyCell { worldspace: c.worldspace, cell: c.to };
        cx.commands.entity(c.entity).queue_silenced(move |mut e: EntityWorldMut| {
            if e.world().get_entity(r.parent).is_err() {
                return;
            }
            let Some(old) = e.get::<Mover>().map(|m| m.frame_origin) else { return };
            if let Some(mut t) = e.get_mut::<Transform>() {
                t.translation = r.local(old.offset(t.translation));
            }
            if let Some(mut m) = e.get_mut::<Mover>() {
                m.frame_origin = r.origin;
            }
            e.insert((ChildOf(r.parent), cell));
        });
    }
}

/// Re-homes every body that crossed into another loaded cell (module docs).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn rehome_crossers(mut cx: Crossers, mut streaming: ResMut<Streaming>) {
    let found = crossings(&cx, |ws| streaming.scheduler().directory(ws).map(|d| d.partition));
    if found.is_empty() {
        cx.stats.unplaced = 0;
        return;
    }
    rehome_found(&mut cx, found, |ws, id, from, to| streaming.rehome(ws, id, from, to));
}

/// The proof line for `rehomed` re-homes and `unplaced` bodies waiting (module docs).
pub(crate) fn proof_line(rehomed: u32, unplaced: u32) -> String {
    if unplaced == 0 {
        format!("actors: {rehomed} re-homed")
    } else {
        format!("actors: {rehomed} re-homed, {unplaced} without a held placement")
    }
}

/// `proof line` ([`proof_line`]): when some re-homed or the waiting count changed, at most once a
/// second.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn report_rehomed(time: Res<Time>, mut stats: ResMut<CrossingStats>, mut last: Local<Option<f64>>) {
    let now_s = time.elapsed_secs_f64();
    let changed = stats.rehomed > 0 || stats.unplaced != stats.logged_unplaced;
    if !changed || last.is_some_and(|t| now_s - t < crate::logical::LOG_EVERY_S) {
        return;
    }
    info!("{}", proof_line(stats.rehomed, stats.unplaced));
    stats.rehomed = 0;
    stats.logged_unplaced = stats.unplaced;
    *last = Some(now_s);
}

#[cfg(test)]
#[path = "crossing_tests.rs"]
mod tests;
