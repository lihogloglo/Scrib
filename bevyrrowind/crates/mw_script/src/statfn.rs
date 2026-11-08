//! The stat, faction and crime functions as typed calls (stats.md S7, stats-adapter.md
//! "Consumers", MWScript bullet): the interpreter decodes them by name once per table and hands
//! each call to [`crate::Host::stat`] as a [`StatCall`]. Semantics live in the host (`mw_rpg`'s
//! runner ports `OpenMW`'s `statsextensions.cpp` onto `sf_stats` ops); this module only names
//! the calls and where their arguments are.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::ast::Ident;

/// The eight attributes in ESM order (`functions.txt` `$ATTRS`).
pub const ATTRIBUTES: [&str; 8] =
    ["Strength", "Intelligence", "Willpower", "Agility", "Speed", "Endurance", "Personality", "Luck"];

/// The 27 skills in ESM order (`functions.txt` `$SKILLS`).
pub const SKILLS: [&str; 27] = [
    "Block",
    "Armorer",
    "MediumArmor",
    "HeavyArmor",
    "BluntWeapon",
    "LongBlade",
    "Axe",
    "Spear",
    "Athletics",
    "Enchant",
    "Destruction",
    "Alteration",
    "Illusion",
    "Conjuration",
    "Mysticism",
    "Restoration",
    "Alchemy",
    "Unarmored",
    "Security",
    "Sneak",
    "Acrobatics",
    "LightArmor",
    "ShortBlade",
    "Marksman",
    "Mercantile",
    "Speechcraft",
    "HandToHand",
];

/// A dynamic stat (`functions.txt` `$DYN`, `OpenMW`'s dynamic index 0-2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dynamic {
    Health,
    Magicka,
    Fatigue,
}

impl Dynamic {
    pub const ALL: [Dynamic; 3] = [Dynamic::Health, Dynamic::Magicka, Dynamic::Fatigue];

    pub fn name(self) -> &'static str {
        match self {
            Dynamic::Health => "Health",
            Dynamic::Magicka => "Magicka",
            Dynamic::Fatigue => "Fatigue",
        }
    }
}

/// One stat, faction or crime function. Attribute and skill indices are ESM order
/// ([`ATTRIBUTES`], [`SKILLS`]). "Pc" functions act on the context player whatever the target;
/// the target picks the faction when the faction argument is omitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatFn {
    GetAttribute(u8),
    SetAttribute(u8),
    ModAttribute(u8),
    GetSkill(u8),
    SetSkill(u8),
    ModSkill(u8),
    GetDynamic(Dynamic),
    SetDynamic(Dynamic),
    ModDynamic(Dynamic),
    ModCurrentDynamic(Dynamic),
    /// `Get<Dyn>GetRatio`.
    GetDynamicRatio(Dynamic),
    GetLevel,
    SetLevel,
    /// The target's own reputation (`dialogueextensions.cpp`), not a faction reputation.
    GetReputation,
    SetReputation,
    ModReputation,
    GetPcCrimeLevel,
    SetPcCrimeLevel,
    ModPcCrimeLevel,
    PcJoinFaction,
    PcRaiseRank,
    PcLowerRank,
    GetPcRank,
    PcExpell,
    PcExpelled,
    PcClearExpelled,
    GetPcFacRep,
    SetPcFacRep,
    ModPcFacRep,
    /// The target NPC's own rank in its faction.
    RaiseRank,
    LowerRank,
}

impl StatFn {
    /// The function a table row names (case-insensitive), `None` for every other function.
    pub fn from_name(name: &str) -> Option<StatFn> {
        static BY_NAME: OnceLock<HashMap<String, StatFn>> = OnceLock::new();
        BY_NAME.get_or_init(build).get(&name.to_lowercase()).copied()
    }

