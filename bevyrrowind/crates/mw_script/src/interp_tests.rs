//! Interpreter fixtures: one synthetic script per builtin and per control-flow form (our
//! own scripts, never vanilla text).

use std::collections::HashMap;

use crate::ast::{Call, Ident, Target, VarRef, VarType};
use crate::functions::Function;
use crate::interp::{Host, Interpreter, Locals, MessageArg, RunError, ScriptOp, store};
use crate::aifn::AiCall;
use crate::statfn::StatCall;
use crate::{Env, FunctionTable, MemberLookup, SourceKind, parse_and_resolve};

/// Records every host call as a line of text; variables and counts live in maps.
#[derive(Default)]
struct Recorder {
    log: Vec<String>,
    globals: HashMap<String, (VarType, f64)>,
    speaker: HashMap<String, (VarType, f64)>,
    stages: HashMap<String, i32>,
    items: HashMap<(String, String), i32>,
    /// Answers stat functions (logged, value 7); off: they fall through to `unhandled`.
    stats: bool,
}

fn who(target: &Target) -> String {
    match target {
        Target::Implicit => "self".into(),
        Target::Explicit(id) => id.key.clone(),
    }
}

impl Recorder {
    fn slot(&mut self, var: &VarRef) -> Option<&mut (VarType, f64)> {
        match var {
            VarRef::Global(n) => self.globals.get_mut(&n.key),
            VarRef::Speaker(n) => self.speaker.get_mut(&n.key),
            _ => None,
        }
    }
}

impl Host for Recorder {
    fn get(&mut self, var: &VarRef) -> f64 {
        if let VarRef::Member { object, var } = var {
            self.log.push(format!("get {}.{}", object.key, var.key));
            return 7.0;
        }
        self.slot(var).map_or(0.0, |s| s.1)
    }
    fn set(&mut self, var: &VarRef, value: f64) {
        if let Some(s) = self.slot(var) {
            s.1 = store(s.0, value);
        } else {
            self.log.push(format!("set {var:?} {value}"));
        }
    }
    fn journal(&mut self, quest: &Ident, index: i32) {
        let stage = self.stages.entry(quest.key.clone()).or_default();
        *stage = (*stage).max(index);
        self.log.push(format!("journal {} {index}", quest.key));
    }
    fn set_journal_index(&mut self, quest: &Ident, index: i32) {
        self.stages.insert(quest.key.clone(), index);
        self.log
            .push(format!("set_journal_index {} {index}", quest.key));
    }
    fn get_journal_index(&mut self, quest: &Ident) -> i32 {
        self.stages.get(&quest.key).copied().unwrap_or(0)
    }
    fn add_topic(&mut self, topic: &Ident) {
        self.log.push(format!("add_topic {}", topic.key));
    }
    fn choice(&mut self, choices: Vec<(String, i32)>) {
        self.log.push(format!("choice {choices:?}"));
    }
    fn goodbye(&mut self) {
        self.log.push("goodbye".into());
    }
    fn add_item(&mut self, target: &Target, item: &Ident, count: i32) {
        *self
            .items
            .entry((who(target), item.key.clone()))
            .or_default() += count;
        self.log
            .push(format!("add_item {} {} {count}", who(target), item.key));
    }
    fn remove_item(&mut self, target: &Target, item: &Ident, count: i32) {
        *self
            .items
            .entry((who(target), item.key.clone()))
            .or_default() -= count;
        self.log
            .push(format!("remove_item {} {} {count}", who(target), item.key));
    }
    fn get_item_count(&mut self, target: &Target, item: &Ident) -> i32 {
        self.items
            .get(&(who(target), item.key.clone()))
            .copied()
            .unwrap_or(0)
    }
    fn set_disposition(&mut self, target: &Target, value: i32) {
        self.log
            .push(format!("set_disposition {} {value}", who(target)));
    }
    fn mod_disposition(&mut self, target: &Target, delta: i32) {
        self.log
            .push(format!("mod_disposition {} {delta}", who(target)));
    }
    fn message_box(&mut self, format: &str, args: &[MessageArg]) {
        self.log.push(format!("message_box {format:?} {args:?}"));
    }
    fn play_sound(&mut self, sound: &Ident) {
        self.log.push(format!("play_sound {}", sound.key));
    }
    fn stat(&mut self, target: &Target, call: &StatCall) -> Option<f64> {
        if !self.stats {
            return None;
        }
        let faction = call.faction.as_ref().map_or("-", |f| f.key.as_str());
        self.log.push(format!(
            "stat {} {:?} {} {faction}",
            who(target),
            call.function,
            call.value
        ));
        Some(7.0)
    }
    /// Answers AI functions when `stats` is on (logged; reads value 3), like `stat`.
    fn ai(&mut self, target: &Target, call: &AiCall) -> Option<f64> {
        if !self.stats {
            return None;
        }
        self.log.push(format!("ai {} {call:?}", who(target)));
        Some(3.0)
    }
    /// Answers script control when `stats` is on (logged, value 1), like `stat`.
    fn script(&mut self, target: &Target, op: ScriptOp, script: &Ident) -> Option<f64> {
        if !self.stats {
            return None;
        }
        self.log.push(format!("script {} {op:?} {}", who(target), script.key));
        Some(1.0)
    }
    fn unhandled(&mut self, function: &Function, _call: &Call) -> f64 {
        self.log.push(format!("unhandled {}", function.name));
        f64::from(function.status.neutral())
    }
}

