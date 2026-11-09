//! ai.md "Tests" for AI2: exact decompositions of a small synthetic library loaded from record
//! text, partial plans, MTR replans, load reports, and tracing on = tracing off.

use sf_records::{RecordDb, RecordFormat, apply_record_file};

use crate::htn::Task;
use crate::*;

/// A guard: fight (engage) if it can, else flee, else eat (get food first), else idle with a
/// pause. Four compound tasks would do; three keep the expected lists short.
const LIBRARY: &str = r#"[
  define(kind: "sf.ai.task", id: "be_guard", value: Compound),
  define(kind: "sf.ai.task", id: "engage", value: Compound),
  define(kind: "sf.ai.task", id: "get_food", value: Compound),
  define(kind: "sf.ai.task", id: "p_moveto", value: Primitive(operator: "MoveTo")),
  define(kind: "sf.ai.task", id: "p_attack", value: Primitive(operator: "Attack", pre: [Ref(key: "threat", set: true)])),
  define(kind: "sf.ai.task", id: "p_draw", value: Primitive(operator: "Draw",
    effects: [SetBit(key: "armed", value: true)])),
  define(kind: "sf.ai.task", id: "p_flee", value: Primitive(operator: "Flee")),
  define(kind: "sf.ai.task", id: "p_buy", value: Primitive(operator: "Buy",
    pre: [Int(key: "gold", op: Ge, value: 5)],
    effects: [AddInt(key: "gold", by: -5), SetBit(key: "has_food", value: true)])),
  define(kind: "sf.ai.task", id: "p_unpack", value: Primitive(operator: "Unpack")),
  define(kind: "sf.ai.task", id: "p_eat", value: Primitive(operator: "Eat",
    pre: [Bit(key: "has_food", value: true)],
    effects: [SetBit(key: "hungry", value: false), SetBit(key: "has_food", value: false)])),
  define(kind: "sf.ai.task", id: "p_wander", value: Primitive(operator: "Wander")),
  define(kind: "sf.ai.task", id: "p_wait", value: Primitive(operator: "Wait")),

  // Defined out of priority order on purpose: the library sorts.
  define(kind: "sf.ai.method", id: "idle", value: (task: "be_guard", priority: 0,
    subtasks: [Task("p_wander"), Pause, Task("p_wait")])),
  define(kind: "sf.ai.method", id: "flee", value: (task: "be_guard", priority: 20,
    pre: [Bit(key: "enemy_visible", value: true)], subtasks: [Task("p_flee")])),
  define(kind: "sf.ai.method", id: "fight", value: (task: "be_guard", priority: 30,
    pre: [Bit(key: "enemy_visible", value: true), Int(key: "health", op: Ge, value: 30)],
    subtasks: [Task("engage")])),
  define(kind: "sf.ai.method", id: "eat", value: (task: "be_guard", priority: 10,
    pre: [Bit(key: "hungry", value: true)], subtasks: [Task("get_food"), Task("p_eat")])),
  define(kind: "sf.ai.method", id: "engage_armed", value: (task: "engage", priority: 10,
    pre: [Bit(key: "armed", value: true), Ref(key: "threat", set: true)],
    subtasks: [Task("p_moveto"), Task("p_attack")])),
  define(kind: "sf.ai.method", id: "engage_draw", value: (task: "engage", priority: 5,
    pre: [Ref(key: "threat", set: true)], subtasks: [Task("p_draw"), Task("p_moveto"), Task("p_attack")])),
  define(kind: "sf.ai.method", id: "food_in_pack", value: (task: "get_food", priority: 10,
    pre: [Bit(key: "has_food", value: true)], subtasks: [Task("p_unpack")])),
  define(kind: "sf.ai.method", id: "food_at_market", value: (task: "get_food", priority: 5,
    subtasks: [Task("p_moveto"), Task("p_buy")])),
]"#;

struct Fixture {
    lib: Library,
    root: TaskId,
    enemy: BitKey,
    armed: BitKey,
    hungry: BitKey,
    health: IntKey,
    gold: IntKey,
    threat: RefKey,
}

fn vocabulary() -> (StateKeys, Operators) {
    let mut keys = StateKeys::new();
    for bit in ["enemy_visible", "armed", "hungry", "has_food"] {
        keys.add_bit(bit).unwrap();
    }
    keys.add_int("health").unwrap();
    keys.add_int("gold").unwrap();
    keys.add_ref("threat").unwrap();
    let mut ops = Operators::new();
    for op in ["MoveTo", "Attack", "Draw", "Flee", "Buy", "Unpack", "Eat", "Wander", "Wait"] {
        ops.add(op);
    }
    (keys, ops)
}

