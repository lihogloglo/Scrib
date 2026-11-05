//! Every NPC and creature of the installed game files initialises to a sane sheet; prints the
//! GMST snapshot, the SKIL data and counts. Skips (passes) without a Morrowind install.

use mw_install::MorrowindInstall;
use mw_stats::{
    ATTRIBUTES, Chargen, MwDerived, MwGmst, MwTable, SKILLS, SPECIALIZATIONS, chargen_ops, creature_ops, is_autocalc,
    new_sheet, npc_ops,
};
use sf_stats::{ActorKey, Inputs, Rules, Sheet, StatOwner, TestModel};
use sf_vfs::Vfs;
use tes3::esp::{Class, ClassFlags, Creature, Npc, Race, RaceFlags};

const OWNER: StatOwner = StatOwner::Actor(ActorKey(1));

#[derive(Default, Debug)]
struct Counts {
    sheets: u32,
    over_100: u32,
    dead: u32,
    magicka_offset: u32,
    fatigue_offset: u32,
}

/// Finite everywhere; attributes and skills in 0..=255 (u8 records; > 100 counted); pools ≥ 0
/// and full; health 0 counted (placed corpses: `OpenMW` spawns them dead).
#[allow(clippy::many_single_char_names, clippy::float_cmp)]
fn check(t: &MwTable, s: &Sheet, id: &str, c: &mut Counts) {
    let v = s.view(&t.table);
    c.sheets += 1;
    for (i, def) in t.table.defs() {
        assert!(v.modified(i).is_finite(), "{id}: {} = {}", def.key, v.modified(i));
    }
    for &i in t.idx.attributes.iter().chain(&t.idx.skills) {
        let x = v.modified(i);
        assert!((0.0..=255.0).contains(&x), "{id}: {} = {x}", t.table.def(i).key);
        if x > 100.0 {
            c.over_100 += 1;
        }
    }
    for p in [t.idx.health, t.idx.magicka, t.idx.fatigue] {
        assert!(v.max(p) >= 0.0 && v.current(p) == v.max(p), "{id}: {} {}/{}", t.table.def(p).key, v.current(p), v.max(p));
    }
    if v.max(t.idx.health) < 1.0 {
        c.dead += 1;
    }
    c.magicka_offset += u32::from(v.base(t.idx.magicka_offset) != 0.0);
    c.fatigue_offset += u32::from(v.base(t.idx.fatigue_offset) != 0.0);
}

#[test]
#[ignore = "real install"]
#[allow(clippy::too_many_lines)]
fn every_npc_and_creature_initialises() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = &content.db;

    let (gmst, missing) = MwGmst::from_db(db);
    let vanilla = MwGmst::vanilla();
    eprintln!("GMST snapshot ({} missing: {missing:?}):", missing.len());
    for ((name, value), (_, v)) in gmst.entries().into_iter().zip(vanilla.entries()) {
        let note = if value.to_bits() == v.to_bits() { String::new() } else { format!("  (vanilla() says {v})") };
        eprintln!("  {name} = {value}{note}");
    }
    let t = MwTable::from_db(db).unwrap();
    for (s, info) in t.skills.iter().enumerate() {
        eprintln!(
            "  SKIL {:<12} {:<12} {:<8} {:?}",
            SKILLS[s], ATTRIBUTES[info.governing], SPECIALIZATIONS[info.specialization], info.use_values
        );
    }
    let rules = Rules {
        derived: Box::new(MwDerived { gmst: gmst.clone(), idx: t.idx.clone() }),
        skill_use: Box::new(TestModel::default()),
        leveling: Box::new(TestModel::default()),
    };
    let npc_inputs = Inputs::default();

    let (mut npcs, mut autocalc, mut errors) = (Counts::default(), 0u32, Vec::new());
    let mut level_range = (i16::MAX, i16::MIN);
    for (_, npc) in db.iter::<Npc>() {
        autocalc += u32::from(is_autocalc(npc));
        level_range = (level_range.0.min(npc.data.level), level_range.1.max(npc.data.level));
        match npc_ops(&t, &gmst, db, npc) {
            Ok(ops) => {
                let (s, rejected) = new_sheet(&t, &rules, OWNER, &npc_inputs, &ops);
                assert_eq!(rejected, 0, "{}", npc.id);
                check(&t, &s, &npc.id, &mut npcs);
            }
            Err(e) => errors.push(e.to_string()),
        }
    }
    eprintln!("NPCs: {npcs:?}, autocalc {autocalc}, levels {level_range:?}, init errors {}", errors.len());
    for e in errors.iter().take(10) {
        eprintln!("  {e}");
    }

    let mut creatures = Counts::default();
    for (_, c) in db.iter::<Creature>() {
        let (s, rejected) = new_sheet(&t, &rules, OWNER, &npc_inputs, &creature_ops(&t, &gmst, c));
        assert_eq!(rejected, 0, "{}", c.id);
        check(&t, &s, &c.id, &mut creatures);
    }
    eprintln!("creatures: {creatures:?}");

    // Chargen: every playable race (both sexes) x playable class from the `player` record.
    let player = db.get::<Npc>(&sf_records::RecordId::new("player")).expect("player record");
    let player_inputs = Inputs { is_player: true, ..Default::default() };
    let mut chargen = Counts::default();
    let races: Vec<&Race> = db.iter::<Race>().map(|(_, r)| r).filter(|r| r.data.flags.contains(RaceFlags::PLAYABLE)).collect();
    let classes: Vec<&Class> =
        db.iter::<Class>().map(|(_, c)| c).filter(|c| c.data.flags.contains(ClassFlags::PLAYABLE)).collect();
    for race in &races {
        for class in &classes {
            for female in [false, true] {
                let ops = chargen_ops(&t, &gmst, player, Chargen { race: Some(race), female, class: Some(class) });
                let (s, rejected) = new_sheet(&t, &rules, StatOwner::Actor(ActorKey(0)), &player_inputs, &ops);
                assert_eq!(rejected, 0);
                check(&t, &s, &format!("{} {}", race.id, class.id), &mut chargen);
                assert!(s.view(&t.table).max(t.idx.health) >= 1.0);
                // Skill progression reads the class from the sheet: 5 majors (2), 5 minors (1).
                let kinds: f32 = t.idx.class_skills.iter().map(|&k| s.view(&t.table).base(k)).sum();
                assert!((kinds - 15.0).abs() < 0.5, "{}: {kinds}", class.id);
                assert!(s.view(&t.table).base(t.idx.class_specialization) >= 1.0);
            }
        }
    }
    eprintln!("chargen: {} races x {} classes x 2: {chargen:?}", races.len(), classes.len());

    assert!(errors.is_empty(), "{errors:?}");
    assert!(npcs.sheets > 2000 && creatures.sheets > 200, "{npcs:?} {creatures:?}");
}
