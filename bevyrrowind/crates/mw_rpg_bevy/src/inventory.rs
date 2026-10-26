//! Taking items into the player's inventory (ROADMAP 2.2 U5 / 2.4; superfargoth `rpg.md`,
//! `net.md` "Net-ready rules", `save.md`).
//!
//! - **Request, then host.** E on the focus writes `Activated`; an unhandled player activation of
//!   a takeable record ([`TakeKind`]) becomes a [`TakeRequest`] (player id, placement id, record:
//!   stable ids only, net rule 7). [`apply_takes`] is the host side: it validates the request
//!   against the live world and applies it. A co-op client would send the request instead.
//! - **Loose items** (MISC, ALCH, APPA, ARMO, BOOK, CLOT, INGR, LOCK, PROB, REPA, WEAP, and LIGH
//!   with the carry flag; `OpenMW` `MWClass::*::activate` -> `ActionTake`, apps/openmw/mwclass,
//!   0.49.0): the placement leaves the world through the overlay (`PlacementOverlay::remove`,
//!   keyed by `PlacementId`: saves serialize it, a cell reload never brings it back), its R0
//!   hooks run now (physics body, script handle: [`ActivePlacements::deactivate`]), its visuals
//!   (the physics `ItemPivot`) despawn, and the reference's stack goes into the player's `sf_rpg`
//!   loadout, so dialogue `Item` filters and `MWScript` `GetItemCount` see it (the `Rpg` host
//!   reads it).
//! - **Stack counts** ([`RefCounts`]): a CELL reference's `NAM9` count (`tes3` `object_count`,
//!   absent = 1) is the stack the take gives. `OpenMW` `ActionTake::executeImp`
//!   (apps/openmw/mwworld/actiontake.cpp, tag openmw-0.49.0 = commit 675146bd) adds
//!   `getCellRef().getCount()`, which is `std::abs(mCount)` (apps/openmw/mwworld/cellref.hpp,
//!   `getCount(absolute = true)`). A negative count is the restocking marker of container
//!   inventories, not of references, so the take uses its magnitude ([`ref_count`], as the
//!   container seeding below does). The counts are a game-side lookup by placement id, built
//!   once from the cells (`sf_world` placements don't carry them); script-spawned placements
//!   count 1.
//! - **Gold piles** ([`is_gold`], [`take_count`]): `Gold_001`, `_005`, `_010`, `_025`, `_100`
//!   (`OpenMW` `Miscellaneous::isGold`, apps/openmw/mwclass/misc.cpp). A take multiplies the
//!   reference's count by the pile's value (`ActionTake::executeImp`), and that value is the MISC
//!   record's `data.value` only for a count of 1: a counted pile is worth 1 a coin
//!   (`Miscellaneous::getValue`; `createGold` writes dropped gold as a pile with count = the
//!   amount). So a `Gold_025` reference gives 25 coins, one with count 40 gives 40. Gold entering
//!   an inventory becomes `Gold_001` with the same count (`ContainerStore::addImp`,
//!   apps/openmw/mwworld/containerstore.cpp: adding 1 `Gold_100` = 1 `Gold_001`; vanilla
//!   inventories only hold `Gold_001`), so [`give`] converts on both paths, the take and the
//!   container seeding. Same tag/commit as above.
//! - **Containers** (CONT): activating one opens the container window (`inventory_ui.rs`, the
//!   stock `sf_ui_screens` container screen). `OpenMW` `ActionOpen::executeImp`
//!   (apps/openmw/mwworld/actionopen.cpp, same tag/commit): only the player opens one (our
//!   [`TakeRequest`]s are the player's). The content lives in `RpgState::inventories` under the
//!   container's placement id (saved with the RPG state) and is `Holder::Actor(ActorKey(id))`,
//!   the store NPC inventories use, so `sf_rpg`'s `Holder` needs no container variant. It is
//!   seeded from the record's item list the first time ([`container_store`]; levelled entries
//!   are skipped). The host validates an open ([`ContainerOpened`]) and applies the window's
//!   intents: [`move_stack`] takes pieces off one stack (`Loadout::take`, instance data kept)
//!   and adds them to the other holder (merging into an identical stack), gold converting as
//!   in [`give`]; [`take_all`] moves every stack in container order, then the window closes
//!   (`ContainerWindow::onTakeAllButtonClicked`, apps/openmw/mwgui/container.cpp). An organic
//!   container (plants: CONT flag `ORGANIC`) refuses items put in
//!   (`ContainerItemModel::onDropItem`, apps/openmw/mwgui/containeritemmodel.cpp,
//!   `sContentsMessage2`). The title is the record name (`ContainerWindow::setPtr`); "owned" is
//!   whether the reference has an owner or a faction owner ([`RefOwners`]), a flag only.
//!   Fargoth's ring (`ring_keley`) is in the `chargen barrel fatigue` barrel outside the Census
//!   and Excise Office (exterior Seyda Neen, cell -2,-9), so this is the ring's path.
//! - Not yet (leftovers): weight and encumbrance (container capacity: `sContentsMessage3`),
//!   drop, equip, theft (the Census office items are owned: vanilla counts the take as
//!   stealing), locked or trapped containers, corpses and pickpocketing, item scripts' `OnActivate`
//!   (the ring's `CharGen_ring_keley` shows the magic menu), books opening to read, dropping
//!   gold back as a pile (`OpenMW` `createGold` picks the pile size for the amount).

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, PoisonError};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_log::prelude::*;
use sf_physics_bevy::ItemPivot;
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, Attrs, CharacterRpg, Holder, PlayerId, RpgCommand, StackId};
use sf_script_bevy::{ActivationOutcome, PLAYER, ScriptPlacements, ScriptSlot};
use sf_world::PlacementId;
use sf_world_bevy::WorldOverlay;
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Cell, Clothing, Container, ContainerFlags, Ingredient, LeveledItem, Light, LightFlags, Lockpick,
    MiscItem, Probe, Reference, RepairItem, Weapon,
};

