//! Morrowind's stat table (stats-adapter.md "Stat table"): 8 attributes, the 27 skills of the
//! SKIL records, the three dynamic pools, counters, derived stats and magnitude stats, keyed by
//! `OpenMW`'s `RefId` names. The format fixes the attribute and skill sets (ESM indices); the SKIL
//! records supply each skill's governing attribute, specialization and use values.

use sf_records::RecordDb;
use sf_stats::{Rebase, Role, StatDef, StatIdx, StatKey, StatKind, StatTable, TableError};
use tes3::esp::Skill;

/// Attribute keys in ESM index order (`ESM::Attribute`).
pub const ATTRIBUTES: [&str; 8] =
    ["strength", "intelligence", "willpower", "agility", "speed", "endurance", "personality", "luck"];

/// Display-name GMSTs of the attributes (the UI's loc keys).
const ATTRIBUTE_LABELS: [&str; 8] = [
    "sAttributeStrength",
    "sAttributeIntelligence",
    "sAttributeWillpower",
    "sAttributeAgility",
    "sAttributeSpeed",
    "sAttributeEndurance",
    "sAttributePersonality",
    "sAttributeLuck",
];

/// Skill keys in ESM index order (`ESM::Skill`, SKIL `INDX`).
pub const SKILLS: [&str; 27] = [
    "block",
    "armorer",
    "mediumarmor",
    "heavyarmor",
    "bluntweapon",
    "longblade",
    "axe",
    "spear",
    "athletics",
    "enchant",
    "destruction",
    "alteration",
    "illusion",
    "conjuration",
    "mysticism",
    "restoration",
    "alchemy",
    "unarmored",
    "security",
    "sneak",
    "acrobatics",
    "lightarmor",
    "shortblade",
    "marksman",
    "mercantile",
    "speechcraft",
    "handtohand",
];

/// Display-name GMSTs of the skills.
const SKILL_LABELS: [&str; 27] = [
    "sSkillBlock",
    "sSkillArmorer",
    "sSkillMediumarmor",
    "sSkillHeavyarmor",
    "sSkillBluntweapon",
    "sSkillLongblade",
    "sSkillAxe",
    "sSkillSpear",
    "sSkillAthletics",
    "sSkillEnchant",
    "sSkillDestruction",
    "sSkillAlteration",
    "sSkillIllusion",
    "sSkillConjuration",
    "sSkillMysticism",
    "sSkillRestoration",
    "sSkillAlchemy",
    "sSkillUnarmored",
    "sSkillSecurity",
    "sSkillSneak",
    "sSkillAcrobatics",
    "sSkillLightarmor",
    "sSkillShortblade",
    "sSkillMarksman",
    "sSkillMercantile",
    "sSkillSpeechcraft",
    "sSkillHandtohand",
];

/// The magnitude stats of the resist, weakness and elemental-shield effects (effects-adapter.md
/// "As built (A1)"): (key, label GMST), keyed by the MGEF effect's `RefId` name, in MGEF index
/// order: Fire / Lightning / Frost Shield (4-6), the nine Weakness effects (28-36), the ten Resist
/// effects (90-99). `mw_effects` checks this list against its MGEF table. Appended after every
/// other def so the older indices (and saves, which are keyed) stay put.
pub const MAGNITUDE_STATS: [(&str, &str); 22] = [
    ("fireshield", "sEffectFireShield"),
    ("lightningshield", "sEffectLightningShield"),
    ("frostshield", "sEffectFrostShield"),
    ("weaknesstofire", "sEffectWeaknesstoFire"),
    ("weaknesstofrost", "sEffectWeaknesstoFrost"),
    ("weaknesstoshock", "sEffectWeaknesstoShock"),
    ("weaknesstomagicka", "sEffectWeaknesstoMagicka"),
    ("weaknesstocommondisease", "sEffectWeaknesstoCommonDisease"),
    ("weaknesstoblightdisease", "sEffectWeaknesstoBlightDisease"),
    ("weaknesstocorprusdisease", "sEffectWeaknesstoCorprusDisease"),
    ("weaknesstopoison", "sEffectWeaknesstoPoison"),
    ("weaknesstonormalweapons", "sEffectWeaknesstoNormalWeapons"),
    ("resistfire", "sEffectResistFire"),
    ("resistfrost", "sEffectResistFrost"),
    ("resistshock", "sEffectResistShock"),
    ("resistmagicka", "sEffectResistMagicka"),
    ("resistcommondisease", "sEffectResistCommonDisease"),
    ("resistblightdisease", "sEffectResistBlightDisease"),
    ("resistcorprusdisease", "sEffectResistCorprusDisease"),
    ("resistpoison", "sEffectResistPoison"),
    ("resistnormalweapons", "sEffectResistNormalWeapons"),
    ("resistparalysis", "sEffectResistParalysis"),
];

