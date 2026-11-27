//! `IN38b2` headless tests of the host stage hook: a removing break takes a live container out
//! once (hooks, overlay, visuals), leaves a standing stage alone, and an unloaded placement's
//! break removes it from the overlay alone. `IN38b3c`: the spill (every stack whole at no
//! overkill, thinned and worn by a vaporising one, the take reading the loose blob, an unloaded
//! container spilling into the overlay alone).

#![allow(clippy::default_trait_access, reason = "the crate has no math dependency to name IVec3 / Vec3")]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use mw_interaction::breakable::draw;
use mw_interaction::{LOOSE_STATE_OWNER, LooseItem, SpillParams, spill_draw};
use mw_scripts_bevy::{ActivePlacement, ActivePlacements, LiveRoot};
use sf_coords::WorldTransform;
use sf_interaction_bevy::Destroyed;
use sf_interaction_bevy::sf_interaction::{self, Tick};
use sf_physics_bevy::ItemMotion;
use sf_records::{RecordDb, RecordId};
use sf_rpg::{AttrValue, PlayerId};
use sf_world::{IndexedPlacement, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};
use sf_world_bevy::{ActivateCx, PlacementSpawner, PlacementSpawners, SpawnCx, WorldOverlay};
use tes3::esp::{Container, FixedString, MiscItem, Weapon};

use crate::breakable::{Removal, remove_broken};
use crate::dialogue::Rpg;
use crate::inventory::{RefOwners, TakeKind, TakeRequest};

const CRATE: PlacementId = PlacementId(80);
const FAR_CRATE: PlacementId = PlacementId(81);

/// The container class's spawner: counts deactivations.
struct Counting(Arc<AtomicU32>);