use crate::dialogue::{Rpg, RpgState};
use mw_scripts_bevy::ActivePlacements;

/// What activating a record does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TakeKind {
    /// A loose item: takes the placement itself.
    Item,
    /// A container: opens its window.
    Container,
}

/// Which records the player can take (the loaded records).
#[derive(Resource, Clone)]
pub struct ItemRecords(pub Arc<RecordDb>);

impl ItemRecords {
    pub fn kind(&self, record: &RecordId) -> Option<TakeKind> {
        take_kind(&self.0, record)
    }
}

/// [`TakeKind`] of `record`: an item class vanilla lets the player take, or a container.
pub fn take_kind(db: &RecordDb, record: &RecordId) -> Option<TakeKind> {
    let item = db.get::<MiscItem>(record).is_some()
        || db.get::<Alchemy>(record).is_some()
        || db.get::<Apparatus>(record).is_some()
        || db.get::<Armor>(record).is_some()
        || db.get::<Book>(record).is_some()
        || db.get::<Clothing>(record).is_some()
        || db.get::<Ingredient>(record).is_some()
        || db.get::<Lockpick>(record).is_some()
        || db.get::<Probe>(record).is_some()
        || db.get::<RepairItem>(record).is_some()
        || db.get::<Weapon>(record).is_some()
        || db.get::<Light>(record).is_some_and(|l| l.data.flags.contains(LightFlags::CAN_CARRY));
    if item {
        Some(TakeKind::Item)
    } else if db.get::<Container>(record).is_some() {
        Some(TakeKind::Container)
    } else {
        None
    }
}

/// Stack counts of the CELL references whose count isn't 1, by placement id (module docs).
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct RefCounts(pub HashMap<PlacementId, u32>);

impl RefCounts {
    /// The counts of every cell reference in `db` (the merged references: the last plugin wins).
    pub fn from_records(db: &RecordDb) -> Self {
        let mut out = HashMap::new();
        for (_, cell) in db.iter::<Cell>() {
            for (&(source, refr_index), reference) in &cell.references {
                let count = ref_count(reference);
                if count != 1 {
                    let file = &db.source(sf_records::SourceId(source)).name;
                    out.insert(mw_world::ids::placement_id(file, refr_index), count);
                }
            }
        }
        Self(out)
    }

