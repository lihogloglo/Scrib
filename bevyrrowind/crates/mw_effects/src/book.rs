//! SPEL records as the framework's [`SourceDef`]s (the source table books and casts name) and
//! the spells a character starts with (effects-adapter.md "Records → `SourceDef`", "Saves").
//!
//! Ported from `OpenMW` `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! `apps/openmw/mwmechanics/spells.cpp` (`addSpell` 85-89: a known spell is not added twice;
//! `setSpells` 307-315 and `addAllToInstance` 317-326: the lists in order, unknown ids skipped
//! with a warning), `apps/openmw/mwclass/npc.cpp` (`getCreatureStats` 311-374: an NPC knows its
//! race's powers, then its record's spells), `apps/openmw/mwclass/creature.cpp`
//! (`getCreatureStats` 154-156: a creature knows its record's spells) and
//! `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp` (`buildPlayer` 111-219: a new player knows
//! its race's powers, then its birthsign's spells; never the player record's own list).
//! Abilities, diseases and curses start their instance when learned (`Learn`, `sf_effects`):
//! `OpenMW` `ActiveSpells::update` adds them from the spell list the same way.
//! Not yet (adapter A3b): the autocalc spells of NPCs with the autocalc flag
//! (`autocalcspell.cpp` `autoCalcNpcSpells`, before the race powers) and the new player's
//! `F_PCStart` spells (`autoCalcPlayerSpells`, after the birthsign).

use std::collections::BTreeSet;

use sf_effects::{GameMinutes, SourceClass, SourceDef, SourceTable, Tag};
use sf_records::RecordId;
use smallvec::SmallVec;
use tes3::esp::Spell;

use crate::mgef::MwEffects;
use crate::source::{Affliction, MapNote, MwEffectGmst, MwSource, Recharge, map_spell};

impl Affliction {
    /// The cure group (`CureFilter::SourceTag`): `disease`, `blight` or `curse`.
    #[must_use]
    pub fn tag(self) -> Tag {
        Tag::new(match self {
            Self::Disease => "disease",
            Self::Blight => "blight",
            Self::Curse => "curse",
        })
    }
}

impl Recharge {
    /// The span on the world clock.
    #[must_use]
    pub fn span(self) -> GameMinutes {
        match self {
            Self::GameDays(days) => GameMinutes::days(days),
        }
    }
}

impl MwSource {
    /// The framework's source: the affliction kind as its cure tag, the recharge in game
    /// minutes, and for spells and powers the school of the first effect as the skill (the cast
    /// model of A5 picks the caster's weakest school per cast, `spellutil.cpp`
    /// `calcSpellBaseSuccessChance`). The cost stays the content's `calcSpellCost`.
    #[must_use]
    pub fn source_def(&self, fx: &MwEffects) -> SourceDef {
        let skill = matches!(self.class, SourceClass::Cast | SourceClass::Power)
            .then(|| {
                self.effects
                    .first()
                    .and_then(|e| fx.info_by_key(&e.effect))
                    .map(|i| i.school.clone())
            })
            .flatten();
        SourceDef {
            id: self.id.clone(),
            class: self.class,
            effects: self.effects.clone(),
            cost: self.cost,
            flags: sf_effects::SourceFlags {
                always_succeeds: self.flags.always_succeeds,
                auto_cost: self.flags.auto_cost,
                starts_known: self.flags.starts_known,
            },
            tags: self
                .affliction
                .map(Affliction::tag)
                .into_iter()
                .collect::<SmallVec<_>>(),
            skill,
            cooldown: 0,
            recharge: self.recharge.map(Recharge::span),
        }
    }
}

/// The source table of `spells` (every SPEL record, in record order; a repeated id keeps its
/// first record), and what the mapping changed per record.
#[must_use]
pub fn source_table<'a>(
    fx: &MwEffects,
    g: &MwEffectGmst,
    spells: impl IntoIterator<Item = &'a Spell>,
) -> (SourceTable, Vec<(RecordId, MapNote)>) {
    let mut seen = BTreeSet::new();
    let mut defs = Vec::new();
    let mut notes = Vec::new();
    for spell in spells {
        let (source, n) = map_spell(fx, g, spell);
        if !seen.insert(source.id.clone()) {
            continue;
        }
        notes.extend(n.into_iter().map(|n| (source.id.clone(), n)));
        defs.push(source.source_def(fx));
    }
    let table =
        SourceTable::new(defs).unwrap_or_else(|_| unreachable!("ids are deduplicated above"));
    (table, notes)
}

/// The spells a character starts with: the ids of `lists` in order, each once, skipping ids
/// `exists` does not know (returned in `missing`). Lists by character (module docs): an NPC
/// `[race powers, record spells]`, a creature `[record spells]`, a new player
/// `[race powers, birthsign spells]`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StartSpells {
    /// What the book learns, in order.
    pub known: Vec<RecordId>,
    /// Ids without a SPEL record (`OpenMW` logs "ignoring nonexistent spell").
    pub missing: Vec<RecordId>,
}

/// [`StartSpells`] of `lists`.
#[must_use]
pub fn start_spells<'a>(
    lists: impl IntoIterator<Item = &'a [String]>,
    exists: impl Fn(&RecordId) -> bool,
) -> StartSpells {
    let mut out = StartSpells::default();
    for id in lists.into_iter().flatten() {
        let id = RecordId::new(id);
        if !exists(&id) {
            if !out.missing.contains(&id) {
                out.missing.push(id);
            }
        } else if !out.known.contains(&id) {
            out.known.push(id);
        }
    }
    out
}
