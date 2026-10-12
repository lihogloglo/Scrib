//! Synthetic records: part priorities, hiding, auto-equip, fallbacks, scale,
//! rigid mirroring, first person.

use std::sync::Arc;

use sf_character::{CharacterSource, CharacterState, Layer, PartKind, PartLookup, View, resolve_recipe};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{
    Armor, ArmorData, ArmorType, BipedObject, BipedObjectType as P, Bodypart, BodypartData, BodypartFlags, BodypartId as M, BodypartType, Class,
    Clothing, ClothingData, ClothingType, Creature, FixedString, GameSetting, Npc, NpcData, NpcFlags, NpcStats, Race, RaceData, RaceFlags,
    Skill, SkillId,
};

use crate::equip::{self, EquipRules, EquipSlot};
use crate::{MwActors, PartModel, part_id, wearable_id};

fn body(db: &mut RecordDb, src: sf_records::SourceId, id: &str, race: &str, part: M, female: bool, kind: BodypartType) {
    let flags = if female { BodypartFlags::FEMALE } else { BodypartFlags::empty() };
    let b = Bodypart {
        id: id.into(),
        race: race.into(),
        mesh: format!("b\\{id}.nif"),
        data: BodypartData { part, vampire: false, flags, bodypart_type: kind },
        ..Default::default()
    };
    db.define(src, RecordId::new(id), b).unwrap();
}

fn bo(t: P, male: &str, female: &str) -> BipedObject {
    BipedObject { biped_object_type: t, male_bodypart: male.into(), female_bodypart: female.into() }
}

fn armor(db: &mut RecordDb, src: sf_records::SourceId, id: &str, armor_type: ArmorType, rating: u32, parts: Vec<BipedObject>) {
    let data = ArmorData { armor_type, weight: 2.0, armor_rating: rating, ..Default::default() };
    db.define(src, RecordId::new(id), Armor { id: id.into(), biped_objects: parts, data, ..Default::default() }).unwrap();
}

fn clothing(db: &mut RecordDb, src: sf_records::SourceId, id: &str, clothing_type: ClothingType, value: u16, parts: Vec<BipedObject>) {
    let data = ClothingData { clothing_type, value, ..Default::default() };
    db.define(src, RecordId::new(id), Clothing { id: id.into(), biped_objects: parts, data, ..Default::default() }).unwrap();
}

fn npc(db: &mut RecordDb, src: sf_records::SourceId, id: &str, race: &str, female: bool, items: &[&str], skills: [u8; 27]) {
    let mut npc_flags = NpcFlags::empty();
    npc_flags.set(NpcFlags::FEMALE, female);
    let n = Npc {
        id: id.into(),
        race: race.into(),
        class: "guard".into(),
        head: if race == "Khajiit" { String::new() } else { format!("b_{race}_head") },
        hair: if race == "Khajiit" { String::new() } else { format!("b_{race}_hair") },
        npc_flags,
        inventory: items.iter().map(|i| (1, FixedString((*i).to_owned()))).collect(),
        data: NpcData { level: 5, stats: Some(NpcStats { skills, ..Default::default() }), ..Default::default() },
        ..Default::default()
    };
    db.define(src, RecordId::new(id), n).unwrap();
}

/// Armor skills 30 (ratings count at face value), unarmored 0.
fn guard_skills() -> [u8; 27] {
    let mut s = [0; 27];
    for k in [SkillId::LightArmor, SkillId::MediumArmor, SkillId::HeavyArmor] {
        s[k as usize] = 30;
    }
    s
}

