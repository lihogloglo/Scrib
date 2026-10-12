//! The curated API registry (scripting.md "Curated API and versioning") and the built-in modules
//! the runtime itself needs: `@sf/core` (identity, log, storage, random, time), `@sf/events` and
//! `@sf/timers`. L3 grows these into API v0.1; world, records, input, UI and the rest live with
//! the systems they expose and register through [`ApiModule`].

use mlua::{Function, Lua, Table, Value, Variadic};

use crate::context::{CtxData, Event, HANDLERS, STORAGE, TIMER_CBS};
use crate::state::StateValue;

/// What an API module may know about the context it's installed into.
pub struct InstallCx<'a> {
    /// The context id (usually the mod id).
    pub context: &'a str,
    /// Mods whose scripts run in this context.
    pub mods: &'a [String],
    /// The framework API version the runtime provides.
    pub api_version: &'a semver::Version,
}

/// One `require`-able API module. Declared once with its Luau types.
pub trait ApiModule: Send + Sync + 'static {
    /// `"@sf/world"`, `"@mw/script"`: `@<alias>/<path>`.
    fn name(&self) -> &'static str;
    /// The module's own version.
    fn version(&self) -> semver::Version;
    /// Builds the module table in `lua`. The runtime makes it read-only.
    fn install(&self, lua: &Lua, cx: &InstallCx<'_>) -> mlua::Result<Table>;
    /// Luau declarations for `.d.luau` generation.
    fn types(&self) -> &'static str;
}

fn data_mut(lua: &Lua) -> mlua::Result<mlua::AppDataRefMut<'_, CtxData>> {
    lua.app_data_mut::<CtxData>()
        .ok_or_else(|| mlua::Error::runtime("script context data missing"))
}

/// Logs through the `log` crate under target `sf_script`, prefixed with the context id.
pub(crate) fn log_fn(lua: &Lua) -> mlua::Result<Function> {
    lua.create_function(|lua, args: Variadic<Value>| {
        let parts: Vec<String> = args
            .iter()
            .map(|v| v.to_string().unwrap_or_else(|_| v.type_name().into()))
            .collect();
        let id = data_mut(lua)?.id.clone();
        log::info!(target: "sf_script", "[{id}] {}", parts.join("\t"));
        Ok(())
    })
}

/// `math.random` semantics on the context's PCG stream.
#[allow(clippy::cast_precision_loss)] // results are within the requested i64 range; Lua numbers are f64
pub(crate) fn random_fn(lua: &Lua) -> mlua::Result<Function> {
    lua.create_function(|lua, (m, n): (Option<i64>, Option<i64>)| {
        let mut d = data_mut(lua)?;
        Ok(match (m, n) {
            (None, _) => Value::Number(d.rng.next_f64()),
            (Some(m), None) if m >= 1 => Value::Number(d.rng.range(1, m) as f64),
            (Some(m), Some(n)) if m <= n => Value::Number(d.rng.range(m, n) as f64),
            _ => return Err(mlua::Error::runtime("random: interval is empty")),
        })
    })
}

/// `@sf/core`.
pub struct CoreModule;

impl ApiModule for CoreModule {
    fn name(&self) -> &'static str {
        "@sf/core"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &Lua, cx: &InstallCx<'_>) -> mlua::Result<Table> {
        let t = lua.create_table()?;
        t.set("api_version", cx.api_version.to_string())?;
        t.set("mod_id", cx.mods.first().map_or(cx.context, String::as_str))?;
        t.set("log", log_fn(lua)?)?;
        t.set("random", random_fn(lua)?)?;
        t.set(
            "storage",
            lua.create_function(|lua, ()| lua.named_registry_value::<Table>(STORAGE))?,
        )?;
        t.set(
            "time",
            lua.create_function(|lua, ()| Ok(data_mut(lua)?.sim_time))?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare core: {\n  api_version: string,\n  mod_id: string,\n  log: (...any) -> (),\n  \
         random: (m: number?, n: number?) -> number,\n  storage: () -> {[any]: any},\n  time: () -> number,\n}\n"
    }
}

/// `@sf/events`: `on(name, fn)`, `emit(name, payload)`. Mod events are namespaced
/// `<context>:<name>`; engine event names have no colon. Emitted events are delivered at the next
/// `Update` slot, in load order (scripting.md "Execution model").
pub struct EventsModule;