/// Knows the globals `gold_short` (short) and `ratio` (float).
struct Globals;

impl Env for Globals {
    fn global(&self, key: &str) -> Option<VarType> {
        match key {
            "gold_short" => Some(VarType::Short),
            "ratio" => Some(VarType::Float),
            _ => None,
        }
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        Some(true)
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Found(VarType::Short)
    }
}

fn recorder() -> Recorder {
    let mut r = Recorder::default();
    r.globals.insert("gold_short".into(), (VarType::Short, 0.0));
    r.globals.insert("ratio".into(), (VarType::Float, 0.0));
    r.speaker.insert("state".into(), (VarType::Short, 0.0));
    r
}

/// Runs `text` as `kind` against `host`; returns the final locals.
fn run_as(text: &str, kind: SourceKind, host: &mut Recorder) -> Result<Locals, RunError> {
    let table = FunctionTable::vanilla();
    let parsed = parse_and_resolve(text, kind, table, &Globals);
    assert!(!parsed.is_inert(), "{:?}", parsed.diagnostics);
    let mut locals = Locals::new(&parsed.script);
    Interpreter::new(table).run(&parsed.script, &mut locals, host)?;
    Ok(locals)
}

fn result(text: &str) -> Recorder {
    let mut host = recorder();
    run_as(text, SourceKind::DialogueResult, &mut host).unwrap();
    host
}

fn script(text: &str) -> (Recorder, Locals) {
    let mut host = recorder();
    let locals = run_as(text, SourceKind::Script, &mut host).unwrap();
    (host, locals)
}

#[test]
fn locals_convert_on_assignment() {
    let (_, l) = script(
        "Begin t\nshort s\nlong l\nfloat f\nset s to 40000\nset l to -7 / 2\nset f to 0.1\nEnd",
    );
    assert!((l.0[0] - (40000.0 - 65536.0)).abs() < f64::EPSILON);
    assert!(
        (l.0[1] + 3.0).abs() < f64::EPSILON,
        "truncation toward zero"
    );
    assert!((l.0[2] - f64::from(0.1f32)).abs() < f64::EPSILON);
}

#[test]
fn store_matches_the_luau_runtime() {
    assert!((store(VarType::Short, 32768.0) + 32768.0).abs() < f64::EPSILON);
    assert!((store(VarType::Short, -32769.0) - 32767.0).abs() < f64::EPSILON);
    assert!((store(VarType::Long, 2_147_483_648.0) + 2_147_483_648.0).abs() < f64::EPSILON);
    assert!((store(VarType::Long, -2.9) + 2.0).abs() < f64::EPSILON);
    assert!(store(VarType::Long, f64::NAN).abs() < f64::EPSILON);
    assert!(store(VarType::Short, f64::INFINITY).abs() < f64::EPSILON);
}