/// Specialization keys in ESM order (`ESM::Class::Specialization`).
pub const SPECIALIZATIONS: [&str; 3] = ["combat", "magic", "stealth"];

/// Attribute indices used by the formulas.
pub mod attr {
    pub const STRENGTH: usize = 0;
    pub const INTELLIGENCE: usize = 1;
    pub const WILLPOWER: usize = 2;
    pub const AGILITY: usize = 3;
    pub const ENDURANCE: usize = 5;
    pub const LUCK: usize = 7;
}

/// A skill's place in the character's class, the value of its `class.<skill>` counter.
pub mod class_skill {
    pub const MISC: f32 = 0.0;
    pub const MINOR: f32 = 1.0;
    pub const MAJOR: f32 = 2.0;
}

/// One skill's SKIL data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SkillInfo {
    /// Governing attribute (index into [`ATTRIBUTES`]).
    pub governing: usize,
    /// Specialization (index into [`SPECIALIZATIONS`]).
    pub specialization: usize,
    /// Progress per use, by use kind (`SKDT` `actions`).
    pub use_values: [f32; 4],
}

/// Why the table can't be built from the content.
#[derive(Debug, thiserror::Error)]
pub enum MwTableError {
    #[error("no SKIL record for skill index {0}")]
    MissingSkill(usize),
    #[error("SKIL index {skill} has governing attribute {value}")]
    BadAttribute { skill: usize, value: i32 },
    #[error("SKIL index {skill} has specialization {value}")]
    BadSpecialization { skill: usize, value: i32 },
    #[error(transparent)]
    Table(#[from] TableError),
}

/// The indices of every Morrowind stat in [`MwTable::table`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MwIdx {
    pub attributes: [StatIdx; 8],
    pub skills: [StatIdx; 27],
    /// Magnitude stat of Fortify Maximum Magicka (feeds the magicka max).
    pub fortify_max_magicka: StatIdx,
    pub health: StatIdx,
    pub magicka: StatIdx,
    pub fatigue: StatIdx,
    pub level: StatIdx,
    pub reputation: StatIdx,
    pub bounty: StatIdx,
    /// Skill raises toward the next level (`iLevelUpTotal`).
    pub level_progress: StatIdx,
    /// Per-skill progress toward the next skill level (0..1).
    pub skill_progress: [StatIdx; 27],
    /// Per-attribute level-up tallies (`mSkillIncreases`).
    pub levelup_attr: [StatIdx; 8],
    /// Per-specialization level-up tallies (`mSpecIncreases`).
    pub levelup_spec: [StatIdx; 3],
    pub werewolf_kills: StatIdx,
    /// The record's magicka minus the formula at init (non-autocalc NPCs, creatures).
    pub magicka_offset: StatIdx,
    /// The record's fatigue minus the formula at init.
    pub fatigue_offset: StatIdx,
    /// 1 for a creature sheet (creature capacity is Str x 5, `creature.cpp` `getCapacity`).
    pub creature: StatIdx,
    /// The class's use of each skill ([`class_skill`]: misc 0, minor 1, major 2), read by skill
    /// progression. The class is character data, so it lives (and saves) with the sheet.
    pub class_skills: [StatIdx; 27],
    /// The class's specialization + 1 (0 = none: creatures, no class).
    pub class_specialization: StatIdx,
    pub capacity: StatIdx,
    pub encumbrance: StatIdx,
    pub fatigue_term: StatIdx,
    /// The [`MAGNITUDE_STATS`], in that order.
    pub magnitudes: [StatIdx; 22],
}