    /// The stack a take of placement `id` gives (1 without a count).
    pub fn get(&self, id: PlacementId) -> u32 {
        self.0.get(&id).copied().unwrap_or(1)
    }
}

/// A reference's stack count. `NAM9` is a signed int in the file (`OpenMW` `ESM::CellRef::mCount`)
/// that `tes3` reads as `u32`: reinterpret it and take the magnitude (absent = 1). A 0 count
/// (the Construction Set doesn't write one) reads as 1, so a take never gives nothing.
pub fn ref_count(reference: &Reference) -> u32 {
    #[allow(clippy::cast_possible_wrap)] // reinterprets the file's signed int
    reference.object_count.map_or(1, |c| (c as i32).unsigned_abs().max(1))
}

/// The CELL references with an owner (an NPC, `CNAM`, or a faction, `ANAM`), by placement id
/// (module docs: the container window's "owned" flag; theft rules come with crime).
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct RefOwners(pub HashSet<PlacementId>);

impl RefOwners {
    /// The owned references of every cell in `db` (the merged references: the last plugin wins).
    pub fn from_records(db: &RecordDb) -> Self {
        let named = |s: &Option<String>| s.as_deref().is_some_and(|s| !s.trim().trim_matches('\0').is_empty());
        let mut out = HashSet::new();
        for (_, cell) in db.iter::<Cell>() {
            for (&(source, refr_index), reference) in &cell.references {
                if named(&reference.owner) || named(&reference.owner_faction) {
                    let file = &db.source(sf_records::SourceId(source)).name;
                    out.insert(mw_world::ids::placement_id(file, refr_index));
                }
            }
        }
        Self(out)
    }
}

/// The gold pile records (`OpenMW` `Miscellaneous::isGold`; ids are folded lowercase).
pub const GOLD_PILES: [&str; 5] = ["gold_001", "gold_005", "gold_010", "gold_025", "gold_100"];

/// The gold every inventory holds (`OpenMW` `ContainerStore::sGoldId`).
pub const GOLD: &str = "gold_001";

/// Whether `item` is a gold pile (module docs).
pub fn is_gold(item: &RecordId) -> bool {
    GOLD_PILES.contains(&item.as_str())
}

/// What a take of a reference of `item` with stack `count` gives: `count`, or for a gold pile
/// `count` x its value, where the value is the MISC record's for a count of 1 and 1 otherwise
/// (module docs; `OpenMW` `ActionTake::executeImp`, `Miscellaneous::getValue`).
pub fn take_count(db: &RecordDb, item: &RecordId, count: u32) -> u32 {
    if count != 1 || !is_gold(item) {
        return count;
    }
    db.get::<MiscItem>(item).map_or(1, |m| u32::try_from(i64::from(m.data.value)).map_or(1, |v| v.max(1)))
}

/// A player asks to take what a placement holds (net rule 2: an intent, validated by the host).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct TakeRequest {
    pub player: PlayerId,
    pub target: PlacementId,
    pub record: RecordId,
    pub kind: TakeKind,
}

/// The host opened a container for a player (validated, its store seeded): the container
/// window shows it (`inventory_ui.rs`). A co-op client would get this back from the host.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ContainerOpened {
    pub player: PlayerId,
    /// The container's placement id (its store: `Holder::Actor(ActorKey(target.0))`).
    pub target: PlacementId,
    pub record: RecordId,
    /// The reference has an owner ([`RefOwners`]).
    pub owned: bool,
}

/// After `dialogue::add` (the `Rpg` host) and `scripts::add` (activations).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>) {
    let Some(db) = db else { return };
    app.insert_resource(RefCounts::from_records(&db))
        .insert_resource(RefOwners::from_records(&db))
        .insert_resource(ItemRecords(db))
        .add_message::<TakeRequest>()
        .add_message::<ContainerOpened>()
        .add_systems(Update, (request_takes, apply_takes).chain().after(ScriptSlot::Update));
}