impl ApiModule for EventsModule {
    fn name(&self) -> &'static str {
        "@sf/events"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &Lua, _cx: &InstallCx<'_>) -> mlua::Result<Table> {
        let t = lua.create_table()?;
        t.set(
            "on",
            lua.create_function(|lua, (name, f): (String, Function)| {
                let handlers: Table = lua.named_registry_value(HANDLERS)?;
                let list: Table =
                    if let Some(l) = handlers.raw_get::<Option<Table>>(name.as_str())? {
                        l
                    } else {
                        let l = lua.create_table()?;
                        handlers.raw_set(name.as_str(), &l)?;
                        l
                    };
                list.raw_push(f)?;
                data_mut(lua)?.subscribed.insert(name);
                Ok(())
            })?,
        )?;
        t.set(
            "emit",
            lua.create_function(|lua, (name, payload): (String, Value)| {
                let payload =
                    StateValue::from_lua(&payload, "payload").map_err(mlua::Error::external)?;
                let mut d = data_mut(lua)?;
                let full = match name.split_once(':') {
                    None => format!("{}:{name}", d.id),
                    Some((ns, _)) if ns == d.id => name,
                    Some(_) => {
                        return Err(mlua::Error::runtime(format!(
                            "emit: can't emit into another namespace ({name})"
                        )));
                    }
                };
                let source = Some(d.id.clone());
                d.outbox.push(Event {
                    name: full,
                    payload,
                    source,
                });
                Ok(())
            })?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare events: {\n  on: (name: string, handler: (payload: any) -> any) -> (),\n  \
         emit: (name: string, payload: any?) -> (),\n}\n"
    }
}

/// `@sf/timers`: `register(name, fn)` at context start, then `after(seconds, name, payload)`,
/// `every(seconds, name, payload)` returning an id, `cancel(id)`.
pub struct TimersModule;

fn schedule(lua: &Lua, secs: f64, name: String, payload: &Value, every: bool) -> mlua::Result<u64> {
    let cbs: Table = lua.named_registry_value(TIMER_CBS)?;
    if cbs.raw_get::<Option<Function>>(name.as_str())?.is_none() {
        return Err(mlua::Error::runtime(format!(
            "timers: no callback registered as {name:?}"
        )));
    }
    if !secs.is_finite() || secs < 0.0 {
        return Err(mlua::Error::runtime(
            "timers: delay must be a finite number >= 0",
        ));
    }
    let payload = StateValue::from_lua(payload, "payload").map_err(mlua::Error::external)?;
    let mut d = data_mut(lua)?;
    let due = d.sim_time + secs;
    Ok(d.timers.schedule(name, due, every.then_some(secs), payload))
}

impl ApiModule for TimersModule {
    fn name(&self) -> &'static str {
        "@sf/timers"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &Lua, _cx: &InstallCx<'_>) -> mlua::Result<Table> {
        let t = lua.create_table()?;
        t.set(
            "register",
            lua.create_function(|lua, (name, f): (String, Function)| {
                if !data_mut(lua)?.starting {
                    return Err(mlua::Error::runtime(
                        "timers.register must run at context start (top level of a script)",
                    ));
                }
                let cbs: Table = lua.named_registry_value(TIMER_CBS)?;
                cbs.raw_set(name, f)
            })?,
        )?;
        t.set(
            "after",
            lua.create_function(|lua, (secs, name, payload): (f64, String, Value)| {
                schedule(lua, secs, name, &payload, false)
            })?,
        )?;
        t.set(
            "every",
            lua.create_function(|lua, (secs, name, payload): (f64, String, Value)| {
                schedule(lua, secs, name, &payload, true)
            })?,
        )?;
        t.set(
            "cancel",
            lua.create_function(|lua, id: u64| Ok(data_mut(lua)?.timers.cancel(id)))?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare timers: {\n  register: (name: string, callback: (payload: any) -> ()) -> (),\n  \
         after: (seconds: number, name: string, payload: any?) -> number,\n  \
         every: (seconds: number, name: string, payload: any?) -> number,\n  cancel: (id: number) -> boolean,\n}\n"
    }
}
