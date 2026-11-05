//! S3 exact formula tests: regen, rest, skill use, level-up, faction rank requirements against
//! `OpenMW` (`ca5e2576`) with vanilla GMSTs. Expected numbers are worked by hand from the `OpenMW`
//! source (written in `OpenMW`'s arithmetic order where `f32` rounding shows), not from this crate.

#![allow(clippy::float_cmp)]

use sf_records::RecordId;
use sf_stats::{
    ActorKey, ApplyCtx, Cause, CauseKind, IncreaseSource, Inputs, Refused, Rules, Sheet, StatEvent, StatIdx, StatOp,
    StatOwner, TickCtx,
};
use smallvec::{SmallVec, smallvec};
use tes3::esp::{AttributeId, Faction, FactionData, FactionRequirement, SkillId};

use super::tests::{table, warrior};
use crate::*;

const OWNER: StatOwner = StatOwner::Actor(ActorKey(7));

fn player_inputs() -> Inputs {
    Inputs { is_player: true, ..Default::default() }
}

fn cause(kind: CauseKind) -> Cause {
    Cause { kind, source: None, tick: 3 }
}

/// A level-1 Warrior: attributes 40 (End 50), skills 30, health 100, the Warrior class counters.
fn player(t: &MwTable, rules: &Rules) -> Sheet {
    let mut ops: Vec<StatOp> = t.idx.attributes.iter().map(|&stat| StatOp::SetBase { stat, value: 40.0 }).collect();
    ops.push(StatOp::SetBase { stat: t.idx.attributes[attr::ENDURANCE], value: 50.0 });
    ops.extend(t.idx.skills.iter().map(|&stat| StatOp::SetBase { stat, value: 30.0 }));
    ops.push(StatOp::SetBase { stat: t.idx.health, value: 100.0 });
    ops.push(StatOp::CounterSet { stat: t.idx.level, value: 1.0 });
    ops.extend(class_ops(t, &ClassInfo::of(&warrior())));
    let (sheet, rejected) = new_sheet(t, rules, OWNER, &player_inputs(), &ops);
    assert_eq!(rejected, 0);
    sheet
}

fn run(t: &MwTable, rules: &Rules, sheet: &mut Sheet, ops: &[StatOp], inputs: &Inputs) -> (usize, Vec<StatEvent>) {
    let mut events = Vec::new();
    let ctx = ApplyCtx { owner: OWNER, cause: cause(CauseKind::Rules), inputs };
    let rejected = sf_stats::apply(&t.table, rules, sheet, ops, &ctx, &mut events);
    (rejected, events)
}

fn base(t: &MwTable, s: &Sheet, stat: StatIdx) -> f32 {
    s.view(&t.table).base(stat)
}

fn skill(t: &MwTable, id: SkillId) -> StatIdx {
    t.idx.skills[id as usize]
}

fn raised(events: &[StatEvent]) -> usize {
    events.iter().filter(|e| matches!(e, StatEvent::SkillRaised { .. })).count()
}

#[test]
fn class_counters_come_from_the_class() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let s = player(&t, &rules);
    let kind = |id: SkillId| base(&t, &s, t.idx.class_skills[id as usize]);
    assert_eq!(kind(SkillId::LongBlade), class_skill::MAJOR);
    assert_eq!(kind(SkillId::Spear), class_skill::MINOR);
    assert_eq!(kind(SkillId::Destruction), class_skill::MISC);
    assert_eq!(base(&t, &s, t.idx.class_specialization), 1.0); // combat + 1
}

