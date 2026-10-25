//! `@sf/interfaces`: synchronous calls between mods (scripting.md "Curated API"; Factorio's
//! `remote.add_interface` / `remote.call`).
//!
//! A mod exports a table of functions at start (`export("api", { greet = fn })`); any mod gets a
//! proxy (`get("other_mod", "api")`) whose functions run in the exporting mod's own Luau state.
//! Arguments and results are copied as plain data ([`StateValue`]), so no reference crosses
//! states. The callee runs under its own watchdog and call limit (its errors and timeouts surface
//! as errors in the caller, who may `pcall`); re-entrant cycles (A → B → A) are errors. The
//! runtime publishes a context's exports only after it started (or reloaded) successfully, so a
//! failed reload keeps the old exports and proxies follow reloads by name.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use mlua::{Function, Lua, MultiValue, Table, Value, Variadic, WeakLua};

use crate::api::{ApiModule, InstallCx};
use crate::context::{CtxData, ScriptContext, unlimited};
use crate::state::StateValue;
use crate::watchdog::Watchdog;

/// Registry table: export name → function table.
pub(crate) const EXPORTS: &str = "sf.exports";

struct Export {
    lua: WeakLua,
    watchdog: Arc<Watchdog>,
    timeout: Duration,
    functions: Vec<String>,
}

/// Published exports of every context, shared by all contexts of a runtime.
#[derive(Default)]
pub(crate) struct Registry {
    exports: Mutex<HashMap<(String, String), Export>>,
    /// Contexts inside an interface call (caller and callees), for cycle detection.
    stack: Mutex<Vec<String>>,
}

impl Registry {
    /// Publishes `ctx`'s exports, replacing its previous ones.
    pub(crate) fn publish(&self, ctx: &ScriptContext) {
        let lua = ctx.lua();
        let names = lua
            .app_data_ref::<CtxData>()
            .map(|d| d.exports.clone())
            .unwrap_or_default();
        let tables: Option<Table> = lua.named_registry_value(EXPORTS).ok();
        let mut exports = self.exports.lock().unwrap_or_else(PoisonError::into_inner);
        exports.retain(|(c, _), _| c != ctx.id());
        for name in names {
            let Some(t) = tables.as_ref().and_then(|t| t.raw_get::<Option<Table>>(name.as_str()).ok().flatten())
            else {
                continue;
            };
            let mut functions: Vec<String> = t
                .pairs::<String, Function>()
                .filter_map(|p| p.ok().map(|(k, _)| k))
                .collect();
            functions.sort_unstable();
            exports.insert(
                (ctx.id().to_owned(), name),
                Export {
                    lua: lua.weak(),
                    watchdog: Arc::clone(ctx.watchdog()),
                    timeout: ctx.config().limits.call_timeout,
                    functions,
                },
            );
        }
    }
}

/// `@sf/interfaces`.
pub struct InterfacesModule {
    pub(crate) registry: Arc<Registry>,
}

fn to_state(args: &[Value], what: &str) -> mlua::Result<Vec<StateValue>> {
    args.iter()
        .enumerate()
        .map(|(i, v)| StateValue::from_lua(v, &format!("{what} {}", i + 1)).map_err(mlua::Error::external))
        .collect()
}

fn to_lua(lua: &Lua, values: &[StateValue]) -> mlua::Result<MultiValue> {
    unlimited(lua, || values.iter().map(|v| v.to_lua(lua)).collect::<mlua::Result<MultiValue>>())
}

struct Call<'a> {
    registry: &'a Registry,
    caller: &'a str,
    target: &'a str,
    name: &'a str,
    function: &'a str,
}

