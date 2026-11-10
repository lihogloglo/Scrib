//! Spellcasting rules (effects-adapter.md "Casting rules", slice A5a) behind the framework's
//! [`CastModel`] seam: [`OpenMwEffectRules`] (classic, `OpenMW`'s cast chance and fizzle) and
//! [`MwModernEffectRules`] (modern, the game's default: no fizzle roll, Sound raises the cost).
//! The landing and item seams of both are the framework's [`StandardEffectRules`] until the
//! classic resist, reflect and charge rules are ported (adapter A6, A7).
//!
//! The step (`sf_effects` "Casting") refuses before the plan an unknown or unlearned source, a
//! non-castable one (abilities, diseases, curses, blights: never cast) and a power used less
//! than its recharge ago; it spends the plan's cost from the caster's magicka, then rolls
//! `roll < chance` (else `Fizzle`, cost spent) and records the power use on the world clock.
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f` (GPL-3.0):
//! - `apps/openmw/mwmechanics/spellutil.cpp` `calcSpellBaseSuccessChance` 225-270 (per effect
//!   `x = max(1, duration)` unless applied once, `× 0.1 baseCost × (min + max) / 2 + area × 0.05
//!   baseCost`, `× 1.5` at range target, `× fEffectCostMult`; the effect with the lowest `2 ×
//!   school skill - x` picks the school; chance `2 × skill - calcSpellCost + 0.2 Willpower + 0.1
//!   Luck`, modified attributes and skills), `getSpellSuccessChance` 272-308 (a power: 100 when
//!   usable; god mode 100; Silence 0; not a spell 100; `F_Always` 100; else `(base - Sound) ×
//!   fatigue term`, clamped to 0..100 for the cast) and `getSpellSchool` 318-330.
//! - `apps/openmw/mwmechanics/spellcasting.cpp` `CastSpell::cast(const ESM::Spell*)` 330-372: the
//!   roll `roll0to99 >= chance` fails (`sMagicSkillFail`, the school's failure sound), a power's
//!   use is recorded on success only; skill use `Spellcast_Success` for the player when
//!   `spellIncreasesSkill` (spells without `F_Always`).
//! - `apps/openmw/mwworld/worldimp.cpp` `World::startSpellCast` 2833-2885: at the cast's start a
//!   cost above 0 needs `current magicka >= cost` (else `sMagicInsufficientSP`, nothing spent),
//!   then the cost is taken; a failed roll does not give it back. The same check covers powers.
//! - `apps/openmw/mwmechanics/spells.cpp` `canUsePower` / `usePower` 200-217 (24 game hours since
//!   the last use; the framework's `SpellBook::recharging`, `MwSource::recharge`).
//! - `apps/openmw/mwmechanics/creaturestats.cpp` `getFatigueTerm` 39-53 ([`mw_stats::fatigue_term`]).
//!
//! The integer roll as a uniform one: `roll0to99 < chance` with `roll0to99 = floor(100 u)` holds
//! exactly when `u < ceil(chance) / 100`, so the plan's chance is `ceil(chance) / 100` and the
//! step's `roll < chance` keeps `OpenMW`'s odds.
//!
//! Not ported here: god mode (a debug cheat, no flag yet), the cast's fatigue loss
//! (`startSpellCast`: `cost × (fFatigueSpellBase + encumbrance × fFatigueSpellMult)`; the
//! `OpenMW` source says both GMSTs are 0 in vanilla, unverified here), the skill use on a success
//! (host glue, with [`spell_school`]) and the failure sound and messages (presentation).

use std::sync::Arc;

use mw_stats::{MwGmst, fatigue_term};
use sf_effects::{
    CastFailReason, CastModel, CastPlan, CastQuery, EffectEntry, ItemModel, ItemQuery, LandModel,
    ProtectQuery, Protection, Range, ResistQuery, RollQuery, SourceClass, StandardEffectRules,
};
use sf_stats::{Role, SheetRead, StatKey};
use smallvec::SmallVec;

use crate::mgef::{MwEffects, flag};
use crate::source::{FOOT_M, MwEffectGmst};

