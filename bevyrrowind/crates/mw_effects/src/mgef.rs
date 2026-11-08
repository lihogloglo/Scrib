//! The MGEF table (effects-adapter.md "MGEF → `EffectDef`"): the 143 effect indices with
//! `OpenMW`'s key names, display GMSTs and hardcoded flags, the resistance and weakness maps,
//! and [`MwEffects`], the table built from the content's MGEF records.
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `components/esm3/loadmgef.cpp` (`HardcodedFlags` 13-27, `load` 180-210: the legacy format
//! keeps only the record's spellmaking / enchanting / negative-light bits and ORs the hardcoded
//! ones in; `makeResistancesMap` / `makeWeaknessesMap` 290-430; `sGmstEffectIds` 434;
//! `sMagicEffectIds` 584), `components/esm3/loadmgef.hpp` (`Flags` 32-61) and
//! `apps/openmw/mwmechanics/spellresistance.cpp` (`getEffectResistanceAttribute` 75-95: the
//! elemental shields also resist their element).

use smallvec::SmallVec;
use sf_effects::{
    ArgKind, Archetype, EffectDef, EffectFlags, EffectKey, EffectTable, LayerRule, PoolMode, StatTarget,
};
use sf_records::RecordDb;
use sf_stats::StatKey;
use tes3::esp::MagicEffect;

/// MGEF flag bits (`ESM::MagicEffect::Flags`).
pub mod flag {
    /// The entry names a skill.
    pub const TARGET_SKILL: u32 = 0x1;
    /// The entry names an attribute.
    pub const TARGET_ATTRIBUTE: u32 = 0x2;
    /// Instant: lands once on cast.
    pub const NO_DURATION: u32 = 0x4;
    /// Has no magnitude.
    pub const NO_MAGNITUDE: u32 = 0x8;
    /// Counts as hostile.
    pub const HARMFUL: u32 = 0x10;
    /// The hit particle repeats for the whole duration.
    pub const CONTINUOUS_VFX: u32 = 0x20;
    /// Range self allowed.
    pub const CAST_SELF: u32 = 0x40;
    /// Range touch allowed.
    pub const CAST_TOUCH: u32 = 0x80;
    /// Range target allowed.
    pub const CAST_TARGET: u32 = 0x100;
    /// Usable in spellmaking (kept from the record).
    pub const ALLOW_SPELLMAKING: u32 = 0x200;
    /// Usable in enchanting (kept from the record).
    pub const ALLOW_ENCHANTING: u32 = 0x400;
    /// Inverted light colour (kept from the record).
    pub const NEGATIVE_LIGHT: u32 = 0x800;
    /// Lands once instead of continuously.
    pub const APPLIED_ONCE: u32 = 0x1000;
    /// Unused (chameleon).
    pub const STEALTH: u32 = 0x2000;
    /// Refused while the same spell affects the target.
    pub const NON_RECASTABLE: u32 = 0x4000;
    /// Unused.
    pub const ILLEGAL_DAEDRA: u32 = 0x8000;
    /// Never reflected.
    pub const UNREFLECTABLE: u32 = 0x1_0000;
    /// Ends when the caster dies or is not an actor.
    pub const CASTER_LINKED: u32 = 0x2_0000;
    /// The bits a legacy-format MGEF record may set (`loadmgef.cpp` 204).
    pub const RECORD_MASK: u32 = ALLOW_SPELLMAKING | ALLOW_ENCHANTING | NEGATIVE_LIGHT;
}

/// Number of MGEF indices (`ESM::MagicEffect::Length`).
pub const EFFECT_COUNT: usize = 143;

/// Skill keys of the six schools, by MGEF school index (`MagicSchool::indexToSkillRefId`).
pub const SCHOOLS: [&str; 6] = ["alteration", "conjuration", "destruction", "illusion", "mysticism", "restoration"];

/// Per index: `OpenMW` `RefId` name (the key, case-folded), display GMST, hardcoded flags.
const EFFECTS: [(&str, &str, u32); EFFECT_COUNT] = [
    ("WaterBreathing", "sEffectWaterBreathing", 0x11C8), // 0
    ("SwiftSwim", "sEffectSwiftSwim", 0x11C0), // 1
    ("WaterWalking", "sEffectWaterWalking", 0x11C8), // 2
    ("Shield", "sEffectShield", 0x11E0), // 3
    ("FireShield", "sEffectFireShield", 0x11E0), // 4
    ("LightningShield", "sEffectLightningShield", 0x11E0), // 5
    ("FrostShield", "sEffectFrostShield", 0x11E0), // 6
    ("Burden", "sEffectBurden", 0x11D0), // 7
    ("Feather", "sEffectFeather", 0x11C0), // 8
    ("Jump", "sEffectJump", 0x11C0), // 9
    ("Levitate", "sEffectLevitate", 0x11E0), // 10
    ("SlowFall", "sEffectSlowFall", 0x11C0), // 11
    ("Lock", "sEffectLock", 0x1_1184), // 12
    ("Open", "sEffectOpen", 0x1_1184), // 13
    ("FireDamage", "sEffectFireDamage", 0x1F0), // 14
    ("ShockDamage", "sEffectShockDamage", 0x1F0), // 15
    ("FrostDamage", "sEffectFrostDamage", 0x1F0), // 16
    ("DrainAttribute", "sEffectDrainAttribute", 0x11D2), // 17
    ("DrainHealth", "sEffectDrainHealth", 0x11F0), // 18
    ("DrainMagicka", "sEffectDrainSpellpoints", 0x11D0), // 19
    ("DrainFatigue", "sEffectDrainFatigue", 0x11D0), // 20
    ("DrainSkill", "sEffectDrainSkill", 0x11D1), // 21
    ("DamageAttribute", "sEffectDamageAttribute", 0x1D2), // 22
    ("DamageHealth", "sEffectDamageHealth", 0x1F0), // 23
    ("DamageMagicka", "sEffectDamageMagicka", 0x1D0), // 24
    ("DamageFatigue", "sEffectDamageFatigue", 0x1D0), // 25
    ("DamageSkill", "sEffectDamageSkill", 0x1D1), // 26
    ("Poison", "sEffectPoison", 0x1F0), // 27
    ("WeaknessToFire", "sEffectWeaknesstoFire", 0x11D0), // 28
    ("WeaknessToFrost", "sEffectWeaknesstoFrost", 0x11D0), // 29
    ("WeaknessToShock", "sEffectWeaknesstoShock", 0x11D0), // 30
    ("WeaknessToMagicka", "sEffectWeaknesstoMagicka", 0x11D0), // 31
    ("WeaknessToCommonDisease", "sEffectWeaknesstoCommonDisease", 0x11D0), // 32
    ("WeaknessToBlightDisease", "sEffectWeaknesstoBlightDisease", 0x11D0), // 33
    ("WeaknessToCorprusDisease", "sEffectWeaknesstoCorprusDisease", 0x11D0), // 34
    ("WeaknessToPoison", "sEffectWeaknesstoPoison", 0x11D0), // 35
    ("WeaknessToNormalWeapons", "sEffectWeaknesstoNormalWeapons", 0x11D0), // 36
    ("DisintegrateWeapon", "sEffectDisintegrateWeapon", 0x1D0), // 37
    ("DisintegrateArmor", "sEffectDisintegrateArmor", 0x1D0), // 38
    ("Invisibility", "sEffectInvisibility", 0x11C8), // 39
    ("Chameleon", "sEffectChameleon", 0x31C0), // 40
    ("Light", "sEffectLight", 0x11C0), // 41
    ("Sanctuary", "sEffectSanctuary", 0x11C0), // 42
    ("NightEye", "sEffectNightEye", 0x11C0), // 43
    ("Charm", "sEffectCharm", 0x1180), // 44
    ("Paralyze", "sEffectParalyze", 0x11D8), // 45
    ("Silence", "sEffectSilence", 0x11D8), // 46
    ("Blind", "sEffectBlind", 0x11D0), // 47
    ("Sound", "sEffectSound", 0x11D0), // 48
    ("CalmHumanoid", "sEffectCalmHumanoid", 0x1_1180), // 49
    ("CalmCreature", "sEffectCalmCreature", 0x1_1180), // 50
    ("FrenzyHumanoid", "sEffectFrenzyHumanoid", 0x1_1180), // 51
    ("FrenzyCreature", "sEffectFrenzyCreature", 0x1_1180), // 52
    ("DemoralizeHumanoid", "sEffectDemoralizeHumanoid", 0x1_1180), // 53
    ("DemoralizeCreature", "sEffectDemoralizeCreature", 0x1_1180), // 54
    ("RallyHumanoid", "sEffectRallyHumanoid", 0x1_1180), // 55
    ("RallyCreature", "sEffectRallyCreature", 0x1_1180), // 56
    ("Dispel", "sEffectDispel", 0x11C4), // 57
    ("Soultrap", "sEffectSoultrap", 0x1_11B8), // 58
    ("Telekinesis", "sEffectTelekinesis", 0x1040), // 59
    ("Mark", "sEffectMark", 0x104C), // 60
    ("Recall", "sEffectRecall", 0x104C), // 61
    ("DivineIntervention", "sEffectDivineIntervention", 0x104C), // 62
    ("AlmsiviIntervention", "sEffectAlmsiviIntervention", 0x104C), // 63
    ("DetectAnimal", "sEffectDetectAnimal", 0x1040), // 64
    ("DetectEnchantment", "sEffectDetectEnchantment", 0x1040), // 65
    ("DetectKey", "sEffectDetectKey", 0x1040), // 66
    ("SpellAbsorption", "sEffectSpellAbsorption", 0x11C0), // 67
    ("Reflect", "sEffectReflect", 0x11C0), // 68
    ("CureCommonDisease", "sEffectCureCommonDisease", 0x1CC), // 69
    ("CureBlightDisease", "sEffectCureBlightDisease", 0x1CC), // 70
    ("CureCorprusDisease", "sEffectCureCorprusDisease", 0x1CC), // 71
    ("CurePoison", "sEffectCurePoison", 0x1CC), // 72
    ("CureParalyzation", "sEffectCureParalyzation", 0x1CC), // 73
    ("RestoreAttribute", "sEffectRestoreAttribute", 0x1C2), // 74
    ("RestoreHealth", "sEffectRestoreHealth", 0x1C0), // 75
    ("RestoreMagicka", "sEffectRestoreSpellPoints", 0x1C0), // 76
    ("RestoreFatigue", "sEffectRestoreFatigue", 0x1C0), // 77
    ("RestoreSkill", "sEffectRestoreSkill", 0x1C1), // 78
    ("FortifyAttribute", "sEffectFortifyAttribute", 0x11C2), // 79
    ("FortifyHealth", "sEffectFortifyHealth", 0x11C0), // 80
    ("FortifyMagicka", "sEffectFortifySpellpoints", 0x11C0), // 81
    ("FortifyFatigue", "sEffectFortifyFatigue", 0x11C0), // 82
    ("FortifySkill", "sEffectFortifySkill", 0x11C1), // 83
    ("FortifyMaximumMagicka", "sEffectFortifyMagickaMultiplier", 0x11C0), // 84
    ("AbsorbAttribute", "sEffectAbsorbAttribute", 0x2_1192), // 85
    ("AbsorbHealth", "sEffectAbsorbHealth", 0x2_0190), // 86
    ("AbsorbMagicka", "sEffectAbsorbSpellPoints", 0x2_0190), // 87
    ("AbsorbFatigue", "sEffectAbsorbFatigue", 0x2_0190), // 88
    ("AbsorbSkill", "sEffectAbsorbSkill", 0x2_1191), // 89
    ("ResistFire", "sEffectResistFire", 0x11C0), // 90
    ("ResistFrost", "sEffectResistFrost", 0x11C0), // 91
    ("ResistShock", "sEffectResistShock", 0x11C0), // 92
    ("ResistMagicka", "sEffectResistMagicka", 0x11C0), // 93
    ("ResistCommonDisease", "sEffectResistCommonDisease", 0x11C0), // 94
    ("ResistBlightDisease", "sEffectResistBlightDisease", 0x11C0), // 95
    ("ResistCorprusDisease", "sEffectResistCorprusDisease", 0x11C0), // 96
    ("ResistPoison", "sEffectResistPoison", 0x11C0), // 97
    ("ResistNormalWeapons", "sEffectResistNormalWeapons", 0x11C0), // 98
    ("ResistParalysis", "sEffectResistParalysis", 0x11C0), // 99
    ("RemoveCurse", "sEffectRemoveCurse", 0x1C0), // 100
    ("TurnUndead", "sEffectTurnUndead", 0x1_1190), // 101
    ("SummonScamp", "sEffectSummonScamp", 0x9048), // 102
    ("SummonClannfear", "sEffectSummonClannfear", 0x9048), // 103
    ("SummonDaedroth", "sEffectSummonDaedroth", 0x9048), // 104
    ("SummonDremora", "sEffectSummonDremora", 0x9048), // 105
    ("SummonAncestralGhost", "sEffectSummonAncestralGhost", 0x9048), // 106
    ("SummonSkeletalMinion", "sEffectSummonSkeletalMinion", 0x9048), // 107
    ("SummonBonewalker", "sEffectSummonLeastBonewalker", 0x9048), // 108
    ("SummonGreaterBonewalker", "sEffectSummonGreaterBonewalker", 0x9048), // 109
    ("SummonBonelord", "sEffectSummonBonelord", 0x9048), // 110
    ("SummonWingedTwilight", "sEffectSummonWingedTwilight", 0x9048), // 111
    ("SummonHunger", "sEffectSummonHunger", 0x9048), // 112
    ("SummonGoldenSaint", "sEffectSummonGoldenSaint", 0x9048), // 113
    ("SummonFlameAtronach", "sEffectSummonFlameAtronach", 0x9048), // 114
    ("SummonFrostAtronach", "sEffectSummonFrostAtronach", 0x9048), // 115
    ("SummonStormAtronach", "sEffectSummonStormAtronach", 0x9048), // 116
    ("FortifyAttack", "sEffectFortifyAttackBonus", 0x11C0), // 117
    ("CommandCreature", "sEffectCommandCreatures", 0x1180), // 118
    ("CommandHumanoid", "sEffectCommandHumanoids", 0x1180), // 119
    ("BoundDagger", "sEffectBoundDagger", 0x5048), // 120
    ("BoundLongsword", "sEffectBoundLongsword", 0x5048), // 121
    ("BoundMace", "sEffectBoundMace", 0x5048), // 122
    ("BoundBattleAxe", "sEffectBoundBattleAxe", 0x5048), // 123
    ("BoundSpear", "sEffectBoundSpear", 0x5048), // 124
    ("BoundLongbow", "sEffectBoundLongbow", 0x5048), // 125
    ("ExtraSpell", "sEffectExtraSpell", 0x1188), // 126
    ("BoundCuirass", "sEffectBoundCuirass", 0x5048), // 127
    ("BoundHelm", "sEffectBoundHelm", 0x5048), // 128
    ("BoundBoots", "sEffectBoundBoots", 0x5048), // 129
    ("BoundShield", "sEffectBoundShield", 0x5048), // 130
    ("BoundGloves", "sEffectBoundGloves", 0x5048), // 131
    ("Corprus", "sEffectCorpus", 0x1048), // 132
    ("Vampirism", "sEffectVampirism", 0x104C), // 133
    ("SummonCenturionSphere", "sEffectSummonCenturionSphere", 0x1048), // 134
    ("SunDamage", "sEffectSunDamage", 0x40), // 135
    ("StuntedMagicka", "sEffectStuntedMagicka", 0x11C8), // 136
    ("SummonFabricant", "sEffectSummonFabricant", 0x1048), // 137
    ("SummonWolf", "sEffectSummonCreature01", 0x1048), // 138
    ("SummonBear", "sEffectSummonCreature02", 0x1048), // 139
    ("SummonBonewolf", "sEffectSummonCreature03", 0x1048), // 140
    ("SummonCreature04", "sEffectSummonCreature04", 0x1048), // 141
    ("SummonCreature05", "sEffectSummonCreature05", 0x1048), // 142
];

// Indices named below (`ESM::MagicEffect` order).
const FIRE_SHIELD: usize = 4;
const LIGHTNING_SHIELD: usize = 5;
const FROST_SHIELD: usize = 6;
const FIRE_DAMAGE: usize = 14;
const SHOCK_DAMAGE: usize = 15;
const FROST_DAMAGE: usize = 16;
const POISON: usize = 27;
const WEAKNESS_TO_FIRE: usize = 28;
const WEAKNESS_TO_FROST: usize = 29;
const WEAKNESS_TO_SHOCK: usize = 30;
const WEAKNESS_TO_MAGICKA: usize = 31;
const WEAKNESS_TO_COMMON_DISEASE: usize = 32;
const WEAKNESS_TO_CORPRUS_DISEASE: usize = 34;
const WEAKNESS_TO_POISON: usize = 35;
const SANCTUARY: usize = 42;
const PARALYZE: usize = 45;
const RESIST_FIRE: usize = 90;
const RESIST_FROST: usize = 91;
const RESIST_SHOCK: usize = 92;
const RESIST_MAGICKA: usize = 93;
const RESIST_COMMON_DISEASE: usize = 94;
const RESIST_CORPRUS_DISEASE: usize = 96;
const RESIST_POISON: usize = 97;
const RESIST_PARALYSIS: usize = 99;
const CORPRUS: usize = 132;
const VAMPIRISM: usize = 133;

/// The three pools in the order of their MGEF effect runs (health, magicka, fatigue).
const POOLS: [&str; 3] = ["health", "magicka", "fatigue"];

/// `OpenMW`'s `RefId` name of effect `index` (`FortifyAttribute`).
#[must_use]
pub fn effect_name(index: usize) -> Option<&'static str> {
    EFFECTS.get(index).map(|e| e.0)
}

