//! scripting.md "Verification", unit part.

use std::sync::Arc;
use std::time::{Duration, Instant};

use mlua::Value;
use sf_vfs::{MemSource, Vfs};

use crate::*;

fn vfs(files: &[(&str, &str)]) -> Arc<dyn ScriptSource> {
    let mut mem = MemSource::new("test");
    for (p, t) in files {
        mem.insert(p, t.as_bytes().to_vec()).unwrap();
    }
    Arc::new(Vfs::builder().mount(mem).build())
}

fn runtime(files: &[(&str, &str)], configs: Vec<ContextConfig>) -> ScriptRuntime {
    let mut rt = ScriptRuntime::default();
    let diags = rt.start(configs, vfs(files));
    assert!(diags.is_empty(), "start diagnostics: {diags:?}");
    rt
}

fn one(id: &str, main: &str) -> ScriptRuntime {
    let path = format!("scripts/{id}/main.luau");
    runtime(&[(&path, main)], vec![ContextConfig::for_mod(id)])
}

fn eval_bool(rt: &mut ScriptRuntime, id: &str, code: &str) -> bool {
    let policy = *rt.policy();
    rt.context_mut(id)
        .unwrap()
        .eval::<bool>(code, &policy)
        .unwrap_or_else(|| panic!("eval failed: {code}"))
}

fn eval_fails(rt: &mut ScriptRuntime, id: &str, code: &str) -> bool {
    let policy = *rt.policy();
    rt.context_mut(id)
        .unwrap()
        .eval::<Value>(code, &policy)
        .is_none()
}

#[test]
fn sandbox_escape_list() {
    let mut rt = one("m", "");
    // Absent capabilities.
    assert!(eval_bool(
        &mut rt,
        "m",
        "return io == nil and os == nil and loadstring == nil and getfenv == nil and setfenv == nil \
         and newproxy == nil and load == nil and dofile == nil and string.dump == nil \
         and debug.getinfo == nil and debug.info == nil and debug.traceback ~= nil",
    ));
    // Read-only globals and libraries.
    for code in [
        "string.upper = nil",
        "math.random = function() return 4 end",
        "table.insert = print",
        "sf.log = nil",
        "_G.string = {}",
        "require('@sf/core').mod_id = 'other'",
        "setmetatable(string, {})",
        "getmetatable('').__index = {}",
    ] {
        assert!(eval_fails(&mut rt, "m", code), "should fail: {code}");
    }
    // Writing a global stays in the context's own environment.
    assert!(eval_bool(&mut rt, "m", "x = 1 return x == 1"));
    let diags = rt.take_diagnostics();
    assert_eq!(diags.len(), 8);
    assert!(rt.context("m").unwrap().is_running(), "8 errors < 30");
}

#[test]
fn require_rules() {
    let bytecode = mlua::chunk::Compiler::new().compile("return 1").unwrap();
    let mut mem = MemSource::new("t");
    for (p, t) in [
        (
            "scripts/a/main.luau",
            "local u = require('./util') local l = require('@lib/shared') x = u.v + l.v",
        ),
        ("scripts/a/util.luau", "return { v = 1 }"),
        ("scripts/a/sub/init.luau", "return { v = 10 }"),
        ("scripts/lib/shared.luau", "return { v = 100 }"),
        ("scripts/other/secret.luau", "return { v = 1000 }"),
    ] {
        mem.insert(p, t.as_bytes().to_vec()).unwrap();
    }
    mem.insert("scripts/a/evil.luau", bytecode).unwrap();
    let source: Arc<dyn ScriptSource> = Arc::new(Vfs::builder().mount(mem).build());
    let mut config = ContextConfig::for_mod("a");
    config.dependencies = vec!["lib".into()];
    let mut rt = ScriptRuntime::default();
    assert!(rt.start(vec![config], source).is_empty());
    let policy = *rt.policy();
    let ctx = rt.context_mut("a").unwrap();
    assert_eq!(ctx.eval::<i64>("return x", &policy), Some(101));
    // `require` from a console chunk isn't allowed (only files under scripts/ may require).
    assert!(
        ctx.eval::<Value>("return require('./util')", &policy)
            .is_none()
    );
    // From a file: API modules, own directories, a dependency; nothing else.
    let cases = [
        ("return require('@sf/core').mod_id == 'a'", true),
        ("return require('./sub').v == 10", true),
        ("return require('@SF/Events').on ~= nil", true),
        ("return require('../other/secret')", false),
        ("return require('../a/util').v == 1", true),
        ("return require('../../x')", false),
        ("return require('@other/secret')", false),
        ("return require('./evil')", false),
        ("return require('@sf/nope')", false),
    ];
    for (code, ok) in cases {
        let f = rt
            .context("a")
            .unwrap()
            .lua()
            .load(code)
            .set_name("@scripts/a/probe.luau")
            .into_function()
            .unwrap();
        let ctx = rt.context_mut("a").unwrap();
        let got = ctx.call::<Value>(&f, (), "probe", &policy);
        assert_eq!(
            matches!(got, Some(Value::Boolean(true))),
            ok,
            "{code}: {got:?} {:?}",
            ctx.last_error()
        );
    }
}

