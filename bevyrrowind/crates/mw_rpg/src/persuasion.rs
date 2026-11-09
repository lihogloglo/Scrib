//! Morrowind's persuasion (ROADMAP 2.7): the outcome of Admire, Intimidate, Taunt and the three
//! bribes. A pure formula, [`persuasion_change`] over a [`PersuasionInput`] holding every number
//! it reads (rule-hooks.md "Declaring a rule"; future rule id `mw.persuasion.change`), plus
//! [`persuasion_input`], which fills both sides from the host's [`MwSheets`].
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`,
//! `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp`:
//! - `getPersuasionRatings` (47-75, [`persuasion_ratings`]): Personality / `fPersonalityMod`,
//!   Luck / `fLuckMod`, reputation x `fReputationMod`, level x `fLevelMod`, the fatigue term
//!   (`CreatureStats::getFatigueTerm`, [`mw_stats::fatigue_term`]); the player's level rating
//!   adds the level term to the speechcraft rating, the NPC's sums its own terms, and only the
//!   NPC's mercantile rating counts reputation.
//! - `getPersuasionDispositionChange` (597-743, [`persuasion_change`]): `d = 1 - 0.02 x
//!   |disposition - 50|`, a target per kind (speechcraft ratings for Admire and Taunt, level
//!   ratings for Intimidate, mercantile ratings + `fBribe<N>Mod` for bribes) raised to
//!   `iPerMinChance`, success when the roll is at most the target, then the temporary change
//!   (`x`, `fPerDieRollMult`, `fPerTempMult`, at least `iPerMinChange` on success) kept inside
//!   0..=100 of the current disposition, and the permanent change from it. Intimidate and Taunt
//!   successes nudge the NPC's base Flee and Fight AI settings ([`AiNudge`]). `OpenMW`'s
//!   deliberate deviation is kept: a marginal Intimidate win (`|c| < iPerMinChange`) raises the
//!   temporary disposition by `iPerMinChange` (Morrowind gave `x = 0, y = -iPerMinChange`).
//!   All arithmetic is `f32` in `OpenMW`'s order and `int(...)` truncates toward zero, as `as`.
//!
//! **Randomness.** The only Rng call, `Misc::Rng::roll0to99(prng)`, is [`PersuasionInput::roll`]
//! (0..=99): the caller draws it from the world's seeded stream, so this stays pure.
//!
//! The dialogue session around it (`apps/openmw/mwdialogue/dialoguemanagerimp.cpp`, the wiring
//! slice): `DialogueManager::persuade` (533-581) adds `temp_change` to the NPC's current
//! disposition at once, accumulates `perm_change` (corrected by [`session_perm_change`]),
//! applies a used skill (Speechcraft success or fail), moves [`PersuasionOutcome::gold`] from the
//! player to the NPC on a successful bribe and answers with the [`PersuasionKind::topic`] DIAL
//! ("Admire Success", ...); `goodbyeSelected` (442-462) then makes the original disposition plus
//! the accumulated permanent change the NPC's new base, clamped so the derived value stays in
//! 0..=100.

use serde::{Deserialize, Serialize};
use sf_rpg::{ActorKey, PlayerId};
use sf_stats::{SheetRead, StatOp};

use mw_stats::{MwGmst, MwTable, attr, fatigue_term};

use crate::MwSheets;
use crate::disposition::PERSONALITY;

/// The ESM skill index of Mercantile.
const MERCANTILE: usize = 24;
/// The ESM skill index of Speechcraft.
const SPEECHCRAFT: usize = 25;

/// `MechanicsManager::PersuasionType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersuasionKind {
    Admire,
    Intimidate,
    Taunt,
    Bribe10,
    Bribe100,
    Bribe1000,
}

impl PersuasionKind {
    /// Gold a successful attempt moves from the player to the NPC (`persuade`).
    pub fn gold(self) -> i32 {
        match self {
            Self::Bribe10 => 10,
            Self::Bribe100 => 100,
            Self::Bribe1000 => 1000,
            Self::Admire | Self::Intimidate | Self::Taunt => 0,
        }
    }