/// The display-name GMST of effect `index` (`sEffectFortifyAttribute`): the UI's loc key.
#[must_use]
pub fn effect_label(index: usize) -> Option<&'static str> {
    EFFECTS.get(index).map(|e| e.1)
}

/// The effect's save and wire key: its `RefId` name, case-folded (`fortifyattribute`).
#[must_use]
pub fn effect_key(index: usize) -> Option<EffectKey> {
    effect_name(index).map(EffectKey::new)
}

/// The index of an effect key or `RefId` name (case-insensitive).
#[must_use]
pub fn effect_index(key: &str) -> Option<usize> {
    EFFECTS.iter().position(|e| e.0.eq_ignore_ascii_case(key))
}

/// `HardcodedFlags[index]`; 0 past the table.
#[must_use]
pub fn hardcoded_flags(index: usize) -> u32 {
    EFFECTS.get(index).map_or(0, |e| e.2)
}

/// A legacy-format record's flags as `OpenMW` loads them: the record keeps only
/// [`flag::RECORD_MASK`], the hardcoded flags are ORed in.
#[must_use]
pub fn effective_flags(index: usize, record_flags: u32) -> u32 {
    (record_flags & flag::RECORD_MASK) | hardcoded_flags(index)
}

/// The stat a magnitude-stat effect writes: its own key (`resistfire`). For Fortify Maximum
/// Magicka this is `mw_stats`'s `fortifymaximummagicka`. Panics past the table.
pub(crate) fn magnitude_stat_key(index: usize) -> StatKey {
    StatKey::new(EFFECTS[index].0)
}

