//! API v0.1 (L3): entity scripts, `on_load`, handle methods, interfaces, type definitions.

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

fn eval_bool(rt: &mut ScriptRuntime, id: &str, code: &str) -> bool {
    let policy = *rt.policy();
    rt.context_mut(id)
        .unwrap()
        .eval::<bool>(code, &policy)
        .unwrap_or_else(|| panic!("eval failed: {code}: {:?}", rt.context(id).unwrap().last_error()))
}

fn eval_fails(rt: &mut ScriptRuntime, id: &str, code: &str) -> bool {
    let policy = *rt.policy();
    rt.context_mut(id).unwrap().eval::<Value>(code, &policy).is_none()
}

/// Exposes the entity script API (in the game, `@sf/world` does) and a handle method.
struct EntityTestModule;

impl ApiModule for EntityTestModule {
    fn name(&self) -> &'static str {
        "@test/entity"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &mlua::Lua, _cx: &InstallCx<'_>) -> mlua::Result<mlua::Table> {
        let t = lua.create_table()?;
        install_entity_api(lua, &t)?;
        t.set("handle", lua.create_function(|_, n: u32| Ok(Handle(u128::from(n))))?)?;
        handle_methods(lua)?.raw_set(
            "double",
            lua.create_function(|_, h: mlua::AnyUserData| {
                #[allow(clippy::cast_precision_loss)]
                Ok(h.borrow::<Handle>()?.0 as f64 * 2.0)
            })?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare sf_test: {}\n"
    }
    fn handle_types(&self) -> &'static str {
        "  function double(self): number\n"
    }
}

fn runtime(files: &[(&str, &str)], ids: &[&str]) -> ScriptRuntime {
    let mut rt = ScriptRuntime::default();
    rt.register_module(Box::new(EntityTestModule));
    let diags = rt.start(ids.iter().map(|id| ContextConfig::for_mod(id)).collect(), vfs(files));
    assert!(diags.is_empty(), "start diagnostics: {diags:?}");
    rt
}

const LAMP: &str = r#"
local e = require("@test/entity")
local events = require("@sf/events")
local sf = require("@sf/core")
e.define_script("lamp", {
    activated = function(h, payload, state)
        state.uses = (state.uses or 0) + 1
        return if state.uses % 2 == 0 then "handled" else nil
    end,
    tick = function(h, dt, state) state.t = (state.t or 0) + dt end,
})
events.on("activated", function(p)
    local s = sf.storage()
    -- the entity script ran first: its state is already updated
    local st = p.target:state()
    s.order = (s.order or "") .. "g" .. tostring(st and st.uses)
end)
sf.on_load(function(info) sf.storage().loads = (sf.storage().loads or "") .. info.reason end)
"#;

fn target(n: u128) -> StateValue {
    StateValue::Table(vec![(StateKey::Str(b"target".to_vec()), StateValue::Handle(Handle(n)))])
}

fn text(v: Option<StateValue>) -> String {
    match v {
        Some(StateValue::Str(s)) => String::from_utf8(s).unwrap(),
        other => format!("{other:?}"),
    }
}