    /// Every function with its canonical `functions.txt` name.
    pub fn all() -> Vec<(String, StatFn)> {
        let mut out = Vec::new();
        for (i, a) in (0u8..).zip(ATTRIBUTES) {
            out.push((format!("Get{a}"), StatFn::GetAttribute(i)));
            out.push((format!("Set{a}"), StatFn::SetAttribute(i)));
            out.push((format!("Mod{a}"), StatFn::ModAttribute(i)));
        }
        for (i, s) in (0u8..).zip(SKILLS) {
            out.push((format!("Get{s}"), StatFn::GetSkill(i)));
            out.push((format!("Set{s}"), StatFn::SetSkill(i)));
            out.push((format!("Mod{s}"), StatFn::ModSkill(i)));
        }
        for d in Dynamic::ALL {
            let n = d.name();
            out.push((format!("Get{n}"), StatFn::GetDynamic(d)));
            out.push((format!("Set{n}"), StatFn::SetDynamic(d)));
            out.push((format!("Mod{n}"), StatFn::ModDynamic(d)));
            out.push((format!("ModCurrent{n}"), StatFn::ModCurrentDynamic(d)));
            out.push((format!("Get{n}GetRatio"), StatFn::GetDynamicRatio(d)));
        }
        let named = [
            ("GetLevel", StatFn::GetLevel),
            ("SetLevel", StatFn::SetLevel),
            ("GetReputation", StatFn::GetReputation),
            ("SetReputation", StatFn::SetReputation),
            ("ModReputation", StatFn::ModReputation),
            ("GetPCCrimeLevel", StatFn::GetPcCrimeLevel),
            ("SetPCCrimeLevel", StatFn::SetPcCrimeLevel),
            ("ModPCCrimeLevel", StatFn::ModPcCrimeLevel),
            ("PCJoinFaction", StatFn::PcJoinFaction),
            ("PCRaiseRank", StatFn::PcRaiseRank),
            ("PCLowerRank", StatFn::PcLowerRank),
            ("GetPCRank", StatFn::GetPcRank),
            ("PCExpell", StatFn::PcExpell),
            ("PCExpelled", StatFn::PcExpelled),
            ("PCClearExpelled", StatFn::PcClearExpelled),
            ("GetPCFacRep", StatFn::GetPcFacRep),
            ("SetPCFacRep", StatFn::SetPcFacRep),
            ("ModPCFacRep", StatFn::ModPcFacRep),
            ("RaiseRank", StatFn::RaiseRank),
            ("LowerRank", StatFn::LowerRank),
        ];
        out.extend(named.into_iter().map(|(n, f)| (n.to_owned(), f)));
        out
    }

    /// The argument index of the numeric value (`Set`/`Mod` amounts), `None` without one.
    pub fn value_arg(self) -> Option<usize> {
        use StatFn as F;
        match self {
            F::SetAttribute(_)
            | F::ModAttribute(_)
            | F::SetSkill(_)
            | F::ModSkill(_)
            | F::SetDynamic(_)
            | F::ModDynamic(_)
            | F::ModCurrentDynamic(_)
            | F::SetLevel
            | F::SetReputation
            | F::ModReputation
            | F::SetPcCrimeLevel
            | F::ModPcCrimeLevel
            | F::SetPcFacRep
            | F::ModPcFacRep => Some(0),
            _ => None,
        }
    }

    /// The argument index of the optional faction id, `None` for functions without one.
    pub fn faction_arg(self) -> Option<usize> {
        use StatFn as F;
        match self {
            F::PcJoinFaction
            | F::PcRaiseRank
            | F::PcLowerRank
            | F::GetPcRank
            | F::PcExpell
            | F::PcExpelled
            | F::PcClearExpelled
            | F::GetPcFacRep => Some(0),
            F::SetPcFacRep | F::ModPcFacRep => Some(1),
            _ => None,
        }
    }

    /// What a read returns when there is no character to read (no sheet loaded, an unknown
    /// target): `GetPCRank` -1 (not a member, `OpenMW`'s value for no faction), else 0.
    pub fn no_sheet_value(self) -> f64 {
        if self == StatFn::GetPcRank { -1.0 } else { 0.0 }
    }
}

fn build() -> HashMap<String, StatFn> {
    StatFn::all().into_iter().map(|(name, f)| (name.to_lowercase(), f)).collect()
}

/// One call, arguments evaluated: what [`crate::Host::stat`] receives.
#[derive(Clone, Debug, PartialEq)]
pub struct StatCall {
    pub function: StatFn,
    /// The numeric argument ([`StatFn::value_arg`]); 0 when there is none or it is missing.
    pub value: f64,
    /// The faction argument as written; `None` when omitted (the target's own faction).
    pub faction: Option<Ident>,
}