#[test]
fn globals_and_speaker_locals_go_through_the_host() {
    let host = result("set gold_short to 70000\nset ratio to 1 / 3\nset state to state - 1");
    assert!((host.globals["gold_short"].1 - 4464.0).abs() < f64::EPSILON);
    assert!((host.globals["ratio"].1 - f64::from(1.0f32 / 3.0)).abs() < f64::EPSILON);
    assert!((host.speaker["state"].1 + 1.0).abs() < f64::EPSILON);
}

#[test]
fn member_reads_reach_the_host() {
    let (host, l) = script("Begin t\nshort x\nset x to other_obj.count + 1\nEnd");
    assert_eq!(host.log, ["get other_obj.count"]);
    assert!((l.0[0] - 8.0).abs() < f64::EPSILON);
}

#[test]
fn if_elseif_else_take_one_arm() {
    let text = |v: i32| {
        format!(
            "Begin t\nshort v\nshort out\nset v to {v}\nif ( v == 1 )\nset out to 10\nelseif ( v >= 2 )\nset out to 20\nelseif ( v == 2 )\nset out to 99\nelse\nset out to 30\nendif\nEnd"
        )
    };
    for (v, out) in [(1, 10.0), (2, 20.0), (5, 20.0), (0, 30.0)] {
        let (_, l) = script(&text(v));
        assert!((l.0[1] - out).abs() < f64::EPSILON, "v = {v}");
    }
}

#[test]
fn if_without_else_and_nested() {
    let (_, l) = script(
        "Begin t\nshort a\nshort b\nif ( a == 0 )\nif ( b != 0 )\nset a to 5\nelse\nset a to 6\nendif\nendif\nif ( a > 100 )\nset b to 1\nendif\nEnd",
    );
    assert_eq!(l.0, [6.0, 0.0]);
}

#[test]
fn while_loops_and_return() {
    let (_, l) = script(
        "Begin t\nshort i\nshort sum\nwhile ( i < 5 )\nset i to i + 1\nset sum to sum + i\nendwhile\nEnd",
    );
    assert_eq!(l.0, [5.0, 15.0]);
    let (_, l) = script(
        "Begin t\nshort i\nwhile ( i < 10 )\nset i to i + 1\nif ( i == 3 )\nreturn\nendif\nendwhile\nset i to 100\nEnd",
    );
    assert_eq!(l.0, [3.0], "return leaves the loop and the script");
}

#[test]
fn runaway_while_stops_at_the_step_limit() {
    let table = FunctionTable::vanilla();
    let parsed = parse_and_resolve(
        "Begin t\nshort i\nwhile ( 1 )\nset i to i + 1\nendwhile\nEnd",
        SourceKind::Script,
        table,
        &Globals,
    );
    let mut interp = Interpreter::new(table);
    interp.step_limit = 1000;
    let mut locals = Locals::new(&parsed.script);
    let err = interp
        .run(&parsed.script, &mut locals, &mut recorder())
        .unwrap_err();
    assert_eq!(err, RunError::StepLimit { line: 3 });
}

#[test]
fn journal_set_and_get_index() {
    let host = result(
        "Journal \"q_test\" 10\nif ( GetJournalIndex q_test >= 10 )\nAddTopic \"next steps\"\nendif\nSetJournalIndex q_test 5\nif ( GetJournalIndex q_test < 10 )\nGoodbye\nendif",
    );
    assert_eq!(
        host.log,
        [
            "journal q_test 10",
            "add_topic next steps",
            "set_journal_index q_test 5",
            "goodbye"
        ]
    );
}