/// The effect whose magnitude resists effect `index` (`MagicEffect::getResistanceEffect`).
#[must_use]
pub fn resistance_effect(index: usize) -> Option<usize> {
    match index {
        37 | 38 => Some(SANCTUARY),
        7 | 17..=26 | 28..=36 | 44 | 46..=56 | 85..=89 | 101 => Some(RESIST_MAGICKA),
        FIRE_DAMAGE => Some(RESIST_FIRE),
        FROST_DAMAGE => Some(RESIST_FROST),
        SHOCK_DAMAGE => Some(RESIST_SHOCK),
        VAMPIRISM => Some(RESIST_COMMON_DISEASE),
        CORPRUS => Some(RESIST_CORPRUS_DISEASE),
        POISON => Some(RESIST_POISON),
        PARALYZE => Some(RESIST_PARALYSIS),
        _ => None,
    }
}

/// The effect whose magnitude weakens resistance to effect `index`
/// (`MagicEffect::getWeaknessEffect`).
#[must_use]
pub fn weakness_effect(index: usize) -> Option<usize> {
    match index {
        7 | 17..=26 | 28..=36 | 44 | 46..=56 | 85..=89 | 101 => Some(WEAKNESS_TO_MAGICKA),
        FIRE_DAMAGE => Some(WEAKNESS_TO_FIRE),
        FROST_DAMAGE => Some(WEAKNESS_TO_FROST),
        SHOCK_DAMAGE => Some(WEAKNESS_TO_SHOCK),
        VAMPIRISM => Some(WEAKNESS_TO_COMMON_DISEASE),
        CORPRUS => Some(WEAKNESS_TO_CORPRUS_DISEASE),
        POISON => Some(WEAKNESS_TO_POISON),
        _ => None,
    }
}

