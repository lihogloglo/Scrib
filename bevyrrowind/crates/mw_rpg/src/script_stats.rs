//! Stat, faction and crime functions in result scripts (stats.md S7, stats-adapter.md
//! "Consumers", `MWScript` bullet): each [`StatCall`] reads the sheet the run sees, and a write
//! becomes `sf_stats` ops, applied to the run's copy of that sheet (later reads in the same run
//! see it, as in the original engine) and handed to the host as one [`ResultEffect::Stats`].
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
//! - `apps/openmw/mwscript/statsextensions.cpp`: `OpGet/Set/ModAttribute` (Get = modified,
//!   Set = base with the damage cleared, Mod through `modStat`: base clamped to 0..=100, a gain
//!   capped at a modified 100 unless a fortification lifts a base under 100), `OpGet/Set/ModSkill`
//!   (writes on NPCs only), `OpGetDynamic` (current; magicka never below 0), `OpSetDynamic`
//!   (base, then current = max), `OpModDynamic` (base + diff, kept ≥ 0 except fatigue;
//!   current + diff, unclamped), `OpModCurrentDynamic` (current + diff, fatigue may go below 0),
//!   `OpGetDynamicGetRatio` (`DynamicStat::getRatio(false)`, `mwmechanics/stat.cpp`: 1 for a
//!   0 max), `OpGet/SetLevel`, `OpGet/Set/ModPCCrimeLevel` (bounty; Mod floors the total at 0),
//!   `OpPCJoinFaction`, `OpPCRaiseRank` (joins a non-member), `OpPCLowerRank`, `OpGetPCRank`,
//!   `OpPcExpell(ed)`, `OpPcClearExpelled`, `OpGet/Set/ModPCFacRep`, `OpRaiseRank`/`OpLowerRank`
//!   (the NPC's own rank, from its record rank the first time; no-op on the player).
//! - `apps/openmw/mwscript/dialogueextensions.cpp`: `OpGet/Set/ModReputation` (the NPC's own),
//!   through `NpcStats::setReputation` (`mwmechanics/npcstats.cpp`: clamped to 0..=255).
//! - `apps/openmw/mwmechanics/npcstats.cpp` `setFactionRank`: members only; below 0 leaves the
//!   faction ([`StatOp::Leave`]); a rank past the faction's 10 rank slots is ignored.
//!
//! Where we differ (named, stats-adapter.md "MWScript"):
//! - `modStat` moves a gain past base 100 into the modifier and adjusts damage; modifiers here are
//!   source-tracked (never mutate-and-undo, stats.md), so that excess is dropped instead.
//! - `setBase(value, true)` also zeroes the modifier; ours clears damage only (fortify effects
//!   belong to their sources).
//! - `OpenMW` throws (the script stops) when a function needs the speaker's faction and there is
//!   none, and checks that a named faction exists; the call is skipped here and the run goes on.
//! - `OpModDynamic`'s `dagoth_ur_1` workaround (an unloaded remote target) is the general rule
//!   here: a target without a sheet is skipped.
//! - Not ported yet: the `PCExpell` message box (`sExpelledMessage`), the fatigue knockdown of
//!   `ModCurrentFatigue` (combat wires fatigue knockdowns, combat-adapter.md), `recordCrimeId`
//!   when the bounty reaches 0 (crime spec).

use std::collections::HashMap;

use mw_script::{Dynamic, StatCall, StatFn};
use mw_stats::{MwTable, RANKS};
use sf_records::RecordId;
use sf_rpg::{ActorKey, ConversationCtx};
use sf_stats::{ApplyCtx, Cause, CauseKind, PlayerKey, Rules, Sheet, SheetRead, StatIdx, StatOp, StatOwner};

use crate::result::{ResultEffect, Target};
use crate::runner::MwWorld;
use crate::stats::MwSheets;

/// Whose sheet a call reads or writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Who {
    /// The context player (`ctx.player`).
    Player,
    Actor(ActorKey),
}

/// The run's copies of the sheets it wrote (dropped with the run; the host applies the effects).
#[derive(Debug, Default)]
pub(crate) struct StatOverlay {
    sheets: HashMap<Who, Sheet>,
}

/// What a call does once its sheet is known.
enum Step {
    Read(f32),
    Write(Vec<StatOp>),
    /// `Set<Dyn>`: the base, then the current to the max that base gives.
    SetPool { pool: StatIdx, base: f32 },
    Skip,
}

