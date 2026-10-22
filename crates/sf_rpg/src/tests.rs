use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use smallvec::smallvec;

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn round_trip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let bytes = postcard::to_allocvec(value).expect("serialize");
    let back: T = postcard::from_bytes(&bytes).expect("deserialize");
    assert_eq!(&back, value);
}

// ---------------------------------------------------------------- ids

#[test]
fn player_id_range_and_serde() {
    assert_eq!(PlayerId::new(3).map(PlayerId::index), Some(3));
    assert!(PlayerId::new(MAX_PLAYERS).is_none());
    round_trip(&PlayerId::HOST);
    round_trip(&Holder::Actor(ActorKey(u128::MAX)));
    let bad = postcard::to_allocvec(&9u8).unwrap();
    assert!(postcard::from_bytes::<PlayerId>(&bad).is_err());
}

// ---------------------------------------------------------------- items

#[test]
fn attrs_sorted_and_float_identity() {
    let a = Attrs::new().with("z", AttrValue::Int(1)).with("a", AttrValue::Float(0.5));
    let b = Attrs::new().with("a", AttrValue::Float(0.5)).with("z", AttrValue::Int(1));
    assert_eq!(a, b);
    assert_ne!(Attrs::new().with("c", AttrValue::Float(0.0)), Attrs::new().with("c", AttrValue::Float(-0.0)));
    assert_eq!(a.get(&id("A")), Some(&AttrValue::Float(0.5)));
    round_trip(&a);
    let mut c = a.clone();
    c.set("z", AttrValue::Id(id("Soul")));
    assert_eq!(c.remove(&id("z")), Some(AttrValue::Id(id("soul"))));
    assert_eq!(c.iter().count(), 1);
}

#[test]
fn add_merges_identical_stacks_only() {
    let mut inv = Inventory::new();
    let a = inv.add(id("arrow"), 10, Attrs::new()).unwrap();
    assert_eq!(inv.add(id("Arrow"), 5, Attrs::new()).unwrap(), a);
    let worn = Attrs::new().with("condition", AttrValue::Int(40));
    let b = inv.add(id("arrow"), 1, worn.clone()).unwrap();
    assert_ne!(a, b);
    assert_eq!(inv.add(id("arrow"), 1, worn).unwrap(), b);
    assert_eq!(inv.count(&id("arrow")), 17);
    assert_eq!(inv.stacks().len(), 2);
    assert_eq!(inv.add(id("arrow"), 0, Attrs::new()), Err(InventoryError::ZeroCount));
    assert_eq!(inv.add(id("arrow"), u32::MAX, Attrs::new()), Err(InventoryError::Overflow));
}

#[test]
fn remove_prefers_plain_stacks_and_is_atomic() {
    let mut inv = Inventory::new();
    let fancy = inv.add(id("gem"), 2, Attrs::new().with("owner", AttrValue::Id(id("guild")))).unwrap();
    let plain = inv.add(id("gem"), 3, Attrs::new()).unwrap();
    let before = inv.clone();
    assert!(matches!(inv.remove(&id("gem"), 6), Err(InventoryError::NotEnough { have: 5, want: 6, .. })));
    assert_eq!(inv, before);
    assert_eq!(inv.remove(&id("gem"), 4).unwrap(), vec![(plain, 3), (fancy, 1)]);
    assert!(inv.get(plain).is_none());
    assert_eq!(inv.get(fancy).unwrap().count, 1);
}

#[test]
fn take_and_split() {
    let mut inv = Inventory::new();
    let s = inv.add(id("potion"), 5, Attrs::new()).unwrap();
    let piece = inv.split(s, 2).unwrap();
    assert_eq!(inv.stacks().iter().map(|s| s.count).collect::<Vec<_>>(), vec![3, 2]);
    assert!(inv.split(s, 3).is_err());
    let taken = inv.take(piece, 2).unwrap();
    assert_eq!((taken.id, taken.count), (piece, 2));
    assert!(inv.get(piece).is_none());
    let partial = inv.take(s, 1).unwrap();
    assert_eq!(partial.count, 1);
    assert_eq!(inv.get(s).unwrap().count, 2);
    assert_eq!(inv.take(StackId(99), 1), Err(InventoryError::NoStack(StackId(99))));
    round_trip(&inv);
}