/// Unhandled player activations of takeable records become [`TakeRequest`]s.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn request_takes(
    mut outcomes: MessageReader<ActivationOutcome>,
    placements: Option<Res<ScriptPlacements>>,
    records: Res<ItemRecords>,
    mut out: MessageWriter<TakeRequest>,
) {
    let Some(placements) = placements else {
        outcomes.clear();
        return;
    };
    for o in outcomes.read().filter(|o| !o.handled && o.activation.actor == PLAYER) {
        let Some(placed) = placements.get(o.activation.target) else { continue };
        let record = RecordId::new(&placed.record);
        if let Some(kind) = records.kind(&record) {
            out.write(TakeRequest { player: PlayerId::HOST, target: PlacementId(o.activation.target.0), record, kind });
        }
    }
}

/// Host side of [`TakeRequest`]: a loose item leaves the world (overlay, hooks, visuals) and
/// goes into the player's loadout; a container's store is seeded and it opens
/// ([`ContainerOpened`]).
pub fn apply_takes(world: &mut World, reader: &mut SystemState<MessageReader<'static, 'static, TakeRequest>>) {
    let requests: Vec<TakeRequest> = reader.get_mut(world).map(|mut r| r.read().cloned().collect()).unwrap_or_default();
    for request in requests {
        if !world.contains_resource::<Rpg>() {
            return;
        }
        match request.kind {
            TakeKind::Item => {
                let removed = world.get_resource::<WorldOverlay>().is_some_and(|o| o.0.read().unwrap_or_else(PoisonError::into_inner).is_removed(request.target));
                if removed || take_placement(world, request.target).is_none() {
                    warn!("take {}: placement {:032x} is not in the world", request.record, request.target.0);
                    continue;
                }
                let count = world.get_resource::<RefCounts>().map_or(1, |c| c.get(request.target));
                let count = take_count(&world.resource::<ItemRecords>().0, &request.record, count);
                let mut rpg = world.resource_mut::<Rpg>();
                let item = give(&mut rpg.state, request.player, &request.record, count);
                info!("take: {count} {item} from {} (now {})", request.record, rpg.state.pc.loadout.count(&item));
            }
            TakeKind::Container => {
                let removed = world.get_resource::<WorldOverlay>().is_some_and(|o| o.0.read().unwrap_or_else(PoisonError::into_inner).is_removed(request.target));
                if removed || world.resource::<ItemRecords>().kind(&request.record) != Some(TakeKind::Container) {
                    warn!("open {}: placement {:032x} is not a container in the world", request.record, request.target.0);
                    continue;
                }
                let db = world.resource::<ItemRecords>().0.clone();
                let mut rpg = world.resource_mut::<Rpg>();
                let stacks = container_store(&mut rpg.state, &db, ActorKey(request.target.0), &request.record).loadout.inventory().stacks().len();
                let owned = world.get_resource::<RefOwners>().is_some_and(|o| o.0.contains(&request.target));
                info!("open {}: {stacks} stacks{}", request.record, if owned { ", owned" } else { "" });
                world.write_message(ContainerOpened { player: request.player, target: request.target, record: request.record, owned });
            }
        }
    }
}

/// Removes a loose item's placement from the world now: R0 hooks, overlay, visuals.
pub fn take_placement(world: &mut World, id: PlacementId) -> Option<()> {
    ActivePlacements::deactivate(world, id)?;
    // After the hooks: physics' deactivation writes a moved item to the overlay, the removal wins.
    if let Some(overlay) = world.get_resource::<WorldOverlay>() {
        overlay.0.write().unwrap_or_else(PoisonError::into_inner).remove(id);
    }
    let pivots: Vec<Entity> = world.query::<(Entity, &ItemPivot)>().iter(world).filter(|(_, p)| p.id == id).map(|(e, _)| e).collect();
    for e in pivots {
        world.despawn(e);
    }
    Some(())
}

/// Adds `count` of `item` to `player`'s loadout (one player character until co-op N slices); a
/// gold pile goes in as [`GOLD`] (module docs). Returns the id it added.
fn give(state: &mut RpgState, player: PlayerId, item: &RecordId, count: u32) -> RecordId {
    let item = gold_as_001(item);
    let command = RpgCommand::AddItem { holder: Holder::Player(player), item: item.clone(), count, attrs: Attrs::new() };
    if let Err(e) = state.pc.apply(command) {
        warn!("take {item}: {e:?}");
    }
    item
}