#[test]
fn memory_limit_suspends() {
    let mut config = ContextConfig::for_mod("hog");
    config.limits.memory_bytes = 4 << 20;
    let mut rt = runtime(
        &[(
            "scripts/hog/main.luau",
            "local events = require('@sf/events') events.on('grow', function() local t = {} for i = 1, 1e8 do t[i] = i end end)",
        )],
        vec![config],
    );
    rt.dispatch("grow", StateValue::Nil);
    let ctx = rt.context("hog").unwrap();
    assert!(
        matches!(ctx.status(), Status::Suspended { .. }),
        "{:?}",
        ctx.status()
    );
    let diags = rt.take_diagnostics();
    assert_eq!(diags[0].kind, Some(FaultKind::Memory), "{diags:?}");
}

#[test]
fn watchdog_aborts_infinite_loops_even_under_pcall() {
    let mut config = ContextConfig::for_mod("spin");
    config.limits.call_timeout = Duration::from_millis(20);
    let mut rt = runtime(
        &[(
            "scripts/spin/main.luau",
            "local events = require('@sf/events')\n\
             events.on('spin', function() while true do end end)\n\
             events.on('sneaky', function() while true do pcall(function() while true do end end) end end)\n\
             events.on('recurse', function() local function f() return f() + 1 end return f() end)",
        )],
        vec![config],
    );
    for name in ["spin", "sneaky"] {
        let t = Instant::now();
        rt.dispatch(name, StateValue::Nil);
        assert!(
            t.elapsed() < Duration::from_millis(500),
            "{name} took {:?}",
            t.elapsed()
        );
    }
    rt.dispatch("recurse", StateValue::Nil); // stack overflow is an ordinary error
    let diags = rt.take_diagnostics();
    let kinds: Vec<_> = diags.iter().filter_map(|d| d.kind).collect();
    assert_eq!(
        kinds,
        [FaultKind::Timeout, FaultKind::Timeout, FaultKind::Error],
        "{diags:?}"
    );
    assert_eq!(rt.context("spin").unwrap().stats().aborts, 2);
    // A third timeout suspends (3 aborts within 60 s).
    rt.dispatch("spin", StateValue::Nil);
    assert!(matches!(
        rt.context("spin").unwrap().status(),
        Status::Suspended { .. }
    ));
}

#[test]
fn errors_are_isolated_and_repeat_offenders_suspended() {
    let mut rt = runtime(
        &[
            (
                "scripts/bad/main.luau",
                "require('@sf/events').on('tick', function() error('boom') end)",
            ),
            (
                "scripts/good/main.luau",
                "local s = sf.storage() s.n = 0 require('@sf/events').on('tick', function() s.n += 1 end)",
            ),
        ],
        vec![
            ContextConfig::for_mod("bad"),
            ContextConfig::for_mod("good"),
        ],
    );
    rt.set_policy(FaultPolicy {
        max_errors: 3,
        ..FaultPolicy::default()
    });
    for _ in 0..5 {
        rt.dispatch("tick", StateValue::Nil);
    }
    assert!(matches!(
        rt.context("bad").unwrap().status(),
        Status::Suspended { .. }
    ));
    assert_eq!(
        rt.context("bad").unwrap().stats().errors,
        3,
        "no calls after suspension"
    );
    assert_eq!(
        storage_value(rt.context("good").unwrap(), "n"),
        Some(StateValue::Number(5.0))
    );
    let diags = rt.take_diagnostics();
    assert!(diags[0].message.contains("boom"), "{diags:?}");
    assert!(diags.iter().all(|d| d.context == "bad"));
}