/// The Morrowind stat table with its indices and the SKIL data.
#[derive(Clone, Debug)]
pub struct MwTable {
    pub table: StatTable,
    pub idx: MwIdx,
    pub skills: [SkillInfo; 27],
}

fn def(key: &str, kind: StatKind, group: &str, public: bool, label: &str) -> StatDef {
    StatDef { key: StatKey::new(key), kind, group: group.into(), public, label: label.into() }
}

const fn counter(min: f32, max: f32) -> StatKind {
    StatKind::Counter { min, max }
}

/// Attributes, skills and magnitudes: `min 0`, no cap (stats-adapter.md "Stat table").
const VALUE: StatKind = StatKind::Value { min: 0.0, cap: None };

/// The definitions, in index order: attributes, skills, Fortify Maximum Magicka, pools, counters,
/// derived, then the [`MAGNITUDE_STATS`] (appended last, 2026-11-08: earlier indices unchanged).
#[must_use]
pub fn defs() -> Vec<StatDef> {
    let mut d = Vec::with_capacity(140);
    for (key, label) in ATTRIBUTES.iter().zip(ATTRIBUTE_LABELS) {
        d.push(def(key, VALUE, "attribute", false, label));
    }
    for (key, label) in SKILLS.iter().zip(SKILL_LABELS) {
        d.push(def(key, VALUE, "skill", false, label));
    }
    d.push(def("fortifymaximummagicka", VALUE, "magnitude", false, ""));
    d.push(def(
        "health",
        StatKind::Pool { depleted_below: 1.0, rebase: Rebase::KeepCurrent },
        "dynamic",
        true,
        "sHealth",
    ));
    d.push(def("magicka", StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepRatio }, "dynamic", true, "sMagic"));
    d.push(def(
        "fatigue",
        StatKind::Pool { depleted_below: 0.0, rebase: Rebase::KeepRatio },
        "dynamic",
        true,
        "sFatigue",
    ));
    d.push(def("level", counter(0.0, f32::MAX), "character", true, "sLevel"));
    d.push(def("reputation", counter(0.0, 255.0), "character", false, "sReputation"));
    d.push(def("bounty", counter(0.0, f32::MAX), "character", false, "sBounty"));
    d.push(def("level.progress", counter(0.0, f32::MAX), "progress", false, ""));
    for s in SKILLS {
        // No lower clamp: OpenMW's skill-progression handlers may push progress negative.
        d.push(def(&format!("progress.{s}"), counter(f32::MIN, f32::MAX), "progress", false, ""));
    }
    for a in ATTRIBUTES {
        d.push(def(&format!("levelup.attr.{a}"), counter(0.0, f32::MAX), "progress", false, ""));
    }
    for s in SPECIALIZATIONS {
        d.push(def(&format!("levelup.spec.{s}"), counter(0.0, f32::MAX), "progress", false, ""));
    }
    d.push(def("werewolf.kills", counter(0.0, f32::MAX), "progress", false, ""));
    d.push(def("magicka.offset", counter(f32::MIN, f32::MAX), "internal", false, ""));
    d.push(def("fatigue.offset", counter(f32::MIN, f32::MAX), "internal", false, ""));
    d.push(def("actor.creature", counter(0.0, 1.0), "internal", false, ""));
    for s in SKILLS {
        d.push(def(&format!("class.{s}"), counter(0.0, 2.0), "class", false, ""));
    }
    d.push(def("class.specialization", counter(0.0, 3.0), "class", false, ""));
    d.push(def("capacity", StatKind::Derived, "derived", false, "sEncumbrance"));
    d.push(def("encumbrance", StatKind::Derived, "derived", false, "sEncumbrance"));
    d.push(def("fatigue.term", StatKind::Derived, "derived", false, ""));
    // New defs go after this point only: indices are positional in `MwIdx`.
    for (key, label) in MAGNITUDE_STATS {
        d.push(def(key, VALUE, "magnitude", false, label));
    }
    d
}

