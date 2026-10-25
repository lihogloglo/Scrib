//! A scripted conversation: a test-double response database and rules, a small host that
//! applies requests, runs result scripts and feeds the view, and input from the "pad".

use std::collections::HashMap;

use sf_rpg::{
    ActorKey, ConversationCtx, DialogueDb, DialogueOutput, DialogueRules, DialogueSession,
    EntryDraft, GameTime, HeardLine, LinkRules, PlayerId, RecordId, RpgCommand, StageFlags,
    TopicEntry, TopicKind, WorldRpg,
};
use sf_ui::{UiValue, ViewModel};

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}
// ---------------------------------------------------------------- test double

enum Cond {
    Always,
    Choice(i32),
    Speaker(&'static str),
}

/// Result-script steps keyed by response.
enum Op {
    Offer(&'static [(&'static str, i32)]),
    Stage(&'static str, i32, &'static str),
    ForceGoodbye,
}

struct Talk<'a> {
    conds: &'a HashMap<RecordId, Vec<Cond>>,
}

impl DialogueRules for Talk<'_> {
    fn accepts(&self, ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> bool {
        self.conds.get(response).is_some_and(|cs| {
            cs.iter().all(|c| match c {
                Cond::Always => true,
                Cond::Choice(v) => ctx.choice == Some(*v),
                Cond::Speaker(s) => ctx.speaker_base == id(s),
            })
        })
    }

    fn render(&self, _ctx: &ConversationCtx, _topic: &RecordId, response: &RecordId) -> String {
        match response.as_str() {
            "g_victim" => "They took my ring! Ask about the weather, or the Mages Guild.",
            "r_ask" => "My ring, stolen. Will you find it?",
            "r_yes" => "Thank you! Try the Mages Guild.",
            "r_no" => "Pity.",
            "mg_any" => "The Mages Guild is in the background of every story.",
            "bg_any" => "Nothing to tell.",
            other => panic!("no text for {other}"),
        }
        .into()
    }
}

fn db(linked: bool) -> DialogueDb {
    let mut db = DialogueDb::new();
    let topic = |kind, name: &str, rs: &[&str]| TopicEntry {
        kind,
        name: name.into(),
        responses: rs.iter().map(|r| id(r)).collect(),
    };
    db.insert_topic(
        id("greeting"),
        topic(TopicKind::Greeting, "Greeting", &["g_victim"]),
    );
    db.insert_topic(
        id("ring"),
        topic(TopicKind::Conversation, "Ring", &["r_yes", "r_no", "r_ask"]),
    );
    db.insert_topic(
        id("mages guild"),
        topic(TopicKind::Conversation, "Mages Guild", &["mg_any"]),
    );
    db.insert_topic(
        id("background"),
        topic(TopicKind::Conversation, "background", &["bg_any"]),
    );
    db.insert_topic(
        id("weather"),
        topic(TopicKind::Conversation, "weather", &["w_other"]),
    );
    db.set_greetings(vec![id("greeting")]);
    if linked {
        db.build_linker(LinkRules {
            word_start: true,
            word_end: true,
        })
        .unwrap();
    }
    db
}

fn conds() -> HashMap<RecordId, Vec<Cond>> {
    use Cond::{Always, Choice, Speaker};
    [
        ("g_victim", vec![Speaker("victim")]),
        ("r_yes", vec![Choice(2)]),
        ("r_no", vec![Choice(1)]),
        ("r_ask", vec![Always]),
        ("mg_any", vec![Always]),
        ("bg_any", vec![Always]),
        ("w_other", vec![Speaker("someone_else")]),
    ]
    .into_iter()
    .map(|(k, v)| (id(k), v))
    .collect()
}

fn script() -> HashMap<RecordId, Vec<Op>> {
    [
        ("r_ask", vec![Op::Offer(&[("No", 1), ("Yes", 2)])]),
        (
            "r_yes",
            vec![
                Op::Stage("lost", 10, "I agreed to find the ring."),
                Op::Stage("guild", 1, "Try the guild."),
            ],
        ),
        ("bg_any", vec![Op::ForceGoodbye]),
    ]
    .into_iter()
    .map(|(k, v)| (id(k), v))
    .collect()
}