fn content() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Npc>("NPC_").unwrap();
    db.register::<Creature>("CREA").unwrap();
    db.register::<Race>("RACE").unwrap();
    db.register::<Class>("CLAS").unwrap();
    db.register::<Skill>("SKIL").unwrap();
    db.register::<GameSetting>("GMST").unwrap();
    db.register::<Bodypart>("BODY").unwrap();
    db.register::<Armor>("ARMO").unwrap();
    db.register::<Clothing>("CLOT").unwrap();
    let src = db.add_source("test", &[]);
    let race = |id: &str, beast: bool| Race {
        id: id.into(),
        data: RaceData {
            height: [1.0, 0.95],
            weight: [1.1, 0.9],
            flags: if beast { RaceFlags::BEAST_RACE } else { RaceFlags::PLAYABLE },
            ..Default::default()
        },
        ..Default::default()
    };
    db.define(src, RecordId::new("Dark Elf"), race("Dark Elf", false)).unwrap();
    db.define(src, RecordId::new("Khajiit"), race("Khajiit", true)).unwrap();
    let skin = BodypartType::Skin;
    for (id, race, part, female) in [
        ("b_de_m_chest", "Dark Elf", M::Chest, false),
        ("b_de_m_groin", "Dark Elf", M::Groin, false),
        ("b_de_m_upperarm", "Dark Elf", M::UpperArm, false),
        ("b_de_m_forearm", "Dark Elf", M::Forearm, false),
        ("b_de_m_hands.1st", "Dark Elf", M::Hand, false),
        ("b_de_f_chest", "Dark Elf", M::Chest, true),
        ("b_kh_m_chest", "Khajiit", M::Chest, false),
        ("b_kh_m_tail", "Khajiit", M::Tail, false),
    ] {
        body(&mut db, src, id, race, part, female, skin);
    }
    body(&mut db, src, "b_Dark Elf_head", "Dark Elf", M::Head, false, skin);
    body(&mut db, src, "b_Dark Elf_hair", "Dark Elf", M::Hair, false, skin);
    let arm = BodypartType::Armor;
    let cl = BodypartType::Clothing;
    for (id, part, kind) in [
        ("a_cuirass", M::Chest, arm),
        ("a_helm", M::Head, arm),
        ("a_pauldron", M::Clavicle, arm),
        ("a_gauntlet", M::Hand, arm),
        ("a_gauntlet.1st", M::Hand, arm),
        ("a_bracer", M::Wrist, arm),
        ("a_boot", M::Foot, arm),
        ("c_shirt", M::Chest, cl),
        ("c_shirt_groin", M::Groin, cl),
        ("c_pants", M::Groin, cl),
        ("c_robe", M::Chest, cl),
        ("c_robe_arm", M::Forearm, cl),
        ("c_skirt", M::Groin, cl),
    ] {
        body(&mut db, src, id, "", part, false, kind);
    }
    armor(&mut db, src, "cuirass", ArmorType::Cuirass, 10, vec![bo(P::Chest, "a_cuirass", "")]);
    armor(&mut db, src, "weak_cuirass", ArmorType::Cuirass, 5, vec![bo(P::Chest, "a_cuirass", "")]);
    armor(&mut db, src, "helm", ArmorType::Helmet, 10, vec![bo(P::Head, "a_helm", "")]);
    armor(&mut db, src, "pauldron_l", ArmorType::LeftPauldron, 10, vec![bo(P::LeftPauldron, "a_pauldron", "")]);
    armor(&mut db, src, "pauldron_r", ArmorType::RightPauldron, 10, vec![bo(P::RightPauldron, "a_pauldron", "")]);
    armor(&mut db, src, "gauntlet_l", ArmorType::LeftGauntlet, 5, vec![bo(P::LeftHand, "a_gauntlet", "")]);
    armor(&mut db, src, "bracer_l", ArmorType::LeftBracer, 50, vec![bo(P::LeftWrist, "a_bracer", "")]);
    armor(&mut db, src, "boots", ArmorType::Boots, 10, vec![bo(P::LeftFoot, "a_boot", ""), bo(P::RightFoot, "a_boot", "")]);
    let shirt_parts = || vec![bo(P::Chest, "c_shirt", ""), bo(P::Groin, "c_shirt_groin", "")];
    clothing(&mut db, src, "shirt", ClothingType::Shirt, 5, shirt_parts());
    clothing(&mut db, src, "fine_shirt", ClothingType::Shirt, 50, shirt_parts());
    clothing(&mut db, src, "pants", ClothingType::Pants, 5, vec![bo(P::Groin, "c_pants", "")]);
    clothing(&mut db, src, "shoes", ClothingType::Shoes, 500, vec![bo(P::LeftFoot, "missing_shoe", "")]);
    clothing(&mut db, src, "robe", ClothingType::Robe, 5, vec![bo(P::Chest, "c_robe", ""), bo(P::LeftForearm, "c_robe_arm", "")]);
    clothing(&mut db, src, "skirt", ClothingType::Skirt, 5, vec![bo(P::Skirt, "c_skirt", "")]);
    let all = ["shirt", "cuirass", "helm", "pauldron_l", "pauldron_r", "gauntlet_l", "bracer_l", "boots", "pants", "shoes"];
    npc(&mut db, src, "guard", "Dark Elf", false, &all, guard_skills());
    npc(&mut db, src, "lady", "Dark Elf", true, &["shirt"], guard_skills());
    npc(&mut db, src, "cat", "Khajiit", false, &all, guard_skills());
    let crea = Creature { id: "guar".into(), mesh: "r\\Guar.NIF".into(), scale: Some(1.2), ..Default::default() };
    db.define(src, RecordId::new("guar"), crea).unwrap();
    db
}