fn load(text: &str) -> Result<Library, LoadReport> {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let src = db.add_source("test", &[]);
    let outcome = apply_record_file(&mut db, src, text, RecordFormat::Ron).unwrap();
    assert_eq!(outcome.errors, []);
    let (keys, ops) = vocabulary();
    Library::build(&db, &keys, &ops)
}

fn fixture() -> Fixture {
    let lib = load(LIBRARY).unwrap();
    let keys = lib.keys().clone();
    let bit = |n: &str| match keys.get(n) {
        Some(StateKey::Bit(k)) => k,
        other => panic!("{n}: {other:?}"),
    };
    let int = |n: &str| match keys.get(n) {
        Some(StateKey::Int(k)) => k,
        other => panic!("{n}: {other:?}"),
    };
    Fixture {
        root: lib.task_id("be_guard").unwrap(),
        enemy: bit("enemy_visible"),
        armed: bit("armed"),
        hungry: bit("hungry"),
        health: int("health"),
        gold: int("gold"),
        threat: match keys.get("threat") {
            Some(StateKey::Ref(k)) => k,
            other => panic!("threat: {other:?}"),
        },
        lib,
    }
}

impl Fixture {
    fn ws(&self) -> WorldState {
        WorldState::new(self.lib.keys())
    }

    /// Enemy in sight, healthy, armed; `threat` set or not.
    fn combat(&self, threat: bool) -> WorldState {
        let mut ws = self.ws();
        ws.set_bit(self.enemy, true);
        ws.set_bit(self.armed, true);
        ws.set_int(self.health, 80);
        ws.set_target(self.threat, threat.then_some(Target(7)));
        ws
    }

    fn hungry(&self, gold: i32) -> WorldState {
        let mut ws = self.ws();
        ws.set_bit(self.hungry, true);
        ws.set_int(self.gold, gold);
        ws
    }

    fn m(&self, name: &str) -> MethodId {
        self.lib.method_id(name).unwrap()
    }

    fn t(&self, name: &str) -> TaskId {
        self.lib.task_id(name).unwrap()
    }

    fn choice(&self, task: &str, method: &str) -> MethodChoice {
        MethodChoice { task: self.t(task), method: self.m(method) }
    }

    fn rej(&self, task: &str, method: &str, reason: RejectReason) -> Rejection {
        Rejection { task: self.t(task), method: self.m(method), reason }
    }

    /// The plan's steps as task names.
    fn steps<'a>(&'a self, steps: &[PlanStep]) -> Vec<&'a str> {
        steps.iter().map(|s| self.lib.task_name(s.task)).collect()
    }

    /// Plans with a fresh trace entry; asserts the untraced plan is identical.
    fn plan(&self, ws: &WorldState) -> (Result<Decomposition, NoPlan>, TraceEntry) {
        let mut entry = TraceEntry::new(1, 2);
        let traced = decompose(ws, self.root, &self.lib, Some(&mut entry));
        assert_eq!(decompose(ws, self.root, &self.lib, None), traced);
        (traced, entry)
    }
}

#[test]
fn methods_sort_by_priority_then_id() {
    let f = fixture();
    let Some(Task::Compound(list)) = f.lib.task(f.root) else { panic!("compound") };
    let names: Vec<_> = list.iter().map(|&m| f.lib.method_name(m)).collect();
    assert_eq!(names, ["fight", "flee", "eat", "idle"]);
    assert_eq!(f.lib.method(f.m("idle")).map(|m| m.rank), Some(3));
    assert_eq!((f.lib.task_count(), f.lib.method_count()), (12, 8));
    let p_eat = f.lib.task(f.t("p_eat"));
    assert!(matches!(p_eat, Some(Task::Primitive(p)) if f.lib.operators().name(p.operator) == "eat"));
}

