//! ai.md "Tests" for AI1: exact assertions on synthetic traces (CONTRIBUTING: logic is exact).

use crate::*;

const A: ActorKey = ActorKey(1);
const B: ActorKey = ActorKey(2);
const C: ActorKey = ActorKey(3);

fn ticks(trace: &Trace) -> Vec<u64> {
    trace.iter().map(|e| e.tick).collect()
}

fn keys(store: &TraceStore) -> Vec<ActorKey> {
    store.iter().map(|(a, _)| a).collect()
}

/// The fixed synthetic trace behind both goldens: a full step, then a replan that backtracked.
fn golden_trace() -> Trace {
    let mut trace = Trace::new();
    trace.push(TraceEntry {
        tick: 120,
        tier: 2,
        trigger: Trigger::Scheduled,
        directive: Some(DirectiveId(5)),
        drives: vec![
            DriveScore {
                drive: DriveId(0),
                raw: 0.5,
                score: 0.75,
                considerations: vec![
                    ConsiderationScore { consideration: ConsiderationId(0), value: 0.5 },
                    ConsiderationScore { consideration: ConsiderationId(1), value: 1.0 },
                ],
            },
            DriveScore {
                drive: DriveId(1),
                raw: 0.25,
                score: 0.25,
                considerations: vec![ConsiderationScore { consideration: ConsiderationId(2), value: 0.25 }],
            },
        ],
        drive: Some(DriveId(0)),
        methods: vec![
            MethodChoice { task: TaskId(1), method: MethodId(7) },
            MethodChoice { task: TaskId(4), method: MethodId(12) },
        ],
        rejected: vec![
            Rejection { task: TaskId(1), method: MethodId(5), reason: RejectReason::Precondition(1) },
            Rejection { task: TaskId(4), method: MethodId(11), reason: RejectReason::Precondition(3) },
        ],
        plan: Plan {
            steps: vec![
                PlanStep { task: TaskId(9), operator: OperatorId(2) },
                PlanStep { task: TaskId(10), operator: OperatorId(4) },
            ],
            cursor: 1,
            partial: false,
        },
        operator: Some(OperatorState { operator: OperatorId(4), status: OperatorStatus::Running }),
        path: PathState::Following,
        budget_us: Some(85),
        mtr: Vec::new(),
        outcome: None,
    });
    trace.push(TraceEntry {
        tick: 126,
        tier: 1,
        trigger: Trigger::Replan,
        directive: Some(DirectiveId(5)),
        drives: Vec::new(),
        drive: Some(DriveId(0)),
        methods: vec![MethodChoice { task: TaskId(1), method: MethodId(6) }],
        rejected: vec![Rejection { task: TaskId(1), method: MethodId(7), reason: RejectReason::SubtaskFailed(TaskId(4)) }],
        plan: Plan { steps: vec![PlanStep { task: TaskId(11), operator: OperatorId(1) }], cursor: 0, partial: true },
        operator: Some(OperatorState { operator: OperatorId(1), status: OperatorStatus::Failed }),
        path: PathState::Failed(PathFailure::NoPath),
        budget_us: None,
        mtr: Vec::new(),
        outcome: None,
    });
    trace
}

#[test]
fn ring_keeps_the_last_64_in_order() {
    let mut trace = Trace::new();
    for tick in 1..=70 {
        trace.push(TraceEntry::new(tick, 2));
    }
    assert_eq!(trace.len(), TRACE_LEN);
    assert_eq!(ticks(&trace), (7..=70).collect::<Vec<_>>());
    assert_eq!(trace.last().map(|e| e.tick), Some(70));
    trace.clear();
    assert!(trace.is_empty());
    assert_eq!(trace.last(), None);
    assert_eq!(trace.iter().count(), 0);
}

#[test]
fn record_recycles_the_oldest_entry_without_growing() {
    let mut trace = Trace::new();
    let ring = trace.entries.capacity();
    for tick in 1..=70 {
        let e = trace.record(tick, 2);
        // A recycled entry comes back empty, whatever it held.
        assert_eq!(*e, TraceEntry::new(tick, 2));
        e.methods.push(MethodChoice { task: TaskId(1), method: MethodId(2) });
        e.rejected.push(Rejection { task: TaskId(1), method: MethodId(3), reason: RejectReason::Precondition(0) });
        e.path = PathState::Pending;
    }
    assert_eq!(trace.entries.capacity(), ring);
    assert_eq!(ticks(&trace), (7..=70).collect::<Vec<_>>());
    // The recycled entry kept its buffer.
    assert!(trace.last().unwrap().methods.capacity() >= 1);
}

