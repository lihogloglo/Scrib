//! Local instance fixtures (DQ-5a): our own scripts, never vanilla text.

#![allow(clippy::float_cmp, reason = "locals hold exactly stored integers")]

use std::collections::{HashMap, HashSet};

use crate::ast::{Call, Ident, Target, VarRef, VarType};
use crate::functions::Function;
use crate::instances::{GlobalInstances, InstanceHost, LoadReport, ScriptSet};
use crate::interp::{Host, Interpreter, MessageArg, store};
use crate::locals::{LocalInstances, LocalKey, LocalReport, LocalsSnapshot};
use crate::{Env, FunctionTable, MemberLookup};

const P2: LocalKey = LocalKey::Placement(2);
const P9: LocalKey = LocalKey::Placement(9);
const S11: LocalKey = LocalKey::Stack { holder: 1, stack: 1 };

/// Global `g` (long); `AddTopic` logs the topic; `Disable` / `Enable` / `GetDisabled` act on the
/// running local's object; runs log `begin <script> <key>`.
#[derive(Default)]
struct Game {
    log: Vec<String>,
    globals: HashMap<String, f64>,
    running: Option<LocalKey>,
    disabled: HashSet<LocalKey>,
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
    fn unhandled(&mut self, function: &Function, _: &Call) -> f64 {
        let key = self.running.expect("a local run");
        match function.name.as_str() {
            "Disable" => {
                self.disabled.insert(key);
            }
            "Enable" => {
                self.disabled.remove(&key);
            }
            "GetDisabled" => return f64::from(u8::from(self.disabled.contains(&key))),
            other => panic!("unexpected {other}"),
        }
        0.0
    }
}

impl InstanceHost for Game {
    fn begin_run(&mut self, script: &str, target: Option<&str>) {
        self.log.push(format!("begin {script} {}", target.unwrap_or("-")));
    }
    fn end_run(&mut self, script: &str, committed: bool) {
        self.running = None;
        if !committed {
            self.log.push(format!("dropped {script}"));
        }
    }
    fn begin_local_run(&mut self, script: &str, key: LocalKey) {
        self.running = Some(key);
        self.log.push(format!("begin {script} {key:?}"));
    }
    fn local_self(&mut self, key: LocalKey) -> Option<String> {
        match key {
            LocalKey::Placement(id) => Some(format!("obj_{id}")),
            LocalKey::Stack { .. } => None,
        }
    }
}

struct Globals;