#[test]
fn skill_progress_requirements() {
    let g = MwGmst::vanilla();
    let special = f64::from(0.8f32);
    // Major, same specialization, base 30: 31 x 0.75 x 0.8 = 18.6.
    assert_eq!(skill_progress_requirement(&g, 30.0, class_skill::MAJOR, true), 31.0 * 0.75 * special);
    // Minor, same specialization: 31 x 1.0 x 0.8; misc, other specialization: 31 x 1.25.
    assert_eq!(skill_progress_requirement(&g, 30.0, class_skill::MINOR, true), 31.0 * special);
    assert_eq!(skill_progress_requirement(&g, 30.0, class_skill::MISC, false), 38.75);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn skill_use_progresses_to_a_raise_in_19_uses() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let lb = skill(&t, SkillId::LongBlade);
    let progress = t.idx.skill_progress[SkillId::LongBlade as usize];
    let one_use = [StatOp::UseSkill { skill: lb, use_kind: 0, scale: 1.0 }];

    let (_, events) = run(&t, &rules, &mut s, &one_use, &player_inputs());
    assert!(matches!(events[0], StatEvent::SkillUsed { gain, .. } if gain == 1.0));
    assert_eq!(base(&t, &s, progress), (1.0 / (31.0 * 0.75 * f64::from(0.8f32))) as f32);

    for _ in 1..18 {
        let (_, events) = run(&t, &rules, &mut s, &one_use, &player_inputs());
        assert_eq!(raised(&events), 0);
    }
    assert_eq!(base(&t, &s, lb), 30.0);
    // Use 19: 19 / 18.6 ≥ 1.
    let (_, events) = run(&t, &rules, &mut s, &one_use, &player_inputs());
    assert_eq!(raised(&events), 1);
    assert_eq!(base(&t, &s, lb), 31.0);
    assert_eq!(base(&t, &s, progress), 0.0);
    assert_eq!(base(&t, &s, t.idx.level_progress), 1.0);
    assert_eq!(base(&t, &s, t.idx.levelup_attr[attr::STRENGTH]), 1.0);
    assert_eq!(base(&t, &s, t.idx.levelup_spec[0]), 1.0);

    // An unknown use kind and a scaled use.
    let (_, events) = run(&t, &rules, &mut s, &[StatOp::UseSkill { skill: lb, use_kind: 4, scale: 1.0 }], &player_inputs());
    assert!(matches!(events[0], StatEvent::SkillUsed { gain, .. } if gain == 0.0));
    let (_, events) = run(&t, &rules, &mut s, &[StatOp::UseSkill { skill: lb, use_kind: 0, scale: 2.0 }], &player_inputs());
    assert!(matches!(events[0], StatEvent::SkillUsed { gain, .. } if gain == 2.0));
    assert_eq!(base(&t, &s, progress), (2.0 / (32.0 * 0.75 * f64::from(0.8f32))) as f32);
}

#[test]
fn skill_use_and_raise_refuse_at_100() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let lb = skill(&t, SkillId::LongBlade);
    run(&t, &rules, &mut s, &[StatOp::SetBase { stat: lb, value: 100.0 }], &player_inputs());
    let (_, events) = run(&t, &rules, &mut s, &[StatOp::UseSkill { skill: lb, use_kind: 0, scale: 1.0 }], &player_inputs());
    assert!(matches!(events[0], StatEvent::SkillUsed { gain, .. } if gain == 0.0));
    let (rejected, events) =
        run(&t, &rules, &mut s, &[StatOp::RaiseSkill { skill: lb, source: IncreaseSource::Training }], &player_inputs());
    assert_eq!((rejected, raised(&events)), (0, 0));
    assert_eq!(base(&t, &s, lb), 100.0);
    assert_eq!(base(&t, &s, t.idx.level_progress), 0.0);
}

