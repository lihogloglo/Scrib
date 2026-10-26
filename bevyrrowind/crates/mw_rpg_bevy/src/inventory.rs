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
//! - **Containers** (CONT): scaffold `TakeAllContainer`. Activating one moves its whole content
//!   to the player (vanilla opens the container window; "Take All" is its button). The content
//!   lives in `RpgState::inventories` under the container's placement id (saved with the RPG
//!   state), seeded from the record's item list the first time. Levelled entries are skipped.
//!   Removal path: the container screen (2.2 U5 follow-up) takes per stack from the same state.
//!   Fargoth's ring (`ring_keley`) is in the `chargen barrel fatigue` barrel in the Census and
//!   Excise Office, so this is the ring's path.
//! - Not yet (leftovers): weight and encumbrance, drop, equip, ownership and theft (the Census
//!   office items are owned: vanilla counts the take as stealing), item scripts' `OnActivate`
//!   (the ring's `CharGen_ring_keley` shows the magic menu), books opening to read, gold piles
//!   (`OpenMW`'s `ActionTake` multiplies a `Gold_*` reference's count by its value).

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_log::prelude::*;
use sf_physics_bevy::ItemPivot;
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, Attrs, CharacterRpg, Holder, PlayerId, RpgCommand};
use sf_script_bevy::{ActivationOutcome, PLAYER, ScriptPlacements, ScriptSlot};
use sf_world::PlacementId;
use sf_world_bevy::WorldOverlay;
use tes3::esp::{
    Alchemy, Apparatus, Armor, Book, Cell, Clothing, Container, Ingredient, LeveledItem, Light, LightFlags, Lockpick,
    MiscItem, Probe, Reference, RepairItem, Weapon,
};

use crate::dialogue::{Rpg, RpgState};
use mw_scripts_bevy::ActivePlacements;

/// What activating a record takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TakeKind {
    /// A loose item: the placement itself.
    Item,
    /// A container: everything in it (scaffold `TakeAllContainer`).
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

/// A player asks to take what a placement holds (net rule 2: an intent, validated by the host).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct TakeRequest {
    pub player: PlayerId,
    pub target: PlacementId,
    pub record: RecordId,
    pub kind: TakeKind,
}

/// After `dialogue::add` (the `Rpg` host) and `scripts::add` (activations).
pub fn add(app: &mut App, db: Option<Arc<RecordDb>>) {
    let Some(db) = db else { return };
    app.insert_resource(RefCounts::from_records(&db))
        .insert_resource(ItemRecords(db))
        .add_message::<TakeRequest>()
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
/// goes into the player's loadout; a container's content moves to the player.
fn apply_takes(world: &mut World, reader: &mut SystemState<MessageReader<'static, 'static, TakeRequest>>) {
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
                let mut rpg = world.resource_mut::<Rpg>();
                give(&mut rpg.state, request.player, &request.record, count);
                info!("take: {count} {} (now {})", request.record, rpg.state.pc.loadout.count(&request.record));
            }
            TakeKind::Container => {
                let db = world.resource::<ItemRecords>().0.clone();
                let mut rpg = world.resource_mut::<Rpg>();
                let taken = take_all(&mut rpg.state, &db, request.player, ActorKey(request.target.0), &request.record);
                info!("take all from {}: {taken:?}", request.record);
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

/// Adds `count` of `item` to `player`'s loadout (one player character until co-op N slices).
fn give(state: &mut RpgState, player: PlayerId, item: &RecordId, count: u32) {
    let command = RpgCommand::AddItem { holder: Holder::Player(player), item: item.clone(), count, attrs: Attrs::new() };
    if let Err(e) = state.pc.apply(command) {
        warn!("take {item}: {e:?}");
    }
}

/// Scaffold `TakeAllContainer`: moves everything in `container` (seeded from the `record`'s
/// item list on first access) to `player`. Returns what moved.
pub fn take_all(state: &mut RpgState, db: &RecordDb, player: PlayerId, container: ActorKey, record: &RecordId) -> Vec<(RecordId, u32)> {
    let slot = state.inventories.entry(container).or_insert_with(|| {
        let mut seeded = CharacterRpg::default();
        for (count, item) in db.get::<Container>(record).map(|c| c.inventory.as_slice()).unwrap_or_default() {
            let item = RecordId::new(item.as_str());
            if db.get::<LeveledItem>(&item).is_some() {
                info!("take all from {record}: levelled list {item} skipped (no levelled items yet)");
                continue;
            }
            let _ = seeded.loadout.add(item, count.unsigned_abs(), Attrs::new());
        }
        seeded
    });
    let held = std::mem::take(slot);
    let taken: Vec<(RecordId, u32)> = held.loadout.inventory().stacks().iter().map(|s| (s.item.clone(), s.count)).collect();
    for (item, count) in &taken {
        give(state, player, item, *count);
    }
    taken
}

#[cfg(test)]
mod tests {
    //! The ring path on real data (skips without a Morrowind install): the Census office barrel
    //! holds Fargoth's ring, taking it lets Fargoth's ring dialogue finish the quest.
    use super::*;
    use mw_install::MorrowindInstall;
    use sf_rpg_ui::DialogueRequest;

    const FARGOTH: ActorKey = ActorKey(1);
    const BARREL: ActorKey = ActorKey(7);

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

        let mut rpg = Rpg::new(db.clone()).unwrap();
        let taken = take_all(&mut rpg.state, &db, PlayerId::HOST, BARREL, &barrel);
        assert!(taken.contains(&(ring.clone(), 1)), "{taken:?}");
        assert_eq!(rpg.state.pc.loadout.count(&ring), 1);
        assert!(take_all(&mut rpg.state, &db, PlayerId::HOST, BARREL, &barrel).is_empty(), "the barrel stays empty");

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