    /// The Persuasion DIAL the NPC answers with (`persuade`'s `executeTopic`).
    pub fn topic(self, success: bool) -> &'static str {
        match (self, success) {
            (Self::Admire, true) => "Admire Success",
            (Self::Admire, false) => "Admire Fail",
            (Self::Intimidate, true) => "Intimidate Success",
            (Self::Intimidate, false) => "Intimidate Fail",
            (Self::Taunt, true) => "Taunt Success",
            (Self::Taunt, false) => "Taunt Fail",
            (Self::Bribe10 | Self::Bribe100 | Self::Bribe1000, true) => "Bribe Success",
            (Self::Bribe10 | Self::Bribe100 | Self::Bribe1000, false) => "Bribe Fail",
        }
    }
}

/// The stats `getPersuasionRatings` reads from one side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PersuaderStats {
    /// Modified Personality.
    pub personality: f32,
    /// Modified Luck.
    pub luck: f32,
    /// Modified Speechcraft.
    pub speechcraft: f32,
    /// Modified Mercantile.
    pub mercantile: f32,
    /// `getReputation` (an integer).
    pub reputation: i32,
    /// `getLevel` (an integer).
    pub level: i32,
    /// Current fatigue.
    pub fatigue_current: f32,
    /// Maximum (modified) fatigue.
    pub fatigue_max: f32,
}

/// `getPersuasionRatings`' `rating1`, `rating2`, `rating3`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PersuasionRatings {
    /// `rating1` (Admire, Taunt).
    pub speech: f32,
    /// `rating2` (Intimidate).
    pub level: f32,
    /// `rating3` (bribes).
    pub mercantile: f32,
}

/// The NPC's base AI settings an Intimidate or Taunt success changes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiNudge {
    pub flee: i32,
    pub fight: i32,
}

/// Every number `getPersuasionDispositionChange` reads.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersuasionInput {
    pub kind: PersuasionKind,
    pub player: PersuaderStats,
    pub npc: PersuaderStats,
    /// The NPC's current derived disposition, clamped (`getDerivedDisposition(npc)`).
    pub disposition: i32,
    /// The NPC's base Flee and Fight (`getAiSetting(..).getBase()`).
    pub npc_ai: AiNudge,
    /// `Misc::Rng::roll0to99`, drawn by the caller (0..=99).
    pub roll: u8,
}

/// `getPersuasionDispositionChange`'s results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersuasionOutcome {
    pub success: bool,
    /// Added to the NPC's disposition now (`tempChange`).
    pub temp_change: i32,
    /// Accumulated for goodbye (`permChange`).
    pub perm_change: i32,
    /// The NPC's new base Flee and Fight (0..=100), on an Intimidate or Taunt success.
    pub ai: Option<AiNudge>,
    /// Gold the player pays (a successful bribe).
    pub gold: i32,
}

/// `getPersuasionRatings(stats, ..., player)`.
#[allow(clippy::cast_precision_loss, reason = "reputation and level are small integers, as OpenMW multiplies them")]
pub fn persuasion_ratings(s: &PersuaderStats, player: bool, g: &MwGmst) -> PersuasionRatings {
    let pers = s.personality / g.personality_mod;
    let luck = s.luck / g.luck_mod;
    let rep = s.reputation as f32 * g.reputation_mod;
    let fatigue = fatigue_term(g, s.fatigue_current, s.fatigue_max);
    let level = s.level as f32 * g.level_mod;
    let speech = (rep + luck + pers + s.speechcraft) * fatigue;
    if player {
        PersuasionRatings { speech, level: speech + level, mercantile: (s.mercantile + luck + pers) * fatigue }
    } else {
        PersuasionRatings {
            speech,
            level: (level + rep + luck + pers + s.speechcraft) * fatigue,
            mercantile: (s.mercantile + rep + luck + pers) * fatigue,
        }
    }
}

