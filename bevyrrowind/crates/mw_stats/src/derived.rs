//! `MwDerived`: the derivation half of Morrowind's `DerivedModel` (magicka and fatigue maxima,
//! carry capacity, encumbrance, fatigue term), needed to settle any initialised sheet. Regen and
//! rest are stats S3 (they push nothing yet).
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`: `apps/openmw/mwmechanics/
//! creaturestats.cpp` (`recalculateMagicka`, `setAttribute` fatigue, `getFatigueTerm`),
//! `apps/openmw/mwclass/npc.cpp` and `creature.cpp` (`getCapacity`).
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
            // Feather/Burden and werewolf form (0) join with the effect runtime.
            inputs.carried
        } else if stat == i.fatigue_term {
            fatigue_term(&self.gmst, s.current(i.fatigue), s.max(i.fatigue))
        } else {
            0.0
        }
    }

    fn regen(&self, _s: &SheetRead<'_>, _dt: f32, _inputs: &Inputs, _out: &mut SmallVec<[StatOp; 3]>) {}

    fn rest(&self, _s: &SheetRead<'_>, _hours: f32, _sleep: bool, _inputs: &Inputs, _out: &mut SmallVec<[StatOp; 4]>) {}
}
