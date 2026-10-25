//! INFO filter evaluation and text rendering (rpg-adapter.md "Filter evaluation"; `OpenMW`
//! `MWDialogue::Filter`).

use std::cell::RefCell;
use std::collections::HashSet;

use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, DialogueRules, PlayerId, TopicKind};
use tes3::esp::{
    Class, DialogueInfo, Faction, Filter, FilterComparison, FilterFunction, FilterType,
    FilterValue, Npc, NpcFlags, Race, Sex,
};

use crate::db::{INFO_REFUSAL, MwDialogue};
use crate::text::{GlobalNumber, Macro, MacroSource, expand};

/// What filters and macros need to know about an actor (speaker or player).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorFacts {
    /// Base record id.
    pub id: RecordId,
    pub name: String,
    /// Creatures pass race/class/faction/rank checks and fail INFOs without a speaker id.
    pub npc: bool,
    pub race: Option<RecordId>,
    pub race_name: String,
    pub class: Option<RecordId>,
    pub class_name: String,
    pub faction: Option<RecordId>,
    pub faction_name: String,
    /// Rank in `faction` (0-based; -1 without a faction).
    pub rank: i32,
    pub female: bool,
    /// Current cell name (exteriors: region or settlement name).
    pub cell: String,
}

impl Default for ActorFacts {
    fn default() -> Self {
        Self {
            id: RecordId::new(""),
            name: String::new(),
            npc: false,
            race: None,
            race_name: String::new(),
            class: None,
            class_name: String::new(),
            faction: None,
            faction_name: String::new(),
            rank: -1,
            female: false,
            cell: String::new(),
        }
    }
}

impl ActorFacts {
    /// From an NPC record, resolving display names through the record db.
    pub fn from_npc(records: &RecordDb, npc: &Npc, cell: &str) -> Self {
        let id_of = |s: &str| (!s.is_empty()).then(|| RecordId::new(s));
        let race = id_of(&npc.race);
        let class = id_of(&npc.class);
        let faction = id_of(&npc.faction);
        let name_of = |id: &Option<RecordId>, f: &dyn Fn(&RecordId) -> Option<String>| {
            id.as_ref().and_then(f).unwrap_or_default()
        };
        Self {
            id: RecordId::new(&npc.id),
            name: npc.name.clone(),
            npc: true,
            race_name: name_of(&race, &|id| records.get::<Race>(id).map(|r| r.name.clone())),
            class_name: name_of(&class, &|id| {
                records.get::<Class>(id).map(|c| c.name.clone())
            }),
            faction_name: name_of(&faction, &|id| {
                records.get::<Faction>(id).map(|f| f.name.clone())
            }),
            rank: if faction.is_some() {
                i32::from(npc.data.rank)
            } else {
                -1
            },
            race,
            class,
            faction,
            female: npc.npc_flags.contains(NpcFlags::FEMALE),
            cell: cell.to_owned(),
        }
    }
}

/// A faction's rank title from the record db (`%Rank`, `%PCRank`).
pub fn rank_name(records: &RecordDb, faction: &RecordId, rank: i32) -> Option<String> {
    let faction = records.get::<Faction>(faction)?;
    faction
        .rank_names
        .get(usize::try_from(rank).ok()?)
        .filter(|n| !n.is_empty())
        .cloned()
}