/// The host side: owns the state, applies requests, runs result scripts, feeds the view.
struct Host {
    db: DialogueDb,
    conds: HashMap<RecordId, Vec<Cond>>,
    script: HashMap<RecordId, Vec<Op>>,
    world: WorldRpg,
    session: DialogueSession,
    view: DialogueView,
}

impl Host {
    fn open() -> Self {
        Self::open_with(db(true), DialogueView::new("Victim"))
    }

    fn open_with(db: DialogueDb, view: DialogueView) -> Self {
        let mut world = WorldRpg::default();
        // Known before this conversation: asked someone about "background"; heard of the weather.
        let line = HeardLine {
            source: id("bg_other"),
            speaker: "Someone".into(),
            text: "...".into(),
        };
        world
            .apply(RpgCommand::HearLine {
                topic: id("background"),
                line,
            })
            .unwrap();
        world.apply(RpgCommand::AddTopic(id("weather"))).unwrap();
        let conds = conds();
        let ctx = ConversationCtx {
            player: PlayerId::HOST,
            speaker: ActorKey(7),
            speaker_base: id("victim"),
            choice: None,
        };
        let (session, out) = DialogueSession::open(&db, &Talk { conds: &conds }, ctx);
        let mut host = Self {
            db,
            conds,
            script: script(),
            world,
            session,
            view,
        };
        host.run(out);
        host
    }

    /// Input → request → session → view.
    fn input(&mut self, nav: DialogueNav) -> Option<DialogueRequest> {
        let request = self.view.nav(nav);
        if let Some(request) = &request {
            self.send(request);
        }
        request
    }

    fn send(&mut self, request: &DialogueRequest) {
        let out = request
            .apply(&mut self.session, &self.db, &Talk { conds: &self.conds })
            .unwrap();
        self.run(out);
    }

    fn run(&mut self, out: Vec<DialogueOutput>) {
        self.view.apply_outputs(&self.db, &out);
        for o in out {
            match o {
                DialogueOutput::Line {
                    topic,
                    response,
                    text,
                } => {
                    let line = HeardLine {
                        source: response,
                        speaker: "Victim".into(),
                        text,
                    };
                    self.world
                        .apply(RpgCommand::HearLine { topic, line })
                        .unwrap();
                }
                DialogueOutput::Learned(topic) => {
                    self.world.apply(RpgCommand::AddTopic(topic)).unwrap();
                }
                DialogueOutput::RunResult { response, .. } => {
                    for op in self.script.get(&response).into_iter().flatten() {
                        match op {
                            Op::Offer(choices) => {
                                let choices = choices.iter().map(|(l, v)| sf_rpg::Choice {
                                    label: (*l).into(),
                                    value: *v,
                                });
                                self.session.offer_choices(choices.collect()).unwrap();
                            }
                            Op::Stage(quest, stage, text) => {
                                let entry = EntryDraft {
                                    source: None,
                                    text: (*text).into(),
                                    time: GameTime::default(),
                                };
                                let cmd = RpgCommand::SetStage {
                                    quest: id(quest),
                                    stage: *stage,
                                    flags: StageFlags::default(),
                                    entry: Some(entry),
                                };
                                let events = self.world.apply(cmd).unwrap();
                                self.view.apply_events(&events);
                            }
                            Op::ForceGoodbye => self.session.force_goodbye(),
                        }
                    }
                }
                DialogueOutput::NoResponse { .. } => {}
            }
        }
        self.refresh();
    }

    fn refresh(&mut self) {
        self.view.refresh_from(
            &self.session,
            &self.db,
            &Talk { conds: &self.conds },
            &self.world.topics,
        );
    }
}

// ---------------------------------------------------------------- helpers

fn line(view: &DialogueView, entry: EntryId) -> &ViewLine {
    match &view.entry(entry).expect("entry").kind {
        EntryKind::Line(line) => line,
        other => panic!("not a line: {other:?}"),
    }
}

/// (text, link topic, state) per run.
fn runs(line: &ViewLine) -> Vec<(&str, Option<(&str, LinkState)>)> {
    line.runs
        .iter()
        .map(|r| {
            (
                r.text.as_str(),
                r.link.as_ref().map(|l| (l.topic.as_str(), l.state)),
            )
        })
        .collect()
}

