//! Sheet initialisation (stats-adapter.md "Initialisation"): an NPC from its record (stored NPDT
//! values or autocalc), a creature from CREA, the player from chargen choices. Each produces init
//! [`StatOp`]s for a building sheet; [`new_sheet`] applies them and settles.
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `apps/openmw/mwclass/npc.cpp` (`autoCalculateAttributes`, `autoCalculateSkills`,
//! `round_ieee_754`, `Npc::ensureCustomData`), `apps/openmw/mwclass/creature.cpp`
//! (`Creature::ensureCustomData`, `getSkill`), `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp`
//! (`buildPlayer`) and `apps/openmw/mwmechanics/npcstats.cpp` (`updateHealth`). Arithmetic is
//! `f32` in `OpenMW`'s order, skills iterate in ESM index order.
//!
//! Not here: spells, abilities and powers (race, birthsign, NPC spell lists, autocalc spells)
//! belong to the effect runtime, which adds their modifiers as sources; base disposition and AI
//! settings are not stats.

use sf_records::{RecordDb, RecordId};
use sf_stats::{
    ApplyCtx, Cause, CauseKind, Inputs, Rules, Settle, Sheet, StatKey, StatOp, StatOwner,
};
use tes3::esp::{AttributeId, Class, Creature, Npc, NpcFlags, Race, SkillId, Specialization};

use crate::derived::{fatigue_max, magicka_max};
use crate::gmst::MwGmst;
use crate::table::{MwTable, SkillInfo, attr, class_skill};

/// Why an NPC can't be initialised (`OpenMW` refuses the reference: `Store::find` throws).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum InitError {
    #[error("NPC `{npc}`: race `{race}` not found")]
    MissingRace { npc: String, race: String },
    #[error("NPC `{npc}`: class `{class}` not found")]
    MissingClass { npc: String, class: String },
}

/// `OpenMW`'s `round_ieee_754`: round half to even.
#[must_use]
pub fn round_half_even(f: f32) -> f32 {
    f.round_ties_even()
}

pub(crate) fn skill_index(s: SkillId) -> Option<usize> {
    usize::try_from(s as i32).ok().filter(|&i| i < 27)
}

pub(crate) fn attribute_index(a: AttributeId) -> Option<usize> {
    usize::try_from(a as i32).ok().filter(|&i| i < 8)
}

fn spec_index(s: Specialization) -> Option<usize> {
    usize::try_from(s as i32).ok().filter(|&i| i < 3)
}

/// A race's base attributes for a sex (RADT pairs: male, female).
#[must_use]
pub fn race_attributes(race: &Race, female: bool) -> [f32; 8] {
    let d = &race.data;
    let sex = usize::from(female);
    #[allow(clippy::cast_precision_loss)]
    [d.strength, d.intelligence, d.willpower, d.agility, d.speed, d.endurance, d.personality, d.luck].map(|v| v[sex] as f32)
}

/// A race's skill bonus for one skill (the first matching RADT entry, as `OpenMW`'s `find_if`).
#[must_use]
pub fn race_skill_bonus(race: &Race, skill: usize) -> i32 {
    let b = &race.data.skill_bonuses;
    [
        (b.skill_0, b.bonus_0),
        (b.skill_1, b.bonus_1),
        (b.skill_2, b.bonus_2),
        (b.skill_3, b.bonus_3),
        (b.skill_4, b.bonus_4),
        (b.skill_5, b.bonus_5),
        (b.skill_6, b.bonus_6),
    ]
    .into_iter()
    .find(|(s, _)| skill_index(*s) == Some(skill))
    .map_or(0, |(_, bonus)| bonus)
}

/// A class's majors, minors, favoured attributes and specialization, as indices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassInfo {
    pub majors: Vec<usize>,
    pub minors: Vec<usize>,
    pub favoured: Vec<usize>,
    pub specialization: Option<usize>,
}

impl ClassInfo {
    #[must_use]
    pub fn of(class: &Class) -> Self {
        let d = &class.data;
        Self {
            majors: [d.major1, d.major2, d.major3, d.major4, d.major5].into_iter().filter_map(skill_index).collect(),
            minors: [d.minor1, d.minor2, d.minor3, d.minor4, d.minor5].into_iter().filter_map(skill_index).collect(),
            favoured: [d.attribute1, d.attribute2].into_iter().filter_map(attribute_index).collect(),
            specialization: spec_index(d.specialization),
        }
    }
}