impl StatOverlay {
    /// Runs one call on `target` (the runner's target of the call); returns what it reads (0 for
    /// writes) and appends a write's [`ResultEffect::Stats`].
    pub(crate) fn call(
        &mut self,
        ctx: &ConversationCtx,
        env: &dyn MwWorld,
        target: &Target,
        call: &StatCall,
        effects: &mut Vec<ResultEffect>,
    ) -> f64 {
        let f = call.function;
        let sheets = env.sheets();
        let Some(t) = sheets.table() else {
            return f.no_sheet_value();
        };
        let player_fn = is_player_fn(f);
        let who = if player_fn || *target == Target::Player {
            Who::Player
        } else {
            match env.actor(ctx, target) {
                Some(k) => Who::Actor(k),
                None => return f.no_sheet_value(),
            }
        };
        let facts = env.facts(ctx, target);
        // The faction a faction function means: as written, else the target's own.
        let faction = call
            .faction
            .as_ref()
            .map(|id| RecordId::new(&id.key))
            .or_else(|| facts.and_then(|a| a.faction.clone()));
        let step = {
            let Some(sheet) = self.sheet(sheets, ctx, who) else {
                return f.no_sheet_value();
            };
            let s = sheet.view(&t.table);
            let record_rank = facts.map_or(-1, |a| a.rank);
            step(t, &s, f, call.value, faction.as_ref(), who, record_rank)
        };
        let ops = match step {
            Step::Read(v) => return f64::from(v),
            Step::Skip => return 0.0,
            Step::Write(ops) => {
                self.apply(t, sheets, ctx, who, &ops);
                ops
            }
            Step::SetPool { pool, base } => {
                let mut ops = vec![StatOp::SetBase { stat: pool, value: base }];
                self.apply(t, sheets, ctx, who, &ops);
                let max = self.sheet(sheets, ctx, who).map_or(base, |s| s.view(&t.table).max(pool));
                let set = [StatOp::PoolSetCurrent { pool, value: max }];
                self.apply(t, sheets, ctx, who, &set);
                ops.extend(set);
                ops
            }
        };
        if !ops.is_empty() {
            let target = if who == Who::Player { Target::Player } else { target.clone() };
            effects.push(ResultEffect::Stats { target, ops });
        }
        0.0
    }

    /// The sheet the run sees: its own copy once it wrote one, else the host's.
    fn sheet<'s>(&'s self, sheets: &'s dyn MwSheets, ctx: &ConversationCtx, who: Who) -> Option<&'s Sheet> {
        self.sheets.get(&who).or_else(|| host_sheet(sheets, ctx, who))
    }

    /// Applies `ops` to the run's copy (made on the first write). Without the host's rules the
    /// copy can't be stepped: later reads in the run see the host's sheet.
    fn apply(&mut self, t: &MwTable, sheets: &dyn MwSheets, ctx: &ConversationCtx, who: Who, ops: &[StatOp]) {
        let Some(rules) = sheets.rules() else { return };
        if let std::collections::hash_map::Entry::Vacant(slot) = self.sheets.entry(who) {
            let Some(base) = host_sheet(sheets, ctx, who) else { return };
            slot.insert(base.clone());
        }
        let Some(sheet) = self.sheets.get_mut(&who) else { return };
        step_copy(t, rules, sheet, who, ops);
    }
}

fn host_sheet<'s>(sheets: &'s dyn MwSheets, ctx: &ConversationCtx, who: Who) -> Option<&'s Sheet> {
    match who {
        Who::Player => sheets.player(ctx.player),
        Who::Actor(k) => sheets.actor(k),
    }
}

/// One `sf_stats` step on the run's copy, with the inputs its owner last derived from. The
/// events are dropped: the host's own step publishes them when it applies the effect.
fn step_copy(t: &MwTable, rules: &Rules, sheet: &mut Sheet, who: Who, ops: &[StatOp]) {
    let inputs = sheet.last_inputs().cloned().unwrap_or_default();
    // The owner only labels events, which a copy never publishes.
    let owner = match who {
        Who::Player => StatOwner::Player(PlayerKey(0)),
        Who::Actor(k) => StatOwner::Actor(k),
    };
    let cause = Cause { kind: CauseKind::Script, source: None, tick: 0 };
    let ctx = ApplyCtx { owner, cause, inputs: &inputs };
    sf_stats::apply(&t.table, rules, sheet, ops, &ctx, &mut Vec::new());
}