#[test]
fn fault_window_slides() {
    let policy = FaultPolicy {
        max_errors: 2,
        window: Duration::from_secs(60),
        ..FaultPolicy::default()
    };
    let mut log = FaultLog::default();
    let t0 = Instant::now();
    assert_eq!(log.record(FaultKind::Error, t0, &policy), Verdict::Continue);
    assert_eq!(
        log.record(FaultKind::Error, t0 + Duration::from_secs(61), &policy),
        Verdict::Continue
    );
    assert!(matches!(
        log.record(FaultKind::Error, t0 + Duration::from_secs(62), &policy),
        Verdict::Suspend(_)
    ));
    assert!(matches!(
        log.record(FaultKind::Panic, t0, &policy),
        Verdict::Fail(_)
    ));
}

#[test]
fn state_round_trip_and_rejections() {
    let mut rt = one("m", "");
    let policy = *rt.policy();
    let ctx = rt.context_mut("m").unwrap();
    ctx.lua()
        .globals()
        .raw_set("h", Handle(0xdead_beef_u128 << 64))
        .unwrap();
    let ok: Option<()> = ctx.eval(
        "local s = sf.storage() s.b = true s.n = 1.5 s.i = 42 s.str = 'h\\0i' s.v = vector.create(1, 2, 3) \
         s.h = h s.list = {1, 2, {x = 'y'}} s.map = {[true] = 1, [7] = 'seven', deep = {deeper = {}}}",
        &policy,
    );
    assert!(ok.is_some(), "{:?}", ctx.last_error());
    let snap = ctx.snapshot().unwrap();
    let bytes = snap.to_bytes().unwrap();
    assert_eq!(ContextSnapshot::from_bytes(&bytes).unwrap(), snap);

    // Restore into a fresh runtime: same data, handles are handles again.
    let mut rt2 = one("m", "");
    rt2.restore("m", &bytes).unwrap();
    let ctx2 = rt2.context_mut("m").unwrap();
    assert_eq!(ctx2.snapshot().unwrap().storage, snap.storage);
    let ok = ctx2.eval::<bool>("local s = sf.storage() return tostring(s.h):sub(1, 6) == 'Handle' and s.list[3].x == 'y' and s.v.y == 2", &policy);
    assert_eq!(ok, Some(true), "{:?}", ctx2.last_error());

    // Rejections name the path.
    let ctx = rt.context_mut("m").unwrap();
    for (code, expected) in [
        (
            "sf.storage().quests = {1, 2, {cb = print}}",
            "storage.quests[3].cb is a function",
        ),
        (
            "local t = {} t.self = t sf.storage().cyc = t",
            "storage.cyc.self is a table reached twice",
        ),
        (
            "local t = {} sf.storage().a = t sf.storage().b = t",
            "is a table reached twice",
        ),
        (
            "sf.storage().m = setmetatable({}, {})",
            "storage.m has a metatable",
        ),
        (
            "sf.storage().co = coroutine.create(print)",
            "storage.co is a coroutine",
        ),
    ] {
        let fresh: Option<()> = ctx.eval(
            &format!("for k in sf.storage() do sf.storage()[k] = nil end {code}"),
            &policy,
        );
        assert!(fresh.is_some());
        let err = ctx.snapshot().unwrap_err().to_string();
        assert!(err.contains(expected), "{err} should contain {expected}");
    }
}