/// One kind's branch: success, `x`, `y`, the AI nudge.
struct Branch {
    success: bool,
    x: f32,
    y: f32,
    ai: Option<AiNudge>,
}

/// The GMSTs the branches read, as `OpenMW` names them.
struct Per {
    min_chance: f32,
    min_change: f32,
    die_roll_mult: f32,
    temp_mult: f32,
}

/// `getPersuasionDispositionChange` (see the module docs).
#[allow(clippy::cast_precision_loss, reason = "the temporary change is a small integer, as OpenMW divides it")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's int(...) truncates toward zero, as `as` does")]
pub fn persuasion_change(i: &PersuasionInput, g: &MwGmst) -> PersuasionOutcome {
    let npc = persuasion_ratings(&i.npc, false, g);
    let pc = persuasion_ratings(&i.player, true, g);
    let d = 1.0 - 0.02 * (i.disposition - 50).abs() as f32;
    let speech_target = d * (pc.speech - npc.speech + 50.0);
    let level_target = d * (pc.level - npc.level + 50.0);
    let bribe_mod = match i.kind {
        PersuasionKind::Bribe10 => g.bribe10_mod,
        PersuasionKind::Bribe100 => g.bribe100_mod,
        _ => g.bribe1000_mod,
    };
    let bribe_target = d * (pc.mercantile - npc.mercantile + 50.0) + bribe_mod;
    let per = Per {
        min_chance: g.per_min_chance,
        min_change: g.per_min_change,
        die_roll_mult: g.per_die_roll_mult,
        temp_mult: g.per_temp_mult,
    };
    let roll = f32::from(i.roll);
    let branch = match i.kind {
        PersuasionKind::Admire => raise(&per, speech_target, roll),
        PersuasionKind::Intimidate => intimidate(&per, level_target, roll, i.npc_ai),
        PersuasionKind::Taunt => taunt(&per, speech_target, roll, i.npc_ai),
        PersuasionKind::Bribe10 | PersuasionKind::Bribe100 | PersuasionKind::Bribe1000 => {
            bribe(&per, bribe_target, roll)
        }
    };
    let keep_in_range = |temp: i32| {
        if i.disposition + temp > 100 {
            100 - i.disposition
        } else if i.disposition + temp < 0 {
            -i.disposition
        } else {
            temp
        }
    };
    let (temp_change, perm_change) = if i.kind == PersuasionKind::Intimidate {
        let temp = keep_in_range(branch.x as i32);
        (temp, if branch.success { -((temp as f32 / per.temp_mult) as i32) } else { branch.y as i32 })
    } else {
        let temp = keep_in_range((branch.x * per.temp_mult) as i32);
        (temp, (temp as f32 / per.temp_mult) as i32)
    };
    PersuasionOutcome {
        success: branch.success,
        temp_change,
        perm_change,
        ai: branch.ai,
        gold: if branch.success { i.kind.gold() } else { 0 },
    }
}

/// `PT_Admire`.
fn raise(per: &Per, target: f32, roll: f32) -> Branch {
    let target = per.min_chance.max(target);
    let success = roll <= target;
    let change = (per.die_roll_mult * (target - roll)).floor();
    Branch { success, x: if success { per.min_change.max(change) } else { change }, y: 0.0, ai: None }
}

/// `PT_Bribe*` (`target` includes `fBribe<N>Mod`).
fn bribe(per: &Per, target: f32, roll: f32) -> Branch {
    let target = per.min_chance.max(target);
    let success = roll <= target;
    let change = ((target - roll) * per.die_roll_mult).floor();
    Branch { success, x: if success { per.min_change.max(change) } else { change }, y: 0.0, ai: None }
}