/// Read access to the world for filters and macros. The host implements it over its
/// state; the context player is `ctx.player` (co-op: never a singleton player).
pub trait MwView {
    fn speaker(&self, ctx: &ConversationCtx) -> Option<&ActorFacts>;
    fn player(&self, player: PlayerId) -> Option<&ActorFacts>;
    /// The player's rank in a faction; `None` if not a member.
    fn player_rank(&self, player: PlayerId, faction: &RecordId) -> Option<i32>;
    fn global(&self, id: &RecordId) -> Option<GlobalNumber>;
    /// A speaker script local; `None` if the speaker has no script or no such variable.
    fn local(&self, speaker: ActorKey, name: &RecordId) -> Option<f32>;
    fn quest_stage(&self, quest: &RecordId) -> i32;
    fn item_count(&self, player: PlayerId, item: &RecordId) -> u64;
    fn dead_count(&self, base: &RecordId) -> i32;
    /// The speaker's derived disposition towards the player.
    fn disposition(&self, ctx: &ConversationCtx) -> i32;
    fn rank_name(&self, faction: &RecordId, rank: i32) -> Option<String>;
    /// A filter function's value (PC stats, `Reaction`, `Weather`, ...). `None`: not
    /// implemented yet; the rules use a neutral 0 and record it.
    fn function(&self, ctx: &ConversationCtx, function: FilterFunction) -> Option<f32> {
        let _ = (ctx, function);
        None
    }
}

/// `DialogueRules` for Morrowind: `OpenMW`'s filter order (actor, player, select structs,
/// disposition) and the "Info Refusal" fallback.
pub struct MwDialogueRules<'a> {
    dialogue: &'a MwDialogue,
    view: &'a dyn MwView,
    neutral: RefCell<HashSet<FilterFunction>>,
}

impl<'a> MwDialogueRules<'a> {
    pub fn new(dialogue: &'a MwDialogue, view: &'a dyn MwView) -> Self {
        Self {
            dialogue,
            view,
            neutral: RefCell::new(HashSet::new()),
        }
    }

    /// Filter functions evaluated as a neutral 0 so far (for the host's log).
    pub fn neutral_functions(&self) -> Vec<FilterFunction> {
        self.neutral.borrow().iter().copied().collect()
    }

    /// Whether `ctx`'s speaker answers `topic` (a response or the refusal fallback): `OpenMW`
    /// learns a linked topic only then (`DialogueManager::addTopicsFromText`).
    pub fn speaker_answers(&self, ctx: &ConversationCtx, topic: &RecordId) -> bool {
        self.dialogue.db.select(self, ctx, topic).is_some() || self.fallback(ctx, topic).is_some()
    }

    /// Macro expansion for `ctx` (responses and journal entries).
    pub fn expand(&self, ctx: &ConversationCtx, text: &str) -> String {
        expand(text, &Macros { rules: self, ctx })
    }

    /// Every check except disposition.
    fn passes_static(&self, ctx: &ConversationCtx, info: &DialogueInfo) -> bool {
        let Some(speaker) = self.view.speaker(ctx) else {
            return false;
        };
        Self::passes_actor(info, speaker)
            && self.passes_player(ctx, info, speaker)
            && self.passes_selects(ctx, info, speaker)
    }

    fn passes_disposition(&self, ctx: &ConversationCtx, info: &DialogueInfo) -> bool {
        self.view.speaker(ctx).is_some_and(|s| !s.npc)
            || self.view.disposition(ctx) >= info.data.disposition
    }

    /// `OpenMW` `Filter::testActor`.
    fn passes_actor(info: &DialogueInfo, speaker: &ActorFacts) -> bool {
        let eq = |field: &str, id: Option<&RecordId>| {
            field.is_empty() || id.is_some_and(|id| id.as_str().eq_ignore_ascii_case(field))
        };
        if !info.speaker_id.is_empty()
            && !speaker.id.as_str().eq_ignore_ascii_case(&info.speaker_id)
        {
            return false;
        }
        if !speaker.npc {
            // Creatures pass the NPC checks, but only INFOs naming them.
            return !info.speaker_id.is_empty();
        }
        if !eq(&info.speaker_race, speaker.race.as_ref())
            || !eq(&info.speaker_class, speaker.class.as_ref())
            || !eq(&info.speaker_faction, speaker.faction.as_ref())
        {
            return false;
        }
        if info.data.speaker_rank != -1
            && (speaker.faction.is_none() || speaker.rank < i32::from(info.data.speaker_rank))
        {
            return false;
        }
        match info.data.speaker_sex {
            Sex::Any => true,
            Sex::Male => !speaker.female,
            Sex::Female => speaker.female,
        }
    }

