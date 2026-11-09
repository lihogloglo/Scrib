//! The modern default effect rules (effects.md "The rules seam", "Modern defaults"): casting
//! never fizzles (chance 1 when the pool covers the cost; the skill lowers the cost), magnitude
//! uniform in `[min, max]`, resist deterministic `1 - clamp(resist - weakness, -100, 100) / 100`,
//! reflect and spell absorption roll their percentage, cooldown per source in ticks.
//!
//! Each formula is a pure rule `fn(&In) -> Out` over one serde input holding every number it
//! reads (constants as fields, rolls drawn by the caller). Rule ids are named in each doc: they
//! are what a later Luau `rules.wrap` targets (rule-hooks.md; no registry yet, owner 2026-11-08).
//! [`StandardEffectRules`] gathers the numbers from the queries and calls the rules.

use serde::{Deserialize, Serialize};
use sf_stats::{Role, SheetRead, StatIdx, StatKey};
use smallvec::SmallVec;

use crate::active::SourceClass;
use crate::def::{EffectDef, EffectTable};
use crate::op::EffectEntry;
use crate::rules::{
    CastFailReason, CastModel, CastPlan, CastQuery, ItemModel, ItemQuery, LandModel, ProtectQuery, Protection,
    ResistQuery, RollQuery,
};

/// Constants of the standard cost formula (GMST-like; content may tune them).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostConsts {
    /// Cost per unit of `base_cost · magnitude · duration · area term`.
    pub scale: f32,
    /// Metres of area radius that add one more full cost (area term `1 + area / area_unit`);
    /// 0 or less = area is free.
    pub area_unit: f32,
    /// Cost fraction removed per skill point, up to `skill_cap` points.
    pub skill_discount: f32,
    pub skill_cap: f32,
}

impl Default for CostConsts {
    /// `scale` 0.1, `area_unit` 5 m, `skill_discount` 0.005 (half price at skill 100), `skill_cap` 100.
    fn default() -> Self {
        Self { scale: 0.1, area_unit: 5.0, skill_discount: 0.005, skill_cap: 100.0 }
    }
}

/// One entry as the cost formula sees it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostEntryIn {
    /// The effect def's `base_cost`.
    pub base_cost: f32,
    /// `(min, max)`; `(0, 0)` for a `no_magnitude` effect.
    pub magnitude: (f32, f32),
    /// Seconds; 0 for an instant or `no_duration` effect.
    pub duration: f32,
    /// Metres.
    pub area: f32,
}

/// One entry's cost: `base_cost · max(1, (min + max) / 2) · max(1, duration) · (1 + area /
/// area_unit) · scale`. Not a rule of its own: part of `sf.effects.cast` and `sf.effects.item_charge`.
pub fn entry_cost(e: &CostEntryIn, k: &CostConsts) -> f32 {
    let magnitude = f32::midpoint(e.magnitude.0, e.magnitude.1).max(1.0);
    let duration = e.duration.max(1.0);
    let area = if k.area_unit > 0.0 { 1.0 + e.area.max(0.0) / k.area_unit } else { 1.0 };
    e.base_cost * magnitude * duration * area * k.scale
}

/// The source's base cost: the fixed cost, else the sum of its entries' costs.
fn base_cost(fixed: Option<f32>, entries: &[CostEntryIn], k: &CostConsts) -> f32 {
    fixed.unwrap_or_else(|| entries.iter().map(|e| entry_cost(e, k)).sum())
}

/// `cost · (1 - clamp(skill, 0, skill_cap) · skill_discount)`, never below 0.
fn discounted(cost: f32, skill: f32, k: &CostConsts) -> f32 {
    (cost * (1.0 - skill.clamp(0.0, k.skill_cap.max(0.0)) * k.skill_discount)).max(0.0)
}

/// Input of rule `sf.effects.cast`: everything one cast plan reads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CastIn {
    /// The source's fixed cost; `None` = the entries' sum.
    pub fixed_cost: Option<f32>,
    pub entries: SmallVec<[CostEntryIn; 4]>,
    /// Costs nothing (powers: their recharge limits them).
    pub free: bool,
    /// The caster's school skill (0 when none).
    pub skill: f32,
    /// The caster's current `Mana`.
    pub mana: f32,
    pub silenced: bool,
    /// The book's cooldown or recharge has not run out.
    pub recharging: bool,
    /// The source's cooldown in fixed ticks.
    pub cooldown: u32,
    pub k: CostConsts,
}