#[test]
fn raise_sources_follow_the_lua_handler() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let lb = skill(&t, SkillId::LongBlade);
    let lb_progress = t.idx.skill_progress[SkillId::LongBlade as usize];
    let raise = |skill, source| [StatOp::RaiseSkill { skill, source }];

    // Training keeps the progress; a minor skill adds level progress too.
    run(&t, &rules, &mut s, &[StatOp::CounterSet { stat: lb_progress, value: 0.5 }], &player_inputs());
    run(&t, &rules, &mut s, &raise(lb, IncreaseSource::Training), &player_inputs());
    assert_eq!((base(&t, &s, lb), base(&t, &s, lb_progress)), (31.0, 0.5));
    run(&t, &rules, &mut s, &raise(skill(&t, SkillId::Spear), IncreaseSource::Book), &player_inputs());
    assert_eq!(base(&t, &s, t.idx.level_progress), 2.0);
    assert_eq!(base(&t, &s, t.idx.levelup_attr[attr::ENDURANCE]), 1.0);

    // A misc skill: no level progress, still the attribute and specialization tallies.
    run(&t, &rules, &mut s, &raise(skill(&t, SkillId::Destruction), IncreaseSource::Training), &player_inputs());
    assert_eq!(base(&t, &s, t.idx.level_progress), 2.0);
    assert_eq!(base(&t, &s, t.idx.levelup_attr[attr::WILLPOWER]), 1.0);
    assert_eq!(base(&t, &s, t.idx.levelup_spec[1]), 1.0);

    // Jail: -1 without tallies, except Security and Sneak, which it raises with tallies.
    let (_, events) = run(&t, &rules, &mut s, &raise(lb, IncreaseSource::Penalty), &player_inputs());
    assert!(matches!(events[0], StatEvent::SkillRaised { base, .. } if base == 30.0));
    assert_eq!(base(&t, &s, t.idx.levelup_attr[attr::STRENGTH]), 1.0);
    assert_eq!(base(&t, &s, lb_progress), 0.5);
    run(&t, &rules, &mut s, &raise(skill(&t, SkillId::Security), IncreaseSource::Penalty), &player_inputs());
    assert_eq!(base(&t, &s, skill(&t, SkillId::Security)), 31.0);
    assert_eq!(base(&t, &s, t.idx.levelup_attr[attr::INTELLIGENCE]), 1.0);
    assert_eq!(base(&t, &s, t.idx.levelup_spec[2]), 1.0);
    // Jail stops at 0.
    run(&t, &rules, &mut s, &[StatOp::SetBase { stat: lb, value: 0.0 }], &player_inputs());
    let (_, events) = run(&t, &rules, &mut s, &raise(lb, IncreaseSource::Penalty), &player_inputs());
    assert_eq!((raised(&events), base(&t, &s, lb)), (0, 0.0));
}

#[test]
fn attribute_multipliers_follow_ilevelup_nn_mult() {
    let g = MwGmst::vanilla();
    let got: Vec<f32> = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 12.0, -1.0]
        .iter()
        .map(|&tally| levelup_attribute_multiplier(&g, tally))
        .collect();
    assert_eq!(got, [1.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 5.0, 5.0, 1.0]);
}