/// The elemental shield that also resists effect `index` (`getEffectResistanceAttribute`).
#[must_use]
pub fn shield_effect(index: usize) -> Option<usize> {
    match index {
        FIRE_DAMAGE => Some(FIRE_SHIELD),
        SHOCK_DAMAGE => Some(LIGHTNING_SHIELD),
        FROST_DAMAGE => Some(FROST_SHIELD),
        _ => None,
    }
}

/// The archetype and argument kind of effect `index` (effects-adapter.md "Vanilla effects by
/// archetype"); `None` for effects outside slice A1. Damage attribute / skill cap at the
/// modified value (`spelleffects.cpp` `damageAttribute` / `damageSkill`); the fortify layer
/// follows the source (`Flag_AffectsBaseValues` for abilities).
#[must_use]
pub fn archetype(index: usize) -> Option<(Archetype, ArgKind)> {
    let pool = |k: &str, mode: PoolMode| Archetype::Pool { pool: StatTarget::Fixed(StatKey::new(k)), mode };
    let attribute = || ArgKind::Stat("attribute".to_owned());
    let skill = || ArgKind::Stat("skill".to_owned());
    Some(match index {
        // Fire / Lightning / Frost Shield, Weakness*, Fortify Maximum Magicka, Resist*.
        FIRE_SHIELD..=FROST_SHIELD | 28..=36 | 84 | 90..=99 => {
            (Archetype::MagnitudeStat { stat: StatTarget::Fixed(magnitude_stat_key(index)) }, ArgKind::None)
        }
        FIRE_DAMAGE..=FROST_DAMAGE | POISON => (pool("health", PoolMode::Hurt), ArgKind::None),
        17 => (Archetype::Drain { stat: StatTarget::Arg }, attribute()),
        18..=20 => (pool(POOLS[index - 18], PoolMode::Drain), ArgKind::None),
        21 => (Archetype::Drain { stat: StatTarget::Arg }, skill()),
        22 => (Archetype::Damage { stat: StatTarget::Arg, cap_at_modified: true }, attribute()),
        23..=25 => (pool(POOLS[index - 23], PoolMode::Hurt), ArgKind::None),
        26 => (Archetype::Damage { stat: StatTarget::Arg, cap_at_modified: true }, skill()),
        74 => (Archetype::Restore { stat: StatTarget::Arg }, attribute()),
        75..=77 => (pool(POOLS[index - 75], PoolMode::Heal), ArgKind::None),
        78 => (Archetype::Restore { stat: StatTarget::Arg }, skill()),
        79 => (Archetype::Modify { stat: StatTarget::Arg, layer: LayerRule::BySource }, attribute()),
        80..=82 => (pool(POOLS[index - 80], PoolMode::Fortify), ArgKind::None),
        83 => (Archetype::Modify { stat: StatTarget::Arg, layer: LayerRule::BySource }, skill()),
        _ => return None,
    })
}