/// Rule `sf.effects.cast` (modern default). Refused, in this order: `Recharging`, `Silenced`,
/// `NoMana` (`mana < cost`). Else `Ready` with `cost` = the base cost lowered by the skill (0 when
/// `free`), `chance` 1 (never fizzles) and the source's cooldown.
pub fn cast_rule(i: &CastIn) -> CastPlan {
    if i.recharging {
        return CastPlan::Refused(CastFailReason::Recharging);
    }
    if i.silenced {
        return CastPlan::Refused(CastFailReason::Silenced);
    }
    let cost = if i.free { 0.0 } else { discounted(base_cost(i.fixed_cost, &i.entries, &i.k), i.skill, &i.k) };
    if i.mana < cost {
        return CastPlan::Refused(CastFailReason::NoMana);
    }
    CastPlan::Ready { cost, chance: 1.0, cooldown: i.cooldown }
}

/// Input of rule `sf.effects.magnitude`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MagnitudeIn {
    pub min: f32,
    pub max: f32,
    /// Uniform in `[0, 1)`.
    pub roll: f32,
}

/// Rule `sf.effects.magnitude` (modern default): uniform in `[min, max]`, `min + (max - min) · roll`.
pub fn magnitude_rule(i: &MagnitudeIn) -> f32 {
    i.min + (i.max - i.min) * i.roll
}

/// Input of rule `sf.effects.protect`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectIn {
    /// The target's reflect chance, percent.
    pub reflect: f32,
    /// The target's spell absorption chance, percent.
    pub absorb: f32,
    pub can_reflect: bool,
    pub can_absorb: bool,
    /// What the source cost its caster: the absorbed amount.
    pub cost: f32,
    /// Uniform in `[0, 1)`: `[absorb, reflect]`.
    pub rolls: [f32; 2],
}

/// Rule `sf.effects.protect` (modern default): absorb first (`rolls[0] · 100 < absorb`: the
/// source is dropped and the target gains its cost), then reflect (`rolls[1] · 100 < reflect`),
/// else applied. A chance of 0 never triggers, 100 always does.
pub fn protect_rule(i: &ProtectIn) -> Protection {
    if i.can_absorb && i.rolls[0] * 100.0 < i.absorb {
        return Protection::Absorbed { gain: i.cost.max(0.0) };
    }
    if i.can_reflect && i.rolls[1] * 100.0 < i.reflect {
        return Protection::Reflected;
    }
    Protection::Applied
}

/// Input of rule `sf.effects.resist`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResistIn {
    /// Sum of the target's `resisted_by` stats, percent.
    pub resist: f32,
    /// Sum of the target's `weakened_by` stats, percent.
    pub weakness: f32,
}

/// Rule `sf.effects.resist` (modern default, deterministic): the magnitude multiplier
/// `1 - clamp(resist - weakness, -100, 100) / 100`, in `[0, 2]`; 0 = fully resisted.
#[allow(clippy::trivially_copy_pass_by_ref, reason = "every rule takes its input by reference (rule-hooks.md)")]
pub fn resist_rule(i: &ResistIn) -> f32 {
    1.0 - (i.resist - i.weakness).clamp(-100.0, 100.0) / 100.0
}

/// Input of rule `sf.effects.item_charge`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemIn {
    /// The record's fixed charge per use; `None` = the entries' sum.
    pub fixed_cost: Option<f32>,
    /// The record's fixed maximum charge; `None` = computed.
    pub fixed_max: Option<f32>,
    pub entries: SmallVec<[CostEntryIn; 4]>,
    /// The user's skill (0 when none or no user).
    pub skill: f32,
    /// Full-price uses a computed maximum holds.
    pub uses: f32,
    pub k: CostConsts,
}

/// Output of rule `sf.effects.item_charge`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemCharge {
    /// Charge per use or strike: the base cost lowered by the user's skill.
    pub cost: f32,
    /// The fixed maximum, else `base cost · uses` (undiscounted).
    pub max: f32,
}

/// Rule `sf.effects.item_charge` (modern default): charge per use and maximum charge.
pub fn item_charge_rule(i: &ItemIn) -> ItemCharge {
    let base = base_cost(i.fixed_cost, &i.entries, &i.k);
    ItemCharge { cost: discounted(base, i.skill, &i.k), max: i.fixed_max.unwrap_or(base * i.uses.max(0.0)) }
}

/// The modern default rules behind all three seams ([`crate::EffectRules::standard`]). The stat
/// keys name the content's stats the rules read (resolved against each sheet's table; `None` or
/// unknown = 0); resist and weakness come from each def's `resisted_by` / `weakened_by`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandardEffectRules {
    pub cost: CostConsts,
    /// Full-price uses a computed item maximum holds (default 10).
    pub item_uses: f32,
    /// A stat whose value above 0 silences its owner.
    pub silence: Option<StatKey>,
    /// The reflect chance stat, percent.
    pub reflect: Option<StatKey>,
    /// The spell absorption chance stat, percent.
    pub spell_absorb: Option<StatKey>,
}