impl Env for Globals {
    fn global(&self, key: &str) -> Option<VarType> {
        (key == "g").then_some(VarType::Long)
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

/// What the runs logged, begin lines dropped.
fn topics(game: &Game) -> Vec<&str> {
    game.log
        .iter()
        .filter(|l| !l.starts_with("begin "))
        .map(String::as_str)
        .collect()
}

/// Counts its runs in local `n`.
const COUNTER: &str = "Begin cnt_scr\nshort n\nset n to n + 1\nEnd\n";

fn interp() -> Interpreter<'static> {
    Interpreter::new(FunctionTable::vanilla())
}

/// One game tick: globals, then locals (the spec's order).
fn tick(set: &ScriptSet, g: &mut GlobalInstances, l: &mut LocalInstances, game: &mut Game) {
    let interp = interp();
    g.tick(set, &interp, game);
    l.tick(set, &interp, g, game);
}

#[test]
fn locals_run_in_key_order_after_globals_and_see_earlier_writes() {
    let set = set_of(&[
        ("glob_scr", "Begin glob_scr\nset g to 1\nAddTopic \"glob\"\nEnd\n"),
        ("a_scr", "Begin a_scr\nset g to g * 10\nAddTopic \"a\"\nEnd\n"),
        ("b_scr", "Begin b_scr\nset g to g + 3\nAddTopic \"b\"\nEnd\n"),
    ]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    g.start(&set, "glob_scr", None);
    // Inserted out of order: the tick runs placements by id, then stacks.
    assert!(l.insert(&set, S11, "a_scr"));
    assert!(l.insert(&set, P9, "b_scr"));
    assert!(l.insert(&set, P2, "A_Scr"));
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!(
        game.log,
        [
            "begin glob_scr -",
            "glob",
            "begin a_scr Placement(2)",
            "a",
            "begin b_scr Placement(9)",
            "b",
            "begin a_scr Stack { holder: 1, stack: 1 }",
            "a",
        ]
    );
    // ((1 * 10) + 3) * 10: each run read the previous run's write.
    assert_eq!(game.globals["g"], 130.0);
    assert_eq!(l.iter().map(|(k, _)| k).collect::<Vec<_>>(), [P2, P9, S11]);
}

#[test]
fn insert_starts_zeroed_and_insert_twice_keeps_locals() {
    let set = set_of(&[("cnt_scr", COUNTER)]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    assert!(l.insert(&set, P2, "cnt_scr"));
    assert_eq!(l.get(P2).unwrap().locals.0, [0.0]);
    tick(&set, &mut g, &mut l, &mut game);
    tick(&set, &mut g, &mut l, &mut game);
    assert!(!l.insert(&set, P2, "CNT_SCR"), "already present: a no-op");
    assert_eq!(l.local(&set, P2, "N"), Some(2.0));
    assert!(l.insert(&set, P9, "cnt_scr"));
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!((l.local(&set, P2, "n"), l.local(&set, P9, "n")), (Some(3.0), Some(1.0)));
    assert_eq!(l.local(&set, P2, "missing"), None);
}

#[test]
fn take_and_restore_keep_locals_across_a_remove() {
    let set = set_of(&[("cnt_scr", COUNTER)]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, P2, "cnt_scr");
    l.insert(&set, P9, "cnt_scr");
    tick(&set, &mut g, &mut l, &mut game);
    tick(&set, &mut g, &mut l, &mut game);
    // P2 leaves the region (stashed with its placement), P9 is gone for good.
    let stash = l.take(&set, P2).unwrap();
    assert!(l.remove(P9));
    assert!(l.is_empty() && l.take(&set, P2).is_none() && !l.remove(P9));
    tick(&set, &mut g, &mut l, &mut game);
    // Back in the region: restored, not re-inserted.
    assert!(l.restore(&set, &stash).is_empty());
    assert!(l.insert(&set, P9, "cnt_scr"));
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!((l.local(&set, P2, "n"), l.local(&set, P9, "n")), (Some(3.0), Some(1.0)));
}

#[test]
fn a_disabled_reference_keeps_running_its_script() {
    // Open question 4 (OpenMW: `World::disable` leaves `LocalScripts` alone): the script
    // disables its object, keeps running, and enables it again.
    let blink = "Begin blink_scr\nshort ticks\nset ticks to ticks + 1\nif ( GetDisabled == 0 )\nDisable\nelse\nEnable\nendif\nEnd\n";
    let set = set_of(&[("blink_scr", blink)]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, P2, "blink_scr");
    tick(&set, &mut g, &mut l, &mut game);
    assert!(game.disabled.contains(&P2) && l.is_running(P2));
    tick(&set, &mut g, &mut l, &mut game);
    assert!(!game.disabled.contains(&P2));
    assert_eq!(l.local(&set, P2, "ticks"), Some(2.0));
}

#[test]
fn a_paused_instance_skips_its_runs_and_keeps_its_locals() {
    let set = set_of(&[("cnt_scr", COUNTER)]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, P2, "cnt_scr");
    tick(&set, &mut g, &mut l, &mut game);
    assert!(l.set_paused(P2, true) && !l.is_running(P2));
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!(l.local(&set, P2, "n"), Some(1.0));
    assert!(l.set_paused(P2, false));
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!(l.local(&set, P2, "n"), Some(2.0));
    assert!(!l.set_paused(P9, true), "no such instance");
}

#[test]
fn start_script_from_a_local_targets_its_object_and_runs_next_tick() {
    let starter = "Begin s_scr\nshort done\nif ( done == 0 )\nStartScript glob_scr\nStartScript other_scr\nStopScript other_scr\nset done to 1\nendif\nEnd\n";
    let set = set_of(&[
        ("s_scr", starter),
        ("glob_scr", "Begin glob_scr\nAddTopic \"glob\"\nEnd\n"),
        ("other_scr", "Begin other_scr\nAddTopic \"other\"\nEnd\n"),
    ]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, P9, "s_scr");
    tick(&set, &mut g, &mut l, &mut game);
    assert!(topics(&game).is_empty(), "globals already ran this tick");
    assert_eq!(g.get("glob_scr").unwrap().target.as_deref(), Some("obj_9"));
    assert!(g.is_running("glob_scr") && !g.is_running("other_scr"), "ops applied in order");
    tick(&set, &mut g, &mut l, &mut game);
    assert_eq!(topics(&game), ["glob"]);
}

#[test]
fn a_step_limit_run_drops_its_ops_pauses_and_reports_once() {
    let runaway = "Begin w_scr\nshort i\nStartScript glob_scr\nset g to 5\nwhile ( 1 )\nset i to i + 1\nendwhile\nEnd\n";
    let set = set_of(&[("w_scr", runaway), ("glob_scr", "Begin glob_scr\nEnd\n")]);
    let mut interp = interp();
    interp.step_limit = 1000;
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, P2, "w_scr");
    l.tick(&set, &interp, &mut g, &mut game);
    l.tick(&set, &interp, &mut g, &mut game);
    assert_eq!(
        l.take_reports(),
        [LocalReport::StepLimit { key: P2, script: "w_scr".into(), line: 5 }]
    );
    assert!(!l.is_running(P2) && l.get(P2).is_some(), "paused, not removed");
    assert!(g.get("glob_scr").is_none(), "the faulted run's StartScript was dropped");
    assert_eq!(game.log, ["begin w_scr Placement(2)", "dropped w_scr"]);
    l.set_paused(P2, false);
    l.tick(&set, &interp, &mut g, &mut game);
    assert_eq!(l.take_reports().len(), 1, "a resumed script may fault again");
}

#[test]
fn unknown_and_inert_inserts_are_reported() {
    let mut set = set_of(&[("cnt_scr", COUNTER)]);
    let broken = crate::parse("Begin x\nFrobnicate 3\nEnd x\n", crate::SourceKind::Script, FunctionTable::vanilla());
    assert!(!set.insert("bad_scr", broken));
    let mut l = LocalInstances::default();
    assert!(!l.insert(&set, P2, "Bad_Scr"));
    assert!(!l.insert(&set, P9, "missing"));
    assert_eq!(
        l.take_reports(),
        [
            LocalReport::InertScript { key: P2, script: "bad_scr".into() },
            LocalReport::UnknownScript { key: P9, script: "missing".into() },
        ]
    );
    assert!(l.is_empty());
}

#[test]
fn set_local_stores_converted() {
    let set = set_of(&[("t_scr", "Begin t_scr\nshort s\nfloat f\nEnd\n")]);
    let mut l = LocalInstances::default();
    l.insert(&set, P2, "t_scr");
    assert!(l.set_local(&set, P2, "S", 40000.0));
    assert!(l.set_local(&set, P2, "f", 0.1));
    assert!(!l.set_local(&set, P2, "nope", 1.0) && !l.set_local(&set, P9, "s", 1.0));
    assert_eq!(l.get(P2).unwrap().locals.0, [store(VarType::Short, 40000.0), store(VarType::Float, 0.1)]);
}

#[test]
fn save_round_trip_and_unknown_script_on_load() {
    let set = set_of(&[("cnt_scr", COUNTER), ("old_scr", "Begin old_scr\nshort k\nset k to 7\nEnd\n")]);
    let (mut g, mut l, mut game) = (GlobalInstances::default(), LocalInstances::default(), Game::default());
    l.insert(&set, S11, "cnt_scr");
    l.insert(&set, P2, "old_scr");
    l.insert(&set, P9, "cnt_scr");
    tick(&set, &mut g, &mut l, &mut game);
    l.set_paused(P9, true);
    let snap = l.snapshot(&set);
    let json = serde_json::to_string(&snap).unwrap();
    let back: LocalsSnapshot = serde_json::from_str(&json).unwrap();
    assert_eq!(back, snap);
    assert_eq!(snap.locals.iter().map(|s| s.key).collect::<Vec<_>>(), [P2, P9, S11]);
    let (loaded, reports) = LocalInstances::load(&back, &set);
    assert!(reports.is_empty(), "{reports:?}");
    assert_eq!(loaded.iter().collect::<Vec<_>>(), l.iter().collect::<Vec<_>>());
    assert!(!loaded.is_running(P9));
    // A mod removed old_scr: its instance is dropped with one report, the rest load.
    let new = set_of(&[("cnt_scr", COUNTER)]);
    let (loaded, reports) = LocalInstances::load(&back, &new);
    assert_eq!(reports, [(P2, LoadReport::UnknownScript { script: "old_scr".into() })]);
    assert_eq!(loaded.iter().map(|(k, _)| k).collect::<Vec<_>>(), [P9, S11]);
    assert_eq!(loaded.local(&new, S11, "n"), Some(1.0));
}
