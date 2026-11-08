//! Exact formula tests against `OpenMW` (`ca5e2576`), vanilla GMSTs, synthetic records. Expected
//! numbers are worked by hand from the `OpenMW` source, not from this crate.

// Exact equality on purpose: the expected values are `OpenMW`'s, computed in the same f32 order.
#![allow(clippy::float_cmp)]

use sf_records::{RecordDb, RecordId};
use sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, Inputs, Layer, ModOp, Modifier, Role, Rules, SourceId, SourceKind, StatOp,
    StatOwner, TestModel,
};
use tes3::esp::{
    AttributeId, Class, ClassData, Creature, CreatureData, Npc, NpcData, NpcFlags, NpcStats, Race, RaceData,
    SkillBonuses, SkillId, Specialization,
};

use crate::*;

/// Vanilla SKIL governing attribute and specialization, by ESM index.
pub(crate) const VANILLA_SKILLS: [(usize, usize); 27] = [
    (3, 0), // block: agility, combat
    (0, 0), // armorer
    (5, 0), // mediumarmor
    (5, 0), // heavyarmor
    (0, 0), // bluntweapon
    (0, 0), // longblade
    (0, 0), // axe
    (5, 0), // spear
    (4, 0), // athletics
    (1, 1), // enchant
    (2, 1), // destruction
    (2, 1), // alteration
    (6, 1), // illusion
    (1, 1), // conjuration
    (2, 1), // mysticism
    (2, 1), // restoration
    (1, 1), // alchemy
    (4, 1), // unarmored
    (1, 2), // security
    (3, 2), // sneak
    (0, 2), // acrobatics
    (3, 2), // lightarmor
    (4, 2), // shortblade
    (3, 2), // marksman
    (6, 2), // mercantile
    (6, 2), // speechcraft
    (4, 2), // handtohand
];

pub(crate) fn table() -> MwTable {
    MwTable::with_skills(VANILLA_SKILLS.map(|(governing, specialization)| SkillInfo {
        governing,
        specialization,
        use_values: [1.0, 0.0, 0.0, 0.0],
    }))
    .unwrap()
}

fn rules(t: &MwTable) -> Rules {
    Rules {
        derived: Box::new(MwDerived { gmst: MwGmst::vanilla(), idx: t.idx.clone() }),
        skill_use: Box::new(TestModel::default()),
        leveling: Box::new(TestModel::default()),
    }
}

const OWNER: StatOwner = StatOwner::Actor(ActorKey(1));

