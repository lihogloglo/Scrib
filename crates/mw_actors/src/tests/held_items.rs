//! Held items (`held`): weapon choice by skill and damage, ammunition,
//! shields, torches at night, mounts and visibility.

use sf_character::{Hand, Readiness};
use tes3::esp::{AtmosphereData, Cell, CellData, CellFlags, LeveledItem, Light, LightData, LightFlags, Weapon, WeaponData, WeaponType as W};

use super::*;
use crate::held::{self, CarriedLeft, DayCycle, HeldContext, bone};

fn weapon(db: &mut RecordDb, src: sf_records::SourceId, id: &str, weapon_type: W, damage: [u8; 3]) {
    let [chop_max, slash_max, thrust_max] = damage;
    let data = WeaponData { weapon_type, chop_max, slash_max, thrust_max, ..Default::default() };
    db.define(src, RecordId::new(id), Weapon { id: id.into(), mesh: format!("w\\{id}.nif"), data, ..Default::default() }).unwrap();
}

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Weapon>("WEAP").unwrap();
    db.register::<Armor>("ARMO").unwrap();
    db.register::<Light>("LIGH").unwrap();
    db.register::<Bodypart>("BODY").unwrap();
    db.register::<LeveledItem>("LEVI").unwrap();
    let src = db.add_source("test", &[]);
    weapon(&mut db, src, "bow", W::MarksmanBow, [40, 1, 1]);
    weapon(&mut db, src, "arrow_a", W::Arrow, [5, 0, 0]);
    weapon(&mut db, src, "arrow_b", W::Arrow, [5, 0, 0]);
    weapon(&mut db, src, "arrow_weak", W::Arrow, [2, 0, 0]);
    weapon(&mut db, src, "longsword", W::LongBladeOneHand, [5, 20, 10]);
    weapon(&mut db, src, "axe", W::AxeOneHand, [10, 5, 1]);
    weapon(&mut db, src, "big_axe", W::AxeTwoHand, [30, 5, 1]);
    weapon(&mut db, src, "same_axe", W::AxeOneHand, [30, 2, 2]);
    weapon(&mut db, src, "stars", W::MarksmanThrown, [3, 3, 3]);
    let shield = |id: &str, health: u32, parts: Vec<BipedObject>| Armor {
        id: id.into(),
        mesh: format!("a\\{id}_gnd.nif"),
        biped_objects: parts,
        data: ArmorData { armor_type: ArmorType::Shield, health, ..Default::default() },
        ..Default::default()
    };
    db.define(src, RecordId::new("buckler"), shield("buckler", 100, vec![])).unwrap();
    db.define(src, RecordId::new("tower"), shield("tower", 300, vec![bo(P::Shield, "a_tower_m", "a_tower_f")])).unwrap();
    db.define(src, RecordId::new("tower2"), shield("tower2", 300, vec![])).unwrap();
    body(&mut db, src, "a_tower_m", "", M::Forearm, false, BodypartType::Armor);
    let light = |id: &str, flags: LightFlags| Light {
        id: id.into(),
        mesh: format!("l\\{id}.nif"),
        data: LightData { radius: 128, color: [255, 128, 0, 0], flags, ..Default::default() },
        ..Default::default()
    };
    db.define(src, RecordId::new("lamp_fixed"), light("lamp_fixed", LightFlags::empty())).unwrap();
    db.define(
        src,
        RecordId::new("torch"),
        light("torch", LightFlags::CAN_CARRY | LightFlags::FLICKER | LightFlags::FIRE | LightFlags::OFF_BY_DEFAULT),
    )
    .unwrap();
    let list = |id: &str, items: &[(&str, u16)]| LeveledItem {
        id: id.into(),
        items: items.iter().map(|(i, l)| ((*i).to_owned(), *l)).collect(),
        ..Default::default()
    };
    db.define(src, RecordId::new("random_axe"), list("random_axe", &[("big_axe", 5), ("axe", 1), ("same_axe", 1)])).unwrap();
    db.define(src, RecordId::new("random_any"), list("random_any", &[("random_axe", 2), ("stars", 3)])).unwrap();
    db
}