#[test]
fn decomposes_three_world_states_exactly() {
    let f = fixture();

    // 1. Armed with a target: fight → engage_armed, nothing rejected.
    let (plan, e) = f.plan(&f.combat(true));
    let plan = plan.unwrap();
    assert_eq!(f.steps(&plan.steps), ["p_moveto", "p_attack"]);
    assert_eq!(plan.methods, [f.choice("be_guard", "fight"), f.choice("engage", "engage_armed")]);
    assert_eq!(plan.mtr, [0, 0]);
    assert!(!plan.is_partial());
    assert_eq!(e.rejected, []);
    assert_eq!(e.methods, plan.methods);
    assert_eq!(e.plan.steps, plan.steps);
    assert_eq!(e.mtr, [0, 0]);
    assert_eq!(e.outcome, Some(PlanOutcome::Planned));

    // 2. No target: both engage methods fail, so fight backtracks (SubtaskFailed) to flee.
    let (plan, e) = f.plan(&f.combat(false));
    let plan = plan.unwrap();
    assert_eq!(f.steps(&plan.steps), ["p_flee"]);
    assert_eq!(plan.methods, [f.choice("be_guard", "flee")]);
    assert_eq!(plan.mtr, [1]);
    assert_eq!(
        e.rejected,
        [
            f.rej("engage", "engage_armed", RejectReason::Precondition(1)),
            f.rej("engage", "engage_draw", RejectReason::Precondition(0)),
            f.rej("be_guard", "fight", RejectReason::SubtaskFailed(f.t("engage"))),
        ]
    );
    assert_eq!(e.methods, plan.methods);
    let mut trace = Trace::new();
    trace.push(e);
    assert_eq!(
        format_trace_with(&trace, 1, &f.lib),
        "#1 t2 scheduled drive - via be_guard:flee rej engage:engage_armed pre1 (+2) plan planned op - path -\n"
    );

    // 3. Hungry, no food, gold: eat → get_food → market. Buying's planning effect (has_food)
    //    is what lets p_eat's precondition hold; the caller's state is untouched.
    let ws = f.hungry(10);
    let before = ws.clone();
    let (plan, e) = f.plan(&ws);
    let plan = plan.unwrap();
    assert_eq!(ws, before);
    assert_eq!(f.steps(&plan.steps), ["p_moveto", "p_buy", "p_eat"]);
    assert_eq!(plan.methods, [f.choice("be_guard", "eat"), f.choice("get_food", "food_at_market")]);
    assert_eq!(plan.mtr, [2, 1]);
    assert_eq!(
        e.rejected,
        [
            f.rej("be_guard", "fight", RejectReason::Precondition(0)),
            f.rej("be_guard", "flee", RejectReason::Precondition(0)),
            f.rej("get_food", "food_in_pack", RejectReason::Precondition(0)),
        ]
    );
    let ops: Vec<_> = plan.steps.iter().map(|s| f.lib.operators().name(s.operator)).collect();
    assert_eq!(ops, ["moveto", "buy", "eat"]);
}

#[test]
fn a_failed_primitive_undoes_its_method_and_backtracks() {
    let f = fixture();
    // Hungry with 3 gold: the market method plans p_moveto, then p_buy fails; its step is undone.
    let (plan, e) = f.plan(&f.hungry(3));
    let plan = plan.unwrap();
    assert_eq!(f.steps(&plan.steps), ["p_wander"]);
    assert_eq!(plan.methods, [f.choice("be_guard", "idle")]);
    assert_eq!(
        e.rejected,
        [
            f.rej("be_guard", "fight", RejectReason::Precondition(0)),
            f.rej("be_guard", "flee", RejectReason::Precondition(0)),
            f.rej("get_food", "food_in_pack", RejectReason::Precondition(0)),
            f.rej("get_food", "food_at_market", RejectReason::SubtaskFailed(f.t("p_buy"))),
            f.rej("be_guard", "eat", RejectReason::SubtaskFailed(f.t("get_food"))),
        ]
    );
}

#[test]
fn a_pause_marker_stops_and_resume_plans_the_rest() {
    let f = fixture();
    let (plan, e) = f.plan(&f.ws());
    let plan = plan.unwrap();
    assert_eq!(f.steps(&plan.steps), ["p_wander"]);
    assert!(plan.is_partial());
    assert_eq!(plan.rest, [Resume { method: f.m("idle"), next: 2 }]);
    assert!(e.plan.partial);
    assert_eq!(e.mtr, [3]);

    let mut e = TraceEntry::new(2, 2);
    let rest = resume(&f.ws(), &plan, &f.lib, Some(&mut e)).unwrap();
    assert_eq!(resume(&f.ws(), &plan, &f.lib, None).as_ref(), Ok(&rest));
    assert_eq!(f.steps(&rest.steps), ["p_wait"]);
    assert!(!rest.is_partial());
    assert_eq!(rest.methods, [f.choice("be_guard", "idle")]);
    assert_eq!(rest.mtr, [3]);
    assert_eq!(e.outcome, Some(PlanOutcome::Continued));
    assert!(!e.plan.partial);
    assert_eq!(e.rejected, []);

    // A finished plan resumes to nothing.
    let done = resume(&f.ws(), &rest, &f.lib, None).unwrap();
    assert!(done.steps.is_empty() && !done.is_partial());
}