/// `autoCalculateAttributes`: race base (by sex), +10 per favoured attribute, then + (level - 1)
/// x Σ over the governed skills of 1.0 major / 0.5 minor / 0.2 misc, rounded half to even, max
/// 100.
#[must_use]
pub fn autocalc_attributes(skills: &[SkillInfo; 27], race: &Race, female: bool, class: &ClassInfo, level: i32) -> [f32; 8] {
    let mut a = race_attributes(race, female);
    for &f in &class.favoured {
        a[f] += 10.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let levels = (level - 1) as f32;
    for (i, value) in a.iter_mut().enumerate() {
        let mut sum = 0.0f32;
        for (s, info) in skills.iter().enumerate() {
            if info.governing != i {
                continue;
            }
            sum += if class.majors.contains(&s) {
                1.0
            } else if class.minors.contains(&s) {
                0.5
            } else {
                0.2
            };
        }
        *value = round_half_even(*value + levels * sum).min(100.0);
    }
    a
}

/// Autocalc health: floor(0.5 x (Str + End)) + m x (level - 1), m = 3, +2 combat / +1 stealth
/// specialization, +1 when Endurance is favoured (`autoCalculateAttributes`).
#[must_use]
pub fn autocalc_health(class: &ClassInfo, attributes: &[f32; 8], level: i32) -> f32 {
    let mut m = 3;
    match class.specialization {
        Some(0) => m += 2,
        Some(2) => m += 1,
        _ => {}
    }
    if class.favoured.contains(&attr::ENDURANCE) {
        m += 1;
    }
    #[allow(clippy::cast_precision_loss)]
    let per_level = (m * (level - 1)) as f32;
    f32::midpoint(attributes[attr::STRENGTH], attributes[attr::ENDURANCE]).floor() + per_level
}

/// `autoCalculateSkills`: +10 minor, +25 major, then 5 + race bonus + 5 same specialization +
/// (level - 1) x (1.0 class skill else 0.1, + 0.5 same specialization), rounded half to even,
/// max 100.
#[must_use]
pub fn autocalc_skills(skills: &[SkillInfo; 27], race: &Race, class: &ClassInfo, level: i32) -> [f32; 27] {
    let mut v = [0.0f32; 27];
    for &s in &class.minors {
        v[s] += 10.0;
    }
    for &s in &class.majors {
        v[s] += 25.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let levels = (level - 1) as f32;
    for (s, info) in skills.iter().enumerate() {
        let class_skill = class.majors.contains(&s) || class.minors.contains(&s);
        let major_mult = if class_skill { 1.0f32 } else { 0.1 };
        let same_spec = class.specialization == Some(info.specialization);
        let (spec_mult, spec_bonus) = if same_spec { (0.5f32, 5.0f32) } else { (0.0, 0.0) };
        #[allow(clippy::cast_precision_loss)]
        let race_bonus = race_skill_bonus(race, s) as f32;
        v[s] = round_half_even(v[s] + 5.0 + race_bonus + spec_bonus + levels * (major_mult + spec_mult)).min(100.0);
    }
    v
}

fn set(stat: sf_stats::StatIdx, value: f32) -> StatOp {
    StatOp::SetBase { stat, value }
}

fn counter(stat: sf_stats::StatIdx, value: f32) -> StatOp {
    StatOp::CounterSet { stat, value }
}

/// The class counters (`class.<skill>`: misc 0, minor 1, major 2; `class.specialization`:
/// specialization + 1) that skill progression reads. Every skill is set, so a class change
/// (chargen redo, a script) replaces the old class entirely.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn class_ops(t: &MwTable, class: &ClassInfo) -> Vec<StatOp> {
    let mut out = Vec::with_capacity(28);
    for (s, &stat) in t.idx.class_skills.iter().enumerate() {
        let kind = if class.majors.contains(&s) {
            class_skill::MAJOR
        } else if class.minors.contains(&s) {
            class_skill::MINOR
        } else {
            class_skill::MISC
        };
        out.push(counter(stat, kind));
    }
    out.push(counter(t.idx.class_specialization, class.specialization.map_or(0.0, |s| (s + 1) as f32)));
    out
}

fn push_values(t: &MwTable, out: &mut Vec<StatOp>, attributes: &[f32; 8], skills: &[f32; 27]) {
    out.extend(t.idx.attributes.iter().zip(attributes).map(|(&i, &v)| set(i, v)));
    out.extend(t.idx.skills.iter().zip(skills).map(|(&i, &v)| set(i, v)));
}

/// Record pools: health stored; magicka and fatigue kept as offsets from the formula (see
/// `derived` module docs).
fn push_record_pools(t: &MwTable, g: &MwGmst, out: &mut Vec<StatOp>, attributes: &[f32; 8], hmf: [f32; 3]) {
    let a = attributes;
    let magicka = magicka_max(g, false, a[attr::INTELLIGENCE], 0.0);
    let fatigue = fatigue_max(a[attr::STRENGTH], a[attr::WILLPOWER], a[attr::AGILITY], a[attr::ENDURANCE]);
    out.push(set(t.idx.health, hmf[0]));
    out.push(counter(t.idx.magicka_offset, hmf[1] - magicka));
    out.push(counter(t.idx.fatigue_offset, hmf[2] - fatigue));
}

/// Level, reputation (`iAutoRepFacMod` x (rank + 1) + `iAutoRepLevMod` x (level - 1) for a
/// faction member, else the record's) and faction rank (`Npc::ensureCustomData`).
fn push_character(t: &MwTable, g: &MwGmst, out: &mut Vec<StatOp>, npc: &Npc) {
    let level = i32::from(npc.data.level);
    #[allow(clippy::cast_precision_loss)]
    out.push(counter(t.idx.level, level as f32));
    if npc.faction.is_empty() {
        out.push(counter(t.idx.reputation, f32::from(npc.data.reputation)));
    } else {
        let rank = i32::from(npc.data.rank);
        #[allow(clippy::cast_precision_loss)]
        let rep = g.auto_rep_fac_mod * (rank + 1) as f32 + g.auto_rep_lev_mod * (level - 1) as f32;
        out.push(counter(t.idx.reputation, rep));
        out.push(StatOp::SetRank { group: RecordId::new(&npc.faction), rank });
    }
}

/// Whether an NPC record uses autocalc: the flag, or no stored stats (a 12-byte NPDT; `OpenMW`
/// would read zeros there, which spawns the NPC dead).
#[must_use]
pub fn is_autocalc(npc: &Npc) -> bool {
    npc.npc_flags.contains(NpcFlags::AUTO_CALCULATE) || npc.data.stats.is_none()
}

/// An NPC's init ops from its record (`Npc::ensureCustomData`).
pub fn npc_ops(t: &MwTable, g: &MwGmst, db: &RecordDb, npc: &Npc) -> Result<Vec<StatOp>, InitError> {
    let mut out = Vec::with_capacity(48);
    push_character(t, g, &mut out, npc);
    if let Some(stats) = npc.data.stats.as_ref().filter(|_| !is_autocalc(npc)) {
        let attributes = stats.attributes.map(f32::from);
        push_values(t, &mut out, &attributes, &stats.skills.map(f32::from));
        let hmf = [stats.health, stats.magicka, stats.fatigue].map(f32::from);
        push_record_pools(t, g, &mut out, &attributes, hmf);
        // A stored NPC doesn't need its class (OpenMW reads the record); skill progression does.
        if let Some(class) = db.get::<Class>(&RecordId::new(&npc.class)) {
            out.extend(class_ops(t, &ClassInfo::of(class)));
        }
    } else {
        let race = db
            .get::<Race>(&RecordId::new(&npc.race))
            .ok_or_else(|| InitError::MissingRace { npc: npc.id.clone(), race: npc.race.clone() })?;
        let class = db
            .get::<Class>(&RecordId::new(&npc.class))
            .ok_or_else(|| InitError::MissingClass { npc: npc.id.clone(), class: npc.class.clone() })?;
        let class = ClassInfo::of(class);
        let level = i32::from(npc.data.level);
        let female = npc.npc_flags.contains(NpcFlags::FEMALE);
        let attributes = autocalc_attributes(&t.skills, race, female, &class, level);
        let skills = autocalc_skills(&t.skills, race, &class, level);
        push_values(t, &mut out, &attributes, &skills);
        // Magicka and fatigue follow the formula (offsets stay 0).
        out.push(set(t.idx.health, autocalc_health(&class, &attributes, level)));
        out.extend(class_ops(t, &class));
    }
    Ok(out)
}

/// A creature's init ops from CREA: attributes, health, magicka, fatigue, level; every skill of
/// a specialization takes the record's combat, magic or stealth value (`Creature::getSkill`).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn creature_ops(t: &MwTable, g: &MwGmst, creature: &Creature) -> Vec<StatOp> {
    let d = &creature.data;
    let attributes =
        [d.strength, d.intelligence, d.willpower, d.agility, d.speed, d.endurance, d.personality, d.luck].map(|v| v as f32);
    let by_spec = [d.combat, d.magic, d.stealth].map(|v| v as f32);
    let skills: [f32; 27] = std::array::from_fn(|s| by_spec[t.skills[s].specialization]);
    let mut out = Vec::with_capacity(44);
    out.push(counter(t.idx.level, d.level as f32));
    out.push(counter(t.idx.creature, 1.0));
    push_values(t, &mut out, &attributes, &skills);
    push_record_pools(t, g, &mut out, &attributes, [d.health, d.magicka, d.fatigue].map(|v| v as f32));
    out
}