fn skills(pairs: &[(SkillId, f32)]) -> [f32; 27] {
    let mut s = [5.0; 27];
    for (k, v) in pairs {
        s[*k as usize] = *v;
    }
    s
}

fn ids<'a>(w: (Option<&'a Weapon>, Option<&'a Weapon>)) -> (Option<&'a str>, Option<&'a str>) {
    (w.0.map(|w| w.id.as_str()), w.1.map(|w| w.id.as_str()))
}

#[test]
fn weapon_by_skill_then_damage_and_ammo() {
    let db = db();
    let archer = skills(&[(SkillId::Marksman, 70.0), (SkillId::Axe, 60.0), (SkillId::LongBlade, 60.0)]);
    // No arrows: the bow is skipped; long blade and axe tie at 60, long blade comes first.
    assert_eq!(ids(held::best_weapon(&db, &["bow", "axe", "longsword"], &archer)), (Some("longsword"), None));
    // Arrows: the bow, with the strongest arrow (ties: the later).
    assert_eq!(ids(held::best_weapon(&db, &["arrow_a", "bow", "arrow_weak", "arrow_b"], &archer)), (Some("bow"), Some("arrow_b")));
    // Within a skill: the highest chop/slash/thrust maximum; ties go to the later item.
    let axeman = skills(&[(SkillId::Axe, 50.0)]);
    assert_eq!(ids(held::best_weapon(&db, &["axe", "big_axe", "same_axe"], &axeman)), (Some("same_axe"), None));
    // Thrown weapons need no ammunition; skills at 0 are never tried.
    assert_eq!(ids(held::best_weapon(&db, &["stars"], &skills(&[(SkillId::Marksman, 30.0)]))), (Some("stars"), None));
    assert_eq!(ids(held::best_weapon(&db, &["axe"], &[0.0; 27])), (None, None));
    // Ammunition alone is not a weapon.
    assert_eq!(ids(held::best_weapon(&db, &["arrow_a"], &archer)), (None, None));
}

#[test]
fn shield_and_torch_share_the_left_hand() {
    let db = db();
    let axeman = skills(&[(SkillId::Axe, 50.0)]);
    let day = HeldContext::default();
    let night = HeldContext { dark: true, in_combat: false };
    let fight = HeldContext { dark: true, in_combat: true };
    let left = |items: &[&str], ctx| match held::auto_held(&db, items, &axeman, ctx).carried_left {
        Some(CarriedLeft::Shield(a)) => Some(a.id.clone()),
        Some(CarriedLeft::Light(l)) => Some(l.id.clone()),
        None => None,
    };
    // Best condition wins; ties keep the earlier.
    assert_eq!(left(&["buckler", "tower", "tower2"], day).as_deref(), Some("tower"));
    // A two-handed weapon (bows too) leaves no hand for a shield.
    assert_eq!(left(&["big_axe", "tower"], day), None);
    // Night: the torch replaces the shield, unless fighting; lights that can't be carried never.
    assert_eq!(left(&["tower", "lamp_fixed", "torch"], night).as_deref(), Some("torch"));
    assert_eq!(left(&["tower", "torch"], fight).as_deref(), Some("tower"));
    assert_eq!(left(&["torch"], fight).as_deref(), Some("torch"));
    assert_eq!(left(&["tower", "torch"], day).as_deref(), Some("tower"));
    assert_eq!(left(&["lamp_fixed"], night), None);
    // With a two-handed weapon the torch still goes in the free left hand.
    assert_eq!(left(&["big_axe", "torch"], night).as_deref(), Some("torch"));
}