#[test]
fn level_up_after_ten_major_raises() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let train = |id, n| vec![StatOp::RaiseSkill { skill: skill(&t, id), source: IncreaseSource::Training }; n];
    let mut ops = train(SkillId::LongBlade, 5); // Strength
    ops.extend(train(SkillId::HeavyArmor, 2)); // Endurance
    let (_, events) = run(&t, &rules, &mut s, &ops, &player_inputs());
    assert!(!events.iter().any(|e| matches!(e, StatEvent::LevelReady { .. })));
    let (_, events) = run(&t, &rules, &mut s, &train(SkillId::Block, 3), &player_inputs()); // Agility
    assert_eq!(events.iter().filter(|e| matches!(e, StatEvent::LevelReady { .. })).count(), 1);
    assert_eq!(base(&t, &s, t.idx.level_progress), 10.0);

    let [str_, agi, end, luck] =
        [attr::STRENGTH, attr::AGILITY, attr::ENDURANCE, attr::LUCK].map(|a| t.idx.attributes[a]);
    let offer = rules.leveling.offer(&s.view(&t.table));
    assert_eq!(offer.picks, 3);
    let gain = |stat| offer.gains.iter().find(|(s, _)| *s == stat).map(|(_, g)| *g);
    // Tallies Str 5, End 2, Agi 3, Luck 0: iLevelUp05Mult 3, 02 2, 03 2, x1.
    assert_eq!([gain(str_), gain(end), gain(agi), gain(luck)], [Some(3.0), Some(2.0), Some(2.0), Some(1.0)]);

    // Refusals change nothing.
    let level_up = |picks: &[StatIdx]| [StatOp::LevelUp { picks: SmallVec::from_slice(picks) }];
    assert_eq!(run(&t, &rules, &mut s, &level_up(&[str_, end]), &player_inputs()).0, 1);
    assert_eq!(run(&t, &rules, &mut s, &level_up(&[str_, str_, end]), &player_inputs()).0, 1);
    assert_eq!(run(&t, &rules, &mut s, &level_up(&[str_, end, t.idx.health]), &player_inputs()).0, 1);
    let check = rules.leveling.level_up(&s.view(&t.table), &[str_, end], &mut SmallVec::new());
    assert_eq!(check, Err(Refused::PickCount { expected: 3, got: 2 }));

    let (rejected, events) = run(&t, &rules, &mut s, &level_up(&[str_, end, luck]), &player_inputs());
    assert_eq!(rejected, 0);
    assert_eq!([base(&t, &s, str_), base(&t, &s, end), base(&t, &s, luck), base(&t, &s, agi)], [43.0, 52.0, 41.0, 40.0]);
    // Health + 0.1 x the new Endurance, base and current.
    let health = 100.0 + 52.0 * 0.1f32;
    assert_eq!(s.view(&t.table).max(t.idx.health), health);
    assert_eq!(s.view(&t.table).current(t.idx.health), health);
    assert_eq!(base(&t, &s, t.idx.level), 2.0);
    assert!(events.iter().any(|e| matches!(e, StatEvent::LevelChanged { level, .. } if *level == 2.0)));
    assert_eq!(base(&t, &s, t.idx.level_progress), 0.0);
    assert!(t.idx.levelup_attr.iter().all(|&a| base(&t, &s, a) == 0.0));
    assert_eq!(base(&t, &s, t.idx.levelup_spec[0]), 10.0); // specialization tallies stay
    // Not ready any more.
    let check = rules.leveling.level_up(&s.view(&t.table), &[str_, end, luck], &mut SmallVec::new());
    assert_eq!(check, Err(Refused::NotReady));
}

#[test]
fn level_up_caps_at_100_and_offers_fewer_picks() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let mut ops: Vec<StatOp> = t.idx.attributes.iter().map(|&stat| StatOp::SetBase { stat, value: 100.0 }).collect();
    ops.push(StatOp::SetBase { stat: t.idx.attributes[attr::STRENGTH], value: 99.0 });
    ops.push(StatOp::CounterSet { stat: t.idx.levelup_attr[attr::STRENGTH], value: 10.0 });
    ops.push(StatOp::CounterSet { stat: t.idx.level_progress, value: 12.0 });
    run(&t, &rules, &mut s, &ops, &player_inputs());
    let offer = rules.leveling.offer(&s.view(&t.table));
    let str_ = t.idx.attributes[attr::STRENGTH];
    assert_eq!((offer.picks, offer.gains.clone()), (1, vec![(str_, 1.0)]));
    let (rejected, _) = run(&t, &rules, &mut s, &[StatOp::LevelUp { picks: smallvec![str_] }], &player_inputs());
    assert_eq!(rejected, 0);
    assert_eq!(base(&t, &s, str_), 100.0);
    assert_eq!(base(&t, &s, t.idx.level_progress), 2.0);
}