/// Redguard-like male: Str 50 Int 30 Wil 30 Agi 40 Spd 40 End 50 Per 30 Luck 40; Long Blade +15,
/// five combat skills +5.
pub(crate) fn redguard() -> Race {
    use SkillId as S;
    Race {
        id: "Redguard".into(),
        data: RaceData {
            strength: [50, 40],
            intelligence: [30, 30],
            willpower: [30, 30],
            agility: [40, 40],
            speed: [40, 40],
            endurance: [50, 50],
            personality: [30, 40],
            luck: [40, 40],
            skill_bonuses: SkillBonuses {
                skill_0: S::LongBlade,
                bonus_0: 15,
                skill_1: S::HeavyArmor,
                bonus_1: 5,
                skill_2: S::MediumArmor,
                bonus_2: 5,
                skill_3: S::Spear,
                bonus_3: 5,
                skill_4: S::BluntWeapon,
                bonus_4: 5,
                skill_5: S::Axe,
                bonus_5: 5,
                skill_6: S::ShortBlade,
                bonus_6: 5,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Warrior-like: favoured Str, End; combat; majors Long Blade, Block, Medium, Heavy, Athletics;
/// minors Spear, Armorer, Axe, Blunt, Hand-to-hand.
pub(crate) fn warrior() -> Class {
    use SkillId as S;
    Class {
        id: "Warrior".into(),
        data: ClassData {
            attribute1: AttributeId::Strength,
            attribute2: AttributeId::Endurance,
            specialization: Specialization::Combat,
            major1: S::LongBlade,
            major2: S::Block,
            major3: S::MediumArmor,
            major4: S::HeavyArmor,
            major5: S::Athletics,
            minor1: S::Spear,
            minor2: S::Armorer,
            minor3: S::Axe,
            minor4: S::BluntWeapon,
            minor5: S::HandToHand,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub(crate) fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Race>("RACE").unwrap();
    db.register::<Class>("CLAS").unwrap();
    let src = db.add_source("test", &[]);
    db.define(src, RecordId::new("Redguard"), redguard()).unwrap();
    db.define(src, RecordId::new("Warrior"), warrior()).unwrap();
    db
}

fn autocalc_npc(level: i16) -> Npc {
    Npc {
        id: "guard".into(),
        race: "Redguard".into(),
        class: "Warrior".into(),
        npc_flags: NpcFlags::AUTO_CALCULATE,
        data: NpcData { level, reputation: 7, ..Default::default() },
        ..Default::default()
    }
}

fn sheet(t: &MwTable, ops: &[StatOp], player: bool) -> sf_stats::Sheet {
    let inputs = Inputs { is_player: player, ..Default::default() };
    let (s, rejected) = new_sheet(t, &rules(t), OWNER, &inputs, ops);
    assert_eq!(rejected, 0);
    s
}

fn attrs(t: &MwTable, s: &sf_stats::Sheet) -> [f32; 8] {
    t.idx.attributes.map(|i| s.view(&t.table).base(i))
}

fn skill(t: &MwTable, s: &sf_stats::Sheet, id: SkillId) -> f32 {
    s.view(&t.table).base(t.idx.skills[id as usize])
}

#[test]
fn table_has_the_format_stats_and_roles() {
    let t = table();
    assert_eq!(t.table.len(), 8 + 27 + 1 + 3 + 4 + 27 + 8 + 3 + 1 + 3 + 27 + 1 + 3 + 22);
    for (i, a) in ATTRIBUTES.iter().enumerate() {
        assert_eq!(t.table.find(a), Some(t.idx.attributes[i]));
        assert_eq!(usize::from(t.idx.attributes[i].0), i);
    }
    assert_eq!(t.table.find("LongBlade"), Some(t.idx.skills[5]));
    assert_eq!(t.table.role(Role::Health), Some(t.idx.health));
    assert_eq!(t.table.role(Role::Mana), Some(t.idx.magicka));
    assert_eq!(t.table.role(Role::Stamina), Some(t.idx.fatigue));
    assert_eq!(t.table.role(Role::Level), Some(t.idx.level));
    assert_eq!(t.table.role(Role::Bounty), Some(t.idx.bounty));
    assert_eq!(t.table.role(Role::CarryCapacity), Some(t.idx.capacity));
    assert_eq!(t.table.role(Role::Encumbrance), Some(t.idx.encumbrance));
}

/// The 22 magnitude stats (2026-11-08) are appended: every older index is where it was (MwIdx
/// fields are positional), and each magnitude key resolves to a base-0 `Value` in `magnitude`.
#[test]
fn magnitude_stats_are_appended_after_the_older_indices() {
    let t = table();
    let i = &t.idx;
    let pos = |s: sf_stats::StatIdx| usize::from(s.0);
    assert_eq!((pos(i.attributes[0]), pos(i.skills[0]), pos(i.skills[26])), (0, 8, 34));
    assert_eq!((pos(i.fortify_max_magicka), pos(i.health), pos(i.magicka), pos(i.fatigue)), (35, 36, 37, 38));
    assert_eq!((pos(i.level), pos(i.reputation), pos(i.bounty), pos(i.level_progress)), (39, 40, 41, 42));
    assert_eq!((pos(i.skill_progress[0]), pos(i.levelup_attr[0]), pos(i.levelup_spec[0])), (43, 70, 78));
    assert_eq!((pos(i.werewolf_kills), pos(i.magicka_offset), pos(i.fatigue_offset), pos(i.creature)), (81, 82, 83, 84));
    assert_eq!((pos(i.class_skills[0]), pos(i.class_specialization)), (85, 112));
    assert_eq!((pos(i.capacity), pos(i.encumbrance), pos(i.fatigue_term)), (113, 114, 115));
    for (n, (key, label)) in MAGNITUDE_STATS.iter().enumerate() {
        let s = t.table.find(key).unwrap_or_else(|| panic!("{key} not in MwTable"));
        assert_eq!(s, i.magnitudes[n], "{key}");
        assert_eq!(pos(s), 116 + n, "{key}");
        let def = t.table.def(s);
        assert_eq!((def.group.as_str(), def.label.as_str(), def.public), ("magnitude", *label, false), "{key}");
        assert!(matches!(def.kind, sf_stats::StatKind::Value { min, cap: None } if min == 0.0), "{key}");
    }
    assert_eq!(t.table.find("ResistFire"), Some(i.magnitudes[12]));
}

#[test]
fn round_half_even_matches_round_ieee_754() {
    assert_eq!([0.5, 1.5, 2.5, 82.5, 63.5, 84.3, -0.5, -1.5].map(round_half_even), [0.0, 2.0, 2.0, 82.0, 64.0, 84.0, -0.0, -2.0]);
}

/// `autoCalculateAttributes` at level 10: race + favoured, + 9 x Σ governed (1 major / 0.5 minor
/// / 0.2 misc), half to even (End 82.5 -> 82); health floor(0.5 x (84 + 82)) + 6 x 9.
#[test]
fn autocalc_attributes_and_health_level_10_warrior() {
    let (t, db) = (table(), db());
    let s = sheet(&t, &npc_ops(&t, &MwGmst::vanilla(), &db, &autocalc_npc(10)).unwrap(), false);
    assert_eq!(attrs(&t, &s), [84.0, 37.0, 37.0, 54.0, 57.0, 82.0, 35.0, 40.0]);
    let v = s.view(&t.table);
    assert_eq!(v.max(t.idx.health), 137.0);
    assert_eq!(v.current(t.idx.health), 137.0);
    // fNPCbaseMagickaMult 2 x Int 37; fatigue Str + Wil + Agi + End.
    assert_eq!((v.max(t.idx.magicka), v.current(t.idx.magicka)), (74.0, 74.0));
    assert_eq!((v.max(t.idx.fatigue), v.current(t.idx.fatigue)), (257.0, 257.0));
    assert_eq!((v.base(t.idx.level), v.base(t.idx.reputation)), (10.0, 7.0));
    assert_eq!(v.modified(t.idx.capacity), 84.0 * 5.0);
    assert_eq!(v.modified(t.idx.fatigue_term), 1.25);
}

/// `autoCalculateSkills` at level 10: +25 major / +10 minor, 5 + race + 5 same specialization +
/// 9 x (1 class skill else 0.1, + 0.5 same specialization), half to even.
#[test]
fn autocalc_skills_level_10_warrior() {
    use SkillId as S;
    let (t, db) = (table(), db());
    let s = sheet(&t, &npc_ops(&t, &MwGmst::vanilla(), &db, &autocalc_npc(10)).unwrap(), false);
    for (id, want) in [
        (S::LongBlade, 64.0), // 25 + 5 + 15 + 5 + 13.5 = 63.5
        (S::Block, 48.0),     // 48.5
        (S::MediumArmor, 54.0),
        (S::HeavyArmor, 54.0),
        (S::Athletics, 48.0),
        (S::Spear, 38.0), // 10 + 5 + 5 + 5 + 13.5 = 38.5
        (S::Armorer, 34.0),
        (S::Axe, 38.0),
        (S::BluntWeapon, 38.0),
        (S::HandToHand, 24.0), // 10 + 5 + 9 (stealth: no spec bonus)
        (S::ShortBlade, 11.0), // 5 + 5 + 0.9
        (S::Destruction, 6.0),
        (S::Mercantile, 6.0),
    ] {
        assert_eq!(skill(&t, &s, id), want, "{id:?}");
    }
}

/// Level 0 ("must gracefully handle level 0"): (level - 1) = -1 lowers the values, nothing NaN.
#[test]
fn autocalc_level_0_is_finite() {
    let (t, db) = (table(), db());
    let s = sheet(&t, &npc_ops(&t, &MwGmst::vanilla(), &db, &autocalc_npc(0)).unwrap(), false);
    let v = s.view(&t.table);
    assert_eq!(skill(&t, &s, SkillId::LongBlade), 48.0); // 25 + 5 + 15 + 5 - 1.5 = 48.5 -> 48
    assert_eq!(attrs(&t, &s)[0], 57.0); // 60 - 2.7 = 57.3
    assert!(t.table.defs().all(|(i, _)| v.modified(i).is_finite()));
}

#[test]
fn autocalc_without_race_or_class_is_refused() {
    let (t, db) = (table(), db());
    let mut npc = autocalc_npc(5);
    npc.race = "Nord".into();
    assert!(matches!(npc_ops(&t, &MwGmst::vanilla(), &db, &npc), Err(InitError::MissingRace { .. })));
    npc.race = "Redguard".into();
    npc.class = "Nobody".into();
    assert!(matches!(npc_ops(&t, &MwGmst::vanilla(), &db, &npc), Err(InitError::MissingClass { .. })));
}

fn stored_npc() -> Npc {
    let mut skills = [10u8; 27];
    skills[SkillId::LongBlade as usize] = 70;
    Npc {
        id: "hlaalu_guard".into(),
        faction: "Hlaalu".into(),
        data: NpcData {
            level: 5,
            reputation: 40,
            rank: 3,
            stats: Some(NpcStats {
                attributes: [60, 40, 30, 50, 50, 60, 30, 40],
                skills,
                health: 90,
                magicka: 100,
                fatigue: 300,
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Stored NPDT values (no autocalc flag): exactly the record; faction member reputation =
/// iAutoRepFacMod x (rank + 1) + iAutoRepLevMod x (level - 1).
#[test]
fn stored_npc_takes_its_record_and_rank() {
    let t = table();
    let s = sheet(&t, &npc_ops(&t, &MwGmst::vanilla(), &RecordDb::new(), &stored_npc()).unwrap(), false);
    let v = s.view(&t.table);
    assert_eq!(attrs(&t, &s), [60.0, 40.0, 30.0, 50.0, 50.0, 60.0, 30.0, 40.0]);
    assert_eq!(skill(&t, &s, SkillId::LongBlade), 70.0);
    assert_eq!(skill(&t, &s, SkillId::Sneak), 10.0);
    assert_eq!([t.idx.health, t.idx.magicka, t.idx.fatigue].map(|p| v.max(p)), [90.0, 100.0, 300.0]);
    assert_eq!([t.idx.health, t.idx.magicka, t.idx.fatigue].map(|p| v.current(p)), [90.0, 100.0, 300.0]);
    // Offsets from the formula: magicka 100 - 2 x 40, fatigue 300 - (60 + 30 + 50 + 60).
    assert_eq!((v.base(t.idx.magicka_offset), v.base(t.idx.fatigue_offset)), (20.0, 100.0));
    let g = MwGmst::vanilla();
    assert_eq!(v.base(t.idx.reputation), g.auto_rep_fac_mod * 4.0 + g.auto_rep_lev_mod * 4.0);
    assert_eq!(v.membership(&RecordId::new("hlaalu")).and_then(|m| m.rank), Some(3));
}

/// Fortify Intelligence 10 on the stored NPC: magicka max 2 x 50 + 20 = 120, ratio kept;
/// Fortify Strength 10 on 50/200-like fatigue keeps the ratio too (stats-adapter.md "Tests").
#[test]
fn record_pools_follow_attributes_by_delta() {
    let t = table();
    let r = rules(&t);
    let inputs = Inputs::default();
    let (mut s, _) = new_sheet(&t, &r, OWNER, &inputs, &npc_ops(&t, &MwGmst::vanilla(), &RecordDb::new(), &stored_npc()).unwrap());
    let ctx = ApplyCtx { owner: OWNER, cause: Cause { kind: CauseKind::Effect, source: None, tick: 1 }, inputs: &inputs };
    let source = SourceId { kind: SourceKind::Effect, id: 9 };
    let fortify = |stat| StatOp::AddModifier(Modifier { stat, layer: Layer::Temporary, op: ModOp::Add, value: 10.0, source });
    sf_stats::apply(&t.table, &r, &mut s, &[
        StatOp::PoolAdjust { pool: t.idx.fatigue, delta: -225.0, below_zero: false, above_max: false },
    ], &ctx, &mut Vec::new());
    sf_stats::apply(&t.table, &r, &mut s, &[fortify(t.idx.attributes[1]), fortify(t.idx.attributes[0])], &ctx, &mut Vec::new());
    let v = s.view(&t.table);
    assert_eq!((v.max(t.idx.magicka), v.current(t.idx.magicka)), (120.0, 120.0));
    assert_eq!((v.max(t.idx.fatigue), v.current(t.idx.fatigue)), (310.0, 77.5));
    assert_eq!(v.modified(t.idx.capacity), 350.0);
    // The fatigue term follows the pool current (1.25 - 0.5 x (1 - 0.25)).
    assert_eq!(v.modified(t.idx.fatigue_term), 0.875);
}

/// Creature: CREA attributes and pools exactly; each skill takes its specialization's value.
#[test]
fn creature_takes_crea_values() {
    let t = table();
    let c = Creature {
        id: "guar".into(),
        data: CreatureData {
            level: 3,
            strength: 40,
            intelligence: 10,
            willpower: 20,
            agility: 30,
            speed: 50,
            endurance: 40,
            personality: 5,
            luck: 30,
            health: 45,
            magicka: 0,
            fatigue: 150,
            combat: 30,
            magic: 5,
            stealth: 20,
            ..Default::default()
        },
        ..Default::default()
    };
    let s = sheet(&t, &creature_ops(&t, &MwGmst::vanilla(), &c), false);
    let v = s.view(&t.table);
    assert_eq!(attrs(&t, &s), [40.0, 10.0, 20.0, 30.0, 50.0, 40.0, 5.0, 30.0]);
    assert_eq!([t.idx.health, t.idx.magicka, t.idx.fatigue].map(|p| v.max(p)), [45.0, 0.0, 150.0]);
    assert_eq!(skill(&t, &s, SkillId::Axe), 30.0);
    assert_eq!(skill(&t, &s, SkillId::Alchemy), 5.0);
    assert_eq!(skill(&t, &s, SkillId::Sneak), 20.0);
    assert_eq!(v.base(t.idx.level), 3.0);
    assert_eq!(v.modified(t.idx.capacity), 200.0); // Str x 5 for creatures
}

/// `buildPlayer`: race base, 5 + race bonus skills, class +10 favoured, +25 major, +10 minor, +5
/// specialization; health floor(0.5 x (Str + End)); magicka with fPCbaseMagickaMult (1) and the
/// race's Fortify Maximum Magicka ability (added as the effect runtime will): Altmer-like 15 ->
/// (1 + 1.5) x 50 = 125, Breton-like 5 -> 75.
#[test]
fn chargen_builds_the_player() {
    use SkillId as S;
    let t = table();
    let race = Race {
        data: RaceData {
            strength: [30, 30],
            intelligence: [50, 50],
            willpower: [40, 40],
            agility: [40, 40],
            speed: [30, 30],
            endurance: [40, 40],
            personality: [40, 40],
            luck: [40, 40],
            skill_bonuses: SkillBonuses { skill_0: S::Destruction, bonus_0: 10, ..Default::default() },
            ..Default::default()
        },
        ..Default::default()
    };
    let class = Class {
        data: ClassData {
            attribute1: AttributeId::Willpower,
            attribute2: AttributeId::Endurance,
            specialization: Specialization::Magic,
            major1: S::Destruction,
            minor1: S::Alchemy,
            minor2: S::ShortBlade,
            ..Default::default()
        },
        ..Default::default()
    };
    let player = Npc { id: "player".into(), data: NpcData { level: 1, ..Default::default() }, ..Default::default() };
    let ops = chargen_ops(&t, &MwGmst::vanilla(), &player, Chargen { race: Some(&race), female: false, class: Some(&class) });
    for (ability, magicka) in [(15.0, 125.0), (5.0, 75.0)] {
        let mut ops = ops.clone();
        ops.push(StatOp::AddModifier(Modifier {
            stat: t.idx.fortify_max_magicka,
            layer: Layer::Permanent,
            op: ModOp::Add,
            value: ability,
            source: SourceId { kind: SourceKind::Ability, id: 1 },
        }));
        let s = sheet(&t, &ops, true);
        let v = s.view(&t.table);
        assert_eq!(attrs(&t, &s), [30.0, 50.0, 50.0, 40.0, 30.0, 50.0, 40.0, 40.0]);
        assert_eq!(skill(&t, &s, S::Destruction), 45.0); // 5 + 10 + 25 + 5
        assert_eq!(skill(&t, &s, S::Alchemy), 20.0); // 5 + 10 + 5
        assert_eq!(skill(&t, &s, S::ShortBlade), 15.0); // 5 + 10
        assert_eq!(skill(&t, &s, S::Block), 5.0);
        assert_eq!(v.max(t.idx.health), 40.0);
        assert_eq!((v.max(t.idx.magicka), v.current(t.idx.magicka)), (magicka, magicka));
        assert_eq!(v.max(t.idx.fatigue), 30.0 + 50.0 + 40.0 + 50.0);
        assert_eq!(v.base(t.idx.level), 1.0);
    }
}

/// Dev start: a stored `player` keeps its NPDT values and gains only the class counters; an
/// autocalc one is built from its race and class as chargen would.
#[test]
fn player_start_keeps_a_stored_record_and_builds_an_autocalc_one() {
    let (t, g, db) = (table(), MwGmst::vanilla(), db());
    let mut player = stored_npc();
    player.faction.clear();
    player.class = "Warrior".into();
    let s = sheet(&t, &player_start_ops(&t, &g, &db, &player), true);
    assert_eq!(attrs(&t, &s), [60.0, 40.0, 30.0, 50.0, 50.0, 60.0, 30.0, 40.0]);
    assert_eq!(skill(&t, &s, SkillId::LongBlade), 70.0);
    let v = s.view(&t.table);
    assert_eq!(v.base(t.idx.class_skills[SkillId::LongBlade as usize]), class_skill::MAJOR);
    assert_eq!(v.base(t.idx.class_specialization), 1.0);

    let auto = autocalc_npc(1);
    let built = sheet(&t, &player_start_ops(&t, &g, &db, &auto), true);
    let race = Chargen { race: db.get::<Race>(&RecordId::new("Redguard")), female: false, class: db.get::<Class>(&RecordId::new("Warrior")) };
    let expected = sheet(&t, &chargen_ops(&t, &g, &auto, race), true);
    assert_eq!(built, expected);
    assert_ne!(attrs(&t, &built), [0.0; 8]);
}

/// No race or class chosen: the player record's NPDT values stay (dev start).
#[test]
fn chargen_without_choices_keeps_the_record() {
    let t = table();
    let mut player = stored_npc();
    player.faction.clear();
    let s = sheet(&t, &chargen_ops(&t, &MwGmst::vanilla(), &player, Chargen::default()), true);
    let v = s.view(&t.table);
    assert_eq!(attrs(&t, &s), [60.0, 40.0, 30.0, 50.0, 50.0, 60.0, 30.0, 40.0]);
    assert_eq!(v.max(t.idx.health), 60.0); // floor(0.5 x (60 + 60))
    assert_eq!(v.max(t.idx.magicka), 40.0); // fPCbaseMagickaMult 1 x Int 40, no offset
    assert_eq!(v.base(t.idx.reputation), 40.0);
}

#[test]
fn fatigue_term_and_magicka_formulas() {
    let g = MwGmst::vanilla();
    assert_eq!(fatigue_term(&g, 50.0, 200.0), 0.875);
    assert_eq!(fatigue_term(&g, 200.0, 200.0), 1.25);
    assert_eq!(fatigue_term(&g, -10.0, 200.0), 0.75);
    assert_eq!(fatigue_term(&g, 0.0, 0.5), 1.25); // max floors to 0: ratio 1
    assert_eq!(magicka_max(&g, true, 50.0, 15.0), 125.0);
    assert_eq!(magicka_max(&g, false, 50.0, 0.0), 100.0);
    assert_eq!(fatigue_max(-5.0, 0.0, 0.0, 0.0), 0.0);
}

#[test]
fn twelve_byte_npdt_counts_as_autocalc() {
    let mut npc = stored_npc();
    assert!(!is_autocalc(&npc));
    npc.data.stats = None;
    assert!(is_autocalc(&npc));
}

#[test]
fn ops_for_missing_keeps_only_missing_stats() {
    let t = table();
    let ops = npc_ops(&t, &MwGmst::vanilla(), &RecordDb::new(), &stored_npc()).unwrap();
    let missing = [sf_stats::StatKey::new("luck"), sf_stats::StatKey::new("magicka.offset")];
    let got = ops_for_missing(&t, &ops, &missing);
    assert_eq!(got, vec![
        StatOp::SetBase { stat: t.idx.attributes[7], value: 40.0 },
        StatOp::CounterSet { stat: t.idx.magicka_offset, value: 20.0 },
    ]);
}
