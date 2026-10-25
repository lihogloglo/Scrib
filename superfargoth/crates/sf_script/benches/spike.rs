//! Spike L1 (scripting.md "Performance budget" and "Spike results"). Plain `Instant` timing,
//! median of rounds; no criterion (heavy dependency for a handful of numbers).
//!
//! `cargo bench -p sf_script` (default features: `send`), then
//! `cargo bench -p sf_script --no-default-features` (`NonSend`) and `--features jit`.

use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

use mlua::{Function, Lua, LuaOptions, StdLib, VmState};
use sf_script::{
    ContextConfig, ScriptRuntime, ScriptSource, StateKey, StateValue, Watchdog, install_every_tick,
};
use sf_vfs::{MemSource, Vfs};

fn vfs(files: &[(String, String)]) -> Arc<dyn ScriptSource> {
    let mut mem = MemSource::new("bench");
    for (p, t) in files {
        mem.insert(p, t.as_bytes().to_vec()).unwrap();
    }
    Arc::new(Vfs::builder().mount(mem).build())
}

/// Median nanoseconds per iteration over `rounds` rounds of `iters` iterations.
#[allow(clippy::cast_precision_loss)]
fn median_ns(iters: u32, rounds: usize, mut f: impl FnMut(u32)) -> f64 {
    let mut v: Vec<f64> = (0..rounds)
        .map(|_| {
            let t = Instant::now();
            f(iters);
            t.elapsed().as_nanos() as f64 / f64::from(iters)
        })
        .collect();
    v.sort_by(f64::total_cmp);
    v[rounds / 2]
}

fn row(name: &str, value: f64, unit: &str, note: &str) {
    println!("| {name} | {value:.3} {unit} | {note} |");
}

fn one_mod(main: &str) -> ScriptRuntime {
    let mut rt = ScriptRuntime::default();
    let diags = rt.start(
        vec![ContextConfig::for_mod("m")],
        vfs(&[("scripts/m/main.luau".into(), main.into())]),
    );
    assert!(diags.is_empty(), "{diags:?}");
    rt
}

fn calls() {
    let mut rt = one_mod("");
    let policy = *rt.policy();
    let ctx = rt.context_mut("m").unwrap();
    let noop: Function = ctx.lua().load("return function() end").eval().unwrap();
    let add: Function = ctx
        .lua()
        .load("return function(a) return a + 1 end")
        .eval()
        .unwrap();
    row(
        "Rust→Luau call, empty fn, raw `Function::call`",
        median_ns(100_000, 21, |n| {
            (0..n).for_each(|_| noop.call::<()>(()).unwrap());
        }),
        "ns",
        "",
    );
    row(
        "Rust→Luau call, f(number) → number, raw",
        median_ns(100_000, 21, |n| {
            (0..n).for_each(|i| {
                black_box(add.call::<f64>(f64::from(i)).unwrap());
            });
        }),
        "ns",
        "",
    );
    row(
        "Rust→Luau call, guarded (watchdog arm/disarm, stats, catch_unwind)",
        median_ns(100_000, 21, |n| {
            (0..n).for_each(|_| ctx.call::<()>(&noop, (), "bench", &policy).unwrap());
        }),
        "ns",
        "what every handler call pays",
    );

    let lua = ctx.lua();
    let get = lua.create_function(|_, ()| Ok(1.0)).unwrap();
    let get_arg = lua.create_function(|_, x: f64| Ok(x * 2.0)).unwrap();
    let looper: Function = lua
        .load("return function(f, n) local x = 0 for i = 1, n do x += f(i) end return x end")
        .eval()
        .unwrap();
    let empty: Function = lua
        .load("return function(n) local x = 0 for i = 1, n do x += i end return x end")
        .eval()
        .unwrap();
    let base = median_ns(1_000_000, 11, |n| {
        black_box(empty.call::<f64>(n).unwrap());
    });
    let getter = median_ns(1_000_000, 11, |n| {
        black_box(looper.call::<f64>((&get, n)).unwrap());
    });
    let with_arg = median_ns(1_000_000, 11, |n| {
        black_box(looper.call::<f64>((&get_arg, n)).unwrap());
    });
    row(
        "Luau→Rust call, getter () → number (loop overhead subtracted)",
        getter - base,
        "ns",
        "",
    );
    row(
        "Luau→Rust call, f(number) → number",
        with_arg - base,
        "ns",
        "",
    );
    row("Luau loop iteration (baseline)", base, "ns", "");
}