impl Default for StandardEffectRules {
    fn default() -> Self {
        Self { cost: CostConsts::default(), item_uses: 10.0, silence: None, reflect: None, spell_absorb: None }
    }
}

/// The modified value of `key` on `sheet`; 0 when absent.
fn stat_value(sheet: &SheetRead<'_>, key: Option<&StatKey>) -> f32 {
    key.and_then(|k| sheet.table().idx(k)).map_or(0.0, |i| sheet.modified(i))
}

/// Entries as the cost formula sees them; entries with unknown defs cost nothing.
fn cost_entries(defs: &EffectTable, entries: &[EffectEntry]) -> SmallVec<[CostEntryIn; 4]> {
    entries
        .iter()
        .filter_map(|e| {
            let def = defs.get(&e.effect)?;
            Some(CostEntryIn {
                base_cost: def.base_cost,
                magnitude: if def.flags.no_magnitude { (0.0, 0.0) } else { e.magnitude },
                duration: if def.flags.no_duration { 0.0 } else { e.duration },
                area: e.area,
            })
        })
        .collect()
}

fn skill_value(sheet: Option<&SheetRead<'_>>, skill: Option<StatIdx>) -> f32 {
    match (sheet, skill) {
        (Some(s), Some(i)) => s.modified(i),
        _ => 0.0,
    }
}

impl CastModel for StandardEffectRules {
    fn plan(&self, q: &CastQuery<'_>) -> CastPlan {
        let caster = &q.caster;
        let input = CastIn {
            fixed_cost: q.cost,
            entries: cost_entries(q.defs, q.entries),
            free: q.class == SourceClass::Power,
            skill: skill_value(Some(caster), q.skill),
            mana: caster.role(Role::Mana).map_or(0.0, |i| caster.current(i)),
            silenced: stat_value(caster, self.silence.as_ref()) > 0.0,
            recharging: q.ready_at.is_some_and(|t| q.tick < t),
            cooldown: q.cooldown,
            k: self.cost,
        };
        match cast_rule(&input) {
            CastPlan::Ready { cost, cooldown, .. } if q.always_succeeds => CastPlan::Ready { cost, chance: 1.0, cooldown },
            plan => plan,
        }
    }
}

impl LandModel for StandardEffectRules {
    fn magnitude(&self, q: &RollQuery<'_>) -> f32 {
        let (min, max) = q.entry.magnitude;
        magnitude_rule(&MagnitudeIn { min, max, roll: q.rolls[0] })
    }

    fn resist(&self, q: &ResistQuery<'_>) -> f32 {
        let Some(target) = q.target else { return 1.0 };
        let sum = |keys: &[StatKey]| keys.iter().map(|k| stat_value(&target, Some(k))).sum::<f32>();
        resist_rule(&ResistIn { resist: sum(q.def.resisted_by.as_slice()), weakness: sum(q.def.weakened_by.as_slice()) })
    }

    /// Absorb needs a caster and no `ignore_absorb`; reflect also needs no `ignore_reflect`, no
    /// `from_reflect` (one reflect), at least one harmful entry and no unreflectable one.
    fn protect(&self, q: &ProtectQuery<'_>) -> Protection {
        let defs: SmallVec<[&EffectDef; 4]> = q.entries.iter().filter_map(|e| q.defs.get(&e.effect)).collect();
        let has_caster = q.caster.is_some();
        let can_reflect = has_caster
            && !q.flags.ignore_reflect
            && !q.flags.from_reflect
            && defs.iter().any(|d| d.flags.harmful)
            && !defs.iter().any(|d| d.flags.unreflectable);
        protect_rule(&ProtectIn {
            reflect: stat_value(&q.target, self.reflect.as_ref()),
            absorb: stat_value(&q.target, self.spell_absorb.as_ref()),
            can_reflect,
            can_absorb: has_caster && !q.flags.ignore_absorb,
            cost: q.cost,
            rolls: q.rolls,
        })
    }
}

impl StandardEffectRules {
    fn item_in(&self, q: &ItemQuery<'_>) -> ItemIn {
        ItemIn {
            fixed_cost: q.cost,
            fixed_max: q.capacity,
            entries: cost_entries(q.defs, q.entries),
            skill: skill_value(q.user.as_ref(), q.skill),
            uses: self.item_uses,
            k: self.cost,
        }
    }
}

impl ItemModel for StandardEffectRules {
    fn charge_cost(&self, q: &ItemQuery<'_>) -> f32 {
        item_charge_rule(&self.item_in(q)).cost
    }

    fn max_charge(&self, q: &ItemQuery<'_>) -> f32 {
        item_charge_rule(&self.item_in(q)).max
    }
}