/// The framework's switches from MGEF flags.
#[must_use]
pub fn effect_flags(flags: u32) -> EffectFlags {
    let has = |bit: u32| flags & bit != 0;
    EffectFlags {
        no_magnitude: has(flag::NO_MAGNITUDE),
        no_duration: has(flag::NO_DURATION),
        applied_once: has(flag::APPLIED_ONCE),
        harmful: has(flag::HARMFUL),
        unreflectable: has(flag::UNREFLECTABLE),
        caster_linked: has(flag::CASTER_LINKED),
        non_recastable: has(flag::NON_RECASTABLE),
    }
}

/// The ranges an effect may be cast at (spellmaking, enchanting; read by casting, A5).
#[allow(clippy::struct_excessive_bools, reason = "independent MGEF flags")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RangeFlags {
    /// Self.
    pub on_self: bool,
    /// Touch.
    pub touch: bool,
    /// Target.
    pub target: bool,
}

impl RangeFlags {
    /// The ranges of MGEF `flags`.
    #[must_use]
    pub fn of(flags: u32) -> Self {
        Self {
            on_self: flags & flag::CAST_SELF != 0,
            touch: flags & flag::CAST_TOUCH != 0,
            target: flags & flag::CAST_TARGET != 0,
        }
    }
}

/// What presentation reads of an MGEF record (net rule 3: never gameplay).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Presentation {
    /// `ITEX`: the effect icon.
    pub icon: String,
    /// `PTEX`: the particle texture.
    pub particle: String,
    /// `CVFX`: the cast static.
    pub cast_static: String,
    /// `BVFX`: the bolt static.
    pub bolt_static: String,
    /// `HVFX`: the hit static.
    pub hit_static: String,
    /// `AVFX`: the area static.
    pub area_static: String,
    /// `CSND`: the cast sound.
    pub cast_sound: String,
    /// `BSND`: the bolt sound.
    pub bolt_sound: String,
    /// `HSND`: the hit sound.
    pub hit_sound: String,
    /// `ASND`: the area sound.
    pub area_sound: String,
    /// Light colour (red, green, blue).
    pub colour: (i32, i32, i32),
    /// Bolt speed.
    pub speed: f32,
    /// Particle size.
    pub size: f32,
    /// Particle size cap.
    pub size_cap: f32,
    /// `DESC`.
    pub description: String,
}