#[test]
fn fatigue_regenerates_per_second_awake() {
    let g = MwGmst::vanilla();
    assert_eq!(fatigue_regen_per_second(&g, 50.0), 3.5); // 2.5 + 0.02 x 50
    let t = table();
    let rules = openmw_rules(&t, &g);
    let mut s = player(&t, &rules);
    let fatigue = t.idx.fatigue;
    let drain = [StatOp::PoolAdjust { pool: fatigue, delta: -100.0, below_zero: false, above_max: false }];
    run(&t, &rules, &mut s, &drain, &player_inputs());
    let start = s.view(&t.table).current(fatigue);
    let tick = |s: &mut Sheet, inputs: &Inputs| {
        let ctx = TickCtx { owner: OWNER, tick: 1, dt: 0.25, inputs };
        sf_stats::tick(&t.table, &rules, s, &ctx, &mut Vec::new());
    };
    for _ in 0..4 {
        tick(&mut s, &player_inputs());
    }
    assert_eq!(s.view(&t.table).current(fatigue), start + 3.5);
    // A combatant's stamina belongs to combat; the dead don't regenerate.
    let combatant = Inputs { combatant: true, ..player_inputs() };
    tick(&mut s, &combatant);
    assert_eq!(s.view(&t.table).current(fatigue), start + 3.5);
    let kill = [StatOp::PoolSetCurrent { pool: t.idx.health, value: 0.5 }];
    run(&t, &rules, &mut s, &kill, &player_inputs());
    tick(&mut s, &player_inputs());
    assert_eq!(s.view(&t.table).current(fatigue), start + 3.5);
    // Health and magicka don't regenerate awake.
    assert_eq!(s.view(&t.table).current(t.idx.health), 0.5);
}

#[test]
fn rest_formulas() {
    let g = MwGmst::vanilla();
    // (2.5 + 0.02 x (1 - enc)) x 0.04 x End x 3600: End 50 → 18144 unencumbered, 18000 full.
    assert_eq!(rest_fatigue_per_hour(&g, 50.0, 0.0), 18144.0);
    assert_eq!(rest_fatigue_per_hour(&g, 50.0, 1.0), 18000.0);
    assert_eq!(rest_fatigue_per_hour(&g, 50.0, 3.0), 18000.0);
    assert_eq!(sleep_restoration_per_hour(&g, 50.0, 40.0), (0.1f32 * 50.0, 0.15f32 * 40.0));
    assert_eq!(normalized_encumbrance(0.0, 0.0), 0.0);
    assert_eq!(normalized_encumbrance(10.0, 0.0), 1.0 + 1e-6);
    assert_eq!(normalized_encumbrance(50.0, 200.0), 0.25);
}

#[test]
fn sleep_restores_health_and_magicka_wait_only_fatigue() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let i = &t.idx;
    let low = [
        StatOp::PoolSetCurrent { pool: i.health, value: 10.0 },
        StatOp::PoolSetCurrent { pool: i.magicka, value: 0.0 },
        StatOp::PoolSetCurrent { pool: i.fatigue, value: 0.0 },
    ];
    run(&t, &rules, &mut s, &low, &player_inputs());
    // Wait 2 hours: fatigue only (18144 per hour fills it).
    run(&t, &rules, &mut s, &[StatOp::Rest { hours: 2.0, sleep: false }], &player_inputs());
    let v = s.view(&t.table);
    assert_eq!((v.current(i.health), v.current(i.magicka)), (10.0, 0.0));
    assert_eq!(v.current(i.fatigue), v.max(i.fatigue));
    // Sleep 2 hours: health + 2 x 0.1 x End 50, magicka + 2 x 0.15 x Int 40 (player max 40).
    run(&t, &rules, &mut s, &[StatOp::Rest { hours: 2.0, sleep: true }], &player_inputs());
    let v = s.view(&t.table);
    assert_eq!(v.current(i.health), 10.0 + 0.1f32 * 50.0 * 2.0);
    assert_eq!(v.current(i.magicka), 0.15f32 * 40.0 * 2.0);
    // Sleep 8 hours: health 20 + 40 = 60, magicka stops at its max.
    run(&t, &rules, &mut s, &[StatOp::Rest { hours: 8.0, sleep: true }], &player_inputs());
    let v = s.view(&t.table);
    assert_eq!(v.current(i.health), 60.0);
    assert_eq!(v.current(i.magicka), 40.0);
    // Fatigue from rest with encumbrance: 1 hour of a carry at capacity, from 0, End 50.
    run(&t, &rules, &mut s, &[StatOp::PoolSetCurrent { pool: i.fatigue, value: -500.0 }], &player_inputs());
    let loaded = Inputs { carried: 200.0, ..player_inputs() }; // capacity Str 40 x 5
    run(&t, &rules, &mut s, &[StatOp::Rest { hours: 0.01, sleep: false }], &loaded);
    assert_eq!(s.view(&t.table).current(i.fatigue), -500.0 + 18000.0 * 0.01);
}

