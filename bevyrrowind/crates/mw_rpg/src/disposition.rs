//! Morrowind's derived disposition (ROADMAP 2.7, stats-adapter.md "Factions, reputation, bounty,
//! disposition", open question 2: the owner default ports `OpenMW`'s formula as the disposition
//! model). A pure formula, [`derived_disposition`] over a [`DispositionInput`] holding every
//! number it reads (rule-hooks.md "Declaring a rule"; future rule id `mw.disposition.derived`),
//! plus [`disposition_input`], which fills the input from the host's [`MwSheets`], the speaker's
//! [`ActorFacts`] and the FACT records.
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! - `MechanicsManager::getDerivedDisposition` (`apps/openmw/mwmechanics/mechanicsmanagerimp.cpp`
//!   475-561): `x = base + crime modifier` (as `float`), `+ fDispRaceMod` (same race as the
//!   player), `+ fDispPersonalityMult x (Personality - fDispPersonalityBase)` (the player's
//!   modified Personality), `+ (fDispFactionRankMult x rank + fDispFactionRankBase) x
//!   fDispFactionMod x reaction`, `- fDispCrimeMod x bounty`, `+ fDispDiseaseMod` (common or
//!   blight disease), `+ fDispWeaponDrawn` (the player's weapon is drawn), `+` the speaker's
//!   Charm magnitude; `static_cast<int>` (truncation), clamped to 0..=100 unless unclamped.
//!   All arithmetic is `f32` in `OpenMW`'s order, so the result matches to the bit.
//! - The faction term's rank and reaction ([`faction_standing`], same function): the player is a
//!   member of the speaker's faction: the faction's reaction to itself and the player's rank
//!   there (0 and 0 when expelled); otherwise the lowest reaction of the speaker's faction to
//!   any faction the player is a non-expelled member of, with the player's rank in it (the
//!   first such faction in id order wins a tie, and `OpenMW`'s quirk is kept: when the first
//!   faction in id order is expelled the start value stays 0, so only negative reactions are
//!   taken); a speaker without a faction: 0 and 0.
//! - `DialogueManager::getFactionReaction` (`apps/openmw/mwdialogue/dialoguemanagerimp.cpp`
//!   724-743): the first FACT reaction entry naming the other faction, 0 without one
//!   ([`faction_reaction`]). `ModFactionReaction` overrides (`mChangedFactionReaction`) do not
//!   exist here yet: [`faction_standing`] takes the lookup as a closure so they can be layered.
//! - `NpcStats::getBaseDisposition` is the NPC's record disposition with the permanent changes
//!   applied; here that is `base + modifier` (the record's value and the `ModDisposition` /
//!   `SetDisposition` sum, as `mw_rpg_bevy`'s `DispositionModel` receives them).
//!
//! Not ours yet, so caller-supplied ([`DispositionExtras`]): the crime disposition modifier
//! (`NpcStats::getCrimeDispositionModifier`, written by the crime system), disease and Charm (the
//! effect runtime) and the drawn weapon (combat state). `OpenMW` reads the speaker as an NPC
//! (`ptr.get<ESM::NPC>()`): [`disposition_input`] is `None` for a creature, which keeps the
//! content-only disposition.

use serde::{Deserialize, Serialize};
use sf_records::{RecordDb, RecordId};
use sf_rpg::PlayerId;
use tes3::esp::Faction;

use mw_stats::MwGmst;

use crate::{ActorFacts, MwSheets};

/// The ESM attribute index of Personality.
pub(crate) const PERSONALITY: usize = 6;

/// Every number `getDerivedDisposition` reads.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DispositionInput {
    /// The NPC record's disposition.
    pub base: i32,
    /// The sum of permanent changes (`ModDisposition` / `SetDisposition`, persuasion at goodbye).
    pub modifier: i32,
    /// `NpcStats::getCrimeDispositionModifier`.
    pub crime_modifier: i32,
    /// The speaker's race is the player's.
    pub same_race: bool,
    /// The player's modified Personality.
    pub personality: f32,
    /// The player's rank in the faction the term reads ([`faction_standing`]).
    pub faction_rank: i32,
    /// That faction's reaction ([`faction_standing`]).
    pub faction_reaction: i32,
    /// The player's bounty (`getBounty`, an integer).
    pub bounty: i32,
    /// The player has a common or blight disease.
    pub diseased: bool,
    /// The player's weapon is drawn (`DrawState::Weapon`; a readied spell does not count).
    pub weapon_drawn: bool,
    /// The magnitude of Charm on the speaker.
    pub charm: f32,
}

/// The caller-supplied inputs [`disposition_input`] can't read from sheets or records.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DispositionExtras {
    /// The NPC record's disposition.
    pub base: i32,
    /// The permanent-change sum (`DispositionModel::disposition`'s `modifier`).
    pub modifier: i32,
    pub crime_modifier: i32,
    pub diseased: bool,
    pub weapon_drawn: bool,
    pub charm: f32,
}