    /// `OpenMW` `Filter::testPlayer`: faction/rank and the cell (the player's, prefix match).
    fn passes_player(
        &self,
        ctx: &ConversationCtx,
        info: &DialogueInfo,
        speaker: &ActorFacts,
    ) -> bool {
        let required = i32::from(info.data.player_rank);
        if !info.player_faction.is_empty() {
            match self
                .view
                .player_rank(ctx.player, &RecordId::new(&info.player_faction))
            {
                Some(rank) if rank >= required => {}
                _ => return false,
            }
        } else if required != -1 {
            let Some(faction) = &speaker.faction else {
                return false;
            };
            if self
                .view
                .player_rank(ctx.player, faction)
                .is_none_or(|rank| rank < required)
            {
                return false;
            }
        }
        if !info.speaker_cell.is_empty() {
            let cell = self.view.player(ctx.player).map_or("", |p| p.cell.as_str());
            if !starts_with_ci(cell, &info.speaker_cell) {
                return false;
            }
        }
        true
    }

    fn passes_selects(
        &self,
        ctx: &ConversationCtx,
        info: &DialogueInfo,
        speaker: &ActorFacts,
    ) -> bool {
        info.filters
            .iter()
            .all(|f| self.passes_select(ctx, f, speaker))
    }

    /// `OpenMW` `Filter::testSelectStruct`. Inverted kinds (`Not*`) ignore the comparison.
    fn passes_select(&self, ctx: &ConversationCtx, filter: &Filter, speaker: &ActorFacts) -> bool {
        let id = RecordId::new(&filter.id);
        let is = |field: Option<&RecordId>| field == Some(&id);
        let value = match filter.filter_type {
            FilterType::None => return true,
            FilterType::NotId => return !is(Some(&speaker.id)),
            FilterType::NotFaction => return !speaker.npc || !is(speaker.faction.as_ref()),
            FilterType::NotClass => return !speaker.npc || !is(speaker.class.as_ref()),
            FilterType::NotRace => return !speaker.npc || !is(speaker.race.as_ref()),
            FilterType::NotCell => return !starts_with_ci(&speaker.cell, &filter.id),
            FilterType::NotLocal => return self.view.local(ctx.speaker, &id).is_none(),
            FilterType::Local => match self.view.local(ctx.speaker, &id) {
                Some(v) => f64::from(v),
                None => return false,
            },
            // A missing global is ignored (OpenMW).
            FilterType::Global => match self.view.global(&id) {
                Some(v) => f64::from(v.as_f32()),
                None => return true,
            },
            FilterType::Journal => f64::from(self.view.quest_stage(&id)),
            #[allow(clippy::cast_precision_loss)]
            FilterType::Item => self.view.item_count(ctx.player, &id) as f64,
            FilterType::Dead => f64::from(self.view.dead_count(&id)),
            FilterType::Function => self.function_value(ctx, filter.function),
        };
        compare(value, filter.comparison, filter.value)
    }

    fn function_value(&self, ctx: &ConversationCtx, function: FilterFunction) -> f64 {
        if function == FilterFunction::Choice {
            return f64::from(ctx.choice.unwrap_or(-1));
        }
        self.view.function(ctx, function).map_or_else(
            || {
                self.neutral.borrow_mut().insert(function);
                0.0
            },
            f64::from,
        )
    }