#[test]
fn the_same_ops_give_the_same_sheet_and_events() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let lb = skill(&t, SkillId::LongBlade);
    let script = || {
        let mut s = player(&t, &rules);
        let mut all = Vec::new();
        let mut ops = vec![StatOp::UseSkill { skill: lb, use_kind: 0, scale: 1.3 }; 40];
        ops.extend(vec![StatOp::RaiseSkill { skill: lb, source: IncreaseSource::Training }; 10]);
        ops.push(StatOp::PoolAdjust { pool: t.idx.fatigue, delta: -50.0, below_zero: false, above_max: false });
        all.extend(run(&t, &rules, &mut s, &ops, &player_inputs()).1);
        let picks = [attr::STRENGTH, attr::ENDURANCE, attr::LUCK].map(|a| t.idx.attributes[a]);
        all.extend(run(&t, &rules, &mut s, &[StatOp::LevelUp { picks: SmallVec::from_slice(&picks) }], &player_inputs()).1);
        for n in 0..30 {
            let ctx = TickCtx { owner: OWNER, tick: n, dt: 1.0 / 60.0, inputs: &player_inputs() };
            sf_stats::tick(&t.table, &rules, &mut s, &ctx, &mut all);
        }
        all.extend(run(&t, &rules, &mut s, &[StatOp::Rest { hours: 1.0, sleep: true }], &player_inputs()).1);
        (s, all)
    };
    let (a, ea) = script();
    let (b, eb) = script();
    assert_eq!(a, b);
    assert_eq!(ea, eb);
    assert_eq!(base(&t, &a, t.idx.level), 2.0);
}