#[test]
fn rejections_round_trip_through_json() {
    let entry = golden_trace().iter().next().unwrap().clone();
    let json = serde_json::to_string(&entry).unwrap();
    let back: TraceEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(back, entry);
    let reasons: Vec<_> = back.rejected.iter().map(|r| (r.method, r.reason)).collect();
    assert_eq!(
        reasons,
        [(MethodId(5), RejectReason::Precondition(1)), (MethodId(11), RejectReason::Precondition(3))]
    );
}

const GOLDEN_JSON: &str = r#"[
  {
    "tick": 120,
    "tier": 2,
    "trigger": "scheduled",
    "directive": 5,
    "drives": [
      {
        "drive": 0,
        "raw": 0.5,
        "score": 0.75,
        "considerations": [
          {
            "consideration": 0,
            "value": 0.5
          },
          {
            "consideration": 1,
            "value": 1.0
          }
        ]
      },
      {
        "drive": 1,
        "raw": 0.25,
        "score": 0.25,
        "considerations": [
          {
            "consideration": 2,
            "value": 0.25
          }
        ]
      }
    ],
    "drive": 0,
    "methods": [
      {
        "task": 1,
        "method": 7
      },
      {
        "task": 4,
        "method": 12
      }
    ],
    "rejected": [
      {
        "task": 1,
        "method": 5,
        "reason": {
          "precondition": 1
        }
      },
      {
        "task": 4,
        "method": 11,
        "reason": {
          "precondition": 3
        }
      }
    ],
    "plan": {
      "steps": [
        {
          "task": 9,
          "operator": 2
        },
        {
          "task": 10,
          "operator": 4
        }
      ],
      "cursor": 1,
      "partial": false
    },
    "operator": {
      "operator": 4,
      "status": "running"
    },
    "path": "following",
    "budget_us": 85,
    "mtr": [],
    "outcome": null
  },
  {
    "tick": 126,
    "tier": 1,
    "trigger": "replan",
    "directive": 5,
    "drives": [],
    "drive": 0,
    "methods": [
      {
        "task": 1,
        "method": 6
      }
    ],
    "rejected": [
      {
        "task": 1,
        "method": 7,
        "reason": {
          "subtask_failed": 4
        }
      }
    ],
    "plan": {
      "steps": [
        {
          "task": 11,
          "operator": 1
        }
      ],
      "cursor": 0,
      "partial": true
    },
    "operator": {
      "operator": 1,
      "status": "failed"
    },
    "path": {
      "failed": "no_path"
    },
    "budget_us": null,
    "mtr": [],
    "outcome": null
  }
]"#;

#[test]
fn golden_trace_json_is_exact() {
    assert_eq!(golden_trace().to_json().unwrap(), GOLDEN_JSON);
}

#[test]
fn store_json_lists_actors_in_key_order() {
    let mut store = TraceStore::new();
    store.set_admit(Admit::All);
    store.push(B, TraceEntry::new(3, 0));
    store.push(A, TraceEntry::new(4, 2));
    let expected = r#"{
  "actors": [
    {
      "actor": 1,
      "entries": [
        {
          "tick": 4,
          "tier": 2,
          "trigger": "scheduled",
          "directive": null,
          "drives": [],
          "drive": null,
          "methods": [],
          "rejected": [],
          "plan": {
            "steps": [],
            "cursor": 0,
            "partial": false
          },
          "operator": null,
          "path": "none",
          "budget_us": null,
          "mtr": [],
          "outcome": null
        }
      ]
    },
    {
      "actor": 2,
      "entries": [
        {
          "tick": 3,
          "tier": 0,
          "trigger": "scheduled",
          "directive": null,
          "drives": [],
          "drive": null,
          "methods": [],
          "rejected": [],
          "plan": {
            "steps": [],
            "cursor": 0,
            "partial": false
          },
          "operator": null,
          "path": "none",
          "budget_us": null,
          "mtr": [],
          "outcome": null
        }
      ]
    }
  ]
}"#;
    assert_eq!(store.to_json().unwrap(), expected);
}

