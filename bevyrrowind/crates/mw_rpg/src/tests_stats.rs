//! The PC stat filter functions against a test sheet (stats.md S6): exact thresholds on either
//! side, faction rank, and neutral without a sheet (no default character).

use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, DialogueRules, PlayerId};
use sf_stats::{ApplyCtx, Cause, CauseKind, Inputs, Sheet, StatOp, StatOwner};
use tes3::esp::{
    Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison, FilterFunction, FilterType,
    FilterValue,
};

use mw_stats::{MwGmst, MwTable, SkillInfo, openmw_rules};

use crate::db::info;
use crate::{
    ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwSheets, MwView, response_id,
};

const FIGHTERS: &str = "fighters guild";
const SPEAKER: ActorKey = ActorKey(7);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn table() -> MwTable {
    MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
        governing: 0,
        specialization: 0,
        use_values: [1.0, 0.0, 0.0, 0.0],
    }))
    .unwrap()
}

/// Strength 50, Long Blade 40, health 60 / 80 (75 %), level 4, Fighters Guild rank 3.
fn player_sheet(t: &MwTable) -> Sheet {
    let idx = &t.idx;
    let rules = openmw_rules(t, &MwGmst::vanilla());
    let owner = StatOwner::Player(sf_stats::PlayerKey(1));
    let inputs = Inputs { is_player: true, ..Inputs::default() };
    let ops = [
        StatOp::SetBase { stat: idx.attributes[0], value: 50.0 },
        StatOp::SetBase { stat: idx.skills[5], value: 40.0 },
        StatOp::SetBase { stat: idx.health, value: 80.0 },
        StatOp::CounterSet { stat: idx.level, value: 4.0 },
        StatOp::SetRank { group: id(FIGHTERS), rank: 3 },
    ];
    let (mut sheet, rejected) = mw_stats::new_sheet(t, &rules, owner, &inputs, &ops);
    assert_eq!(rejected, 0);
    let hurt = [StatOp::PoolAdjust { pool: idx.health, delta: -20.0, below_zero: false, above_max: false }];
    let ctx = ApplyCtx { owner, cause: Cause { kind: CauseKind::Combat, source: None, tick: 0 }, inputs: &inputs };
    assert_eq!(sf_stats::apply(&t.table, &rules, &mut sheet, &hurt, &ctx, &mut Vec::new()), 0);
    sheet
}

/// The host's sheets: the host player's, nobody else's.
struct Sheets {
    table: Option<MwTable>,
    host: Option<Sheet>,
}

impl MwSheets for Sheets {
    fn table(&self) -> Option<&MwTable> {
        self.table.as_ref()
    }

    fn player(&self, player: PlayerId) -> Option<&Sheet> {
        self.host.as_ref().filter(|_| player == PlayerId::HOST)
    }

    fn actor(&self, _actor: ActorKey) -> Option<&Sheet> {
        None
    }
}

/// A host view that answers `function` and `player_rank` from the sheets only.
struct View {
    sheets: Sheets,
    speaker: ActorFacts,
    records: RecordDb,
}

impl View {
    fn with_sheet() -> Self {
        let t = table();
        let host = player_sheet(&t);
        Self::new(Sheets { table: Some(t), host: Some(host) })
    }

    fn new(sheets: Sheets) -> Self {
        let speaker = ActorFacts {
            id: id("guildmaster"),
            npc: true,
            faction: Some(id(FIGHTERS)),
            rank: 1,
            ..ActorFacts::default()
        };
        Self { sheets, speaker, records: RecordDb::new() }
    }
}

impl MwView for View {
    fn speaker(&self, _ctx: &ConversationCtx) -> Option<&ActorFacts> {
        Some(&self.speaker)
    }

    fn player(&self, _player: PlayerId) -> Option<&ActorFacts> {
        None
    }

    fn player_rank(&self, player: PlayerId, faction: &RecordId) -> Option<i32> {
        crate::player_rank(&self.sheets, player, faction)
    }

    fn global(&self, _id: &RecordId) -> Option<GlobalNumber> {
        None
    }

    fn local(&self, _speaker: ActorKey, _name: &RecordId) -> Option<f32> {
        None
    }

    fn quest_stage(&self, _quest: &RecordId) -> i32 {
        0
    }

    fn item_count(&self, _player: PlayerId, _item: &RecordId) -> u64 {
        0
    }

    fn dead_count(&self, _base: &RecordId) -> i32 {
        0
    }

    fn disposition(&self, _ctx: &ConversationCtx) -> i32 {
        100
    }

    fn rank_name(&self, _faction: &RecordId, _rank: i32) -> Option<String> {
        None
    }

    fn function(&self, ctx: &ConversationCtx, function: FilterFunction) -> Option<f32> {
        crate::sheet_function(&self.sheets, &self.records, ctx, &self.speaker, function)
    }
}

fn ctx(player: PlayerId) -> ConversationCtx {
    ConversationCtx { player, speaker: SPEAKER, speaker_base: id("guildmaster"), choice: None }
}

/// Whether the one INFO of a one-topic dialogue passes for `player`.
fn passes_info(view: &View, player: PlayerId, info: DialogueInfo) -> bool {
    let topic = Dialogue { id: "t".into(), dialogue_type: DialogueType2::Topic, ..Dialogue::default() };
    let dlg = MwDialogue::from_topics([(topic, vec![info])]).unwrap();
    let rules = MwDialogueRules::new(&dlg, view);
    rules.accepts(&ctx(player), &id("t"), &response_id("t", "i"))
}