// ---------------------------------------------------------------- equipment

struct Rules {
    table: SlotTable,
}

impl EquipRules for Rules {
    fn placement(&self, stack: &ItemStack) -> Option<EquipPlacement> {
        let slot = |n: &str| self.table.id(n).unwrap();
        match stack.item.as_str() {
            "ring" => Some(EquipPlacement { slots: smallvec![slot("finger")], whole_stack: false }),
            "sword" => Some(EquipPlacement { slots: smallvec![slot("right")], whole_stack: false }),
            "claymore" => Some(EquipPlacement { slots: smallvec![slot("right"), slot("left")], whole_stack: false }),
            "shield" => Some(EquipPlacement { slots: smallvec![slot("left")], whole_stack: false }),
            "arrow" => Some(EquipPlacement { slots: smallvec![slot("ammo")], whole_stack: true }),
            _ => None,
        }
    }
}

fn rules() -> Rules {
    Rules { table: SlotTable::new(["finger", "right", "left", "ammo"]).unwrap() }
}

#[test]
fn slot_table_validates() {
    assert_eq!(SlotTable::new(["a", "A"]), Err(SlotTableError::Duplicate(id("a"))));
    let many: Vec<String> = (0..=MAX_SLOTS).map(|i| format!("s{i}")).collect();
    assert_eq!(SlotTable::new(many), Err(SlotTableError::TooMany));
    let t = rules().table;
    assert_eq!(t.name(SlotId(2)), Some(&id("left")));
    assert_eq!(t.len(), 4);
}

#[test]
fn equip_splits_one_piece_and_merges_back() {
    let r = rules();
    let finger = r.table.id("finger").unwrap();
    let mut lo = Loadout::new();
    let rings = lo.add(id("ring"), 2, Attrs::new()).unwrap();
    let change = lo.equip(rings, &r).unwrap();
    assert_ne!(change.equipped, rings);
    assert_eq!(lo.slot(finger), Some(change.equipped));
    assert_eq!(lo.inventory().get(rings).unwrap().count, 1);
    // A new identical ring merges into the unequipped stack, not the worn one.
    assert_eq!(lo.add(id("ring"), 1, Attrs::new()).unwrap(), rings);
    assert_eq!(lo.unequip(finger), Some(id("ring")));
    assert_eq!(lo.inventory().stacks().len(), 1);
    assert_eq!(lo.count(&id("ring")), 3);
    round_trip(&lo);
}

#[test]
fn equip_displaces_and_two_handed_occupies_both() {
    let r = rules();
    let (right, left) = (r.table.id("right").unwrap(), r.table.id("left").unwrap());
    let mut lo = Loadout::new();
    let sword = lo.add(id("sword"), 1, Attrs::new()).unwrap();
    let shield = lo.add(id("shield"), 1, Attrs::new()).unwrap();
    let big = lo.add(id("claymore"), 1, Attrs::new()).unwrap();
    lo.equip(sword, &r).unwrap();
    lo.equip(shield, &r).unwrap();
    let change = lo.equip(big, &r).unwrap();
    assert_eq!(change.displaced, vec![id("sword"), id("shield")]);
    assert_eq!((lo.slot(right), lo.slot(left)), (Some(big), Some(big)));
    let change = lo.equip(shield, &r).unwrap();
    assert_eq!(change.displaced, vec![id("claymore")]);
    assert_eq!((lo.slot(right), lo.slot(left)), (None, Some(shield)));
    // Re-equipping in place is a no-op.
    assert!(lo.equip(shield, &r).unwrap().displaced.is_empty());
    let junk = lo.add(id("junk"), 1, Attrs::new()).unwrap();
    assert_eq!(lo.equip(junk, &r), Err(InventoryError::Unequippable));
}