/// The magnitude stat of Silence (`mw_stats::MAGNITUDE_STATS`).
pub const SILENCE_STAT: &str = "silence";
/// The magnitude stat of Sound.
pub const SOUND_STAT: &str = "sound";

/// One effect of a spell as `calcSpellBaseSuccessChance` reads it (record units).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChanceEffectIn {
    /// The MGEF base cost.
    pub base_cost: f32,
    /// The caster's modified skill of the effect's school.
    pub school_skill: f32,
    /// Record seconds.
    pub duration: f32,
    /// Record `(min, max)`.
    pub magnitude: (f32, f32),
    /// Record feet.
    pub area: f32,
    /// Range target (`× 1.5`).
    pub on_target: bool,
    /// MGEF `AppliedOnce` (a duration of 0 stays 0).
    pub applied_once: bool,
}

/// Input of rule `mw.effects.cast_chance` (classic): everything one spell's chance reads.
#[derive(Clone, Debug, PartialEq)]
pub struct CastChanceIn {
    pub effects: SmallVec<[ChanceEffectIn; 4]>,
    /// `calcSpellCost`: the record's cost, or the autocalc total.
    pub spell_cost: f32,
    /// Modified Willpower and Luck.
    pub willpower: f32,
    pub luck: f32,
    /// The caster's Sound and Silence magnitudes.
    pub sound: f32,
    pub silence: f32,
    /// [`mw_stats::fatigue_term`] of the caster.
    pub fatigue_term: f32,
    pub class: SourceClass,
    /// SPEL `F_Always`.
    pub always_succeeds: bool,
    /// `fEffectCostMult`.
    pub effect_cost_mult: f32,
}

/// Output of `calcSpellBaseSuccessChance`: the uncapped base chance and the index (into the
/// input's effects) of the effect whose school the cast uses; `None` for an empty spell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BaseChance {
    pub chance: f32,
    pub school_effect: Option<usize>,
}

/// `calcSpellBaseSuccessChance` (module docs): the weakest school and `2 × its skill - cost +
/// 0.2 Willpower + 0.1 Luck`, in `f32` as `OpenMW`.
#[must_use]
pub fn base_success_chance(i: &CastChanceIn) -> BaseChance {
    let mut y = f32::MAX;
    let mut lowest_skill = 0.0;
    let mut school_effect = None;
    for (n, e) in i.effects.iter().enumerate() {
        let mut x = e.duration;
        if !e.applied_once {
            x = x.max(1.0);
        }
        x *= 0.1 * e.base_cost;
        // `0.5 × (min + max)`: halving is exact, so this is the same value.
        x *= f32::midpoint(e.magnitude.0, e.magnitude.1);
        x += e.area * 0.05 * e.base_cost;
        if e.on_target {
            x *= 1.5;
        }
        x *= i.effect_cost_mult;
        let s = 2.0 * e.school_skill;
        if s - x < y {
            y = s - x;
            school_effect = Some(n);
            lowest_skill = s;
        }
    }
    let chance = lowest_skill - i.spell_cost + 0.2 * i.willpower + 0.1 * i.luck;
    BaseChance {
        chance,
        school_effect,
    }
}

/// Rule `mw.effects.cast_chance` (classic): `getSpellSuccessChance` with the cap, in percent
/// (0..=100). The power's recharge is the step's (a recharging power never gets here).
#[must_use]
pub fn success_chance(i: &CastChanceIn) -> f32 {
    let base = base_success_chance(i).chance;
    if i.class == SourceClass::Power {
        return 100.0;
    }
    if i.silence != 0.0 {
        return 0.0;
    }
    if i.class != SourceClass::Cast || i.always_succeeds {
        return 100.0;
    }
    ((base - i.sound) * i.fatigue_term).clamp(0.0, 100.0)
}

/// The plan's chance (0..=1) for a percent chance under `OpenMW`'s integer roll (module docs).
#[must_use]
pub fn roll_chance(percent: f32) -> f32 {
    (percent.clamp(0.0, 100.0).ceil() / 100.0).min(1.0)
}

