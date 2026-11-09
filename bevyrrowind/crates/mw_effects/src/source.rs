//! SPEL and ALCH records as effect sources (effects-adapter.md "Records → `SourceDef`"):
//! [`MwSource`], the adapter's `SourceDef` until `sf_effects` has one (E3), with the effect list
//! mapped to `sf_effects` entries and the spell cost.
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `apps/openmw/mwmechanics/spellutil.cpp` (`calcEffectCost` 42-84, `getTotalCost` 23-39,
//! `calcSpellCost` 86-94), `apps/openmw/mwworld/esmstore.cpp` (`getSpellsToReplace` 161-210:
//! entries of unknown effects are dropped, arguments the effect doesn't take are cleared),
//! `apps/openmw/mwmechanics/spellcasting.cpp` (`cast(Potion)`: every potion effect on self, no
//! area), `apps/openmw/mwmechanics/spells.cpp` (`canUsePower`: 24 game hours) and
//! `components/misc/constants.hpp` (`UnitsPerFoot` 21.33, so one foot is 0.3048 m with
//! Morrowind's 64 units per yard).

use mw_stats::{ATTRIBUTES, SKILLS};
use sf_effects::{
    ActorRef, ArgRef, EffectEntry, EffectKey, EffectList, EffectOp, InstanceFlags, MAX_EFFECTS_PER_SOURCE, Range,
    RollSeed, SourceClass, SourceRef,
};
use sf_records::{RecordDb, RecordId};
use sf_stats::StatKey;
use tes3::esp::{Alchemy, Effect, EffectRange, GameSetting, GameSettingValue, Spell, SpellFlags, SpellType};

use crate::mgef::{MwEffectInfo, MwEffects, flag};

/// Metres per Morrowind foot (effect areas are in feet).
pub const FOOT_M: f32 = 0.3048;

/// What kind of affliction a SPEL of type 2-4 is (cured by kind; `spells.cpp` 168-186).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Affliction {
    /// Type 3: common disease.
    Disease,
    /// Type 2: blight disease.
    Blight,
    /// Type 4: curse.
    Curse,
}

/// When a power can be used again.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Recharge {
    /// After this many game days on the world clock (24 game hours each).
    GameDays(u32),
}

/// Per-source switches (`ESM::Spell` flags).
#[allow(clippy::struct_excessive_bools, reason = "independent record flags")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourceFlags {
    /// `F_Always`: casting never fails.
    pub always_succeeds: bool,
    /// `F_Autocalc`: the cost is computed from the effects.
    pub auto_cost: bool,
    /// `F_PCStart`: a new character may start with it.
    pub starts_known: bool,
}

/// One SPEL or ALCH record as an effect source.
#[derive(Clone, Debug, PartialEq)]
pub struct MwSource {
    /// The record.
    pub id: RecordId,
    /// Lifetime, layer and default stacking.
    pub class: SourceClass,
    /// Powers only.
    pub recharge: Option<Recharge>,
    /// Diseases, blights and curses only.
    pub affliction: Option<Affliction>,
    /// The mapped effect list.
    pub effects: EffectList,
    /// Spells: magicka per cast ([`spell_cost`]); potions: `None`.
    pub cost: Option<f32>,
    /// Record flags.
    pub flags: SourceFlags,
}

impl MwSource {
    /// The op that lands this source on a target. Abilities ignore resistances
    /// (`activespells.cpp` 115-126).
    #[must_use]
    pub fn inflict(&self, caster: Option<ActorRef>, rolls: RollSeed) -> EffectOp {
        EffectOp::Inflict {
            source: SourceRef { record: self.id.clone(), item: None },
            class: self.class,
            caster,
            entries: self.effects.clone(),
            flags: InstanceFlags { ignore_resist: self.class == SourceClass::Innate, ..InstanceFlags::default() },
            rolls,
        }
    }
}