#[allow(clippy::too_many_lines, reason = "one linear measurement script")]
fn memory_and_start() {
    let mut rt = one_mod("");
    let ctx = rt.context_mut("m").unwrap();
    ctx.lua().gc_collect().unwrap();
    ctx.lua().gc_collect().unwrap();
    #[allow(clippy::cast_precision_loss)]
    let kib = ctx.used_memory() as f64 / 1024.0;
    row(
        "Empty context, Luau heap after full GC",
        kib,
        "KiB",
        "sandbox + `@sf/core,events,timers` installed",
    );
    let bare = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).unwrap();
    bare.gc_collect().unwrap();
    #[allow(clippy::cast_precision_loss)]
    let bare_kib = bare.used_memory() as f64 / 1024.0;
    row(
        "Bare `Lua::new` (all safe libs), heap after GC",
        bare_kib,
        "KiB",
        "reference",
    );

    let main = "local ev = require('@sf/events') local timers = require('@sf/timers')\n\
                local s = sf.storage() s.count = 0\n\
                timers.register('t', function() end)\n\
                for i = 1, 20 do ev.on('tick', function(dt) s.count += 1 end) end";
    let files: Vec<(String, String)> = (0..50)
        .map(|i| (format!("scripts/mod{i}/main.luau"), main.to_owned()))
        .collect();
    let source = vfs(&files);
    let configs: Vec<ContextConfig> = (0..50)
        .map(|i| ContextConfig::for_mod(&format!("mod{i}")))
        .collect();
    let t = Instant::now();
    let mut rt = ScriptRuntime::default();
    assert!(rt.start(configs, source).is_empty());
    let start_ms = t.elapsed().as_secs_f64() * 1e3;
    row(
        "Start 50 contexts (20 handlers each)",
        start_ms,
        "ms",
        "budget ≤ 200 ms",
    );
    let total: usize = rt
        .contexts()
        .iter()
        .map(sf_script::ScriptContext::used_memory)
        .sum();
    #[allow(clippy::cast_precision_loss)]
    let per = total as f64 / 50.0 / 1024.0;
    row("Per-context heap with 20 handlers (no GC)", per, "KiB", "");

    // Many-handler dispatch: 50 contexts × 20 handlers = 1000 calls per event.
    let ns = median_ns(100, 11, |n| {
        for _ in 0..n {
            black_box(rt.dispatch("tick", StateValue::Number(0.016)));
        }
    });
    row(
        "Dispatch to 1000 handlers (50 ctx × 20), per event",
        ns / 1e3,
        "µs",
        "",
    );
    row(
        "… per handler call",
        ns / 1000.0,
        "ns",
        "includes payload conversion + guard",
    );
    let payload = StateValue::Table(vec![
        (
            StateKey::Str(b"actor".to_vec()),
            StateValue::Handle(sf_script::Handle(7)),
        ),
        (
            StateKey::Str(b"target".to_vec()),
            StateValue::Handle(sf_script::Handle(9)),
        ),
        (
            StateKey::Str(b"kind".to_vec()),
            StateValue::Str(b"door".to_vec()),
        ),
    ]);
    let ns = median_ns(100, 11, |n| {
        for _ in 0..n {
            black_box(rt.dispatch("tick", payload.clone()));
        }
    });
    row(
        "… per handler call, 3-field table payload with 2 handles",
        ns / 1000.0,
        "ns",
        "payload built once per context",
    );
    let lua = rt.contexts()[0].lua();
    let conv = median_ns(10_000, 11, |n| {
        for _ in 0..n {
            black_box(payload.to_lua(lua).unwrap());
        }
    });
    let limit = lua.set_memory_limit(0).unwrap();
    let conv_free = median_ns(10_000, 11, |n| {
        for _ in 0..n {
            black_box(payload.to_lua(lua).unwrap());
        }
    });
    lua.set_memory_limit(limit).unwrap();
    row(
        "Payload → Lua (3 fields, 2 handles), memory limit set",
        conv,
        "ns",
        "mlua protects every allocation",
    );
    row("… same, limit lifted (`unlimited`)", conv_free, "ns", "");
    let ns = median_ns(100, 11, |n| {
        for _ in 0..n {
            black_box(rt.dispatch("nobody", StateValue::Nil));
        }
    });
    row(
        "Dispatch an event nobody handles, 50 contexts",
        ns / 1e3,
        "µs",
        "",
    );
}

