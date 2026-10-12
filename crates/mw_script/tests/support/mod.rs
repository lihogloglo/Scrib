//! Loads generated modules into a real `sf_script` context (sandboxed shared legacy context, as
//! the game will) with the `mw` runtime skeleton and no host: every non-own function answers its
//! neutral value. Shared by the emitter fixtures and the real-install corpus test.

#![allow(dead_code)]

use std::sync::Arc;

use mw_script::FunctionTable;
use mw_script::emit::{RUNTIME, runtime_functions};
use sf_script::mlua::chunk::ChunkMode;
use sf_script::mlua::{Function, Lua, Table, Value};
use sf_script::{ContextConfig, FaultPolicy, Limits, ScriptRuntime};

pub const CONTEXT: &str = "mwlegacy";

pub struct Harness {
    pub rt: ScriptRuntime,
    pub policy: FaultPolicy,
}

/// What loading one module did.
#[derive(Debug, Default)]
pub struct Outcome {
    /// Compile (syntax) error in the sandboxed state.
    pub compile: Option<String>,
    /// Error running the module constructor, a script's first tick or a result.
    pub run: Option<String>,
    /// The module's definition, kept in the registry under its label (`None` on failure).
    pub ran: bool,
}

impl Harness {
    pub fn new(table: &FunctionTable) -> Self {
        let vfs = sf_vfs::Vfs::builder()
            .mount(sf_vfs::MemSource::new("mem").with(&format!("scripts/{CONTEXT}/main.luau"), ""))
            .build();
        let mut rt = ScriptRuntime::default();
        let policy = FaultPolicy {
            max_aborts: usize::MAX,
            max_errors: usize::MAX,
            ..FaultPolicy::default()
        };
        rt.set_policy(policy);
        let mut config = ContextConfig::for_mod(CONTEXT);
        config.limits = Limits::SHARED;
        let diags = rt.start(vec![config], Arc::new(vfs));
        assert!(diags.is_empty(), "{diags:?}");
        let functions = runtime_functions(table);
        let mut h = Self { rt, policy };
        h.ctx_guarded("runtime", |lua| {
            let runtime: Table = load(lua, RUNTIME, "@scripts/mwlegacy/runtime.luau")?.call(())?;
            let functions: Table = load(lua, &functions, "=functions")?.call(())?;
            let mw: Table = runtime.get::<Function>("new")?.call((Value::Nil, functions))?;
            lua.set_named_registry_value("mw", mw)
        })
        .expect("runtime loads");
        h
    }

    fn ctx_guarded<R>(&mut self, label: &str, body: impl FnOnce(&Lua) -> sf_script::mlua::Result<R>) -> Result<R, String> {
        let policy = self.policy;
        let ctx = self.rt.context_mut(CONTEXT).unwrap();
        let errors_before = ctx.stats().errors + ctx.stats().aborts;
        let out = ctx.guarded(label, &policy, body);
        let ctx = self.rt.context_mut(CONTEXT).unwrap();
        match out {
            Some(r) if ctx.stats().errors + ctx.stats().aborts == errors_before => Ok(r),
            _ => Err(ctx.last_error().unwrap_or("failed").to_owned()),
        }
    }

    /// Compiles `source` in the context's sandboxed state, runs its constructor with `mw`, then
    /// a script's `tick` once (fresh zeroed instance) or every dialogue result once.
    pub fn run_module(&mut self, label: &str, source: &str) -> Outcome {
        let lua = self.rt.context(CONTEXT).unwrap().lua().clone();
        let chunk = match load(&lua, source, &format!("={label}")) {
            Ok(f) => f,
            Err(e) => {
                return Outcome {
                    compile: Some(e.to_string()),
                    ..Outcome::default()
                };
            }
        };
        let label_owned = label.to_owned();
        let run = self.ctx_guarded(label, move |lua| {
            let mw: Table = lua.named_registry_value("mw")?;
            let ctor: Function = chunk.call(())?;
            let def: Table = ctor.call(mw.clone())?;
            if let Ok(tick) = def.get::<Function>("tick") {
                let vars: Table = mw.get::<Function>("new_vars")?.call(def.clone())?;
                let types: Table = def.get("vars")?;
                let handle: Table = mw
                    .get::<Function>("instance")?
                    .call((label_owned.clone(), vars.clone(), types))?;
                tick.call::<()>((handle, vars.clone()))?;
                lua.set_named_registry_value(&format!("vars:{label_owned}"), vars)?;
            } else {
                let results: Table = def.get("results")?;
                let speaker: Table = mw
                    .get::<Function>("instance")?
                    .call(("speaker", lua.create_table()?, lua.create_table()?))?;
                for pair in results.pairs::<Value, Function>() {
                    let (_, f) = pair?;
                    f.call::<()>(speaker.clone())?;
                }
            }
            lua.set_named_registry_value(&format!("def:{label_owned}"), def)
        });
        Outcome {
            compile: None,
            ran: run.is_ok(),
            run: run.err(),
        }
    }

    /// Reads a field of a registry table after `run_module`: `def:<label>` (the definition) or
    /// `vars:<label>` (a script instance's variables after its first tick).
    pub fn field<R: sf_script::mlua::FromLua>(&mut self, key: &str, field: &str) -> R {
        let key = key.to_owned();
        self.ctx_guarded("read", |lua| {
            let def: Table = lua.named_registry_value(&key)?;
            def.get::<R>(field)
        })
        .unwrap()
    }

    /// Fallback (stub / later / missing live) call counts by name.
    pub fn fallback_calls(&mut self) -> Vec<(String, u32)> {
        let mut v = self
            .ctx_guarded("read", |lua| {
                let mw: Table = lua.named_registry_value("mw")?;
                let calls: Table = mw.get("fallback_calls")?;
                calls.pairs::<String, u32>().collect::<sf_script::mlua::Result<Vec<_>>>()
            })
            .unwrap();
        v.sort();
        v
    }
}

fn load(lua: &Lua, source: &str, name: &str) -> sf_script::mlua::Result<Function> {
    lua.load(source).set_name(name).set_mode(ChunkMode::Text).into_function()
}