/// Something the mapping changed in a record (reported; `OpenMW` logs these at load).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapNote {
    /// An entry of an effect index without an MGEF record was dropped.
    DroppedEffect {
        /// The raw index.
        index: i32,
    },
    /// A skill or attribute effect names no valid skill or attribute: the entry is kept with no
    /// argument (the effect step rejects it as an unknown stat when it lands).
    MissingArg {
        /// The effect.
        effect: EffectKey,
        /// The raw skill or attribute index.
        value: i32,
    },
    /// Entries past [`MAX_EFFECTS_PER_SOURCE`] were dropped.
    TooManyEffects {
        /// How many.
        dropped: usize,
    },
}

/// The GMSTs the mapping reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MwEffectGmst {
    /// `fEffectCostMult`.
    pub effect_cost_mult: f32,
}

impl MwEffectGmst {
    /// Morrowind.esm's values.
    #[must_use]
    pub fn vanilla() -> Self {
        Self { effect_cost_mult: 0.5 }
    }

    /// The values of `db`, and the names that fell back to vanilla.
    #[must_use]
    pub fn from_db(db: &RecordDb) -> (Self, Vec<String>) {
        let mut g = Self::vanilla();
        let mut missing = Vec::new();
        match db.get::<GameSetting>(&RecordId::new("fEffectCostMult")).map(|s| &s.value) {
            Some(GameSettingValue::Float(v)) => g.effect_cost_mult = *v,
            _ => missing.push("fEffectCostMult".to_owned()),
        }
        (g, missing)
    }
}

fn range_of(range: EffectRange) -> Range {
    match range {
        EffectRange::OnSelf => Range::OnSelf,
        EffectRange::OnTouch => Range::Touch,
        EffectRange::OnTarget => Range::Target,
    }
}

/// The entry's MGEF info, if the effect has a record.
fn info_of<'a>(fx: &'a MwEffects, e: &Effect) -> Result<&'a MwEffectInfo, i32> {
    let raw = i32::from(e.magic_effect as i16);
    usize::try_from(raw).ok().and_then(|i| fx.info(i)).filter(|i| i.has_record).ok_or(raw)
}

/// The entry's skill or attribute, for effects that take one; other arguments are cleared.
fn arg_of(info: &MwEffectInfo, e: &Effect, notes: &mut Vec<MapNote>) -> Option<ArgRef> {
    let (value, names): (i32, &[&str]) = if info.has(flag::TARGET_SKILL) {
        (i32::from(e.skill as i8), &SKILLS[..])
    } else if info.has(flag::TARGET_ATTRIBUTE) {
        (i32::from(e.attribute as i8), &ATTRIBUTES[..])
    } else {
        return None;
    };
    if let Some(key) = usize::try_from(value).ok().and_then(|i| names.get(i)) {
        Some(ArgRef::Stat(StatKey::new(key)))
    } else {
        notes.push(MapNote::MissingArg { effect: info.key.clone(), value });
        None
    }
}

/// An ESM effect list as `sf_effects` entries: seconds, metres, `(min, max)` magnitudes.
/// `self_only` (potions, ingredients): every entry on self with no area.
#[allow(clippy::cast_precision_loss, reason = "ESM magnitudes, durations and areas are far below 2^24")]
pub fn map_effects(fx: &MwEffects, effects: &[Effect], self_only: bool, notes: &mut Vec<MapNote>) -> EffectList {
    let mut list = EffectList::new();
    let mut dropped = 0;
    for e in effects {
        let info = match info_of(fx, e) {
            Ok(info) => info,
            Err(index) => {
                notes.push(MapNote::DroppedEffect { index });
                continue;
            }
        };
        if list.len() == MAX_EFFECTS_PER_SOURCE {
            dropped += 1;
            continue;
        }
        let arg = arg_of(info, e, notes);
        let (range, area) = if self_only { (Range::OnSelf, 0.0) } else { (range_of(e.range), e.area as f32 * FOOT_M) };
        list.push(EffectEntry {
            effect: info.key.clone(),
            arg,
            range,
            area,
            duration: e.duration as f32,
            magnitude: (e.min_magnitude as f32, e.max_magnitude as f32),
        });
    }
    if dropped > 0 {
        notes.push(MapNote::TooManyEffects { dropped });
    }
    list
}

