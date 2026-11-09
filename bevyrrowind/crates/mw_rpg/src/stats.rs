//! Dialogue filter functions that read character sheets (stats.md S6, stats-adapter.md
//! "Consumers"): the host's [`crate::MwView`] answers `function` and `player_rank` through
//! [`sheet_function`] and [`player_rank`] over its [`MwSheets`].
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`,
//! `apps/openmw/mwdialogue/filter.cpp`:
//! - `Filter::getSelectStructInteger`: the 8 `Pc<Attribute>` and 27 `Pc<Skill>` functions are
//!   the modified value truncated to an integer; `PcLevel`, `PcReputation`, `PcCrimeLevel`
//!   (bounty), `PcWerewolfKills`; the speaker's `Level` and `Reputation` (0 for a creature);
//!   `RankRequirement` (`mw_stats::FactionRanks::rank_requirement`) and `FactionRankDifference`
//!   (player rank, -1 outside, minus the speaker's), both 0 when the speaker has no faction.
//! - `Filter::testSelectStructNumeric`: `PcHealth` / `PcMagicka` / `PcFatigue` are the pool's
//!   current value (not truncated); `PcHealthPercent` and the speaker's `HealthPercent` are
//!   `int(current / max x 100)` (`DynamicStat::getRatio`, 0 for an empty pool).
//! - `Filter::getSelectStructBoolean`: `SameFaction` (the player is a member of the speaker's
//!   faction), `PcExpelled` (expelled from the speaker's faction; false without one), as 1 / 0.
//! - `Filter::testPlayer` and `getFactionRank` (`apps/openmw/mwmechanics/npcstats.cpp`): the
//!   player's rank in a faction, `None` outside it.
//!
//! Every other function is `None` here (the rules' neutral 0, logged): the magic-effect ones
//! (`PcCorprus`, `PcVampire`, `PcCommonDisease`, `PcBlightDisease`) until the effect runtime,
//! and the non-stat ones (`PcSex`, `SameSex`, `SameRace`, AI settings, `Weather`, ...). The host
//! adds `PcClothingModifier` from the player's equipment ([`crate::clothing_value`]).
//! No sheet (no stats loaded, an unknown player or speaker) is neutral too, never a default
//! character.

use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, PlayerId};
use sf_stats::{Rules, Sheet, SheetRead, StatIdx};
use tes3::esp::{Faction, FilterFunction};

use mw_stats::{FactionRanks, MwTable};

use crate::ActorFacts;

/// The character sheets a conversation reads, owned by the host's stats. Co-op: the player is
/// looked up by id (`ctx.player`), never a singleton.
pub trait MwSheets {
    /// Morrowind's stat table (`None`: no stats in this run; every sheet function is neutral).
    fn table(&self) -> Option<&MwTable>;
    fn player(&self, player: PlayerId) -> Option<&Sheet>;
    /// An NPC's or creature's sheet by its actor key (the speaker's: `ctx.speaker`).
    fn actor(&self, actor: ActorKey) -> Option<&Sheet>;
    /// The rules the host steps sheets with. Result scripts need them to see their own stat
    /// writes later in the same run (stats.md S7); `None`: they read the host's sheets only.
    fn rules(&self) -> Option<&Rules> {
        None
    }
}

/// No stats: every sheet function is neutral (headless tools, tests, a run without records).
pub struct NoSheets;

impl MwSheets for NoSheets {
    fn table(&self) -> Option<&MwTable> {
        None
    }

    fn player(&self, _player: PlayerId) -> Option<&Sheet> {
        None
    }

    fn actor(&self, _actor: ActorKey) -> Option<&Sheet> {
        None
    }
}

/// The player's rank in `faction` (`None`: not a member, or no sheet).
pub fn player_rank(sheets: &dyn MwSheets, player: PlayerId, faction: &RecordId) -> Option<i32> {
    let t = sheets.table()?;
    sheets.player(player)?.view(&t.table).membership(faction)?.rank
}

/// A filter function's value from the sheets (see the module docs); `None`: not a sheet
/// function, or the sheet it needs is missing.
#[allow(clippy::cast_precision_loss)] // ranks and rank requirements are small integers
pub fn sheet_function(
    sheets: &dyn MwSheets,
    records: &RecordDb,
    ctx: &ConversationCtx,
    speaker: &ActorFacts,
    function: FilterFunction,
) -> Option<f32> {
    use FilterFunction as F;
    let t = sheets.table()?;
    let idx = &t.idx;
    let pc = || sheets.player(ctx.player).map(|s| s.view(&t.table));
    let npc = || sheets.actor(ctx.speaker).map(|s| s.view(&t.table));
    if let Some(a) = attribute(function) {
        return pc().map(|s| s.modified(idx.attributes[a]).trunc());
    }
    if let Some(k) = skill(function) {
        return pc().map(|s| s.modified(idx.skills[k]).trunc());
    }
    let faction = speaker.faction.as_ref();
    match function {
        F::PcHealthPercent => pc().map(|s| percent(&s, idx.health)),
        F::PcHealth => pc().map(|s| s.current(idx.health)),
        F::PcMagicka => pc().map(|s| s.current(idx.magicka)),
        F::PcFatigue => pc().map(|s| s.current(idx.fatigue)),
        F::PcLevel => pc().map(|s| s.modified(idx.level)),
        F::PcReputation => pc().map(|s| s.modified(idx.reputation)),
        F::PcCrimeLevel => pc().map(|s| s.modified(idx.bounty)),
        F::WerewolfKills => pc().map(|s| s.modified(idx.werewolf_kills)),
        F::SameFaction => pc().map(|s| flag(faction.is_some_and(|f| rank_in(&s, f).is_some()))),
        F::PcExpelled => pc().map(|s| flag(faction.is_some_and(|f| s.membership(f).is_some_and(|m| m.expelled)))),
        F::RankRequirement => {
            let s = pc()?;
            let Some(f) = faction else { return Some(0.0) };
            let ranks = FactionRanks::of(records.get::<Faction>(f)?);
            Some(ranks.rank_requirement(idx, &s, f) as f32)
        }
        F::FactionRankDifference => {
            let s = pc()?;
            let Some(f) = faction else { return Some(0.0) };
            // The speaker's live rank (its sheet), else its record's.
            let theirs = npc().and_then(|n| rank_in(&n, f)).unwrap_or(speaker.rank);
            Some((rank_in(&s, f).unwrap_or(-1) - theirs) as f32)
        }
        F::Level => npc().map(|s| s.modified(idx.level)),
        F::Reputation if !speaker.npc => Some(0.0),
        F::Reputation => npc().map(|s| s.modified(idx.reputation)),
        F::HealthPercent => npc().map(|s| percent(&s, idx.health)),
        _ => None,
    }
}

fn rank_in(s: &SheetRead<'_>, faction: &RecordId) -> Option<i32> {
    s.membership(faction)?.rank
}

/// `static_cast<int>(getRatio() * 100)`.
fn percent(s: &SheetRead<'_>, pool: StatIdx) -> f32 {
    (s.ratio(pool) * 100.0).trunc()
}

fn flag(b: bool) -> f32 {
    if b { 1.0 } else { 0.0 }
}

/// The ESM attribute index a `Pc<Attribute>` function reads.
fn attribute(function: FilterFunction) -> Option<usize> {
    use FilterFunction as F;
    Some(match function {
        F::PcStrength => 0,
        F::PcIntelligence => 1,
        F::PcWillpower => 2,
        F::PcAgility => 3,
        F::PcSpeed => 4,
        F::PcEndurance => 5,
        F::PcPersonality => 6,
        F::PcLuck => 7,
        _ => return None,
    })
}

/// The ESM skill index a `Pc<Skill>` function reads (codes 11-37 in ESM skill order).
fn skill(function: FilterFunction) -> Option<usize> {
    use FilterFunction as F;
    const SKILLS: [FilterFunction; 27] = [
        F::PcBlock,
        F::PcArmorer,
        F::PcMediumArmor,
        F::PcHeavyArmor,
        F::PcBluntWeapon,
        F::PcLongBlade,
        F::PcAxe,
        F::PcSpear,
        F::PcAthletics,
        F::PcEnchant,
        F::PcDestruction,
        F::PcAlteration,
        F::PcIllusion,
        F::PcConjuration,
        F::PcMysticism,
        F::PcRestoration,
        F::PcAlchemy,
        F::PcUnarmored,
        F::PcSecurity,
        F::PcSneak,
        F::PcAcrobatics,
        F::PcLightArmor,
        F::PcShortBlade,
        F::PcMarksman,
        F::PcMercantile,
        F::PcSpeechcraft,
        F::PcHandToHand,
    ];
    SKILLS.iter().position(|f| *f == function)
}