fn order(view: &mut DialogueView) -> Vec<&str> {
    view.topic_order().iter().map(RecordId::as_str).collect()
}

fn row(view: &DialogueView, topic: &str) -> TopicRow {
    view.topics().get(&id(topic)).expect("row").clone()
}

fn topic_focus(topic: &str) -> DialogueFocus {
    DialogueFocus::Topic(id(topic))
}

fn link(entry: u64, run: usize) -> DialogueFocus {
    DialogueFocus::Link {
        entry: EntryId(entry),
        run,
    }
}

// ---------------------------------------------------------------- tests

#[test]
fn greeting_links_and_sorted_topic_list() {
    let mut h = Host::open();
    let v = &mut h.view;
    assert_eq!(v.entries().len(), 1);
    let greeting = line(v, EntryId(0));
    assert_eq!(greeting.heading, None);
    assert_eq!(
        greeting.text(),
        "They took my ring! Ask about the weather, or the Mages Guild."
    );
    assert_eq!(
        runs(greeting),
        vec![
            ("They took my ", None),
            ("ring", Some(("ring", LinkState::New))),
            ("! Ask about the ", None),
            ("weather", Some(("weather", LinkState::Unavailable))),
            (", or the ", None),
            ("Mages Guild", Some(("mages guild", LinkState::New))),
            (".", None),
        ]
    );
    // Learned from the greeting (ring, mages guild) + known before (background); weather is
    // known but this speaker has nothing on it. Sorted by name, case-insensitively.
    assert_eq!(order(v), vec!["background", "mages guild", "ring"]);
    assert!(!row(v, "background").new, "heard before, from someone else");
    assert!(row(v, "ring").new && row(v, "mages guild").new);
    assert!(row(v, "ring").enabled);
    assert_eq!(v.focus(), Some(&topic_focus("background")));
    assert_eq!(v.take_scroll_request(), Some(EntryId(0)));
    assert_eq!(v.take_scroll_request(), None);
    assert_eq!(v.mode(), DialogueMode::Talking);
    assert!(v.can_goodbye() && v.links_active());
    assert_eq!(v.field("speaker"), Some(UiValue::Text("Victim".into())));
    assert_eq!(row(v, "ring").field("new"), Some(UiValue::Bool(true)));
    assert_eq!(TopicRow::KIND, "dialogue.topic");
}