/// `item`, or [`GOLD`] for a gold pile (`ContainerStore::addImp`, module docs).
fn gold_as_001(item: &RecordId) -> RecordId {
    if is_gold(item) { RecordId::new(GOLD) } else { item.clone() }
}

/// The store of `container` (a placement of CONT `record`), seeded from the record's item list
/// on first access (levelled entries skipped; negative restocking counts by magnitude).
pub fn container_store<'a>(state: &'a mut RpgState, db: &RecordDb, container: ActorKey, record: &RecordId) -> &'a mut CharacterRpg {
    state.inventories.entry(container).or_insert_with(|| {
        let mut seeded = CharacterRpg::default();
        for (count, item) in db.get::<Container>(record).map(|c| c.inventory.as_slice()).unwrap_or_default() {
            let item = RecordId::new(item.as_str());
            if db.get::<LeveledItem>(&item).is_some() {
                info!("open {record}: levelled list {item} skipped (no levelled items yet)");
                continue;
            }
            let _ = seeded.loadout.add(item, count.unsigned_abs(), Attrs::new());
        }
        seeded
    })
}

/// Whether CONT `record` is organic (plants): it refuses items put in (module docs).
pub fn is_organic(db: &RecordDb, record: &RecordId) -> bool {
    db.get::<Container>(record).is_some_and(|c| c.container_flags.contains(ContainerFlags::ORGANIC))
}

/// The character behind `holder`: the player, or an existing store in `RpgState::inventories`
/// (a container the host opened, an NPC). One player character until co-op N slices.
fn holder_mut(state: &mut RpgState, holder: Holder) -> Option<&mut CharacterRpg> {
    match holder {
        Holder::Player(PlayerId::HOST) => Some(&mut state.pc),
        Holder::Player(_) => None,
        Holder::Actor(key) => state.inventories.get_mut(&key),
    }
}

/// Moves `count` pieces of `stack` from `from` to `to` (the container window's Move): they
/// leave the stack (`Loadout::take`, attributes kept) and join an identical stack of `to` or
/// start one; gold enters as [`GOLD`]. An unknown holder or stack changes nothing; a failed
/// add (count overflow) gives the pieces back to `from`. Returns the item and count that moved.
pub fn move_stack(state: &mut RpgState, from: Holder, to: Holder, stack: StackId, count: u32) -> Option<(RecordId, u32)> {
    if from == to || holder_mut(state, to).is_none() {
        return None;
    }
    let taken = holder_mut(state, from)?.loadout.take(stack, count).ok()?;
    let item = gold_as_001(&taken.item);
    let command = RpgCommand::AddItem { holder: to, item, count: taken.count, attrs: taken.attrs.clone() };
    let target = holder_mut(state, to).expect("checked above");
    if let Err(e) = target.apply(command) {
        warn!("move {}: {e:?}", taken.item);
        // Put it back as it was (the source had it a moment ago).
        let source = holder_mut(state, from).expect("taken from above");
        let _ = source.loadout.add(taken.item, taken.count, taken.attrs);
        return None;
    }
    Some((taken.item, taken.count))
}

/// The container window's Take all: moves every stack of `container` (seeded from `record` on
/// first access) to `player`, in container order. Returns what moved.
pub fn take_all(state: &mut RpgState, db: &RecordDb, player: PlayerId, container: ActorKey, record: &RecordId) -> Vec<(RecordId, u32)> {
    let stacks: Vec<(StackId, u32)> = container_store(state, db, container, record).loadout.inventory().stacks().iter().map(|s| (s.id, s.count)).collect();
    stacks
        .into_iter()
        .filter_map(|(stack, count)| move_stack(state, Holder::Actor(container), Holder::Player(player), stack, count))
        .collect()
}

#[cfg(test)]
mod tests {
    //! The ring path on real data (skips without a Morrowind install): the Census office barrel
    //! holds Fargoth's ring, taking it through the container window lets Fargoth's ring dialogue
    //! finish the quest.
    use super::*;
    use mw_install::MorrowindInstall;
    use sf_rpg_ui::DialogueRequest;