#[test]
fn replan_adopts_only_a_higher_priority_mtr() {
    let f = fixture();
    let running = decompose(&f.hungry(10), f.root, &f.lib, None).unwrap();
    assert_eq!(running.mtr, [2, 1]);

    // Same state, same MTR: equal is not better.
    let mut e = TraceEntry::new(3, 2);
    assert_eq!(replan(&f.hungry(10), f.root, &f.lib, &running, Some(&mut e)), Replan::KeepNotBetter);
    assert_eq!(e.outcome, Some(PlanOutcome::KeptNotBetter));
    assert_eq!(e.methods, running.methods);
    assert_eq!(e.plan.steps, running.steps);

    // Out of gold: the only plan left (idle) ranks lower, and the search stops the moment it
    // can't win (NotBetter), before checking idle at all.
    let mut e = TraceEntry::new(4, 2);
    assert_eq!(replan(&f.hungry(3), f.root, &f.lib, &running, Some(&mut e)), Replan::KeepNotBetter);
    assert_eq!(replan(&f.hungry(3), f.root, &f.lib, &running, None), Replan::KeepNotBetter);
    assert_eq!(
        e.rejected,
        [
            f.rej("be_guard", "fight", RejectReason::Precondition(0)),
            f.rej("be_guard", "flee", RejectReason::Precondition(0)),
            f.rej("get_food", "food_in_pack", RejectReason::Precondition(0)),
            f.rej("get_food", "food_at_market", RejectReason::SubtaskFailed(f.t("p_buy"))),
            f.rej("be_guard", "eat", RejectReason::SubtaskFailed(f.t("get_food"))),
            f.rej("be_guard", "idle", RejectReason::NotBetter),
        ]
    );
    assert_eq!(e.mtr, [2, 1]);
    let mut trace = Trace::new();
    trace.push(e);
    assert_eq!(
        format_trace(&trace, 1),
        format!(
            "#4 t2 scheduled drive - via {}:{}>{}:{} rej {}:{} pre0 (+5) plan kept_not_better op - path -\n",
            f.t("be_guard"),
            f.m("eat"),
            f.t("get_food"),
            f.m("food_at_market"),
            f.t("be_guard"),
            f.m("fight")
        )
    );

    // An enemy with a target: fight (rank 0 < 2) wins.
    let mut e = TraceEntry::new(5, 2);
    let Replan::Replace(new) = replan(&f.combat(true), f.root, &f.lib, &running, Some(&mut e)) else {
        panic!("expected a replacement");
    };
    assert_eq!(new.mtr, [0, 0]);
    assert_eq!(f.steps(&new.steps), ["p_moveto", "p_attack"]);
    assert_eq!(e.outcome, Some(PlanOutcome::Replaced));
    assert_eq!(e.mtr, [0, 0]);

    // Idle (rank 3) running: eating (rank 2) beats it.
    let idle = decompose(&f.ws(), f.root, &f.lib, None).unwrap();
    assert!(matches!(replan(&f.hungry(10), f.root, &f.lib, &idle, None), Replan::Replace(p) if p.mtr == [2, 1]));
}

#[test]
fn mtr_comparison_is_lexicographic_on_the_common_prefix() {
    assert!(mtr_better(&[0], &[1]));
    assert!(mtr_better(&[1, 0], &[1, 1]));
    assert!(mtr_better(&[0, 5], &[1]));
    assert!(!mtr_better(&[1, 1], &[1, 1]));
    assert!(!mtr_better(&[1], &[1, 0]));
    assert!(!mtr_better(&[1, 0], &[1]));
    assert!(!mtr_better(&[2], &[1, 0]));
    assert!(!mtr_better(&[], &[]));
}