#[test]
fn whole_stack_ammo_takes_merges_and_removal_unequips() {
    let r = rules();
    let ammo = r.table.id("ammo").unwrap();
    let mut lo = Loadout::new();
    let arrows = lo.add(id("arrow"), 20, Attrs::new()).unwrap();
    assert_eq!(lo.equip(arrows, &r).unwrap().equipped, arrows);
    assert_eq!(lo.add(id("arrow"), 5, Attrs::new()).unwrap(), arrows);
    lo.remove(&id("arrow"), 25).unwrap();
    assert_eq!(lo.slot(ammo), None);
    assert!(lo.inventory().is_empty());
}

#[test]
fn remove_prefers_unequipped() {
    let r = rules();
    let mut lo = Loadout::new();
    let rings = lo.add(id("ring"), 2, Attrs::new()).unwrap();
    let worn = lo.equip(rings, &r).unwrap().equipped;
    lo.remove(&id("ring"), 1).unwrap();
    assert!(lo.inventory().get(rings).is_none());
    assert!(lo.is_equipped(worn));
}

#[test]
fn split_and_merge_respect_equipment() {
    let r = rules();
    let (finger, ammo) = (r.table.id("finger").unwrap(), r.table.id("ammo").unwrap());
    let mut lo = Loadout::new();
    let arrows = lo.add(id("arrow"), 20, Attrs::new()).unwrap();
    lo.equip(arrows, &r).unwrap();
    // Splitting a quiver leaves the rest equipped; the piece is a plain stack after it.
    let loose = lo.split(arrows, 5).unwrap();
    assert_eq!(lo.inventory().stacks()[1].id, loose);
    assert_eq!((lo.slot(ammo), lo.inventory().get(arrows).unwrap().count), (Some(arrows), 15));
    assert!(!lo.is_equipped(loose));
    // Merging the quiver into loose arrows moves the slot with it.
    assert_eq!(lo.merge_targets(arrows).collect::<Vec<_>>(), [loose]);
    assert_eq!(lo.merge(arrows, loose).unwrap(), loose);
    assert_eq!((lo.slot(ammo), lo.inventory().get(loose).unwrap().count), (Some(loose), 20));
    assert!(lo.inventory().get(arrows).is_none());

    // A worn ring (split-equipped) neither splits nor merges; failures change nothing.
    let rings = lo.add(id("ring"), 3, Attrs::new()).unwrap();
    let worn = lo.equip(rings, &r).unwrap().equipped;
    assert_eq!(lo.slot(finger), Some(worn));
    let before = lo.clone();
    assert_eq!(lo.merge(worn, rings), Err(InventoryError::Equipped(worn)));
    assert_eq!(lo.merge(rings, worn), Err(InventoryError::Equipped(worn)));
    assert_eq!(lo.split(worn, 1), Err(InventoryError::Equipped(worn)));
    assert_eq!(lo.merge(rings, rings), Err(InventoryError::SameStack));
    assert_eq!(lo.merge(rings, loose), Err(InventoryError::NotSameKind(rings, loose)));
    assert_eq!(lo.merge(rings, StackId(99)), Err(InventoryError::NoStack(StackId(99))));
    assert!(matches!(lo.split(rings, 2), Err(InventoryError::NotEnough { .. })), "two left: split 1 at most");
    let piece = lo.split(rings, 1).unwrap();
    assert_eq!(lo.merge(piece, rings).unwrap(), rings);
    assert_eq!(lo.count(&id("ring")), before.count(&id("ring")));
    assert_eq!(lo.inventory().get(rings).unwrap().count, 2);
    // Different instance data never merges.
    let worn_out = lo.add(id("ring"), 1, Attrs::new().with("condition", AttrValue::Int(3))).unwrap();
    assert_eq!(lo.can_merge(worn_out, rings), Err(InventoryError::NotSameKind(worn_out, rings)));
    round_trip(&lo);
}

