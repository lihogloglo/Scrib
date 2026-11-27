//! `WorldChange` (world-reactions.md "Pipeline" 1 "Emit" and "`WorldChange` vocabulary", IN14a): the lasting
//! state changes the world reports to reactions (AI19 concern board, crime witnesses), one event per
//! transition.
//!
//! - The kinds are `TagId` rows ([`CHANGE_OPENED`] ... [`CHANGE_KILLED`], [`SHIPPED_KINDS`]); a game adds its
//!   own the same way (`WorldChangeKinds` records carry severity and the concern fed, AI19).
//! - The pure mappers turn the core's events into changes: [`from_door`] (`Opened` / `Closed`),
//!   [`from_light`] (`Snuffed` / `Relit`), [`from_destroyed`] (`Destroyed`, final stage only) and
//!   [`from_fire`] (`Ignited` / `Burnt`; `Doused` is not a change kind). What the core events do not know
//!   (where the object is, its owner, its home placement, a rule's cause) comes from the host as a
//!   [`ChangeSite`].
//! - [`fold_tick`] keeps one event per transition: a paired kind (open/close, snuff/relight,
//!   lock/unlock) toggled back within one tick is no change, the same kind twice one event.
//!
//! Noise is separate (`InteractionNoise`, [`crate::noise`]). Never names a game, `sf_ai` or `sf_crime`:
//! the culprit is a [`UserKey`], the owner a [`ChangeOwner`] the host maps.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use sf_ids::ActorKey;
use sf_world::PlacementId;

use crate::claims::{Tick, UserKey};
use crate::def::TagId;
use crate::destroy::Destroyed;
use crate::door::{DoorChanged, VERB_CLOSE, VERB_OPEN};
use crate::fire::{FireEvent, FireOutput};
use crate::light::{LightChanged, VERB_RELIGHT, VERB_SNUFF};

/// A door or container opened (IN33 `DoorChanged`).
pub const CHANGE_OPENED: TagId = TagId::of("change.opened");
/// A door or container closed.
pub const CHANGE_CLOSED: TagId = TagId::of("change.closed");
/// A lock locked (crime's `LockState` through the activation gate).
pub const CHANGE_LOCKED: TagId = TagId::of("change.locked");
/// A lock unlocked.
pub const CHANGE_UNLOCKED: TagId = TagId::of("change.unlocked");
/// One bash blow on a lock or door (IN37).
pub const CHANGE_BASHED: TagId = TagId::of("change.bashed");
/// A lock broken by bashing (crime settles it).
pub const CHANGE_LOCK_BROKEN: TagId = TagId::of("change.lock_broken");
/// A lever toggled (IN34).
pub const CHANGE_SWITCHED: TagId = TagId::of("change.switched");
/// A light put out (IN19).
pub const CHANGE_SNUFFED: TagId = TagId::of("change.snuffed");
/// A light lit again.
pub const CHANGE_RELIT: TagId = TagId::of("change.relit");
/// An object caught fire.
pub const CHANGE_IGNITED: TagId = TagId::of("change.ignited");
/// An object burnt out (its burnt stage shown).
pub const CHANGE_BURNT: TagId = TagId::of("change.burnt");
/// A thrown or released prop came to rest.
pub const CHANGE_LANDED: TagId = TagId::of("change.landed");
/// An object reached its final damage stage (IN1).
pub const CHANGE_DESTROYED: TagId = TagId::of("change.destroyed");
/// A placed object at rest away from its authored placement.
pub const CHANGE_DISPLACED: TagId = TagId::of("change.displaced");
/// An owned seat or bed used by a non-owner.
pub const CHANGE_OCCUPIED: TagId = TagId::of("change.occupied");
/// An item dropped from an inventory into the world.
pub const CHANGE_DROPPED: TagId = TagId::of("change.dropped");
/// A container's contents spilled (IN1 spill).
pub const CHANGE_SPILLED: TagId = TagId::of("change.spilled");
/// An item taken.
pub const CHANGE_TAKEN: TagId = TagId::of("change.taken");
/// A death that left a body.
pub const CHANGE_KILLED: TagId = TagId::of("change.killed");

/// The kinds the framework ships (the world-reactions.md vocabulary table, in its order).
pub const SHIPPED_KINDS: [TagId; 19] = [
    CHANGE_OPENED,
    CHANGE_CLOSED,
    CHANGE_LOCKED,
    CHANGE_UNLOCKED,
    CHANGE_BASHED,
    CHANGE_LOCK_BROKEN,
    CHANGE_SWITCHED,
    CHANGE_SNUFFED,
    CHANGE_RELIT,
    CHANGE_IGNITED,
    CHANGE_BURNT,
    CHANGE_LANDED,
    CHANGE_DESTROYED,
    CHANGE_DISPLACED,
    CHANGE_OCCUPIED,
    CHANGE_DROPPED,
    CHANGE_SPILLED,
    CHANGE_TAKEN,
    CHANGE_KILLED,
];

/// The paired kinds: a state that toggles back (`(a, b)`: `a` undoes `b` and the other way round).
const PAIRS: [(TagId, TagId); 3] = [
    (CHANGE_OPENED, CHANGE_CLOSED),
    (CHANGE_SNUFFED, CHANGE_RELIT),
    (CHANGE_LOCKED, CHANGE_UNLOCKED),
];

/// The kind that undoes `kind`, if it is a paired kind.
#[must_use]
pub fn inverse(kind: TagId) -> Option<TagId> {
    PAIRS.iter().find_map(|&(a, b)| {
        if kind == a {
            Some(b)
        } else if kind == b {
            Some(a)
        } else {
            None
        }
    })
}

