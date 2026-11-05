//! `MwDerived`: Morrowind's `DerivedModel`: magicka and fatigue maxima, carry capacity,
//! encumbrance, fatigue term (S2), per-tick fatigue regeneration and rest (S3).
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`: `apps/openmw/mwmechanics/
//! creaturestats.cpp` (`recalculateMagicka`, `setAttribute` fatigue, `getFatigueTerm`),
//! `apps/openmw/mwclass/npc.cpp` and `creature.cpp` (`getCapacity`), `apps/openmw/mwworld/
//! class.cpp` (`getNormalizedEncumbrance`), `apps/openmw/mwmechanics/actors.cpp`
//! (`calculateRestoration`, `getRestorationPerHourOfSleep`, `restoreDynamicStats`).
//!
//! Waiting for the effect runtime: Stunted Magicka (no magicka on rest while it lasts),
//! Feather/Burden on encumbrance, werewolf form (encumbrance 0).
//!
//! One deliberate difference: `OpenMW` keeps a record's stored magicka and fatigue (non-autocalc
//! NPCs, creatures) until an attribute changes, then snaps them to the formula. Here the record's
//! difference from the formula is kept as an offset counter (`magicka.offset`, `fatigue.offset`),
//! so the max follows attribute changes by their delta instead of snapping.

use sf_stats::{DerivedModel, Inputs, SheetRead, StatIdx, StatOp};
use smallvec::SmallVec;

use crate::gmst::MwGmst;
use crate::table::{MwIdx, attr};

/// Magicka max before the record offset: (`fPCbaseMagickaMult` or `fNPCbaseMagickaMult` + 0.1 x
/// Fortify Maximum Magicka) x Intelligence (`recalculateMagicka`).
#[must_use]
pub fn magicka_max(g: &MwGmst, is_player: bool, intelligence: f32, fortify_max_magicka: f32) -> f32 {
    let base = if is_player { g.pc_base_magicka_mult } else { g.npc_base_magicka_mult };
    (base + fortify_max_magicka * 0.1) * intelligence
}

/// Fatigue max before the record offset: max(0, Str + Wil + Agi + End) (`setAttribute`).
#[must_use]
pub fn fatigue_max(strength: f32, willpower: f32, agility: f32, endurance: f32) -> f32 {
    (strength + willpower + agility + endurance).max(0.0)
}

/// `fFatigueBase` - `fFatigueMult` x (1 - current / max); 1 for the ratio when the max floors
/// to 0 (`getFatigueTerm`).
#[must_use]
pub fn fatigue_term(g: &MwGmst, current: f32, max: f32) -> f32 {
    let normalised = if max.floor() == 0.0 { 1.0 } else { (current / max).max(0.0) };
    g.fatigue_base - g.fatigue_mult * (1.0 - normalised)
}

/// Carry capacity: Strength x `fEncumbranceStrMult` (NPCs), x 5 (creatures).
#[must_use]
pub fn capacity(g: &MwGmst, creature: bool, strength: f32) -> f32 {
    strength * if creature { 5.0 } else { g.encumbrance_str_mult }
}

/// Morrowind's derived maxima and stats (the `OpenMW` formulas, the first `DerivedModel`).
#[derive(Clone, Debug)]
pub struct MwDerived {
    pub gmst: MwGmst,
    pub idx: MwIdx,
}

impl MwDerived {
    fn attribute(&self, s: &SheetRead<'_>, a: usize) -> f32 {
        s.modified(self.idx.attributes[a])
    }
}

impl DerivedModel for MwDerived {
    fn pool_base(&self, s: &SheetRead<'_>, pool: StatIdx, inputs: &Inputs) -> Option<f32> {
        let i = &self.idx;
        if pool == i.magicka {
            let int = self.attribute(s, attr::INTELLIGENCE);
            let fortify = s.modified(i.fortify_max_magicka);
            Some(magicka_max(&self.gmst, inputs.is_player, int, fortify) + s.base(i.magicka_offset))
        } else if pool == i.fatigue {
            let max = fatigue_max(
                self.attribute(s, attr::STRENGTH),
                self.attribute(s, attr::WILLPOWER),
                self.attribute(s, attr::AGILITY),
                self.attribute(s, attr::ENDURANCE),
            );
            Some(max + s.base(i.fatigue_offset))
        } else {
            // Health is stored: the record or chargen value, + End x fLevelUpHealthEndMult per level.
            None
        }
    }

    fn derived(&self, s: &SheetRead<'_>, stat: StatIdx, inputs: &Inputs) -> f32 {
        let i = &self.idx;
        if stat == i.capacity {
            capacity(&self.gmst, s.base(i.creature) > 0.0, self.attribute(s, attr::STRENGTH))
        } else if stat == i.encumbrance {
            self.encumbrance(inputs)
        } else if stat == i.fatigue_term {
            fatigue_term(&self.gmst, s.current(i.fatigue), s.max(i.fatigue))
        } else {
            0.0
        }
    }

