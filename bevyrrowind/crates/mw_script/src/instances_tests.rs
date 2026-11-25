//! Global instance fixtures (DQ-4a): our own scripts, never vanilla text.

#![allow(clippy::float_cmp, reason = "locals hold exactly stored integers")]

use std::collections::HashMap;

use crate::ast::{Ident, Target, VarRef, VarType};
use crate::instances::{
    GlobalInstances, GlobalsSnapshot, InstanceHost, LoadReport, LocalType, Report, ScriptSet,
};
use crate::interp::{Host, Interpreter, MessageArg, store};
use crate::{Env, FunctionTable, MemberLookup};

/// Globals `g` (long) and `seen` (long); `AddTopic` logs the topic, runs log begin/end.
#[derive(Default)]
struct Game {
    log: Vec<String>,
    globals: HashMap<String, f64>,
}

impl Host for Game {
    fn get(&mut self, var: &VarRef) -> f64 {
        match var {
            VarRef::Global(n) => self.globals.get(&n.key).copied().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    fn set(&mut self, var: &VarRef, value: f64) {
        if let VarRef::Global(n) = var {
            self.globals.insert(n.key.clone(), store(VarType::Long, value));
        }
    }
    fn journal(&mut self, _: &Ident, _: i32) {}
    fn set_journal_index(&mut self, _: &Ident, _: i32) {}
    fn get_journal_index(&mut self, _: &Ident) -> i32 {
        0
    }
    fn add_topic(&mut self, topic: &Ident) {
        self.log.push(topic.key.clone());
    }
    fn choice(&mut self, _: Vec<(String, i32)>) {}
    fn goodbye(&mut self) {}
    fn add_item(&mut self, _: &Target, _: &Ident, _: i32) {}
    fn remove_item(&mut self, _: &Target, _: &Ident, _: i32) {}
    fn get_item_count(&mut self, _: &Target, _: &Ident) -> i32 {
        0
    }
    fn set_disposition(&mut self, _: &Target, _: i32) {}
    fn mod_disposition(&mut self, _: &Target, _: i32) {}
    fn message_box(&mut self, _: &str, _: &[MessageArg]) {}
    fn play_sound(&mut self, _: &Ident) {}
}

impl InstanceHost for Game {
    fn begin_run(&mut self, script: &str, target: Option<&str>) {
        self.log.push(format!("begin {script} {}", target.unwrap_or("-")));
    }
    fn end_run(&mut self, script: &str, committed: bool) {
        if !committed {
            self.log.push(format!("dropped {script}"));
        }
    }
}

struct Globals;

impl Env for Globals {
    fn global(&self, key: &str) -> Option<VarType> {
        matches!(key, "g" | "seen").then_some(VarType::Long)
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        Some(true)
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Found(VarType::Short)
    }
}

fn set_of(scripts: &[(&str, &str)]) -> ScriptSet {
    let mut set = ScriptSet::default();
    for (id, text) in scripts {
        assert!(set.parse_insert(id, text, FunctionTable::vanilla(), &Globals), "{id} inert");
    }
    set
}

/// The topics the runs logged (begin lines dropped).
fn topics(game: &Game) -> Vec<&str> {
    game.log
        .iter()
        .filter(|l| !l.starts_with("begin "))
        .map(String::as_str)
        .collect()
}

/// A script that logs its name each run.
fn logger(name: &str) -> String {
    format!("Begin {name}\nAddTopic \"{name}\"\nEnd {name}\n")
}

fn local(g: &GlobalInstances, id: &str, slot: usize) -> f64 {
    g.get(id).unwrap().locals.0[slot]
}

#[test]
fn globals_run_in_start_order_and_see_earlier_writes() {
    let set = set_of(&[
        ("c_scr", "Begin c_scr\nset g to g * 10\nAddTopic \"c\"\nEnd\n"),
        ("a_scr", "Begin a_scr\nset g to g + 1\nAddTopic \"a\"\nEnd\n"),
        ("b_scr", "Begin b_scr\nset seen to g\nAddTopic \"b\"\nEnd\n"),
    ]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    for id in ["c_scr", "a_scr", "b_scr"] {
        g.start(&set, id, None);
    }
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    assert_eq!(topics(&game), ["c", "a", "b"]);
    // c: 0 * 10, a: 0 + 1, b reads a's write.
    assert_eq!((game.globals["g"], game.globals["seen"]), (1.0, 1.0));
    assert_eq!(
        g.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
        ["c_scr", "a_scr", "b_scr"]
    );
}

#[test]
fn a_script_started_mid_tick_first_runs_next_tick() {
    let starter = "Begin a_scr\nshort done\nif ( done == 0 )\nStartScript z_scr\nset done to 1\nendif\nAddTopic \"a\"\nEnd\n";
    let set = set_of(&[("a_scr", starter), ("z_scr", &logger("z"))]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&set, "A_Scr", Some("fargoth"));
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    assert_eq!(topics(&game), ["a"]);
    assert!(g.is_running("z_scr"));
    // The implicit StartScript target is the starter's own target.
    assert_eq!(g.get("z_scr").unwrap().target.as_deref(), Some("fargoth"));
    game.log.clear();
    g.tick(&set, &interp, &mut game);
    assert_eq!(game.log, ["begin a_scr fargoth", "a", "begin z_scr fargoth", "z"]);
}

#[test]
fn start_on_a_running_script_is_a_no_op() {
    let set = set_of(&[("a_scr", &logger("a")), ("b_scr", &logger("b"))]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&set, "a_scr", Some("first"));
    g.start(&set, "b_scr", None);
    g.start(&set, "a_scr", Some("second"));
    let a = g.get("a_scr").unwrap();
    assert_eq!((a.start_seq, a.target.as_deref()), (0, Some("first")));
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    assert_eq!(topics(&game), ["a", "b"]);
}

#[test]
fn stop_then_start_keeps_locals_and_moves_to_the_end() {
    let counter = "Begin a_scr\nshort n\nset n to n + 1\nAddTopic \"a\"\nEnd\n";
    let set = set_of(&[("a_scr", counter), ("b_scr", &logger("b"))]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    let mut game = Game::default();
    g.start(&set, "a_scr", None);
    g.start(&set, "b_scr", None);
    g.tick(&set, &interp, &mut game);
    g.tick(&set, &interp, &mut game);
    g.stop("a_scr");
    assert!(!g.is_running("a_scr"));
    g.tick(&set, &interp, &mut game);
    assert_eq!(local(&g, "a_scr", 0), 2.0);
    g.start(&set, "a_scr", Some("t"));
    game.log.clear();
    g.tick(&set, &interp, &mut game);
    assert_eq!(local(&g, "a_scr", 0), 3.0);
    assert_eq!(topics(&game), ["b", "a"], "a restart takes a new start sequence");
    assert_eq!(g.get("a_scr").unwrap().target.as_deref(), Some("t"));
}

#[test]
fn stop_script_in_its_own_run_stops_after_that_run() {
    let text = "Begin a_scr\nshort r1\nshort r2\nset r1 to ScriptRunning a_scr\nStopScript a_scr\nset r2 to ScriptRunning a_scr\nAddTopic \"a\"\nEnd\n";
    let set = set_of(&[("a_scr", text)]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&set, "a_scr", None);
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    assert_eq!(topics(&game), ["a"], "the run finished after StopScript");
    assert_eq!((local(&g, "a_scr", 0), local(&g, "a_scr", 1)), (1.0, 0.0));
    assert!(!g.is_running("a_scr"));
    g.tick(&set, &interp, &mut game);
    assert_eq!(topics(&game), ["a"]);
}

#[test]
fn script_running_reads_the_flag() {
    let probe = "Begin p_scr\nshort ra\nshort rb\nshort rx\nshort rc\nset ra to ScriptRunning a_scr\nset rb to ScriptRunning b_scr\nset rx to ScriptRunning nothere\nStartScript c_scr\nset rc to ScriptRunning c_scr\nStopScript b_scr\nEnd\n";
    let set = set_of(&[
        ("p_scr", probe),
        ("a_scr", &logger("a")),
        ("b_scr", &logger("b")),
        ("c_scr", &logger("c")),
    ]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&set, "a_scr", None);
    g.stop("a_scr");
    g.start(&set, "p_scr", None);
    g.start(&set, "b_scr", None);
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    let slots: Vec<f64> = (0..4).map(|s| local(&g, "p_scr", s)).collect();
    assert_eq!(slots, [0.0, 1.0, 0.0, 1.0]);
    // b was stopped by the earlier p run: it doesn't run this tick; c runs from the next.
    assert_eq!(topics(&game), Vec::<&str>::new());
    assert!(!g.is_running("b_scr") && g.is_running("c_scr"));
}

#[test]
fn step_limit_drops_the_run_stops_and_reports_once() {
    let runaway = "Begin w_scr\nshort i\nStartScript b_scr\nwhile ( 1 )\nset i to i + 1\nendwhile\nEnd\n";
    let set = set_of(&[("w_scr", runaway), ("b_scr", &logger("b"))]);
    let mut interp = Interpreter::new(FunctionTable::vanilla());
    interp.step_limit = 1000;
    let mut g = GlobalInstances::default();
    g.start(&set, "w_scr", None);
    let mut game = Game::default();
    g.tick(&set, &interp, &mut game);
    g.tick(&set, &interp, &mut game);
    assert_eq!(
        g.take_reports(),
        [Report::StepLimit { script: "w_scr".into(), line: 5 }]
    );
    assert!(!g.is_running("w_scr"));
    assert!(g.get("b_scr").is_none(), "the faulted run's StartScript was dropped");
    assert_eq!(game.log, ["begin w_scr -", "dropped w_scr"]);
    g.start(&set, "w_scr", None);
    g.tick(&set, &interp, &mut game);
    assert_eq!(g.take_reports().len(), 1, "a restart may fault again");
}

#[test]
fn unknown_and_inert_starts_are_reported() {
    let mut set = set_of(&[("a_scr", &logger("a"))]);
    let broken = crate::parse("Begin x\nFrobnicate 3\nEnd x\n", crate::SourceKind::Script, FunctionTable::vanilla());
    assert!(!set.insert("Bad_Scr", broken));
    assert_eq!(set.inert().map(|(id, _)| id).collect::<Vec<_>>(), ["bad_scr"]);
    let mut g = GlobalInstances::default();
    g.start(&set, "bad_scr", None);
    g.start(&set, "missing", None);
    assert_eq!(
        g.take_reports(),
        [
            Report::InertScript { script: "bad_scr".into() },
            Report::UnknownScript { script: "missing".into() },
        ]
    );
    assert!(g.get("bad_scr").is_none() && g.get("missing").is_none());
}

#[test]
fn save_round_trip() {
    let text = "Begin a_scr\nshort s\nlong l\nfloat f\nset s to s + 1\nset l to 70000\nset f to 0.5\nEnd\n";
    let set = set_of(&[("a_scr", text), ("b_scr", &logger("b"))]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&set, "b_scr", None);
    g.start(&set, "a_scr", Some("ref_1"));
    g.tick(&set, &interp, &mut Game::default());
    g.stop("b_scr");
    let snap = g.snapshot(&set);
    let json = serde_json::to_string(&snap).unwrap();
    let back: GlobalsSnapshot = serde_json::from_str(&json).unwrap();
    assert_eq!(back, snap);
    let a = &snap.globals[1];
    assert_eq!((a.id.as_str(), a.running, a.target.as_deref(), a.start_seq), ("a_scr", true, Some("ref_1"), 1));
    assert_eq!(
        a.locals.iter().map(|l| (l.name.as_str(), l.ty, l.value)).collect::<Vec<_>>(),
        [("s", LocalType::Short, 1.0), ("l", LocalType::Long, 70000.0), ("f", LocalType::Float, 0.5)]
    );
    let (loaded, reports) = GlobalInstances::load(&back, &set);
    assert!(reports.is_empty(), "{reports:?}");
    assert_eq!(loaded.iter().collect::<Vec<_>>(), g.iter().collect::<Vec<_>>());
    // Start order and the sequence counter survive: a new start goes last.
    let mut loaded = loaded;
    loaded.start(&set, "b_scr", None);
    assert_eq!(loaded.get("b_scr").unwrap().start_seq, 2);
}

#[test]
fn load_remaps_locals_by_name() {
    let old = set_of(&[
        ("a_scr", "Begin a_scr\nshort keep\nshort gone\nlong wide\nset keep to 5\nset gone to 6\nset wide to 40000\nEnd\n"),
        ("old_scr", &logger("o")),
    ]);
    let interp = Interpreter::new(FunctionTable::vanilla());
    let mut g = GlobalInstances::default();
    g.start(&old, "a_scr", None);
    g.start(&old, "old_scr", None);
    g.tick(&old, &interp, &mut Game::default());
    let snap = g.snapshot(&old);
    // The mod changed a_scr (new local first, `gone` removed, `wide` now short) and removed old_scr.
    let new = set_of(&[(
        "a_scr",
        "Begin a_scr\nshort fresh\nshort keep\nshort wide\nset fresh to fresh\nEnd\n",
    )]);
    let (loaded, reports) = GlobalInstances::load(&snap, &new);
    assert_eq!(loaded.get("a_scr").unwrap().locals.0, [0.0, 5.0, store(VarType::Short, 40000.0)]);
    assert!(loaded.get("old_scr").is_none());
    let s = |x: &str| x.to_owned();
    assert_eq!(
        reports,
        [
            LoadReport::MissingLocal { script: s("a_scr"), local: s("fresh") },
            LoadReport::Retyped {
                script: s("a_scr"),
                local: s("wide"),
                from: LocalType::Long,
                to: LocalType::Short,
            },
            LoadReport::DroppedLocal { script: s("a_scr"), local: s("gone") },
            LoadReport::UnknownScript { script: s("old_scr") },
        ]
    );
}