/// Who owns the changed object, as the host maps its ownership data (crime.md "Ownership"): an actor, or a
/// group (faction, guild) by its record's tag. Rank and the lifting global stay crime's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChangeOwner {
    Actor(ActorKey),
    Group(TagId),
}

/// The kind-specific part of a change (world-reactions.md "Carries"); later kinds add variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ChangeDetail {
    #[default]
    None,
    /// The damage or burnt stage shown (`Destroyed`, `Burnt`); `None` inside = the object is removed or
    /// keeps its model.
    Stage(Option<u8>),
}

/// One lasting change of one object (world-reactions.md "Pipeline" 1).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldChange {
    pub tick: Tick,
    /// A `CHANGE_*` row (or a game's own).
    pub kind: TagId,
    pub object: PlacementId,
    /// Where the object is, world metres.
    pub at: Vec3,
    /// Who made it; `None` = a rule (water, fire spread) or a script.
    pub culprit: Option<UserKey>,
    pub owner: Option<ChangeOwner>,
    /// The placement the object belongs to (a container's or a building's), for tidy and discovery.
    pub home: Option<PlacementId>,
    /// The verb or rule tag that made it (a torch, water, an impact, a bash).
    pub cause: Option<TagId>,
    pub detail: ChangeDetail,
}

/// What the host knows of the changed object and the core events do not carry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChangeSite {
    /// Where the object is, world metres.
    pub at: Vec3,
    pub owner: Option<ChangeOwner>,
    pub home: Option<PlacementId>,
}

fn change(tick: Tick, kind: TagId, object: PlacementId, site: &ChangeSite) -> WorldChange {
    WorldChange {
        tick,
        kind,
        object,
        at: site.at,
        culprit: None,
        owner: site.owner,
        home: site.home,
        cause: None,
        detail: ChangeDetail::None,
    }
}

/// `Opened` or `Closed`; the cause is the verb when a user did it, else none (close-behind, a script).
#[must_use]
pub fn from_door(e: &DoorChanged, site: &ChangeSite) -> WorldChange {
    let (kind, verb) = if e.open {
        (CHANGE_OPENED, VERB_OPEN)
    } else {
        (CHANGE_CLOSED, VERB_CLOSE)
    };
    WorldChange {
        culprit: e.user,
        cause: e.user.map(|_| verb),
        ..change(e.tick, kind, e.placement, site)
    }
}

/// `Snuffed` or `Relit` at the light; the cause is the verb when someone did it, else `rule` (water, a
/// script's tag).
#[must_use]
pub fn from_light(e: &LightChanged, site: &ChangeSite, rule: Option<TagId>) -> WorldChange {
    let (kind, verb) = if e.lit {
        (CHANGE_RELIT, VERB_RELIGHT)
    } else {
        (CHANGE_SNUFFED, VERB_SNUFF)
    };
    WorldChange {
        at: e.at,
        culprit: e.culprit,
        cause: if e.culprit.is_some() {
            Some(verb)
        } else {
            rule
        },
        ..change(e.tick, kind, e.placement, site)
    }
}

/// `Destroyed` when `e` crossed the def's final stage (`stage_count` stages) or removes the object; an
/// earlier stage is damage, not a change. The cause is the hit's damage kind ([`Destroyed::kind`]).
#[must_use]
pub fn from_destroyed(e: &Destroyed, stage_count: usize, site: &ChangeSite) -> Option<WorldChange> {
    let last = usize::from(e.stage) + 1 >= stage_count;
    (last || e.removes).then(|| WorldChange {
        culprit: e.culprit,
        cause: Some(e.kind),
        detail: ChangeDetail::Stage((!e.removes).then_some(e.stage)),
        ..change(e.tick, CHANGE_DESTROYED, e.placement, site)
    })
}

/// `Ignited` or `Burnt` (a fire tick's event; `Doused` is none). The fire core knows no users: the culprit
/// is the source's user (`Ignited`) or the one inherited from it (`Burnt`), kept by the host, and `cause`
/// the source tag.
#[must_use]
pub fn from_fire(
    e: &FireEvent,
    tick: Tick,
    site: &ChangeSite,
    culprit: Option<UserKey>,
    cause: Option<TagId>,
) -> Option<WorldChange> {
    let (kind, detail) = match e.output {
        FireOutput::Ignited => (CHANGE_IGNITED, ChangeDetail::None),
        FireOutput::Burnt { stage } => (CHANGE_BURNT, ChangeDetail::Stage(stage)),
        FireOutput::Doused => return None,
    };
    Some(WorldChange {
        culprit,
        cause,
        detail,
        ..change(tick, kind, e.placement, site)
    })
}

/// One event per transition, in place, keeping the order of what is left: within one `(tick, object)` the
/// changes of a pair (open/close, snuff/relight, lock/unlock) reduce to the last one, dropped when it
/// undoes the first (the object ends the tick as it started); any other kind is kept once, its first.
pub fn fold_tick(changes: &mut Vec<WorldChange>) {
    let n = changes.len();
    let mut keep = vec![false; n];
    for i in 0..n {
        let c = &changes[i];
        let channel = inverse(c.kind);
        let same = |o: &WorldChange| {
            o.tick == c.tick
                && o.object == c.object
                && (o.kind == c.kind || Some(o.kind) == channel)
        };
        if changes[..i].iter().any(same) {
            continue;
        }
        if channel.is_none() {
            keep[i] = true;
            continue;
        }
        // The first of this channel: the last of the run decides.
        let last = (i..n).rev().find(|&j| same(&changes[j])).unwrap_or(i);
        if changes[last].kind == c.kind {
            keep[last] = true;
        }
    }
    let mut k = keep.iter();
    changes.retain(|_| k.next().copied().unwrap_or(false));
}