/// The caster's numbers the cast rules read, by key (`None` or unknown: 0).
struct Caster<'a> {
    sheet: SheetRead<'a>,
}

impl Caster<'_> {
    fn modified(&self, key: &str) -> f32 {
        self.sheet
            .table()
            .find(key)
            .map_or(0.0, |i| self.sheet.modified(i))
    }

    fn skill(&self, key: &StatKey) -> f32 {
        self.sheet
            .table()
            .idx(key)
            .map_or(0.0, |i| self.sheet.modified(i))
    }

    fn mana(&self) -> f32 {
        self.sheet
            .role(Role::Mana)
            .map_or(0.0, |i| self.sheet.current(i))
    }

    fn fatigue_term(&self, g: &MwGmst) -> f32 {
        self.sheet.role(Role::Stamina).map_or(1.0, |i| {
            fatigue_term(g, self.sheet.current(i), self.sheet.max(i))
        })
    }
}

/// An entry back in record units (areas in feet, rounded: records hold whole feet).
fn record_area(e: &EffectEntry) -> f32 {
    (e.area / FOOT_M).round()
}

/// The classic chance input of a cast query.
fn chance_in(
    fx: &MwEffects,
    effect_gmst: MwEffectGmst,
    gmst: &MwGmst,
    q: &CastQuery<'_>,
) -> CastChanceIn {
    let c = Caster { sheet: q.caster };
    let effects = q
        .entries
        .iter()
        .filter_map(|e| {
            let info = fx.info_by_key(&e.effect)?;
            Some(ChanceEffectIn {
                base_cost: info.base_cost,
                school_skill: c.skill(&info.school),
                duration: e.duration,
                magnitude: e.magnitude,
                area: record_area(e),
                on_target: e.range == Range::Target,
                applied_once: info.has(flag::APPLIED_ONCE),
            })
        })
        .collect();
    CastChanceIn {
        effects,
        spell_cost: q.cost.unwrap_or(0.0),
        willpower: c.modified("willpower"),
        luck: c.modified("luck"),
        sound: c.modified(SOUND_STAT),
        silence: c.modified(SILENCE_STAT),
        fatigue_term: c.fatigue_term(gmst),
        class: q.class,
        always_succeeds: q.always_succeeds,
        effect_cost_mult: effect_gmst.effect_cost_mult,
    }
}

/// `getSpellSchool`: the school skill a cast of `entries` by `caster` uses (the weakest, as the
/// classic chance picks it), for the skill use on a success; `None` for no known effect.
#[must_use]
pub fn spell_school(
    fx: &MwEffects,
    effect_gmst: MwEffectGmst,
    q: &CastQuery<'_>,
) -> Option<StatKey> {
    let i = chance_in(fx, effect_gmst, &MwGmst::vanilla(), q);
    let n = base_success_chance(&i).school_effect?;
    let known: SmallVec<[&EffectEntry; 4]> = q
        .entries
        .iter()
        .filter(|e| fx.info_by_key(&e.effect).is_some())
        .collect();
    fx.info_by_key(&known.get(n)?.effect)
        .map(|info| info.school.clone())
}

/// `startSpellCast`'s magicka check, then the plan.
fn ready(q: &CastQuery<'_>, cost: f32, chance: f32) -> CastPlan {
    if q.ready_at.is_some_and(|t| q.tick < t) {
        return CastPlan::Refused(CastFailReason::Recharging);
    }
    if cost > 0.0 && (Caster { sheet: q.caster }).mana() < cost {
        return CastPlan::Refused(CastFailReason::NoMana);
    }
    CastPlan::Ready {
        cost,
        chance,
        cooldown: q.cooldown,
    }
}

/// Classic rules: `OpenMW`'s cast chance and fizzle (module docs); landing and items are the
/// framework defaults for now.
#[derive(Clone, Debug)]
pub struct OpenMwEffectRules {
    pub fx: Arc<MwEffects>,
    pub effect_gmst: MwEffectGmst,
    pub gmst: MwGmst,
    pub land: StandardEffectRules,
}