#[test]
fn timers_survive_save_restore_and_fire_once() {
    let main = "local timers = require('@sf/timers')\n\
                local s = sf.storage() s.fired = s.fired or 0\n\
                timers.register('ping', function(p) sf.storage().fired += p.n end)\n\
                require('@sf/events').on('arm', function() timers.after(2, 'ping', { n = 5 }) end)";
    let mut rt = one("m", main);
    rt.dispatch("arm", StateValue::Nil);
    rt.update(1.0);
    let bytes = rt.snapshot("m").unwrap();

    let mut rt2 = one("m", main);
    rt2.restore("m", &bytes).unwrap();
    rt2.update(0.5);
    assert_eq!(
        storage_value(rt2.context("m").unwrap(), "fired"),
        Some(StateValue::Number(0.0))
    );
    rt2.update(0.6);
    assert_eq!(
        storage_value(rt2.context("m").unwrap(), "fired"),
        Some(StateValue::Number(5.0))
    );
    rt2.update(10.0);
    assert_eq!(
        storage_value(rt2.context("m").unwrap(), "fired"),
        Some(StateValue::Number(5.0))
    );

    // Registration is start-only; unknown names fail at schedule time.
    assert!(eval_fails(
        &mut rt2,
        "m",
        "require('@sf/timers').register('late', print)"
    ));
    assert!(eval_fails(
        &mut rt2,
        "m",
        "require('@sf/timers').after(1, 'nope')"
    ));
    assert!(eval_fails(
        &mut rt2,
        "m",
        "require('@sf/timers').after(1, 'ping', { f = print })"
    ));
}

#[test]
fn every_repeats_and_cancel_stops() {
    let main = "local timers = require('@sf/timers')\n\
                sf.storage().n = 0\n\
                timers.register('tick', function() sf.storage().n += 1 end)\n\
                local id = timers.every(1, 'tick')\n\
                require('@sf/events').on('stop', function() timers.cancel(id) end)";
    let mut rt = one("m", main);
    for _ in 0..3 {
        rt.update(1.0);
    }
    rt.dispatch("stop", StateValue::Nil);
    rt.update(1.0);
    assert_eq!(
        storage_value(rt.context("m").unwrap(), "n"),
        Some(StateValue::Number(3.0))
    );
}

#[test]
fn events_between_mods_next_update_in_load_order() {
    let mut rt = runtime(
        &[
            (
                "scripts/a/main.luau",
                "local ev = require('@sf/events')\n\
                 ev.on('activated', function(p) ev.emit('saw', { who = p.who }) return 'handled' end)",
            ),
            (
                "scripts/b/main.luau",
                "local ev = require('@sf/events') sf.storage().log = {}\n\
                 ev.on('a:saw', function(p) table.insert(sf.storage().log, p.who) end)",
            ),
        ],
        vec![ContextConfig::for_mod("a"), ContextConfig::for_mod("b")],
    );
    let payload = StateValue::Table(vec![(
        StateKey::Str(b"who".to_vec()),
        StateValue::Str(b"door".to_vec()),
    )]);
    let d = rt.dispatch("activated", payload);
    assert!(d.handled);
    assert_eq!(
        storage_value(rt.context("b").unwrap(), "log"),
        Some(StateValue::table()),
        "not delivered yet"
    );
    let report = rt.update(0.016);
    assert_eq!(report.events, 1);
    let log = storage_value(rt.context("b").unwrap(), "log").unwrap();
    assert_eq!(
        log,
        StateValue::Table(vec![(StateKey::Int(1), StateValue::Str(b"door".to_vec()))])
    );
    // No spoofing another namespace.
    assert!(eval_fails(
        &mut rt,
        "a",
        "require('@sf/events').emit('b:x')"
    ));
}

fn scenario() -> Vec<u8> {
    let main = "local ev = require('@sf/events') local timers = require('@sf/timers')\n\
                local s = sf.storage() s.rolls = {} s.sum = 0\n\
                timers.register('roll', function(p) table.insert(s.rolls, math.random(1, 100) + p.k) end)\n\
                ev.on('update', function(dt) s.sum += sf.random() * dt if #s.rolls < 20 then timers.after(0.05, 'roll', { k = #s.rolls }) end end)";
    let mut rt = one("det", main);
    for _ in 0..60 {
        rt.update(1.0 / 60.0);
    }
    rt.snapshot("det").unwrap()
}