/// `PT_Intimidate`, with `OpenMW`'s marginal-win deviation.
#[allow(clippy::float_cmp, reason = "OpenMW compares the integer roll with the float target exactly")]
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's int(...) truncates toward zero, as `as` does")]
fn intimidate(per: &Per, target: f32, roll: f32, ai: AiNudge) -> Branch {
    let target = per.min_chance.max(target);
    let success = roll <= target;
    let margin = if roll == target { 1.0 } else { (target - roll).floor() };
    let nudge = success.then(|| {
        let by = (margin * per.die_roll_mult * per.temp_mult).floor();
        AiNudge {
            flee: (ai.flee + per.min_change.max(by) as i32).clamp(0, 100),
            fight: (ai.fight + (-per.min_change).min(-by) as i32).clamp(0, 100),
        }
    });
    let change = -((margin * per.die_roll_mult).floor().abs());
    let (x, y) = if success && change.abs() < per.min_change {
        // Deviating from Morrowind (OpenMW, MCP): a marginal win raises the disposition
        // (Morrowind: x = 0, y = -iPerMinChange). `y` is unused on success.
        (per.min_change, per.min_change)
    } else if success {
        (-((change * per.temp_mult).floor()), change)
    } else {
        ((change * per.temp_mult).floor(), change)
    };
    Branch { success, x, y, ai: nudge }
}

/// `PT_Taunt`.
#[allow(clippy::cast_possible_truncation, reason = "OpenMW's int(...) truncates toward zero, as `as` does")]
fn taunt(per: &Per, target: f32, roll: f32, ai: AiNudge) -> Branch {
    let target = per.min_chance.max(target);
    let success = roll <= target;
    let change = (target - roll).floor().abs();
    let nudge = success.then(|| {
        let by = change * per.die_roll_mult * per.temp_mult;
        let min_change = per.min_change as i32;
        AiNudge {
            flee: (ai.flee + (-min_change).min((-by) as i32)).clamp(0, 100),
            fight: (ai.fight + min_change.max(by as i32)).clamp(0, 100),
        }
    });
    let mut x = (-change * per.die_roll_mult).floor();
    if success && x.abs() < per.min_change {
        x = -per.min_change;
    }
    Branch { success, x, y: 0.0, ai: nudge }
}

/// `DialogueManager::persuade`'s correction of the permanent change: a positive attempt never
/// takes the session's permanent total below zero. `original`: the base disposition the session
/// started from (`mOriginalDisposition`: `OpenMW`'s base, the record's value plus the permanent
/// changes); `perm_so_far`: the permanent changes accumulated in it.
pub fn session_perm_change(temp_change: i32, perm_change: i32, original: i32, perm_so_far: i32) -> i32 {
    if temp_change > 0 && perm_change > 0 && original + perm_change + perm_so_far < 0 {
        -(original + perm_so_far)
    } else {
        perm_change
    }
}

/// One conversation's persuasion bookkeeping (`DialogueManager`'s `mOriginalDisposition`,
/// `mCurrentDisposition`, `mPermanentDispositionChange`, `dialoguemanagerimp.cpp` at
/// `ca5e2576`), host-side and per conversation. Every `base` here is `OpenMW`'s base disposition
/// (`NpcStats::getBaseDisposition`): the record's value plus the permanent-change sum, unclamped.
/// The session starts all zero, as `OpenMW`'s (`startDialogue` sets none of them).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersuasionSession {
    /// `mOriginalDisposition`.
    pub original: i32,
    /// `mCurrentDisposition`.
    pub current: i32,
    /// `mPermanentDispositionChange`.
    pub perm: i32,
    /// Attempts so far (seeds [`persuasion_roll`]; not `OpenMW`'s).
    pub attempts: u32,
}

impl PersuasionSession {
    /// `updateOriginalDisposition`: a base changed by a script since the last attempt discards
    /// the session's notion of it.
    pub fn refresh(&mut self, base: i32) {
        if base != self.current {
            self.current = base;
            self.original = base;
        }
    }

    /// `persuade` after the roll: corrects and accumulates the permanent change, applies the
    /// temporary one. Returns the speaker's new base disposition (set it at once).
    pub fn attempt(&mut self, base: i32, outcome: &PersuasionOutcome) -> i32 {
        self.refresh(base);
        let perm = session_perm_change(outcome.temp_change, outcome.perm_change, self.original, self.perm);
        self.current += outcome.temp_change;
        self.perm += perm;
        self.attempts += 1;
        self.current
    }