/// One MGEF index as loaded: key, school, cost, effective flags, presentation.
#[derive(Clone, Debug, PartialEq)]
pub struct MwEffectInfo {
    /// The ESM index (0..[`EFFECT_COUNT`]).
    pub index: usize,
    /// [`effect_key`] of the index.
    pub key: EffectKey,
    /// The school's skill key ([`SCHOOLS`]).
    pub school: StatKey,
    /// `MEDT` base cost.
    pub base_cost: f32,
    /// [`effective_flags`]: hardcoded flags alone when there is no record.
    pub flags: u32,
    /// The content has an MGEF record for this index. `OpenMW` drops effect-list entries of
    /// indices without one (`mwworld/esmstore.cpp` `getSpellsToReplace` 161-210).
    pub has_record: bool,
    /// Presentation fields of the record.
    pub presentation: Presentation,
}

impl MwEffectInfo {
    /// The index with no record: hardcoded flags, school alteration (ESM default), cost 0.
    fn hardcoded(index: usize) -> Self {
        Self {
            index,
            key: EffectKey::new(EFFECTS[index].0),
            school: StatKey::new(SCHOOLS[0]),
            base_cost: 0.0,
            flags: hardcoded_flags(index),
            has_record: false,
            presentation: Presentation::default(),
        }
    }