    const FARGOTH: ActorKey = ActorKey(1);

    /// Synthetic `Master.esm`: one cell with a pile of 5 arrows (`NAM9`), a single arrow (no
    /// count) and a restocking-style negative count.
    #[test]
    fn a_reference_take_gives_its_stack_count() {
        use tes3::esp::{FileType, Header, Plugin, TES3Object};
        let arrow = "iron arrow";
        let reference = |refr_index, object_count| Reference {
            refr_index,
            id: arrow.into(),
            object_count,
            ..Reference::default()
        };
        let mut cell = Cell { name: "Room".into(), ..Cell::default() };
        cell.data.flags = tes3::esp::CellFlags::IS_INTERIOR;
        for r in [reference(1, Some(5)), reference(2, None), reference(3, Some((-3i32).cast_unsigned()))] {
            cell.references.insert((r.mast_index, r.refr_index), r);
        }
        let objects: Vec<TES3Object> = vec![
            Header { file_type: FileType::Esm, ..Header::default() }.into(),
            MiscItem { id: arrow.into(), ..MiscItem::default() }.into(),
            cell.into(),
        ];
        let bytes = Plugin { objects }.save_bytes().unwrap();
        let vfs = sf_vfs::Vfs::builder().mount(sf_vfs::MemSource::new("data").with("Master.esm", bytes)).build();
        let db = mw_records::load(&vfs, &["Master.esm".to_owned()]).unwrap().db;
        let counts = RefCounts::from_records(&db);
        let id = |i| mw_world::ids::placement_id("Master.esm", i);
        assert_eq!((counts.get(id(1)), counts.get(id(2)), counts.get(id(3))), (5, 1, 3));

        let item = RecordId::new(arrow);
        let mut state = RpgState::default();
        give(&mut state, PlayerId::HOST, &item, counts.get(id(1)));
        assert_eq!(state.pc.loadout.count(&item), 5);
        give(&mut state, PlayerId::HOST, &item, counts.get(id(2)));
        assert_eq!(state.pc.loadout.count(&item), 6, "a reference without a count gives 1");
    }

    /// Synthetic `Master.esm`: gold pile references (`Gold_025` without a count and with 2,
    /// `Gold_001` with 7) and a `Gold_100` from a container list.
    #[test]
    fn gold_piles_become_gold_001() {
        use tes3::esp::{FileType, Header, Plugin, TES3Object};
        let reference = |refr_index, id: &str, object_count| Reference {
            refr_index,
            id: id.into(),
            object_count,
            ..Reference::default()
        };
        let mut cell = Cell { name: "Room".into(), ..Cell::default() };
        cell.data.flags = tes3::esp::CellFlags::IS_INTERIOR;
        for r in [reference(1, "Gold_025", None), reference(2, "Gold_025", Some(2)), reference(3, "Gold_001", Some(7))] {
            cell.references.insert((r.mast_index, r.refr_index), r);
        }
        let pile = |id: &str, value| {
            let mut m = MiscItem { id: id.into(), ..MiscItem::default() };
            m.data.value = value;
            TES3Object::from(m)
        };
        let objects: Vec<TES3Object> = vec![
            Header { file_type: FileType::Esm, ..Header::default() }.into(),
            pile("Gold_001", 1),
            pile("Gold_025", 25),
            pile("Gold_100", 100),
            cell.into(),
        ];
        let bytes = Plugin { objects }.save_bytes().unwrap();
        let vfs = sf_vfs::Vfs::builder().mount(sf_vfs::MemSource::new("data").with("Master.esm", bytes)).build();
        let db = mw_records::load(&vfs, &["Master.esm".to_owned()]).unwrap().db;
        let counts = RefCounts::from_records(&db);
        let id = |i| mw_world::ids::placement_id("Master.esm", i);
        let (gold, gold_25) = (RecordId::new("Gold_001"), RecordId::new("Gold_025"));
        assert!(is_gold(&gold_25) && !is_gold(&RecordId::new("iron arrow")));

        let mut state = RpgState::default();
        let take = |state: &mut RpgState, i, item: &RecordId| give(state, PlayerId::HOST, item, take_count(&db, item, counts.get(id(i))));
        assert_eq!(take(&mut state, 1, &gold_25), gold);
        assert_eq!(state.pc.loadout.count(&gold), 25, "a single pile is worth its value");
        take(&mut state, 2, &gold_25);
        assert_eq!(state.pc.loadout.count(&gold), 27, "a counted pile is worth 1 a coin (OpenMW getValue)");
        take(&mut state, 3, &gold);
        assert_eq!(state.pc.loadout.count(&gold), 34);
        give(&mut state, PlayerId::HOST, &RecordId::new("Gold_100"), 1);
        assert_eq!(state.pc.loadout.count(&gold), 35, "container gold converts by id only (OpenMW addImp)");
        assert_eq!(state.pc.loadout.count(&gold_25), 0);
        assert_eq!(state.pc.loadout.count(&RecordId::new("gold_100")), 0);
    }