#[test]
fn determinism_same_inputs_same_state() {
    let a = scenario();
    assert_eq!(a, scenario());
    let snap = ContextSnapshot::from_bytes(&a).unwrap();
    match snap.storage.get("rolls") {
        Some(StateValue::Table(rolls)) => assert!(rolls.len() >= 20, "{}", rolls.len()),
        other => panic!("{other:?}"),
    }
}

#[test]
fn api_version_gating() {
    let mut config = ContextConfig::for_mod("future");
    config.api = semver::VersionReq::parse("^0.2").unwrap();
    let mut rt = ScriptRuntime::default();
    let diags = rt.start(vec![config], vfs(&[("scripts/future/main.luau", "")]));
    assert!(
        diags[0].message.contains("requires script API ^0.2"),
        "{diags:?}"
    );
    assert!(rt.context("future").is_none());
}

#[test]
fn reload_keeps_state_and_falls_back_on_error() {
    let v1 = "local s = sf.storage() s.count = s.count or 0\n\
              require('@sf/timers').register('t', function() end)\n\
              require('@sf/events').on('door', function() s.count += 1 s.msg = 'v1' end)";
    let v2 = v1.replace("'v1'", "'v2'");
    let mut rt = one("m", v1);
    rt.dispatch("door", StateValue::Nil);
    rt.dispatch("door", StateValue::Nil);
    assert!(eval_bool(
        &mut rt,
        "m",
        "require('@sf/timers').after(5, 't') return true"
    ));

    rt.reload("m", Some(vfs(&[("scripts/m/main.luau", &v2)])))
        .unwrap();
    rt.dispatch("door", StateValue::Nil);
    let ctx = rt.context("m").unwrap();
    assert_eq!(storage_value(ctx, "count"), Some(StateValue::Number(3.0)));
    assert_eq!(
        storage_value(ctx, "msg"),
        Some(StateValue::Str(b"v2".to_vec()))
    );
    assert_eq!(
        ctx.snapshot().unwrap().timers.timers.len(),
        1,
        "timer carried over"
    );

    // A broken edit keeps the old code running.
    let err = rt.reload(
        "m",
        Some(vfs(&[("scripts/m/main.luau", "this is not luau")])),
    );
    assert!(err.is_err());
    rt.dispatch("door", StateValue::Nil);
    assert_eq!(
        storage_value(rt.context("m").unwrap(), "count"),
        Some(StateValue::Number(4.0))
    );
}

#[test]
fn start_failure_is_contained() {
    let mut rt = ScriptRuntime::default();
    let diags = rt.start(
        vec![
            ContextConfig::for_mod("broken"),
            ContextConfig::for_mod("fine"),
        ],
        vfs(&[
            ("scripts/broken/main.luau", "error('nope')"),
            ("scripts/fine/main.luau", "sf.storage().ok = true"),
        ]),
    );
    assert_eq!(diags.len(), 1);
    assert!(diags[0].message.contains("nope"), "{diags:?}");
    assert!(matches!(
        rt.context("broken").unwrap().status(),
        Status::Failed { .. }
    ));
    assert!(rt.context("fine").unwrap().is_running());
}

#[test]
fn shared_legacy_context_spans_mods() {
    let mut config = ContextConfig::for_mod("legacy");
    config.mods = vec!["base".into(), "plugin".into()];
    config.entry_points = vec![
        "scripts/base/main.luau".into(),
        "scripts/plugin/main.luau".into(),
    ];
    config.limits = Limits::SHARED;
    let rt = runtime(
        &[
            ("scripts/base/main.luau", "shared_var = 1"),
            (
                "scripts/plugin/main.luau",
                "sf.storage().seen = shared_var + require('../base/lib').v",
            ),
            ("scripts/base/lib.luau", "return { v = 1 }"),
        ],
        vec![config],
    );
    assert_eq!(
        storage_value(rt.context("legacy").unwrap(), "seen"),
        Some(StateValue::Number(2.0))
    );
}

#[test]
fn type_definitions_list_builtins() {
    let defs = ScriptRuntime::default().type_definitions();
    for name in ["@sf/core", "@sf/events", "@sf/timers"] {
        assert!(defs.contains(name), "{defs}");
    }
}