    fn load(&mut self, r: &MagicEffect) {
        let school = r.data.school as usize;
        self.school = StatKey::new(SCHOOLS.get(school).copied().unwrap_or(SCHOOLS[0]));
        self.base_cost = r.data.base_cost;
        self.flags = effective_flags(self.index, r.data.flags.bits());
        self.has_record = true;
        self.presentation = Presentation {
            icon: r.icon.clone(),
            particle: r.texture.clone(),
            cast_static: r.cast_visual.clone(),
            bolt_static: r.bolt_visual.clone(),
            hit_static: r.hit_visual.clone(),
            area_static: r.area_visual.clone(),
            cast_sound: r.cast_sound.clone(),
            bolt_sound: r.bolt_sound.clone(),
            hit_sound: r.hit_sound.clone(),
            area_sound: r.area_sound.clone(),
            colour: r.data.color,
            speed: r.data.speed,
            size: r.data.size,
            size_cap: r.data.size_cap,
            description: r.description.clone(),
        };
    }

    /// Every bit of `bits` is set.
    #[must_use]
    pub fn has(&self, bits: u32) -> bool {
        self.flags & bits == bits
    }

    /// The ranges it may be cast at.
    #[must_use]
    pub fn ranges(&self) -> RangeFlags {
        RangeFlags::of(self.flags)
    }