    fn macro_value(&self, ctx: &ConversationCtx, which: Macro) -> String {
        let speaker = self.view.speaker(ctx);
        let player = self.view.player(ctx.player);
        let speaker_faction = speaker.and_then(|s| s.faction.as_ref());
        let pc_rank =
            || speaker_faction.and_then(|f| self.view.player_rank(ctx.player, f).map(|r| (f, r)));
        let text = |f: fn(&ActorFacts) -> &str, who: Option<&ActorFacts>| {
            who.map(f).unwrap_or_default().to_owned()
        };
        match which {
            Macro::PcName => text(|a| a.name.as_str(), player),
            Macro::PcRace => text(|a| a.race_name.as_str(), player),
            Macro::PcClass => text(|a| a.class_name.as_str(), player),
            Macro::PcRank => pc_rank()
                .and_then(|(f, r)| self.view.rank_name(f, r))
                .unwrap_or_default(),
            Macro::PcNextRank => pc_rank()
                .and_then(|(f, r)| self.view.rank_name(f, r + 1))
                .unwrap_or_default(),
            Macro::PcCrimeLevel => {
                let v = self.function_value(ctx, FilterFunction::PcCrimeLevel);
                format!("{v}")
            }
            Macro::Name => text(|a| a.name.as_str(), speaker),
            Macro::Race => text(|a| a.race_name.as_str(), speaker),
            Macro::Class => text(|a| a.class_name.as_str(), speaker),
            Macro::Faction => text(|a| a.faction_name.as_str(), speaker),
            Macro::Rank => speaker
                .and_then(|s| self.view.rank_name(s.faction.as_ref()?, s.rank))
                .unwrap_or_default(),
            Macro::NextRank => speaker
                .and_then(|s| self.view.rank_name(s.faction.as_ref()?, s.rank + 1))
                .unwrap_or_default(),
            Macro::Cell => text(|a| a.cell.as_str(), player),
        }
    }
}

impl DialogueRules for MwDialogueRules<'_> {
    fn accepts(&self, ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> bool {
        self.dialogue
            .info(response)
            .is_some_and(|info| self.passes_static(ctx, info) && self.passes_disposition(ctx, info))
    }

    fn render(&self, ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> String {
        self.dialogue
            .info(response)
            .map(|info| self.expand(ctx, &info.text))
            .unwrap_or_default()
    }

    /// "Info Refusal" when some response under a conversation topic failed only on
    /// disposition (not for greetings: `OpenMW` `startDialogue` searches without fallback).
    fn fallback(&self, ctx: &ConversationCtx, topic: &RecordId) -> Option<(RecordId, RecordId)> {
        let entry = self.dialogue.db.topic(topic)?;
        if entry.kind != TopicKind::Conversation {
            return None;
        }
        let refused = entry
            .responses
            .iter()
            .filter_map(|r| self.dialogue.info(r))
            .any(|info| self.passes_static(ctx, info) && !self.passes_disposition(ctx, info));
        if !refused {
            return None;
        }
        let refusal = RecordId::new(INFO_REFUSAL);
        let response = self.dialogue.db.select(self, ctx, &refusal)?;
        Some((refusal, response))
    }
}

struct Macros<'r, 'a> {
    rules: &'r MwDialogueRules<'a>,
    ctx: &'r ConversationCtx,
}

impl MacroSource for Macros<'_, '_> {
    fn macro_value(&self, which: Macro) -> String {
        self.rules.macro_value(self.ctx, which)
    }

    fn global(&self, id: &str) -> Option<GlobalNumber> {
        self.rules.view.global(&RecordId::new(id))
    }
}

fn starts_with_ci(text: &str, prefix: &str) -> bool {
    text.get(..prefix.len())
        .is_some_and(|p| p.eq_ignore_ascii_case(prefix))
}

fn compare(value: f64, op: FilterComparison, literal: FilterValue) -> bool {
    let literal = match literal {
        FilterValue::Integer(i) => f64::from(i),
        FilterValue::Float(f) => f64::from(f),
    };
    match op {
        FilterComparison::Equal => (value - literal).abs() < f64::EPSILON,
        FilterComparison::NotEqual => (value - literal).abs() >= f64::EPSILON,
        FilterComparison::Greater => value > literal,
        FilterComparison::GreaterEqual => value >= literal,
        FilterComparison::Less => value < literal,
        FilterComparison::LessEqual => value <= literal,
    }
}