fn function(f: FilterFunction, op: FilterComparison, value: i32) -> DialogueInfo {
    let mut i = info("i", "", "Line.");
    i.filters = vec![Filter {
        filter_type: FilterType::Function,
        function: f,
        comparison: op,
        value: FilterValue::Integer(value),
        ..Filter::default()
    }];
    i
}

fn passes(view: &View, f: FilterFunction, op: FilterComparison, value: i32) -> bool {
    passes_info(view, PlayerId::HOST, function(f, op, value))
}

#[test]
fn attribute_filter_reads_the_modified_value() {
    use FilterComparison::{Equal, GreaterEqual, Less};
    let v = View::with_sheet();
    assert!(passes(&v, FilterFunction::PcStrength, GreaterEqual, 50));
    assert!(!passes(&v, FilterFunction::PcStrength, GreaterEqual, 51));
    assert!(passes(&v, FilterFunction::PcStrength, Equal, 50));
    assert!(!passes(&v, FilterFunction::PcStrength, Less, 50));
    // Another attribute is the table's start value, not Strength's.
    assert!(!passes(&v, FilterFunction::PcLuck, Equal, 50));
}

#[test]
fn skill_filter_reads_its_own_skill() {
    use FilterComparison::{Equal, GreaterEqual};
    let v = View::with_sheet();
    assert!(passes(&v, FilterFunction::PcLongBlade, GreaterEqual, 40));
    assert!(!passes(&v, FilterFunction::PcLongBlade, GreaterEqual, 41));
    assert!(!passes(&v, FilterFunction::PcShortBlade, Equal, 40));
}

#[test]
fn health_percent_is_truncated_current_over_max() {
    use FilterComparison::{Equal, GreaterEqual, Less};
    let v = View::with_sheet();
    assert!(passes(&v, FilterFunction::PcHealthPercent, GreaterEqual, 75));
    assert!(!passes(&v, FilterFunction::PcHealthPercent, GreaterEqual, 76));
    assert!(!passes(&v, FilterFunction::PcHealthPercent, Less, 75));
    assert!(passes(&v, FilterFunction::PcHealth, Equal, 60));
    assert!(passes(&v, FilterFunction::PcLevel, Equal, 4));
}

#[test]
fn faction_rank_filters_read_the_players_membership() {
    use FilterComparison::Equal;
    let v = View::with_sheet();
    let ranked = |faction: &str, rank: i8| {
        let mut i = info("i", "", "Line.");
        i.player_faction = faction.into();
        i.data.player_rank = rank;
        i
    };
    assert!(passes_info(&v, PlayerId::HOST, ranked("Fighters Guild", 3)));
    assert!(!passes_info(&v, PlayerId::HOST, ranked("Fighters Guild", 4)));
    assert!(!passes_info(&v, PlayerId::HOST, ranked("Mages Guild", 0)));
    // No PC faction named: the speaker's (rank 3 in the Fighters Guild).
    assert!(passes_info(&v, PlayerId::HOST, ranked("", 3)));
    assert!(!passes_info(&v, PlayerId::HOST, ranked("", 4)));
    assert!(passes(&v, FilterFunction::SameFaction, Equal, 1));
    assert!(passes(&v, FilterFunction::PcExpelled, Equal, 0));
    // Player rank 3 minus the speaker's record rank 1.
    assert!(passes(&v, FilterFunction::FactionRankDifference, Equal, 2));
    assert!(!passes(&v, FilterFunction::FactionRankDifference, Equal, 3));
}

#[test]
fn no_sheet_is_neutral_not_a_default_character() {
    use FilterComparison::{Equal, GreaterEqual};
    let v = View::with_sheet();
    // A second co-op player without a sheet: neutral 0, never the host's values.
    let guest = PlayerId::new(1).unwrap();
    assert!(passes_info(&v, guest, function(FilterFunction::PcLevel, Equal, 0)));
    assert!(!passes_info(&v, guest, function(FilterFunction::PcLevel, GreaterEqual, 1)));
    assert!(!passes_info(&v, guest, function(FilterFunction::PcHealthPercent, Equal, 100)));
    let mut ranked = info("i", "", "Line.");
    ranked.player_faction = "Fighters Guild".into();
    ranked.data.player_rank = 0;
    assert!(!passes_info(&v, guest, ranked));
    // No stats in the run at all: the same.
    let none = View::new(Sheets { table: None, host: None });
    assert!(passes(&none, FilterFunction::PcLevel, Equal, 0));
    assert!(!passes(&none, FilterFunction::PcClothingModifier, Equal, 10));
    let dlg = MwDialogue::from_topics([(
        Dialogue { id: "t".into(), dialogue_type: DialogueType2::Topic, ..Dialogue::default() },
        vec![function(FilterFunction::PcLevel, Equal, 0)],
    )])
    .unwrap();
    let rules = MwDialogueRules::new(&dlg, &none);
    assert!(rules.accepts(&ctx(PlayerId::HOST), &id("t"), &response_id("t", "i")));
    assert_eq!(rules.neutral_functions(), [FilterFunction::PcLevel]);
}