#[test]
#[allow(clippy::too_many_lines)] // one scripted conversation, start to goodbye
fn gamepad_drives_a_quest_from_topic_to_goodbye() {
    use DialogueNav::{Back, Confirm, Next, NextRegion, Prev, PrevRegion};
    let mut h = Host::open();

    // Topic list: down twice to "Ring", confirm.
    assert_eq!(h.input(Next), None);
    assert_eq!(h.input(Next), None);
    assert_eq!(h.view.focus(), Some(&topic_focus("ring")));
    assert_eq!(h.input(Confirm), Some(DialogueRequest::Ask(id("ring"))));
    let asked = line(&h.view, EntryId(1));
    assert_eq!(asked.heading.as_deref(), Some("Ring"));
    assert_eq!(runs(asked)[1], ("ring", Some(("ring", LinkState::Known))));
    assert!(!row(&h.view, "ring").new);
    assert_eq!(
        runs(line(&h.view, EntryId(0)))[1].1,
        Some(("ring", LinkState::Known)),
        "old links restyle"
    );

    // The result offered choices: only they are live.
    assert_eq!(h.view.mode(), DialogueMode::Choosing);
    assert_eq!(h.view.field("mode"), Some(UiValue::Text("choosing".into())));
    assert_eq!(
        h.view
            .choices()
            .iter()
            .map(|c| c.label.as_str())
            .collect::<Vec<_>>(),
        vec!["No", "Yes"]
    );
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Choice(0)));
    assert!(!row(&h.view, "ring").enabled && !h.view.can_goodbye() && !h.view.links_active());
    assert_eq!(h.input(Back), None, "a choice must be made");
    assert_eq!(h.input(PrevRegion), None);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Choice(0)));
    assert_eq!(h.view.activate(DialogueFocus::Topic(id("ring"))), None);
    assert_eq!(h.view.activate(DialogueFocus::Goodbye), None);
    h.input(Next);
    h.input(Next);
    assert_eq!(
        h.view.focus(),
        Some(&DialogueFocus::Choice(1)),
        "clamped, no wrap"
    );
    assert_eq!(h.input(Confirm), Some(DialogueRequest::Choose(2)));

    // Echo, answer, one journal notice for both entries the result added.
    let kinds: Vec<&EntryKind> = h.view.entries().skip(2).map(|e| &e.kind).collect();
    assert_eq!(
        kinds[0],
        &EntryKind::Chosen {
            label: "Yes".into()
        }
    );
    assert!(matches!(kinds[1], EntryKind::Line(l) if l.response == id("r_yes")));
    assert_eq!(
        kinds[2],
        &EntryKind::JournalUpdated {
            quests: vec![id("lost"), id("guild")]
        }
    );
    assert_eq!(kinds.len(), 3);
    assert_eq!(h.world.quests.stage(&id("lost")), 10);
    assert_eq!(
        h.view.take_scroll_request(),
        Some(EntryId(3)),
        "the new line, not the notice"
    );

    // Talking again: focus returns to the remembered topic.
    assert_eq!(h.view.mode(), DialogueMode::Talking);
    assert_eq!(h.view.focus(), Some(&topic_focus("ring")));

    // Link cursor: into the text on the newest line with a live link, then back in reading
    // order (the unavailable "weather" link is skipped).
    h.input(PrevRegion);
    assert_eq!(h.view.focus(), Some(&link(3, 1)));
    assert_eq!(h.view.take_scroll_request(), Some(EntryId(3)));
    h.input(Prev);
    assert_eq!(h.view.focus(), Some(&link(1, 1)));
    h.input(Prev);
    assert_eq!(h.view.focus(), Some(&link(0, 5)));
    assert_eq!(h.view.take_scroll_request(), Some(EntryId(0)));
    assert_eq!(
        h.input(Confirm),
        Some(DialogueRequest::Ask(id("mages guild")))
    );
    let guild = line(&h.view, EntryId(5));
    assert_eq!(
        runs(guild),
        vec![
            ("The ", None),
            ("Mages Guild", Some(("mages guild", LinkState::Known))),
            (" is in the ", None),
            ("background", Some(("background", LinkState::Known))),
            (" of every story.", None),
        ]
    );
    assert_eq!(
        h.view.focus(),
        Some(&link(0, 5)),
        "following a link keeps the cursor"
    );
    assert!(h.view.entries().all(|e| {
        match &e.kind {
            EntryKind::Line(l) => l
                .runs
                .iter()
                .filter_map(|r| r.link.as_ref())
                .all(|l| l.state != LinkState::New),
            _ => true,
        }
    }));

    // Back to the list: remembered row; goodbye sits below the last topic.
    h.input(NextRegion);
    assert_eq!(h.view.focus(), Some(&topic_focus("ring")));
    h.input(Next);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));
    h.input(Next);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));
    for _ in 0..3 {
        h.input(Prev);
    }
    assert_eq!(h.view.focus(), Some(&topic_focus("background")));

    // "background" forces goodbye: only goodbye is live, and back takes it.
    assert_eq!(
        h.input(Confirm),
        Some(DialogueRequest::Ask(id("background")))
    );
    assert_eq!(h.view.mode(), DialogueMode::GoodbyeOnly);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));
    assert!(!row(&h.view, "background").enabled);
    h.input(Prev);
    h.input(PrevRegion);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));
    assert_eq!(h.view.activate(DialogueFocus::Topic(id("ring"))), None);
    assert_eq!(h.input(Back), Some(DialogueRequest::Goodbye));
    assert!(h.view.is_closed());
    assert_eq!(h.view.focus(), None);
    assert_eq!(h.input(Confirm), None);
    assert_eq!(h.input(Back), None);
}