/// The player's chargen choices (`buildPlayer`'s `mRaceSelected` / `mClassSelected`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Chargen<'a> {
    pub race: Option<&'a Race>,
    pub female: bool,
    pub class: Option<&'a Class>,
}

/// The player's init ops (`buildPlayer`): the player record's level and NPDT values, then the
/// race's base attributes and 5 + bonus skills, then the class's +10 favoured attributes, +10
/// minor, +25 major and +5 specialization skills; health = floor(0.5 x (Str + End))
/// (`updateHealth`); magicka and fatigue from the formula. Birthsign and race powers are spells
/// (effect runtime).
#[must_use]
pub fn chargen_ops(t: &MwTable, g: &MwGmst, player: &Npc, choice: Chargen<'_>) -> Vec<StatOp> {
    let mut out = Vec::with_capacity(48);
    push_character(t, g, &mut out, player);
    let (mut attributes, mut skills) = match &player.data.stats {
        Some(s) => (s.attributes.map(f32::from), s.skills.map(f32::from)),
        None => ([0.0; 8], [0.0; 27]),
    };
    if let Some(race) = choice.race {
        attributes = race_attributes(race, choice.female);
        #[allow(clippy::cast_precision_loss)]
        for (s, v) in skills.iter_mut().enumerate() {
            *v = 5.0 + race_skill_bonus(race, s) as f32;
        }
    }
    if let Some(class) = choice.class {
        let c = ClassInfo::of(class);
        out.extend(class_ops(t, &c));
        for &f in &c.favoured {
            attributes[f] += 10.0;
        }
        for &s in &c.minors {
            skills[s] += 10.0;
        }
        for &s in &c.majors {
            skills[s] += 25.0;
        }
        for (s, info) in t.skills.iter().enumerate() {
            if c.specialization == Some(info.specialization) {
                skills[s] += 5.0;
            }
        }
    }
    push_values(t, &mut out, &attributes, &skills);
    out.push(set(t.idx.health, f32::midpoint(attributes[attr::STRENGTH], attributes[attr::ENDURANCE]).floor()));
    out
}

