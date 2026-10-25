//! Entity scripts (scripting.md "Entity handles and entity scripts").
//!
//! A mod defines named entity scripts at start (`define_script("lamp", { activated = fn, tick =
//! fn })`, like named timers: code is looked up by name, so attachments are plain data) and
//! attaches them to placements (`attach(h, "lamp")`). Each attachment owns a plain-data state
//! table, passed to every handler as `handler(h, payload_or_dt, state)` and readable through
//! `state(h)` / `h:state()`.
//!
//! L3 decision: the states live in the owning context and travel in its snapshot (saves and hot
//! reload use one blob per mod), not in the placement overlay as the draft said; they survive
//! cell unloads because the context, not the cell, holds them. `sf_script` stays Bevy-free:
//! [`install`] adds the Lua functions to whatever module exposes them (`@sf/world` in
//! `sf_script_bevy`), and the runtime runs the handlers (`ScriptRuntime::activate`,
//! `tick_entities`).

use std::collections::{BTreeMap, HashSet};

use mlua::{AnyUserData, Function, Lua, Table, Value};
use serde::{Deserialize, Serialize};

use crate::context::CtxData;
use crate::handle::Handle;
use crate::state::{StateError, StateValue};

/// Registry table: script name → handler table.
pub(crate) const DEFS: &str = "sf.entity_defs";
/// Registry table: handle key → state table.
pub(crate) const STATES: &str = "sf.entity_states";

/// One attachment as plain data (in `ContextSnapshot`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityState {
    pub handle: Handle,
    pub script: String,
    pub state: StateValue,
}

fn key(h: Handle) -> String {
    format!("{:032x}", h.0)
}

fn handle_of(ud: &AnyUserData) -> mlua::Result<Handle> {
    ud.borrow::<Handle>()
        .map(|h| *h)
        .map_err(|_| mlua::Error::runtime("expected a handle"))
}

fn data(lua: &Lua) -> mlua::Result<mlua::AppDataRefMut<'_, CtxData>> {
    lua.app_data_mut::<CtxData>()
        .ok_or_else(|| mlua::Error::runtime("script context data missing"))
}

/// The handler `event` of `script` and the state of `h` (runtime side).
pub(crate) fn handler(lua: &Lua, h: Handle, script: &str, event: &str) -> mlua::Result<(Function, Table)> {
    let defs: Table = lua.named_registry_value(DEFS)?;
    let def: Table = defs.raw_get(script)?;
    let f: Function = def.raw_get(event)?;
    let states: Table = lua.named_registry_value(STATES)?;
    let state: Table = states.raw_get(key(h))?;
    Ok((f, state))
}

/// Adds `define_script`, `attach`, `detach`, `script_of` and `state` to `t`, and `state`,
/// `attach`, `detach` as handle methods.
pub fn install(lua: &Lua, t: &Table) -> mlua::Result<()> {
    t.set(
        "define_script",
        lua.create_function(|lua, (name, handlers): (String, Table)| {
            let mut hooks = HashSet::new();
            let copy = lua.create_table()?;
            for pair in handlers.pairs::<Value, Value>() {
                let (k, v) = pair?;
                if let (Value::String(k), Value::Function(f)) = (&k, &v) {
                    hooks.insert(k.to_str()?.to_owned());
                    copy.raw_set(k.clone(), f.clone())?;
                }
            }
            copy.set_readonly(true);
            let mut d = data(lua)?;
            if !d.starting {
                return Err(mlua::Error::runtime(
                    "define_script must run at context start (top level of a script)",
                ));
            }
            d.entity_defs.insert(name.clone(), hooks);
            drop(d);
            let defs: Table = lua.named_registry_value(DEFS)?;
            defs.raw_set(name, copy)
        })?,
    )?;
    let attach = lua.create_function(|lua, (h, name): (AnyUserData, String)| {
        let h = handle_of(&h)?;
        let mut d = data(lua)?;
        if !d.entity_defs.contains_key(&name) {
            return Err(mlua::Error::runtime(format!(
                "attach: no entity script defined as {name:?}"
            )));
        }
        let states: Table = lua.named_registry_value(STATES)?;
        match d.entities.get(&h) {
            Some(old) if *old == name => return states.raw_get::<Table>(key(h)),
            Some(old) => {
                return Err(mlua::Error::runtime(format!(
                    "attach: {h:?} already runs {old:?} (detach it first)"
                )));
            }
            None => {}
        }
        d.entities.insert(h, name);
        drop(d);
        let state = lua.create_table()?;
        states.raw_set(key(h), &state)?;
        Ok(state)
    })?;
    let detach = lua.create_function(|lua, h: AnyUserData| {
        let h = handle_of(&h)?;
        let removed = data(lua)?.entities.remove(&h).is_some();
        if removed {
            let states: Table = lua.named_registry_value(STATES)?;
            states.raw_set(key(h), Value::Nil)?;
        }
        Ok(removed)
    })?;
    let state = lua.create_function(|lua, h: AnyUserData| {
        let h = handle_of(&h)?;
        let states: Table = lua.named_registry_value(STATES)?;
        states.raw_get::<Option<Table>>(key(h))
    })?;
    let script_of = lua.create_function(|lua, h: AnyUserData| {
        let h = handle_of(&h)?;
        Ok(data(lua)?.entities.get(&h).cloned())
    })?;
    let methods = crate::handle::handle_methods(lua)?;
    for (name, f) in [("attach", &attach), ("detach", &detach), ("state", &state), ("script_of", &script_of)] {
        t.set(name, f)?;
        methods.raw_set(name, f)?;
    }
    Ok(())
}

/// Attachments with their states as plain data, in handle order.
pub(crate) fn snapshot(lua: &Lua, entities: &BTreeMap<Handle, String>) -> Result<Vec<EntityState>, StateError> {
    let states: Option<Table> = lua.named_registry_value(STATES).ok();
    entities
        .iter()
        .map(|(h, script)| {
            let v: Value = states
                .as_ref()
                .and_then(|t| t.raw_get(key(*h)).ok())
                .unwrap_or(Value::Nil);
            let state = StateValue::from_lua(&v, &format!("entity[{h:?}]"))?;
            Ok(EntityState {
                handle: *h,
                script: script.clone(),
                state,
            })
        })
        .collect()
}

/// Replaces attachments and states; returns the ones dropped because their script is no longer
/// defined.
pub(crate) fn restore(lua: &Lua, entities: &[EntityState]) -> mlua::Result<Vec<(Handle, String)>> {
    let states: Table = lua.named_registry_value(STATES)?;
    states.clear()?;
    let mut kept = BTreeMap::new();
    let mut dropped = Vec::new();
    let defined: HashSet<String> = data(lua)?.entity_defs.keys().cloned().collect();
    for e in entities {
        if !defined.contains(&e.script) {
            dropped.push((e.handle, e.script.clone()));
            continue;
        }
        let state = match e.state.to_lua(lua)? {
            Value::Table(t) => t,
            _ => lua.create_table()?,
        };
        states.raw_set(key(e.handle), state)?;
        kept.insert(e.handle, e.script.clone());
    }
    data(lua)?.entities = kept;
    Ok(dropped)
}