    /// `goodbyeSelected`: the speaker's new base disposition, the original plus the permanent
    /// changes clamped so the derived value (without Charm) stays in 0..=100; `None` when the
    /// session changed nothing. `zero`: the derived disposition with a base of 0 and no Charm
    /// (unclamped). Resets the session (the attempt count stays, so later rolls differ).
    pub fn goodbye(&mut self, base: i32, zero: i32) -> Option<i32> {
        let changed = self.perm != 0 || self.original != self.current;
        let out = changed.then(|| {
            self.refresh(base);
            (self.original + self.perm).clamp(-zero, 100 - zero)
        });
        *self = Self { attempts: self.attempts, ..Self::default() };
        out
    }
}

/// `splitmix64`'s finaliser (Steele, Lea, Flood 2014; public domain constants).
const fn splitmix64(z: u64) -> u64 {
    let z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The roll of one attempt (`Misc::Rng::roll0to99`, 0..=99), deterministic so every peer and a
/// replay agree: `splitmix64` folded over (salt, the speaker key's low and high halves, the
/// player's index, `tick`, `attempt`), modulo 100. `tick`: the host's simulation tick at the
/// request; `attempt`: [`PersuasionSession::attempts`] (two attempts in one tick differ).
#[allow(clippy::cast_possible_truncation, reason = "splits the u128 key into its two u64 halves")]
pub fn persuasion_roll(speaker: ActorKey, player: PlayerId, tick: u64, attempt: u32) -> u8 {
    /// "PERSUADE" in ASCII: keeps this stream apart from other `splitmix64` users.
    const SALT: u64 = 0x5045_5253_5541_4445;
    let words = [speaker.0 as u64, (speaker.0 >> 64) as u64, u64::from(player.index()), tick, u64::from(attempt)];
    let h = words.iter().fold(SALT, |h, w| splitmix64(h ^ w));
    u8::try_from(h % 100).unwrap_or(0)
}

/// The player's Speechcraft use after an attempt (`skillUsageSucceeded(player, Speechcraft,
/// success ? 0 : 1)`): SKIL use 0 is a successful persuasion, 1 a failed one.
pub fn speechcraft_use(t: &MwTable, success: bool) -> StatOp {
    StatOp::UseSkill { skill: t.idx.skills[SPEECHCRAFT], use_kind: u8::from(!success), scale: 1.0 }
}

/// One side's [`PersuaderStats`] from its sheet.
#[allow(clippy::cast_possible_truncation, reason = "reputation and level are whole numbers stored as f32 on the sheet")]
pub fn persuader_stats(t: &MwTable, s: &SheetRead<'_>) -> PersuaderStats {
    let idx = &t.idx;
    PersuaderStats {
        personality: s.modified(idx.attributes[PERSONALITY]),
        luck: s.modified(idx.attributes[attr::LUCK]),
        speechcraft: s.modified(idx.skills[SPEECHCRAFT]),
        mercantile: s.modified(idx.skills[MERCANTILE]),
        reputation: s.modified(idx.reputation) as i32,
        level: s.modified(idx.level) as i32,
        fatigue_current: s.current(idx.fatigue),
        fatigue_max: s.max(idx.fatigue),
    }
}

/// The input for `player` persuading `speaker`. `disposition`: the speaker's current derived
/// disposition (clamped [`crate::derived_disposition`]: a session's temporary changes are already in
/// the base, see [`PersuasionSession`]); `npc_ai` and `roll` come from the caller.
/// `None`: no stats, or a sheet is missing.
pub fn persuasion_input(
    sheets: &dyn MwSheets,
    player: PlayerId,
    speaker: ActorKey,
    kind: PersuasionKind,
    disposition: i32,
    npc_ai: AiNudge,
    roll: u8,
) -> Option<PersuasionInput> {
    let t = sheets.table()?;
    let pc = persuader_stats(t, &sheets.player(player)?.view(&t.table));
    let npc = persuader_stats(t, &sheets.actor(speaker)?.view(&t.table));
    Some(PersuasionInput { kind, player: pc, npc, disposition, npc_ai, roll })
}

#[cfg(test)]
mod session_tests {
    //! The dialogue session around [`persuasion_change`] (`dialoguemanagerimp.cpp` at
    //! `ca5e2576`: `persuade`, `goodbyeSelected`, `updateOriginalDisposition`) and the roll's
    //! derivation. Hand-computed; the roll values from a reference `splitmix64`.
    use super::*;

    fn outcome(temp_change: i32, perm_change: i32) -> PersuasionOutcome {
        PersuasionOutcome { success: temp_change > 0, temp_change, perm_change, ai: None, gold: 0 }
    }

    #[test]
    fn attempts_move_the_base_and_goodbye_keeps_the_permanent_part() {
        let mut s = PersuasionSession::default();
        // Base 30: the first attempt adopts it, +5 now, +2 kept.
        assert_eq!(s.attempt(30, &outcome(5, 2)), 35);
        assert_eq!(s, PersuasionSession { original: 30, current: 35, perm: 2, attempts: 1 });
        // The host set the base to 35; -3 now, -1 kept.
        assert_eq!(s.attempt(35, &outcome(-3, -1)), 32);
        assert_eq!((s.original, s.current, s.perm), (30, 32, 1));
        // zero -10: the new base is clamp(30 + 1, 10, 110).
        assert_eq!(s.goodbye(32, -10), Some(31));
        assert_eq!(s, PersuasionSession { attempts: 2, ..PersuasionSession::default() });
        // Nothing changed since: goodbye is a no-op.
        assert_eq!(s.goodbye(31, -10), None);
    }

    #[test]
    fn a_script_change_mid_conversation_resets_the_original() {
        let mut s = PersuasionSession::default();
        assert_eq!(s.attempt(30, &outcome(5, 2)), 35);
        // `SetDisposition` made the base 50: goodbye starts from it.
        assert_eq!(s.goodbye(50, 0), Some(52));
    }

    #[test]
    fn goodbye_clamps_so_the_derived_value_stays_in_range() {
        let mut s = PersuasionSession::default();
        assert_eq!(s.attempt(95, &outcome(5, 10)), 100);
        // zero 20: at most 80, so base + 20 is 100.
        assert_eq!(s.goodbye(100, 20), Some(80));
        let mut s = PersuasionSession::default();
        assert_eq!(s.attempt(5, &outcome(-20, -20)), -15);
        assert_eq!(s.goodbye(-15, -10), Some(10));
    }

    #[test]
    fn a_positive_attempt_never_takes_the_permanent_total_below_zero() {
        let mut s = PersuasionSession::default();
        // Base -10, +3 permanent would leave -7 < 0: corrected to +10.
        assert_eq!(s.attempt(-10, &outcome(4, 3)), -6);
        assert_eq!(s.perm, 10);
        assert_eq!(session_perm_change(4, 3, 20, 0), 3);
        assert_eq!(session_perm_change(-4, 3, -10, 0), 3);
    }

    #[test]
    fn no_attempt_no_change() {
        assert_eq!(PersuasionSession::default().goodbye(40, 0), None);
    }

    #[test]
    fn the_roll_is_derived_from_speaker_player_tick_and_attempt() {
        let host = PlayerId::HOST;
        assert_eq!(splitmix64(0), 0xE220_A839_7B1D_CDAF, "the reference splitmix64");
        assert_eq!(persuasion_roll(ActorKey(1), host, 0, 0), 8);
        assert_eq!(persuasion_roll(ActorKey(1), host, 0, 1), 47);
        assert_eq!(persuasion_roll(ActorKey(1), PlayerId::new(1).unwrap(), 0, 0), 24);
        assert_eq!(persuasion_roll(ActorKey(u128::MAX), PlayerId::new(3).unwrap(), 12_345, 7), 87);
        for attempt in 0..200 {
            assert!(persuasion_roll(ActorKey(42), host, 99, attempt) < 100);
        }
    }
}