fn watchdog() {
    let code = "return function(n) local x = 0 local function f(a) return a + 1 end for i = 1, n do x = f(x) end return x end";
    let make = || {
        let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).unwrap();
        lua.sandbox(true).unwrap();
        let f: Function = lua.load(code).eval().unwrap();
        (lua, f)
    };
    let (_lua0, f0) = make();
    let (lua1, f1) = make();
    lua1.set_interrupt(|_| Ok(VmState::Continue));
    let (lua2, f2) = make();
    let wd2 = Watchdog::new();
    install_every_tick(&lua2, &wd2);
    wd2.arm(std::time::Duration::from_secs(600));
    let (lua3, f3) = make();
    let wd3 = Watchdog::new();
    wd3.install(&lua3);
    wd3.arm(std::time::Duration::from_secs(600));
    // Interleaved rounds, best median per variant: the machine's clock speed drifts.
    let mut best = [f64::MAX; 4];
    for _ in 0..5 {
        for (i, f) in [&f0, &f1, &f2, &f3].into_iter().enumerate() {
            let ns = median_ns(1_000_000, 5, |n| {
                black_box(f.call::<f64>(n).unwrap());
            });
            best[i] = best[i].min(ns);
        }
    }
    let [none, empty_cb, every, nth] = best;
    let pct = |x: f64| format!("+{:.0} %", (x / none - 1.0) * 100.0);
    row(
        "`Instant::now()`",
        median_ns(1_000_000, 11, |n| {
            (0..n).for_each(|_| {
                black_box(Instant::now());
            });
        }),
        "ns",
        "this machine's clock read",
    );
    row(
        "Call-heavy loop iteration, no interrupt",
        none,
        "ns",
        "2 safepoints/iteration (call + back-edge)",
    );
    row(
        "… interrupt installed, callback returns at once",
        empty_cb,
        "ns",
        &pct(empty_cb),
    );
    row(
        "… interrupt, clock read every safepoint",
        every,
        "ns",
        &pct(every),
    );
    row(
        &format!(
            "… interrupt, clock read every {}th (shipped)",
            Watchdog::CHECK_EVERY
        ),
        nth,
        "ns",
        &pct(nth),
    );
}