impl Call<'_> {
    fn label(&self) -> String {
        format!("interface {}.{}.{}", self.target, self.name, self.function)
    }

    fn run(&self, lua: &Lua, args: &[Value]) -> mlua::Result<MultiValue> {
        let args = to_state(args, "argument")?;
        let (weak, watchdog, timeout) = {
            let exports = self.registry.exports.lock().unwrap_or_else(PoisonError::into_inner);
            let e = exports
                .get(&(self.target.to_owned(), self.name.to_owned()))
                .ok_or_else(|| mlua::Error::runtime(format!("{}: not exported", self.label())))?;
            (e.lua.clone(), Arc::clone(&e.watchdog), e.timeout)
        };
        let callee = weak
            .try_upgrade()
            .ok_or_else(|| mlua::Error::runtime(format!("{}: context unavailable", self.label())))?;
        if !callee.app_data_ref::<CtxData>().is_some_and(|d| d.running) {
            return Err(mlua::Error::runtime(format!("{}: mod is not running", self.label())));
        }
        // The stack holds the outermost caller, then each callee: the caller is always on top.
        let depth = {
            let mut stack = self.registry.stack.lock().unwrap_or_else(PoisonError::into_inner);
            if self.caller == self.target || stack.iter().any(|c| c == self.target) {
                return Err(mlua::Error::runtime(format!("{}: re-entrant interface call", self.label())));
            }
            let depth = stack.len();
            if depth == 0 {
                stack.push(self.caller.to_owned());
            }
            stack.push(self.target.to_owned());
            depth
        };
        let result = (|| {
            let tables: Table = callee.named_registry_value(EXPORTS)?;
            let t: Table = tables.raw_get(self.name)?;
            let f: Function = t.raw_get(self.function)?;
            let args = to_lua(&callee, &args)?;
            watchdog.arm_from(Instant::now(), timeout);
            let r = f.call::<MultiValue>(args);
            let tripped = watchdog.disarm();
            let r = r.map_err(|e| {
                let what = if tripped { "timed out" } else { "failed" };
                mlua::Error::runtime(format!("{} {what}: {e}", self.label()))
            })?;
            to_state(&r.into_iter().collect::<Vec<_>>(), "result")
        })();
        self.registry
            .stack
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .truncate(depth);
        to_lua(lua, &result?)
    }
}

impl ApiModule for InterfacesModule {
    fn name(&self) -> &'static str {
        "@sf/interfaces"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &Lua, cx: &InstallCx<'_>) -> mlua::Result<Table> {
        let t = lua.create_table()?;
        t.set(
            "export",
            lua.create_function(|lua, (name, functions): (String, Table)| {
                let copy = lua.create_table()?;
                for pair in functions.pairs::<String, Function>() {
                    let (k, f) = pair.map_err(|_| {
                        mlua::Error::runtime("interfaces.export: the table must map names to functions")
                    })?;
                    copy.raw_set(k, f)?;
                }
                copy.set_readonly(true);
                let mut d = lua
                    .app_data_mut::<CtxData>()
                    .ok_or_else(|| mlua::Error::runtime("script context data missing"))?;
                if !d.starting {
                    return Err(mlua::Error::runtime(
                        "interfaces.export must run at context start (top level of a script)",
                    ));
                }
                if !d.exports.contains(&name) {
                    d.exports.push(name.clone());
                }
                drop(d);
                let tables: Table = lua.named_registry_value(EXPORTS)?;
                tables.raw_set(name, copy)
            })?,
        )?;
        let registry = Arc::clone(&self.registry);
        let caller = cx.context.to_owned();
        t.set(
            "get",
            lua.create_function(move |lua, (target, name): (String, String)| {
                let functions = {
                    let exports = registry.exports.lock().unwrap_or_else(PoisonError::into_inner);
                    match exports.get(&(target.clone(), name.clone())) {
                        Some(e) => e.functions.clone(),
                        None => return Ok(Value::Nil),
                    }
                };
                let proxy = lua.create_table()?;
                for function in functions {
                    let (registry, caller, target, name) =
                        (Arc::clone(&registry), caller.clone(), target.clone(), name.clone());
                    let fname = function.clone();
                    let f = lua.create_function(move |lua, args: Variadic<Value>| {
                        Call {
                            registry: &registry,
                            caller: &caller,
                            target: &target,
                            name: &name,
                            function: &fname,
                        }
                        .run(lua, &args)
                    })?;
                    proxy.raw_set(function, f)?;
                }
                proxy.set_readonly(true);
                Ok(Value::Table(proxy))
            })?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare sf_interfaces: {\n  \
         -- At context start: publish functions other mods may call (plain-data arguments and results).\n  \
         export: (name: string, functions: { [string]: (...any) -> ...any }) -> (),\n  \
         -- A proxy whose functions run in `mod_id`'s state, or nil if it exports no `name`.\n  \
         get: (mod_id: string, name: string) -> { [string]: (...any) -> ...any }?,\n}\n"
    }
}