impl OpenMwEffectRules {
    #[must_use]
    pub fn new(fx: Arc<MwEffects>, effect_gmst: MwEffectGmst, gmst: MwGmst) -> Self {
        Self {
            fx,
            effect_gmst,
            gmst,
            land: StandardEffectRules::default(),
        }
    }

    /// The input of `mw.effects.cast_chance` for `q`.
    #[must_use]
    pub fn chance_in(&self, q: &CastQuery<'_>) -> CastChanceIn {
        chance_in(&self.fx, self.effect_gmst, &self.gmst, q)
    }
}

impl CastModel for OpenMwEffectRules {
    /// Refused: a cooldown running (none in Morrowind content), magicka below a cost above 0.
    /// Else the cost (`calcSpellCost`, the source's) and the chance of [`success_chance`]: a
    /// silenced caster fizzles with the cost spent, as `OpenMW`.
    fn plan(&self, q: &CastQuery<'_>) -> CastPlan {
        let cost = q.cost.unwrap_or(0.0);
        ready(q, cost, roll_chance(success_chance(&self.chance_in(q))))
    }
}

/// Input of rule `mw.effects.cast` (modern).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModernCastIn {
    /// `calcSpellCost`.
    pub spell_cost: f32,
    pub sound: f32,
    pub silence: f32,
    pub class: SourceClass,
}

/// Output of rule `mw.effects.cast` (modern): the cost, or the caster is silenced.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModernCast {
    Cost(f32),
    Silenced,
}

/// Rule `mw.effects.cast` (modern, effects-adapter.md "Casting rules"): never fizzles; Silence
/// stops spells (refused, nothing spent) and Sound raises a spell's cost by its magnitude in
/// percent (`cost × (1 + Sound / 100)`, owner question). Powers are free of both, as in
/// `getSpellSuccessChance` (a power returns before Silence and Sound).
#[must_use]
pub fn modern_cast(i: &ModernCastIn) -> ModernCast {
    if i.class != SourceClass::Cast {
        return ModernCast::Cost(i.spell_cost);
    }
    if i.silence != 0.0 {
        return ModernCast::Silenced;
    }
    ModernCast::Cost(i.spell_cost * (1.0 + i.sound.max(0.0) / 100.0))
}

/// Modern rules, the game's default: casts never fizzle (chance 1 once the magicka covers the
/// cost), the cost is the content's `calcSpellCost` raised by Sound, Silence refuses spells;
/// landing and items are the framework defaults.
#[derive(Clone, Debug, Default)]
pub struct MwModernEffectRules {
    pub land: StandardEffectRules,
}

impl CastModel for MwModernEffectRules {
    fn plan(&self, q: &CastQuery<'_>) -> CastPlan {
        let c = Caster { sheet: q.caster };
        let input = ModernCastIn {
            spell_cost: q.cost.unwrap_or(0.0),
            sound: c.modified(SOUND_STAT),
            silence: c.modified(SILENCE_STAT),
            class: q.class,
        };
        match modern_cast(&input) {
            ModernCast::Silenced => CastPlan::Refused(CastFailReason::Silenced),
            ModernCast::Cost(cost) => ready(q, cost, 1.0),
        }
    }
}

macro_rules! delegate_land_and_items {
    ($t:ty) => {
        impl LandModel for $t {
            fn magnitude(&self, q: &RollQuery<'_>) -> f32 {
                self.land.magnitude(q)
            }

            fn resist(&self, q: &ResistQuery<'_>) -> f32 {
                self.land.resist(q)
            }

            fn protect(&self, q: &ProtectQuery<'_>) -> Protection {
                self.land.protect(q)
            }
        }

        impl ItemModel for $t {
            fn charge_cost(&self, q: &ItemQuery<'_>) -> f32 {
                self.land.charge_cost(q)
            }

            fn max_charge(&self, q: &ItemQuery<'_>) -> f32 {
                self.land.max_charge(q)
            }
        }
    };
}

delegate_land_and_items!(OpenMwEffectRules);
delegate_land_and_items!(MwModernEffectRules);