/// Functions that act on the context player whatever the target (the target only picks the
/// default faction).
fn is_player_fn(f: StatFn) -> bool {
    use StatFn as F;
    matches!(
        f,
        F::GetPcCrimeLevel
            | F::SetPcCrimeLevel
            | F::ModPcCrimeLevel
            | F::PcJoinFaction
            | F::PcRaiseRank
            | F::PcLowerRank
            | F::GetPcRank
            | F::PcExpell
            | F::PcExpelled
            | F::PcClearExpelled
            | F::GetPcFacRep
            | F::SetPcFacRep
            | F::ModPcFacRep
    )
}

/// `OpenMW` `modStat` (`statsextensions.cpp`): the new base for `amount` added to a value with
/// this base and modified value. The gain is capped so the modified value stays ≤ 100 unless a
/// fortification lifts a base under 100; the base stays in 0..=100.
pub(crate) fn mod_stat(base: f32, modified: f32, amount: f32) -> f32 {
    if !amount.is_finite() {
        return base;
    }
    let bonus = modified - base;
    let mut amount = amount;
    if (bonus <= 0.0 || base >= 100.0) && amount > 0.0 {
        amount = (100.0 - modified).clamp(0.0, amount);
    }
    (base + amount).clamp(0.0, 100.0)
}

/// An integer argument as `OpenMW` takes it (`runtime[0].mInteger`, `static_cast<int>`).
fn int(v: f64) -> i32 {
    mw_script::interp::to_int(v)
}

#[allow(clippy::cast_possible_truncation)]
fn float(v: f64) -> f32 {
    v as f32
}

fn flag(b: bool) -> f32 {
    if b { 1.0 } else { 0.0 }
}

/// `NpcStats::setFactionRank` for a member at `new`: below 0 leaves, past the rank slots is
/// ignored.
fn set_rank(group: &RecordId, new: i32) -> Option<StatOp> {
    if new < 0 {
        Some(StatOp::Leave { group: group.clone() })
    } else if usize::try_from(new).is_ok_and(|n| n < RANKS) {
        Some(StatOp::SetRank { group: group.clone(), rank: new })
    } else {
        None
    }
}