#[test]
fn stack_commands_emit_events() {
    let holder = Holder::Player(PlayerId::HOST);
    let mut pc = CharacterRpg::default();
    pc.apply(RpgCommand::AddItem { holder, item: id("gold"), count: 10, attrs: Attrs::new() }).unwrap();
    let gold = pc.loadout.inventory().stacks()[0].id;
    let split = RpgCommand::SplitStack { holder, stack: gold, count: 4 };
    assert_eq!(split.holder(), Some(holder));
    assert_eq!(WorldRpg::default().apply(split.clone()), Err(RpgError::NotAWorldCommand));
    let ev = pc.apply(split).unwrap();
    let RpgEvent::StackSplit { source, new, .. } = ev[0] else { panic!("{ev:?}") };
    assert_eq!((source, pc.loadout.inventory().get(new).unwrap().count), (gold, 4));
    let merge = RpgCommand::MergeStacks { holder, from: new, into: gold };
    assert_eq!(pc.apply(merge.clone()).unwrap(), vec![RpgEvent::StacksMerged { holder, from: new, into: gold }]);
    assert!(matches!(pc.apply(merge), Err(RpgError::Inventory(InventoryError::NoStack(_)))));
    assert_eq!(pc.loadout.count(&id("gold")), 10);
    round_trip(&RpgCommand::SplitStack { holder, stack: gold, count: 1 });
}

// ---------------------------------------------------------------- quests, journal, topics

fn draft(text: &str, day: u32) -> EntryDraft {
    EntryDraft { source: Some(id(&format!("src_{text}"))), text: text.into(), time: GameTime { day, minute: 600 } }
}

#[test]
fn quest_stages_and_flags() {
    let mut q = QuestLog::default();
    assert_eq!(q.get(&id("q")), QuestState::default());
    q.set_stage(id("q"), 10, StageFlags::default());
    let change = q.set_stage(id("Q"), 100, StageFlags { finish: true, restart: false });
    assert_eq!(change.old.stage, 10);
    assert_eq!(change.new, QuestState { stage: 100, finished: true });
    q.set_stage(id("q"), 5, StageFlags::default());
    assert_eq!(q.get(&id("q")), QuestState { stage: 5, finished: true });
    q.set_stage(id("q"), 20, StageFlags { finish: false, restart: true });
    assert!(!q.get(&id("q")).finished);
    round_trip(&q);
}

#[test]
fn journal_order_and_queries() {
    let mut j = Journal::default();
    j.add(Some(id("b")), Some(10), draft("b10", 1));
    j.add(None, None, draft("note", 1));
    j.add(Some(id("a")), Some(10), draft("a10", 2));
    let seq = j.add(Some(id("b")), Some(20), draft("b20", 3));
    assert_eq!(seq, 3);
    assert_eq!(j.quests(), vec![id("b"), id("a")]);
    assert_eq!(j.for_quest(&id("b")).map(|e| e.stage).collect::<Vec<_>>(), vec![Some(10), Some(20)]);
    assert_eq!(j.last_for_quest(&id("b")).unwrap().text, "b20");
    round_trip(&j);
}

#[test]
fn topics_learn_once_and_record_heard_lines() {
    let mut t = Topics::default();
    assert!(t.learn(id("weather")));
    assert!(!t.learn(id("Weather")));
    let line = HeardLine { source: id("r1"), speaker: "Ann".into(), text: "Rain.".into() };
    t.hear(id("rumours"), line.clone());
    t.hear(id("rumours"), line);
    assert_eq!(t.get(&id("rumours")).unwrap().heard.len(), 1);
    assert_eq!(t.in_learned_order(), vec![&id("weather"), &id("rumours")]);
    assert_eq!(t.len(), 2);
    round_trip(&t);
}