#[test]
fn held_items_mount_on_the_weapon_and_shield_bones() {
    let db = db();
    let model = |mesh: &str| Some((VfsPath::new(&mw_world::model_path(mesh)?).ok()?, glam::Affine3A::IDENTITY));
    let axeman = skills(&[(SkillId::Axe, 50.0)]);
    // Female body without a female shield BODY: the male one.
    let l = held::auto_held(&db, &["axe", "tower"], &axeman, HeldContext::default());
    let shield_mesh = held::shield_mesh(&db, l.shield().unwrap(), true);
    assert_eq!(shield_mesh, "b\\a_tower_m.nif");
    let items = held::held_items(&l, model, Some(shield_mesh));
    let [axe, shield] = items.as_slice() else { panic!("{items:?}") };
    assert_eq!((axe.item.as_str(), axe.hand), ("axe", Hand::Main));
    assert_eq!(axe.mount(Readiness::Drawn).map(|m| m.socket.as_str()), Some(bone::WEAPON));
    assert!(axe.mount(Readiness::Sheathed).is_none(), "no visible sheaths in the original");
    assert_eq!((shield.model.as_str(), shield.hand), ("meshes/b/a_tower_m.nif", Hand::Off));
    for r in [Readiness::Drawn, Readiness::Sheathed] {
        assert_eq!(shield.mount(r).map(|m| m.socket.as_str()), Some(bone::SHIELD));
    }
    // A shield without a BODY shows its ground model.
    let buckler = db.get::<Armor>(&RecordId::new("buckler")).unwrap();
    assert_eq!(held::shield_mesh(&db, buckler, false), "a\\buckler_gnd.nif");
    // A torch next to a two-handed weapon hides while the weapon is drawn, and carries its light.
    let l = held::auto_held(&db, &["big_axe", "torch"], &axeman, HeldContext { dark: true, in_combat: false });
    let items = held::held_items(&l, model, None);
    let torch = &items[1];
    assert!(torch.mount(Readiness::Drawn).is_none());
    assert_eq!(torch.mount(Readiness::Sheathed).map(|m| m.socket.as_str()), Some(bone::SHIELD));
    let light = torch.light.as_ref().unwrap();
    assert_eq!(light.attach_node.as_deref(), Some(held::ATTACH_LIGHT));
    assert!((light.radius - 128.0 * 0.9144 / 64.0).abs() < 1e-4 && light.color.x > 0.99);
    // A `BoneOffset` node moves the model by its position, turned -90 degrees about X.
    let o = held::mount_offset(glam::Affine3A::IDENTITY, Some(glam::Vec3::new(0.0, 0.1, 0.0)));
    assert!(o.transform_point3(glam::Vec3::ZERO).abs_diff_eq(glam::Vec3::new(0.0, 0.1, 0.0), 1e-6));
    assert!(o.transform_vector3(glam::Vec3::Y).abs_diff_eq(glam::Vec3::NEG_Z, 1e-6));
}

#[test]
fn darkness_rule() {
    let c = DayCycle::default();
    assert!(held::exterior_dark(5.9, c) && !held::exterior_dark(6.0, c) && !held::exterior_dark(19.0, c) && held::exterior_dark(19.5, c));
    assert!(held::interior_dark([67, 67, 67], false) && !held::interior_dark([68, 67, 67], false) && !held::interior_dark([0, 0, 0], true));
    let cell = |flags: CellFlags| Cell {
        data: CellData { flags, ..Default::default() },
        atmosphere_data: Some(AtmosphereData { ambient_color: [30, 30, 40, 0], ..Default::default() }),
        ..Default::default()
    };
    let cave = cell(CellFlags::IS_INTERIOR);
    let shop = cell(CellFlags::IS_INTERIOR | CellFlags::RESTING_IS_ILLEGAL);
    let plaza = cell(CellFlags::IS_INTERIOR | CellFlags::BEHAVES_LIKE_EXTERIOR);
    let outside = cell(CellFlags::empty());
    assert!(held::is_dark(&cave, 12.0, c) && !held::is_dark(&shop, 23.0, c));
    assert!(
        !held::is_dark(&plaza, 12.0, c) && held::is_dark(&plaza, 23.0, c) && held::is_dark(&outside, 2.0, c) && !held::is_dark(&outside, 12.0, c)
    );
}

/// Scaffold: leveled item lists in an inventory hold their lowest-level item.
#[test]
fn leveled_lists_pick_the_lowest_level() {
    let db = db();
    assert_eq!(held::leveled_item_pick(&db, "random_axe"), Some("axe"));
    assert_eq!(held::leveled_item_pick(&db, "random_any"), Some("axe"));
    assert_eq!(held::leveled_item_pick(&db, "torch"), Some("torch"));
}