#[test]
fn format_trace_of_the_golden_trace_is_exact() {
    let trace = golden_trace();
    assert_eq!(
        format_trace(&trace, 10),
        "#120 t2 scheduled drive d0 via t1:m7>t4:m12 rej t1:m5 pre1 (+1) op o4 running path following 85us\n\
         #126 t1 replan drive d0 via t1:m6 rej t1:m7 sub t4 op o1 failed path failed(no_path)\n"
    );
    assert_eq!(
        format_trace(&trace, 1),
        "#126 t1 replan drive d0 via t1:m6 rej t1:m7 sub t4 op o1 failed path failed(no_path)\n"
    );
    assert_eq!(format_trace(&trace, 0), "");
    assert_eq!(format_trace(&Trace::new(), 5), "");
    assert_eq!(format_trace_one(&TraceEntry::new(9, 0)), "#9 t0 scheduled drive - via - rej - op - path -\n");
}

fn format_trace_one(entry: &TraceEntry) -> String {
    let mut trace = Trace::new();
    trace.push(entry.clone());
    format_trace(&trace, 1)
}

#[test]
fn disabled_store_stays_empty_and_builds_nothing() {
    let mut store = TraceStore::new();
    store.set_admit(Admit::All);
    store.watch(C);
    store.set_enabled(false);
    let mut built = 0;
    for tick in 0..100 {
        for actor in [A, B, C] {
            assert!(!store.wants(actor, 2));
            if let Some(e) = store.record(actor, tick, 2) {
                built += 1;
                e.drive = Some(DriveId(0));
            }
        }
    }
    assert_eq!(built, 0);
    assert!(store.is_empty());
    // Back on, the same loop records.
    store.set_enabled(true);
    assert!(store.record(A, 100, 2).is_some());
    assert_eq!(keys(&store), [A]);
}

#[test]
fn cap_keeps_the_first_admitted() {
    let mut store = TraceStore::new();
    store.set_admit(Admit::All);
    store.set_cap(2);
    for tick in 0..10 {
        for actor in [A, B, C] {
            store.record(actor, tick, 2);
        }
    }
    assert_eq!(keys(&store), [A, B]);
    assert!(!store.wants(C, 2));
    assert_eq!(store.get(A).map(Trace::len), Some(10));
    assert_eq!(store.get(C), None);
    // Room frees on forget, and the next actor to record takes it.
    store.forget(A);
    store.record(C, 10, 2);
    assert_eq!(keys(&store), [B, C]);
}

#[test]
fn watching_evicts_the_newest_unwatched_and_the_oldest_watch() {
    let mut store = TraceStore::new();
    store.set_admit(Admit::All);
    store.set_cap(2);
    store.record(A, 0, 2);
    store.record(B, 0, 2);
    // Full: watching C evicts B, the newest unwatched admission, at C's next step.
    store.watch(C);
    store.record(C, 1, 2);
    assert_eq!(keys(&store), [A, C]);
    // Watching a second actor fits (two watches, cap 2) and evicts A, the last unwatched.
    store.watch(B);
    store.record(B, 2, 2);
    assert_eq!(keys(&store), [B, C]);
    // A third watch drops the oldest (C) with its trace.
    store.watch(A);
    assert_eq!(store.watched(), [B, A]);
    assert_eq!(keys(&store), [B]);
    store.record(A, 3, 2);
    store.record(C, 3, 2);
    assert_eq!(keys(&store), [A, B]);
}

#[test]
fn tier2_admission_skips_lower_tiers_but_keeps_admitted_actors() {
    let mut store = TraceStore::new();
    assert_eq!(store.admit(), Admit::Tier2);
    assert!(store.record(A, 0, 1).is_none());
    assert!(store.record(B, 0, 2).is_some());
    // B demoted to tier 1 is still traced: the tier change is part of its story.
    assert!(store.record(B, 1, 1).is_some());
    assert_eq!(keys(&store), [B]);
    assert_eq!(store.get(B).map(|t| t.iter().map(|e| e.tier).collect::<Vec<_>>()), Some(vec![2, 1]));
    // Watched-only admission ignores unwatched actors at any tier.
    store.set_admit(Admit::Watched);
    assert!(store.record(C, 2, 2).is_none());
    store.watch(C);
    assert!(store.record(C, 2, 0).is_some());
}

#[test]
fn ids_display_with_prefixes_and_serialize_bare() {
    assert_eq!(
        [
            DriveId(3).to_string(),
            ConsiderationId(2).to_string(),
            TaskId(1).to_string(),
            MethodId(7).to_string(),
            OperatorId(4).to_string(),
            DirectiveId(5).to_string(),
        ],
        ["d3", "c2", "t1", "m7", "o4", "dir5"]
    );
    assert_eq!(serde_json::to_string(&MethodId(7)).unwrap(), "7");
    assert_eq!(serde_json::from_str::<TaskId>("9").unwrap(), TaskId(9));
}