    /// The ring barrel's placement id (the real reference, so its owner is read).
    fn barrel_placement(db: &RecordDb) -> PlacementId {
        let found = db.iter::<Cell>().find_map(|(_, c)| {
            let (&k, r) = c.references.iter().find(|(_, r)| r.id.eq_ignore_ascii_case("chargen barrel fatigue"))?;
            Some((format!("{:?} grid {:?} at {:?}", c.name, c.data.grid, r.translation), k))
        });
        let (cell, (source, refr_index)) = found.expect("the ring barrel is placed");
        eprintln!("ring barrel in {cell}");
        mw_world::ids::placement_id(&db.source(sf_records::SourceId(source)).name, refr_index)
    }

    /// Headless container window: activating the barrel (a [`TakeRequest`]) opens the window
    /// with its stacks; moving the ring puts it in the player's loadout and takes it out of the
    /// barrel, moving it back reverses that; Take all empties the barrel and closes the window.
    /// Returns the RPG host after and the barrel's store key.
    fn ring_through_the_window(db: &Arc<RecordDb>) -> (Rpg, ActorKey) {
        use crate::inventory_ui::{
            CONTAINER_SCREEN, OpenContainer, PcSlots, close_inventory, host_containers, open_containers, screens, sync_open,
        };
        use sf_ui_bevy::{ScreenStackRes, UiRegistryRes};
        use sf_ui_screens::{ContainerEvent, ContainerIntent, ContainerScreen};

        let target = barrel_placement(db);
        let (barrel, ring) = (RecordId::new("chargen barrel fatigue"), RecordId::new("ring_keley"));
        let mut app = App::new();
        app.add_message::<TakeRequest>()
            .add_message::<ContainerOpened>()
            .add_message::<ContainerEvent>()
            .insert_resource(Rpg::new(db.clone()).unwrap())
            .insert_resource(ItemRecords(db.clone()))
            .insert_resource(RefOwners::from_records(db))
            .insert_resource(UiRegistryRes(screens()))
            .init_resource::<ScreenStackRes>()
            .init_resource::<PcSlots>()
            .add_systems(Update, (apply_takes, open_containers, host_containers, sync_open, close_inventory).chain());
        app.world_mut().write_message(TakeRequest { player: PlayerId::HOST, target, record: barrel.clone(), kind: TakeKind::Container });
        app.update();

        let rows = |view: &sf_ui_inventory::InventoryView| -> Vec<(StackId, RecordId, u32)> {
            view.rows().iter().map(|(_, r)| (r.stack.id, r.stack.item.clone(), r.stack.count)).collect()
        };
        let window = |app: &mut App| {
            let mut q = app.world_mut().query::<(Entity, &ContainerScreen, &OpenContainer)>();
            q.iter(app.world()).map(|(e, s, o)| (e, rows(&s.container), rows(&s.player), (s.container.holder(), s.player.holder()), s.title.clone(), s.owned, o.clone())).next()
        };
        let ring_row = |rows: &[(StackId, RecordId, u32)]| rows.iter().find(|(_, item, _)| *item == ring).map(|&(id, _, n)| (id, n));
        let (screen, container, player, (from, to), title, owned, open) = window(&mut app).expect("the container window opened");
        assert!(app.world().resource::<ScreenStackRes>().0.is_open(CONTAINER_SCREEN));
        assert_eq!((open.key, open.record.clone(), open.organic), (ActorKey(target.0), barrel.clone(), false));
        assert_eq!((from, to), (Holder::Actor(open.key), Holder::Player(PlayerId::HOST)));
        assert_eq!(title, crate::inventory_ui::container_title(db, &barrel));
        eprintln!("barrel {title:?}, owned {owned}, {} stacks", container.len());
        assert_ne!(title, barrel.as_str(), "the record's name, not its id");
        let (stack, count) = ring_row(&container).expect("the barrel lists Fargoth's ring");
        assert_eq!(count, 1);
        assert!(ring_row(&player).is_none());

        // Barrel -> player.
        app.world_mut().write_message(ContainerEvent { screen, intent: ContainerIntent::Move { from, to, stack, count } });
        app.update();
        let (_, container, player, ..) = window(&mut app).expect("still open after a move");
        assert!(ring_row(&container).is_none(), "the ring left the barrel's pane");
        let (stack, n) = ring_row(&player).expect("the ring is in the player's pane");
        assert_eq!(n, 1);
        let rpg = app.world().resource::<Rpg>();
        assert_eq!(rpg.state.pc.loadout.count(&ring), 1);
        assert_eq!(rpg.state.inventories[&open.key].loadout.count(&ring), 0);

        // Player -> barrel (the barrel isn't organic), then Take all.
        app.world_mut().write_message(ContainerEvent { screen, intent: ContainerIntent::Move { from: to, to: from, stack, count: 1 } });
        app.update();
        let rpg = app.world().resource::<Rpg>();
        assert_eq!((rpg.state.pc.loadout.count(&ring), rpg.state.inventories[&open.key].loadout.count(&ring)), (0, 1), "put back");
        app.world_mut().write_message(ContainerEvent { screen, intent: ContainerIntent::TakeAll { from, to } });
        app.update();
        let rpg = app.world().resource::<Rpg>();
        assert!(rpg.state.inventories[&open.key].loadout.inventory().stacks().is_empty(), "Take all empties the barrel");
        assert_eq!(rpg.state.pc.loadout.count(&ring), 1);
        assert!(!app.world().resource::<ScreenStackRes>().0.is_open(CONTAINER_SCREEN), "Take all closes the window");
        assert!(window(&mut app).is_none(), "the window's root despawned");
        (app.world_mut().remove_resource::<Rpg>().unwrap(), open.key)
    }