#[test]
fn plans_record_the_keys_they_read() {
    let f = fixture();
    let ws = f.combat(true);
    let plan = decompose(&ws, f.root, &f.lib, None).unwrap();
    for key in [StateKey::Bit(f.enemy), StateKey::Int(f.health), StateKey::Bit(f.armed), StateKey::Ref(f.threat)] {
        assert!(plan.reads.contains(key), "{key:?}");
    }
    // Never evaluated: fight held, so flee/eat/idle were not tried.
    assert!(!plan.reads.contains(StateKey::Bit(f.hungry)));
    let mut lost = ws.clone();
    lost.set_target(f.threat, None);
    assert!(ws.changed(&lost).intersects(&plan.reads));
    let mut fed = ws.clone();
    fed.set_bit(f.hungry, true);
    assert!(!ws.changed(&fed).intersects(&plan.reads));
    // The running operator's monitoring check.
    assert!(f.lib.holds(f.t("p_attack"), &ws));
    assert!(!f.lib.holds(f.t("p_attack"), &lost));
}

#[test]
fn unknown_names_fail_the_load_with_a_report_naming_them() {
    let text = r#"[
      define(kind: "sf.ai.task", id: "root", value: Compound),
      define(kind: "sf.ai.task", id: "p_dance", value: Primitive(operator: "Dance")),
      define(kind: "sf.ai.task", id: "p_wait", value: Primitive(operator: "Wait",
        pre: [Int(key: "armed", op: Ge, value: 1)])),
      define(kind: "sf.ai.method", id: "jump", value: (task: "root",
        pre: [Bit(key: "stamina_ok", value: true)], subtasks: [Task("p_jump"), Task("p_wait")])),
      define(kind: "sf.ai.method", id: "orphan", value: (task: "roam", subtasks: [])),
      define(kind: "sf.ai.method", id: "on_primitive", value: (task: "p_wait", subtasks: [])),
    ]"#;
    let report = load(text).unwrap_err();
    assert_eq!(
        report.to_string(),
        "HTN library: 6 error(s)\n  \
         sf.ai.task `p_dance`: unknown operator `Dance`\n  \
         sf.ai.task `p_wait`: state key `armed` is of kind bit, expected int\n  \
         sf.ai.method `jump`: unknown task `p_jump`\n  \
         sf.ai.method `jump`: unknown state key `stamina_ok`\n  \
         sf.ai.method `orphan`: unknown task `roam`\n  \
         sf.ai.method `on_primitive`: task `p_wait` is primitive and can't have methods"
    );
    assert!(matches!(&report.errors[2], LoadError::UnknownTask { name, .. } if name == "p_jump"));
}

#[test]
fn a_recursive_method_hits_the_depth_limit_instead_of_hanging() {
    let text = r#"[
      define(kind: "sf.ai.task", id: "loop", value: Compound),
      define(kind: "sf.ai.method", id: "again", value: (task: "loop", subtasks: [Task("loop")])),
    ]"#;
    let lib = load(text).unwrap();
    let ws = WorldState::new(lib.keys());
    let mut e = TraceEntry::new(0, 2);
    let root = lib.task_id("loop").unwrap();
    assert_eq!(decompose(&ws, root, &lib, Some(&mut e)), Err(NoPlan::Failed));
    assert_eq!(e.outcome, Some(PlanOutcome::NoPlan));
    let again = lib.method_id("again").unwrap();
    assert_eq!(e.rejected[0], Rejection { task: root, method: again, reason: RejectReason::Limit });
    assert_eq!(e.rejected.len(), SEARCH_DEPTH + 1);
    assert_eq!(decompose(&ws, TaskId(99), &lib, None), Err(NoPlan::UnknownTask));
}

#[test]
fn state_keys_fold_names_share_same_kind_and_reject_clashes() {
    let mut keys = StateKeys::new();
    let a = keys.add_bit("Enemy_Visible").unwrap();
    assert_eq!(keys.add_bit("enemy_visible"), Ok(a));
    assert_eq!(keys.get("ENEMY_VISIBLE"), Some(StateKey::Bit(a)));
    assert_eq!(keys.name(StateKey::Bit(a)), "enemy_visible");
    assert_eq!(
        keys.add_int("enemy_visible"),
        Err(StateKeyError::KindClash { name: "enemy_visible".into(), existing: KeyKind::Bit, wanted: KeyKind::Int })
    );
    for i in 0..70 {
        keys.add_bit(&format!("b{i}")).unwrap();
    }
    let mut ws = WorldState::new(&keys);
    let far = keys.add_bit("b69").unwrap();
    ws.set_bit(far, true);
    assert!(ws.bit(far) && !ws.bit(a));
    let json = serde_json::to_string(&ws).unwrap();
    assert_eq!(serde_json::from_str::<WorldState>(&json).unwrap(), ws);
}