#[test]
fn choice_pairs_labels_and_values() {
    let host = result("Choice \"No.\" 1 \"Yes, here.\" 2 \"Later\" ( state + 3 )");
    assert_eq!(
        host.log,
        [r#"choice [("No.", 1), ("Yes, here.", 2), ("Later", 3)]"#]
    );
}

#[test]
fn items_with_targets() {
    let mut host = recorder();
    host.items.insert(("player".into(), "gem".into()), 2);
    run_as(
        "if ( player->GetItemCount \"gem\" > 1 )\nplayer->RemoveItem \"gem\" 2\nAddItem gem 2\nendif\n\"other_npc\"->AddItem gold_001 5\nset state to GetItemCount gem",
        SourceKind::DialogueResult,
        &mut host,
    )
    .unwrap();
    assert_eq!(
        host.log,
        [
            "remove_item player gem 2",
            "add_item self gem 2",
            "add_item other_npc gold_001 5"
        ]
    );
    assert!((host.speaker["state"].1 - 2.0).abs() < f64::EPSILON);
}

#[test]
fn dispositions() {
    let host = result("ModDisposition -5\n\"arrille\"->ModDisposition 40\nSetDisposition 70.9");
    assert_eq!(
        host.log,
        [
            "mod_disposition self -5",
            "mod_disposition arrille 40",
            "set_disposition self 70"
        ]
    );
}

#[test]
fn message_box_hands_values_and_buttons_to_the_host() {
    let host = result(
        "MessageBox \"You have %g gems, %.1f%% done (%d).\" state 12.345 7.9\nMessageBox \"Go?\" \"Yes\" \"No\"",
    );
    assert_eq!(
        host.log,
        [
            r#"message_box "You have %g gems, %.1f%% done (%d)." [Num(0.0), Num(12.345), Num(7.9)]"#,
            r#"message_box "Go?" [Str("Yes"), Str("No")]"#
        ]
    );
}

#[test]
fn message_box_keeps_bare_words_as_words() {
    // OpenMW issue 8129: after the values, bare words (keywords and numbers included) are
    // potential buttons kept as written; only quoted strings count as buttons (mw_ui decides).
    let host = result(
        "MessageBox \"Pick\" A B \"C\" D \"E\"\nMessageBox \"Nums\" 1 -2 .todd, \"A\"\n\
         MessageBox \"Keys\" set state \"set2\"",
    );
    assert_eq!(
        host.log,
        [
            r#"message_box "Pick" [Word("A"), Word("B"), Str("C"), Word("D"), Str("E")]"#,
            r#"message_box "Nums" [Word("1"), Word("-2"), Word(".todd"), Str("A")]"#,
            r#"message_box "Keys" [Word("set"), Word("state"), Str("set2")]"#
        ]
    );
}

#[test]
fn message_box_text_placeholders_take_strings_and_words() {
    // A `%s` takes a quoted string (not a button) or a bare word as text; number placeholders
    // take expressions; `%%` and unknown conversions take nothing.
    let host = result(
        "MessageBox \"Go %s?\" \"north\"\nMessageBox \"Hi %s, %d%% %q %5.1f\" Fargoth 2 4.5 \"Ok\"",
    );
    assert_eq!(
        host.log,
        [
            r#"message_box "Go %s?" [Str("north")]"#,
            r#"message_box "Hi %s, %d%% %q %5.1f" [Word("Fargoth"), Num(2.0), Num(4.5), Str("Ok")]"#
        ]
    );
}

#[test]
fn play_sound_and_unhandled_functions() {
    let host =
        result("PlaySound \"Item Gold Up\"\nModPCFacRep 5 \"Temple\"\nset state to GetDisposition");
    assert_eq!(
        host.log,
        [
            "play_sound item gold up",
            "unhandled ModPCFacRep",
            "unhandled GetDisposition"
        ]
    );
    assert!(
        (host.speaker["state"].1 - 50.0).abs() < f64::EPSILON,
        "stub=50 neutral value"
    );
}

#[test]
fn undeclared_variables_read_zero_and_drop_writes() {
    let (host, l) = script("Begin t\nshort a\nset nothing to 5\nset a to nothing + 1\nEnd");
    assert!(host.log.is_empty(), "{:?}", host.log);
    assert_eq!(l.0, [1.0]);
}

#[test]
fn stat_functions_reach_the_host_typed() {
    let mut host = recorder();
    host.stats = true;
    run_as(
        "set state to GetStrength\nplayer->ModLongBlade 5\nSetHealth ( state + 1 )\n\
         ModPCFacRep 3 \"Fighters Guild\"\nPCRaiseRank\nset state to GetPCRank \"Mages Guild\"\n\
         ModCurrentFatigue -2\nset state to GetMagickaGetRatio",
        SourceKind::DialogueResult,
        &mut host,
    )
    .unwrap();
    assert_eq!(
        host.log,
        [
            "stat self GetAttribute(0) 0 -",
            "stat player ModSkill(5) 5 -",
            "stat self SetDynamic(Health) 8 -",
            "stat self ModPcFacRep 3 fighters guild",
            "stat self PcRaiseRank 0 -",
            "stat self GetPcRank 0 mages guild",
            "stat self ModCurrentDynamic(Fatigue) -2 -",
            "stat self GetDynamicRatio(Magicka) 0 -",
        ]
    );
    assert!(
        (host.speaker["state"].1 - 7.0).abs() < f64::EPSILON,
        "a stat read returns the host's value"
    );
}

#[test]
fn every_stat_function_is_a_live_table_function() {
    let t = FunctionTable::vanilla();
    let all = crate::StatFn::all();
    for (name, f) in &all {
        let row = t.function(t.get(name).unwrap_or_else(|| panic!("{name}")));
        assert_eq!(
            (row.status, crate::StatFn::from_name(&row.name)),
            (crate::Status::Live, Some(*f)),
            "{name}: functions.txt status column"
        );
    }
    let mut names: Vec<String> = all.iter().map(|(n, _)| n.to_lowercase()).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), 152, "8 x 3 attribute + 27 x 3 skill + 3 x 5 dynamic + 4 x 3 AI setting + 20 others");
}