fn world_guard() {
    struct Guard(Option<Vec<f64>>);
    // (a) `Lua::scope`: borrow non-'static host data for one slot; scoped functions are created
    // per slot. (b) Guard in app data: the slot moves the data in and out; API functions borrow it.
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).unwrap();
    let handler: Function = lua
        .load("return function(api, n) local x = 0 for i = 1, n do x += api.get(i % 100 + 1) end return x end")
        .eval()
        .unwrap();
    let mut world: Vec<f64> = (0..100).map(f64::from).collect();
    let api = lua.create_table().unwrap();
    for calls in [1u32, 100] {
        let ns = median_ns(10_000, 11, |n| {
            for _ in 0..n {
                lua.scope(|s| {
                    let w = &world;
                    api.raw_set("get", s.create_function(move |_, i: usize| Ok(w[i - 1]))?)?;
                    black_box(handler.call::<f64>((&api, calls))?);
                    Ok(())
                })
                .unwrap();
            }
        });
        row(
            &format!("World guard (a) `scope`, slot with {calls} API calls"),
            ns,
            "ns",
            "",
        );
    }
    lua.set_app_data(Guard(None));
    let get = lua
        .create_function(|lua, i: usize| {
            let g = lua
                .app_data_ref::<Guard>()
                .ok_or_else(|| mlua::Error::runtime("no guard"))?;
            g.0.as_ref()
                .map(|w| w[i - 1])
                .ok_or_else(|| mlua::Error::runtime("outside a script slot"))
        })
        .unwrap();
    api.raw_set("get", get).unwrap();
    for calls in [1u32, 100] {
        let ns = median_ns(10_000, 11, |n| {
            for _ in 0..n {
                lua.app_data_mut::<Guard>().unwrap().0 = Some(std::mem::take(&mut world));
                black_box(handler.call::<f64>((&api, calls)).unwrap());
                world = lua.app_data_mut::<Guard>().unwrap().0.take().unwrap();
            }
        });
        row(
            &format!("World guard (b) app-data guard, slot with {calls} API calls"),
            ns,
            "ns",
            "",
        );
    }
}

fn numeric() {
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).unwrap();
    let f: Function = lua
        .load("return function(n) local s = 0 for i = 1, n do s += math.sqrt(i) * 0.5 + (i % 7) end return s end")
        .eval()
        .unwrap();
    row(
        "Numeric loop iteration (sqrt, mul, mod)",
        median_ns(2_000_000, 11, |n| {
            black_box(f.call::<f64>(n).unwrap());
        }),
        "ns",
        if cfg!(feature = "jit") {
            "native codegen ON"
        } else {
            "interpreter"
        },
    );
}

fn state_io() {
    let main = "local s = sf.storage() s.items = {}\n\
                for i = 1, 20000 do s.items[i] = { id = 'item_' .. i, count = i, pos = vector.create(i, 2, 3), tags = { 'a', 'b' } } end";
    let mut rt = one_mod(main);
    let t = Instant::now();
    let snap = rt.context("m").unwrap().snapshot().unwrap();
    let walk = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let bytes = snap.to_bytes().unwrap();
    let encode = t.elapsed().as_secs_f64() * 1e3;
    let save = walk + encode;
    let values = {
        fn count(v: &StateValue) -> usize {
            match v {
                StateValue::Table(e) => 1 + e.iter().map(|(_, v)| count(v)).sum::<usize>(),
                _ => 1,
            }
        }
        count(&snap.storage)
    };
    #[allow(clippy::cast_precision_loss)]
    let per_value = walk * 1e6 / values as f64;
    row(
        "… of which the Lua walk, per value",
        per_value,
        "ns",
        &format!("{values} values; postcard {encode:.1} ms"),
    );
    let t = Instant::now();
    rt.restore("m", &bytes).unwrap();
    let load = t.elapsed().as_secs_f64() * 1e3;
    #[allow(clippy::cast_precision_loss)]
    let mib = bytes.len() as f64 / (1024.0 * 1024.0);
    row("Snapshot size, 20k small tables", mib, "MiB", "");
    row(
        "Snapshot (walk + postcard)",
        save / mib,
        "ms/MiB",
        "budget ≤ 5 ms/MiB (save + restore)",
    );
    row(
        "Restore (postcard + rebuild tables)",
        load / mib,
        "ms/MiB",
        "",
    );
    let t = Instant::now();
    rt.reload("m", None).unwrap();
    row(
        "Hot reload of that mod (snapshot → new state → entry → restore)",
        t.elapsed().as_secs_f64() * 1e3,
        "ms",
        "budget ≤ 100 ms",
    );
}

fn main() {
    if std::env::args().any(|a| a == "--list") {
        return;
    }
    println!(
        "sf_script spike: send={}, jit={}, debug_assertions={}",
        cfg!(feature = "send"),
        cfg!(feature = "jit"),
        cfg!(debug_assertions)
    );
    println!("| Measure | Value | Note |\n|---|---|---|");
    calls();
    memory_and_start();
    watchdog();
    world_guard();
    numeric();
    state_io();
}