    /// The framework def, for effects of slice A1 (else `None`): archetype, argument, flags,
    /// cost, resistances as magnitude stats, GMST label; stacking from the source class.
    #[must_use]
    pub fn def(&self) -> Option<EffectDef> {
        let i = self.index;
        let (archetype, arg) = archetype(i)?;
        let mut def = EffectDef::new(self.key.as_str(), archetype);
        def.arg = arg;
        def.flags = effect_flags(self.flags);
        def.base_cost = self.base_cost;
        def.resisted_by = resistance_effect(i).into_iter().chain(shield_effect(i)).map(magnitude_stat_key).collect();
        def.weakened_by = weakness_effect(i).into_iter().map(magnitude_stat_key).collect::<SmallVec<_>>();
        def.label = EFFECTS[i].1.to_owned();
        Some(def)
    }
}

/// The content's effects: an [`EffectTable`] of the A1 defs, and every index's [`MwEffectInfo`].
#[derive(Clone, Debug)]
pub struct MwEffects {
    table: EffectTable,
    info: Vec<MwEffectInfo>,
    bad_records: Vec<i32>,
}

impl MwEffects {
    /// The table from the MGEF records of `db`.
    #[must_use]
    pub fn from_db(db: &RecordDb) -> Self {
        Self::from_records(db.iter::<MagicEffect>().map(|(_, r)| r))
    }

    /// The table from MGEF records in load order (a later record of an index replaces the
    /// earlier). Records with an index outside the table are ignored and listed in
    /// [`Self::bad_records`] (`OpenMW` fails the load: "Invalid Index!").
    pub fn from_records<'a>(records: impl IntoIterator<Item = &'a MagicEffect>) -> Self {
        let mut info: Vec<MwEffectInfo> = (0..EFFECT_COUNT).map(MwEffectInfo::hardcoded).collect();
        let mut bad_records = Vec::new();
        for r in records {
            let raw = r.effect_id as i32;
            match usize::try_from(raw).ok().filter(|&i| i < EFFECT_COUNT) {
                Some(i) => info[i].load(r),
                None => bad_records.push(raw),
            }
        }
        let defs = info.iter().filter_map(MwEffectInfo::def).collect();
        let table = EffectTable::new(defs).expect("the 143 effect names are distinct");
        Self { table, info, bad_records }
    }

    /// The framework table (A1 defs only).
    #[must_use]
    pub fn table(&self) -> &EffectTable {
        &self.table
    }

    /// Index `index`'s info; `None` past the table.
    #[must_use]
    pub fn info(&self, index: usize) -> Option<&MwEffectInfo> {
        self.info.get(index)
    }

    /// The info of an effect key.
    #[must_use]
    pub fn info_by_key(&self, key: &EffectKey) -> Option<&MwEffectInfo> {
        effect_index(key.as_str()).and_then(|i| self.info(i))
    }

    /// Indices without an MGEF record (their entries are dropped by the source mapping).
    pub fn missing(&self) -> impl Iterator<Item = usize> + '_ {
        self.info.iter().filter(|i| !i.has_record).map(|i| i.index)
    }

    /// Indices of MGEF records outside the table, as read.
    #[must_use]
    pub fn bad_records(&self) -> &[i32] {
        &self.bad_records
    }
}
