//! Interpreter fixtures: one synthetic script per builtin and per control-flow form (our
//! own scripts, never vanilla text).

use std::collections::HashMap;

use crate::ast::{Call, Ident, Target, VarRef, VarType};
use crate::functions::Function;
use crate::interp::{Host, Interpreter, Locals, RunError, format_message, store};
use crate::{Env, FunctionTable, MemberLookup, SourceKind, parse_and_resolve};

/// Records every host call as a line of text; variables and counts live in maps.
#[derive(Default)]
struct Recorder {
    log: Vec<String>,
    globals: HashMap<String, (VarType, f64)>,
    speaker: HashMap<String, (VarType, f64)>,
    stages: HashMap<String, i32>,
    items: HashMap<(String, String), i32>,
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
    fn message_box(&mut self, text: String, buttons: Vec<String>) {
        self.log.push(format!("message_box {text:?} {buttons:?}"));
    }
    fn play_sound(&mut self, sound: &Ident) {
        self.log.push(format!("play_sound {}", sound.key));
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
fn message_box_formats_and_buttons() {
    let host = result(
        "MessageBox \"You have %g gems, %.1f%% done (%d).\" state 12.345 7.9\nMessageBox \"Go?\" \"Yes\" \"No\"",
    );
    assert_eq!(
        host.log,
        [
            r#"message_box "You have 0 gems, 12.3% done (7)." []"#,
            r#"message_box "Go?" ["Yes", "No"]"#
        ]
    );
}

#[test]
fn message_format_rules() {
    assert_eq!(
        format_message("%g %g %g", &[0.5, 1e7, 123_456.7]),
        "0.5 1e+07 123457"
    );
    assert_eq!(
        format_message("%f|%.0f|%G", &[1.5, 2.5, 0.0001]),
        "1.500000|2|0.0001"
    );
    assert_eq!(format_message("%x %", &[]), "%x %");
    assert_eq!(format_message("%g", &[]), "0", "missing values read 0");
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