/// Pauldrons and helmets are rigid; everything else skinned.
fn actors() -> MwActors {
    let probe = |p: &VfsPath| Some(PartModel::new(!(p.as_str().contains("pauldron") || p.as_str().contains("helm"))));
    MwActors::with_probe(Arc::new(content()), Arc::new(Vfs::builder().build()), Box::new(probe))
}

fn visible(a: &MwActors, worn: &[&str], id: &str, view: View) -> Vec<(String, String)> {
    let state = CharacterState { worn: Some(worn.iter().map(|w| RecordId::new(w)).collect()), held: None };
    let recipe = a.recipe("NPC_", &RecordId::new(id), &state).unwrap();
    let outfit = resolve_recipe(a, a.slots(), &recipe, view).unwrap();
    outfit.parts.iter().map(|(s, p)| (a.slots().name(*s).unwrap().to_owned(), p.as_str().to_owned())).collect()
}

fn has(v: &[(String, String)], slot: &str, part: &str) -> bool {
    v.iter().any(|(s, p)| s == slot && p == part)
}

fn shows(v: &[(String, String)], slot: &str) -> bool {
    v.iter().any(|(s, _)| s == slot)
}

#[test]
fn priorities_armor_over_clothing_robe_and_skirt_on_top() {
    let a = actors();
    let naked = visible(&a, &[], "guard", View::ThirdPerson);
    assert!(has(&naked, "chest", "b_de_m_chest#chest") && has(&naked, "hair", "b_dark elf_hair#hair"), "{naked:?}");
    assert!(has(&naked, "left_forearm", "b_de_m_forearm#left_forearm") && has(&naked, "right_forearm", "b_de_m_forearm#right_forearm"));

    // Armor (3) beats clothing (2) in the chest whatever the order; the shirt keeps the groin.
    for worn in [["shirt", "cuirass"], ["cuirass", "shirt"]] {
        let v = visible(&a, &worn, "guard", View::ThirdPerson);
        assert!(has(&v, "chest", "a_cuirass#chest") && has(&v, "pelvis", "c_shirt_groin#pelvis"), "{v:?}");
    }
    // Equal priority: the earlier slot (shirt before pants) wins the groin, in any equip order.
    for worn in [["pants", "shirt"], ["shirt", "pants"]] {
        assert!(has(&visible(&a, &worn, "guard", View::ThirdPerson), "pelvis", "c_shirt_groin#pelvis"));
    }
    // The robe (24) covers the cuirass and hides legs, upper arms, forearms (its own left forearm part shows).
    let robe = visible(&a, &["cuirass", "shirt", "robe"], "guard", View::ThirdPerson);
    assert!(has(&robe, "chest", "c_robe#chest") && has(&robe, "left_forearm", "c_robe_arm#left_forearm"), "{robe:?}");
    for hidden in ["right_forearm", "left_upper_arm", "right_upper_arm", "pelvis", "skirt"] {
        assert!(!shows(&robe, hidden), "{hidden}: {robe:?}");
    }
    // The skirt (8) hides the groin under it.
    let skirt = visible(&a, &["pants", "skirt"], "guard", View::ThirdPerson);
    assert!(has(&skirt, "skirt", "c_skirt#skirt") && !shows(&skirt, "pelvis"), "{skirt:?}");
    // A helmet hides the hair; a slot whose BODY is missing is hidden (reserved), not skin.
    let helm = visible(&a, &["helm"], "guard", View::ThirdPerson);
    assert!(has(&helm, "head", "a_helm#head") && !shows(&helm, "hair"), "{helm:?}");
    let shoes = a.wearable(&wearable_id("shoes", false)).unwrap();
    assert_eq!(shoes.hides, vec![a.slots().get("left_foot").unwrap()]);
    assert_eq!(a.wearable(&wearable_id("robe", false)).unwrap().layer, Layer::OUTER);
    assert_eq!(a.wearable(&wearable_id("skirt", false)).unwrap().layer, Layer(175));
}