#[test]
fn pointer_search_and_versions() {
    let mut h = Host::open();
    // Clicking a row asks it and moves the focus there.
    assert_eq!(
        h.view.activate(DialogueFocus::Topic(id("mages guild"))),
        Some(DialogueRequest::Ask(id("mages guild")))
    );
    h.send(&DialogueRequest::Ask(id("mages guild")));
    assert_eq!(h.view.focus(), Some(&topic_focus("mages guild")));
    // Unavailable links and unknown targets are not live.
    assert_eq!(
        h.view.activate(link(0, 3)),
        None,
        "weather: nothing from this speaker"
    );
    assert_eq!(h.view.activate(link(0, 0)), None, "plain run");
    assert_eq!(h.view.activate(link(99, 1)), None);
    assert!(!h.view.set_focus(DialogueFocus::Choice(0)));
    assert!(
        h.view.set_focus(link(1, 3)),
        "hover on a link moves the cursor"
    );
    assert!(h.view.set_focus(DialogueFocus::Topic(id("ring"))));

    // Search: the list filters; a filtered-out focus lands on the row now at its index.
    h.view.set_topic_search("GU");
    assert_eq!(order(&mut h.view), vec!["mages guild"]);
    assert_eq!(h.view.focus(), Some(&topic_focus("mages guild")));
    assert_eq!(
        h.view.activate(DialogueFocus::Topic(id("ring"))),
        None,
        "hidden by the search"
    );
    h.view.nav(DialogueNav::Next);
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));
    h.view.set_topic_search("");
    assert_eq!(
        order(&mut h.view),
        vec!["background", "mages guild", "ring"]
    );
    assert_eq!(h.view.focus(), Some(&DialogueFocus::Goodbye));

    // A refresh with nothing new changes nothing; any change moves the version.
    let v0 = h.view.version();
    h.refresh();
    assert_eq!(h.view.version(), v0);
    h.view.nav(DialogueNav::Prev);
    assert!(h.view.version() > v0);
    assert_eq!(
        h.view.activate(DialogueFocus::Goodbye),
        Some(DialogueRequest::Goodbye)
    );
}

#[test]
fn scrollback_is_capped_and_focus_survives_trimming() {
    let mut h = Host::open_with(db(true), DialogueView::new("Victim").with_max_entries(3));
    let ask = DialogueRequest::Ask(id("mages guild"));
    h.send(&ask);
    h.send(&ask);
    assert_eq!(
        h.view.entries().map(|e| e.id.0).collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    assert!(
        h.view.set_focus(link(1, 3)),
        "\"background\" in the first guild line"
    );
    h.view.take_scroll_request();
    h.send(&ask);
    h.send(&ask);
    assert_eq!(
        h.view.entries().map(|e| e.id.0).collect::<Vec<_>>(),
        vec![2, 3, 4]
    );
    assert!(h.view.entry(EntryId(1)).is_none());
    assert_eq!(line(&h.view, EntryId(4)).response, id("mg_any"));
    assert_eq!(
        h.view.focus(),
        Some(&topic_focus("background")),
        "the trimmed link's cursor falls back to the list"
    );
    assert_eq!(h.view.take_scroll_request(), Some(EntryId(4)));
}

#[test]
fn unlinked_text_and_no_greeting() {
    // No linker: one plain run per line.
    let mut h = Host::open_with(db(false), DialogueView::new("Victim"));
    assert_eq!(
        runs(line(&h.view, EntryId(0))),
        vec![(
            "They took my ring! Ask about the weather, or the Mages Guild.",
            None
        )]
    );
    // Without a linker nothing was learned from the greeting: only "background" is listed.
    assert_eq!(order(&mut h.view), vec!["background"]);
    assert_eq!(h.input(DialogueNav::PrevRegion), None);
    assert_eq!(
        h.view.focus(),
        Some(&topic_focus("background")),
        "no links: the cursor stays"
    );

    // A speaker with no greeting: empty scrollback, focus on goodbye when nothing is listed.
    let db = db(true);
    let conds = conds();
    let ctx = ConversationCtx {
        player: PlayerId::HOST,
        speaker: ActorKey(8),
        speaker_base: id("other"),
        choice: None,
    };
    let (session, out) = DialogueSession::open(&db, &Talk { conds: &conds }, ctx);
    let mut view = DialogueView::new("Other");
    view.apply_outputs(&db, &out);
    view.refresh_from(
        &session,
        &db,
        &Talk { conds: &conds },
        &WorldRpg::default().topics,
    );
    assert_eq!(view.entries().len(), 0);
    assert_eq!(view.focus(), Some(&DialogueFocus::Goodbye));
    assert_eq!(
        view.nav(DialogueNav::Confirm),
        Some(DialogueRequest::Goodbye)
    );
}