#[test]
fn world_and_character_commands() {
    let mut world = WorldRpg::default();
    let ev = world
        .apply(RpgCommand::SetStage {
            quest: id("q"),
            stage: 10,
            flags: StageFlags::default(),
            entry: Some(draft("met", 1)),
        })
        .unwrap();
    assert_eq!(ev.len(), 2);
    assert!(matches!(ev[1], RpgEvent::JournalAdded { seq: 0, .. }));
    assert_eq!(world.apply(RpgCommand::AddTopic(id("t"))).unwrap(), vec![RpgEvent::TopicLearned(id("t"))]);
    assert!(world.apply(RpgCommand::AddTopic(id("t"))).unwrap().is_empty());
    let holder = Holder::Player(PlayerId::HOST);
    let add = RpgCommand::AddItem { holder, item: id("gold"), count: 5, attrs: Attrs::new() };
    assert_eq!(add.holder(), Some(holder));
    assert_eq!(world.apply(add.clone()), Err(RpgError::NotAWorldCommand));
    let mut pc = CharacterRpg::default();
    assert_eq!(
        pc.apply(add).unwrap(),
        vec![RpgEvent::ItemsChanged { holder, item: id("gold"), delta: 5 }]
    );
    let too_many = RpgCommand::RemoveItem { holder, item: id("gold"), count: 6 };
    assert!(matches!(pc.apply(too_many), Err(RpgError::Inventory(InventoryError::NotEnough { .. }))));
    assert_eq!(pc.apply(RpgCommand::AddTopic(id("x"))), Err(RpgError::NotAnItemCommand));
    round_trip(&world);
    round_trip(&pc);
}

// ---------------------------------------------------------------- topic links

fn linker(rules: LinkRules) -> TopicLinker {
    TopicLinker::new(
        [(id("t_ring"), "ring"), (id("t_ring_heal"), "ring of healing"), (id("t_town"), "Seaside"), (id("t_e"), "")],
        rules,
    )
    .unwrap()
}

fn linked(l: &TopicLinker, text: &str) -> Vec<(String, String)> {
    l.link(text).into_iter().map(|s| (text[s.range].to_owned(), s.topic.to_string())).collect()
}

#[test]
fn links_are_leftmost_longest_and_case_insensitive() {
    let l = linker(LinkRules { word_start: true, word_end: false });
    assert_eq!(l.len(), 3);
    assert_eq!(
        linked(&l, "A RING OF HEALING from seaside, a ring."),
        vec![
            ("RING OF HEALING".into(), "t_ring_heal".into()),
            ("seaside".into(), "t_town".into()),
            ("ring".into(), "t_ring".into())
        ]
    );
}

#[test]
fn link_boundaries() {
    let start_only = linker(LinkRules { word_start: true, word_end: false });
    assert!(linked(&start_only, "bring it").is_empty());
    assert_eq!(linked(&start_only, "rings"), vec![("ring".into(), "t_ring".into())]);
    let both = linker(LinkRules { word_start: true, word_end: true });
    assert!(linked(&both, "rings").is_empty());
    // The longer match fails the end boundary; the shorter one still links.
    assert_eq!(linked(&both, "ring of healings"), vec![("ring".into(), "t_ring".into())]);
    let none = linker(LinkRules::default());
    assert_eq!(linked(&none, "bring"), vec![("ring".into(), "t_ring".into())]);
    // Non-ASCII neighbours count as letters.
    assert!(linked(&both, "éring").is_empty());
}

// ---------------------------------------------------------------- dialogue