#[test]
fn entity_scripts_run_before_global_handlers_and_keep_state() {
    let mut rt = runtime(&[("scripts/m/main.luau", LAMP)], &["m"]);
    assert!(eval_bool(
        &mut rt,
        "m",
        r#"local e = require("@test/entity")
        local s = e.attach(e.handle(7), "lamp"); return type(s) == "table" and e.script_of(e.handle(7)) == "lamp""#
    ));
    // Attaching an undefined script fails; `define_script` only at start.
    assert!(eval_fails(&mut rt, "m", r#"local e = require("@test/entity"); e.attach(e.handle(8), "nope")"#));
    assert!(eval_fails(&mut rt, "m", r#"require("@test/entity").define_script("x", {})"#));
    assert!(!rt.activate(Handle(7), target(7)).handled);
    assert!(rt.activate(Handle(7), target(7)).handled, "second use returns handled");
    assert!(!rt.activate(Handle(9), target(9)).handled);
    assert_eq!(text(storage_value(rt.context("m").unwrap(), "order")), "g1g2gnil");
    // Ticks only for loaded handles.
    assert_eq!(rt.tick_entities(0.5, &|h| h == Handle(7)), 1);
    assert_eq!(rt.tick_entities(0.5, &|_| false), 0);
    assert_eq!(rt.attached(), vec![Handle(7)]);
    assert!(eval_bool(
        &mut rt,
        "m",
        r#"local e = require("@test/entity"); local h = e.handle(7)
        return h:state().t == 0.5 and h:state().uses == 2 and e.state(e.handle(9)) == nil"#
    ));
    // State survives a snapshot round trip into a fresh context.
    let bytes = rt.snapshot("m").unwrap();
    let mut fresh = runtime(&[("scripts/m/main.luau", LAMP)], &["m"]);
    fresh.restore("m", &bytes).unwrap();
    assert!(eval_bool(&mut fresh, "m", r#"local e = require("@test/entity"); return e.state(e.handle(7)).uses == 2"#));
    assert!(eval_bool(
        &mut fresh,
        "m",
        r#"local e = require("@test/entity"); return e.detach(e.handle(7)) and e.state(e.handle(7)) == nil"#
    ));
    // A restore whose script the code no longer defines drops the attachment with a notice.
    let mut other = runtime(&[("scripts/m/main.luau", "")], &["m"]);
    other.restore("m", &bytes).unwrap();
    assert!(other.attached().is_empty());
    assert!(other.take_diagnostics().iter().any(|d| d.message.contains("no longer defined")));
}

#[test]
fn on_load_runs_once_per_start_reload_and_restore() {
    let mut rt = runtime(&[("scripts/m/main.luau", LAMP)], &["m"]);
    // Queued at start; nothing ran yet (the host runs it inside a slot).
    assert_eq!(storage_value(rt.context("m").unwrap(), "loads"), Some(StateValue::Nil));
    assert_eq!(rt.run_loads(), 1);
    assert_eq!(rt.run_loads(), 0, "runs once");
    rt.reload("m", None).unwrap();
    rt.update(0.1);
    let bytes = rt.snapshot("m").unwrap();
    rt.restore("m", &bytes).unwrap();
    rt.update(0.1);
    assert_eq!(text(storage_value(rt.context("m").unwrap(), "loads")), "startreloadrestore");
    // Not outside start.
    assert!(eval_fails(&mut rt, "m", r#"require("@sf/core").on_load(function() end)"#));
}

#[test]
fn core_settings_version_and_game_time() {
    let mut rt = ScriptRuntime::default();
    let mut config = ContextConfig::for_mod("m");
    config.version = semver::Version::new(1, 2, 3);
    config.settings = StateValue::Table(vec![(StateKey::Str(b"volume".to_vec()), StateValue::Number(0.5))]);
    assert!(rt.start(vec![config], vfs(&[("scripts/m/main.luau", "")])).is_empty());
    assert!(eval_bool(&mut rt, "m", r#"local sf = require("@sf/core")
        local s = sf.settings(); s.volume = 9
        return sf.version == "1.2.3" and sf.settings().volume == 0.5 and sf.game_time() == nil"#));
    rt.set_game_time(Some(13.5));
    assert!(eval_bool(&mut rt, "m", r#"return require("@sf/core").game_time() == 13.5"#));
}

#[test]
fn handle_methods_and_unknown_method_errors() {
    let mut rt = runtime(&[("scripts/m/main.luau", "")], &["m"]);
    assert!(eval_bool(
        &mut rt,
        "m",
        r#"local h = require("@test/entity").handle(21); return h:double() == 42 and h:id() == string.rep("0", 30) .. "15""#
    ));
    assert!(eval_fails(&mut rt, "m", r#"local h = require("@test/entity").handle(1); return h:nope()"#));
    // Scripts can't add methods.
    assert!(eval_fails(&mut rt, "m", r#"local h = require("@test/entity").handle(1); getmetatable(h).__index.x = 1"#));
}

#[test]
fn interfaces_copy_plain_data_between_mods() {
    let a = r#"
local interfaces = require("@sf/interfaces")
local sf = require("@sf/core")
interfaces.export("api", {
    add = function(x, y) sf.storage().calls = (sf.storage().calls or 0) + 1; return x + y, { sum = x + y } end,
    boom = function() error("boom") end,
    spin = function() while true do end end,
    back = function() return require("@sf/interfaces").get("b", "api").ping() end,
})
"#;
    let b = r#"
local interfaces = require("@sf/interfaces")
interfaces.export("api", {
    ping = function() return "pong" end,
    call_a = function() return require("@sf/interfaces").get("a", "api").back() end,
})
"#;
    let mut rt = runtime(&[("scripts/a/main.luau", a), ("scripts/b/main.luau", b)], &["a", "b"]);
    assert!(eval_bool(
        &mut rt,
        "b",
        r#"local api = require("@sf/interfaces").get("a", "api")
        local n, t = api.add(2, 3); return n == 5 and t.sum == 5"#
    ));
    assert_eq!(storage_value(rt.context("a").unwrap(), "calls"), Some(StateValue::Number(1.0)));
    // Unknown export: nil. Non-plain arguments and callee errors: errors in the caller.
    assert!(eval_bool(&mut rt, "b", r#"return require("@sf/interfaces").get("a", "nope") == nil"#));
    assert!(eval_bool(
        &mut rt,
        "b",
        r#"local api = require("@sf/interfaces").get("a", "api")
        return not pcall(api.add, function() end, 1) and not pcall(api.boom)"#
    ));
    // The callee's watchdog stops a runaway export.
    let t = Instant::now();
    assert!(eval_bool(&mut rt, "b", r#"return not pcall(require("@sf/interfaces").get("a", "api").spin)"#));
    assert!(t.elapsed() < Duration::from_secs(5));
    // a → b is fine; b → a → b is re-entrant.
    assert!(eval_bool(&mut rt, "a", r#"return require("@sf/interfaces").get("b", "api").ping() == "pong""#));
    assert!(eval_bool(
        &mut rt,
        "b",
        r#"local ok, e = pcall(require("@sf/interfaces").get("b", "api").call_a)
        return not ok and string.find(tostring(e), "re%-entrant") ~= nil"#
    ));
    // Still usable after the failed cycle; export only at start.
    assert!(eval_bool(&mut rt, "a", r#"return require("@sf/interfaces").get("b", "api").ping() == "pong""#));
    assert!(eval_fails(&mut rt, "a", r#"require("@sf/interfaces").export("late", {})"#));
    // A reload keeps the export reachable by name (new code).
    rt.reload("a", None).unwrap();
    assert!(eval_bool(&mut rt, "b", r#"return (require("@sf/interfaces").get("a", "api").add(1, 1)) == 2"#));
}

#[test]
fn type_definitions_declare_handle_methods_and_modules() {
    let mut rt = ScriptRuntime::default();
    rt.register_module(Box::new(EntityTestModule));
    let defs = rt.type_definitions();
    assert!(defs.contains("declare class Handle") && defs.contains("function double(self): number"), "{defs}");
    for name in ["sf_core", "sf_events", "sf_timers", "sf_interfaces", "on_load", "LoadInfo"] {
        assert!(defs.contains(name), "{name} missing:\n{defs}");
    }
}