/// What `f` does on sheet `s`. `faction`: the call's faction (argument or the target's own);
/// `record_rank`: the target's rank in its record (`RaiseRank` / `LowerRank` the first time).
// One arm per function; ranks, reputations and bounties are small integers.
#[allow(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names
)]
fn step(
    t: &MwTable,
    s: &SheetRead<'_>,
    f: StatFn,
    value: f64,
    faction: Option<&RecordId>,
    who: Who,
    record_rank: i32,
) -> Step {
    use StatFn as F;
    let i = &t.idx;
    let v = float(value);
    let npc = s.modified(i.creature) < 1.0;
    let pool = |d: Dynamic| match d {
        Dynamic::Health => i.health,
        Dynamic::Magicka => i.magicka,
        Dynamic::Fatigue => i.fatigue,
    };
    let write = |op: StatOp| Step::Write(vec![op]);
    let rank = faction.and_then(|g| s.membership(g)).and_then(|m| m.rank);
    match f {
        F::GetAttribute(a) => Step::Read(s.modified(i.attributes[usize::from(a)])),
        F::GetSkill(k) => Step::Read(s.modified(i.skills[usize::from(k)])),
        F::SetAttribute(_) | F::SetSkill(_) | F::ModAttribute(_) | F::ModSkill(_) => {
            let (stat, set) = match f {
                F::SetAttribute(a) => (i.attributes[usize::from(a)], true),
                F::ModAttribute(a) => (i.attributes[usize::from(a)], false),
                F::SetSkill(k) if npc => (i.skills[usize::from(k)], true),
                F::ModSkill(k) if npc => (i.skills[usize::from(k)], false),
                _ => return Step::Skip,
            };
            if set {
                // `setBase(value, true)`: the damage goes too.
                Step::Write(vec![StatOp::SetBase { stat, value: v }, StatOp::Restore { stat, amount: f32::MAX }])
            } else {
                write(StatOp::SetBase { stat, value: mod_stat(s.base(stat), s.modified(stat), v) })
            }
        }
        F::GetDynamic(d) => {
            let current = s.current(pool(d));
            Step::Read(if d == Dynamic::Magicka { current.max(0.0) } else { current })
        }
        F::SetDynamic(d) => Step::SetPool { pool: pool(d), base: v },
        F::ModDynamic(d) => {
            let p = pool(d);
            let (base, current) = (s.base(p), s.current(p));
            if d == Dynamic::Fatigue || base + v >= 0.0 {
                // Base and current move together (`PoolModMax`, no rebase).
                write(StatOp::PoolModMax { pool: p, delta: v })
            } else {
                // The base stops at 0; the current still moves by the whole diff.
                Step::Write(vec![
                    StatOp::PoolModMax { pool: p, delta: -base },
                    StatOp::PoolSetCurrent { pool: p, value: current + v },
                ])
            }
        }
        F::ModCurrentDynamic(d) => write(StatOp::PoolAdjust {
            pool: pool(d),
            delta: v,
            below_zero: d == Dynamic::Fatigue,
            above_max: false,
        }),
        F::GetDynamicRatio(d) => {
            let p = pool(d);
            let max = s.max(p);
            // `getRatio(false)`: a max of exactly 0 reads 1.
            #[allow(clippy::float_cmp)]
            let empty = max == 0.0;
            Step::Read(if empty { 1.0 } else { s.current(p) / max })
        }
        F::GetLevel => Step::Read(s.modified(i.level)),
        F::SetLevel => write(StatOp::CounterSet { stat: i.level, value: int(value) as f32 }),
        F::GetReputation if npc => Step::Read(s.modified(i.reputation)),
        F::GetReputation => Step::Read(0.0),
        F::SetReputation | F::ModReputation if npc => {
            let old = s.modified(i.reputation) as i32;
            let new = if f == F::SetReputation { int(value) } else { old.saturating_add(int(value)) };
            write(StatOp::CounterSet { stat: i.reputation, value: new.clamp(0, 255) as f32 })
        }
        F::SetReputation | F::ModReputation => Step::Skip,
        F::GetPcCrimeLevel => Step::Read(s.modified(i.bounty)),
        F::SetPcCrimeLevel => write(StatOp::CounterSet { stat: i.bounty, value: int(value) as f32 }),
        F::ModPcCrimeLevel => {
            let bounty = (int(value).saturating_add(s.modified(i.bounty) as i32)).max(0);
            write(StatOp::CounterSet { stat: i.bounty, value: bounty as f32 })
        }
        F::GetPcRank => Step::Read(if faction.is_some() { rank.unwrap_or(-1) as f32 } else { -1.0 }),
        F::PcExpelled => Step::Read(flag(faction.and_then(|g| s.membership(g)).is_some_and(|m| m.expelled))),
        F::GetPcFacRep => Step::Read(faction.and_then(|g| s.membership(g)).map_or(0.0, |m| m.reputation as f32)),
        F::PcJoinFaction
        | F::PcRaiseRank
        | F::PcLowerRank
        | F::PcExpell
        | F::PcClearExpelled
        | F::SetPcFacRep
        | F::ModPcFacRep
        | F::RaiseRank
        | F::LowerRank => {
            let Some(group) = faction else { return Step::Skip };
            let g = group.clone();
            let ops: Vec<StatOp> = match f {
                F::PcJoinFaction => vec![StatOp::Join { group: g }],
                F::PcRaiseRank => match rank {
                    Some(r) => set_rank(group, r + 1).into_iter().collect(),
                    None => vec![StatOp::Join { group: g }],
                },
                F::PcLowerRank => rank.and_then(|r| set_rank(group, r - 1)).into_iter().collect(),
                F::PcExpell => vec![StatOp::Expel { group: g }],
                F::PcClearExpelled => vec![StatOp::ClearExpelled { group: g }],
                F::SetPcFacRep => vec![StatOp::GroupRep { group: g, delta: int(value), set: true }],
                F::ModPcFacRep => vec![StatOp::GroupRep { group: g, delta: int(value), set: false }],
                // The NPC's own rank: never the player's (`OpRaiseRank` "no-op when executed on
                // the player"), from its record's rank until its sheet holds one.
                F::RaiseRank | F::LowerRank if who == Who::Player => return Step::Skip,
                F::RaiseRank => match rank {
                    Some(r) => set_rank(group, r + 1).into_iter().collect(),
                    None => std::iter::once(StatOp::Join { group: g }).chain(set_rank(group, record_rank + 1)).collect(),
                },
                F::LowerRank => match rank {
                    Some(0) => Vec::new(),
                    Some(r) => set_rank(group, r - 1).into_iter().collect(),
                    None => std::iter::once(StatOp::Join { group: g })
                        .chain(set_rank(group, (record_rank - 1).max(0)))
                        .collect(),
                },
                _ => Vec::new(),
            };
            if ops.is_empty() { Step::Skip } else { Step::Write(ops) }
        }
    }
}