/// The role assignments.
#[must_use]
pub fn roles() -> Vec<(Role, StatKey)> {
    [
        (Role::Health, "health"),
        (Role::Mana, "magicka"),
        (Role::Stamina, "fatigue"),
        (Role::Level, "level"),
        (Role::CarryCapacity, "capacity"),
        (Role::Encumbrance, "encumbrance"),
        (Role::Bounty, "bounty"),
    ]
    .into_iter()
    .map(|(r, k)| (r, StatKey::new(k)))
    .collect()
}

impl MwTable {
    /// The table with the SKIL data of `db` (every skill must have its record).
    pub fn from_db(db: &RecordDb) -> Result<Self, MwTableError> {
        let mut found: [Option<SkillInfo>; 27] = [None; 27];
        for (_, skill) in db.iter::<Skill>() {
            let Ok(i) = usize::try_from(skill.skill_id as i32) else { continue };
            let Some(slot) = found.get_mut(i) else { continue };
            let governing = usize::try_from(skill.data.governing_attribute)
                .ok()
                .filter(|&a| a < 8)
                .ok_or(MwTableError::BadAttribute { skill: i, value: skill.data.governing_attribute })?;
            let specialization = usize::try_from(skill.data.specialization)
                .ok()
                .filter(|&s| s < 3)
                .ok_or(MwTableError::BadSpecialization { skill: i, value: skill.data.specialization })?;
            *slot = Some(SkillInfo { governing, specialization, use_values: skill.data.actions });
        }
        let mut skills = [SkillInfo { governing: 0, specialization: 0, use_values: [0.0; 4] }; 27];
        for (i, f) in found.iter().enumerate() {
            skills[i] = f.ok_or(MwTableError::MissingSkill(i))?;
        }
        Self::with_skills(skills)
    }

    /// The table with given SKIL data (tests, or content without SKIL records).
    pub fn with_skills(skills: [SkillInfo; 27]) -> Result<Self, MwTableError> {
        let table = StatTable::new(defs(), &roles())?;
        let at = |key: &str| table.find(key).expect("defined in defs()");
        let idx = MwIdx {
            attributes: ATTRIBUTES.map(at),
            skills: SKILLS.map(at),
            fortify_max_magicka: at("fortifymaximummagicka"),
            health: at("health"),
            magicka: at("magicka"),
            fatigue: at("fatigue"),
            level: at("level"),
            reputation: at("reputation"),
            bounty: at("bounty"),
            level_progress: at("level.progress"),
            skill_progress: SKILLS.map(|s| at(&format!("progress.{s}"))),
            levelup_attr: ATTRIBUTES.map(|a| at(&format!("levelup.attr.{a}"))),
            levelup_spec: SPECIALIZATIONS.map(|s| at(&format!("levelup.spec.{s}"))),
            werewolf_kills: at("werewolf.kills"),
            magicka_offset: at("magicka.offset"),
            fatigue_offset: at("fatigue.offset"),
            creature: at("actor.creature"),
            class_skills: SKILLS.map(|s| at(&format!("class.{s}"))),
            class_specialization: at("class.specialization"),
            capacity: at("capacity"),
            encumbrance: at("encumbrance"),
            fatigue_term: at("fatigue.term"),
            magnitudes: MAGNITUDE_STATS.map(|(key, _)| at(key)),
        };
        Ok(Self { table, idx, skills })
    }
}