fn guild() -> Faction {
    use SkillId as S;
    let mut requirements: [FactionRequirement; 10] = Default::default();
    for (r, req) in requirements.iter_mut().enumerate() {
        let r = i32::try_from(r).unwrap();
        *req = FactionRequirement { attributes: [30 + r, 30 + r], primary_skill: 10 * r, favored_skill: 5 * r, reputation: 3 * r };
    }
    Faction {
        id: "Fighters Guild".into(),
        data: FactionData {
            favored_attributes: [AttributeId::Strength, AttributeId::Endurance],
            requirements,
            favored_skills: [S::LongBlade, S::Block, S::Axe, S::None, S::None, S::None, S::None],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn faction_rank_requirements() {
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let ranks = FactionRanks::of(&guild());
    let fg = RecordId::new("Fighters Guild");
    let set = |id, value| StatOp::SetBase { stat: skill(&t, id), value };
    assert_eq!(ranks.skills, vec![5, 0, 6]);
    assert_eq!(ranks.ranks[3], RankRequirement { attributes: [33, 33], primary_skill: 30, favoured_skill: 15, reputation: 9 });

    // Rank 4: best ≥ 40, next two ≥ 20 (base values, truncated).
    run(&t, &rules, &mut s, &[set(SkillId::LongBlade, 40.0), set(SkillId::Block, 20.0), set(SkillId::Axe, 19.9)], &player_inputs());
    assert!(!ranks.has_skills_for_rank(&t.idx, &s.view(&t.table), 4));
    run(&t, &rules, &mut s, &[set(SkillId::Axe, 20.0)], &player_inputs());
    assert!(ranks.has_skills_for_rank(&t.idx, &s.view(&t.table), 4));
    // Any order: the best of the three is the primary.
    run(&t, &rules, &mut s, &[set(SkillId::LongBlade, 20.0), set(SkillId::Axe, 40.0)], &player_inputs());
    assert!(ranks.has_skills_for_rank(&t.idx, &s.view(&t.table), 4));
    assert!(!ranks.has_skills_for_rank(&t.idx, &s.view(&t.table), 5));
    assert!(!ranks.has_skills_for_rank(&t.idx, &s.view(&t.table), 10));
    // One faction skill checks only the primary; none passes.
    let one = FactionRanks { skills: vec![SkillId::Axe as usize], ..ranks.clone() };
    assert!(one.has_skills_for_rank(&t.idx, &s.view(&t.table), 4));
    let none = FactionRanks { skills: vec![], ..ranks.clone() };
    assert!(none.has_skills_for_rank(&t.idx, &s.view(&t.table), 9));

    // Attributes (base Str 40, End 50) and reputation.
    let v = s.view(&t.table);
    assert!(ranks.meets_skill_requirements(&t.idx, &v, 4)); // 34
    let weak = FactionRanks { attributes: [Some(attr::STRENGTH), Some(attr::LUCK)], ..ranks.clone() };
    assert!(weak.meets_skill_requirements(&t.idx, &v, 4));
    assert!(!ranks.meets_reputation(&v, &fg, 1));
    assert!(ranks.meets_reputation(&v, &fg, 0));

    // The dialogue function: a non-member checks rank 0 (skills 0, reputation 0: 3).
    assert_eq!(ranks.rank_requirement(&t.idx, &v, &fg), 3);
    let join = [StatOp::SetRank { group: fg.clone(), rank: 3 }, StatOp::GroupRep { group: fg.clone(), delta: 12, set: true }];
    run(&t, &rules, &mut s, &join, &player_inputs());
    let v = s.view(&t.table);
    assert_eq!(ranks.rank_requirement(&t.idx, &v, &fg), 3); // rank 4: skills, 34 attributes, rep 12
    run(&t, &rules, &mut s, &[StatOp::GroupRep { group: fg.clone(), delta: 11, set: true }], &player_inputs());
    assert_eq!(ranks.rank_requirement(&t.idx, &s.view(&t.table), &fg), 1);
    run(&t, &rules, &mut s, &[StatOp::SetRank { group: fg.clone(), rank: 4 }], &player_inputs());
    assert_eq!(ranks.rank_requirement(&t.idx, &s.view(&t.table), &fg), 0); // rank 5 skills fail, rep 11 < 15
    run(&t, &rules, &mut s, &[StatOp::SetRank { group: fg.clone(), rank: 9 }], &player_inputs());
    assert_eq!(ranks.rank_requirement(&t.idx, &s.view(&t.table), &fg), 0);
}

#[test]
fn skill_progress_has_no_lower_clamp() {
    // OpenMW's handlers may push progress negative (a mod's `skillUsed` handler); level
    // progress keeps its 0 floor.
    let t = table();
    let rules = openmw_rules(&t, &MwGmst::vanilla());
    let mut s = player(&t, &rules);
    let p = t.idx.skill_progress[SkillId::LongBlade as usize];
    let (rejected, _) = run(&t, &rules, &mut s, &[StatOp::CounterSet { stat: p, value: -0.25 }], &player_inputs());
    assert_eq!(rejected, 0);
    assert_eq!(base(&t, &s, p), -0.25);
    run(&t, &rules, &mut s, &[StatOp::CounterAdd { stat: p, delta: -0.5 }], &player_inputs());
    assert_eq!(base(&t, &s, p), -0.75);
    run(&t, &rules, &mut s, &[StatOp::CounterSet { stat: t.idx.level_progress, value: -1.0 }], &player_inputs());
    assert_eq!(base(&t, &s, t.idx.level_progress), 0.0);
}