#[test]
fn every_ai_function_is_a_live_table_function() {
    let t = FunctionTable::vanilla();
    for a in crate::AiFn::ALL {
        let f = t.function(t.get(a.name()).unwrap_or_else(|| panic!("{a:?}")));
        assert_eq!(
            (f.name.as_str(), f.status, crate::AiFn::from_name(&f.name)),
            (a.name(), crate::Status::Live, Some(a)),
            "functions.txt status column"
        );
    }
}

/// `OpenMW`'s argument reading (`aiextensions.cpp`): the reset argument's presence makes a
/// package repeat, escort hours truncate, wander drops the `Idle` chance, clamps
/// `Idle2`-`Idle9` and takes the argument after them as reset; reads return the host's value.
#[test]
fn ai_functions_reach_the_host_typed() {
    let mut host = recorder();
    host.stats = true;
    run_as(
        "AITravel 100 -200.5 30
player->AITravel 1 2 3 1
         AIEscort \"fargoth\" 5.9 10 20 30
AIEscortCell fargoth \"Seyda Neen\" 2 1 2 3 0
         AIFollow player 1.5 0 0 0
AIFollowCell player \"Balmora\" 0 4 5 6
         AIWander 512 5 0
AIWander 128 0 6 99 40 300 -3
         AIWander 64 1 2 0 10 20 30 40 50 60 70 80 0 77
         set state to GetAIPackageDone
set state to state + GetCurrentAIPackage",
        SourceKind::DialogueResult,
        &mut host,
    )
    .unwrap();
    let ai = |s: &str| format!("ai {s}");
    assert_eq!(
        host.log,
        [
            ai("self Travel { to: [100.0, -200.5, 30.0], repeat: false }"),
            ai("player Travel { to: [1.0, 2.0, 3.0], repeat: true }"),
            ai(r#"self Escort { actor: Ident { key: "fargoth", text: "fargoth" }, cell: None, hours: 5, to: [10.0, 20.0, 30.0], repeat: false }"#),
            ai(r#"self Escort { actor: Ident { key: "fargoth", text: "fargoth" }, cell: Some(Ident { key: "seyda neen", text: "Seyda Neen" }), hours: 2, to: [1.0, 2.0, 3.0], repeat: true }"#),
            ai(r#"self Follow { actor: Ident { key: "player", text: "player" }, cell: None, hours: 1.5, to: [0.0, 0.0, 0.0], repeat: false }"#),
            ai(r#"self Follow { actor: Ident { key: "player", text: "player" }, cell: Some(Ident { key: "balmora", text: "Balmora" }), hours: 0.0, to: [4.0, 5.0, 6.0], repeat: false }"#),
            ai("self Wander { range: 512, hours: 5, hour: 0, idles: [], repeat: false }"),
            ai("self Wander { range: 128, hours: 0, hour: 6, idles: [40, 255, 0], repeat: true }"),
            ai("self Wander { range: 64, hours: 1, hour: 2, idles: [10, 20, 30, 40, 50, 60, 70, 80], repeat: false }"),
            ai("self PackageDone"),
            ai("self CurrentPackage"),
        ]
    );
    assert!((host.speaker["state"].1 - 6.0).abs() < f64::EPSILON, "reads return the host's value");
}

#[test]
fn every_builtin_is_a_live_table_function() {
    let t = FunctionTable::vanilla();
    for b in crate::Builtin::ALL {
        let f = t.function(t.get(b.name()).unwrap_or_else(|| panic!("{b:?}")));
        assert_eq!(
            (f.name.as_str(), f.status),
            (b.name(), crate::Status::Live),
            "functions.txt status column"
        );
    }
}

/// `Binding::of` is the interpreter's real dispatch: for every table function, a bare call
/// reaches `Host::unhandled` exactly when its binding is `Unbound`, and `Host::stat` when it
/// is `Stat`.
#[test]
fn binding_matches_the_dispatch_for_every_function() {
    use crate::ast::{Script, Stmt, StmtKind};
    use crate::functions::FnId;
    use crate::interp::Binding;

    let t = FunctionTable::vanilla();
    let interp = Interpreter::new(t);
    let mut bound = 0;
    for (i, f) in t.functions.iter().enumerate() {
        let call = Call {
            target: Target::Implicit,
            function: FnId(u16::try_from(i).unwrap()),
            args: Vec::new(),
            line: 1,
        };
        let script = Script {
            kind: SourceKind::DialogueResult,
            name: None,
            locals: Vec::new(),
            body: vec![Stmt { kind: StmtKind::Call(call), line: 1 }],
        };
        let mut host = Recorder { stats: true, ..Recorder::default() };
        let mut l = Locals::new(&script);
        interp.run(&script, &mut l, &mut host).unwrap();
        let binding = Binding::of(&f.name);
        let unhandled = host.log.iter().any(|e| e.starts_with("unhandled "));
        assert_eq!(unhandled, binding == Binding::Unbound, "{}: {binding:?} {:?}", f.name, host.log);
        if let Binding::Stat(_) = binding {
            assert!(host.log.first().is_some_and(|e| e.starts_with("stat ")), "{}", f.name);
        }
        if let Binding::Builtin(
            crate::Builtin::StartScript | crate::Builtin::StopScript | crate::Builtin::ScriptRunning,
        ) = binding
        {
            assert!(host.log.first().is_some_and(|e| e.starts_with("script ")), "{}", f.name);
        }
        if let Binding::Ai(_) = binding {
            assert!(host.log.first().is_some_and(|e| e.starts_with("ai ")), "{}", f.name);
        }
        bound += usize::from(binding.is_bound());
    }
    assert_eq!(
        bound,
        152 + crate::AiFn::ALL.len() + crate::Builtin::ALL.len(),
        "stat functions + AI package functions + builtins"
    );
}

/// DQ-2 counts: bound / total by family and origin from parsed fixtures.
#[test]
fn bound_report_counts_by_family_and_origin() {
    use crate::coverage::{CallSites, Origin};

    let t = FunctionTable::vanilla();
    let mut sites = CallSites::default();
    let result = parse_and_resolve(
        "Journal \"a_quest\" 10\nset gold_short to Random 5\nModStrength 1\n",
        SourceKind::DialogueResult,
        t,
        &Globals,
    );
    sites.add_parsed(&result, Origin::DialogueResult, t);
    let script = parse_and_resolve(
        "Begin s\nif ( Random 10 > 5 )\nJournal \"a_quest\" 20\nendif\nEnd s\n",
        SourceKind::Script,
        t,
        &Globals,
    );
    sites.add_parsed(&script, Origin::Script, t);
    let report = sites.report(t);
    let all = report.bound();
    assert_eq!(
        (all.scripts, all.scripts_bound, all.results, all.results_bound, all.via_stat),
        (2, 1, 3, 2, 1)
    );
    let random = report.rows.iter().find(|r| r.function == "Random").unwrap();
    assert!(!random.binding.is_bound());
    let family = |name: &str| {
        report
            .bound_by_family()
            .into_iter()
            .find(|(f, _)| f == name)
            .unwrap()
            .1
    };
    let journal = t.function(t.get("Journal").unwrap()).family.clone();
    let j = family(&journal);
    assert_eq!((j.total(), j.bound()), (2, 2));
    let text = report.bound_report(5).to_string();
    assert!(text.contains("TOTAL") && text.contains("Random"), "{text}");
}