/// `getDerivedDisposition(ptr, false)`: the unclamped value.
#[allow(clippy::cast_precision_loss, reason = "small integers, cast as OpenMW casts them")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's static_cast<int> truncates toward zero, as `as` does")]
pub fn derived_disposition_unclamped(i: &DispositionInput, g: &MwGmst) -> i32 {
    let mut x = (i.base + i.modifier + i.crime_modifier) as f32;
    if i.same_race {
        x += g.disp_race_mod;
    }
    x += g.disp_personality_mult * (i.personality - g.disp_personality_base);
    let reaction = i.faction_reaction as f32;
    x += (g.disp_faction_rank_mult * i.faction_rank as f32 + g.disp_faction_rank_base) * g.disp_faction_mod * reaction;
    x -= g.disp_crime_mod * i.bounty as f32;
    if i.diseased {
        x += g.disp_disease_mod;
    }
    if i.weapon_drawn {
        x += g.disp_weapon_drawn;
    }
    x += i.charm;
    x as i32
}

/// `getDerivedDisposition(ptr)`: the value the dialogue filters and persuasion read, 0..=100.
pub fn derived_disposition(i: &DispositionInput, g: &MwGmst) -> i32 {
    derived_disposition_unclamped(i, g).clamp(0, 100)
}

/// One faction the player has a rank in, in `OpenMW`'s map order (ids ascending).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerFaction<'a> {
    pub id: &'a RecordId,
    pub rank: i32,
    pub expelled: bool,
}

/// The faction term's `(rank, reaction)` (see the module docs). `player_factions` are the
/// factions the player is a member of (has a rank in), ids ascending; `reaction(a, b)` is
/// faction `a`'s reaction towards `b`.
pub fn faction_standing(
    npc_faction: Option<&RecordId>,
    player_factions: &[PlayerFaction<'_>],
    reaction: impl Fn(&RecordId, &RecordId) -> i32,
) -> (i32, i32) {
    let Some(npc_faction) = npc_faction else { return (0, 0) };
    if let Some(own) = player_factions.iter().find(|f| f.id == npc_faction) {
        if own.expelled {
            return (0, 0);
        }
        // The faction's reaction towards itself ("yes, that exists").
        return (own.rank, reaction(npc_faction, npc_faction));
    }
    let (mut rank, mut best) = (0, 0);
    for (n, f) in player_factions.iter().enumerate() {
        if f.expelled {
            continue;
        }
        let r = reaction(npc_faction, f.id);
        if n == 0 || r < best {
            best = r;
            rank = f.rank;
        }
    }
    (rank, best)
}

/// FACT `from`'s reaction towards `to` (`getFactionReaction` without script overrides); 0 when
/// `from` has no record or no entry for `to`.
pub fn faction_reaction(records: &RecordDb, from: &RecordId, to: &RecordId) -> i32 {
    records
        .get::<Faction>(from)
        .and_then(|f| f.reactions.iter().find(|r| RecordId::new(&r.faction) == *to))
        .map_or(0, |r| r.reaction)
}

/// The input for `speaker`'s disposition towards `player`, from the player's sheet and the FACT
/// records. `player_race`: the player's race id (not on the sheet). `None`: no stats loaded, no
/// player sheet, or the speaker is a creature (callers keep the content-only disposition).
#[allow(clippy::cast_possible_truncation, reason = "bounty and ranks are whole numbers stored as f32 on the sheet")]
pub fn disposition_input(
    sheets: &dyn MwSheets,
    records: &RecordDb,
    player: PlayerId,
    player_race: Option<&RecordId>,
    speaker: &ActorFacts,
    extras: DispositionExtras,
) -> Option<DispositionInput> {
    if !speaker.npc {
        return None;
    }
    let t = sheets.table()?;
    let pc = sheets.player(player)?.view(&t.table);
    let factions: Vec<PlayerFaction<'_>> = pc
        .groups()
        .filter_map(|(id, m)| m.rank.map(|rank| PlayerFaction { id, rank, expelled: m.expelled }))
        .collect();
    let (faction_rank, faction_reaction) =
        faction_standing(speaker.faction.as_ref(), &factions, |a, b| faction_reaction(records, a, b));
    Some(DispositionInput {
        base: extras.base,
        modifier: extras.modifier,
        crime_modifier: extras.crime_modifier,
        same_race: speaker.race.as_ref() == player_race,
        personality: pc.modified(t.idx.attributes[PERSONALITY]),
        faction_rank,
        faction_reaction,
        bounty: pc.modified(t.idx.bounty) as i32,
        diseased: extras.diseased,
        weapon_drawn: extras.weapon_drawn,
        charm: extras.charm,
    })
}