impl PlacementSpawner for Counting {
    fn spawn(&self, _: &mut World, _: &SpawnCx<'_>, _: &IndexedPlacement) -> u32 {
        0
    }
    fn deactivate(&self, _: &mut World, _: &ActivateCx<'_>, _: &IndexedPlacement) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

fn ws() -> WorldspaceId {
    WorldspaceId::new("test")
}

fn placed(id: PlacementId) -> IndexedPlacement {
    IndexedPlacement {
        placement: Placement {
            id,
            base: RecordRef::new("CONT", "contain_crate_01"),
            model: None,
            transform: WorldTransform::IDENTITY,
            class: mw_world::source::CONTAINER,
            flags: PlacementFlags::empty(),
        },
        radius: 0.5,
    }
}

/// The hook over a live crate in R0 (its `ActivePlacements` entry and a root with one visual
/// under it); returns (app, deactivation count, root, visual).
fn app() -> (App, Arc<AtomicU32>, Entity, Entity) {
    let deactivated = Arc::new(AtomicU32::new(0));
    let mut app = App::new();
    let mut spawners = PlacementSpawners::default();
    spawners.set(mw_world::source::CONTAINER, Arc::new(Counting(deactivated.clone())));
    app.insert_resource(spawners).init_resource::<ActivePlacements>().init_resource::<WorldOverlay>();
    crate::breakable::add(&mut app, |_| SpillParams::default());
    let world = app.world_mut();
    let cell_entity = world.spawn_empty().id();
    let root = world.spawn((LiveRoot(CRATE), ChildOf(cell_entity))).id();
    let visual = world.spawn(ChildOf(root)).id();
    let entry = ActivePlacement { placement: placed(CRATE), worldspace: ws(), cell: Default::default(), cell_entity };
    world.resource_mut::<ActivePlacements>().0.insert(CRATE, entry);
    (app, deactivated, root, visual)
}

/// One released break of `placement`'s first stage.
fn broke(placement: PlacementId, removes: bool) -> Destroyed {
    Destroyed(sf_interaction::Destroyed {
        tick: Tick(3),
        placement,
        stage: 0,
        point: Default::default(),
        dir: Default::default(),
        seed: 7,
        culprit: None,
        overkill: 0.0,
        removes,
    })
}

fn removed(app: &App, id: PlacementId) -> bool {
    app.world().resource::<WorldOverlay>().0.read().unwrap().is_removed(id)
}

#[test]
fn a_removing_break_takes_a_live_container_out_once() {
    let (mut app, deactivated, root, visual) = app();
    // A standing stage changes nothing.
    app.world_mut().write_message(broke(CRATE, false));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(deactivated.load(Ordering::Relaxed), 0);
    assert!(!removed(&app, CRATE) && app.world().get_entity(root).is_ok());

    // The removing stage: hooks once, overlay, root and visual; the same break again finds it gone.
    app.world_mut().write_message(broke(CRATE, true));
    app.world_mut().run_schedule(FixedUpdate);
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);
    assert!(removed(&app, CRATE));
    assert!(app.world().get_entity(root).is_err() && app.world().get_entity(visual).is_err());
    assert!(app.world().resource::<ActivePlacements>().0.is_empty());
    assert_eq!(remove_broken(app.world_mut(), CRATE), Removal::Gone);
    assert_eq!(deactivated.load(Ordering::Relaxed), 1);
}

#[test]
fn an_unloaded_placement_break_removes_it_from_the_overlay_only() {
    let (mut app, deactivated, root, _) = app();
    assert_eq!(remove_broken(app.world_mut(), FAR_CRATE), Removal::Unloaded);
    assert!(removed(&app, FAR_CRATE) && !removed(&app, CRATE));
    // No hook ran, the live crate kept its visuals.
    assert_eq!(deactivated.load(Ordering::Relaxed), 0);
    assert!(app.world().get_entity(root).is_ok());
    assert_eq!(remove_broken(app.world_mut(), FAR_CRATE), Removal::Gone);
    assert_eq!(remove_broken(app.world_mut(), CRATE), Removal::Live);
}

// -- IN38b3c: the spill ------------------------------------------------------------------------

/// `contain_crate_01` holds 25 gold, 10 bottles and an iron dagger of condition 400.
fn spill_db() -> Arc<RecordDb> {
    let mut db = RecordDb::new();
    mw_records::register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let inventory = [(25, "gold_001"), (10, "misc_bottle"), (1, "iron_dagger")].map(|(n, i)| (n, FixedString(i.into()))).to_vec();
    let crate_record = Container { id: "contain_crate_01".into(), inventory, ..Default::default() };
    db.define(src, RecordId::new("contain_crate_01"), crate_record).unwrap();
    for (id, mesh) in [("gold_001", "m\\gold.nif"), ("misc_bottle", "m\\bottle.nif")] {
        db.define(src, RecordId::new(id), MiscItem { id: id.into(), mesh: mesh.into(), ..Default::default() }).unwrap();
    }
    let mut dagger = Weapon { id: "iron_dagger".into(), mesh: "w\\dagger.nif".into(), ..Default::default() };
    dagger.data.health = 400;
    db.define(src, RecordId::new("iron_dagger"), dagger).unwrap();
    Arc::new(db)
}

/// [`app`] with the RPG host over [`spill_db`], the take, and `CRATE` owned.
fn spill_app() -> App {
    let (mut app, ..) = app();
    let db = spill_db();
    crate::inventory::add(&mut app, Some(db.clone()));
    app.insert_resource(Rpg::new(db).unwrap()).insert_resource(RefOwners([CRATE].into_iter().collect()));
    app
}

fn break_with(app: &mut App, placement: PlacementId, overkill: f32) {
    let mut b = broke(placement, true);
    b.0.overkill = overkill;
    app.world_mut().write_message(b);
    app.world_mut().run_schedule(FixedUpdate);
}

/// The overlay's spawned placements in `test`, by id: (item, loose blob, placement); each starts
/// awake at rest (the `physics` blob).
fn spilled(app: &App) -> Vec<(String, LooseItem, Placement)> {
    let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap();
    overlay
        .apply(&ws(), Partition::Single, Default::default(), &[])
        .into_iter()
        .filter(|p| overlay.is_spawned(p.placement.id))
        .map(|p| {
            let id = p.placement.id;
            assert_eq!(overlay.state(id, sf_physics_bevy::STATE_OWNER).and_then(ItemMotion::decode), Some(ItemMotion::DROPPED));
            let loose = LooseItem::decode(overlay.state(id, LOOSE_STATE_OWNER).unwrap()).unwrap();
            (p.placement.base.id.as_str().to_owned(), loose, p.placement)
        })
        .collect()
}

fn by_item(mut v: Vec<(String, LooseItem, Placement)>) -> Vec<(String, LooseItem)> {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v.into_iter().map(|(i, l, _)| (i, l)).collect()
}

#[test]
fn an_ordinary_smash_spills_every_stack_whole_and_owned() {
    let mut app = spill_app();
    break_with(&mut app, CRATE, 0.0);
    // The store is gone: nothing stays keyed by the broken placement.
    assert!(app.world().resource::<Rpg>().state.inventories.is_empty());
    let all = spilled(&app);
    for (item, _, p) in &all {
        assert_eq!(p.class, PlacementClass::Dynamic, "{item}");
        assert!(p.model.is_some(), "{item}");
        // Inside the former footprint, above the crate's origin.
        let d = WorldTransform::IDENTITY.pos.delta(p.transform.pos);
        assert!(d.x.hypot(d.z) <= 0.25 + 1e-6 && d.y >= 0.3 - 1e-6, "{item}: {d}");
    }
    let whole = |count| LooseItem { count, owned: true, health: None };
    let want = [("gold_001", whole(25)), ("iron_dagger", whole(1)), ("misc_bottle", whole(10))];
    assert_eq!(by_item(all), want.map(|(i, l)| (i.to_owned(), l)).to_vec());
}

#[test]
fn a_vaporising_hit_thins_the_stacks_wears_the_weapon_and_keeps_the_gold() {
    let mut app = spill_app();
    // Defaults: loss 0.9 at overkill 2.5 (10 bottles keep exactly 1); condition loss capped at
    // 0.75 of 400 = 300.
    break_with(&mut app, CRATE, 2.5);
    let owned = |count, health| LooseItem { count, owned: true, health };
    let mut want = vec![("gold_001".to_owned(), owned(25, None)), ("misc_bottle".to_owned(), owned(1, None))];
    // The single dagger (store index 2) is lost iff its draw is at least 1 - 0.9.
    if spill_draw(7, 2, draw::COUNT) < 0.1 {
        want.insert(1, ("iron_dagger".to_owned(), owned(1, Some(100))));
    }
    assert_eq!(by_item(spilled(&app)), want);
}

#[test]
fn taking_a_spilled_stack_gives_its_count_and_condition() {
    let mut app = spill_app();
    // At the knee: no piece lost, the dagger loses 0.5 x 0.5 x 400 = 100.
    break_with(&mut app, CRATE, 0.5);
    let all = spilled(&app);
    assert_eq!(all.len(), 3);
    for (item, loose, p) in &all {
        assert_eq!(loose.health, (item == "iron_dagger").then_some(300), "{item}");
        // The item went live (its streaming activation), then the player takes it.
        let placement = IndexedPlacement { placement: p.clone(), radius: 0.0 };
        let entry = ActivePlacement { placement, worldspace: ws(), cell: Default::default(), cell_entity: Entity::PLACEHOLDER };
        app.world_mut().resource_mut::<ActivePlacements>().0.insert(p.id, entry);
        let request = TakeRequest { player: PlayerId::HOST, target: p.id, record: RecordId::new(item), kind: TakeKind::Item };
        app.world_mut().write_message(request);
        app.world_mut().run_schedule(Update);
        let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap();
        assert!(overlay.state(p.id, LOOSE_STATE_OWNER).is_none() && overlay.placed(p.id).is_none(), "{item}: forgotten");
    }
    let pc = &app.world().resource::<Rpg>().state.pc.loadout;
    assert_eq!((pc.count(&RecordId::new("gold_001")), pc.count(&RecordId::new("misc_bottle"))), (25, 10));
    let dagger = pc.inventory().stacks().iter().find(|s| s.item.as_str() == "iron_dagger").unwrap();
    assert_eq!((dagger.count, dagger.attrs.get(&RecordId::new(mw_rpg::ATTR_HEALTH))), (1, Some(&AttrValue::Int(300))));
}

#[test]
fn an_unloaded_container_spills_into_the_overlay_only() {
    let mut app = spill_app();
    // A spawned crate (it carries its transform in the overlay) whose cell left R0.
    let at = WorldTransform::IDENTITY.pos.offset([10.0, 0.0, 0.0].into());
    let crate_at = Placement { transform: WorldTransform::from_pos(at), ..placed(CRATE).placement };
    let far = app.world().resource::<WorldOverlay>().0.write().unwrap().spawn(ws(), crate_at);
    break_with(&mut app, far, 0.0);
    assert!(app.world().resource::<WorldOverlay>().0.read().unwrap().placed(far).is_none(), "the crate is gone");
    let all = spilled(&app);
    assert_eq!(all.len(), 3);
    for (item, loose, p) in &all {
        // Not owned (a spawned crate has no reference owner), around its own position.
        assert!(!loose.owned, "{item}");
        let d = at.delta(p.transform.pos);
        assert!(d.x.hypot(d.z) <= 0.25 + 1e-6, "{item}: {d}");
    }
    // The live crate was untouched: its store was never seeded.
    assert!(app.world().resource::<ActivePlacements>().0.contains_key(&CRATE));
    assert!(app.world().resource::<Rpg>().state.inventories.is_empty());
}