    /// `calculateRestoration`: fatigue only, `duration` x (`fFatigueReturnBase` +
    /// `fFatigueReturnMult` x End) while it is below its max; nothing for the dead (health < 1)
    /// or for a `Combatant` (combat's regen owns its stamina: stats.md "Consumers").
    fn regen(&self, s: &SheetRead<'_>, dt: f32, inputs: &Inputs, out: &mut SmallVec<[StatOp; 3]>) {
        let i = &self.idx;
        if inputs.combatant || is_dead(s, i) || s.current(i.fatigue) >= s.max(i.fatigue) {
            return;
        }
        let per_second = fatigue_regen_per_second(&self.gmst, self.attribute(s, attr::ENDURANCE));
        out.push(adjust(i.fatigue, dt * per_second));
    }

    /// `restoreDynamicStats` for `hours` (`OpenMW` calls it one hour at a time; the result is
    /// the same, every term being constant over the rest): sleeping restores health and magicka,
    /// both rest and wait restore fatigue. Applies to combatants too: rest is a time skip.
    fn rest(&self, s: &SheetRead<'_>, hours: f32, sleep: bool, inputs: &Inputs, out: &mut SmallVec<[StatOp; 4]>) {
        let i = &self.idx;
        if is_dead(s, i) || hours <= 0.0 {
            return;
        }
        let endurance = self.attribute(s, attr::ENDURANCE);
        if sleep {
            let (health, magicka) = sleep_restoration_per_hour(&self.gmst, endurance, self.attribute(s, attr::INTELLIGENCE));
            out.push(adjust(i.health, health * hours));
            // Stunted Magicka (no magicka while it lasts) waits for the effect runtime.
            out.push(adjust(i.magicka, magicka * hours));
        }
        // A fortified fatigue above its max isn't restored.
        if s.current(i.fatigue) >= s.max(i.fatigue) {
            return;
        }
        let creature = s.base(i.creature) > 0.0;
        let cap = capacity(&self.gmst, creature, self.attribute(s, attr::STRENGTH));
        let enc = normalized_encumbrance(self.encumbrance(inputs), cap);
        out.push(adjust(i.fatigue, rest_fatigue_per_hour(&self.gmst, endurance, enc) * hours));
    }
}

impl MwDerived {
    /// Carried weight (Feather/Burden and werewolf form join with the effect runtime).
    #[allow(clippy::unused_self)]
    fn encumbrance(&self, inputs: &Inputs) -> f32 {
        inputs.carried
    }
}

/// `CreatureStats::isDead`: health below 1.
fn is_dead(s: &SheetRead<'_>, i: &MwIdx) -> bool {
    s.current(i.health) < 1.0
}

/// A pool change with `setCurrent`'s default flags (stops at 0 and at the max).
fn adjust(pool: StatIdx, delta: f32) -> StatOp {
    StatOp::PoolAdjust { pool, delta, below_zero: false, above_max: false }
}

/// Fatigue regained per second awake: `fFatigueReturnBase` + `fFatigueReturnMult` x End
/// (`calculateRestoration`).
#[must_use]
pub fn fatigue_regen_per_second(g: &MwGmst, endurance: f32) -> f32 {
    g.fatigue_return_base + g.fatigue_return_mult * endurance
}

/// Health and magicka per hour of sleep: 0.1 x End, `fRestMagicMult` x Int
/// (`getRestorationPerHourOfSleep`).
#[must_use]
pub fn sleep_restoration_per_hour(g: &MwGmst, endurance: f32, intelligence: f32) -> (f32, f32) {
    (0.1 * endurance, g.rest_magic_mult * intelligence)
}

/// Encumbrance / capacity: 0 when carrying nothing, just over 1 for a zero capacity
/// (`getNormalizedEncumbrance`, both `OpenMW`'s deliberate deviations).
#[must_use]
pub fn normalized_encumbrance(encumbrance: f32, capacity: f32) -> f32 {
    if encumbrance == 0.0 {
        0.0
    } else if capacity == 0.0 {
        1.0 + 1e-6
    } else {
        encumbrance / capacity
    }
}

/// Fatigue per hour of rest or wait: 3600 x (`fFatigueReturnBase` + `fFatigueReturnMult` x (1 -
/// min(1, normalised encumbrance))) x `fEndFatigueMult` x End (`restoreDynamicStats`).
#[must_use]
pub fn rest_fatigue_per_hour(g: &MwGmst, endurance: f32, normalized_encumbrance: f32) -> f32 {
    let enc = normalized_encumbrance.min(1.0);
    let x = (g.fatigue_return_base + g.fatigue_return_mult * (1.0 - enc)) * (g.end_fatigue_mult * endurance);
    3600.0 * x
}