    #[test]
    fn take_kinds_and_the_ring_barrel_finish_fargoths_ring() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
        let ring = RecordId::new("ring_keley");
        let barrel = RecordId::new("chargen barrel fatigue");
        assert_eq!(take_kind(&db, &ring), Some(TakeKind::Item));
        assert_eq!(take_kind(&db, &RecordId::new("misc_com_bottle_01")), Some(TakeKind::Item));
        assert_eq!(take_kind(&db, &barrel), Some(TakeKind::Container));
        assert_eq!(take_kind(&db, &RecordId::new("fargoth")), None);
        assert_eq!(take_kind(&db, &RecordId::new("ex_common_house_tall_02")), None);

        let (mut rpg, store) = ring_through_the_window(&db);
        assert!(take_all(&mut rpg.state, &db, PlayerId::HOST, store, &barrel).is_empty(), "the barrel stays empty");

        assert!(rpg.add_actor(FARGOTH, &RecordId::new("fargoth"), "Seyda Neen"));
        let (mut talk, mut view) = rpg.open(FARGOTH).unwrap();
        let topic = RecordId::new("ring");
        let quest = RecordId::new("ms_fargothring");
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(topic));
        let yes = view.choices()[1].value;
        rpg.request(&mut talk, &mut view, &DialogueRequest::Choose(yes));
        assert_eq!(rpg.state.world.quests.stage(&quest), 100);
        assert_eq!(rpg.state.pc.loadout.count(&ring), 0, "the ring went to Fargoth");
    }
}