#[test]
fn auto_equip_rules() {
    let a = actors();
    let db = a.db();
    let rules = EquipRules::from_db(db);
    let skills = guard_skills().map(f32::from);
    let ids = |v: &[(EquipSlot, equip::Item<'_>)]| v.iter().map(|(s, i)| (*s, i.id().to_owned())).collect::<Vec<_>>();
    let got = |items: &[&str], beast: bool| ids(&equip::auto_equip(db, &rules, items.iter().copied(), &skills, beast));
    // A gauntlet beats a bracer whatever the order and rating; the better cuirass wins.
    for items in [["gauntlet_l", "bracer_l"], ["bracer_l", "gauntlet_l"]] {
        assert_eq!(got(&items, false), vec![(EquipSlot::LeftGauntlet, "gauntlet_l".into())]);
    }
    assert_eq!(got(&["weak_cuirass", "cuirass"], false), vec![(EquipSlot::Cuirass, "cuirass".into())]);
    // Clothing: the more valuable shirt; shoes never replace boots.
    assert_eq!(got(&["shirt", "fine_shirt"], false), vec![(EquipSlot::Shirt, "fine_shirt".into())]);
    assert_eq!(got(&["shoes", "boots"], false), vec![(EquipSlot::Boots, "boots".into())]);
    // Beast races refuse helmets and boots; they take the shoes? No: shoes have a foot part too.
    let cat = got(&["helm", "boots", "shoes", "cuirass"], true);
    assert_eq!(cat, vec![(EquipSlot::Cuirass, "cuirass".into())]);
    // Armor no better than unarmored is left in the pack (unarmored 100: 0.1·100 × 0.065·100 = 65).
    let mut monk = skills;
    monk[SkillId::Unarmored as usize] = 100.0;
    assert!(equip::auto_equip(db, &rules, ["cuirass"], &monk, false).is_empty());

    // The guard's whole inventory: no shoes (boots won), the gauntlet, both pauldrons.
    let guard = a.recipe("NPC_", &RecordId::new("guard"), &CharacterState::default()).unwrap();
    let mut worn: Vec<&str> = guard.worn.iter().map(RecordId::as_str).collect();
    worn.sort_unstable();
    assert_eq!(worn, ["boots#m", "cuirass#m", "gauntlet_l#m", "helm#m", "pants#m", "pauldron_l#m", "pauldron_r#m", "shirt#m"]);
}

#[test]
fn beast_female_scale_and_skeletons() {
    let a = actors();
    let cat = a.recipe("NPC_", &RecordId::new("cat"), &CharacterState::default()).unwrap();
    assert_eq!(cat.skeleton.as_str(), "meshes/base_animkna.nif");
    assert!(cat.base.contains(&part_id("b_kh_m_tail", crate::slots::prt::TAIL)), "{:?}", cat.base);
    assert!(!cat.worn.iter().any(|w| w.as_str().starts_with("helm") || w.as_str().starts_with("boots")));

    // Female: female chest, male forearms as the fallback; female skeleton and scale.
    let lady = visible(&a, &[], "lady", View::ThirdPerson);
    assert!(has(&lady, "chest", "b_de_f_chest#chest") && has(&lady, "left_forearm", "b_de_m_forearm#left_forearm"), "{lady:?}");
    let r = a.recipe("NPC_", &RecordId::new("lady"), &CharacterState::default()).unwrap();
    assert_eq!(r.skeleton.as_str(), "meshes/base_anim_female.nif");
    assert_eq!(r.scale, glam::Vec3::new(0.9, 0.95, 0.9));
    assert_eq!(r.worn, vec![wearable_id("shirt", true)]);

    let guar = a.recipe("CREA", &RecordId::new("guar"), &CharacterState::default()).unwrap();
    assert_eq!(guar.skeleton.as_str(), "meshes/r/guar.nif");
    assert_eq!(guar.scale, glam::Vec3::splat(1.2));
    assert!(guar.head.is_none() && guar.first_person.is_none());
    assert_eq!(a.part(&guar.base[0]).unwrap().slot, a.slots().get("body").unwrap());
    assert!(a.recipe("NPC_", &RecordId::new("nobody"), &CharacterState::default()).is_err());
}

#[test]
fn rigid_parts_mirror_on_the_left_and_skinned_parts_filter() {
    let a = actors();
    let left = a.part(&part_id("a_pauldron", crate::slots::prt::L_PAULDRON)).unwrap();
    let right = a.part(&part_id("a_pauldron", crate::slots::prt::R_PAULDRON)).unwrap();
    let (PartKind::Rigid { joint: lj, offset: lo }, PartKind::Rigid { joint: rj, offset: ro }) = (&left.kind, &right.kind) else {
        panic!("{left:?} {right:?}");
    };
    assert_eq!((lj.as_str(), rj.as_str()), ("Left Clavicle", "Right Clavicle"));
    assert!(lo.matrix3.determinant() < 0.0 && ro.matrix3.determinant() > 0.0);
    assert_eq!(left.model.as_str(), "meshes/b/a_pauldron.nif");
    let fore = a.part(&part_id("b_de_m_forearm", crate::slots::prt::L_FOREARM)).unwrap();
    assert_eq!((&fore.kind, fore.node_filter.as_deref()), (&PartKind::Skinned, Some("Left Forearm")));
    assert!(fore.keeps_node(["Root", "Tri Left Forearm"]) && !fore.keeps_node(["Root", "Tri Right Forearm"]));
    let hair = a.part(&part_id("b_dark elf_hair", crate::slots::prt::HAIR)).unwrap();
    assert_eq!(hair.node_filter.as_deref(), Some("Hair"));
}

#[test]
fn first_person_view_model() {
    let a = actors();
    let bare = visible(&a, &[], "guard", View::FirstPerson);
    assert!(has(&bare, "left_hand", "b_de_m_hands.1st#left_hand"), "{bare:?}");
    // Third-person forearms stand in when no `.1st` part exists; chest never shows.
    assert!(has(&bare, "left_forearm", "b_de_m_forearm#left_forearm") && !shows(&bare, "chest"), "{bare:?}");
    let gloved = visible(&a, &["gauntlet_l", "cuirass"], "guard", View::FirstPerson);
    assert!(has(&gloved, "left_hand", "a_gauntlet.1st#left_hand"), "{gloved:?}");
}

#[test]
fn auto_calculated_skills() {
    let mut db = content();
    let src = db.add_source("more", &[]);
    let class = Class {
        id: "guard".into(),
        data: tes3::esp::ClassData { major1: SkillId::LightArmor, minor1: SkillId::Block, ..Default::default() },
        ..Default::default()
    };
    db.define(src, RecordId::new("guard"), class).unwrap();
    let mut n = db.get::<Npc>(&RecordId::new("guard")).unwrap().clone();
    n.npc_flags.insert(NpcFlags::AUTO_CALCULATE);
    let s = equip::npc_skills(&db, &n);
    // Level 5: major 5 + 25 + 4 × 1 = 34; minor 5 + 10 + 4 = 19; misc 5 + 0.4 → 5.
    assert_eq!((s[SkillId::LightArmor as usize], s[SkillId::Block as usize], s[SkillId::Alchemy as usize]), (34.0, 19.0, 5.0));
}

// Held items (`held`): weapon choice, ammunition, shields, torches at night, mounts.
mod held_items;