#[derive(Clone)]
enum Cond {
    Always,
    Choice(i32),
    StageIs(&'static str, i32),
    Has(&'static str, bool),
    Speaker(&'static str),
}

struct Talk<'a> {
    conds: HashMap<RecordId, Vec<Cond>>,
    world: &'a WorldRpg,
    pc: &'a CharacterRpg,
}

impl DialogueRules for Talk<'_> {
    fn accepts(&self, ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> bool {
        self.conds.get(response).is_some_and(|cs| {
            cs.iter().all(|c| match c {
                Cond::Always => true,
                Cond::Choice(v) => ctx.choice == Some(*v),
                Cond::StageIs(q, s) => self.world.quests.stage(&id(q)) == *s,
                Cond::Has(item, yes) => (self.pc.loadout.count(&id(item)) > 0) == *yes,
                Cond::Speaker(s) => ctx.speaker_base == id(s),
            })
        })
    }

    fn render(&self, _ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> String {
        match response.as_str() {
            "g_lost" => "They took my ring!".into(),
            other => format!("<{other}>"),
        }
    }
}

fn db() -> DialogueDb {
    let mut db = DialogueDb::new();
    let topic = |kind, name: &str, rs: &[&str]| TopicEntry {
        kind,
        name: name.into(),
        responses: rs.iter().map(|r| id(r)).collect(),
    };
    db.insert_topic(id("greet0"), topic(TopicKind::Greeting, "greet0", &["g_crime"]));
    db.insert_topic(id("greet1"), topic(TopicKind::Greeting, "greet1", &["g_lost", "g_any"]));
    db.insert_topic(id("ring"), topic(TopicKind::Conversation, "ring", &["r_no", "r_lie", "r_yes", "r_ask"]));
    db.insert_topic(id("weather"), topic(TopicKind::Conversation, "weather", &["w_other"]));
    db.set_greetings(vec![id("greet0"), id("greet1")]);
    db.build_linker(LinkRules { word_start: true, word_end: false }).unwrap();
    db
}

fn conds() -> HashMap<RecordId, Vec<Cond>> {
    use Cond::{Always, Choice, Has, Speaker, StageIs};
    [
        ("g_crime", vec![StageIs("never", 1)]),
        ("g_lost", vec![Speaker("victim"), StageIs("lost", 0)]),
        ("g_any", vec![Always]),
        ("r_no", vec![Choice(1)]),
        ("r_lie", vec![Choice(2), Has("lost_ring", false)]),
        ("r_yes", vec![Choice(2), Has("lost_ring", true)]),
        ("r_ask", vec![Speaker("victim")]),
        ("w_other", vec![Speaker("someone_else")]),
    ]
    .into_iter()
    .map(|(k, v)| (id(k), v))
    .collect()
}

fn ctx() -> ConversationCtx {
    ConversationCtx { player: PlayerId::HOST, speaker: ActorKey(7), speaker_base: id("victim"), choice: None }
}

fn line_of(out: &[DialogueOutput]) -> Option<&str> {
    out.iter().find_map(|o| match o {
        DialogueOutput::Line { response, .. } => Some(response.as_str()),
        _ => None,
    })
}

#[test]
fn quest_flow_through_a_session() {
    let db = db();
    let mut world = WorldRpg::default();
    let mut pc = CharacterRpg::default();

    // Greeting: greet0 has nothing, greet1's first response matches and links "ring".
    let (mut s, out) = DialogueSession::open(&db, &Talk { conds: conds(), world: &world, pc: &pc }, ctx());
    assert_eq!(line_of(&out), Some("g_lost"));
    assert!(out.contains(&DialogueOutput::Learned(id("ring"))));
    assert_eq!(out.last(), Some(&DialogueOutput::RunResult { topic: id("greet1"), response: id("g_lost") }));
    world.apply(RpgCommand::AddTopic(id("ring"))).unwrap();
    world.apply(RpgCommand::AddTopic(id("weather"))).unwrap();
    {
        let talk = Talk { conds: conds(), world: &world, pc: &pc };
        // "weather" is known but this speaker has nothing on it.
        assert_eq!(s.available_topics(&db, &talk, &world.topics), vec![id("ring")]);
        let out = s.ask(&db, &talk, &id("ring")).unwrap();
        assert_eq!(line_of(&out), Some("r_ask"));
    }
    // The result script: stage 10 + choices.
    world
        .apply(RpgCommand::SetStage { quest: id("lost"), stage: 10, flags: StageFlags::default(), entry: Some(draft("met", 1)) })
        .unwrap();
    s.offer_choices(vec![Choice { label: "No".into(), value: 1 }, Choice { label: "Yes".into(), value: 2 }]).unwrap();
    {
        let talk = Talk { conds: conds(), world: &world, pc: &pc };
        assert_eq!(s.ask(&db, &talk, &id("ring")), Err(DialogueError::ChoicePending));
        assert_eq!(s.choose(&db, &talk, 3), Err(DialogueError::InvalidChoice(3)));
        assert_eq!(line_of(&s.choose(&db, &talk, 2).unwrap()), Some("r_lie"));
        assert_eq!(s.ctx().choice, None);
        assert_eq!(s.choose(&db, &talk, 2), Err(DialogueError::NotChoosing));
    }
    // Find the ring, ask again, say yes.
    pc.apply(RpgCommand::AddItem {
        holder: Holder::Player(PlayerId::HOST),
        item: id("lost_ring"),
        count: 1,
        attrs: Attrs::new(),
    })
    .unwrap();
    s.offer_choices(vec![Choice { label: "Yes".into(), value: 2 }]).unwrap();
    {
        let talk = Talk { conds: conds(), world: &world, pc: &pc };
        assert_eq!(line_of(&s.choose(&db, &talk, 2).unwrap()), Some("r_yes"));
    }
    // Its result: finish the quest, take the ring, then goodbye.
    world
        .apply(RpgCommand::SetStage {
            quest: id("lost"),
            stage: 100,
            flags: StageFlags { finish: true, restart: false },
            entry: Some(draft("returned", 1)),
        })
        .unwrap();
    pc.apply(RpgCommand::RemoveItem { holder: Holder::Player(PlayerId::HOST), item: id("lost_ring"), count: 1 })
        .unwrap();
    s.force_goodbye();
    {
        let talk = Talk { conds: conds(), world: &world, pc: &pc };
        assert_eq!(s.ask(&db, &talk, &id("ring")), Err(DialogueError::GoodbyeOnly));
    }
    s.goodbye();
    assert_eq!(s.state(), &SessionState::Ended);
    assert_eq!(s.history().len(), 4);
    assert_eq!(world.quests.get(&id("lost")), QuestState { stage: 100, finished: true });
    assert_eq!(world.journal.entries().len(), 2);
    assert_eq!(pc.loadout.count(&id("lost_ring")), 0);
    round_trip(&s);
    round_trip(&world);
    round_trip(&pc);

    // Next visit: the quest is past stage 0, so the generic greeting answers.
    let talk = Talk { conds: conds(), world: &world, pc: &pc };
    let (_, out) = DialogueSession::open(&db, &talk, ctx());
    assert_eq!(line_of(&out), Some("g_any"));
}

#[test]
fn no_greeting_and_unknown_topic() {
    let db = db();
    let world = WorldRpg::default();
    let pc = CharacterRpg::default();
    let talk = Talk { conds: HashMap::new(), world: &world, pc: &pc };
    let (mut s, out) = DialogueSession::open(&db, &talk, ctx());
    assert_eq!(out, vec![DialogueOutput::NoResponse { topic: None }]);
    assert_eq!(s.ask(&db, &talk, &id("nope")), Err(DialogueError::UnknownTopic(id("nope"))));
    assert_eq!(s.ask(&db, &talk, &id("ring")).unwrap(), vec![DialogueOutput::NoResponse { topic: Some(id("ring")) }]);
    assert_eq!(s.offer_choices(Vec::new()), Err(DialogueError::NoChoices));
    s.goodbye();
    assert_eq!(s.ask(&db, &talk, &id("ring")), Err(DialogueError::Ended));
}

#[test]
fn one_conversation_per_speaker() {
    let mut c = Conversations::default();
    let p1 = PlayerId::new(1).unwrap();
    c.claim(ActorKey(1), PlayerId::HOST).unwrap();
    c.claim(ActorKey(1), PlayerId::HOST).unwrap();
    assert_eq!(c.claim(ActorKey(1), p1), Err(DialogueError::Busy { speaker: ActorKey(1), holder: PlayerId::HOST }));
    c.claim(ActorKey(2), p1).unwrap();
    c.release_player(PlayerId::HOST);
    assert_eq!(c.holder(ActorKey(1)), None);
    c.claim(ActorKey(1), p1).unwrap();
    c.release(ActorKey(2));
    assert_eq!(c.holder(ActorKey(2)), None);
    round_trip(&c);
}