/// One entry's cost by `calcEffectCost` with `EffectCostMethod::GameSpell`: magnitudes and
/// duration at least 1 (1 when the effect has none; an applied-once effect keeps duration 0),
/// `0.5 (min + max) · 0.1 base · duration + 0.05 area · base`, times `fEffectCostMult`.
#[allow(clippy::cast_precision_loss, reason = "ESM magnitudes, durations and areas are far below 2^24")]
#[must_use]
pub fn effect_cost(info: &MwEffectInfo, e: &Effect, g: &MwEffectGmst) -> f32 {
    let has_magnitude = !info.has(flag::NO_MAGNITUDE);
    let has_duration = !info.has(flag::NO_DURATION);
    let applied_once = info.has(flag::APPLIED_ONCE);
    let (min, max) = if has_magnitude { (e.min_magnitude.max(1), e.max_magnitude.max(1)) } else { (1, 1) };
    let mut duration = if has_duration { e.duration } else { 1 };
    if !applied_once {
        duration = duration.max(1);
    }
    let base = info.base_cost;
    let mut x = 0.5 * (u64::from(min) + u64::from(max)) as f32;
    x *= 0.1 * base;
    x *= duration as f32;
    x += 0.05 * e.area as f32 * base;
    x * g.effect_cost_mult
}

/// `calcSpellCost`: the record's cost, or with `F_Autocalc` the rounded `getTotalCost` of the
/// effects that map (each cost at least 0, ×1.5 at range target).
#[allow(clippy::cast_precision_loss, reason = "a SPEL cost is far below 2^24")]
#[must_use]
pub fn spell_cost(fx: &MwEffects, g: &MwEffectGmst, spell: &Spell) -> f32 {
    if !spell.data.flags.contains(SpellFlags::AUTO_CALCULATE) {
        return spell.data.cost as f32;
    }
    let mut total = 0.0_f32;
    for e in &spell.effects {
        let Ok(info) = info_of(fx, e) else { continue };
        let mut cost = effect_cost(info, e, g).max(0.0);
        if e.range == EffectRange::OnTarget {
            cost *= 1.5;
        }
        total += cost;
    }
    total.round()
}

/// A SPEL record as a source: type 0 spell `Cast`, 1 ability `Innate`, 2-4 blight, disease,
/// curse `Affliction`, 5 power `Power` (one game day); flags and [`spell_cost`].
#[must_use]
pub fn map_spell(fx: &MwEffects, g: &MwEffectGmst, spell: &Spell) -> (MwSource, Vec<MapNote>) {
    let (class, recharge, affliction) = match spell.data.spell_type {
        SpellType::Spell => (SourceClass::Cast, None, None),
        SpellType::Ability => (SourceClass::Innate, None, None),
        SpellType::Blight => (SourceClass::Affliction, None, Some(Affliction::Blight)),
        SpellType::Disease => (SourceClass::Affliction, None, Some(Affliction::Disease)),
        SpellType::Curse => (SourceClass::Affliction, None, Some(Affliction::Curse)),
        SpellType::Power => (SourceClass::Power, Some(Recharge::GameDays(1)), None),
    };
    let f = spell.data.flags;
    let flags = SourceFlags {
        always_succeeds: f.contains(SpellFlags::ALWAYS_SUCCEEDS),
        auto_cost: f.contains(SpellFlags::AUTO_CALCULATE),
        starts_known: f.contains(SpellFlags::PC_START_SPELL),
    };
    let mut notes = Vec::new();
    let effects = map_effects(fx, &spell.effects, false, &mut notes);
    let source = MwSource {
        id: RecordId::new(&spell.id),
        class,
        recharge,
        affliction,
        effects,
        cost: Some(spell_cost(fx, g, spell)),
        flags,
    };
    (source, notes)
}

/// An ALCH record as a `Consumable` source (stacks): every effect on self, no area, no cost.
#[must_use]
pub fn map_potion(fx: &MwEffects, potion: &Alchemy) -> (MwSource, Vec<MapNote>) {
    let mut notes = Vec::new();
    let effects = map_effects(fx, &potion.effects, true, &mut notes);
    let source = MwSource {
        id: RecordId::new(&potion.id),
        class: SourceClass::Consumable,
        recharge: None,
        affliction: None,
        effects,
        cost: None,
        flags: SourceFlags::default(),
    };
    (source, notes)
}