/// The player's init ops when chargen is skipped (stats-adapter.md "Player, dev start"): a
/// stored `player` record keeps its NPDT values and gets its class's counters only (skill
/// progression needs them; no chargen bonuses on top of finished values); an autocalc one is
/// built as `buildPlayer` with the record's race, sex and class.
#[must_use]
pub fn player_start_ops(t: &MwTable, g: &MwGmst, db: &RecordDb, player: &Npc) -> Vec<StatOp> {
    let class = db.get::<Class>(&RecordId::new(&player.class));
    if !is_autocalc(player) {
        let mut ops = chargen_ops(t, g, player, Chargen::default());
        if let Some(class) = class {
            ops.extend(class_ops(t, &ClassInfo::of(class)));
        }
        return ops;
    }
    let choice = Chargen {
        race: db.get::<Race>(&RecordId::new(&player.race)),
        female: player.npc_flags.contains(NpcFlags::FEMALE),
        class,
    };
    chargen_ops(t, g, player, choice)
}

/// A new sheet from init ops: builds, applies them (events dropped) and settles with full pools.
/// Returns the sheet and how many ops were rejected (0 for ops from this module).
#[must_use]
pub fn new_sheet(t: &MwTable, rules: &Rules, owner: StatOwner, inputs: &Inputs, ops: &[StatOp]) -> (Sheet, usize) {
    let mut sheet = Sheet::new(&t.table);
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Rules, source: None, tick: 0 }, inputs };
    let rejected = sf_stats::apply(&t.table, rules, &mut sheet, ops, &ctx, &mut Vec::new());
    sheet.settle(&t.table, rules, inputs, Settle::NewCharacter);
    (sheet, rejected)
}

/// The init ops that set a stat a loaded save lacked (`LoadReport::missing`): the load path runs
/// these on the building sheet before sources re-add modifiers and `Settle::Loaded`.
#[must_use]
pub fn ops_for_missing(t: &MwTable, ops: &[StatOp], missing: &[StatKey]) -> Vec<StatOp> {
    let wanted = |stat| missing.contains(&t.table.def(stat).key);
    ops.iter()
        .filter(|op| match op {
            StatOp::SetBase { stat, .. } | StatOp::CounterSet { stat, .. } => wanted(*stat),
            _ => false,
        })
        .cloned()
        .collect()
}
